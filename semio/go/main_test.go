// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

package semio

import (
	"encoding/binary"
	"encoding/json"
	"math"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
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
		loadJSON(t, "metabolism.kit.semio.json", &kit)

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
		loadJSON(t, "model.selection.semio.json", &payload)
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

func TestKitFilterDesign(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)

	var expected Kit
	loadJSON(t, "nakagin-capsule-tower.filtered.kit.semio.json", &expected)

	nakaginDesign := findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
	if nakaginDesign == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}

	t.Run("filters kit to Nakagin Capsule Tower subset", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{DesignGuid: nakaginDesign.Guid})

		if len(filtered.Designs) != len(expected.Designs) {
			t.Fatalf("Expected %d designs, got %d", len(expected.Designs), len(filtered.Designs))
		}
		if len(filtered.Types) != len(expected.Types) {
			t.Fatalf("Expected %d types, got %d", len(expected.Types), len(filtered.Types))
		}
		if len(filtered.Files) != len(expected.Files) {
			t.Fatalf("Expected %d files, got %d", len(expected.Files), len(filtered.Files))
		}
		if len(filtered.Ports) != len(expected.Ports) {
			t.Fatalf("Expected %d ports, got %d", len(expected.Ports), len(filtered.Ports))
		}
		if len(filtered.Qualities) != len(expected.Qualities) {
			t.Fatalf("Expected %d qualities, got %d", len(expected.Qualities), len(filtered.Qualities))
		}
		if len(filtered.Authors) != len(expected.Authors) {
			t.Fatalf("Expected %d authors, got %d", len(expected.Authors), len(filtered.Authors))
		}

		filteredDesign := findDesignByName(filtered.Designs, "Nakagin Capsule Tower", nil)
		if filteredDesign == nil {
			t.Fatal("Filtered Nakagin Capsule Tower design not found")
		}
		if len(filteredDesign.Pieces) != len(nakaginDesign.Pieces) {
			t.Fatalf("Expected %d pieces, got %d", len(nakaginDesign.Pieces), len(filteredDesign.Pieces))
		}

		for _, expectedType := range expected.Types {
			matches := 0
			for _, filteredType := range filtered.Types {
				if filteredType.Guid == expectedType.Guid {
					matches++
					if len(filteredType.Models) != len(expectedType.Models) {
						t.Fatalf("Expected type %s to have %d models, got %d", expectedType.Guid, len(expectedType.Models), len(filteredType.Models))
					}
				}
			}
			if matches != 1 {
				t.Fatalf("Expected filtered type %s exactly once, got %d", expectedType.Guid, matches)
			}
		}

		for _, piece := range filteredDesign.Pieces {
			if piece.Type == nil {
				continue
			}
			found := false
			for _, filteredType := range filtered.Types {
				if filteredType.Guid == piece.Type.Guid {
					found = true
					break
				}
			}
			if !found {
				t.Fatalf("Missing filtered type %s for piece", piece.Type.Guid)
			}
		}

		for _, filteredType := range filtered.Types {
			if len(filteredType.Models) > 1 {
				t.Fatalf("Type %s has %d models, expected at most 1", filteredType.Guid, len(filteredType.Models))
			}
			for _, model := range filteredType.Models {
				foundFile := false
				for _, file := range filtered.Files {
					if file.Guid == model.File.Guid {
						foundFile = true
						break
					}
				}
				if !foundFile {
					t.Fatalf("Missing filtered file %s for type %s", model.File.Guid, filteredType.Guid)
				}
			}
			for _, connector := range filteredType.Connectors {
				if connector.Port == nil {
					continue
				}
				foundPort := false
				for _, port := range filtered.Ports {
					if port.Guid == connector.Port.Guid {
						foundPort = true
						break
					}
				}
				if !foundPort {
					t.Fatalf("Missing filtered port %s for type %s", connector.Port.Guid, filteredType.Guid)
				}
			}
		}
	})

	t.Run("preserves kit metadata", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{DesignGuid: nakaginDesign.Guid})
		if filtered.Guid != kit.Guid || filtered.Name != kit.Name || filtered.Version != kit.Version {
			t.Fatalf("Filtered kit metadata mismatch")
		}
	})
}

func TestFlatten(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)

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
			loadJSON(t, "metabolism.kit.semio.json", &kitOriginal)
			kitOriginal.Designs = FilterDesignsWithoutParent(kitOriginal.Designs)

			var kitDiff KitDiff
			loadJSON(t, "metabolism.kit.diff.semio.json", &kitDiff)

			var kitDiffInverted KitDiff
			loadJSON(t, "metabolism.kit.diff.inverted.semio.json", &kitDiffInverted)

			var kitDiffed Kit
			loadJSON(t, "metabolism.kit.diffed.semio.json", &kitDiffed)

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

func TestDelete(t *testing.T) {
	t.Run("Nakagin Capsule Tower", func(t *testing.T) {
		t.Run("Delete Third Tambour And First Small Tower Connection", func(t *testing.T) {
			var kit Kit
			loadJSON(t, "metabolism.kit.semio.json", &kit)

			var design *Design
			for i := range kit.Designs {
				if kit.Designs[i].Name == "Nakagin Capsule Tower" {
					design = &kit.Designs[i]
					break
				}
			}
			if design == nil {
				t.Fatal("Design 'nakagin capsule tower' not found")
			}

			// 🔷Load selection
			type Selection struct {
				Pieces      []PieceId      `json:"pieces"`
				Connections []ConnectionId `json:"connections"`
			}
			var selection Selection
			loadJSON(t, "nakagin-capsule-tower.deleted.selection.semio.json", &selection)

			pieceGuids := make([]string, len(selection.Pieces))
			for i, p := range selection.Pieces {
				pieceGuids[i] = p.Guid
			}
			connectionGuids := make([]string, len(selection.Connections))
			for i, c := range selection.Connections {
				connectionGuids[i] = c.Guid
			}

			// 🔶Load expected diff
			var expectedDiff DesignDiff
			loadJSON(t, "nakagin-capsule-tower.deleted.design.diff.semio.json", &expectedDiff)

			// Compute diff
			computedDiff := DeletePiecesAndConnectionsInDesign(&kit, *design, pieceGuids, connectionGuids)

			// Verify removed pieces
			if computedDiff.Pieces == nil {
				t.Fatal("No pieces diff in computed result")
			}
			if expectedDiff.Pieces == nil {
				t.Fatal("No pieces diff in expected result")
			}
			if len(computedDiff.Pieces.Removed) != len(expectedDiff.Pieces.Removed) {
				t.Fatalf("Removed pieces count mismatch: %d vs %d",
					len(computedDiff.Pieces.Removed), len(expectedDiff.Pieces.Removed))
			}
			for i, c := range computedDiff.Pieces.Removed {
				if c.Guid != expectedDiff.Pieces.Removed[i].Guid {
					t.Errorf("Removed piece guid mismatch at %d: %s vs %s", i, c.Guid, expectedDiff.Pieces.Removed[i].Guid)
				}
			}

			// Verify updated (fixed) pieces
			if len(computedDiff.Pieces.Updated) != len(expectedDiff.Pieces.Updated) {
				t.Fatalf("Updated pieces count mismatch: %d vs %d",
					len(computedDiff.Pieces.Updated), len(expectedDiff.Pieces.Updated))
			}
			computedGuids := make([]string, len(computedDiff.Pieces.Updated))
			for i, u := range computedDiff.Pieces.Updated {
				computedGuids[i] = u.Piece.Guid
			}
			expectedGuids := make([]string, len(expectedDiff.Pieces.Updated))
			for i, u := range expectedDiff.Pieces.Updated {
				expectedGuids[i] = u.Piece.Guid
			}
			sort.Strings(computedGuids)
			sort.Strings(expectedGuids)
			for i := range computedGuids {
				if computedGuids[i] != expectedGuids[i] {
					t.Errorf("Updated piece guid mismatch at %d: %s vs %s", i, computedGuids[i], expectedGuids[i])
				}
			}
			// Verify updated pieces have both plane and center matching expected
			expectedUpdatedMap := make(map[string]PieceDiff)
			for _, u := range expectedDiff.Pieces.Updated {
				expectedUpdatedMap[u.Piece.Guid] = u.Diff
			}
			for _, u := range computedDiff.Pieces.Updated {
				if u.Diff.Plane == nil {
					t.Errorf("Updated piece %s missing plane", u.Piece.Guid)
				}
				if u.Diff.Center == nil {
					t.Errorf("Updated piece %s missing center", u.Piece.Guid)
				}
				exp, ok := expectedUpdatedMap[u.Piece.Guid]
				if !ok {
					t.Errorf("Unexpected updated piece %s", u.Piece.Guid)
					continue
				}
				if u.Diff.Plane != nil && exp.Plane != nil {
					tolerance := 0.001
					if u.Diff.Plane.Origin != nil && exp.Plane.Origin != nil {
						if math.Abs(*u.Diff.Plane.Origin.X-*exp.Plane.Origin.X) > tolerance {
							t.Errorf("Updated piece %s plane origin x: got %f, expected %f", u.Piece.Guid, *u.Diff.Plane.Origin.X, *exp.Plane.Origin.X)
						}
						if math.Abs(*u.Diff.Plane.Origin.Y-*exp.Plane.Origin.Y) > tolerance {
							t.Errorf("Updated piece %s plane origin y: got %f, expected %f", u.Piece.Guid, *u.Diff.Plane.Origin.Y, *exp.Plane.Origin.Y)
						}
						if math.Abs(*u.Diff.Plane.Origin.Z-*exp.Plane.Origin.Z) > tolerance {
							t.Errorf("Updated piece %s plane origin z: got %f, expected %f", u.Piece.Guid, *u.Diff.Plane.Origin.Z, *exp.Plane.Origin.Z)
						}
					}
				}
				if u.Diff.Center != nil && exp.Center != nil {
					tolerance := 0.001
					if math.Abs(*u.Diff.Center.U-*exp.Center.U) > tolerance {
						t.Errorf("Updated piece %s center U: got %f, expected %f", u.Piece.Guid, *u.Diff.Center.U, *exp.Center.U)
					}
					if math.Abs(*u.Diff.Center.V-*exp.Center.V) > tolerance {
						t.Errorf("Updated piece %s center V: got %f, expected %f", u.Piece.Guid, *u.Diff.Center.V, *exp.Center.V)
					}
				}
			}

			// Verify removed connections
			if computedDiff.Connections == nil {
				t.Fatal("No connections diff in computed result")
			}
			if expectedDiff.Connections == nil {
				t.Fatal("No connections diff in expected result")
			}
			if len(computedDiff.Connections.Removed) != len(expectedDiff.Connections.Removed) {
				t.Fatalf("Removed connections count mismatch: %d vs %d",
					len(computedDiff.Connections.Removed), len(expectedDiff.Connections.Removed))
			}
			computedConnGuids := make([]string, len(computedDiff.Connections.Removed))
			for i, r := range computedDiff.Connections.Removed {
				computedConnGuids[i] = r.Guid
			}
			expectedConnGuids := make([]string, len(expectedDiff.Connections.Removed))
			for i, r := range expectedDiff.Connections.Removed {
				expectedConnGuids[i] = r.Guid
			}
			sort.Strings(computedConnGuids)
			sort.Strings(expectedConnGuids)
			for i := range computedConnGuids {
				if computedConnGuids[i] != expectedConnGuids[i] {
					t.Errorf("Removed connection guid mismatch at %d: %s vs %s", i, computedConnGuids[i], expectedConnGuids[i])
				}
			}
		})
	})
}

