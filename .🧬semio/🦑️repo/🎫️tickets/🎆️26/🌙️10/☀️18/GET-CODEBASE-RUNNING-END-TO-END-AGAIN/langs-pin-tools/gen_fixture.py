import json, subprocess, sys
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧰️tool-setup"
table = json.load(open(D + "/🔣️.json", encoding="utf8"))
TC = "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🧰️toolchains/🔣️.json"
BOOT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json"
TOOLS = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"
files = {
 "package.json": json.dumps({"packageManager": "bun@9.8.7", "devDependencies": {"playwright": "^1.0.0"}}),
 TC: json.dumps({"go": {"tool": {"version": "9.9.9"}}, "python": {"tool": {"version": "8.8.8"}, "interpreters": {"main": {"series": "7.7"}, "oracle": {"series": "6.6"}}}, "dotnet": {"sdk": "5.5.5"}, "cmake": {"cmake": {"version": "4.4.4"}, "ninja": {"version": "3.3.3"}}}),
 BOOT: json.dumps({"definitions": {"Pins": {"const": {"assets": [{"tool": "go", "version": "9.9.9"}, {"tool": "node", "version": "11.1.1", "platform": "darwin"}, {"tool": "node", "version": "11.1.1", "platform": "linux"}, {"tool": "rustup", "version": "2.2.2"}]}}}}),
 TOOLS + "/🏗️bazel/🧬️schema/🔣️.json": json.dumps({"definitions": {"Release": {"const": {"version": "1.2.3"}}}}),
 TOOLS + "/🕸️wasm/🧬️schema/🔣️.json": json.dumps({"definitions": {"Release": {"const": {"version": "77"}}}}),
 "🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/🔣️.json": json.dumps({"version": "0.0.1"}),
 "rust-toolchain.toml": '[toolchain]\nprofile = "minimal"\nchannel = "nightly-2030-01-02"\ncomponents = ["rust-src"]\n',
 "Cargo.lock": '[[package]]\nname = "wasm-bindgen-macro"\nversion = "0.1.1"\n\n[[package]]\nname = "wasm-bindgen"\nversion = "0.9.9"\n\n[[package]]\nname = "wasm-bindgen-shared"\nversion = "0.1.1"\n',
 "node_modules/.bun/playwright-core@4.0.0/node_modules/playwright-core/browsers.json": json.dumps({"browsers": [{"name": "firefox", "revision": "1"}, {"name": "chromium", "revision": "4242"}]}),
}
packages = {"playwright-core": "node_modules/.bun/playwright-core@4.0.0/node_modules/playwright-core"}
matches = [{"found": "3.14.4", "pin": "3.14"}, {"found": "3.140.1", "pin": "3.14"}, {"found": "3.1", "pin": "3.14"}, {"found": "1.4.2", "pin": "1.4.2"}, {"found": "1.4.20", "pin": "1.4.2"}, {"found": "nightly-2030-01-02", "pin": "nightly-2030-01-02"}, {"found": "130", "pin": "130"}, {"found": "1300", "pin": "130"}]
oracle = D + "/🧪️tests/🐍️oracle/🐍️.py"
run = subprocess.run([sys.executable, "-I", oracle], input=json.dumps({"table": table, "files": files, "packages": packages, "matches": matches}), capture_output=True, text=True, check=True)
result = json.loads(run.stdout)
fixture = {"version": 1, "files": files, "packages": packages, "expected": result["pins"], "matches": [{**case, "expected": verdict} for case, verdict in zip(matches, result["matches"])]}
open(D + "/🧪️tests/🧫️fixtures/🔣️.json", "w", encoding="utf8").write(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
print(json.dumps(result["pins"]))
