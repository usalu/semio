"""🎞️ Exercise localized title contracts and real FFmetadata round trips."""
import importlib
import json
import os
import subprocess
import sys
import tempfile
import types
import unittest
from pathlib import Path
from unittest.mock import patch

import av
import imageio_ffmpeg
import jsonschema

HERE = Path(__file__).resolve().parent
MEDIA = HERE.parent.parent / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🧰️r3-media"
sys.path.insert(0, str(MEDIA))
import chapter_titles
import lib


class TitlesTest(unittest.TestCase):
    def test_catalog(self):
        videos = lib.runtime().load_manifest()
        hierarchy = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🥞️hierarchy.🐍️")
        entries = [v for t in hierarchy.hierarchy_contract()["topics"] for v in t["videos"]]
        schema = json.loads((HERE / "chapter-title.schema.json").read_text(encoding="utf-8"))
        self.assertEqual(len(videos), 26)
        for video in videos:
            found = [entry for entry in entries if entry["id"] == video["id"]]
            self.assertEqual(len(found), 1)
            jsonschema.validate(found[0]["title"], schema)
        for language in ("en", "de"):
            labels = chapter_titles.titles(videos, language)
            self.assertEqual(len(labels), 26)
            self.assertEqual(len({label.casefold() for label in labels.values()}), 26)

    def test_invalid_contracts(self):
        videos = [{"id": "one"}]
        entry = {"id": "one", "title": {"en": "One", "de": "Eins"}}
        for entries in ([], [entry, entry], [{"id": "one", "title": {"en": "One"}}], [{"id": "one", "title": {"en": "One", "de": " "}}]):
            with patch.object(chapter_titles.importlib, "import_module", return_value=types.SimpleNamespace(hierarchy_contract=lambda: {"topics": [{"title": {"en": "Topic", "de": "Thema"}, "videos": entries}]})), self.assertRaises(ValueError):
                chapter_titles.titles(videos, "en")
        with self.assertRaises(ValueError):
            chapter_titles.titles(videos, "fr")

    def test_ffmetadata_round_trip(self):
        out = HERE / "🗑️generated"
        out.mkdir(parents=True, exist_ok=True)
        env = dict(os.environ, SEMIO_TOOL_FFMPEG=imageio_ffmpeg.get_ffmpeg_exe())
        with tempfile.TemporaryDirectory(dir=out) as temp:
            root = Path(temp)
            for language, names in (("en", ["Energy = Heat; Light", "Capsule #2 \\ Tower"]), ("de", ["Lüftung und Wärme", "Größe = Fläche; Öl"])):
                metadata = root / f"{language}.txt"
                metadata.write_text(";FFMETADATA1\n" + "".join(f"[CHAPTER]\nTIMEBASE=1/1000\nSTART={i * 1000}\nEND={(i + 1) * 1000}\ntitle={chapter_titles.ffmetadata(name)}\n" for i, name in enumerate(names)), encoding="utf-8")
                video = root / f"{language}.mp4"
                subprocess.run(["repo", "tools", "exec", "--light", "ffmpeg", "--", "-hide_banner", "-v", "error", "-f", "lavfi", "-i", "color=size=64x64:rate=15:duration=2", "-i", str(metadata), "-map", "0:v", "-map_metadata", "1", "-map_chapters", "1", "-c:v", "libx264", "-y", str(video)], check=True, env=env)
                with av.open(str(video)) as container:
                    self.assertEqual([chapter["metadata"]["title"] for chapter in container.chapters()], names)


if __name__ == "__main__":
    unittest.main()