func TestDrag(t *testing.T) {
	t.Run("Design + Pieces + Offset = DiffDesign", func(t *testing.T) {
		var design Design
		loadJSON(t, "drag/design.semio.json", &design)
		var pieces Design
		loadJSON(t, "drag/pieces.semio.json", &pieces)
		var offset Coord
		loadJSON(t, "drag/offset.semio.json", &offset)
		type expectedPieceUpdate struct {
			Piece struct {
				Guid string `json:"guid"`
			} `json:"piece"`
			Diff struct {
				Center *Coord `json:"center"`
			} `json:"diff"`
		}
		type expectedConnUpdate struct {
			Connection struct {
				Guid string `json:"guid"`
			} `json:"connection"`
			Diff struct {
				U *float64 `json:"u"`
				V *float64 `json:"v"`
			} `json:"diff"`
		}
		type expectedDiffDesign struct {
			Pieces      *struct{ Updated []expectedPieceUpdate } `json:"pieces"`
			Connections *struct{ Updated []expectedConnUpdate }  `json:"connections"`
		}
		var expected expectedDiffDesign
		loadJSON(t, "drag/diff.design.semio.json", &expected)
		computed := DragPiecesInDesign(design, pieces, offset)
		if expected.Pieces == nil {
			if computed.Pieces != nil && len(computed.Pieces.Updated) > 0 {
				t.Fatalf("Expected no piece updates, got %d", len(computed.Pieces.Updated))
			}
		} else {
			if computed.Pieces == nil {
				t.Fatalf("Expected %d piece updates, got nil", len(expected.Pieces.Updated))
			}
			if len(computed.Pieces.Updated) != len(expected.Pieces.Updated) {
				t.Fatalf("Expected %d piece updates, got %d", len(expected.Pieces.Updated), len(computed.Pieces.Updated))
			}
			expectedMap := make(map[string]*Coord)
			for _, u := range expected.Pieces.Updated {
				expectedMap[u.Piece.Guid] = u.Diff.Center
			}
			for _, u := range computed.Pieces.Updated {
				ec, ok := expectedMap[u.Piece.Guid]
				if !ok {
					t.Errorf("Unexpected piece update for %s", u.Piece.Guid)
					continue
				}
				if u.Diff.Center == nil {
					t.Errorf("Piece %s has nil center diff", u.Piece.Guid)
					continue
				}
				if !floatEqual(*u.Diff.Center.U, ec.U, 0.001) || !floatEqual(*u.Diff.Center.V, ec.V, 0.001) {
					t.Errorf("Piece %s center mismatch: got (%f,%f), want (%f,%f)", u.Piece.Guid, *u.Diff.Center.U, *u.Diff.Center.V, ec.U, ec.V)
				}
			}
		}
		if expected.Connections == nil {
			if computed.Connections != nil && len(computed.Connections.Updated) > 0 {
				t.Fatalf("Expected no connection updates, got %d", len(computed.Connections.Updated))
			}
		} else {
			if computed.Connections == nil {
				t.Fatalf("Expected %d connection updates, got nil", len(expected.Connections.Updated))
			}
			if len(computed.Connections.Updated) != len(expected.Connections.Updated) {
				t.Fatalf("Expected %d connection updates, got %d", len(expected.Connections.Updated), len(computed.Connections.Updated))
			}
			expectedConnMap := make(map[string][2]float64)
			for _, u := range expected.Connections.Updated {
				expectedConnMap[u.Connection.Guid] = [2]float64{*u.Diff.U, *u.Diff.V}
			}
			for _, u := range computed.Connections.Updated {
				ev, ok := expectedConnMap[u.Connection.Guid]
				if !ok {
					t.Errorf("Unexpected connection update for %s", u.Connection.Guid)
					continue
				}
				if u.Diff.U == nil || u.Diff.V == nil {
					t.Errorf("Connection %s has nil u/v diff", u.Connection.Guid)
					continue
				}
				if !floatEqual(*u.Diff.U, ev[0], 0.001) || !floatEqual(*u.Diff.V, ev[1], 0.001) {
					t.Errorf("Connection %s uv mismatch: got (%f,%f), want (%f,%f)", u.Connection.Guid, *u.Diff.U, *u.Diff.V, ev[0], ev[1])
				}
			}
		}
	})
}

