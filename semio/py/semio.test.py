# region Header
# [👤semio📚py🥼semiotest](repo://p/u/semio/b/l/py/f/semio.test.py)

# 2025 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header

import json
import os
import struct
import sys
import tempfile
from base64 import b64encode
from pathlib import Path
from typing import Any

import pytest

sys.path.insert(0, os.path.dirname(__file__))

from semio import (
    GeometricInsights,
    Kit,
    KitData,
    _applyDesignDiff,
    _build_file_path,
    applyKitDiffDict,
    areKitDiffsDictEqual,
    areKitsDictEqual,
    areValidationResultsEqual,
    export_design_model,
    export_kit,
    flattenDesignDict,
    geometric_insights_to_report_dict,
    get_geometric_insights_for_model,
    getKitChange,
    getKitDiffDict,
    import_kit,
    inverseKitDiffDict,
    parseValidationResult,
    sumQualityInDesignDict,
    validateKitDict,
)

TOLERANCE = 0.001
ASSETS_DIR = "../assets/semio"
REPORTS_EXPORT_DIR = Path(__file__).resolve().parents[2] / "reports" / "export-design-model"
REPORTS_MODEL_KPI_DIR = Path(__file__).resolve().parents[2] / "reports" / "model-kpi"


def load_json(filename: str) -> dict:
    path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def load_kit(filename: str) -> dict:
    """Load and normalize kit JSON for Kit.parse (flattens parent/folder refs, etc.)."""
    data = load_json(filename)
    if "guid" in data and "uri" not in data:
        data["uri"] = data["guid"]
    for key in ["types", "designs", "files", "folders", "authors", "concepts", "models", "connectors", "pieces", "connections", "layers", "groups", "stats", "ports", "qualities", "attributes"]:
        if key not in data or data[key] is None:
            data[key] = []
    for collection in ["types", "designs", "folders"]:
        if collection in data:
            for item in data[collection]:
                if "parent" in item and isinstance(item["parent"], dict) and "guid" in item["parent"]:
                    item["parent"] = item["parent"]["guid"]
                if "folder" in item and isinstance(item["folder"], dict) and "guid" in item["folder"]:
                    item["folder"] = item["folder"]["guid"]
    if "types" in data:
        for t in data["types"]:
            if "models" in t:
                for m in t["models"]:
                    if "file" in m and isinstance(m["file"], dict) and "guid" in m["file"]:
                        m["file"] = m["file"]["guid"]
                    if "file" not in m or m["file"] is None:
                        m["file"] = ""
                    if "url" not in m or m["url"] is None:
                        m["url"] = ""
                    if "tags" in m and isinstance(m["tags"], list):
                        new_tags = [tag["guid"] if isinstance(tag, dict) and "guid" in tag else tag for tag in m["tags"]]
                        m["tags"] = new_tags
                    elif "tags" not in m:
                        m["tags"] = []
    return data


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


def _contains_all_tags(model: dict[str, Any], selected_tag_guids: list[str]) -> bool:
    model_tag_guids = [t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])]
    return all(guid in model_tag_guids for guid in selected_tag_guids)


def _jaccard_tag_guids(model_tag_guids: list[str], selected_tag_guids: list[str]) -> float:
    if len(model_tag_guids) == 0 and len(selected_tag_guids) == 0:
        return 1.0
    set_a = set(model_tag_guids)
    set_b = set(selected_tag_guids)
    union = set_a | set_b
    if len(union) == 0:
        return 0.0
    return len(set_a & set_b) / len(union)


def _select_best_model_like_semio_ts(models: list[dict[str, Any]], selected_tag_guids: list[str]) -> dict[str, Any] | None:
    if len(models) == 0:
        return None
    if len(selected_tag_guids) == 0:
        default_model = next((model for model in models if len(model.get("tags", [])) == 0), None)
        return default_model if default_model is not None else models[0]
    filtered_models = [model for model in models if _contains_all_tags(model, selected_tag_guids)]
    if len(filtered_models) == 0:
        return None
    indexed_scores = [_jaccard_tag_guids([t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])], selected_tag_guids) for model in filtered_models]
    max_score = max(indexed_scores)
    max_score_index = indexed_scores.index(max_score)
    return filtered_models[max_score_index]


