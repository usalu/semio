"""🧪 Check freeze comparisons and independent schema validation."""
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import jsonschema

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("preview_freeze", HERE / "preview-freeze.py")
freeze = importlib.util.module_from_spec(spec)
spec.loader.exec_module(freeze)
cases = json.loads((HERE / "preview-freeze-cases.json").read_text(encoding="utf-8"))
schema = json.loads((HERE / "preview-freeze.schema.json").read_text(encoding="utf-8"))


class FreezeTests(unittest.TestCase):
    def test_schema_oracle(self):
        jsonschema.validate(cases["valid"], schema)
        for case in cases["invalid"]:
            with self.assertRaises(jsonschema.ValidationError):
                jsonschema.validate(case, schema)

    def test_entry_mismatches(self):
        v = cases["valid"]
        manifest = {"file": v["path"], "sha256": v["sha256"], "media": v["media"]}
        joined = {"master": v["path"], "sha256": v["sha256"], "media": v["media"]}
        freeze.verify_entry("lesson", v["path"], v["sha256"], v["media"], manifest, joined)
        for target in (manifest, joined):
            for field in target:
                changed = dict(target, **{field: "wrong"})
                with self.subTest(target=target, field=field), self.assertRaisesRegex(ValueError, "lesson.*mismatch"):
                    freeze.verify_entry("lesson", v["path"], v["sha256"], v["media"], changed if target is manifest else manifest, changed if target is joined else joined)

    def test_complete_gate_and_corruption(self):
        oracle = json.loads((HERE / "🗑️generated" / "preview-freeze-node-oracle.json").read_text(encoding="utf-8"))
        with tempfile.TemporaryDirectory(dir=HERE / "🗑️generated") as temp:
            root = Path(temp)
            folder = root / "video"
            folder.mkdir()
            (folder / "preview").mkdir()
            preview = folder / "preview" / "Architecture_course_speaker_en_480p15.mp4"
            preview.write_bytes(b"preview bytes")
            names = [f"lesson{i:02}" for i in range(26)]
            videos = [{"id": name, "output": name} for name in names]
            (root / "course-preview-contract.json").write_text(json.dumps({"lessons": names}), encoding="utf-8")
            document = {"version": 1, "language": "en", "audience": "speaker", "videos": {}}
            lessons = []
            for name in names:
                master = folder / f"{name}.mp4"
                master.write_bytes(b"master bytes")
                relative = str(master.relative_to(root))
                document["videos"][name] = {"master": {"file": relative, "sha256": oracle["master"], "media": oracle["media"], "fresh": True}}
                lessons.append({"id": name, "title": name, "master": relative, "sha256": oracle["master"], "media": oracle["media"]})
            manifest = folder / "media-manifest.json"
            receipt = preview.with_suffix(".json")
            joined = {"language": "en", "audience": "speaker", "lessons": lessons, "sha256": oracle["preview"], "media": oracle["combined"]}
            manifest.write_text(json.dumps(document), encoding="utf-8")
            receipt.write_text(json.dumps(joined), encoding="utf-8")
            changed = {"fresh": True, "state": oracle["state"], "casts": {}}
            def status(name, language, audience, output, states):
                return {"master": folder / f"{name}.mp4", "segments": [{"state": {}, "fresh": True, "state_sha256": changed["state"]}]}
            resolved_titles = {name: name for name in names}
            with patch.object(freeze, "HERE", root), patch.object(freeze.lib, "ROOT", root), patch.object(freeze.lib, "base", return_value=folder), patch.object(freeze.lib, "runtime") as runtime, patch.object(freeze.lib, "video_status", side_effect=status), patch.object(freeze.lib, "master_fresh", side_effect=lambda _: changed["fresh"]), patch.object(freeze.lib, "provenance", side_effect=lambda *_: {"casts": changed["casts"], "records": {}}), patch.object(freeze.chapter_titles, "titles", return_value=resolved_titles):
                runtime.return_value.load_manifest.return_value = videos
                freeze.main("en")
                changed["fresh"] = False
                with self.assertRaisesRegex(ValueError, "stale canonical"):
                    freeze.main("en")
                changed["fresh"] = True
                joined["lessons"].reverse()
                receipt.write_text(json.dumps(joined), encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "lesson order"):
                    freeze.main("en")
                joined["lessons"].reverse()
                joined["lessons"][0]["title"] = "changed"
                receipt.write_text(json.dumps(joined), encoding="utf-8")
                with self.assertRaisesRegex(ValueError, "localized title mismatch"):
                    freeze.main("en")
                joined["lessons"][0]["title"] = names[0]
                receipt.write_text(json.dumps(joined), encoding="utf-8")
                resolved_titles[names[0]] = "changed"
                with self.assertRaisesRegex(ValueError, "localized title mismatch"):
                    freeze.main("en")
                resolved_titles[names[0]] = names[0]
                master = folder / f"{names[0]}.mp4"
                master.write_bytes(b"changed")
                with self.assertRaisesRegex(ValueError, "sha256 mismatch"):
                    freeze.main("en")
                master.write_bytes(b"master bytes")
                for field, value in (("state", "changed"), ("casts", {"person": "changed"})):
                    original = changed[field]
                    changed[field] = value
                    with self.assertRaisesRegex(ValueError, "media mismatch"):
                        freeze.main("en")
                    changed[field] = original
                preview.write_bytes(b"changed")
                with self.assertRaisesRegex(ValueError, "preview sha256 mismatch"):
                    freeze.main("en")


if __name__ == "__main__":
    unittest.main()
