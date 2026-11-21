"""🗃️ Stages stable Cargo compiler identity for test execution policy."""

import json
import sys
from pathlib import Path
import re

ticket = Path(__file__).resolve().parent
output = ticket / "🗑️generated/cargo-test-lane"
output.mkdir(exist_ok=True)
root = Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching").resolve()
owner = root / "💾️storage/🔒️enforcement/🏗️compiler/🎯️product/🟦️.ts"
tests = root / "🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🎯️product/🧪️tests/🟦️.ts"
fixture = tests.with_name("🔣️.json")
originals = {path: path.read_bytes() for path in (owner, tests, fixture)}
source = originals[owner].decode("utf-8")
anchor = ' const hash=createHash("sha256");'
assert source.count(anchor) == 1
source = source.replace(anchor, ''' const separator=captured.arguments.indexOf("--");
 const productArguments=captured.command==="test"
  ? captured.arguments.filter((argument,index)=>argument!=="--no-fail-fast"||(separator!==-1&&index>=separator))
  : captured.arguments;
''' + anchor).replace("captured.toolchain,...captured.arguments]", "captured.toolchain,...productArguments]")
source = source.replace("Bounded exact spelling preserves path-observable compiler behavior without conflating execution owners.",
                        "Bounded compiler arguments select the lane while the caller retains Cargo's execution options.")
(output / "product.ts").write_text(source)

cases = [
    {"command": "test", "arguments": ["-p", "pdf", "--lib", "--no-fail-fast"],
     "productArguments": ["-p", "pdf", "--lib"]},
    {"command": "test", "arguments": ["-p", "pdf", "--lib", "--no-fail-fast", "--", "--no-fail-fast"],
     "productArguments": ["-p", "pdf", "--lib", "--", "--no-fail-fast"]},
    {"command": "test", "arguments": ["-p", "pdf", "--no-fail-fast"],
     "productArguments": ["-p", "pdf"]},
    {"command": "test", "arguments": ["-p", "another", "--lib", "--no-fail-fast"],
     "productArguments": ["-p", "another", "--lib"]},
    {"command": "build", "arguments": ["-p", "pdf", "--lib", "--no-fail-fast"],
     "productArguments": ["-p", "pdf", "--lib", "--no-fail-fast"]},
]
fixture_data = json.loads(fixture.read_text())
original = originals[fixture].decode("utf-8").rstrip()
assert original.endswith("}") and "testRunnerOptions" not in fixture_data
body = json.dumps(cases, ensure_ascii=False, indent=2).splitlines()
(output / "🔣️.json").write_text(original[:-1].rstrip() + ',\n  "testRunnerOptions": ' + body[0] + "\n"
                                + "\n".join("  " + line for line in body[1:]) + "\n}\n")
test_source = originals[tests].decode("utf-8")
position = test_source.index('test("exact product claims')
test_source = test_source[:position] + '''test("Cargo test execution policy reuses the compiler product lane",()=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:compiler-product-test-policy",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),async original=>{
 const control=createIssuedResourceControlV1(original,"test:product-test-policy",0),keys:string[]=[];
 for(const row of fixture.testRunnerOptions){
  const intent={...fixture.intent,command:row.command,arguments:row.arguments} as CompilerProductIntentV1;
  const preimage="semio.cargo.product/v1\\n"+[intent.version,intent.root,intent.manifest,intent.mode,intent.command,intent.producer,intent.toolchain,...row.productArguments].map(value=>JSON.stringify(value)+"\\n").join("");
  const actual=await compilerProductKeyV1(intent,control);
  expect(actual).toBe(new Bun.CryptoHasher("sha256").update(preimage).digest("hex"));
  expect(await compilerProductKeyV1({...intent,arguments:row.productArguments},control)).toBe(actual);
  expect(intent.arguments).toEqual(row.arguments);
  keys.push(actual);
 }
 expect(new Set(keys).size).toBe(keys.length);
}));

''' + test_source[position:]
(output / "product-tests.ts").write_text(test_source)

def imports(text, location, alternatives):
    def relocate(m):
        path = (location.parent / m[2]).resolve()
        return m[1] + json.dumps(alternatives.get(path, path).as_posix(), ensure_ascii=False)
    return re.sub(r'(from\s+)"(\.[^"\n]+)"', relocate, text)

(output / "product-live.ts").write_text(imports(source, owner, {}))
(output / "product-probe.test.ts").write_text(imports(test_source, tests, {
    owner: output / "product-live.ts", fixture: output / "🔣️.json",
}))
(output / "overlay.json").write_text(json.dumps({str(owner.relative_to(Path.cwd())): "product.ts",
    str(tests.relative_to(Path.cwd())): "product-tests.ts", str(fixture.relative_to(Path.cwd())): "🔣️.json"}, ensure_ascii=False, indent=2) + "\n")

if "--apply" in sys.argv[1:]:
    for path in originals:
        assert path.read_bytes() == originals[path], f"source changed during staging {path}"
    for path, candidate in [(owner, "product.ts"), (tests, "product-tests.ts"), (fixture, "🔣️.json")]:
        assert path.read_bytes() == originals[path], f"source changed before edit {path}"
        path.write_bytes((output / candidate).read_bytes())
    print("[DEBUG] Applied the Cargo product lane policy and fixture")