def _create_glb_blob(vertices: list[tuple[float, float, float]], faces: list[tuple[int, int, int]]) -> str:
    def _pad4(data: bytes, fill: bytes) -> bytes:
        padding = (-len(data)) % 4
        return data + fill * padding

    position_bytes = struct.pack("<" + "f" * (len(vertices) * 3), *(value for vertex in vertices for value in vertex))
    index_values = [index for face in faces for index in face]
    index_bytes = struct.pack("<" + "H" * len(index_values), *index_values)
    position_bytes = _pad4(position_bytes, b"\x00")
    index_bytes = _pad4(index_bytes, b"\x00")
    position_length = len(position_bytes)
    index_length = len(index_bytes)
    binary_chunk = position_bytes + index_bytes
    min_x = min(vertex[0] for vertex in vertices)
    min_y = min(vertex[1] for vertex in vertices)
    min_z = min(vertex[2] for vertex in vertices)
    max_x = max(vertex[0] for vertex in vertices)
    max_y = max(vertex[1] for vertex in vertices)
    max_z = max(vertex[2] for vertex in vertices)
    json_chunk = json.dumps(
        {
            "asset": {"version": "2.0"},
            "buffers": [{"byteLength": len(binary_chunk)}],
            "bufferViews": [
                {"buffer": 0, "byteOffset": 0, "byteLength": position_length, "target": 34962},
                {"buffer": 0, "byteOffset": position_length, "byteLength": index_length, "target": 34963},
            ],
            "accessors": [
                {
                    "bufferView": 0,
                    "componentType": 5126,
                    "count": len(vertices),
                    "type": "VEC3",
                    "min": [min_x, min_y, min_z],
                    "max": [max_x, max_y, max_z],
                },
                {
                    "bufferView": 1,
                    "componentType": 5123,
                    "count": len(index_values),
                    "type": "SCALAR",
                },
            ],
            "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "indices": 1}]}],
            "nodes": [{"mesh": 0}],
            "scenes": [{"nodes": [0]}],
            "scene": 0,
        },
        separators=(",", ":"),
    ).encode("utf-8")
    json_chunk = _pad4(json_chunk, b" ")
    total_length = 12 + 8 + len(json_chunk) + 8 + len(binary_chunk)
    glb = b"".join(
        [
            struct.pack("<4sII", b"glTF", 2, total_length),
            struct.pack("<I4s", len(json_chunk), b"JSON"),
            json_chunk,
            struct.pack("<I4s", len(binary_chunk), b"BIN\x00"),
            binary_chunk,
        ]
    )
    return "data:model/gltf-binary;base64," + b64encode(glb).decode("ascii")


class TestRoundtrip:
    class TestMetabolism:
        def test_roundtrip(self):
            import base64

            kit_dict = load_json("kit_metabolism.json")
            serialized = json.dumps(kit_dict)
            deserialized = json.loads(serialized)
            assert areKitsDictEqual(kit_dict, deserialized), "JSON -> Memory -> JSON: serialized and deserialized kit should be equal"

            files: dict[str, bytes] = {}
            for file_entry in kit_dict.get("files", []):
                blob = file_entry.get("blob")
                if blob:
                    b64 = blob.split(",", 1)[1] if blob.startswith("data:") else blob
                    decoded = base64.b64decode(b64)
                    file_path = _build_file_path(kit_dict, file_entry)
                    files[file_path] = decoded

            with tempfile.TemporaryDirectory() as tmpdir:
                roundtrip_path = os.path.join(tmpdir, "metabolism_roundtrip.zip")
                export_kit(KitData(kit_dict), files, roundtrip_path)

                kit2, files2 = import_kit(roundtrip_path)

            assert areKitsDictEqual(kit_dict, kit2.to_dict()), "ZIP -> JSON: roundtrip kit should be equal"
            assert len(files2) == len(files), f"Expected {len(files)} files, got {len(files2)}"


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


class TestChange:
    class TestMetabolism:
        def test_kit_change_forward_backward_inverse_behavior(self):
            kit_original = load_json("kit_metabolism.json")
            kit_original["designs"] = [d for d in kit_original.get("designs", []) if not d.get("parent")]
            kit_diff = load_json("diff_kit_metabolism.json")
            kit_diff_inverted = load_json("diff_kit_metabolism_inverted.json")
            kit_diffed = load_json("kit_metabolism_diffed.json")

            change = getKitChange(kit_original, kit_diffed)
            computed_diff = getKitDiffDict(kit_original, kit_diffed)
            assert areKitDiffsDictEqual(computed_diff, kit_diff)
            computed_inverse_diff = inverseKitDiffDict(kit_original, change.forward)
            assert areKitDiffsDictEqual(computed_inverse_diff, kit_diff_inverted)
            assert areKitDiffsDictEqual(change.forward, kit_diff)
            assert areKitDiffsDictEqual(change.backward, kit_diff_inverted)
            applied_forward = applyKitDiffDict(kit_original, change.forward)
            assert areKitsDictEqual(applied_forward, kit_diffed)
            applied_inverse = applyKitDiffDict(kit_diffed, change.backward)
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


