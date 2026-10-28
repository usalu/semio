"""🖼️ Extract hashed actual-source review frames without rendering timelines."""

from pathlib import Path
import hashlib
import json
import sys

import av
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
SOURCE = TICKET / "🗑️generated" / "full" / "s"
OUTPUT = TICKET / "🗑️generated" / "source-review"
OUTPUT.mkdir(parents=True, exist_ok=True)


def extract(selection: str) -> dict:
    """🎞️ Decode deterministic source frames and retain media fingerprints."""
    ordinal, _, chosen = selection.partition(chr(58))
    paths = [path for path in (SOURCE / ordinal / "v").glob("*.mp4") if ".partial" not in path.name and path.stat().st_size > 1000]
    if len(paths) != 1:
        raise ValueError(f"Expected one complete source for {ordinal}: {paths}")
    source = paths[0]
    if not source.with_suffix(".render.json").is_file():
        raise ValueError(f"Source is not published: {source}")
    digest = hashlib.sha256(source.read_bytes()).hexdigest()[:12]
    with av.open(source) as container:
        stream = container.streams.video[0]
        duration = float(stream.duration * stream.time_base)
        targets = [float(value) for value in chosen.split(chr(44))] if chosen else [duration * fraction for fraction in (0.12, 0.30, 0.48, 0.66, 0.84, 0.97)]
        digest += chr(45) + hashlib.sha256(str(targets).encode()).hexdigest()[:6]
        frames = []
        for frame in container.decode(video=0):
            timestamp = float(frame.time)
            while len(frames) < len(targets) and timestamp >= targets[len(frames)]:
                frames.append((timestamp, frame.to_image()))
            if len(frames) == len(targets):
                break
    if len(frames) != len(targets):
        raise ValueError(f"Incomplete source: {source}")
    sheet = Image.new("RGB", (1280, 1170), (22, 22, 22))
    draw = ImageDraw.Draw(sheet)
    images = []
    for index, (timestamp, image) in enumerate(frames):
        name = f"{ordinal}-{digest}-{index + 1}.png"
        image.save(OUTPUT / name)
        images.append({"time": timestamp, "image": name})
        x, y = (index % 2) * 640, (index // 2) * 390
        draw.text((x + 8, y + 6), f"{ordinal}: {source.stem} | {timestamp:.2f}s / {duration:.2f}s", fill="white")
        sheet.paste(image, (x, y + 26))
    contact = OUTPUT / f"{ordinal}-{digest}-contact.png"
    sheet.save(contact)
    result = {"ordinal": ordinal, "source": str(source), "hash": digest, "duration": duration, "contact": str(contact), "frames": images}
    (OUTPUT / f"{ordinal}-{digest}.json").write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(result, ensure_ascii=False), flush=True)
    return result


if __name__ == "__main__":
    for ordinal in sys.argv[1:]:
        extract(ordinal)
