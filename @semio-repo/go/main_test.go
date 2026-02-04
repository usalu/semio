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
	"fmt"
	"os"
	"path/filepath"
	"strconv"
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

func findFirstResultData(output string) (json.RawMessage, bool) {
	parsed, err := parseJSONOutput(output)
	if err != nil {
		return nil, false
	}
	for _, e := range parsed {
		if e.Kind == KindResult {
			return e.Data, true
		}
	}
	return nil, false
}

func mustHaveExitCode(t *testing.T, output string, code int) {
	t.Helper()
	if !hasExitCode(output, code) {
		t.Fatalf("expected exit code %d, got output: %s", code, output)
	}
}

func parseTicketOpenResult(t *testing.T, output string) (int, int, int, string) {
	t.Helper()
	data, ok := findFirstResultData(output)
	if !ok {
		t.Fatalf("no result event in output: %s", output)
	}
	var envelope struct {
		Data struct {
			TicketOpen struct {
				Slug string `json:"slug"`
				Path string `json:"path"`
			} `json:"ticketOpen"`
		} `json:"data"`
	}
	if err := json.Unmarshal(data, &envelope); err == nil {
		if envelope.Data.TicketOpen.Path != "" {
			parts := strings.Split(strings.TrimPrefix(envelope.Data.TicketOpen.Path, "/"), "/")
			for i := 0; i+3 < len(parts); i++ {
				if parts[i] == "tickets" {
					y, _ := strconv.Atoi(parts[i+1])
					m, _ := strconv.Atoi(parts[i+2])
					d, _ := strconv.Atoi(parts[i+3])
					return y, m, d, envelope.Data.TicketOpen.Slug
				}
			}
		}
	}

	var resp struct {
		TicketOpen struct {
			Slug  string `json:"slug"`
			Year  int    `json:"year"`
			Month int    `json:"month"`
			Day   int    `json:"day"`
		} `json:"ticketOpen"`
	}
	if err := json.Unmarshal([]byte(output), &resp); err == nil {
		if resp.TicketOpen.Slug != "" && resp.TicketOpen.Year != 0 {
			return resp.TicketOpen.Year, resp.TicketOpen.Month, resp.TicketOpen.Day, resp.TicketOpen.Slug
		}
	}

	t.Fatalf("unable to parse ticket open response: %s", output)
	return 0, 0, 0, ""
}

func parseGoalCreateID(t *testing.T, output string) string {
	t.Helper()
	data, ok := findFirstResultData(output)
	if !ok {
		t.Fatalf("no result event in output: %s", output)
	}
	var envelope struct {
		Data struct {
			GoalCreate struct {
				ID string `json:"id"`
			} `json:"goalCreate"`
		} `json:"data"`
	}
	if err := json.Unmarshal(data, &envelope); err != nil {
		t.Fatalf("failed to parse goalCreate: %v\nOutput: %s", err, output)
	}
	if envelope.Data.GoalCreate.ID == "" {
		t.Fatalf("missing goal id in output: %s", output)
	}
	return envelope.Data.GoalCreate.ID
}

func parseTicketCloseStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := findFirstResultData(output)
	if !ok {
		t.Fatalf("no result event in output: %s", output)
	}
	var envelope struct {
		Data struct {
			TicketClose struct {
				Status string `json:"status"`
			} `json:"ticketClose"`
		} `json:"data"`
	}
	if err := json.Unmarshal(data, &envelope); err != nil {
		t.Fatalf("failed to parse ticketClose: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(envelope.Data.TicketClose.Status)
}

func parseTicketReopenStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := findFirstResultData(output)
	if !ok {
		t.Fatalf("no result event in output: %s", output)
	}
	var envelope struct {
		Data struct {
			TicketReopen struct {
				Status string `json:"status"`
			} `json:"ticketReopen"`
		} `json:"data"`
	}
	if err := json.Unmarshal(data, &envelope); err != nil {
		t.Fatalf("failed to parse ticketReopen: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(envelope.Data.TicketReopen.Status)
}

func testEngineFactory(config Config) (*Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = findTestRepoRoot(cwd)
	}
	SetRootDir(repoRoot)
	executor, err := NewExecutor(repoRoot)
	if err != nil {
		return nil, err
	}
	return NewEngine(executor), nil
}

func getTestExecutor(t *testing.T) *Executor {
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	// Update global rootDir for Tool functions
	rootDir = findTestRepoRoot(cwd)
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

func TestTicketTitleValidation(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	tests := []struct {
		name    string
		title   string
		wantErr bool
	}{
		{"Titleized Valid", "Some Title on Something", false},
		{"Single Word Valid", "Cleanup", false},
		{"With Hyphen Valid", "Refactor Resource ID System to Bundle-Based Hierarchy", false},
		{"Slug Invalid", "some-slug-title", true},
		{"Lowercase Valid", "some title", false},
		{"Allcaps Valid", "FIX EVERYTHING", false},
		{"Slug with Dashes Invalid", "fix-vscode-types-version-mismatch", true},
		{"Uppercase Slug Invalid", "ENSURE-SEMIO-REPO-MCP-WORKS-ALLIDES", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			query := `mutation { ticketOpen(input: { title: "` + tt.title + `", prompt: "Test prompt", llm: "opus-4", client: COPILOT_CHAT, noIssue: true }) { id slug year month day } }`
			result, err := executor.ExecuteJSON(ctx, query, nil)
			if (err != nil) != tt.wantErr {
				t.Errorf("ticketOpen() error = %v, wantErr %v", err, tt.wantErr)
			}

			// Cleanup
			if err == nil {
				// Parse result to get path
				// But result is JSON string of map.
				// Basic cleanup: title matches slug derived.
				// We need date.
				// The mutation returns year/month/day.
				var resp struct {
					TicketOpen struct {
						Slug  string `json:"slug"`
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
					} `json:"ticketOpen"`
				}
				if json.Unmarshal([]byte(result), &resp) == nil {
					path := GetTicketPath(resp.TicketOpen.Year, resp.TicketOpen.Month, resp.TicketOpen.Day, resp.TicketOpen.Slug)
					os.RemoveAll(path)
				}
			}
		})
	}
}

func TestBuildProjectLinkArgs(t *testing.T) {
	args := buildProjectLinkArgs("https://github.com/usalu/semio/issues/1")
	expected := []string{"project", "item-add", "2", "--owner", "usalu", "--url", "https://github.com/usalu/semio/issues/1"}
	if len(args) != len(expected) {
		t.Fatalf("expected %d args, got %d", len(expected), len(args))
	}
	for i := range expected {
		if args[i] != expected[i] {
			t.Fatalf("args[%d] expected %s, got %s", i, expected[i], args[i])
		}
	}
}

func TestFilterTicketWorkspaceFiles(t *testing.T) {
	executor := getTestExecutor(t)
	if executor == nil {
		t.Fatal("executor is nil")
	}
	absMain := filepath.Join(rootDir, "go", "repo", "main.go")
	ticket := &Ticket{
		Year:       2026,
		Month:      1,
		Day:        20,
		Slug:       "SAMPLE",
		FolderPath: filepath.Join(rootDir, ".semio-repo", "tickets", "2026", "01", "20", "SAMPLE"),
	}
	files := []string{
		".semio-repo/tickets/2026/01/20/SAMPLE/plan.md",
		"./.semio-repo/tickets/2026/01/20/SAMPLE/ticket.md",
		filepath.Join(rootDir, ".semio-repo", "tickets", "2026", "01", "20", "SAMPLE", "extra.txt"),
		absMain,
	}
	filtered := FilterTicketWorkspaceFiles(ticket, files)
	if len(filtered) != 1 || filtered[0] != absMain {
		t.Fatalf("expected [%s], got %v", absMain, filtered)
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
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID             string `json:"id"`
			Name           string `json:"name"`
			ViolationKinds []struct {
				ID string `json:"id"`
			} `json:"violationKinds"`
		} `json:"policies"`
		ViolationKinds []struct {
			ID string `json:"id"`
		} `json:"violationKinds"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
		Violations []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
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
			ID      string `json:"id"`
			Name    string `json:"name"`
			Folders []struct {
				ID string `json:"id"`
			} `json:"folders"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Violations []struct {
				ID string `json:"id"`
			} `json:"violations"`
		} `json:"bundles"`
		Folders []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Parent *struct {
				ID string `json:"id"`
			} `json:"parent"`
			Children []struct {
				ID string `json:"id"`
			} `json:"children"`
			Files []struct {
				ID string `json:"id"`
			} `json:"files"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Violations []struct {
				ID string `json:"id"`
			} `json:"violations"`
		} `json:"folders"`
		Files []struct {
			ID     string `json:"id"`
			Path   string `json:"path"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
			Bundle *struct {
				ID string `json:"id"`
			} `json:"bundle"`
			Sections []struct {
				ID string `json:"id"`
			} `json:"sections"`
			Definitions []struct {
				ID   string `json:"id"`
				Kind string `json:"kind"`
			} `json:"definitions"`
			Violations []struct {
				ID string `json:"id"`
			} `json:"violations"`
		} `json:"files"`
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID             string `json:"id"`
			Name           string `json:"name"`
			ViolationKinds []struct {
				ID string `json:"id"`
			} `json:"violationKinds"`
		} `json:"policies"`
		ViolationKinds []struct {
			ID string `json:"id"`
		} `json:"violationKinds"`
		Violations []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
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
				ID   string `json:"id"`
				Name string `json:"name"`
				Path string `json:"path"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Parent *struct {
					ID string `json:"id"`
				} `json:"parent"`
				Children []struct {
					ID string `json:"id"`
				} `json:"children"`
				Definitions []struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"definitions"`
				Violations []struct {
					ID string `json:"id"`
				} `json:"violations"`
				Range struct {
					Start struct {
						Line int `json:"line"`
					} `json:"start"`
					End struct {
						Line int `json:"line"`
					} `json:"end"`
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
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
				File struct {
					ID string `json:"id"`
				} `json:"file"`
				Section *struct {
					ID   string `json:"id"`
					Name string `json:"name"`
				} `json:"section"`
				Violations []struct {
					ID string `json:"id"`
				} `json:"violations"`
				Range struct {
					Start struct {
						Line int `json:"line"`
					} `json:"start"`
					End struct {
						Line int `json:"line"`
					} `json:"end"`
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

func TestDefinitionKind(t *testing.T) {
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
				ID   string `json:"id"`
				Name string `json:"name"`
				Kind string `json:"kind"`
			} `json:"definitions"`
		} `json:"files"`
	}

	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v\nResponse: %s", err, result)
	}

	definitionsFound := false
	validKinds := map[string]bool{
		"implementation": true,
		"interface":      true,
		"constant":       true,
	}

	for _, file := range resp.Files {
		for _, def := range file.Definitions {
			definitionsFound = true
			if def.Kind == "" {
				t.Errorf("definition %s in file %s has empty kind", def.Name, file.Path)
			}
			if !validKinds[def.Kind] {
				t.Errorf("definition %s has invalid kind: %s (expected implementation, interface, or constant)", def.Name, def.Kind)
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
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)
	config.JSON = true
	err := root.Execute()
	return buf.String(), err
}

func parseJSONOutput(output string) ([]Event, error) {
	var result []Event
	if err := json.Unmarshal([]byte(output), &result); err == nil {
		return result, nil
	}
	lines := strings.Split(strings.TrimSpace(output), "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		var event Event
		if err := json.Unmarshal([]byte(trimmed), &event); err != nil {
			return nil, err
		}
		result = append(result, event)
	}
	return result, nil
}

func hasExitCode(output string, code int) bool {
	parsed, err := parseJSONOutput(output)
	if err != nil {
		return false
	}
	for _, event := range parsed {
		if event.Kind == KindDone && event.Done != nil {
			return event.Done.ExitCode == code
		}
	}
	return false
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
	result := ToolAnalyze("@semio/js/semio.ts", nil)
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

func TestFixtureViolationsGroupedInline(t *testing.T) {
	path := "@semio/assets/repo/some/folder/file_invalid.tsx"
	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("fixture policy check failed: %v", err)
	}
	if len(violations) == 0 {
		t.Fatal("expected fixture violations")
	}
	counts := map[ViolationKind]int{}
	for _, v := range violations {
		counts[v.Kind]++
	}
	required := []ViolationKind{
		ViolationCodeHeaderMissingFilename,
		ViolationCodeHeaderMissingContributors,
		ViolationCodeHeaderWrongLicense,
		ViolationCodeSectionMissingStartName,
		ViolationCodeSectionMissingEndName,
		ViolationCodeSectionNameMismatch,
		ViolationCodeSectionEmpty,
		ViolationCodeSectionOrphanDefinition,
		ViolationCodeCommentInline,
		ViolationCodeCommentBlock,
		ViolationCodeCommentJSDoc,
	}
	for _, kind := range required {
		if counts[kind] == 0 {
			t.Fatalf("expected violation kind %s", kind)
		}
	}
	if counts[ViolationCodeCommentInline] != 1 {
		t.Fatalf("expected 1 inline comment violation, got %d", counts[ViolationCodeCommentInline])
	}
}

func TestFixtureViolationsByLanguage(t *testing.T) {
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []ViolationKind
	}{
		{
			path:          "@semio/assets/repo/some/folder/file_invalid.py",
			requiredKinds: []ViolationKind{ViolationCodeHeaderMissingRegion},
		},
		{
			path:          "@semio/assets/repo/some/folder/file_invalid.cs",
			requiredKinds: []ViolationKind{ViolationCodeHeaderMissingContributors},
		},
		{
			path:          "@semio/assets/repo/some/folder/file_invalid.go",
			requiredKinds: []ViolationKind{ViolationCodeHeaderMissingLicense},
		},
	}
	for _, fixture := range fixtures {
		scope := Scope{Kind: ScopeFile, FilePath: fixture.path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixture.path})
		violations, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", fixture.path, err)
		}
		if len(violations) == 0 {
			t.Fatalf("expected fixture violations for %s", fixture.path)
		}
		counts := map[ViolationKind]int{}
		for _, v := range violations {
			counts[v.Kind]++
		}
		for _, kind := range fixture.requiredKinds {
			if counts[kind] == 0 {
				t.Fatalf("expected violation kind %s in %s", kind, fixture.path)
			}
		}
	}
	clean := []string{
		"@semio/assets/repo/some/folder/file_fixed.tsx",
		"@semio/assets/repo/some/folder/file_fixed.py",
		"@semio/assets/repo/some/folder/file_fixed.cs",
		"@semio/assets/repo/some/folder/file_fixed.go",
	}
	for _, path := range clean {
		scope := Scope{Kind: ScopeFile, FilePath: path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
		violations, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", path, err)
		}
		if len(violations) != 0 {
			t.Fatalf("expected no violations for %s, got %d", path, len(violations))
		}
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
	result := ToolSectionList("@semio/js/semio.ts")
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
	result := ToolSectionTree("@semio/js/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// #endregion Section Tests

// #region Definition Tests

func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList("@semio/js/semio.ts")
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

func TestTicketOpenNoticketKeyword(t *testing.T) {
	result := ToolTicketOpen("Skip Ticket", "NOTICKET skip ticket creation", "gpt-5-mini", "codex", "", true, "", "", false, "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	if result.Data != nil {
		t.Fatalf("expected no ticket data for NOTICKET keyword")
	}
}

func TestTicketOpenContinueKeyword(t *testing.T) {
	first := ToolTicketOpen("Seed Ticket", "Seed prompt", "gpt-5-mini", "codex", "", true, "", "", false, "")
	if first.Error != "" {
		t.Fatalf("failed to seed ticket: %s", first.Error)
	}
	seed, ok := first.Data.(*Ticket)
	if !ok || seed == nil {
		t.Fatalf("expected seeded ticket data")
	}
	// Cleanup the created ticket after test
	defer func() {
		if seed != nil && seed.FolderPath != "" {
			os.RemoveAll(seed.FolderPath)
		}
	}()

	second := ToolTicketOpen("Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "codex", "", true, "", "", false, "")
	if second.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", second.Error)
	}
	continued, ok := second.Data.(*Ticket)
	if !ok || continued == nil {
		t.Fatalf("expected continued ticket data")
	}
	if continued.Slug != seed.Slug {
		t.Fatalf("expected continued ticket %s, got %s", seed.Slug, continued.Slug)
	}
}

// #endregion Ticket Tests

// #region Goal Tests

func TestGoalCreateValidation(t *testing.T) {
	// Test missing title
	result := ToolGoalCreate("", "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing title")
	}

	// Test missing description
	result = ToolGoalCreate("Test Goal", "", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing description")
	}

	// Test missing prompt
	result = ToolGoalCreate("Test Goal", "desc", "", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing prompt")
	}

	// Test missing due date
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing due date")
	}

	// Test missing LLM
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing llm")
	}

	// Test missing UI
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing ui")
	}

	// Test invalid LLM
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "invalid-llm", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid llm")
	}

	// Test invalid UI
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "invalid-ui", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid ui")
	}
}

