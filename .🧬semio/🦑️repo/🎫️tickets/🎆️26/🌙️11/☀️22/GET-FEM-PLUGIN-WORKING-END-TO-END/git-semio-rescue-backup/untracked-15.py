"""🖼️ Verify that every overview card shows a real, declared poster frame and that no placeholder pictogram remains."""

from __future__ import annotations

import importlib
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
PRESENTATION = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation"
MANIFEST = ROOT / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
SETTINGS = {"language": "de", "profile": "preview", "resolution": "854,480", "fps": 15, "audience": "speaker"}


def module(name: str):
    """📦 Import one clip package module by its dotted path."""
    return importlib.import_module(PACKAGE + "." + name)


class PosterDeclarationTest(unittest.TestCase):
    def setUp(self):
        self.posters = module("🪟️presentation.🖼️posters.🐍️")

    def test_declaration_cases(self):
        for case in CASES["declarations"]:
            with self.subTest(case=case["id"]):
                if "error" in case:
                    with self.assertRaisesRegex(ValueError, case["error"]):
                        self.posters.declaration(case["chapter"])
                else:
                    self.assertEqual(self.posters.declaration(case["chapter"]), case["expected"])

    def test_designated_chapter_cases(self):
        for case in CASES["designated"]:
            with self.subTest(case=case["id"]):
                if "error" in case:
                    with self.assertRaisesRegex(ValueError, case["error"]):
                        self.posters.designated(case["video"])
                else:
                    self.assertEqual(self.posters.designated(case["video"]), case["expected"])

    def test_manifest_accepts_only_declared_poster_shapes(self):
        runtime = module("⚙️runtime.🐍️")
        script = "🏛️architecture/a/🎬️clip/🐍️.py"
        for case in CASES["manifest"]:
            with self.subTest(case=case["id"]), tempfile.TemporaryDirectory() as directory:
                root = Path(directory).resolve()
                video = {"id": "v", "script": script, "output": "Out", "chapters": [{"scene": "A", "segments": [{"script": script, "scene": "One"}]}]}
                video.update(case.get("video", {}))
                video["chapters"][0].update(case.get("chapter", {}))
                (root / "🏛️architecture/⚡️energy/🎬️clip").mkdir(parents=True)
                (root / "🏛️architecture/⚡️energy/🎬️clip/🔣️.json").write_text(json.dumps({"videos": [video]}), encoding="utf-8")
                with patch.object(runtime, "teaching_root", return_value=root):
                    if "error" in case:
                        with self.assertRaisesRegex(ValueError, case["error"]):
                            runtime.load_manifest(source_ids=[])
                    else:
                        self.assertEqual(runtime.load_manifest(source_ids=[])[0]["id"], "v")

    def test_every_card_of_the_contract_references_a_declared_poster(self):
        hierarchy = module("🪟️presentation.🥞️hierarchy.🐍️")
        contract = hierarchy.hierarchy_contract()
        manifest = {video["id"]: video for video in json.loads(MANIFEST.read_text(encoding="utf-8"))["videos"]}
        scenes = {chapter["scene"] for video in manifest.values() for chapter in video["chapters"]}
        course = contract["courses"][0]
        self.assertEqual(course["poster"], CASES["references"]["course"])
        for topic in course["topics"]:
            with self.subTest(topic=topic["id"]):
                self.assertEqual(topic["poster"], CASES["references"]["topics"][topic["id"]])
                for video in topic["videos"]:
                    chapters = [chapter["scene"] for chapter in manifest[video["id"]]["chapters"]]
                    self.assertEqual(video["poster"], manifest[video["id"]].get("poster", {}).get("chapter", chapters[0]))
                    self.assertEqual([chapter["poster"] for chapter in video["chapters"]], chapters)
                    self.assertIn(video["poster"], scenes)
        for video, scene in CASES["references"]["lessons"].items():
            self.assertEqual(hierarchy.video(video)["poster"], scene)
        self.assertNotIn('"symbol"', json.dumps(contract))

    def test_required_posters_are_exactly_the_cards_of_the_branch_grids(self):
        hierarchy = module("🪟️presentation.🥞️hierarchy.🐍️")
        for case in CASES["required"]:
            with self.subTest(branch=case["branch"][-1]):
                self.assertEqual(hierarchy.path(case["branch"][-1]), case["branch"])
                self.assertEqual(self.posters.required(case["branch"]), case["scenes"])
        with self.assertRaises(ValueError):
            self.posters.required(["architecture", "missing"])

    def test_no_placeholder_pictogram_code_or_symbols_remain(self):
        browser = (PRESENTATION / "🌐️browser/🟦️.tsx").read_text(encoding="utf-8")
        self.assertNotIn("Thumbnail", browser)
        self.assertNotIn("symbol", browser)
        self.assertNotIn("symbol", (PRESENTATION / "🔣️.json").read_text(encoding="utf-8"))
        self.assertNotIn("symbol", (PRESENTATION / "🧬️schema.json").read_text(encoding="utf-8"))
        self.assertIn("Missing poster image", browser)