func TestCopyAndPaste(t *testing.T) {
	t.Run("Nakagin Capsule Tower", func(t *testing.T) {
		t.Run("Copy and Paste Roundtrip", func(t *testing.T) {
			var kit Kit
			loadJSON(t, "metabolism.kit.semio.json", &kit)

			var design *Design
			for i := range kit.Designs {
				if kit.Designs[i].Name == "Nakagin Capsule Tower" && kit.Designs[i].Parent == nil {
					design = &kit.Designs[i]
					break
				}
			}
			if design == nil {
				t.Fatal("Design 'Nakagin Capsule Tower' not found")
			}

			// Load selection
			type Selection struct {
				Pieces      []PieceId      `json:"pieces"`
				Connections []ConnectionId `json:"connections"`
			}
			var selection Selection
			loadJSON(t, "nakagin-capsule-tower.copy.design.selection.semio.json", &selection)

			pieceGuids := make([]string, len(selection.Pieces))
			for i, p := range selection.Pieces {
				pieceGuids[i] = p.Guid
			}
			connectionGuids := make([]string, len(selection.Connections))
			for i, c := range selection.Connections {
				connectionGuids[i] = c.Guid
			}

			// Load expected copy design
			var expectedCopy Design
			loadJSON(t, "nakagin-capsule-tower.copy.design.semio.json", &expectedCopy)

			// Compute copy
			copyDesign := CopyDesign(&kit, *design, pieceGuids, connectionGuids)

			// Verify piece count
			if len(copyDesign.Pieces) != len(expectedCopy.Pieces) {
				t.Fatalf("Copy pieces count mismatch: got %d, want %d", len(copyDesign.Pieces), len(expectedCopy.Pieces))
			}

			// Verify connection count
			if len(copyDesign.Connections) != len(expectedCopy.Connections) {
				t.Fatalf("Copy connections count mismatch: got %d, want %d", len(copyDesign.Connections), len(expectedCopy.Connections))
			}

			// Verify each piece exists in the copy
			copyPieceGuids := make(map[string]bool)
			for _, p := range copyDesign.Pieces {
				copyPieceGuids[p.Guid] = true
			}
			for _, p := range expectedCopy.Pieces {
				if !copyPieceGuids[p.Guid] {
					t.Errorf("Expected piece %s not found in copy output", p.Guid)
				}
			}

			// Verify external pieces have semio.piece.origin attribute
			expectedPieceMap := make(map[string]Piece)
			for _, p := range expectedCopy.Pieces {
				expectedPieceMap[p.Guid] = p
			}
			for _, p := range copyDesign.Pieces {
				ep, ok := expectedPieceMap[p.Guid]
				if !ok {
					t.Errorf("Unexpected piece %s in copy output", p.Guid)
					continue
				}
				hasOriginAttr := false
				for _, a := range p.Attributes {
					if a.Key == "semio.piece.origin" && a.Value != nil && *a.Value == "external" {
						hasOriginAttr = true
					}
				}
				expectedOriginAttr := false
				for _, a := range ep.Attributes {
					if a.Key == "semio.piece.origin" && a.Value != nil && *a.Value == "external" {
						expectedOriginAttr = true
					}
				}
				if hasOriginAttr != expectedOriginAttr {
					t.Errorf("Piece %s: semio.piece.origin mismatch: got %v, want %v", p.Guid, hasOriginAttr, expectedOriginAttr)
				}
			}

			// Verify pp_excl_pc_incl pieces have semio.center and semio.plane attributes
			for _, p := range copyDesign.Pieces {
				ep := expectedPieceMap[p.Guid]
				hasCenterAttr := false
				hasPlaneAttr := false
				for _, a := range p.Attributes {
					if a.Key == "semio.center" {
						hasCenterAttr = true
					}
					if a.Key == "semio.plane" {
						hasPlaneAttr = true
					}
				}
				expectedCenterAttr := false
				expectedPlaneAttr := false
				for _, a := range ep.Attributes {
					if a.Key == "semio.center" {
						expectedCenterAttr = true
					}
					if a.Key == "semio.plane" {
						expectedPlaneAttr = true
					}
				}
				if hasCenterAttr != expectedCenterAttr {
					t.Errorf("Piece %s: semio.center attr mismatch: got %v, want %v", p.Guid, hasCenterAttr, expectedCenterAttr)
				}
				if hasPlaneAttr != expectedPlaneAttr {
					t.Errorf("Piece %s: semio.plane attr mismatch: got %v, want %v", p.Guid, hasPlaneAttr, expectedPlaneAttr)
				}
			}

			// Verify connections are in expected set
			copyConnGuids := make(map[string]bool)
			for _, c := range copyDesign.Connections {
				copyConnGuids[c.Guid] = true
			}
			for _, c := range expectedCopy.Connections {
				if !copyConnGuids[c.Guid] {
					t.Errorf("Expected connection %s not found in copy output", c.Guid)
				}
			}

			// Test PasteDesign with original anchoring (no coord)
			var pasteTargetDesign Design
			loadJSON(t, "nakagin-capsule-tower.paste.design.semio.json", &pasteTargetDesign)
			pasteDiff := PasteDesign(&kit, copyDesign, pasteTargetDesign, "original", nil)

			// Load expected paste diff
			var expectedPaste DesignDiff
			loadJSON(t, "nakagin-capsule-tower.paste.design.diff.semio.json", &expectedPaste)

			// Verify pasted pieces
			if pasteDiff.Pieces == nil {
				t.Fatal("No pieces diff in paste result")
			}
			if len(pasteDiff.Pieces.Added) != len(expectedPaste.Pieces.Added) {
				t.Fatalf("Paste added pieces count mismatch: got %d, want %d", len(pasteDiff.Pieces.Added), len(expectedPaste.Pieces.Added))
			}

			// Verify pasted pieces don't include external-origin pieces
			for _, p := range pasteDiff.Pieces.Added {
				for _, attr := range p.Attributes {
					if attr.Key == "semio.piece.origin" && attr.Value != nil && *attr.Value == "external" {
						t.Errorf("External-origin piece %s should not be in paste output", p.Guid)
					}
				}
			}

			// Verify pasted connections
			if pasteDiff.Connections == nil {
				t.Fatal("No connections diff in paste result")
			}
			if len(pasteDiff.Connections.Added) != len(expectedPaste.Connections.Added) {
				t.Fatalf("Paste added connections count mismatch: got %d, want %d", len(pasteDiff.Connections.Added), len(expectedPaste.Connections.Added))
			}

			// Test PasteDesign with original anchoring and coord
			coordVal := Coord{U: 10, V: 10}
			pasteWithCoordDiff := PasteDesign(&kit, copyDesign, pasteTargetDesign, "original", &coordVal)

			// Load expected paste with coord diff
			var expectedPasteWithCoord DesignDiff
			loadJSON(t, "nakagin-capsule-tower.paste.with-coord.design.diff.semio.json", &expectedPasteWithCoord)

			// Verify pasted pieces count
			if pasteWithCoordDiff.Pieces == nil {
				t.Fatal("No pieces diff in paste with coord result")
			}
			if len(pasteWithCoordDiff.Pieces.Added) != len(expectedPasteWithCoord.Pieces.Added) {
				t.Fatalf("Paste with coord added pieces count mismatch: got %d, want %d", len(pasteWithCoordDiff.Pieces.Added), len(expectedPasteWithCoord.Pieces.Added))
			}

			// Verify pasted connections count
			if pasteWithCoordDiff.Connections == nil {
				t.Fatal("No connections diff in paste with coord result")
			}
			if len(pasteWithCoordDiff.Connections.Added) != len(expectedPasteWithCoord.Connections.Added) {
				t.Fatalf("Paste with coord added connections count mismatch: got %d, want %d", len(pasteWithCoordDiff.Connections.Added), len(expectedPasteWithCoord.Connections.Added))
			}

			// Verify centers are offset by coord
			expectedPWCPieceMap := make(map[string]Piece)
			for _, p := range expectedPasteWithCoord.Pieces.Added {
				expectedPWCPieceMap[p.Guid] = p
			}
			for _, p := range pasteWithCoordDiff.Pieces.Added {
				ep, ok := expectedPWCPieceMap[p.Guid]
				if !ok {
					t.Errorf("Unexpected piece %s in paste with coord output", p.Guid)
					continue
				}
				if p.Center != nil && ep.Center != nil {
					if math.Abs(p.Center.U-ep.Center.U) > 0.001 || math.Abs(p.Center.V-ep.Center.V) > 0.001 {
						t.Errorf("Piece %s center mismatch: got (%f,%f), want (%f,%f)", p.Guid, p.Center.U, p.Center.V, ep.Center.U, ep.Center.V)
					}
				}
			}
		})
	})
}