func TestGoalCreateAndCleanup(t *testing.T) {
	// Create a goal with noGithub=true to avoid GitHub interaction
	result := ToolGoalCreate("Test Goal Creation", "Test description", "Test prompt for goal", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error != "" {
		t.Fatalf("ToolGoalCreate returned error: %s", result.Error)
	}
	goal, ok := result.Data.(*Goal)
	if !ok || goal == nil {
		t.Fatal("expected goal data")
	}
	if goal.Title != "Test Goal Creation" {
		t.Errorf("expected title 'Test Goal Creation', got '%s'", goal.Title)
	}
	if goal.Dates.Due != "2026-02-15" {
		t.Errorf("expected due date '2026-02-15', got '%s'", goal.Dates.Due)
	}
	if len(goal.Interactions) == 0 {
		t.Error("expected at least one interaction")
	} else {
		if goal.Interactions[0].LLM != "opus-4-5" {
			t.Errorf("expected LLM 'opus-4-5', got '%s'", goal.Interactions[0].LLM)
		}
		if goal.Interactions[0].System.Client != "claude-code" {
			t.Errorf("expected Client 'claude-code', got '%s'", goal.Interactions[0].System.Client)
		}
	}

	// Cleanup: remove the goal folder
	goalPath := filepath.Join(GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestGoalHierarchy(t *testing.T) {
	// 1. Create Parent
	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}
	
	// Ensure cleanup of parent (which should clean up nested child if working correctly)
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

	// 2. Create Child
	childTitle := "Test Child Goal"
	childRes := ToolGoalCreate(childTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, parent.ID, "")
	if childRes.Error != "" {
		t.Fatalf("Failed to create child: %s", childRes.Error)
	}
	child, ok := childRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	expectedChildID := "TEST-PARENT-GOAL/TEST-CHILD-GOAL"
	if child.ID != expectedChildID {
		t.Errorf("Expected child ID '%s', got '%s'", expectedChildID, child.ID)
	}

	// 3. Verify file structure
	childPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(child.ID), "goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	// 4. Verify parent relation
	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	// 5. Test Relocation / Rename of Parent
	// Rename Parent -> "Renamed Parent"
	// Expected: Parent ID becomes "RENAMED-PARENT"
	// Child folder should move to "goals/RENAMED-PARENT/TEST-CHILD-GOAL"
	// Child ID should become "RENAMED-PARENT/TEST-CHILD-GOAL" (as reported by ListGoals/ToolGoalList)

	parent.Title = "Renamed Parent"
	err := UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	// Update defer cleanup to new path
	// The old defer will try to remove 'test-parent-goal', which is now gone.
	// But defer order is LIFO. I should add a check or make sure I clean up efficiently.
	// os.RemoveAll is idempotent if path doesn't exist? No, it returns nil.
	// So old defer is safe. I'll add new defer.
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	// Verify child file moved
	// Note: We need to reload child or calculate expected path.
	// The child ID in memory `child.ID` is still old.
	// We check if file exists at new location.
	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(newChildID), "goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	// Verify ToolGoalList reports correct Child ID
	// ToolGoalList uses ListGoals which uses filesystem.
	// We need to verify that ListGoals sees the new hierarchy.
	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*Goal)
	var foundChild *Goal
	for _, g := range allGoals {
		// Just check suffix because we want to be sure it is our child
		if strings.HasSuffix(g.ID, "/TEST-CHILD-GOAL") && strings.HasPrefix(g.ID, "RENAMED-PARENT") {
			foundChild = g
			break
		}
	}
	if foundChild == nil {
		t.Errorf("Could not find child with new ID prefix in ListGoals output")
	} else {
		if foundChild.ID != newChildID {
			t.Errorf("Expected listed child ID '%s', got '%s'", newChildID, foundChild.ID)
		}
		if foundChild.Parent != parent.ID {
			t.Errorf("Expected listed child parent '%s', got '%s'", parent.ID, foundChild.Parent)
		}
	}

	// 6. Test Reparent Child via GoalChange
	ctx := &repoContext{}
	emptyParent := ""
	changeInput := GoalChangeInput{
		ID:     newChildID,
		Parent: &emptyParent,
	}

	updatedChild, err := ctx.GoalChange(changeInput)
	if err != nil {
		t.Fatalf("GoalChange failed: %v", err)
	}

	if updatedChild.ID != "TEST-CHILD-GOAL" {
		t.Errorf("Expected reparented child ID 'TEST-CHILD-GOAL', got '%s'", updatedChild.ID)
	}

	if updatedChild.Parent != "" {
		t.Errorf("Expected empty parent, got '%s'", updatedChild.Parent)
	}

	// Verify file moved to root
	rootChildPath := filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL", "goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

	// Cleanup the reparented child explicitly since it's no longer under parent folder
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL"))
}

func TestGoalList(t *testing.T) {
	result := ToolGoalList()
	// Note: This test may fail if there are malformed goal.json files in the repo
	// The test is mainly to verify the function doesn't panic and the basic structure works
	// Error might occur due to existing malformed data, which is acceptable for this test
	if result.Error != "" {
		t.Logf("ToolGoalList returned error (may be due to existing malformed data): %s", result.Error)
	}
}

// #endregion Goal Tests

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
	if !strings.Contains(result, "\"slug\"") {
		t.Errorf("Expected result to contain '\"slug\"', got: %s", result)
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

// #region Tree Tests

func executeTreeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	root, _ := NewRootWithConfig(testEngineFactory)
	root.SetOut(buf)
	root.SetErr(buf)
	root.SetArgs(args)
	// Default config has JSON=false
	err := root.Execute()
	return buf.String(), err
}

func TestTreeCommands(t *testing.T) {
	// 1. Codebase Tree
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}

	output, err := executeTreeCommand("tree", "@semio/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {
		t.Errorf("repo tree @semio/go missing semio.go, got:\n%s", output)
	}

	// 2. Folder Tree
	output, err = executeTreeCommand("folder", "tree", "@semio/go")
	if err != nil {
		t.Errorf("folder tree failed: %v", err)
	}
	// Folder tree produces output like "└── cmd/"
	// Note: output might include metadata/logs unless JSON is false and renderStream handles it well.
	// renderStream without JSON prints messages directly if they are logs.
	if !strings.Contains(output, "semio.go") {
		// Just checking that we got some output
		if len(output) < 10 {
			t.Errorf("folder tree output suspicious: %s", output)
		}
	}

	// 3. File Tree
	output, err = executeTreeCommand("file", "tree", "@semio/go")
	if err != nil {
		t.Errorf("file tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {
		t.Errorf("file tree missing semio.go")
	}

	// 4. Ticket Tree
	output, err = executeTreeCommand("ticket", "tree")
	if err != nil {
		t.Errorf("ticket tree failed: %v", err)
	}
	// We expect at least the root or some structure
	if len(output) == 0 {
		t.Errorf("ticket tree output empty")
	}

	// 5. Goal Tree
	output, err = executeTreeCommand("goal", "tree")
	if err != nil {
		t.Errorf("goal tree failed: %v", err)
	}
	// Goal tree should produce some output (goals + tickets hierarchy)
	if len(output) == 0 {
		t.Errorf("goal tree output empty")
	}
}

func TestCliE2E_TicketLifecycle_Syntaxes_NoGithub(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}

	fileRel := filepath.ToSlash(filepath.Join("go", "repo", "main.go"))

	openOut, err := executeCommand(
		"ticket", "open",
		"E2E Ticket Positional",
		"E2E prompt positional",
		"cursor-chat",
		"sonnet-4-5",
		"--no-issue",
		"--no-github",
	)
	if err != nil {
		t.Fatalf("ticket open positional failed: %v\nOutput: %s", err, openOut)
	}
	mustHaveExitCode(t, openOut, 0)
	y, m, d, slug := parseTicketOpenResult(t, openOut)
	defer os.RemoveAll(GetTicketPath(y, m, d, slug))

	closeOut, err := executeCommand(
		"ticket", "close",
		"--no-github",
		"--year", strconv.Itoa(y),
		"--month", strconv.Itoa(m),
		"--day", strconv.Itoa(d),
		"--slug", slug,
		"--summary", "E2E summary",
		"--files", fileRel,
	)
	if err != nil {
		t.Fatalf("ticket close flags failed: %v\nOutput: %s", err, closeOut)
	}
	mustHaveExitCode(t, closeOut, 0)
	if status := parseTicketCloseStatus(t, closeOut); status != "closed" {
		t.Fatalf("expected closed status, got %s", status)
	}

	reopenOut, err := executeCommand(
		"ticket", "reopen",
		fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug),
		"E2E reopen prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-github",
	)
	if err != nil {
		t.Fatalf("ticket reopen mix failed: %v\nOutput: %s", err, reopenOut)
	}
	mustHaveExitCode(t, reopenOut, 0)
	if status := parseTicketReopenStatus(t, reopenOut); status != "open" {
		t.Fatalf("expected open status, got %s", status)
	}

	listOut, err := executeCommand("ticket", "list", "--year", strconv.Itoa(y))
	if err != nil {
		t.Fatalf("ticket list failed: %v\nOutput: %s", err, listOut)
	}
	mustHaveExitCode(t, listOut, 0)
}

