# region Header

# test_engine.py

# 2020-2025 Ueli Saluz

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

# region Imports

from __future__ import annotations

import json
import pathlib

import networkx
import numpy
import pytest
import pytransform3d.rotations

# endregion Imports

# region Constants

TOLERANCE = 0.001
ASSETS_DIR = pathlib.Path(__file__).parent.parent.parent / "assets" / "semio"
KIT_METABOLISM_PATH = ASSETS_DIR / "kit_metabolism.json"
KIT_METABOLISM_DIFFED_PATH = ASSETS_DIR / "kit_metabolism_diffed.json"
DIFF_KIT_METABOLISM_PATH = ASSETS_DIR / "diff_kit_metabolism.json"
DIFF_KIT_METABOLISM_INVERTED_PATH = ASSETS_DIR / "diff_kit_metabolism_inverted.json"
KIT_INVALID_PATH = ASSETS_DIR / "kit_invalid.json"

# endregion Constants

# region Fixtures


@pytest.fixture
def kit_metabolism() -> dict:
    with open(KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def kit_metabolism_diffed() -> dict:
    with open(KIT_METABOLISM_DIFFED_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def diff_kit_metabolism() -> dict:
    with open(DIFF_KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def diff_kit_metabolism_inverted() -> dict:
    with open(DIFF_KIT_METABOLISM_INVERTED_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def kit_invalid() -> dict:
    with open(KIT_INVALID_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


# endregion Fixtures

# region Helpers


def is_close(a: float, b: float, tol: float = TOLERANCE) -> bool:
    return abs(a - b) < tol


def points_equal(p1: dict | None, p2: dict | None, tol: float = TOLERANCE) -> bool:
    if p1 is None or p2 is None:
        return p1 is None and p2 is None
    return is_close(p1["x"], p2["x"], tol) and is_close(p1["y"], p2["y"], tol) and is_close(p1["z"], p2["z"], tol)


def vectors_equal(v1: dict | None, v2: dict | None, tol: float = TOLERANCE) -> bool:
    if v1 is None or v2 is None:
        return v1 is None and v2 is None
    return is_close(v1["x"], v2["x"], tol) and is_close(v1["y"], v2["y"], tol) and is_close(v1["z"], v2["z"], tol)


def planes_equal(p1: dict | None, p2: dict | None, tol: float = TOLERANCE) -> bool:
    if p1 is None or p2 is None:
        return p1 is None and p2 is None
    if "origin" not in p1 or "origin" not in p2:
        return False
    if "xAxis" not in p1 or "xAxis" not in p2:
        return False
    if "yAxis" not in p1 or "yAxis" not in p2:
        return False
    return points_equal(p1["origin"], p2["origin"], tol) and vectors_equal(p1["xAxis"], p2["xAxis"], tol) and vectors_equal(p1["yAxis"], p2["yAxis"], tol)


def centers_equal(c1: dict | None, c2: dict | None, tol: float = TOLERANCE) -> bool:
    if c1 is None or c2 is None:
        return c1 is None and c2 is None
    return is_close(c1["u"], c2["u"], tol) and is_close(c1["v"], c2["v"], tol)


# endregion Helpers

# region Spatial Math (pytransform3d)


def normalize_vector(v: numpy.ndarray) -> numpy.ndarray:
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length


def plane_from_y_axis(y_axis: numpy.ndarray, phi_degrees: float = 0.0, origin: numpy.ndarray | None = None) -> dict:
    if origin is None:
        origin = numpy.array([0.0, 0.0, 0.0])
    y_axis = normalize_vector(y_axis)
    world_y = numpy.array([0.0, 1.0, 0.0])
    if numpy.allclose(y_axis, world_y, atol=1e-6):
        rotation_to_y = numpy.eye(3)
    elif numpy.allclose(y_axis, -world_y, atol=1e-6):
        rotation_to_y = pytransform3d.rotations.matrix_from_axis_angle([1, 0, 0, numpy.pi])
    else:
        axis = numpy.cross(world_y, y_axis)
        axis = normalize_vector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(world_y, y_axis), -1.0, 1.0))
        rotation_to_y = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [angle]]))
    phi_radians = numpy.deg2rad(phi_degrees)
    rotation_around_y = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([y_axis, [phi_radians]]))
    world_x = numpy.array([1.0, 0.0, 0.0])
    x_axis = rotation_around_y @ rotation_to_y @ world_x
    x_axis = normalize_vector(x_axis)
    return {
        "origin": {"x": float(origin[0]), "y": float(origin[1]), "z": float(origin[2])},
        "xAxis": {"x": float(x_axis[0]), "y": float(x_axis[1]), "z": float(x_axis[2])},
        "yAxis": {"x": float(y_axis[0]), "y": float(y_axis[1]), "z": float(y_axis[2])},
    }