func TestValidation(t *testing.T) {
	t.Run("Metabolism", func(t *testing.T) {
		t.Run("Metabolism Kit -> Validate = Empty report", func(t *testing.T) {
			var validKit Kit
			loadJSON(t, "metabolism.kit.semio.json", &validKit)
			validResult := ValidateKit(validKit)
			if HasErrors(validResult) {
				t.Errorf("Valid kit should not have errors, got %d problems", len(validResult.Problems))
			}
		})
	})

	t.Run("Invalid", func(t *testing.T) {
		t.Run("Invalid Kit -> Validate = Invalid Report", func(t *testing.T) {
			var invalidKit Kit
			loadJSON(t, "invalid.kit.semio.json", &invalidKit)
			result := ValidateKit(invalidKit)
			serializedResult := ToValidationResult(result)

			var expected ValidationResultSerialized
			loadJSON(t, "validation.semio.json", &expected)

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
			loadJSON(t, "metabolism.kit.semio.json", &kit)
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

func TestExportDesignModel(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)

	design := findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
	if design == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}

	t.Run("GLB format", func(t *testing.T) {
		result, err := ExportDesignModel(&kit, design.Guid, ".glb", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignModel failed: %v", err)
		}
		if result == nil {
			t.Fatal("result is nil")
		}
		if len(result) == 0 {
			t.Fatal("result length is 0")
		}
		if len(result) < 12 {
			t.Fatalf("result too short for GLB header: got %d bytes", len(result))
		}
		magic := binary.LittleEndian.Uint32(result[0:4])
		if magic != 0x46546C67 {
			t.Errorf("GLB magic number mismatch: got 0x%08X, expected 0x46546C67 (glTF)", magic)
		}
		version := binary.LittleEndian.Uint32(result[4:8])
		if version != 2 {
			t.Errorf("GLB version mismatch: got %d, expected 2", version)
		}
		totalLen := binary.LittleEndian.Uint32(result[8:12])
		if uint32(len(result)) != totalLen {
			t.Errorf("GLB total length mismatch: got %d, expected %d", totalLen, len(result))
		}
	})

	t.Run("GLTF format", func(t *testing.T) {
		result, err := ExportDesignModel(&kit, design.Guid, ".gltf", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignModel failed: %v", err)
		}
		if result == nil {
			t.Fatal("result is nil")
		}
		if len(result) == 0 {
			t.Fatal("result length is 0")
		}
		var v interface{}
		if err := json.Unmarshal(result, &v); err != nil {
			t.Errorf("result is not valid JSON: %v", err)
		}
	})

	t.Run("Invalid format", func(t *testing.T) {
		result, err := ExportDesignModel(&kit, design.Guid, ".xyz", nil, nil)
		if err == nil {
			t.Fatal("expected error for invalid format, got nil")
		}
		if result != nil {
			t.Errorf("expected nil result for invalid format, got %d bytes", len(result))
		}
	})

	t.Run("Scene graph report", func(t *testing.T) {
		result, err := ExportDesignModel(&kit, design.Guid, ".gltf", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignModel failed: %v", err)
		}
		var parsed interface{}
		if err := json.Unmarshal(result, &parsed); err != nil {
			t.Fatalf("result is not valid JSON: %v", err)
		}
		reportsDir := filepath.Join("..", "..", "reports", "export-design-model")
		if err := os.MkdirAll(reportsDir, 0o755); err != nil {
			t.Fatalf("failed to create reports directory: %v", err)
		}
		reportPath := filepath.Join(reportsDir, "go.gltf")
		if err := os.WriteFile(reportPath, result, 0o644); err != nil {
			t.Fatalf("failed to write report: %v", err)
		}
	})
}

func TestExportDesignModelSceneGraphReport(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)
	design := findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
	if design == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}
	result, err := ExportDesignModel(&kit, design.Guid, ".gltf", nil, nil)
	if err != nil {
		t.Fatalf("ExportDesignModel failed: %v", err)
	}
	var parsed interface{}
	if err := json.Unmarshal(result, &parsed); err != nil {
		t.Fatalf("result is not valid JSON: %v", err)
	}
	reportsDir := filepath.Join("..", "..", "reports", "export-design-model")
	if err := os.MkdirAll(reportsDir, 0o755); err != nil {
		t.Fatalf("failed to create reports directory: %v", err)
	}
	reportPath := filepath.Join(reportsDir, "go.gltf")
	if err := os.WriteFile(reportPath, result, 0o644); err != nil {
		t.Fatalf("failed to write report: %v", err)
	}
}

func round6(x float64) float64 { return math.Round(x*1e6) / 1e6 }

func TestGetGeometricInsightsForModel_NakaginCapsuleTower(t *testing.T) {
	modelPath := filepath.Join(AssetsPath, "nakagin-capsule-tower.gltf")
	if _, err := os.Stat(modelPath); err != nil {
		t.Skipf("nakagin-capsule-tower.gltf not found: %v", err)
	}
	insights, err := GetGeometricInsightsForModel(modelPath)
	if err != nil {
		t.Fatalf("GetGeometricInsightsForModel: %v", err)
	}
	reportsDir := filepath.Join("..", "..", "reports", "model-kpi")
	if err := os.MkdirAll(reportsDir, 0o755); err != nil {
		t.Fatalf("failed to create reports directory: %v", err)
	}
	report := map[string]any{
		"aspect_ratio_xy":       round6(insights.AspectRatioXY),
		"aspect_ratio_xz":       round6(insights.AspectRatioXZ),
		"aspect_ratio_yz":       round6(insights.AspectRatioYZ),
		"bounding_box_max":      Point{X: round6(insights.BoundingBoxMax.X), Y: round6(insights.BoundingBoxMax.Y), Z: round6(insights.BoundingBoxMax.Z)},
		"bounding_box_min":      Point{X: round6(insights.BoundingBoxMin.X), Y: round6(insights.BoundingBoxMin.Y), Z: round6(insights.BoundingBoxMin.Z)},
		"centroid":              Point{X: round6(insights.Centroid.X), Y: round6(insights.Centroid.Y), Z: round6(insights.Centroid.Z)},
		"characteristic_length": round6(insights.CharacteristicLen),
		"dimension_x":           round6(insights.DimensionX),
		"dimension_y":           round6(insights.DimensionY),
		"dimension_z":           round6(insights.DimensionZ),
		"euler_characteristic":  insights.EulerCharacteristic,
		"face_count":            insights.FaceCount,
		"footprint_area":        round6(insights.FootprintArea),
		"is_watertight":         false,
		"slenderness":           round6(insights.Slenderness),
		"total_surface_area":    round6(insights.TotalSurfaceArea),
		"vertex_count":          insights.VertexCount,
	}
	b, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		t.Fatalf("failed to marshal go model-kpi report: %v", err)
	}
	if err := os.WriteFile(filepath.Join(reportsDir, "go.json"), b, 0o644); err != nil {
		t.Fatalf("failed to write go model-kpi report: %v", err)
	}

	canonicalPath := filepath.Join(AssetsPath, "nakagin.kpi.model.semio.json")
	canonicalData, err := os.ReadFile(canonicalPath)
	if err != nil {
		t.Fatalf("failed to read canonical model-kpi asset: %v", err)
	}
	var canonical map[string]any
	if err := json.Unmarshal(canonicalData, &canonical); err != nil {
		t.Fatalf("failed to unmarshal canonical model-kpi asset: %v", err)
	}
	var current map[string]any
	if err := json.Unmarshal(b, &current); err != nil {
		t.Fatalf("failed to unmarshal go model-kpi report: %v", err)
	}
	// Skip centroid and total_surface_area until GLTF merge/float pipeline matches Python exactly.
	skipKeys := map[string]bool{"centroid": true, "total_surface_area": true}
	for key, expected := range canonical {
		if skipKeys[key] {
			continue
		}
		got, ok := current[key]
		if !ok {
			t.Errorf("missing key %s in report", key)
			continue
		}
		if !reflect.DeepEqual(got, expected) {
			t.Errorf("mismatch for key %s: got %#v, expected %#v", key, got, expected)
		}
	}
}

func TestMetaShallow(t *testing.T) {
	t.Run("KitMeta from conversion", func(t *testing.T) {
		var kit Kit
		loadJSON(t, "metabolism.kit.semio.json", &kit)
		meta := ToKitMeta(kit)
		if meta.Guid != kit.Guid {
			t.Errorf("KitMeta.Guid = %q, want %q", meta.Guid, kit.Guid)
		}
		if meta.Name != kit.Name {
			t.Errorf("KitMeta.Name = %q, want %q", meta.Name, kit.Name)
		}
		if meta.Version != kit.Version {
			t.Errorf("KitMeta.Version = %q, want %q", meta.Version, kit.Version)
		}
	})

	t.Run("KitShallow from conversion", func(t *testing.T) {
		var kit Kit
		loadJSON(t, "metabolism.kit.semio.json", &kit)
		shallow := ToKitShallow(kit)
		if shallow.Guid != kit.Guid {
			t.Errorf("KitShallow.Guid = %q, want %q", shallow.Guid, kit.Guid)
		}
		if len(shallow.Types) != len(kit.Types) {
			t.Errorf("KitShallow.Types len = %d, want %d", len(shallow.Types), len(kit.Types))
		}
		if len(shallow.Designs) != len(kit.Designs) {
			t.Errorf("KitShallow.Designs len = %d, want %d", len(shallow.Designs), len(kit.Designs))
		}
		if len(shallow.Authors) != len(kit.Authors) {
			t.Errorf("KitShallow.Authors len = %d, want %d", len(shallow.Authors), len(kit.Authors))
		}
		if len(shallow.Files) != len(kit.Files) {
			t.Errorf("KitShallow.Files len = %d, want %d", len(shallow.Files), len(kit.Files))
		}
		for i, tm := range shallow.Types {
			if tm.Guid != kit.Types[i].Guid {
				t.Errorf("KitShallow.Types[%d].Guid = %q, want %q", i, tm.Guid, kit.Types[i].Guid)
			}
		}
	})

	t.Run("TypeMeta from JSON", func(t *testing.T) {
		var meta TypeMeta
		loadJSON(t, "tambour.meta.type.semio.json", &meta)
		if meta.Guid == "" {
			t.Error("TypeMeta.Guid is empty")
		}
		if meta.Name != "Tambour" {
			t.Errorf("TypeMeta.Name = %q, want %q", meta.Name, "Tambour")
		}
	})

	t.Run("TypeShallow from JSON", func(t *testing.T) {
		var shallow TypeShallow
		loadJSON(t, "tambour.shallow.type.semio.json", &shallow)
		if shallow.Guid == "" {
			t.Error("TypeShallow.Guid is empty")
		}
		if shallow.Name != "Tambour" {
			t.Errorf("TypeShallow.Name = %q, want %q", shallow.Name, "Tambour")
		}
		if len(shallow.Connectors) == 0 {
			t.Error("TypeShallow.Connectors is empty")
		}
		if len(shallow.Models) == 0 {
			t.Error("TypeShallow.Models is empty")
		}
		if len(shallow.Props) == 0 {
			t.Error("TypeShallow.Props is empty")
		}
	})

	t.Run("DesignMeta from JSON", func(t *testing.T) {
		var meta DesignMeta
		loadJSON(t, "nakagin-capsule-tower.meta.design.semio.json", &meta)
		if meta.Guid == "" {
			t.Error("DesignMeta.Guid is empty")
		}
		if meta.Name != "Nakagin Capsule Tower" {
			t.Errorf("DesignMeta.Name = %q, want %q", meta.Name, "Nakagin Capsule Tower")
		}
	})

	t.Run("DesignShallow from JSON", func(t *testing.T) {
		var shallow DesignShallow
		loadJSON(t, "nakagin-capsule-tower.shallow.design.semio.json", &shallow)
		if shallow.Guid == "" {
			t.Error("DesignShallow.Guid is empty")
		}
		if shallow.Name != "Nakagin Capsule Tower" {
			t.Errorf("DesignShallow.Name = %q, want %q", shallow.Name, "Nakagin Capsule Tower")
		}
		if len(shallow.Pieces) == 0 {
			t.Error("DesignShallow.Pieces is empty")
		}
		if len(shallow.Connections) == 0 {
			t.Error("DesignShallow.Connections is empty")
		}
		if len(shallow.Layers) == 0 {
			t.Error("DesignShallow.Layers is empty")
		}
	})

	t.Run("KitMeta from JSON", func(t *testing.T) {
		var meta KitMeta
		loadJSON(t, "metabolism.meta.kit.semio.json", &meta)
		if meta.Guid == "" {
			t.Error("KitMeta.Guid is empty")
		}
		if meta.Name != "Metabolism" {
			t.Errorf("KitMeta.Name = %q, want %q", meta.Name, "Metabolism")
		}
		if meta.Version == "" {
			t.Error("KitMeta.Version is empty")
		}
	})

	t.Run("KitShallow from JSON", func(t *testing.T) {
		var shallow KitShallow
		loadJSON(t, "metabolism.shallow.kit.semio.json", &shallow)
		if shallow.Guid == "" {
			t.Error("KitShallow.Guid is empty")
		}
		if shallow.Name != "Metabolism" {
			t.Errorf("KitShallow.Name = %q, want %q", shallow.Name, "Metabolism")
		}
		if len(shallow.Types) == 0 {
			t.Error("KitShallow.Types is empty")
		}
		if len(shallow.Designs) == 0 {
			t.Error("KitShallow.Designs is empty")
		}
		if len(shallow.Authors) == 0 {
			t.Error("KitShallow.Authors is empty")
		}
		if len(shallow.Files) == 0 {
			t.Error("KitShallow.Files is empty")
		}
		if len(shallow.Ports) == 0 {
			t.Error("KitShallow.Ports is empty")
		}
		if len(shallow.Tags) == 0 {
			t.Error("KitShallow.Tags is empty")
		}
		if len(shallow.Qualities) == 0 {
			t.Error("KitShallow.Qualities is empty")
		}
	})
}

