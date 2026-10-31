"""📼 Fallback preview from the older 1080p30 delivery: scale to 854x480 at 15 fps and write the json record.
Usage: fallback_delivery.py <Output> [<Output>...]"""
import hashlib, json, subprocess, sys, time
from pathlib import Path
import imageio_ffmpeg
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
base = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos/en/speaker"
for output in sys.argv[1:]:
    source = base / "delivery" / f"{output}_speaker_en_1080p30.mp4"
    target = base / "preview" / f"{output}_speaker_en_480p15.mp4"
    target.parent.mkdir(parents=True, exist_ok=True)
    started = time.time()
    subprocess.run([imageio_ffmpeg.get_ffmpeg_exe(), "-y", "-loglevel", "error", "-i", str(source), "-vf", "scale=854:480,fps=15",
                    "-c:v", "libx264", "-preset", "veryfast", "-crf", "26", "-pix_fmt", "yuv420p", "-c:a", "aac", "-b:a", "96k", "-movflags", "+faststart", str(target)], check=True)
    record = {"scene": output, "language": "en", "audience": "speaker", "size": [854, 480], "fps": 15, "render_seconds": round(time.time() - started),
              "rendered_at": time.strftime("%Y-%m-%dT%H:%M:%S"), "sha256": hashlib.sha256(target.read_bytes()).hexdigest(),
              "source": str(source.relative_to(ROOT)), "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
              "note": "fallback: older delivery; current BIM drafting code needs about 100 minutes of exact geometry before the first frame"}
    target.with_suffix(".json").write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print("[DEBUG] fallback", target.name, flush=True)