class TestDesignModel:
    def test_model_selection_from_shared_semio_assets(self):
        payload = load_json("model_selection.json")
        for case in payload.get("cases", []):
            models = [
                {
                    "guid": model["guid"],
                    "file": {"guid": model["fileGuid"]},
                    "tags": [{"guid": guid} for guid in model.get("tagGuids", [])],
                }
                for model in case.get("models", [])
            ]
            selected = _select_best_model_like_semio_ts(models, case.get("selectedTagGuids", []))
            selected_guid = selected.get("guid") if selected else None
            assert selected_guid == case.get("expectedGuid"), f"Case {case.get('name')} failed"


class TestDesignQualitySum:
    class TestNakaginCapsuleTower:
        def test_sum_effective_floor_area(self):
            kit_dict = load_json("kit_metabolism.json")
            design = find_design(kit_dict, "Nakagin Capsule Tower")
            quality = next(q for q in kit_dict.get("qualities", []) if q.get("name") == "effective floor area")
            result = sumQualityInDesignDict(kit_dict, design["guid"], quality["guid"])
            assert abs(result - 2349.53) < TOLERANCE


class TestExportDesignModel:
    def test_export_glb_returns_valid_glb(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".glb")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert result[:4] == b"glTF"
        assert struct.unpack("<I", result[4:8])[0] == 2
        assert struct.unpack("<I", result[8:12])[0] == len(result)

    def test_export_gltf_returns_valid_json(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        assert isinstance(result, bytes)
        assert len(result) > 0
        parsed = json.loads(result.decode("utf-8"))
        assert "asset" in parsed
        assert "scenes" in parsed

    def test_export_invalid_format_raises(self):
        kit_dict = load_json("kit_metabolism.json")
        with pytest.raises(ValueError, match="Unsupported export format"):
            export_design_model(kit_dict, "Nakagin Capsule Tower", ".invalid")

    def test_export_scene_graph_report(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        parsed = json.loads(result.decode("utf-8"))
        assert "nodes" in parsed
        assert "scenes" in parsed
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.gltf").write_bytes(result)

    def test_export_ifc_returns_valid_ifc(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        assert isinstance(result, bytes)
        assert len(result) > 0
        ifc_text = result.decode("utf-8")
        assert "ISO-10303-21" in ifc_text
        assert "IFC4" in ifc_text
        assert "IFCPROJECT" in ifc_text
        assert "IFCSITE" in ifc_text
        assert "IFCBUILDING" in ifc_text
        assert "IFCBUILDINGSTOREY" in ifc_text
        assert "IFCELEMENTASSEMBLY" in ifc_text

    def test_export_ifc_contains_types_and_occurrences(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCBUILDINGELEMENTPROXYTYPE" in ifc_text
        assert "IFCBUILDINGELEMENTPROXY(" in ifc_text

    def test_export_ifc_contains_mesh_geometry(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCSHAPEREPRESENTATION" in ifc_text

    def test_export_ifc_converts_gltf_mesh_axes_to_semio_axes(self):
        import ifcopenshell

        kit_dict = {
            "name": "Axis Test Kit",
            "guid": "axis-test-kit",
            "uri": "axis-test-kit",
            "types": [
                {
                    "guid": "axis-test-kind",
                    "name": "Axis Test Kind",
                    "variant": "",
                    "attributes": [],
                    "connectors": [],
                    "models": [
                        {
                            "guid": "axis-test-model",
                            "file": {"guid": "axis-test-file"},
                            "tags": [],
                        }
                    ],
                }
            ],
            "designs": [
                {
                    "guid": "axis-test-design",
                    "name": "Axis Test Design",
                    "pieces": [
                        {
                            "guid": "axis-test-piece",
                            "name": "Axis Test Piece",
                            "type": {"guid": "axis-test-kind"},
                        }
                    ],
                    "connections": [],
                }
            ],
            "files": [
                {
                    "guid": "axis-test-file",
                    "name": "axis-test.glb",
                    "blob": _create_glb_blob(
                        [(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)],
                        [(0, 1, 2)],
                    ),
                }
            ],
            "tags": [],
            "authors": [],
        }

        result = export_design_model(kit_dict, "Axis Test Design", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        point_lists = ifc.by_type("IfcCartesianPointList3D")

        assert len(point_lists) == 1
        coordinates = [tuple(float(value) for value in row) for row in point_lists[0].CoordList]
        assert any(abs(x) < 1e-6 and abs(y) < 1e-6 and z > 0 for x, y, z in coordinates)
        assert any(abs(x) < 1e-6 and y < 0 and abs(z) < 1e-6 for x, y, z in coordinates)
        assert not any(abs(x) < 1e-6 and y > 0 and abs(z) < 1e-6 for x, y, z in coordinates)

    def test_export_ifc_contains_ports_and_connections(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCDISTRIBUTIONPORT" in ifc_text
        assert "IFCRELCONNECTSPORTS" in ifc_text
        assert "IFCRELCONNECTSELEMENTS" in ifc_text

    def test_export_ifc_roundtrip_with_ifcopenshell(self):
        import ifcopenshell

        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        projects = ifc.by_type("IfcProject")
        assert len(projects) == 1
        sites = ifc.by_type("IfcSite")
        assert len(sites) == 1
        buildings = ifc.by_type("IfcBuilding")
        assert len(buildings) == 1
        storeys = ifc.by_type("IfcBuildingStorey")
        assert len(storeys) == 1
        assemblies = ifc.by_type("IfcElementAssembly")
        assert len(assemblies) == 1
        assert assemblies[0].Name == "Nakagin Capsule Tower"
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        assert len(type_products) > 0
        occurrences = ifc.by_type("IfcBuildingElementProxy")
        assert len(occurrences) > 0
        pieces = next(d for d in kit_dict.get("designs", []) if d.get("name") == "Nakagin Capsule Tower").get("pieces", [])
        assert len(occurrences) == len(pieces)
        ports = ifc.by_type("IfcDistributionPort")
        assert len(ports) > 0
        port_connections = ifc.by_type("IfcRelConnectsPorts")
        connections = next(d for d in kit_dict.get("designs", []) if d.get("name") == "Nakagin Capsule Tower").get("connections", [])
        assert len(port_connections) == len(connections)

    def test_export_ifc_report(self):
        kit_dict = load_json("kit_metabolism.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.ifc").write_bytes(result)


class TestGetGeometricInsightsForModel:
    """Model/KPI tests for get_geometric_insights_for_model using nakagin-capsule-tower.gltf."""

    def test_nakagin_capsule_tower_gltf_returns_insights(self):
        model_path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, "nakagin-capsule-tower.gltf")
        if not os.path.exists(model_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        insights = get_geometric_insights_for_model(model_path)
        REPORTS_MODEL_KPI_DIR.mkdir(parents=True, exist_ok=True)
        data = geometric_insights_to_report_dict(insights)
        (REPORTS_MODEL_KPI_DIR / "py.json").write_text(json.dumps(data, indent=2, sort_keys=True), encoding="utf-8")

        canonical_path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, "model-kpi-nakagin.json")
        with open(canonical_path, "r", encoding="utf-8") as f:
            canonical = json.load(f)
        for key, expected in canonical.items():
            assert key in data, f"missing key {key}"
            assert data[key] == expected, f"mismatch for {key}: {data[key]!r} != {expected!r}"
        assert isinstance(insights, GeometricInsights)
        assert insights.bounding_box_min is not None
        assert insights.bounding_box_max is not None
        assert insights.dimension_x is not None and insights.dimension_x >= 0
        assert insights.dimension_y is not None and insights.dimension_y >= 0
        assert insights.dimension_z is not None and insights.dimension_z >= 0
        assert insights.characteristic_length is not None and insights.characteristic_length >= 0
        assert insights.total_surface_area is not None and insights.total_surface_area >= 0
        assert insights.vertex_count is not None and insights.vertex_count > 0
        assert insights.face_count is not None and insights.face_count > 0
        assert insights.centroid is not None
        assert insights.euler_characteristic is not None

    def test_nakagin_capsule_tower_from_bytes_gltf(self):
        model_path = os.path.join(os.path.dirname(__file__), ASSETS_DIR, "nakagin-capsule-tower.gltf")
        if not os.path.exists(model_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        with open(model_path, "rb") as f:
            data = f.read()
        insights = get_geometric_insights_for_model(data)
        assert isinstance(insights, GeometricInsights)
        assert insights.face_count is not None and insights.face_count > 0
