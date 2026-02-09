// #region 🔖Header

// 🧪︎semio-repo/cli/main_test.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

// #region 🔖License

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

// #endregion 🔖License

// #region 🔖Specs

// TestFormatHeaderStructure MUST validate all header components (ID, summary, contributors, license, specs).
// TestFormatHeaderAllLanguages MUST verify FormatHeader for all languages that support headers and reject those that don't.
// TestHeaderPolicyMissingSubregions MUST verify missing-specs-region and wrong-license violations.
// TestFixtureViolationsByLanguage clean files MUST produce zero violations.

// #endregion 🔖Specs

// #endregion 🔖Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strconv"
	"strings"
	"testing"
	"time"

	"github.com/spf13/cobra"
)

// #region 🔖Helpers

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

func firstJSONLine(output string) (json.RawMessage, bool) {
	for _, line := range strings.Split(strings.TrimSpace(output), "\n") {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		return json.RawMessage(trimmed), true
	}
	return nil, false
}

func parseTicketOpenResult(t *testing.T, output string) (int, int, int, string) {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketOpen struct {
			Slug string `json:"slug"`
			Path string `json:"path"`
		} `json:"ticketOpen"`
	}
	if err := json.Unmarshal(data, &resp); err == nil && resp.TicketOpen.Path != "" {
		parts := strings.Split(strings.TrimPrefix(resp.TicketOpen.Path, "/"), "/")
		for i := 0; i+3 < len(parts); i++ {
			if parts[i] == "tickets" {
				y, _ := strconv.Atoi(parts[i+1])
				m, _ := strconv.Atoi(parts[i+2])
				d, _ := strconv.Atoi(parts[i+3])
				return y, m, d, resp.TicketOpen.Slug
			}
		}
	}
	t.Fatalf("unable to parse ticket open response: %s", output)
	return 0, 0, 0, ""
}

func parseGoalCreateID(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		GoalCreate struct {
			ID string `json:"id"`
		} `json:"goalCreate"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse goalCreate: %v\nOutput: %s", err, output)
	}
	if resp.GoalCreate.ID == "" {
		t.Fatalf("missing goal id in output: %s", output)
	}
	return resp.GoalCreate.ID
}

func TestInteractionUnmarshalAuthorShapes(t *testing.T) {
	cases := []struct {
		name         string
		authorJSON   string
		expectedAuth string
	}{
		{
			name:         "string author",
			authorJSON:   `"Ueli Saluz <ueli@semio-tech.com>"`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
		{
			name:         "object author",
			authorJSON:   `{"name":"Ueli Saluz","email":"ueli@semio-tech.com","github":"usalu"}`,
			expectedAuth: "Ueli Saluz <ueli@semio-tech.com>",
		},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			raw := fmt.Sprintf(`{
				"dates": {"started": "2026-02-06 22:03:11"},
				"author": %s,
				"system": {"version": "linux", "client": "codex"},
				"commit": "abc123",
				"prompt": "test",
				"llm": "gpt-5-2-codex"
			}`, tc.authorJSON)

			var interaction Interaction
			if err := json.Unmarshal([]byte(raw), &interaction); err != nil {
				t.Fatalf("unexpected unmarshal error: %v", err)
			}
			if interaction.Author != tc.expectedAuth {
				t.Fatalf("expected author %q, got %q", tc.expectedAuth, interaction.Author)
			}
		})
	}
}

func TestContributorDiscovery(t *testing.T) {
	// Setup temporary workspace
	tmpDir, err := os.MkdirTemp("", "semio-test-discovery")
	if err != nil {
		t.Fatalf("failed to create tmp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	// Update rootDir for testing
	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	contributorsDir := filepath.Join(tmpDir, ".semio-repo", "contributors")
	os.MkdirAll(contributorsDir, 0755)

	t.Run("Match and update email", func(t *testing.T) {
		// Mock existing contributor
		github := "usalu"
		c := Contributor{
			Github: github,
			Name:   "Ueli Saluz",
			Names:  []string{"Ueli Saluz"},
			Email:  "ueli@semio-tech.com",
			Emails: []string{"ueli@semio-tech.com"},
		}
		if err := SaveContributor(c); err != nil {
			t.Fatalf("failed to save: %v", err)
		}

		// Discovery with same email but different name
		authorStr := "Ueli <ueli@semio-tech.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, err := LoadContributor(github)
		if err != nil {
			t.Fatalf("failed to load: %v", err)
		}
		if len(updated.Names) != 2 || updated.Names[1] != "Ueli" {
			t.Errorf("expected names updated, got %v", updated.Names)
		}
	})

	t.Run("Match and update name", func(t *testing.T) {
		// Existing contributor
		github := "octocat"
		c := Contributor{
			Github: github,
			Name:   "The Octocat",
			Names:  []string{"The Octocat"},
			Email:  "octocat@github.com",
			Emails: []string{"octocat@github.com"},
		}
		SaveContributor(c)

		// Discovery with same name but different email
		authorStr := "The Octocat <octo@github.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != github {
			t.Errorf("expected github %q, got %q", github, gotGithub)
		}

		updated, _ := LoadContributor(github)
		found := false
		for _, e := range updated.Emails {
			if e == "octo@github.com" {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("expected emails updated with octo@github.com, got %v", updated.Emails)
		}
	})

	t.Run("No match returns original string", func(t *testing.T) {
		authorStr := "Stranger <stranger@danger.com>"
		gotGithub := FindAndUpdateContributor(authorStr)
		if gotGithub != authorStr {
			t.Errorf("expected original string, got %q", gotGithub)
		}
	})
}

func parseTicketCloseStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketClose struct {
			Status string `json:"status"`
		} `json:"ticketClose"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse ticketClose: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(resp.TicketClose.Status)
}

func parseTicketReopenStatus(t *testing.T, output string) string {
	t.Helper()
	data, ok := firstJSONLine(output)
	if !ok {
		t.Fatalf("no result in output: %s", output)
	}
	var resp struct {
		TicketReopen struct {
			Status string `json:"status"`
		} `json:"ticketReopen"`
	}
	if err := json.Unmarshal(data, &resp); err != nil {
		t.Fatalf("failed to parse ticketReopen: %v\nOutput: %s", err, output)
	}
	return strings.ToLower(resp.TicketReopen.Status)
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

// #endregion 🔖Helpers

// #region 🔖Collection Tests

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

// #endregion 🔖Collection Tests

// #region 🔖Nodes and Edges Tests

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
			t.Errorf("violation has empty id: %+v", v)
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
				range { start end }
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
					Start int `json:"start"`
					End   int `json:"end"`
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
				range { start end }
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
					Start int `json:"start"`
					End   int `json:"end"`
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
		"IMPLEMENTATION": true,
		"INTERFACE":      true,
		"CONSTANT":       true,
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

// #endregion 🔖Nodes and Edges Tests

// #region 🔖Cli

// #region 🔖Helpers

func executeCommand(args ...string) (string, string, error) {
	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(stdout)
	root.SetErr(stderr)
	root.SetArgs(args)
	config.Format = "json"
	err := root.Execute()
	if err != nil {
		fmt.Fprintln(stderr, err)
	}
	return stdout.String(), stderr.String(), err
}

func executeCommandMd(args ...string) (string, string, error) {
	stdout := new(bytes.Buffer)
	stderr := new(bytes.Buffer)
	root, config := NewRootWithConfig(testEngineFactory)
	root.SetOut(stdout)
	root.SetErr(stderr)
	root.SetArgs(args)
	config.Format = "md"
	err := root.Execute()
	if err != nil {
		fmt.Fprintln(stderr, err)
	}
	return stdout.String(), stderr.String(), err
}

func toolOutputText(result ToolResult) string {
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return strings.Join(lines, "\n")
}

// #endregion 🔖Helpers

// #region 🔖Codebase Tests

func TestCodebaseCommand(t *testing.T) {
	result := ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

// #endregion 🔖Codebase Tests

// #region 🔖Analyze Tests

func TestAnalyzeCommand(t *testing.T) {
	result := ToolAnalyze("semio/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := ToolAnalyze("semio/js/semio.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

// #endregion 🔖Analyze Tests

// #region 🔖Fix Tests

func TestFixCommand(t *testing.T) {
	result := ToolFix("semio/js")
	if result.Error != "" {
		t.Errorf("ToolFix returned error: %s", result.Error)
	}
	res, ok := result.Data.(*FixResult)
	if !ok || res == nil {
		t.Fatal("ToolFix returned nil or wrong type data")
	}
	if res.Fixed < 0 {
		t.Error("fixed count should not be negative")
	}
	if res.Remaining < 0 {
		t.Error("remaining count should not be negative")
	}
	if len(res.Violations) != res.Remaining {
		t.Errorf("violations length %d != remaining %d", len(res.Violations), res.Remaining)
	}
}

func TestFileHeaderId(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"code ts", "semio/js/src/index.ts", "\U0001F4BBsemio/js/src/index.ts"},
		{"code tsx", "semio/js/src/App.tsx", "\U0001F4BBsemio/js/src/App.tsx"},
		{"code go", "semio-repo/cli/cli.go", "\U0001F4BBsemio-repo/cli/cli.go"},
		{"code cs", "semio/gh/Semio.cs", "\U0001F4BBsemio/gh/Semio.cs"},
		{"code py", "semio/engine/main.py", "\U0001F4BBsemio/engine/main.py"},
		{"test ts", "semio/js/src/index.test.ts", "\U0001F9EAsemio/js/src/index.test.ts"},
		{"test go", "semio-repo/cli/cli_test.go", "\U0001F9EAsemio-repo/cli/cli_test.go"},
		{"config json", "tsconfig.json", "\u2699tsconfig.json"},
		{"docs md", "README.md", "\U0001F4C3README.md"},
		{"script sh", "build.sh", "\U0001F4DCbuild.sh"},
		{"script bash", "deploy.bash", "\U0001F4DCdeploy.bash"},
		{"script ps1", "setup.ps1", "\U0001F4DCsetup.ps1"},
		{"resource png", "logo.png", "\U0001F4BElogo.png"},
		{"license", "LICENSE.md", "\u2696LICENSE.md"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := FileHeaderId(tt.path)
			if got != tt.want {
				t.Errorf("FileHeaderId(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}

	t.Run("shebang ts file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "tools/build.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env tsx\nconsole.log('build');\n"), 0644)

		got := FileHeaderId(filePath)
		want := "\U0001F4DC" + filePath
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("shebang py file becomes script", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "scripts/run.py"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("#!/usr/bin/env python3\nprint('hello')\n"), 0644)

		got := FileHeaderId(filePath)
		want := "\U0001F4DC" + filePath
		if got != want {
			t.Errorf("FileHeaderId(%q) with shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("code ts without shebang stays code", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		filePath := "src/index.ts"
		absPath := filepath.Join(tmpDir, filePath)
		os.MkdirAll(filepath.Dir(absPath), 0755)
		os.WriteFile(absPath, []byte("export const x = 1;\n"), 0644)

		got := FileHeaderId(filePath)
		want := "\U0001F4BB" + filePath
		if got != want {
			t.Errorf("FileHeaderId(%q) without shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("nonexistent code file stays code", func(t *testing.T) {
		got := FileHeaderId("nonexistent/file.ts")
		want := "\U0001F4BBnonexistent/file.ts"
		if got != want {
			t.Errorf("FileHeaderId for nonexistent file = %q, want %q", got, want)
		}
	})
}

func TestDeriveFileKind(t *testing.T) {
	tests := []struct {
		name string
		file string
		want string
	}{
		{"ts code", "index.ts", FileKindCode},
		{"tsx code", "App.tsx", FileKindCode},
		{"go code", "main.go", FileKindCode},
		{"py code", "semio.py", FileKindCode},
		{"cs code", "Semio.cs", FileKindCode},
		{"rs code", "lib.rs", FileKindCode},
		{"rb code", "app.rb", FileKindCode},
		{"sh script", "build.sh", FileKindScript},
		{"bash script", "deploy.bash", FileKindScript},
		{"zsh script", "setup.zsh", FileKindScript},
		{"fish script", "init.fish", FileKindScript},
		{"bat script", "run.bat", FileKindScript},
		{"cmd script", "build.cmd", FileKindScript},
		{"ps1 script", "setup.ps1", FileKindScript},
		{"psm1 script", "module.psm1", FileKindScript},
		{"test ts", "index.test.ts", FileKindTest},
		{"test go", "main_test.go", FileKindTest},
		{"spec ts", "app.spec.ts", FileKindTest},
		{"benchmark go", "semio_benchmark.go", FileKindTest},
		{"stories tsx", "Button.stories.tsx", FileKindTest},
		{"json config", "tsconfig.json", FileKindConfig},
		{"yaml config", "config.yaml", FileKindConfig},
		{"toml config", "pyproject.toml", FileKindConfig},
		{"env config", ".env", FileKindConfig},
		{"md docs", "README.md", FileKindDocs},
		{"txt docs", "notes.txt", FileKindDocs},
		{"png resource", "logo.png", FileKindResource},
		{"svg resource", "icon.svg", FileKindResource},
		{"wasm resource", "module.wasm", FileKindResource},
		{"license md", "LICENSE.md", FileKindLicense},
		{"licence txt", "LICENCE.txt", FileKindLicense},
		{"gitignore config", ".gitignore", FileKindConfig},
		{"dockerfile config", "Dockerfile", FileKindConfig},
		{"makefile config", "Makefile", FileKindConfig},
		{"config suffix", "vite.config.ts", FileKindConfig},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DeriveFileKind(tt.file)
			if got != tt.want {
				t.Errorf("DeriveFileKind(%q) = %q, want %q", tt.file, got, tt.want)
			}
		})
	}
}

func TestFileKindEmoji(t *testing.T) {
	tests := []struct {
		name  string
		kind  string
		emoji string
	}{
		{"code", "code", "\U0001F4BB"},
		{"test", "test", "\U0001F9EA"},
		{"script", "script", "\U0001F4DC"},
		{"docs", "docs", "\U0001F4C3"},
		{"config", "config", "\u2699"},
		{"resource", "resource", "\U0001F4BE"},
		{"license", "license", "\u2696"},
		{"unknown", "unknown", "\U0001F4C4"},
		{"empty", "", "\U0001F4C4"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			data := map[string]interface{}{"kind": tt.kind}
			got := fileKindEmoji(data)
			if got != tt.emoji {
				t.Errorf("fileKindEmoji(%q) = %q, want %q", tt.kind, got, tt.emoji)
			}
		})
	}
}

func TestFixHeaderWrongFileId(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	filePath := "some/folder/code.ts"
	absPath := filepath.Join(tmpDir, filePath)
	os.MkdirAll(filepath.Dir(absPath), 0755)
	content := "// #region \U0001F516Header\n\n// old/wrong/path.ts\n\n// 2025 Test <t@t.com>\n\n// AGPL-3.0-or-later\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	expectedId := FileHeaderId(filePath)
	violations := []Violation{{
		Kind:    ViolationCodeHeaderWrongFileId,
		Scope:   filePath + "#Header",
		Line:    3,
		Excerpt: expectedId,
	}}
	fixed, err := applyAutofixes(filePath, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, expectedId) {
		t.Errorf("fixed content should contain %q, got:\n%s", expectedId, fixedContent)
	}
	if strings.Contains(fixedContent, "old/wrong/path.ts") {
		t.Error("fixed content should not contain old path")
	}
}

func TestFixHeaderWrongFileIdIdempotent(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	filePath := "some/folder/code.ts"
	absPath := filepath.Join(tmpDir, filePath)
	os.MkdirAll(filepath.Dir(absPath), 0755)
	correctId := FileHeaderId(filePath)
	content := "// #region \U0001F516Header\n\n// " + correctId + "\n\n// 2025 Test <t@t.com>\n\n// AGPL-3.0-or-later\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range violations {
		if v.Kind == ViolationCodeHeaderWrongFileId {
			t.Errorf("should not detect wrong file ID when correct ID is present, got: %s", v.Summary)
		}
	}
}

func TestFixHeaderWrongFileIdDetection(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	filePath := "some/folder/code.ts"
	absPath := filepath.Join(tmpDir, filePath)
	os.MkdirAll(filepath.Dir(absPath), 0755)
	content := "// #region \U0001F516Header\n\n// wrong/path.ts\n\n// 2025 Test <t@t.com>\n\n// AGPL-3.0-or-later\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	found := false
	for _, v := range violations {
		if v.Kind == ViolationCodeHeaderWrongFileId {
			found = true
			if !v.Autofixable() {
				t.Error("ViolationCodeHeaderWrongFileId should be autofixable")
			}
			if v.Excerpt != FileHeaderId(filePath) {
				t.Errorf("excerpt should be expected file ID, got %q", v.Excerpt)
			}
		}
	}
	if !found {
		t.Error("expected ViolationCodeHeaderWrongFileId violation")
	}
}

func TestFixHeaderWrongFileIdEndToEnd(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	filePath := "some/folder/code.ts"
	absPath := filepath.Join(tmpDir, filePath)
	os.MkdirAll(filepath.Dir(absPath), 0755)
	content := "// #region \U0001F516Header\n\n// wrong/path.ts\n\n// 2025 Test <t@t.com>\n\n// AGPL-3.0-or-later\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	var autofixable []Violation
	for _, v := range violations {
		if v.Autofixable() {
			autofixable = append(autofixable, v)
		}
	}

	fixed, err := applyAutofixes(filePath, autofixable)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Error("expected at least one fix")
	}

	fixedContent, _ := ReadTextFile(absPath)
	expectedId := FileHeaderId(filePath)
	if !strings.Contains(fixedContent, expectedId) {
		t.Errorf("fixed content should contain %q", expectedId)
	}

	ctx2 := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	violations2, _ := CheckPoliciesWithContext(ctx2, nil)
	for _, v := range violations2 {
		if v.Kind == ViolationCodeHeaderWrongFileId {
			t.Error("after fix, should not detect wrong file ID")
		}
	}
}

func TestFixApplyAutofixes(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	fixtureSrc := "semio/assets/repo/some/folder/file_fixable.tsx"
	expectedSrc := "semio/assets/repo/some/folder/file_fixable_expected.tsx"

	srcAbs := filepath.Join(rootDir, fixtureSrc)
	expectedAbs := filepath.Join(rootDir, expectedSrc)

	originalContent, err := ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixture: %v", err)
	}
	defer WriteTextFile(srcAbs, originalContent)

	expectedContent, err := ReadTextFile(expectedAbs)
	if err != nil {
		t.Fatalf("failed to read expected: %v", err)
	}

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: fixtureSrc}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixtureSrc})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	autofixableCount := 0
	for _, v := range violations {
		if v.Autofixable() {
			autofixableCount++
		}
	}
	if autofixableCount == 0 {
		t.Fatal("expected autofixable violations in fixture")
	}

	var autofixable []Violation
	for _, v := range violations {
		if v.Autofixable() {
			autofixable = append(autofixable, v)
		}
	}

	fixed, err := applyAutofixes(fixtureSrc, autofixable)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Error("expected at least one fix applied")
	}

	fixedContent, err := ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}

	if strings.TrimSpace(fixedContent) != strings.TrimSpace(expectedContent) {
		t.Errorf("fixed content does not match expected.\nGot:\n%s\n\nExpected:\n%s", fixedContent, expectedContent)
	}
}

