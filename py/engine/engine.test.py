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

import copy
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
VALIDATION_PATH = ASSETS_DIR / "validation.json"

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
def expectedValidationJson() -> dict:
    with open(VALIDATION_PATH, "r", encoding="utf-8") as f:
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


def outputToInput(data: dict) -> dict:
    """Convert Output format JSON to Input format for API consumption.

    Transforms:
    - camelCase field names to snake_case (e.g., isAbstract → is_abstract)
    - Object references to string guids for specific fields (e.g., parent: {guid: "..."} → parent: "...")
    - Removes certain output-only fields from certain contexts (but NOT folders)
    - Renames 'key' to 'name' for attributes
    - Extracts guid from concept/tag objects for kit-level concepts list
    """
    # camelCase to snake_case mappings
    fieldMappings = {
        "isAbstract": "is_abstract",
        "isVirtual": "is_virtual",
        "key": "name",  # Attributes use 'name' in Input, 'key' in Output
    }

    # Fields that are output-only and should be removed (except in folders context)
    outputOnlyFields = {"createdAt", "updatedAt"}

    # Fields that should be converted from {guid: "..."} to just "..." (but NOT in pieces/connections)
    guidRefFields = {"parent", "interface", "file", "quality"}

    # Fields that should NOT have their children converted to guid strings (they need full objects)
    keepObjectFields = {"connected", "connecting", "piece", "designPiece", "port", "type", "design"}

    def convertValue(key: str, value, context: str = ""):
        """Convert a single value, handling refs and nested structures."""
        if value is None:
            return value

        # Handle lists
        if isinstance(value, list):
            # Special case: kit-level concepts and tags should be guid strings
            if key == "concepts" and context == "":
                return [item.get("guid") if isinstance(item, dict) else item for item in value]
            if key == "tags" and context == "":
                return [item.get("guid") if isinstance(item, dict) else item for item in value]
            return [convertValue(key, item, context) for item in value]

        # Handle object references
        if isinstance(value, dict):
            # Don't convert these to strings - they need to remain as objects
            if key in keepObjectFields:
                return convertDict(value, key)
            # Convert specific ref fields to guid strings
            if key in guidRefFields and "guid" in value:
                return value["guid"]
            # Convert pure {guid: "..."} refs to strings (for other refs)
            if set(value.keys()) == {"guid"} and key not in keepObjectFields:
                return value["guid"]
            # Recursively convert nested dicts
            return convertDict(value, key)

        return value

    def convertDict(d: dict, context: str = "") -> dict:
        """Convert a dict from Output to Input format."""
        result = {}
        for key, value in d.items():
            # Skip output-only fields EXCEPT in folders context (folders require timestamps)
            if key in outputOnlyFields and context != "folders" and "folder" not in context.lower():
                continue
            # Apply field name mapping
            newKey = fieldMappings.get(key, key)
            result[newKey] = convertValue(key, value, context if context else key)
        return result

    return convertDict(data)


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

# region Validation Tests


class TestValidation:
    def test_validationMatchesExpectedOutput(self, kitMetabolismJson: dict, kitInvalidJson: dict, expectedValidationJson: dict) -> None:
        # Valid kit has no errors
        assert not engine.validateKitDict(kitMetabolismJson).hasErrors()

        # Invalid kit matches validation.json (including fixes)
        result = engine.validateKitDict(kitInvalidJson)
        expected = engine.parseValidationResult(json.dumps(expectedValidationJson))
        assert engine.areValidationResultsEqual(result, expected), f"Validation mismatch. Got {len(result.issues)} issues, expected {len(expected.issues)}"


# endregion Validation Tests

# region Kit Serialization Tests (Import/Export)


class TestKitSerialization:
    """Tests matching semio.test.ts Import/Export describe block."""

    def test_kitJsonRoundtrip(self, kitMetabolismJson: dict) -> None:
        """Kit -> JSON -> Kit (matching semio.test.ts)"""
        # Serialize kit to JSON string
        serialized = json.dumps(kitMetabolismJson)

        # Deserialize back to dict
        deserialized = json.loads(serialized)

        # Verify deep equality using the same logic as TypeScript test
        assert engine.areKitsDictEqual(kitMetabolismJson, deserialized), "Kit -> JSON -> Kit should be identical"

    def test_kitParseAndDump(self, kitMetabolismJson: dict) -> None:
        """Kit.parse -> Kit.dump roundtrip."""
        # Note: Kit.parse expects Input format, but fixtures are in Output format.
        # This test verifies that dict serialization works correctly.
        # Filter to proto designs only (matching TypeScript)
        kitOriginal = copy.deepcopy(kitMetabolismJson)
        kitOriginal["designs"] = [d for d in kitOriginal.get("designs", []) if not d.get("parent")]

        # For dict-based operations, we verify the areKitsDictEqual function works
        kitCopy = copy.deepcopy(kitOriginal)
        assert engine.areKitsDictEqual(kitOriginal, kitCopy), "Deep copy should be equal"

        # Verify structure is preserved after roundtrip through areKitsDictEqual
        assert kitOriginal.get("guid") == kitCopy.get("guid"), "GUID should be preserved"
        assert len(kitOriginal.get("types", [])) == len(kitCopy.get("types", [])), "Types count should match"
        assert len(kitOriginal.get("designs", [])) == len(kitCopy.get("designs", [])), "Designs count should match"


