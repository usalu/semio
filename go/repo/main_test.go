// #region Header

// go/repo/repo_test.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

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

// #endregion Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// #region Helpers

func findTestRepoRoot(start string) string {
	dir := start
	for {
		if _, err := os.Stat(filepath.Join(dir, "AGENTS.md")); err == nil {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return start
		}
		dir = parent
	}
}

func getTestExecutor(t *testing.T) *Executor {
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	rootDir := findTestRepoRoot(cwd)
	executor, err := NewExecutor(rootDir)
	if err != nil {
		t.Fatalf("failed to create executor: %v", err)
	}
	return executor
}

// #endregion Helpers

// #region Collection Tests

func TestBundlesNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow bundle test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ bundles { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Bundles []struct {
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Bundles) == 0 {
		t.Error("bundles collection should not be empty")
	}
}

func TestContributorsNonEmpty(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow contributor test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ contributors { github } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Contributors []struct {
			Github string `json:"github"`
		} `json:"contributors"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Contributors) == 0 {
		t.Error("contributors collection should not be empty")
	}
}

func TestTicketsNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ tickets { slug } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Tickets []struct {
			Slug string `json:"slug"`
		} `json:"tickets"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets collection should not be empty")
	}
}

func TestPoliciesNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ policies { name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Policies []struct {
			Name string `json:"name"`
		} `json:"policies"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Policies) == 0 {
		t.Error("policies collection should not be empty")
	}
}

func TestViolationKindsNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ violationKinds { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		ViolationKinds []struct {
			ID string `json:"id"`
		} `json:"violationKinds"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.ViolationKinds) == 0 {
		t.Error("violationKinds collection should not be empty")
	}
}


func TestFoldersNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ folders { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Folders []struct {
			Path string `json:"path"`
		} `json:"folders"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Folders) == 0 {
		t.Error("folders collection should not be empty")
	}
}

func TestFilesNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ files { path } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Files []struct {
			Path string `json:"path"`
		} `json:"files"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Files) == 0 {
		t.Error("files collection should not be empty")
	}
}

func TestViolationsNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ violations { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Violations []struct {
			ID string `json:"id"`
		} `json:"violations"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Violations) == 0 {
		t.Error("violations collection should not be empty")
	}
}

func TestNodesAndEdgesQuick(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	query := `{
		tickets {
			id
			slug
		}
		policies {
			id
			name
			violationKinds { id }
		}
		violationKinds {
			id
		}
		folders {
			id
			path
			parent { id }
			children { id }
		}
		files {
			id
			path
			folder { id }
			sections { id name }
			definitions { id name kind }
		}
		violations {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Tickets []struct {
			ID          string `json:"id"`
			Slug        string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID             string `json:"id"`
			Name           string `json:"name"`
			ViolationKinds []struct{ ID string `json:"id"` } `json:"violationKinds"`
		} `json:"policies"`
		ViolationKinds []struct {
			ID string `json:"id"`
		} `json:"violationKinds"`
		Folders []struct {
			ID       string                            `json:"id"`
			Path     string                            `json:"path"`
			Parent   *struct{ ID string `json:"id"` } `json:"parent"`
			Children []struct{ ID string `json:"id"` } `json:"children"`
		} `json:"folders"`
		Files []struct {
			ID          string                            `json:"id"`
			Path        string                            `json:"path"`
			Folder      *struct{ ID string `json:"id"` } `json:"folder"`
			Sections    []struct{ ID string `json:"id"` } `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
		Violations []struct {
			ID     string                            `json:"id"`
			File   *struct{ ID string `json:"id"` } `json:"file"`
			Folder *struct{ ID string `json:"id"` } `json:"folder"`
		} `json:"violations"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.ViolationKinds) == 0 {
		t.Error("violationKinds should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
}

// #endregion Collection Tests

// #region Nodes and Edges Tests

func TestNodesAndEdges(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow nodes/edges test in short mode")
	}
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		bundles {
			id
			name
			folders { id path }
			files { id path }
			violations { id }
		}
		folders {
			id
			path
			parent { id }
			children { id }
			files { id }
			bundle { id }
			violations { id }
		}
		files {
			id
			path
			folder { id }
			bundle { id }
			sections { id name }
			definitions { id name kind }
			violations { id }
		}
		tickets {
			id
			slug
		}
		policies {
			id
			name
			violationKinds { id }
		}
		violationKinds {
			id
		}
		violations {
			id
			file { id }
			folder { id }
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Bundles []struct {
			ID           string `json:"id"`
			Name         string `json:"name"`
			Folders      []struct{ ID string `json:"id"` } `json:"folders"`
			Files        []struct{ ID string `json:"id"` } `json:"files"`
			Violations   []struct{ ID string `json:"id"` } `json:"violations"`
		} `json:"bundles"`
		Folders []struct {
			ID         string `json:"id"`
			Path       string `json:"path"`
			Parent     *struct{ ID string `json:"id"` } `json:"parent"`
			Children   []struct{ ID string `json:"id"` } `json:"children"`
			Files      []struct{ ID string `json:"id"` } `json:"files"`
			Bundle     *struct{ ID string `json:"id"` } `json:"bundle"`
			Violations []struct{ ID string `json:"id"` } `json:"violations"`
		} `json:"folders"`
		Files []struct {
			ID           string `json:"id"`
			Path         string `json:"path"`
			Folder       *struct{ ID string `json:"id"` } `json:"folder"`
			Bundle       *struct{ ID string `json:"id"` } `json:"bundle"`
			Sections     []struct{ ID string `json:"id"` } `json:"sections"`
			Definitions  []struct{ ID string `json:"id"`; Kind string `json:"kind"` } `json:"definitions"`
			Violations   []struct{ ID string `json:"id"` } `json:"violations"`
		} `json:"files"`
		Tickets []struct {
			ID          string `json:"id"`
			Slug        string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID             string `json:"id"`
			Name           string `json:"name"`
			ViolationKinds []struct{ ID string `json:"id"` } `json:"violationKinds"`
		} `json:"policies"`
		ViolationKinds []struct {
			ID         string `json:"id"`
		} `json:"violationKinds"`
		Violations []struct {
			ID     string `json:"id"`
			File   *struct{ ID string `json:"id"` } `json:"file"`
			Folder *struct{ ID string `json:"id"` } `json:"folder"`
		} `json:"violations"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	if len(resp.Bundles) == 0 {
		t.Error("bundles should not be empty")
	}
	if len(resp.Folders) == 0 {
		t.Error("folders should not be empty")
	}
	if len(resp.Files) == 0 {
		t.Error("files should not be empty")
	}
	if len(resp.Tickets) == 0 {
		t.Error("tickets should not be empty")
	}
	if len(resp.Policies) == 0 {
		t.Error("policies should not be empty")
	}
	if len(resp.ViolationKinds) == 0 {
		t.Error("violationKinds should not be empty")
	}
	if len(resp.Violations) == 0 {
		t.Error("violations should not be empty")
	}

	for _, bundle := range resp.Bundles {
		if bundle.ID == "" {
			t.Errorf("bundle %s has empty id", bundle.Name)
		}
	}
	for _, folder := range resp.Folders {
		if folder.ID == "" {
			t.Errorf("folder %s has empty id", folder.Path)
		}
	}
	for _, file := range resp.Files {
		if file.ID == "" {
			t.Errorf("file %s has empty id", file.Path)
		}
	}
	for _, ticket := range resp.Tickets {
		if ticket.ID == "" {
			t.Errorf("ticket %s has empty id", ticket.Slug)
		}
	}
	for _, policy := range resp.Policies {
		if policy.ID == "" {
			t.Errorf("policy %s has empty id", policy.Name)
		}
	}
	for _, vk := range resp.ViolationKinds {
		if vk.ID == "" {
			t.Error("violationKind has empty id")
		}
	}
	for _, v := range resp.Violations {
		if v.ID == "" {
			t.Error("violation has empty id")
		}
	}
}

func TestNodeQuery(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	bundleResult, err := executor.ExecuteJSON(ctx, "{ bundles { id name } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var bundleResp struct {
		Bundles []struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"bundles"`
	}
	if err := json.Unmarshal([]byte(bundleResult), &bundleResp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(bundleResp.Bundles) == 0 {
		t.Skip("no bundles to test node query")
	}

	testID := bundleResp.Bundles[0].ID
	nodeResult, err := executor.ExecuteJSON(ctx, `query($id: ID!) { node(id: $id) { ... on Bundle { id name } } }`, map[string]interface{}{"id": testID})
	if err != nil {
		t.Fatalf("node query failed: %v", err)
	}
	var nodeResp struct {
		Node struct {
			ID   string `json:"id"`
			Name string `json:"name"`
		} `json:"node"`
	}
	if err := json.Unmarshal([]byte(nodeResult), &nodeResp); err != nil {
		t.Fatalf("failed to parse node response: %v", err)
	}
	if nodeResp.Node.ID != testID {
		t.Errorf("node query returned wrong id: got %s, want %s", nodeResp.Node.ID, testID)
	}
	if nodeResp.Node.Name != bundleResp.Bundles[0].Name {
		t.Errorf("node query returned wrong name: got %s, want %s", nodeResp.Node.Name, bundleResp.Bundles[0].Name)
	}
}

func TestSectionsEdges(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		files {
			id
			path
			sections {
				id
				name
				path
				file { id }
				parent { id }
				children { id }
				definitions { id name }
				violations { id }
				range { start { line } end { line } }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID       string `json:"id"`
			Path     string `json:"path"`
			Sections []struct {
				ID          string `json:"id"`
				Name        string `json:"name"`
				Path        string `json:"path"`
				File        struct{ ID string `json:"id"` } `json:"file"`
				Parent      *struct{ ID string `json:"id"` } `json:"parent"`
				Children    []struct{ ID string `json:"id"` } `json:"children"`
				Definitions []struct{ ID string `json:"id"`; Name string `json:"name"` } `json:"definitions"`
				Violations  []struct{ ID string `json:"id"` } `json:"violations"`
				Range       struct {
					Start struct{ Line int `json:"line"` } `json:"start"`
					End   struct{ Line int `json:"line"` } `json:"end"`
				} `json:"range"`
			} `json:"sections"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	sectionsFound := false
	for _, file := range resp.Files {
		for _, section := range file.Sections {
			sectionsFound = true
			if section.ID == "" {
				t.Errorf("section %s in file %s has empty id", section.Name, file.Path)
			}
			if section.File.ID == "" {
				t.Errorf("section %s has file with empty id", section.Name)
			}
			if section.File.ID != file.ID {
				t.Errorf("section %s file id mismatch: got %s, want %s", section.Name, section.File.ID, file.ID)
			}
		}
	}
	if !sectionsFound {
		t.Skip("no sections found in any file - may be expected for test repository")
	}
}

func TestDefinitionsEdges(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	query := `{
		files {
			id
			path
			definitions {
				id
				name
				kind
				file { id }
				section { id name }
				violations { id }
				range { start { line } end { line } }
			}
		}
	}`

	result, err := executor.ExecuteJSON(ctx, query, nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	var resp struct {
		Files []struct {
			ID          string `json:"id"`
			Path        string `json:"path"`
			Definitions []struct {
				ID         string `json:"id"`
				Name       string `json:"name"`
				Kind       string `json:"kind"`
				File       struct{ ID string `json:"id"` } `json:"file"`
				Section    *struct{ ID string `json:"id"`; Name string `json:"name"` } `json:"section"`
				Violations []struct{ ID string `json:"id"` } `json:"violations"`
				Range      struct {
					Start struct{ Line int `json:"line"` } `json:"start"`
					End   struct{ Line int `json:"line"` } `json:"end"`
				} `json:"range"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.ID == "" {
				t.Errorf("definition %s in file %s has empty id", def.Name, file.Path)
			}
			if def.File.ID == "" {
				t.Errorf("definition %s has file with empty id", def.Name)
			}
			if def.File.ID != file.ID {
				t.Errorf("definition %s file id mismatch: got %s, want %s", def.Name, def.File.ID, file.ID)
			}
			if def.Kind == "" {
				t.Errorf("definition %s has empty kind", def.Name)
			}
		}
	}
	if !definitionsFound {
		t.Skip("no definitions found in any file - may be expected for test repository")
	}
}


// #endregion Nodes and Edges Tests

// #region Cli

// #region Helpers

func executeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	rootCmd.SetOut(buf)
	rootCmd.SetErr(buf)
	rootCmd.SetArgs(args)
	err := rootCmd.Execute()
	return buf.String(), err
}

func parseJSONOutput(output string) (map[string]interface{}, error) {
	var result map[string]interface{}
	err := json.Unmarshal([]byte(output), &result)
	return result, err
}

func hasExitCode(output string, code int) bool {
	parsed, err := parseJSONOutput(output)
	if err != nil {
		return false
	}
	outputMap, ok := parsed["output"].(map[string]interface{})
	if !ok {
		return false
	}
	exitCode, ok := outputMap["exitCode"].(float64)
	if !ok {
		return false
	}
	return int(exitCode) == code
}

// #endregion Helpers

// #region Codebase Tests

func TestCodebaseCommand(t *testing.T) {
	result := ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

// #endregion Codebase Tests

// #region Analyze Tests

func TestAnalyzeCommand(t *testing.T) {
	result := ToolAnalyze("@semio/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := ToolAnalyze("js/semio/semio.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

// #endregion Analyze Tests

// #region Fix Tests

func TestFixCommand(t *testing.T) {
	result := ToolFix("@semio/js")
	if result.Error != "" {
		t.Errorf("ToolFix returned error: %s", result.Error)
	}
}

// #endregion Fix Tests

// #region Policy Tests

func TestPolicyListCommand(t *testing.T) {
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]PolicyDef)
	if !ok {
		t.Error("ToolPolicyList data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyList returned no policies")
	}
	foundCode := false
	for _, p := range policies {
		if p.ID == "code" {
			foundCode = true
			break
		}
	}
	if !foundCode {
		t.Error("Expected to find 'code' policy")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "@semio/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestPolicyViolationListCommand(t *testing.T) {
	result := ToolPolicyViolationList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyViolationList returned error: %s", result.Error)
	}
}

// #endregion Policy Tests

// #region Bundle Tests

func TestBundleListCommand(t *testing.T) {
	result := ToolProjectList()
	if result.Error != "" {
		t.Errorf("ToolProjectList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolProjectList returned nil data")
	}
	bundles, ok := result.Data.([]Bundle)
	if !ok {
		t.Error("ToolProjectList data is not []Bundle")
		return
	}
	if len(bundles) == 0 {
		t.Error("ToolProjectList returned no bundles")
	}
	foundJS := false
	for _, b := range bundles {
		if b.Name == "@semio/js" {
			foundJS = true
			break
		}
	}
	if !foundJS {
		t.Error("Expected to find '@semio/js' bundle")
	}
}

func TestBundleTreeCommand(t *testing.T) {
	result := ToolProjectTree()
	if result.Error != "" {
		t.Errorf("ToolProjectTree returned error: %s", result.Error)
	}
}

// #endregion Bundle Tests

// #region Folder Tests

func TestFolderListCommand(t *testing.T) {
	result := ToolFolderList("go")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	result := ToolFolderTree("go")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestFolderCreateMoveDelete(t *testing.T) {
	testFolder := "temp/test-folder-cli"
	createResult := ToolFolderCreate(testFolder)
	if createResult.Error != "" {
		t.Errorf("ToolFolderCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFolderMove(testFolder, testFolder+"-moved")
	if moveResult.Error != "" {
		t.Errorf("ToolFolderMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFolderDelete(testFolder + "-moved")
	if deleteResult.Error != "" {
		t.Errorf("ToolFolderDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion Folder Tests

// #region File Tests

func TestFileListCommand(t *testing.T) {
	result := ToolFileList("@semio/js")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestFileTreeCommand(t *testing.T) {
	result := ToolFileTree("go")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestFileCreateMoveDelete(t *testing.T) {
	testFile := "temp/test-file-cli.txt"
	createResult := ToolFileCreate(testFile)
	if createResult.Error != "" {
		t.Errorf("ToolFileCreate returned error: %s", createResult.Error)
	}
	moveResult := ToolFileMove(testFile, "temp/test-file-cli-moved.txt")
	if moveResult.Error != "" {
		t.Errorf("ToolFileMove returned error: %s", moveResult.Error)
	}
	deleteResult := ToolFileDelete("temp/test-file-cli-moved.txt")
	if deleteResult.Error != "" {
		t.Errorf("ToolFileDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion File Tests

// #region Section Tests

func TestSectionListCommand(t *testing.T) {
	result := ToolSectionList("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolSectionList returned nil data")
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Error("ToolSectionList data is not []SectionInfo")
		return
	}
	if len(sections) == 0 {
		t.Error("ToolSectionList returned no sections")
	}
	foundHeader := false
	for _, s := range sections {
		if s.Name == "Header" {
			foundHeader = true
			break
		}
	}
	if !foundHeader {
		t.Error("Expected to find 'Header' section in js/semio/semio.ts")
	}
}

func TestSectionTreeCommand(t *testing.T) {
	result := ToolSectionTree("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// #endregion Section Tests

// #region Definition Tests

func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

// #endregion Definition Tests

// #region Ticket Tests

func TestTicketListCommand(t *testing.T) {
	year := 2025
	result := ToolTicketList(&year, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
}

// #endregion Ticket Tests

// #region Contributor Tests

func TestContributorListCommand(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

// #endregion Contributor Tests

// #region GraphQL Tests

func TestGraphQLRepoQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "semio") {
		t.Errorf("Expected result to contain 'semio', got: %s", result)
	}
}

func TestGraphQLBundlesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL bundles returned error: %v", err)
	}
	if !strings.Contains(result, "@semio/js") {
		t.Errorf("Expected result to contain '@semio/js', got: %s", result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestGraphQLTicketsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets(year: 2025) { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "ticket:") {
		t.Errorf("Expected result to contain 'ticket:', got: %s", result)
	}
}

func TestGraphQLAnalyzeQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ analyze { metrics { total } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL analyze returned error: %v", err)
	}
	if !strings.Contains(result, "total") {
		t.Errorf("Expected result to contain 'total', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

func TestGraphQLFixMutation(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `mutation { fix { fixed remaining } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL fix mutation returned error: %v", err)
	}
	if !strings.Contains(result, "fixed") {
		t.Errorf("Expected result to contain 'fixed', got: %s", result)
	}
}

// #endregion GraphQL Tests

// #endregion Cli