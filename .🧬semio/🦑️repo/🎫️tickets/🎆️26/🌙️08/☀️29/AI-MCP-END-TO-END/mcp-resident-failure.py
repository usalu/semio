from pathlib import Path
import json
import re
import struct
import subprocess
import sys
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
build = generated / "native-mcp/debug/build"
runtime = max((build / "semio-os-mcp-plugin-runtime").glob("*/out/*.rlib"), key=lambda p:p.stat().st_mtime)
unit = runtime.parent.parent
fprints = {}
search = []
native = []
for out in build.glob("*/*/out"):
    search.append(str(out))
    for lib in out.glob("*.rlib"):
        name = lib.name[3:].rsplit("-", 1)[0]
        stamp = out.parent / "fingerprint" / ("lib-" + name)
        if stamp.is_file():
            fprints[stamp.read_text().strip()] = lib
    if any(out.glob("*.a")):
        native.append(str(out))
config = json.loads((unit / "fingerprint/lib-semio_os_mcp_plugin_runtime.json").read_bytes())
externs = {}
for _, name, _, digest in config["deps"]:
    if name in ("serde_json","semio_framework","semio_framework_value","semio_os_kernel","semio_plugin_guest_contract","semio_plugin_host"):
        externs[name] = fprints[struct.pack("<Q",digest).hex()]
externs.setdefault("serde_json", max((build / "serde_json").glob("*/out/*.rlib"), key=lambda p:p.stat().st_mtime))
assert len(externs) == 6, sorted(externs)
tools = json.loads(subprocess.check_output(["repo", "tools", "list", "--json"]))
rustup = next(r["command"] for r in tools if r["id"] == "rustup")
channel = re.search(r'^channel\s*=\s*"([^"]+)"',Path("rust-toolchain.toml").read_text(),re.M)[1]
out = generated / ("mcp-resident-failure.exe" if sys.platform == "win32" else "mcp-resident-failure")
args = [rustup,"run",channel,"rustc","--test","--edition=2024","-Cdebuginfo=0",str(ticket / "mcp-resident-failure.rs"),"-o",str(out)]
for name, library in externs.items():
    metadata = library.with_suffix(".rmeta")
    if metadata.is_file():
        args.extend(["--extern",f"{name}={metadata}"])
    args.extend(["--extern",f"{name}={library}"])
for path in search:
    args.extend(["-L","dependency=" + path])
for path in native:
    args.extend(["-L","native=" + path])
response = generated / "mcp-resident-failure.args"
response.write_text("\n".join(args[4:])+"\n", encoding="utf8")
subprocess.run(args[:4]+["@"+str(response)],check=True)
subprocess.run([str(out),*sys.argv[1:]],check=True)
