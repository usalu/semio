"""🎬 Join the fresh 480p15 lesson masters of the architecture course, in manifest order, into one chaptered preview with poster and thumbnail.

Usage: python course_preview.py <en|de> [--allow-missing]
Input: `media-manifest.json` of `manifest.py`; a lesson whose master is stale or missing aborts the join (no old artwork is served),
unless `--allow-missing` skips it and records it under `missing`.
Output: .🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/preview/Architecture_course_speaker_<lang>_480p15.mp4 with `.json` (media key,
per-lesson media keys and staffage cast keys), `.poster.png` and `.thumb.png`.
"""
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import lib

import imageio_ffmpeg

language = sys.argv[1]
allow_missing = "--allow-missing" in sys.argv
ffmpeg = imageio_ffmpeg.get_ffmpeg_exe()
folder = lib.base(language, "speaker")
document = json.loads((folder / "media-manifest.json").read_text(encoding="utf-8"))
runtime = lib.runtime()
order = [video["id"] for video in runtime.load_manifest()]
parts, missing = [], []
for video_id in order:
    record = document["videos"].get(video_id, {}).get("master", {})
    (parts if record.get("fresh") else missing).append((video_id, record))
if missing and not allow_missing:
    raise SystemExit("stale or missing lesson masters: " + ", ".join(name for name, _ in missing))
work = Path(tempfile.mkdtemp(prefix="course-", dir=lib.CACHE / "media-logs" if (lib.CACHE / "media-logs").is_dir() else None))
normalised = []
for index, (name, record) in enumerate(parts):
    source = lib.ROOT / record["file"]
    target = work / f"{index:02d}-{name}.mp4"
    subprocess.run([ffmpeg, "-y", "-hide_banner", "-loglevel", "error", "-i", str(source), "-an",
                    "-vf", "scale=854:480:force_original_aspect_ratio=decrease,pad=854:480:(ow-iw)/2:(oh-ih)/2,fps=15,format=yuv420p",
                    "-c:v", "libx264", "-preset", "veryfast", "-crf", "23", "-g", "30", str(target)], check=True)
    normalised.append((name, record, target, lib.duration(target)))
    print("[DEBUG] normalised", name, round(normalised[-1][3], 1), "s", flush=True)
listing = work / "list.txt"
listing.write_text("".join(f"file '{target.as_posix()}'\n" for _, _, target, _ in normalised), encoding="utf-8")
chapters, start = [";FFMETADATA1\n"], 0.0
for name, _, _, seconds in normalised:
    chapters.append(f"[CHAPTER]\nTIMEBASE=1/1000\nSTART={round(start * 1000)}\nEND={round((start + seconds) * 1000)}\ntitle={name}\n")
    start += seconds
metadata = work / "chapters.txt"
metadata.write_text("".join(chapters), encoding="utf-8")
preview = folder / "preview"
preview.mkdir(parents=True, exist_ok=True)
output = preview / f"Architecture_course_speaker_{language}_480p15.mp4"
subprocess.run([ffmpeg, "-y", "-hide_banner", "-loglevel", "error", "-f", "concat", "-safe", "0", "-i", str(listing), "-i", str(metadata),
                "-map_metadata", "1", "-map_chapters", "1", "-c", "copy", "-movflags", "+faststart", str(output)], check=True)
made = lib.stills(output, preview, output.stem)
lessons = [{"id": name, "master": record["file"], "media": record["media"], "sha256": record["sha256"], "seconds": round(seconds, 1),
            "casts": record["provenance"]["casts"]} for name, record, _, seconds in normalised]
key = hashlib.sha256(lib.canonical({"sha256": lib.digest(output), "lessons": [lesson["media"] for lesson in lessons]}).encode()).hexdigest()
result = {"language": language, "audience": "speaker", "size": [854, 480], "fps": 15, "seconds": round(lib.duration(output), 1),
          "sha256": lib.digest(output), "media": key, "missing": [name for name, _ in missing], "lessons": lessons,
          "stills": {"poster": made["poster"].name, "thumbnail": made["thumbnail"].name}}
output.with_suffix(".json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
shutil.rmtree(work, ignore_errors=True)
print("[DEBUG] course", output, result["seconds"], "s", len(lessons), "lessons", "missing:", result["missing"], flush=True)
