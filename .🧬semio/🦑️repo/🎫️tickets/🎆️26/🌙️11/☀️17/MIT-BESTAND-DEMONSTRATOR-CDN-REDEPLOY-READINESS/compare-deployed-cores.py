"""🔍 Compare a static site's delivered modules with the staged compiler outputs."""
import hashlib
import sys
import runpy
from pathlib import Path

ticket = Path(__file__).resolve().parent
checks = runpy.run_path(str(ticket / "release-receipts.py"))
site, staged = (Path(arg).resolve() for arg in sys.argv[1:])
count = 0
for group in ("🔌️plugin-modules", "🧩️extension-modules"):
    for module in (site / group).iterdir():
        if not (module / "🧾️receipt.json").is_file():
            continue
        source = staged / module.name
        files = []
        for core in source.rglob("*.wasm"):
            digest = hashlib.sha256()
            with core.open("rb") as stream:
                for block in iter(lambda: stream.read(1024 * 1024), b""):
                    digest.update(block)
            files.append({"path": core.relative_to(source).as_posix(), "bytes": core.stat().st_size, "sha256": digest.hexdigest()})
        checks["verify_recorded_cores"](module, {"files": files})
        checks["verify_deployed_module"](source, module)
        count += 1
if count == 0:
    raise ValueError("No modules verified")
print(f"{count} staged module inventories, bindings and decoded WebAssembly payloads matched the candidate")
