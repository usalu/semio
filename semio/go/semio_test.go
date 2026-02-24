// #region 🔖Header

// [👤semio📚go🥼semiotestgo](semiorepo://file/semio/go/semio_test.go)

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖Header

package semio

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

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
			t.Errorf("Piece %q plane mismatch", *piece.Name)
		}
		if !centersEqual(piece.Center, expectedPiece.Center, tolerance) {
			t.Errorf("Piece %q center mismatch", *piece.Name)
		}
	}
}

func TestRoundtrip(t *testing.T) {
	t.Run("Json", func(t *testing.T) {
		t.Run("Metabolism", func(t *testing.T) {
			t.Run("Kit -> Json -> Kit", func(t *testing.T) {
				var kit Kit
				loadJSON(t, "kit_metabolism.json", &kit)

				data, err := SerializeKit(kit)
				if err != nil {
					t.Fatalf("SerializeKit failed: %v", err)
				}

				parsed, err := DeserializeKit(data)
				if err != nil {
					t.Fatalf("DeserializeKit failed: %v", err)
				}

				if !AreKitsEqual(kit, parsed) {
					t.Error("Serialized and deserialized kit should be equal")
				}
			})
		})
	})

	t.Run("Zip", func(t *testing.T) {
		t.Run("Metabolism", func(t *testing.T) {
			t.Run("Zip -> Kit -> Zip -> Kit", func(t *testing.T) {
				zipPath := filepath.Join(AssetsPath, "metabolism.zip")
				kit, files, err := KitFromZip(zipPath)
				if err != nil {
					t.Fatalf("KitFromZip failed: %v", err)
				}
				if kit.Guid == "" {
					t.Error("Kit GUID should not be empty")
				}
				if kit.Name != "Metabolism" {
					t.Errorf("Expected kit name Metabolism, got %s", kit.Name)
				}
				if len(kit.Types) == 0 {
					t.Error("Kit should have types")
				}
				if len(kit.Designs) == 0 {
					t.Error("Kit should have designs")
				}
				if len(files) == 0 {
					t.Error("Kit should have files")
				}

				schemaPath := filepath.Join(AssetsPath, "..", "..", "sql", "sqlite", "semio", "schema.sql")
				schemaData, err := os.ReadFile(schemaPath)
				if err != nil {
					t.Fatalf("Failed to read schema.sql: %v", err)
				}

				roundtripPath := filepath.Join(t.TempDir(), "metabolism_roundtrip.zip")
				if err := KitToZip(kit, files, roundtripPath, string(schemaData)); err != nil {
					t.Fatalf("KitToZip failed: %v", err)
				}

				kit2, files2, err := KitFromZip(roundtripPath)
				if err != nil {
					t.Fatalf("KitFromZip (roundtrip) failed: %v", err)
				}
				if kit2.Guid != kit.Guid {
					t.Errorf("Expected kit GUID %s, got %s", kit.Guid, kit2.Guid)
				}
				if kit2.Name != kit.Name {
					t.Errorf("Expected kit name %s, got %s", kit.Name, kit2.Name)
				}
				if len(kit2.Types) != len(kit.Types) {
					t.Errorf("Expected %d types, got %d", len(kit.Types), len(kit2.Types))
				}
				if len(kit2.Designs) != len(kit.Designs) {
					t.Errorf("Expected %d designs, got %d", len(kit.Designs), len(kit2.Designs))
				}
				if len(files2) != len(files) {
					t.Errorf("Expected %d files, got %d", len(files), len(files2))
				}
			})
		})
	})
}

func TestFlatten(t *testing.T) {
	var kit Kit
	loadJSON(t, "kit_metabolism.json", &kit)

	t.Run("Nakagin Capsule Tower", func(t *testing.T) {
		t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
			testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower"})
		})
		t.Run("Slanted", func(t *testing.T) {
			t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
				testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Slanted"})
			})
		})
		t.Run("Twisted", func(t *testing.T) {
			t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
				testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Twisted"})
			})
		})
		t.Run("Dancing", func(t *testing.T) {
			t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
				testFlattenDesign(t, kit, []string{"Nakagin Capsule Tower", "Dancing"})
			})
		})
	})

	t.Run("Capsule Dream", func(t *testing.T) {
		t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
			testFlattenDesign(t, kit, []string{"Capsule Dream"})
		})
	})
}

func TestDiff(t *testing.T) {
	t.Run("Metabolism", func(t *testing.T) {
		t.Run("Kit + Diff = DiffedKit & DiffedKit + InvertedDiff = Kit", func(t *testing.T) {
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
		})
	})
}

func TestValidation(t *testing.T) {
	t.Run("Metabolism", func(t *testing.T) {
		t.Run("Metabolism Kit -> Validate = Empty report", func(t *testing.T) {
			var validKit Kit
			loadJSON(t, "kit_metabolism.json", &validKit)
			validResult := ValidateKit(validKit)
			if HasErrors(validResult) {
				t.Errorf("Valid kit should not have errors, got %d problems", len(validResult.Problems))
			}
		})
	})

	t.Run("Invalid", func(t *testing.T) {
		t.Run("Invalid Kit -> Validate = Invalid Report", func(t *testing.T) {
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
		})
	})
}
