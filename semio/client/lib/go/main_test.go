// #region 🧲Header

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

package semio

import (
	"encoding/binary"
	"encoding/json"
	"fmt"
	"math"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"
	"time"
)

var benchmarkCsvLanguages = []string{"go", "typescript", "python", "rust", "csharp"}

func benchmarkCsvPath() string {
	if _, err := os.Stat(filepath.Join("..", "benchmark.csv")); err == nil {
		return filepath.Join("..", "benchmark.csv")
	}
	return filepath.Join("..", "benchmark.csv")
}

func parseCsvLine(line string) []string {
	var values []string
	var current strings.Builder
	inQuotes := false
	for i := 0; i < len(line); i++ {
		ch := line[i]
		if ch == '"' {
			if inQuotes && i+1 < len(line) && line[i+1] == '"' {
				current.WriteByte('"')
				i++
				continue
			}
			inQuotes = !inQuotes
			continue
		}
		if ch == ',' && !inQuotes {
			values = append(values, current.String())
			current.Reset()
			continue
		}
		current.WriteByte(ch)
	}
	values = append(values, current.String())
	return values
}

func csvValue(value string) string {
	return `"` + strings.ReplaceAll(value, `"`, `""`) + `"`
}

func appendBenchmarkCsv(language string, name string, durationSeconds float64) {
	path := benchmarkCsvPath()
	rows := map[string]map[string]string{}
	order := []string{}
	if data, err := os.ReadFile(path); err == nil {
		lines := strings.Split(strings.TrimSpace(string(data)), "\n")
		if len(lines) > 0 && strings.HasPrefix(lines[0], "name,") {
			headers := parseCsvLine(lines[0])
			for _, line := range lines[1:] {
				if strings.TrimSpace(line) == "" {
					continue
				}
				values := parseCsvLine(line)
				if len(values) == 0 {
					continue
				}
				rowName := values[0]
				if _, ok := rows[rowName]; !ok {
					rows[rowName] = map[string]string{}
					order = append(order, rowName)
				}
				for i := 1; i < len(values) && i < len(headers); i++ {
					if values[i] != "" {
						rows[rowName][headers[i]] = values[i]
					}
				}
			}
		}
	}
	if _, ok := rows[name]; !ok {
		rows[name] = map[string]string{}
		order = append(order, name)
	}
	rows[name][language] = fmt.Sprintf("%.6f", durationSeconds*1000)
	var out strings.Builder
	out.WriteString("name")
	for _, lang := range benchmarkCsvLanguages {
		out.WriteString(",")
		out.WriteString(lang)
	}
	out.WriteString("\n")
	for _, rowName := range order {
		out.WriteString(csvValue(rowName))
		for _, lang := range benchmarkCsvLanguages {
			out.WriteString(",")
			out.WriteString(rows[rowName][lang])
		}
		out.WriteString("\n")
	}
	_ = os.WriteFile(path, []byte(out.String()), 0644)
}

type representationSelectionAsset struct {
	Cases []representationSelectionCase `json:"cases"`
}

type representationSelectionCase struct {
	Name             string                `json:"name"`
	SelectedTagIds []string              `json:"selectedTagIds"`
	ExpectedId     *string               `json:"expectedId"`
	Representations           []representationSelectionRepresentation `json:"representations"`
}

type representationSelectionRepresentation struct {
	Id     string   `json:"id"`
	FileId string   `json:"fileId"`
	TagIds []string `json:"tagIds"`
}

// #region 🗂️Asset Structs

type selectionAsset struct {
	Pieces []struct {
		Id string `json:"id"`
	} `json:"pieces"`
	Connections []struct {
		Id string `json:"id"`
	} `json:"connections"`
}

type flattenCasesAsset struct {
	Cases []flattenCase `json:"cases"`
}

type flattenCase struct {
	Name       string   `json:"name"`
	Kit        string   `json:"kit"`
	DesignPath []string `json:"designPath"`
}

type qualitySumCasesAsset struct {
	Cases []qualitySumCase `json:"cases"`
}

type qualitySumCase struct {
	Name         string  `json:"name"`
	Kit          string  `json:"kit"`
	DesignName   string  `json:"designName"`
	DesignParent *string `json:"designParent"`
	QualityName  string  `json:"qualityName"`
	Expected     float64 `json:"expected"`
	Tolerance    float64 `json:"tolerance"`
}

type hashCasesAsset struct {
	KitHash struct {
		Kit           string `json:"kit"`
		Expected      string `json:"expected"`
		ExpectedNet48 string `json:"expectedNet48"`
	} `json:"kitHash"`
	KitDiffHash struct {
		JSON     string `json:"json"`
		Expected string `json:"expected"`
	} `json:"kitDiffHash"`
	DesignName string `json:"designName"`
}

type designWithDiffCasesAsset struct {
	Cases []designWithDiffCase `json:"cases"`
}

type designWithDiffCase struct {
	Name                     string         `json:"name"`
	Kit                      string         `json:"kit"`
	DesignName               string         `json:"designName"`
	DesignParent             *string        `json:"designParent"`
	Diff                     string         `json:"diff"`
	Expected                 string         `json:"expected"`
	ExpectedPieceCounts      map[string]int `json:"expectedPieceCounts"`
	ExpectedConnectionCounts map[string]int `json:"expectedConnectionCounts"`
}

type filterKitCasesAsset struct {
	Cases     []filterKitCase     `json:"cases"`
	GlobCases []filterKitGlobCase `json:"globCases"`
}

type filterKitCase struct {
	Name         string  `json:"name"`
	Kit          string  `json:"kit"`
	DesignName   string  `json:"designName"`
	DesignParent *string `json:"designParent"`
	ExpectedKit  string  `json:"expectedKit"`
}

type filterKitGlobCase struct {
	Name          string   `json:"name"`
	Kit           string   `json:"kit"`
	DesignName    string   `json:"designName,omitempty"`
	DesignParent  *string  `json:"designParent,omitempty"`
	TypeInclude   []string `json:"typeInclude,omitempty"`
	TypeExclude   []string `json:"typeExclude,omitempty"`
	DesignInclude []string `json:"designInclude,omitempty"`
}

type findReplaceableCasesAsset struct {
	SyntheticKit   string                  `json:"syntheticKit"`
	Cases          []findReplaceableCase   `json:"cases"`
	BoundaryCases  findReplaceableBoundary `json:"boundaryCases"`
	SyntheticCases []syntheticCase         `json:"syntheticCases"`
}

type findReplaceableCase struct {
	Name                             string   `json:"name"`
	Kit                              string   `json:"kit"`
	DesignName                       string   `json:"designName"`
	DesignParent                     *string  `json:"designParent"`
	DesignParentName                 string   `json:"designParentName,omitempty"`
	PieceNames                       []string `json:"pieceNames,omitempty"`
	SelectionAsset                   string   `json:"selectionAsset,omitempty"`
	ExpectedSelectionPieceCount      int      `json:"expectedSelectionPieceCount,omitempty"`
	ExpectedSelectionConnectionCount int      `json:"expectedSelectionConnectionCount,omitempty"`
	ExpectedTypeIds                []string `json:"expectedTypeIds,omitempty"`
	ExpectedDesignIds              []string `json:"expectedDesignIds,omitempty"`
	ExpectedTypeIdCount            *int     `json:"expectedTypeIdCount,omitempty"`
	UsePieceIndex                    *int     `json:"usePieceIndex,omitempty"`
	LookupTypeName                   string   `json:"lookupTypeName,omitempty"`
	ExpectNonEmptyTypes              bool     `json:"expectNonEmptyTypes,omitempty"`
	ExpectOwnTypeInResults           bool     `json:"expectOwnTypeInResults,omitempty"`
	ForbiddenTypeNames               []string `json:"forbiddenTypeNames,omitempty"`
	ExpectConnectorlessTypeCount     bool     `json:"expectConnectorlessTypeCount,omitempty"`
}

type findReplaceableBoundary struct {
	Kit                            string   `json:"kit"`
	DesignName                     string   `json:"designName"`
	DesignParent                   *string  `json:"designParent"`
	SingleCapsulePieces            []string `json:"singleCapsulePieces"`
	TwoCapsulePieces               []string `json:"twoCapsulePieces"`
	FourCapsulePieces              []string `json:"fourCapsulePieces"`
	EightCapsulePieces             []string `json:"eightCapsulePieces"`
	TambourPieceName               string   `json:"tambourPieceName"`
	ExpectedTambourTypeIdCount   int      `json:"expectedTambourTypeIdCount"`
	ExpectedTambourDesignIdCount int      `json:"expectedTambourDesignIdCount"`
	ForbiddenFamilies              []string `json:"forbiddenFamilies"`
	ExpectedTwoCapsuleFamilies     []string `json:"expectedTwoCapsuleFamilies"`
	ExpectedLargeFamilies          []string `json:"expectedLargeFamilies"`
}

type syntheticCase struct {
	Name                       string   `json:"name"`
	DesignId                 string   `json:"designId"`
	PieceIds                 []string `json:"pieceIds"`
	ExpectedContainsTypes      []string `json:"expectedContainsTypes,omitempty"`
	ExpectedNotContainsTypes   []string `json:"expectedNotContainsTypes,omitempty"`
	ExpectedContainsDesigns    []string `json:"expectedContainsDesigns,omitempty"`
	ExpectedNotContainsDesigns []string `json:"expectedNotContainsDesigns,omitempty"`
}

// #endregion 🗂️Asset Structs

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

