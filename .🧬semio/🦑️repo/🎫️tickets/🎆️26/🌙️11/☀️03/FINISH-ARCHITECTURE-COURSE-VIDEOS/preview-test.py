"""🎬 Course preview audio, silence, chapter and order acceptance cases."""
import importlib.util
import json
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

import av
import imageio_ffmpeg
import numpy as np

HERE = Path(__file__).resolve().parent
SOURCE = HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media" / "course_preview.py"
spec = importlib.util.spec_from_file_location("course_preview", SOURCE)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
CASES = json.loads((HERE / "preview-cases.json").read_text(encoding="utf-8"))
FFMPEG = imageio_ffmpeg.get_ffmpeg_exe()


class CoursePreview(unittest.TestCase):
    """🧪 Verify actual encoded streams using PyAV and NumPy."""

    def test_master_change_does_not_publish(self):
        folder = HERE / "🗑️generated" / "preview-race"
        folder.mkdir(parents=True, exist_ok=True)
        master = folder / "master.mp4"
        master.write_bytes(b"original")
        record = {"fresh": True, "file": str(master), "sha256": module.lib.digest(master)}
        (folder / "media-manifest.json").write_text(json.dumps({"videos": {"one": {"master": record}}}), encoding="utf-8")
        preview = folder / "preview"
        preview.mkdir(exist_ok=True)
        output = preview / "Architecture_course_speaker_en_480p15.mp4"
        output.write_bytes(b"old")
        def create_partial(parts, path, work, titles):
            path.write_bytes(b"new")
            master.write_bytes(b"changed")
            return []
        runtime = type("Runtime", (), {"load_manifest": lambda self: [{"id": "one", "output": "one"}]})()
        with patch.object(module.lib, "base", return_value=folder), patch.object(module.lib, "runtime", return_value=runtime), patch.object(module.lib, "video_status", return_value={"master": master}), patch.object(module.lib, "master_fresh", return_value=True), patch.object(module.chapter_titles, "titles", return_value={"one": "One"}), patch.object(module, "join", side_effect=create_partial):
            with self.assertRaises(SystemExit):
                module.main("en", folder / "work")
        self.assertEqual(output.read_bytes(), b"old")
        self.assertEqual(list(preview.glob("*.partial.mp4")), [])

    def test_order_audio_and_chapters(self):
        work = HERE / "🗑️generated" / "preview-test"
        work.mkdir(parents=True, exist_ok=True)
        sources = []
        for lesson in CASES["lessons"]:
            path = work / (lesson["id"] + ".mp4")
            command = [FFMPEG, "-y", "-loglevel", "error", "-f", "lavfi", "-i", f"color={lesson['color']}:s=64x48:r=15:d={CASES['duration']}"]
            if lesson["tone"]:
                command += ["-f", "lavfi", "-i", f"sine=frequency={lesson['tone']}:sample_rate=44100:duration={CASES['duration']}", "-c:a", "aac"]
            command += ["-c:v", "libx264", "-pix_fmt", "yuv420p", str(path)]
            subprocess.run(command, check=True)
            sources.append((lesson["id"], {}, path))
        output = work / "joined.mp4"
        titles = {lesson["id"]: f"Lesson {index + 1}" for index, lesson in enumerate(CASES["lessons"])}
        module.join(sources, output, work / "work", titles)
        with av.open(str(output)) as container:
            self.assertEqual(len(container.streams.audio), 1)
            self.assertEqual(container.streams.audio[0].layout.name, "stereo")
            self.assertAlmostEqual(container.duration / av.time_base, len(sources) * CASES["duration"], delta=0.15)
            chapters = container.chapters()
            self.assertEqual([chapter["metadata"]["title"] for chapter in chapters], [titles[lesson["id"]] for lesson in CASES["lessons"]])
            for index, chapter in enumerate(chapters):
                self.assertAlmostEqual(float(chapter["start"] * chapter["time_base"]), index * CASES["duration"], delta=0.1)
        with av.open(str(output)) as container:
            samples = np.concatenate([frame.to_ndarray().reshape(-1) for frame in container.decode(audio=0)])
            rate = container.streams.audio[0].rate
            for index, lesson in enumerate(CASES["lessons"]):
                section = samples[int((index + 0.2) * rate * 2):int((index + 0.8) * rate * 2)]
                rms = np.sqrt(np.mean(section.astype(float) ** 2))
                self.assertGreater(rms, 0.001) if lesson["tone"] else self.assertLess(rms, 0.001)
        with av.open(str(output)) as container:
            frames = list(container.decode(video=0))
            for index, lesson in enumerate(CASES["lessons"]):
                pixel = frames[index * 15 + 7].to_ndarray(format="rgb24").mean(axis=(0, 1))
                self.assertEqual(int(np.argmax(pixel)), index)


if __name__ == "__main__":
    unittest.main()
