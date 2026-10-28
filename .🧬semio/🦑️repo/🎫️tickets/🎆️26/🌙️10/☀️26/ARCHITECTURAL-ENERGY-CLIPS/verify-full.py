"""🎞️ Decode completed curriculum movies and prepare representative visual review sheets."""

from __future__ import annotations

import importlib
import argparse
import hashlib
import json
from pathlib import Path
import sys

ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))

import av
from PIL import Image, ImageDraw

RUNTIME = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
OUTPUT = GENERATED / "full"
REVIEW = GENERATED / "full-review"
REVIEW.mkdir(parents=True, exist_ok=True)
PREVIOUS = {row["id"]: row for row in json.loads((REVIEW / "metadata.json").read_text(encoding="utf-8"))} if (REVIEW / "metadata.json").is_file() else {}


def clock(seconds: float) -> str:
    """⏱️ Format decoded media timestamps for review captions."""
    minutes, seconds = divmod(seconds, 60)
    return f"{int(minutes):02d}:{seconds:04.1f}"


def verify(video: dict) -> dict:
    """🔍 Decode every frame and retain eight deterministic representative frames."""
    path = OUTPUT / "rendered" / f"{video['output']}_640x360p6.mp4"
    if not path.is_file() or path.stat().st_size <= 1000:
        raise FileNotFoundError(path)
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    previous = PREVIOUS.get(video["id"])
    if previous and previous.get("media_sha256") == digest and all(Path(item).is_file() for item in [previous["contact_sheet"], *previous["frames"]]):
        print(f"[DEBUG] Reusing unchanged full-decode evidence for {video['id']}", flush=True)
        return previous
    with av.open(path) as container:
        stream = container.streams.video[0]
        duration = float(stream.duration * stream.time_base) if stream.duration else float(container.duration / av.time_base)
        rate = float(stream.average_rate)
        if (stream.width, stream.height) != (640, 360) or abs(rate - 6) > 0.001:
            raise ValueError(f"Unexpected media format: {path}")
        targets = [min(3.0, duration * 0.005), *[duration * fraction for fraction in (0.08, 0.20, 0.35, 0.50, 0.65, 0.80, 0.95)]]
        selected, count, previous = [], 0, -1.0
        for frame in container.decode(video=0):
            count += 1
            current = float(frame.time)
            if current < previous or (frame.width, frame.height) != (640, 360):
                raise ValueError(f"Invalid decoded frame timeline: {path}")
            previous = current
            while len(selected) < len(targets) and current >= targets[len(selected)]:
                selected.append((current, frame.to_image()))
        if not count or len(selected) != len(targets):
            raise ValueError(f"Incomplete movie decoding: {path}")
        audio_streams = len(container.streams.audio)
        codec = stream.codec_context.name
    identity = digest[:12]
    folder = REVIEW / f"{video['id']}-{identity}"
    folder.mkdir(exist_ok=True)
    sheet = Image.new("RGB", (1320, 440), (20, 20, 20))
    draw = ImageDraw.Draw(sheet)
    frame_paths = []
    for index, (timestamp, frame) in enumerate(selected):
        target = folder / f"frame-{index + 1:02d}.jpg"
        frame.save(target, quality=92)
        frame_paths.append(str(target))
        x, y = 12 + (index % 4) * 326, 12 + (index // 4) * 210
        draw.text((x, y), f"{video['id']}  {clock(timestamp)}", fill=(235, 235, 235))
        sheet.paste(frame.resize((320, 180)), (x, y + 20))
    contact = REVIEW / f"{video['id']}-{identity}.png"
    sheet.save(contact)
    result = {"id": video["id"], "path": str(path), "bytes": path.stat().st_size, "width": 640, "height": 360,
              "fps": rate, "duration": duration, "decoded_frames": count, "last_frame_time": previous,
              "codec": codec, "audio_streams": audio_streams, "media_sha256_prefix": identity, "media_sha256": digest, "contact_sheet": str(contact), "frames": frame_paths}
    print(f"[DEBUG] Verified {video['id']}: {count} decoded frames, {duration:.3f}s, {rate:g}fps", flush=True)
    return result


def main() -> None:
    """📋 Record decoding evidence without claiming completion of visual review."""
    curriculum = RUNTIME.load_manifest()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--video", action="append", choices=[video["id"] for video in curriculum])
    args = parser.parse_args()
    videos = [video for video in curriculum if not args.video or video["id"] in args.video]
    results = [verify(video) for video in videos]
    combined = {**PREVIOUS, **{row["id"]: row for row in results}}
    (REVIEW / "metadata.json").write_text(json.dumps(list(combined.values()), indent=2) + "\n", encoding="utf-8")
    status = "All five requested movies were produced" if len(results) == len(curriculum) else f"{len(results)} completed movie(s) verified; full curriculum verification is still pending. These completed movies were produced"
    runs = json.loads((OUTPUT / "render-invocations.json").read_text(encoding="utf-8"))
    workers = max(run["workers"] for run in runs)
    lines = ["# Full Curriculum Render", "", status + " through the canonical authoring runtime at 640 × 360, 6 fps, low quality, with the recorded capacity-managed worker limits. Every video frame was decoded with PyAV; all decoded frames retained the requested dimensions and nondecreasing timestamps.", "",
             "| Video | Duration (s) | Decoded Frames | FPS | Codec | Audio Streams |", "|---|---:|---:|---:|---|---:|"]
    lines += [f"| {row['id']} | {row['duration']:.3f} | {row['decoded_frames']} | {row['fps']:g} | {row['codec']} | {row['audio_streams']} |" for row in results]
    lines += ["", f"Execution phases: {len(runs)}; maximum configured source workers: {workers}. The initial run used two workers; later normal resumes reused completed sources and retried failed or stale source fingerprints.", "", "See `full-render-events.md` for initial source failures and intentional caption repairs.", "", "## Visual Review", "", "Eight deterministic representative frames per lesson were extracted into individual JPEGs and one contact sheet per movie. Visual review is pending; successful decoding does not prove that every source scene is free of layout, physics or animation issues.", "",
              "The current corpus contains no canonical narration audio assets, so silence is expected unless a source already contains an authored soundtrack. No speech generation was invoked.", "",
              "## Reproduction", "", "`repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --quality l --resolution 640,360 --fps 6 --jobs 2 --no-play --log-output --output-root <ticket>/🗑️generated/full`", "",
              "Managed Python environment: `.🧬semio/🎓️teaching/⚡️cache/python`. Source attempt logs and successful provenance are under the generated full-render directory. The retained `verify-full.py` input reproduces the decoding and contact sheets. The implementation inventory is in `files-reuse.md`.", "",
              "Generated metadata and contact sheets are under `🗑️generated/full-review`. Complete rendered movies are under `🗑️generated/full/rendered`.", ""]
    (TICKET / "full-render.md").write_text("\n".join(lines), encoding="utf-8")


if __name__ == "__main__":
    main()