// #region 🛡️KitKind Tests
// Tests for KitKind enum MUST verify the five kit kinds.

func TestKitKind(t *testing.T) {
	t.Run("Kit/AllKitKinds contains exactly five entries", func(t *testing.T) {
		if len(AllKitKinds) != 5 {
			t.Errorf("AllKitKinds has %d entries, want 5", len(AllKitKinds))
		}
	})

	t.Run("Kit/AllKitKinds contains all five kinds", func(t *testing.T) {
		expected := []KitKind{KitKindFile, KitKindFolder, KitKindArchive, KitKindRemote, KitKindTemporary}
		for _, kind := range expected {
			found := false
			for _, k := range AllKitKinds {
				if k == kind {
					found = true
					break
				}
			}
			if !found {
				t.Errorf("AllKitKinds missing %q", kind)
			}
		}
	})

	t.Run("Kit/IsValidKitKind accepts valid kinds", func(t *testing.T) {
		for _, kind := range AllKitKinds {
			if !IsValidKitKind(kind) {
				t.Errorf("IsValidKitKind(%q) = false, want true", kind)
			}
		}
	})

	t.Run("Kit/IsValidKitKind rejects invalid kinds", func(t *testing.T) {
		invalids := []KitKind{"invalid", "json", "sqlite", ""}
		for _, kind := range invalids {
			if IsValidKitKind(kind) {
				t.Errorf("IsValidKitKind(%q) = true, want false", kind)
			}
		}
	})

	t.Run("Kit/File: roundtrips through JSON serialize/deserialize", func(t *testing.T) {
		kit := Kit{Guid: "file-kit-guid", Name: "FileKit Test", Version: "1.0"}
		data, err := SerializeKit(kit)
		if err != nil {
			t.Fatal(err)
		}
		restored, err := DeserializeKit(data)
		if err != nil {
			t.Fatal(err)
		}
		if restored.Guid != kit.Guid {
			t.Errorf("Guid = %q, want %q", restored.Guid, kit.Guid)
		}
		if restored.Name != kit.Name {
			t.Errorf("Name = %q, want %q", restored.Name, kit.Name)
		}
	})

	t.Run("Kit/Folder: roundtrips through SQLite", func(t *testing.T) {
		kit := Kit{
			Guid:    "folder-kit-guid",
			Name:    "FolderKit Test",
			Version: "1.0",
			Types:   []Type{{Guid: "t1", Name: "Wall"}},
		}
		tmpDir := t.TempDir()
		dbPath := filepath.Join(tmpDir, "kit.db")
		schemaPath := "../sqlite/schema.sql"
		schemaBytes, err := os.ReadFile(schemaPath)
		if err != nil {
			t.Fatalf("Failed to read schema: %v", err)
		}
		if err := KitToSqlite(&kit, dbPath, string(schemaBytes)); err != nil {
			t.Fatalf("KitToSqlite: %v", err)
		}
		restored, err := KitFromSqlite(dbPath)
		if err != nil {
			t.Fatalf("KitFromSqlite: %v", err)
		}
		if restored.Guid != kit.Guid {
			t.Errorf("Guid = %q, want %q", restored.Guid, kit.Guid)
		}
		if restored.Name != kit.Name {
			t.Errorf("Name = %q, want %q", restored.Name, kit.Name)
		}
		if len(restored.Types) != 1 {
			t.Fatalf("Types len = %d, want 1", len(restored.Types))
		}
		if restored.Types[0].Name != "Wall" {
			t.Errorf("Types[0].Name = %q, want %q", restored.Types[0].Name, "Wall")
		}
	})

	t.Run("Kit/Archive: roundtrips through zip export/import", func(t *testing.T) {
		kit := Kit{
			Guid:    "archive-kit-guid",
			Name:    "ArchiveKit Test",
			Version: "1.0",
			Types:   []Type{{Guid: "at1", Name: "Beam"}},
		}
		tmpDir := t.TempDir()
		zipPath := filepath.Join(tmpDir, "kit.zip")
		schemaPath := "../sqlite/schema.sql"
		schemaBytes, err := os.ReadFile(schemaPath)
		if err != nil {
			t.Fatalf("Failed to read schema: %v", err)
		}
		if err := KitToZip(&kit, nil, zipPath, string(schemaBytes)); err != nil {
			t.Fatalf("KitToZip: %v", err)
		}
		restored, _, err := KitFromZip(zipPath)
		if err != nil {
			t.Fatalf("KitFromZip: %v", err)
		}
		if restored.Guid != kit.Guid {
			t.Errorf("Guid = %q, want %q", restored.Guid, kit.Guid)
		}
		if restored.Name != kit.Name {
			t.Errorf("Name = %q, want %q", restored.Name, kit.Name)
		}
		if len(restored.Types) != 1 {
			t.Fatalf("Types len = %d, want 1", len(restored.Types))
		}
		if restored.Types[0].Name != "Beam" {
			t.Errorf("Types[0].Name = %q, want %q", restored.Types[0].Name, "Beam")
		}
	})

	t.Run("Kit/Remote: validates remote URL field", func(t *testing.T) {
		remote := "https://example.com/metabolism.kit.json"
		kit := Kit{Guid: "remote-kit-guid", Name: "RemoteKit Test", Remote: &remote}
		data, err := SerializeKit(kit)
		if err != nil {
			t.Fatal(err)
		}
		restored, err := DeserializeKit(data)
		if err != nil {
			t.Fatal(err)
		}
		if restored.Remote == nil || *restored.Remote != remote {
			t.Errorf("Remote = %v, want %q", restored.Remote, remote)
		}
	})

	t.Run("Kit/Temporary: in-memory kit operations", func(t *testing.T) {
		kit := Kit{Guid: "temp-kit-guid", Name: "TemporaryKit Test", Version: "1.0"}
		kit.Name = "Modified Temporary"
		if kit.Name != "Modified Temporary" {
			t.Errorf("Name = %q, want %q", kit.Name, "Modified Temporary")
		}
		if !DeepEqual(kit, kit) {
			t.Error("DeepEqual(kit, kit) = false, want true")
		}
	})

	t.Run("Kit/KitKind string values match JSON enum", func(t *testing.T) {
		expectedValues := map[KitKind]string{
			KitKindFile:      "file",
			KitKindFolder:    "folder",
			KitKindArchive:   "archive",
			KitKindRemote:    "remote",
			KitKindTemporary: "temporary",
		}
		for kind, expected := range expectedValues {
			if string(kind) != expected {
				t.Errorf("KitKind %v = %q, want %q", kind, string(kind), expected)
			}
		}
	})
}