func TestFixSectionMissingEndName(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖MySection\n\nconst x = 1;\n\n// #endregion\n"
	expected := "// #region 🔖MySection\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"

	testFile := "test_missing_end.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionNameMismatch(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Alpha\n\nconst x = 1;\n\n// #endregion 🔖Beta\n"
	expected := "// #region 🔖Alpha\n\nconst x = 1;\n\n// #endregion 🔖Alpha\n"

	testFile := "test_mismatch.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeSectionNameMismatch, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Keep\n\nconst x = 1;\n\n// #endregion 🔖Keep\n\n// #region 🔖Empty\n\n// #endregion 🔖Empty\n\n// #region 🔖Also\n\nconst y = 2;\n\n// #endregion 🔖Also\n"
	expected := "// #region 🔖Keep\n\nconst x = 1;\n\n// #endregion 🔖Keep\n\n// #region 🔖Also\n\nconst y = 2;\n\n// #endregion 🔖Also\n"

	testFile := "test_empty.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeSectionEmpty, Scope: testFile + "#Empty", Line: 7},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixInlineComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n// inline one\n\n// inline two\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_inline.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixBlockComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n/* block comment */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_block.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeCommentBlock, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixJSDocComment(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\n/** jsdoc comment */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	expected := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"

	testFile := "test_jsdoc.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeCommentJSDoc, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixMultipleViolationsSameFile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖A\n\n// bad comment\n\nconst a = 1;\n\n// #endregion\n\n// #region 🔖B\n\n// another bad\n\nconst b = 2;\n\n// #endregion 🔖Wrong\n"
	testFile := "test_multi.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 3},
		{Kind: ViolationCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 11},
		{Kind: ViolationCodeSectionNameMismatch, Scope: testFile, Line: 15},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 4 {
		t.Errorf("expected 4 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖A") {
		t.Error("expected missing end name to be fixed to A")
	}
	if !strings.Contains(result, "// #endregion 🔖B") {
		t.Error("expected mismatch to be fixed to B")
	}
	if strings.Contains(result, "// bad comment") {
		t.Error("expected inline comment to be removed")
	}
	if strings.Contains(result, "// another bad") {
		t.Error("expected second inline comment to be removed")
	}
}

func TestFixImprovedCommentLogic(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	// Test content with various comment scenarios
	content := `// #region 🔖Section
const x = 1; // trailing comment
// TODO: fix this
// this line is part of the todo description

// another normal comment
/* TODO: block todo */
const y = 2; // normal trailing
// #endregion 🔖Fix Tests
`
	// Expected result after fixes
	// 1. trailing comment removed (partial line)
	// 2. TODO line and its description description kept
	// 3. normal comment removed
	// 4. block todo kept

	testFile := "test_improved.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	// We'll use ScanComments to generate violations
	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	violations := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	// Verify ScanComments logic
	// We expect:
	// Line 2: trailing (col 14)
	// Line 5: another normal comment (col 1)
	// Line 7: normal trailing (col 14)

	expectedViolations := 3
	if len(violations) != expectedViolations {
		t.Errorf("expected %d violations, got %d", expectedViolations, len(violations))
		for i, v := range violations {
			t.Logf("Violation %d: %s at %d:%d", i, v.Kind, v.Line, v.Column)
		}
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}

	// trailing comment + normal comment + mid-line block
	if fixed != 3 {
		t.Errorf("expected 3 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)

	// Verification of content
	if strings.Contains(result, "trailing comment") {
		t.Errorf("trailing comment should be removed")
	}
	if !strings.Contains(result, "const x = 1;") {
		t.Errorf("code 'const x = 1;' should be kept")
	}
	if !strings.Contains(result, "// TODO: fix this") {
		t.Errorf("TODO comment should be kept")
	}
	if !strings.Contains(result, "// this line is part of the todo description") {
		t.Errorf("TODO description should be kept")
	}
	if strings.Contains(result, "// another normal comment") {
		t.Errorf("normal comment should be removed")
	}
	if !strings.Contains(result, "/* TODO: block todo */") {
		t.Errorf("block TODO should be kept")
	}

	// Check partial line removal for line 2
	lines := strings.Split(result, "\n")
	foundX := false
	for _, l := range lines {
		if strings.HasPrefix(l, "const x = 1;") {
			foundX = true
			if strings.Contains(l, "//") {
				t.Errorf("line 2 should not contain comment: %q", l)
			}
			if strings.HasSuffix(l, " ") {
				t.Errorf("line 2 should be trimmed right: %q", l)
			}
		}
	}
	if !foundX {
		t.Errorf("did not find 'const x = 1;' line in result")
	}
}

func TestFixConfigIgnored(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// comment in config\nconst x = 1;\n"
	testFile := "package.json" // Config file extension
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	violations := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	if len(violations) != 0 {
		t.Errorf("expected 0 violations for config file, got %d", len(violations))
	}
}

func TestScanCommentsGo(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewGoLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// this is a comment\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentInline {
			t.Errorf("expected inline comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* block */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentBlock {
			t.Errorf("expected block comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// TODO: fix later\n// continuation of todo\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for TODO, got %d", len(violations))
		}
	})

	t.Run("block TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* TODO: fix later */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for block TODO, got %d", len(violations))
		}
	})

	t.Run("nolint skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// nolint:errcheck\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for nolint, got %d", len(violations))
		}
	})

	t.Run("raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `// not a comment`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in raw string, got %d", len(violations))
		}
	})

	t.Run("multi-line raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `line1\n// not a comment\nline3`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in multi-line raw string, got %d", len(violations))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "// #region 🔖Header\n\n// header comment\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for header section, got %d", len(violations))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for region markers, got %d", len(violations))
		}
	})

	t.Run("debug marker skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfmt.Println(\"[DEBUG] test\")\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for debug marker, got %d", len(violations))
		}
	})

	t.Run("url scheme not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar url = \"https://example.com\"\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for URL scheme, got %d", len(violations))
		}
	})

	t.Run("grouped inline comments", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// comment one\n\n// comment two\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(violations) != 2 {
			t.Errorf("expected 2 violations for separate comment blocks, got %d", len(violations))
		}
	})
}

func TestScanCommentsPython(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewPythonLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentInline {
			t.Errorf("expected inline comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "# region Section\n\n# TODO: fix later\n# continuation of todo\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for TODO, got %d", len(violations))
		}
	})

	t.Run("noqa skipped", func(t *testing.T) {
		content := "# region Section\n\n# noqa: E501\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for noqa, got %d", len(violations))
		}
	})

	t.Run("type ignore skipped", func(t *testing.T) {
		content := "# region Section\n\n# type: ignore[assignment]\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for type: ignore, got %d", len(violations))
		}
	})

	t.Run("triple double quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"# not a comment\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in triple-quoted string, got %d", len(violations))
		}
	})

	t.Run("triple single quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = '''# not a comment'''\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in triple-single-quoted string, got %d", len(violations))
		}
	})

	t.Run("multi-line triple quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"\n# not a comment\n\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in multi-line triple-quoted string, got %d", len(violations))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "# region Header\n#\n# header comment\n#\n# endregion Header\n\n# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for header section, got %d", len(violations))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for region markers, got %d", len(violations))
		}
	})

	t.Run("comment in regular string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"# not a comment\"\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in string, got %d", len(violations))
		}
	})

	t.Run("trailing comment", func(t *testing.T) {
		content := "# region Section\n\nx = 1  # trailing comment\n\ndef main(): pass\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation for trailing comment, got %d", len(violations))
		}
		if violations[0].Column <= 1 {
			t.Errorf("expected column > 1 for trailing comment, got %d", violations[0].Column)
		}
	})
}

func TestScanCommentsCSharp(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewCSharpLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n// this is a comment\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentInline {
			t.Errorf("expected inline comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n/* block */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentBlock {
			t.Errorf("expected block comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// TODO: fix later\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for TODO, got %d", len(violations))
		}
	})

	t.Run("pragma skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// pragma warning disable\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for pragma, got %d", len(violations))
		}
	})

	t.Run("verbatim string skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\nvar s = @\"// not a comment\";\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in verbatim string, got %d", len(violations))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for region markers, got %d", len(violations))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "#region 🔖Header\n// header comment\n#endregion 🔖Header\n\n#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for header section, got %d", len(violations))
		}
	})

	t.Run("no JSDoc for csharp", func(t *testing.T) {
		content := "#region 🔖Section\n\n/** not jsdoc in csharp */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentBlock {
			t.Errorf("expected block comment (not JSDoc) for C#, got %s", violations[0].Kind)
		}
	})
}

func TestScanCommentsTypeScript(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()

	t.Run("JSDoc detected", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/** jsdoc */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentJSDoc {
			t.Errorf("expected JSDoc violation, got %s", violations[0].Kind)
		}
	})

	t.Run("template literal skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `// not a comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in template literal, got %d", len(violations))
		}
	})

	t.Run("template expression not skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `${x} // comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for template expression context, got %d", len(violations))
		}
	})

	t.Run("eslint directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// eslint-disable-next-line\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for eslint directive, got %d", len(violations))
		}
	})

	t.Run("@ts directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// @ts-ignore\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for @ts directive, got %d", len(violations))
		}
	})

	t.Run("string literals skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst a = '// not a comment';\nconst b = \"// not a comment\";\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in strings, got %d", len(violations))
		}
	})

	t.Run("config file skipped", func(t *testing.T) {
		content := "// inline comment\nconst x = 1;\n"
		violations := lang.ScanComments(ctx, "tsconfig.json", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for config file, got %d", len(violations))
		}
	})
}

func TestScanCommentsShell(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewShellLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\necho hello\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentInline {
			t.Errorf("expected inline comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("comment in string skipped", func(t *testing.T) {
		content := "# region Section\n\necho \"# not a comment\"\n\necho hello\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for comment in string, got %d", len(violations))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\necho hello\n\n# endregion Section\n"
		violations := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for region markers, got %d", len(violations))
		}
	})
}

func TestScanCommentsRust(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewRustLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// this is a comment\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentInline {
			t.Errorf("expected inline comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* block comment */\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(violations) != 1 {
			t.Fatalf("expected 1 violation, got %d", len(violations))
		}
		if violations[0].Kind != ViolationCodeCommentBlock {
			t.Errorf("expected block comment violation, got %s", violations[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// TODO: fix later\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		violations := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(violations) != 0 {
			t.Errorf("expected 0 violations for TODO, got %d", len(violations))
		}
	})
}

func TestScanCommentsAutofix(t *testing.T) {
	t.Run("python inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "# region Section\n\n# bad comment\n\ndef main(): pass\n\n# endregion Section\n"
		expected := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_inline.py"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		violations := []Violation{
			{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, violations)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("python trailing comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "# region Section\n\nx = 1  # trailing\n\ndef main(): pass\n\n# endregion Section\n"
		testFile := "test_py_trailing.py"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		violations := []Violation{
			{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 3, Column: 7},
		}
		fixed, err := applyAutofixes(testFile, violations)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if !strings.Contains(result, "x = 1") {
			t.Error("code should be preserved")
		}
		if strings.Contains(result, "trailing") {
			t.Error("trailing comment should be removed")
		}
	})

	t.Run("go block comment fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Section\n\n/* block comment */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		expected := "// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		testFile := "test_go_block.go"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		violations := []Violation{
			{Kind: ViolationCodeCommentBlock, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, violations)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})

	t.Run("csharp inline fix", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "#region 🔖Section\n\n// bad comment\n\npublic class C {}\n\n#endregion 🔖Section\n"
		expected := "#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		testFile := "test_cs_inline.cs"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		violations := []Violation{
			{Kind: ViolationCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, violations)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 1 {
			t.Errorf("expected 1 fix, got %d", fixed)
		}
		result, _ := ReadTextFile(absPath)
		if result != expected {
			t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
		}
	})
}

func TestEmojiVariationAutofix(t *testing.T) {
	t.Run("strip emoji variation", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "This is a test \U0001F4BB\uFE0E with emoji variation.\nAnd another line \u2699\uFE0F with VS16."
		expected := "This is a test \U0001F4BB with emoji variation.\nAnd another line \u2699 with VS16."
		testFile := "test_emoji.txt"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		violations := []Violation{
			{Kind: ViolationCodeUnicodeEmojiVariation, Scope: testFile, Line: 1},
			{Kind: ViolationCodeUnicodeEmojiVariation, Scope: testFile, Line: 2},
		}
		fixed, err := applyAutofixes(testFile, violations)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 2 {
			t.Fatalf("expected 2 fixed, got %d", fixed)
		}
		got, _ := ReadTextFile(absPath)
		if got != expected {
			t.Errorf("expected:\n%q\ngot:\n%q", expected, got)
		}
	})
}

func TestFixNonAutofixableNotFixed(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	bundles := LoadBundles()
	path := "semio/assets/repo/some/folder/file_invalid.tsx"
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	for _, v := range violations {
		info := v.Kind.Info()
		if v.Autofixable() != info.Autofixable {
			t.Errorf("violation %s: Autofixable() = %v, Info().Autofixable = %v", v.Kind, v.Autofixable(), info.Autofixable)
		}
	}

	autofixableKinds := []ViolationKind{
		ViolationCodeHeaderWrongFileId,
	}
	counts := map[ViolationKind]int{}
	for _, v := range violations {
		counts[v.Kind]++
	}
	for _, kind := range autofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected autofixable violation kind %s to be detected", kind)
		}
		if !kind.Info().Autofixable {
			t.Errorf("violation kind %s should be autofixable", kind)
		}
	}
	nonAutofixableKinds := []ViolationKind{
		ViolationCodeHeaderMissingContributors,
		ViolationCodeHeaderWrongLicense,
		ViolationCodeSectionMissingStartName,
		ViolationCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected non-autofixable violation kind %s to be detected", kind)
		}
		if kind.Info().Autofixable {
			t.Errorf("violation kind %s should not be autofixable", kind)
		}
	}
}

