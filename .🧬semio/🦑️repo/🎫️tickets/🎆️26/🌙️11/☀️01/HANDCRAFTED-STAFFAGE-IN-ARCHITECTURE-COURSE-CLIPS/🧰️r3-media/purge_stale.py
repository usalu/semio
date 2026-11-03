"""🧹 List or delete the preview files that no fresh lesson master of the media manifest backs, so old artwork cannot be served.

Usage: python purge_stale.py <en|de> [--apply]
Keeps the course preview of `course_preview.py` and its sidecars; every other file of `preview/` must equal the SHA-256 of a fresh master.
"""
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib

language, apply = sys.argv[1], "--apply" in sys.argv
folder = lib.base(language, "speaker")
document = json.loads((folder / "media-manifest.json").read_text(encoding="utf-8"))
fresh = {video["master"]["sha256"] for video in document["videos"].values() if video["master"].get("fresh")}
course = f"Architecture_course_speaker_{language}_480p15"
removed = 0
for file in sorted((folder / "preview").glob("*")):
    if file.name.startswith(course) or (file.suffix == ".mp4" and lib.digest(file) in fresh):
        continue
    if file.suffix in (".json", ".png") and file.with_suffix(".mp4").exists() and lib.digest(file.with_suffix(".mp4")) in fresh:
        continue
    print("[DEBUG]", "delete" if apply else "stale", file.name)
    removed += 1
    if apply:
        file.unlink()
print(f"[DEBUG] {removed} stale preview files")