class PosterAssetTest(unittest.TestCase):
    def setUp(self):
        self.posters = module("🪟️presentation.🖼️posters.🐍️")
        self.runtime = module("⚙️runtime.🐍️")
        self.scene = "Ventilation_01_WarumLueften"
        self.directory = Path(self.enterContext(tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])))
        self.enterContext(patch.object(self.posters, "asset", return_value=(self.directory / "poster.png", self.directory / "poster.json")))
        self.source = "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
        self.digest = self.posters._digest(ROOT / self.source)

    def frame(self, colour):
        from PIL import Image, ImageDraw

        path = self.directory / "frame.png"
        image = Image.new("RGB", (854, 480), (0, 10, 15))
        ImageDraw.Draw(image).rectangle((200, 120, 650, 360), fill=colour)
        image.save(path)
        return path

    def test_coverage_guard_cases(self):
        from PIL import Image, ImageDraw

        spec = CASES["coverage"]
        for case in spec["cases"]:
            with self.subTest(case=case["id"]):
                image = Image.new("RGB", tuple(CASES["size"]), tuple(spec["background"]))
                draw = ImageDraw.Draw(image)
                for left, top, right, bottom in case["boxes"]:
                    draw.rectangle((left * 640, top * 360, right * 640, bottom * 360), fill=(230, 200, 40))
                self.assertEqual(self.posters.coverage(image) >= spec["minimum"], case["accept"])
        self.assertEqual(self.posters.MINIMUM_COVERAGE, spec["minimum"])

    def test_an_almost_empty_frame_is_refused_naming_its_chapter_and_nothing_is_stored(self):
        from PIL import Image

        empty = self.directory / "empty.png"
        Image.new("RGB", (854, 480), (0, 10, 15)).save(empty)
        with self.assertRaisesRegex(ValueError, self.scene + ".*almost empty"):
            self.posters.store(self.scene, empty, {}, sources_state={self.source: self.digest})
        self.assertFalse((self.directory / "poster.png").exists())
        self.assertFalse((self.directory / "poster.json").exists())

    def test_every_stored_poster_has_content_and_none_is_the_roadmap(self):
        from PIL import Image

        for scene, info in self.posters.chapters().items():
            with self.subTest(scene=scene):
                self.assertNotIn("Fahrplan", info["segment"]["scene"])
                with Image.open(info["folder"] / f"{scene}.png") as picture:
                    self.assertGreaterEqual(self.posters.coverage(picture), self.posters.MINIMUM_COVERAGE)

    def test_a_roadmap_poster_segment_is_refused(self):
        runtime = self.runtime
        video = {"id": "v", "script": "🏛️architecture/a/🎬️clip/🐍️.py", "output": "O", "chapters": [{"scene": "A", "segments": [{"script": "s", "scene": "Beat1_Fahrplan"}]}]}
        with patch.object(runtime, "load_manifest", return_value=[video]), patch.object(runtime, "teaching_root", return_value=ROOT / "🎓️teaching"):
            with self.assertRaisesRegex(ValueError, "roadmap"):
                self.posters.chapters()

    def test_assets_live_beside_the_lesson_clip_sources_under_a_short_path(self):
        for scene, info in self.posters.chapters().items():
            with self.subTest(scene=scene):
                folder = info["folder"]
                self.assertEqual(folder.name, "🖼️posters")
                self.assertEqual(folder.parent, (ROOT / "🎓️teaching" / info["video"]["script"]).parent)
                self.assertLess(len(str(folder / f"{scene}.json")), 256)

    def test_a_stored_poster_is_card_sized_fingerprinted_and_stale_only_when_a_source_changes(self):
        from PIL import Image

        settings = {"language": "de", "profile": "preview", "audience": "speaker"}
        self.posters.store(self.scene, self.frame((200, 30, 30)), settings, sources_state={self.source: self.digest})
        with Image.open(self.directory / "poster.png") as image:
            self.assertEqual(image.size, tuple(CASES["size"]))
        record = json.loads((self.directory / "poster.json").read_text(encoding="utf-8"))
        self.assertEqual(record["captured"], settings)
        self.assertEqual(record["moment"], {"segment": 2, "animation": None})
        self.assertFalse(self.posters.stale(self.scene))
        self.posters.store(self.scene, self.frame((200, 30, 30)), settings, sources_state={self.source: "0" * 64})
        self.assertTrue(self.posters.stale(self.scene))
        self.assertEqual(self.posters.resolve([self.scene])[self.scene]["sha256"], self.posters._digest(self.directory / "poster.png"))

    def test_the_overview_reads_only_stored_assets_and_fails_loudly_on_a_missing_or_damaged_one(self):
        with self.assertRaisesRegex(FileNotFoundError, self.scene):
            self.posters.resolve([self.scene])
        self.posters.store(self.scene, self.frame((230, 200, 40)), {}, sources_state={self.source: self.digest})
        (self.directory / "poster.png").write_bytes(b"damaged")
        with self.assertRaisesRegex(FileNotFoundError, self.scene):
            self.posters.resolve([self.scene])

    def test_stale_posters_are_listed_in_one_warning_line_and_never_rerendered(self):
        import contextlib
        import io

        self.posters.store(self.scene, self.frame((230, 200, 40)), {}, sources_state={self.source: "1" * 64})
        output = io.StringIO()
        with contextlib.redirect_stdout(output), patch.object(self.posters, "render_frame", side_effect=AssertionError("must not render")):
            self.posters.resolve([self.scene])
        self.assertEqual(output.getvalue().count("\n"), 1)
        self.assertIn("stale stored posters", output.getvalue())
        self.assertIn(self.scene, output.getvalue())

    def test_the_last_frame_of_a_rendered_clip_becomes_the_poster_without_a_scene_render(self):
        import numpy as np
        from PIL import Image

        clip = self.directory / "clip.mp4"
        self.runtime.run_command([*self.runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-y", "-f", "lavfi", "-i", "color=c=0x3264c8:s=320x180:r=4:d=1.5,drawbox=x=80:y=45:w=160:h=90:color=white:t=fill", "-pix_fmt", "yuv420p", str(clip)])
        with patch.object(self.posters, "render_frame", side_effect=AssertionError("must not render")):
            self.posters.capture_clip(self.scene, clip, SETTINGS)
        with Image.open(self.directory / "poster.png") as image:
            pixels = np.asarray(image.convert("RGB"), dtype=float)
        self.assertEqual(pixels.shape, (CASES["size"][1], CASES["size"][0], 3))
        self.assertLess(np.abs(pixels[10, 10] - np.array([0x32, 0x64, 0xC8])).max(), 8)
        self.assertGreater(pixels[180, 320].min(), 240)

    def test_rendering_a_chapter_refreshes_only_stale_posters_from_the_declared_segment_clip(self):
        clips = [str(self.directory / name) for name in ("first.mp4", "second.mp4", "third.mp4")]
        for name in clips:
            Path(name).write_bytes(b"clip")
        with patch.object(self.posters, "capture_clip") as capture, patch.object(self.posters, "resolve", return_value={"x": 1}) as read, patch.object(self.posters, "stale", return_value=True):
            self.assertEqual(self.posters.prepare([(self.scene, clips)], {"language": "de"}), {"x": 1})
        capture.assert_called_once_with(self.scene, clips[2], {"language": "de"})
        self.assertEqual(list(read.call_args.args[0]), self.posters.required(module("🪟️presentation.🥞️hierarchy.🐍️").path(self.scene)))
        with patch.object(self.posters, "capture_clip") as capture, patch.object(self.posters, "resolve", return_value={}), patch.object(self.posters, "stale", return_value=False):
            self.posters.prepare([(self.scene, clips)], {"language": "de"})
        capture.assert_not_called()

    def test_capture_reports_current_posters_without_any_render(self):
        with patch.object(self.posters, "stale", return_value=False), patch.object(self.posters, "render_frame", side_effect=AssertionError("must not render")):
            self.assertEqual(self.posters.capture([self.scene, self.scene], {"language": "de", "profile": "preview", "audience": "speaker"}, self.directory), {self.scene: "current"})
        with self.assertRaises(ValueError):
            self.posters.capture(["Missing"], {}, self.directory)


class PosterBrowserTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.presentation = module("🪟️presentation.🐍️")
        cls.hierarchy = module("🪟️presentation.🥞️hierarchy.🐍️")
        cls.renderer = module("🪟️presentation.🥞️hierarchy.🖌️render.🐍️")
        cls.synthetic = module("🪟️presentation.🖼️posters.🧪️tests.🧰️synthetic.🐍️")
        cls.output = Path(tempfile.mkdtemp(prefix="posters-", dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])).resolve()

    @classmethod
    def tearDownClass(cls):
        import shutil

        shutil.rmtree(cls.output, ignore_errors=True)

    def nearest(self, image, point, expected: dict) -> str:
        import numpy as np

        sample = np.asarray(image.convert("RGB"))[int(point[1]) - 1:int(point[1]) + 2, int(point[0]) - 1:int(point[0]) + 2].reshape(-1, 3).mean(axis=0)
        return min(expected, key=lambda scene: float(np.linalg.norm(sample - np.array(self.synthetic.colour(scene)))))

    def test_lesson_cards_show_their_designated_chapter_poster_at_sixteen_by_nine(self):
        from PIL import Image

        scene = "Ventilation_01_WarumLueften"
        with self.synthetic.captured():
            result = self.presentation.render_interaction_frame(scene, "focus", 0, self.output / "lessons.png", SETTINGS)
        shown = {item["owner"]: item for item in result["browser"]["posters"] if item["owner"] in CASES["references"]["lessons"]}
        self.assertEqual(set(shown), set(CASES["references"]["lessons"]))
        scale = SETTINGS["resolution"].split(",")
        factor = int(scale[0]) / 1600
        with Image.open(result["path"]) as image:
            self.assertEqual(image.size, (int(scale[0]), int(scale[1])))
            for lesson, expected in CASES["references"]["lessons"].items():
                with self.subTest(lesson=lesson):
                    item = shown[lesson]
                    self.assertEqual(item["reference"], expected)
                    self.assertTrue(item["decoded"])
                    self.assertEqual(item["natural"], list(self.synthetic.SIZE))
                    self.assertAlmostEqual(item["rect"]["width"] / item["rect"]["height"], 16 / 9, delta=0.04)
                    center = ((item["rect"]["x"] + item["rect"]["width"] / 2) * factor, (item["rect"]["y"] + item["rect"]["height"] / 2) * factor)
                    self.assertEqual(self.nearest(image, center, {scene: None for scene in CASES["references"]["lessons"].values()}), expected)

    def test_chapter_cards_show_their_own_poster(self):
        scene = "Ventilation_01_WarumLueften"
        with self.synthetic.captured():
            result = self.presentation.render_interaction_frame(scene, "focus", 0.4, self.output / "chapters.png", SETTINGS)
        owners = {item["owner"]: item["reference"] for item in result["browser"]["posters"]}
        for chapter in self.hierarchy.video("ventilation")["chapters"]:
            self.assertEqual(owners[chapter["id"]], chapter["id"])

    def test_a_topic_without_lessons_shows_an_empty_card_and_no_pictogram(self):
        state = {**self.renderer.initial_hierarchy(), "opened_path": ["architecture"]}
        with self.synthetic.captured():
            result = self.presentation.render_interaction_frame("Ventilation_01_WarumLueften", "focus", 0, self.output / "topics.png", SETTINGS, hierarchy=state)
        self.assertEqual(result["browser"]["empty_cards"], ["parametric"])
        owners = {item["owner"] for item in result["browser"]["posters"]}
        self.assertTrue({"energy", "bim", "grasshopper", "excel", "optimization"} <= owners)
        self.assertNotIn("parametric", owners)

    def test_a_missing_poster_fails_loudly_instead_of_drawing_a_placeholder(self):
        with self.hierarchy.capture_catalog(self.hierarchy.hierarchy_contract()):
            with self.assertRaises((RuntimeError, subprocess.CalledProcessError)):
                self.presentation.render_interaction_frame("Ventilation_01_WarumLueften", "focus", 0, self.output / "missing" / "frame.png", SETTINGS)
        logs = "\n".join(path.read_text(encoding="utf-8", errors="replace") for path in (self.output / "missing").rglob("run.log"))
        self.assertIn("Missing poster image for", logs)


@unittest.skipUnless(os.environ.get("SEMIO_TEST_MANIM") == "1", "Enable real segment rendering with SEMIO_TEST_MANIM=1")
class PosterRenderTest(unittest.TestCase):
    def test_the_single_frame_poster_equals_the_last_frame_of_the_real_lesson_render(self):
        import numpy as np
        from PIL import Image

        posters, runtime = module("🪟️presentation.🖼️posters.🐍️"), module("⚙️runtime.🐍️")
        scene = "Ventilation_01_WarumLueften"
        settings = {**SETTINGS, "resolution": "320,180", "fps": 4}

        with tempfile.TemporaryDirectory(dir=os.environ["SEMIO_TEACHING_OUTPUT_ROOT"]) as name:
            root = Path(name).resolve()
            with patch.object(posters, "asset", return_value=(root / "poster.png", root / "poster.json")):
                made = posters.render_frame(scene, settings, root / "frame", runtime.ToolSession())
                self.assertFalse(posters.stale(scene))
            clip = runtime.render_segment(posters.chapters()[scene]["segment"], root / "s", settings, output_root=root)
            last = root / "last.png"
            runtime.run_command([*runtime.ffmpeg_command(), "-hide_banner", "-loglevel", "error", "-nostdin", "-y", "-sseof", "-0.3", "-i", str(clip), "-update", "1", "-frames:v", "1", str(last)])
            with Image.open(made) as poster, Image.open(last) as movie:
                self.assertEqual(poster.size, tuple(CASES["size"]))
                actual = np.asarray(poster.convert("RGB").resize(movie.size, Image.Resampling.LANCZOS), dtype=float)
                expected = np.asarray(movie.convert("RGB"), dtype=float)
                self.assertGreater(actual.std(), 10)
                self.assertLess(np.abs(actual - expected).mean(), 8)


if __name__ == "__main__":
    unittest.main()
