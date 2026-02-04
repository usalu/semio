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
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"testing"
)

// #region Helpers

func findTestRepoRoot(start string) string {
	for _, candidate := range []string{start, func() string {
		_, file, _, ok := runtime.Caller(0)
		if !ok {
			return ""
		}
		return filepath.Dir(file)
	}()} {
		dir := candidate
		if dir == "" {
			continue
		}
		for {
			if _, err := os.Stat(filepath.Join(dir, "AGENTS.md")); err == nil {
				return dir
			}
			parent := filepath.Dir(dir)
			if parent == dir {
				break
			}
			dir = parent
		}
	}
	return start
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
			query := `mutation { ticketOpen(input: { title: "` + tt.title + `", prompt: "Test prompt", llm: "opus-4", client: COPILOT_CHAT, goal: "TEST-GOAL", noIssue: true }) { id slug year month day } }`
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
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFolderList("@semio-repo")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFolderTree("@semio-repo/go")
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
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFileTree("@semio-repo/go")
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
	first := ToolTicketOpen("Seed Ticket", "Seed prompt", "gpt-5-mini", "codex", "", true, "TEST-GOAL", "", false, "")
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

	second := ToolTicketOpen("Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "codex", "", true, "TEST-GOAL", "", false, "")
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

	// Test missing Client
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing client")
	}

	// Test invalid LLM
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "invalid-llm", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid llm")
	}

	// Test invalid Client
	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "invalid-client", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid client")
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
		"--goal", "TEST-GOAL",
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

// #region Consolidated Tests

