"""📨 Execute both artifact refusal channels in the linked MCP port."""
import json
from pathlib import Path
import re
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
out = generated / "artifact-reply-law"
out.mkdir(exist_ok=True)
build = generated / "native-mcp/debug/build"
fixture = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🧪️tests/🔬️quick/artifact-reply.json")
registry = json.loads(subprocess.check_output(["repo", "tools", "list", "--json"]))
rustup = next(tool["command"] for tool in registry if tool["id"] == "rustup")
channel = re.search(r'^channel = "([^"]+)"', Path("rust-toolchain.toml").read_text(), re.M)[1]
rustc = [rustup, "run", channel, "rustc"]
library = max((build / "semio-os-mcp-port").glob("*/out/libsemio_os_mcp_port-*.rlib"), key=lambda path: path.stat().st_mtime)
metadata = subprocess.check_output([*rustc, "-Zls=root", library.with_suffix(".rmeta")], text=True, cwd=out)
units = {edge for edge in re.findall(r"^\d+ ([\w]+-[a-f0-9]+) hash", metadata, re.M)}
paths = {path.parent for path in build.glob("*/*/out/lib*") if path.stem[3:] in units}
native = {path.parent for path in build.glob("*/*/out/*.a")}
source = out / "main.rs"
source.write_text('''use semio_os_mcp_port::channel_port::{first_artifact_reply,AppFrame,Fault};
fn main() {
    for code in std::env::args().skip(1) {
        for framed in [false,true] {
            let fault=Fault { code:code.clone(), message:String::from("original diagnosis") };
            let result=if framed { Ok(vec![AppFrame::Error(fault)]) } else { Err(fault) };
            let error=first_artifact_reply(result).unwrap_err();
            assert_eq!(error.message,"original diagnosis");
            println!("{code} {:?} {}",error.code,error.retryable);
        }
    }
}
''')
binary = out / "law"
subprocess.run([*rustc, "--edition=2024", source, "--extern", f"semio_os_mcp_port={library.with_suffix('.rmeta')}", "--extern", f"semio_os_mcp_port={library}", *[f"-Ldependency={p}" for p in sorted(paths)], *[f"-Lnative={p}" for p in sorted(native)], "-o", binary], cwd=out, check=True)
cases = json.loads(fixture.read_text())
lines = subprocess.check_output([binary, *[case["fault"] for case in cases]], text=True).splitlines()
expected = [f'{case["fault"]} {case["code"].title().replace("_", "")} {str(case["retryable"]).lower()}' for case in cases for _ in range(2)]
assert lines == expected, (lines, expected)
print(f"{len(lines)} linked artifact refusal routes matched")