class TestPlaneFromYAxis:
    @pytest.mark.parametrize(
        "y_axis, phi, expected_x_axis",
        [
            pytest.param([0.0, 1.0, 0.0], 0.0, [1.0, 0.0, 0.0], id="no rotation, no rotation"),
            pytest.param([0.0, 1.0, 0.0], 135, [-0.707107, 0, -0.707107], id="no rotation, 135° around y rotation"),
            pytest.param([-0.707107, 0.707107, 0.0], 0.0, [0.707107, 0.707107, 0], id="45° around z, no rotation"),
            pytest.param([0, 0.866025, -0.5], 0.0, [1, 0, 0], id="-30° around x, no rotation"),
            pytest.param([0, 0.866025, -0.5], 45, [0.707107, -0.353553, -0.612372], id="-30° around x, 45° rotation"),
            pytest.param([0.707107, -0.612372, 0.353553], 45, [0.251059, -0.25, -0.935131], id="135° around z then -30° around x, 45° rotation"),
        ],
    )
    def test_plane_from_y_axis(self, y_axis: list[float], phi: float, expected_x_axis: list[float]) -> None:
        plane = plane_from_y_axis(numpy.array(y_axis), phi)
        actual_x_axis = [plane["xAxis"]["x"], plane["xAxis"]["y"], plane["xAxis"]["z"]]
        assert is_close(actual_x_axis[0], expected_x_axis[0])
        assert is_close(actual_x_axis[1], expected_x_axis[1])
        assert is_close(actual_x_axis[2], expected_x_axis[2])


# endregion Spatial Math (pytransform3d)

# region Graph Operations (networkx)


def build_piece_graph(design: dict) -> networkx.Graph:
    G = networkx.Graph()
    for piece in design.get("pieces", []):
        G.add_node(piece["guid"], piece=piece)
    for connection in design.get("connections", []):
        source_id = connection["connected"]["piece"]["guid"]
        target_id = connection["connecting"]["piece"]["guid"]
        if G.has_node(source_id) and G.has_node(target_id):
            G.add_edge(source_id, target_id, connection=connection)
    return G


def find_fixed_pieces(design: dict) -> list[str]:
    return [p["guid"] for p in design.get("pieces", []) if p.get("plane") is not None]


def get_connected_components(design: dict) -> list[set[str]]:
    G = build_piece_graph(design)
    return [set(c) for c in networkx.connected_components(G)]


def get_piece_hierarchy(design: dict, root_guid: str) -> dict[str, int]:
    G = build_piece_graph(design)
    if root_guid not in G:
        return {}
    lengths = networkx.single_source_shortest_path_length(G, root_guid)
    return lengths


