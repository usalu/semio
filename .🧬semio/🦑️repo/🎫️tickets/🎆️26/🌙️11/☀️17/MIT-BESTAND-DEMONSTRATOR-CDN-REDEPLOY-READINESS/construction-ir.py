import argparse
from pathlib import Path
import os
import subprocess
import json
import time

parser = argparse.ArgumentParser()
parser.add_argument("lane", type=Path)
parser.add_argument("--native", action="store_true")
args = parser.parse_args()
workspace = Path.cwd()
directory = Path(__file__).resolve().parent
out = directory / "🗑️generated" / ("construction-native" if args.native else "construction-ir")
out.mkdir(exist_ok=True)
deps = out / "d"
deps.mkdir(exist_ok=True)
toolchain, = (workspace / ".cache/b/t/m/r").glob("*/toolchains/nightly-2026-07-20-*")
llvm = next(toolchain.glob("lib/rustlib/*/bin/llvm-dis"))
lane = args.lane.resolve()
if args.native:
    for path in (out / "cargo.jsonl").read_text().splitlines():
        row = json.loads(path)
        if row["reason"] == "compiler-artifact":
            for name in row["filenames"]:
                library = Path(name)
                local = deps / library.name
                if library.suffix in {".rmeta", ".rlib", ".dylib"} and not local.is_symlink():
                    local.symlink_to(library)
else:
    for base, pattern in [("wasm32-wasip2/release/build", "*.rmeta"), ("release/build", "*.dylib")]:
        for library in (lane / base).glob("*/*/out/" + pattern):
            local = deps / library.name
            if not local.is_symlink():
                local.symlink_to(library)
extern = []
required = ["semio_framework_job", "semio_framework_value", "semio_framework_value_retained_clone", "semio_framework_value_native_owned"]
if args.native:
    required.extend(("serde_json", "semio_framework_value_testing_allocator", "semio_framework_test_fixtures", "semio_framework_trace"))
for crate in required:
    for suffix in (("rlib", "rmeta") if args.native else ("rmeta",)):
        library, = sorted(deps.glob("lib" + crate + "-*." + suffix))
        extern.extend(("--extern", f"{crate}={library}"))

source = workspace / "🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs"
original = source.read_text()
helper = (directory / "construction-shared.rs").read_text()
candidate = original.replace(
    " let source=super::OriginalNativeReceivingOutcome::<()>::placement();let capture=",
    " construction_reserve_bytes(demand,work,std::mem::size_of::<Option<O>>())\n}\nfn construction_reserve_bytes(demand:OriginalConstructionDemand,work:u64,candidate:usize)->Result<(OriginalConstructionDemand,u64),ValueError>{\n let source=super::OriginalNativeReceivingOutcome::<()>::placement();let capture=",
).replace("demand.copy_bytes.checked_add(std::mem::size_of::<Option<O>>())", "demand.copy_bytes.checked_add(candidate)")
start = candidate.index("pub fn admit_original_construction<O>")
end = candidate.index("\n admission.validate(context)?;", start)
candidate = candidate[:end] + """
 let occupied = output.is_some();
 let mut call = CandidateCall { output, construct: Some(construct) };
 admit_construction_call(context, admission, demand, work, occupied, std::mem::size_of::<Option<O>>(), &mut call)
}
""" + helper + """
#[inline(never)]
fn admit_construction_call(context:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,demand:OriginalConstructionDemand,work:u64,occupied:bool,candidate:usize,call:&mut dyn ConstructionCall)->Result<OriginalConstructionStatus,OriginalTurnFailure>{
""" + candidate[end:]
start = candidate.index("fn admit_construction_call")
end = candidate.index("#[cfg(test)]", start)
body = candidate[start:end].replace("output.is_some()", "occupied")
body = body.replace("let candidate=std::mem::size_of::<Option<O>>();", "")
body = body.replace("original_construction_reserve::<O>(demand,work)?", "construction_reserve_bytes(demand,work,candidate)?")
body = body.replace(" context.with_native_receiving_quoted(source,", " let prepared = ScopedConstructionCall(call);\n context.with_native_receiving_quoted(source,")
body = body.replace("  let witness=native.source_witness()?;let mut construct=Some(construct);", "  let prepared = prepared;\n  let witness=native.source_witness()?;")
lo = body.index("    let construct=construct.take()")
hi = body.index("    let owed=", lo)
body = body[:lo] + "    let (step,total)=prepared.0.run(scope,effect,body_grant,work)?;\n" + body[hi:]
candidate = candidate[:start] + body + candidate[end:]

