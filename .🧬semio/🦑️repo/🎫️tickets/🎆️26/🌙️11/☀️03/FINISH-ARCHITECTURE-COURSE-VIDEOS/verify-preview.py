"""🎬 Validate the complete course preview and retain per-lesson review frames."""
import json
import subprocess
import sys
from pathlib import Path

import av
import imageio_ffmpeg
from PIL import Image, ImageDraw

HERE = Path(__file__).resolve().parent
MEDIA = HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"
sys.path.insert(0, str(MEDIA))
import lib
import chapter_titles

contract = json.loads((HERE / "course-preview-contract.json").read_text(encoding="utf-8"))
language = sys.argv[1]
assert language in contract["languages"]
folder = lib.base(language, "speaker") / "preview"
video = folder / f"Architecture_course_speaker_{language}_480p15.mp4"
record = json.loads(video.with_suffix(".json").read_text(encoding="utf-8"))
order = [lesson["id"] for lesson in lib.runtime().load_manifest()]
titles = chapter_titles.titles(lib.runtime().load_manifest(), language)
assert order == contract["lessons"]
assert [lesson["id"] for lesson in record["lessons"]] == order
assert [lesson["title"] for lesson in record["lessons"]] == [titles[name] for name in order]
assert not record.get("missing")
assert record["sha256"] == lib.digest(video)
out = HERE / "🗑️generated" / "final-review"
out.mkdir(parents=True, exist_ok=True)
with av.open(str(video)) as container:
    stream = container.streams.video[0]
    audio = container.streams.audio[0]
    assert (stream.width, stream.height) == (854, 480)
    assert float(stream.average_rate) == 15
    assert audio.layout.name == "stereo" and audio.rate == 48000
    chapters = container.chapters()
    assert [chapter["metadata"]["title"] for chapter in chapters] == [titles[name] for name in order]
    duration = float(stream.duration * stream.time_base)
    expected = sum(lesson["seconds"] for lesson in record["lessons"])
    assert abs(duration - expected) <= len(order) * .06 + .2
    assert abs(float(container.duration) / av.time_base - duration) < 1
    for index, (lesson, chapter) in enumerate(zip(record["lessons"], chapters, strict=True)):
        start = float(chapter["start"] * chapter["time_base"])
        end = float(chapter["end"] * chapter["time_base"])
        assert abs(end - start - lesson["seconds"]) < .1
        sheet = Image.new("RGB", (1281, 270), "white")
        ImageDraw.Draw(sheet).text((8, 8), f"{index + 1:02d} {lesson['id']}", fill="black")
        for column, fraction in enumerate((.25, .5, .75)):
            at = start + (end - start) * fraction
            container.seek(int(at / stream.time_base), stream=stream, backward=True)
            picked = None
            for frame in container.decode(stream):
                picked = frame
                if frame.time is not None and frame.time >= at:
                    break
            assert picked is not None
            sheet.paste(picked.to_image().resize((427, 240), Image.Resampling.LANCZOS), (427 * column, 30))
        sheet.save(out / f"{index + 1:02d}-{lesson['id']}.png")
        print("[DEBUG] reviewed", lesson["id"], round(end - start, 1), "s", flush=True)
command = [imageio_ffmpeg.get_ffmpeg_exe(), "-hide_banner", "-v", "error", "-xerror", "-i", str(video), "-progress", "pipe:1", "-stats_period", "10", "-f", "null", "-"]
with subprocess.Popen(command, stdout=subprocess.PIPE, text=True) as child:
    try:
        for line in child.stdout:
            if line.startswith("out_time="):
                print("[DEBUG] decoded", line.strip().split("=", 1)[1], flush=True)
        if child.wait():
            raise subprocess.CalledProcessError(child.returncode, command)
    except KeyboardInterrupt:
        child.terminate()
        raise

report = {"video": str(video), "lessons": order, "seconds": duration, "sha256": record["sha256"], "audio": "48000 Hz stereo", "videoFormat": "854x480 15fps", "decode": "complete"}
(out / "verification.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
(HERE / "preview-verification.md").write_text("# Complete Course Preview Verification\n\n" + json.dumps(report, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] complete preview verified", duration, "seconds", len(order), "lessons", flush=True)