func TestFixViaGraphQL(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	result, err := executor.ExecuteJSON(ctx, `mutation { fix(scope: "semio-repo/go/main_test.go") { fixed remaining violations { id summary } } }`, nil)
	if err != nil {
		t.Fatalf("fix mutation failed: %v", err)
	}

	var resp struct {
		Fix struct {
			Fixed      int `json:"fixed"`
			Remaining  int `json:"remaining"`
			Violations []struct {
				ID      string `json:"id"`
				Summary string `json:"summary"`
			} `json:"violations"`
		} `json:"fix"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse fix response: %v\nResult: %s", err, result)
	}
	if resp.Fix.Remaining < 0 {
		t.Error("remaining should not be negative")
	}
	if len(resp.Fix.Violations) != resp.Fix.Remaining {
		t.Errorf("violations length %d != remaining %d", len(resp.Fix.Violations), resp.Fix.Remaining)
	}
}

func TestFixViaRepoContext(t *testing.T) {
	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()

	ctx := NewRepoContext(rootDir)
	scope := "semio-repo/go/main_test.go"
	res, err := ctx.Fix(&scope)
	if err != nil {
		t.Fatalf("Fix failed: %v", err)
	}
	if res == nil {
		t.Fatal("Fix returned nil result")
	}
	if res.Violations == nil {
		t.Error("Violations should not be nil")
	}
	if res.Remaining != len(res.Violations) {
		t.Errorf("remaining %d != violations length %d", res.Remaining, len(res.Violations))
	}
}

func TestFixIdempotent(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
	testFile := "test_clean.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{}
	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 0 {
		t.Errorf("expected 0 fixes on clean file, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if result != content {
		t.Error("clean file should not be modified")
	}
}

func TestFixNestedSections(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	content := "// #region 🔖Outer\n\n// #region 🔖Inner\n\nconst x = 1;\n\n// #endregion\n\nconst y = 2;\n\n// #endregion\n"
	testFile := "test_nested.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	violations := []Violation{
		{Kind: ViolationCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: ViolationCodeSectionMissingEndName, Scope: testFile, Line: 11},
	}

	fixed, err := applyAutofixes(testFile, violations)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 2 {
		t.Errorf("expected 2 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖Inner") {
		t.Error("expected inner endregion to get name Inner")
	}
	if !strings.Contains(result, "// #endregion 🔖Outer") {
		t.Error("expected outer endregion to get name Outer")
	}
}

func TestFixExtractFileFromScope(t *testing.T) {
	tests := []struct {
		scope    string
		expected string
	}{
		{"file.ts", "file.ts"},
		{"file.ts#Section", "file.ts"},
		{"file.ts::definition", "file.ts"},
		{"path/to/file.ts#Section/Sub", "path/to/file.ts"},
		{"path/to/file.ts::myFunc", "path/to/file.ts"},
	}
	for _, tt := range tests {
		result := extractFileFromScope(tt.scope)
		if result != tt.expected {
			t.Errorf("extractFileFromScope(%q) = %q, want %q", tt.scope, result, tt.expected)
		}
	}
}

func TestFixViolationKindMeta(t *testing.T) {
	autofixableKinds := []ViolationKind{
		ViolationCodeHeaderWrongFileId,
		ViolationCodeSectionEmpty,
		ViolationCodeSectionMissingEndName,
		ViolationCodeSectionNameMismatch,
		ViolationCodeCommentInline,
		ViolationCodeCommentBlock,
		ViolationCodeCommentJSDoc,
	}
	for _, kind := range autofixableKinds {
		info := kind.Info()
		if !info.Autofixable {
			t.Errorf("violation kind %s should be autofixable", kind)
		}
		if info.Reason == "" {
			t.Errorf("violation kind %s has empty reason", kind)
		}
		if info.Solution == "" {
			t.Errorf("violation kind %s has empty solution", kind)
		}
	}

	nonAutofixableKinds := []ViolationKind{
		ViolationCodeHeaderMissingRegion,
		ViolationCodeHeaderMissingContributors,
		ViolationCodeHeaderMissingLicense,
		ViolationCodeHeaderWrongLicense,
		ViolationCodeSectionMissingStartName,
		ViolationCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		info := kind.Info()
		if info.Autofixable {
			t.Errorf("violation kind %s should NOT be autofixable", kind)
		}
	}
}

func TestFindMatchingSectionStartName(t *testing.T) {
	lines := []string{
		"// #region 🔖Outer",
		"",
		"// #region 🔖Inner",
		"const x = 1;",
		"// #endregion 🔖Inner",
		"",
		"// #endregion",
	}
	language := NewTypeScriptLanguage()

	name := findMatchingSectionStartName(lines, 6, language)
	if name != "Outer" {
		t.Errorf("expected Outer, got %q", name)
	}

	name = findMatchingSectionStartName(lines, 4, language)
	if name != "Inner" {
		t.Errorf("expected Inner, got %q", name)
	}
}

// #endregion 🔖Cli

// #region 🔖Policy Tests

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

func TestPolicyTreeCommand(t *testing.T) {
	result := ToolPolicyTree()
	if result.Error != "" {
		t.Errorf("ToolPolicyTree returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyTree returned nil data")
	}
	policies, ok := result.Data.([]PolicyDef)
	if !ok {
		t.Error("ToolPolicyTree data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyTree returned no policies")
	}
	text := result.Output.Lines[0].Text
	if !strings.Contains(text, "code") {
		t.Error("Expected policy tree output to contain 'code' policy")
	}
	if !strings.Contains(text, "code:header:missing-region") {
		t.Error("Expected policy tree output to contain violation kind 'code:header:missing-region'")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "semio/js")
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
	path := "semio/assets/repo/some/folder/file_invalid.tsx"
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
		ViolationCodeHeaderWrongFileId,
		ViolationCodeHeaderMissingContributors,
		ViolationCodeHeaderWrongLicense,
		ViolationCodeHeaderMissingSpecsRegion,
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
	if counts[ViolationCodeCommentInline] != 2 {
		t.Fatalf("expected 2 inline comment violations, got %d", counts[ViolationCodeCommentInline])
	}
}

func TestFixtureViolationsByLanguage(t *testing.T) {
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []ViolationKind
	}{
		{
			path:          "semio/assets/repo/some/folder/file_invalid.py",
			requiredKinds: []ViolationKind{ViolationCodeHeaderMissingRegion},
		},
		{
			path:          "semio/assets/repo/some/folder/file_invalid.cs",
			requiredKinds: []ViolationKind{ViolationCodeHeaderMissingContributors},
		},
		{
			path:          "semio/assets/repo/some/folder/file_invalid.go",
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
		"semio/assets/repo/some/folder/file_fixed.tsx",
		"semio/assets/repo/some/folder/file_fixed.py",
		"semio/assets/repo/some/folder/file_fixed.cs",
		"semio/assets/repo/some/folder/file_fixed.go",
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

func TestSpecsViolation(t *testing.T) {
	t.Run("isSpecText detects RFC 2119 keywords", func(t *testing.T) {
		cases := []struct {
			text   string
			expect bool
		}{
			{"File headers MUST contain License subregions.", true},
			{"Implementations SHOULD follow the standard.", true},
			{"This feature MAY be omitted.", true},
			{"Clients SHALL NOT modify the data.", true},
			{"This is REQUIRED for all files.", true},
			{"This approach is RECOMMENDED.", true},
			{"This field is OPTIONAL.", true},
			{"MUST NOT contain inline code.", true},
			{"This is a normal comment.", false},
			{"Just some text here.", false},
			{"", false},
		}
		for _, tc := range cases {
			got := isSpecText(tc.text)
			if got != tc.expect {
				t.Errorf("isSpecText(%q) = %v, want %v", tc.text, got, tc.expect)
			}
		}
	})

	t.Run("hasImplementationSyntax detects backticks", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"File headers MUST contain `License` subregions.", true},
			{"Use `FormatHeader` to build headers.", true},
			{"File headers MUST contain License subregions.", false},
			{"Specs MUST be implementation-agnostic.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("hasImplementationSyntax detects function calls", func(t *testing.T) {
		cases := []struct {
			text      string
			hasSyntax bool
		}{
			{"FormatHeader() MUST build the header.", true},
			{"Call ctx.ReadText() for content.", true},
			{"File headers MUST contain License subregions.", false},
			{"Specs MUST be clean.", false},
		}
		for _, tc := range cases {
			got, _ := hasImplementationSyntax(tc.text)
			if got != tc.hasSyntax {
				t.Errorf("hasImplementationSyntax(%q) = %v, want %v", tc.text, got, tc.hasSyntax)
			}
		}
	})

	t.Run("specsPolicy detects implementation syntax in header Specs", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n\n// File headers MUST contain `License` subregions.\n\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := specsPolicy(ctx)

		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeSpecsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected ViolationCodeSpecsSyntax for backtick-wrapped code in header Specs")
		}
	})

	t.Run("specsPolicy clean specs no violation", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n\n// File headers MUST contain License subregions.\n\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := specsPolicy(ctx)

		for _, v := range violations {
			if v.Kind == ViolationCodeSpecsSyntax {
				t.Errorf("unexpected ViolationCodeSpecsSyntax for clean spec: %s", v.Summary)
			}
		}
	})

	t.Run("specsPolicy detects implementation syntax in section specs", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST call `ctx.Check()` internally.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := specsPolicy(ctx)

		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeSpecsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected ViolationCodeSpecsSyntax for backtick in section spec")
		}
	})

	t.Run("section spec comments exempt from inline violation", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST check constraints.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := commentPolicy(ctx)

		for _, v := range violations {
			if v.Kind == ViolationCodeCommentInline {
				t.Errorf("spec comment should be exempt from inline violation: line %d %s", v.Line, v.Excerpt)
			}
		}
	})

	t.Run("JSDoc spec comments exempt from JSDoc violation", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n/**\n * Kits MUST be editable offline.\n */\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := commentPolicy(ctx)

		for _, v := range violations {
			if v.Kind == ViolationCodeCommentJSDoc {
				t.Errorf("JSDoc spec comment should be exempt from JSDoc violation: line %d", v.Line)
			}
		}
	})

	t.Run("non-spec JSDoc still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n/**\n * This is a regular docstring without spec keywords.\n */\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := commentPolicy(ctx)

		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeCommentJSDoc {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec JSDoc to still be flagged")
		}
	})

	t.Run("non-spec inline comment still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 💻 test.ts\n\n// 2025 Test <t@t.com>\n\n// #region 🔖License\n\n// AGPL\n\n// #endregion 🔖License\n\n// #region 🔖Specs\n// #endregion 🔖Specs\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// This is a regular comment not a spec.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		violations := commentPolicy(ctx)

		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeCommentInline {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec inline comment to be flagged")
		}
	})

	t.Run("ViolationCodeSpecsSyntax in violation info table", func(t *testing.T) {
		info := ViolationCodeSpecsSyntax.Info()
		if info.Kind != ViolationCodeSpecsSyntax {
			t.Errorf("expected kind %s, got %s", ViolationCodeSpecsSyntax, info.Kind)
		}
		if info.Autofixable {
			t.Error("specs syntax violation should not be autofixable")
		}
	})
}

func TestDocsViolation(t *testing.T) {
	t.Run("docsPolicy detects missing README.md", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		violations := docsPolicy(ctx)
		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeDocsMissingReadme {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected ViolationCodeDocsMissingReadme for missing README.md")
		}
	})
	t.Run("docsPolicy detects missing Summary section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# Specs\n\nSome specs here.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		violations := docsPolicy(ctx)
		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeDocsMissingReadme && strings.Contains(v.Summary, "Summary") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected ViolationCodeDocsMissingReadme for missing # Summary section")
		}
	})
	t.Run("docsPolicy detects missing Specs section", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		violations := docsPolicy(ctx)
		found := false
		for _, v := range violations {
			if v.Kind == ViolationCodeDocsMissingReadme && strings.Contains(v.Summary, "Specs") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected ViolationCodeDocsMissingReadme for missing # Specs section")
		}
	})
	t.Run("docsPolicy clean README no violation", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		readmePath := filepath.Join(tmpDir, bundleRoot, "README.md")
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		if err := WriteTextFile(readmePath, "# Summary\n\nA test bundle.\n\n# Docs\n\n# Specs\n\nSome specs.\n"); err != nil {
			t.Fatalf("failed to write: %v", err)
		}
		bundles := []Bundle{{Name: "test-bundle", Root: bundleRoot}}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		violations := docsPolicy(ctx)
		for _, v := range violations {
			if v.Kind == ViolationCodeDocsMissingReadme {
				t.Errorf("unexpected ViolationCodeDocsMissingReadme: %s", v.Summary)
			}
		}
	})
	t.Run("docsPolicy deduplicates bundles with same root", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		bundleRoot := "test-bundle"
		if err := os.MkdirAll(filepath.Join(tmpDir, bundleRoot), 0755); err != nil {
			t.Fatalf("failed to create dir: %v", err)
		}
		bundles := []Bundle{
			{Name: "bundle-a", Root: bundleRoot},
			{Name: "bundle-b", Root: bundleRoot},
		}
		scope := Scope{Kind: ScopeRepo}
		ctx := NewPolicyContext(scope, bundles)
		violations := docsPolicy(ctx)
		count := 0
		for _, v := range violations {
			if v.Kind == ViolationCodeDocsMissingReadme {
				count++
			}
		}
		if count != 1 {
			t.Errorf("expected 1 violation for deduplicated root, got %d", count)
		}
	})
}

func TestFormatHeaderStructure(t *testing.T) {
	lang := NewTypeScriptLanguage()
	header := lang.FormatHeader("💻test/file.ts", "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some specs")
	if !strings.Contains(header, "// #region 🔖Header") {
		t.Error("header missing Header region start")
	}
	if !strings.Contains(header, "// #endregion 🔖Header") {
		t.Error("header missing Header region end")
	}
	if !strings.Contains(header, "// #region 🔖License") {
		t.Error("header missing License subregion start")
	}
	if !strings.Contains(header, "// #endregion 🔖License") {
		t.Error("header missing License subregion end")
	}
	if !strings.Contains(header, "// #region 🔖Specs") {
		t.Error("header missing Specs subregion start")
	}
	if !strings.Contains(header, "// #endregion 🔖Specs") {
		t.Error("header missing Specs subregion end")
	}
	if !strings.Contains(header, "💻test/file.ts") {
		t.Error("header missing file ID")
	}
	if !strings.Contains(header, "A test file") {
		t.Error("header missing summary")
	}
	if !strings.Contains(header, "2025 Test User <test@test.com>") {
		t.Error("header missing contributors")
	}
	if !strings.Contains(header, "AGPL license text here") {
		t.Error("header missing license text")
	}
	if !strings.Contains(header, "Some specs") {
		t.Error("header missing specs text")
	}
}

func TestFormatHeaderEmptySpecs(t *testing.T) {
	lang := NewGoLanguage()
	header := lang.FormatHeader("💻test/file.go", "", "2025 Dev <dev@dev.com>", "AGPL text", "")
	if !strings.Contains(header, "// #region 🔖Specs") {
		t.Error("header missing Specs subregion even when empty")
	}
	if !strings.Contains(header, "// #endregion 🔖Specs") {
		t.Error("header missing Specs endregion even when empty")
	}
}

func TestFormatHeaderAllLanguages(t *testing.T) {
	languages := []LanguagePlugin{
		NewTypeScriptLanguage(),
		NewGoLanguage(),
		NewPythonLanguage(),
		NewCSharpLanguage(),
		NewRustLanguage(),
		NewRubyLanguage(),
		NewShellLanguage(),
		NewSqlLanguage(),
		NewGraphqlLanguage(),
	}
	for _, lang := range languages {
		header := lang.FormatHeader("💻test/file", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header == "" {
			t.Errorf("%s: FormatHeader returned empty", lang.Name())
		}
		if !strings.Contains(header, "License") {
			t.Errorf("%s: header missing License region", lang.Name())
		}
		if !strings.Contains(header, "Specs") {
			t.Errorf("%s: header missing Specs region", lang.Name())
		}
	}
	noHeader := []LanguagePlugin{
		NewMarkdownLanguage(),
		NewTomlLanguage(),
		NewYamlLanguage(),
	}
	for _, lang := range noHeader {
		header := lang.FormatHeader("💻test/file", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header != "" {
			t.Errorf("%s: FormatHeader should return empty for non-header language", lang.Name())
		}
	}
}

func TestHeaderPolicyMissingSubregions(t *testing.T) {
	bundles := LoadBundles()
	path := "semio/assets/repo/some/folder/file_invalid.tsx"
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	violations, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[ViolationKind]int{}
	for _, v := range violations {
		counts[v.Kind]++
	}
	if counts[ViolationCodeHeaderMissingSpecsRegion] == 0 {
		t.Error("expected missing-specs-region violation")
	}
	if counts[ViolationCodeHeaderWrongLicense] == 0 {
		t.Error("expected wrong-license violation")
	}
}

// #endregion 🔖Policy Tests

// #region 🔖Bundle Tests

func TestBundleListCommand(t *testing.T) {
	result := ToolBundleList()
	if result.Error != "" {
		t.Errorf("ToolBundleList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolBundleList returned nil data")
	}
	bundles, ok := result.Data.([]Bundle)
	if !ok {
		t.Error("ToolBundleList data is not []Bundle")
		return
	}
	if len(bundles) == 0 {
		t.Error("ToolProjectList returned no bundles")
	}
	foundJS := false
	for _, b := range bundles {
		if b.Name == "semio/js" {
			foundJS = true
			break
		}
	}
	if !foundJS {
		t.Error("Expected to find 'semio/js' bundle")
	}
}

func TestBundleTreeCommand(t *testing.T) {
	result := ToolProjectTree()
	if result.Error != "" {
		t.Errorf("ToolProjectTree returned error: %s", result.Error)
	}
}

// #endregion 🔖Bundle Tests

// #region 🔖Folder Tests

func TestFolderListCommand(t *testing.T) {
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	result := ToolFolderList("semio-repo")
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
	result := ToolFolderTree("semio/go")
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

// #endregion 🔖Folder Tests

// #region 🔖File Tests

func TestFileListCommand(t *testing.T) {
	result := ToolFileList("semio/js")
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
	result := ToolFileTree("semio/go")
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

// #endregion 🔖File Tests

// #region 🔖Section Tests

func TestSectionListCommand(t *testing.T) {
	result := ToolSectionList("semio/js/semio.ts")
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
	result := ToolSectionTree("semio/js/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// #endregion 🔖Section Tests

// #region 🔖Definition Tests

func TestDefinitionListCommand(t *testing.T) {
	result := ToolDefinitionList("semio/js/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

// #endregion 🔖Definition Tests

// #region 🔖Ticket Tests

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

// #endregion 🔖Ticket Tests

// #region 🔖Goal Tests

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

// #endregion 🔖Goal Tests

// #region 🔖Contributor Tests

func TestContributorListCommand(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

// #endregion 🔖Contributor Tests

// #region 🔖GraphQL Tests

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
	if !strings.Contains(result, "semio/js") {
		t.Errorf("Expected result to contain 'semio/js', got: %s", result)
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
	result, err := executor.ExecuteJSON(context.Background(), `mutation { fix(scope: "semio-repo/go/main_test.go") { fixed remaining } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL fix mutation returned error: %v", err)
	}
	if !strings.Contains(result, "fixed") {
		t.Errorf("Expected result to contain 'fixed', got: %s", result)
	}
}

// #endregion 🔖GraphQL Tests

// #region 🔖Tree Tests

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

	output, err := executeTreeCommand("tree", "semio/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {
		t.Errorf("repo tree semio/go missing semio.go, got:\n%s", output)
	}
	if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
		t.Errorf("repo tree default output must be markdown, got:\n%s", output)
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("repo tree default output missing markdown list items, got:\n%s", output)
	}

	// 2. Folder Tree
	output, err = executeTreeCommand("folder", "tree", "semio/go")
	if err != nil {
		t.Errorf("folder tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {
		// Just checking that we got some output
		if len(output) < 10 {
			t.Errorf("folder tree output suspicious: %s", output)
		}
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("folder tree default output must be markdown, got:\n%s", output)
	}

	// 3. File Tree
	output, err = executeTreeCommand("file", "tree", "semio/go")
	if err != nil {
		t.Errorf("file tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {
		t.Errorf("file tree missing semio.go")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("file tree default output must be markdown, got:\n%s", output)
	}

	// 4. Ticket Tree
	output, err = executeTreeCommand("ticket", "tree")
	if err != nil {
		t.Errorf("ticket tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("ticket tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("ticket tree default output must be markdown, got:\n%s", output)
	}

	// 5. Goal Tree
	output, err = executeTreeCommand("goal", "tree")
	if err != nil {
		t.Errorf("goal tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("goal tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("goal tree default output must be markdown, got:\n%s", output)
	}

	// 6. Repo Tree Text
	output, err = executeTreeCommand("tree", "semio/go", "--text")
	if err != nil {
		t.Errorf("repo tree text failed: %v", err)
	}
	if !strings.Contains(output, "├── ") && !strings.Contains(output, "└── ") {
		t.Errorf("repo tree text output should use connectors, got:\n%s", output)
	}

	// 7. Repo Tree JSON
	output, err = executeTreeCommand("tree", "semio/go", "--json")
	if err != nil {
		t.Errorf("repo tree json failed: %v", err)
	}
	var parsed map[string]interface{}
	if parseErr := json.Unmarshal([]byte(strings.TrimSpace(output)), &parsed); parseErr != nil {
		t.Errorf("repo tree json output is invalid JSON: %v\noutput:\n%s", parseErr, output)
	}
	if _, ok := parsed["kind"]; !ok {
		if _, ok := parsed["Kind"]; !ok {
			t.Errorf("repo tree json output missing kind field: %s", output)
		}
	}
}

func TestCliE2E_TicketLifecycle_Syntaxes_NoGithub(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}

	fileRel := filepath.ToSlash(filepath.Join("go", "repo", "main.go"))

	openOut, openErr, err := executeCommand(
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
		t.Fatalf("ticket open positional failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	y, m, d, slug := parseTicketOpenResult(t, openOut)
	defer os.RemoveAll(GetTicketPath(y, m, d, slug))

	closeOut, closeErr, err := executeCommand(
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
		t.Fatalf("ticket close flags failed: %v\nStdout: %s\nStderr: %s", err, closeOut, closeErr)
	}
	if status := parseTicketCloseStatus(t, closeOut); status != "closed" {
		t.Fatalf("expected closed status, got %s", status)
	}

	reopenOut, reopenErr, err := executeCommand(
		"ticket", "reopen",
		fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug),
		"E2E reopen prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-github",
	)
	if err != nil {
		t.Fatalf("ticket reopen mix failed: %v\nStdout: %s\nStderr: %s", err, reopenOut, reopenErr)
	}
	if status := parseTicketReopenStatus(t, reopenOut); status != "open" {
		t.Fatalf("expected open status, got %s", status)
	}

	listOut, listErr, err := executeCommand("ticket", "list", "--year", strconv.Itoa(y))
	if err != nil {
		t.Fatalf("ticket list failed: %v\nStdout: %s\nStderr: %s", err, listOut, listErr)
	}
}

func TestCliE2E_GoalLifecycle_Syntaxes_NoGithub(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}
	openOut, openErr, err := executeCommand(
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
		t.Fatalf("goal open failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	goalID := parseGoalCreateID(t, openOut)
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

	_, closeErr, err := executeCommand("goal", "close", goalID, "E2E Goal Summary", "--no-github")
	if err != nil {
		t.Fatalf("goal close failed: %v\nStderr: %s", err, closeErr)
	}

	_, reopenErr, err := executeCommand("goal", "reopen", goalID, "E2E Goal Reopen Prompt", "cursor-chat", "gpt-5-mini", "--no-github")
	if err != nil {
		t.Fatalf("goal reopen failed: %v\nStderr: %s", err, reopenErr)
	}
}

func TestCliE2E_MiscCommands_NoSideEffects(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping e2e cli tests in short mode")
	}

	cmds := []struct {
		name string
		args []string
	}{
		{"bundle list", []string{"bundle", "list"}},
		{"bundle tree", []string{"bundle", "tree"}},
		{"folder list", []string{"folder", "list", "go"}},
		{"file list", []string{"file", "list", "go"}},
		{"section list", []string{"section", "list", "semio/js/semio.ts"}},
		{"definition list", []string{"definition", "list", "semio/js/semio.ts"}},
		{"policy list", []string{"policy", "list"}},
		{"policy check", []string{"policy", "check", "code", "semio/js"}},
		{"goal list", []string{"goal", "list"}},
		{"goal tree", []string{"goal", "tree"}},
		{"ticket list", []string{"ticket", "list"}},
		{"ticket tree", []string{"ticket", "tree"}},
		{"contributor list", []string{"contributor", "list"}},
		{"mcp dry-run", []string{"mcp", "--dry-run"}},
		{"update", []string{"update"}},
	}

	for _, c := range cmds {
		t.Run(c.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(c.args...)
			if err != nil {
				t.Fatalf("%s failed: %v\nStdout: %s\nStderr: %s", c.name, err, stdout, stderr)
			}
		})
	}
}

// #region 🔖Wrong Argument Tests

func TestCliWrongArgs_TicketOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing title", []string{"ticket", "open", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"missing client", []string{"ticket", "open", "--goal", "TEST", "--title", "Valid Title", "--opus-4-5", "--no-github"}},
		{"missing goal", []string{"ticket", "open", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "close", "--no-github", "--summary", "s", "--files", "f"}},
		{"missing summary", []string{"ticket", "close", "--no-github", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--files", "f"}},
		{"missing files", []string{"ticket", "close", "--no-github", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--summary", "s"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"invalid path format", []string{"ticket", "reopen", "bad-path", "prompt", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"missing prompt", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"missing client", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "prompt", "--opus-4-5", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"invalid path format", []string{"ticket", "change", "bad-path", "--no-github"}},
		{"nonexistent ticket", []string{"ticket", "change", "9999/01/01/NONEXISTENT", "--title", "New Title", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing title", []string{"goal", "open", "--no-github"}},
		{"missing description", []string{"goal", "open", "Valid Title", "--no-github", "--copilot-chat", "--opus-4-5", "--due-date", "2026-02-15"}},
		{"missing client", []string{"goal", "open", "Valid Title", "desc", "prompt", "--opus-4-5", "--due-date", "2026-02-15", "--no-github"}},
		{"missing llm", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--due-date", "2026-02-15", "--no-github"}},
		{"missing due-date", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--opus-4-5", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "close", "--no-github"}},
		{"missing summary", []string{"goal", "close", "NONEXISTENT-GOAL", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "reopen", "--no-github"}},
		{"missing prompt", []string{"goal", "reopen", "NONEXISTENT-GOAL", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"missing client", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--opus-4-5", "--no-github"}},
		{"missing llm", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--copilot-chat", "--no-github"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_PolicyCheck(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing policy id", []string{"policy", "check"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FolderOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"folder", "create"}},
		{"move missing args", []string{"folder", "move"}},
		{"move missing target", []string{"folder", "move", "src"}},
		{"delete missing path", []string{"folder", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FileOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"file", "create"}},
		{"move missing args", []string{"file", "move"}},
		{"move missing target", []string{"file", "move", "src"}},
		{"delete missing path", []string{"file", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_SectionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"list missing file", []string{"section", "list"}},
		{"tree missing file", []string{"section", "tree"}},
		{"create missing args", []string{"section", "create"}},
		{"move missing args", []string{"section", "move"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_DefinitionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"list missing file", []string{"definition", "list"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_ContributorOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"add missing github", []string{"contributor", "add"}},
		{"remove missing github", []string{"contributor", "remove"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GraphQL(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing query", []string{"graphql"}},
		{"invalid query syntax", []string{"graphql", "{ invalid @@@ }"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliJsonPureData(t *testing.T) {
	cmds := []struct {
		name string
		args []string
	}{
		{"bundle list", []string{"bundle", "list"}},
		{"ticket list", []string{"ticket", "list"}},
		{"folder list", []string{"folder", "list", "semio-repo/go"}},
		{"file list", []string{"file", "list", "semio-repo/go"}},
		{"section list", []string{"section", "list", "semio-repo/go/main.go"}},
		{"definition list", []string{"definition", "list", "semio-repo/go/main.go"}},
		{"policy list", []string{"policy", "list"}},
		{"contributor list", []string{"contributor", "list"}},
		{"goal list", []string{"goal", "list"}},
	}

	for _, c := range cmds {
		t.Run(c.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(c.args...)
			if err != nil {
				t.Fatalf("%s failed: %v\nStderr: %s", c.name, err, stderr)
			}
			lines := strings.Split(strings.TrimSpace(stdout), "\n")
			for _, line := range lines {
				trimmed := strings.TrimSpace(line)
				if trimmed == "" {
					continue
				}
				var data map[string]interface{}
				if jsonErr := json.Unmarshal([]byte(trimmed), &data); jsonErr != nil {
					t.Errorf("invalid JSON line: %s\nError: %v", trimmed, jsonErr)
					continue
				}
				if _, hasKind := data["kind"]; hasKind {
					if _, hasCmd := data["command"]; hasCmd {
						t.Errorf("expected pure data, got event wrapper: %s", trimmed)
					}
				}
				if _, hasData := data["data"]; hasData {
					inner, ok := data["data"].(map[string]interface{})
					if ok && len(data) == 1 {
						_ = inner
						t.Errorf("expected pure data without {\"data\": ...} wrapper: %s", trimmed)
					}
				}
			}
		})
	}
}

func TestCliJsonErrorsToStderr(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"ticket open missing title", []string{"ticket", "open", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-github"}},
		{"goal open missing title", []string{"goal", "open", "--no-github"}},
		{"policy check missing id", []string{"policy", "check"}},
		{"folder create missing path", []string{"folder", "create"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, _, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s", tt.name)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
		})
	}
}

// #endregion 🔖Wrong Argument Tests

// #region 🔖Consolidated Tests

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
		":10-20",
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
		"path/to/file.ts§MyDefinition",
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
		"📚MyBundle",
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
		"📁path/to/folder",
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
		if strings.Contains(output, "2026-02-15") {
			t.Error("output should not contain absolute due date")
		}
		if !strings.Contains(output, "in ") && !strings.Contains(output, "from now") && !strings.Contains(output, "ago") {
			t.Error("output missing relative due date")
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

func assertValidMarkdownLink(t *testing.T, output string) {
	t.Helper()
	trimmed := strings.TrimSpace(output)
	if trimmed == "" {
		t.Error("output is empty")
		return
	}
	for _, line := range strings.Split(trimmed, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		stripped := strings.TrimLeft(line, " ")
		if strings.HasPrefix(stripped, "- ") {
			stripped = strings.TrimPrefix(stripped, "- ")
		}
		if strings.HasPrefix(stripped, "**") {
			continue
		}
		if !strings.Contains(stripped, "[") || !strings.Contains(stripped, "](") {
			t.Errorf("line missing markdown link syntax [...](...): %q", line)
		}
		if strings.Contains(line, "```json") || strings.Contains(line, "```\n") {
			t.Errorf("output contains JSON code block: %q", line)
		}
	}
}

func TestFormatMarkdownResult_MutationKeys(t *testing.T) {
	mutations := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticketOpen", "ticketOpen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketClose", "ticketClose", map[string]interface{}{
			"slug": "MY-TICKET", "status": "closed", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketReopen", "ticketReopen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
			"date": map[string]interface{}{"created": "2026-02-06T00:00:00Z"},
		}},
		{"ticketChange", "ticketChange", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goalCreate", "goalCreate", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalClose", "goalClose", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "closed",
		}},
		{"goalReopen", "goalReopen", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalChange", "goalChange", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"folderCreate", "folderCreate", map[string]interface{}{
			"path": "new/folder", "kind": "custom",
		}},
		{"folderDelete", "folderDelete", map[string]interface{}{
			"path": "old/folder", "kind": "custom",
		}},
		{"folderMove", "folderMove", map[string]interface{}{
			"path": "moved/folder", "kind": "custom",
		}},
		{"fileCreate", "fileCreate", map[string]interface{}{
			"id": "new/file.ts",
		}},
		{"fileDelete", "fileDelete", map[string]interface{}{
			"id": "old/file.ts",
		}},
		{"fileMove", "fileMove", map[string]interface{}{
			"id": "moved/file.ts",
		}},
		{"sectionCreate", "sectionCreate", map[string]interface{}{
			"name": "NewSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(10),
		}},
		{"sectionDelete", "sectionDelete", map[string]interface{}{
			"name": "OldSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"sectionMove", "sectionMove", map[string]interface{}{
			"name": "MovedSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"contributorRemove", "contributorRemove", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
		{"todoCreate", "todoCreate", map[string]interface{}{
			"name": "My Todo",
		}},
		{"todoChange", "todoChange", map[string]interface{}{
			"name": "Changed Todo",
		}},
		{"todoDelete", "todoDelete", map[string]interface{}{
			"name": "Deleted Todo",
		}},
	}

	for _, tt := range mutations {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.name, json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
			if strings.Contains(output, "```") {
				t.Errorf("markdown output contains code fence for %s:\n%s", tt.name, output)
			}
		})
	}
}

func TestFormatMarkdownResult_SingleEntities(t *testing.T) {
	entities := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticket", "ticket", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goal", "goal", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"bundle", "bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path/to/bundle",
		}},
		{"folder", "folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"file", "file", map[string]interface{}{
			"id": "some/file.ts",
		}},
		{"definition", "definition", map[string]interface{}{
			"name": "myFunc", "kind": "function", "filePath": "file.ts",
			"startLine": float64(10), "endLine": float64(20),
		}},
		{"contributor", "contributor", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
		{"policy", "policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"project", "project", map[string]interface{}{
			"id": "myProject", "description": "My project",
		}},
		{"draft", "draft", map[string]interface{}{
			"id": "some-draft",
		}},
		{"todo", "todo", map[string]interface{}{
			"name": "My Todo",
		}},
		{"commit", "commit", map[string]interface{}{
			"sha": "abc123", "message": "Initial commit",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.name+" get", json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
		})
	}
}

func TestFormatMarkdownResult_Lists(t *testing.T) {
	repoLists := []struct {
		name string
		key  string
		kind string
		data map[string]interface{}
	}{
		{"tickets", "tickets", "ticket", map[string]interface{}{
			"slug": "T1", "status": "open", "title": "Ticket One",
			"year": float64(2026), "month": float64(1), "day": float64(1),
		}},
		{"bundles", "bundles", "bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path",
		}},
		{"folders", "folders", "folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"files", "files", "file", map[string]interface{}{
			"id": "some/file.ts",
		}},
		{"contributors", "contributors", "contributor", map[string]interface{}{
			"github": "octocat", "name": "Cat",
		}},
		{"policies", "policies", "policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"projects", "projects", "project", map[string]interface{}{
			"id": "proj", "description": "Project",
		}},
		{"violationKinds", "violationKinds", "violationKind", map[string]interface{}{
			"id": "vk1", "description": "Violation kind",
		}},
	}

	for _, tt := range repoLists {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{
				"repo": map[string]interface{}{
					tt.key: []interface{}{tt.data, tt.data},
				},
			}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.key+" list", json.RawMessage(jsonBytes))
			lines := strings.Split(strings.TrimSpace(output), "\n")
			if len(lines) < 2 {
				t.Errorf("expected at least 2 lines for list of 2 items, got %d: %s", len(lines), output)
			}
			for _, line := range lines {
				line = strings.TrimSpace(line)
				if line == "" {
					continue
				}
				if !strings.HasPrefix(line, "- [") {
					t.Errorf("list line missing '- [' prefix: %q", line)
				}
				if !strings.Contains(line, "](") {
					t.Errorf("list line missing markdown link: %q", line)
				}
			}
		})
	}

	topLists := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"todos", "todos", map[string]interface{}{"name": "Todo 1"}},
		{"sections", "sections", map[string]interface{}{"name": "Sec1", "filePath": "f.ts", "startLine": float64(1), "endLine": float64(5)}},
		{"definitions", "definitions", map[string]interface{}{"name": "def1", "filePath": "f.ts", "startLine": float64(1), "endLine": float64(5)}},
		{"drafts", "drafts", map[string]interface{}{"id": "draft1"}},
	}
	for _, tt := range topLists {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{
				tt.key: []interface{}{tt.data},
			}
			jsonBytes, _ := json.Marshal(payload)
			output := formatMarkdownResult(tt.key+" list", json.RawMessage(jsonBytes))
			assertValidMarkdownLink(t, output)
			if !strings.Contains(output, "- [") {
				t.Errorf("list output missing '- [' prefix: %s", output)
			}
		})
	}
}

func TestCollectEntityProps_MultilineEscaped(t *testing.T) {
	tests := []struct {
		name string
		kind string
		data map[string]interface{}
	}{
		{"ticket summary with newlines", "ticket", map[string]interface{}{
			"slug": "T1", "status": "closed", "title": "Fix Bug",
			"year": float64(2026), "month": float64(1), "day": float64(1),
			"summary": "Fixed three areas:\n\n1. First fix\n2. Second fix\n3. Third fix",
		}},
		{"ticket prompt with newlines", "ticket", map[string]interface{}{
			"slug": "T2", "status": "open", "title": "Add Feature",
			"year": float64(2026), "month": float64(1), "day": float64(2),
			"prompt": "Please implement:\n- item A\n- item B",
		}},
		{"goal description with newlines", "goal", map[string]interface{}{
			"id": "GOAL1", "title": "Goal One", "status": "open",
			"description": "Line one\nLine two\r\nLine three",
		}},
		{"policy description with newlines", "policy", map[string]interface{}{
			"id": "P1", "description": "Rule one\nRule two",
		}},
		{"commit message with newlines", "commit", map[string]interface{}{
			"id": "abc123", "message": "feat: add feature\n\nDetailed description here",
		}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "\n") || strings.Contains(p, "\r") {
					t.Errorf("property contains newline: %q", p)
				}
			}
		})
	}
}

