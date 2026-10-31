"""📚 List every video of the architecture course manifest with its segments and the newest existing render per language.

Usage: python course.py [en|de]
"""
import json
import sys
from pathlib import Path

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.stdout.reconfigure(encoding="utf-8")
language = sys.argv[1] if len(sys.argv) > 1 else "en"
manifest = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
videos = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / "speaker"
for video in manifest["videos"]:
    segments = [segment for chapter in video["chapters"] for segment in chapter["segments"]]
    output = video.get("output", video["id"])
    found = sorted((p for p in videos.rglob(f"{output}_speaker_{language}_*.mp4")), key=lambda p: p.stat().st_mtime)
    print(f"{video['id']:32} {output:28} chapters={len(video['chapters'])} segments={len(segments)} render={found[-1].relative_to(videos) if found else '-'}")
