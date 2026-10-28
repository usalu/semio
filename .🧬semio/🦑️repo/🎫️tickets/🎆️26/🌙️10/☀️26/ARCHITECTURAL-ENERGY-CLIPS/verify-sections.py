"""🎥 Retain exact media integrity and endpoint evidence for section previews."""

from pathlib import Path
import hashlib
import sys

import av

TICKET = Path(__file__).resolve().parent
BASE = TICKET / "🗑️generated" / "sections-preview"
FOLDER = (BASE / sys.argv[1]).resolve(strict=True)
if not FOLDER.is_relative_to(BASE.resolve()):
    raise RuntimeError("Preview identity must stay inside its output directory")

rows = []
for path in sorted(FOLDER.rglob("*.mp4")):
    if "partial_movie_files" in path.parts:
        continue
    with av.open(path) as container:
        stream = container.streams.video[0]
        rate = float(stream.average_rate)
        duration = float(stream.duration * stream.time_base)
        previous, count = -1, 0
        for frame in container.decode(video=0):
            if (frame.width, frame.height) != (1600, 900) or float(frame.time) < previous:
                raise RuntimeError("Invalid preview movie timeline or dimensions")
            previous, count = float(frame.time), count + 1
        if count != stream.frames or abs(previous + 1 / rate - duration) > 0.02:
            raise RuntimeError("Incomplete preview movie decoding")
    rows.append((path.name, count, rate, duration, hashlib.sha256(path.read_bytes()).hexdigest()))
if len(rows) != 2:
    raise RuntimeError(f"Expected two real movies, found {len(rows)}")

lines = ["# Uniform Section Preview Media", "", f"Current actual-helper preview input identity: `{FOLDER.name}`.", "",
         "Both previews render real open/close animations through the production house, plant and room constructors. Every frame was decoded with PyAV; dimensions and nondecreasing timestamps were checked. This is bounded section verification, not a rerender of all five curriculum movies.", "",
         "| Movie | Frames | FPS | Duration (s) | SHA-256 |", "|---|---:|---:|---:|---|"]
for name, count, rate, duration, digest in rows:
    lines.append(f"| {name} | {count} | {rate:g} | {duration:.3f} | {digest} |")
lines += ["", "## Endpoint Images", "", "| PNG | SHA-256 | Matches Initial Preview |", "|---|---|---|"]
for path in sorted(FOLDER.glob("*.png")):
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    initial = BASE / "291cef86846f" / path.name
    same = initial.is_file() and hashlib.sha256(initial.read_bytes()).hexdigest() == digest
    lines.append(f"| {path.name} | {digest} | {'yes' if same else 'no'} |")
lines += ["", "All six endpoint images are independently reviewed in sections-audit.md. The initial endpoint-only preview is retained in the audit chronology; the movies above establish actual frame generation through both directions of window movement. Reproduce with the managed authoring environment and `repo tools exec uv -- run --project <authoring-project> --python 3.12 --group test python -B <ticket>/sections-preview.py`, then this input with the printed identity.", "",
          "Temporary binaries, PNGs, fonts and logs are deleted at ticket close. This Markdown and the preview/verifier inputs remain.", ""]
(TICKET / "sections-media.md").write_text("\n".join(lines), encoding="utf-8")
print("[DEBUG] Decoded " + str(sum(row[1] for row in rows)) + " section preview frames from two movies.")