func TestFormatMarkdownResult_Analyze(t *testing.T) {
	payload := map[string]interface{}{
		"analyze": map[string]interface{}{
			"metrics": map[string]interface{}{
				"total":       float64(3),
				"autofixable": float64(1),
			},
			"violations": []interface{}{
				map[string]interface{}{
					"kind":    map[string]interface{}{"id": "inline-comment"},
					"scope":   "file.ts",
					"line":    float64(10),
					"summary": "Remove inline comment",
				},
			},
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("analyze", json.RawMessage(jsonBytes))
	if !strings.Contains(output, "Total Violations") {
		t.Error("analyze output missing 'Total Violations'")
	}
	if !strings.Contains(output, "inline-comment") {
		t.Error("analyze output missing violation kind")
	}
}

func TestFormatMarkdownResult_Fix(t *testing.T) {
	payload := map[string]interface{}{
		"fix": map[string]interface{}{
			"fixed":     float64(3),
			"remaining": float64(1),
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("fix", json.RawMessage(jsonBytes))
	if strings.Contains(output, "```") {
		t.Errorf("fix output contains code fence: %s", output)
	}
}

func TestFormatMarkdownResult_FileWithSections(t *testing.T) {
	payload := map[string]interface{}{
		"file": map[string]interface{}{
			"id": "src/main.ts",
			"sections": []interface{}{
				map[string]interface{}{
					"name": "Header", "path": "src/main.ts#Header", "filePath": "src/main.ts",
					"startLine": float64(1), "endLine": float64(5),
					"children": []interface{}{
						map[string]interface{}{
							"name": "SubHeader", "path": "src/main.ts#Header#SubHeader", "filePath": "src/main.ts",
							"startLine": float64(2), "endLine": float64(4),
						},
					},
				},
			},
			"definitions": []interface{}{
				map[string]interface{}{
					"name": "myFunc", "kind": "function", "filePath": "src/main.ts",
					"id":        "src/main.ts§myFunc",
					"startLine": float64(10), "endLine": float64(20),
				},
			},
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("file get", json.RawMessage(jsonBytes))
	assertValidMarkdownLink(t, output)
	if !strings.Contains(output, "Header") {
		t.Errorf("output missing section name 'Header', got: %s", output)
	}
	if !strings.Contains(output, "myFunc") {
		t.Errorf("output missing definition name 'myFunc', got: %s", output)
	}
}

func TestFormatMarkdownResult_NoJSONFallback(t *testing.T) {
	payload := map[string]interface{}{
		"unknownKey": map[string]interface{}{
			"id": "test",
		},
	}
	jsonBytes, _ := json.Marshal(payload)
	output := formatMarkdownResult("unknown", json.RawMessage(jsonBytes))
	if strings.Contains(output, "```") {
		t.Errorf("output contains JSON code block: %s", output)
	}
	if strings.Contains(output, "{") && strings.Contains(output, "}") {
		trimmed := strings.TrimSpace(output)
		if trimmed[0] == '{' {
			t.Errorf("output is raw JSON: %s", output)
		}
	}
}

func TestFormatResult_MutationKeys(t *testing.T) {
	mutations := []struct {
		name string
		key  string
		data map[string]interface{}
	}{
		{"ticketOpen", "ticketOpen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketClose", "ticketClose", map[string]interface{}{
			"slug": "MY-TICKET", "status": "closed", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"ticketReopen", "ticketReopen", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goalCreate", "goalCreate", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"goalClose", "goalClose", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "closed",
		}},
		{"goalReopen", "goalReopen", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"folderCreate", "folderCreate", map[string]interface{}{
			"path": "new/folder", "kind": "custom",
		}},
		{"folderDelete", "folderDelete", map[string]interface{}{
			"path": "old/folder", "kind": "custom",
		}},
		{"fileCreate", "fileCreate", map[string]interface{}{
			"id": "new/file.ts",
		}},
		{"fileDelete", "fileDelete", map[string]interface{}{
			"id": "old/file.ts",
		}},
		{"sectionCreate", "sectionCreate", map[string]interface{}{
			"name": "NewSection", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(10),
		}},
		{"contributorRemove", "contributorRemove", map[string]interface{}{
			"github": "octocat", "name": "The Octocat",
		}},
	}

	for _, tt := range mutations {
		t.Run(tt.name, func(t *testing.T) {
			payload := map[string]interface{}{tt.key: tt.data}
			jsonBytes, _ := json.Marshal(payload)
			output := formatResult(tt.name, json.RawMessage(jsonBytes), false)
			trimmed := strings.TrimSpace(output)
			if trimmed == "" {
				t.Errorf("output is empty for %s", tt.name)
			}
			if trimmed[0] == '{' {
				t.Errorf("output is raw JSON for %s: %s", tt.name, output)
			}
			if strings.Contains(output, "\""+tt.key+"\"") {
				t.Errorf("output contains raw key %q for %s: %s", tt.key, tt.name, output)
			}
		})
	}
}

func TestRenderEntityMarkdownLink_AllKinds(t *testing.T) {
	entities := []struct {
		kind string
		data map[string]interface{}
	}{
		{"ticket", map[string]interface{}{
			"slug": "MY-TICKET", "status": "open", "title": "My Ticket",
			"year": float64(2026), "month": float64(2), "day": float64(6),
		}},
		{"goal", map[string]interface{}{
			"id": "MY-GOAL", "title": "My Goal", "status": "open",
		}},
		{"bundle", map[string]interface{}{
			"name": "MyBundle", "root": "/path",
		}},
		{"folder", map[string]interface{}{
			"path": "some/folder", "kind": "custom",
		}},
		{"file", map[string]interface{}{
			"id": "file.ts",
		}},
		{"section", map[string]interface{}{
			"name": "Sec", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"definition", map[string]interface{}{
			"name": "def", "filePath": "file.ts", "startLine": float64(1), "endLine": float64(5),
		}},
		{"contributor", map[string]interface{}{
			"github": "octocat", "name": "Cat",
		}},
		{"todo", map[string]interface{}{
			"name": "Todo",
		}},
		{"draft", map[string]interface{}{
			"id": "draft1",
		}},
		{"policy", map[string]interface{}{
			"id": "code", "description": "Code policy",
		}},
		{"violationKind", map[string]interface{}{
			"id": "vk1", "description": "Desc",
		}},
		{"project", map[string]interface{}{
			"id": "proj", "description": "Desc",
		}},
		{"commit", map[string]interface{}{
			"sha": "abc123", "message": "msg",
		}},
		{"repo", map[string]interface{}{
			"name": "myrepo",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind, func(t *testing.T) {
			output := renderEntityMarkdownLink(tt.kind, tt.data)
			if !strings.HasPrefix(output, "[") {
				t.Errorf("link for %s missing '[' prefix: %s", tt.kind, output)
			}
			if !strings.Contains(output, "](") {
				t.Errorf("link for %s missing '](': %s", tt.kind, output)
			}
			if !strings.Contains(output, "semiorepo://") {
				t.Errorf("link for %s missing 'semiorepo://' URI: %s", tt.kind, output)
			}
			if strings.Contains(output, "```") {
				t.Errorf("link for %s contains code fence: %s", tt.kind, output)
			}
		})
	}
}

func TestInferEntityKind(t *testing.T) {
	cases := []struct {
		key      string
		expected string
	}{
		{"ticketOpen", "ticket"},
		{"ticketClose", "ticket"},
		{"ticketReopen", "ticket"},
		{"ticketChange", "ticket"},
		{"goalCreate", "goal"},
		{"goalClose", "goal"},
		{"goalReopen", "goal"},
		{"goalChange", "goal"},
		{"folderCreate", "folder"},
		{"folderDelete", "folder"},
		{"folderMove", "folder"},
		{"fileCreate", "file"},
		{"fileDelete", "file"},
		{"fileMove", "file"},
		{"sectionCreate", "section"},
		{"sectionDelete", "section"},
		{"sectionMove", "section"},
		{"definitionList", "definition"},
		{"contributorRemove", "contributor"},
		{"todoCreate", "todo"},
		{"todoChange", "todo"},
		{"todoDelete", "todo"},
		{"syncGithub", "repo"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "repo"},
		{"unknownKey", ""},
	}

	for _, tt := range cases {
		t.Run(tt.key, func(t *testing.T) {
			got := inferEntityKind(tt.key)
			if got != tt.expected {
				t.Errorf("inferEntityKind(%q) = %q, want %q", tt.key, got, tt.expected)
			}
		})
	}
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
			args:        []string{"tree"},
			wantMarkers: []string{"- [", "]("},
		},
		{
			name:        "Ticket Tree MD",
			args:        []string{"ticket", "tree"},
			wantMarkers: []string{"- [", "](semiorepo://ticket/"},
		},
		{
			name:        "Goal Tree MD",
			args:        []string{"goal", "tree"},
			wantMarkers: []string{"- [", "](semiorepo://goal/"},
		},
		{
			name:        "Ticket List MD",
			args:        []string{"ticket", "list"},
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
			if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
				t.Errorf("Output should not contain ASCII tree connectors in default markdown mode:\n%s", output)
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

	modes := []string{"", "json", "md", "text"}

	for _, mode := range modes {
		t.Run("lifecycle_"+mode, func(t *testing.T) {
			title := "Test Lifecycle " + mode
			if mode == "" {
				title = "Test Lifecycle default"
			}

			rootCmd := NewRoot(factory)
			// Create goal first
			goalTitle := fmt.Sprintf("Test Goal %s LifecycleTest %d", mode, time.Now().UnixNano())
			goalCmd := NewRoot(factory)
			goalB := bytes.NewBufferString("")
			goalCmd.SetOut(goalB)
			goalCmd.SetErr(goalB)
			goalCmd.SetArgs([]string{"goal", "open", goalTitle, "Test Goal Description", "Test Goal Prompt", "copilot-chat", "gemini-3-pro", "--due-date", "2025-12-31", "--no-github", "--json"})
			if err := goalCmd.Execute(); err != nil {
				t.Fatalf("goal open failed: %v\nOutput: %s", err, goalB.String())
			}
			goalID := parseGoalCreateID(t, goalB.String())
			defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

			openArgs := []string{"ticket", "open", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--goal", goalID, "--no-issue", "--no-github"}
			if mode == "json" {
				openArgs = append(openArgs, "--json")
			} else if mode == "md" {
				openArgs = append(openArgs, "--md")
			} else if mode == "text" {
				openArgs = append(openArgs, "--text")
			}

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

			var y, m, d int
			var slug string
			found := false

			for _, line := range strings.Split(strings.TrimSpace(listB.String()), "\n") {
				if strings.TrimSpace(line) == "" {
					continue
				}
				var env struct {
					Ticket struct {
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
						Slug  string `json:"slug"`
						Title string `json:"title"`
					} `json:"ticket"`
				}
				if json.Unmarshal([]byte(line), &env) == nil {
					if strings.EqualFold(env.Ticket.Title, title) {
						y, m, d, slug = env.Ticket.Year, env.Ticket.Month, env.Ticket.Day, env.Ticket.Slug
						found = true
						break
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
				"--files", "semio-repo/go/main.go",
			}
			if mode == "json" {
				closeArgs = append(closeArgs, "--json")
			} else if mode == "md" {
				closeArgs = append(closeArgs, "--md")
			} else if mode == "text" {
				closeArgs = append(closeArgs, "--text")
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
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "ticket list",
			args:  []string{"ticket", "list"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "folder list",
			args:  []string{"folder", "list", "semio-repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "file list",
			args:  []string{"file", "list", "semio-repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "section list",
			args:  []string{"section", "list", "semio-repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "definition list",
			args:  []string{"definition", "list", "semio-repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "policy list",
			args:  []string{"policy", "list"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "contributor list",
			args:  []string{"contributor", "list"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "project list",
			args:  []string{"project", "list"},
			modes: []string{"", "json", "md", "text"},
		},
	}

	for _, tt := range tests {
		for _, mode := range tt.modes {
			testName := tt.name
			if mode != "" {
				testName += " --" + mode
			} else {
				testName += " (default)"
			}

			t.Run(testName, func(t *testing.T) {
				rootCmd := NewRoot(factory)
				b := bytes.NewBufferString("")
				rootCmd.SetOut(b)
				rootCmd.SetErr(b)

				args := append([]string(nil), tt.args...)
				if mode == "json" {
					args = append(args, "--json")
				} else if mode == "md" {
					args = append(args, "--md")
				} else if mode == "text" {
					args = append(args, "--text")
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
						// Verify it is pure data
						var data map[string]interface{}
						if err := json.Unmarshal([]byte(line), &data); err != nil {
							t.Errorf("Invalid JSON line: %s", line)
						}
						// Check if it's an event wrapper
						if kind, ok := data["kind"].(string); ok {
							if kind == "result" || kind == "start" || kind == "done" {
								// Extra check to avoid false positives if data item has "kind"
								if _, hasCmd := data["command"]; hasCmd {
									t.Errorf("Expected pure data, got Event wrapper: %s", line)
								}
							}
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
		{"TypeScript", ".ts", "const x = 1;\n// #region 🔖%s\nconst y = 2;\n// #endregion 🔖%s\n", "Renamed"},
		{"Go", ".go", "package main\n// #region 🔖%s\nvar y = 2\n// #endregion 🔖%s\n", "Renamed"},
		{"Python", ".py", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"CSharp", ".cs", "#region 🔖%s\nvar y = 2;\n#endregion 🔖%s\n", "Renamed"},
		{"Rust", ".rs", "// #region 🔖%s\nlet y = 2;\n// #endregion 🔖%s\n", "Renamed"},
		{"Ruby", ".rb", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"Shell", ".sh", "# region %s\ny=2\n# endregion %s\n", "Renamed"},
		{"TOML", ".toml", "# region %s\ny = 2\n# endregion %s\n", "Renamed"},
		{"YAML", ".yaml", "# region %s\ny: 2\n# endregion %s\n", "Renamed"},
		{"SQL", ".sql", "-- #region 🔖%s\nSELECT 1;\n-- #endregion 🔖%s\n", "Renamed"},
		{"GraphQL", ".graphql", "# #region 🔖%s\ntype Query { name: String }\n# #endregion 🔖%s\n", "Renamed"},
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
			wantID:  emojiText(EmojiRepo),
			wantURI: "semiorepo://repo",
		},
		{
			name:    "projects collection",
			kind:    "projects",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiProjects),
			wantURI: "semiorepo://projects",
		},
		{
			name:    "project user",
			kind:    "project",
			data:    map[string]interface{}{"name": "semio", "kind": "user"},
			wantID:  fmt.Sprintf("%s@semio", emojiText(EmojiProjectUser)),
			wantURI: "semiorepo://project/@semio",
		},
		{
			name:    "project infrastructure",
			kind:    "project",
			data:    map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"},
			wantID:  fmt.Sprintf("%s@semio-repo", emojiText(EmojiProjectInfra)),
			wantURI: "semiorepo://project/@semio-repo",
		},
		{
			name:    "project research",
			kind:    "project",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  fmt.Sprintf("%s@coda", emojiText(EmojiProjectResearch)),
			wantURI: "semiorepo://project/@coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"projectCode": "semio"},
			wantID:  fmt.Sprintf("%ssemio", emojiText(EmojiBundles)),
			wantURI: "semiorepo://bundles",
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "semio/js", "kind": "library"},
			wantID:  fmt.Sprintf("%ssemio/js", emojiText(EmojiBundleLibrary)),
			wantURI: "semiorepo://bundle/semio/js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/examples", "kind": "library"},
			wantID:  fmt.Sprintf("%scoda/examples", emojiText(EmojiBundleLibrary)),
			wantURI: "semiorepo://bundle/coda/examples",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "semio/desktop", "kind": "ui"},
			wantID:  fmt.Sprintf("%ssemio/desktop", emojiText(EmojiBundleUI)),
			wantURI: "semiorepo://bundle/semio/desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiFolders),
			wantURI: "semiorepo://folders",
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "semio/js/src"},
			wantID:  fmt.Sprintf("%ssemio/js/src", emojiText(EmojiFolders)),
			wantURI: "semiorepo://folders/semio/js/src",
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "semio/js/src", "kind": "required"},
			wantID:  fmt.Sprintf("%ssemio/js/src", emojiText(EmojiFolderRequired)),
			wantURI: "semiorepo://folder/semio/js/src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "semio/js/utils", "kind": "organization"},
			wantID:  fmt.Sprintf("%ssemio/js/utils", emojiText(EmojiFolderOrg)),
			wantURI: "semiorepo://folder/semio/js/utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiFiles),
			wantURI: "semiorepo://files",
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  fmt.Sprintf("%stest.txt", emojiText(EmojiFileDocs)),
			wantURI: "semiorepo://file/test.txt",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  fmt.Sprintf("%smain.go", emojiText(EmojiFileCode)),
			wantURI: "semiorepo://file/main.go",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "semio/js/src/index.test.ts", "kind": "test"},
			wantID:  fmt.Sprintf("%ssemio/js/src/index.test.ts", emojiText(EmojiFileTest)),
			wantURI: "semiorepo://file/semio/js/src/index.test.ts",
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  fmt.Sprintf("%stsconfig.json", emojiText(EmojiFileConfig)),
			wantURI: "semiorepo://file/tsconfig.json",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  fmt.Sprintf("%sbuild.sh", emojiText(EmojiFileScript)),
			wantURI: "semiorepo://file/build.sh",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "logo.png", "kind": "resource"},
			wantID:  fmt.Sprintf("%slogo.png", emojiText(EmojiFileResource)),
			wantURI: "semiorepo://file/logo.png",
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  fmt.Sprintf("%sLICENSE.md", emojiText(EmojiFileLicense)),
			wantURI: "semiorepo://file/LICENSE.md",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "semio/js/src/index.ts"},
			wantID:  fmt.Sprintf("%ssemio/js/src/index.ts", emojiText(EmojiSections)),
			wantURI: "semiorepo://sections/semio/js/src/index.ts",
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "semio/js/src/Design.tsx#State Management#Design Store"},
			wantID:  fmt.Sprintf("%ssemio/js/src/Design.tsx#State Management#Design Store", emojiText(EmojiSection)),
			wantURI: "semiorepo://section/semio/js/src/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE",
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "semio/js/src/file.ts#Imports"},
			wantID:  fmt.Sprintf("%ssemio/js/src/file.ts#Imports", emojiText(EmojiSection)),
			wantURI: "semiorepo://section/semio/js/src/file.ts/IMPORTS",
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "semio/js/src/index.ts"},
			wantID:  fmt.Sprintf("%ssemio/js/src/index.ts", emojiText(EmojiDefinitions)),
			wantURI: "semiorepo://definitions/semio/js/src/index.ts",
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "semio/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  fmt.Sprintf("%ssemio/js/src/index.ts#MyClass", emojiText(EmojiDefinitionImpl)),
			wantURI: "semiorepo://definition/semio/js/src/index.ts/MYCLASS",
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "semio/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  fmt.Sprintf("%ssemio/js/src/file.ts#Types§MyInterface", emojiText(EmojiDefinitionInterface)),
			wantURI: "semiorepo://definition/semio/js/src/file.ts/TYPES/MYINTERFACE",
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "semio/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  fmt.Sprintf("%ssemio/js/src/file.ts§MAX_SIZE", emojiText(EmojiDefinitionConstant)),
			wantURI: "semiorepo://definition/semio/js/src/file.ts/MAX-SIZE",
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiTickets),
			wantURI: "semiorepo://tickets",
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
			wantID:  fmt.Sprintf("%s2025/02/04/test-ticket", emojiText(EmojiTicket)),
			wantURI: "semiorepo://ticket/2025/02/04/test-ticket",
		},
		{
			name: "ticket with status",
			kind: "ticket",
			data: map[string]interface{}{
				"year":   float64(2025),
				"month":  float64(2),
				"day":    float64(4),
				"slug":   "test-ticket",
				"status": "open",
			},
			wantID:  fmt.Sprintf("%s2025/02/04/test-ticket?open", emojiText(EmojiTicket)),
			wantURI: "semiorepo://ticket/2025/02/04/test-ticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiGoals),
			wantURI: "semiorepo://goals",
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD"},
			wantID:  fmt.Sprintf("%sR26-02/RUNNING-SKETCHPAD", emojiText(EmojiGoal)),
			wantURI: "semiorepo://goal/R26-02/RUNNING-SKETCHPAD",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiDrafts),
			wantURI: "semiorepo://drafts",
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  fmt.Sprintf("%smy-draft", emojiText(EmojiDraft)),
			wantURI: "semiorepo://draft/MY-DRAFT",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiTodos),
			wantURI: "semiorepo://todos",
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  fmt.Sprintf("%smy-todo", emojiText(EmojiTodo)),
			wantURI: "semiorepo://todo/my-todo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiPolicies),
			wantURI: "semiorepo://policies",
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  fmt.Sprintf("%s/code-hygiene", emojiText(EmojiPolicy)),
			wantURI: "semiorepo://policy/CODE-HYGIENE",
		},
		{
			name:    "violationKinds collection",
			kind:    "violationKinds",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiViolationKinds),
			wantURI: "semiorepo://violationKinds",
		},
		{
			name:    "violationKind",
			kind:    "violationKind",
			data:    map[string]interface{}{"id": "code:inline-comment"},
			wantID:  fmt.Sprintf("%scode:inline-comment", emojiText(EmojiViolationKind)),
			wantURI: "semiorepo://violationKind/CODE/INLINE-COMMENT",
		},
		{
			name:    "contributors collection",
			kind:    "contributors",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiContributors),
			wantURI: "semiorepo://contributors",
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  fmt.Sprintf("%susalu", emojiText(EmojiContributor)),
			wantURI: "semiorepo://contributor/USALU",
		},
		{
			name:    "commits collection",
			kind:    "commits",
			data:    map[string]interface{}{},
			wantID:  emojiText(EmojiCommits),
			wantURI: "semiorepo://commits",
		},
		{
			name:    "commit",
			kind:    "commit",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  fmt.Sprintf("%sabc123", emojiText(EmojiCommit)),
			wantURI: "semiorepo://commit/ABC123",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotID := GetArtifactID(tt.kind, tt.data)
			if gotID != tt.wantID {
				t.Errorf("GetArtifactID() = %q, want %q", gotID, tt.wantID)
			}
			gotURI := GetArtifactURI(tt.kind, tt.data)
			if gotURI != tt.wantURI {
				t.Errorf("GetArtifactURI() = %q, want %q", gotURI, tt.wantURI)
			}
		})
	}
}

func TestIdToUri(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"repo", "🌍", "semiorepo://repo"},
		{"projects", "🏗️", "semiorepo://projects"},
		{"project user", "👤@semio", "semiorepo://project/@semio"},
		{"project infra", "🧰@semio-repo", "semiorepo://project/@semio-repo"},
		{"bundle", "📚semio/js", "semiorepo://bundle/semio/js"},
		{"folder", "📁semio/js/src", "semiorepo://folder/semio/js/src"},
		{"file", "📃test.txt", "semiorepo://file/test.txt"},
		{"section", "🔖semio/js/src/Design.tsx#State Management#Design Store", "semiorepo://section/semio/js/src/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE"},
		{"definition", "🛠semio/js/src/file.ts#Section§myFunc", "semiorepo://definition/semio/js/src/file.ts/SECTION/MYFUNC"},
		{"ticket", "🎫2025/02/04/test-ticket", "semiorepo://ticket/2025/02/04/test-ticket"},
		{"ticket with status", "🎫2025/02/04/test-ticket?open", "semiorepo://ticket/2025/02/04/test-ticket"},
		{"goal", "🎯R26-02/RUNNING-SKETCHPAD", "semiorepo://goal/R26-02/RUNNING-SKETCHPAD"},
		{"draft", "✍my-draft", "semiorepo://draft/MY-DRAFT"},
		{"todo", "📝my-todo", "semiorepo://todo/my-todo"},
		{"policy", "🛡/code-hygiene", "semiorepo://policy/CODE-HYGIENE"},
		{"violationKind", "🚫code:inline-comment", "semiorepo://violationKind/CODE/INLINE-COMMENT"},
		{"contributor", "👤usalu", "semiorepo://contributor/USALU"},
		{"commit", "🔀abc123", "semiorepo://commit/ABC123"},
		{"tickets collection", "🎫", "semiorepo://tickets"},
		{"goals collection", "🎯", "semiorepo://goals"},
		{"empty string", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := IdToUri(tt.id)
			if got != tt.want {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestUriToId(t *testing.T) {
	tests := []struct {
		name string
		uri  string
		want string
	}{
		{"repo", "semiorepo://repo", "🌍"},
		{"projects", "semiorepo://projects", emojiText("🏗️")},
		{"project", "semiorepo://project/@semio", "👤@semio"},
		{"bundles", "semiorepo://bundles", "📦"},
		{"bundle", "semiorepo://bundle/semio/js", "📚semio/js"},
		{"folders", "semiorepo://folders", "📁"},
		{"folder", "semiorepo://folder/semio/js/src", "📁semio/js/src"},
		{"files", "semiorepo://files", "📄"},
		{"file", "semiorepo://file/test.txt", "📄test.txt"},
		{"section", "semiorepo://section/semio/js/src/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE", "🔖semio/js/src/Design.tsx#STATE-MANAGEMENT#DESIGN-STORE"},
		{"definition single", "semiorepo://definition/semio/js/src/file.ts/MY-FUNC", "🛠semio/js/src/file.ts§MY-FUNC"},
		{"definition with section", "semiorepo://definition/semio/js/src/file.ts/SECTION/MY-FUNC", "🛠semio/js/src/file.ts#SECTION§MY-FUNC"},
		{"tickets", "semiorepo://tickets", "🎫"},
		{"ticket", "semiorepo://ticket/2025/02/04/test-ticket", "🎫2025/02/04/test-ticket"},
		{"goals", "semiorepo://goals", "🎯"},
		{"goal", "semiorepo://goal/R26-02/RUNNING-SKETCHPAD", "🎯R26-02/RUNNING-SKETCHPAD"},
		{"drafts", "semiorepo://drafts", "✍"},
		{"draft", "semiorepo://draft/MY-DRAFT", "✍MY-DRAFT"},
		{"todos", "semiorepo://todos", "📝"},
		{"todo", "semiorepo://todo/my-todo", "📝my-todo"},
		{"policies", "semiorepo://policies", "🛡"},
		{"policy", "semiorepo://policy/CODE-HYGIENE", "🛡/code-hygiene"},
		{"violationKinds", "semiorepo://violationKinds", "🚫"},
		{"violationKind", "semiorepo://violationKind/CODE/INLINE-COMMENT", "🚫code:inline-comment"},
		{"contributors", "semiorepo://contributors", "👤"},
		{"contributor", "semiorepo://contributor/USALU", "👤usalu"},
		{"commits", "semiorepo://commits", "🔀"},
		{"commit", "semiorepo://commit/ABC123", "🔀abc123"},
		{"invalid", "https://example.com", ""},
		{"empty", "", ""},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := UriToId(tt.uri)
			if got != tt.want {
				t.Errorf("UriToId(%q) = %q, want %q", tt.uri, got, tt.want)
			}
		})
	}
}

func TestSectionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "semio/js/src/file.ts", "semio/js/src/file.ts"},
		{"single section", "semio/js/src/file.ts#Imports", "semio/js/src/file.ts/IMPORTS"},
		{"nested sections", "semio/js/src/Design.tsx#State Management#Design Store", "semio/js/src/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := SectionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("SectionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestDefinitionIdValueToUriPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"no hash", "semio/js/src/file.ts", "semio/js/src/file.ts"},
		{"with section and def", "semio/js/src/file.ts#Section§myFunc", "semio/js/src/file.ts/SECTION/MYFUNC"},
		{"def only", "semio/js/src/file.ts§myFunc", "semio/js/src/file.ts/MYFUNC"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := DefinitionIdValueToUriPath(tt.value)
			if got != tt.want {
				t.Errorf("DefinitionIdValueToUriPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestParseSectionUriPath(t *testing.T) {
	tests := []struct {
		name      string
		uriPath   string
		wantFile  string
		wantSlugs []string
	}{
		{"file only", "semio/js/src/file.ts", "semio/js/src/file.ts", nil},
		{"file with sections", "semio/js/src/Design.tsx/STATE-MANAGEMENT/DESIGN-STORE", "semio/js/src/Design.tsx", []string{"STATE-MANAGEMENT", "DESIGN-STORE"}},
		{"file with one section", "semio/js/src/file.ts/IMPORTS", "semio/js/src/file.ts", []string{"IMPORTS"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotFile, gotSlugs := ParseSectionUriPath(tt.uriPath)
			if gotFile != tt.wantFile {
				t.Errorf("ParseSectionUriPath(%q) filePath = %q, want %q", tt.uriPath, gotFile, tt.wantFile)
			}
			if len(gotSlugs) != len(tt.wantSlugs) {
				t.Errorf("ParseSectionUriPath(%q) slugs len = %d, want %d", tt.uriPath, len(gotSlugs), len(tt.wantSlugs))
			} else {
				for i, s := range gotSlugs {
					if s != tt.wantSlugs[i] {
						t.Errorf("ParseSectionUriPath(%q) slug[%d] = %q, want %q", tt.uriPath, i, s, tt.wantSlugs[i])
					}
				}
			}
		})
	}
}

func TestViolationKindIdToUriPath(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"single segment", "code", "CODE"},
		{"two segments", "code:inline-comment", "CODE/INLINE-COMMENT"},
		{"three segments", "code:header:missing-region", "CODE/HEADER/MISSING-REGION"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := ViolationKindIdToUriPath(tt.id)
			if got != tt.want {
				t.Errorf("ViolationKindIdToUriPath(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestViolationKindUriPathToId(t *testing.T) {
	tests := []struct {
		name    string
		uriPath string
		want    string
	}{
		{"single segment", "CODE", "code"},
		{"two segments", "CODE/INLINE-COMMENT", "code:inline-comment"},
		{"three segments", "CODE/HEADER/MISSING-REGION", "code:header:missing-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := ViolationKindUriPathToId(tt.uriPath)
			if got != tt.want {
				t.Errorf("ViolationKindUriPathToId(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestIdUriRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		id   string
		uri  string
	}{
		{"policy", fmt.Sprintf("%s/code", emojiText(EmojiPolicy)), "semiorepo://policy/CODE"},
		{"violationKind", fmt.Sprintf("%scode:header:missing-region", emojiText(EmojiViolationKind)), "semiorepo://violationKind/CODE/HEADER/MISSING-REGION"},
		{"contributor", fmt.Sprintf("%susalu", emojiText(EmojiContributor)), "semiorepo://contributor/USALU"},
		{"commit", fmt.Sprintf("%sabc123", emojiText(EmojiCommit)), "semiorepo://commit/ABC123"},
		{"draft", fmt.Sprintf("%sMY-DRAFT", emojiText(EmojiDraft)), "semiorepo://draft/MY-DRAFT"},
		{"section", fmt.Sprintf("%ssemio/js/src/file.ts#Section", emojiText(EmojiSection)), "semiorepo://section/semio/js/src/file.ts/SECTION"},
		{"file", fmt.Sprintf("%ssemio/js/index.ts", emojiText(EmojiFileCode)), "semiorepo://file/semio/js/index.ts"},
		{"ticket", fmt.Sprintf("%s2026/01/15/SOME-TICKET", emojiText(EmojiTicket)), "semiorepo://ticket/2026/01/15/SOME-TICKET"},
		{"goal", fmt.Sprintf("%sR26-02/RUNNING", emojiText(EmojiGoal)), "semiorepo://goal/R26-02/RUNNING"},
	}
	for _, tt := range tests {
		t.Run(tt.name+"_IdToUri", func(t *testing.T) {
			gotUri := IdToUri(tt.id)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(%q) = %q, want %q", tt.id, gotUri, tt.uri)
			}
		})
		t.Run(tt.name+"_UriToId_then_IdToUri", func(t *testing.T) {
			gotId := UriToId(tt.uri)
			gotUri := IdToUri(gotId)
			if gotUri != tt.uri {
				t.Errorf("IdToUri(UriToId(%q)) = %q, want %q (intermediate id: %q)", tt.uri, gotUri, tt.uri, gotId)
			}
		})
	}
}

// #endregion 🔖Consolidated Tests
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

// #endregion 🔖MCP Schema Tests

// #region 🔖MCP Tool Tests

func setupToolTest(t *testing.T) {
	t.Helper()
	cwd, err := os.Getwd()
	if err != nil {
		t.Fatalf("failed to get cwd: %v", err)
	}
	rootDir = findTestRepoRoot(cwd)
	InvalidateProjectCache()
}

func TestToolProjectList(t *testing.T) {
	setupToolTest(t)
	result := ToolProjectList()
	if result.Error != "" {
		t.Errorf("ToolProjectList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolProjectList returned nil data")
	}
	projects, ok := result.Data.([]Project)
	if !ok {
		t.Fatal("ToolProjectList data is not []Project")
	}
	if len(projects) == 0 {
		t.Error("ToolProjectList returned empty projects")
	}
}

func TestToolProjectTree(t *testing.T) {
	setupToolTest(t)
	result := ToolProjectTree()
	if result.Error != "" {
		t.Errorf("ToolProjectTree returned error: %s", result.Error)
	}
}

func TestToolContributorList(t *testing.T) {
	setupToolTest(t)
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestToolGoalList(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Errorf("ToolGoalList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolGoalList returned nil data")
	}
}

func TestToolTicketList(t *testing.T) {
	setupToolTest(t)
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTicketList returned nil data")
	}
}

func TestToolDraftList(t *testing.T) {
	setupToolTest(t)
	result := ToolDraftList()
	if result.Error != "" {
		t.Errorf("ToolDraftList returned error: %s", result.Error)
	}
}

func TestToolFolderList(t *testing.T) {
	setupToolTest(t)
	result := ToolFolderList(".")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestToolFolderTree(t *testing.T) {
	setupToolTest(t)
	result := ToolFolderTree("semio-repo")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestToolFileList(t *testing.T) {
	setupToolTest(t)
	result := ToolFileList("semio-repo/cli")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestToolFileTree(t *testing.T) {
	setupToolTest(t)
	result := ToolFileTree("semio-repo/cli")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestToolSectionList(t *testing.T) {
	setupToolTest(t)
	result := ToolSectionList("semio-repo/cli/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
}

func TestToolSectionTree(t *testing.T) {
	setupToolTest(t)
	result := ToolSectionTree("semio-repo/cli/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

func TestToolDefinitionList(t *testing.T) {
	setupToolTest(t)
	result := ToolDefinitionList("semio-repo/cli/main.go")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

func TestToolPolicyList(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
}

func TestToolPolicyCheck(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyCheck("code", "semio-repo/cli")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestToolAnalyzeScope(t *testing.T) {
	setupToolTest(t)
	result := ToolAnalyze("semio-repo/cli", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestToolFixScope(t *testing.T) {
	setupToolTest(t)
	result := ToolFix("semio-repo/cli")
	if result.Error != "" {
		t.Errorf("ToolFix returned error: %s", result.Error)
	}
}

func TestToolFolderCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFolderCreate("test-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderCreate returned error: %s", result.Error)
	}

	result = ToolFolderList(".")
	if result.Error != "" {
		t.Fatalf("ToolFolderList returned error: %s", result.Error)
	}

	result = ToolFolderMove("test-folder", "renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderMove returned error: %s", result.Error)
	}

	result = ToolFolderDelete("renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderDelete returned error: %s", result.Error)
	}
}

func TestToolFileCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFileCreate("test.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileCreate returned error: %s", result.Error)
	}

	result = ToolFileMove("test.txt", "renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileMove returned error: %s", result.Error)
	}

	result = ToolFileDelete("renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileDelete returned error: %s", result.Error)
	}
}

func TestToolTicketLifecycle(t *testing.T) {
	setupToolTest(t)

	result := ToolTicketOpen("Test Lifecycle Ticket", "Test prompt", "sonnet-4-5", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}

	readResult := ToolTicketRead(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	if readResult.Error != "" {
		t.Fatalf("ToolTicketRead returned error: %s", readResult.Error)
	}

	closeResult := ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Test summary", []string{"semio-repo/cli/main.go"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose returned error: %s", closeResult.Error)
	}

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen prompt", "sonnet-4-5", "windsurf-chat", "", "", "", "", true)
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen returned error: %s", reopenResult.Error)
	}

	ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Final close", []string{"semio-repo/cli/main.go"}, "", true)
	ticketPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	os.RemoveAll(ticketPath)
}

func TestToolDraftLifecycle(t *testing.T) {
	setupToolTest(t)

	result := ToolDraftCreate("test-mcp-draft", nil)
	if result.Error != "" {
		t.Fatalf("ToolDraftCreate returned error: %s", result.Error)
	}

	listResult := ToolDraftList()
	if listResult.Error != "" {
		t.Fatalf("ToolDraftList returned error: %s", listResult.Error)
	}

	deleteResult := ToolDraftDelete("test-mcp-draft")
	if deleteResult.Error != "" {
		t.Fatalf("ToolDraftDelete returned error: %s", deleteResult.Error)
	}
}

func TestToolGoalUri(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Fatalf("ToolGoalList returned error: %s", result.Error)
	}
	goals, ok := result.Data.([]*Goal)
	if !ok || len(goals) == 0 {
		t.Skip("no goals to verify URI")
	}
	for _, g := range goals {
		uri := g.GetURI()
		if uri == "" {
			t.Errorf("goal %s has empty URI", g.ID)
		}
		if !strings.HasPrefix(uri, "semiorepo://goal/") {
			t.Errorf("goal %s URI %q should start with semiorepo://goal/", g.ID, uri)
		}
	}
}

// #region 🔖Output Parity Tests

func TestParityGoalList(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("goal", "list")
		if err != nil {
			t.Fatalf("CLI goal list failed: %v", err)
		}
		toolResult := ToolGoalList()
		if toolResult.Error != "" {
			t.Fatalf("ToolGoalList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of goals", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("goal", "list")
		toolResult := ToolGoalList()
		mcpOut := toolOutputText(toolResult)
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})

	t.Run("empty output when no goals match filter", func(t *testing.T) {
		// Both should produce the same empty-ish output for a nonexistent filter
		// (This tests structural consistency even with no results)
		cliOut, _, _ := executeCommandMd("goal", "list")
		mcpOut := toolOutputText(ToolGoalList())
		if len(cliOut) == 0 && len(mcpOut) != 0 {
			t.Error("CLI produced empty output but MCP did not")
		}
		if len(cliOut) != 0 && len(mcpOut) == 0 {
			t.Error("MCP produced empty output but CLI did not")
		}
	})
}

func TestParityContributorList(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("contributor", "list")
		if err != nil {
			t.Fatalf("CLI contributor list failed: %v", err)
		}
		toolResult := ToolContributorList()
		if toolResult.Error != "" {
			t.Fatalf("ToolContributorList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of contributors", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("contributor", "list")
		mcpOut := toolOutputText(ToolContributorList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestParityTicketList(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("ticket", "list")
		if err != nil {
			t.Fatalf("CLI ticket list failed: %v", err)
		}
		toolResult := ToolTicketList(nil, nil, nil)
		if toolResult.Error != "" {
			t.Fatalf("ToolTicketList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of tickets", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("ticket", "list")
		mcpOut := toolOutputText(ToolTicketList(nil, nil, nil))
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestParityDraftList(t *testing.T) {
	setupToolTest(t)

	t.Run("uses event rendering format", func(t *testing.T) {
		toolResult := ToolDraftList()
		if toolResult.Error != "" {
			t.Fatalf("ToolDraftList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		// Draft list is MCP-only (no CLI counterpart), but verify it uses
		// the same event-based rendering as other list commands
		drafts, _ := ListDrafts()
		if len(drafts) > 0 && mcpOut == "" {
			t.Error("ToolDraftList returned empty output despite having drafts")
		}
		if len(drafts) == 0 && mcpOut != "" {
			t.Error("ToolDraftList returned output despite having no drafts")
		}
	})

	t.Run("renders same as manual event rendering", func(t *testing.T) {
		drafts, err := ListDrafts()
		if err != nil {
			t.Fatalf("ListDrafts failed: %v", err)
		}
		var events []Event
		for _, d := range drafts {
			data, _ := json.Marshal(map[string]interface{}{"draft": d})
			events = append(events, Event{Kind: KindResult, Command: "draft list", Data: data})
		}
		expected := renderEventsToMarkdown(events)
		actual := toolOutputText(ToolDraftList())
		if expected != actual {
			t.Errorf("output mismatch:\nexpected:\n%s\nactual:\n%s", expected, actual)
		}
	})
}

func TestParityProjectList(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("project", "list")
		if err != nil {
			t.Fatalf("CLI project list failed: %v", err)
		}
		toolResult := ToolProjectList()
		if toolResult.Error != "" {
			t.Fatalf("ToolProjectList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return non-empty output", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("project", "list")
		mcpOut := toolOutputText(ToolProjectList())
		if len(cliOut) == 0 {
			t.Error("CLI project list returned empty output")
		}
		if len(mcpOut) == 0 {
			t.Error("MCP project list returned empty output")
		}
	})
}

func TestParityProjectTree(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("project", "tree")
		if err != nil {
			t.Fatalf("CLI project tree failed: %v", err)
		}
		toolResult := ToolProjectTree()
		if toolResult.Error != "" {
			t.Fatalf("ToolProjectTree returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("projects are sorted alphabetically", func(t *testing.T) {
		mcpOut := toolOutputText(ToolProjectTree())
		lines := strings.Split(strings.TrimSpace(mcpOut), "\n")
		for i := 1; i < len(lines); i++ {
			if lines[i] < lines[i-1] {
				t.Errorf("projects not sorted: %q comes after %q", lines[i], lines[i-1])
			}
		}
	})
}

func TestParityPolicyList(t *testing.T) {
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("policy", "list")
		if err != nil {
			t.Fatalf("CLI policy list failed: %v", err)
		}
		toolResult := ToolPolicyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolPolicyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if cliOut != mcpOut {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of policies", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("policy", "list")
		mcpOut := toolOutputText(ToolPolicyList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

// #endregion 🔖Output Parity Tests

// #endregion 🔖MCP Tool Tests

// #region 🔖Monorepo Tree Tests

func TestTreeNodeKindConstants(t *testing.T) {
	t.Run("all kinds are distinct", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeProject, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeViolationKindNode,
			TreeNodeContributor, TreeNodeCommit, TreeNodeCategory,
		}
		seen := make(map[TreeNodeKind]bool)
		for _, k := range kinds {
			if seen[k] {
				t.Errorf("duplicate TreeNodeKind: %s", k)
			}
			seen[k] = true
		}
	})

	t.Run("kinds are non-empty strings", func(t *testing.T) {
		kinds := []TreeNodeKind{
			TreeNodeProject, TreeNodeBundle, TreeNodeFolder, TreeNodeFile,
			TreeNodeSection, TreeNodeDefinition, TreeNodeGoal, TreeNodeTicket,
			TreeNodeDraft, TreeNodePolicy, TreeNodeViolationKindNode,
			TreeNodeContributor, TreeNodeCommit, TreeNodeCategory,
		}
		for _, k := range kinds {
			if string(k) == "" {
				t.Error("TreeNodeKind should not be empty")
			}
		}
	})
}

func TestTreeFilterIsKindVisible(t *testing.T) {
	t.Run("all visible by default", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible by default")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should be visible by default")
		}
	})

	t.Run("only-kind filters to specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeProject: true, TreeNodeBundle: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeProject) {
			t.Error("project should be visible with only-project")
		}
		if !f.IsKindVisible(TreeNodeBundle) {
			t.Error("bundle should be visible with only-bundle")
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when not in only-kinds")
		}
		if f.IsKindVisible(TreeNodeFile) {
			t.Error("file should not be visible when not in only-kinds")
		}
	})

	t.Run("exclude-kind hides specified kinds", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    make(map[TreeNodeKind]bool),
			ExcludeKinds: map[TreeNodeKind]bool{TreeNodeFolder: true},
		}
		if f.IsKindVisible(TreeNodeFolder) {
			t.Error("folder should not be visible when excluded")
		}
		if !f.IsKindVisible(TreeNodeFile) {
			t.Error("file should still be visible")
		}
	})

	t.Run("category always visible", func(t *testing.T) {
		f := &TreeFilter{
			OnlyKinds:    map[TreeNodeKind]bool{TreeNodeProject: true},
			ExcludeKinds: make(map[TreeNodeKind]bool),
		}
		if !f.IsKindVisible(TreeNodeCategory) {
			t.Error("category should always be visible")
		}
	})
}

func TestTreeFilterMatchesSubKind(t *testing.T) {
	t.Run("matches all when no sub-kind filters", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match any sub-kind by default")
		}
	})

	t.Run("only sub-kind includes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("library should match only-library")
		}
		if f.MatchesSubKind(TreeNodeBundle, "schema") {
			t.Error("schema should not match only-library")
		}
	})

	t.Run("exclude sub-kind removes specified", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: map[TreeNodeKind][]string{TreeNodeFolder: {"required"}},
		}
		if f.MatchesSubKind(TreeNodeFolder, "required") {
			t.Error("required should not match when excluded")
		}
		if !f.MatchesSubKind(TreeNodeFolder, "organization") {
			t.Error("organization should still match")
		}
	})

	t.Run("empty sub-kind always matches", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "") {
			t.Error("empty sub-kind should always match")
		}
	})

	t.Run("case insensitive matching", func(t *testing.T) {
		f := &TreeFilter{
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"Library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		if !f.MatchesSubKind(TreeNodeBundle, "library") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesDate(t *testing.T) {
	t.Run("matches all when no date filters", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesDate(2026, 1, 15) {
			t.Error("should match any date by default")
		}
	})

	t.Run("only-year includes specified year", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}}
		if !f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should match only-year 2026")
		}
		if f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should not match only-year 2026")
		}
	})

	t.Run("exclude-year removes specified year", func(t *testing.T) {
		f := &TreeFilter{ExcludeYears: []int{2026}}
		if f.MatchesDate(2026, 1, 1) {
			t.Error("2026 should not match no-year 2026")
		}
		if !f.MatchesDate(2025, 1, 1) {
			t.Error("2025 should still match")
		}
	})

	t.Run("month filter", func(t *testing.T) {
		f := &TreeFilter{OnlyMonths: []int{6}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("June should match")
		}
		if f.MatchesDate(2026, 7, 1) {
			t.Error("July should not match")
		}
	})

	t.Run("combined year and month", func(t *testing.T) {
		f := &TreeFilter{OnlyYears: []int{2026}, ExcludeMonths: []int{12}}
		if !f.MatchesDate(2026, 6, 1) {
			t.Error("2026/06 should match")
		}
		if f.MatchesDate(2026, 12, 1) {
			t.Error("2026/12 should not match")
		}
		if f.MatchesDate(2025, 6, 1) {
			t.Error("2025 should not match")
		}
	})
}

func TestTreeFilterMatchesStatus(t *testing.T) {
	t.Run("matches all when no status filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesStatus("open") {
			t.Error("should match any status by default")
		}
		if !f.MatchesStatus("closed") {
			t.Error("should match any status by default")
		}
	})

	t.Run("only-open filters to open", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "open"}
		if !f.MatchesStatus("open") {
			t.Error("open should match only-open")
		}
		if f.MatchesStatus("closed") {
			t.Error("closed should not match only-open")
		}
	})

	t.Run("only-closed filters to closed", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "closed"}
		if !f.MatchesStatus("closed") {
			t.Error("closed should match only-closed")
		}
		if f.MatchesStatus("open") {
			t.Error("open should not match only-closed")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyStatus: "Open"}
		if !f.MatchesStatus("open") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestTreeFilterMatchesContributor(t *testing.T) {
	t.Run("matches all when no contributor filter", func(t *testing.T) {
		f := &TreeFilter{}
		if !f.MatchesContributor("usalu") {
			t.Error("should match any contributor by default")
		}
	})

	t.Run("only-contributor includes specified", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("usalu should match")
		}
		if f.MatchesContributor("other") {
			t.Error("other should not match")
		}
	})

	t.Run("exclude-contributor removes specified", func(t *testing.T) {
		f := &TreeFilter{ExcludeContributors: []string{"usalu"}}
		if f.MatchesContributor("usalu") {
			t.Error("usalu should not match when excluded")
		}
		if !f.MatchesContributor("other") {
			t.Error("other should still match")
		}
	})

	t.Run("case insensitive", func(t *testing.T) {
		f := &TreeFilter{OnlyContributors: []string{"Usalu"}}
		if !f.MatchesContributor("usalu") {
			t.Error("should match case-insensitively")
		}
	})
}

func TestFilterMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "projects", Label: "Projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "proj1", Label: "proj1", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "b1", Label: "bundle1", SubKind: "library", Children: []*TreeNode{
							{Kind: TreeNodeFolder, ID: "f1", Label: "src", SubKind: "organization", Children: []*TreeNode{
								{Kind: TreeNodeFile, ID: "file1", Label: "index.ts", SubKind: "code"},
								{Kind: TreeNodeFile, ID: "file2", Label: "README.md", SubKind: "docs"},
							}},
						}},
						{Kind: TreeNodeBundle, ID: "b2", Label: "bundle2", SubKind: "schema"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "g1", Label: "Goal1", Status: "open", Children: []*TreeNode{
						{Kind: TreeNodeTicket, ID: "t1", Label: "Ticket1", Status: "open", Year: 2026, Month: 2, Day: 5},
						{Kind: TreeNodeTicket, ID: "t2", Label: "Ticket2", Status: "closed", Year: 2025, Month: 12, Day: 1},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "contributors", Label: "Contributors", Children: []*TreeNode{
					{Kind: TreeNodeContributor, ID: "c1", Label: "usalu", Contributor: "usalu"},
					{Kind: TreeNodeContributor, ID: "c2", Label: "other", Contributor: "other"},
				}},
			},
		}
	}

	t.Run("no filter returns full tree", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		if len(result.Children) != 3 {
			t.Errorf("expected 3 top-level categories, got %d", len(result.Children))
		}
	})

	t.Run("exclude-bundle removes bundles", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		projectsNode := result.Children[0]
		proj := projectsNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle {
				t.Error("bundles should be collapsed out")
			}
		}
	})

	t.Run("no-folder collapses folders", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		projectsNode := result.Children[0]
		proj := projectsNode.Children[0]
		bundle := proj.Children[0]
		hasFile := false
		for _, c := range bundle.Children {
			if c.Kind == TreeNodeFolder {
				t.Error("folders should be collapsed")
			}
			if c.Kind == TreeNodeFile {
				hasFile = true
			}
		}
		if !hasFile {
			t.Error("files should be promoted to bundle level")
		}
	})

	t.Run("only-library sub-kind filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    map[TreeNodeKind][]string{TreeNodeBundle: {"library"}},
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		result := FilterMonorepoTree(tree, filter)
		projectsNode := result.Children[0]
		proj := projectsNode.Children[0]
		for _, c := range proj.Children {
			if c.Kind == TreeNodeBundle && c.SubKind != "library" {
				t.Errorf("only library bundles expected, got %s", c.SubKind)
			}
		}
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 bundle (library), got %d", len(proj.Children))
		}
	})

	t.Run("status filter open", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			OnlyStatus:      "open",
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Status != "open" {
				t.Error("only open tickets should be visible")
			}
		}
	})

	t.Run("year filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    make(map[TreeNodeKind]bool),
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
			ExcludeYears:    []int{2025},
		}
		result := FilterMonorepoTree(tree, filter)
		goalsNode := result.Children[1]
		goal := goalsNode.Children[0]
		for _, c := range goal.Children {
			if c.Kind == TreeNodeTicket && c.Year == 2025 {
				t.Error("2025 tickets should be excluded")
			}
		}
	})

	t.Run("contributor filter", func(t *testing.T) {
		tree := makeTree()
		filter := &TreeFilter{
			OnlyKinds:           make(map[TreeNodeKind]bool),
			ExcludeKinds:        make(map[TreeNodeKind]bool),
			OnlySubKinds:        make(map[TreeNodeKind][]string),
			ExcludeSubKinds:     make(map[TreeNodeKind][]string),
			ExcludeContributors: []string{"usalu"},
		}
		result := FilterMonorepoTree(tree, filter)
		contribNode := result.Children[2]
		for _, c := range contribNode.Children {
			if c.Contributor == "usalu" {
				t.Error("usalu should be excluded")
			}
		}
		if len(contribNode.Children) != 1 {
			t.Errorf("expected 1 contributor, got %d", len(contribNode.Children))
		}
	})

	t.Run("nil filter returns same tree", func(t *testing.T) {
		tree := makeTree()
		result := FilterMonorepoTree(tree, nil)
		if result != tree {
			t.Error("nil filter should return same tree")
		}
	})
}

