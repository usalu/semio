"""One-shot narrow patch of the staffage bridge tests and their fixture for levels of detail (staffage owner)."""
import sys
from pathlib import Path

folder = Path(sys.argv[1])


def edit(name, steps):
    path = folder / name
    s = path.read_bytes().decode("utf-8").replace("\r\n", "\n")
    for old, new in steps:
        if old not in s:
            raise SystemExit(f"MISSING in {name}: {old[:80]}")
        s = s.replace(old, new, 1)
    path.write_bytes(s.encode("utf-8"))


edit("🔣️.json", [(
    '    "heightM": 4,\n    "boundsM": {',
    '    "logical": {\n      "heightM": 4,\n      "figureBoundsM": { "minX": -2.5, "minY": -0.5, "maxX": 2.5, "maxY": 4.5 },\n      "partsM": { "slat-0": [[0.4, 0.2], [0.6, 0.2], [0.6, 0.4], [0.4, 0.4]], "slat-1": [[1, 0.2], [1.2, 0.2], [1.2, 0.4]], "box": [[2, 0], [3, 0], [3, 1], [2, 1]] },\n      "maxPaths": 2\n    },\n    "boundsM": {')])

edit("🐍️.py", [
    ('CASE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))\n',
     '''CASE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
LOD = S.lod_level("full", 15)
CASE["index"]["lod"] = {"id": LOD.id, "revision": LOD.revision, "canonicalFrames": 3, "canonicalFps": 10}
CASE["index"]["provenance"]["lod"] = {"id": LOD.id, "revision": LOD.revision}


def setUpModule():
    """🔭 Draw every cast of this module at the full level: a cast API call has no level of its own."""
    S._SCOPE.append(LOD)


def tearDownModule():
    S._SCOPE.remove(LOD)
'''),
    ('    provenance = S.expected_provenance(cast)\n    provenance["asset"]["category"] = "humans"\n    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}\n    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}',
     '    provenance = S.expected_provenance(cast, LOD)\n    provenance["asset"]["category"] = "humans"\n    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}\n    return {**index, "cast": cast, "lod": CASE["index"]["lod"], "provenance": {**provenance, "clock": clock}}'),
    ('            parts = S._parts(folder, "slat")\n            self.assertEqual(list(parts), ["slat-0"])\n', '            with serving(folder):\n                parts = S.staffage_parts("case", "slat")\n            self.assertEqual(list(parts), ["slat-0", "slat-1"])\n'),
    ('''        self.assertEqual(list(parts[0]), list(parts[1]))
        np.testing.assert_array_equal(parts[0]["a"], parts[1]["a"])
        self.assertEqual(len(parts[1]["a"]), 4)
''', ''),
    ('''        with tempfile.TemporaryDirectory() as root:
            parts = []
            for body in (long, compact):
                folder = Path(root) / str(len(parts))
                folder.mkdir()
                (folder / "🔣️.json").write_text(json.dumps({**index, "frames": ["0000.svg"]}), encoding="utf-8")
                (folder / "0000.svg").write_text(head + body + "</svg>", encoding="utf-8")
                parts.append(S._parts(folder, ""))
''', ''),
    ('    return patch.object(S, "ensure_casts", lambda ids=None: {cast: folder for cast in (ids or ["case"])})', '    return patch.object(S, "ensure_casts", lambda ids=None, progress=None, lod=None: {cast: folder for cast in (ids or ["case"])})'),
    ('            S._SLOT_COUNTS.pop(folder, None)\n            S._INDEXES.pop(folder, None)\n            self.assertEqual(S._slot_count(folder), len(paths))',
     '            S._INDEXES.pop(folder, None)\n            self.assertEqual(S._slot_count(folder), CASE["index"]["logical"]["maxPaths"])\n            self.assertEqual(S._slot_count(folder), len(CASE["frames"][0].split("<path")) - 1)'),
    ('''                for folder in ("🔨️modules", "🧬️schema", "🗂️catalog/humans/case-asset"):
                    (tree / folder).mkdir(parents=True)
                (tree / "🔨️modules/a.ts").write_text("a", encoding="utf-8")
                (tree / "🧬️schema/b.ts").write_text("b", encoding="utf-8")''', '''                for folder in ("🔨️modules/📤️export/🎭️cast", "🧬️schema", "🗂️catalog/humans/case-asset"):
                    (tree / folder).mkdir(parents=True)
                (tree / "🔨️modules/📤️export/🎭️cast/🟦️.ts").write_text('import { a } from "../../a.ts";', encoding="utf-8")
                (tree / "🔨️modules/a.ts").write_text("a", encoding="utf-8")
                (tree / "🔨️modules/dxf.ts").write_text("never loaded", encoding="utf-8")
                (tree / "🧬️schema/b.ts").write_text("b", encoding="utf-8")'''),
    ('''                self.assertNotEqual(before, S.cast_key(entry, tree), relative)
''', '''                self.assertNotEqual(before, S.cast_key(entry, tree), relative)
        with tempfile.TemporaryDirectory() as root:
            tree = Path(root)
            for folder in ("🔨️modules/📤️export/🎭️cast", "🧬️schema", "🗂️catalog/humans/case-asset"):
                (tree / folder).mkdir(parents=True)
            (tree / "🔨️modules/📤️export/🎭️cast/🟦️.ts").write_text('import { a } from "../../a.ts";', encoding="utf-8")
            (tree / "🔨️modules/a.ts").write_text("a", encoding="utf-8")
            (tree / "🔨️modules/dxf.ts").write_text("never loaded", encoding="utf-8")
            (tree / "🗂️catalog/humans/case-asset/🔣️.json").write_text("{}", encoding="utf-8")
            before = S.cast_key(entry, tree)
            S._source_digest.cache_clear()
            (tree / "🔨️modules/dxf.ts").write_text("edited", encoding="utf-8")
            self.assertEqual(before, S.cast_key(entry, tree), "a module the cast exporter never imports must not re-key the casts")
'''),
    ('''            (tree / "🔨️modules").mkdir()
            for name in names:
                (tree / "🔨️modules" / name).write_text("x", encoding="utf-8")''', '''            (tree / "🔨️modules/📤️export/🎭️cast").mkdir(parents=True)
            (tree / "🔨️modules/📤️export/🎭️cast/🟦️.ts").write_text("".join(f'import "../../{name}";' for name in names), encoding="utf-8")
            for name in names:
                (tree / "🔨️modules" / name).write_text("x", encoding="utf-8")'''),
    ('        self.assertEqual([Path(name).name for name in seen if name], sorted(names))', '        self.assertEqual([Path(name).name for name in seen if name.endswith(".ts") and Path(name).name != "🟦️.ts"], sorted(names))'),
    ('    provenance = S.expected_provenance(cast)\n    provenance["asset"]["category"] = "humans"\n    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}\n    return {**index, "cast": cast, "provenance": {**provenance, "clock": clock}}\ncounter',
     '    lod = S.lod_level("full", 15)\n    provenance = S.expected_provenance(cast, lod)\n    provenance["asset"]["category"] = "humans"\n    clock = {"fps": index.get("fps"), "durationS": index.get("durationS"), "loop": index.get("loop"), "offsetS": 0, "frames": len(index["frames"])}\n    return {**index, "cast": cast, "lod": {"id": lod.id, "revision": lod.revision, "canonicalFrames": len(index["frames"]), "canonicalFps": index["fps"]}, "provenance": {**provenance, "clock": clock}}\nS._SCOPE.append(S.lod_level("full", 15))\ncounter'),
    ('lock = S._lock(sys.argv[2])', 'lock = S._lock(sys.argv[2], S.lod_level("full", 15))'),
])
print("patched tests")
