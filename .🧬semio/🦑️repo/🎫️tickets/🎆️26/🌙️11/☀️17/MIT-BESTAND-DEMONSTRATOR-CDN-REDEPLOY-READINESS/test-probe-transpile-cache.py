import json
import runpy
import subprocess
import tempfile
from pathlib import Path

ticket = Path(__file__).resolve().parent
locate = runpy.run_path(str(ticket / "probe-transpile-cache.py"))["locate"]
base = "semio_hub_fem_component"


def checksum(data):
    return subprocess.check_output(["node", "-e", "const h=require('crypto').createHash('sha256');process.stdin.on('data',v=>h.update(v)).on('end',()=>console.log(h.digest('hex')))"], input=data).decode().strip()


def refuses():
    try:
        locate(run, "fem")
    except (ValueError, OSError):
        return
    raise AssertionError("invalid input accepted")


with tempfile.TemporaryDirectory(dir=ticket / "🗑️generated") as scratch:
    root = Path(scratch)
    run = root / "runs/sample"
    raw = run / "w" / ("a" * 24) / "component.wasm"
    raw.parent.mkdir(parents=True)
    raw.write_bytes(b"component")
    (run / "r").mkdir()
    stages = [{"stage": s, "component": "semio-hub-fem-component", "ok": True} for s in ("stage", "transpile")]
    units = run / "r/units.json"
    units.write_text(json.dumps({"stages": stages}))
    key = {"stage": "transpile", "schemaVersion": 1, "inputs": {"component": checksum(raw.read_bytes())}, "tools": {"base": base}}
    identifier = checksum(json.dumps(key, sort_keys=True, separators=(",", ":")).encode())
    cache = root / "stages/transpile" / identifier
    cache.mkdir(parents=True)
    files = []
    for extension in (".js", ".d.ts", ".core.wasm", ".core2.wasm"):
        path = cache / (base + extension)
        body = extension.encode()
        path.write_bytes(body)
        files.append({"path": path.name, "bytes": len(body), "sha256": checksum(body)})
    manifest = cache / "🧾️entry.json"
    manifest.write_text(json.dumps({"key": key, "digest": identifier, "files": files}))
    assert locate(run, "fem") == cache
    second = cache / (base + ".core2.wasm")
    second.write_bytes(b"unexpected")
    refuses()
    second.unlink()
    refuses()
    second.write_bytes(b".core2.wasm")
    raw.write_bytes(b"other")
    refuses()
    raw.write_bytes(b"component")
    units.write_text(json.dumps({"stages": stages[:1]}))
    refuses()
    print("Exact raw input and complete two-core output accepted. Altered, missing, unrelated and unsuccessful inputs refused.")