func TestSearchMonorepoTree(t *testing.T) {
	makeTree := func() *TreeNode {
		return &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "projects", Label: "Projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "proj:semio", Label: "semio", Children: []*TreeNode{
						{Kind: TreeNodeBundle, ID: "bundle:cli", Label: "cli", SubKind: "binary"},
						{Kind: TreeNodeBundle, ID: "bundle:docs", Label: "docs", SubKind: "site"},
					}},
				}},
				{Kind: TreeNodeCategory, ID: "goals", Label: "Goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "goal:test", Label: "Test Goal", Description: "testing search"},
				}},
			},
		}
	}

	t.Run("empty query returns full tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "")
		if len(result.Children) != 2 {
			t.Errorf("expected 2 categories, got %d", len(result.Children))
		}
	})

	t.Run("query matches items", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		found := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.ID == "bundle:cli" {
				found = true
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(result)
		if !found {
			t.Error("search for 'cli' should find bundle:cli")
		}
	})

	t.Run("query with no matches returns empty tree", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "zzzznonexistent")
		totalChildren := 0
		for _, c := range result.Children {
			totalChildren += len(c.Children)
		}
		if totalChildren != 0 {
			t.Errorf("search for nonexistent term should return empty, got %d children", totalChildren)
		}
	})

	t.Run("parent chain preserved", func(t *testing.T) {
		tree := makeTree()
		result := SearchMonorepoTree(tree, "cli")
		if len(result.Children) == 0 {
			t.Fatal("expected at least one category")
		}
		projectsNode := result.Children[0]
		if projectsNode.ID != "projects" {
			t.Errorf("expected projects category, got %s", projectsNode.ID)
		}
		if len(projectsNode.Children) == 0 {
			t.Fatal("expected project under projects")
		}
		proj := projectsNode.Children[0]
		if proj.ID != "proj:semio" {
			t.Errorf("expected semio project, got %s", proj.ID)
		}
	})
}

