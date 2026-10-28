"""📦 Export distribution MP4s of the BIM and optimization render masters and record their provenance.

Usage: python deliver.py <en|de> [speaker|spectator]
Reads masters from .🧬semio/🎓️teaching/⚡️cache/videos/<language>/<audience>/rendered, writes delivery/<file> and delivery.json.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

import av
from imageio_ffmpeg import get_ffmpeg_exe

sys.stdout.reconfigure(encoding="utf-8")
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
language, audience = sys.argv[1], (sys.argv[2] if len(sys.argv) > 2 else "speaker")
BASE = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / audience
NAMES = {**{f"bim_{name}": f"BIM_{name}" for name in ("information", "objects", "representations", "models", "exchange", "consistency")},
         **{f"optimization_{name.lower()}": f"Optimization_{name}" for name in ("Extrema", "Sampling", "Experiments", "Index", "Pareto", "Algorithms")}}
VIDEO = {"codec": "H.264", "profile": "High", "profileIdc": 100, "levelIdc": 41, "pixelFormat": "yuv420p", "crf": 18,
         "preset": "fast", "threads": 4, "faststart": True}


def digest(path: Path) -> str:
    """🔏 Hash a file in chunks for provenance links."""
    sha = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            sha.update(chunk)
    return sha.hexdigest()


def decode(path: Path) -> tuple[int, float, int, int]:
    """🎞️ Decode every frame to prove the file is complete and measure it."""
    with av.open(str(path)) as container:
        stream = container.streams.video[0]
        frames = sum(1 for _ in container.decode(stream))
        return frames, float(stream.duration * stream.time_base), stream.width, stream.height


delivery = BASE / "delivery"
delivery.mkdir(parents=True, exist_ok=True)
records = []
for index, (video_id, name) in enumerate(NAMES.items(), 1):
    file = f"{name}_{audience}_{language}_1080p30.mp4"
    master, target = BASE / "rendered" / file, delivery / file
    certificate = master.with_name(master.stem + ".render.json")
    print(f"[DEBUG] {index}/{len(NAMES)} exporting {file}", flush=True)
    subprocess.run([get_ffmpeg_exe(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-i", str(master),
                    "-c:v", "libx264", "-profile:v", "high", "-level:v", "4.1", "-pix_fmt", "yuv420p", "-crf", "18",
                    "-preset", "fast", "-threads", "4", "-c:a", "copy", "-movflags", "+faststart", str(target)], check=True)
    frames, seconds, width, height = decode(target)
    master_frames = decode(master)[0]
    if (frames, width, height) != (master_frames, 1920, 1080):
        raise SystemExit(f"{file}: {frames}/{master_frames} frames at {width}x{height}")
    records.append({"id": video_id, "file": file, "sha256": digest(target), "bytes": target.stat().st_size,
                    "frames": frames, "durationSeconds": seconds, "master": f"../rendered/{file}",
                    "masterSha256": digest(master), "masterCertificate": f"../rendered/{certificate.name}",
                    "masterCertificateSha256": digest(certificate)})
(delivery / "delivery.json").write_text(json.dumps({
    "schema": "semio.architecture.delivery/v1", "language": language, "audience": audience,
    "width": 1920, "height": 1080, "fps": 30, "video": VIDEO,
    "audio": {"codec": "AAC LC", "sampleRate": 48000, "channels": 2, "copy": True, "spokenNarration": False},
    "videos": records}, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] delivered", len(records), "videos to", delivery, flush=True)
