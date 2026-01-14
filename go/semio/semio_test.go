// SPDX-License-Identifier: LGPL-3.0-only
// Copyright (c) 2025 Ueli Saluz and semio contributors

package semio

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

const AssetsPath = "../../assets/semio"

func loadJSON(t *testing.T, filename string, v interface{}) {
	path := filepath.Join(AssetsPath, filename)
	data, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("Failed to read %s: %v", filename, err)
	}
	if err := json.Unmarshal(data, v); err != nil {
		t.Fatalf("Failed to parse %s: %v", filename, err)
	}
}

func TestKitSerializationFromAsset(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)

	data, err := SerializeKit(kit)
	if err != nil {
		t.Fatalf("SerializeKit failed: %v", err)
	}

	var parsed Kit
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("Unmarshal failed: %v", err)
	}

	if !AreKitsEqual(kit, parsed) {
		t.Error("Serialized and deserialized kit should be equal")
	}
}

func TestKitDiffOperations(t *testing.T) {
	var kitOriginal Kit
	loadJSON(t, "kit_metabolism.json", &kitOriginal)
	kitOriginal.Designs = FilterDesignsWithoutParent(kitOriginal.Designs)

	var kitDiff KitDiff
	loadJSON(t, "diff_kit_metabolism.json", &kitDiff)

	var kitDiffInverted KitDiff
	loadJSON(t, "diff_kit_metabolism_inverted.json", &kitDiffInverted)

	var kitDiffed Kit
	loadJSON(t, "kit_metabolism_diffed.json", &kitDiffed)

	computedDiff := GetKitDiff(kitOriginal, kitDiffed)
	if !AreKitDiffsEqual(computedDiff, kitDiff) {
		t.Error("Computed diff should equal expected diff")
	}

	computedInverseDiff := InverseKitDiff(kitOriginal, kitDiff)
	if !AreKitDiffsEqual(computedInverseDiff, kitDiffInverted) {
		t.Error("Computed inverse diff should equal expected inverse diff")
	}

	appliedForward := ApplyKitDiff(kitOriginal, kitDiff)
	if !AreKitsEqual(appliedForward, kitDiffed) {
		t.Error("Original + Diff should equal DiffedKit")
	}

	appliedInverse := ApplyKitDiff(kitDiffed, kitDiffInverted)
	if !AreKitsEqual(appliedInverse, kitOriginal) {
		t.Error("DiffedKit + InverseDiff should equal original Kit")
	}
}

func TestValidationMatchesExpectedOutput(t *testing.T) {
	var validKit Kit
	loadJSON(t, "kit_metabolism.json", &validKit)
	validResult := ValidateKit(validKit)
	if HasErrors(validResult) {
		t.Errorf("Valid kit should not have errors, got %d problems", len(validResult.Problems))
	}

	var invalidKit Kit
	loadJSON(t, "kit_invalid.json", &invalidKit)
	result := ValidateKit(invalidKit)
	serializedResult := ToValidationResult(result)

	var expected ValidationResultSerialized
	loadJSON(t, "validation.json", &expected)

	if !AreValidationResultsEqual(serializedResult, expected) {
		t.Errorf("Validation mismatch. Got %d problems, expected %d",
			len(serializedResult.Problems), len(expected.Problems))
	}
}

func planesEqual(p1, p2 *Plane, tolerance float64) bool {
	if p1 == nil || p2 == nil {
		return false
	}
	return floatEqual(p1.Origin.X, p2.Origin.X, tolerance) &&
		floatEqual(p1.Origin.Y, p2.Origin.Y, tolerance) &&
		floatEqual(p1.Origin.Z, p2.Origin.Z, tolerance) &&
		floatEqual(p1.XAxis.X, p2.XAxis.X, tolerance) &&
		floatEqual(p1.XAxis.Y, p2.XAxis.Y, tolerance) &&
		floatEqual(p1.XAxis.Z, p2.XAxis.Z, tolerance) &&
		floatEqual(p1.YAxis.X, p2.YAxis.X, tolerance) &&
		floatEqual(p1.YAxis.Y, p2.YAxis.Y, tolerance) &&
		floatEqual(p1.YAxis.Z, p2.YAxis.Z, tolerance)
}

