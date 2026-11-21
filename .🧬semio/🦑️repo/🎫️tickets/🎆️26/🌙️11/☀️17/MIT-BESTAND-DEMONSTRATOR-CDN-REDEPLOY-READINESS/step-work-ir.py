import argparse
import ast
import json
from pathlib import Path
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("lane", type=Path)
parser.add_argument("--native", action="store_true")
parser.add_argument("--context", type=Path)
parser.add_argument("--check-context", action="store_true")
args = parser.parse_args()
root = Path(__file__).resolve().parent
workspace = Path.cwd()
toolchain, = (workspace / ".cache/b/t/m/r").glob("*/toolchains/nightly-2026-07-20-*")
llvm = next(toolchain.glob("lib/rustlib/*/bin/llvm-dis"))
kind = "native" if args.native else "wasm"
build = args.lane / ("release/build" if args.native else "wasm32-wasip2/release/build")
scratch = root / "🗑️generated" / ("step-ir-" + kind)
scratch.mkdir(exist_ok=True)
deps = scratch / "d"
deps.mkdir(exist_ok=True)
products = [file for pattern in ("*.rlib", "*.rmeta") for file in build.glob("*/*/out/"+pattern)]
products.extend(file for pattern in ("*.dylib", "*.so", "*.dll") for file in (args.lane / "release/build").glob("*/*/out/"+pattern))
for library in products:
    local = deps / library.name
    if not local.exists():
        local.symlink_to(library.resolve())

extern = []
if args.check_context:
    owner, = (build / "semio-framework-job-context").glob("*/fingerprint/lib-semio_framework_job_context.json")
    sidecar, = (owner.parent.parent / "out").glob("libsemio_framework_job_context-*.rmeta.semio-deps")
    dependency_stems = {stem.rsplit("-", 1)[0]: stem for stem, _ in json.loads(sidecar.read_text())["dependencies"]}
    for _, name, _, _ in json.loads(owner.read_text())["deps"]:
        stem = "lib" + dependency_stems.get(name, name + "-*")
        products = list(deps.glob(stem + ".rmeta"))
        if not products:
            products = [file for file in deps.glob(stem + ".*") if file.suffix in {".dylib", ".so", ".dll"}]
        product, = products
        extern.extend(("--extern", f"{name}={product}"))
    entry = workspace / "🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/🦀️.rs"
    subprocess.run(["repo", "tools", "exec", "rustc", "--", str(entry), "--crate-name", "semio_framework_job_context",
                    "--crate-type=rlib", "--edition=2021", "--target=wasm32-wasip2", "-Z", "threads=1", "--cap-lints", "allow",
                    "--emit=metadata", "--out-dir", str(scratch), "-L", f"dependency={deps}", *extern], check=True)
    print("current Job Context metadata checked")
    raise SystemExit(0)

for name in ("semio_framework_job_receiving", "semio_framework_value_control_contract"):
    for extension in (("rlib", "rmeta") if args.native else ("rmeta",)):
        library, = sorted(deps.glob("lib" + name + "-*." + extension))
        extern.extend(("--extern", f"{name}={library}"))

context = args.context or workspace / "🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs"
source = context.read_text()
start = source.index("    fn with_original_effect_at<R>")
end = source.index("    pub fn retained_work_can_enter", start)
method = source[start:end]
common = """use semio_framework_value_control_contract::{
    OriginalSourceTurnIdentity, OriginalWork, OriginalWorkActionIdentity, OriginalWorkActionProof,
    OriginalWorkPerformedReceipt, OriginalWorkSink, RetainedCloneGrant, ValueError, ValueRefusalKind,
};
struct Context {
    fuel_remaining: u64,
    original_retirement: bool,
    deadline_us: u64,
    now_us: fn() -> Option<u64>,
    hits: u64,
}
"""
functions = []
for index in range(1, 25):
    functions.append(f"""
#[no_mangle]
pub fn run_{index}(fuel: u64, work: u64, until: u64, retiring: bool) -> (u64, u128, String) {{
    let mut context = Context {{fuel_remaining: fuel, original_retirement: retiring, deadline_us: until, now_us: || Some(12), hits: 0}};
    let turn = context.with_original_effect_at(None, RetainedCloneGrant::default(), |context, effect| {{
        context.hits += 1;
        let paid = effect.performed(work).unwrap_or(false);
        let invoked = effect.invocation().is_some();
        [if paid && invoked {{ 1 }} else {{ 2 }}; {index}]
    }});
    match turn {{
        Ok(Some((result, spent))) => (context.hits, spent, format!("return:{{}}", result[{index - 1}])),
        Ok(None) => (context.hits, 0, "denied".to_owned()),
        Err(error) => (context.hits, 0, format!("{{:?}}:{{}}", error.kind, error.message)),
    }}
}}
""")