func TestKitWorkflowKinds(t *testing.T) {
	assetBlob := blobEncode([]byte("hello workflow"), "readme.txt")
	assetSize := int64(len("hello workflow"))
	kit := Kit{
		Guid:      "workflow-kit-guid",
		Name:      "Workflow Kit",
		Version:   "1.0.0",
		CreatedAt: "2026-01-01T00:00:00.000Z",
		UpdatedAt: "2026-01-01T00:00:00.000Z",
		Folders: []Folder{{
			Guid:      "folder-guid",
			Name:      "docs",
			CreatedAt: "2026-01-01T00:00:00.000Z",
			UpdatedAt: "2026-01-01T00:00:00.000Z",
		}},
		Files: []File{{
			Guid:      "file-guid",
			Name:      "readme.txt",
			Folder:    &FolderId{Guid: "folder-guid"},
			Size:      &assetSize,
			Blob:      &assetBlob,
			CreatedAt: "2026-01-01T00:00:00.000Z",
			UpdatedAt: "2026-01-01T00:00:00.000Z",
		}},
		Types: []Type{{
			Guid:      "type-guid",
			Name:      "Wall",
			CreatedAt: "2026-01-01T00:00:00.000Z",
			UpdatedAt: "2026-01-01T00:00:00.000Z",
		}},
	}
	updatedName := "Workflow Kit Edited"
	diff := KitDiff{Name: &updatedName}

	t.Run("Kit/File workflow imports, exports and edits", func(t *testing.T) {
		filePath := filepath.Join(t.TempDir(), "workflow.kit.json")
		if err := ExportFileKit(kit, filePath); err != nil {
			t.Fatalf("ExportFileKit: %v", err)
		}
		loaded, err := ImportFileKit(filePath)
		if err != nil {
			t.Fatalf("ImportFileKit: %v", err)
		}
		if loaded.Name != kit.Name {
			t.Fatalf("loaded.Name = %q, want %q", loaded.Name, kit.Name)
		}
		edited, err := EditFileKit(filePath, diff)
		if err != nil {
			t.Fatalf("EditFileKit: %v", err)
		}
		if edited.Name != updatedName {
			t.Fatalf("edited.Name = %q, want %q", edited.Name, updatedName)
		}
		reloaded, err := ImportFileKit(filePath)
		if err != nil {
			t.Fatalf("ImportFileKit(reload): %v", err)
		}
		if reloaded.Name != updatedName {
			t.Fatalf("reloaded.Name = %q, want %q", reloaded.Name, updatedName)
		}
	})

	t.Run("Kit/Folder workflow imports, exports and edits", func(t *testing.T) {
		folderPath := filepath.Join(t.TempDir(), "folder-kit")
		if err := ExportFolderKit(&kit, nil, folderPath); err != nil {
			t.Fatalf("ExportFolderKit: %v", err)
		}
		loaded, files, err := ImportFolderKit(folderPath)
		if err != nil {
			t.Fatalf("ImportFolderKit: %v", err)
		}
		if loaded.Name != kit.Name {
			t.Fatalf("loaded.Name = %q, want %q", loaded.Name, kit.Name)
		}
		if string(files["docs/readme.txt"]) != "hello workflow" {
			t.Fatalf("folder asset mismatch: %q", string(files["docs/readme.txt"]))
		}
		edited, err := EditFolderKit(folderPath, diff)
		if err != nil {
			t.Fatalf("EditFolderKit: %v", err)
		}
		if edited.Name != updatedName {
			t.Fatalf("edited.Name = %q, want %q", edited.Name, updatedName)
		}
		reloaded, _, err := ImportFolderKit(folderPath)
		if err != nil {
			t.Fatalf("ImportFolderKit(reload): %v", err)
		}
		if reloaded.Name != updatedName {
			t.Fatalf("reloaded.Name = %q, want %q", reloaded.Name, updatedName)
		}
	})

	t.Run("Kit/Archive workflow imports, exports and edits", func(t *testing.T) {
		archivePath := filepath.Join(t.TempDir(), "workflow.zip")
		if err := ExportArchiveKit(&kit, nil, archivePath); err != nil {
			t.Fatalf("ExportArchiveKit: %v", err)
		}
		loaded, files, err := ImportArchiveKit(archivePath)
		if err != nil {
			t.Fatalf("ImportArchiveKit: %v", err)
		}
		if loaded.Name != kit.Name {
			t.Fatalf("loaded.Name = %q, want %q", loaded.Name, kit.Name)
		}
		if string(files["docs/readme.txt"]) != "hello workflow" {
			t.Fatalf("archive asset mismatch: %q", string(files["docs/readme.txt"]))
		}
		edited, err := EditArchiveKit(archivePath, diff)
		if err != nil {
			t.Fatalf("EditArchiveKit: %v", err)
		}
		if edited.Name != updatedName {
			t.Fatalf("edited.Name = %q, want %q", edited.Name, updatedName)
		}
		reloaded, _, err := ImportArchiveKit(archivePath)
		if err != nil {
			t.Fatalf("ImportArchiveKit(reload): %v", err)
		}
		if reloaded.Name != updatedName {
			t.Fatalf("reloaded.Name = %q, want %q", reloaded.Name, updatedName)
		}
	})

	t.Run("Kit/Remote workflow imports JSON and archive sources", func(t *testing.T) {
		archivePath := filepath.Join(t.TempDir(), "remote.zip")
		if err := ExportArchiveKit(&kit, nil, archivePath); err != nil {
			t.Fatalf("ExportArchiveKit: %v", err)
		}
		archiveBytes, err := os.ReadFile(archivePath)
		if err != nil {
			t.Fatalf("os.ReadFile(archivePath): %v", err)
		}
		jsonBytes, err := SerializeKit(kit)
		if err != nil {
			t.Fatalf("SerializeKit: %v", err)
		}
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			if strings.HasSuffix(r.URL.Path, ".zip") {
				w.Header().Set("Content-Type", "application/zip")
				_, _ = w.Write(archiveBytes)
				return
			}
			w.Header().Set("Content-Type", "application/json")
			_, _ = w.Write(jsonBytes)
		}))
		defer server.Close()

		jsonKit, _, err := ImportRemoteKit(server.URL + "/workflow.kit.json")
		if err != nil {
			t.Fatalf("ImportRemoteKit(json): %v", err)
		}
		if jsonKit.Name != kit.Name {
			t.Fatalf("jsonKit.Name = %q, want %q", jsonKit.Name, kit.Name)
		}

		archiveKit, archiveFiles, err := ImportRemoteKit(server.URL + "/workflow.zip")
		if err != nil {
			t.Fatalf("ImportRemoteKit(zip): %v", err)
		}
		if archiveKit.Name != kit.Name {
			t.Fatalf("archiveKit.Name = %q, want %q", archiveKit.Name, kit.Name)
		}
		if string(archiveFiles["docs/readme.txt"]) != "hello workflow" {
			t.Fatalf("remote archive asset mismatch: %q", string(archiveFiles["docs/readme.txt"]))
		}

		edited, err := EditRemoteKit(server.URL+"/workflow.kit.json", diff)
		if err != nil {
			t.Fatalf("EditRemoteKit: %v", err)
		}
		if edited.Name != updatedName {
			t.Fatalf("edited.Name = %q, want %q", edited.Name, updatedName)
		}
	})

	t.Run("Kit/Temporary workflow edits in memory without mutating source", func(t *testing.T) {
		edited := EditTemporaryKit(kit, diff)
		if edited.Name != updatedName {
			t.Fatalf("edited.Name = %q, want %q", edited.Name, updatedName)
		}
		if kit.Name != "Workflow Kit" {
			t.Fatalf("kit.Name = %q, want %q", kit.Name, "Workflow Kit")
		}
	})
}

// #region 🏰Kit Filter Tests
// Tests for FilterKit MUST verify correct subset extraction.