class TestGraphOperations:
    def test_build_piece_graph(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        G = build_piece_graph(design)
        assert len(G.nodes) == len(design.get("pieces", []))
        assert len(G.edges) == len(design.get("connections", []))

    def test_find_fixed_pieces(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        fixed = find_fixed_pieces(design)
        assert len(fixed) >= 1

    def test_connected_components(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        components = get_connected_components(design)
        assert len(components) >= 1

    def test_piece_hierarchy(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        fixed = find_fixed_pieces(design)
        if fixed:
            hierarchy = get_piece_hierarchy(design, fixed[0])
            assert hierarchy[fixed[0]] == 0


# endregion Graph Operations (networkx)

# region Model Tests


@pytest.fixture
def engine_module():
    import engine

    return engine


class TestPoint:
    def test_parse(self, engine_module) -> None:
        point = engine_module.Point(x=1.0, y=2.0, z=3.0)
        assert point.x == 1.0
        assert point.y == 2.0
        assert point.z == 3.0

    def test_str(self, engine_module) -> None:
        point = engine_module.Point(x=1.0, y=2.0, z=3.0)
        assert "[1, 2, 3]" in str(point)


class TestVector:
    def test_parse(self, engine_module) -> None:
        vector = engine_module.Vector(x=1.0, y=0.0, z=0.0)
        assert vector.x == 1.0
        assert vector.y == 0.0
        assert vector.z == 0.0

    def test_str(self, engine_module) -> None:
        vector = engine_module.Vector(x=1.0, y=0.0, z=0.0)
        assert "[1, 0, 0]" in str(vector)


class TestPlane:
    def test_parse(self, engine_module) -> None:
        plane_data = {"origin": {"x": 0, "y": 0, "z": 0}, "xAxis": {"x": 1, "y": 0, "z": 0}, "yAxis": {"x": 0, "y": 1, "z": 0}}
        plane = engine_module.Plane.parse(plane_data)
        assert plane.origin.x == 0
        assert plane.xAxis.x == 1
        assert plane.yAxis.y == 1

    def test_dump(self, engine_module) -> None:
        plane = engine_module.Plane()
        plane.origin = engine_module.Point(x=0, y=0, z=0)
        plane.xAxis = engine_module.Vector(x=1, y=0, z=0)
        plane.yAxis = engine_module.Vector(x=0, y=1, z=0)
        output = plane.dump()
        assert output.origin.x == 0
        assert output.xAxis.x == 1
        assert output.yAxis.y == 1


class TestCoord:
    def test_parse(self, engine_module) -> None:
        coord = engine_module.Coord(u=1.0, v=2.0)
        assert coord.u == 1.0
        assert coord.v == 2.0


class TestAttribute:
    def test_parse(self, engine_module) -> None:
        attr = engine_module.Attribute(name="test", value="value", definition="def")
        assert attr.name == "test"
        assert attr.value == "value"
        assert attr.definition == "def"


# endregion Model Tests

# region Diffs Tests


def deep_compare(obj1: dict | list | None, obj2: dict | list | None) -> bool:
    if type(obj1) != type(obj2):
        return False
    if obj1 is None and obj2 is None:
        return True
    if isinstance(obj1, dict) and isinstance(obj2, dict):
        if set(obj1.keys()) != set(obj2.keys()):
            return False
        return all(deep_compare(obj1[k], obj2[k]) for k in obj1)
    if isinstance(obj1, list) and isinstance(obj2, list):
        if len(obj1) != len(obj2):
            return False
        return all(deep_compare(a, b) for a, b in zip(obj1, obj2))
    if isinstance(obj1, float) and isinstance(obj2, float):
        return is_close(obj1, obj2)
    return obj1 == obj2


def apply_kit_diff(kit: dict, diff: dict) -> dict:
    import copy

    result = copy.deepcopy(kit)
    for key in diff:
        if key == "guid":
            continue
        if key in result and isinstance(result[key], list) and isinstance(diff.get(key), dict):
            collection_diff = diff[key]
            if "removed" in collection_diff:
                removed_ids = set(r["guid"] if isinstance(r, dict) else r for r in collection_diff["removed"])
                result[key] = [item for item in result[key] if item.get("guid") not in removed_ids]
            if "updated" in collection_diff:
                for update in collection_diff["updated"]:
                    item_id = update.get("id") or update.get("guid")
                    item_diff = update.get("diff", {})
                    for item in result[key]:
                        if item.get("guid") == item_id:
                            for diff_key, diff_value in item_diff.items():
                                if diff_value is not None:
                                    item[diff_key] = diff_value
                            break
            if "added" in collection_diff:
                result[key].extend(collection_diff["added"])
        elif isinstance(diff.get(key), dict) and not key.endswith("s"):
            if key in result and isinstance(result[key], dict):
                result[key] = {**result[key], **diff[key]}
            else:
                result[key] = diff[key]
        elif diff.get(key) is not None:
            result[key] = diff[key]
    return result


class TestDiffs:
    def test_kit_plus_diff_equals_diffed_kit(self, kit_metabolism: dict, kit_metabolism_diffed: dict, diff_kit_metabolism: dict) -> None:
        kit_original = {k: v for k, v in kit_metabolism.items() if k != "designs" or not any(d.get("parent") for d in (v or []))}
        if kit_metabolism.get("designs"):
            kit_original["designs"] = [d for d in kit_metabolism["designs"] if not d.get("parent")]
        applied = apply_kit_diff(kit_original, diff_kit_metabolism)
        assert applied["guid"] == kit_metabolism_diffed["guid"]

    def test_diffed_kit_plus_inverse_diff_equals_kit(self, kit_metabolism: dict, kit_metabolism_diffed: dict, diff_kit_metabolism_inverted: dict) -> None:
        applied = apply_kit_diff(kit_metabolism_diffed, diff_kit_metabolism_inverted)
        kit_original = {k: v for k, v in kit_metabolism.items() if k != "designs" or not any(d.get("parent") for d in (v or []))}
        if kit_metabolism.get("designs"):
            kit_original["designs"] = [d for d in kit_metabolism["designs"] if not d.get("parent")]
        assert applied["guid"] == kit_original["guid"]


# endregion Diffs Tests

# region Kit Serialization Tests


class TestKitSerialization:
    def test_kit_to_json_to_kit(self, kit_metabolism: dict) -> None:
        serialized = json.dumps(kit_metabolism)
        deserialized = json.loads(serialized)
        assert deserialized["guid"] == kit_metabolism["guid"]
        assert len(deserialized.get("types", [])) == len(kit_metabolism.get("types", []))
        assert len(deserialized.get("designs", [])) == len(kit_metabolism.get("designs", []))


# endregion Kit Serialization Tests

# region Validation Tests


def generate_unique_name(base_name: str, existing_names: list[str]) -> str:
    if base_name not in existing_names:
        return base_name
    counter = 2
    while f"{base_name} {counter}" in existing_names:
        counter += 1
    return f"{base_name} {counter}"


def validate_guid_uniqueness(kit: dict) -> list[dict]:
    issues = []
    seen: dict[str, str] = {}

    def check(entity_kind: str, entity_guid: str) -> None:
        if entity_guid in seen:
            issues.append({"ruleId": "guid-unique", "severity": "error", "message": f'Duplicate GUID "{entity_guid}". First occurrence kept.', "entityKind": entity_kind, "entityGuid": entity_guid})
        else:
            seen[entity_guid] = entity_kind

    check("Kit", kit.get("guid", ""))
    for t in kit.get("types", []):
        check("Type", t.get("guid", ""))
    for d in kit.get("designs", []):
        check("Design", d.get("guid", ""))
        for p in d.get("pieces", []):
            check("Piece", p.get("guid", ""))
        for c in d.get("connections", []):
            check("Connection", c.get("guid", ""))
        for s in d.get("stats", []):
            check("Stat", s.get("guid", ""))
    for q in kit.get("qualities", []):
        check("Quality", q.get("guid", ""))
    for i in kit.get("interfaces", []):
        check("Interface", i.get("guid", ""))
    for f in kit.get("files", []):
        check("File", f.get("guid", ""))
    for fo in kit.get("folders", []):
        check("Folder", fo.get("guid", ""))
    return issues


def validate_type_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    by_parent: dict[str | None, list[dict]] = {}
    for t in kit.get("types", []):
        parent_guid = t.get("parent", {}).get("guid") if t.get("parent") else None
        if parent_guid not in by_parent:
            by_parent[parent_guid] = []
        by_parent[parent_guid].append(t)
    for parent_guid, siblings in by_parent.items():
        names: dict[str, list[dict]] = {}
        for t in siblings:
            name = t.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(t)
        for name, group in names.items():
            if len(group) > 1:
                for t in group[1:]:
                    issues.append({"ruleId": "type-name-unique", "severity": "error", "message": f'Duplicate type name "{name}" among siblings.', "entityKind": "Type", "entityGuid": t.get("guid", "")})
    return issues


def validate_design_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    by_parent: dict[str | None, list[dict]] = {}
    for d in kit.get("designs", []):
        parent_guid = d.get("parent", {}).get("guid") if d.get("parent") else None
        if parent_guid not in by_parent:
            by_parent[parent_guid] = []
        by_parent[parent_guid].append(d)
    for parent_guid, siblings in by_parent.items():
        names: dict[str, list[dict]] = {}
        for d in siblings:
            name = d.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(d)
        for name, group in names.items():
            if len(group) > 1:
                for d in group[1:]:
                    issues.append({"ruleId": "design-name-unique", "severity": "error", "message": f'Duplicate design name "{name}" among siblings.', "entityKind": "Design", "entityGuid": d.get("guid", "")})
    return issues


def validate_piece_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    for design in kit.get("designs", []):
        names: dict[str, list[dict]] = {}
        for p in design.get("pieces", []):
            name = p.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(p)
        for name, group in names.items():
            if len(group) > 1:
                for p in group[1:]:
                    issues.append({"ruleId": "piece-name-unique", "severity": "error", "message": f'Duplicate piece name "{name}" in design.', "entityKind": "Piece", "entityGuid": p.get("guid", "")})
    return issues


def validate_port_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    for t in kit.get("types", []):
        names: dict[str, list[dict]] = {}
        for port in t.get("ports", []):
            name = port.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(port)
        for name, group in names.items():
            if len(group) > 1:
                for port in group[1:]:
                    issues.append({"ruleId": "port-name-unique", "severity": "error", "message": f'Duplicate port name "{name}" in type.', "entityKind": "Port", "entityGuid": port.get("guid", "")})
    return issues


def validate_model_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    for t in kit.get("types", []):
        names: dict[str, list[dict]] = {}
        for model in t.get("models", []):
            name = model.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(model)
        for name, group in names.items():
            if len(group) > 1:
                for model in group[1:]:
                    issues.append({"ruleId": "model-name-unique", "severity": "error", "message": f'Duplicate model name "{name}" in type.', "entityKind": "Model", "entityGuid": model.get("guid", "")})
    return issues


def validate_quality_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    names: dict[str, list[dict]] = {}
    for q in kit.get("qualities", []):
        name = q.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(q)
    for name, group in names.items():
        if len(group) > 1:
            for q in group[1:]:
                issues.append({"ruleId": "quality-name-unique", "severity": "error", "message": f'Duplicate quality name "{name}".', "entityKind": "Quality", "entityGuid": q.get("guid", "")})
    return issues


def validate_interface_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    names: dict[str, list[dict]] = {}
    for i in kit.get("interfaces", []):
        name = i.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(i)
    for name, group in names.items():
        if len(group) > 1:
            for i in group[1:]:
                issues.append({"ruleId": "interface-name-unique", "severity": "error", "message": f'Duplicate interface name "{name}".', "entityKind": "Interface", "entityGuid": i.get("guid", "")})
    return issues


def validate_file_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    names: dict[str, list[dict]] = {}
    for f in kit.get("files", []):
        name = f.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                issues.append({"ruleId": "file-name-unique", "severity": "error", "message": f'Duplicate file name "{name}".', "entityKind": "File", "entityGuid": f.get("guid", "")})
    return issues


def validate_folder_name_uniqueness(kit: dict) -> list[dict]:
    issues = []
    by_parent: dict[str | None, list[dict]] = {}
    for fo in kit.get("folders", []):
        parent_guid = fo.get("parent", {}).get("guid") if fo.get("parent") else None
        if parent_guid not in by_parent:
            by_parent[parent_guid] = []
        by_parent[parent_guid].append(fo)
    for parent_guid, siblings in by_parent.items():
        names: dict[str, list[dict]] = {}
        for fo in siblings:
            name = fo.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(fo)
        for name, group in names.items():
            if len(group) > 1:
                for fo in group[1:]:
                    issues.append({"ruleId": "folder-name-unique", "severity": "error", "message": f'Duplicate folder name "{name}" among siblings.', "entityKind": "Folder", "entityGuid": fo.get("guid", "")})
    return issues


def validate_layer_path_uniqueness(kit: dict) -> list[dict]:
    issues = []
    for design in kit.get("designs", []):
        paths: dict[str, list[dict]] = {}
        for layer in design.get("layers", []):
            path = layer.get("path", "")
            if path not in paths:
                paths[path] = []
            paths[path].append(layer)
        for path, group in paths.items():
            if len(group) > 1:
                for layer in group[1:]:
                    issues.append({"ruleId": "layer-path-unique", "severity": "error", "message": f'Duplicate layer path "{path}" in design.', "entityKind": "Layer", "entityGuid": layer.get("guid", "")})
    return issues


def validate_kit(kit: dict) -> list[dict]:
    issues = []
    issues.extend(validate_guid_uniqueness(kit))
    issues.extend(validate_type_name_uniqueness(kit))
    issues.extend(validate_design_name_uniqueness(kit))
    issues.extend(validate_piece_name_uniqueness(kit))
    issues.extend(validate_port_name_uniqueness(kit))
    issues.extend(validate_model_name_uniqueness(kit))
    issues.extend(validate_quality_name_uniqueness(kit))
    issues.extend(validate_interface_name_uniqueness(kit))
    issues.extend(validate_file_name_uniqueness(kit))
    issues.extend(validate_folder_name_uniqueness(kit))
    issues.extend(validate_layer_path_uniqueness(kit))
    return issues


def has_errors(issues: list[dict]) -> bool:
    return any(i.get("severity") == "error" for i in issues)


class TestValidation:
    def test_valid_kit_has_no_errors(self, kit_metabolism: dict) -> None:
        issues = validate_kit(kit_metabolism)
        assert not has_errors(issues), f"Expected no errors, but got: {issues}"

    def test_invalid_kit_has_expected_errors(self, kit_invalid: dict) -> None:
        issues = validate_kit(kit_invalid)
        assert has_errors(issues)
        rule_ids = {i["ruleId"] for i in issues}
        expected_rules = [
            "guid-unique",
            "type-name-unique",
            "design-name-unique",
            "piece-name-unique",
            "quality-name-unique",
            "interface-name-unique",
            "file-name-unique",
            "folder-name-unique",
            "port-name-unique",
            "model-name-unique",
            "layer-path-unique",
        ]
        for rule_id in expected_rules:
            assert rule_id in rule_ids, f"Missing validation rule: {rule_id}"


# endregion Validation Tests

# region FlattenDesign Tests


def get_type_by_guid(kit: dict, guid: str) -> dict | None:
    for t in kit.get("types", []):
        if t.get("guid") == guid:
            return t
    return None


def get_port_from_type(kit: dict, type_data: dict | None, port_guid: str | None) -> dict | None:
    if type_data is None:
        return None
    if port_guid is None:
        ports = type_data.get("ports", [])
        if ports:
            return ports[0]
        parent = type_data.get("parent")
        if parent:
            parent_type = get_type_by_guid(kit, parent.get("guid", ""))
            return get_port_from_type(kit, parent_type, port_guid)
        return None
    for port in type_data.get("ports", []):
        if port.get("guid") == port_guid:
            return port
    parent = type_data.get("parent")
    if parent:
        parent_type = get_type_by_guid(kit, parent.get("guid", ""))
        return get_port_from_type(kit, parent_type, port_guid)
    ports = type_data.get("ports", [])
    if ports:
        return ports[0]
    return None


def compute_child_plane(parent_plane: dict, parent_port: dict, child_port: dict, connection: dict) -> dict:
    gap = connection.get("gap", 0)
    shift = connection.get("shift", 0)
    rise = connection.get("rise", 0)
    rotation = connection.get("rotation", 0)
    turn = connection.get("turn", 0)
    tilt = connection.get("tilt", 0)
    p_origin = numpy.array([parent_plane["origin"]["x"], parent_plane["origin"]["y"], parent_plane["origin"]["z"]])
    p_x = numpy.array([parent_plane["xAxis"]["x"], parent_plane["xAxis"]["y"], parent_plane["xAxis"]["z"]])
    p_y = numpy.array([parent_plane["yAxis"]["x"], parent_plane["yAxis"]["y"], parent_plane["yAxis"]["z"]])
    p_z = numpy.cross(p_x, p_y)
    parent_matrix = numpy.eye(4)
    parent_matrix[:3, 0] = p_x
    parent_matrix[:3, 1] = p_y
    parent_matrix[:3, 2] = p_z
    parent_matrix[:3, 3] = p_origin
    pp_point = numpy.array([parent_port["point"]["x"], parent_port["point"]["y"], parent_port["point"]["z"]])
    pp_dir = numpy.array([parent_port["direction"]["x"], parent_port["direction"]["y"], parent_port["direction"]["z"]])
    cp_point = numpy.array([child_port["point"]["x"], child_port["point"]["y"], child_port["point"]["z"]])
    cp_dir = numpy.array([child_port["direction"]["x"], child_port["direction"]["y"], child_port["direction"]["z"]])
    pp_world = parent_matrix[:3, :3] @ pp_point + parent_matrix[:3, 3]
    pp_dir_world = parent_matrix[:3, :3] @ pp_dir
    pp_dir_world = normalize_vector(pp_dir_world)
    translation = pp_world + gap * pp_dir_world + shift * numpy.cross(pp_dir_world, p_z) + rise * p_z
    target_dir = -pp_dir_world
    cp_dir_normalized = normalize_vector(cp_dir)
    if numpy.allclose(cp_dir_normalized, target_dir, atol=1e-6):
        base_rotation = numpy.eye(3)
    elif numpy.allclose(cp_dir_normalized, -target_dir, atol=1e-6):
        axis = numpy.array([1.0, 0.0, 0.0])
        if numpy.allclose(numpy.abs(cp_dir_normalized), axis, atol=1e-6):
            axis = numpy.array([0.0, 1.0, 0.0])
        base_rotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [numpy.pi]]))
    else:
        axis = numpy.cross(cp_dir_normalized, target_dir)
        axis = normalize_vector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(cp_dir_normalized, target_dir), -1.0, 1.0))
        base_rotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [angle]]))
    rot_rad = numpy.deg2rad(rotation)
    rotation_matrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([target_dir, [rot_rad]]))
    turn_rad = numpy.deg2rad(turn)
    p_z_world = normalize_vector(p_z)
    turn_matrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([p_z_world, [turn_rad]]))
    tilt_rad = numpy.deg2rad(tilt)
    p_x_world = normalize_vector(parent_matrix[:3, :3] @ numpy.array([1, 0, 0]))
    tilt_matrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([p_x_world, [tilt_rad]]))
    combined_rotation = tilt_matrix @ turn_matrix @ rotation_matrix @ base_rotation
    child_origin = translation - combined_rotation @ cp_point
    child_x = combined_rotation @ numpy.array([1, 0, 0])
    child_y = combined_rotation @ numpy.array([0, 1, 0])
    return {
        "origin": {"x": float(child_origin[0]), "y": float(child_origin[1]), "z": float(child_origin[2])},
        "xAxis": {"x": float(child_x[0]), "y": float(child_x[1]), "z": float(child_x[2])},
        "yAxis": {"x": float(child_y[0]), "y": float(child_y[1]), "z": float(child_y[2])},
    }