if args.native:
    cases = json.loads((root / "step-work-cases.json").read_text())
    functions = ["""
use std::cell::Cell;
struct Output<'a, const N: usize> { bytes: [u8; N], drops: &'a Cell<u64> }
impl<const N: usize> Drop for Output<'_, N> {
    fn drop(&mut self) { self.drops.set(self.drops.get() + 1); }
}
fn native_law<const N: usize>(fuel: u64, work: Option<u64>, until: u64, retiring: bool) -> (u64, u64, u64, u128, String) {
    let drops = Cell::new(0);
    let mut context = Context {fuel_remaining: fuel, original_retirement: retiring, deadline_us: until, now_us: || Some(12), hits: 0};
    let turn = context.with_original_effect_at(None, RetainedCloneGrant::default(), |context, effect| {
        context.hits += 1;
        if let Some(work) = work { let _ = effect.performed(work); }
        Output { bytes: [u8::from(effect.invocation().is_some()); N], drops: &drops }
    });
    let before = drops.get();
    let (spent, status) = match turn {
        Ok(Some((result, spent))) => (spent, format!("return:{}", result.bytes[N - 1])),
        Ok(None) => (0, "denied".to_owned()),
        Err(error) => (0, format!("{:?}:{}", error.kind, error.message)),
    };
    (context.hits, before, drops.get(), spent, status)
}
fn main() {
"""]
    for idx, case in enumerate(cases, 1):
        fuel, work, until, retiring = case["args"]
        args_form = f"{fuel}, " + ("None" if work is None else f"Some({work})") + f", {until}, {str(retiring).lower()}"
        functions.append(f'println!("{{:?}}", native_law::<{idx}>({args_form}));')
    functions.append("}")

    def reference(case):
        fuel, work, until, retiring = case["args"]
        if not retiring and until <= 12:
            return (0, 0, 0, 0, "denied")
        if work is not None and work > fuel:
            return (1, 1, 1, 0, "WorkLimit:original effect performed more work than its ceiling")
        return (1, 0, 1, work or 0, "return:1")

production = "let performed=settle_original_effect_work" in method
for variant in (("production",) if production else ("current", "outlined")):
    chosen = method
    helpers = ""
    if variant == "production":
        lo = source.index("#[inline(never)]\nfn settle_original_effect_work")
        hi = source.index("fn replaced_context", lo)
        helpers = source[lo:hi]
    if variant == "outlined":
        chosen = chosen.replace(",OriginalWorkSink,", ",")
        chosen = chosen.replace("let mut receipt=None;let mut partial=None;", "let mut receipt=None;")
        lo = chosen.index("            let performed=OriginalWorkSink::observed_work")
        hi = chosen.index("                Ok(Some((output,performed)))", lo)
        chosen = chosen[:lo] + "            let performed=settle_original_effect_work(&mut work,&mut action,&mut receipt,&mut proof)?;\n" + chosen[hi:]
        lo = chosen.index("                if receipt.is_some()")
        hi = chosen.index("                Err(error)", lo)
        chosen = chosen[:lo] + "                recover_original_effect_work(&mut work,&mut action,&mut receipt,&mut proof);\n" + chosen[hi:]
        helpers = (root / "step-work-extract.rs").read_text()
    generated = common + helpers + "impl Context {\n" + chosen + "}\n" + "\n".join(functions)
    flags = ["--crate-name", "step_work_probe", "--edition=2021", "-Z", "threads=1", "-Zembed-metadata=no", "--cap-lints", "allow",
             "-L", f"dependency={deps}", *extern]
    if args.native:
        output = scratch / variant
        flags.extend(("-C", "opt-level=1", "-o", str(output)))
    else:
        output = scratch / variant
        output.mkdir(exist_ok=True)
        flags.extend(("--crate-type=rlib", "--target=wasm32-wasip2", "-C", "opt-level=s", "-C", "codegen-units=16",
                      "-C", "linker-plugin-lto", "-C", "strip=debuginfo", "--emit=llvm-bc", "--out-dir", str(output)))
    start = time.monotonic()
    subprocess.run(["repo", "tools", "exec", "rustc", "--", *flags, "-"], text=True, input=generated, check=True)
    elapsed = time.monotonic() - start
    if args.native:
        rows = [ast.literal_eval(line) for line in subprocess.check_output([str(output)], text=True).splitlines()]
        expect = [reference(case) for case in cases]
        if rows != expect:
            raise RuntimeError(f"{variant} native effect differs: {list(zip(rows, expect))}")
        for case, row in zip(cases, rows):
            print(variant, case["name"], json.dumps(row))
    else:
        files = sorted(output.glob("*.bc"))
        totals = {"bytes": 0, "functions": 0, "body_lines": 0, "work_loan_functions": 0, "work_loan_lines": 0}
        for file in files:
            totals["bytes"] += file.stat().st_size
            with subprocess.Popen([str(llvm), str(file), "-o", "-"], stdout=subprocess.PIPE, text=True) as dis:
                name = None
                for line in dis.stdout:
                    if line.startswith("define "):
                        name = line
                        totals["functions"] += 1
                        if "with_loan" in name:
                            totals["work_loan_functions"] += 1
                    elif line.startswith("}"):
                        name = None
                    elif name:
                        totals["body_lines"] += 1
                        if "with_loan" in name:
                            totals["work_loan_lines"] += 1
                if dis.wait():
                    raise RuntimeError("LLVM disassembly failed")
        print(variant, round(elapsed, 3), len(files), json.dumps(totals))