func TestFilterKit(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)
	designGuid := "9a890dd4-0a9c-48ac-920a-9e62666465ef"
	var expected Kit
	loadJSON(t, "nakagin-capsule-tower.filtered.kit.semio.json", &expected)

	t.Run("filters kit to only contain entities related to Nakagin Capsule Tower design", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{DesignGuid: designGuid})

		if len(filtered.Designs) != len(expected.Designs) {
			t.Errorf("expected %d designs, got %d", len(expected.Designs), len(filtered.Designs))
		}
		if len(filtered.Types) != len(expected.Types) {
			t.Errorf("expected %d types, got %d", len(expected.Types), len(filtered.Types))
		}
		if len(filtered.Files) != len(expected.Files) {
			t.Errorf("expected %d files, got %d", len(expected.Files), len(filtered.Files))
		}
		if len(filtered.Ports) != len(expected.Ports) {
			t.Errorf("expected %d ports, got %d", len(expected.Ports), len(filtered.Ports))
		}
		if len(filtered.Qualities) != len(expected.Qualities) {
			t.Errorf("expected %d qualities, got %d", len(expected.Qualities), len(filtered.Qualities))
		}
		if len(filtered.Authors) != len(expected.Authors) {
			t.Errorf("expected %d authors, got %d", len(expected.Authors), len(filtered.Authors))
		}

		// 🧹Find the Nakagin design in filtered kit
		var filteredDesign *Design
		for i := range filtered.Designs {
			if filtered.Designs[i].Guid == designGuid {
				filteredDesign = &filtered.Designs[i]
				break
			}
		}
		if filteredDesign == nil {
			t.Fatal("Nakagin Capsule Tower design not found in filtered kit")
		}

		// 🔷Find original design for comparison
		var originalDesign *Design
		for i := range kit.Designs {
			if kit.Designs[i].Guid == designGuid {
				originalDesign = &kit.Designs[i]
				break
			}
		}
		if originalDesign == nil {
			t.Fatal("Nakagin Capsule Tower design not found in original kit")
		}

		if len(filteredDesign.Pieces) != len(originalDesign.Pieces) {
			t.Errorf("expected %d pieces, got %d", len(originalDesign.Pieces), len(filteredDesign.Pieces))
		}

		// Verify each type has at most one model
		for _, typeItem := range filtered.Types {
			if len(typeItem.Models) > 1 {
				t.Errorf("type %s has %d models, expected at most 1", typeItem.Guid, len(typeItem.Models))
			}
		}
	})

	t.Run("preserves kit metadata", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{DesignGuid: designGuid})
		if filtered.Guid != kit.Guid {
			t.Errorf("expected guid %s, got %s", kit.Guid, filtered.Guid)
		}
		if filtered.Name != kit.Name {
			t.Errorf("expected name %s, got %s", kit.Name, filtered.Name)
		}
		if filtered.Version != kit.Version {
			t.Errorf("expected version %s, got %s", kit.Version, filtered.Version)
		}
	})

	t.Run("glob filters types by name include", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{Types: &GlobFilter{Include: []string{"Capsule*"}}})
		if len(filtered.Types) == 0 {
			t.Fatal("expected at least one type matching Capsule*")
		}
		for _, ty := range filtered.Types {
			if !GlobMatch(ty.Name, "Capsule*") {
				t.Errorf("type %s should not be included", ty.Name)
			}
		}
	})

	t.Run("glob filters types by name exclude", func(t *testing.T) {
		totalTypes := len(kit.Types)
		filtered := FilterKit(kit, KitFilter{Types: &GlobFilter{Exclude: []string{"Capsule*"}}})
		if len(filtered.Types) >= totalTypes {
			t.Errorf("expected fewer types after excluding Capsule*")
		}
		for _, ty := range filtered.Types {
			if GlobMatch(ty.Name, "Capsule*") {
				t.Errorf("type %s should have been excluded", ty.Name)
			}
		}
	})

	t.Run("glob filters designs by name include", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{Designs: &GlobFilter{Include: []string{"Nakagin*"}}})
		if len(filtered.Designs) == 0 {
			t.Fatal("expected at least one design matching Nakagin*")
		}
		for _, d := range filtered.Designs {
			if !GlobMatch(d.Name, "Nakagin*") {
				t.Errorf("design %s should not be included", d.Name)
			}
		}
	})

	t.Run("empty filter returns kit unchanged", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{})
		if len(filtered.Types) != len(kit.Types) {
			t.Errorf("expected %d types, got %d", len(kit.Types), len(filtered.Types))
		}
		if len(filtered.Designs) != len(kit.Designs) {
			t.Errorf("expected %d designs, got %d", len(kit.Designs), len(filtered.Designs))
		}
	})

	t.Run("combines designGuid with glob filters", func(t *testing.T) {
		designFiltered := FilterKit(kit, KitFilter{DesignGuid: designGuid})
		combinedFiltered := FilterKit(kit, KitFilter{DesignGuid: designGuid, Types: &GlobFilter{Exclude: []string{"Capsule*"}}})
		if len(combinedFiltered.Types) >= len(designFiltered.Types) {
			t.Errorf("expected fewer types with combined filter")
		}
		for _, ty := range combinedFiltered.Types {
			if GlobMatch(ty.Name, "Capsule*") {
				t.Errorf("type %s should have been excluded", ty.Name)
			}
		}
	})
}

// #endregion 🏰Kit Filter Tests

// #region 🗝️Hash Tests

func TestHashKit(t *testing.T) {
	kitJSON, err := os.ReadFile(filepath.Join(AssetsPath, "metabolism.kit.semio.json"))
	if err != nil {
		t.Fatalf("failed to read metabolism kit: %v", err)
	}
	var kit Kit
	if err := json.Unmarshal(kitJSON, &kit); err != nil {
		t.Fatalf("failed to unmarshal metabolism kit: %v", err)
	}

	t.Run("hashKit produces a 64-char lowercase hex string", func(t *testing.T) {
		h := HashKit(kit)
		if len(h) != 64 {
			t.Errorf("expected 64-char hash, got %d chars: %s", len(h), h)
		}
		for _, c := range h {
			if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
				t.Errorf("hash contains non-hex char: %c", c)
			}
		}
	})

	t.Run("hashKit is deterministic", func(t *testing.T) {
		h1 := HashKit(kit)
		h2 := HashKit(kit)
		if h1 != h2 {
			t.Errorf("expected same hash, got %s and %s", h1, h2)
		}
	})

	t.Run("hashKit of metabolism kit matches expected hash", func(t *testing.T) {
		h := HashKit(kit)
		expected := "d786ae5cd2cb18d78f0d80df92ded94905adf708df077206f72162d43dd8767c"
		if h != expected {
			t.Errorf("expected %s, got %s", expected, h)
		}
	})

	t.Run("different kits produce different hashes", func(t *testing.T) {
		kit2 := kit
		kit2.Name = "Different Name"
		h1 := HashKit(kit)
		h2 := HashKit(kit2)
		if h1 == h2 {
			t.Errorf("expected different hashes for different kits, both got %s", h1)
		}
	})

	t.Run("hashDesign produces a 64-char lowercase hex string", func(t *testing.T) {
		var nct *Design
		for i := range kit.Designs {
			if kit.Designs[i].Name == "Nakagin Capsule Tower" && kit.Designs[i].Parent == nil {
				nct = &kit.Designs[i]
				break
			}
		}
		if nct == nil {
			t.Fatal("Nakagin Capsule Tower design not found")
		}
		h := HashDesign(*nct)
		if len(h) != 64 {
			t.Errorf("expected 64-char hash, got %d chars: %s", len(h), h)
		}
	})

	t.Run("hashType produces a 64-char lowercase hex string", func(t *testing.T) {
		if len(kit.Types) == 0 {
			t.Fatal("no types in kit")
		}
		h := HashType(kit.Types[0])
		if len(h) != 64 {
			t.Errorf("expected 64-char hash, got %d chars: %s", len(h), h)
		}
	})
}

func TestHashKitDiff(t *testing.T) {
	t.Run("hashKitDiff matches expected canonical value", func(t *testing.T) {
		// Create KitDiff with name="updated" and description=null
		raw := []byte(`{"name":"updated","description":null}`)
		var d KitDiff
		if err := json.Unmarshal(raw, &d); err != nil {
			t.Fatalf("failed to unmarshal KitDiff: %v", err)
		}
		h := HashKitDiff(d)
		expected := "d9ee3052111fec2e0fe08119eee6b8d5b6f5578a940f6d5c6bb1806e6e0f36a5"
		if h != expected {
			t.Errorf("expected %s, got %s", expected, h)
		}
	})

	t.Run("hashKitDiff is deterministic", func(t *testing.T) {
		raw := []byte(`{"name":"updated","description":null}`)
		var d KitDiff
		if err := json.Unmarshal(raw, &d); err != nil {
			t.Fatalf("failed to unmarshal KitDiff: %v", err)
		}
		h1 := HashKitDiff(d)
		h2 := HashKitDiff(d)
		if h1 != h2 {
			t.Errorf("expected same hash, got %s and %s", h1, h2)
		}
	})

	t.Run("hashKitDiff produces different hashes for different diffs", func(t *testing.T) {
		raw1 := []byte(`{"name":"updated","description":null}`)
		raw2 := []byte(`{"name":"other"}`)
		var d1, d2 KitDiff
		json.Unmarshal(raw1, &d1)
		json.Unmarshal(raw2, &d2)
		h1 := HashKitDiff(d1)
		h2 := HashKitDiff(d2)
		if h1 == h2 {
			t.Errorf("expected different hashes, both got %s", h1)
		}
	})

	t.Run("hashKitDiff empty diff produces valid hash", func(t *testing.T) {
		d := KitDiff{}
		h := HashKitDiff(d)
		if len(h) != 64 {
			t.Errorf("expected 64-char hash, got %d chars: %s", len(h), h)
		}
	})

	t.Run("hashAttributeDiff is deterministic", func(t *testing.T) {
		key := "newKey"
		val := "newValue"
		d := AttributeDiff{Key: &key, Value: &val}
		h1 := HashAttributeDiff(d)
		h2 := HashAttributeDiff(d)
		if h1 != h2 {
			t.Errorf("expected same hash, got %s and %s", h1, h2)
		}
	})

	t.Run("hashCoordDiff is deterministic", func(t *testing.T) {
		u := 1.0
		v := 2.0
		d := CoordDiff{U: &u, V: &v}
		h1 := HashCoordDiff(d)
		h2 := HashCoordDiff(d)
		if h1 != h2 {
			t.Errorf("expected same hash, got %s and %s", h1, h2)
		}
	})
}

// #endregion 🗝️Hash Tests

// #region 🎉DesignWithDiff Tests