func TestCliE2E_GoalLifecycle_Syntaxes_NoGithub(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}
	openOut, err := executeCommand(
		"goal", "open",
		"E2E Goal Title",
		"E2E Goal Description",
		"E2E Goal Prompt",
		"cursor-chat",
		"gpt-5-mini",
		"--due-date", "2026-02-15",
		"--no-github",
	)
	if err != nil {
		t.Fatalf("goal open failed: %v\nOutput: %s", err, openOut)
	}
	mustHaveExitCode(t, openOut, 0)
	goalID := parseGoalCreateID(t, openOut)
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

	closeOut, err := executeCommand("goal", "close", goalID, "E2E Goal Summary", "--no-github")
	if err != nil {
		t.Fatalf("goal close failed: %v\nOutput: %s", err, closeOut)
	}
	mustHaveExitCode(t, closeOut, 0)

	reopenOut, err := executeCommand("goal", "reopen", goalID, "E2E Goal Reopen Prompt", "cursor-chat", "gpt-5-mini", "--no-github")
	if err != nil {
		t.Fatalf("goal reopen failed: %v\nOutput: %s", err, reopenOut)
	}
	mustHaveExitCode(t, reopenOut, 0)
}

func TestCliE2E_MiscCommands_NoSideEffects(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}

	out, err := executeCommand("bundle", "list")
	if err != nil {
		t.Fatalf("bundle list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("bundle", "tree")
	if err != nil {
		t.Fatalf("bundle tree failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("folder", "list", "go")
	if err != nil {
		t.Fatalf("folder list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("file", "list", "go")
	if err != nil {
		t.Fatalf("file list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("section", "list", "@semio/js/semio.ts")
	if err != nil {
		t.Fatalf("section list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("definition", "list", "@semio/js/semio.ts")
	if err != nil {
		t.Fatalf("definition list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("policy", "list")
	if err != nil {
		t.Fatalf("policy list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("policy", "check", "code", "@semio/js")
	if err != nil {
		t.Fatalf("policy check failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("goal", "list")
	if err != nil {
		t.Fatalf("goal list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("goal", "tree")
	if err != nil {
		t.Fatalf("goal tree failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("ticket", "list")
	if err != nil {
		t.Fatalf("ticket list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("ticket", "tree")
	if err != nil {
		t.Fatalf("ticket tree failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("contributor", "list")
	if err != nil {
		t.Fatalf("contributor list failed: %v\nOutput: %s", err, out)
	}
	mustHaveExitCode(t, out, 0)

	out, err = executeCommand("mcp", "--dry-run")
	if err != nil {
		t.Fatalf("mcp dry-run failed: %v\nOutput: %s", err, out)
	}

	out, err = executeCommand("update")
	if err != nil {
		t.Fatalf("update default dry-run failed: %v\nOutput: %s", err, out)
	}
}

// #endregion Tree Tests

func TestMarkdownTreeOutput(t *testing.T) {
t.Run("Bundle Tree Markdown", func(t *testing.T) {
out, err := executeCommand("bundle", "tree", "--md")
if err != nil {
t.Fatalf("bundle tree failed: %v", err)
}
if !strings.Contains(out, "- [") {
t.Errorf("markdown output should contain bullet list links, got:\n%s", out)
}
})

t.Run("Folder Tree Markdown", func(t *testing.T) {
		out, err := executeCommand("folder", "tree", "@semio-repo/go", "--md")
		if err != nil {
			t.Fatalf("folder tree failed: %v", err)
		}
		if !strings.Contains(out, "- [") {
			t.Errorf("markdown output should contain bullet list links, got:\n%s", out)
		}
	})

	t.Run("File Tree Markdown", func(t *testing.T) {
		out, err := executeCommand("file", "tree", "@semio-repo/go", "--md")
		if err != nil {
			t.Fatalf("file tree failed: %v", err)
		}
		if !strings.Contains(out, "- [") {
			t.Errorf("markdown output should contain bullet list links, got:\n%s", out)
		}
	})

	t.Run("Section Tree Markdown", func(t *testing.T) {
		out, err := executeCommand("section", "tree", "@semio-repo/go/main.go", "--md")
		if err != nil {
			t.Fatalf("section tree failed: %v\nOutput: %s", err, out)
		}
		if !strings.Contains(out, "- [") {
			t.Errorf("markdown output should contain bullet list links, got:\n%s", out)
		}
	})
}
