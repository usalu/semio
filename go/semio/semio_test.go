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

func TestGuid(t *testing.T) {
	g1 := Guid()
	g2 := Guid()
	if g1 == g2 {
		t.Error("Guids should be unique")
	}
	if len(g1) != 32 {
		t.Errorf("Guid should be 32 chars, got %d", len(g1))
	}
}

func TestNormalize(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"  Hello World  ", "hello world"},
		{"UPPERCASE", "uppercase"},
		{"  spaces  ", "spaces"},
	}
	for _, tt := range tests {
		result := Normalize(tt.input)
		if result != tt.expected {
			t.Errorf("Normalize(%q) = %q, want %q", tt.input, result, tt.expected)
		}
	}
}

func TestNewKit(t *testing.T) {
	kit := NewKit("Test Kit")
	if kit.Name != "Test Kit" {
		t.Errorf("Kit name = %q, want %q", kit.Name, "Test Kit")
	}
	if kit.Version != "0.0.1" {
		t.Errorf("Kit version = %q, want %q", kit.Version, "0.0.1")
	}
	if kit.Guid == "" {
		t.Error("Kit should have a guid")
	}
}

func TestNewType(t *testing.T) {
	typ := NewType("Wall")
	if typ.Name != "Wall" {
		t.Errorf("Type name = %q, want %q", typ.Name, "Wall")
	}
	if typ.Guid == "" {
		t.Error("Type should have a guid")
	}
}

func TestNewDesign(t *testing.T) {
	design := NewDesign("My Design")
	if design.Name != "My Design" {
		t.Errorf("Design name = %q, want %q", design.Name, "My Design")
	}
	if design.Guid == "" {
		t.Error("Design should have a guid")
	}
}

func TestNewConnector(t *testing.T) {
	point := Point{X: 0, Y: 0, Z: 0}
	direction := Vector{X: 0, Y: 1, Z: 0}
	connector := NewConnector(point, direction, 0.5)
	if connector.Point.X != 0 || connector.Point.Y != 0 || connector.Point.Z != 0 {
		t.Error("Connector point should be (0, 0, 0)")
	}
	if connector.Direction.X != 0 || connector.Direction.Y != 1 || connector.Direction.Z != 0 {
		t.Error("Connector direction should be (0, 1, 0)")
	}
	if connector.T != 0.5 {
		t.Errorf("Connector T = %f, want %f", connector.T, 0.5)
	}
}

func TestKitSerialization(t *testing.T) {
	kit := NewKit("Serialization Test")
	kit.Types = []Type{NewType("TestType")}
	kit.Designs = []Design{NewDesign("TestDesign")}

	data, err := SerializeKit(kit)
	if err != nil {
		t.Fatalf("SerializeKit failed: %v", err)
	}

	var parsed Kit
	err = json.Unmarshal(data, &parsed)
	if err != nil {
		t.Fatalf("Unmarshal failed: %v", err)
	}

	if parsed.Name != kit.Name {
		t.Errorf("Parsed kit name = %q, want %q", parsed.Name, kit.Name)
	}
	if len(parsed.Types) != 1 {
		t.Errorf("Parsed kit should have 1 type, got %d", len(parsed.Types))
	}
	if len(parsed.Designs) != 1 {
		t.Errorf("Parsed kit should have 1 design, got %d", len(parsed.Designs))
	}
}

func TestFindTypeInKit(t *testing.T) {
	kit := NewKit("Find Test")
	typ := NewType("Findable")
	kit.Types = []Type{typ}

	found := FindTypeInKit(&kit, typ.Guid)
	if found == nil {
		t.Error("Should find type by guid")
	}
	if found.Name != "Findable" {
		t.Errorf("Found type name = %q, want %q", found.Name, "Findable")
	}

	notFound := FindTypeInKit(&kit, "nonexistent")
	if notFound != nil {
		t.Error("Should not find nonexistent type")
	}
}

func TestFindDesignInKit(t *testing.T) {
	kit := NewKit("Find Test")
	design := NewDesign("Findable Design")
	kit.Designs = []Design{design}

	found := FindDesignInKit(&kit, design.Guid)
	if found == nil {
		t.Error("Should find design by guid")
	}
	if found.Name != "Findable Design" {
		t.Errorf("Found design name = %q, want %q", found.Name, "Findable Design")
	}
}

