"""🎬 Join the 480p15 previews of every architecture course video, in manifest order, into one chaptered preview.

Usage: python course_preview.py <en|de> [--allow-missing]
Input: .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/preview/<Output>_speaker_<lang>_480p15.mp4 per manifest video.
Output: .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/preview/Architecture_course_speaker_<lang>_480p15.mp4 with one chapter per video,
plus a .json record of the sources.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

import imageio_ffmpeg

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
TICKET = Path(__file__).resolve().parents[1]
sys.stdout.reconfigure(encoding="utf-8", errors="backslashreplace")
language = sys.argv[1]
allow_missing = "--allow-missing" in sys.argv
ffmpeg = imageio_ffmpeg.get_ffmpeg_exe()
folder = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / "speaker/preview"
manifest = json.loads((ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json").read_text(encoding="utf-8"))
work = TICKET / "🗑️generated/course" / language
work.mkdir(parents=True, exist_ok=True)


def duration(path: Path) -> float:
    """⏱️ Container duration in seconds as reported by ffmpeg."""
    probe = subprocess.run([ffmpeg, "-hide_banner", "-i", str(path)], capture_output=True, text=True, encoding="utf-8", errors="replace").stderr
    hours, minutes, seconds = probe.split("Duration: ")[1].split(",")[0].split(":")
    return int(hours) * 3600 + int(minutes) * 60 + float(seconds)


parts, missing = [], []
for video in manifest["videos"]:
    source = folder / f"{video.get('output', video['id'])}_speaker_{language}_480p15.mp4"
    (parts if source.is_file() else missing).append((video["id"], source))
if missing and not allow_missing:
    raise SystemExit("missing previews: " + ", ".join(name for name, _ in missing))
normalised = []
for index, (name, source) in enumerate(parts):
    target = work / f"{index:02d}-{name}.mp4"
    if not target.is_file() or target.stat().st_mtime < source.stat().st_mtime:
        subprocess.run([ffmpeg, "-y", "-hide_banner", "-loglevel", "error", "-i", str(source), "-an",
                        "-vf", "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2,fps=15,format=yuv420p",
                        "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-g", "30", str(target)], check=True)
    normalised.append((name, source, target, duration(target)))
    print("[DEBUG] normalised", name, round(normalised[-1][3], 1), "s", flush=True)
listing = work / "list.txt"
listing.write_text("".join(f"file '{target.as_posix()}'\n" for _, _, target, _ in normalised), encoding="utf-8")
chapters, start = [";FFMETADATA1\n"], 0.0
for name, _, _, seconds in normalised:
    chapters.append(f"[CHAPTER]\nTIMEBASE=1/1000\nSTART={round(start * 1000)}\nEND={round((start + seconds) * 1000)}\ntitle={name}\n")
    start += seconds
metadata = work / "chapters.txt"
metadata.write_text("".join(chapters), encoding="utf-8")
output = folder / f"Architecture_course_speaker_{language}_480p15.mp4"
subprocess.run([ffmpeg, "-y", "-hide_banner", "-loglevel", "error", "-f", "concat", "-safe", "0", "-i", str(listing), "-i", str(metadata),
                "-map_metadata", "1", "-map_chapters", "1", "-c", "copy", "-movflags", "+faststart", str(output)], check=True)
record = {"language": language, "audience": "speaker", "size": [854, 480], "fps": 15, "seconds": round(duration(output), 1),
          "sha256": hashlib.sha256(output.read_bytes()).hexdigest(), "missing": [name for name, _ in missing],
          "videos": [{"id": name, "source": str(source.relative_to(ROOT)), "seconds": round(seconds, 1),
                      "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest()} for name, source, _, seconds in normalised]}
output.with_suffix(".json").write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print("[DEBUG] course", output, record["seconds"], "s", len(normalised), "videos", "missing:", record["missing"], flush=True)