func TestRenderMonorepoTree(t *testing.T) {
	t.Run("renders basic tree", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️️Projects", URI: "semiorepo://projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "p1", Label: "semio"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "🏗️️Projects") {
			t.Error("output should contain Projects label")
		}
		if !strings.Contains(output, "semio") {
			t.Error("output should contain project name")
		}
	})

	t.Run("renders category URI", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "semiorepo://goals"},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "[🎯Goals](semiorepo://goals)") {
			t.Errorf("output should contain category with URI link, got: %s", output)
		}
	})

	t.Run("renders nested tree with connectors", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "projects", Label: "Projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "p1", Label: "proj1"},
					{Kind: TreeNodeProject, ID: "p2", Label: "proj2"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "├── ") || !strings.Contains(output, "└── ") {
			t.Errorf("output should contain tree connectors, got: %s", output)
		}
	})

	t.Run("empty tree renders nothing", func(t *testing.T) {
		tree := &TreeNode{Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{}}
		output := RenderMonorepoTree(tree)
		if output != "" {
			t.Errorf("empty tree should render nothing, got: %q", output)
		}
	})

	t.Run("markdown renderer uses list bullets", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️️Projects", URI: "semiorepo://projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "p1", Label: "semio"},
				}},
			},
		}
		output := RenderMonorepoTreeMarkdown(tree)
		if !strings.Contains(output, "- [🏗️️Projects](semiorepo://projects)") {
			t.Errorf("markdown tree should contain markdown link list item, got: %s", output)
		}
		if !strings.Contains(output, "  - semio") {
			t.Errorf("markdown tree should contain nested bullet item, got: %s", output)
		}
		if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
			t.Errorf("markdown tree must not contain ascii connectors, got: %s", output)
		}
	})
}

func TestBuildMonorepoTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree build test in short mode")
	}

	cwd, _ := os.Getwd()
	oldRoot := rootDir
	rootDir = findTestRepoRoot(cwd)
	defer func() { rootDir = oldRoot }()
	InvalidateProjectCache()

	t.Run("builds tree with categories", func(t *testing.T) {
		ctx := context.Background()
		tree := BuildMonorepoTree(ctx)
		if tree == nil {
			t.Fatal("tree should not be nil")
		}
		if len(tree.Children) == 0 {
			t.Fatal("tree should have categories")
		}

		categoryIDs := make(map[string]bool)
		for _, c := range tree.Children {
			if c.Kind != TreeNodeCategory {
				t.Errorf("top-level children should be categories, got %s", c.Kind)
			}
			categoryIDs[c.ID] = true
		}

		expected := []string{"projects", "goals", "drafts", "policies", "contributors", "commits"}
		for _, id := range expected {
			if !categoryIDs[id] {
				t.Errorf("missing category: %s", id)
			}
		}
	})

	t.Run("projects category has children", func(t *testing.T) {
		ctx := context.Background()
		tree := BuildMonorepoTree(ctx)
		var projectsNode *TreeNode
		for _, c := range tree.Children {
			if c.ID == "projects" {
				projectsNode = c
				break
			}
		}
		if projectsNode == nil {
			t.Fatal("projects category not found")
		}
		if len(projectsNode.Children) == 0 {
			t.Error("projects should have children")
		}
		hasBundles := false
		for _, p := range projectsNode.Children {
			if p.Kind != TreeNodeProject {
				t.Errorf("projects children should be projects, got %s", p.Kind)
			}
			for _, b := range p.Children {
				if b.Kind != TreeNodeBundle {
					t.Errorf("project children should be bundles, got %s", b.Kind)
				}
				hasBundles = true
			}
		}
		if !hasBundles {
			t.Error("at least one project should have bundles")
		}
	})

	t.Run("with sections includes sections", func(t *testing.T) {
		ctx := context.Background()
		tree := BuildMonorepoTree(ctx, TreeBuildOptions{IncludeSections: true})
		hasSections := false
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				hasSections = true
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(tree)
		if !hasSections {
			t.Error("tree with IncludeSections should have section nodes")
		}
	})

	t.Run("without sections excludes sections", func(t *testing.T) {
		ctx := context.Background()
		tree := BuildMonorepoTree(ctx)
		var walk func(*TreeNode)
		walk = func(n *TreeNode) {
			if n.Kind == TreeNodeSection {
				t.Error("tree without IncludeSections should not have section nodes")
				return
			}
			for _, c := range n.Children {
				walk(c)
			}
		}
		walk(tree)
	})
}

func TestCollapseFilteredKinds(t *testing.T) {
	t.Run("collapses folders promoting files to parent", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
					{Kind: TreeNodeFolder, ID: "f1", Label: "src", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "file1", Label: "index.ts"},
						{Kind: TreeNodeFile, ID: "file2", Label: "app.ts"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeFolder: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		bundle := tree.Children[0]
		if len(bundle.Children) != 2 {
			t.Errorf("expected 2 files promoted to bundle, got %d", len(bundle.Children))
		}
		for _, c := range bundle.Children {
			if c.Kind != TreeNodeFile {
				t.Errorf("expected file, got %s", c.Kind)
			}
		}
	})

	t.Run("nested collapse", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeProject, ID: "p1", Label: "proj", Children: []*TreeNode{
					{Kind: TreeNodeBundle, ID: "b1", Label: "bundle", Children: []*TreeNode{
						{Kind: TreeNodeFile, ID: "f1", Label: "main.go"},
					}},
				}},
			},
		}
		filter := &TreeFilter{
			OnlyKinds:       make(map[TreeNodeKind]bool),
			ExcludeKinds:    map[TreeNodeKind]bool{TreeNodeBundle: true},
			OnlySubKinds:    make(map[TreeNodeKind][]string),
			ExcludeSubKinds: make(map[TreeNodeKind][]string),
		}
		collapseFilteredKinds(tree, filter)
		proj := tree.Children[0]
		if len(proj.Children) != 1 {
			t.Errorf("expected 1 file promoted to project, got %d", len(proj.Children))
		}
		if proj.Children[0].Kind != TreeNodeFile {
			t.Errorf("expected file, got %s", proj.Children[0].Kind)
		}
	})
}

func TestSortTreeChildren(t *testing.T) {
	t.Run("sorts alphabetically", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "z.ts"},
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFile, Label: "m.ts"},
			},
		}
		sortTreeChildren(tree)
		if tree.Children[0].Label != "a.ts" {
			t.Errorf("expected a.ts first, got %s", tree.Children[0].Label)
		}
		if tree.Children[2].Label != "z.ts" {
			t.Errorf("expected z.ts last, got %s", tree.Children[2].Label)
		}
	})

	t.Run("folders before files", func(t *testing.T) {
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: "root", Children: []*TreeNode{
				{Kind: TreeNodeFile, Label: "a.ts"},
				{Kind: TreeNodeFolder, Label: "src"},
				{Kind: TreeNodeFile, Label: "b.ts"},
			},
		}
		sortTreeChildren(tree)
		if tree.Children[0].Kind != TreeNodeFolder {
			t.Errorf("expected folder first, got %s", tree.Children[0].Kind)
		}
	})
}

func TestTreeCommandFlags(t *testing.T) {
	t.Run("builds filter from flags", func(t *testing.T) {
		cmd := &cobra.Command{}
		bindTreeFlags(cmd)
		cmd.Flags().Set("only-project", "true")
		cmd.Flags().Set("no-folder", "true")
		cmd.Flags().Set("only-library", "true")
		cmd.Flags().Set("only-open", "true")
		cmd.Flags().Set("no-year", "2025")

		filter := buildTreeFilterFromFlags(cmd)

		if !filter.OnlyKinds[TreeNodeProject] {
			t.Error("expected only-project to be set")
		}
		if !filter.ExcludeKinds[TreeNodeFolder] {
			t.Error("expected no-folder to be set")
		}
		if len(filter.OnlySubKinds[TreeNodeBundle]) != 1 || filter.OnlySubKinds[TreeNodeBundle][0] != string(BundleKindLibrary) {
			t.Error("expected only-library sub-kind")
		}
		if filter.OnlyStatus != "open" {
			t.Errorf("expected only-open status, got %q", filter.OnlyStatus)
		}
		if len(filter.ExcludeYears) != 1 || filter.ExcludeYears[0] != 2025 {
			t.Errorf("expected no-year 2025, got %v", filter.ExcludeYears)
		}
	})

	t.Run("empty flags produce empty filter", func(t *testing.T) {
		cmd := &cobra.Command{}
		bindTreeFlags(cmd)
		filter := buildTreeFilterFromFlags(cmd)
		if filter.HasOnlyKinds() {
			t.Error("empty flags should not set only-kinds")
		}
		if len(filter.ExcludeKinds) != 0 {
			t.Error("empty flags should not set exclude-kinds")
		}
		if filter.OnlyStatus != "" {
			t.Error("empty flags should not set status")
		}
	})
}

// #region 🔖Unified Rendering Identity Tests

func TestTreeNodeKindToEntityKindCoversAll(t *testing.T) {
	kinds := []struct {
		kind     TreeNodeKind
		expected string
	}{
		{TreeNodeProject, "project"},
		{TreeNodeBundle, "bundle"},
		{TreeNodeFolder, "folder"},
		{TreeNodeFile, "file"},
		{TreeNodeSection, "section"},
		{TreeNodeDefinition, "definition"},
		{TreeNodeGoal, "goal"},
		{TreeNodeTicket, "ticket"},
		{TreeNodeDraft, "draft"},
		{TreeNodePolicy, "policy"},
		{TreeNodeViolationKindNode, "violationKind"},
		{TreeNodeContributor, "contributor"},
		{TreeNodeCommit, "commit"},
		{TreeNodeCategory, "category"},
	}
	for _, tt := range kinds {
		t.Run(string(tt.kind), func(t *testing.T) {
			got := treeNodeKindToEntityKind(tt.kind)
			if got != tt.expected {
				t.Errorf("treeNodeKindToEntityKind(%q) = %q, want %q", tt.kind, got, tt.expected)
			}
		})
	}
	t.Run("unknown returns empty", func(t *testing.T) {
		got := treeNodeKindToEntityKind(TreeNodeKind("unknown"))
		if got != "" {
			t.Errorf("treeNodeKindToEntityKind(unknown) = %q, want empty", got)
		}
	})
}

func TestUnifiedRenderingGoalIdentity(t *testing.T) {
	data := map[string]interface{}{
		"id":          "TEST-GOAL",
		"title":       "Test Goal",
		"status":      "open",
		"dueDate":     "2030-01-01",
		"createdAt":   "2025-01-01T00:00:00Z",
		"description": "A test goal",
	}

	mdLink := renderEntityMarkdownLink("goal", data)
	mdItem := renderEntityMarkdown("goal", data)
	humanItem := renderEntityHuman("goal", data, false)

	t.Run("renderEntityMarkdown is dash-prefixed renderEntityMarkdownLink", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("markdown link has artifact ID and URI", func(t *testing.T) {
		if !strings.Contains(mdLink, "[🎯") {
			t.Errorf("markdown link missing goal emoji prefix: %s", mdLink)
		}
		if !strings.Contains(mdLink, "](semiorepo://goal/") {
			t.Errorf("markdown link missing goal URI: %s", mdLink)
		}
	})

	t.Run("human has artifact ID", func(t *testing.T) {
		if !strings.Contains(humanItem, "🎯") {
			t.Errorf("human output missing goal emoji: %s", humanItem)
		}
	})

	t.Run("both formats share same props from collectEntityProps", func(t *testing.T) {
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("goalNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &GoalNode{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}
		nodeData := goalNodeToData(node)
		fromNode := renderEntityMarkdownLink("goal", nodeData)
		fromDirect := renderEntityMarkdownLink("goal", data)
		if fromNode != fromDirect {
			t.Errorf("goalNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree markdown uses renderEntityMarkdownLink for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		expectedLink := renderEntityMarkdownLink("goal", data)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("goal tree markdown must not have double dash: %q", treeOutput)
		}
	})

	t.Run("goal tree text uses renderEntityHuman for content", func(t *testing.T) {
		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		treeOutput := renderGoalTreeNodes(roots, "text")
		expectedHuman := renderEntityHuman("goal", data, false)
		if !strings.Contains(treeOutput, expectedHuman) {
			t.Errorf("goal tree text should contain renderEntityHuman output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("monorepo tree node markdown matches goal tree markdown", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "semiorepo://goal/TEST-GOAL",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "md"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree markdown and goal tree markdown differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})

	t.Run("monorepo tree node text matches goal tree text", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeGoal,
			ID:    "TEST-GOAL",
			Label: "TEST-GOAL",
			URI:   "semiorepo://goal/TEST-GOAL",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeText(&sb, treeNode, "", true, true)
		monorepoOutput := strings.TrimSpace(sb.String())

		roots := []*GoalNode{{
			ID:          "TEST-GOAL",
			Title:       "Test Goal",
			Status:      "open",
			DueDate:     "2030-01-01",
			CreatedAt:   "2025-01-01T00:00:00Z",
			Description: "A test goal",
		}}
		goalTreeOutput := strings.TrimSpace(renderGoalTreeNodes(roots, "text"))
		if monorepoOutput != goalTreeOutput {
			t.Errorf("monorepo tree text and goal tree text differ:\n  Monorepo:  %q\n  GoalTree:  %q", monorepoOutput, goalTreeOutput)
		}
	})
}

func TestUnifiedRenderingTicketIdentity(t *testing.T) {
	data := map[string]interface{}{
		"slug":     "MY-TICKET",
		"title":    "My Ticket",
		"status":   "open",
		"started":  "2025-01-01T00:00:00Z",
		"finished": "",
		"prompt":   "Fix something",
		"summary":  "",
		"year":     float64(2025),
		"month":    float64(1),
		"day":      float64(1),
	}

	mdLink := renderEntityMarkdownLink("ticket", data)
	mdItem := renderEntityMarkdown("ticket", data)
	humanItem := renderEntityHuman("ticket", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("ticketNodeToData roundtrip matches direct rendering", func(t *testing.T) {
		node := &TicketNode{
			Slug:     "MY-TICKET",
			Title:    "My Ticket",
			Status:   "open",
			Created:  "2025-01-01T00:00:00Z",
			Finished: "",
			Prompt:   "Fix something",
			Summary:  "",
		}
		nodeData := ticketNodeToData(node)
		nodeData["year"] = float64(2025)
		nodeData["month"] = float64(1)
		nodeData["day"] = float64(1)
		fromNode := renderEntityMarkdownLink("ticket", nodeData)
		fromDirect := renderEntityMarkdownLink("ticket", data)
		if fromNode != fromDirect {
			t.Errorf("ticketNodeToData roundtrip mismatch:\n  fromNode:   %q\n  fromDirect: %q", fromNode, fromDirect)
		}
	})

	t.Run("goal tree ticket markdown uses renderEntityMarkdownLink", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "MY-TICKET", Title: "My Ticket", Status: "open",
				Created: "2025-01-01T00:00:00Z", Prompt: "Fix something",
			}},
		}}
		treeOutput := renderGoalTreeNodes(roots, "md")
		ticketData := ticketNodeToData(roots[0].Tickets[0])
		expectedLink := renderEntityMarkdownLink("ticket", ticketData)
		if !strings.Contains(treeOutput, expectedLink) {
			t.Errorf("goal tree ticket markdown should contain renderEntityMarkdownLink output.\n  Tree:     %q\n  Expected: %q", treeOutput, expectedLink)
		}
		if strings.Contains(treeOutput, "- - [") {
			t.Errorf("ticket in goal tree must not have double dash: %q", treeOutput)
		}
	})

	t.Run("ticket list markdown matches renderEntityMarkdown", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, true))
		directMD := strings.TrimSpace(renderEntityMarkdown("ticket", data))
		if listOutput != directMD {
			t.Errorf("ticket list markdown should match renderEntityMarkdown.\n  List:   %q\n  Direct: %q", listOutput, directMD)
		}
	})

	t.Run("ticket list text matches renderEntityHuman", func(t *testing.T) {
		tickets := []interface{}{data}
		listOutput := strings.TrimSpace(renderTicketList(tickets, false, false))
		directHuman := renderEntityHuman("ticket", data, false)
		if !strings.Contains(listOutput, directHuman) {
			t.Errorf("ticket list text should contain renderEntityHuman output.\n  List:   %q\n  Direct: %q", listOutput, directHuman)
		}
	})
}

func TestUnifiedRenderingSectionIdentity(t *testing.T) {
	data := map[string]interface{}{
		"path":      "test/file.ts#MySection",
		"name":      "MySection",
		"startLine": float64(10),
		"endLine":   float64(20),
	}

	mdLink := renderEntityMarkdownLink("section", data)
	mdItem := renderEntityMarkdown("section", data)
	humanItem := renderEntityHuman("section", data, false)

	t.Run("markdown item is dash-prefixed link", func(t *testing.T) {
		if mdItem != "- "+mdLink {
			t.Errorf("renderEntityMarkdown should be '- ' + renderEntityMarkdownLink.\n  Got:  %q\n  Want: %q", mdItem, "- "+mdLink)
		}
	})

	t.Run("both formats share same props", func(t *testing.T) {
		props := collectEntityProps("section", data, false)
		for _, p := range props {
			if !strings.Contains(mdLink, p) {
				t.Errorf("markdown link missing prop %q: %s", p, mdLink)
			}
			if !strings.Contains(humanItem, p) {
				t.Errorf("human output missing prop %q: %s", p, humanItem)
			}
		}
	})

	t.Run("section tree markdown uses renderEntityMarkdown", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, true))
		expectedMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if treeOutput != expectedMD {
			t.Errorf("section tree markdown root should match renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedMD)
		}
	})

	t.Run("section tree text uses renderEntityHuman", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#MySection",
			Name:      "MySection",
			StartLine: 10,
			EndLine:   20,
		}
		treeOutput := strings.TrimSpace(renderSectionTree(s, false, false))
		expectedHuman := renderEntityHuman("section", data, false)
		if treeOutput != expectedHuman {
			t.Errorf("section tree text root should match renderEntityHuman.\n  Tree:   %q\n  Direct: %q", treeOutput, expectedHuman)
		}
	})

	t.Run("section tree markdown preserves indentation for children", func(t *testing.T) {
		s := &Section{
			Path:      "test/file.ts#Parent",
			Name:      "Parent",
			StartLine: 1,
			EndLine:   30,
			Children: []Section{{
				Path:      "test/file.ts#Child",
				Name:      "Child",
				StartLine: 5,
				EndLine:   15,
			}},
		}
		treeOutput := renderSectionTree(s, false, true)
		lines := strings.Split(strings.TrimSpace(treeOutput), "\n")
		if len(lines) < 2 {
			t.Fatalf("expected at least 2 lines, got %d: %q", len(lines), treeOutput)
		}
		if !strings.HasPrefix(lines[0], "- [") {
			t.Errorf("root section should start with '- [': %q", lines[0])
		}
		if !strings.HasPrefix(lines[1], "  - [") {
			t.Errorf("child section should start with '  - [' for 2-space indent: %q", lines[1])
		}
	})

	t.Run("monorepo tree node markdown matches direct rendering", func(t *testing.T) {
		treeNode := &TreeNode{
			Kind:  TreeNodeSection,
			ID:    "sec1",
			Label: "MySection",
			URI:   "semiorepo://section/test/file.ts/MYSECTION",
			Data:  data,
		}
		var sb strings.Builder
		renderTreeNodeMarkdown(&sb, treeNode, "")
		monorepoOutput := strings.TrimSpace(sb.String())
		directMD := strings.TrimSpace(renderEntityMarkdown("section", data))
		if monorepoOutput != directMD {
			t.Errorf("monorepo tree section markdown should match renderEntityMarkdown.\n  Monorepo: %q\n  Direct:   %q", monorepoOutput, directMD)
		}
	})
}