func TestAddTypeToKit(t *testing.T) {
	typ := NewType("New Type")
	diff := AddTypeToKit(typ)

	if diff.Types == nil {
		t.Fatal("Diff should have types")
	}
	if len(diff.Types.Added) != 1 {
		t.Errorf("Should have 1 added type, got %d", len(diff.Types.Added))
	}
	if diff.Types.Added[0].Name != "New Type" {
		t.Errorf("Added type name = %q, want %q", diff.Types.Added[0].Name, "New Type")
	}
}

func TestRemoveTypeFromKit(t *testing.T) {
	diff := RemoveTypeFromKit("some-guid")

	if diff.Types == nil {
		t.Fatal("Diff should have types")
	}
	if len(diff.Types.Removed) != 1 {
		t.Errorf("Should have 1 removed type, got %d", len(diff.Types.Removed))
	}
	if diff.Types.Removed[0].Guid != "some-guid" {
		t.Errorf("Removed type guid = %q, want %q", diff.Types.Removed[0].Guid, "some-guid")
	}
}

func TestRound(t *testing.T) {
	tests := []struct {
		value    float64
		decimals int
		expected float64
	}{
		{3.14159, 2, 3.14},
		{3.145, 2, 3.15},
		{2.5, 0, 3},
		{1.234, 2, 1.23},
	}
	for _, tt := range tests {
		result := Round(tt.value, tt.decimals)
		if result != tt.expected {
			t.Errorf("Round(%f, %d) = %f, want %f", tt.value, tt.decimals, result, tt.expected)
		}
	}
}

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

	if kit.Guid == "" {
		t.Error("Kit should have a guid")
	}
	if kit.Name != "Metabolism" {
		t.Errorf("Kit name should be 'Metabolism', got %q", kit.Name)
	}
	if len(kit.Types) == 0 {
		t.Error("Kit should have types")
	}
	if len(kit.Designs) == 0 {
		t.Error("Kit should have designs")
	}

	data, err := SerializeKit(kit)
	if err != nil {
		t.Fatalf("SerializeKit failed: %v", err)
	}

	var parsed Kit
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("Unmarshal failed: %v", err)
	}

	kit.Designs = FilterDesignsWithoutParent(kit.Designs)
	parsed.Designs = FilterDesignsWithoutParent(parsed.Designs)
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

func TestAreKitsEqual(t *testing.T) {
	kit1 := NewKit("Test")
	kit2 := kit1
	kit2.Types = kit1.Types
	kit2.Designs = kit1.Designs

	if !AreKitsEqual(kit1, kit2) {
		t.Error("Identical kits should be equal")
	}

	kit3 := NewKit("Different")
	if AreKitsEqual(kit1, kit3) {
		t.Error("Different kits should not be equal")
	}
}

func TestFilterDesignsWithoutParent(t *testing.T) {
	parent := DesignId{Guid: "parent-guid"}
	designs := []Design{
		{Guid: "d1", Name: "Root Design", Parent: nil},
		{Guid: "d2", Name: "Child Design", Parent: &parent},
		{Guid: "d3", Name: "Another Root", Parent: nil},
	}

	filtered := FilterDesignsWithoutParent(designs)
	if len(filtered) != 2 {
		t.Errorf("Should have 2 root designs, got %d", len(filtered))
	}
	for _, d := range filtered {
		if d.Parent != nil {
			t.Errorf("Filtered design %q should not have a parent", d.Name)
		}
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

func TestKitJSONRoundtrip(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)

	serialized, err := SerializeKit(kit)
	if err != nil {
		t.Fatalf("SerializeKit failed: %v", err)
	}

	deserialized, err := DeserializeKit(serialized)
	if err != nil {
		t.Fatalf("DeserializeKit failed: %v", err)
	}

	if !AreKitsEqual(kit, deserialized) {
		t.Error("Kit -> JSON -> Kit roundtrip should produce equal kits")
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