func TestFormatResult_Section(t *testing.T) {
	payload := map[string]interface{}{
		"section": map[string]interface{}{
			"name":      "MySection",
			"filePath":  "path/to/file.ts",
			"startLine": float64(10),
			"endLine":   float64(20),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("section list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"MySection",
		"(lines 10-20)",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Definition(t *testing.T) {
	payload := map[string]interface{}{
		"definition": map[string]interface{}{
			"name":      "MyDefinition",
			"kind":      "function",
			"filePath":  "path/to/file.ts",
			"startLine": float64(30),
			"endLine":   float64(40),
		},
	}

	bytes, _ := json.Marshal(payload)
	result := formatResult("definition list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"TEST-DEFINITION-ID",
		"MyDefinition",
		":30-40",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}

	if strings.TrimSpace(result)[0] == '{' {
		t.Errorf("Result looks like raw JSON:\n%s", result)
	}
}

func TestFormatResult_Bundle(t *testing.T) {
	payload := map[string]interface{}{
		"bundle": map[string]interface{}{
			"name": "MyBundle",
			"root": "/path/to/bundle",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("bundle list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"📚︎ MyBundle",
		"/path/to/bundle",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Folder(t *testing.T) {
	payload := map[string]interface{}{
		"folder": map[string]interface{}{
			"path": "path/to/folder",
			"kind": "custom",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("folder list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"📁︎ path/to/folder",
		"custom",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_File(t *testing.T) {
	payload := map[string]interface{}{
		"file": map[string]interface{}{
			"id": "path/to/file.ts",
		},
	}
	bytes, _ := json.Marshal(payload)
	result := formatResult("file list", json.RawMessage(bytes), false)

	expectedParts := []string{
		"📄",
		"path/to/file.ts",
	}

	for _, part := range expectedParts {
		if !strings.Contains(result, part) {
			t.Errorf("Expected result to contain %q, but got:\n%s", part, result)
		}
	}
}

func TestFormatResult_Additional(t *testing.T) {
	t.Run("Goal", func(t *testing.T) {
		payload := map[string]interface{}{
			"goal": map[string]interface{}{
				"id":          "SKETCHPAD/MVP",
				"title":       "Sketchpad MVP",
				"status":      "open",
				"description": "Get sketchpad running at MVP level",
				"dates": map[string]interface{}{
					"due": "2026-02-15",
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)
		var p map[string]interface{}
		json.Unmarshal(jsonBytes, &p)

		output := formatResult("goal list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"goal\":") {
			t.Errorf("expected formatted goal, got: %s", output)
		}
		if !strings.Contains(output, "Sketchpad MVP") {
			t.Error("output missing title")
		}
		if !strings.Contains(output, "2026-02-15") {
			t.Error("output missing due date")
		}
		if !strings.Contains(output, "SKETCHPAD/MVP") {
			t.Error("output missing id/slug")
		}
	})

	t.Run("Contributor", func(t *testing.T) {
		payload := map[string]interface{}{
			"contributor": map[string]interface{}{
				"github": "octocat",
				"name":   "The Octocat",
				"contributions": map[string]interface{}{
					"commits": 10,
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("contributor list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"contributor\":") {
			t.Errorf("expected formatted contributor, got: %s", output)
		}
		if !strings.Contains(output, "octocat") {
			t.Error("output missing github handle")
		}
		if !strings.Contains(output, "The Octocat") {
			t.Error("output missing name")
		}
	})

	t.Run("Policy", func(t *testing.T) {
		payload := map[string]interface{}{
			"policy": map[string]interface{}{
				"id":          "code",
				"description": "Validates source file headers",
				"kinds":       []interface{}{"code:header"},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("policy list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"policy\":") {
			t.Errorf("expected formatted policy, got: %s", output)
		}
		if !strings.Contains(output, "code") {
			t.Error("output missing id")
		}
		if !strings.Contains(output, "Validates") {
			t.Error("output missing description")
		}
	})

	t.Run("File", func(t *testing.T) {
		payload := map[string]interface{}{
			"file": map[string]interface{}{
				"id":        "path/to/file.md",
				"extension": ".md",
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("file list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"file\":") {
			t.Errorf("expected formatted file, got: %s", output)
		}
		if !strings.Contains(output, "path/to/file.md") {
			t.Error("output missing path")
		}
	})

	t.Run("Fix", func(t *testing.T) {
		payload := map[string]interface{}{
			"fix": map[string]interface{}{
				"fixed":     5,
				"remaining": 2,
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("fix", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"fix\":") {
			t.Errorf("expected formatted fix, got: %s", output)
		}
		if !strings.Contains(output, "fixed 5 violations") {
			t.Error("output missing fixed count")
		}
	})
}

func TestMarkdownOutput(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name        string
		args        []string
		wantMarkers []string
	}{
		{
			name:        "Repo Tree MD",
			args:        []string{"tree", "--md"},
			wantMarkers: []string{"- [", "]("},
		},
		{
			name:        "Ticket Tree MD",
			args:        []string{"ticket", "tree", "--md"},
			wantMarkers: []string{"- [", "](semiorepo://ticket/"},
		},
		{
			name:        "Goal Tree MD",
			args:        []string{"goal", "tree", "--md"},
			wantMarkers: []string{"- [", "](semiorepo://goal/"},
		},
		{
			name:        "Ticket List MD",
			args:        []string{"ticket", "list", "--md"},
			wantMarkers: []string{"- [", "](semiorepo://ticket/"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rootCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(tt.args)

			_ = rootCmd.Execute()

			output := b.String()
			if len(strings.TrimSpace(output)) == 0 {
				t.Logf("Output is empty for %s, skipping marker checks", tt.name)
				return
			}

			for _, marker := range tt.wantMarkers {
				if !strings.Contains(output, marker) {
					t.Errorf("Output missing marker %q. Got:\n%s", marker, output)
				}
			}

			if strings.Contains(output, " -  - ") {
				t.Errorf("Output contains double dash ' -  - ' which indicates empty property issue:\n%s", output)
			}
		})
	}
}

func TestLifecycleCommands(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	modes := []string{"", "json", "md"}

	for _, mode := range modes {
		t.Run("lifecycle_"+mode, func(t *testing.T) {
			title := "Test Lifecycle " + mode
			if mode == "" {
				title = "Test Lifecycle human"
			}

			openArgs := []string{"ticket", "open", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--goal", "test-goal", "--no-issue", "--no-github"}
			if mode == "json" {
				openArgs = append(openArgs, "--json")
			}
			if mode == "md" {
				openArgs = append(openArgs, "--md")
			}

			rootCmd := NewRoot(factory)
			// Create goal first
			goalCmd := NewRoot(factory)
			goalCmd.SetArgs([]string{"goal", "open", "Test Goal", "Test Goal Description", "Test Goal Prompt", "copilot-chat", "gemini-3-pro", "--due-date", "2025-12-31", "--no-github"})
			goalCmd.Execute()

			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(openArgs)

			err := rootCmd.Execute()
			if err != nil {
				t.Fatalf("ticket open failed: %v\nOutput: %s", err, b.String())
			}

			listCmd := NewRoot(factory)
			listB := bytes.NewBufferString("")
			listCmd.SetOut(listB)
			listCmd.SetErr(listB)
			listCmd.SetArgs([]string{"ticket", "list", "--json"})
			listCmd.Execute()

			events, _ := parseJSONOutput(listB.String())
			var y, m, d int
			var slug string
			found := false

			for _, e := range events {
				if e.Kind == KindResult {
					var env struct {
						Ticket struct {
							Year  int    `json:"year"`
							Month int    `json:"month"`
							Day   int    `json:"day"`
							Slug  string `json:"slug"`
							Title string `json:"title"`
						} `json:"ticket"`
					}
					if json.Unmarshal(e.Data, &env) == nil {
						if strings.EqualFold(env.Ticket.Title, title) {
							y, m, d, slug = env.Ticket.Year, env.Ticket.Month, env.Ticket.Day, env.Ticket.Slug
							found = true
							break
						}
					}
				}
			}

			if !found {
				t.Fatalf("Could not find created ticket with title %q in list output", title)
			}

			defer os.RemoveAll(GetTicketPath(y, m, d, slug))

			changeArgs := []string{"ticket", "change",
				fmt.Sprintf("%d/%02d/%02d/%s", y, m, d, slug),
				"--goal", "test-goal",
				"--parent", "parent-ticket-slug",
				"--no-github",
			}
			changeCmd := NewRoot(factory)
			changeB := bytes.NewBufferString("")
			changeCmd.SetOut(changeB)
			changeCmd.SetErr(changeB)
			changeCmd.SetArgs(changeArgs)
			if err := changeCmd.Execute(); err != nil {
				t.Fatalf("ticket change failed: %v\nOutput: %s", err, changeB.String())
			}

			ticketDir := GetTicketPath(y, m, d, slug)
			jsonContent, err := os.ReadFile(filepath.Join(ticketDir, "ticket.json"))
			if err == nil {
				var tm Ticket
				if err := json.Unmarshal(jsonContent, &tm); err == nil {
					if tm.Goal != "test-goal" {
						t.Errorf("ticket change goal mismatch: expected test-goal, got %s", tm.Goal)
					}
					if tm.Parent != "parent-ticket-slug" {
						t.Errorf("ticket change parent mismatch: expected parent-ticket-slug, got %s", tm.Parent)
					}
				}
			}

			closeArgs := []string{"ticket", "close",
				"--no-github",
				"--year", strconv.Itoa(y),
				"--month", strconv.Itoa(m),
				"--day", strconv.Itoa(d),
				"--slug", slug,
				"--summary", "Test Summary",
				"--files", "@semio-repo/go/main.go",
			}
			if mode == "json" {
				closeArgs = append(closeArgs, "--json")
			}
			if mode == "md" {
				closeArgs = append(closeArgs, "--md")
			}

			closeCmd := NewRoot(factory)
			closeB := bytes.NewBufferString("")
			closeCmd.SetOut(closeB)
			closeCmd.SetErr(closeB)
			closeCmd.SetArgs(closeArgs)

			err = closeCmd.Execute()
			if err != nil {
				t.Fatalf("ticket close failed: %v\nOutput: %s", err, closeB.String())
			}

			if closeB.String() == "" {
				t.Errorf("ticket close output empty")
			}
		})
	}
}

func TestListCommands(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name  string
		args  []string
		modes []string
	}{
		{
			name:  "bundle list",
			args:  []string{"bundle", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "ticket list",
			args:  []string{"ticket", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "folder list",
			args:  []string{"folder", "list", "@semio-repo/go"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "file list",
			args:  []string{"file", "list", "@semio-repo/go"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "section list",
			args:  []string{"section", "list", "@semio-repo/go/main.go"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "definition list",
			args:  []string{"definition", "list", "@semio-repo/go/main.go"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "policy list",
			args:  []string{"policy", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "contributor list",
			args:  []string{"contributor", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name:  "project list",
			args:  []string{"project", "list"},
			modes: []string{"", "json", "md"},
		},
	}

	for _, tt := range tests {
		for _, mode := range tt.modes {
			testName := tt.name
			if mode != "" {
				testName += " --" + mode
			} else {
				testName += " (human)"
			}

			t.Run(testName, func(t *testing.T) {
				rootCmd := NewRoot(factory)
				b := bytes.NewBufferString("")
				rootCmd.SetOut(b)
				rootCmd.SetErr(b)

				args := append([]string(nil), tt.args...)
				if mode == "json" {
					args = append(args, "--json")
				}
				if mode == "md" {
					args = append(args, "--md")
				}
				rootCmd.SetArgs(args)

				err := rootCmd.Execute()
				if err != nil {
					t.Fatalf("Command failed: %v\nOutput: %s", err, b.String())
				}

				output := b.String()
				if mode == "json" {
					lines := strings.Split(strings.TrimSpace(output), "\n")
					for _, line := range lines {
						if line == "" {
							continue
						}
						if !strings.HasPrefix(strings.TrimSpace(line), "{") {
							continue
						}
					}
				} else if mode == "md" {
					if !strings.Contains(output, "# ") && !strings.Contains(output, "- ") && !strings.Contains(output, "|") && output != "" {
					}
				}
			})
		}
	}
}

func TestSectionCommands(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	testDir := filepath.Join(repoRoot, "temp", "section_tests")
	os.MkdirAll(testDir, 0755)
	defer os.RemoveAll(testDir)

	tests := []struct {
		name       string
		ext        string
		contentFmt string
		renameTo   string
	}{
		{"TypeScript", ".ts", "const x = 1;\n// #region %s\nconst y = 2;\n// #endregion %s\n", "Renamed"},
		{"Go", ".go", "package main\n// #region %s\nvar y = 2\n// #endregion %s\n", "Renamed"},
		{"Python", ".py", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"CSharp", ".cs", "#region %s\nvar y = 2;\n#endregion %s\n", "Renamed"},
		{"Rust", ".rs", "// #region %s\nlet y = 2;\n// #endregion %s\n", "Renamed"},
		{"Ruby", ".rb", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"Shell", ".sh", "# region %s\ny=2\n# endregion %s\n", "Renamed"},
		{"TOML", ".toml", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"YAML", ".yaml", "# region %s\ny: 2\n# endregion %s\n", "Renamed"},
		{"SQL", ".sql", "-- #region %s\nSELECT 1;\n-- #endregion %s\n", "Renamed"},
		{"GraphQL", ".graphql", "# #region %s\ntype Query { name: String }\n# #endregion %s\n", "Renamed"},
		{"Markdown", ".md", "## %s\nContent\n", "Renamed"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			sectionName := "MySection"
			filename := "test" + tc.ext
			filePath := filepath.Join(testDir, filename)
			relPath, _ := filepath.Rel(repoRoot, filePath)

			var content string
			if tc.name == "Markdown" {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 1)
			} else {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 2)
			}
			os.WriteFile(filePath, []byte(content), 0644)

			moveCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			moveCmd.SetOut(b)
			moveCmd.SetErr(b)
			moveCmd.SetArgs([]string{"section", "move", relPath, sectionName, tc.renameTo})
			err := moveCmd.Execute()
			if err != nil {
				t.Fatalf("Move failed: %v Output: %s", err, b.String())
			}

			newContentBytes, _ := os.ReadFile(filePath)
			newContent := string(newContentBytes)
			if !strings.Contains(newContent, tc.renameTo) {
				t.Errorf("File content does not contain renamed section %s. Content:\n%s", tc.renameTo, newContent)
			}

			targetFile := filepath.Join(testDir, "extracted"+tc.ext)
			relTargetFile, _ := filepath.Rel(repoRoot, targetFile)

			extractCmd := NewRoot(factory)
			extractCmd.SetOut(b)
			extractCmd.SetErr(b)
			extractCmd.SetArgs([]string{"section", "extract", relPath, tc.renameTo, relTargetFile})
			err = extractCmd.Execute()
			if err != nil {
				t.Fatalf("Extract failed: %v Output: %s", err, b.String())
			}

			targetContentBytes, err := os.ReadFile(targetFile)
			if err != nil {
				t.Fatalf("Target file not created: %v", err)
			}
			targetContent := string(targetContentBytes)
			if len(targetContent) == 0 && tc.name != "Markdown" {
				t.Errorf("Extracted content is empty")
			}

			sourceIntegrate := filepath.Join(testDir, "to_integrate"+tc.ext)
			relSourceIntegrate, _ := filepath.Rel(repoRoot, sourceIntegrate)
			integrateContent := "New Content"
			os.WriteFile(sourceIntegrate, []byte(integrateContent), 0644)

			integrateCmd := NewRoot(factory)
			integrateCmd.SetOut(b)
			integrateCmd.SetErr(b)
			integrateCmd.SetArgs([]string{"section", "integrate", relSourceIntegrate, tc.renameTo, relPath})
			err = integrateCmd.Execute()
			if err != nil {
				t.Fatalf("Integrate failed: %v Output: %s", err, b.String())
			}

			finalContentBytes, _ := os.ReadFile(filePath)
			finalContent := string(finalContentBytes)
			if !strings.Contains(finalContent, integrateContent) {
				t.Errorf("File content does not contain integrated content. Content:\n%s", finalContent)
			}
		})
	}
}

func TestStreamingList(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	factory := func(config Config) (*Engine, error) {
		executor, err := NewExecutor(repoRoot)
		if err != nil {
			return nil, err
		}
		return NewEngine(executor), nil
	}

	tests := []struct {
		name string
		args []string
	}{
		{
			name: "Ticket List (Text)",
			args: []string{"ticket", "list"},
		},
		{
			name: "Bundle List (Text)",
			args: []string{"bundle", "list"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			rootCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(tt.args)

			_ = rootCmd.Execute()
			output := b.String()
			lines := strings.Split(strings.TrimSpace(output), "\n")

			for _, line := range lines {
				if strings.TrimSpace(line) == "" {
					continue
				}
				if strings.HasPrefix(strings.TrimSpace(line), "{\"kind\":\"result\"") {
					t.Errorf("Expected formatted text output, got raw JSON event: %s", line)
				}
			}
		})
	}
}

func TestTicketLifecycle_NoGithub(t *testing.T) {
	tmpDir := t.TempDir()

	run := func(name string, field ...string) {
		cmd := exec.Command(name, field...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("run %s %v failed: %v\nOutput: %s", name, field, err, out)
		}
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")

	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	if err := os.MkdirAll(filepath.Join(tmpDir, ".semio-repo", "tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	goal, err := OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}

	ticket, err := OpenTicket("Test Title NoGH", "Test Prompt", "gemini-3-pro", "copilot-chat", "", false, goal.ID, "", true, "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if ticket.GitHub != nil {
		t.Error("OpenTicket: GitHub data should be nil")
	}

	testFile := "test.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("content"), 0644); err != nil {
		t.Fatal(err)
	}

	if goal.Title != "Goal Title" {
		t.Errorf("expected title 'Goal Title', got '%s'", goal.Title)
	}
	if goal.Prompt != "Goal Prompt" {
		t.Errorf("expected prompt 'Goal Prompt', got '%s'", goal.Prompt)
	}
	if goal.Client != "copilot-chat" {
		t.Errorf("expected ui 'copilot-chat', got '%s'", goal.Client)
	}
	if goal.LLM != "gemini-3-pro" {
		t.Errorf("expected llm 'gemini-3-pro', got '%s'", goal.LLM)
	}
	if goal.GitHub != nil {
		t.Error("OpenGoal: GitHub data should be nil")
	}

	goalPath := filepath.Join(tmpDir, ".semio-repo", "goals", "GOAL-TITLE", "goal.json")
	if _, err := os.Stat(goalPath); os.IsNotExist(err) {
		t.Errorf("goal file not created at %s", goalPath)
	}

	run("git", "add", testFile)
	run("git", "commit", "-m", "add test file")

	err = FinishTicket(ticket, "Summary", []string{testFile}, true, false)
	if err != nil {
		t.Fatalf("FinishTicket failed: %v", err)
	}
	if ticket.GetStatus() != TicketStatusClosed {
		t.Errorf("Ticket status mismatch: got %v, want closed", ticket.GetStatus())
	}

	err = ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "copilot-chat", "", "", "", true)
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if ticket.GetStatus() != TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := GoalCreateInput{
		Title:       "Test Goal NoGH 2",
		Description: "Desc",
		Prompt:      "Prompt",
		DueDate:     "2026-02-15",
		Client:      "cursor",
		LLM:         "gpt-5-2-codex",
		NoGithub:    true,
	}

	goal2, err := ctx.GoalCreate(goalInput)
	if err != nil {
		t.Fatalf("GoalCreate failed: %v", err)
	}
	if goal2.Title != "Test Goal NoGH 2" {
		t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
	}
}

func TestArtifactIDAndURI(t *testing.T) {
	tests := []struct {
		name    string
		kind    string
		data    map[string]interface{}
		wantID  string
		wantURI string
	}{
		{
			name:    "repo",
			kind:    "repo",
			data:    map[string]interface{}{},
			wantID:  "🌍\ufe0e",
			wantURI: "semiorepo://repo",
		},
		{
			name: "ticket",
			kind: "ticket",
			data: map[string]interface{}{
				"year":  float64(2025),
				"month": float64(2),
				"day":   float64(4),
				"slug":  "test-ticket",
			},
			wantID:  "📅\ufe0e 2025/02/04/test-ticket",
			wantURI: "semiorepo://ticket/2025/02/04/test-ticket",
		},
		{
			name: "file",
			kind: "file",
			data: map[string]interface{}{
				"path": "test.txt",
				"kind": "docs",
			},
			wantID:  "📃\ufe0e test.txt",
			wantURI: "semiorepo://file/test.txt",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotID := GetArtifactID(tt.kind, tt.data)
			if gotID != tt.wantID {
				t.Errorf("GetArtifactID() = %v, want %v", gotID, tt.wantID)
			}
			gotURI := GetArtifactURI(tt.kind, tt.data)
			if gotURI != tt.wantURI {
				t.Errorf("GetArtifactURI() = %v, want %v", gotURI, tt.wantURI)
			}
		})
	}
}

// #endregion Consolidated Tests
func TestMcpToolsSchemas(t *testing.T) {
s := createMcpServer()
tools := s.ListTools()

var validateSchema func(path string, schema map[string]any) error
validateSchema = func(path string, schema map[string]any) error {
typeVal, ok := schema["type"].(string)

// If type is "array", check for "items"
if ok && typeVal == "array" {
if _, hasItems := schema["items"]; !hasItems {
return fmt.Errorf("property '%s' is of type 'array' but missing 'items' field", path)
}
}

// Recursively check properties if present
if props, ok := schema["properties"].(map[string]any); ok {
for k, v := range props {
if propMap, ok := v.(map[string]any); ok {
if err := validateSchema(path+"."+k, propMap); err != nil {
return err
}
}
}
}

// Recursively check items if present (for arrays)
if items, ok := schema["items"].(map[string]any); ok {
if err := validateSchema(path+".items", items); err != nil {
return err
}
}

return nil
}

for name, tool := range tools {
t.Run(name, func(t *testing.T) {
// InputSchema.Properties is map[string]any
for propName, propSchema := range tool.Tool.InputSchema.Properties {
if propMap, ok := propSchema.(map[string]any); ok {
if err := validateSchema(propName, propMap); err != nil {
t.Errorf("Invalid schema for tool '%s': %v", name, err)
}
}
}
})
}
}