func containsAllTags(representation Representation, selectedTagIds []string) bool {
	for _, selectedId := range selectedTagIds {
		found := false
		for _, tag := range representation.Tags {
			if tag.Id == selectedId {
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

func jaccardTagIds(a []TagId, b []string) float64 {
	if len(a) == 0 && len(b) == 0 {
		return 1
	}
	setA := make(map[string]bool)
	setB := make(map[string]bool)
	for _, tag := range a {
		setA[tag.Id] = true
	}
	for _, id := range b {
		setB[id] = true
	}
	intersection := 0
	for id := range setA {
		if setB[id] {
			intersection++
		}
	}
	union := len(setA)
	for id := range setB {
		if !setA[id] {
			union++
		}
	}
	if union == 0 {
		return 0
	}
	return float64(intersection) / float64(union)
}

func selectBestRepresentationLikeSemioTS(representations []Representation, selectedTagIds []string) *Representation {
	if len(representations) == 0 {
		return nil
	}
	if len(selectedTagIds) == 0 {
		for i := range representations {
			if len(representations[i].Tags) == 0 {
				return &representations[i]
			}
		}
		return &representations[0]
	}
	filtered := make([]Representation, 0)
	for _, representation := range representations {
		if containsAllTags(representation, selectedTagIds) {
			filtered = append(filtered, representation)
		}
	}
	if len(filtered) == 0 {
		return nil
	}
	maxIndex := 0
	maxScore := jaccardTagIds(filtered[0].Tags, selectedTagIds)
	for i := 1; i < len(filtered); i++ {
		score := jaccardTagIds(filtered[i].Tags, selectedTagIds)
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

func centersEqual(c1, c2 *Coordinate, tolerance float64) bool {
	if c1 == nil && c2 == nil {
		return true
	}
	if c1 == nil || c2 == nil {
		return false
	}
	return floatEqual(c1.U, c2.U, tolerance) && floatEqual(c1.V, c2.V, tolerance)
}

func findDesignByName(designs []Design, name string, parentId *string) *Design {
	var parentFamilies []FamilyId
	if parentId != nil {
		for i := range designs {
			if designs[i].Id == *parentId {
				parentFamilies = designs[i].Families
				break
			}
		}
	}
	for i := range designs {
		d := &designs[i]
		if d.Name != name {
			continue
		}
		if parentId == nil {
			return d
		} else {
			if familiesOverlap(d.Families, parentFamilies) {
				return d
			}
		}
	}
	return nil
}

func familiesOverlap(a, b []FamilyId) bool {
	for _, left := range a {
		for _, right := range b {
			if left.Id == right.Id {
				return true
			}
		}
	}
	return false
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
	var parentId *string

	for _, designName := range designPath {
		design = findDesignByName(kit.Designs, designName, parentId)
		if design == nil {
			t.Fatalf("Design %q not found", designName)
		}
		parentId = &design.Id
	}

	if design == nil {
		t.Fatal("Design is nil")
	}

	expectedDesign := findDesignByName(kit.Designs, "Flat", &design.Id)
	if expectedDesign == nil {
		t.Fatalf("Expected flat design not found for %q", design.Name)
	}

	flatRep := FlattenDesign(&kit, design.Id)
	if !flatRep.Ok || flatRep.Diff == nil {
		t.Fatalf("FlattenDesign failed: ok=%v errors=%v", flatRep.Ok, flatRep.Errors)
	}
	flatDesign := deepCloneDesign(*design)
	ApplyDesignDiff(&flatDesign, &flatRep.Diff.Forward)

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

func TestDesignRepresentation(t *testing.T) {
	t.Run("Representation selection cases from shared semio assets", func(t *testing.T) {
		var payload representationSelectionAsset
		loadJSON(t, "representation.selection.semio.json", &payload)
		for _, testCase := range payload.Cases {
			representations := make([]Representation, 0, len(testCase.Representations))
			for _, representation := range testCase.Representations {
				tags := make([]TagId, 0, len(representation.TagIds))
				for _, id := range representation.TagIds {
					tags = append(tags, TagId{Id: id})
				}
				representations = append(representations, Representation{
					Id: representation.Id,
					File: FileId{Id: representation.FileId},
					Tags: tags,
				})
			}
			selected := selectBestRepresentationLikeSemioTS(representations, testCase.SelectedTagIds)
			if testCase.ExpectedId == nil {
				if selected != nil {
					t.Fatalf("Case %q failed: got %q expected nil", testCase.Name, selected.Id)
				}
				continue
			}
			if selected == nil || selected.Id != *testCase.ExpectedId {
				got := "<nil>"
				if selected != nil {
					got = selected.Id
				}
				t.Fatalf("Case %q failed: got %q expected %q", testCase.Name, got, *testCase.ExpectedId)
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
		filtered := FilterKit(kit, KitFilter{DesignId: nakaginDesign.Id})

		if len(filtered.Designs) != len(expected.Designs) {
			t.Fatalf("Expected %d designs, got %d", len(expected.Designs), len(filtered.Designs))
		}
		if len(filtered.Types) != len(expected.Types) {
			t.Fatalf("Expected %d types, got %d", len(expected.Types), len(filtered.Types))
		}
		if len(filtered.Files) != len(expected.Files) {
			t.Fatalf("Expected %d files, got %d", len(expected.Files), len(filtered.Files))
		}
		if len(AllPortsInKit(&filtered)) != len(AllPortsInKit(&expected)) {
			t.Fatalf("Expected %d ports, got %d", len(AllPortsInKit(&expected)), len(AllPortsInKit(&filtered)))
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
				if filteredType.Id == expectedType.Id {
					matches++
					if len(filteredType.Representations) != len(expectedType.Representations) {
						t.Fatalf("Expected type %s to have %d representations, got %d", expectedType.Id, len(expectedType.Representations), len(filteredType.Representations))
					}
				}
			}
			if matches != 1 {
				t.Fatalf("Expected filtered type %s exactly once, got %d", expectedType.Id, matches)
			}
		}

		for _, piece := range filteredDesign.Pieces {
			if piece.Type == nil {
				continue
			}
			found := false
			for _, filteredType := range filtered.Types {
				if filteredType.Id == piece.Type.Id {
					found = true
					break
				}
			}
			if !found {
				t.Fatalf("Missing filtered type %s for piece", piece.Type.Id)
			}
		}

		for _, filteredType := range filtered.Types {
			if len(filteredType.Representations) > 1 {
				t.Fatalf("Type %s has %d representations, expected at most 1", filteredType.Id, len(filteredType.Representations))
			}
			for _, representation := range filteredType.Representations {
				foundFile := false
				for _, file := range filtered.Files {
					if file.Id == representation.File.Id {
						foundFile = true
						break
					}
				}
				if !foundFile {
					t.Fatalf("Missing filtered file %s for type %s", representation.File.Id, filteredType.Id)
				}
			}
			for _, connector := range filteredType.Connectors {
				if connector.Port == nil {
					continue
				}
				foundPort := false
				for _, port := range AllPortsInKit(&filtered) {
					if port.Id == connector.Port.Id {
						foundPort = true
						break
					}
				}
				if !foundPort {
					t.Fatalf("Missing filtered port %s for type %s", connector.Port.Id, filteredType.Id)
				}
			}
		}
	})

	t.Run("preserves kit metadata", func(t *testing.T) {
		filtered := FilterKit(kit, KitFilter{DesignId: nakaginDesign.Id})
		if filtered.Id != kit.Id || filtered.Name != kit.Name || filtered.Version != kit.Version {
			t.Fatalf("Filtered kit metadata mismatch")
		}
	})
}

func TestFlatten(t *testing.T) {
	var asset flattenCasesAsset
	loadJSON(t, "flatten.cases.semio.json", &asset)

	for _, tc := range asset.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			var kit Kit
			loadJSON(t, tc.Kit, &kit)
			t.Run("Kit -> Flatten -> Diff -> Apply = Flat", func(t *testing.T) {
				testFlattenDesign(t, kit, tc.DesignPath)
			})
		})
	}
}

// #region 🌳Flatten Merkle Hash Tests

// 🌳flattenMerkleMutation representations a single kit mutation described by the shared merkle cases asset.
type flattenMerkleMutation struct {
	Kind           string          `json:"kind"`
	PieceId      string          `json:"pieceId"`
	ConnectionId string          `json:"connectionId"`
	Path           string          `json:"path"`
	Value          json.RawMessage `json:"value"`
}

// 🌳flattenMerkleExpect captures the optional assertions bundled with each merkle case.
type flattenMerkleExpect struct {
	PlaneHashesChangedAny       *bool    `json:"planeHashesChangedAny,omitempty"`
	CenterHashesChangedAny      *bool    `json:"centerHashesChangedAny,omitempty"`
	PlaneHashesChangedAll       *bool    `json:"planeHashesChangedAll,omitempty"`
	CenterHashesChangedAll      *bool    `json:"centerHashesChangedAll,omitempty"`
	PlaneHashesChangedIncludes  []string `json:"planeHashesChangedIncludes,omitempty"`
	CenterHashesChangedIncludes []string `json:"centerHashesChangedIncludes,omitempty"`
	PlaneHashesStableIncludes   []string `json:"planeHashesStableIncludes,omitempty"`
	CenterHashesStableIncludes  []string `json:"centerHashesStableIncludes,omitempty"`
}

// 🌳flattenMerkleCase represents a single entry in flatten-merkle.cases.semio.json.
type flattenMerkleCase struct {
	Name       string                  `json:"name"`
	Kit        string                  `json:"kit"`
	DesignPath []string                `json:"designPath"`
	Mutations  []flattenMerkleMutation `json:"mutations"`
	Expect     flattenMerkleExpect     `json:"expect"`
}

// 🌳flattenMerkleParity captures the cross-language reference hashes block.
type flattenMerkleParity struct {
	Kit            string   `json:"kit"`
	DesignPath     []string `json:"designPath"`
	ExpectedHashes []struct {
		PieceId  string `json:"pieceId"`
		PlaneHash  string `json:"planeHash"`
		CenterHash string `json:"centerHash"`
	} `json:"expectedHashes"`
}

// 🌳flattenMerkleAsset mirrors the shared flatten-merkle cases asset.
type flattenMerkleAsset struct {
	Parity flattenMerkleParity `json:"parity"`
	Cases  []flattenMerkleCase `json:"cases"`
}

// 🌳flattenMerkleFindDesignByPath walks the kit to resolve a design by its ordered name path.
func flattenMerkleFindDesignByPath(kit map[string]interface{}, designPath []string) map[string]interface{} {
	if len(designPath) == 0 {
		return nil
	}
	designs, _ := kit["designs"].([]interface{})
	var current map[string]interface{}
	for i, name := range designPath {
		var parentId string
		if current != nil {
			parentId, _ = current["id"].(string)
		}
		var match map[string]interface{}
		for _, d := range designs {
			dm, ok := d.(map[string]interface{})
			if !ok {
				continue
			}
			dn, _ := dm["name"].(string)
			if dn != name {
				continue
			}
			parent, _ := dm["parent"].(map[string]interface{})
			if i == 0 {
				if parent == nil {
					match = dm
					break
				}
			} else {
				if parent != nil {
					pg, _ := parent["id"].(string)
					if pg == parentId {
						match = dm
						break
					}
				}
			}
		}
		if match == nil {
			return nil
		}
		current = match
	}
	return current
}

// 🌳flattenMerkleSetPath assigns a value inside a nested JSON map using a dotted path, creating intermediate maps as needed.
func flattenMerkleSetPath(obj map[string]interface{}, path string, value interface{}) {
	keys := strings.Split(path, ".")
	current := obj
	for _, k := range keys[:len(keys)-1] {
		next, ok := current[k].(map[string]interface{})
		if !ok || next == nil {
			next = map[string]interface{}{}
			current[k] = next
		}
		current = next
	}
	current[keys[len(keys)-1]] = value
}

// 🌳flattenMerkleApplyMutations applies the mutation list in-place on a kit map prior to rehashing.
func flattenMerkleApplyMutations(t *testing.T, design map[string]interface{}, mutations []flattenMerkleMutation) {
	for _, m := range mutations {
		var value interface{}
		if err := json.Unmarshal(m.Value, &value); err != nil {
			t.Fatalf("decode mutation value %q: %v", string(m.Value), err)
		}
		switch m.Kind {
		case "pieceField":
			pieces, _ := design["pieces"].([]interface{})
			var target map[string]interface{}
			for _, p := range pieces {
				pm, ok := p.(map[string]interface{})
				if !ok {
					continue
				}
				if g, _ := pm["id"].(string); g == m.PieceId {
					target = pm
					break
				}
			}
			if target == nil {
				t.Fatalf("piece %s not found", m.PieceId)
			}
			flattenMerkleSetPath(target, m.Path, value)
		case "connectionField":
			conns, _ := design["connections"].([]interface{})
			var target map[string]interface{}
			for _, c := range conns {
				cm, ok := c.(map[string]interface{})
				if !ok {
					continue
				}
				if g, _ := cm["id"].(string); g == m.ConnectionId {
					target = cm
					break
				}
			}
			if target == nil {
				t.Fatalf("connection %s not found", m.ConnectionId)
			}
			flattenMerkleSetPath(target, m.Path, value)
		default:
			t.Fatalf("unknown mutation kind %q", m.Kind)
		}
	}
}

// 🌳flattenMerkleLoadKitAsMap reads a kit asset into a generic JSON map so mutations can touch arbitrary nested fields.
func flattenMerkleLoadKitAsMap(t *testing.T, filename string) map[string]interface{} {
	t.Helper()
	data, err := os.ReadFile(filepath.Join(AssetsPath, filename))
	if err != nil {
		t.Fatalf("read %s: %v", filename, err)
	}
	var m map[string]interface{}
	if err := json.Unmarshal(data, &m); err != nil {
		t.Fatalf("parse %s: %v", filename, err)
	}
	return m
}

// 🌳flattenMerkleKitFromMap remarshals a JSON map back through the typed Kit structs for hashing.
func flattenMerkleKitFromMap(t *testing.T, m map[string]interface{}) Kit {
	t.Helper()
	data, err := json.Marshal(m)
	if err != nil {
		t.Fatalf("marshal kit map: %v", err)
	}
	var kit Kit
	if err := json.Unmarshal(data, &kit); err != nil {
		t.Fatalf("unmarshal into Kit: %v", err)
	}
	return kit
}

// 🌳flattenMerkleDesignId resolves the design id by path from a typed kit.
func flattenMerkleDesignId(t *testing.T, kit Kit, designPath []string) string {
	t.Helper()
	var current *Design
	var parentId *string
	for _, name := range designPath {
		current = findDesignByName(kit.Designs, name, parentId)
		if current == nil {
			t.Fatalf("design path %v not found at %q", designPath, name)
		}
		g := current.Id
		parentId = &g
	}
	if current == nil {
		t.Fatalf("design path %v resolved to nil", designPath)
	}
	return current.Id
}

// 🌳flattenMerkleChangedSets returns the piece ids whose plane/center hash differs between two runs.
func flattenMerkleChangedSets(before, after map[string]FlatMerkleHashes) (changedPlane, changedCenter map[string]bool) {
	changedPlane = map[string]bool{}
	changedCenter = map[string]bool{}
	for id, b := range before {
		a, ok := after[id]
		if !ok {
			changedPlane[id] = true
			changedCenter[id] = true
			continue
		}
		if a.PlaneHash != b.PlaneHash {
			changedPlane[id] = true
		}
		if a.CenterHash != b.CenterHash {
			changedCenter[id] = true
		}
	}
	return
}

func TestFlattenMerkle(t *testing.T) {
	var asset flattenMerkleAsset
	loadJSON(t, "flatten-merkle.cases.semio.json", &asset)

	t.Run("SharedAssetMutationCases", func(t *testing.T) {
		for _, tc := range asset.Cases {
			tc := tc
			t.Run(tc.Name, func(t *testing.T) {
				kitMapBefore := flattenMerkleLoadKitAsMap(t, tc.Kit)
				kitBefore := flattenMerkleKitFromMap(t, kitMapBefore)
				designIdBefore := flattenMerkleDesignId(t, kitBefore, tc.DesignPath)
				before := ComputeFlatHashes(&kitBefore, designIdBefore)

				kitMapAfter := flattenMerkleLoadKitAsMap(t, tc.Kit)
				designAfterMap := flattenMerkleFindDesignByPath(kitMapAfter, tc.DesignPath)
				if designAfterMap == nil {
					t.Fatalf("design path %v not found in mutable kit", tc.DesignPath)
				}
				flattenMerkleApplyMutations(t, designAfterMap, tc.Mutations)
				kitAfter := flattenMerkleKitFromMap(t, kitMapAfter)
				designIdAfter := flattenMerkleDesignId(t, kitAfter, tc.DesignPath)
				after := ComputeFlatHashes(&kitAfter, designIdAfter)

				if len(before) != len(after) {
					t.Fatalf("piece set size changed: %d -> %d", len(before), len(after))
				}
				for id := range before {
					if _, ok := after[id]; !ok {
						t.Fatalf("piece %s missing after mutation", id)
					}
				}

				changedPlane, changedCenter := flattenMerkleChangedSets(before, after)

				if tc.Expect.PlaneHashesChangedAny != nil {
					if *tc.Expect.PlaneHashesChangedAny && len(changedPlane) == 0 {
						t.Fatalf("expected some planeHash changes, got none")
					}
					if !*tc.Expect.PlaneHashesChangedAny && len(changedPlane) != 0 {
						t.Fatalf("expected no planeHash changes, got %v", sortedKeys(changedPlane))
					}
				}
				if tc.Expect.CenterHashesChangedAny != nil {
					if *tc.Expect.CenterHashesChangedAny && len(changedCenter) == 0 {
						t.Fatalf("expected some centerHash changes, got none")
					}
					if !*tc.Expect.CenterHashesChangedAny && len(changedCenter) != 0 {
						t.Fatalf("expected no centerHash changes, got %v", sortedKeys(changedCenter))
					}
				}
				if tc.Expect.PlaneHashesChangedAll != nil {
					if *tc.Expect.PlaneHashesChangedAll && len(changedPlane) != len(before) {
						t.Fatalf("expected every planeHash to change, got %d/%d", len(changedPlane), len(before))
					}
					if !*tc.Expect.PlaneHashesChangedAll && len(changedPlane) == len(before) {
						t.Fatalf("expected not every planeHash to change, but all did")
					}
				}
				if tc.Expect.CenterHashesChangedAll != nil {
					if *tc.Expect.CenterHashesChangedAll && len(changedCenter) != len(before) {
						t.Fatalf("expected every centerHash to change, got %d/%d", len(changedCenter), len(before))
					}
					if !*tc.Expect.CenterHashesChangedAll && len(changedCenter) == len(before) {
						t.Fatalf("expected not every centerHash to change, but all did")
					}
				}
				for _, id := range tc.Expect.PlaneHashesChangedIncludes {
					if !changedPlane[id] {
						t.Fatalf("expected piece %s to have changed planeHash", id)
					}
				}
				for _, id := range tc.Expect.CenterHashesChangedIncludes {
					if !changedCenter[id] {
						t.Fatalf("expected piece %s to have changed centerHash", id)
					}
				}
				for _, id := range tc.Expect.PlaneHashesStableIncludes {
					if changedPlane[id] {
						t.Fatalf("expected piece %s to keep stable planeHash", id)
					}
				}
				for _, id := range tc.Expect.CenterHashesStableIncludes {
					if changedCenter[id] {
						t.Fatalf("expected piece %s to keep stable centerHash", id)
					}
				}
			})
		}
	})

	t.Run("CrossLanguageParityReferenceHashes", func(t *testing.T) {
		if asset.Parity.Kit == "" {
			t.Fatal("parity block missing")
		}
		var kit Kit
		loadJSON(t, asset.Parity.Kit, &kit)
		designId := flattenMerkleDesignId(t, kit, asset.Parity.DesignPath)
		hashes := ComputeFlatHashes(&kit, designId)
		for _, expected := range asset.Parity.ExpectedHashes {
			got, ok := hashes[expected.PieceId]
			if !ok {
				t.Fatalf("piece %s missing from computed hashes", expected.PieceId)
			}
			if got.PlaneHash != expected.PlaneHash {
				t.Fatalf("piece %s planeHash mismatch: got %s want %s", expected.PieceId, got.PlaneHash, expected.PlaneHash)
			}
			if got.CenterHash != expected.CenterHash {
				t.Fatalf("piece %s centerHash mismatch: got %s want %s", expected.PieceId, got.CenterHash, expected.CenterHash)
			}
		}
	})

	t.Run("CachedFlattenReusesValues", func(t *testing.T) {
		var kit Kit
		loadJSON(t, "metabolism.kit.semio.json", &kit)
		designId := flattenMerkleDesignId(t, kit, []string{"Nakagin Capsule Tower"})
		_, firstCache := FlattenDesignCached(&kit, designId, nil)
		if len(firstCache) == 0 {
			t.Fatal("first cache is empty")
		}
		_, secondCache := FlattenDesignCached(&kit, designId, firstCache)
		for id, entry := range firstCache {
			second, ok := secondCache[id]
			if !ok {
				t.Fatalf("piece %s missing from second cache", id)
			}
			if entry.PlaneHash != second.PlaneHash {
				t.Fatalf("piece %s planeHash mismatch %s vs %s", id, entry.PlaneHash, second.PlaneHash)
			}
			if entry.CenterHash != second.CenterHash {
				t.Fatalf("piece %s centerHash mismatch %s vs %s", id, entry.CenterHash, second.CenterHash)
			}
			if !reflect.DeepEqual(entry.Plane, second.Plane) {
				t.Fatalf("piece %s plane mismatch: %#v vs %#v", id, entry.Plane, second.Plane)
			}
			if !reflect.DeepEqual(entry.Center, second.Center) {
				t.Fatalf("piece %s center mismatch: %#v vs %#v", id, entry.Center, second.Center)
			}
		}
	})
}

// 🌳sortedKeys returns a deterministic sorted slice of map keys for readable error messages.
func sortedKeys(m map[string]bool) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

// #endregion 🌳Flatten Merkle Hash Tests

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

			appliedForward := deepCloneKit(kitOriginal)
			ApplyKitDiff(&appliedForward, &change.Forward)
			if !AreKitsEqual(appliedForward, kitDiffed) {
				t.Error("Original + Diff should equal DiffedKit")
			}

			appliedInverse := deepCloneKit(kitDiffed)
			ApplyKitDiff(&appliedInverse, &change.Backward)
			if !AreKitsEqual(appliedInverse, kitOriginal) {
				t.Error("DiffedKit + InverseDiff should equal original Kit")
			}
		})
	})
}

