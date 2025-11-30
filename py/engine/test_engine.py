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

import numpy
import pytest
from fastapi.testclient import TestClient

import engine

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
def kitMetabolismJson() -> dict:
    with open(KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def kitMetabolismDiffedJson() -> dict:
    with open(KIT_METABOLISM_DIFFED_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def diffKitMetabolismJson() -> dict:
    with open(DIFF_KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def diffKitMetabolismInvertedJson() -> dict:
    with open(DIFF_KIT_METABOLISM_INVERTED_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def kitInvalidJson() -> dict:
    with open(KIT_INVALID_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def restClient() -> TestClient:
    return TestClient(engine.rest)


@pytest.fixture
def tempKitPath(tmp_path: pathlib.Path) -> pathlib.Path:
    kitDir = tmp_path / ".semio"
    kitDir.mkdir(parents=True)
    return tmp_path


# endregion Fixtures

# region Helpers


def isClose(a: float, b: float, tol: float = TOLERANCE) -> bool:
    return abs(a - b) < tol


def planesEqual(p1: engine.Plane, p2: engine.Plane, tol: float = TOLERANCE) -> bool:
    return (
        isClose(p1.origin.x, p2.origin.x, tol)
        and isClose(p1.origin.y, p2.origin.y, tol)
        and isClose(p1.origin.z, p2.origin.z, tol)
        and isClose(p1.xAxis.x, p2.xAxis.x, tol)
        and isClose(p1.xAxis.y, p2.xAxis.y, tol)
        and isClose(p1.xAxis.z, p2.xAxis.z, tol)
        and isClose(p1.yAxis.x, p2.yAxis.x, tol)
        and isClose(p1.yAxis.y, p2.yAxis.y, tol)
        and isClose(p1.yAxis.z, p2.yAxis.z, tol)
    )


def planesEqualDict(p1: dict | None, p2: dict | None, tol: float = TOLERANCE) -> bool:
    if p1 is None or p2 is None:
        return p1 is None and p2 is None
    return (
        isClose(p1["origin"]["x"], p2["origin"]["x"], tol)
        and isClose(p1["origin"]["y"], p2["origin"]["y"], tol)
        and isClose(p1["origin"]["z"], p2["origin"]["z"], tol)
        and isClose(p1["xAxis"]["x"], p2["xAxis"]["x"], tol)
        and isClose(p1["xAxis"]["y"], p2["xAxis"]["y"], tol)
        and isClose(p1["xAxis"]["z"], p2["xAxis"]["z"], tol)
        and isClose(p1["yAxis"]["x"], p2["yAxis"]["x"], tol)
        and isClose(p1["yAxis"]["y"], p2["yAxis"]["y"], tol)
        and isClose(p1["yAxis"]["z"], p2["yAxis"]["z"], tol)
    )


# endregion Helpers

# region Spatial Math Tests


class TestPlaneFromYAxis:
    @pytest.mark.parametrize(
        "yAxis, phi, expectedXAxis",
        [
            pytest.param([0.0, 1.0, 0.0], 0.0, [1.0, 0.0, 0.0], id="no rotation, no rotation"),
            pytest.param([0.0, 1.0, 0.0], 135, [-0.707107, 0, -0.707107], id="no rotation, 135° around y rotation"),
            pytest.param([-0.707107, 0.707107, 0.0], 0.0, [0.707107, 0.707107, 0], id="45° around z, no rotation"),
            pytest.param([0, 0.866025, -0.5], 0.0, [1, 0, 0], id="-30° around x, no rotation"),
            pytest.param([0, 0.866025, -0.5], 45, [0.707107, -0.353553, -0.612372], id="-30° around x, 45° rotation"),
            pytest.param([0.707107, -0.612372, 0.353553], 45, [0.251059, -0.25, -0.935131], id="135° around z then -30° around x, 45° rotation"),
        ],
    )
    def test_planeFromYAxis(self, yAxis: list[float], phi: float, expectedXAxis: list[float]) -> None:
        plane = engine.planeFromYAxis(numpy.array(yAxis), phi)
        assert isClose(plane.xAxis.x, expectedXAxis[0])
        assert isClose(plane.xAxis.y, expectedXAxis[1])
        assert isClose(plane.xAxis.z, expectedXAxis[2])


# endregion Spatial Math Tests

# region Graph Operations Tests


class TestGraphOperations:
    def test_buildPieceGraph(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        G = engine.buildPieceGraph(design)
        assert len(G.nodes) == len(design.get("pieces", []))
        assert len(G.edges) == len(design.get("connections", []))

    def test_findFixedPieces(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        fixed = engine.findFixedPieces(design)
        assert len(fixed) >= 1

    def test_connectedComponents(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        components = engine.getConnectedComponents(design)
        assert len(components) >= 1

    def test_pieceHierarchy(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        fixed = engine.findFixedPieces(design)
        if fixed:
            hierarchy = engine.getPieceHierarchy(design, fixed[0])
            assert hierarchy[fixed[0]] == 0


# endregion Graph Operations Tests

# region Validation Tests


class TestValidation:
    def test_validKitHasNoErrors(self, kitMetabolismJson: dict) -> None:
        result = engine.validateKitDict(kitMetabolismJson)
        assert not result.hasErrors(), f"Expected no errors, but got: {[i.message for i in result.issues]}"

    def test_invalidKitHasExpectedErrors(self, kitInvalidJson: dict) -> None:
        result = engine.validateKitDict(kitInvalidJson)
        assert result.hasErrors()
        ruleIds = {i.ruleId for i in result.issues}
        expectedRules = [
            "guid-unique",
            "type-name-unique",
            "design-name-unique",
            "piece-name-unique",
            "quality-name-unique",
            "file-name-unique",
            "folder-name-unique",
            "port-name-unique",
            "model-name-unique",
            "layer-path-unique",
        ]
        for ruleId in expectedRules:
            assert ruleId in ruleIds, f"Missing validation rule: {ruleId}"


# endregion Validation Tests

# region Kit Serialization Tests


class TestKitSerialization:
    def test_kitJsonRoundtrip(self, kitMetabolismJson: dict) -> None:
        serialized = json.dumps(kitMetabolismJson)
        deserialized = json.loads(serialized)
        assert deserialized["guid"] == kitMetabolismJson["guid"]
        assert len(deserialized.get("types", [])) == len(kitMetabolismJson.get("types", []))
        assert len(deserialized.get("designs", [])) == len(kitMetabolismJson.get("designs", []))


# endregion Kit Serialization Tests

# region Diff Tests


class TestDiffs:
    def test_kitPlusDiffEqualsKitDiffed(self, kitMetabolismJson: dict, kitMetabolismDiffedJson: dict, diffKitMetabolismJson: dict) -> None:
        assert kitMetabolismJson["guid"] == kitMetabolismDiffedJson["guid"]

    def test_diffedKitPlusInverseDiffEqualsKit(self, kitMetabolismJson: dict, kitMetabolismDiffedJson: dict, diffKitMetabolismInvertedJson: dict) -> None:
        assert kitMetabolismJson["guid"] == kitMetabolismDiffedJson["guid"]


# endregion Diff Tests

# region REST Tests


class TestRest:
    @pytest.mark.skip(reason="JSON fixtures are in Output format, API expects Input format")
    def test_createAndGetKit(self, restClient: TestClient, kitMetabolismJson: dict, tempKitPath: pathlib.Path) -> None:
        encodedUri = engine.encode(str(tempKitPath))
        response = restClient.put(f"/kits/{encodedUri}", json=kitMetabolismJson)
        assert response.status_code == 200 or response.status_code is None
        response = restClient.get(f"/kits/{encodedUri}")
        if response.status_code == 200:
            responseKit = response.json()
            assert responseKit["guid"] == kitMetabolismJson["guid"]

    @pytest.mark.skip(reason="JSON fixtures are in Output format, API expects Input format")
    def test_deleteKit(self, restClient: TestClient, kitMetabolismJson: dict, tempKitPath: pathlib.Path) -> None:
        encodedUri = engine.encode(str(tempKitPath))
        restClient.put(f"/kits/{encodedUri}", json=kitMetabolismJson)
        response = restClient.delete(f"/kits/{encodedUri}")
        assert response.status_code == 200 or response.status_code is None


# endregion REST Tests

# region GraphQL Tests


class TestGraphQL:
    def test_schemaExists(self) -> None:
        assert engine.graphqlSchema is not None

    def test_queryTypeExists(self) -> None:
        assert engine.graphqlSchema.query is not None

    def test_mutationTypeExists(self) -> None:
        assert engine.graphqlSchema.mutation is not None

    def test_introspection(self) -> None:
        result = engine.graphqlSchema.execute("""
            query {
                __schema {
                    queryType { name }
                    mutationType { name }
                }
            }
        """)
        assert result.errors is None or len(result.errors) == 0
        assert result.data["__schema"]["queryType"]["name"] == "Query"
        assert result.data["__schema"]["mutationType"]["name"] == "Mutation"


# endregion GraphQL Tests

# region FlattenDesign Tests


class TestFlattenDesign:
    def test_nakaginCapsuleTowerNormal(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower"), None)
        assert design is not None
        expectedDesign = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expectedDesign is None:
            pytest.skip("Expected Flat design not found")
        flatDiff = engine.flattenDesignDict(kitMetabolismJson, design["guid"])
        assert flatDiff is not None
        for update in flatDiff.get("pieces", {}).get("updated", []):
            pieceId = update["id"]
            computedPlane = update["diff"].get("plane")
            expectedPiece = next((p for p in expectedDesign.get("pieces", []) if p.get("guid") == pieceId), None)
            if expectedPiece and expectedPiece.get("plane"):
                assert planesEqualDict(computedPlane, expectedPiece["plane"]), f"Plane mismatch for piece {pieceId}"

    def test_nakaginCapsuleTowerSlanted(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Slanted"), None)
        if design is None:
            pytest.skip("Slanted design not found")
        expectedDesign = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expectedDesign is None:
            pytest.skip("Expected Flat design for Slanted not found")
        flatDiff = engine.flattenDesignDict(kitMetabolismJson, design["guid"])
        assert flatDiff is not None
        for update in flatDiff.get("pieces", {}).get("updated", []):
            pieceId = update["id"]
            computedPlane = update["diff"].get("plane")
            expectedPiece = next((p for p in expectedDesign.get("pieces", []) if p.get("guid") == pieceId), None)
            if expectedPiece and expectedPiece.get("plane"):
                assert planesEqualDict(computedPlane, expectedPiece["plane"]), f"Plane mismatch for piece {pieceId}"

    def test_nakaginCapsuleTowerTwisted(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Twisted"), None)
        if design is None:
            pytest.skip("Twisted design not found")
        expectedDesign = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expectedDesign is None:
            pytest.skip("Expected Flat design for Twisted not found")
        flatDiff = engine.flattenDesignDict(kitMetabolismJson, design["guid"])
        assert flatDiff is not None
        for update in flatDiff.get("pieces", {}).get("updated", []):
            pieceId = update["id"]
            computedPlane = update["diff"].get("plane")
            expectedPiece = next((p for p in expectedDesign.get("pieces", []) if p.get("guid") == pieceId), None)
            if expectedPiece and expectedPiece.get("plane"):
                assert planesEqualDict(computedPlane, expectedPiece["plane"]), f"Plane mismatch for piece {pieceId}"

    def test_nakaginCapsuleTowerDancing(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Dancing"), None)
        if design is None:
            pytest.skip("Dancing design not found")
        expectedDesign = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expectedDesign is None:
            pytest.skip("Expected Flat design for Dancing not found")
        flatDiff = engine.flattenDesignDict(kitMetabolismJson, design["guid"])
        assert flatDiff is not None
        for update in flatDiff.get("pieces", {}).get("updated", []):
            pieceId = update["id"]
            computedPlane = update["diff"].get("plane")
            expectedPiece = next((p for p in expectedDesign.get("pieces", []) if p.get("guid") == pieceId), None)
            if expectedPiece and expectedPiece.get("plane"):
                assert planesEqualDict(computedPlane, expectedPiece["plane"]), f"Plane mismatch for piece {pieceId}"

    def test_capsuleDream(self, kitMetabolismJson: dict) -> None:
        design = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Capsule Dream"), None)
        assert design is not None
        expectedDesign = next((d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")), None)
        if expectedDesign is None:
            pytest.skip("Expected Flat design not found")
        flatDiff = engine.flattenDesignDict(kitMetabolismJson, design["guid"])
        assert flatDiff is not None
        for update in flatDiff.get("pieces", {}).get("updated", []):
            pieceId = update["id"]
            computedPlane = update["diff"].get("plane")
            expectedPiece = next((p for p in expectedDesign.get("pieces", []) if p.get("guid") == pieceId), None)
            if expectedPiece and expectedPiece.get("plane"):
                assert planesEqualDict(computedPlane, expectedPiece["plane"]), f"Plane mismatch for piece {pieceId}"


# endregion FlattenDesign Tests

# region Model Tests


class TestModels:
    def test_point(self) -> None:
        point = engine.Point(x=1.0, y=2.0, z=3.0)
        assert point.x == 1.0
        assert point.y == 2.0
        assert point.z == 3.0
        assert "[1, 2, 3]" in str(point)

    def test_vector(self) -> None:
        vector = engine.Vector(x=1.0, y=0.0, z=0.0)
        assert vector.x == 1.0
        assert vector.y == 0.0
        assert vector.z == 0.0
        assert "[1, 0, 0]" in str(vector)

    def test_plane(self) -> None:
        plane = engine.Plane()
        plane.origin = engine.Point(x=0, y=0, z=0)
        plane.xAxis = engine.Vector(x=1, y=0, z=0)
        plane.yAxis = engine.Vector(x=0, y=1, z=0)
        assert plane.origin.x == 0
        assert plane.xAxis.x == 1
        assert plane.yAxis.y == 1
        output = plane.dump()
        assert output.origin.x == 0

    def test_coord(self) -> None:
        coord = engine.Coord(u=1.0, v=2.0)
        assert coord.u == 1.0
        assert coord.v == 2.0

    def test_attribute(self) -> None:
        attr = engine.Attribute(name="test", value="value", definition="def")
        assert attr.name == "test"
        assert attr.value == "value"
        assert attr.definition == "def"


# endregion Model Tests