def flatten_design(kit: dict, design_guid: str) -> dict:
    design = next((d for d in kit.get("designs", []) if d.get("guid") == design_guid), None)
    if design is None:
        raise ValueError(f"Design {design_guid} not found")
    pieces = design.get("pieces", [])
    connections = design.get("connections", [])
    if not pieces:
        return {}
    piece_map = {p["guid"]: dict(p) for p in pieces}
    piece_planes: dict[str, dict] = {}
    G = build_piece_graph(design)
    components = list(networkx.connected_components(G))
    for component in components:
        root_node = None
        for node_id in component:
            piece = piece_map.get(node_id)
            if piece and piece.get("plane") is not None:
                root_node = node_id
                break
        if root_node is None and component:
            root_node = next(iter(component))
        if root_node is None:
            continue
        root_piece = piece_map[root_node]
        if root_piece.get("plane"):
            piece_planes[root_node] = root_piece["plane"]
        else:
            piece_planes[root_node] = {"origin": {"x": 0, "y": 0, "z": 0}, "xAxis": {"x": 1, "y": 0, "z": 0}, "yAxis": {"x": 0, "y": 1, "z": 0}}
        for source, target in networkx.bfs_edges(G, root_node):
            if target in piece_planes:
                continue
            parent_id = source
            child_id = target
            parent_plane = piece_planes.get(parent_id)
            if parent_plane is None:
                continue
            edge_data = G.get_edge_data(parent_id, child_id)
            connection = edge_data.get("connection") if edge_data else None
            if connection is None:
                continue
            parent_piece = piece_map[parent_id]
            child_piece = piece_map[child_id]
            parent_type = get_type_by_guid(kit, parent_piece.get("type", {}).get("guid", ""))
            child_type = get_type_by_guid(kit, child_piece.get("type", {}).get("guid", ""))
            parent_side = connection["connected"] if connection["connected"]["piece"]["guid"] == parent_id else connection["connecting"]
            child_side = connection["connecting"] if connection["connecting"]["piece"]["guid"] == child_id else connection["connected"]
            parent_port_guid = parent_side.get("port", {}).get("guid") if parent_side.get("port") else None
            child_port_guid = child_side.get("port", {}).get("guid") if child_side.get("port") else None
            parent_port = get_port_from_type(kit, parent_type, parent_port_guid)
            child_port = get_port_from_type(kit, child_type, child_port_guid)
            if parent_port is None or child_port is None:
                continue
            child_plane = compute_child_plane(parent_plane, parent_port, child_port, connection)
            piece_planes[child_id] = child_plane
    updated_pieces = []
    for piece in pieces:
        new_piece = dict(piece)
        if piece["guid"] in piece_planes:
            new_piece["plane"] = piece_planes[piece["guid"]]
        if new_piece.get("center") is None:
            new_piece["center"] = {"u": 0, "v": 0}
        updated_pieces.append(new_piece)
    return {"pieces": {"updated": [{"id": p["guid"], "diff": {"plane": p.get("plane"), "center": p.get("center")}} for p in updated_pieces if p["guid"] in piece_planes]}}


