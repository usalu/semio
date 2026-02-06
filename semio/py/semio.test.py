# region Header

# 🧪︎ semio/py/semio.test.py

# 2025 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

import json
import os
import tempfile

import pytest

from semio import (
    _applyDesignDiff,
    applyKitDiffDict,
    areKitDiffsDictEqual,
    areKitsDictEqual,
    areValidationResultsEqual,
    export_kit,
    flattenDesignDict,
    getKitDiffDict,
    import_kit,
    inverseKitDiffDict,
    parseValidationResult,
    validateKitDict,
)

TOLERANCE = 0.001
ASSETS_DIR = "../../assets/semio"


def load_json(filename: str) -> dict:
    path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def is_close(a, b):
    return abs(a - b) < TOLERANCE


def vectors_equal(v1, v2):
    if v1 is None or v2 is None:
        return False
    return is_close(v1.get("x", 0), v2.get("x", 0)) and is_close(v1.get("y", 0), v2.get("y", 0)) and is_close(v1.get("z", 0), v2.get("z", 0))


def planes_equal(p1, p2):
    if p1 is None or p2 is None:
        return False
    if not p1.get("origin") or not p2.get("origin"):
        return False
    if not p1.get("xAxis") or not p2.get("xAxis"):
        return False
    if not p1.get("yAxis") or not p2.get("yAxis"):
        return False
    return vectors_equal(p1.get("origin"), p2.get("origin")) and vectors_equal(p1.get("xAxis"), p2.get("xAxis")) and vectors_equal(p1.get("yAxis"), p2.get("yAxis"))


def centers_equal(c1, c2):
    if c1 is None or c2 is None:
        return c1 == c2
    return is_close(c1.get("u", 0), c2.get("u", 0)) and is_close(c1.get("v", 0), c2.get("v", 0))


def find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    parent_guid = None
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_guid = d.get("guid")
                break
        if not parent_guid:
            raise ValueError(f"Parent {parent_name} not found")

    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_guid:
                if p and p.get("guid") == parent_guid:
                    return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")


def flatten_test(design_name, parent_name=None):
    kit_dict = load_json("kit_metabolism.json")
    design = find_design(kit_dict, design_name, parent_name)

    expected_design = next(
        (d for d in kit_dict.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")),
        None,
    )
    assert expected_design is not None, f"Expected Flat design for {design_name} not found"

    flat_design_diff = flattenDesignDict(kit_dict, design.get("guid"))
    flat_design = _applyDesignDiff(design, flat_design_diff)

    for piece in flat_design.get("pieces", []):
        expected_piece = next(
            (x for x in expected_design.get("pieces", []) if x.get("name") == piece.get("name")),
            None,
        )
        assert expected_piece is not None, f"Piece {piece.get('name')} not found in expected design"
        assert piece.get("plane") is not None
        assert piece.get("center") is not None
        assert planes_equal(piece.get("plane"), expected_piece.get("plane"))
        assert centers_equal(piece.get("center"), expected_piece.get("center"))


class TestRoundtrip:
    class TestJson:
        class TestMetabolism:
            def test_kit_json_kit(self):
                kit_dict = load_json("kit_metabolism.json")
                serialized = json.dumps(kit_dict)
                deserialized = json.loads(serialized)
                assert areKitsDictEqual(kit_dict, deserialized)

    class TestZip:
        class TestMetabolism:
            def test_zip_kit_zip_kit(self):
                zip_path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, "metabolism.zip")
                kit, files = import_kit(zip_path)

                assert kit.name == "Metabolism"
                assert len(kit.types or []) > 0
                assert len(kit.designs or []) > 0
                assert len(files) > 0

                with tempfile.TemporaryDirectory() as tmpdir:
                    roundtrip_path = os.path.join(tmpdir, "metabolism_roundtrip.zip")
                    export_kit(kit, files, roundtrip_path)
                    kit2, files2 = import_kit(roundtrip_path)

                assert kit2.name == kit.name
                assert len(kit2.types or []) == len(kit.types or [])
                assert len(kit2.designs or []) == len(kit.designs or [])
                assert len(files2) == len(files)


class TestFlatten:
    class TestNakaginCapsuleTower:
        def test_kit_flatten_diff_apply_flat(self):
            flatten_test("Nakagin Capsule Tower")

        class TestSlanted:
            def test_kit_flatten_diff_apply_flat(self):
                flatten_test("Slanted", "Nakagin Capsule Tower")

        class TestTwisted:
            def test_kit_flatten_diff_apply_flat(self):
                flatten_test("Twisted", "Nakagin Capsule Tower")

        class TestDancing:
            def test_kit_flatten_diff_apply_flat(self):
                flatten_test("Dancing", "Nakagin Capsule Tower")

    class TestCapsuleDream:
        def test_kit_flatten_diff_apply_flat(self):
            flatten_test("Capsule Dream")


class TestDiff:
    class TestMetabolism:
        def test_kit_diff_diffedkit_diffedkit_inversediff_kit(self):
            kit_original = load_json("kit_metabolism.json")
            kit_original["designs"] = [d for d in kit_original.get("designs", []) if not d.get("parent")]
            kit_diff = load_json("diff_kit_metabolism.json")
            kit_diff_inverted = load_json("diff_kit_metabolism_inverted.json")
            kit_diffed = load_json("kit_metabolism_diffed.json")

            computed_diff = getKitDiffDict(kit_original, kit_diffed)
            assert areKitDiffsDictEqual(computed_diff, kit_diff)
            computed_inverse_diff = inverseKitDiffDict(kit_original, kit_diff)
            assert areKitDiffsDictEqual(computed_inverse_diff, kit_diff_inverted)
            applied_forward = applyKitDiffDict(kit_original, kit_diff)
            assert areKitsDictEqual(applied_forward, kit_diffed)
            applied_inverse = applyKitDiffDict(kit_diffed, kit_diff_inverted)
            assert areKitsDictEqual(applied_inverse, kit_original)


class TestValidation:
    class TestMetabolism:
        def test_metabolism_kit_validate_empty_report(self):
            valid_kit = load_json("kit_metabolism.json")
            valid_result = validateKitDict(valid_kit)
            assert not valid_result.hasErrors()

    class TestInvalid:
        def test_invalid_kit_validate_invalid_report(self):
            invalid_kit = load_json("kit_invalid.json")
            result = validateKitDict(invalid_kit)
            expected = parseValidationResult(json.dumps(load_json("validation.json")))
            assert areValidationResultsEqual(result, expected)
