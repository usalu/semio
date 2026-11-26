"""🗒️ Link the Note replay command against the exact published guest dependency identities."""
import json
from pathlib import Path
import re
import subprocess

ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
target = generated / "wfc"
guest = target / "wasm32-wasip2" / "prototype" / "build"
host = target / "prototype" / "build"
output = generated / "note-replay"
output.mkdir(exist_ok=True)
channel = re.search(r'^channel\s*=\s*"([^"]+)"', Path("rust-toolchain.toml").read_text(), re.M)[1]
rustc = ["repo", "tools", "exec", "--light", "rustup", "--", "run", channel, "rustc"]
owners = ["semio-hub-note","semio-s-artifact-note-note-mutations"]
roots = [max(guest.glob(name+"/*/out/*.rmeta"),key=lambda path:path.stat().st_mtime) for name in owners]
libraries = {path.stem[3:]: path for path in guest.glob("*/*/out/lib*.rlib")}
selected = {}
for root in roots:
    metadata = subprocess.check_output([*rustc, "-Zls=root", root], text=True)
    names = re.findall(r"^\d+ (\w+-[a-f0-9]+) hash ", metadata, re.M) + [root.stem[3:]]
    selected.update({name.rsplit("-", 1)[0]: libraries[name] for name in names if name in libraries})
search = {path.parent for path in selected.values()}
for pattern in ("*.dylib", "*.so", "*.dll"):
    search.update(path.parent for path in host.glob("*/*/out/" + pattern))
manifest = (ticket / "note-replay" / "Cargo.toml").read_text()
direct = [name.replace("-", "_") for name in re.findall(r"^(semio-[\w-]+)\s*=", manifest, re.M)]
binary = output / "note-mcp-replay.wasm"
args = ["--edition=2021", "--crate-name=note_mcp_replay", "--target=wasm32-wasip2", "-Cdebuginfo=0",
        "-Cpanic=abort", "-Zthreads=1", str(ticket / "note-mcp-replay.rs"), "-o", str(binary)]
for path in sorted(search):
    args.extend(["-L", "dependency=" + str(path)])
for name in direct:
    args.extend(["--extern", name + "=" + str(selected[name].with_suffix(".rmeta"))])
    args.extend(["--extern", name + "=" + str(selected[name])])
response = output / "wasi-replay.args"
response.write_text("\n".join(args) + "\n")
subprocess.run([*rustc, "@" + str(response)], check=True)
print(json.dumps({"binary": str(binary),"bytes": binary.stat().st_size,"owner": str(roots[0])}))