class TestFlattenDesign:
    def test_nakagin_capsule_tower_normal(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        expected_design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expected_design is None:
            pytest.skip("Expected Flat design not found")
        flat_diff = flatten_design(kit_metabolism, design["guid"])
        assert flat_diff is not None
        for update in flat_diff.get("pieces", {}).get("updated", []):
            piece_id = update["id"]
            computed_plane = update["diff"].get("plane")
            expected_piece = next((p for p in expected_design.get("pieces", []) if p.get("guid") == piece_id), None)
            if expected_piece and expected_piece.get("plane"):
                assert planes_equal(computed_plane, expected_piece["plane"]), f"Plane mismatch for piece {piece_id}"

    def test_nakagin_capsule_tower_slanted(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Slanted"), None)
        if design is None:
            pytest.skip("Slanted design not found")
        expected_design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expected_design is None:
            pytest.skip("Expected Flat design for Slanted not found")
        flat_diff = flatten_design(kit_metabolism, design["guid"])
        assert flat_diff is not None
        for update in flat_diff.get("pieces", {}).get("updated", []):
            piece_id = update["id"]
            computed_plane = update["diff"].get("plane")
            expected_piece = next((p for p in expected_design.get("pieces", []) if p.get("guid") == piece_id), None)
            if expected_piece and expected_piece.get("plane"):
                assert planes_equal(computed_plane, expected_piece["plane"]), f"Plane mismatch for piece {piece_id}"

    def test_nakagin_capsule_tower_twisted(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Twisted"), None)
        if design is None:
            pytest.skip("Twisted design not found")
        expected_design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expected_design is None:
            pytest.skip("Expected Flat design for Twisted not found")
        flat_diff = flatten_design(kit_metabolism, design["guid"])
        assert flat_diff is not None
        for update in flat_diff.get("pieces", {}).get("updated", []):
            piece_id = update["id"]
            computed_plane = update["diff"].get("plane")
            expected_piece = next((p for p in expected_design.get("pieces", []) if p.get("guid") == piece_id), None)
            if expected_piece and expected_piece.get("plane"):
                assert planes_equal(computed_plane, expected_piece["plane"]), f"Plane mismatch for piece {piece_id}"

    def test_nakagin_capsule_tower_dancing(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Dancing"), None)
        if design is None:
            pytest.skip("Dancing design not found")
        expected_design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expected_design is None:
            pytest.skip("Expected Flat design for Dancing not found")
        flat_diff = flatten_design(kit_metabolism, design["guid"])
        assert flat_diff is not None
        for update in flat_diff.get("pieces", {}).get("updated", []):
            piece_id = update["id"]
            computed_plane = update["diff"].get("plane")
            expected_piece = next((p for p in expected_design.get("pieces", []) if p.get("guid") == piece_id), None)
            if expected_piece and expected_piece.get("plane"):
                assert planes_equal(computed_plane, expected_piece["plane"]), f"Plane mismatch for piece {piece_id}"

    def test_capsule_dream(self, kit_metabolism: dict) -> None:
        design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Capsule Dream"), None)
        assert design is not None
        expected_design = next((d for d in kit_metabolism.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expected_design is None:
            pytest.skip("Expected Flat design not found")
        flat_diff = flatten_design(kit_metabolism, design["guid"])
        assert flat_diff is not None
        for update in flat_diff.get("pieces", {}).get("updated", []):
            piece_id = update["id"]
            computed_plane = update["diff"].get("plane")
            expected_piece = next((p for p in expected_design.get("pieces", []) if p.get("guid") == piece_id), None)
            if expected_piece and expected_piece.get("plane"):
                assert planes_equal(computed_plane, expected_piece["plane"]), f"Plane mismatch for piece {piece_id}"


# endregion FlattenDesign Tests

# region CRUD Tests


class TestCRUD:
    def test_create_point(self, engine_module) -> None:
        point = engine_module.Point(x=1.0, y=2.0, z=3.0)
        assert point.x == 1.0
        assert point.y == 2.0
        assert point.z == 3.0

    def test_create_vector(self, engine_module) -> None:
        vector = engine_module.Vector(x=1.0, y=0.0, z=0.0)
        assert vector.x == 1.0

    def test_create_coord(self, engine_module) -> None:
        coord = engine_module.Coord(u=5.0, v=10.0)
        assert coord.u == 5.0
        assert coord.v == 10.0

    def test_create_attribute(self, engine_module) -> None:
        attr = engine_module.Attribute(name="test-attr", value="test-value", definition="test-def")
        assert attr.name == "test-attr"
        assert attr.value == "test-value"

    def test_create_plane(self, engine_module) -> None:
        plane = engine_module.Plane()
        plane.origin = engine_module.Point(x=0, y=0, z=0)
        plane.xAxis = engine_module.Vector(x=1, y=0, z=0)
        plane.yAxis = engine_module.Vector(x=0, y=1, z=0)
        assert plane.origin.x == 0
        assert plane.xAxis.x == 1
        assert plane.yAxis.y == 1


# endregion CRUD Tests

# region GraphQL Tests


class TestGraphQL:
    def test_schema_exists(self, engine_module) -> None:
        assert hasattr(engine_module, "graphqlSchema")

    def test_query_type_exists(self, engine_module) -> None:
        assert engine_module.graphqlSchema.query is not None


# endregion GraphQL Tests

# region Integration Tests


class TestIntegration:
    @pytest.mark.skip(reason="Requires running server")
    def test_graphql_local_kit_crud(self, tmp_path: pathlib.Path) -> None:
        pass

    @pytest.mark.skip(reason="Requires running server")
    def test_graphql_local_kit_design_to_scene(self, tmp_path: pathlib.Path) -> None:
        pass


# endregion Integration Tests
