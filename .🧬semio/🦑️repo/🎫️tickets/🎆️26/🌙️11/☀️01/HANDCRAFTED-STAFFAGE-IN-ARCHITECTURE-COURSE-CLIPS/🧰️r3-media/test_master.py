"""🧪 Lesson masters stay fresh when the chapter record keeps the curriculum clause range."""
import json
import tempfile
import unittest
from pathlib import Path

import lib


def _status(folder: Path, declared: list[dict], recorded: list[dict], fresh: list[bool]) -> dict:
    """📦 Build one master whose source fingerprint omits clauses."""
    master = folder / "Lesson_speaker_en_480p15.mp4"
    master.write_bytes(b"x" * 1001)
    source = {"settings": {"language": "en"}, "sources": {"a.py": "abc"}, "segment": {"script": "a.py", "scene": "Beat"}, "casts": {}}
    master.with_suffix(".render.json").write_text(json.dumps({"content": {"chapters": [{"segments": recorded}]}}), encoding="utf-8")
    return {"master": master, "segments": [{"fresh": item, "state": source, "declared": clause} for item, clause in zip(fresh, declared, strict=True)]}


class MasterClauses(unittest.TestCase):
    """♻️ Chapter records keep clause ranges that canonical source clips omit."""

    def test_clause_ranges_match_the_curriculum(self):
        """🎯 A published clause range matches the declared segment."""
        declared = [
            {"script": "a.py", "scene": "Beat", "clauses": {"first": "store", "last": "room"}},
            {"script": "a.py", "scene": "Beat", "clauses": {"first": "winter", "last": "winter"}},
        ]
        source = {"settings": {"language": "en"}, "sources": {"a.py": "abc"}, "segment": {"script": "a.py", "scene": "Beat"}, "casts": {}}
        recorded = [{**source, "segment": clause} for clause in declared]
        with tempfile.TemporaryDirectory() as folder:
            self.assertTrue(lib.master_fresh(_status(Path(folder), declared, recorded, [True, True])))

    def test_missing_clause_range_stays_stale(self):
        """🎯 A chapter record that dropped its clause range is stale."""
        declared = [{"script": "a.py", "scene": "Beat", "clauses": {"first": "store", "last": "room"}}]
        source = {"settings": {"language": "en"}, "sources": {"a.py": "abc"}, "segment": {"script": "a.py", "scene": "Beat"}, "casts": {}}
        with tempfile.TemporaryDirectory() as folder:
            self.assertFalse(lib.master_fresh(_status(Path(folder), declared, [source], [True])))

    def test_stale_source_blocks_the_master(self):
        """🎯 A stale source clip blocks the lesson master."""
        declared = [{"script": "a.py", "scene": "Beat"}]
        source = {"settings": {"language": "en"}, "sources": {"a.py": "abc"}, "segment": {"script": "a.py", "scene": "Beat"}, "casts": {}}
        with tempfile.TemporaryDirectory() as folder:
            self.assertFalse(lib.master_fresh(_status(Path(folder), declared, [source], [False])))


if __name__ == "__main__":
    unittest.main()