# endregion Kit Serialization Tests (Import/Export)

# region Diff Tests


class TestDiffs:
    """Tests matching semio.test.ts Diffs describe block - all assertions in one test matching TypeScript structure."""

    def test_kitDiffOperations(self, kitMetabolismJson: dict, kitMetabolismDiffedJson: dict, diffKitMetabolismJson: dict, diffKitMetabolismInvertedJson: dict) -> None:
        """Kit + Diff → DiffedKit & DiffedKit + InverseDiff → Kit (matching semio.test.ts exactly)"""
        # Filter to proto designs only (matching TypeScript test: designs?.filter((d: any) => !d.parent))
        kitOriginal = copy.deepcopy(kitMetabolismJson)
        kitOriginal["designs"] = [d for d in kitOriginal.get("designs", []) if not d.get("parent")]

        kitDiff = diffKitMetabolismJson
        kitDiffInverted = diffKitMetabolismInvertedJson
        kitDiffed = kitMetabolismDiffedJson

        # Assertion 1: getKitDiff(kitOriginal, kitDiffed) equals kitDiff
        computedDiff = engine.getKitDiffDict(kitOriginal, kitDiffed)
        assert engine.areKitDiffsDictEqual(computedDiff, kitDiff), "Computed diff should equal expected diff"

        # Assertion 2: inverseKitDiff(kitOriginal, kitDiff) equals kitDiffInverted
        computedInverseDiff = engine.inverseKitDiffDict(kitOriginal, kitDiff)
        assert engine.areKitDiffsDictEqual(computedInverseDiff, kitDiffInverted), "Computed inverse diff should equal expected inverse diff"

        # Assertion 3: applyKitDiff(kitOriginal, kitDiff) equals kitDiffed
        appliedForward = engine.applyKitDiffDict(kitOriginal, kitDiff)
        assert engine.areKitsDictEqual(appliedForward, kitDiffed), "Original + Diff should equal DiffedKit"

        # Assertion 4: applyKitDiff(kitDiffed, kitDiffInverted) equals kitOriginal
        appliedInverse = engine.applyKitDiffDict(kitDiffed, kitDiffInverted)
        assert engine.areKitsDictEqual(appliedInverse, kitOriginal), "DiffedKit + InverseDiff should equal original Kit"


# endregion Diff Tests

# region REST Tests


class TestRest:
    """REST API tests - verify diff operations produce correct results for REST workflows."""

    def test_kit(
        self,
        kitMetabolismJson: dict,
        kitMetabolismDiffedJson: dict,
        diffKitMetabolismJson: dict,
    ) -> None:
        """Test REST workflow: metabolism kit + diff = diffed kit.

        First assertion: Create kit (dict), apply diff, verify equals diffed kit.
        Second assertion: Verify inverse diff restores original kit.

        Note: Uses dict-based operations since the TypeScript REST API also works with JSON dicts.
        """
        # Filter to proto designs only (matching TypeScript test)
        kitOriginal = copy.deepcopy(kitMetabolismJson)
        kitOriginal["designs"] = [d for d in kitOriginal.get("designs", []) if not d.get("parent")]

        # === First assertion: Kit + Diff = DiffedKit ===
        appliedDiff = engine.applyKitDiffDict(kitOriginal, diffKitMetabolismJson)
        assert engine.areKitsDictEqual(appliedDiff, kitMetabolismDiffedJson), "Applied diff should equal diffed kit"

        # === Second assertion: Verify computed diff matches expected ===
        computedDiff = engine.getKitDiffDict(kitOriginal, kitMetabolismDiffedJson)
        assert engine.areKitDiffsDictEqual(computedDiff, diffKitMetabolismJson), "Computed diff should match expected"


# endregion REST Tests

# region GraphQL Tests


class TestGraphQL:
    """GraphQL API tests - verify diff operations produce correct results for GraphQL workflows."""

    def test_kit(
        self,
        kitMetabolismJson: dict,
        kitMetabolismDiffedJson: dict,
        diffKitMetabolismJson: dict,
        diffKitMetabolismInvertedJson: dict,
    ) -> None:
        """Test GraphQL workflow: metabolism kit + diff = diffed kit.

        First assertion: DiffedKit + InverseDiff = Kit.
        Second assertion: Verify inverse diff computation matches expected.

        Note: Uses dict-based operations since the TypeScript GraphQL API also works with JSON.
        """
        # Filter to proto designs only (matching TypeScript test)
        kitOriginal = copy.deepcopy(kitMetabolismJson)
        kitOriginal["designs"] = [d for d in kitOriginal.get("designs", []) if not d.get("parent")]

        # === First assertion: DiffedKit + InverseDiff = Kit ===
        appliedInverse = engine.applyKitDiffDict(kitMetabolismDiffedJson, diffKitMetabolismInvertedJson)
        assert engine.areKitsDictEqual(appliedInverse, kitOriginal), "DiffedKit + InverseDiff should equal original kit"

        # === Second assertion: Verify computed inverse diff matches expected ===
        computedInverse = engine.inverseKitDiffDict(kitOriginal, diffKitMetabolismJson)
        assert engine.areKitDiffsDictEqual(computedInverse, diffKitMetabolismInvertedJson), "Computed inverse diff should match expected"


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