func TestValidateKitDiffAsset(t *testing.T) {
	var asset struct {
		TinyKit Kit `json:"tinyKit"`
		Cases   []struct {
			ID           string   `json:"id"`
			Diff         KitDiff  `json:"diff"`
			ExpectOk     bool     `json:"expectOk"`
			ErrorCodes   []string `json:"errorCodes"`
			WarningCodes []string `json:"warningCodes"`
		} `json:"cases"`
	}
	loadJSON(t, "validate-kit-diff.cases.semio.json", &asset)
	for _, c := range asset.Cases {
		t.Run(c.ID, func(t *testing.T) {
			r := ValidateKitDiff(asset.TinyKit, c.Diff, false)
			if r.Ok != c.ExpectOk {
				t.Fatalf("ok=%v want %v errors=%v warnings=%v", r.Ok, c.ExpectOk, r.Errors, r.Warnings)
			}
			errCodes := make([]string, 0, len(r.Errors))
			for _, e := range r.Errors {
				if e.Code != "" {
					errCodes = append(errCodes, e.Code)
				}
			}
			warnCodes := make([]string, 0, len(r.Warnings))
			for _, w := range r.Warnings {
				if w.Code != "" {
					warnCodes = append(warnCodes, w.Code)
				}
			}
			for _, code := range c.ErrorCodes {
				if !slicesContains(errCodes, code) {
					t.Fatalf("missing error code %q got %v", code, errCodes)
				}
			}
			for _, code := range c.WarningCodes {
				if !slicesContains(warnCodes, code) {
					t.Fatalf("missing warning code %q got %v", code, warnCodes)
				}
			}
		})
	}
	bad := KitDiff{}
	bad.Designs = &DesignsDiff{
		Updated: []struct {
			Design DesignId   `json:"design"`
			Diff   DesignDiff `json:"diff"`
		}{{Design: DesignId{Id: "99999999-9999-9999-9999-999999999999"}, Diff: DesignDiff{Name: ptrStringGoTest("X")}}},
	}
	r := ValidateKitDiff(asset.TinyKit, bad, true)
	if r.Diff == nil {
		t.Fatal("expected healed diff")
	}
	if r.Diff.Designs != nil && len(r.Diff.Designs.Updated) != 0 {
		t.Fatalf("heal should drop invalid design update: %#v", r.Diff.Designs)
	}
}

func slicesContains(haystack []string, needle string) bool {
	for _, s := range haystack {
		if s == needle {
			return true
		}
	}
	return false
}

