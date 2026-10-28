"""🎞️ Extract middle-frame posters from finalized physical-video segments without Manim."""

from pathlib import Path
import json
import sys
import time

import av
from PIL import Image, ImageDraw

TICKET = Path(__file__).resolve().parent
MEDIA = TICKET / "🗑️generated/o/s"
OUTPUT = TICKET / "🗑️generated/qa-review"
OUTPUT.mkdir(parents=True, exist_ok=True)
records = []
for number in range(1, 10):
    videos = sorted((MEDIA / str(number) / "v").glob("*.mp4"))
    videos = [video for video in videos if "partial" not in video.name.lower()]
    if len(videos) != 1:
        records.append({"segment": number, "status": "not-ready", "files": len(videos)})
        continue
    video = videos[0]
    initial = video.stat()
    if initial.st_size == 0 or time.time() - initial.st_mtime < 3:
        records.append({"segment": number, "status": "not-ready", "file": str(video)})
        continue
    try:
        with av.open(str(video)) as container:
            stream = container.streams.video[0]
            duration = float(stream.duration * stream.time_base) if stream.duration is not None else float(container.duration / av.time_base)
            target = duration / 2
            container.seek(int(target / float(stream.time_base)), stream=stream, backward=True)
            chosen = None
            for frame in container.decode(stream):
                chosen = frame
                if frame.time is not None and frame.time >= target:
                    break
            if chosen is None:
                raise ValueError("No decoded middle frame")
            image = chosen.to_image().convert("RGB")
            poster = OUTPUT / f"segment-{number}-middle.png"
            final = video.stat()
            if (initial.st_size, initial.st_mtime_ns) != (final.st_size, final.st_mtime_ns):
                records.append({"segment": number, "status": "not-ready", "reason": "file changed during read"})
                continue
            image.save(poster)
            record = {"segment": number, "status": "sampled", "file": str(video), "bytes": final.st_size,
                      "duration_seconds": duration, "frame_seconds": chosen.time, "size": list(image.size), "poster": str(poster)}
            records.append(record)
            print("[DEBUG] " + json.dumps(record, ensure_ascii=False), flush=True)
    except Exception as error:
        records.append({"segment": number, "status": "not-ready", "error": str(error), "file": str(video)})
sampled = [record for record in records if record["status"] == "sampled"]
if sampled:
    sheet = Image.new("RGB", (1920, 390 * ((len(sampled) + 2) // 3)), "#001117")
    draw = ImageDraw.Draw(sheet)
    for index, record in enumerate(sampled):
        image = Image.open(record["poster"])
        x, y = index % 3 * 640, index // 3 * 390
        draw.text((x + 8, y + 8), f"Segment {record['segment']} - {record['frame_seconds']:.1f}s / {record['duration_seconds']:.1f}s", fill="#f7f3e3")
        sheet.paste(image, (x, y + 30))
    sheet.save(OUTPUT / "contact.png")
(OUTPUT / "metadata.json").write_text(json.dumps(records, indent=2, ensure_ascii=False), encoding="utf-8")
report = "# Physical Video Render Visual Review\n\n"
report += "Sampled only finalized top-level segment MP4s, excluding partial output files. Files were at least three seconds old and unchanged before/after decoding. The lightweight sampling process used AV and Pillow in the shared native environment, without Manim or UV. All posters and metadata are generated review artifacts under this ticket. No video was modified.\n\n"
report += "| Segment | Status | Duration (s) | Sample (s) | Source |\n| --- | --- | ---: | ---: | --- |\n"
for record in records:
    report += f"| {record['segment']} | {record['status']} | {record.get('duration_seconds', 0):.2f} | {record.get('frame_seconds', 0):.2f} | {Path(record.get('file', '')).name} |\n"
report += "\nVisual review notes follow after inspecting the extracted posters.\n"
(TICKET / "render-visual-review.md").write_text(report, encoding="utf-8")
print("[DEBUG] Sampled segments: " + str(len(sampled)), flush=True)