endpoints = []
for n in range(1, 1 if args.native else 25):
    endpoints.append(f"""
#[no_mangle]
pub fn admit_{n}(cx: &mut StepContext<'_>, output: &mut Option<[u8; {n}]>, admission: &mut construction::OriginalConstructionAdmission, value: u8) -> Result<construction::OriginalConstructionStatus, OriginalTurnFailure> {{
    let demand = construction::OriginalConstructionDemand {{ items: 1, copy_bytes: {n}, capacity_bytes: 0, release_bytes: 0, depth: 1 }};
    construction::admit_original_construction(cx, output, admission, demand, 1, |_, _| {{
        let progress = semio_framework_value_retained_clone::RetainedCloneProgress {{ copied_items: 1, copied_bytes: {n}, ..Default::default() }};
        semio_framework_value::RetainedWorkOutcome {{ result: Ok(([value; {n}], progress)), work: 1 }}
    }})
}}
""")
for variant, code in [("current", original), ("outlined", candidate)]:
    if args.native:
        test = source.parent / "🧪️tests/🦀️.rs"
        cases = test.read_text()
        cases = "\n".join(line for line in cases.splitlines() if not line.startswith("//!"))
        cases = cases.replace('include_str!("admission.json")', "include_str!(" + json.dumps(str(test.parent / "admission.json"), ensure_ascii=False) + ")")
        cases = cases.replace("let mut hits=Vec::new();", "let mut hits=Vec::new();let mut discarded=0;")
        original_call = "let turn=caller.refreshed_turn_with_decoder(quote_grant(quote),&mut decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut first,demand,1,|_,actual|valued_byte_birth(7,&ran_first,actual)));"
        drop_call = """
 let released=std::cell::Cell::new(0);
 let turn=caller.refreshed_turn_with_decoder(quote_grant(quote),&mut decoder,|cx|{
  let guard=ConstructorDropProbe(&released);
  let ran=&ran_first;
  let result=admit_original_construction_settled(cx,&mut admission,&mut first,demand,1,move|_,actual|{
   let result=valued_byte_birth(7,ran,actual);
   drop(guard);
   result
  });
  assert_eq!(released.get(),1,"constructor custody dropped exactly once");
  result
 });
 if admission.authority.is_some()&&!ran_first.get()&&released.get()==1{discarded+=1;}
"""
        assert cases.count(original_call) == 1
        cases = cases.replace(original_call, drop_call)
        cases = cases.replace('assert!(!hits.is_empty(),"no variant cut the reset");',
                              'assert!(!hits.is_empty(),"no variant cut the reset");\n assert!(discarded>0,"no variant declined after preparing native construction");')
        guard = """
struct ConstructorDropProbe<'a>(&'a std::cell::Cell<u8>);
impl Drop for ConstructorDropProbe<'_> {
 fn drop(&mut self) { self.0.set(self.0.get()+1); }
}
"""
        code = code.replace('#[path="🧪️tests/🦀️.rs"]\nmod tests;', "mod tests {\n" + cases + guard + "\n}")
        program = out / (variant + "-native")
        manifest = source.parent.parent / "📦️packages/🦀️rust"
        env = {**os.environ, "CARGO_MANIFEST_DIR": str(manifest)}
        crate = "pub use semio_framework_job::*;\npub mod construction {\n" + code + "\n}\n"
        subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "construction_ir", "--test",
                        "-Zembed-metadata=no", "-Z", "threads=1", "--cap-lints", "allow", "-L", f"dependency={deps}", *extern, "-o", str(program), "-"],
                       input=crate, env=env, text=True, check=True)
        print(variant, flush=True)
        subprocess.run([str(program), "--test-threads=1"], check=True)
        continue
    output = out / variant
    output.mkdir(exist_ok=True)
    crate = "pub use semio_framework_job::*;\npub mod construction {\n" + code + "\n}\n" + "\n".join(endpoints)
    start = time.monotonic()
    subprocess.run(["repo", "tools", "exec", "rustc", "--", "--edition=2021", "--crate-name", "construction_ir",
                    "--crate-type=rlib", "--target=wasm32-wasip2", "-C", "opt-level=s", "-C", "codegen-units=16",
                    "-C", "linker-plugin-lto", "-C", "strip=debuginfo", "-Zembed-metadata=no", "-Z", "threads=1",
                    "--cap-lints", "allow", "-L", f"dependency={deps}", *extern, "--emit=llvm-bc", "--out-dir", str(output), "-"],
                   input=crate, text=True, check=True)
    elapsed = time.monotonic() - start
    counts = {"bytes": 0, "body_lines": 0, "constructor_lines": 0, "constructor_functions": 0}
    for file in output.glob("*.bc"):
        counts["bytes"] += file.stat().st_size
        with subprocess.Popen([str(llvm), str(file), "-o", "-"], stdout=subprocess.PIPE, text=True) as dis:
            function = None
            for line in dis.stdout:
                if line.startswith("define "):
                    function = line
                    if any(name in line for name in ("admit_original_construction", "admit_construction_call", "CandidateCall")):
                        counts["constructor_functions"] += 1
                elif line.startswith("}"):
                    function = None
                elif function:
                    counts["body_lines"] += 1
                    if any(name in function for name in ("admit_original_construction", "admit_construction_call", "CandidateCall")):
                        counts["constructor_lines"] += 1
            if dis.wait():
                raise RuntimeError("LLVM disassembly failed")
    print(variant, round(elapsed, 3), json.dumps(counts), flush=True)
