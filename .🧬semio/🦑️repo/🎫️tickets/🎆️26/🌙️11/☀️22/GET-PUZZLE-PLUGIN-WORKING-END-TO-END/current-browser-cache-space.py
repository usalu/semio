from pathlib import Path
import shutil

generated = Path(__file__).resolve().parent / "🗑️generated"
profile = generated / "current-wasm/b/wasm32-unknown-unknown/prototype"
for crate in sorted((profile / "incremental").iterdir(), key=lambda path: path.stat().st_mtime):
    if shutil.disk_usage(profile).free >= 22 * 1024 ** 3:
        break
    if not crate.is_dir() or any(crate.glob("s-*-working")):
        continue
    package = crate.name.rsplit("-", 1)[0].replace("_", "-")
    completed = (profile / "build" / package).glob("*/out/*.rlib")
    archive_time = max((archive.stat().st_mtime for archive in completed), default=0)
    for snapshot in crate.glob("s-*"):
        if snapshot.is_dir() and not snapshot.name.endswith("-working") and snapshot.stat().st_mtime <= archive_time and not any(crate.glob("s-*-working")):
            shutil.rmtree(snapshot)
print(shutil.disk_usage(profile).free >> 30, "GiB available")
