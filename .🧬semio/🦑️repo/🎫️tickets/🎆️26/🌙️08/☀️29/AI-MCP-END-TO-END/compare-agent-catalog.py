from pathlib import Path
import json
import re
import struct
import subprocess
import sys
ticket = Path(__file__).resolve().parent
generated = ticket / "🗑️generated"
build = generated / "native-mcp/debug/build"
catalog = max((build / "semio-os-mcp-catalog").glob("*/out/*.rlib"), key=lambda p:p.stat().st_mtime)
unit = catalog.parent.parent
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
config = json.loads((unit / "fingerprint/lib-semio_os_mcp_catalog.json").read_bytes())
externs = {"semio_os_mcp_catalog": catalog}
for _, name, _, digest in config["deps"]:
    if name in ("serde_json", "semio_ui_locale"):
        externs[name] = fprints[struct.pack("<Q",digest).hex()]
assert len(externs) == 3
tools = json.loads(subprocess.check_output(["repo", "tools", "list", "--json"]))
rustup = next(r["command"] for r in tools if r["id"] == "rustup")
channel = re.search(r'^channel\s*=\s*"([^"]+)"',Path("rust-toolchain.toml").read_text(),re.M)[1]
out = generated / ("compare-agent-catalog.exe" if sys.platform == "win32" else "compare-agent-catalog")
args = [rustup,"run",channel,"rustc","--edition=2024","-Cdebuginfo=0",str(ticket / "compare-agent-catalog.rs"),"-o",str(out)]
for name, library in externs.items():
    metadata = library.with_suffix(".rmeta")
    if metadata.is_file():
        args.extend(["--extern",f"{name}={metadata}"])
    args.extend(["--extern",f"{name}={library}"])
for path in search:
    args.extend(["-L","dependency=" + path])
for path in native:
    args.extend(["-L","native=" + path])
response = generated / "compare-agent-catalog.args"
response.write_text("\n".join(args[4:])+"\n", encoding="utf8")
subprocess.run(args[:4]+["@"+str(response)],check=True)
subprocess.run([str(out),*sys.argv[1:]],check=True)
