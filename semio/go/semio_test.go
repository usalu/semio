// #region 🔖Header
// [👤semio📚go🥼semiotest](semiorepo://p/u/semio/b/l/go/f/semio_test.go)

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
	"math"
	"os"
	"path/filepath"
	"testing"
)

type modelSelectionAsset struct {
	Cases []modelSelectionCase `json:"cases"`
}

type modelSelectionCase struct {
	Name             string                `json:"name"`
	SelectedTagGuids []string              `json:"selectedTagGuids"`
	ExpectedGuid     *string               `json:"expectedGuid"`
	Models           []modelSelectionModel `json:"models"`
}

type modelSelectionModel struct {
	Guid     string   `json:"guid"`
	FileGuid string   `json:"fileGuid"`
	TagGuids []string `json:"tagGuids"`
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

func containsAllTags(model Model, selectedTagGuids []string) bool {
	for _, selectedGuid := range selectedTagGuids {
		found := false
		for _, tag := range model.Tags {
			if tag.Guid == selectedGuid {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}
	return true
}

func jaccardTagGuids(a []TagId, b []string) float64 {
	if len(a) == 0 && len(b) == 0 {
		return 1
	}
	setA := make(map[string]bool)
	setB := make(map[string]bool)
	for _, tag := range a {
		setA[tag.Guid] = true
	}
	for _, guid := range b {
		setB[guid] = true
	}
	intersection := 0
	for guid := range setA {
		if setB[guid] {
			intersection++
		}
	}
	union := len(setA)
	for guid := range setB {
		if !setA[guid] {
			union++
		}
	}
	if union == 0 {
		return 0
	}
	return float64(intersection) / float64(union)
}

func selectBestModelLikeSemioTS(models []Model, selectedTagGuids []string) *Model {
	if len(models) == 0 {
		return nil
	}
	if len(selectedTagGuids) == 0 {
		for i := range models {
			if len(models[i].Tags) == 0 {
				return &models[i]
			}
		}
		return &models[0]
	}
	filtered := make([]Model, 0)
	for _, model := range models {
		if containsAllTags(model, selectedTagGuids) {
			filtered = append(filtered, model)
		}
	}
	if len(filtered) == 0 {
		return nil
	}
	maxIndex := 0
	maxScore := jaccardTagGuids(filtered[0].Tags, selectedTagGuids)
	for i := 1; i < len(filtered); i++ {
		score := jaccardTagGuids(filtered[i].Tags, selectedTagGuids)
		if score > maxScore {
			maxScore = score
			maxIndex = i
		}
	}
	return &filtered[maxIndex]
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
	t.Run("Metabolism", func(t *testing.T) {

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
			t.Error("JSON -> Memory -> JSON: serialized and deserialized kit should be equal")
		}

		files := make(map[string][]byte)
		for i := range kit.Files {
			if kit.Files[i].Blob != nil {
				decoded, err := blobDecode(*kit.Files[i].Blob)
				if err != nil {
					t.Fatalf("Failed to decode blob for %s: %v", kit.Files[i].Name, err)
				}
				filePath := buildFilePath(&kit, &kit.Files[i])
				files[filePath] = decoded
			}
		}

		roundtripPath := filepath.Join(t.TempDir(), "metabolism_roundtrip.zip")
		if err := KitToZip(&kit, files, roundtripPath, ""); err != nil {
			t.Fatalf("KitToZip failed: %v", err)
		}

		kit2, files2, err := KitFromZip(roundtripPath)
		if err != nil {
			t.Fatalf("KitFromZip failed: %v", err)
		}

		if !AreKitsEqual(kit, *kit2) {
			t.Error("ZIP -> JSON: roundtrip kit should be equal")
		}
		if len(files2) != len(files) {
			t.Errorf("Expected %d files, got %d", len(files), len(files2))
		}
	})
}

func TestDesignModel(t *testing.T) {
	t.Run("Model selection cases from shared semio assets", func(t *testing.T) {
		var payload modelSelectionAsset
		loadJSON(t, "model_selection.json", &payload)
		for _, testCase := range payload.Cases {
			models := make([]Model, 0, len(testCase.Models))
			for _, model := range testCase.Models {
				tags := make([]TagId, 0, len(model.TagGuids))
				for _, guid := range model.TagGuids {
					tags = append(tags, TagId{Guid: guid})
				}
				models = append(models, Model{
					Guid: model.Guid,
					File: FileId{Guid: model.FileGuid},
					Tags: tags,
				})
			}
			selected := selectBestModelLikeSemioTS(models, testCase.SelectedTagGuids)
			if testCase.ExpectedGuid == nil {
				if selected != nil {
					t.Fatalf("Case %q failed: got %q expected nil", testCase.Name, selected.Guid)
				}
				continue
			}
			if selected == nil || selected.Guid != *testCase.ExpectedGuid {
				got := "<nil>"
				if selected != nil {
					got = selected.Guid
				}
				t.Fatalf("Case %q failed: got %q expected %q", testCase.Name, got, *testCase.ExpectedGuid)
			}
		}
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

func TestChange(t *testing.T) {
	t.Run("Metabolism", func(t *testing.T) {
		t.Run("Kit + Change.Forward = DiffedKit & DiffedKit + Change.Backward = Kit", func(t *testing.T) {
			var kitOriginal Kit
			loadJSON(t, "kit_metabolism.json", &kitOriginal)
			kitOriginal.Designs = FilterDesignsWithoutParent(kitOriginal.Designs)

			var kitDiff KitDiff
			loadJSON(t, "diff_kit_metabolism.json", &kitDiff)

			var kitDiffInverted KitDiff
			loadJSON(t, "diff_kit_metabolism_inverted.json", &kitDiffInverted)

			var kitDiffed Kit
			loadJSON(t, "kit_metabolism_diffed.json", &kitDiffed)

			change := GetKitChange(kitOriginal, kitDiffed, nil, nil)

			if !AreKitDiffsEqual(change.Forward, kitDiff) {
				t.Error("Computed diff should equal expected diff")
			}

			if !AreKitDiffsEqual(change.Backward, kitDiffInverted) {
				t.Error("Computed inverse diff should equal expected inverse diff")
			}

			appliedForward := ApplyKitDiff(kitOriginal, change.Forward)
			if !AreKitsEqual(appliedForward, kitDiffed) {
				t.Error("Original + Diff should equal DiffedKit")
			}

			appliedInverse := ApplyKitDiff(kitDiffed, change.Backward)
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

func TestDesignQualitySum(t *testing.T) {
	t.Run("Nakagin Capsule Tower", func(t *testing.T) {
		t.Run("Sum Effective Floor Area", func(t *testing.T) {
			var kit Kit
			loadJSON(t, "kit_metabolism.json", &kit)
			var designGuid string
			for _, d := range kit.Designs {
				if d.Name == "Nakagin Capsule Tower" && d.Parent == nil {
					designGuid = d.Guid
					break
				}
			}
			if designGuid == "" {
				t.Fatal("Nakagin Capsule Tower design not found")
			}
			var qualityGuid string
			for _, q := range kit.Qualities {
				if q.Name == "effective floor area" {
					qualityGuid = q.Guid
					break
				}
			}
			if qualityGuid == "" {
				t.Fatal("effective floor area quality not found")
			}
			result := SumQualityInDesign(&kit, designGuid, qualityGuid)
			if math.Abs(result-2349.53) > 0.01 {
				t.Errorf("Expected ~2349.53, got %f", result)
			}
		})
	})
}