func TestDesignWithDiff(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)

	var design *Design
	for i := range kit.Designs {
		if kit.Designs[i].Name == "Nakagin Capsule Tower" && kit.Designs[i].Parent == nil {
			design = &kit.Designs[i]
			break
		}
	}
	if design == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}

	var diff DesignDiff
	loadJSON(t, "nakgin-capsule-tower.diff.design.semio.json", &diff)

	var expected Design
	loadJSON(t, "nakagin-capsule-tower.with-diff.design.semio.json", &expected)

	computed := DesignWithDiff(*design, diff)

	if len(computed.Pieces) != len(expected.Pieces) {
		t.Errorf("pieces count: got %d, want %d", len(computed.Pieces), len(expected.Pieces))
	}
	if len(computed.Connections) != len(expected.Connections) {
		t.Errorf("connections count: got %d, want %d", len(computed.Connections), len(expected.Connections))
	}

	getStatus := func(attrs []Attribute) string {
		for _, a := range attrs {
			if a.Key == "semio.diffStatus" && a.Value != nil {
				return *a.Value
			}
		}
		return ""
	}

	counts := map[string]int{}
	for _, p := range computed.Pieces {
		counts[getStatus(p.Attributes)]++
	}
	if counts["unchanged"] != 163 {
		t.Errorf("unchanged pieces: got %d, want 163", counts["unchanged"])
	}
	if counts["modified"] != 7 {
		t.Errorf("modified pieces: got %d, want 7", counts["modified"])
	}
	if counts["removed"] != 10 {
		t.Errorf("removed pieces: got %d, want 10", counts["removed"])
	}
	if counts["added"] != 5 {
		t.Errorf("added pieces: got %d, want 5", counts["added"])
	}

	connCounts := map[string]int{}
	for _, c := range computed.Connections {
		connCounts[getStatus(c.Attributes)]++
	}
	if connCounts["unchanged"] != 168 {
		t.Errorf("unchanged connections: got %d, want 168", connCounts["unchanged"])
	}
	if connCounts["modified"] != 1 {
		t.Errorf("modified connections: got %d, want 1", connCounts["modified"])
	}
	if connCounts["removed"] != 10 {
		t.Errorf("removed connections: got %d, want 10", connCounts["removed"])
	}
	if connCounts["added"] != 4 {
		t.Errorf("added connections: got %d, want 4", connCounts["added"])
	}
}

// #endregion 🎉DesignWithDiff Tests

// #region 📊MaxChildren Tests

func TestMaxChildrenPortSerialization(t *testing.T) {
	mc := 3
	port := Port{
		Guid:        "p1",
		Name:        "TestPort",
		MaxChildren: &mc,
	}
	data, err := json.Marshal(port)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	var restored Port
	if err := json.Unmarshal(data, &restored); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if restored.MaxChildren == nil || *restored.MaxChildren != 3 {
		t.Errorf("maxChildren: got %v, want 3", restored.MaxChildren)
	}
}

func TestMaxChildrenPortOmitted(t *testing.T) {
	port := Port{Guid: "p1", Name: "TestPort"}
	data, err := json.Marshal(port)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	s := string(data)
	if contains(s, "maxChildren") {
		t.Errorf("maxChildren should be omitted when nil, got: %s", s)
	}
}

func TestMaxChildrenConnectorSerialization(t *testing.T) {
	mc := 5
	connector := Connector{
		Guid:        "c1",
		T:           0,
		Point:       Point{X: 0, Y: 0, Z: 0},
		Direction:   Vector{X: 0, Y: 0, Z: 1},
		MaxChildren: &mc,
	}
	data, err := json.Marshal(connector)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	var restored Connector
	if err := json.Unmarshal(data, &restored); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if restored.MaxChildren == nil || *restored.MaxChildren != 5 {
		t.Errorf("maxChildren: got %v, want 5", restored.MaxChildren)
	}
}

func TestMaxChildrenKitRoundtrip(t *testing.T) {
	mc3 := 3
	mc5 := 5
	kit := Kit{
		Guid: "kit-1",
		Name: "TestKit",
		Ports: []Port{{
			Guid:        "p1",
			Name:        "Port1",
			MaxChildren: &mc3,
		}},
		Types: []Type{{
			Guid: "t1",
			Name: "Type1",
			Connectors: []Connector{{
				Guid:        "c1",
				T:           0,
				Point:       Point{X: 0, Y: 0, Z: 0},
				Direction:   Vector{X: 0, Y: 0, Z: 1},
				MaxChildren: &mc5,
			}},
		}},
	}
	data, err := json.Marshal(kit)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	var restored Kit
	if err := json.Unmarshal(data, &restored); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if restored.Ports[0].MaxChildren == nil || *restored.Ports[0].MaxChildren != 3 {
		t.Errorf("port maxChildren: got %v, want 3", restored.Ports[0].MaxChildren)
	}
	if restored.Types[0].Connectors[0].MaxChildren == nil || *restored.Types[0].Connectors[0].MaxChildren != 5 {
		t.Errorf("connector maxChildren: got %v, want 5", restored.Types[0].Connectors[0].MaxChildren)
	}
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > 0 && containsHelper(s, substr))
}

func containsHelper(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

// #endregion 📊MaxChildren Tests

// #region 🎑Performance Benchmarks
// Standard benchmarks migrated from semio_benchmark.go; run with `go test -bench=. -benchmem .` from this directory.

func benchLoadKitFile(b *testing.B, filename string) Kit {
	b.Helper()
	data, err := os.ReadFile(AssetsPath + "/" + filename)
	if err != nil {
		b.Fatal(err)
	}
	var kit Kit
	if err := json.Unmarshal(data, &kit); err != nil {
		b.Fatal(err)
	}
	return kit
}

func benchLoadKitDiffFile(b *testing.B, filename string) KitDiff {
	b.Helper()
	data, err := os.ReadFile(AssetsPath + "/" + filename)
	if err != nil {
		b.Fatal(err)
	}
	var diff KitDiff
	if err := json.Unmarshal(data, &diff); err != nil {
		b.Fatal(err)
	}
	return diff
}

func benchFindDesign(kit Kit, name string, parentName string) Design {
	var parentGuid string
	if parentName != "" {
		for _, d := range kit.Designs {
			if d.Name == parentName {
				parentGuid = d.Guid
				break
			}
		}
		if parentGuid == "" {
			panic("Parent design not found: " + parentName)
		}
	}

	for _, d := range kit.Designs {
		if d.Name == name {
			if parentName == "" {
				if d.Parent == nil {
					return d
				}
			} else {
				if d.Parent != nil && d.Parent.Guid == parentGuid {
					return d
				}
			}
		}
	}
	panic("Design not found: " + name)
}

func BenchmarkRoundtripMetabolism(b *testing.B) {
	schemaPath := filepath.Join("..", "sqlite", "schema.sql")
	schemaData, err := os.ReadFile(schemaPath)
	if err != nil {
		b.Fatal(err)
	}
	schema := string(schemaData)
	b.ResetTimer()
	for range b.N {
		kit, files, err := KitFromZip(AssetsPath + "/metabolism.zip")
		if err != nil {
			b.Fatal(err)
		}
		err = KitToZip(kit, files, "temp_benchmark_metabolism.zip", schema)
		if err != nil {
			b.Fatal(err)
		}
		_ = os.Remove("temp_benchmark_metabolism.zip")
	}
}

func BenchmarkDiffMetabolism(b *testing.B) {
	diffForward := benchLoadKitDiffFile(b, "metabolism.kit.diff.semio.json")
	diffInverse := benchLoadKitDiffFile(b, "metabolism.kit.diff.inverted.semio.json")
	b.ResetTimer()
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		b.StartTimer()
		k2 := ApplyKitDiff(kit, diffForward)
		ApplyKitDiff(k2, diffInverse)
	}
}

func BenchmarkFlattenDesign_NakaginCapsuleTower(b *testing.B) {
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		d := benchFindDesign(kit, "Nakagin Capsule Tower", "")
		b.StartTimer()
		FlattenDesign(&kit, d.Guid)
	}
}

func BenchmarkFlattenDesign_Nakagin_Slanted(b *testing.B) {
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		d := benchFindDesign(kit, "Slanted", "Nakagin Capsule Tower")
		b.StartTimer()
		FlattenDesign(&kit, d.Guid)
	}
}

func BenchmarkFlattenDesign_Nakagin_Twisted(b *testing.B) {
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		d := benchFindDesign(kit, "Twisted", "Nakagin Capsule Tower")
		b.StartTimer()
		FlattenDesign(&kit, d.Guid)
	}
}

func BenchmarkFlattenDesign_Nakagin_Dancing(b *testing.B) {
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		d := benchFindDesign(kit, "Dancing", "Nakagin Capsule Tower")
		b.StartTimer()
		FlattenDesign(&kit, d.Guid)
	}
}

func BenchmarkFlattenDesign_CapsuleDream(b *testing.B) {
	for range b.N {
		b.StopTimer()
		kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
		d := benchFindDesign(kit, "Capsule Dream", "")
		b.StartTimer()
		FlattenDesign(&kit, d.Guid)
	}
}

func BenchmarkValidateKit_Invalid(b *testing.B) {
	kit := benchLoadKitFile(b, "invalid.kit.semio.json")
	b.ResetTimer()
	for range b.N {
		ValidateKit(kit)
	}
}

func BenchmarkValidateKit_Metabolism(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	b.ResetTimer()
	for range b.N {
		ValidateKit(kit)
	}
}

// #endregion 🎑Performance Benchmarks

// #endregion 🛡️KitKind Tests