func centersEqual(c1, c2 *Coord, tolerance float64) bool {
	if c1 == nil && c2 == nil {
		return true
	}
	if c1 == nil || c2 == nil {
		return false
	}
	return floatEqual(c1.U, c2.U, tolerance) && floatEqual(c1.V, c2.V, tolerance)
}

func floatEqual(a, b, tolerance float64) bool {
	if a > b {
		return a-b < tolerance
	}
	return b-a < tolerance
}

func findDesignByName(designs []Design, name string, parentGuid *string) *Design {
	for i := range designs {
		d := &designs[i]
		if d.Name != name {
			continue
		}
		if parentGuid == nil {
			if d.Parent == nil {
				return d
			}
		} else {
			if d.Parent != nil && d.Parent.Guid == *parentGuid {
				return d
			}
		}
	}
	return nil
}

func findPieceByName(pieces []Piece, name string) *Piece {
	for i := range pieces {
		if pieces[i].Name != nil && *pieces[i].Name == name {
			return &pieces[i]
		}
	}
	return nil
}

func testFlattenDesign(t *testing.T, kit Kit, designPath []string) {
	var design *Design
	var parentGuid *string

	for _, designName := range designPath {
		design = findDesignByName(kit.Designs, designName, parentGuid)
		if design == nil {
			t.Fatalf("Design %q not found", designName)
		}
		parentGuid = &design.Guid
	}

	if design == nil {
		t.Fatal("Design is nil")
	}

	expectedDesign := findDesignByName(kit.Designs, "Flat", &design.Guid)
	if expectedDesign == nil {
		t.Fatalf("Expected flat design not found for %q", design.Name)
	}

	flatDesignDiff := FlattenDesign(&kit, design.Guid)
	flatDesign := ApplyDesignDiff(*design, flatDesignDiff)

	const tolerance = 0.001
	for _, piece := range flatDesign.Pieces {
		if piece.Name == nil {
			continue
		}
		expectedPiece := findPieceByName(expectedDesign.Pieces, *piece.Name)
		if expectedPiece == nil {
			t.Errorf("Expected piece %q not found", *piece.Name)
			continue
		}
		if piece.Plane == nil {
			t.Errorf("Piece %q has no plane", *piece.Name)
			continue
		}
		if piece.Center == nil {
			t.Errorf("Piece %q has no center", *piece.Name)
			continue
		}
		if !planesEqual(piece.Plane, expectedPiece.Plane, tolerance) {
			t.Errorf("Piece %q plane mismatch: got origin(%.4f,%.4f,%.4f) xAxis(%.4f,%.4f,%.4f) yAxis(%.4f,%.4f,%.4f), expected origin(%.4f,%.4f,%.4f) xAxis(%.4f,%.4f,%.4f) yAxis(%.4f,%.4f,%.4f)",
				*piece.Name,
				piece.Plane.Origin.X, piece.Plane.Origin.Y, piece.Plane.Origin.Z,
				piece.Plane.XAxis.X, piece.Plane.XAxis.Y, piece.Plane.XAxis.Z,
				piece.Plane.YAxis.X, piece.Plane.YAxis.Y, piece.Plane.YAxis.Z,
				expectedPiece.Plane.Origin.X, expectedPiece.Plane.Origin.Y, expectedPiece.Plane.Origin.Z,
				expectedPiece.Plane.XAxis.X, expectedPiece.Plane.XAxis.Y, expectedPiece.Plane.XAxis.Z,
				expectedPiece.Plane.YAxis.X, expectedPiece.Plane.YAxis.Y, expectedPiece.Plane.YAxis.Z)
		}
		if !centersEqual(piece.Center, expectedPiece.Center, tolerance) {
			t.Errorf("Piece %q center mismatch", *piece.Name)
		}
	}
}

func TestFlattenDesignNakaginCapsuleTower(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)
	testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower"})
}

func TestFlattenDesignSlanted(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)
	testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Slanted"})
}

func TestFlattenDesignTwisted(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)
	testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Twisted"})
}

func TestFlattenDesignDancing(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)
	testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Dancing"})
}

func TestFlattenDesignCapsuleDream(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)
	testFlattenDesign(t, kit, []string{"Capsule Dream"})
}
