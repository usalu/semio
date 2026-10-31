"""🖼️ Tile one mid-chapter frame of every video of the course preview into one contact sheet: contact_sheet.py <en|de>"""
import json
import subprocess
import sys
from pathlib import Path

import imageio_ffmpeg

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
TICKET = Path(__file__).resolve().parents[1]
language = sys.argv[1]
course = ROOT / ".🧬semio/🎓️teaching/⚡️cache/videos" / language / "speaker/preview" / f"Architecture_course_speaker_{language}_480p15.mp4"
record = json.loads(course.with_suffix(".json").read_text(encoding="utf-8"))
work = TICKET / "🗑️generated/contact" / language
work.mkdir(parents=True, exist_ok=True)
ffmpeg, start, frames = imageio_ffmpeg.get_ffmpeg_exe(), 0.0, []
for index, video in enumerate(record["videos"]):
    frame = work / f"{index:02d}.png"
    subprocess.run([ffmpeg, "-y", "-loglevel", "error", "-ss", str(start + video["seconds"] * 0.6), "-i", str(course), "-frames:v", "1",
                    "-vf", f"scale=427:240,drawtext=text='{index:02d} {video['id']}':x=4:y=4:fontsize=14:fontcolor=yellow:box=1:boxcolor=black", str(frame)], check=True)
    frames.append(frame)
    start += video["seconds"]
subprocess.run([ffmpeg, "-y", "-loglevel", "error", "-framerate", "1", "-i", str(work / "%02d.png"), "-vf", "tile=5x6", "-frames:v", "1", str(work / "sheet.png")], check=True)
print("[DEBUG] sheet", work / "sheet.png", len(frames), flush=True)
