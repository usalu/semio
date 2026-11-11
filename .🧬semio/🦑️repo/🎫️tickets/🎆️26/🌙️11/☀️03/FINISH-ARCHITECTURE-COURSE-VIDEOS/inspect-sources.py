"""🖼️ Create contact sheets of available current source renders."""
import json
import sys
from pathlib import Path
import av
from PIL import Image, ImageDraw
HERE = Path(__file__).resolve().parent
MEDIA = HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"
sys.path.insert(0, str(MEDIA))
import lib
out = HERE / "🗑️generated" / "source-review"
out.mkdir(parents=True, exist_ok=True)
for old in out.glob("*.png"):
    if old.stem.isdigit():
        old.unlink()
records = []
base = lib.base(sys.argv[1], "speaker") / "s"
expected = {(str(job["source_ordinal"]), job["segment"]["scene"]) for job in lib.runtime().segment_plan()}
for folder in sorted(base.iterdir(), key=lambda path: int(path.name) if path.name.isdigit() else 99999):
    if not folder.name.isdigit():
        continue
    for video in sorted((folder / "v").glob("*.mp4")):
        if (folder.name, video.stem) not in expected:
            continue
        certificate = video.with_suffix(".render.json")
        if not certificate.is_file():
            continue
        with av.open(str(video)) as container:
            stream = container.streams.video[0]
            at = float(stream.duration * stream.time_base) * .5
            container.seek(int(at / stream.time_base), stream=stream, backward=True)
            chosen = None
            for frame in container.decode(stream):
                chosen = frame
                if frame.time is not None and frame.time >= at:
                    break
            if chosen is None:
                raise ValueError(str(video))
            records.append((folder.name, video.stem, chosen.to_image()))
for start in range(0, len(records), 6):
    sheet = Image.new("RGB", (1281, 540), "white")
    draw = ImageDraw.Draw(sheet)
    for offset, (ordinal, name, frame) in enumerate(records[start:start + 6]):
        x, y = offset % 3 * 427, offset // 3 * 270
        draw.text((x + 4, y + 6), ordinal + " " + name, fill="black")
        sheet.paste(frame.resize((427, 240), Image.Resampling.LANCZOS), (x, y + 30))
    sheet.save(out / f"{start // 6 + 1:02d}.png")
(out / "index.json").write_text(json.dumps([{"source": ordinal, "scene": name} for ordinal, name, _ in records], indent=2), encoding="utf-8")
print("[DEBUG]", len(records), "source midpoints extracted; certificates present, canonical freshness must be checked separately", flush=True)