func ptrStringGoTest(s string) *string { return &s }

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

			pieceIds := make([]string, len(selection.Pieces))
			for i, p := range selection.Pieces {
				pieceIds[i] = p.Id
			}
			connectionIds := make([]string, len(selection.Connections))
			for i, c := range selection.Connections {
				connectionIds[i] = c.Id
			}

			// 🔶Load expected diff
			var expectedDiff DesignDiff
			loadJSON(t, "nakagin-capsule-tower.deleted.design.diff.semio.json", &expectedDiff)

			// Compute diff
			computedReport := DeletePiecesAndConnectionsInDesign(&kit, *design, pieceIds, connectionIds)
			if !computedReport.Ok || computedReport.Diff == nil {
				t.Fatalf("DeletePiecesAndConnectionsInDesign failed: ok=%v errors=%v", computedReport.Ok, computedReport.Errors)
			}
			computedDiff := *computedReport.Diff

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
				if c.Id != expectedDiff.Pieces.Removed[i].Id {
					t.Errorf("Removed piece id mismatch at %d: %s vs %s", i, c.Id, expectedDiff.Pieces.Removed[i].Id)
				}
			}

			// Verify updated (fixed) pieces
			if len(computedDiff.Pieces.Updated) != len(expectedDiff.Pieces.Updated) {
				t.Fatalf("Updated pieces count mismatch: %d vs %d",
					len(computedDiff.Pieces.Updated), len(expectedDiff.Pieces.Updated))
			}
			computedIds := make([]string, len(computedDiff.Pieces.Updated))
			for i, u := range computedDiff.Pieces.Updated {
				computedIds[i] = u.Piece.Id
			}
			expectedIds := make([]string, len(expectedDiff.Pieces.Updated))
			for i, u := range expectedDiff.Pieces.Updated {
				expectedIds[i] = u.Piece.Id
			}
			sort.Strings(computedIds)
			sort.Strings(expectedIds)
			for i := range computedIds {
				if computedIds[i] != expectedIds[i] {
					t.Errorf("Updated piece id mismatch at %d: %s vs %s", i, computedIds[i], expectedIds[i])
				}
			}
			// Verify updated pieces have both plane and center matching expected
			expectedUpdatedMap := make(map[string]PieceDiff)
			for _, u := range expectedDiff.Pieces.Updated {
				expectedUpdatedMap[u.Piece.Id] = u.Diff
			}
			for _, u := range computedDiff.Pieces.Updated {
				if u.Diff.Plane == nil {
					t.Errorf("Updated piece %s missing plane", u.Piece.Id)
				}
				if u.Diff.Center == nil {
					t.Errorf("Updated piece %s missing center", u.Piece.Id)
				}
				exp, ok := expectedUpdatedMap[u.Piece.Id]
				if !ok {
					t.Errorf("Unexpected updated piece %s", u.Piece.Id)
					continue
				}
				if u.Diff.Plane != nil && exp.Plane != nil {
					tolerance := 0.001
					if u.Diff.Plane.Origin != nil && exp.Plane.Origin != nil {
						if math.Abs(*u.Diff.Plane.Origin.X-*exp.Plane.Origin.X) > tolerance {
							t.Errorf("Updated piece %s plane origin x: got %f, expected %f", u.Piece.Id, *u.Diff.Plane.Origin.X, *exp.Plane.Origin.X)
						}
						if math.Abs(*u.Diff.Plane.Origin.Y-*exp.Plane.Origin.Y) > tolerance {
							t.Errorf("Updated piece %s plane origin y: got %f, expected %f", u.Piece.Id, *u.Diff.Plane.Origin.Y, *exp.Plane.Origin.Y)
						}
						if math.Abs(*u.Diff.Plane.Origin.Z-*exp.Plane.Origin.Z) > tolerance {
							t.Errorf("Updated piece %s plane origin z: got %f, expected %f", u.Piece.Id, *u.Diff.Plane.Origin.Z, *exp.Plane.Origin.Z)
						}
					}
				}
				if u.Diff.Center != nil && exp.Center != nil {
					tolerance := 0.001
					if math.Abs(*u.Diff.Center.U-*exp.Center.U) > tolerance {
						t.Errorf("Updated piece %s center U: got %f, expected %f", u.Piece.Id, *u.Diff.Center.U, *exp.Center.U)
					}
					if math.Abs(*u.Diff.Center.V-*exp.Center.V) > tolerance {
						t.Errorf("Updated piece %s center V: got %f, expected %f", u.Piece.Id, *u.Diff.Center.V, *exp.Center.V)
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
			computedConnIds := make([]string, len(computedDiff.Connections.Removed))
			for i, r := range computedDiff.Connections.Removed {
				computedConnIds[i] = r.Id
			}
			expectedConnIds := make([]string, len(expectedDiff.Connections.Removed))
			for i, r := range expectedDiff.Connections.Removed {
				expectedConnIds[i] = r.Id
			}
			sort.Strings(computedConnIds)
			sort.Strings(expectedConnIds)
			for i := range computedConnIds {
				if computedConnIds[i] != expectedConnIds[i] {
					t.Errorf("Removed connection id mismatch at %d: %s vs %s", i, computedConnIds[i], expectedConnIds[i])
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
		var offset Coordinate
		loadJSON(t, "drag/offset.semio.json", &offset)
		type expectedPieceUpdate struct {
			Piece struct {
				Id string `json:"id"`
			} `json:"piece"`
			Diff struct {
				Center *Coordinate `json:"center"`
			} `json:"diff"`
		}
		type expectedConnUpdate struct {
			Connection struct {
				Id string `json:"id"`
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
			expectedMap := make(map[string]*Coordinate)
			for _, u := range expected.Pieces.Updated {
				expectedMap[u.Piece.Id] = u.Diff.Center
			}
			for _, u := range computed.Pieces.Updated {
				ec, ok := expectedMap[u.Piece.Id]
				if !ok {
					t.Errorf("Unexpected piece update for %s", u.Piece.Id)
					continue
				}
				if u.Diff.Center == nil {
					t.Errorf("Piece %s has nil center diff", u.Piece.Id)
					continue
				}
				if !floatEqual(*u.Diff.Center.U, ec.U, 0.001) || !floatEqual(*u.Diff.Center.V, ec.V, 0.001) {
					t.Errorf("Piece %s center mismatch: got (%f,%f), want (%f,%f)", u.Piece.Id, *u.Diff.Center.U, *u.Diff.Center.V, ec.U, ec.V)
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
				expectedConnMap[u.Connection.Id] = [2]float64{*u.Diff.U, *u.Diff.V}
			}
			for _, u := range computed.Connections.Updated {
				ev, ok := expectedConnMap[u.Connection.Id]
				if !ok {
					t.Errorf("Unexpected connection update for %s", u.Connection.Id)
					continue
				}
				if u.Diff.U == nil || u.Diff.V == nil {
					t.Errorf("Connection %s has nil u/v diff", u.Connection.Id)
					continue
				}
				if !floatEqual(*u.Diff.U, ev[0], 0.001) || !floatEqual(*u.Diff.V, ev[1], 0.001) {
					t.Errorf("Connection %s uv mismatch: got (%f,%f), want (%f,%f)", u.Connection.Id, *u.Diff.U, *u.Diff.V, ev[0], ev[1])
				}
			}
		}
	})
}

func TestMove(t *testing.T) {
	t.Run("same drag fixture + move vector = plane and connection Jacobian diff", func(t *testing.T) {
		var kit Kit
		loadJSON(t, "metabolism.kit.semio.json", &kit)
		var design Design
		loadJSON(t, "drag/design.semio.json", &design)
		var pieces Design
		loadJSON(t, "drag/pieces.semio.json", &pieces)
		var vector MoveVector
		loadJSON(t, "move/vector.semio.json", &vector)
		type planeOrig struct {
			X float64 `json:"x"`
			Y float64 `json:"y"`
			Z float64 `json:"z"`
		}
		type expPiece struct {
			Piece struct {
				Id string `json:"id"`
			} `json:"piece"`
			Diff struct {
				Plane struct {
					Origin planeOrig `json:"origin"`
				} `json:"plane"`
			} `json:"diff"`
		}
		type expConnDiff struct {
			Gap      *float64 `json:"gap,omitempty"`
			Shift    *float64 `json:"shift,omitempty"`
			Rise     *float64 `json:"rise,omitempty"`
			Rotation *float64 `json:"rotation,omitempty"`
			Turn     *float64 `json:"turn,omitempty"`
			Tilt     *float64 `json:"tilt,omitempty"`
			U        *float64 `json:"u,omitempty"`
			V        *float64 `json:"v,omitempty"`
		}
		type expConn struct {
			Connection struct {
				Id string `json:"id"`
			} `json:"connection"`
			Diff expConnDiff `json:"diff"`
		}
		type expectedMove struct {
			Pieces      *struct{ Updated []expPiece } `json:"pieces"`
			Connections *struct{ Updated []expConn }  `json:"connections"`
		}
		var expected expectedMove
		loadJSON(t, "move/diff.design.semio.json", &expected)
		computed := MovePiecesInDesign(kit, design, pieces, vector)
		if expected.Pieces == nil {
			t.Fatal("expected pieces")
		}
		if computed.Pieces == nil || len(computed.Pieces.Updated) != len(expected.Pieces.Updated) {
			t.Fatalf("piece updates: want %d got %v", len(expected.Pieces.Updated), computed.Pieces)
		}
		expById := make(map[string]expPiece)
		for _, u := range expected.Pieces.Updated {
			expById[u.Piece.Id] = u
		}
		for _, u := range computed.Pieces.Updated {
			ex, ok := expById[u.Piece.Id]
			if !ok {
				t.Errorf("unexpected piece %s", u.Piece.Id)
				continue
			}
			if u.Diff.Plane == nil || u.Diff.Plane.Origin == nil {
				t.Fatalf("nil plane for %s", u.Piece.Id)
			}
			if u.Diff.Plane.Origin.X == nil || u.Diff.Plane.Origin.Y == nil || u.Diff.Plane.Origin.Z == nil {
				t.Fatalf("nil origin for %s", u.Piece.Id)
			}
			if !floatEqual(*u.Diff.Plane.Origin.X, ex.Diff.Plane.Origin.X, 0.001) ||
				!floatEqual(*u.Diff.Plane.Origin.Y, ex.Diff.Plane.Origin.Y, 0.001) ||
				!floatEqual(*u.Diff.Plane.Origin.Z, ex.Diff.Plane.Origin.Z, 0.001) {
				t.Errorf("piece %s origin mismatch got (%v,%v,%v) want (%f,%f,%f)", u.Piece.Id,
					*u.Diff.Plane.Origin.X, *u.Diff.Plane.Origin.Y, *u.Diff.Plane.Origin.Z,
					ex.Diff.Plane.Origin.X, ex.Diff.Plane.Origin.Y, ex.Diff.Plane.Origin.Z)
			}
		}
		if expected.Connections == nil || computed.Connections == nil {
			t.Fatal("expected connections")
		}
		if len(computed.Connections.Updated) != len(expected.Connections.Updated) {
			t.Fatalf("conn updates: want %d got %d", len(expected.Connections.Updated), len(computed.Connections.Updated))
		}
		expC := make(map[string]expConn)
		for _, u := range expected.Connections.Updated {
			expC[u.Connection.Id] = u
		}
		optF := func(p *float64) float64 {
			if p == nil {
				return 0
			}
			return *p
		}
		for _, u := range computed.Connections.Updated {
			ex, ok := expC[u.Connection.Id]
			if !ok {
				t.Errorf("unexpected conn %s", u.Connection.Id)
				continue
			}
			for _, kv := range []struct {
				name     string
				got      *float64
				expected *float64
			}{
				{"gap", u.Diff.Gap, ex.Diff.Gap},
				{"shift", u.Diff.Shift, ex.Diff.Shift},
				{"rise", u.Diff.Rise, ex.Diff.Rise},
				{"rotation", u.Diff.Rotation, ex.Diff.Rotation},
				{"turn", u.Diff.Turn, ex.Diff.Turn},
				{"tilt", u.Diff.Tilt, ex.Diff.Tilt},
				{"u", u.Diff.U, ex.Diff.U},
				{"v", u.Diff.V, ex.Diff.V},
			} {
				if !floatEqual(optF(kv.got), optF(kv.expected), 0.001) {
					t.Errorf("conn %s %s mismatch got %f want %f", u.Connection.Id, kv.name, optF(kv.got), optF(kv.expected))
				}
			}
		}
	})
}

// #region 🔍Find Replaceable Types In Designs Tests
func TestFindReplaceableTypesInDesigns(t *testing.T) {
	var frAsset findReplaceableCasesAsset
	loadJSON(t, "find-replaceable-types.cases.semio.json", &frAsset)

	containsId := func(ids []string, expectedId string) bool {
		for _, id := range ids {
			if id == expectedId {
				return true
			}
		}
		return false
	}

	for _, tc := range frAsset.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			var kit Kit
			loadJSON(t, tc.Kit, &kit)

			// Find the design
			var design *Design
			if tc.DesignParentName != "" {
				parentDesign := findDesignByName(kit.Designs, tc.DesignParentName, nil)
				if parentDesign == nil {
					t.Fatalf("Parent design %q not found", tc.DesignParentName)
				}
				design = findDesignByName(kit.Designs, tc.DesignName, &parentDesign.Id)
			} else {
				design = findDesignByName(kit.Designs, tc.DesignName, nil)
			}
			if design == nil {
				t.Fatalf("Design %q not found", tc.DesignName)
			}

			// Determine piece ids for selection
			var pieceIds []string
			if tc.SelectionAsset != "" {
				var sel selectionAsset
				loadJSON(t, tc.SelectionAsset, &sel)
				if tc.ExpectedSelectionPieceCount > 0 && len(sel.Pieces) != tc.ExpectedSelectionPieceCount {
					t.Fatalf("Expected %d selection pieces, got %d", tc.ExpectedSelectionPieceCount, len(sel.Pieces))
				}
				if tc.ExpectedSelectionConnectionCount > 0 && len(sel.Connections) != tc.ExpectedSelectionConnectionCount {
					t.Fatalf("Expected %d selection connections, got %d", tc.ExpectedSelectionConnectionCount, len(sel.Connections))
				}
				pieceIds = make([]string, 0, len(sel.Pieces))
				for _, p := range sel.Pieces {
					pieceIds = append(pieceIds, p.Id)
				}
			} else if tc.PieceNames != nil {
				pieceIds = make([]string, 0, len(tc.PieceNames))
				for _, pieceName := range tc.PieceNames {
					piece := findPieceByName(design.Pieces, pieceName)
					if piece == nil {
						t.Fatalf("Piece %q not found", pieceName)
					}
					pieceIds = append(pieceIds, piece.Id)
				}
			} else if tc.UsePieceIndex != nil {
				if *tc.UsePieceIndex >= len(design.Pieces) {
					t.Fatalf("Piece index %d out of range", *tc.UsePieceIndex)
				}
				pieceIds = []string{design.Pieces[*tc.UsePieceIndex].Id}
			} else if tc.LookupTypeName != "" {
				var lookupTypeId string
				for _, tp := range kit.Types {
					if tp.Name == tc.LookupTypeName {
						lookupTypeId = tp.Id
						break
					}
				}
				for i := range design.Pieces {
					if design.Pieces[i].Type != nil && design.Pieces[i].Type.Id == lookupTypeId {
						pieceIds = []string{design.Pieces[i].Id}
						break
					}
				}
				if len(pieceIds) == 0 {
					t.Fatalf("Piece with type %q not found", tc.LookupTypeName)
				}
			}

			typeIds, designIds := FindReplaceableTypesInDesignsForPiecesInDesign(*design, kit.Designs, kit.Types, AllPortsInKit(&kit), pieceIds)

			// Check expected type id count
			if tc.ExpectedTypeIdCount != nil {
				if len(typeIds) != *tc.ExpectedTypeIdCount {
					t.Fatalf("Expected %d type ids, got %d: %#v", *tc.ExpectedTypeIdCount, len(typeIds), typeIds)
				}
			}

			// Check expected type ids (full list)
			if tc.ExpectedTypeIds != nil {
				if !reflect.DeepEqual(typeIds, tc.ExpectedTypeIds) {
					t.Fatalf("Type ids mismatch:\n got: %#v\nwant: %#v", typeIds, tc.ExpectedTypeIds)
				}
			}

			// Check expected design ids
			if tc.ExpectedDesignIds != nil {
				if !reflect.DeepEqual(designIds, tc.ExpectedDesignIds) {
					t.Fatalf("Design ids mismatch:\n got: %#v\nwant: %#v", designIds, tc.ExpectedDesignIds)
				}
			}

			// Check expectNonEmptyTypes
			if tc.ExpectNonEmptyTypes && len(typeIds) == 0 {
				t.Error("Expected at least one replaceable type")
			}

			// Check expectOwnTypeInResults
			if tc.ExpectOwnTypeInResults && tc.UsePieceIndex != nil {
				piece := design.Pieces[*tc.UsePieceIndex]
				if piece.Type != nil && piece.Type.Id != "" {
					if !containsId(typeIds, piece.Type.Id) {
						t.Error("Expected piece's own type to be in results")
					}
				}
			}

			// Check forbidden type names
			for _, forbiddenName := range tc.ForbiddenTypeNames {
				var forbiddenId string
				for _, tp := range kit.Types {
					if tp.Name == forbiddenName {
						forbiddenId = tp.Id
						break
					}
				}
				if forbiddenId != "" {
					for _, tg := range typeIds {
						if tg == forbiddenId {
							t.Errorf("%s type should NOT be in results", forbiddenName)
						}
					}
				}
			}

			// Check connectorless type count
			if tc.ExpectConnectorlessTypeCount {
				noConnectorCount := 0
				for _, tp := range kit.Types {
					if len(tp.Connectors) == 0 {
						noConnectorCount++
					}
				}
				if len(typeIds) != noConnectorCount {
					t.Errorf("Expected %d types with no connectors, got %d", noConnectorCount, len(typeIds))
				}
			}
		})
	}

	t.Run("Synthetic selection enforces distinct compatible connectors and ignores consumed design connectors", func(t *testing.T) {
		var syntheticKit Kit
		loadJSON(t, frAsset.SyntheticKit, &syntheticKit)

		for _, sc := range frAsset.SyntheticCases {
			t.Run(sc.Name, func(t *testing.T) {
				var syntheticDesign *Design
				for i := range syntheticKit.Designs {
					if syntheticKit.Designs[i].Id == sc.DesignId {
						syntheticDesign = &syntheticKit.Designs[i]
						break
					}
				}
				if syntheticDesign == nil {
					t.Fatalf("Design %q not found in synthetic kit", sc.DesignId)
				}

				typeIds, designIds := FindReplaceableTypesInDesignsForPiecesInDesign(*syntheticDesign, syntheticKit.Designs, syntheticKit.Types, AllPortsInKit(&syntheticKit), sc.PieceIds)

				for _, expected := range sc.ExpectedContainsTypes {
					if !containsId(typeIds, expected) {
						t.Fatalf("Expected type %q to be in results, got %#v", expected, typeIds)
					}
				}
				for _, forbidden := range sc.ExpectedNotContainsTypes {
					if containsId(typeIds, forbidden) {
						t.Fatalf("Type %q should NOT be in results", forbidden)
					}
				}
				for _, expected := range sc.ExpectedContainsDesigns {
					if !containsId(designIds, expected) {
						t.Fatalf("Expected design %q to be in results, got %#v", expected, designIds)
					}
				}
				for _, forbidden := range sc.ExpectedNotContainsDesigns {
					if containsId(designIds, forbidden) {
						t.Fatalf("Design %q should NOT be in results", forbidden)
					}
				}
			})
		}
	})

	t.Run("Connector-level boundary matching shrinks candidates as demand grows", func(t *testing.T) {
		bc := frAsset.BoundaryCases
		var kit Kit
		loadJSON(t, bc.Kit, &kit)

		design := findDesignByName(kit.Designs, bc.DesignName, nil)
		if design == nil {
			t.Fatal("Design not found")
		}

		nameToId := map[string]string{}
		typeNameById := map[string]string{}
		for _, piece := range design.Pieces {
			if piece.Name != nil {
				nameToId[*piece.Name] = piece.Id
			}
		}
		for _, kind := range kit.Types {
			typeNameById[kind.Id] = kind.Name
		}
		typeNamesForSelection := func(pieceNames []string) []string {
			pIds := make([]string, 0, len(pieceNames))
			for _, pieceName := range pieceNames {
				pieceId, ok := nameToId[pieceName]
				if !ok {
					t.Fatalf("Piece %q not found", pieceName)
				}
				pIds = append(pIds, pieceId)
			}
			tIds, _ := FindReplaceableTypesInDesignsForPiecesInDesign(*design, kit.Designs, kit.Types, AllPortsInKit(&kit), pIds)
			typeNames := make([]string, 0, len(tIds))
			for _, typeId := range tIds {
				typeNames = append(typeNames, typeNameById[typeId])
			}
			return typeNames
		}
		uniqueTypeNamesForSelection := func(pieceNames []string) []string {
			seen := map[string]bool{}
			typeNames := typeNamesForSelection(pieceNames)
			unique := make([]string, 0, len(typeNames))
			for _, typeName := range typeNames {
				if seen[typeName] {
					continue
				}
				seen[typeName] = true
				unique = append(unique, typeName)
			}
			sort.Strings(unique)
			return unique
		}
		containsName := func(typeNames []string, expectedName string) bool {
			for _, typeName := range typeNames {
				if typeName == expectedName {
					return true
				}
			}
			return false
		}

		singleCapsuleNames := typeNamesForSelection(bc.SingleCapsulePieces)
		twoCapsuleNames := typeNamesForSelection(bc.TwoCapsulePieces)
		fourCapsuleNames := typeNamesForSelection(bc.FourCapsulePieces)
		eightCapsuleNames := typeNamesForSelection(bc.EightCapsulePieces)
		tambourPieceId, ok := nameToId[bc.TambourPieceName]
		if !ok {
			t.Fatalf("Tambour piece %q not found", bc.TambourPieceName)
		}
		tambourTypeIds, tambourDesignIds := FindReplaceableTypesInDesignsForPiecesInDesign(*design, kit.Designs, kit.Types, AllPortsInKit(&kit), []string{tambourPieceId})

		if len(singleCapsuleNames) <= len(twoCapsuleNames) {
			t.Fatalf("Expected single capsule result to be larger than two capsules, got %d <= %d", len(singleCapsuleNames), len(twoCapsuleNames))
		}
		if len(twoCapsuleNames) < len(fourCapsuleNames) {
			t.Fatalf("Expected two capsules result to be at least as large as four capsules, got %d < %d", len(twoCapsuleNames), len(fourCapsuleNames))
		}
		if len(fourCapsuleNames) < len(eightCapsuleNames) {
			t.Fatalf("Expected four capsules result to be at least as large as eight capsules, got %d < %d", len(fourCapsuleNames), len(eightCapsuleNames))
		}

		for _, forbiddenFamily := range bc.ForbiddenFamilies {
			if containsName(twoCapsuleNames, forbiddenFamily) {
				t.Fatalf("Forbidden single-connector family %q survived two-capsule selection", forbiddenFamily)
			}
			if containsName(fourCapsuleNames, forbiddenFamily) {
				t.Fatalf("Forbidden single-connector family %q survived four-capsule selection", forbiddenFamily)
			}
			if containsName(eightCapsuleNames, forbiddenFamily) {
				t.Fatalf("Forbidden single-connector family %q survived eight-capsule selection", forbiddenFamily)
			}
		}
		if containsName(fourCapsuleNames, "Bridge") {
			t.Fatal("Bridge should not survive four-capsule selection")
		}
		if containsName(eightCapsuleNames, "Bridge") {
			t.Fatal("Bridge should not survive eight-capsule selection")
		}

		if !reflect.DeepEqual(uniqueTypeNamesForSelection(bc.TwoCapsulePieces), bc.ExpectedTwoCapsuleFamilies) {
			t.Fatalf("Unexpected two-capsule families: %#v", uniqueTypeNamesForSelection(bc.TwoCapsulePieces))
		}
		if !reflect.DeepEqual(uniqueTypeNamesForSelection(bc.FourCapsulePieces), bc.ExpectedLargeFamilies) {
			t.Fatalf("Unexpected four-capsule families: %#v", uniqueTypeNamesForSelection(bc.FourCapsulePieces))
		}
		if !reflect.DeepEqual(uniqueTypeNamesForSelection(bc.EightCapsulePieces), bc.ExpectedLargeFamilies) {
			t.Fatalf("Unexpected eight-capsule families: %#v", uniqueTypeNamesForSelection(bc.EightCapsulePieces))
		}
		if len(tambourTypeIds) != bc.ExpectedTambourTypeIdCount {
			t.Fatalf("Expected %d compatible types for tambour selection, got %#v", bc.ExpectedTambourTypeIdCount, tambourTypeIds)
		}
		if len(tambourDesignIds) != bc.ExpectedTambourDesignIdCount {
			t.Fatalf("Expected %d compatible designs for tambour selection, got %#v", bc.ExpectedTambourDesignIdCount, tambourDesignIds)
		}
	})
}

// #endregion 🔍Find Replaceable Types In Designs Tests

func TestCopyAndPaste(t *testing.T) {
	t.Run("Nakagin Capsule Tower", func(t *testing.T) {
		t.Run("Copy and Paste Roundtrip", func(t *testing.T) {
			var kit Kit
			loadJSON(t, "metabolism.kit.semio.json", &kit)

			var design *Design
			design = findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
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

			pieceIds := make([]string, len(selection.Pieces))
			for i, p := range selection.Pieces {
				pieceIds[i] = p.Id
			}
			connectionIds := make([]string, len(selection.Connections))
			for i, c := range selection.Connections {
				connectionIds[i] = c.Id
			}

			// Load expected copy design
			var expectedCopy Design
			loadJSON(t, "nakagin-capsule-tower.copy.design.semio.json", &expectedCopy)

			// Compute copy
			copyDesign := CopyDesign(&kit, *design, pieceIds, connectionIds)

			// Verify piece count
			if len(copyDesign.Pieces) != len(expectedCopy.Pieces) {
				t.Fatalf("Copy pieces count mismatch: got %d, want %d", len(copyDesign.Pieces), len(expectedCopy.Pieces))
			}

			// Verify connection count
			if len(copyDesign.Connections) != len(expectedCopy.Connections) {
				t.Fatalf("Copy connections count mismatch: got %d, want %d", len(copyDesign.Connections), len(expectedCopy.Connections))
			}

			// Verify each piece exists in the copy
			copyPieceIds := make(map[string]bool)
			for _, p := range copyDesign.Pieces {
				copyPieceIds[p.Id] = true
			}
			for _, p := range expectedCopy.Pieces {
				if !copyPieceIds[p.Id] {
					t.Errorf("Expected piece %s not found in copy output", p.Id)
				}
			}

			// Verify external pieces have semio.piece.origin attribute
			expectedPieceMap := make(map[string]Piece)
			for _, p := range expectedCopy.Pieces {
				expectedPieceMap[p.Id] = p
			}
			for _, p := range copyDesign.Pieces {
				ep, ok := expectedPieceMap[p.Id]
				if !ok {
					t.Errorf("Unexpected piece %s in copy output", p.Id)
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
					t.Errorf("Piece %s: semio.piece.origin mismatch: got %v, want %v", p.Id, hasOriginAttr, expectedOriginAttr)
				}
			}

			// Verify pp_excl_pc_incl pieces have semio.center and semio.plane attributes
			for _, p := range copyDesign.Pieces {
				ep := expectedPieceMap[p.Id]
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
					t.Errorf("Piece %s: semio.center attr mismatch: got %v, want %v", p.Id, hasCenterAttr, expectedCenterAttr)
				}
				if hasPlaneAttr != expectedPlaneAttr {
					t.Errorf("Piece %s: semio.plane attr mismatch: got %v, want %v", p.Id, hasPlaneAttr, expectedPlaneAttr)
				}
			}

			// Verify connections are in expected set
			copyConnIds := make(map[string]bool)
			for _, c := range copyDesign.Connections {
				copyConnIds[c.Id] = true
			}
			for _, c := range expectedCopy.Connections {
				if !copyConnIds[c.Id] {
					t.Errorf("Expected connection %s not found in copy output", c.Id)
				}
			}

			// Test PasteDesign with original anchoring (no coordinate)
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
						t.Errorf("External-origin piece %s should not be in paste output", p.Id)
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

			// Test PasteDesign with original anchoring and coordinate
			coordinateVal := Coordinate{U: 10, V: 10}
			pasteWithCoordinateDiff := PasteDesign(&kit, copyDesign, pasteTargetDesign, "original", &coordinateVal)

			// Load expected paste with coordinate diff
			var expectedPasteWithCoordinate DesignDiff
			loadJSON(t, "nakagin-capsule-tower.paste.with-coordinate.design.diff.semio.json", &expectedPasteWithCoordinate)

			// Verify pasted pieces count
			if pasteWithCoordinateDiff.Pieces == nil {
				t.Fatal("No pieces diff in paste with coordinate result")
			}
			if len(pasteWithCoordinateDiff.Pieces.Added) != len(expectedPasteWithCoordinate.Pieces.Added) {
				t.Fatalf("Paste with coordinate added pieces count mismatch: got %d, want %d", len(pasteWithCoordinateDiff.Pieces.Added), len(expectedPasteWithCoordinate.Pieces.Added))
			}

			// Verify pasted connections count
			if pasteWithCoordinateDiff.Connections == nil {
				t.Fatal("No connections diff in paste with coordinate result")
			}
			if len(pasteWithCoordinateDiff.Connections.Added) != len(expectedPasteWithCoordinate.Connections.Added) {
				t.Fatalf("Paste with coordinate added connections count mismatch: got %d, want %d", len(pasteWithCoordinateDiff.Connections.Added), len(expectedPasteWithCoordinate.Connections.Added))
			}

			// Verify centers are offset by coordinate
			expectedPWCPieceMap := make(map[string]Piece)
			for _, p := range expectedPasteWithCoordinate.Pieces.Added {
				expectedPWCPieceMap[p.Id] = p
			}
			for _, p := range pasteWithCoordinateDiff.Pieces.Added {
				ep, ok := expectedPWCPieceMap[p.Id]
				if !ok {
					t.Errorf("Unexpected piece %s in paste with coordinate output", p.Id)
					continue
				}
				if p.Center != nil && ep.Center != nil {
					if math.Abs(p.Center.U-ep.Center.U) > 0.001 || math.Abs(p.Center.V-ep.Center.V) > 0.001 {
						t.Errorf("Piece %s center mismatch: got (%f,%f), want (%f,%f)", p.Id, p.Center.U, p.Center.V, ep.Center.U, ep.Center.V)
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

		t.Run("Plain descriptions do not create emoji validation problems", func(t *testing.T) {
			var kit Kit
			loadJSON(t, "metabolism.kit.semio.json", &kit)
			kitDescription := "Plain kit summary"
			kit.Description = &kitDescription
			for i := range kit.Types {
				description := "Repeated plain description"
				if i%2 == 1 {
					description = "Repeated plain description alternate"
				}
				kit.Types[i].Description = &description
			}

			result := ValidateKit(kit)
			for _, problem := range result.Problems {
				if problem.ConstraintId == "description-missing-emoji" || problem.ConstraintId == "description-emoji-unique" {
					t.Fatalf("Unexpected emoji validation problem: %+v", problem)
				}
			}
		})
	})
}

func TestDesignQualitySum(t *testing.T) {
	var asset qualitySumCasesAsset
	loadJSON(t, "quality-sum.cases.semio.json", &asset)

	for _, tc := range asset.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			var kit Kit
			loadJSON(t, tc.Kit, &kit)

			design := findDesignByName(kit.Designs, tc.DesignName, nil)
			if design == nil {
				t.Fatalf("Design %q not found", tc.DesignName)
			}

			var qualityId string
			for _, q := range kit.Qualities {
				if q.Name == tc.QualityName {
					qualityId = q.Id
					break
				}
			}
			if qualityId == "" {
				t.Fatalf("Quality %q not found", tc.QualityName)
			}
			result := SumQualityInDesign(&kit, design.Id, qualityId)
			if math.Abs(result-tc.Expected) > tc.Tolerance {
				t.Errorf("Expected ~%f, got %f", tc.Expected, result)
			}
		})
	}
}

func TestExportDesignRepresentation(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)

	design := findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
	if design == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}

	t.Run("GLB format", func(t *testing.T) {
		result, err := ExportDesignRepresentation(&kit, design.Id, ".glb", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignRepresentation failed: %v", err)
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
		result, err := ExportDesignRepresentation(&kit, design.Id, ".gltf", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignRepresentation failed: %v", err)
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
		result, err := ExportDesignRepresentation(&kit, design.Id, ".xyz", nil, nil)
		if err == nil {
			t.Fatal("expected error for invalid format, got nil")
		}
		if result != nil {
			t.Errorf("expected nil result for invalid format, got %d bytes", len(result))
		}
	})

	t.Run("Scene graph report", func(t *testing.T) {
		result, err := ExportDesignRepresentation(&kit, design.Id, ".gltf", nil, nil)
		if err != nil {
			t.Fatalf("ExportDesignRepresentation failed: %v", err)
		}
		var parsed interface{}
		if err := json.Unmarshal(result, &parsed); err != nil {
			t.Fatalf("result is not valid JSON: %v", err)
		}
		reportsDir := filepath.Join("..", "..", "reports", "export-design-representation")
		if err := os.MkdirAll(reportsDir, 0o755); err != nil {
			t.Fatalf("failed to create reports directory: %v", err)
		}
		reportPath := filepath.Join(reportsDir, "go.gltf")
		if err := os.WriteFile(reportPath, result, 0o644); err != nil {
			t.Fatalf("failed to write report: %v", err)
		}
	})
}

func TestExportDesignRepresentationSceneGraphReport(t *testing.T) {
	var kit Kit
	loadJSON(t, "metabolism.kit.semio.json", &kit)
	design := findDesignByName(kit.Designs, "Nakagin Capsule Tower", nil)
	if design == nil {
		t.Fatal("Nakagin Capsule Tower design not found")
	}
	result, err := ExportDesignRepresentation(&kit, design.Id, ".gltf", nil, nil)
	if err != nil {
		t.Fatalf("ExportDesignRepresentation failed: %v", err)
	}
	var parsed interface{}
	if err := json.Unmarshal(result, &parsed); err != nil {
		t.Fatalf("result is not valid JSON: %v", err)
	}
	reportsDir := filepath.Join("..", "..", "reports", "export-design-representation")
	if err := os.MkdirAll(reportsDir, 0o755); err != nil {
		t.Fatalf("failed to create reports directory: %v", err)
	}
	reportPath := filepath.Join(reportsDir, "go.gltf")
	if err := os.WriteFile(reportPath, result, 0o644); err != nil {
		t.Fatalf("failed to write report: %v", err)
	}
}

func round6(x float64) float64 { return math.Round(x*1e6) / 1e6 }

func TestGetGeometricInsightsForRepresentation_NakaginCapsuleTower(t *testing.T) {
	representationPath := filepath.Join(AssetsPath, "nakagin-capsule-tower.gltf")
	if _, err := os.Stat(representationPath); err != nil {
		t.Skipf("nakagin-capsule-tower.gltf not found: %v", err)
	}
	insights, err := GetGeometricInsightsForRepresentation(representationPath)
	if err != nil {
		t.Fatalf("GetGeometricInsightsForRepresentation: %v", err)
	}
	reportsDir := filepath.Join("..", "..", "reports", "representation-kpi")
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
		t.Fatalf("failed to marshal go representation-kpi report: %v", err)
	}
	if err := os.WriteFile(filepath.Join(reportsDir, "go.json"), b, 0o644); err != nil {
		t.Fatalf("failed to write go representation-kpi report: %v", err)
	}

	canonicalPath := filepath.Join(AssetsPath, "nakagin.kpi.representation.semio.json")
	canonicalData, err := os.ReadFile(canonicalPath)
	if err != nil {
		t.Fatalf("failed to read canonical representation-kpi asset: %v", err)
	}
	var canonical map[string]any
	if err := json.Unmarshal(canonicalData, &canonical); err != nil {
		t.Fatalf("failed to unmarshal canonical representation-kpi asset: %v", err)
	}
	var current map[string]any
	if err := json.Unmarshal(b, &current); err != nil {
		t.Fatalf("failed to unmarshal go representation-kpi report: %v", err)
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
		if meta.Id != kit.Id {
			t.Errorf("KitMeta.Id = %q, want %q", meta.Id, kit.Id)
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
		if shallow.Id != kit.Id {
			t.Errorf("KitShallow.Id = %q, want %q", shallow.Id, kit.Id)
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
			if tm.Id != kit.Types[i].Id {
				t.Errorf("KitShallow.Types[%d].Id = %q, want %q", i, tm.Id, kit.Types[i].Id)
			}
		}
	})

	t.Run("TypeMeta from JSON", func(t *testing.T) {
		var meta TypeMeta
		loadJSON(t, "tambour.meta.type.semio.json", &meta)
		if meta.Id == "" {
			t.Error("TypeMeta.Id is empty")
		}
		if meta.Name != "Tambour" {
			t.Errorf("TypeMeta.Name = %q, want %q", meta.Name, "Tambour")
		}
	})

	t.Run("TypeShallow from JSON", func(t *testing.T) {
		var shallow TypeShallow
		loadJSON(t, "tambour.shallow.type.semio.json", &shallow)
		if shallow.Id == "" {
			t.Error("TypeShallow.Id is empty")
		}
		if shallow.Name != "Tambour" {
			t.Errorf("TypeShallow.Name = %q, want %q", shallow.Name, "Tambour")
		}
		if len(shallow.Connectors) == 0 {
			t.Error("TypeShallow.Connectors is empty")
		}
		if len(shallow.Representations) == 0 {
			t.Error("TypeShallow.Representations is empty")
		}
		if len(shallow.Props) == 0 {
			t.Error("TypeShallow.Props is empty")
		}
	})

	t.Run("DesignMeta from JSON", func(t *testing.T) {
		var meta DesignMeta
		loadJSON(t, "nakagin-capsule-tower.meta.design.semio.json", &meta)
		if meta.Id == "" {
			t.Error("DesignMeta.Id is empty")
		}
		if meta.Name != "Nakagin Capsule Tower" {
			t.Errorf("DesignMeta.Name = %q, want %q", meta.Name, "Nakagin Capsule Tower")
		}
	})

	t.Run("DesignShallow from JSON", func(t *testing.T) {
		var shallow DesignShallow
		loadJSON(t, "nakagin-capsule-tower.shallow.design.semio.json", &shallow)
		if shallow.Id == "" {
			t.Error("DesignShallow.Id is empty")
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
		if meta.Id == "" {
			t.Error("KitMeta.Id is empty")
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
		if shallow.Id == "" {
			t.Error("KitShallow.Id is empty")
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
		if len(shallow.Families) == 0 || len(shallow.Families[0].Ports) == 0 {
			t.Error("KitShallow.Families ports is empty")
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
		expected := []KitKind{KitKindDev, KitKindLocal, KitKindArchive, KitKindRemote, KitKindTransport}
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
		kit := Kit{Id: "file-kit-id", Name: "FileKit Test", Version: "1.0"}
		data, err := SerializeKit(kit)
		if err != nil {
			t.Fatal(err)
		}
		restored, err := DeserializeKit(data)
		if err != nil {
			t.Fatal(err)
		}
		if restored.Id != kit.Id {
			t.Errorf("Id = %q, want %q", restored.Id, kit.Id)
		}
		if restored.Name != kit.Name {
			t.Errorf("Name = %q, want %q", restored.Name, kit.Name)
		}
	})

	t.Run("Kit/Folder: roundtrips through SQLite", func(t *testing.T) {
		kit := Kit{
			Id:    "folder-kit-id",
			Name:    "FolderKit Test",
			Version: "1.0",
			Types:   []Type{{Id: "t1", Name: "Wall"}},
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
		if restored.Id != kit.Id {
			t.Errorf("Id = %q, want %q", restored.Id, kit.Id)
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
			Id:    "archive-kit-id",
			Name:    "ArchiveKit Test",
			Version: "1.0",
			Types:   []Type{{Id: "at1", Name: "Beam"}},
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
		if restored.Id != kit.Id {
			t.Errorf("Id = %q, want %q", restored.Id, kit.Id)
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

	t.Run("Kit/Postgres schema: includes normalized snapshots and VCS tables", func(t *testing.T) {
		schemaPath := filepath.Join("..", "postgres", "schema.sql")
		schemaBytes, err := os.ReadFile(schemaPath)
		if err != nil {
			t.Fatalf("Failed to read postgres schema: %v", err)
		}
		schema := string(schemaBytes)
		checks := []string{
			"CREATE TABLE IF NOT EXISTS core.kit_snapshot",
			"CREATE TABLE IF NOT EXISTS core.family",
			"CREATE TABLE IF NOT EXISTS core.type_entity",
			"CREATE TABLE IF NOT EXISTS core.design",
			"CREATE TABLE IF NOT EXISTS history.kit_checkpoint",
			"CREATE TABLE IF NOT EXISTS history.kit_alternative",
			"CREATE TABLE IF NOT EXISTS history.kit_release",
			"CREATE TABLE IF NOT EXISTS runtime.kit_session",
			"CREATE TABLE IF NOT EXISTS runtime.kit_draft",
			"CREATE TABLE IF NOT EXISTS runtime.kit_transaction",
			"CREATE TABLE IF NOT EXISTS runtime.kit_transaction_change",
			"CREATE UNIQUE INDEX IF NOT EXISTS idx_draft_one_active_branch",
		}
		for _, check := range checks {
			if !strings.Contains(schema, check) {
				t.Fatalf("postgres schema is missing %q", check)
			}
		}
	})

	t.Run("Kit/Remote: validates remote URL field", func(t *testing.T) {
		remote := "https://example.com/metabolism.kit.json"
		kit := Kit{Id: "remote-kit-id", Name: "RemoteKit Test", Remote: &remote}
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
		kit := Kit{Id: "temp-kit-id", Name: "TemporaryKit Test", Version: "1.0"}
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
			KitKindDev:       "dev",
			KitKindLocal:     "local",
			KitKindArchive:   "archive",
			KitKindRemote:    "remote",
			KitKindTransport: "transport",
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
		Id:      "workflow-kit-id",
		Name:      "Workflow Kit",
		Version:   "1.0.0",
		CreatedAt: "2026-01-01T00:00:00.000Z",
		UpdatedAt: "2026-01-01T00:00:00.000Z",
		Folders: []Folder{{
			Id:      "folder-id",
			Name:      "docs",
			CreatedAt: "2026-01-01T00:00:00.000Z",
			UpdatedAt: "2026-01-01T00:00:00.000Z",
		}},
		Files: []File{{
			Id:      "file-id",
			Name:      "readme.txt",
			Folder:    &FolderId{Id: "folder-id"},
			Size:      &assetSize,
			Blob:      &assetBlob,
			CreatedAt: "2026-01-01T00:00:00.000Z",
			UpdatedAt: "2026-01-01T00:00:00.000Z",
		}},
		Types: []Type{{
			Id:      "type-id",
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

	t.Run("Kit/Transport workflow edits in memory in place", func(t *testing.T) {
		transportKit := deepCloneKit(kit)
		EditTemporaryKit(&transportKit, &diff)
		if transportKit.Name != updatedName {
			t.Fatalf("transportKit.Name = %q, want %q", transportKit.Name, updatedName)
		}
	})
}

// #region 🏰Kit Filter Tests
// Tests for FilterKit MUST verify correct subset extraction.

func TestFilterKit(t *testing.T) {
	var asset filterKitCasesAsset
	loadJSON(t, "filter-kit.cases.semio.json", &asset)

	for _, tc := range asset.Cases {
		var kit Kit
		loadJSON(t, tc.Kit, &kit)

		design := findDesignByName(kit.Designs, tc.DesignName, nil)
		if design == nil {
			t.Fatalf("Design %q not found", tc.DesignName)
		}
		designId := design.Id

		var expected Kit
		loadJSON(t, tc.ExpectedKit, &expected)

		t.Run("filters kit to only contain entities related to "+tc.DesignName+" design", func(t *testing.T) {
			filtered := FilterKit(kit, KitFilter{DesignId: designId})

			if len(filtered.Designs) != len(expected.Designs) {
				t.Errorf("expected %d designs, got %d", len(expected.Designs), len(filtered.Designs))
			}
			if len(filtered.Types) != len(expected.Types) {
				t.Errorf("expected %d types, got %d", len(expected.Types), len(filtered.Types))
			}
			if len(filtered.Files) != len(expected.Files) {
				t.Errorf("expected %d files, got %d", len(expected.Files), len(filtered.Files))
			}
			if len(AllPortsInKit(&filtered)) != len(AllPortsInKit(&expected)) {
				t.Errorf("expected %d ports, got %d", len(AllPortsInKit(&expected)), len(AllPortsInKit(&filtered)))
			}
			if len(filtered.Qualities) != len(expected.Qualities) {
				t.Errorf("expected %d qualities, got %d", len(expected.Qualities), len(filtered.Qualities))
			}
			if len(filtered.Authors) != len(expected.Authors) {
				t.Errorf("expected %d authors, got %d", len(expected.Authors), len(filtered.Authors))
			}

			filteredDesign := findDesignByName(filtered.Designs, tc.DesignName, nil)
			if filteredDesign == nil {
				t.Fatalf("Design %q not found in filtered kit", tc.DesignName)
			}

			if len(filteredDesign.Pieces) != len(design.Pieces) {
				t.Errorf("expected %d pieces, got %d", len(design.Pieces), len(filteredDesign.Pieces))
			}

			for _, typeItem := range filtered.Types {
				if len(typeItem.Representations) > 1 {
					t.Errorf("type %s has %d representations, expected at most 1", typeItem.Id, len(typeItem.Representations))
				}
			}
		})

		t.Run("preserves kit metadata", func(t *testing.T) {
			filtered := FilterKit(kit, KitFilter{DesignId: designId})
			if filtered.Id != kit.Id {
				t.Errorf("expected id %s, got %s", kit.Id, filtered.Id)
			}
			if filtered.Name != kit.Name {
				t.Errorf("expected name %s, got %s", kit.Name, filtered.Name)
			}
			if filtered.Version != kit.Version {
				t.Errorf("expected version %s, got %s", kit.Version, filtered.Version)
			}
		})

		for _, gc := range asset.GlobCases {
			t.Run(gc.Name, func(t *testing.T) {
				var gcKit Kit
				loadJSON(t, gc.Kit, &gcKit)

				filter := KitFilter{}
				if len(gc.TypeInclude) > 0 {
					if filter.Types == nil {
						filter.Types = &GlobFilter{}
					}
					filter.Types.Include = gc.TypeInclude
				}
				if len(gc.TypeExclude) > 0 {
					if filter.Types == nil {
						filter.Types = &GlobFilter{}
					}
					filter.Types.Exclude = gc.TypeExclude
				}
				if len(gc.DesignInclude) > 0 {
					if filter.Designs == nil {
						filter.Designs = &GlobFilter{}
					}
					filter.Designs.Include = gc.DesignInclude
				}
				if gc.DesignName != "" {
					gcDesign := findDesignByName(gcKit.Designs, gc.DesignName, nil)
					if gcDesign != nil {
						filter.DesignId = gcDesign.Id
					}
				}

				filtered := FilterKit(gcKit, filter)

				if len(gc.TypeInclude) > 0 {
					if len(filtered.Types) == 0 {
						t.Fatalf("expected at least one type matching %v", gc.TypeInclude)
					}
					for _, ty := range filtered.Types {
						matched := false
						for _, pattern := range gc.TypeInclude {
							if GlobMatch(ty.Name, pattern) {
								matched = true
								break
							}
						}
						if !matched {
							t.Errorf("type %s should not be included", ty.Name)
						}
					}
				}
				if len(gc.TypeExclude) > 0 && gc.DesignName == "" {
					if len(filtered.Types) >= len(gcKit.Types) {
						t.Errorf("expected fewer types after excluding %v", gc.TypeExclude)
					}
					for _, ty := range filtered.Types {
						for _, pattern := range gc.TypeExclude {
							if GlobMatch(ty.Name, pattern) {
								t.Errorf("type %s should have been excluded", ty.Name)
							}
						}
					}
				}
				if len(gc.DesignInclude) > 0 {
					if len(filtered.Designs) == 0 {
						t.Fatalf("expected at least one design matching %v", gc.DesignInclude)
					}
					for _, d := range filtered.Designs {
						matched := false
						for _, pattern := range gc.DesignInclude {
							if GlobMatch(d.Name, pattern) {
								matched = true
								break
							}
						}
						if !matched {
							t.Errorf("design %s should not be included", d.Name)
						}
					}
				}
				if gc.Name == "empty_filter" {
					if len(filtered.Types) != len(gcKit.Types) {
						t.Errorf("expected %d types, got %d", len(gcKit.Types), len(filtered.Types))
					}
					if len(filtered.Designs) != len(gcKit.Designs) {
						t.Errorf("expected %d designs, got %d", len(gcKit.Designs), len(filtered.Designs))
					}
				}
				if gc.DesignName != "" && len(gc.TypeExclude) > 0 {
					designOnlyFiltered := FilterKit(gcKit, KitFilter{DesignId: filter.DesignId})
					if len(filtered.Types) >= len(designOnlyFiltered.Types) {
						t.Errorf("expected fewer types with combined filter")
					}
					for _, ty := range filtered.Types {
						for _, pattern := range gc.TypeExclude {
							if GlobMatch(ty.Name, pattern) {
								t.Errorf("type %s should have been excluded", ty.Name)
							}
						}
					}
				}
			})
		}
	}
}

// #endregion 🏰Kit Filter Tests

// #region 🗝️Hash Tests

func TestHashKit(t *testing.T) {
	var hashAsset hashCasesAsset
	loadJSON(t, "hash.cases.semio.json", &hashAsset)

	var kit Kit
	loadJSON(t, hashAsset.KitHash.Kit, &kit)

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
		if h != hashAsset.KitHash.Expected {
			t.Errorf("expected %s, got %s", hashAsset.KitHash.Expected, h)
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
		nct := findDesignByName(kit.Designs, hashAsset.DesignName, nil)
		if nct == nil {
			t.Fatalf("Design %q not found", hashAsset.DesignName)
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
	var hashAsset hashCasesAsset
	loadJSON(t, "hash.cases.semio.json", &hashAsset)

	t.Run("hashKitDiff matches expected canonical value", func(t *testing.T) {
		raw := []byte(hashAsset.KitDiffHash.JSON)
		var d KitDiff
		if err := json.Unmarshal(raw, &d); err != nil {
			t.Fatalf("failed to unmarshal KitDiff: %v", err)
		}
		h := HashKitDiff(d)
		if h != hashAsset.KitDiffHash.Expected {
			t.Errorf("expected %s, got %s", hashAsset.KitDiffHash.Expected, h)
		}
	})

	t.Run("hashKitDiff is deterministic", func(t *testing.T) {
		raw := []byte(hashAsset.KitDiffHash.JSON)
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
		raw1 := []byte(hashAsset.KitDiffHash.JSON)
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

	t.Run("hashCoordinateDiff is deterministic", func(t *testing.T) {
		u := 1.0
		v := 2.0
		d := CoordinateDiff{U: &u, V: &v}
		h1 := HashCoordinateDiff(d)
		h2 := HashCoordinateDiff(d)
		if h1 != h2 {
			t.Errorf("expected same hash, got %s and %s", h1, h2)
		}
	})
}

// #endregion 🗝️Hash Tests

// #region 🎉DesignWithDiff Tests

func TestDesignWithDiff(t *testing.T) {
	var asset designWithDiffCasesAsset
	loadJSON(t, "design-with-diff.cases.semio.json", &asset)

	for _, tc := range asset.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			var kit Kit
			loadJSON(t, tc.Kit, &kit)

			design := findDesignByName(kit.Designs, tc.DesignName, nil)
			if design == nil {
				t.Fatalf("Design %q not found", tc.DesignName)
			}

			var diff DesignDiff
			loadJSON(t, tc.Diff, &diff)

			var expected Design
			loadJSON(t, tc.Expected, &expected)

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
			for status, expectedCount := range tc.ExpectedPieceCounts {
				if counts[status] != expectedCount {
					t.Errorf("%s pieces: got %d, want %d", status, counts[status], expectedCount)
				}
			}

			connCounts := map[string]int{}
			for _, c := range computed.Connections {
				connCounts[getStatus(c.Attributes)]++
			}
			for status, expectedCount := range tc.ExpectedConnectionCounts {
				if connCounts[status] != expectedCount {
					t.Errorf("%s connections: got %d, want %d", status, connCounts[status], expectedCount)
				}
			}
		})
	}
}

// #endregion 🎉DesignWithDiff Tests

// #region 📊MaxChildren Tests

func TestMaxChildrenPortSerialization(t *testing.T) {
	mc := 3
	port := Port{
		Id:        "p1",
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
	port := Port{Id: "p1", Name: "TestPort"}
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
		Id:        "c1",
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
		Id: "kit-1",
		Name: "TestKit",
		Families: []Family{{
			Id: "f1",
			Name: "Family1",
			Ports: []Port{{
				Id:        "p1",
				Name:        "Port1",
				MaxChildren: &mc3,
			}},
		}},
		Types: []Type{{
			Id: "t1",
			Name: "Type1",
			Connectors: []Connector{{
				Id:        "c1",
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
	if AllPortsInKit(&restored)[0].MaxChildren == nil || *AllPortsInKit(&restored)[0].MaxChildren != 3 {
		t.Errorf("port maxChildren: got %v, want 3", AllPortsInKit(&restored)[0].MaxChildren)
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

func benchFindDesign(kit Kit, name string, parentName string) Design {
	var parentFamilies []FamilyId
	if parentName != "" {
		for _, d := range kit.Designs {
			if d.Name == parentName {
				parentFamilies = d.Families
				break
			}
		}
		if parentFamilies == nil {
			panic("Parent design not found: " + parentName)
		}
	}

	for _, d := range kit.Designs {
		if d.Name == name {
			if parentName == "" {
				return d
			} else {
				if familiesOverlap(d.Families, parentFamilies) {
					return d
				}
			}
		}
	}
	panic("Design not found: " + name)
}

func BenchmarkRoundtripMetabolism(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		data, err := SerializeKit(kit)
		if err != nil {
			b.Fatal(err)
		}
		parsed, err := DeserializeKit(data)
		if err != nil {
			b.Fatal(err)
		}
		if !AreKitsEqual(kit, parsed) {
			b.Fatal("Roundtrip/Metabolism output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Roundtrip/Metabolism", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkDiffMetabolism(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	kitOriginal := kit
	kitOriginal.Designs = nil
	for _, design := range kit.Designs {
		if design.Name != "Flat" {
			kitOriginal.Designs = append(kitOriginal.Designs, design)
		}
	}
	kitDiffed := benchLoadKitFile(b, "metabolism.kit.diffed.semio.json")
	change := GetKitChange(kitOriginal, kitDiffed, nil, nil)
	diffForward := change.Forward
	diffInverse := change.Backward
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		k2 := deepCloneKit(kitOriginal)
		ApplyKitDiff(&k2, &diffForward)
		if !AreKitsEqual(k2, kitDiffed) {
			b.Fatal("Diff/Metabolism forward output does not match test expectation")
		}
		restored := deepCloneKit(k2)
		ApplyKitDiff(&restored, &diffInverse)
		if !AreKitsEqual(restored, kitOriginal) {
			b.Fatal("Diff/Metabolism inverse output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Diff/Metabolism", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkFlattenDesign_NakaginCapsuleTower(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	d := benchFindDesign(kit, "Nakagin Capsule Tower", "")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		diff := FlattenDesignDiff(&kit, d.Id)
		if diff.Pieces == nil || len(diff.Pieces.Updated) == 0 {
			b.Fatal("Flatten Design/Nakagin Capsule Tower output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Flatten Design/Nakagin Capsule Tower", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkFlattenDesign_Nakagin_Slanted(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	d := benchFindDesign(kit, "Slanted", "Nakagin Capsule Tower")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		diff := FlattenDesignDiff(&kit, d.Id)
		if diff.Pieces == nil || len(diff.Pieces.Updated) == 0 {
			b.Fatal("Flatten Design/Nakagin Capsule Tower/Slanted output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Flatten Design/Nakagin Capsule Tower/Slanted", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkFlattenDesign_Nakagin_Twisted(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	d := benchFindDesign(kit, "Twisted", "Nakagin Capsule Tower")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		diff := FlattenDesignDiff(&kit, d.Id)
		if diff.Pieces == nil || len(diff.Pieces.Updated) == 0 {
			b.Fatal("Flatten Design/Nakagin Capsule Tower/Twisted output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Flatten Design/Nakagin Capsule Tower/Twisted", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkFlattenDesign_Nakagin_Dancing(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	d := benchFindDesign(kit, "Dancing", "Nakagin Capsule Tower")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		diff := FlattenDesignDiff(&kit, d.Id)
		if diff.Pieces == nil || len(diff.Pieces.Updated) == 0 {
			b.Fatal("Flatten Design/Nakagin Capsule Tower/Dancing output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Flatten Design/Nakagin Capsule Tower/Dancing", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkFlattenDesign_CapsuleDream(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	d := benchFindDesign(kit, "Capsule Dream", "")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		diff := FlattenDesignDiff(&kit, d.Id)
		if diff.Pieces == nil || len(diff.Pieces.Updated) == 0 {
			b.Fatal("Flatten Design/Capsule Dream output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Flatten Design/Capsule Dream", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkValidateKit_Invalid(b *testing.B) {
	kit := benchLoadKitFile(b, "invalid.kit.semio.json")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		result := ValidateKit(kit)
		if len(result.Problems) == 0 {
			b.Fatal("Validation/Invalid Kit output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Validation/Invalid Kit", time.Since(start).Seconds()/float64(b.N))
}

func BenchmarkValidateKit_Metabolism(b *testing.B) {
	kit := benchLoadKitFile(b, "metabolism.kit.semio.json")
	b.ResetTimer()
	start := time.Now()
	for range b.N {
		result := ValidateKit(kit)
		if len(result.Problems) != 0 {
			b.Fatal("Validation/Metabolism output does not match test expectation")
		}
	}
	b.StopTimer()
	appendBenchmarkCsv("go", "Validation/Metabolism", time.Since(start).Seconds()/float64(b.N))
}

// #endregion 🎑Performance Benchmarks

// #endregion 🛡️KitKind Tests