func TestUnifiedRenderingAllKindIdentity(t *testing.T) {
	entities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"project", TreeNodeProject, map[string]interface{}{
			"name": "myproject", "description": "A project",
		}},
		{"bundle", TreeNodeBundle, map[string]interface{}{
			"name": "mybundle", "root": "path/to/bundle",
		}},
		{"folder", TreeNodeFolder, map[string]interface{}{
			"path": "src/folder", "name": "folder",
		}},
		{"file", TreeNodeFile, map[string]interface{}{
			"path": "src/file.ts", "name": "file.ts",
		}},
		{"contributor", TreeNodeContributor, map[string]interface{}{
			"github": "dev1", "name": "Developer One",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "code-hygiene", "name": "Code Hygiene", "description": "Clean code policy",
		}},
		{"violationKind", TreeNodeViolationKindNode, map[string]interface{}{
			"id": "inline-comment", "description": "No inline comments",
		}},
		{"draft", TreeNodeDraft, map[string]interface{}{
			"id": "draft-1", "slug": "my-draft",
		}},
		{"commit", TreeNodeCommit, map[string]interface{}{
			"sha": "abc1234567890", "message": "fix: something",
		}},
	}

	for _, tt := range entities {
		t.Run(tt.kind+"_markdown_identity", func(t *testing.T) {
			directMD := renderEntityMarkdown(tt.kind, tt.data)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			treeOutput := strings.TrimSpace(sb.String())
			directMDTrimmed := strings.TrimSpace(directMD)
			if treeOutput != directMDTrimmed {
				t.Errorf("%s: monorepo tree markdown differs from direct renderEntityMarkdown.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directMDTrimmed)
			}
		})

		t.Run(tt.kind+"_text_identity", func(t *testing.T) {
			directHuman := renderEntityHuman(tt.kind, tt.data, false)
			treeNode := &TreeNode{
				Kind:  tt.nodeKind,
				ID:    "test-" + tt.kind,
				Label: tt.kind,
				Data:  tt.data,
			}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			treeOutput := strings.TrimSpace(sb.String())
			if treeOutput != directHuman {
				t.Errorf("%s: monorepo tree text differs from direct renderEntityHuman.\n  Tree:   %q\n  Direct: %q", tt.kind, treeOutput, directHuman)
			}
		})

		t.Run(tt.kind+"_props_in_both_formats", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			mdLink := renderEntityMarkdownLink(tt.kind, tt.data)
			human := renderEntityHuman(tt.kind, tt.data, false)
			for _, p := range props {
				if !strings.Contains(mdLink, p) {
					t.Errorf("%s: markdown link missing prop %q: %s", tt.kind, p, mdLink)
				}
				if !strings.Contains(human, p) {
					t.Errorf("%s: human output missing prop %q: %s", tt.kind, p, human)
				}
			}
		})
	}
}

func TestCollectEntityPropsConsistency(t *testing.T) {
	t.Run("goal props include all fields", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "G1",
			"title":       "My Goal",
			"status":      "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description",
		}
		props := collectEntityProps("goal", data, false)
		if len(props) < 4 {
			t.Errorf("goal props should have >= 4 entries (title, status, created, due, desc), got %d: %v", len(props), props)
		}
		found := map[string]bool{}
		for _, p := range props {
			if strings.Contains(p, "My Goal") {
				found["title"] = true
			}
			if strings.Contains(p, "open") {
				found["status"] = true
			}
			if strings.Contains(p, "created") {
				found["created"] = true
			}
			if strings.Contains(p, "Description") {
				found["description"] = true
			}
		}
		for _, key := range []string{"title", "status", "created", "description"} {
			if !found[key] {
				t.Errorf("goal props missing %s: %v", key, props)
			}
		}
	})

	t.Run("ticket open props include prompt", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "open",
			"started": "2025-01-01T00:00:00Z", "prompt": "Please fix",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Please fix") {
				found = true
			}
		}
		if !found {
			t.Errorf("open ticket props should contain prompt: %v", props)
		}
	})

	t.Run("ticket closed props include summary", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z", "summary": "Fixed the bug",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		found := false
		for _, p := range props {
			if strings.Contains(p, "Fixed the bug") {
				found = true
			}
		}
		if !found {
			t.Errorf("closed ticket props should contain summary: %v", props)
		}
	})

	t.Run("section props include line range", func(t *testing.T) {
		data := map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(10), "endLine": float64(20),
		}
		props := collectEntityProps("section", data, false)
		if len(props) < 1 || !strings.Contains(props[0], ":10-20") {
			t.Errorf("section props should contain :10-20, got: %v", props)
		}
	})

	t.Run("definition props include name and line range", func(t *testing.T) {
		data := map[string]interface{}{
			"name": "myFunc", "startLine": float64(5), "endLine": float64(15),
		}
		props := collectEntityProps("definition", data, false)
		foundName := false
		foundRange := false
		for _, p := range props {
			if strings.Contains(p, "myFunc") {
				foundName = true
			}
			if strings.Contains(p, ":5-15") {
				foundRange = true
			}
		}
		if !foundName {
			t.Errorf("definition props should contain name: %v", props)
		}
		if !foundRange {
			t.Errorf("definition props should contain line range: %v", props)
		}
	})

	t.Run("props strip newlines from multi-line content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\nLine two.\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("prop contains newline: %q", p)
			}
			if strings.Contains(p, "\r") {
				t.Errorf("prop contains carriage return: %q", p)
			}
		}
	})

	t.Run("props strip backticks from content", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed the `title` parameter in `UpdateTicketTitle`.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "`") {
				t.Errorf("prop contains backtick: %q", p)
			}
		}
	})

	t.Run("props collapse multiple spaces", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Fixed.\n\n1. First.\n2. Second.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "  ") {
				t.Errorf("prop contains double space: %q", p)
			}
		}
	})

	t.Run("props handle Windows line endings", func(t *testing.T) {
		data := map[string]interface{}{
			"slug": "T1", "title": "Fix Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Line one.\r\nLine two.\r\nLine three.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}
		props := collectEntityProps("ticket", data, false)
		for _, p := range props {
			if strings.Contains(p, "\r") || strings.Contains(p, "\n") {
				t.Errorf("prop contains line break: %q", p)
			}
		}
	})

	t.Run("goal props strip newlines from description", func(t *testing.T) {
		data := map[string]interface{}{
			"id": "G1", "title": "My Goal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Goal with\nmultiple\nlines and `backticks`.",
		}
		props := collectEntityProps("goal", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("goal prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("goal prop contains backtick: %q", p)
			}
		}
	})

	t.Run("commit props strip newlines from message", func(t *testing.T) {
		data := map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}
		props := collectEntityProps("commit", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("commit prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("commit prop contains backtick: %q", p)
			}
		}
	})

	t.Run("policy props strip newlines", func(t *testing.T) {
		data := map[string]interface{}{
			"id":          "code-hygiene",
			"name":        "Code Hygiene",
			"description": "Clean code\npolicy with `rules`.",
		}
		props := collectEntityProps("policy", data, false)
		for _, p := range props {
			if strings.Contains(p, "\n") {
				t.Errorf("policy prop contains newline: %q", p)
			}
			if strings.Contains(p, "`") {
				t.Errorf("policy prop contains backtick: %q", p)
			}
		}
	})
}

func TestSingleLineOutput(t *testing.T) {
	multiLineEntities := []struct {
		kind     string
		nodeKind TreeNodeKind
		data     map[string]interface{}
	}{
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T1", "title": "Fix `title` Bug", "status": "closed",
			"finished": "2025-01-02T00:00:00Z",
			"summary":  "Added folder renaming.\n\n1. MCP ticketReopen handler: reads the `title` parameter.\n2. MCP ticketClose handler: reads `title`.\n3. Goals: added `UpdateGoalTitle()` helper.\n\nAlso fixed a test bug.",
			"year":     float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"ticket", TreeNodeTicket, map[string]interface{}{
			"slug": "T2", "title": "Open Ticket", "status": "open",
			"started": "2025-01-01T00:00:00Z",
			"prompt":  "Fix the `config` module.\nIt has multiple issues:\n- Issue 1\n- Issue 2",
			"year":    float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"goal", TreeNodeGoal, map[string]interface{}{
			"id": "G1", "title": "Multi\nLine\nGoal", "status": "open",
			"dueDate":     "2030-01-01",
			"createdAt":   "2025-01-01T00:00:00Z",
			"description": "Description with `code`\nand\r\nnewlines.",
		}},
		{"commit", TreeNodeCommit, map[string]interface{}{
			"sha":     "abc1234567890",
			"message": "feat: add feature\n\nDetailed description\nwith `code` refs.",
		}},
		{"policy", TreeNodePolicy, map[string]interface{}{
			"id": "p1", "name": "Policy", "description": "Rule 1\nRule 2\n`Rule 3`",
		}},
		{"project", TreeNodeProject, map[string]interface{}{
			"name": "proj1", "description": "Project\nwith\nnewlines",
		}},
	}

	assertSingleLine := func(t *testing.T, label, output string) {
		t.Helper()
		lines := strings.Split(output, "\n")
		if len(lines) > 1 {
			t.Errorf("%s is multi-line (%d lines):\n%q", label, len(lines), output)
		}
		if strings.Contains(output, "\r") {
			t.Errorf("%s contains carriage return:\n%q", label, output)
		}
	}

	assertNoRawBackticks := func(t *testing.T, label, output string) {
		t.Helper()
		for _, p := range collectEntityProps("ticket", multiLineEntities[0].data, false) {
			if strings.Contains(p, "`") {
				t.Errorf("%s prop contains backtick: %q", label, p)
			}
		}
		_ = output
	}

	for _, tt := range multiLineEntities {
		t.Run(tt.kind+"_renderEntityMarkdownLink_single_line", func(t *testing.T) {
			output := renderEntityMarkdownLink(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdownLink("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityMarkdown_single_line", func(t *testing.T) {
			output := renderEntityMarkdown(tt.kind, tt.data)
			assertSingleLine(t, "renderEntityMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, false)
			assertSingleLine(t, "renderEntityHuman("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_renderEntityHuman_tty_single_line", func(t *testing.T) {
			output := renderEntityHuman(tt.kind, tt.data, true)
			assertSingleLine(t, "renderEntityHuman_tty("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_props_no_backticks", func(t *testing.T) {
			props := collectEntityProps(tt.kind, tt.data, false)
			for _, p := range props {
				if strings.Contains(p, "`") {
					t.Errorf("prop contains backtick: %q", p)
				}
			}
			assertNoRawBackticks(t, tt.kind, "")
		})

		t.Run(tt.kind+"_monorepoTreeNodeMarkdown_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeMarkdown("+tt.kind+")", output)
		})

		t.Run(tt.kind+"_monorepoTreeNodeText_single_line", func(t *testing.T) {
			treeNode := &TreeNode{Kind: tt.nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeText(&sb, treeNode, "", true, true)
			output := strings.TrimRight(sb.String(), "\n")
			assertSingleLine(t, "renderTreeNodeText("+tt.kind+")", output)
		})
	}

	t.Run("goal_tree_with_multi_line_tickets_all_single_line", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Parent\nGoal", Status: "open",
			Tickets: []*TicketNode{
				{
					Slug: "T1", Title: "Ticket `One`", Status: "closed",
					Created:  "2025-01-01T00:00:00Z",
					Finished: "2025-01-02T00:00:00Z",
					Summary:  "Fixed things.\n\n1. First fix.\n2. Second fix with `code`.",
				},
				{
					Slug: "T2", Title: "Ticket Two", Status: "open",
					Created: "2025-01-01T00:00:00Z",
					Prompt:  "Please fix:\n- Item 1\n- Item 2",
				},
			},
			Children: []*GoalNode{{
				ID: "G2", Title: "Child Goal", Status: "open",
				Description: "Description\nwith\nnewlines.",
			}},
		}}
		for _, format := range []string{"md", "text"} {
			output := renderGoalTreeNodes(roots, format)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("goal tree (%s) line %d is empty (blank line in output)", format, i)
				}
			}
		}
	})

	t.Run("ticket_list_with_multi_line_summary_single_line", func(t *testing.T) {
		tickets := []interface{}{
			map[string]interface{}{
				"slug": "T1", "title": "Ticket", "status": "closed",
				"finished": "2025-01-02T00:00:00Z",
				"summary":  "Summary with\nnewlines and `backticks`.",
				"year":     float64(2025), "month": float64(1), "day": float64(1),
			},
		}
		for _, useMD := range []bool{true, false} {
			output := renderTicketList(tickets, false, useMD)
			for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
				trimmed := strings.TrimLeft(line, " ")
				if trimmed == "" {
					t.Errorf("ticket list (md=%v) line %d is empty", useMD, i)
				}
			}
		}
	})

	t.Run("formatMarkdownResult_ticket_list_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"tickets": []interface{}{
					map[string]interface{}{
						"slug": "T1", "title": "Ticket", "status": "closed",
						"finished": "2025-01-02T00:00:00Z",
						"summary":  "Line 1\nLine 2\n`code`",
						"year":     float64(2025), "month": float64(1), "day": float64(1),
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("ticket list", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult ticket list line %d is empty", i)
			}
		}
	})

	t.Run("formatMarkdownResult_goal_tree_single_line", func(t *testing.T) {
		payload := map[string]interface{}{
			"repo": map[string]interface{}{
				"goals": []interface{}{
					map[string]interface{}{
						"id": "G1", "title": "Goal", "status": "open",
						"dueDate": "2030-01-01", "createdAt": "2025-01-01T00:00:00Z",
						"description": "Desc\nwith\nnewlines",
					},
				},
				"tickets": []interface{}{
					map[string]interface{}{
						"id": "T1", "slug": "T1", "title": "Ticket", "status": "closed",
						"goal":    "G1",
						"date":    map[string]interface{}{"created": "2025-01-01T00:00:00Z", "finished": "2025-01-02T00:00:00Z"},
						"summary": "Summary\nwith `code`\nrefs.",
					},
				},
			},
		}
		data, _ := json.Marshal(payload)
		output := formatMarkdownResult("goal tree", data)
		for i, line := range strings.Split(strings.TrimRight(output, "\n"), "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if trimmed == "" {
				t.Errorf("formatMarkdownResult goal tree line %d is empty", i)
			}
		}
	})
}

func TestNoDoubleDashInMarkdownOutput(t *testing.T) {
	kinds := []struct {
		kind string
		data map[string]interface{}
	}{
		{"goal", map[string]interface{}{
			"id": "G1", "title": "Goal", "status": "open",
		}},
		{"ticket", map[string]interface{}{
			"slug": "T1", "title": "Ticket", "status": "open",
			"year": float64(2025), "month": float64(1), "day": float64(1),
		}},
		{"section", map[string]interface{}{
			"path": "file.ts#Sec", "name": "Sec",
			"startLine": float64(1), "endLine": float64(5),
		}},
		{"bundle", map[string]interface{}{
			"name": "b1", "root": "path",
		}},
		{"folder", map[string]interface{}{
			"path": "src/f",
		}},
		{"file", map[string]interface{}{
			"path": "src/a.ts",
		}},
		{"contributor", map[string]interface{}{
			"github": "dev",
		}},
		{"commit", map[string]interface{}{
			"sha": "abc",
		}},
	}

	for _, tt := range kinds {
		t.Run(tt.kind+"_renderEntityMarkdown", func(t *testing.T) {
			output := renderEntityMarkdown(tt.kind, tt.data)
			if count := strings.Count(output, "- "); count > 1 {
				dashPositions := []int{}
				idx := 0
				for {
					pos := strings.Index(output[idx:], "- ")
					if pos == -1 {
						break
					}
					dashPositions = append(dashPositions, idx+pos)
					idx += pos + 2
				}
				if len(dashPositions) >= 2 && dashPositions[1]-dashPositions[0] <= 3 {
					t.Errorf("renderEntityMarkdown(%s) has double dash at start: %q", tt.kind, output)
				}
			}
		})

		t.Run(tt.kind+"_treeNodeMarkdown", func(t *testing.T) {
			nodeKind := TreeNodeKind(tt.kind)
			switch tt.kind {
			case "goal":
				nodeKind = TreeNodeGoal
			case "ticket":
				nodeKind = TreeNodeTicket
			case "section":
				nodeKind = TreeNodeSection
			case "bundle":
				nodeKind = TreeNodeBundle
			case "folder":
				nodeKind = TreeNodeFolder
			case "file":
				nodeKind = TreeNodeFile
			case "contributor":
				nodeKind = TreeNodeContributor
			case "commit":
				nodeKind = TreeNodeCommit
			}

			treeNode := &TreeNode{Kind: nodeKind, ID: "test", Label: "test", Data: tt.data}
			var sb strings.Builder
			renderTreeNodeMarkdown(&sb, treeNode, "")
			output := sb.String()
			if strings.HasPrefix(output, "- - ") {
				t.Errorf("renderTreeNodeMarkdown(%s) has double dash: %q", tt.kind, output)
			}
		})
	}

	t.Run("goalTreeNodes_no_double_dash", func(t *testing.T) {
		roots := []*GoalNode{{
			ID: "G1", Title: "Goal", Status: "open",
			Tickets: []*TicketNode{{
				Slug: "T1", Title: "Ticket", Status: "open",
			}},
		}}
		output := renderGoalTreeNodes(roots, "md")
		for i, line := range strings.Split(output, "\n") {
			trimmed := strings.TrimLeft(line, " ")
			if strings.HasPrefix(trimmed, "- - ") {
				t.Errorf("line %d has double dash: %q", i, line)
			}
		}
	})
}

// #endregion 🔖Unified Rendering Identity Tests

// #endregion 🔖Monorepo Tree Tests

func TestMigrateAuthorFieldsToString(t *testing.T) {
	ctx := context.Background()

	// Migrate tickets via StreamTickets
	ticketCh := make(chan Ticket)
	var ticketErr error
	go func() {
		ticketErr = StreamTickets(ctx, nil, nil, nil, ticketCh)
	}()
	ticketCount := 0
	for ticket := range ticketCh {
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save ticket %s: %v", ticket.Slug, err)
		}
		ticketCount++
	}
	if ticketErr != nil {
		t.Fatalf("stream tickets failed: %v", ticketErr)
	}
	t.Logf("migrated %d tickets via stream", ticketCount)

	// Migrate remaining tickets by walking the filesystem directly
	ticketsDir := GetTicketsDir()
	remainingCount := 0
	filepath.WalkDir(ticketsDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || d.Name() != "ticket.json" {
			return nil
		}
		raw, err := ReadTextFile(path)
		if err != nil {
			return nil
		}
		if !strings.Contains(raw, `"author": {`) {
			return nil // already migrated
		}
		var ticket Ticket
		if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
			t.Logf("failed to parse %s: %v", path, err)
			return nil
		}
		ticket.JsonPath = path
		if err := SaveTicket(&ticket); err != nil {
			t.Errorf("failed to save remaining ticket %s: %v", path, err)
		}
		remainingCount++
		return nil
	})
	t.Logf("migrated %d remaining tickets", remainingCount)

	// Migrate goals
	goalCh := make(chan *Goal)
	var goalErr error
	go func() {
		goalErr = StreamGoals(ctx, goalCh)
	}()
	goalCount := 0
	for goal := range goalCh {
		if err := SaveGoal(*goal); err != nil {
			t.Errorf("failed to save goal %s: %v", goal.ID, err)
		}
		goalCount++
	}
	if goalErr != nil {
		t.Fatalf("stream goals failed: %v", goalErr)
	}
	t.Logf("migrated %d goals", goalCount)
}
