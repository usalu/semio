"""📊 Inspect ticket-local render progress without modifying media."""

from pathlib import Path
import time

root = Path(__file__).resolve().parent / "🗑️generated/videos"
for media in sorted(root.iterdir()):
    if not media.is_dir() or not media.name.isdigit():
        continue
    videos = list((media / "v").glob("*.mp4"))
    partials = list((media / "p").rglob("*.mp4"))
    print("[DEBUG]", media.name, "chapters", len(videos), "partials", len(partials), "bytes", sum(path.stat().st_size for path in partials))
pre = root.parent / "pre"
print("[DEBUG] staged complete", sum(1 for _ in pre.glob("*/v/*.render.json")), "clips", sum(1 for _ in pre.glob("*/v/*.mp4")))
for number in (12, 22):
    partials = list((pre / str(number) / "p").rglob("*.mp4"))
    svgs = list((pre / str(number) / "t").glob("*.svg"))
    newest_partial = max((path.stat().st_mtime for path in partials), default=0)
    newest_svg = max((path.stat().st_mtime for path in svgs), default=0)
    print("[DEBUG] job", number, "partials", len(partials), "svg", len(svgs),
          "partial_age", round(time.time() - newest_partial), "svg_age", round(time.time() - newest_svg))
