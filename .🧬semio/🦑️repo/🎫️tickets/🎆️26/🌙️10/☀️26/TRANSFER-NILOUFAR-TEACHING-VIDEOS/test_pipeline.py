"""🧪 Language-neutral teaching pipeline contract checks."""
import importlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import wave
import unittest
from unittest.mock import patch

HERE = Path(__file__).resolve().parent
ROOT = Path(sys.argv.pop(1)).resolve() if len(sys.argv) > 1 else next(parent for parent in HERE.parents if (parent / "🧰️framework").is_dir())
MAPPING = json.loads((HERE / "taxonomy-paths.cases.json").read_text(encoding="utf-8"))
MOVED = {row["old"]: row["new"] for row in MAPPING["moves"] if row["new"]}
RUNTIME_NAMESPACE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️"
CASES = json.loads((HERE / "pipeline-cases.json").read_text(encoding="utf-8"))
sys.path.insert(0, str(ROOT))

class PipelineTests(unittest.TestCase):
    def module(self):
        spec = importlib.util.spec_from_file_location("teaching_tts_under_test", ROOT / MOVED["🎓️teaching/tts_pipeline.py"])
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def test_portable_sources(self):
        paths = [ROOT / row["new"] for row in MAPPING["moves"] if row["new"] and (row["old"].endswith("build_full_video.py") or row["old"].endswith("generate_audio.py") or row["old"].endswith("tts_pipeline.py"))]
        for path in paths:
            text = path.read_text(encoding="utf-8")
            for token in CASES["forbidden"]:
                with self.subTest(path=path, token=token):
                    self.assertFalse(token in text, f"Forbidden reference: {token}")

    def test_import_without_manim_or_http_client(self):
        with patch.dict(sys.modules, {"manim": None, "httpx": None, "manim_visuals": None}):
            self.assertTrue(callable(self.module().synthesize_narration))

    def test_section_build_contract(self):
        cases = json.loads((HERE / "section-build-cases.json").read_text(encoding="utf-8"))
        for index, case in enumerate(cases):
            spec = importlib.util.spec_from_file_location(f"section_{index}", ROOT / case["path"])
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            names = [scene for _, scene, _ in module.SCENES]
            expected = (["Demo_Intro_PhysikalischeGrundlagen"] if case["intro"] else []) + case["names"]
            self.assertEqual(names, expected)

    def test_section_build_disables_partial_cache(self):
        module = self.module()
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            folder = Path(directory)

            def render(arguments, **kwargs):
                media = configuration.call_args.args[0]
                output = media / "v" / f"{arguments[arguments.index('-o') + 1]}.mp4"
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_bytes(b"rendered fixture")

            with patch.dict(os.environ, {"SEMIO_TEACHING_OUTPUT_ROOT": str(folder)}), patch.object(importlib.import_module(RUNTIME_NAMESPACE), "render_config", return_value=folder / "render.cfg") as configuration, patch.object(module, "run_command", side_effect=render) as command, patch.object(module, "mux_scene_audio", return_value=str(folder / "muxed.mp4")), patch.object(module, "compose_final_video"):
                module.build_section_video((ROOT / MOVED["🎓️teaching/energy/demand/1_physical_fundamentals/build_full_video.py"]).parent.parent, [(folder / "fixture.py", "Fixture", folder / "absent.mp3")], "fixture.mp4")
            command.assert_called_once()
            self.assertIn(CASES["render_policy"]["required_argument"], command.call_args.args[0])

    def test_subtitle_contract(self):
        module = self.module()
        for case in CASES["narration"]:
            self.assertEqual(module.subtitle_narration_text(case["clauses"]), case["expected"])

    def test_missing_credentials_skip_without_network(self):
        module = self.module()
        with patch.dict(os.environ, {}, clear=True), patch.object(module, "_ENV_FILE", HERE / "not-present.env"), patch("urllib.request.urlopen") as network:
            self.assertEqual(list(module.synthesize_narration(CASES["skipped"]["text"], Path(CASES["skipped"]["path"]))), CASES["skipped"]["expected"])
            network.assert_not_called()

    def test_wave_measurement_matches_numpy(self):
        import numpy as np
        import av
        module = self.module()
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            target = Path(directory) / "tone.wav"
            samples = np.zeros(CASES["wave"]["frames"], dtype=np.int16)
            with wave.open(str(target), "wb") as audio:
                audio.setnchannels(1)
                audio.setsampwidth(samples.dtype.itemsize)
                audio.setframerate(CASES["wave"]["sample_rate"])
                audio.writeframes(samples.tobytes())
            with av.open(str(target)) as reference:
                stream = reference.streams.audio[0]
                expected = float(stream.duration * stream.time_base)
            self.assertEqual(expected, CASES["wave"]["seconds"])
            self.assertEqual(module.probe_duration(target), expected)

    def test_optional_clauses_create_no_files(self):
        module = self.module()
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            target = Path(directory) / "clauses"
            with patch.dict(os.environ, {}, clear=True), patch.object(module, "_ENV_FILE", HERE / "not-present.env"):
                result = module.synthesize_clause_audio(CASES["narration"][0]["clauses"], target)
            self.assertTrue(all(path is None and seconds == 0 for _, path, seconds in result))
            self.assertFalse(target.exists())

    def test_http_payload_and_audio_output(self):
        from unittest.mock import MagicMock
        module = self.module()
        response = MagicMock()
        response.__enter__.return_value = response
        response.read.return_value = b"sample-mp3-response"
        response.headers = {"Content-Type": "audio/mpeg"}
        values = {"TTS_MODEL": "chosen-model", "TTS_BASE_URL": "https://example.invalid/v1", "TTS_API_KEY": "test-key", "TTS_VOICE": "test-voice"}
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            with patch.dict(os.environ, values, clear=True), patch.object(module, "_ENV_FILE", HERE / "not-present.env"), patch("urllib.request.urlopen", return_value=response) as network:
                output, skipped = module.synthesize_narration(" Speech text ", Path(directory) / "speech.mp3")
            self.assertFalse(skipped)
            self.assertEqual(Path(output).read_bytes(), response.read.return_value)
            request = network.call_args.args[0]
            self.assertEqual(request.full_url, "https://example.invalid/v1/audio/speech")
            self.assertEqual(json.loads(request.data), {"model": "chosen-model", "input": "Speech text", "voice": "test-voice", "response_format": "mp3"})

    def test_video_measurement_without_ffprobe(self):
        import av
        import imageio_ffmpeg
        module = self.module()
        executable = imageio_ffmpeg.get_ffmpeg_exe()
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            target = Path(directory) / "video.mp4"
            module.run_command([executable, "-v", "error", "-f", "lavfi", "-i", "color=c=black:s=32x32:r=10:d=0.6", "-c:v", "libx264", "-pix_fmt", "yuv420p", str(target)])
            with av.open(str(target)) as reference:
                stream = reference.streams.video[0]
                expected = float(stream.duration * stream.time_base)
            with patch.dict(os.environ, {"SEMIO_FFMPEG_BINARY": executable}, clear=True), patch("shutil.which", return_value=None):
                self.assertAlmostEqual(module.probe_duration(target), expected, places=2)

    def test_mux_preserves_longer_stream_and_concat(self):
        import av
        import imageio_ffmpeg
        module = self.module()
        executable = imageio_ffmpeg.get_ffmpeg_exe()
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as directory:
            base = Path(directory)
            clips = []
            for case in CASES["mux"]:
                video = base / f"{case['id']}_video.mp4"
                module.run_command([executable, "-v", "error", "-f", "lavfi", "-i", f"color=c=black:s=32x32:r=10:d={case['video_seconds']}", "-c:v", "libx264", "-pix_fmt", "yuv420p", str(video)])
                audio = None
                if case["audio_seconds"]:
                    audio = base / "narration.wav"
                    with wave.open(str(audio), "wb") as output:
                        output.setnchannels(1)
                        output.setsampwidth(2)
                        output.setframerate(24000)
                        output.writeframes(bytes(int(case["audio_seconds"] * 24000) * 2))
                with patch.dict(os.environ, {"SEMIO_FFMPEG_BINARY": executable}, clear=True), patch("shutil.which", return_value=None):
                    result = module.mux_scene_audio(video, audio, base / f"{case['id']}.mp4")
                clips.append(result)
                with av.open(result) as reference:
                    self.assertEqual(len(reference.streams.audio), 1)
                    stream = reference.streams.video[0]
                    self.assertAlmostEqual(float(stream.duration * stream.time_base), case["expected_seconds"], delta=1 / 60)
                    self.assertGreater(sum(1 for _ in reference.decode(video=0)), 0)
            with patch.dict(os.environ, {"SEMIO_FFMPEG_BINARY": executable}, clear=True):
                result = module.compose_final_video(clips, base / "complete.mp4")
            with av.open(result) as reference:
                expected_frames = round(sum(case["expected_seconds"] for case in CASES["mux"]) * 60)
                self.assertEqual(sum(1 for _ in reference.decode(video=0)), expected_frames)

    def test_explicit_endpoint_is_preserved(self):
        module = self.module()
        values = {"TTS_MODEL": "chosen-model", "TTS_BASE_URL": "https://example.invalid/v1", "TTS_API_KEY": "test-key"}
        with patch.dict(os.environ, values, clear=True), patch.object(module, "_ENV_FILE", HERE / "not-present.env"):
            module.configure_tutorial_tts()
            self.assertEqual(os.environ["TTS_MODEL"], "chosen-model")
            self.assertEqual(os.environ["TTS_BASE_URL"], "https://example.invalid/v1")

if __name__ == "__main__":
    unittest.main()
