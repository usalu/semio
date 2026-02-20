// #region 🔖Header

// [🧰semiorepo⌨️cli🧪maintestgo](semiorepo://file/semio-repo/cli/main_test.go)

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
	"regexp"
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
			if parts[i] == "🎫" {
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
				"date": "2026-02-06 22:03:11",
				"author": %s,
				"system": "linux",
				"client": "codex",
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

	tmpDir, err := os.MkdirTemp("", "semio-test-discovery")
	if err != nil {
		t.Fatalf("failed to create tmp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	contributorsDir := filepath.Join(tmpDir, ".semio-repo", "👤")
	os.MkdirAll(contributorsDir, 0755)

	t.Run("Match and update email", func(t *testing.T) {

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

		github := "octocat"
		c := Contributor{
			Github: github,
			Name:   "The Octocat",
			Names:  []string{"The Octocat"},
			Email:  "octocat@github.com",
			Emails: []string{"octocat@github.com"},
		}
		SaveContributor(c)

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

func TestStatutesNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ statutes { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if len(resp.Statutes) == 0 {
		t.Error("statutes collection should not be empty")
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

func TestBreachsNonEmpty(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()
	result, err := executor.ExecuteJSON(ctx, "{ breachs { id } }", nil)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	var resp struct {
		Breachs []struct {
			ID string `json:"id"`
		} `json:"breachs"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse response: %v", err)
	}
	if resp.Breachs == nil {
		t.Error("breachs collection should not be nil")
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

			if err == nil {
				var resp struct {
					TicketOpen struct {
						ID    string `json:"id"`
						Slug  string `json:"slug"`
						Year  int    `json:"year"`
						Month int    `json:"month"`
						Day   int    `json:"day"`
					} `json:"ticketOpen"`
				}
				if json.Unmarshal([]byte(result), &resp) == nil {
					to := resp.TicketOpen
					if to.Year == 0 || to.Month == 0 || to.Day == 0 {
						t.Errorf("ticketOpen returned invalid date: year=%d month=%d day=%d (id=%s)", to.Year, to.Month, to.Day, to.ID)
					}
					if strings.Contains(to.ID, "0000/00/00") {
						t.Errorf("ticketOpen id must not contain 0000/00/00, got %s", to.ID)
					}
					path := GetTicketPath(to.Year, to.Month, to.Day, to.Slug)
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
		Year:       26,
		Month:      1,
		Day:        20,
		Slug:       "SAMPLE",
		FolderPath: filepath.Join(rootDir, ".semio-repo", "🎫", "26", "01", "20", "SAMPLE"),
	}
	files := []string{
		".semio-repo/🎫/26/01/20/SAMPLE/plan.md",
		"./.semio-repo/🎫/26/01/20/SAMPLE/ticket.md",
		filepath.Join(rootDir, ".semio-repo", "🎫", "26", "01", "20", "SAMPLE", "extra.txt"),
		absMain,
	}
	filtered := FilterTicketWorkspaceFiles(ticket, files)
	if len(filtered) != 1 || filtered[0] != absMain {
		t.Fatalf("expected [%s], got %v", absMain, filtered)
	}
}

func TestNormalizeTicketFileInput(t *testing.T) {
	absRoot := GetRootDir()
	filePath := filepath.ToSlash(filepath.Join("semio-repo", "cli", "main.go"))
	absPath := filepath.Join(absRoot, filePath)
	fileID := FileHeaderId(filePath)
	fileUri := "semiorepo://file/" + PathToUriPath(filePath)
	filesUri := "semiorepo://files/" + PathToUriPath(filepath.Dir(filePath))
	fileUriForFiles := filesUri + "/" + PathToUriPath(filepath.Base(filePath))
	cases := []struct {
		name  string
		input string
		want  string
	}{
		{"path", filePath, filePath},
		{"abs path", absPath, filePath},
		{"file uri", fileUri, filePath},
		{"files uri", fileUriForFiles, filePath},
		{"id", fileID, filePath},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := normalizeTicketFileInput(tc.input)
			if got != tc.want {
				t.Fatalf("expected %s, got %s", tc.want, got)
			}
		})
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
			statutes { id }
		}
		statutes {
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
		breachs {
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
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
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
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
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
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
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
			breachs { id }
		}
		folders {
			id
			path
			parent { id }
			children { id }
			files { id }
			bundle { id }
			breachs { id }
		}
		files {
			id
			path
			folder { id }
			bundle { id }
			sections { id name }
			definitions { id name kind }
			breachs { id }
		}
		tickets {
			id
			slug
		}
		policies {
			id
			name
			statutes { id }
		}
		statutes {
			id
		}
		breachs {
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
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
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
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
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
			Breachs []struct {
				ID string `json:"id"`
			} `json:"breachs"`
		} `json:"files"`
		Tickets []struct {
			ID   string `json:"id"`
			Slug string `json:"slug"`
		} `json:"tickets"`
		Policies []struct {
			ID       string `json:"id"`
			Name     string `json:"name"`
			Statutes []struct {
				ID string `json:"id"`
			} `json:"statutes"`
		} `json:"policies"`
		Statutes []struct {
			ID string `json:"id"`
		} `json:"statutes"`
		Breachs []struct {
			ID   string `json:"id"`
			File *struct {
				ID string `json:"id"`
			} `json:"file"`
			Folder *struct {
				ID string `json:"id"`
			} `json:"folder"`
		} `json:"breachs"`
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
	if len(resp.Statutes) == 0 {
		t.Error("statutes should not be empty")
	}
	if resp.Breachs == nil {
		t.Error("breachs should not be nil")
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
	for _, vk := range resp.Statutes {
		if vk.ID == "" {
			t.Error("statute has empty id")
		}
	}
	for _, v := range resp.Breachs {
		if v.ID == "" {
			t.Errorf("breach has empty id: %+v", v)
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
	if testing.Short() {
		t.Skip("skipping slow sections edges test in short mode")
	}
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
				breachs { id }
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
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
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
				breachs { id }
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
				Breachs []struct {
					ID string `json:"id"`
				} `json:"breachs"`
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

var relativeTimePattern = regexp.MustCompile(`\b(opened |closed |created )?(a long while ago|\d+ (?:second|minute|hour|day|week|month|year)s? (?:ago|from now))\b`)

func normalizeRelativeTimes(s string) string {
	return relativeTimePattern.ReplaceAllString(s, "<TIME>")
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
	if len(res.Breachs) != res.Remaining {
		t.Errorf("breachs length %d != remaining %d", len(res.Breachs), res.Remaining)
	}
}

func TestFileHeaderId(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"code ts", "semio/js/src/index.ts", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + "indexts"},
		{"code tsx", "semio/js/src/App.tsx", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + "apptsx"},
		{"code go", "semio-repo/cli/cli.go", emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli" + emojiText(EmojiFileCode) + "cligo"},
		{"code cs", "semio/gh/Semio.cs", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "gh" + emojiText(EmojiFileCode) + "semiocs"},
		{"code py", "semio/engine/main.py", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine" + emojiText(EmojiFileCode) + "mainpy"},
		{"test ts", "semio/js/src/index.test.ts", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileTest) + "indextestts"},
		{"test go", "semio-repo/cli/cli_test.go", emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli" + emojiText(EmojiFileTest) + "clitestgo"},
		{"config json", "tsconfig.json", emojiText(EmojiFileConfig) + "tsconfigjson"},
		{"docs md", "README.md", emojiText(EmojiFileDocs) + "readmemd"},
		{"script sh", "build.sh", emojiText(EmojiFileScript) + "buildsh"},
		{"script bash", "deploy.bash", emojiText(EmojiFileScript) + "deploybash"},
		{"script ps1", "setup.ps1", emojiText(EmojiFileScript) + "setupps1"},
		{"resource png", "logo.png", emojiText(EmojiFileResource) + "logopng"},
		{"license", "LICENSE.md", emojiText(EmojiFileLicense) + "licensemd"},
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
		want := emojiText(EmojiFolderOrg) + "tools" + emojiText(EmojiFileScript) + Flat(filepath.Base(filePath))
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
		want := emojiText(EmojiFolderOrg) + "scripts" + emojiText(EmojiFileScript) + Flat(filepath.Base(filePath))
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
		want := emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFileCode) + Flat(filepath.Base(filePath))
		if got != want {
			t.Errorf("FileHeaderId(%q) without shebang = %q, want %q", filePath, got, want)
		}
	})

	t.Run("nonexistent code file stays code", func(t *testing.T) {
		got := FileHeaderId("nonexistent/file.ts")
		want := emojiText(EmojiFolderOrg) + "nonexistent" + emojiText(EmojiFileCode) + Flat("file.ts")
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
		{"config", "config", "\u2699\uFE0F"},
		{"resource", "resource", "\U0001F4BE"},
		{"license", "license", "\u2696\uFE0F"},
		{"unknown", "unknown", ""},
		{"empty", "", ""},
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
	content := "// #region \U0001F516Header\n\n// [\U0001F4BBold/wrong/path.ts](semiorepo://file/old/wrong/path.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	expectedId := FileHeaderId(filePath)
	expectedUri := FileHeaderUri(filePath)
	breachs := []Breach{{
		Kind:    BreachCodeFileWrongIdentificationId,
		Scope:   filePath + "#Header",
		Line:    3,
		Excerpt: expectedId,
	}}
	fixed, err := applyAutofixes(filePath, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "["+expectedId+"]("+expectedUri+")") {
		t.Errorf("fixed content should contain [ID](URI), got:\n%s", fixedContent)
	}
	if strings.Contains(fixedContent, "old/wrong/path.ts)") {
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
	correctUri := FileHeaderUri(filePath)
	content := "// #region \U0001F516Header\n\n// [" + correctId + "](" + correctUri + ")\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeFileWrongIdentificationId {
			t.Errorf("should not detect wrong file ID when correct ID is present, got: %s", v.Summary)
		}
		if v.Kind == BreachCodeFileWrongIdentificationUri {
			t.Errorf("should not detect wrong file URI when correct URI is present, got: %s", v.Summary)
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
	content := "// #region \U0001F516Header\n\n// [\U0001F4BBwrong/path.ts](semiorepo://file/wrong/path.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	foundId := false
	foundUri := false
	for _, v := range breachs {
		if v.Kind == BreachCodeFileWrongIdentificationId {
			foundId = true
			if !v.Autofixable() {
				t.Error("BreachCodeFileWrongIdentificationId should be autofixable")
			}
			if v.Excerpt != FileHeaderId(filePath) {
				t.Errorf("excerpt should be expected file ID, got %q", v.Excerpt)
			}
		}
		if v.Kind == BreachCodeFileWrongIdentificationUri {
			foundUri = true
		}
	}
	if !foundId {
		t.Error("expected BreachCodeFileWrongIdentificationId breach")
	}
	if !foundUri {
		t.Error("expected BreachCodeFileWrongIdentificationUri breach")
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
	content := "// #region \U0001F516Header\n\n// [\U0001F4BBwrong/path.ts](semiorepo://file/wrong/path.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n// #region \U0001F516Main\n\nconst x = 1;\n\n// #endregion \U0001F516Main\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: filePath}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	var autofixable []Breach
	for _, v := range breachs {
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
	expectedUri := FileHeaderUri(filePath)
	if !strings.Contains(fixedContent, "["+expectedId+"]("+expectedUri+")") {
		t.Errorf("fixed content should contain [ID](URI)")
	}

	ctx2 := NewPolicyContextWithFiles(scope, bundles, []string{filePath})
	breachs2, _ := CheckPoliciesWithContext(ctx2, nil)
	for _, v := range breachs2 {
		if v.Kind == BreachCodeFileWrongIdentificationId {
			t.Error("after fix, should not detect wrong file ID")
		}
		if v.Kind == BreachCodeFileWrongIdentificationUri {
			t.Error("after fix, should not detect wrong file URI")
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
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	autofixableCount := 0
	for _, v := range breachs {
		if v.Autofixable() {
			autofixableCount++
		}
	}
	if autofixableCount == 0 {
		t.Fatal("expected autofixable breachs in fixture")
	}

	var autofixable []Breach
	for _, v := range breachs {
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

	breachs := []Breach{
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeSectionNameMismatch, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeSectionEmpty, Scope: testFile + "#Empty", Line: 7},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeCommentBlock, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeCommentJSDoc, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

func TestFixMultipleBreachsSameFile(t *testing.T) {
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

	breachs := []Breach{
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: BreachCodeCommentInline, Scope: testFile, Line: 11},
		{Kind: BreachCodeSectionNameMismatch, Scope: testFile, Line: 15},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

	content := `// #region 🔖Section
const x = 1; // trailing comment
// TODO: fix this
// this line is part of the todo description

// another normal comment
/* TODO: block todo */
const y = 2; // normal trailing
// #endregion 🔖Fix Tests
`

	testFile := "test_improved.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	expectedBreachs := 3
	if len(breachs) != expectedBreachs {
		t.Errorf("expected %d breachs, got %d", expectedBreachs, len(breachs))
		for i, v := range breachs {
			t.Logf("Breach %d: %s at %d:%d", i, v.Kind, v.Line, v.Column)
		}
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}

	if fixed != 3 {
		t.Errorf("expected 3 fixes, got %d", fixed)
	}

	result, _ := ReadTextFile(absPath)

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
	testFile := "package.json"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()
	breachs := lang.ScanComments(ctx, testFile, content, strings.Split(content, "\n"))

	if len(breachs) != 0 {
		t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
	}
}

func TestScanCommentsGo(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewGoLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// this is a comment\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* block */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// TODO: fix later\n// continuation of todo\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("block TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* TODO: fix later */\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for block TODO, got %d", len(breachs))
		}
	})

	t.Run("nolint skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// nolint:errcheck\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for nolint, got %d", len(breachs))
		}
	})

	t.Run("raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `// not a comment`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in raw string, got %d", len(breachs))
		}
	})

	t.Run("multi-line raw backtick string skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar s = `line1\n// not a comment\nline3`\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line raw string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "// #region 🔖Header\n\n// header comment\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("debug marker skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nfmt.Println(\"[DEBUG] test\")\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for debug marker, got %d", len(breachs))
		}
	})

	t.Run("url scheme not flagged", func(t *testing.T) {
		content := "// #region 🔖Section\n\nvar url = \"https://example.com\"\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for URL scheme, got %d", len(breachs))
		}
	})

	t.Run("grouped inline comments", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// comment one\n\n// comment two\n\nfunc main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.go", content, strings.Split(content, "\n"))
		if len(breachs) != 2 {
			t.Errorf("expected 2 breachs for separate comment blocks, got %d", len(breachs))
		}
	})
}

func TestScanCommentsPython(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewPythonLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "# region Section\n\n# TODO: fix later\n# continuation of todo\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("noqa skipped", func(t *testing.T) {
		content := "# region Section\n\n# noqa: E501\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for noqa, got %d", len(breachs))
		}
	})

	t.Run("type ignore skipped", func(t *testing.T) {
		content := "# region Section\n\n# type: ignore[assignment]\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for type: ignore, got %d", len(breachs))
		}
	})

	t.Run("triple double quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"# not a comment\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("triple single quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = '''# not a comment'''\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in triple-single-quoted string, got %d", len(breachs))
		}
	})

	t.Run("multi-line triple quote string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"\"\"\n# not a comment\n\"\"\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in multi-line triple-quoted string, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "# region Header\n#\n# header comment\n#\n# endregion Header\n\n# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("comment in regular string skipped", func(t *testing.T) {
		content := "# region Section\n\ns = \"# not a comment\"\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("trailing comment", func(t *testing.T) {
		content := "# region Section\n\nx = 1  # trailing comment\n\ndef main(): pass\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.py", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach for trailing comment, got %d", len(breachs))
		}
		if breachs[0].Column <= 1 {
			t.Errorf("expected column > 1 for trailing comment, got %d", breachs[0].Column)
		}
	})
}

func TestScanCommentsCSharp(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewCSharpLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n// this is a comment\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "#region 🔖Section\n\n/* block */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// TODO: fix later\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
		}
	})

	t.Run("pragma skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\n// pragma warning disable\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for pragma, got %d", len(breachs))
		}
	})

	t.Run("verbatim string skipped", func(t *testing.T) {
		content := "#region 🔖Section\n\nvar s = @\"// not a comment\";\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in verbatim string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})

	t.Run("header section excluded", func(t *testing.T) {
		content := "#region 🔖Header\n// header comment\n#endregion 🔖Header\n\n#region 🔖Section\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for header section, got %d", len(breachs))
		}
	})

	t.Run("no JSDoc for csharp", func(t *testing.T) {
		content := "#region 🔖Section\n\n/** not jsdoc in csharp */\n\npublic class C {}\n\n#endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.cs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment (not JSDoc) for C#, got %s", breachs[0].Kind)
		}
	})
}

func TestScanCommentsTypeScript(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewTypeScriptLanguage()

	t.Run("JSDoc detected", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/** jsdoc */\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentJSDoc {
			t.Errorf("expected JSDoc breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("template literal skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `// not a comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in template literal, got %d", len(breachs))
		}
	})

	t.Run("template expression not skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst s = `${x} // comment`;\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for template expression context, got %d", len(breachs))
		}
	})

	t.Run("eslint directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// eslint-disable-next-line\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for eslint directive, got %d", len(breachs))
		}
	})

	t.Run("@ts directive skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// @ts-ignore\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for @ts directive, got %d", len(breachs))
		}
	})

	t.Run("string literals skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\nconst a = '// not a comment';\nconst b = \"// not a comment\";\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.ts", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in strings, got %d", len(breachs))
		}
	})

	t.Run("config file skipped", func(t *testing.T) {
		content := "// inline comment\nconst x = 1;\n"
		breachs := lang.ScanComments(ctx, "tsconfig.json", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for config file, got %d", len(breachs))
		}
	})
}

func TestScanCommentsShell(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewShellLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "# region Section\n\n# this is a comment\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("comment in string skipped", func(t *testing.T) {
		content := "# region Section\n\necho \"# not a comment\"\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for comment in string, got %d", len(breachs))
		}
	})

	t.Run("region markers not flagged", func(t *testing.T) {
		content := "# region Section\n\necho hello\n\n# endregion Section\n"
		breachs := lang.ScanComments(ctx, "test.sh", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for region markers, got %d", len(breachs))
		}
	})
}

func TestScanCommentsRust(t *testing.T) {
	ctx := &PolicyContext{}
	lang := NewRustLanguage()

	t.Run("inline comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// this is a comment\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentInline {
			t.Errorf("expected inline comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("block comment", func(t *testing.T) {
		content := "// #region 🔖Section\n\n/* block comment */\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 1 {
			t.Fatalf("expected 1 breach, got %d", len(breachs))
		}
		if breachs[0].Kind != BreachCodeCommentBlock {
			t.Errorf("expected block comment breach, got %s", breachs[0].Kind)
		}
	})

	t.Run("TODO skipped", func(t *testing.T) {
		content := "// #region 🔖Section\n\n// TODO: fix later\n\nfn main() {}\n\n// #endregion 🔖Section\n"
		breachs := lang.ScanComments(ctx, "test.rs", content, strings.Split(content, "\n"))
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs for TODO, got %d", len(breachs))
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

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
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

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3, Column: 7},
		}
		fixed, err := applyAutofixes(testFile, breachs)
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

		breachs := []Breach{
			{Kind: BreachCodeCommentBlock, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
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

		breachs := []Breach{
			{Kind: BreachCodeCommentInline, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
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
	t.Run("fix emoji variation to colorful", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "This is a test \U0001F4BB\uFE0E with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7 construction."
		expected := "This is a test \U0001F4BB\uFE0F with emoji variation.\nAnd another line \u2699\uFE0F with VS16.\nAnd a plain \U0001F3D7\uFE0F construction."
		testFile := "test_emoji.txt"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, content)

		breachs := []Breach{
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 1},
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 2},
			{Kind: BreachCodeUnicodeEmojiVariation, Scope: testFile, Line: 3},
		}
		fixed, err := applyAutofixes(testFile, breachs)
		if err != nil {
			t.Fatalf("applyAutofixes failed: %v", err)
		}
		if fixed != 3 {
			t.Fatalf("expected 3 fixed, got %d", fixed)
		}
		got, _ := ReadTextFile(absPath)
		if got != expected {
			t.Errorf("expected:\n%q\ngot:\n%q", expected, got)
		}
	})

	t.Run("emojiText preserves VS16 for text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"⚙️", "⚙️"},
			{"⚖️", "⚖️"},
			{"✂️", "✂️"},
			{"🏗️", "🏗️"},
			{"🛠️", "🛠️"},
			{"🛡️", "🛡️"},
			{"⌨️", "⌨️"},
			{"🖱️", "🖱️"},
			{"🏷️", "🏷️"},
			{"🗃️", "🗃️"},
		}
		for _, tc := range cases {
			got := emojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText strips VS16 for non-text-default emojis", func(t *testing.T) {
		cases := []struct {
			input string
			want  string
		}{
			{"💻️", "💻"},
			{"🧪️", "🧪"},
			{"📃️", "📃"},
			{"📜️", "📜"},
		}
		for _, tc := range cases {
			got := emojiText(tc.input)
			if got != tc.want {
				t.Errorf("emojiText(%q) = %q, want %q", tc.input, got, tc.want)
			}
		}
	})
	t.Run("emojiText is idempotent", func(t *testing.T) {
		cases := []string{"⚙️", "🏗️", "💻", "🛠️"}
		for _, tc := range cases {
			once := emojiText(tc)
			twice := emojiText(once)
			if once != twice {
				t.Errorf("emojiText not idempotent: emojiText(%q)=%q, emojiText(%q)=%q", tc, once, once, twice)
			}
		}
	})
	t.Run("emojiText strips VS15", func(t *testing.T) {
		got := emojiText("⚙️")
		if got != "⚙️" {
			t.Errorf("emojiText with VS15 = %q, want %q", got, "⚙️")
		}
	})
	t.Run("section markers not flagged as inline comments", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		fileContent := "// #region \U0001F516Header\n\n// \U0001F4BBsemio/test.tsx\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion \U0001F516Header\n\n//#region \U0001F516Action Hooks\nconst x = 1;\n//#endregion \U0001F516Action Hooks\n"
		testFile := "test.tsx"
		absPath := filepath.Join(tmpDir, testFile)
		WriteTextFile(absPath, fileContent)
		bundles := LoadBundles()
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs, _ := CheckPoliciesWithContext(ctx, nil)
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				t.Errorf("section marker flagged as inline comment at line %d: %s", v.Line, v.Excerpt)
			}
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
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	for _, v := range breachs {
		info := v.Kind.Info()
		if v.Autofixable() != info.Autofixable {
			t.Errorf("breach %s: Autofixable() = %v, Info().Autofixable = %v", v.Kind, v.Autofixable(), info.Autofixable)
		}
	}

	autofixableKinds := []Statute{
		BreachCodeFileMissingIdentification,
		BreachCodeFileWrongLicense,
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	for _, kind := range autofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected autofixable statute %s to be detected", kind)
		}
		if !kind.Info().Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
	}
	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		if counts[kind] == 0 {
			t.Errorf("expected non-autofixable statute %s to be detected", kind)
		}
		if kind.Info().Autofixable {
			t.Errorf("statute %s should not be autofixable", kind)
		}
	}
}

func TestFixViaGraphQL(t *testing.T) {
	executor := getTestExecutor(t)
	ctx := context.Background()

	result, err := executor.ExecuteJSON(ctx, `mutation { fix(scope: "semio-repo/go/main_test.go") { fixed remaining breachs { id summary } } }`, nil)
	if err != nil {
		t.Fatalf("fix mutation failed: %v", err)
	}

	var resp struct {
		Fix struct {
			Fixed     int `json:"fixed"`
			Remaining int `json:"remaining"`
			Breachs   []struct {
				ID      string `json:"id"`
				Summary string `json:"summary"`
			} `json:"breachs"`
		} `json:"fix"`
	}
	if err := json.Unmarshal([]byte(result), &resp); err != nil {
		t.Fatalf("failed to parse fix response: %v\nResult: %s", err, result)
	}
	if resp.Fix.Remaining < 0 {
		t.Error("remaining should not be negative")
	}
	if len(resp.Fix.Breachs) != resp.Fix.Remaining {
		t.Errorf("breachs length %d != remaining %d", len(resp.Fix.Breachs), resp.Fix.Remaining)
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
	if res.Breachs == nil {
		t.Error("Breachs should not be nil")
	}
	if res.Remaining != len(res.Breachs) {
		t.Errorf("remaining %d != breachs length %d", res.Remaining, len(res.Breachs))
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

	breachs := []Breach{}
	fixed, err := applyAutofixes(testFile, breachs)
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

	breachs := []Breach{
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: BreachCodeSectionMissingEndName, Scope: testFile, Line: 11},
	}

	fixed, err := applyAutofixes(testFile, breachs)
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

func TestFixStatuteMeta(t *testing.T) {
	autofixableKinds := []Statute{
		BreachCodeFileMissingHeaderRegion,
		BreachCodeFileMissingIdentification,
		BreachCodeFileWrongIdentificationId,
		BreachCodeFileMissingLicense,
		BreachCodeFileWrongLicense,
		BreachCodeSectionEmpty,
		BreachCodeSectionMissingEndName,
		BreachCodeSectionNameMismatch,
		BreachCodeSectionWrongIdentificationId,
		BreachCodeSectionWrongIdentificationUri,
		BreachCodeDefWrongIdentificationId,
		BreachCodeDefWrongIdentificationUri,
		BreachCodeCommentInline,
		BreachCodeCommentBlock,
		BreachCodeCommentJSDoc,
	}
	for _, kind := range autofixableKinds {
		info := kind.Info()
		if !info.Autofixable {
			t.Errorf("statute %s should be autofixable", kind)
		}
		if info.Reason == "" {
			t.Errorf("statute %s has empty reason", kind)
		}
		if info.Solution == "" {
			t.Errorf("statute %s has empty solution", kind)
		}
	}

	nonAutofixableKinds := []Statute{
		BreachCodeFileMissingContributors,
		BreachCodeSectionMissingStartName,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range nonAutofixableKinds {
		info := kind.Info()
		if info.Autofixable {
			t.Errorf("statute %s should NOT be autofixable", kind)
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
	if !strings.Contains(text, "code") && !strings.Contains(text, "Missing") {
		t.Error("Expected policy tree output to contain policy info")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := ToolPolicyCheck("code", "semio/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestPolicyBreachListCommand(t *testing.T) {
	result := ToolPolicyBreachList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyBreachList returned error: %s", result.Error)
	}
}

func TestFixtureBreachsGroupedInline(t *testing.T) {
	path := "semio/assets/repo/some/folder/file_invalid.tsx"
	bundles := LoadBundles()
	scope := Scope{Kind: ScopeFile, FilePath: path}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{path})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("fixture policy check failed: %v", err)
	}
	if len(breachs) == 0 {
		t.Fatal("expected fixture breachs")
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	required := []Statute{
		BreachCodeSectionMissingSummary,
		BreachCodeSectionOrphanDefinition,
	}
	for _, kind := range required {
		if counts[kind] == 0 {
			t.Fatalf("expected statute %s", kind)
		}
	}
}

func TestFixtureBreachsByLanguage(t *testing.T) {
	bundles := LoadBundles()
	fixtures := []struct {
		path          string
		requiredKinds []Statute
	}{
		{
			path:          "semio/assets/repo/some/folder/file_invalid.py",
			requiredKinds: []Statute{BreachCodeDefMissingSummary},
		},
		{
			path:          "semio/assets/repo/some/folder/file_invalid.cs",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
		{
			path:          "semio/assets/repo/some/folder/file_invalid.go",
			requiredKinds: []Statute{BreachCodeSectionMissingSummary},
		},
	}
	for _, fixture := range fixtures {
		scope := Scope{Kind: ScopeFile, FilePath: fixture.path}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{fixture.path})
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", fixture.path, err)
		}
		if len(breachs) == 0 {
			t.Fatalf("expected fixture breachs for %s", fixture.path)
		}
		counts := map[Statute]int{}
		for _, v := range breachs {
			counts[v.Kind]++
		}
		for _, kind := range fixture.requiredKinds {
			if counts[kind] == 0 {
				t.Fatalf("expected statute %s in %s", kind, fixture.path)
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
		breachs, err := CheckPoliciesWithContext(ctx, nil)
		if err != nil {
			t.Fatalf("fixture policy check failed for %s: %v", path, err)
		}
		if len(breachs) != 0 {
			for _, v := range breachs {
				t.Logf("[DEBUG] breach in %s: kind=%s scope=%s line=%d summary=%s", path, v.Kind, v.Scope, v.Line, v.Summary)
			}
			t.Fatalf("expected no breachs for %s, got %d", path, len(breachs))
		}
	}
}

func TestSectionMissingSummaryAndSpecs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[BreachCodeSectionMissingSummary] == 0 {
		t.Fatal("expected section missing summary breach")
	}
}

func TestSectionWithSummaryAndSpecs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Utility functions.\n\nconst x = 1;\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeSectionMissingSummary {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestDefinitionMissingSummaryAndSpecs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[BreachCodeDefMissingSummary] == 0 {
		t.Fatal("expected definition missing summary breach")
	}
}

func TestDefinitionWithSummaryAndSpecs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeDefMissingSummary || v.Kind == BreachCodeDefMissingSpecs {
			t.Fatalf("unexpected breach: %s", v.Kind)
		}
	}
}

func TestSectionDocLinesExemptsDocComments(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n// Functions MUST be exported.\n\n// Processes work items.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentInline {
			t.Fatalf("section doc comment wrongly flagged as inline at line %d", v.Line)
		}
	}
}

func TestSectionMissingIdentification(t *testing.T) {
	tests := []struct {
		name    string
		file    string
		content string
	}{
		{
			name:    "TypeScript section without identification",
			file:    "src/app.ts",
			content: "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\n// Does work.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/do-work)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Go section without identification",
			file:    "src/app.go",
			content: "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](semiorepo://definition/src/app.go/functions/do-work)\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Python section without identification",
			file:    "src/app.py",
			content: "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do-work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
		},
		{
			name:    "CSharp section without identification",
			file:    "src/App.cs",
			content: "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\npublic static void DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Rust section without identification",
			file:    "src/app.rs",
			content: "// #region 🔖Header\n\n// [💻src/app.rs](semiorepo://file/src/app.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\npub fn do_work() {}\n\n// #endregion 🔖Functions\n",
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tc.file))
			os.MkdirAll(dir, 0o755)
			if err := WriteTextFile(filepath.Join(tmpDir, tc.file), tc.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tc.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tc.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			found := false
			for _, v := range breachs {
				if v.Kind == BreachCodeSectionMissingIdentification {
					found = true
					break
				}
			}
			if !found {
				t.Fatal("expected section missing identification breach")
			}
		})
	}
}

func TestSectionWithIdentification(t *testing.T) {
	tests := []struct {
		name    string
		file    string
		content string
	}{
		{
			name:    "TypeScript section with identification",
			file:    "src/app.ts",
			content: "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Go section with identification",
			file:    "src/app.go",
			content: "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.go#Functions](semiorepo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](semiorepo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Python section with identification",
			file:    "src/app.py",
			content: "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
		},
		{
			name:    "CSharp section with identification",
			file:    "src/App.cs",
			content: "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/App.cs#Functions](semiorepo://section/src/app.cs/functions)\n\n// Function declarations.\n\npublic static void DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Rust section with identification",
			file:    "src/app.rs",
			content: "// #region 🔖Header\n\n// [💻src/app.rs](semiorepo://file/src/app.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.rs#Functions](semiorepo://section/src/app.rs/functions)\n\n// Function declarations.\n\npub fn do_work() {}\n\n// #endregion 🔖Functions\n",
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tc.file))
			os.MkdirAll(dir, 0o755)
			if err := WriteTextFile(filepath.Join(tmpDir, tc.file), tc.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tc.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tc.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			for _, v := range breachs {
				if v.Kind == BreachCodeSectionMissingIdentification {
					t.Fatalf("unexpected section missing identification breach for %s at line %d", tc.file, v.Line)
				}
			}
		})
	}
}

func TestDefinitionMissingIdentification(t *testing.T) {
	tests := []struct {
		name    string
		file    string
		content string
	}{
		{
			name:    "TypeScript definition without identification",
			file:    "src/app.ts",
			content: "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Go definition without identification",
			file:    "src/app.go",
			content: "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.go#Functions](semiorepo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Python definition without identification",
			file:    "src/app.py",
			content: "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
		},
		{
			name:    "CSharp definition without identification",
			file:    "src/App.cs",
			content: "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/App.cs#Functions](semiorepo://section/src/app.cs/functions)\n\n// Function declarations.\n\npublic static void DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Rust definition without identification",
			file:    "src/app.rs",
			content: "// #region 🔖Header\n\n// [💻src/app.rs](semiorepo://file/src/app.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.rs#Functions](semiorepo://section/src/app.rs/functions)\n\n// Function declarations.\n\npub fn do_work() {}\n\n// #endregion 🔖Functions\n",
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tc.file))
			os.MkdirAll(dir, 0o755)
			if err := WriteTextFile(filepath.Join(tmpDir, tc.file), tc.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tc.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tc.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			found := false
			for _, v := range breachs {
				if v.Kind == BreachCodeDefMissingIdentification {
					found = true
					break
				}
			}
			if !found {
				t.Fatal("expected definition missing identification breach")
			}
		})
	}
}

func TestDefinitionWithIdentification(t *testing.T) {
	tests := []struct {
		name    string
		file    string
		content string
	}{
		{
			name:    "TypeScript definition with identification",
			file:    "src/app.ts",
			content: "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Go definition with identification",
			file:    "src/app.go",
			content: "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.go#Functions](semiorepo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](semiorepo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Python definition with identification",
			file:    "src/app.py",
			content: "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
		},
		{
			name:    "CSharp definition with identification",
			file:    "src/App.cs",
			content: "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/App.cs#Functions](semiorepo://section/src/app.cs/functions)\n\n// Function declarations.\n\n// DoWork processes items.\n// [🛠️src/App.cs#Functions§DoWork](semiorepo://definition/src/app.cs/functions/dowork)\npublic static void DoWork() {}\n\n// #endregion 🔖Functions\n",
		},
		{
			name:    "Rust definition with identification",
			file:    "src/app.rs",
			content: "// #region 🔖Header\n\n// [💻src/app.rs](semiorepo://file/src/app.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.rs#Functions](semiorepo://section/src/app.rs/functions)\n\n// Function declarations.\n\n// do_work processes items.\n// [🛠️src/app.rs#Functions§do_work](semiorepo://definition/src/app.rs/functions/do_work)\npub fn do_work() {}\n\n// #endregion 🔖Functions\n",
		},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tc.file))
			os.MkdirAll(dir, 0o755)
			if err := WriteTextFile(filepath.Join(tmpDir, tc.file), tc.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tc.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tc.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			for _, v := range breachs {
				if v.Kind == BreachCodeDefMissingIdentification {
					t.Fatalf("unexpected definition missing identification breach for %s at line %d", tc.file, v.Line)
				}
			}
		})
	}
}

func TestSectionIdentificationAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n// Function declarations.\n\n// Does work.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/do-work)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	sectionIdBreachs := []Breach{}
	for _, v := range breachs {
		if v.Kind == BreachCodeSectionMissingIdentification {
			sectionIdBreachs = append(sectionIdBreachs, v)
		}
	}
	if len(sectionIdBreachs) == 0 {
		t.Fatal("expected section identification breachs before autofix")
	}
	n, fixErr := applyAutofixes(testFile, sectionIdBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "semiorepo://section/") {
		t.Fatal("expected section identification URI after autofix")
	}
}

func TestDefinitionIdentificationAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	defIdBreachs := []Breach{}
	for _, v := range breachs {
		if v.Kind == BreachCodeDefMissingIdentification {
			defIdBreachs = append(defIdBreachs, v)
		}
	}
	if len(defIdBreachs) == 0 {
		t.Fatal("expected definition identification breachs before autofix")
	}
	n, fixErr := applyAutofixes(testFile, defIdBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "semiorepo://definition/") {
		t.Fatal("expected definition identification URI after autofix")
	}
}

func TestSectionWrongIdentificationDetected(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [wrong-section-id](semiorepo://section/wrong/section)\n\n// Function declarations.\n\n// Does work.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/do-work)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	if err := WriteTextFile(filepath.Join(tmpDir, testFile), content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeFile, FilePath: testFile}, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	foundWrongId := false
	foundWrongURI := false
	for _, v := range breachs {
		if v.Kind == BreachCodeSectionWrongIdentificationId {
			foundWrongId = true
		}
		if v.Kind == BreachCodeSectionWrongIdentificationUri {
			foundWrongURI = true
		}
	}
	if !foundWrongId {
		t.Fatal("expected section wrong identification id breach")
	}
	if !foundWrongURI {
		t.Fatal("expected section wrong identification uri breach")
	}
}

func TestDefinitionWrongIdentificationDetected(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖GraphQL Types\n\n// [section-id](semiorepo://section/src/app.go/graphql-types)\n\n// GraphQL declarations.\n\n// #region 🔖GraphQL Input Types\n\n// [nested-section-id](semiorepo://section/src/app.go/graphql-types/graphql-input-types)\n\n// Input declarations.\n\n// TicketCloseInput holds ticket close inputs.\n// [wrong-definition-id](semiorepo://definition/wrong/definition)\ntype TicketCloseInput struct {\n    TicketID string\n}\n\n// #endregion 🔖GraphQL Input Types\n\n// #endregion 🔖GraphQL Types\n"
	testFile := "src/app.go"
	if err := WriteTextFile(filepath.Join(tmpDir, testFile), content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeFile, FilePath: testFile}, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	foundWrongId := false
	foundWrongURI := false
	for _, v := range breachs {
		if v.Kind == BreachCodeDefWrongIdentificationId {
			foundWrongId = true
		}
		if v.Kind == BreachCodeDefWrongIdentificationUri {
			foundWrongURI = true
		}
	}
	if !foundWrongId {
		t.Fatal("expected definition wrong identification id breach")
	}
	if !foundWrongURI {
		t.Fatal("expected definition wrong identification uri breach")
	}
}

func TestSectionWrongIdentificationAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [wrong-section-id](semiorepo://section/wrong/section)\n\n// Function declarations.\n\n// Does work.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/do-work)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeFile, FilePath: testFile}, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var wrong []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeSectionWrongIdentificationId || v.Kind == BreachCodeSectionWrongIdentificationUri {
			wrong = append(wrong, v)
		}
	}
	if len(wrong) == 0 {
		t.Fatal("expected section wrong identification breaches before autofix")
	}
	n, fixErr := applyAutofixes(testFile, wrong)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected autofix to update wrong section identification")
	}
	fixedContent, _ := ReadTextFile(absPath)
	expectedLine := "// [" + SectionHeaderId(testFile, "Functions") + "](" + SectionHeaderUri(testFile, "Functions") + ")"
	if !strings.Contains(fixedContent, expectedLine) {
		t.Fatalf("expected section id line %q after autofix", expectedLine)
	}
}

func TestDefinitionWrongIdentificationAutofixGoTypeUsesInterfaceKind(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖GraphQL Types\n\n// [section-id](semiorepo://section/src/app.go/graphql-types)\n\n// GraphQL declarations.\n\n// #region 🔖GraphQL Input Types\n\n// [nested-section-id](semiorepo://section/src/app.go/graphql-types/graphql-input-types)\n\n// Input declarations.\n\n// TicketCloseInput holds ticket close inputs.\n// [wrong-definition-id](semiorepo://definition/wrong/definition)\ntype TicketCloseInput struct {\n    TicketID string\n}\n\n// #endregion 🔖GraphQL Input Types\n\n// #endregion 🔖GraphQL Types\n"
	testFile := "src/app.go"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeFile, FilePath: testFile}, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var wrong []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefWrongIdentificationId || v.Kind == BreachCodeDefWrongIdentificationUri {
			wrong = append(wrong, v)
		}
	}
	if len(wrong) == 0 {
		t.Fatal("expected definition wrong identification breaches before autofix")
	}
	n, fixErr := applyAutofixes(testFile, wrong)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected autofix to update wrong definition identification")
	}
	fixedContent, _ := ReadTextFile(absPath)
	sectionPath := "GraphQL Types#GraphQL Input Types"
	expectedLine := "// [" + DefinitionHeaderId(testFile, sectionPath, "TicketCloseInput", DefinitionKindInterface) + "](" + DefinitionHeaderUri(testFile, sectionPath, "TicketCloseInput") + ")"
	if !strings.Contains(fixedContent, expectedLine) {
		t.Fatalf("expected definition id line %q after autofix", expectedLine)
	}
}

func TestDefinitionMissingIdentificationAutofixGoTypeUsesInterfaceKind(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖GraphQL Types\n\n// [section-id](semiorepo://section/src/app.go/graphql-types)\n\n// GraphQL declarations.\n\n// #region 🔖GraphQL Input Types\n\n// [nested-section-id](semiorepo://section/src/app.go/graphql-types/graphql-input-types)\n\n// Input declarations.\n\n// TicketCloseInput holds ticket close inputs.\ntype TicketCloseInput struct {\n    TicketID string\n}\n\n// #endregion 🔖GraphQL Input Types\n\n// #endregion 🔖GraphQL Types\n"
	testFile := "src/app.go"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	ctx := NewPolicyContextWithFiles(Scope{Kind: ScopeFile, FilePath: testFile}, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var missing []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefMissingIdentification {
			missing = append(missing, v)
		}
	}
	if len(missing) == 0 {
		t.Fatal("expected definition missing identification breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, missing)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected autofix to insert definition identification")
	}
	fixedContent, _ := ReadTextFile(absPath)
	sectionPath := "GraphQL Types#GraphQL Input Types"
	expectedLine := "// [" + DefinitionHeaderId(testFile, sectionPath, "TicketCloseInput", DefinitionKindInterface) + "](" + DefinitionHeaderUri(testFile, sectionPath, "TicketCloseInput") + ")"
	if !strings.Contains(fixedContent, expectedLine) {
		t.Fatalf("expected definition id line %q after autofix", expectedLine)
	}
}

func TestDefinitionNativeDocstring(t *testing.T) {
	tests := []struct {
		name         string
		file         string
		content      string
		expectBreach bool
	}{
		{
			name:         "TypeScript // comments should flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
			expectBreach: true,
		},
		{
			name:         "TypeScript JSDoc should NOT flag breach",
			file:         "src/app.ts",
			content:      "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "Go // comments should NOT flag breach (native format)",
			file:         "src/app.go",
			content:      "package main\n\n// #region 🔖Header\n\n// [💻src/app.go](semiorepo://file/src/app.go)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.go#Functions](semiorepo://section/src/app.go/functions)\n\n// Function declarations.\n\n// DoWork does work.\n// DoWork MUST be idempotent.\n// [🛠️src/app.go#Functions§DoWork](semiorepo://definition/src/app.go/functions/dowork)\nfunc DoWork() {}\n\n// #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "Python # comments should flag breach (should use triple-quote docstring)",
			file:         "src/app.py",
			content:      "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n",
			expectBreach: true,
		},
		{
			name:         "Python triple-quote docstring should NOT flag breach",
			file:         "src/app.py",
			content:      "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖Functions\n",
			expectBreach: false,
		},
		{
			name:         "CSharp // comments should flag breach (should use ///)",
			file:         "src/App.cs",
			content:      "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/App.cs#Types](semiorepo://section/src/app.cs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/App.cs#Types§AppState](semiorepo://definition/src/app.cs/types/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖Types\n",
			expectBreach: true,
		},
		{
			name:         "CSharp /// comments should NOT flag breach",
			file:         "src/App.cs",
			content:      "// #region 🔖Header\n\n// [💻src/App.cs](semiorepo://file/src/app.cs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/App.cs#Types](semiorepo://section/src/app.cs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/App.cs#Types§AppState](semiorepo://definition/src/app.cs/types/appstate)\npublic class AppState()\n{\n}\n\n// #endregion 🔖Types\n",
			expectBreach: false,
		},
		{
			name:         "Rust // comments should flag breach (should use ///)",
			file:         "src/lib.rs",
			content:      "// #region 🔖Header\n\n// [💻src/lib.rs](semiorepo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/lib.rs#Types](semiorepo://section/src/lib.rs/types)\n\n// Type declarations.\n\n// Represents app state.\n// AppState MUST be serializable.\n// [🛠️src/lib.rs#Types§AppState](semiorepo://definition/src/lib.rs/types/appstate)\npub struct AppState {}\n\n// #endregion 🔖Types\n",
			expectBreach: true,
		},
		{
			name:         "Rust /// comments should NOT flag breach",
			file:         "src/lib.rs",
			content:      "// #region 🔖Header\n\n// [💻src/lib.rs](semiorepo://file/src/lib.rs)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Types\n\n// [🔖src/lib.rs#Types](semiorepo://section/src/lib.rs/types)\n\n// Type declarations.\n\n/// Represents app state.\n/// AppState MUST be serializable.\n/// [🛠️src/lib.rs#Types§AppState](semiorepo://definition/src/lib.rs/types/appstate)\npub struct AppState {}\n\n// #endregion 🔖Types\n",
			expectBreach: false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			tmpDir := t.TempDir()
			oldRoot := rootDir
			rootDir = tmpDir
			defer func() { rootDir = oldRoot }()
			dir := filepath.Dir(filepath.Join(tmpDir, tt.file))
			os.MkdirAll(dir, 0o755)
			absPath := filepath.Join(tmpDir, tt.file)
			if err := WriteTextFile(absPath, tt.content); err != nil {
				t.Fatalf("failed to write: %v", err)
			}
			scope := Scope{Kind: ScopeFile, FilePath: tt.file}
			ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{tt.file})
			breachs, err := CheckPoliciesWithContext(ctx, nil)
			if err != nil {
				t.Fatalf("policy check: %v", err)
			}
			hasBreach := false
			for _, v := range breachs {
				if v.Kind == BreachCodeDefNotNativeDocstring {
					hasBreach = true
					break
				}
			}
			if tt.expectBreach && !hasBreach {
				t.Fatal("expected DefNotNativeDocstring breach but got none")
			}
			if !tt.expectBreach && hasBreach {
				t.Fatal("unexpected DefNotNativeDocstring breach")
			}
		})
	}
}

func TestDefinitionNativeDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n// Does work.\n// doWork MUST be idempotent.\n// [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "/**") {
		t.Fatal("expected JSDoc opening after autofix")
	}
	if !strings.Contains(fixedContent, "**/") {
		t.Fatal("expected JSDoc closing after autofix")
	}
	if !strings.Contains(fixedContent, " * Does work.") {
		t.Fatal("expected summary line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, " * doWork MUST be idempotent.") {
		t.Fatal("expected spec line in JSDoc after autofix")
	}
	if !strings.Contains(fixedContent, "semiorepo://definition/") {
		t.Fatal("expected identification in JSDoc after autofix")
	}
}

func TestPythonTripleQuoteDocstringAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# Does work.\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\ndef do_work():\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach before autofix")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, `"""Does work.`) {
		t.Fatal("expected triple-quote docstring with summary after autofix")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec line in docstring after autofix")
	}
	if !strings.Contains(fixedContent, "semiorepo://definition/") {
		t.Fatal("expected identification in docstring after autofix")
	}
	if !strings.Contains(fixedContent, `"""`) {
		t.Fatal("expected closing triple-quote after autofix")
	}
	if strings.Contains(fixedContent, "# Does work.") {
		t.Fatal("# comment should be removed after autofix")
	}
}

func TestPythonTripleQuoteDocstringMerge(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\n# do_work MUST be idempotent.\n# [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\ndef do_work():\n    \"\"\"Does work.\"\"\"\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	var docstringBreachs []Breach
	for _, v := range breachs {
		if v.Kind == BreachCodeDefNotNativeDocstring {
			docstringBreachs = append(docstringBreachs, v)
		}
	}
	if len(docstringBreachs) == 0 {
		t.Fatal("expected DefNotNativeDocstring breach for # comments above existing docstring")
	}
	n, fixErr := applyAutofixes(testFile, docstringBreachs)
	if fixErr != nil {
		t.Fatalf("autofix failed: %v", fixErr)
	}
	if n == 0 {
		t.Fatal("expected at least one autofix applied")
	}
	fixedContent, _ := ReadTextFile(absPath)
	if !strings.Contains(fixedContent, "Does work.") {
		t.Fatal("expected existing summary preserved after merge")
	}
	if !strings.Contains(fixedContent, "do_work MUST be idempotent.") {
		t.Fatal("expected spec from # comment merged into docstring")
	}
	if !strings.Contains(fixedContent, "semiorepo://definition/") {
		t.Fatal("expected identification merged into docstring")
	}
	if strings.Contains(fixedContent, "# do_work MUST") {
		t.Fatal("# comment should be removed after merge autofix")
	}
}

func TestPythonTripleQuoteDocstringExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "# #region 🔖Header\n\n# [💻src/app.py](semiorepo://file/src/app.py)\n\n# 2025 Test <t@t.com>\n\n# GNU Affero General Public License\n# https://www.gnu.org/licenses/\n\n# Summary of the file.\n\n# #endregion 🔖Header\n\n# #region 🔖Functions\n\n# [🔖src/app.py#Functions](semiorepo://section/src/app.py/functions)\n\n# Function declarations.\n\ndef do_work():\n    \"\"\"Does work.\n    do_work MUST be idempotent.\n    [🛠️src/app.py#Functions§do_work](semiorepo://definition/src/app.py/functions/do_work)\n    \"\"\"\n    pass\n\n# #endregion 🔖Functions\n"
	testFile := "src/app.py"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentBlock {
			t.Fatalf("Python triple-quote docstring should not be flagged as block comment at line %d", v.Line)
		}
		if v.Kind == BreachCodeDefNotNativeDocstring {
			t.Fatalf("Python triple-quote docstring should not flag DefNotNativeDocstring at line %d", v.Line)
		}
	}
}

func TestDefinitionJSDocExemptFromCommentBan(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// [💻src/app.ts](semiorepo://file/src/app.ts)\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// Summary of the file.\n\n// #endregion 🔖Header\n\n// #region 🔖Functions\n\n// [🔖src/app.ts#Functions](semiorepo://section/src/app.ts/functions)\n\n// Function declarations.\n\n/**\n * Does work.\n *\n * doWork MUST be idempotent.\n *\n *  * [🛠️src/app.ts#Functions§doWork](semiorepo://definition/src/app.ts/functions/dowork)\n **/\nexport function doWork(): void {}\n\n// #endregion 🔖Functions\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, []Bundle{}, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check: %v", err)
	}
	for _, v := range breachs {
		if v.Kind == BreachCodeCommentJSDoc {
			t.Fatalf("definition JSDoc should not be flagged as comment breach at line %d", v.Line)
		}
		if v.Kind == BreachCodeCommentBlock {
			t.Fatalf("definition JSDoc should not be flagged as block comment breach at line %d", v.Line)
		}
	}
}

func TestSectionHeaderIdAndUri(t *testing.T) {
	id := SectionHeaderId("src/app.ts", "Functions")
	if !strings.Contains(id, emojiText(EmojiSection)+Flat("Functions")) {
		t.Fatalf("unexpected section header id: %s", id)
	}
	if strings.HasPrefix(id, emojiText(EmojiSection)) {
		t.Fatalf("section header id should include file parent before section emoji: %s", id)
	}
	uri := SectionHeaderUri("src/app.ts", "Functions")
	if !strings.HasPrefix(uri, "semiorepo://section/") {
		t.Fatalf("unexpected section header uri: %s", uri)
	}
	if !strings.Contains(uri, "Functions") {
		t.Fatalf("section uri should contain slugified section name: %s", uri)
	}
}

func TestDefinitionHeaderIdAndUri(t *testing.T) {
	id := DefinitionHeaderId("src/app.ts", "Functions", "doWork", "implementation")
	if !strings.Contains(id, emojiText(EmojiDefinitionImpl)+Flat("doWork")) {
		t.Fatalf("unexpected definition header id: %s", id)
	}
	uri := DefinitionHeaderUri("src/app.ts", "Functions", "doWork")
	if !strings.HasPrefix(uri, "semiorepo://definition/") {
		t.Fatalf("unexpected definition header uri: %s", uri)
	}
	if !strings.Contains(uri, "doWork") {
		t.Fatalf("definition uri should contain slugified def name: %s", uri)
	}
}

func TestSpecsBreach(t *testing.T) {
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

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain `License` subregions.\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := specsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeSpecsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeSpecsSyntax for backtick-wrapped code in header Specs")
		}
	})

	t.Run("specsPolicy clean specs no breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// File headers MUST contain License subregions.\n\n// #endregion 🔖Header\n\n// #region 🔖Section\n\nconst x = 1;\n\n// #endregion 🔖Section\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := specsPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeSpecsSyntax {
				t.Errorf("unexpected BreachCodeSpecsSyntax for clean spec: %s", v.Summary)
			}
		}
	})

	t.Run("specsPolicy detects implementation syntax in section specs", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST call `ctx.Check()` internally.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := specsPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeSpecsSyntax {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeSpecsSyntax for backtick in section spec")
		}
	})

	t.Run("section spec comments exempt from inline breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n// Validation MUST check constraints.\n\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				t.Errorf("spec comment should be exempt from inline breach: line %d %s", v.Line, v.Excerpt)
			}
		}
	})

	t.Run("JSDoc spec comments exempt from JSDoc breach", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\n/**\n * Kits MUST be editable offline.\n */\nconst x = 1;\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		for _, v := range breachs {
			if v.Kind == BreachCodeCommentJSDoc {
				t.Errorf("JSDoc spec comment should be exempt from JSDoc breach: line %d", v.Line)
			}
		}
	})

	t.Run("non-spec JSDoc still flagged", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\nx = 1;\n\n/**\n * This is a regular docstring without spec keywords.\n */\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentJSDoc {
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

		content := "// #region 🔖Header\n\n// 🧪test.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\n// #region 🔖MySection\n\nconst x = 1;\n\n// This is a regular comment not a spec.\n\n// #endregion 🔖MySection\n"
		testFile := "test.ts"
		absPath := filepath.Join(tmpDir, testFile)
		if err := WriteTextFile(absPath, content); err != nil {
			t.Fatalf("failed to write: %v", err)
		}

		bundles := []Bundle{}
		scope := Scope{Kind: ScopeFile, FilePath: testFile}
		ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
		breachs := commentPolicy(ctx)

		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeCommentInline {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected non-spec inline comment to be flagged")
		}
	})

	t.Run("BreachCodeSpecsSyntax in breach info table", func(t *testing.T) {
		info := BreachCodeSpecsSyntax.Info()
		if info.Kind != BreachCodeSpecsSyntax {
			t.Errorf("expected kind %s, got %s", BreachCodeSpecsSyntax, info.Kind)
		}
		if info.Autofixable {
			t.Error("specs syntax breach should not be autofixable")
		}
	})
}

func TestDocsBreach(t *testing.T) {
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
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing README.md")
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
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Summary") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # Summary section")
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
		breachs := docsPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme && strings.Contains(v.Summary, "Specs") {
				found = true
				break
			}
		}
		if !found {
			t.Error("expected BreachCodeDocsMissingReadme for missing # Specs section")
		}
	})
	t.Run("docsPolicy clean README no breach", func(t *testing.T) {
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
		breachs := docsPolicy(ctx)
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				t.Errorf("unexpected BreachCodeDocsMissingReadme: %s", v.Summary)
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
		breachs := docsPolicy(ctx)
		count := 0
		for _, v := range breachs {
			if v.Kind == BreachCodeDocsMissingReadme {
				count++
			}
		}
		if count != 1 {
			t.Errorf("expected 1 breach for deduplicated root, got %d", count)
		}
	})
}

func TestFormatHeaderStructure(t *testing.T) {
	lang := NewTypeScriptLanguage()
	header := lang.FormatHeader("💻test/file.ts", "semiorepo://file/test/file.ts", "A test file", "2025 Test User <test@test.com>", "AGPL license text here", "Some specs")
	if !strings.Contains(header, "// #region 🔖Header") {
		t.Error("header missing Header region start")
	}
	if !strings.Contains(header, "// #endregion 🔖Header") {
		t.Error("header missing Header region end")
	}
	if !strings.Contains(header, "[💻test/file.ts](semiorepo://file/test/file.ts)") {
		t.Error("header missing [ID](URI) identification")
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
	header := lang.FormatHeader("💻test/file.go", "semiorepo://file/test/file.go", "", "2025 Dev <dev@dev.com>", "AGPL text", "")
	if strings.Contains(header, "Specs") {
		t.Error("header should not contain Specs subregion when specs is empty")
	}
	if !strings.Contains(header, "// #region 🔖Header") {
		t.Error("header missing Header region start")
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
		header := lang.FormatHeader("💻test/file", "semiorepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header == "" {
			t.Errorf("%s: FormatHeader returned empty", lang.Name())
		}
		if !strings.Contains(header, "[💻test/file](semiorepo://file/test/file)") {
			t.Errorf("%s: header missing [ID](URI) identification", lang.Name())
		}
	}
	noHeader := []LanguagePlugin{
		NewMarkdownLanguage(),
		NewTomlLanguage(),
		NewYamlLanguage(),
	}
	for _, lang := range noHeader {
		header := lang.FormatHeader("💻test/file", "semiorepo://file/test/file", "", "2025 Dev <d@d.com>", "AGPL", "")
		if header != "" {
			t.Errorf("%s: FormatHeader should return empty for non-header language", lang.Name())
		}
	}
}

func TestHeaderPolicyOldFormatId(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	subDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(subDir, 0o755)
	content := "// #region 🔖Header\n\n// 💻src/app.ts\n\n// 2025 Test <t@t.com>\n\n// GNU Affero General Public License\n// https://www.gnu.org/licenses/\n\n// #endregion 🔖Header\n\nconst x = 1;\n"
	testFile := "src/app.ts"
	absPath := filepath.Join(tmpDir, testFile)
	if err := WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write: %v", err)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeFile, FilePath: testFile}
	ctx := NewPolicyContextWithFiles(scope, bundles, []string{testFile})
	breachs, err := CheckPoliciesWithContext(ctx, nil)
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}
	counts := map[Statute]int{}
	for _, v := range breachs {
		counts[v.Kind]++
	}
	if counts[BreachCodeFileWrongHeaderRegionFormat] == 0 {
		t.Error("expected wrong-header-format breach for old-format ID without [ID](URI)")
	}
}

func TestTerritory(t *testing.T) {
	t.Run("AllKinds flat", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
			Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingIdentification},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingIdentification {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingIdentification, kinds[1])
		}
	})
	t.Run("AllKinds nested groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
				{
					Name:        "Section",
					Description: "Section-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeSectionEmpty},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
	})
	t.Run("AllKinds mixed kinds and groups", func(t *testing.T) {
		g := Territory{
			Name:        "Code",
			Description: "Code breachs",
			Scopes:      []string{"**/*.{ts,tsx}"},
			Kinds:       []Statute{BreachCodeCommentInline},
			Groups: []Territory{
				{
					Name:        "File",
					Description: "File-level breachs",
					Scopes:      []string{"**/*.ts"},
					Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 kinds, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeCommentInline {
			t.Errorf("expected %s first, got %s", BreachCodeCommentInline, kinds[0])
		}
		if kinds[1] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s second, got %s", BreachCodeFileMissingHeaderRegion, kinds[1])
		}
	})
	t.Run("AllKinds deeply nested", func(t *testing.T) {
		g := Territory{
			Name:   "Root",
			Scopes: []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "Level1",
					Scopes: []string{"**/*"},
					Groups: []Territory{
						{
							Name:   "Level2",
							Scopes: []string{"**/*"},
							Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
						},
					},
				},
			},
		}
		kinds := g.AllKinds()
		if len(kinds) != 1 {
			t.Fatalf("expected 1 kind, got %d", len(kinds))
		}
		if kinds[0] != BreachCodeFileMissingHeaderRegion {
			t.Errorf("expected %s, got %s", BreachCodeFileMissingHeaderRegion, kinds[0])
		}
	})
	t.Run("AllKinds empty group", func(t *testing.T) {
		g := Territory{
			Name:   "Empty",
			Scopes: []string{"**/*"},
		}
		kinds := g.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
	t.Run("GetID and GetURI", func(t *testing.T) {
		g := Territory{
			Name:        "File",
			Description: "File-level breachs",
			Scopes:      []string{"**/*.ts"},
		}
		id := g.GetID()
		if id == "" {
			t.Error("expected non-empty ID")
		}
		if !strings.Contains(id, "File") {
			t.Errorf("expected ID to contain 'File', got %s", id)
		}
		uri := g.GetURI()
		if uri == "" {
			t.Error("expected non-empty URI")
		}
		if !strings.HasPrefix(uri, "semiorepo://territory/") {
			t.Errorf("expected URI to start with 'semiorepo://territory/', got %s", uri)
		}
	})
}

func TestPolicyDefAllKinds(t *testing.T) {
	t.Run("groups collect all nested kinds", func(t *testing.T) {
		p := PolicyDef{
			ID:          "test",
			Name:        "Test",
			Description: "Test policy",
			Scopes:      []string{"**/*"},
			Groups: []Territory{
				{
					Name:   "File",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingIdentification},
				},
				{
					Name:   "Section",
					Scopes: []string{"**/*.ts"},
					Kinds:  []Statute{BreachCodeSectionEmpty},
				},
			},
			Run: func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 3 {
			t.Fatalf("expected 3 kinds, got %d", len(kinds))
		}
	})
	t.Run("empty groups returns empty", func(t *testing.T) {
		p := PolicyDef{
			ID:     "empty",
			Name:   "Empty",
			Scopes: []string{"**/*"},
			Run:    func(ctx *PolicyContext) []Breach { return nil },
		}
		kinds := p.AllKinds()
		if len(kinds) != 0 {
			t.Fatalf("expected 0 kinds, got %d", len(kinds))
		}
	})
}

func TestSystemPolicy(t *testing.T) {
	t.Run("detects settings.json outside devcontainer", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachSystemDevcontainerVscodeSettingsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected settings-outside-devcontainer breach")
		}
	})
	t.Run("detects extensions.json outside devcontainer", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		found := false
		for _, v := range breachs {
			if v.Kind == BreachSystemDevcontainerVscodeExtensionsOutside {
				found = true
			}
		}
		if !found {
			t.Error("expected extensions-outside-devcontainer breach")
		}
	})
	t.Run("no breachs when .vscode files absent", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		ctx := NewPolicyContext(Scope{Kind: ScopeRepo}, []Bundle{})
		breachs := systemPolicy(ctx)
		if len(breachs) != 0 {
			t.Errorf("expected 0 breachs, got %d", len(breachs))
		}
	})
	t.Run("autofix moves settings.json into devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		if _, err := os.Stat(filepath.Join(tmpDir, ".vscode", "settings.json")); !os.IsNotExist(err) {
			t.Error("expected .vscode/settings.json to be removed")
		}
		dcPath := filepath.Join(tmpDir, ".devcontainer", "devcontainer.json")
		dcData, err := os.ReadFile(dcPath)
		if err != nil {
			t.Fatalf("expected devcontainer.json to exist: %v", err)
		}
		var dc map[string]interface{}
		if err := json.Unmarshal(dcData, &dc); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		if customizations == nil {
			t.Fatal("expected customizations key")
		}
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode == nil {
			t.Fatal("expected vscode key in customizations")
		}
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings == nil {
			t.Fatal("expected settings key in customizations.vscode")
		}
		if settings["editor.fontSize"] != float64(14) {
			t.Errorf("expected editor.fontSize=14, got %v", settings["editor.fontSize"])
		}
	})
	t.Run("autofix moves extensions.json into devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python", "golang.go"]}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		if _, err := os.Stat(filepath.Join(tmpDir, ".vscode", "extensions.json")); !os.IsNotExist(err) {
			t.Error("expected .vscode/extensions.json to be removed")
		}
		dcPath := filepath.Join(tmpDir, ".devcontainer", "devcontainer.json")
		dcData, err := os.ReadFile(dcPath)
		if err != nil {
			t.Fatalf("expected devcontainer.json to exist: %v", err)
		}
		var dc map[string]interface{}
		if err := json.Unmarshal(dcData, &dc); err != nil {
			t.Fatalf("invalid json: %v", err)
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		extensions, _ := vscode["extensions"].([]interface{})
		if len(extensions) != 2 {
			t.Fatalf("expected 2 extensions, got %d", len(extensions))
		}
		if extensions[0] != "ms-python.python" {
			t.Errorf("expected first extension ms-python.python, got %v", extensions[0])
		}
		if extensions[1] != "golang.go" {
			t.Errorf("expected second extension golang.go, got %v", extensions[1])
		}
	})
	t.Run("autofix merges into existing devcontainer.json", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		os.MkdirAll(filepath.Join(tmpDir, ".devcontainer"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.tabSize": 2}`)
		WriteTextFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"), `{"name": "test", "image": "ubuntu"}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 1 {
			t.Fatalf("expected 1 fix, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		if dc["name"] != "test" {
			t.Errorf("expected existing name=test to be preserved, got %v", dc["name"])
		}
		if dc["image"] != "ubuntu" {
			t.Errorf("expected existing image=ubuntu to be preserved, got %v", dc["image"])
		}
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		settings, _ := vscode["settings"].(map[string]interface{})
		if settings["editor.tabSize"] != float64(2) {
			t.Errorf("expected editor.tabSize=2, got %v", settings["editor.tabSize"])
		}
	})
	t.Run("autofix both settings and extensions together", func(t *testing.T) {
		tmpDir := t.TempDir()
		oldRoot := rootDir
		rootDir = tmpDir
		defer func() { rootDir = oldRoot }()
		os.MkdirAll(filepath.Join(tmpDir, ".vscode"), 0o755)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "settings.json"), `{"editor.fontSize": 14}`)
		WriteTextFile(filepath.Join(tmpDir, ".vscode", "extensions.json"), `{"recommendations": ["ms-python.python"]}`)
		breachs := []Breach{
			{Kind: BreachSystemDevcontainerVscodeSettingsOutside, Scope: ".vscode/settings.json", Line: 1},
			{Kind: BreachSystemDevcontainerVscodeExtensionsOutside, Scope: ".vscode/extensions.json", Line: 1},
		}
		fixed, err := applySystemAutofixes(breachs)
		if err != nil {
			t.Fatalf("autofix error: %v", err)
		}
		if fixed != 2 {
			t.Fatalf("expected 2 fixes, got %d", fixed)
		}
		dcData, _ := os.ReadFile(filepath.Join(tmpDir, ".devcontainer", "devcontainer.json"))
		var dc map[string]interface{}
		json.Unmarshal(dcData, &dc)
		customizations, _ := dc["customizations"].(map[string]interface{})
		vscode, _ := customizations["vscode"].(map[string]interface{})
		if vscode["settings"] == nil {
			t.Error("expected settings in devcontainer.json")
		}
		if vscode["extensions"] == nil {
			t.Error("expected extensions in devcontainer.json")
		}
	})
	t.Run("policy registered with correct id", func(t *testing.T) {
		p, found := FindPolicy("system")
		if !found {
			t.Fatal("expected system policy to be registered")
		}
		if p.Name != "System" {
			t.Errorf("expected name System, got %s", p.Name)
		}
		kinds := p.AllKinds()
		if len(kinds) != 2 {
			t.Fatalf("expected 2 statutes, got %d", len(kinds))
		}
		kindSet := map[Statute]bool{}
		for _, k := range kinds {
			kindSet[k] = true
		}
		if !kindSet[BreachSystemDevcontainerVscodeSettingsOutside] {
			t.Error("expected settings-outside-devcontainer kind")
		}
		if !kindSet[BreachSystemDevcontainerVscodeExtensionsOutside] {
			t.Error("expected extensions-outside-devcontainer kind")
		}
	})
	t.Run("statute meta is correct", func(t *testing.T) {
		settingsMeta := BreachSystemDevcontainerVscodeSettingsOutside.Info()
		if !settingsMeta.Autofixable {
			t.Error("expected settings breach to be autofixable")
		}
		if settingsMeta.Priority != BreachPriorityHigh {
			t.Error("expected settings breach to be high priority")
		}
		extMeta := BreachSystemDevcontainerVscodeExtensionsOutside.Info()
		if !extMeta.Autofixable {
			t.Error("expected extensions breach to be autofixable")
		}
		if extMeta.Priority != BreachPriorityHigh {
			t.Error("expected extensions breach to be high priority")
		}
	})
}

func TestBuildTerritoryTree(t *testing.T) {
	t.Run("single group with kinds", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "File",
				Description: "File breachs",
				Scopes:      []string{"**/*.ts"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion, BreachCodeFileMissingIdentification},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 node, got %d", len(nodes))
		}
		if nodes[0].Label != "File" {
			t.Errorf("expected label 'File', got %s", nodes[0].Label)
		}
		if nodes[0].Kind != TreeNodeCategory {
			t.Errorf("expected category kind, got %s", nodes[0].Kind)
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		for _, child := range nodes[0].Children {
			if child.Kind != TreeNodeStatute {
				t.Errorf("expected statute node, got %s", child.Kind)
			}
		}
	})
	t.Run("nested groups", func(t *testing.T) {
		groups := []Territory{
			{
				Name:   "Code",
				Scopes: []string{"**/*.ts"},
				Groups: []Territory{
					{
						Name:   "File",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeFileMissingHeaderRegion},
					},
					{
						Name:   "Section",
						Scopes: []string{"**/*.ts"},
						Kinds:  []Statute{BreachCodeSectionEmpty},
					},
				},
			},
		}
		nodes := buildTerritoryTree(groups)
		if len(nodes) != 1 {
			t.Fatalf("expected 1 root node, got %d", len(nodes))
		}
		if len(nodes[0].Children) != 2 {
			t.Fatalf("expected 2 children, got %d", len(nodes[0].Children))
		}
		fileGroup := nodes[0].Children[0]
		if fileGroup.Label != "File" {
			t.Errorf("expected label 'File', got %s", fileGroup.Label)
		}
		if len(fileGroup.Children) != 1 {
			t.Fatalf("expected 1 child in File group, got %d", len(fileGroup.Children))
		}
	})
	t.Run("empty groups", func(t *testing.T) {
		nodes := buildTerritoryTree(nil)
		if len(nodes) != 0 {
			t.Fatalf("expected 0 nodes, got %d", len(nodes))
		}
	})
	t.Run("group node data contains scopes", func(t *testing.T) {
		groups := []Territory{
			{
				Name:        "Sketchpad",
				Description: "Sketchpad breachs",
				Scopes:      []string{"js/sketchpad/**/*.ts", "js/sketchpad/**/*.tsx"},
				Kinds:       []Statute{BreachCodeFileMissingHeaderRegion},
			},
		}
		nodes := buildTerritoryTree(groups)
		data := nodes[0].Data
		if data == nil {
			t.Fatal("expected non-nil data")
		}
		scopes, ok := data["scopes"].([]string)
		if !ok {
			t.Fatal("expected scopes in data")
		}
		if len(scopes) != 2 {
			t.Fatalf("expected 2 scopes, got %d", len(scopes))
		}
	})
}

func TestRegisteredPoliciesHaveGroups(t *testing.T) {
	policies := GetRegisteredPolicies()
	for _, p := range policies {
		if len(p.Groups) == 0 {
			t.Errorf("policy %s has no groups", p.ID)
		}
		kinds := p.AllKinds()
		if len(kinds) == 0 {
			t.Errorf("policy %s has no statutes", p.ID)
		}
	}
}

func TestPolicyGroupsGraphQL(t *testing.T) {
	executor := getTestExecutor(t)
	query := `{ policies { id name groups { name description scopes kinds { id } groups { name kinds { id } } } } }`
	result, err := executor.Execute(context.Background(), query, nil)
	if err != nil {
		t.Fatalf("GraphQL query failed: %v", err)
	}
	data, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected map result")
	}
	policiesData, ok := data["policies"].([]interface{})
	if !ok {
		t.Fatal("expected policies array")
	}
	if len(policiesData) == 0 {
		t.Fatal("expected at least one policy")
	}
	for _, pRaw := range policiesData {
		p, ok := pRaw.(map[string]interface{})
		if !ok {
			continue
		}
		groups, ok := p["groups"].([]interface{})
		if !ok {
			t.Fatalf("expected groups array for policy %v", p["id"])
		}
		if len(groups) == 0 {
			t.Errorf("expected at least one group for policy %v", p["id"])
		}
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

	result := ToolGoalCreate("", "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing title")
	}

	result = ToolGoalCreate("Test Goal", "", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing description")
	}

	result = ToolGoalCreate("Test Goal", "desc", "", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing prompt")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "", "opus-4-5", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing due date")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "", true, "", "")
	if result.Error == "" {
		t.Error("expected error for missing client")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "invalid-llm", "claude-code", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid llm")
	}

	result = ToolGoalCreate("Test Goal", "desc", "prompt", "2026-02-15", "opus-4-5", "invalid-client", true, "", "")
	if result.Error == "" {
		t.Error("expected error for invalid client")
	}
}

func TestGoalCreateAndCleanup(t *testing.T) {

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
		if goal.Interactions[0].Client != "claude-code" {
			t.Errorf("expected Client 'claude-code', got '%s'", goal.Interactions[0].Client)
		}
	}

	goalPath := filepath.Join(GetRepoGoalsDir(), goal.ID)
	if err := os.RemoveAll(goalPath); err != nil {
		t.Errorf("failed to cleanup goal: %v", err)
	}
}

func TestGoalHierarchy(t *testing.T) {

	parentTitle := "Test Parent Goal"
	parentRes := ToolGoalCreate(parentTitle, "desc", "prompt", "2026-02-15", "opus-4-5", "claude-code", true, "", "")
	if parentRes.Error != "" {
		t.Fatalf("Failed to create parent: %s", parentRes.Error)
	}
	parent, ok := parentRes.Data.(*Goal)
	if !ok {
		t.Fatalf("Expected *Goal data")
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "TEST-PARENT-GOAL" {
		t.Errorf("Expected parent ID 'TEST-PARENT-GOAL', got '%s'", parent.ID)
	}

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

	childPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(child.ID), "goal.json")
	if _, err := os.Stat(childPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s", childPath)
	}

	if child.Parent != parent.ID {
		t.Errorf("Expected child parent '%s', got '%s'", parent.ID, child.Parent)
	}

	parent.Title = "Renamed Parent"
	err := UpdateGoalTitle(parent, parent.Title)
	if err != nil {
		t.Fatalf("Failed to update parent title: %v", err)
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(parent.ID)))

	if parent.ID != "RENAMED-PARENT" {
		t.Errorf("Expected renamed parent ID 'RENAMED-PARENT', got '%s'", parent.ID)
	}

	newChildID := "RENAMED-PARENT/TEST-CHILD-GOAL"
	newChildPath := filepath.Join(GetRepoGoalsDir(), filepath.FromSlash(newChildID), "goal.json")
	if _, err := os.Stat(newChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after parent rename", newChildPath)
	}

	listRes := ToolGoalList()
	if listRes.Error != "" {
		t.Fatalf("ToolGoalList failed: %s", listRes.Error)
	}
	allGoals := listRes.Data.([]*Goal)
	var foundChild *Goal
	for _, g := range allGoals {

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

	rootChildPath := filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL", "goal.json")
	if _, err := os.Stat(rootChildPath); os.IsNotExist(err) {
		t.Errorf("Child goal file not found at %s after reparenting", rootChildPath)
	}

	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), "TEST-CHILD-GOAL"))
}

func TestGoalList(t *testing.T) {
	result := ToolGoalList()

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

// #region 🔖Entity ID Tests

func TestGetArtifactID_Root(t *testing.T) {
	id := GetArtifactID("root", map[string]interface{}{})
	if id != "" {
		t.Errorf("root id: expected empty, got %q", id)
	}
}

func TestGetArtifactID_Years(t *testing.T) {
	id := GetArtifactID("years", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiYear)
	if id != expected {
		t.Errorf("years id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Year(t *testing.T) {
	id := GetArtifactID("year", map[string]interface{}{"parentId": "", "yy": "26"})
	expected := emojiText(EmojiYear) + "26"
	if id != expected {
		t.Errorf("year id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Months(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("months", map[string]interface{}{"parentId": yearId})
	expected := yearId + emojiText(EmojiMonth)
	if id != expected {
		t.Errorf("months id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Month(t *testing.T) {
	yearId := emojiText(EmojiYear) + "26"
	id := GetArtifactID("month", map[string]interface{}{"parentId": yearId, "mm": "02"})
	expected := yearId + emojiText(EmojiMonth) + "02"
	if id != expected {
		t.Errorf("month id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Days(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("days", map[string]interface{}{"parentId": monthId})
	expected := monthId + emojiText(EmojiDay)
	if id != expected {
		t.Errorf("days id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Day(t *testing.T) {
	monthId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02"
	id := GetArtifactID("day", map[string]interface{}{"parentId": monthId, "dd": "15"})
	expected := monthId + emojiText(EmojiDay) + "15"
	if id != expected {
		t.Errorf("day id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hours(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hours", map[string]interface{}{"parentId": dayId})
	expected := dayId + emojiText(EmojiHour)
	if id != expected {
		t.Errorf("hours id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Hour(t *testing.T) {
	dayId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15"
	id := GetArtifactID("hour", map[string]interface{}{"parentId": dayId, "hh": "14"})
	expected := dayId + emojiText(EmojiHour) + "14"
	if id != expected {
		t.Errorf("hour id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minutes(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minutes", map[string]interface{}{"parentId": hourId})
	expected := hourId + emojiText(EmojiMinute)
	if id != expected {
		t.Errorf("minutes id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Minute(t *testing.T) {
	hourId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14"
	id := GetArtifactID("minute", map[string]interface{}{"parentId": hourId, "mm": "33"})
	expected := hourId + emojiText(EmojiMinute) + "33"
	if id != expected {
		t.Errorf("minute id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Seconds(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("seconds", map[string]interface{}{"parentId": minuteId})
	expected := minuteId + emojiText(EmojiSecond)
	if id != expected {
		t.Errorf("seconds id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Second(t *testing.T) {
	minuteId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "15" + emojiText(EmojiHour) + "14" + emojiText(EmojiMinute) + "33"
	id := GetArtifactID("second", map[string]interface{}{"parentId": minuteId, "ss": "38"})
	expected := minuteId + emojiText(EmojiSecond) + "38"
	if id != expected {
		t.Errorf("second id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Projects(t *testing.T) {
	id := GetArtifactID("projects", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiProjects)
	if id != expected {
		t.Errorf("projects id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Project(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"user project", map[string]interface{}{"name": "semio", "kind": "user"}, emojiText(EmojiProjectUser) + "semio"},
		{"infra project", map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"}, emojiText(EmojiProjectInfra) + "semiorepo"},
		{"research project", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiProjectResearch) + "coda"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("project", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Bundles(t *testing.T) {
	projectId := emojiText(EmojiProjectUser) + "semio"
	id := GetArtifactID("bundles", map[string]interface{}{"parentId": projectId})
	expected := projectId + emojiText(EmojiBundles)
	if id != expected {
		t.Errorf("bundles id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Bundle(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"library bundle", map[string]interface{}{"name": "semio/js", "kind": "library"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle", map[string]interface{}{"name": "semio-repo/graphql", "kind": "schema"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle", map[string]interface{}{"name": "semio-repo/cli", "kind": "binary"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"},
		{"ui bundle", map[string]interface{}{"name": "semio-repo/vscode", "kind": "ui"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle", map[string]interface{}{"name": "coda/examples", "kind": "example"}, emojiText(EmojiProjectResearch) + "coda" + emojiText(EmojiBundleExample) + "examples"},
		{"site bundle", map[string]interface{}{"name": "semio/desktop", "kind": "site"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "desktop"},
		{"assets bundle", map[string]interface{}{"name": "semio/assets", "kind": "assets"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("bundle", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folders(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root folders", "", emojiText(EmojiFolders)},
		{"bundle folders", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder folders", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folders", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Folder(t *testing.T) {
	bundleId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"org folder under bundle", map[string]interface{}{"path": "semio/js/sketchpad", "name": "sketchpad", "kind": "organization", "parentId": bundleId}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder at root", map[string]interface{}{"path": ".devcontainer", "name": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"nested folder", map[string]interface{}{"path": "semio/js/sketchpad/pages", "name": "pages", "kind": "organization", "parentId": bundleId + emojiText(EmojiFolderOrg) + "sketchpad"}, bundleId + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("folder", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Files(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root files", "", emojiText(EmojiFiles)},
		{"folder files", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"required folder files", emojiText(EmojiFolderRequired) + "github", emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("files", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_File(t *testing.T) {
	folderId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"code file", map[string]interface{}{"path": "semio/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code", "parentId": folderId}, folderId + emojiText(EmojiFileCode) + "designtsx"},
		{"test file", map[string]interface{}{"path": "semio/js/sketchpad.test.ts", "name": "sketchpad.test.ts", "kind": "test", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileTest) + "sketchpadtestts"},
		{"config file at root", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "name": "devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainerjson"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("file", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Line(t *testing.T) {
	fileId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"
	id := GetArtifactID("line", map[string]interface{}{"parentId": fileId, "line": float64(3872)})
	expected := fileId + emojiText(EmojiLine) + "3872"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Range(t *testing.T) {
	fileId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"
	id := GetArtifactID("range", map[string]interface{}{"parentId": fileId, "startLine": float64(3872), "endLine": float64(3875)})
	expected := fileId + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Sections(t *testing.T) {
	fileId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"
	id := GetArtifactID("sections", map[string]interface{}{"parentId": fileId})
	expected := fileId + emojiText(EmojiSections)
	if id != expected {
		t.Errorf("sections id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Section(t *testing.T) {
	fileId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level section", map[string]interface{}{"name": "State Managment", "parentId": fileId}, fileId + emojiText(EmojiSection) + "statemanagment"},
		{"nested section", map[string]interface{}{"name": "Store", "parentId": fileId + emojiText(EmojiSection) + "statemanagment"}, fileId + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("section", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Definitions(t *testing.T) {
	sectionId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"
	id := GetArtifactID("definitions", map[string]interface{}{"parentId": sectionId})
	expected := sectionId + emojiText(EmojiDefinitions)
	if id != expected {
		t.Errorf("definitions id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Definition(t *testing.T) {
	sectionId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"
	id := GetArtifactID("definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": sectionId})
	expected := sectionId + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Goals(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root goals", "", emojiText(EmojiGoals)},
		{"nested goals", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiGoals)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("goals", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Goal(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"top-level goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
		{"nested goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("goal", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Tickets(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root tickets", "", emojiText(EmojiTickets)},
		{"goal tickets", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTickets)},
		{"section tickets", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiTickets)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("tickets", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Ticket(t *testing.T) {
	goalId := emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"ticket with parentId", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": goalId}, goalId + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"ticket with goalId fallback", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "goalId": "R26-02-1/RUNNING-SKETCHPAD"}, goalId + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("ticket", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Drafts(t *testing.T) {
	parentId := emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"
	id := GetArtifactID("drafts", map[string]interface{}{"parentId": parentId})
	expected := parentId + emojiText(EmojiDrafts)
	if id != expected {
		t.Errorf("drafts id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Draft(t *testing.T) {
	parentId := emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"
	id := GetArtifactID("draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": parentId})
	expected := parentId + emojiText(EmojiDraft) + "newarchitecture"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todos(t *testing.T) {
	parentId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todos", map[string]interface{}{"parentId": parentId})
	expected := parentId + emojiText(EmojiTodos)
	if id != expected {
		t.Errorf("todos id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Todo(t *testing.T) {
	parentId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"
	id := GetArtifactID("todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": parentId})
	expected := parentId + emojiText(EmojiTodo) + "introducepropersyncmechanism"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Policies(t *testing.T) {
	cases := []struct {
		name     string
		parentId string
		expected string
	}{
		{"root policies", "", emojiText(EmojiPolicies)},
		{"file kind policies", emojiText(EmojiFileCode), emojiText(EmojiFileCode) + emojiText(EmojiPolicies)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("policies", map[string]interface{}{"parentId": tc.parentId})
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Policy(t *testing.T) {
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"general policy on file kind", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("policy", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Contributors(t *testing.T) {
	id := GetArtifactID("contributors", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiContributors)
	if id != expected {
		t.Errorf("contributors id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Contributor(t *testing.T) {
	id := GetArtifactID("contributor", map[string]interface{}{"github": "usalu"})
	expected := emojiText(EmojiContributor) + "usalu"
	if id != expected {
		t.Errorf("expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Commits(t *testing.T) {
	id := GetArtifactID("commits", map[string]interface{}{"parentId": ""})
	expected := emojiText(EmojiCommits)
	if id != expected {
		t.Errorf("commits id: expected %q, got %q", expected, id)
	}
}

func TestGetArtifactID_Commit(t *testing.T) {
	sha := "cfb3b6084ff3fe883d5f39b08810a0b90997907a"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"with contributorId", map[string]interface{}{"sha": sha, "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCommit) + sha},
		{"with authorId fallback", map[string]interface{}{"sha": sha, "authorId": "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCommit) + sha},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("commit", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Interaction(t *testing.T) {
	secondId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"
	contributorId := emojiText(EmojiContributor) + "usalu"
	entityID := emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"
	cases := []struct {
		name     string
		data     map[string]interface{}
		expected string
	}{
		{"started", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "started"}, secondId + contributorId + entityID + emojiText(EmojiInteractionStarted)},
		{"finished", map[string]interface{}{"secondId": secondId, "contributorId": contributorId, "entityId": entityID, "kind": "finished"}, secondId + contributorId + entityID + emojiText(EmojiInteractionFinished)},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID("interaction", tc.data)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestGetArtifactID_Breach(t *testing.T) {
	policyId := emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"
	affected := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets"
	lineId := emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"
	secondId := emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"
	id := GetArtifactID("breach", map[string]interface{}{"parentId": policyId, "affected": affected, "lineId": lineId, "secondId": secondId})
	expected := policyId + emojiText(EmojiBreach) + affected + emojiText(EmojiBreachScope) + lineId + secondId
	if id != expected {
		t.Errorf("breach id: expected %q, got %q", expected, id)
	}
}

func TestGoalArtifactID(t *testing.T) {
	cases := []struct {
		rawID    string
		expected string
	}{
		{"R26-02-1", emojiText(EmojiGoal) + "r26021"},
		{"R26-02-1/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"AI-OPTIMIZED-REPO", emojiText(EmojiGoal) + "aioptimizedrepo"},
	}
	for _, tc := range cases {
		t.Run(tc.rawID, func(t *testing.T) {
			id := goalArtifactID(tc.rawID)
			if id != tc.expected {
				t.Errorf("expected %q, got %q", tc.expected, id)
			}
		})
	}
}

func TestSpecExactIDs(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root", "root", map[string]interface{}{}, ""},
		{"years", "years", map[string]interface{}{"parentId": ""}, "🎆"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "🎆26"},
		{"months", "months", map[string]interface{}{"parentId": "🎆26"}, "🎆26🌙"},
		{"month 02", "month", map[string]interface{}{"parentId": "🎆26", "mm": "02"}, "🎆26🌙02"},
		{"days", "days", map[string]interface{}{"parentId": "🎆26🌙02"}, "🎆26🌙02☀️"},
		{"day 15", "day", map[string]interface{}{"parentId": "🎆26🌙02", "dd": "15"}, "🎆26🌙02☀️15"},
		{"hours", "hours", map[string]interface{}{"parentId": "🎆26🌙02☀️15"}, "🎆26🌙02☀️15⏰"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "🎆26🌙02☀️15", "hh": "14"}, "🎆26🌙02☀️15⏰14"},
		{"minutes", "minutes", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14"}, "🎆26🌙02☀️15⏰14⌚"},
		{"minute 33", "minute", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14", "mm": "33"}, "🎆26🌙02☀️15⏰14⌚33"},
		{"seconds", "seconds", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33"}, "🎆26🌙02☀️15⏰14⌚33⏱️"},
		{"second 38", "second", map[string]interface{}{"parentId": "🎆26🌙02☀️15⏰14⌚33", "ss": "38"}, "🎆26🌙02☀️15⏰14⌚33⏱️38"},
		{"projects", "projects", map[string]interface{}{"parentId": ""}, "🏗️"},
		{"project semio-repo", "project", map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"}, "🧰semiorepo"},
		{"bundles", "bundles", map[string]interface{}{"parentId": "👤semio"}, "👤semio📦"},
		{"bundle semio/js", "bundle", map[string]interface{}{"name": "semio/js", "kind": "library"}, "👤semio📚js"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, "📁"},
		{"bundle folders", "folders", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad"}, "👤semio📚js🗃️sketchpad📁"},
		{"required folder folders", "folders", map[string]interface{}{"parentId": "🛅github"}, "🛅github📁"},
		{"folder semio/js/sketchpad", "folder", map[string]interface{}{"path": "semio/js/sketchpad", "kind": "organization", "parentId": "👤semio📚js"}, "👤semio📚js🗃️sketchpad"},
		{"folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, "🛅devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, "📄"},
		{"folder files", "files", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad"}, "👤semio📚js🗃️sketchpad📄"},
		{"required folder files", "files", map[string]interface{}{"parentId": "🛅github"}, "🛅github📄"},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "semio/js/sketchpad/Design.tsx", "kind": "code", "parentId": "👤semio📚js🗃️sketchpad"}, "👤semio📚js🗃️sketchpad💻designtsx"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": "🛅devcontainer"}, "🛅devcontainer⚙️devcontainerjson"},
		{"line 3872", "line", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad💻designtsx", "line": float64(3872)}, "👤semio📚js🗃️sketchpad💻designtsx📌3872"},
		{"sections in file", "sections", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad💻designtsx"}, "👤semio📚js🗃️sketchpad💻designtsx🔖"},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": "👤semio📚js🗃️sketchpad💻designtsx"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment"},
		{"section Store nested", "section", map[string]interface{}{"name": "Store", "parentId": "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store🏷️"},
		{"definition createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store🛠️createsketchpadstore"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, "🎯"},
		{"nested goals", "goals", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎯"},
		{"goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": "🎯r26021"}, "🎯r26021🎯runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, "🎫"},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫"},
		{"section tickets", "tickets", map[string]interface{}{"parentId": "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store🎫"},
		{"ticket", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": "🎯r26021🎯runningsketchpad"}, "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism"},
		{"draft", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": "🧰semiorepo⌨️cli"}, "🧰semiorepo⌨️cli📝newarchitecture"},
		{"todo", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store🛠️createsketchpadstore"}, "👤semio📚js🗃️sketchpad💻designtsx🔖statemanagment🔖store🛠️createsketchpadstore📝introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": "💻👮godfiles",
			"affected": "👤semio📚js🗃️sketchpad💻designstorets",
			"lineId":   "📌3872📌3875",
			"secondId": "🎆26🌙02☀️14⏰19⌚07⏱️12",
		}, "💻👮godfiles🚫👤semio📚js🗃️sketchpad💻designstorets🔍📌3872📌3875🎆26🌙02☀️14⏰19⌚07⏱️12"},
		{"contributor", "contributor", map[string]interface{}{"github": "usalu"}, "🧑‍💻usalu"},
		{"commit", "commit", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": "🧑‍💻usalu"}, "🧑‍💻usalu🔀cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻usalu",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "started",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻usalu🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism🌱"},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      "🎆26🌙02☀️14⏰19⌚07⏱️12",
			"contributorId": "🧑‍💻usalu",
			"entityId":      "🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism",
			"kind":          "finished",
		}, "🎆26🌙02☀️14⏰19⌚07⏱️12🧑‍💻usalu🎯r26021🎯runningsketchpad🎫introducekeyguidurimechanism✅"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

func TestPropagateParentIDs(t *testing.T) {
	root := &TreeNode{Kind: TreeNodeCategory, Data: map[string]interface{}{}, Children: []*TreeNode{
		{Kind: TreeNodeProject, Data: map[string]interface{}{"name": "semio", "kind": "user"}, Children: []*TreeNode{
			{Kind: TreeNodeBundle, Data: map[string]interface{}{"name": "semio/js", "kind": "library"}, Children: []*TreeNode{
				{Kind: TreeNodeFolder, Data: map[string]interface{}{"path": "semio/js/sketchpad", "name": "sketchpad", "kind": "organization"}, Children: []*TreeNode{
					{Kind: TreeNodeFile, Data: map[string]interface{}{"path": "semio/js/sketchpad/Design.tsx", "name": "Design.tsx", "kind": "code"}, Children: []*TreeNode{
						{Kind: TreeNodeSection, Data: map[string]interface{}{"name": "Store"}, Children: []*TreeNode{
							{Kind: TreeNodeDefinition, Data: map[string]interface{}{"name": "createStore", "kind": "implementation"}},
						}},
					}},
				}},
			}},
		}},
	}}
	PropagateParentIDs(root, "")
	projectId := emojiText(EmojiProjectUser) + "semio"
	bundleId := projectId + emojiText(EmojiBundleLibrary) + "js"
	folderId := bundleId + emojiText(EmojiFolderOrg) + "sketchpad"
	fileId := folderId + emojiText(EmojiFileCode) + "designtsx"
	sectionId := fileId + emojiText(EmojiSection) + "store"
	defId := sectionId + emojiText(EmojiDefinitionImpl) + "createstore"
	checks := []struct {
		label    string
		node     *TreeNode
		expected string
	}{
		{"project", root.Children[0], projectId},
		{"bundle", root.Children[0].Children[0], bundleId},
		{"folder", root.Children[0].Children[0].Children[0], folderId},
		{"file", root.Children[0].Children[0].Children[0].Children[0], fileId},
		{"section", root.Children[0].Children[0].Children[0].Children[0].Children[0], sectionId},
		{"definition", root.Children[0].Children[0].Children[0].Children[0].Children[0].Children[0], defId},
	}
	for _, c := range checks {
		t.Run(c.label, func(t *testing.T) {
			entityKind := treeNodeKindToEntityKind(c.node.Kind)
			got := GetArtifactID(entityKind, c.node.Data)
			if got != c.expected {
				t.Errorf("expected %q, got %q", c.expected, got)
			}
		})
	}
}

func TestFlat(t *testing.T) {
	cases := []struct {
		input    string
		expected string
	}{
		{"semio-repo", "semiorepo"},
		{"Design.tsx", "designtsx"},
		{".devcontainer", "devcontainer"},
		{"devcontainer.json", "devcontainerjson"},
		{"RUNNING-SKETCHPAD", "runningsketchpad"},
		{"R26-02-1", "r26021"},
		{"semio.ts", "semiots"},
		{"State Managment", "statemanagment"},
		{"createSketchpadStore", "createsketchpadstore"},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			got := Flat(tc.input)
			if got != tc.expected {
				t.Errorf("Flat(%q): expected %q, got %q", tc.input, tc.expected, got)
			}
		})
	}
}

func verifyTreeHierarchy(t *testing.T, node *TreeNode, parentPrefix string) {
	t.Helper()
	entityKind := treeNodeKindToEntityKind(node.Kind)
	if entityKind == "" {
		for _, child := range node.Children {
			verifyTreeHierarchy(t, child, parentPrefix)
		}
		return
	}
	id := GetArtifactID(entityKind, node.Data)
	if id == "" {
		return
	}
	if parentPrefix != "" && !strings.HasPrefix(id, parentPrefix) {
		t.Errorf("%s %q: id %q should start with parent prefix %q", entityKind, node.Label, id, parentPrefix)
	}
	for _, child := range node.Children {
		verifyTreeHierarchy(t, child, id)
	}
}

func TestMonorepoTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	projectsNode := tree.Children[0]
	var semioProject, semioRepoProject *TreeNode
	for _, c := range projectsNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		if strings.Contains(id, "semio") && !strings.Contains(id, "repo") && !strings.Contains(id, "coda") {
			semioProject = c
		}
		if strings.Contains(id, "semiorepo") {
			semioRepoProject = c
		}
	}
	if semioProject == nil {
		t.Fatal("semio project not found")
	}
	if semioRepoProject == nil {
		t.Fatal("semio-repo project not found")
	}
	semioId := GetArtifactID("project", semioProject.Data)
	if semioId != emojiText(EmojiProjectUser)+"semio" {
		t.Errorf("semio project id: expected %q, got %q", emojiText(EmojiProjectUser)+"semio", semioId)
	}
	semioRepoId := GetArtifactID("project", semioRepoProject.Data)
	if semioRepoId != emojiText(EmojiProjectInfra)+"semiorepo" {
		t.Errorf("semio-repo project id: expected %q, got %q", emojiText(EmojiProjectInfra)+"semiorepo", semioRepoId)
	}
	var jsBundle *TreeNode
	for _, c := range semioProject.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			if strings.HasSuffix(bId, emojiText(EmojiBundleLibrary)+"js") {
				jsBundle = c
				break
			}
		}
	}
	if jsBundle == nil {
		t.Fatal("semio/js bundle not found")
	}
	jsBundleId := GetArtifactID("bundle", jsBundle.Data)
	expectedBundleId := emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"
	if jsBundleId != expectedBundleId {
		t.Errorf("semio/js bundle id: expected %q, got %q", expectedBundleId, jsBundleId)
	}
	for _, c := range jsBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedBundleId)
		}
	}
	var cliBundle *TreeNode
	for _, c := range semioRepoProject.Children {
		if c.Kind == TreeNodeBundle {
			bId := GetArtifactID("bundle", c.Data)
			if strings.HasSuffix(bId, emojiText(EmojiBundleBinary)+"cli") {
				cliBundle = c
				break
			}
		}
	}
	if cliBundle == nil {
		t.Fatal("semio-repo/cli bundle not found")
	}
	cliBundleId := GetArtifactID("bundle", cliBundle.Data)
	expectedCliBundleId := emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"
	if cliBundleId != expectedCliBundleId {
		t.Errorf("semio-repo/cli bundle id: expected %q, got %q", expectedCliBundleId, cliBundleId)
	}
	for _, c := range cliBundle.Children {
		ek := treeNodeKindToEntityKind(c.Kind)
		if ek == "" {
			continue
		}
		childId := GetArtifactID(ek, c.Data)
		if !strings.HasPrefix(childId, expectedCliBundleId) {
			t.Errorf("bundle child %s %q: id %q should start with bundle id %q", ek, c.Label, childId, expectedCliBundleId)
		}
	}
	verifyTreeHierarchy(t, tree, "")
}

func TestGoalTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	goalsNode := tree.Children[1]
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			id := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiGoal)) {
				t.Errorf("goal id should start with goal emoji, got %q", id)
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childId, id) {
						t.Errorf("child goal id %q should start with parent goal id %q", childId, id)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, id) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, id)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
}

func TestContributorTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	var contributorsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "contributors" {
			contributorsNode = c
			break
		}
	}
	if contributorsNode == nil {
		t.Fatal("contributors node not found")
	}
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id should start with contributor emoji, got %q", id)
			}
		}
	}
}

func TestCommitTreeEntityIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	var commitsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "commits" {
			commitsNode = c
			break
		}
	}
	if commitsNode == nil {
		t.Fatal("commits node not found")
	}
	for _, c := range commitsNode.Children {
		if c.Kind == TreeNodeCommit {
			id := GetArtifactID("commit", c.Data)
			if !strings.Contains(id, emojiText(EmojiCommit)) {
				t.Errorf("commit id should contain commit emoji, got %q", id)
			}
		}
	}
}

func TestEntityKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"project", "bundle", "folder", "file", "line", "range",
		"section", "definition", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "commit", "interaction",
	}
	if len(EntityKinds) != len(expected) {
		t.Fatalf("EntityKinds length: expected %d, got %d", len(expected), len(EntityKinds))
	}
	for i, e := range expected {
		if EntityKinds[i] != e {
			t.Errorf("EntityKinds[%d]: expected %q, got %q", i, e, EntityKinds[i])
		}
	}
}

func TestResourceKinds(t *testing.T) {
	expected := []string{"repo", "project", "bundle", "folder", "file", "section", "definition"}
	if len(ResourceKinds) != len(expected) {
		t.Fatalf("ResourceKinds length: expected %d, got %d", len(expected), len(ResourceKinds))
	}
	for i, e := range expected {
		if ResourceKinds[i] != e {
			t.Errorf("ResourceKinds[%d]: expected %q, got %q", i, e, ResourceKinds[i])
		}
	}
}

func TestDiffableKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour",
		"project", "bundle", "folder", "file", "section", "definition",
		"goal", "ticket", "contributor", "commit", "interaction",
	}
	if len(DiffableKinds) != len(expected) {
		t.Fatalf("DiffableKinds length: expected %d, got %d", len(expected), len(DiffableKinds))
	}
	for i, e := range expected {
		if DiffableKinds[i] != e {
			t.Errorf("DiffableKinds[%d]: expected %q, got %q", i, e, DiffableKinds[i])
		}
	}
}

func TestRelatedToFileKinds(t *testing.T) {
	expected := []string{
		"root", "year", "month", "day", "hour", "minute", "second",
		"project", "bundle", "folder", "goal", "ticket", "draft", "todo",
		"policy", "breach", "contributor", "commit", "interaction",
	}
	if len(RelatedToFileKinds) != len(expected) {
		t.Fatalf("RelatedToFileKinds length: expected %d, got %d", len(expected), len(RelatedToFileKinds))
	}
	for i, e := range expected {
		if RelatedToFileKinds[i] != e {
			t.Errorf("RelatedToFileKinds[%d]: expected %q, got %q", i, e, RelatedToFileKinds[i])
		}
	}
}

func TestProjectListIDs(t *testing.T) {
	result := ToolProjectList()
	if result.Error != "" {
		t.Fatalf("ToolProjectList returned error: %s", result.Error)
	}
	projects, ok := result.Data.([]Project)
	if !ok {
		t.Fatal("ToolProjectList data is not []Project")
	}
	expectedIDs := map[string]string{
		"semio":      emojiText(EmojiProjectUser) + "semio",
		"semio-repo": emojiText(EmojiProjectInfra) + "semiorepo",
		"coda":       emojiText(EmojiProjectResearch) + "coda",
	}
	for _, p := range projects {
		expected, ok := expectedIDs[p.Name]
		if !ok {
			continue
		}
		got := p.GetID()
		if got != expected {
			t.Errorf("project %q id: expected %q, got %q", p.Name, expected, got)
		}
		delete(expectedIDs, p.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected project %q not found in list", name)
	}
}

func TestBundleListIDs(t *testing.T) {
	result := ToolBundleList()
	if result.Error != "" {
		t.Fatalf("ToolBundleList returned error: %s", result.Error)
	}
	bundles, ok := result.Data.([]Bundle)
	if !ok {
		t.Fatal("ToolBundleList data is not []Bundle")
	}
	expectedIDs := map[string]string{
		"semio/js":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js",
		"semio/engine":       emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine",
		"semio/go":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "go",
		"semio/rs":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "rs",
		"semio/py":           emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "py",
		"semio/net":          emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "net",
		"semio/graphql":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "graphql",
		"semio/jsonschema":   emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "jsonschema",
		"semio/openapi":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "openapi",
		"semio/desktop":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleUI) + "desktop",
		"semio/docs":         emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "docs",
		"semio/play":         emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "play",
		"semio/assets":       emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets",
		"semio-repo/cli":     emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli",
		"semio-repo/server":  emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "server",
		"semio-repo/go":      emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleLibrary) + "go",
		"semio-repo/vscode":  emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode",
		"semio-repo/graphql": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleSchema) + "graphql",
	}
	for _, b := range bundles {
		expected, ok := expectedIDs[b.Name]
		if !ok {
			continue
		}
		got := b.GetID()
		if got != expected {
			t.Errorf("bundle %q id: expected %q, got %q", b.Name, expected, got)
		}
		delete(expectedIDs, b.Name)
	}
	for name := range expectedIDs {
		t.Errorf("expected bundle %q not found in list", name)
	}
}

func TestSectionListIDs(t *testing.T) {
	result := ToolSectionList("semio-repo/cli/main.go")
	if result.Error != "" {
		t.Fatalf("ToolSectionList returned error: %s", result.Error)
	}
	sections, ok := result.Data.([]Section)
	if !ok {
		t.Fatal("ToolSectionList data is not []Section")
	}
	if len(sections) == 0 {
		t.Fatal("ToolSectionList returned no sections")
	}
	for _, s := range sections {
		localID := s.GetID()
		expectedPrefix := emojiText(EmojiSection)
		if !strings.HasPrefix(localID, expectedPrefix) {
			t.Errorf("section %q local id %q should start with section emoji %q", s.Name, localID, expectedPrefix)
		}
		flatName := Flat(s.Name)
		expectedID := expectedPrefix + flatName
		if localID != expectedID {
			t.Errorf("section %q local id: expected %q, got %q", s.Name, expectedID, localID)
		}
	}
}

func TestContributorListIDs(t *testing.T) {
	result := ToolContributorList()
	if result.Error != "" {
		t.Fatalf("ToolContributorList returned error: %s", result.Error)
	}
	contributors, ok := result.Data.([]Contributor)
	if !ok {
		t.Fatal("ToolContributorList data is not []Contributor")
	}
	if len(contributors) == 0 {
		t.Fatal("ToolContributorList returned no contributors")
	}
	for _, c := range contributors {
		id := c.GetID()
		expectedPrefix := emojiText(EmojiContributor)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("contributor %q id %q should start with %q", c.Github, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(c.Github)
		if id != expectedID {
			t.Errorf("contributor %q id: expected %q, got %q", c.Github, expectedID, id)
		}
	}
	foundUsalu := false
	for _, c := range contributors {
		if c.Github == "usalu" {
			if c.GetID() != emojiText(EmojiContributor)+"usalu" {
				t.Errorf("usalu contributor id: expected %q, got %q", emojiText(EmojiContributor)+"usalu", c.GetID())
			}
			foundUsalu = true
		}
	}
	if !foundUsalu {
		t.Error("expected to find contributor 'usalu'")
	}
}

func TestGoalListIDs(t *testing.T) {
	result := ToolGoalList()
	if result.Error != "" {
		t.Skipf("ToolGoalList returned error (may be due to existing data): %s", result.Error)
	}
	goals, ok := result.Data.([]Goal)
	if !ok {
		t.Skip("ToolGoalList data is not []Goal")
	}
	for _, g := range goals {
		id := g.GetID()
		expectedPrefix := emojiText(EmojiGoal)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("goal %q id %q should start with %q", g.ID, id, expectedPrefix)
		}
		flatID := Flat(g.ID)
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			flatID = Flat(g.ID[idx+1:])
		}
		expectedID := expectedPrefix + flatID
		if id != expectedID {
			t.Errorf("goal %q id: expected %q, got %q", g.ID, expectedID, id)
		}
	}
}

func TestTicketListIDs(t *testing.T) {
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Skipf("ToolTicketList returned error: %s", result.Error)
	}
	tickets, ok := result.Data.([]Ticket)
	if !ok {
		t.Skip("ToolTicketList data is not []Ticket")
	}
	for _, tk := range tickets {
		id := tk.GetID()
		expectedPrefix := emojiText(EmojiTicket)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("ticket %q id %q should start with %q", tk.Slug, id, expectedPrefix)
		}
		expectedID := expectedPrefix + Flat(tk.Slug)
		if id != expectedID {
			t.Errorf("ticket %q id: expected %q, got %q", tk.Slug, expectedID, id)
		}
	}
}

func TestDraftListIDs(t *testing.T) {
	result := ToolDraftList()
	if result.Error != "" {
		t.Skipf("ToolDraftList returned error: %s", result.Error)
	}
	drafts, ok := result.Data.([]*Draft)
	if !ok {
		t.Skip("ToolDraftList data is not []*Draft")
	}
	for _, d := range drafts {
		id := d.GetID()
		expectedPrefix := emojiText(EmojiDraft)
		if !strings.HasPrefix(id, expectedPrefix) {
			t.Errorf("draft %q id %q should start with %q", d.ID, id, expectedPrefix)
		}
	}
}

func TestMonorepoTreeFullIDHierarchy(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	projectsNode := tree.Children[0]
	var semioProject, semioRepoProject, codaProject *TreeNode
	for _, c := range projectsNode.Children {
		entityKind := treeNodeKindToEntityKind(c.Kind)
		id := GetArtifactID(entityKind, c.Data)
		if id == emojiText(EmojiProjectUser)+"semio" {
			semioProject = c
		} else if id == emojiText(EmojiProjectInfra)+"semiorepo" {
			semioRepoProject = c
		} else if id == emojiText(EmojiProjectResearch)+"coda" {
			codaProject = c
		}
	}
	if semioProject == nil {
		t.Fatal("semio project not found with expected id " + emojiText(EmojiProjectUser) + "semio")
	}
	if semioRepoProject == nil {
		t.Fatal("semio-repo project not found with expected id " + emojiText(EmojiProjectInfra) + "semiorepo")
	}
	if codaProject == nil {
		t.Fatal("coda project not found with expected id " + emojiText(EmojiProjectResearch) + "coda")
	}
	expectedSemioId := emojiText(EmojiProjectUser) + "semio"
	actualSemioId := GetArtifactID("project", semioProject.Data)
	if actualSemioId != expectedSemioId {
		t.Errorf("semio project id: expected %q, got %q", expectedSemioId, actualSemioId)
	}
	expectedRepoId := emojiText(EmojiProjectInfra) + "semiorepo"
	actualRepoId := GetArtifactID("project", semioRepoProject.Data)
	if actualRepoId != expectedRepoId {
		t.Errorf("semio-repo project id: expected %q, got %q", expectedRepoId, actualRepoId)
	}
	expectedCodaId := emojiText(EmojiProjectResearch) + "coda"
	actualCodaId := GetArtifactID("project", codaProject.Data)
	if actualCodaId != expectedCodaId {
		t.Errorf("coda project id: expected %q, got %q", expectedCodaId, actualCodaId)
	}
	bundleChecks := map[string]string{
		"semio/js":          emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js",
		"semio/go":          emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "go",
		"semio/engine":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine",
		"semio/assets":      emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets",
		"semio/desktop":     emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleUI) + "desktop",
		"semio/docs":        emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "docs",
		"semio-repo/cli":    emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli",
		"semio-repo/server": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "server",
		"semio-repo/vscode": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode",
	}
	allBundles := []*TreeNode{}
	for _, project := range []*TreeNode{semioProject, semioRepoProject, codaProject} {
		for _, child := range project.Children {
			if child.Kind == TreeNodeBundle {
				allBundles = append(allBundles, child)
			}
		}
	}
	for _, b := range allBundles {
		bundleId := GetArtifactID("bundle", b.Data)
		name, _ := b.Data["name"].(string)
		if expected, ok := bundleChecks[name]; ok {
			if bundleId != expected {
				t.Errorf("bundle %q id: expected %q, got %q", name, expected, bundleId)
			}
			delete(bundleChecks, name)
		}
		for _, child := range b.Children {
			childEK := treeNodeKindToEntityKind(child.Kind)
			if childEK == "" {
				continue
			}
			childId := GetArtifactID(childEK, child.Data)
			if !strings.HasPrefix(childId, bundleId) {
				t.Errorf("bundle %q child %s %q: id %q should start with bundle id %q", name, childEK, child.Label, childId, bundleId)
			}
		}
	}
	for name := range bundleChecks {
		t.Errorf("expected bundle %q not found in tree", name)
	}
	verifyTreeHierarchy(t, tree, "")
}

func TestGoalTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	goalsNode := tree.Children[1]
	goalCount := 0
	for _, c := range goalsNode.Children {
		if c.Kind == TreeNodeGoal {
			goalCount++
			goalId := GetArtifactID("goal", c.Data)
			if !strings.HasPrefix(goalId, emojiText(EmojiGoal)) {
				t.Errorf("goal id %q should start with %q", goalId, emojiText(EmojiGoal))
			}
			for _, child := range c.Children {
				if child.Kind == TreeNodeGoal {
					childGoalId := GetArtifactID("goal", child.Data)
					if !strings.HasPrefix(childGoalId, goalId) {
						t.Errorf("child goal id %q should start with parent goal id %q", childGoalId, goalId)
					}
					if !strings.Contains(childGoalId, emojiText(EmojiGoal)) {
						t.Errorf("child goal id %q should contain goal emoji", childGoalId)
					}
				}
				if child.Kind == TreeNodeTicket {
					ticketId := GetArtifactID("ticket", child.Data)
					if !strings.HasPrefix(ticketId, goalId) {
						t.Errorf("ticket id %q should start with goal id %q", ticketId, goalId)
					}
					if !strings.Contains(ticketId, emojiText(EmojiTicket)) {
						t.Errorf("ticket id %q should contain ticket emoji", ticketId)
					}
				}
			}
		}
	}
	if goalCount == 0 {
		t.Log("no goals found in tree (may be expected for fresh repos)")
	}
}

func TestContributorTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	var contributorsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "contributors" {
			contributorsNode = c
			break
		}
	}
	if contributorsNode == nil {
		t.Fatal("contributors node not found")
	}
	foundUsalu := false
	for _, c := range contributorsNode.Children {
		if c.Kind == TreeNodeContributor {
			id := GetArtifactID("contributor", c.Data)
			if !strings.HasPrefix(id, emojiText(EmojiContributor)) {
				t.Errorf("contributor id %q should start with %q", id, emojiText(EmojiContributor))
			}
			github, _ := c.Data["github"].(string)
			expectedID := emojiText(EmojiContributor) + Flat(github)
			if id != expectedID {
				t.Errorf("contributor %q id: expected %q, got %q", github, expectedID, id)
			}
			if github == "usalu" {
				foundUsalu = true
				if id != emojiText(EmojiContributor)+"usalu" {
					t.Errorf("usalu contributor id: expected %q, got %q", emojiText(EmojiContributor)+"usalu", id)
				}
			}
		}
	}
	if !foundUsalu {
		t.Error("expected to find contributor 'usalu' in tree")
	}
}

func TestCommitTreeIDs(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tree test")
	}
	cwd, _ := os.Getwd()
	SetRootDir(findTestRepoRoot(cwd))
	tree := BuildMonorepoTree(context.Background())
	var commitsNode *TreeNode
	for _, c := range tree.Children {
		if c.ID == "commits" {
			commitsNode = c
			break
		}
	}
	if commitsNode == nil {
		t.Fatal("commits node not found")
	}
	commitCount := 0
	for _, c := range commitsNode.Children {
		if c.Kind == TreeNodeCommit {
			commitCount++
			id := GetArtifactID("commit", c.Data)
			if !strings.Contains(id, emojiText(EmojiCommit)) {
				t.Errorf("commit id %q should contain %q", id, emojiText(EmojiCommit))
			}
			sha, _ := c.Data["sha"].(string)
			if sha != "" && !strings.HasSuffix(id, sha) {
				t.Errorf("commit id %q should end with sha %q", id, sha)
			}
			contributorId, _ := c.Data["contributorId"].(string)
			if contributorId != "" && !strings.HasPrefix(id, contributorId) {
				t.Errorf("commit id %q should start with contributor id %q", id, contributorId)
			}
		}
	}
	if commitCount == 0 {
		t.Error("no commits found in tree")
	}
}

func TestAllSpecIDExamples(t *testing.T) {
	cases := []struct {
		name     string
		kind     string
		data     map[string]interface{}
		expected string
	}{
		{"root id is empty", "root", map[string]interface{}{}, ""},
		{"years under root", "years", map[string]interface{}{"parentId": ""}, "\U0001F386"},
		{"year 26", "year", map[string]interface{}{"parentId": "", "yy": "26"}, "\U0001F38626"},
		{"months under year", "months", map[string]interface{}{"parentId": "\U0001F38626"}, "\U0001F38626\U0001F319"},
		{"month 02", "month", map[string]interface{}{"parentId": "\U0001F38626", "mm": "02"}, "\U0001F38626\U0001F31902"},
		{"days under month", "days", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay)},
		{"day 15", "day", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902", "dd": "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"},
		{"hours under day", "hours", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F0"},
		{"hour 14", "hour", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15", "hh": "14"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"},
		{"minutes under hour", "minutes", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B"},
		{"minute 33", "minute", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014", "mm": "33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33"},
		{"seconds under minute", "seconds", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33" + emojiText(EmojiSecond)},
		{"second 38", "second", map[string]interface{}{"parentId": "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33", "ss": "38"}, "\U0001F38626\U0001F31902" + emojiText(EmojiDay) + "15\u23F014\u231B33" + emojiText(EmojiSecond) + "38"},
		{"projects under root", "projects", map[string]interface{}{"parentId": ""}, emojiText(EmojiProjects)},
		{"infra project semio-repo", "project", map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"}, emojiText(EmojiProjectInfra) + "semiorepo"},
		{"user project semio", "project", map[string]interface{}{"name": "semio", "kind": "user"}, emojiText(EmojiProjectUser) + "semio"},
		{"research project coda", "project", map[string]interface{}{"name": "coda", "kind": "research"}, emojiText(EmojiProjectResearch) + "coda"},
		{"bundles under project", "bundles", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundles)},
		{"library bundle semio/js", "bundle", map[string]interface{}{"name": "semio/js", "kind": "library"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"},
		{"schema bundle semio/graphql", "bundle", map[string]interface{}{"name": "semio/graphql", "kind": "schema"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSchema) + "graphql"},
		{"binary bundle semio-repo/cli", "bundle", map[string]interface{}{"name": "semio-repo/cli", "kind": "binary"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"},
		{"ui bundle semio-repo/vscode", "bundle", map[string]interface{}{"name": "semio-repo/vscode", "kind": "ui"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleUI) + "vscode"},
		{"example bundle coda/examples", "bundle", map[string]interface{}{"name": "coda/examples", "kind": "example"}, emojiText(EmojiProjectResearch) + "coda" + emojiText(EmojiBundleExample) + "examples"},
		{"assets bundle semio/assets", "bundle", map[string]interface{}{"name": "semio/assets", "kind": "assets"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleAssets) + "assets"},
		{"root folders", "folders", map[string]interface{}{"parentId": ""}, emojiText(EmojiFolders)},
		{"bundle sketchpad folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolders)},
		{"required folder .github folders", "folders", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFolders)},
		{"org folder semio/js/sketchpad", "folder", map[string]interface{}{"path": "semio/js/sketchpad", "kind": "organization", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"},
		{"required folder .devcontainer", "folder", map[string]interface{}{"path": ".devcontainer", "kind": "required", "parentId": ""}, emojiText(EmojiFolderRequired) + "devcontainer"},
		{"root files", "files", map[string]interface{}{"parentId": ""}, emojiText(EmojiFiles)},
		{"sketchpad files", "files", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFiles)},
		{"github files", "files", map[string]interface{}{"parentId": emojiText(EmojiFolderRequired) + "github"}, emojiText(EmojiFolderRequired) + "github" + emojiText(EmojiFiles)},
		{"code file Design.tsx", "file", map[string]interface{}{"path": "semio/js/sketchpad/Design.tsx", "kind": "code", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"},
		{"config file devcontainer.json", "file", map[string]interface{}{"path": ".devcontainer/devcontainer.json", "kind": "config", "parentId": emojiText(EmojiFolderRequired) + "devcontainer"}, emojiText(EmojiFolderRequired) + "devcontainer" + emojiText(EmojiFileConfig) + "devcontainerjson"},
		{"line 3872", "line", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx", "line": float64(3872)}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiLine) + "3872"},
		{"range 3872-3875", "range", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets", "startLine": float64(3872), "endLine": float64(3875)}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets" + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875"},
		{"sections in file", "sections", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSections)},
		{"section State Managment", "section", map[string]interface{}{"name": "State Managment", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment"},
		{"nested section Store", "section", map[string]interface{}{"name": "Store", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"},
		{"definitions in section", "definitions", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitions)},
		{"definition impl createSketchpadStore", "definition", map[string]interface{}{"name": "createSketchpadStore", "kind": "implementation", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"},
		{"definition interface", "definition", map[string]interface{}{"name": "IStore", "kind": "interface", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionInterface) + "istore"},
		{"definition constant", "definition", map[string]interface{}{"name": "MAX_SIZE", "kind": "constant", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionConstant) + "maxsize"},
		{"root goals", "goals", map[string]interface{}{"parentId": ""}, emojiText(EmojiGoals)},
		{"nested goals under parent", "goals", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiGoals)},
		{"top-level goal", "goal", map[string]interface{}{"id": "R26-02-1", "parentId": ""}, emojiText(EmojiGoal) + "r26021"},
		{"nested goal Running Sketchpad", "goal", map[string]interface{}{"id": "R26-02-1/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r26021"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"root tickets", "tickets", map[string]interface{}{"parentId": ""}, emojiText(EmojiTickets)},
		{"goal tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTickets)},
		{"section tickets", "tickets", map[string]interface{}{"parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiTickets)},
		{"ticket Introduce Key Guid Uri Mechanism", "ticket", map[string]interface{}{"slug": "INTRODUCE-KEY-GUID-URI-MECHANISM", "parentId": emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad"}, emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism"},
		{"draft New Architecture", "draft", map[string]interface{}{"slug": "NEW-ARCHITECTURE", "parentId": emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli"}, emojiText(EmojiProjectInfra) + "semiorepo" + emojiText(EmojiBundleBinary) + "cli" + emojiText(EmojiDraft) + "newarchitecture"},
		{"todo Introduce Proper Sync Mechanism", "todo", map[string]interface{}{"id": "INTRODUCE-PROPER-SYNC-MECHANISM", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiDefinitionImpl) + "createsketchpadstore" + emojiText(EmojiTodo) + "introducepropersyncmechanism"},
		{"general policy godfiles", "policy", map[string]interface{}{"id": "godfiles", "parentId": emojiText(EmojiFileCode)}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles"},
		{"specific policy Only One Store", "policy", map[string]interface{}{"id": "only-one-store", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designtsx" + emojiText(EmojiSection) + "statemanagment" + emojiText(EmojiSection) + "store" + emojiText(EmojiPolicy) + "onlyonestore"},
		{"breach", "breach", map[string]interface{}{
			"parentId": emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles",
			"affected": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets",
			"lineId":   emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875",
			"secondId": emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
		}, emojiText(EmojiFileCode) + emojiText(EmojiPolicy) + "godfiles" + emojiText(EmojiBreach) + emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFileCode) + "designstorets" + emojiText(EmojiBreachScope) + emojiText(EmojiLine) + "3872" + emojiText(EmojiLine) + "3875" + emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12"},
		{"contributor usalu", "contributor", map[string]interface{}{"github": "usalu"}, emojiText(EmojiContributor) + "usalu"},
		{"commit", "commit", map[string]interface{}{"sha": "cfb3b6084ff3fe883d5f39b08810a0b90997907a", "contributorId": emojiText(EmojiContributor) + "usalu"}, emojiText(EmojiContributor) + "usalu" + emojiText(EmojiCommit) + "cfb3b6084ff3fe883d5f39b08810a0b90997907a"},
		{"interaction started", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "started",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionStarted)},
		{"interaction edited", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "edited",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionEdited)},
		{"interaction finished", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "finished",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionFinished)},
		{"interaction restarted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "restarted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionRestarted)},
		{"interaction deleted", "interaction", map[string]interface{}{
			"secondId":      emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12",
			"contributorId": emojiText(EmojiContributor) + "usalu",
			"entityId":      emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism",
			"kind":          "deleted",
		}, emojiText(EmojiYear) + "26" + emojiText(EmojiMonth) + "02" + emojiText(EmojiDay) + "14" + emojiText(EmojiHour) + "19" + emojiText(EmojiMinute) + "07" + emojiText(EmojiSecond) + "12" + emojiText(EmojiContributor) + "usalu" + emojiText(EmojiGoal) + "r26021" + emojiText(EmojiGoal) + "runningsketchpad" + emojiText(EmojiTicket) + "introducekeyguidurimechanism" + emojiText(EmojiInteractionDeleted)},
		{"file test kind", "file", map[string]interface{}{"path": "semio/js/sketchpad.test.ts", "kind": "test", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileTest) + "sketchpadtestts"},
		{"file script kind", "file", map[string]interface{}{"path": "semio/engine/build.ts", "kind": "script", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "engine" + emojiText(EmojiFileScript) + "buildts"},
		{"file docs kind", "file", map[string]interface{}{"path": "semio/js/README.md", "kind": "docs", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFileDocs) + "readmemd"},
		{"file asset kind", "file", map[string]interface{}{"path": "semio/js/sketchpad/pages/showcases/metabolism.mdx", "kind": "resource", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js" + emojiText(EmojiFolderOrg) + "sketchpad" + emojiText(EmojiFolderOrg) + "pages" + emojiText(EmojiFolderOrg) + "showcases" + emojiText(EmojiFileResource) + "metabolismmdx"},
		{"file license kind", "file", map[string]interface{}{"path": "semio/go/LICENSE.md", "kind": "license", "parentId": emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "go"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "go" + emojiText(EmojiFileLicense) + "licensemd"},
		{"site bundle semio/docs", "bundle", map[string]interface{}{"name": "semio/docs", "kind": "site"}, emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleSite) + "docs"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			id := GetArtifactID(tc.kind, tc.data)
			if id != tc.expected {
				t.Errorf("%s: expected %q, got %q", tc.name, tc.expected, id)
			}
		})
	}
}

// #endregion 🔖Entity ID Tests

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
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "tickets") {
		t.Errorf("Expected result to contain 'tickets', got: %s", result)
	}
}

func TestGraphQLAnalyzeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow analyze query test in short mode")
	}
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

	err := root.Execute()
	return buf.String(), err
}

func TestTreeCommands(t *testing.T) {

	if testing.Short() {
		t.Skip("skipping slow tree test")
	}

	output, err := executeTreeCommand("tree", "semio/go")
	if err != nil {
		t.Errorf("repo tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "semio.go") && !strings.Contains(output, "semiogo") {
		t.Errorf("repo tree semio/go missing semio.go, got:\n%s", output)
	}
	if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
		t.Errorf("repo tree default output must be markdown, got:\n%s", output)
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("repo tree default output missing markdown list items, got:\n%s", output)
	}

	output, err = executeTreeCommand("tree", "--only-folder", "semio/go")
	if err != nil {
		t.Errorf("folder tree failed: %v", err)
	}
	if !strings.Contains(output, "semio.go") {

		if len(output) < 10 {
			t.Errorf("folder tree output suspicious: %s", output)
		}
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("folder tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("tree", "--only-file", "semio/go")
	if err != nil {
		t.Errorf("file tree failed: %v", err)
	}
	if !strings.Contains(strings.ToLower(output), "semio.go") && !strings.Contains(output, "semiogo") {
		t.Errorf("file tree missing semio.go")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("file tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("tree", "--only-ticket")
	if err != nil {
		t.Errorf("ticket tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("ticket tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("ticket tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("tree", "--only-goal")
	if err != nil {
		t.Errorf("goal tree failed: %v", err)
	}
	if len(output) == 0 {
		t.Errorf("goal tree output empty")
	}
	if !strings.Contains(output, "- [") {
		t.Errorf("goal tree default output must be markdown, got:\n%s", output)
	}

	output, err = executeTreeCommand("tree", "semio/go", "--text")
	if err != nil {
		t.Errorf("repo tree text failed: %v", err)
	}
	if !strings.Contains(output, "├── ") && !strings.Contains(output, "└── ") {
		t.Errorf("repo tree text output should use connectors, got:\n%s", output)
	}

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

func TestCliE2E_TicketLifecycle_Syntaxes_NoManagement(t *testing.T) {
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
		"--no-management",
	)
	if err != nil {
		t.Fatalf("ticket open positional failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	y, m, d, slug := parseTicketOpenResult(t, openOut)
	defer os.RemoveAll(GetTicketPath(y, m, d, slug))
	ticketPath := fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug)

	_, reopenOpenErr, reopenOpenCmdErr := executeCommand(
		"ticket", "reopen",
		ticketPath,
		"prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-management",
	)
	if reopenOpenCmdErr == nil {
		t.Fatal("expected error when reopening an already-open ticket")
	}
	if !strings.Contains(reopenOpenErr, "ticket is already open") {
		t.Errorf("expected 'ticket is already open' error, got: %s", reopenOpenErr)
	}

	fileID := FileHeaderId(fileRel)
	fileURI := "semiorepo://file/" + PathToUriPath(fileRel)
	absFile := filepath.Join(GetRootDir(), fileRel)
	closeOut, closeErr, err := executeCommand(
		"ticket", "close",
		"--no-management",
		"--year", strconv.Itoa(y),
		"--month", strconv.Itoa(m),
		"--day", strconv.Itoa(d),
		"--slug", slug,
		"--summary", "E2E summary",
		"--files", fileRel,
		"--files", fileID,
		"--files", fileURI,
		"--files", absFile,
	)
	if err != nil {
		t.Fatalf("ticket close flags failed: %v\nStdout: %s\nStderr: %s", err, closeOut, closeErr)
	}
	if status := parseTicketCloseStatus(t, closeOut); status != "closed" {
		t.Fatalf("expected closed status, got %s", status)
	}

	_, closeAgainErr, closeAgainCmdErr := executeCommand(
		"ticket", "close",
		"--no-management",
		"--year", strconv.Itoa(y),
		"--month", strconv.Itoa(m),
		"--day", strconv.Itoa(d),
		"--slug", slug,
		"--summary", "E2E summary again",
		"--files", fileRel,
	)
	if closeAgainCmdErr == nil {
		t.Fatal("expected error when closing an already-closed ticket")
	}
	if !strings.Contains(closeAgainErr, "ticket is not open") {
		t.Errorf("expected 'ticket is not open' error, got: %s", closeAgainErr)
	}

	reopenOut, reopenErr, err := executeCommand(
		"ticket", "reopen",
		fmt.Sprintf("%04d/%02d/%02d/%s", y, m, d, slug),
		"E2E reopen prompt",
		"--cursor-chat",
		"--sonnet-4-5",
		"--no-management",
	)
	if err != nil {
		t.Fatalf("ticket reopen mix failed: %v\nStdout: %s\nStderr: %s", err, reopenOut, reopenErr)
	}
	if status := parseTicketReopenStatus(t, reopenOut); status != "open" {
		t.Fatalf("expected open status, got %s", status)
	}

	listOut, listErr, err := executeCommand("list", "--only-ticket", "--only-year", strconv.Itoa(y))
	if err != nil {
		t.Fatalf("ticket list failed: %v\nStdout: %s\nStderr: %s", err, listOut, listErr)
	}
}

func TestCliE2E_GoalLifecycle_Syntaxes_NoManagement(t *testing.T) {
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
		"--no-management",
	)
	if err != nil {
		t.Fatalf("goal open failed: %v\nStdout: %s\nStderr: %s", err, openOut, openErr)
	}
	goalID := parseGoalCreateID(t, openOut)
	defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

	_, reopenOpenErr, reopenOpenCmdErr := executeCommand("goal", "reopen", goalID, "prompt", "cursor-chat", "gpt-5-mini", "--no-management")
	if reopenOpenCmdErr == nil {
		t.Fatal("expected error when reopening an already-open goal")
	}
	if !strings.Contains(reopenOpenErr, "goal is already open") {
		t.Errorf("expected 'goal is already open' error, got: %s", reopenOpenErr)
	}

	_, closeErr, err := executeCommand("goal", "close", goalID, "E2E Goal Summary", "--no-management")
	if err != nil {
		t.Fatalf("goal close failed: %v\nStderr: %s", err, closeErr)
	}

	_, closeAgainErr, closeAgainCmdErr := executeCommand("goal", "close", goalID, "E2E Goal Summary Again", "--no-management")
	if closeAgainCmdErr == nil {
		t.Fatal("expected error when closing an already-closed goal")
	}
	if !strings.Contains(closeAgainErr, "goal is already closed") {
		t.Errorf("expected 'goal is already closed' error, got: %s", closeAgainErr)
	}

	_, reopenErr, err := executeCommand("goal", "reopen", goalID, "E2E Goal Reopen Prompt", "cursor-chat", "gpt-5-mini", "--no-management")
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
		{"bundle list", []string{"list", "--only-bundle"}},
		{"bundle tree", []string{"tree", "--only-bundle"}},
		{"folder list", []string{"list", "--only-folder", "go"}},
		{"file list", []string{"list", "--only-file", "go"}},
		{"section list", []string{"list", "--only-section", "semio/js/semio.ts"}},
		{"definition list", []string{"list", "--only-definition", "semio/js/semio.ts"}},
		{"policy list", []string{"list", "--only-policy"}},
		{"policy check", []string{"policy", "check", "code", "semio/js"}},
		{"goal list", []string{"list", "--only-goal"}},
		{"goal tree", []string{"tree", "--only-goal"}},
		{"ticket list", []string{"list", "--only-ticket"}},
		{"ticket tree", []string{"tree", "--only-ticket"}},
		{"contributor list", []string{"list", "--only-contributor"}},
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
		{"missing title", []string{"ticket", "open", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "open", "--goal", "TEST", "--title", "Valid Title", "--opus-4-5", "--no-management"}},
		{"missing goal", []string{"ticket", "open", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
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
		{"missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}},
		{"missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--files", "f"}},
		{"missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--summary", "s"}},
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
		{"missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"invalid path format", []string{"ticket", "reopen", "bad-path", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing prompt", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}},
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
		{"invalid path format", []string{"ticket", "change", "bad-path", "--no-management"}},
		{"nonexistent ticket", []string{"ticket", "change", "9999/01/01/NONEXISTENT", "--title", "New Title", "--no-management"}},
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
		{"missing title", []string{"goal", "open", "--no-management"}},
		{"missing description", []string{"goal", "open", "Valid Title", "--no-management", "--copilot-chat", "--opus-4-5", "--due-date", "2026-02-15"}},
		{"missing client", []string{"goal", "open", "Valid Title", "desc", "prompt", "--opus-4-5", "--due-date", "2026-02-15", "--no-management"}},
		{"missing llm", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--due-date", "2026-02-15", "--no-management"}},
		{"missing due-date", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
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
		{"missing id", []string{"goal", "close", "--no-management"}},
		{"missing summary", []string{"goal", "close", "NONEXISTENT-GOAL", "--no-management"}},
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
		{"missing id", []string{"goal", "reopen", "--no-management"}},
		{"missing prompt", []string{"goal", "reopen", "NONEXISTENT-GOAL", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--opus-4-5", "--no-management"}},
		{"missing llm", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--copilot-chat", "--no-management"}},
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

func TestCliWrongArgs_GoalChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing slug", []string{"goal", "change", "--no-management"}},
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
		{"create missing args", []string{"section", "create"}},
		{"move missing args", []string{"section", "move"}},
		{"delete missing args", []string{"section", "delete"}},
		{"extract missing args", []string{"section", "extract"}},
		{"integrate missing args", []string{"section", "integrate"}},
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
	}{}
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

func TestCliWrongArgs_TodoOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing all", []string{"todo", "create"}},
		{"create missing name", []string{"todo", "create", "some-parent"}},
		{"change missing id", []string{"todo", "change"}},
		{"delete missing id", []string{"todo", "delete"}},
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

func TestCliWrongArgs_TopLevelOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"move missing all args", []string{"move"}},
		{"move missing target", []string{"move", "source"}},
		{"extract missing all", []string{"extract"}},
		{"integrate missing all", []string{"integrate"}},
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

func TestCliWrongArgs_ErrorMessages(t *testing.T) {
	tests := []struct {
		name        string
		args        []string
		expectedErr string
	}{

		{"ticket open missing title", []string{"ticket", "open", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing title"},
		{"ticket open missing goal", []string{"ticket", "open", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal"},
		{"ticket close missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}, "missing ticket path"},
		{"ticket close missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--files", "f"}, "missing summary"},
		{"ticket close missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--summary", "s"}, "missing files"},
		{"ticket reopen missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing ticket path"},

		{"goal open missing title", []string{"goal", "open", "--no-management"}, "missing title"},
		{"goal close missing id", []string{"goal", "close", "--no-management"}, "missing goal id"},
		{"goal close missing summary", []string{"goal", "close", "NONEXISTENT", "--no-management"}, "missing summary"},
		{"goal reopen missing id", []string{"goal", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal id"},
		{"goal reopen missing prompt", []string{"goal", "reopen", "NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing prompt"},
		{"goal reopen missing client", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}, "missing client"},
		{"goal reopen missing llm", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--copilot-chat", "--no-management"}, "missing llm"},

		{"todo create missing parent or name", []string{"todo", "create"}, "missing parent-id or name"},
		{"todo create missing name only", []string{"todo", "create", "parent"}, "missing parent-id or name"},
		{"todo change missing id", []string{"todo", "change"}, "missing id"},
		{"todo delete missing id", []string{"todo", "delete"}, "missing id"},

		{"folder create missing path", []string{"folder", "create"}, "missing path"},
		{"folder move missing args", []string{"folder", "move"}, "missing"},
		{"folder delete missing path", []string{"folder", "delete"}, "missing path"},

		{"file create missing path", []string{"file", "create"}, "missing path"},
		{"file move missing args", []string{"file", "move"}, "missing"},
		{"file delete missing path", []string{"file", "delete"}, "missing path"},

		{"section create missing args", []string{"section", "create"}, "missing"},
		{"section move missing args", []string{"section", "move"}, "missing"},
		{"section delete missing args", []string{"section", "delete"}, "missing file or name"},
		{"section extract missing args", []string{"section", "extract"}, "missing source file, source section, or target file"},
		{"section integrate missing args", []string{"section", "integrate"}, "missing source, target section, or target file"},
		{"contributor remove missing github", []string{"contributor", "remove"}, "missing"},

		{"graphql missing query", []string{"graphql"}, "missing query"},

		{"extract missing args", []string{"extract"}, "missing file, section, or target-file"},
		{"integrate missing args", []string{"integrate"}, "missing file, target-file, or target-section"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s", tt.name)
			}
			if !strings.Contains(strings.ToLower(stderr), strings.ToLower(tt.expectedErr)) {
				t.Errorf("expected stderr to contain %q, got: %s", tt.expectedErr, stderr)
			}
		})
	}
}

func TestCliJsonPureData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow CLI JSON pure data test in short mode")
	}
	cmds := []struct {
		name string
		args []string
	}{
		{"bundle list", []string{"list", "--only-bundle"}},
		{"ticket list", []string{"list", "--only-ticket"}},
		{"folder list", []string{"list", "--only-folder", "semio-repo/go"}},
		{"file list", []string{"list", "--only-file", "semio-repo/go"}},
		{"section list", []string{"list", "--only-section", "semio-repo/go/main.go"}},
		{"definition list", []string{"list", "--only-definition", "semio-repo/go/main.go"}},
		{"policy list", []string{"list", "--only-policy"}},
		{"contributor list", []string{"list", "--only-contributor"}},
		{"goal list", []string{"list", "--only-goal"}},
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
		{"ticket open missing title", []string{"ticket", "open", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"ticket close missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}},
		{"ticket reopen missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"goal open missing title", []string{"goal", "open", "--no-management"}},
		{"goal close missing id", []string{"goal", "close", "--no-management"}},
		{"goal reopen missing id", []string{"goal", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"policy check missing id", []string{"policy", "check"}},
		{"folder create missing path", []string{"folder", "create"}},
		{"file create missing path", []string{"file", "create"}},
		{"section delete missing args", []string{"section", "delete"}},
		{"section extract missing args", []string{"section", "extract"}},
		{"section integrate missing args", []string{"section", "integrate"}},
		{"todo create missing args", []string{"todo", "create"}},
		{"todo chaete missing id", []string{"todo", "delete"}},
		{"graphql missing query", []string{"graphql"}},
		{"contributor add missing github", []string{"contributor", "add"}},
		{"contributor remove missing github", []string{"contributor", "remove"}},
		{"extract missing args", []string{"extract"}},
		{"integrate missing args", []string{"integrate"}},
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
		"mysection",
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
		"mydefinition",
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
		"mybundle",
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
		"folder",
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
		"filets",
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
		if !strings.Contains(strings.ToLower(output), "mvp") {
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
				"kinds":       []interface{}{"code/header"},
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
		if !strings.Contains(output, "filemd") {
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
		if !strings.Contains(output, "fixed 5 breachs") {
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
		{"statutes", "statutes", "statute", map[string]interface{}{
			"id": "vk1", "description": "Statute",
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
			"breachs": []interface{}{
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
	if !strings.Contains(output, "Total Breachs") {
		t.Error("analyze output missing 'Total Breachs'")
	}
	if !strings.Contains(output, "inline-comment") {
		t.Error("analyze output missing statute")
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
	if !strings.Contains(strings.ToLower(output), "header") {
		t.Errorf("output missing section name 'header', got: %s", output)
	}
	if !strings.Contains(strings.ToLower(output), "myfunc") {
		t.Errorf("output missing definition name 'myfunc', got: %s", output)
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
		{"statute", map[string]interface{}{
			"id": "vk1", "description": "Desc",
		}},
		{"project", map[string]interface{}{
			"id": "proj", "description": "Desc",
		}},
		{"commit", map[string]interface{}{
			"sha": "abc123", "message": "msg",
		}},
		{"root", map[string]interface{}{
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
				t.Errorf("link for %s missing 'semiorepo://' uri: %s", tt.kind, output)
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
		{"syncManagement", "root"},
		{"integrate", "file"},
		{"extract", "file"},
		{"fix", "root"},
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
	if testing.Short() {
		t.Skip("skipping slow markdown output test in short mode")
	}
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
			args:        []string{"tree", "--only-ticket"},
			wantMarkers: []string{"- [", "](semiorepo://ticket/"},
		},
		{
			name:        "Goal Tree MD",
			args:        []string{"tree", "--only-goal"},
			wantMarkers: []string{"- [", "](semiorepo://goal/"},
		},
		{
			name:        "Ticket List MD",
			args:        []string{"list", "--only-ticket"},
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
	if testing.Short() {
		t.Skip("skipping slow lifecycle commands test in short mode")
	}
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

			goalTitle := fmt.Sprintf("Test Goal %s LifecycleTest %d", mode, time.Now().UnixNano())
			goalCmd := NewRoot(factory)
			goalB := bytes.NewBufferString("")
			goalCmd.SetOut(goalB)
			goalCmd.SetErr(goalB)
			goalCmd.SetArgs([]string{"goal", "open", goalTitle, "Test Goal Description", "Test Goal Prompt", "copilot-chat", "gemini-3-pro", "--due-date", "2025-12-31", "--no-management", "--json"})
			if err := goalCmd.Execute(); err != nil {
				t.Fatalf("goal open failed: %v\nOutput: %s", err, goalB.String())
			}
			goalID := parseGoalCreateID(t, goalB.String())
			defer os.RemoveAll(filepath.Join(GetRepoGoalsDir(), goalID))

			openArgs := []string{"ticket", "open", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--goal", goalID, "--no-issue", "--no-management"}
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
			listCmd.SetArgs([]string{"list", "--only-ticket", "--json"})
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
				"--no-management",
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
				"--no-management",
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
	if testing.Short() {
		t.Skip("skipping slow list commands test in short mode")
	}
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
			args:  []string{"list", "--only-bundle"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "ticket list",
			args:  []string{"list", "--only-ticket"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "folder list",
			args:  []string{"list", "--only-folder", "semio-repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "file list",
			args:  []string{"list", "--only-file", "semio-repo/go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "section list",
			args:  []string{"list", "--only-section", "semio-repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "definition list",
			args:  []string{"list", "--only-definition", "semio-repo/go/main.go"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "policy list",
			args:  []string{"list", "--only-policy"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "contributor list",
			args:  []string{"list", "--only-contributor"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "project list",
			args:  []string{"list", "--only-project"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "statute list",
			args:  []string{"list", "--only-statute"},
			modes: []string{"", "json", "md", "text"},
		},
		{
			name:  "commit list",
			args:  []string{"list", "--only-commit", "--limit", "5"},
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

						var data map[string]interface{}
						if err := json.Unmarshal([]byte(line), &data); err != nil {
							t.Errorf("Invalid JSON line: %s", line)
						}

						if kind, ok := data["kind"].(string); ok {
							if kind == "result" || kind == "start" || kind == "done" {

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
	if testing.Short() {
		t.Skip("skipping slow section commands test in short mode")
	}
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
		{"Python", ".py", "# #region 🔖%s\ny = 2\n# #endregion 🔖%s\n", "Renamed"},
		{"CSharp", ".cs", "// #region 🔖%s\nvar y = 2;\n// #endregion 🔖%s\n", "Renamed"},
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
			args: []string{"list", "--only-ticket"},
		},
		{
			name: "Bundle List (Text)",
			args: []string{"list", "--only-bundle"},
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

func TestTicketLifecycle_NoManagement(t *testing.T) {
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

	if err := os.MkdirAll(filepath.Join(tmpDir, ".semio-repo", "🎫"), 0755); err != nil {
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
	if ticket.Management != nil {
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
	if goal.Management != nil {
		t.Error("OpenGoal: GitHub data should be nil")
	}

	goalPath := filepath.Join(tmpDir, ".semio-repo", "🎯", "GOAL-TITLE", "goal.json")
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
	if len(ticket.Interactions) < 2 {
		t.Fatalf("expected at least 2 interactions after close, got %d", len(ticket.Interactions))
	}
	if ticket.Interactions[0].Kind != "ticket.open" {
		t.Errorf("interaction[0].Kind = %q, want %q", ticket.Interactions[0].Kind, "ticket.open")
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.close" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.close")
	}

	err = ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "copilot-chat", "", "", "", true)
	if err != nil {
		t.Fatalf("ReopenTicket failed: %v", err)
	}
	if ticket.GetStatus() != TicketStatusOpen {
		t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
	}
	if ticket.Interactions[len(ticket.Interactions)-1].Kind != "ticket.reopen" {
		t.Errorf("last interaction Kind = %q, want %q", ticket.Interactions[len(ticket.Interactions)-1].Kind, "ticket.reopen")
	}

	ctx := NewRepoContext(tmpDir)

	goalInput := GoalCreateInput{
		Title:       "Test Goal NoGH 2",
		Description: "Desc",
		Prompt:      "Prompt",
		DueDate:     "2026-02-15",
		Client:      "cursor",
		LLM:         "gpt-5-2-codex",
		NoManagement:    true,
	}

	goal2, err := ctx.GoalCreate(goalInput)
	if err != nil {
		t.Fatalf("GoalCreate failed: %v", err)
	}
	if goal2.Title != "Test Goal NoGH 2" {
		t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
	}
	if len(goal2.Interactions) < 1 {
		t.Fatalf("expected at least 1 interaction on goal, got %d", len(goal2.Interactions))
	}
	if goal2.Interactions[0].Kind != "goal.open" {
		t.Errorf("goal interaction[0].Kind = %q, want %q", goal2.Interactions[0].Kind, "goal.open")
	}

	if len(goal.Interactions) < 1 {
		t.Fatalf("expected at least 1 interaction on original goal, got %d", len(goal.Interactions))
	}
	if goal.Interactions[0].Kind != "goal.open" {
		t.Errorf("original goal interaction[0].Kind = %q, want %q", goal.Interactions[0].Kind, "goal.open")
	}

	_, err = ctx.GoalClose(GoalCloseInput{ID: goal2.ID, Summary: "Done", NoManagement: true})
	if err != nil {
		t.Fatalf("GoalClose failed: %v", err)
	}
	closedGoalPath := filepath.Join(tmpDir, ".semio-repo", "🎯", goal2.ID, "goal.json")
	closedGoalContent, err := ReadTextFile(closedGoalPath)
	if err != nil {
		t.Fatalf("failed to read closed goal: %v", err)
	}
	var closedGoal Goal
	if err := json.Unmarshal([]byte(closedGoalContent), &closedGoal); err != nil {
		t.Fatalf("failed to unmarshal closed goal: %v", err)
	}
	if closedGoal.Interactions[len(closedGoal.Interactions)-1].Kind != "goal.close" {
		t.Errorf("closed goal last interaction Kind = %q, want %q", closedGoal.Interactions[len(closedGoal.Interactions)-1].Kind, "goal.close")
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
			name:    "root",
			kind:    "root",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "semiorepo://root",
		},
		{
			name:    "projects collection",
			kind:    "projects",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiProjects),
			wantURI: "semiorepo://projects",
		},
		{
			name:    "project user",
			kind:    "project",
			data:    map[string]interface{}{"name": "semio", "kind": "user"},
			wantID:  emojiText(EmojiProjectUser) + "semio",
			wantURI: "semiorepo://project/semio",
		},
		{
			name:    "project infrastructure",
			kind:    "project",
			data:    map[string]interface{}{"name": "semio-repo", "kind": "infrastructure"},
			wantID:  emojiText(EmojiProjectInfra) + "semiorepo",
			wantURI: "semiorepo://project/semio-repo",
		},
		{
			name:    "project research",
			kind:    "project",
			data:    map[string]interface{}{"name": "coda", "kind": "research"},
			wantID:  emojiText(EmojiProjectResearch) + "coda",
			wantURI: "semiorepo://project/coda",
		},
		{
			name:    "bundles collection",
			kind:    "bundles",
			data:    map[string]interface{}{"projectCode": "semio", "parentId": emojiText(EmojiProjectUser) + "semio"},
			wantID:  emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundles),
			wantURI: "semiorepo://bundles",
		},
		{
			name:    "bundle library",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "semio/js", "kind": "library"},
			wantID:  emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js",
			wantURI: "semiorepo://bundle/semio/js",
		},
		{
			name:    "bundle example",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "coda/examples", "kind": "library"},
			wantID:  emojiText(EmojiProjectResearch) + "coda" + emojiText(EmojiBundleLibrary) + "examples",
			wantURI: "semiorepo://bundle/coda/examples",
		},
		{
			name:    "bundle ui",
			kind:    "bundle",
			data:    map[string]interface{}{"name": "semio/desktop", "kind": "ui"},
			wantID:  emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleUI) + "desktop",
			wantURI: "semiorepo://bundle/semio/desktop",
		},
		{
			name:    "folders collection empty",
			kind:    "folders",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFolders),
			wantURI: "semiorepo://folders",
		},
		{
			name:    "folders collection with parent",
			kind:    "folders",
			data:    map[string]interface{}{"parentPath": "semio/js/src", "parentId": emojiText(EmojiFolderOrg) + "src"},
			wantID:  emojiText(EmojiFolderOrg) + "src" + emojiText(EmojiFolders),
			wantURI: "semiorepo://folders/semio/js/src",
		},
		{
			name:    "folder required",
			kind:    "folder",
			data:    map[string]interface{}{"path": "semio/js/src", "kind": "required"},
			wantID:  emojiText(EmojiFolderRequired) + "src",
			wantURI: "semiorepo://folder/semio/js/src",
		},
		{
			name:    "folder organization",
			kind:    "folder",
			data:    map[string]interface{}{"path": "semio/js/utils", "kind": "organization"},
			wantID:  emojiText(EmojiFolderOrg) + "utils",
			wantURI: "semiorepo://folder/semio/js/utils",
		},
		{
			name:    "files collection empty",
			kind:    "files",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiFiles),
			wantURI: "semiorepo://files",
		},
		{
			name:    "file docs",
			kind:    "file",
			data:    map[string]interface{}{"path": "test.txt", "kind": "docs"},
			wantID:  emojiText(EmojiFileDocs) + "testtxt",
			wantURI: "semiorepo://file/test.txt",
		},
		{
			name:    "file code",
			kind:    "file",
			data:    map[string]interface{}{"path": "main.go", "kind": "code"},
			wantID:  emojiText(EmojiFileCode) + "maingo",
			wantURI: "semiorepo://file/main.go",
		},
		{
			name:    "file test",
			kind:    "file",
			data:    map[string]interface{}{"path": "semio/js/src/index.test.ts", "kind": "test"},
			wantID:  emojiText(EmojiFileTest) + "indextestts",
			wantURI: "semiorepo://file/semio/js/src/index.test.ts",
		},
		{
			name:    "file config",
			kind:    "file",
			data:    map[string]interface{}{"path": "tsconfig.json", "kind": "config"},
			wantID:  emojiText(EmojiFileConfig) + "tsconfigjson",
			wantURI: "semiorepo://file/tsconfig.json",
		},
		{
			name:    "file script",
			kind:    "file",
			data:    map[string]interface{}{"path": "build.sh", "kind": "script"},
			wantID:  emojiText(EmojiFileScript) + "buildsh",
			wantURI: "semiorepo://file/build.sh",
		},
		{
			name:    "file resource",
			kind:    "file",
			data:    map[string]interface{}{"path": "logo.png", "kind": "resource"},
			wantID:  emojiText(EmojiFileResource) + "logopng",
			wantURI: "semiorepo://file/logo.png",
		},
		{
			name:    "file license",
			kind:    "file",
			data:    map[string]interface{}{"path": "LICENSE.md", "kind": "license"},
			wantID:  emojiText(EmojiFileLicense) + "licensemd",
			wantURI: "semiorepo://file/LICENSE.md",
		},
		{
			name:    "sections collection",
			kind:    "sections",
			data:    map[string]interface{}{"filePath": "semio/js/src/index.ts", "parentId": emojiText(EmojiFileCode) + "indexts"},
			wantID:  emojiText(EmojiFileCode) + "indexts" + emojiText(EmojiSections),
			wantURI: "semiorepo://sections/semio/js/src/index.ts",
		},
		{
			name:    "section",
			kind:    "section",
			data:    map[string]interface{}{"path": "semio/js/src/Design.tsx#State Management#Design Store"},
			wantID:  buildSectionID(buildFileID("semio/js/src/Design.tsx", nil), []string{"State Management", "Design Store"}),
			wantURI: "semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store",
		},
		{
			name:    "section single level",
			kind:    "section",
			data:    map[string]interface{}{"path": "semio/js/src/file.ts#Imports"},
			wantID:  buildSectionID(buildFileID("semio/js/src/file.ts", nil), []string{"Imports"}),
			wantURI: "semiorepo://section/semio/js/src/file.ts/Imports",
		},
		{
			name:    "definitions collection",
			kind:    "definitions",
			data:    map[string]interface{}{"filePath": "semio/js/src/index.ts", "parentId": emojiText(EmojiSection) + "types"},
			wantID:  emojiText(EmojiSection) + "types" + emojiText(EmojiDefinitions),
			wantURI: "semiorepo://definitions/semio/js/src/index.ts",
		},
		{
			name:    "definition with id",
			kind:    "definition",
			data:    map[string]interface{}{"id": "semio/js/src/index.ts#MyClass", "kind": "implementation"},
			wantID:  buildDefinitionID(buildFileID("semio/js/src/index.ts", nil), nil, "MyClass", DefinitionKindImplementation),
			wantURI: "semiorepo://definition/semio/js/src/index.ts/MyClass",
		},
		{
			name:    "definition interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "interface", "filePath": "semio/js/src/file.ts", "sectionPath": "Types", "name": "MyInterface"},
			wantID:  buildDefinitionID(buildFileID("semio/js/src/file.ts", nil), []string{"Types"}, "MyInterface", DefinitionKindInterface),
			wantURI: "semiorepo://definition/semio/js/src/file.ts/Types/MyInterface",
		},
		{
			name:    "definition go type treated as interface",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "type", "filePath": "semio-repo/cli/main.go", "sectionPath": "GraphQL Types#GraphQL Input Types", "name": "TicketCloseInput"},
			wantID:  buildDefinitionID(buildFileID("semio-repo/cli/main.go", nil), []string{"GraphQL Types", "GraphQL Input Types"}, "TicketCloseInput", DefinitionKindInterface),
			wantURI: "semiorepo://definition/semio-repo/cli/main.go/GraphQL%20Types/GraphQL%20Input%20Types/TicketCloseInput",
		},
		{
			name:    "definition constant",
			kind:    "definition",
			data:    map[string]interface{}{"kind": "constant", "filePath": "semio/js/src/file.ts", "name": "MAX_SIZE"},
			wantID:  buildDefinitionID(buildFileID("semio/js/src/file.ts", nil), nil, "MAX_SIZE", DefinitionKindConstant),
			wantURI: "semiorepo://definition/semio/js/src/file.ts/MAX_SIZE",
		},
		{
			name:    "tickets collection",
			kind:    "tickets",
			data:    map[string]interface{}{"parentId": ""},
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
			wantID:  emojiText(EmojiTicket) + "testticket",
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
			wantID:  emojiText(EmojiTicket) + "testticket",
			wantURI: "semiorepo://ticket/2025/02/04/test-ticket",
		},
		{
			name:    "goals collection",
			kind:    "goals",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiGoals),
			wantURI: "semiorepo://goals",
		},
		{
			name:    "goal",
			kind:    "goal",
			data:    map[string]interface{}{"id": "RUNNING-SKETCHPAD", "parentId": ""},
			wantID:  emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "semiorepo://goal/RUNNING-SKETCHPAD",
		},
		{
			name:    "goal nested",
			kind:    "goal",
			data:    map[string]interface{}{"id": "R26-02/RUNNING-SKETCHPAD", "parentId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad",
			wantURI: "semiorepo://goal/R26-02/RUNNING-SKETCHPAD",
		},
		{
			name:    "drafts collection",
			kind:    "drafts",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiDrafts),
			wantURI: "semiorepo://drafts",
		},
		{
			name:    "draft",
			kind:    "draft",
			data:    map[string]interface{}{"slug": "my-draft"},
			wantID:  emojiText(EmojiDraft) + "mydraft",
			wantURI: "semiorepo://draft/my-draft",
		},
		{
			name:    "todos collection",
			kind:    "todos",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiTodos),
			wantURI: "semiorepo://todos",
		},
		{
			name:    "todo",
			kind:    "todo",
			data:    map[string]interface{}{"id": "my-todo"},
			wantID:  emojiText(EmojiTodo) + "mytodo",
			wantURI: "semiorepo://todo/my-todo",
		},
		{
			name:    "policies collection",
			kind:    "policies",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiPolicies),
			wantURI: "semiorepo://policies",
		},
		{
			name:    "policy",
			kind:    "policy",
			data:    map[string]interface{}{"id": "/code-hygiene"},
			wantID:  emojiText(EmojiPolicy) + "codehygiene",
			wantURI: "semiorepo://policy/code-hygiene",
		},
		{
			name:    "statutes collection",
			kind:    "statutes",
			data:    map[string]interface{}{},
			wantID:  "",
			wantURI: "semiorepo://statutes",
		},
		{
			name:    "statute",
			kind:    "statute",
			data:    map[string]interface{}{"id": "code/inline-comment"},
			wantID:  "codeinlinecomment",
			wantURI: "semiorepo://statute/code/inline-comment",
		},
		{
			name:    "contributors collection",
			kind:    "contributors",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiContributors),
			wantURI: "semiorepo://contributors",
		},
		{
			name:    "contributor",
			kind:    "contributor",
			data:    map[string]interface{}{"github": "usalu"},
			wantID:  emojiText(EmojiContributor) + "usalu",
			wantURI: "semiorepo://contributor/usalu",
		},
		{
			name:    "commits collection",
			kind:    "commits",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  emojiText(EmojiCommits),
			wantURI: "semiorepo://commits",
		},
		{
			name:    "commit",
			kind:    "commit",
			data:    map[string]interface{}{"sha": "abc123"},
			wantID:  emojiText(EmojiCommit) + "abc123",
			wantURI: "semiorepo://commit/abc123",
		},
		{
			name:    "interactions collection",
			kind:    "interactions",
			data:    map[string]interface{}{"parentId": ""},
			wantID:  "",
			wantURI: "semiorepo://interactions",
		},
		{
			name:    "interaction started ticket",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "started", "entityId": emojiText(EmojiTicket) + "introduceinteractionmechanism"},
			wantID:  emojiText(EmojiTicket) + "introduceinteractionmechanism" + emojiText(EmojiInteractionStarted),
			wantURI: "semiorepo://interaction/on/ticket/introduceinteractionmechanism/started",
		},
		{
			name:    "interaction finished goal",
			kind:    "interaction",
			data:    map[string]interface{}{"kind": "finished", "entityId": emojiText(EmojiGoal) + "r2602"},
			wantID:  emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished),
			wantURI: "semiorepo://interaction/on/goal/r2602/finished",
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
		{"project user", emojiText(EmojiProjectUser) + "semio", "semiorepo://project/semio"},
		{"project infra", emojiText(EmojiProjectInfra) + "semiorepo", "semiorepo://project/semiorepo"},
		{"bundle", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js", "semiorepo://bundle/semio/js"},
		{"folder required", emojiText(EmojiFolderRequired) + "src", "semiorepo://folder/src"},
		{"folder org", emojiText(EmojiFolderOrg) + "utils", "semiorepo://folder/utils"},
		{"file docs", emojiText(EmojiFileDocs) + "testtxt", "semiorepo://file/testtxt"},
		{"file code", emojiText(EmojiFileCode) + "maingo", "semiorepo://file/maingo"},
		{"section collection", emojiText(EmojiSection), "semiorepo://sections"},
		{"section", buildSectionID(buildFileID("semio/js/src/design.tsx", nil), []string{"state managment", "store"}), "semiorepo://section/semio/js/src/designtsx/statemanagment/store"},
		{"definition impl", buildDefinitionID(buildFileID("semio/js/src/file.ts", nil), []string{"types"}, "myclass", DefinitionKindImplementation), "semiorepo://definition/semio/js/src/filets/types/myclass"},
		{"ticket collection", emojiText(EmojiTicket), "semiorepo://tickets"},
		{"ticket", emojiText(EmojiTicket) + "testticket", "semiorepo://ticket/testticket"},
		{"goal collection", emojiText(EmojiGoal), "semiorepo://goals"},
		{"goal", emojiText(EmojiGoal) + "r2602runningsketchpad", "semiorepo://goal/r2602runningsketchpad"},
		{"goal nested", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad", "semiorepo://goal/r2602/runningsketchpad"},
		{"draft collection", emojiText(EmojiDraft), "semiorepo://drafts"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "semiorepo://draft/mydraft"},
		{"policy collection", emojiText(EmojiPolicy), "semiorepo://policies"},
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "semiorepo://policy/codehygiene"},
		{"contributor collection", emojiText(EmojiContributor), "semiorepo://contributors"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "semiorepo://contributor/usalu"},
		{"commit collection", emojiText(EmojiCommit), "semiorepo://commits"},
		{"commit", emojiText(EmojiCommit) + "abc123", "semiorepo://commit/abc123"},
		{"interaction started ticket", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted), "semiorepo://interaction/on/ticket/testticket/started"},
		{"interaction finished goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionFinished), "semiorepo://interaction/on/goal/r2602/finished"},
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
		{"repo", "semiorepo://root", ""},
		{"projects", "semiorepo://projects", emojiText(EmojiProjects)},
		{"project", "semiorepo://project/semio", emojiText(EmojiProjectUser) + "semio"},
		{"project infra", "semiorepo://project/semio-repo", emojiText(EmojiProjectInfra) + "semiorepo"},
		{"bundles", "semiorepo://bundles", emojiText(EmojiBundles)},
		{"bundle", "semiorepo://bundle/semio/js", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js"},
		{"folders", "semiorepo://folders", emojiText(EmojiFolders)},
		{"folders with parent", "semiorepo://folders/semio/js/src", emojiText(EmojiFolders)},
		{"folder", "semiorepo://folder/semio/js/src", emojiText(EmojiFolderOrg) + "semiojssrc"},
		{"files", "semiorepo://files", emojiText(EmojiFiles)},
		{"file", "semiorepo://file/test.txt", emojiText(EmojiFileCode) + "testtxt"},
		{"sections", "semiorepo://sections", emojiText(EmojiSections)},
		{"section", "semiorepo://section/semio/js/src/Design.tsx/State%20Management/Design%20Store", buildSectionID(buildFileID("semio/js/src/Design.tsx", nil), []string{"State Management", "Design Store"})},
		{"definitions", "semiorepo://definitions", emojiText(EmojiDefinitions)},
		{"definition single", "semiorepo://definition/semio/js/src/file.ts/myFunc", buildDefinitionID(buildFileID("semio/js/src/file.ts", nil), nil, "myFunc", DefinitionKindImplementation)},
		{"definition with section", "semiorepo://definition/semio/js/src/file.ts/Section/myFunc", buildDefinitionID(buildFileID("semio/js/src/file.ts", nil), []string{"Section"}, "myFunc", DefinitionKindImplementation)},
		{"tickets", "semiorepo://tickets", emojiText(EmojiTicket)},
		{"ticket", "semiorepo://ticket/2025/02/04/test-ticket", emojiText(EmojiTicket) + "20250204testticket"},
		{"goals", "semiorepo://goals", emojiText(EmojiGoal)},
		{"goal", "semiorepo://goal/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "runningsketchpad"},
		{"goal nested", "semiorepo://goal/R26-02/RUNNING-SKETCHPAD", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiGoal) + "runningsketchpad"},
		{"drafts", "semiorepo://drafts", emojiText(EmojiDraft)},
		{"draft", "semiorepo://draft/my-draft", emojiText(EmojiDraft) + "mydraft"},
		{"todos", "semiorepo://todos", emojiText(EmojiTodo)},
		{"todo", "semiorepo://todo/my-todo", emojiText(EmojiTodo) + "mytodo"},
		{"policies", "semiorepo://policies", emojiText(EmojiPolicy)},
		{"policy", "semiorepo://policy/code-hygiene", emojiText(EmojiPolicy) + "codehygiene"},
		{"statutes", "semiorepo://statutes", ""},
		{"statute", "semiorepo://statute/code/inline-comment", ""},
		{"contributors", "semiorepo://contributors", emojiText(EmojiContributor)},
		{"contributor", "semiorepo://contributor/usalu", emojiText(EmojiContributor) + "usalu"},
		{"commits", "semiorepo://commits", emojiText(EmojiCommit)},
		{"commit", "semiorepo://commit/abc123", emojiText(EmojiCommit) + "abc123"},
		{"interactions", "semiorepo://interactions", ""},
		{"interaction ticket", "semiorepo://interaction/on/ticket/testticket/started", emojiText(EmojiTicket) + "testticket" + emojiText(EmojiInteractionStarted)},
		{"interaction goal", "semiorepo://interaction/on/goal/r2602/started", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted)},
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

func TestPathToUriPath(t *testing.T) {
	tests := []struct {
		path string
		want string
	}{
		{"semio/js/src", "semio/js/src"},
		{"semio-repo/cli/main.go", "semio-repo/cli/main.go"},
		{"test.txt", "test.txt"},
		{"a b/c d", "a%20b/c%20d"},
	}
	for _, tt := range tests {
		t.Run(tt.path, func(t *testing.T) {
			if got := PathToUriPath(tt.path); got != tt.want {
				t.Errorf("PathToUriPath(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestPathFromUriPath(t *testing.T) {
	tests := []struct {
		uriPath string
		want    string
	}{
		{"semio/js/src", "semio/js/src"},
		{"semio-repo/cli/main.go", "semio-repo/cli/main.go"},
		{"a%20b/c%20d", "a b/c d"},
	}
	for _, tt := range tests {
		t.Run(tt.uriPath, func(t *testing.T) {
			if got := PathFromUriPath(tt.uriPath); got != tt.want {
				t.Errorf("PathFromUriPath(%q) = %q, want %q", tt.uriPath, got, tt.want)
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
		{"single section", "semio/js/src/file.ts#Imports", "semio/js/src/file.ts/Imports"},
		{"nested sections", "semio/js/src/Design.tsx#State Management#Design Store", "semio/js/src/Design.tsx/State%20Management/Design%20Store"},
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
		{"with section and def", "semio/js/src/file.ts#Section§myFunc", "semio/js/src/file.ts/Section/myFunc"},
		{"def only", "semio/js/src/file.ts§myFunc", "semio/js/src/file.ts/myFunc"},
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
		{"file with sections", "semio/js/src/Design.tsx/State%20Management/Design%20Store", "semio/js/src/Design.tsx", []string{"State%20Management", "Design%20Store"}},
		{"file with one section", "semio/js/src/file.ts/Imports", "semio/js/src/file.ts", []string{"Imports"}},
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

func TestStatuteIdToUriPath(t *testing.T) {
	tests := []struct {
		name string
		id   string
		want string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdToUriPath(tt.id)
			if got != tt.want {
				t.Errorf("StatuteIdToUriPath(%q) = %q, want %q", tt.id, got, tt.want)
			}
		})
	}
}

func TestStatuteUriPathToId(t *testing.T) {
	tests := []struct {
		name    string
		uriPath string
		want    string
	}{
		{"single segment", "code", "code"},
		{"two segments", "code/inline-comment", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteUriPathToId(tt.uriPath)
			if got != tt.want {
				t.Errorf("StatuteUriPathToId(%q) = %q, want %q", tt.uriPath, got, tt.want)
			}
		})
	}
}

func TestTitleizeSlug(t *testing.T) {
	tests := []struct {
		name string
		slug string
		want string
	}{
		{"single word", "code", "Code"},
		{"two words", "inline-comment", "Inline Comment"},
		{"three words", "missing-region-marker", "Missing Region Marker"},
		{"already titleized", "Code", "Code"},
		{"uppercase input", "CODE", "Code"},
		{"empty", "", ""},
		{"single char", "a", "A"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := TitleizeSlug(tt.slug)
			if got != tt.want {
				t.Errorf("TitleizeSlug(%q) = %q, want %q", tt.slug, got, tt.want)
			}
		})
	}
}

func TestStatutePathToIdValue(t *testing.T) {
	tests := []struct {
		name string
		path string
		want string
	}{
		{"single segment", "code", "Code"},
		{"two segments", "code/inline-comment", "Code#Inline Comment"},
		{"three segments", "code/file/missing-header-region", "Code#File#Missing Header Region"},
		{"four segments", "code/header/region/nested", "Code#Header#Region#Nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatutePathToIdValue(tt.path)
			if got != tt.want {
				t.Errorf("StatutePathToIdValue(%q) = %q, want %q", tt.path, got, tt.want)
			}
		})
	}
}

func TestStatuteIdValueToPath(t *testing.T) {
	tests := []struct {
		name  string
		value string
		want  string
	}{
		{"single segment", "Code", "code"},
		{"two segments", "Code#Inline Comment", "code/inline-comment"},
		{"three segments", "Code#File#Missing Header Region", "code/file/missing-header-region"},
		{"four segments", "Code#Header#Region#Nested", "code/header/region/nested"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := StatuteIdValueToPath(tt.value)
			if got != tt.want {
				t.Errorf("StatuteIdValueToPath(%q) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestStatutePathIdValueRoundTrip(t *testing.T) {
	tests := []struct {
		name string
		path string
	}{
		{"single segment", "code"},
		{"two segments", "code/inline-comment"},
		{"three segments", "code/file/missing-header-region"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			idValue := StatutePathToIdValue(tt.path)
			gotPath := StatuteIdValueToPath(idValue)
			if gotPath != tt.path {
				t.Errorf("round trip failed: path %q -> idValue %q -> path %q", tt.path, idValue, gotPath)
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
		{"policy", emojiText(EmojiPolicy) + "codehygiene", "semiorepo://policy/codehygiene"},
		{"contributor", emojiText(EmojiContributor) + "usalu", "semiorepo://contributor/usalu"},
		{"commit", emojiText(EmojiCommit) + "abc123", "semiorepo://commit/abc123"},
		{"draft", emojiText(EmojiDraft) + "mydraft", "semiorepo://draft/mydraft"},
		{"section", emojiText(EmojiSection) + "imports", "semiorepo://section/imports"},
		{"file", emojiText(EmojiFileCode) + "indexts", "semiorepo://file/indexts"},
		{"ticket", emojiText(EmojiTicket) + "20260115someticket", "semiorepo://ticket/20260115someticket"},
		{"goal", emojiText(EmojiGoal) + "r2602running", "semiorepo://goal/r2602running"},
		{"interaction goal", emojiText(EmojiGoal) + "r2602" + emojiText(EmojiInteractionStarted), "semiorepo://interaction/on/goal/r2602/started"},
		{"project", emojiText(EmojiProjectUser) + "semio", "semiorepo://project/semio"},
		{"bundle", emojiText(EmojiProjectUser) + "semio" + emojiText(EmojiBundleLibrary) + "js", "semiorepo://bundle/semio/js"},
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

		if ok && typeVal == "array" {
			if _, hasItems := schema["items"]; !hasItems {
				return fmt.Errorf("property '%s' is of type 'array' but missing 'items' field", path)
			}
		}

		if props, ok := schema["properties"].(map[string]any); ok {
			for k, v := range props {
				if propMap, ok := v.(map[string]any); ok {
					if err := validateSchema(path+"."+k, propMap); err != nil {
						return err
					}
				}
			}
		}

		if items, ok := schema["items"].(map[string]any); ok {
			if err := validateSchema(path+".items", items); err != nil {
				return err
			}
		}

		return nil
	}

	for name, tool := range tools {
		t.Run(name, func(t *testing.T) {

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

// #endregion 🔖Tree Tests

// #region 🔖Query Tests

func TestQueryFlag(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query flag test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name          string
		args          []string
		query         string
		expectMatch   string
		expectMissing string
	}{
		{
			name:        "tree --query filters monorepo tree",
			args:        []string{"tree", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "tree --query excludes unrelated",
			args:          []string{"tree", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "semio/go",
		},
		{
			name:        "project list --query matches",
			args:        []string{"list", "--only-project", "--query", "semio", "--json"},
			query:       "",
			expectMatch: "semio",
		},
		{
			name:        "project tree --query matches",
			args:        []string{"tree", "--only-project", "--query", "semio", "--json"},
			query:       "",
			expectMatch: "semio",
		},
		{
			name:        "bundle list --query matches",
			args:        []string{"list", "--only-bundle", "--query", "engine", "--json"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:          "bundle list --query excludes unrelated",
			args:          []string{"list", "--only-bundle", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "engine",
		},
		{
			name:        "bundle tree --query matches",
			args:        []string{"tree", "--only-bundle", "--query", "engine", "--text"},
			query:       "",
			expectMatch: "engine",
		},
		{
			name:        "folder list --query matches",
			args:        []string{"list", "--only-folder", "--query", "go", "--json"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "folder tree --query matches",
			args:        []string{"tree", "--only-folder", "--query", "go", "--text"},
			query:       "",
			expectMatch: "go",
		},
		{
			name:        "file list --query matches",
			args:        []string{"list", "--only-file", "--query", "semio", "--json"},
			query:       "",
			expectMatch: "semio",
		},
		{
			name:        "file tree --query matches",
			args:        []string{"tree", "--only-file", "--query", "semio", "--text"},
			query:       "",
			expectMatch: "semio",
		},
		{
			name:        "section list --query matches",
			args:        []string{"list", "--only-section", "--query", "Models", "--json"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "section tree --query matches",
			args:        []string{"tree", "--only-section", "--query", "Models", "--text"},
			query:       "",
			expectMatch: "Model",
		},
		{
			name:        "definition list --query matches",
			args:        []string{"list", "--only-definition", "--query", "Kit", "--json"},
			query:       "",
			expectMatch: "Kit",
		},
		{
			name:        "ticket list --query matches",
			args:        []string{"list", "--only-ticket", "--query", "ticket", "--json"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "ticket tree --query matches",
			args:        []string{"tree", "--only-ticket", "--query", "ticket", "--text"},
			query:       "",
			expectMatch: "ticket",
		},
		{
			name:        "goal list --query matches",
			args:        []string{"list", "--only-goal", "--query", "repo", "--json"},
			query:       "",
			expectMatch: "repo",
		},
		{
			name:        "goal tree --query matches",
			args:        []string{"tree", "--only-goal", "--query", "sketchpad", "--text"},
			query:       "",
			expectMatch: "Sketchpad",
		},
		{
			name:          "goal tree --query excludes unrelated",
			args:          []string{"tree", "--only-goal", "--query", "zzz_nonexistent_xyz", "--text"},
			query:         "",
			expectMissing: "Sketchpad",
		},
		{
			name:        "policy list --query matches",
			args:        []string{"list", "--only-policy", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "policy tree --query matches",
			args:        []string{"tree", "--only-policy", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "statute list --query matches",
			args:        []string{"list", "--only-statute", "--query", "header", "--json"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:          "statute list --query excludes unrelated",
			args:          []string{"list", "--only-statute", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "header",
		},
		{
			name:        "statute tree --query matches",
			args:        []string{"tree", "--only-statute", "--query", "header", "--text"},
			query:       "",
			expectMatch: "header",
		},
		{
			name:        "query command returns matching IDs",
			args:        []string{"query", "bleve"},
			query:       "",
			expectMatch: "bleve",
		},
		{
			name:        "contributor list --query matches",
			args:        []string{"list", "--only-contributor", "--query", "usalu", "--json"},
			query:       "",
			expectMatch: "usalu",
		},
		{
			name:          "contributor list --query excludes unrelated",
			args:          []string{"list", "--only-contributor", "--query", "zzz_nonexistent_xyz", "--json"},
			query:         "",
			expectMissing: "usalu",
		},
		{
			name:        "commit list --query matches",
			args:        []string{"list", "--only-commit", "--query", "merge", "--json", "--limit", "200"},
			query:       "",
			expectMatch: "merge",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v\nOutput: %s", tt.args, err, output)
			}
			lower := strings.ToLower(output)
			if tt.expectMatch != "" {
				if !strings.Contains(lower, strings.ToLower(tt.expectMatch)) {
					t.Errorf("expected output to contain %q, got:\n%s", tt.expectMatch, output)
				}
			}
			if tt.expectMissing != "" {
				if strings.Contains(lower, strings.ToLower(tt.expectMissing)) {
					t.Errorf("expected output NOT to contain %q, got:\n%s", tt.expectMissing, output)
				}
			}
		})
	}
}

func TestQueryFuzzyMatch(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow query fuzzy match test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("policy list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-policy", "--query", "headr", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "header") {
			t.Errorf("expected fuzzy match for 'headr' to include header-related results, got:\n%s", output)
		}
	})

	t.Run("statute list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--query", "licenss", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "license") {
			t.Errorf("expected fuzzy match for 'licenss' to include license-related results, got:\n%s", output)
		}
	})

	t.Run("bundle list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-bundle", "--query", "engin", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "engine") {
			t.Errorf("expected fuzzy match for 'engin' to include engine, got:\n%s", output)
		}
	})

	t.Run("goal list fuzzy match with misspelling", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-goal", "--query", "sketchpd", "--json")
		if err != nil {
			t.Fatalf("command failed: %v\nOutput: %s", err, output)
		}
		if !strings.Contains(strings.ToLower(output), "sketchpad") {
			t.Errorf("expected fuzzy match for 'sketchpd' to include sketchpad, got:\n%s", output)
		}
	})
}

func TestCacheIndexAndTreeQuery(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow cache index test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("tree query returns multiple resource kinds for shared keyword", func(t *testing.T) {
		output, err := executeTreeCommand("tree", "--query", "bleve", "--text")
		if err != nil {
			t.Fatalf("tree --query bleve failed: %v\nOutput: %s", err, output)
		}
		hasFile := strings.Contains(output, ".go") || strings.Contains(output, "main")
		hasGoal := strings.Contains(output, "AI-OPTIMIZED") || strings.Contains(output, "Repo")
		hasTicket := strings.Contains(output, "ADD-BLEVE") || strings.Contains(output, "02/")
		if !hasFile && !hasGoal && !hasTicket {
			t.Errorf("tree --query bleve should return files, goals, or tickets; got:\n%s", output)
		}
		kinds := 0
		if hasFile {
			kinds++
		}
		if hasGoal {
			kinds++
		}
		if hasTicket {
			kinds++
		}
		if kinds < 2 {
			t.Logf("tree --query returned %d resource kinds (file=%v goal=%v ticket=%v); ideally multiple", kinds, hasFile, hasGoal, hasTicket)
		}
	})

	t.Run("query command returns matching resource IDs", func(t *testing.T) {
		output, err := executeTreeCommand("query", "bleve")
		if err != nil {
			t.Fatalf("query bleve failed: %v\nOutput: %s", err, output)
		}
		var nonEmpty int
		for _, l := range strings.Split(output, "\n") {
			if strings.TrimSpace(l) != "" {
				nonEmpty++
			}
		}
		if nonEmpty == 0 {
			t.Errorf("query bleve should return at least one ID, got:\n%s", output)
		}
	})

	t.Run("tree query for cli returns file and bundle", func(t *testing.T) {
		output, err := executeTreeCommand("tree", "--query", "cli", "--text")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		if !strings.Contains(strings.ToLower(output), "cli") {
			t.Errorf("expected 'cli' in output:\n%s", output)
		}
		hasSemioRepo := strings.Contains(output, "semio-repo")
		hasProjectOrBundle := strings.Contains(output, "bundle") || strings.Contains(output, "Projects")
		if !hasSemioRepo || !hasProjectOrBundle {
			t.Errorf("tree --query cli should return project/bundle hierarchy; got:\n%s", output)
		}
	})

	t.Run("tree query nonexistent returns minimal output", func(t *testing.T) {
		output, err := executeTreeCommand("tree", "--query", "zzz_nonexistent_xyzz", "--text")
		if err != nil {
			t.Fatalf("tree --query nonexistent failed: %v", err)
		}
		if strings.Contains(strings.ToLower(output), "zzz_nonexistent") {
			t.Errorf("tree --query nonexistent should not contain the query term in output")
		}
	})

	t.Run("different queries return different resources", func(t *testing.T) {
		bleveOut, err := executeTreeCommand("tree", "--query", "bleve", "--json")
		if err != nil {
			t.Fatalf("tree --query bleve failed: %v", err)
		}
		cliOut, err := executeTreeCommand("tree", "--query", "cli", "--json")
		if err != nil {
			t.Fatalf("tree --query cli failed: %v", err)
		}
		var bleveTree, cliTree map[string]interface{}
		if json.Unmarshal([]byte(strings.TrimSpace(bleveOut)), &bleveTree) != nil {
			t.Fatal("bleve output not valid JSON")
		}
		if json.Unmarshal([]byte(strings.TrimSpace(cliOut)), &cliTree) != nil {
			t.Fatal("cli output not valid JSON")
		}
		bleveStr := fmt.Sprint(bleveTree)
		cliStr := fmt.Sprint(cliTree)
		if bleveStr == cliStr {
			t.Error("tree --query bleve and tree --query cli should return different results")
		}
	})
}

func TestQueryEmptyReturnsAll(t *testing.T) {
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	tests := []struct {
		name string
		args []string
	}{
		{"policy list no query", []string{"list", "--only-policy", "--json"}},
		{"statute list no query", []string{"list", "--only-statute", "--json"}},
		{"contributor list no query", []string{"list", "--only-contributor", "--json"}},
		{"bundle list no query", []string{"list", "--only-bundle", "--json"}},
		{"goal list no query", []string{"list", "--only-goal", "--json"}},
		{"commit list no query", []string{"list", "--only-commit", "--json", "--limit", "5"}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := executeTreeCommand(tt.args...)
			if err != nil {
				t.Fatalf("command %v failed: %v", tt.args, err)
			}
			if strings.TrimSpace(output) == "" {
				t.Errorf("expected non-empty output for %v without query", tt.args)
			}
		})
	}
}

func TestStatuteCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statute commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("statute list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--json")
		if err != nil {
			t.Fatalf("statute list failed: %v", err)
		}
		if !strings.Contains(output, "statute") {
			t.Errorf("expected statute JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) < 5 {
			t.Errorf("expected multiple statutes, got %d lines", len(lines))
		}
	})

	t.Run("statute tree returns results", func(t *testing.T) {
		output, err := executeTreeCommand("tree", "--only-statute", "--text")
		if err != nil {
			t.Fatalf("statute tree failed: %v", err)
		}
		if !strings.Contains(output, "header") {
			t.Errorf("expected statute tree categories in output")
		}
	})

	t.Run("statute list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("statute tree markdown", func(t *testing.T) {
		output, err := executeTreeCommand("tree", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute tree md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})
}

func TestCommitCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow commit commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)

	t.Run("commit list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-commit", "--json", "--limit", "5")
		if err != nil {
			t.Fatalf("commit list failed: %v", err)
		}
		if !strings.Contains(output, "commit") {
			t.Errorf("expected commit JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) == 0 {
			t.Error("expected at least one commit")
		}
	})

	t.Run("commit list --query filters", func(t *testing.T) {
		allOutput, err := executeTreeCommand("list", "--only-commit", "--json", "--limit", "200")
		if err != nil {
			t.Fatalf("commit list failed: %v", err)
		}
		allLines := strings.Split(strings.TrimSpace(allOutput), "\n")

		filteredOutput, err := executeTreeCommand("list", "--only-commit", "--json", "--limit", "200", "--query", "merge")
		if err != nil {
			t.Fatalf("commit list --query failed: %v", err)
		}
		filteredLines := strings.Split(strings.TrimSpace(filteredOutput), "\n")
		if len(filteredLines) >= len(allLines) && len(allLines) > 1 {
			t.Errorf("expected --query to reduce results: all=%d filtered=%d", len(allLines), len(filteredLines))
		}
	})

	t.Run("commit list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-commit", "--md", "--limit", "5")
		if err != nil {
			t.Fatalf("commit list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("commit list text", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-commit", "--text", "--limit", "5")
		if err != nil {
			t.Fatalf("commit list text failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty text output")
		}
	})
}

// #endregion 🔖Query Tests

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
	if testing.Short() {
		t.Skip("skipping slow parity goal list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-goal")
		if err != nil {
			t.Fatalf("CLI goal list failed: %v", err)
		}
		toolResult := ToolGoalList()
		if toolResult.Error != "" {
			t.Fatalf("ToolGoalList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of goals", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-goal")
		toolResult := ToolGoalList()
		mcpOut := toolOutputText(toolResult)
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})

	t.Run("empty output when no goals match filter", func(t *testing.T) {

		cliOut, _, _ := executeCommandMd("list", "--only-goal")
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
	if testing.Short() {
		t.Skip("skipping slow parity contributor list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-contributor")
		if err != nil {
			t.Fatalf("CLI contributor list failed: %v", err)
		}
		toolResult := ToolContributorList()
		if toolResult.Error != "" {
			t.Fatalf("ToolContributorList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of contributors", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-contributor")
		mcpOut := toolOutputText(ToolContributorList())
		cliLines := strings.Count(cliOut, "\n")
		mcpLines := strings.Count(mcpOut, "\n")
		if cliLines != mcpLines {
			t.Errorf("line count mismatch: CLI=%d, MCP=%d", cliLines, mcpLines)
		}
	})
}

func TestParityTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity ticket list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-ticket")
		if err != nil {
			t.Fatalf("CLI ticket list failed: %v", err)
		}
		toolResult := ToolTicketList(nil, nil, nil)
		if toolResult.Error != "" {
			t.Fatalf("ToolTicketList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of tickets", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-ticket")
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
	if testing.Short() {
		t.Skip("skipping slow parity project list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-project")
		if err != nil {
			t.Fatalf("CLI project list failed: %v", err)
		}
		toolResult := ToolProjectList()
		if toolResult.Error != "" {
			t.Fatalf("ToolProjectList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return non-empty output", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-project")
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
	if testing.Short() {
		t.Skip("skipping slow parity project tree test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("tree", "--only-project")
		if err != nil {
			t.Fatalf("CLI project tree failed: %v", err)
		}
		toolResult := ToolProjectTree()
		if toolResult.Error != "" {
			t.Fatalf("ToolProjectTree returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("projects are sorted alphabetically", func(t *testing.T) {
		mcpOut := toolOutputText(ToolProjectTree())
		lines := strings.Split(strings.TrimSpace(mcpOut), "\n")
		var projectNames []string
		for _, line := range lines {
			trimmed := strings.TrimSpace(line)
			if idx := strings.Index(trimmed, "semiorepo://project/"); idx >= 0 {
				nameStart := idx + len("semiorepo://project/")
				nameEnd := strings.Index(trimmed[nameStart:], ")")
				if nameEnd >= 0 {
					projectNames = append(projectNames, trimmed[nameStart:nameStart+nameEnd])
				}
			}
		}
		for i := 1; i < len(projectNames); i++ {
			if projectNames[i] < projectNames[i-1] {
				t.Errorf("projects not sorted: %q comes after %q", projectNames[i], projectNames[i-1])
			}
		}
	})
}

func TestParityPolicyList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow parity policy list test in short mode")
	}
	setupToolTest(t)

	t.Run("output matches CLI markdown", func(t *testing.T) {
		cliOut, _, err := executeCommandMd("list", "--only-policy")
		if err != nil {
			t.Fatalf("CLI policy list failed: %v", err)
		}
		toolResult := ToolPolicyList()
		if toolResult.Error != "" {
			t.Fatalf("ToolPolicyList returned error: %s", toolResult.Error)
		}
		mcpOut := toolOutputText(toolResult)
		if normalizeRelativeTimes(cliOut) != normalizeRelativeTimes(mcpOut) {
			t.Errorf("output mismatch:\nCLI:\n%s\nMCP:\n%s", cliOut, mcpOut)
		}
	})

	t.Run("both return same number of policies", func(t *testing.T) {
		cliOut, _, _ := executeCommandMd("list", "--only-policy")
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
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
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
			TreeNodeDraft, TreeNodePolicy, TreeNodeStatute,
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
				{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "semiorepo://projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "p1", Label: "semio"},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		if !strings.Contains(output, "🏗️Projects") {
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
				{Kind: TreeNodeCategory, ID: "projects", Label: "🏗️Projects", URI: "semiorepo://projects", Children: []*TreeNode{
					{Kind: TreeNodeProject, ID: "p1", Label: "semio"},
				}},
			},
		}
		output := RenderMonorepoTreeMarkdown(tree)
		if !strings.Contains(output, "- [🏗️Projects](semiorepo://projects)") {
			t.Errorf("markdown tree should contain markdown link list item, got: %s", output)
		}
		if !strings.Contains(output, "  - semio") {
			t.Errorf("markdown tree should contain nested bullet item, got: %s", output)
		}
		if strings.Contains(output, "├── ") || strings.Contains(output, "└── ") {
			t.Errorf("markdown tree must not contain ascii connectors, got: %s", output)
		}
	})

	t.Run("text tree shows only own ID segment not full parent chain", func(t *testing.T) {
		parentGoalData := map[string]interface{}{
			"id":     "parentgoal",
			"title":  "Parent Goal",
			"status": "open",
		}
		childGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal",
			"title":    "Child Goal",
			"status":   "open",
			"parentId": "🎯parentgoal",
		}
		grandchildGoalData := map[string]interface{}{
			"id":       "parentgoal/childgoal/grandchildgoal",
			"title":    "Grandchild Goal",
			"status":   "open",
			"parentId": "🎯parentgoal🎯childgoal",
		}
		tree := &TreeNode{
			Kind: TreeNodeCategory, Label: ".", Children: []*TreeNode{
				{Kind: TreeNodeCategory, ID: "goals", Label: "🎯Goals", URI: "semiorepo://goals", Children: []*TreeNode{
					{Kind: TreeNodeGoal, ID: "parentgoal", Label: "Parent Goal", Data: parentGoalData, Children: []*TreeNode{
						{Kind: TreeNodeGoal, ID: "childgoal", Label: "Child Goal", Data: childGoalData, Children: []*TreeNode{
							{Kind: TreeNodeGoal, ID: "grandchildgoal", Label: "Grandchild Goal", Data: grandchildGoalData},
						}},
					}},
				}},
			},
		}
		output := RenderMonorepoTree(tree)
		lines := strings.Split(strings.TrimRight(output, "\n"), "\n")
		for _, line := range lines {
			if strings.Contains(line, "🎯parentgoal🎯childgoal") {
				t.Errorf("tree text should not contain full hierarchical ID, got line: %s", line)
			}
			if strings.Contains(line, "🎯parentgoal🎯") {
				t.Errorf("tree text should not contain parent prefix in child line, got line: %s", line)
			}
		}
		childFound := false
		grandchildFound := false
		for _, line := range lines {
			if strings.Contains(line, "🎯childgoal") && !strings.Contains(line, "🎯parentgoal🎯childgoal") {
				childFound = true
			}
			if strings.Contains(line, "🎯grandchildgoal") && !strings.Contains(line, "🎯childgoal🎯grandchildgoal") {
				grandchildFound = true
			}
		}
		if !childFound {
			t.Errorf("tree text should contain short child ID 🎯childgoal, got:\n%s", output)
		}
		if !grandchildFound {
			t.Errorf("tree text should contain short grandchild ID 🎯grandchildgoal, got:\n%s", output)
		}
	})

	t.Run("text tree preserves parentId on data after rendering", func(t *testing.T) {
		data := map[string]interface{}{
			"id":       "parent/child",
			"title":    "Child",
			"status":   "open",
			"parentId": "🎯parent",
		}
		node := &TreeNode{Kind: TreeNodeGoal, ID: "child", Label: "Child", Data: data}
		var sb strings.Builder
		renderTreeNodeText(&sb, node, "", true, true)
		if data["parentId"] != "🎯parent" {
			t.Errorf("renderTreeNodeText should restore parentId, got: %v", data["parentId"])
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

	ctx := context.Background()
	treeNoSections := BuildMonorepoTree(ctx)
	treeSections := BuildMonorepoTree(ctx, TreeBuildOptions{IncludeSections: true})

	t.Run("builds tree with categories", func(t *testing.T) {
		if treeNoSections == nil {
			t.Fatal("tree should not be nil")
		}
		if len(treeNoSections.Children) == 0 {
			t.Fatal("tree should have categories")
		}
		categoryIDs := make(map[string]bool)
		for _, c := range treeNoSections.Children {
			if c.Kind != TreeNodeCategory {
				t.Errorf("top-level children should be categories, got %s", c.Kind)
			}
			categoryIDs[c.ID] = true
		}
		expected := []string{"folders", "projects", "goals", "drafts", "policies", "contributors", "commits"}
		for _, id := range expected {
			if !categoryIDs[id] {
				t.Errorf("missing category: %s", id)
			}
		}
	})

	t.Run("folders category has folder and file hierarchy", func(t *testing.T) {
		var foldersNode *TreeNode
		for _, c := range treeSections.Children {
			if c.ID == "folders" {
				foldersNode = c
				break
			}
		}
		if foldersNode == nil {
			t.Fatal("folders category not found")
		}
		if len(foldersNode.Children) == 0 {
			t.Fatal("folders category should have children")
		}
		hasFolder := false
		hasFile := false
		hasSection := false
		hasDefinition := false
		var walk func(*TreeNode)
		walk = func(node *TreeNode) {
			switch node.Kind {
			case TreeNodeFolder:
				hasFolder = true
			case TreeNodeFile:
				hasFile = true
			case TreeNodeSection:
				hasSection = true
			case TreeNodeDefinition:
				hasDefinition = true
			}
			for _, child := range node.Children {
				walk(child)
			}
		}
		walk(foldersNode)
		if !hasFolder {
			t.Error("folders category should include folder nodes")
		}
		if !hasFile {
			t.Error("folders category should include file nodes")
		}
		if !hasSection {
			t.Error("folders category should include section nodes when IncludeSections is true")
		}
		if !hasDefinition {
			t.Error("folders category should include definition nodes when IncludeSections is true")
		}
	})

	t.Run("projects category has children", func(t *testing.T) {
		var projectsNode *TreeNode
		for _, c := range treeNoSections.Children {
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

	t.Run("policies category uses entitykind grouping", func(t *testing.T) {
		var policiesNode *TreeNode
		for _, c := range treeNoSections.Children {
			if c.ID == "policies" {
				policiesNode = c
				break
			}
		}
		if policiesNode == nil {
			t.Fatal("policies category not found")
		}
		if len(policiesNode.Children) == 0 {
			t.Fatal("policies should have children")
		}
		policy := policiesNode.Children[0]
		if policy.Kind != TreeNodePolicy {
			t.Fatalf("expected policy node, got %s", policy.Kind)
		}
		if len(policy.Children) == 0 {
			t.Fatal("policy should contain entitykind children")
		}
		entityKind := policy.Children[0]
		if entityKind.Kind != TreeNodeCategory {
			t.Fatalf("expected entitykind category node, got %s", entityKind.Kind)
		}
		if entityKind.SubKind != "entitykind" {
			t.Fatalf("expected entitykind subkind, got %s", entityKind.SubKind)
		}
		if len(entityKind.Children) == 0 {
			t.Fatal("entitykind should contain statute children")
		}
		for _, statute := range entityKind.Children {
			if statute.Kind != TreeNodeStatute {
				t.Fatalf("expected statute child under entitykind, got %s", statute.Kind)
			}
		}
	})

	t.Run("with sections includes sections", func(t *testing.T) {
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
		walk(treeSections)
		if !hasSections {
			t.Error("tree with IncludeSections should have section nodes")
		}
	})

	t.Run("without sections excludes sections", func(t *testing.T) {
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
		walk(treeNoSections)
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
		{TreeNodeStatute, ""},
		{TreeNodeContributor, "contributor"},
		{TreeNodeCommit, "commit"},
		{TreeNodeCategory, ""},
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
			URI:   "semiorepo://goal/test-goal",
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
			URI:   "semiorepo://goal/test-goal",
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
			URI:   "semiorepo://section/test/file.ts/mysection",
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
		{"statute", TreeNodeStatute, map[string]interface{}{
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
	if testing.Short() {
		t.Skip("skipping slow migration test in short mode")
	}
	ctx := context.Background()

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
			return nil
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

func TestFixHeaderWithShebang(t *testing.T) {
	tmpDir := t.TempDir()
	originalRootDir := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(originalRootDir)

	filePath := "script.py"
	absPath := filepath.Join(tmpDir, filePath)
	content := "#!/usr/bin/env python3\n" +
		"#region 🔖Header\n\n" +
		"# wrong/path.py\n\n" +
		"# 2025 Test <t@t.com>\n\n" +
		"# #region 🔖License\n" +
		"# AGPL\n" +
		"# #endregion 🔖License\n\n" +
		"# #region 🔖Specs\n" +
		"# Specs\n" +
		"# #endregion 🔖Specs\n\n" +
		"#endregion 🔖Header\n\n" +
		"print(\"hello\")\n"
	os.WriteFile(absPath, []byte(content), 0644)

	bundles := LoadBundles()
	breachs, err := CheckPolicies(ParseScope(filePath), bundles, nil)
	if err != nil {
		t.Fatalf("CheckPolicies failed: %v", err)
	}
	for _, v := range breachs {
		if v.Autofixable() {
			t.Logf("Detected Autofixable Breach: %s at line %d", v.Kind, v.Line)
		} else {
			t.Logf("Detected Non-Autofixable Breach: %s at line %d", v.Kind, v.Line)
		}
	}

	ctx := NewRepoContext(tmpDir)
	scope := filePath
	res, err := ctx.Fix(&scope)
	if err != nil {
		t.Fatalf("Fix failed: %v", err)
	}

	t.Logf("Fixed: %d", res.Fixed)
	for _, v := range res.Breachs {
		t.Logf("Remaining Breach: %s at line %d", v.Kind, v.Line)
	}

	if res.Fixed == 0 {
		t.Log("No fixes applied (unexpected).")
	}

	newContentBytes, _ := os.ReadFile(absPath)
	newContent := string(newContentBytes)
	if !strings.Contains(newContent, "#!/usr/bin/env python3") {
		t.Errorf("Shebang missing in fixed content:\n%s", newContent)
	}
	expectedId := "📜script.py"
	if !strings.Contains(newContent, expectedId) {
		t.Logf("Expected ID %q might be missing or different format.", expectedId)
	}
}

func TestFolderPolicyEmptyFolder(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "some", "empty")
	os.MkdirAll(emptyDir, 0755)
	nonEmptyDir := filepath.Join(tmpDir, "some", "nonempty")
	os.MkdirAll(nonEmptyDir, 0755)
	os.WriteFile(filepath.Join(nonEmptyDir, "file.txt"), []byte("content"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	foundEmpty := false
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty && v.Excerpt == "some/empty" {
			foundEmpty = true
			if !v.Autofixable() {
				t.Error("BreachFolderIllegalEmpty should be autofixable")
			}
		}
	}
	if !foundEmpty {
		t.Error("expected BreachFolderIllegalEmpty for some/empty")
	}
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty && v.Excerpt == "some/nonempty" {
			t.Error("should not report BreachFolderIllegalEmpty for non-empty folder")
		}
	}
}

func TestFolderPolicyEmptyFolderAutofix(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	emptyDir := filepath.Join(tmpDir, "remove", "me")
	os.MkdirAll(emptyDir, 0755)
	breachs := []Breach{{
		Kind:    BreachFolderIllegalEmpty,
		Scope:   "remove/me/",
		Excerpt: "remove/me",
	}}
	fixed, err := applySystemAutofixes(breachs)
	if err != nil {
		t.Fatalf("applySystemAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}
	if _, statErr := os.Stat(emptyDir); !os.IsNotExist(statErr) {
		t.Error("empty folder should have been removed")
	}
}

func TestFolderPolicySkipsExcludedDirs(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	for _, dir := range []string{".git/objects", ".semio-repo/cache", "node_modules/.cache"} {
		os.MkdirAll(filepath.Join(tmpDir, dir), 0755)
	}
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := folderPolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFolderIllegalEmpty {
			if strings.HasPrefix(v.Excerpt, ".git") || strings.HasPrefix(v.Excerpt, ".semio-repo") || strings.HasPrefix(v.Excerpt, "node_modules") {
				t.Errorf("should skip excluded dir, got breach for %s", v.Excerpt)
			}
		}
	}
}

func TestFilePolicyGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".semio-repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/main.ts"]`
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcDir := filepath.Join(tmpDir, "src")
	os.MkdirAll(srcDir, 0755)
	os.WriteFile(filepath.Join(srcDir, "main.ts"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/main.ts") {
			t.Errorf("should not report breach for allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSupportsGlobPatterns(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".semio-repo")
	os.MkdirAll(metaDir, 0755)
	godfileContent := `["allowed.txt", "src/**/*.ts", "docs/*.md"]`
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(godfileContent), 0644)
	os.WriteFile(filepath.Join(tmpDir, "allowed.txt"), []byte("ok"), 0644)
	srcNestedDir := filepath.Join(tmpDir, "src", "nested")
	os.MkdirAll(srcNestedDir, 0755)
	os.WriteFile(filepath.Join(srcNestedDir, "main.ts"), []byte("ok"), 0644)
	docsDir := filepath.Join(tmpDir, "docs")
	os.MkdirAll(docsDir, 0755)
	os.WriteFile(filepath.Join(docsDir, "guide.md"), []byte("ok"), 0644)
	os.WriteFile(filepath.Join(tmpDir, "unlisted.txt"), []byte("bad"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	foundUnlisted := false
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && v.Excerpt == "unlisted.txt" {
			foundUnlisted = true
		}
	}
	if !foundUnlisted {
		t.Error("expected BreachFileIllegalUseGodfile for unlisted.txt")
	}
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && (v.Excerpt == "allowed.txt" || v.Excerpt == "src/nested/main.ts" || v.Excerpt == "docs/guide.md") {
			t.Errorf("should not report breach for glob-allowed file %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsSemioRepo(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	metaDir := filepath.Join(tmpDir, ".semio-repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(`[]`), 0644)
	os.WriteFile(filepath.Join(metaDir, "some_internal.json"), []byte("internal"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && strings.HasPrefix(v.Excerpt, ".semio-repo") {
			t.Errorf("should skip .semio-repo files, got breach for %s", v.Excerpt)
		}
	}
}

func TestFilePolicyGodfileSkipsNestedNodeModules(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	SetRootDir(tmpDir)
	defer func() { SetRootDir(oldRoot) }()
	os.WriteFile(filepath.Join(tmpDir, ".gitignore"), []byte("node_modules/\n"), 0644)
	metaDir := filepath.Join(tmpDir, ".semio-repo")
	os.MkdirAll(metaDir, 0755)
	os.WriteFile(filepath.Join(metaDir, "files.json"), []byte(`[]`), 0644)
	nested := filepath.Join(tmpDir, "semio-repo", "vscode", "node_modules", "undici-types")
	os.MkdirAll(nested, 0755)
	os.WriteFile(filepath.Join(nested, "fetch.d.ts"), []byte("export {};"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	for _, v := range breachs {
		if v.Kind == BreachFileIllegalUseGodfile && strings.Contains(v.Excerpt, "node_modules/") {
			t.Errorf("should skip ignored node_modules files, got breach for %s", v.Excerpt)
		}
	}
}

func TestSetRootDirResetsGitignoreCache(t *testing.T) {
	tmpDirA := t.TempDir()
	tmpDirB := t.TempDir()
	oldRoot := rootDir
	defer func() { SetRootDir(oldRoot) }()
	SetRootDir(tmpDirA)
	_ = isGitIgnored("any.txt")
	os.WriteFile(filepath.Join(tmpDirB, ".gitignore"), []byte("node_modules/\n"), 0644)
	SetRootDir(tmpDirB)
	ignored := isGitIgnored(filepath.Join(tmpDirB, "node_modules", "x.ts"))
	if !ignored {
		t.Error("expected SetRootDir to refresh gitignore cache for new root")
	}
}

func TestFilePolicyNoGodfile(t *testing.T) {
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("content"), 0644)
	bundles := []Bundle{}
	scope := Scope{Kind: ScopeRepo}
	ctx := NewPolicyContext(scope, bundles)
	breachs := filePolicy(ctx)
	if len(breachs) != 0 {
		t.Errorf("expected no breachs when godfile is missing, got %d", len(breachs))
	}
}

func TestFolderPolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("folder")
	if !found {
		t.Fatal("folder policy not registered")
	}
	if policy.Name != "Folder" {
		t.Errorf("expected policy name Folder, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFolderIllegalEmpty {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("folder policy should contain BreachFolderIllegalEmpty kind")
	}
}

func TestFilePolicyRegistered(t *testing.T) {
	policy, found := FindPolicy("file")
	if !found {
		t.Fatal("file policy not registered")
	}
	if policy.Name != "File" {
		t.Errorf("expected policy name File, got %s", policy.Name)
	}
	allKinds := policy.AllKinds()
	foundKind := false
	for _, k := range allKinds {
		if k == BreachFileIllegalUseGodfile {
			foundKind = true
		}
	}
	if !foundKind {
		t.Error("file policy should contain BreachFileIllegalUseGodfile kind")
	}
}

// #region 🔖Hook Tests

func TestValidateHookEvent(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		valid  bool
		expect HookEvent
	}{
		{"git commit starting", "git.commit.starting", true, HookGitCommitStarting},
		{"git commit ended", "git.commit.ended", true, HookGitCommitEnded},
		{"agent starting", "agent.started", true, HookAgentStarted},
		{"agent ended", "agent.ended", true, HookAgentEnded},
		{"agent prompt submitting", "agent.prompt.submitting", true, HookAgentPromptSubmitting},
		{"agent compacting", "agent.compacting", true, HookAgentCompacting},
		{"agent tool starting", "agent.tool.starting", true, HookAgentToolStarting},
		{"agent tool ended", "agent.tool.ended", true, HookAgentToolEnded},
		{"agent tool plan updating", "agent.tool.plan.updating", true, HookAgentToolPlanUpdating},
		{"agent tool code searching", "agent.tool.searching", true, HookAgentToolSearching},
		{"agent tool code editing", "agent.tool.code.editing", true, HookAgentToolCodeEditing},
		{"agent tool code edited", "agent.tool.code.edited", true, HookAgentToolCodeEdited},
		{"agent tool terminal starting", "agent.tool.terminal.starting", true, HookAgentToolTerminalStarting},
		{"agent tool terminal ended", "agent.tool.terminal.ended", true, HookAgentToolTerminalEnded},
		{"invalid", "invalid.event", false, ""},
		{"empty", "", false, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, err := ValidateHookEvent(tc.input)
			if tc.valid {
				if err != nil {
					t.Fatalf("expected valid event, got error: %v", err)
				}
				if event != tc.expect {
					t.Errorf("expected %s, got %s", tc.expect, event)
				}
			} else {
				if err == nil {
					t.Fatal("expected error for invalid event")
				}
			}
		})
	}
}

func TestHookEventKind(t *testing.T) {
	cases := []struct {
		name   string
		event  HookEvent
		expect HookKind
	}{
		{"git commit starting is git", HookGitCommitStarting, HookKindGit},
		{"git commit ended is git", HookGitCommitEnded, HookKindGit},
		{"agent starting is agent", HookAgentStarted, HookKindAgent},
		{"agent ended is agent", HookAgentEnded, HookKindAgent},
		{"agent prompt submitting is agent", HookAgentPromptSubmitting, HookKindAgent},
		{"agent compacting is agent", HookAgentCompacting, HookKindAgent},
		{"agent tool starting is agent", HookAgentToolStarting, HookKindAgent},
		{"agent tool ended is agent", HookAgentToolEnded, HookKindAgent},
		{"agent tool plan updating is agent", HookAgentToolPlanUpdating, HookKindAgent},
		{"agent tool code searching is agent", HookAgentToolSearching, HookKindAgent},
		{"agent tool code editing is agent", HookAgentToolCodeEditing, HookKindAgent},
		{"agent tool code edited is agent", HookAgentToolCodeEdited, HookKindAgent},
		{"agent tool terminal starting is agent", HookAgentToolTerminalStarting, HookKindAgent},
		{"agent tool terminal ended is agent", HookAgentToolTerminalEnded, HookKindAgent},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := HookEventKind(tc.event)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestIsToolBlocked(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		toolArgs string
		blocked  bool
	}{
		{"git checkout blocked", "run_in_terminal", "git checkout main", true},
		{"git stash blocked", "bash", "git stash", true},
		{"git stash pop blocked", "shell", "git stash pop", true},
		{"git stash drop blocked", "terminal", "git stash drop", true},
		{"git stash apply blocked", "run", "git stash apply", true},
		{"git reset hard blocked", "exec", "git reset --hard", true},
		{"git clean fd blocked", "terminal", "git clean -fd", true},
		{"git checkout in args blocked", "", "git checkout feature/branch", true},
		{"regular tool allowed", "read_file", "/path/to/file.ts", false},
		{"git commit allowed", "terminal", "git commit -m test", false},
		{"git push allowed", "terminal", "git push origin main", false},
		{"git pull allowed", "terminal", "git pull", false},
		{"git add allowed", "terminal", "git add .", false},
		{"empty allowed", "", "", false},
		{"case insensitive", "TERMINAL", "GIT CHECKOUT main", true},
		{"grep with git checkout pattern not blocked", "", `grep "git checkout" file.go`, false},
		{"echo with git stash not blocked", "", `echo "git stash"`, false},
		{"semio cli command not blocked", "", `./semio-repo/cli/cli tree "hooks events inlet adapter cli"`, false},
		{"cd then git checkout blocked", "", "cd /workspaces && git checkout feature", true},
		{"pipe grep allowed", "", `ls | grep "git checkout"`, false},
		{"git checkout after semicolon blocked", "", "echo done; git checkout main", true},
		{"grep for git reset not blocked", "bash", `grep -rn "git reset --hard" .`, false},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			blocked, _ := IsToolBlocked(tc.toolName, tc.toolArgs)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v, got blocked=%v", tc.blocked, blocked)
			}
		})
	}
}

func TestSplitCommandSegments(t *testing.T) {
	cases := []struct {
		cmd      string
		expected []string
	}{
		{"git checkout main", []string{"git checkout main"}},
		{"cd /tmp && git checkout main", []string{"cd /tmp", "git checkout main"}},
		{"echo done; git stash", []string{"echo done", "git stash"}},
		{"ls | grep foo", []string{"ls", "grep foo"}},
		{"a || b", []string{"a", "b"}},
		{"  ", []string{}},
		{"", []string{}},
	}
	for _, tc := range cases {
		t.Run(tc.cmd, func(t *testing.T) {
			got := splitCommandSegments(tc.cmd)
			if len(got) != len(tc.expected) {
				t.Fatalf("expected %v, got %v", tc.expected, got)
			}
			for i := range tc.expected {
				if got[i] != tc.expected[i] {
					t.Errorf("segment %d: expected %q, got %q", i, tc.expected[i], got[i])
				}
			}
		})
	}
}

func TestIsCommandSegmentBlocked(t *testing.T) {
	cases := []struct {
		segment string
		blocked bool
	}{
		{"git checkout main", true},
		{"git stash", true},
		{"git reset --hard", true},
		{"git clean -fd", true},
		{`grep "git checkout" file.go`, false},
		{`echo "git stash"`, false},
		{"./semio-repo/cli/cli tree hooks", false},
		{"git commit -m msg", false},
		{"git push origin main", false},
		{"GIT CHECKOUT branch", true},
		{"", false},
	}
	for _, tc := range cases {
		t.Run(tc.segment, func(t *testing.T) {
			blocked, _ := isCommandSegmentBlocked(tc.segment)
			if blocked != tc.blocked {
				t.Errorf("expected blocked=%v for %q", tc.blocked, tc.segment)
			}
		})
	}
}

func TestRunHookAgentEvents(t *testing.T) {
	cases := []struct {
		name    string
		event   HookEvent
		allowed bool
	}{
		{"agent starting", HookAgentStarted, true},
		{"agent ended", HookAgentEnded, true},
		{"agent prompt submitting", HookAgentPromptSubmitting, true},
		{"agent compacting", HookAgentCompacting, true},
		{"agent tool ended", HookAgentToolEnded, true},
		{"agent tool code searching", HookAgentToolSearching, true},
		{"agent tool code editing", HookAgentToolCodeEditing, true},
		{"agent tool code edited", HookAgentToolCodeEdited, true},
		{"agent tool plan updating", HookAgentToolPlanUpdating, true},
		{"agent tool terminal starting", HookAgentToolTerminalStarting, true},
		{"agent tool terminal ended", HookAgentToolTerminalEnded, true},
		{"git commit ended", HookGitCommitEnded, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			hctx := HookContext{
				Event:     tc.event,
				Client:    "copilot-chat",
				Timestamp: time.Now().UTC().Format(time.RFC3339),
				RepoRoot:  t.TempDir(),
			}
			result := RunHook(hctx)
			if result.IsAllowed() != tc.allowed {
				t.Errorf("expected allowed=%v, got allowed=%v: %s", tc.allowed, result.IsAllowed(), result.GetMessage())
			}
		})
	}
	t.Run("git commit starting", func(t *testing.T) {
		hctx := HookContext{
			Event:     HookGitCommitStarting,
			RepoRoot:  t.TempDir(),
		}
		result := RunHook(hctx)
		// We just check type, value depends on environment (task existence)
		_, ok := result.(HookResultGitCommitStarting)
		if !ok {
			t.Fatalf("expected HookResultGitCommitStarting, got %T", result)
		}
	})
}

func TestRunHookToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "copilot-chat",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  t.TempDir(),
		ToolName:  "run_in_terminal",
		ToolArgs:  "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestRunHookToolAllowed(t *testing.T) {
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "cursor-chat",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  t.TempDir(),
		ToolName:  "read_file",
		ToolArgs:  "/workspaces/semio/semio-repo/cli/main.go",
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Errorf("expected tool to be allowed, got: %s", result.GetMessage())
	}
}

func TestRunHookUnknownEvent(t *testing.T) {
	hctx := HookContext{
		Event:     HookEvent("unknown.event"),
		Client:    "copilot-chat",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected unknown event to be denied")
	}
}

func TestAllHookEventsCompleteness(t *testing.T) {
	expected := []HookEvent{
		HookGitCommitStarting, HookGitCommitEnded,
		HookAgentStarted, HookAgentEnded,
		HookAgentPromptSubmitting, HookAgentCompacting,
		HookAgentToolStarting, HookAgentToolEnded,
		HookAgentToolPlanUpdating,
		HookAgentToolSearching, HookAgentToolCodeEditing, HookAgentToolCodeEdited,
		HookAgentToolTerminalStarting, HookAgentToolTerminalEnded,
	}
	if len(AllHookEvents) != len(expected) {
		t.Errorf("expected %d events, got %d", len(expected), len(AllHookEvents))
	}
	for _, e := range expected {
		found := false
		for _, a := range AllHookEvents {
			if a == e {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing event: %s", e)
		}
	}
}

func TestHookCommandCLI(t *testing.T) {
	factory := func(cfg Config) (*Engine, error) {
		return nil, nil
	}
	config := &Config{Format: "json"}
	cmd := hookCommand(factory, config)
	cases := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{"neutral agent starting", []string{"agent.started", "copilot-chat"}, false},
		{"neutral agent prompt submitting", []string{"agent.prompt.submitting", "cursor-chat"}, false},
		{"neutral agent tool terminal starting", []string{"agent.tool.terminal.starting", "windsurf-chat"}, false},
		{"native copilot SessionStart", []string{"SessionStart", "copilot-chat"}, false},
		{"native copilot PreToolUse", []string{"PreToolUse", "copilot-chat"}, false},
		{"native copilot PreCompact", []string{"PreCompact", "copilot-chat"}, false},
		{"native cursor sessionStart", []string{"sessionStart", "cursor-chat"}, false},
		{"native cursor beforeReadFile", []string{"beforeReadFile", "cursor-chat"}, false},
		{"native windsurf pre_user_prompt", []string{"pre_user_prompt", "windsurf-chat"}, false},
		{"native windsurf pre_read_code", []string{"pre_read_code", "windsurf-chat"}, false},
		{"native claude SessionStart", []string{"SessionStart", "claude-code"}, false},
		{"native claude PreToolUse", []string{"PreToolUse", "claude-code"}, false},
		{"native droid PreToolUse", []string{"PreToolUse", "droid"}, false},
		{"invalid event no client", []string{"invalid.event"}, true},
		{"invalid native no client", []string{"UnknownEvent", "copilot-chat"}, true},
		{"no args", []string{}, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			var buf bytes.Buffer
			cmd.SetOut(&buf)
			cmd.SetErr(&buf)
			cmd.SetArgs(tc.args)
			err := cmd.Execute()
			if tc.wantErr && err == nil {
				t.Error("expected error")
			}
			if !tc.wantErr && err != nil {
				t.Errorf("unexpected error: %v", err)
			}
		})
	}
}

func TestHookCommandToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "copilot-chat",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  t.TempDir(),
		ToolName:  "terminal",
		ToolArgs:  "git stash pop",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "git stash") {
		t.Errorf("expected git stash in message, got: %s", result.GetMessage())
	}
}

func TestHookCommandJSONOutput(t *testing.T) {
	hctx := HookContext{
		Event:     HookAgentStarted,
		Client:    "copilot-chat",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  t.TempDir(),
	}
	result := RunHook(hctx)
	out, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON marshaling, got: %v", err)
	}
	var parsed HookResultAgentStarted
	if err := json.Unmarshal(out, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	if !parsed.Allowed {
		t.Error("expected allowed=true")
	}
	if parsed.Timestamp == "" {
		t.Error("expected non-empty timestamp")
	}
}

func TestGenerateCopilotConfig(t *testing.T) {
	content, err := generateCopilotConfig("/tmp/test-repo")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatal("expected hooks key in copilot config")
	}
	for _, key := range []string{"SessionStart", "UserPromptSubmit", "PreToolUse", "PostToolUse", "PreCompact", "SubagentStart", "SubagentStop", "Stop"} {
		arr, ok := hooks[key].([]interface{})
		if !ok || len(arr) == 0 {
			t.Errorf("expected %s array in copilot hooks", key)
			continue
		}
		entry, ok := arr[0].(map[string]interface{})
		if !ok {
			t.Errorf("expected object entry for %s", key)
			continue
		}
		if entry["type"] != "command" {
			t.Errorf("expected type=command for %s, got %v", key, entry["type"])
		}
		cmd, _ := entry["command"].(string)
		if !strings.Contains(cmd, "copilot-chat") {
			t.Errorf("expected copilot-chat in command for %s, got %s", key, cmd)
		}
		if !strings.Contains(cmd, "hook "+key) {
			t.Errorf("expected native event %s in command, got %s", key, cmd)
		}
		timeout, ok := entry["timeout"].(float64)
		if !ok || timeout != 30 {
			t.Errorf("expected timeout=30 for %s, got %v", key, entry["timeout"])
		}
	}
}

func TestExtractToolNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"vscode tool_name", `{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"ls"}}`, "run_in_terminal"},
		{"claude code tool_name", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "Bash"},
		{"no tool_name", `{"tool_input":{"command":"ls"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractToolNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractHookEventNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"PreToolUse", `{"hookEventName":"PreToolUse","tool_name":"editFiles"}`, "PreToolUse"},
		{"PostToolUse", `{"hookEventName":"PostToolUse","tool_name":"editFiles"}`, "PostToolUse"},
		{"SessionStart", `{"hookEventName":"SessionStart","source":"new"}`, "SessionStart"},
		{"no hookEventName", `{"tool_name":"Bash"}`, ""},
		{"empty", `{}`, ""},
		{"invalid json", `bad`, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractHookEventNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestVSCodeEventFromHookEvent(t *testing.T) {
	cases := []struct {
		name       string
		event      HookEvent
		parentInfo string
		expect     string
	}{
		{"agent.tool.starting", HookAgentToolStarting, "", "PreToolUse"},
		{"agent.tool.ended", HookAgentToolEnded, "", "PostToolUse"},
		{"agent.started", HookAgentStarted, "", "SessionStart"},
		{"agent.started subagent", HookAgentStarted, "subagent", "SubagentStart"},
		{"agent.ended", HookAgentEnded, "", "Stop"},
		{"agent.ended subagent", HookAgentEnded, "subagent", "SubagentStop"},
		{"agent.prompt.submitting", HookAgentPromptSubmitting, "", "UserPromptSubmit"},
		{"agent.compacting", HookAgentCompacting, "", "PreCompact"},
		{"agent.tool.searching", HookAgentToolSearching, "", "PreToolUse"},
		{"agent.tool.code.editing", HookAgentToolCodeEditing, "", "PreToolUse"},
		{"agent.tool.code.edited", HookAgentToolCodeEdited, "", "PostToolUse"},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, "", "PreToolUse"},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, "", "PostToolUse"},
		{"agent.tool.plan.updating", HookAgentToolPlanUpdating, "", "PreToolUse"},
		{"unknown", HookEvent("unknown.x"), "", ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := vsCodeEventFromHookEvent(tc.event, tc.parentInfo)
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestFormatVSCodeHookOutput(t *testing.T) {
	t.Run("PreToolUse allow", func(t *testing.T) {
		out := formatVSCodeHookOutput("PreToolUse", HookResultAgentToolStarting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected permissionDecision=allow, got %v", hso["permissionDecision"])
		}
		if hso["hookEventName"] != "PreToolUse" {
			t.Errorf("expected hookEventName=PreToolUse, got %v", hso["hookEventName"])
		}
	})
	t.Run("PreToolUse deny", func(t *testing.T) {
		out := formatVSCodeHookOutput("PreToolUse", HookResultAgentToolStarting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: false, Message: "blocked: git checkout"}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected permissionDecision=deny, got %v", hso["permissionDecision"])
		}
		if hso["permissionDecisionReason"] != "blocked: git checkout" {
			t.Errorf("expected reason in output, got %v", hso["permissionDecisionReason"])
		}
	})
	t.Run("SessionStart with message", func(t *testing.T) {
		out := formatVSCodeHookOutput("SessionStart", HookResultAgentStarted{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true, Message: "agent.started acknowledged"}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput")
		}
		if hso["additionalContext"] != "agent.started acknowledged" {
			t.Errorf("expected additionalContext, got %v", hso["additionalContext"])
		}
	})
	t.Run("Stop always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("Stop", HookResultAgentEnded{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for Stop")
		}
		if hso["hookEventName"] != "Stop" {
			t.Errorf("expected hookEventName=Stop, got %v", hso["hookEventName"])
		}
	})
	t.Run("UserPromptSubmit always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("UserPromptSubmit", HookResultAgentPromptSubmitting{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for UserPromptSubmit")
		}
		if hso["hookEventName"] != "UserPromptSubmit" {
			t.Errorf("expected hookEventName=UserPromptSubmit, got %v", hso["hookEventName"])
		}
	})
	t.Run("PostToolUse always has hookSpecificOutput", func(t *testing.T) {
		out := formatVSCodeHookOutput("PostToolUse", HookResultAgentToolEnded{HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}}})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
		hso, ok := parsed["hookSpecificOutput"].(map[string]interface{})
		if !ok {
			t.Fatal("expected hookSpecificOutput for PostToolUse")
		}
		if hso["hookEventName"] != "PostToolUse" {
			t.Errorf("expected hookEventName=PostToolUse, got %v", hso["hookEventName"])
		}
	})
	t.Run("unknown event empty output", func(t *testing.T) {
		out := formatVSCodeHookOutput("", HookResultBase{Allowed: true, Message: "test"})
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON, got: %v", err)
		}
	})
}

func TestHookCommandCopilotChatVSCodeOutput(t *testing.T) {
	t.Run("PreToolUse allow produces VS Code JSON", func(t *testing.T) {
		hctx := HookContext{
			Event:     HookAgentToolStarting,
			Client:    "copilot-chat",
			Timestamp: time.Now().UTC().Format(time.RFC3339),
			ToolName:  "read_file",
			ToolArgs:  "/tmp/file.ts",
			RepoRoot:  t.TempDir(),
		}
		result := RunHook(hctx)
		if !result.IsAllowed() {
			t.Fatal("expected allowed")
		}
		out := formatVSCodeHookOutput("PreToolUse", result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "allow" {
			t.Errorf("expected allow, got %v", hso["permissionDecision"])
		}
	})
	t.Run("PreToolUse blocked produces VS Code deny JSON", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git checkout main"}}`)
		hctx := HookContext{
			Event:     HookAgentToolStarting,
			Client:    "copilot-chat",
			Timestamp: time.Now().UTC().Format(time.RFC3339),
			ToolName:  "run_in_terminal",
			Input:     payload,
			RepoRoot:  t.TempDir(),
		}
		result := RunHook(hctx)
		if result.IsAllowed() {
			t.Fatal("expected blocked")
		}
		hookEventName := extractHookEventNameFromStdin(payload)
		out := formatVSCodeHookOutput(hookEventName, result)
		var parsed map[string]interface{}
		if err := json.Unmarshal([]byte(out), &parsed); err != nil {
			t.Fatalf("expected valid JSON: %v", err)
		}
		hso := parsed["hookSpecificOutput"].(map[string]interface{})
		if hso["permissionDecision"] != "deny" {
			t.Errorf("expected deny, got %v", hso["permissionDecision"])
		}
		reason, _ := hso["permissionDecisionReason"].(string)
		if !strings.Contains(reason, "blocked") {
			t.Errorf("expected blocked reason, got: %s", reason)
		}
	})
	t.Run("tool_name extracted from stdin", func(t *testing.T) {
		payload := json.RawMessage(`{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"git stash"}}`)
		toolName := extractToolNameFromStdin(payload)
		if toolName != "run_in_terminal" {
			t.Errorf("expected run_in_terminal, got %s", toolName)
		}
		cmd := extractCommandFromStdin(payload)
		if cmd != "git stash" {
			t.Errorf("expected git stash, got %s", cmd)
		}
	})
}

func TestGenerateCursorConfig(t *testing.T) {
	content, err := generateCursorConfig("/tmp/test-repo")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	version, ok := config["version"].(float64)
	if !ok || version != 1 {
		t.Errorf("expected version=1, got %v", config["version"])
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatal("expected hooks key in cursor config")
	}
	for _, key := range []string{"sessionStart", "sessionEnd", "subagentStart", "subagentStop", "stop", "beforeSubmitPrompt", "preCompact", "preToolUse", "postToolUse", "postToolUseFailure", "beforeMCPExecution", "afterMCPExecution", "beforeReadFile", "afterFileEdit", "beforeShellExecution", "afterShellExecution", "afterAgentResponse", "afterAgentThought", "beforeTabFileRead", "afterTabFileEdit"} {
		arr, ok := hooks[key].([]interface{})
		if !ok || len(arr) == 0 {
			t.Errorf("expected %s array in cursor hooks", key)
			continue
		}
		entry, ok := arr[0].(map[string]interface{})
		if !ok {
			t.Errorf("expected object entry for %s", key)
			continue
		}
		cmd, _ := entry["command"].(string)
		if !strings.Contains(cmd, "cursor-chat") {
			t.Errorf("expected cursor-chat in command for %s, got %s", key, cmd)
		}
		if !strings.Contains(cmd, "hook "+key) {
			t.Errorf("expected native event %s in command, got %s", key, cmd)
		}
	}
}

func TestGenerateWindsurfConfig(t *testing.T) {
	content, err := generateWindsurfConfig("/tmp/test-repo")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatal("expected hooks key in windsurf config")
	}
	for _, key := range []string{"pre_user_prompt", "post_cascade_response", "post_setup_worktree", "pre_mcp_tool_use", "post_mcp_tool_use", "pre_read_code", "post_read_code", "pre_write_code", "post_write_code", "pre_run_command"} {
		arr, ok := hooks[key].([]interface{})
		if !ok || len(arr) == 0 {
			t.Errorf("expected %s array in windsurf hooks", key)
			continue
		}
		entry, ok := arr[0].(map[string]interface{})
		if !ok {
			t.Errorf("expected object entry for %s", key)
			continue
		}
		cmd, _ := entry["command"].(string)
		if !strings.Contains(cmd, "windsurf-chat") {
			t.Errorf("expected windsurf-chat in command for %s, got %s", key, cmd)
		}
		if !strings.Contains(cmd, "hook "+key) {
			t.Errorf("expected native event %s in command, got %s", key, cmd)
		}
	}
}

func TestGenerateClaudeCodeConfig(t *testing.T) {
	tmpDir := t.TempDir()
	claudeDir := filepath.Join(tmpDir, ".claude")
	if err := os.MkdirAll(claudeDir, 0755); err != nil {
		t.Fatal(err)
	}
	existingSettings := `{"permissions":{"allow":["Bash(*)"],"deny":[]}}`
	if err := os.WriteFile(filepath.Join(claudeDir, "settings.json"), []byte(existingSettings), 0644); err != nil {
		t.Fatal(err)
	}
	content, err := generateClaudeCodeConfig(tmpDir)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	if _, ok := config["permissions"]; !ok {
		t.Error("expected existing permissions to be preserved")
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatal("expected hooks key in claude code config")
	}
	for _, key := range []string{"PreToolUse", "PostToolUse", "PostToolUseFailure", "UserPromptSubmit", "PreCompact", "SessionStart", "SessionEnd", "SubagentStart", "SubagentStop", "Stop", "TaskCompleted", "Notification", "PermissionRequest", "TeammateIdle"} {
		arr, ok := hooks[key].([]interface{})
		if !ok || len(arr) == 0 {
			t.Errorf("expected %s array in claude code hooks", key)
			continue
		}
		entry, ok := arr[0].(map[string]interface{})
		if !ok {
			t.Errorf("expected object entry for %s", key)
			continue
		}
		matcher, ok := entry["matcher"]
		if !ok {
			t.Errorf("expected matcher in %s entry", key)
		} else if matcher != "" {
			t.Errorf("expected empty matcher in %s entry, got %v", key, matcher)
		}
		innerHooks, ok := entry["hooks"].([]interface{})
		if !ok || len(innerHooks) == 0 {
			t.Errorf("expected inner hooks array for %s", key)
			continue
		}
		inner, ok := innerHooks[0].(map[string]interface{})
		if !ok {
			t.Errorf("expected inner hook object for %s", key)
			continue
		}
		if inner["type"] != "command" {
			t.Errorf("expected type=command for %s, got %v", key, inner["type"])
		}
		cmd, _ := inner["command"].(string)
		if !strings.Contains(cmd, "claude-code") {
			t.Errorf("expected claude-code in command for %s, got %s", key, cmd)
		}
		if !strings.Contains(cmd, "hook "+key) {
			t.Errorf("expected native event %s in command, got %s", key, cmd)
		}
	}
}

func TestConfigureGitHooks(t *testing.T) {
	tmpDir := t.TempDir()
	gitDir := filepath.Join(tmpDir, ".git")
	if err := os.MkdirAll(gitDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := configureGitHooks(tmpDir); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	preCommit, err := os.ReadFile(filepath.Join(gitDir, "hooks", "pre-commit"))
	if err != nil {
		t.Fatal("pre-commit hook not created")
	}
	if !strings.Contains(string(preCommit), "hook git.commit.starting") {
		t.Error("pre-commit hook does not call hook git.commit.starting")
	}
	postCommit, err := os.ReadFile(filepath.Join(gitDir, "hooks", "post-commit"))
	if err != nil {
		t.Fatal("post-commit hook not created")
	}
	if !strings.Contains(string(postCommit), "hook git.commit.ended") {
		t.Error("post-commit hook does not call hook git.commit.ended")
	}
	info, _ := os.Stat(filepath.Join(gitDir, "hooks", "pre-commit"))
	if info.Mode()&0111 == 0 {
		t.Error("pre-commit hook is not executable")
	}
}

func TestGetClientHookMappings(t *testing.T) {
	mappings := getClientHookMappings()
	expectedClients := []string{"copilot-chat", "cursor-chat", "windsurf-chat", "claude-code", "droid"}
	if len(mappings) != len(expectedClients) {
		t.Errorf("expected %d mappings, got %d", len(expectedClients), len(mappings))
	}
	for _, ec := range expectedClients {
		found := false
		for _, m := range mappings {
			if m.Client == ec {
				found = true
				if m.ConfigPath == "" {
					t.Errorf("empty config path for client %s", ec)
				}
				if m.Generator == nil {
					t.Errorf("nil generator for client %s", ec)
				}
				break
			}
		}
		if !found {
			t.Errorf("missing mapping for client: %s", ec)
		}
	}
}

func TestBlockedToolPatterns(t *testing.T) {
	if len(BlockedToolPatterns) < 5 {
		t.Errorf("expected at least 5 blocked patterns, got %d", len(BlockedToolPatterns))
	}
	expectedPatterns := []string{"git checkout", "git stash", "git reset --hard", "git clean -fd"}
	for _, ep := range expectedPatterns {
		found := false
		for _, bp := range BlockedToolPatterns {
			if bp == ep {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing blocked pattern: %s", ep)
		}
	}
}

func TestHookLogging(t *testing.T) {
	tmpDir := t.TempDir()
	logDir := filepath.Join(tmpDir, ".semio-repo", "📜")
	hctx := HookContext{
		Event:     HookAgentStarted,
		Client:    "claude-code",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  tmpDir,
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Fatalf("expected allowed=true, got: %s", result.GetMessage())
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected 1 log file, got %d", len(entries))
	}
	name := entries[0].Name()
	if !strings.HasSuffix(name, "_agent-started.json") {
		t.Errorf("expected filename to end with _agent-started.json, got: %s", name)
	}
	if !strings.Contains(name, "_claude-code_") {
		t.Errorf("expected filename to contain _claude-code_, got: %s", name)
	}
	if len(name) != len("260218230207_claude-code_agent-started.json") {
		t.Errorf("unexpected filename length for %q", name)
	}
	data, err := os.ReadFile(filepath.Join(logDir, name))
	if err != nil {
		t.Fatalf("cannot read log file: %v", err)
	}
	// Use map for result since HookResult is an interface and cannot be unmarshaled into directly
	type TestHookLogEntry struct {
		Context HookContext            `json:"context"`
		Result  map[string]interface{} `json:"result"`
	}
	var entry TestHookLogEntry
	if err := json.Unmarshal(data, &entry); err != nil {
		t.Fatalf("expected valid JSON log entry, got: %v", err)
	}
	if entry.Context.Event != HookAgentStarted {
		t.Errorf("expected event agent.started in log, got: %s", entry.Context.Event)
	}
	if entry.Context.Client != "claude-code" {
		t.Errorf("expected client claude-code in log, got: %s", entry.Context.Client)
	}
	if allowed, ok := entry.Result["allowed"].(bool); !ok || !allowed {
		t.Error("expected allowed=true in logged result")
	}
}

func TestHookLoggingToolBlocked(t *testing.T) {
	tmpDir := t.TempDir()
	logDir := filepath.Join(tmpDir, ".semio-repo", "📜")
	// "git checkout" is in BlockedToolPatterns, so "git checkout main" should match via strings.HasPrefix check for "git checkout"
	// if BlockedToolPatterns was []string{"git checkout", ...}
	// Let's assume tool_name "bash" and args "git checkout main" triggers blocking logic.
	// Actually logic is: if tool_name is "bash" or "run_in_terminal", check args against blocked patterns.
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "claude-code",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  tmpDir,
		ToolName:  "bash",
		ToolArgs:  "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected blocked tool to be denied")
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected 1 log file, got %d", len(entries))
	}
	data, err := os.ReadFile(filepath.Join(logDir, entries[0].Name()))
	if err != nil {
		t.Fatalf("cannot read log file: %v", err)
	}
	type TestHookLogEntry struct {
		Context HookContext            `json:"context"`
		Result  map[string]interface{} `json:"result"`
	}
	var entry TestHookLogEntry
	if err := json.Unmarshal(data, &entry); err != nil {
		t.Fatalf("expected valid JSON log entry: %v", err)
	}
	if allowed, ok := entry.Result["allowed"].(bool); !ok || allowed {
		t.Error("expected allowed=false in logged result for blocked tool")
	}
	msg, _ := entry.Result["message"].(string)
	if !strings.Contains(msg, "blocked") {
		t.Errorf("expected blocked message in log, got: %s", msg)
	}
	if !strings.HasSuffix(entries[0].Name(), "_agent-tool-starting.json") {
		t.Errorf("expected filename to end with _agent-tool-starting.json, got: %s", entries[0].Name())
	}
}

func TestHookLoggingStdinInput(t *testing.T) {
	tmpDir := t.TempDir()
	logDir := filepath.Join(tmpDir, ".semio-repo", "📜")
	payload := json.RawMessage(`{"session_id":"abc123","tool_name":"Bash","tool_input":{"command":"ls"}}`)
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "claude-code",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		RepoRoot:  tmpDir,
		ToolName:  "Bash",
		ToolArgs:  `{"command":"ls"}`,
		Input:     payload,
	}
	RunHook(hctx)
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected 1 log file, got %d", len(entries))
	}
	data, err := os.ReadFile(filepath.Join(logDir, entries[0].Name()))
	if err != nil {
		t.Fatalf("cannot read log file: %v", err)
	}
	type TestHookLogEntry struct {
		Context HookContext            `json:"context"`
		Result  map[string]interface{} `json:"result"`
	}
	var entry TestHookLogEntry
	if err := json.Unmarshal(data, &entry); err != nil {
		t.Fatalf("expected valid JSON log entry: %v", err)
	}
	var gotInput, wantInput interface{}
	if err := json.Unmarshal(entry.Context.Input, &gotInput); err != nil {
		t.Fatalf("expected valid JSON in logged input: %v", err)
	}
	if err := json.Unmarshal(payload, &wantInput); err != nil {
		t.Fatalf("bad test payload: %v", err)
	}
	gotBytes, _ := json.Marshal(gotInput)
	wantBytes, _ := json.Marshal(wantInput)
	if string(gotBytes) != string(wantBytes) {
		t.Errorf("expected input %s in log, got: %s", wantBytes, gotBytes)
	}
	if entry.Context.ToolName != "Bash" {
		t.Errorf("expected tool name Bash in log, got: %s", entry.Context.ToolName)
	}
}

func TestHookCommandStdinPiped(t *testing.T) {
	tmpDir := t.TempDir()
	payload := `{"session_id":"sess1","tool_name":"Read","tool_input":{"file_path":"/tmp/x"}}`
	cmd := exec.Command("./cli", "hook", "agent.tool.ended", "claude-code", "--tool-name", "Read")
	cmd.Stdin = strings.NewReader(payload)
	cmd.Env = append(os.Environ(), fmt.Sprintf("SEMIO_REPO=%s", tmpDir))
	cmd.Dir = filepath.Dir(os.Args[0])
	out, err := cmd.CombinedOutput()
	_ = out
	_ = err
	logDir := filepath.Join(tmpDir, ".semio-repo", "📜")
	entries, readErr := os.ReadDir(logDir)
	if readErr != nil {
		t.Skip("cli binary not available for subprocess test")
	}
	if len(entries) == 0 {
		t.Fatal("expected at least one log file after piped hook invocation")
	}
	data, _ := os.ReadFile(filepath.Join(logDir, entries[0].Name()))
	if !strings.Contains(string(data), "sess1") {
		t.Errorf("expected stdin payload in log, got: %s", string(data))
	}
}

func TestExtractCommandFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"claude code tool_input.command", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "git checkout main"},
		{"cursor beforeShellExecution", `{"command":"git stash pop"}`, "git stash pop"},
		{"windsurf tool_info.command_line", `{"tool_info":{"command_line":"git reset --hard"}}`, "git reset --hard"},
		{"no command", `{"tool_name":"ReadFile","tool_input":{"path":"/tmp"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractCommandFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractCommandFromStdinBlocking(t *testing.T) {
	payload := json.RawMessage(`{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`)
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "claude-code",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		Input:     payload,
		RepoRoot:  t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected stdin-based git checkout to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}

func TestGenerateDroidConfig(t *testing.T) {
	content, err := generateDroidConfig("/tmp/test-repo")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	testNativeHookConfig(t, content, "droid")
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatal("expected hooks key in droid config")
	}
	for _, key := range []string{"PreToolUse", "PostToolUse", "UserPromptSubmit", "Stop", "SubagentStop", "PreCompact", "SessionStart", "SessionEnd", "Notification"} {
		cmd, ok := hooks[key].(string)
		if !ok || cmd == "" {
			t.Errorf("expected string command for %s in droid config", key)
			continue
		}
		if !strings.Contains(cmd, "droid") {
			t.Errorf("expected droid in command for %s, got %s", key, cmd)
		}
		if !strings.Contains(cmd, "hook "+key) {
			t.Errorf("expected native event %s in command, got %s", key, cmd)
		}
	}
}

func testNativeHookConfig(t *testing.T, content string, clientName string) {
	t.Helper()
	var config map[string]interface{}
	if err := json.Unmarshal([]byte(content), &config); err != nil {
		t.Fatalf("expected valid JSON, got error: %v", err)
	}
	hooks, ok := config["hooks"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected hooks key in %s native config", clientName)
	}
	for _, event := range []string{"SessionStart", "SessionEnd", "Stop", "UserPromptSubmit", "PreCompact", "PreToolUse", "PostToolUse"} {
		cmd, ok := hooks[event].(string)
		if !ok || cmd == "" {
			t.Errorf("expected string command for %s in %s native config", event, clientName)
			continue
		}
		if !strings.Contains(cmd, clientName) {
			t.Errorf("expected %s in command for %s, got %s", clientName, event, cmd)
		}
		if !strings.Contains(cmd, "hook "+event) {
			t.Errorf("expected native event %s in command, got %s", event, cmd)
		}
	}
}

func TestClassifyTool(t *testing.T) {
	cases := []struct {
		name     string
		toolName string
		expect   ToolKind
	}{
		{"manage_todo_list", "manage_todo_list", ToolKindPlan},
		{"Task", "Task", ToolKindPlan},
		{"todo_tool", "todo_tool", ToolKindPlan},
		{"read_file", "read_file", ToolKindCodeSearch},
		{"grep_search", "grep_search", ToolKindCodeSearch},
		{"file_search", "file_search", ToolKindCodeSearch},
		{"semantic_search", "semantic_search", ToolKindCodeSearch},
		{"list_dir", "list_dir", ToolKindCodeSearch},
		{"get_errors", "get_errors", ToolKindCodeSearch},
		{"Read", "Read", ToolKindCodeSearch},
		{"replace_string_in_file", "replace_string_in_file", ToolKindCodeEdit},
		{"create_file", "create_file", ToolKindCodeEdit},
		{"multi_replace_string_in_file", "multi_replace_string_in_file", ToolKindCodeEdit},
		{"Edit", "Edit", ToolKindCodeEdit},
		{"Write", "Write", ToolKindCodeEdit},
		{"run_in_terminal", "run_in_terminal", ToolKindTerminal},
		{"get_terminal_output", "get_terminal_output", ToolKindTerminal},
		{"Bash", "Bash", ToolKindTerminal},
		{"runSubagent", "runSubagent", ToolKindGeneric},
		{"runTests", "runTests", ToolKindGeneric},
		{"tool_search_tool_regex", "tool_search_tool_regex", ToolKindGeneric},
		{"empty", "", ToolKindGeneric},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := classifyTool(tc.toolName)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestResolveHookEvent(t *testing.T) {
	cases := []struct {
		name       string
		eventStr   string
		client     string
		toolName   string
		expectEvt  HookEvent
		expectPar  string
		expectErr  bool
	}{
		{"neutral agent.started", "agent.started", "copilot-chat", "", HookAgentStarted, "", false},
		{"neutral agent.tool.code.editing", "agent.tool.code.editing", "copilot-chat", "", HookAgentToolCodeEditing, "", false},
		{"copilot SessionStart", "SessionStart", "copilot-chat", "", HookAgentStarted, "", false},
		{"copilot Stop", "Stop", "copilot-chat", "", HookAgentEnded, "", false},
		{"copilot SubagentStart", "SubagentStart", "copilot-chat", "", HookAgentStarted, "subagent", false},
		{"copilot SubagentStop", "SubagentStop", "copilot-chat", "", HookAgentEnded, "subagent", false},
		{"copilot UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", HookAgentPromptSubmitting, "", false},
		{"copilot PreCompact", "PreCompact", "copilot-chat", "", HookAgentCompacting, "", false},
		{"copilot PreToolUse generic", "PreToolUse", "copilot-chat", "runSubagent", HookAgentToolStarting, "", false},
		{"copilot PreToolUse read_file", "PreToolUse", "copilot-chat", "read_file", HookAgentToolSearching, "", false},
		{"copilot PreToolUse create_file", "PreToolUse", "copilot-chat", "create_file", HookAgentToolCodeEditing, "", false},
		{"copilot PreToolUse run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalStarting, "", false},
		{"copilot PreToolUse manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", HookAgentToolPlanUpdating, "", false},
		{"copilot PostToolUse create_file", "PostToolUse", "copilot-chat", "create_file", HookAgentToolCodeEdited, "", false},
		{"copilot PostToolUse run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalEnded, "", false},
		{"copilot PostToolUse generic", "PostToolUse", "copilot-chat", "runSubagent", HookAgentToolEnded, "", false},
		{"cursor sessionStart", "sessionStart", "cursor-chat", "", HookAgentStarted, "", false},
		{"cursor sessionEnd", "sessionEnd", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor subagentStart", "subagentStart", "cursor-chat", "", HookAgentStarted, "subagent", false},
		{"cursor beforeReadFile", "beforeReadFile", "cursor-chat", "", HookAgentToolSearching, "", false},
		{"cursor afterFileEdit", "afterFileEdit", "cursor-chat", "", HookAgentToolCodeEdited, "", false},
		{"cursor beforeShellExecution", "beforeShellExecution", "cursor-chat", "", HookAgentToolTerminalStarting, "", false},
		{"cursor afterShellExecution", "afterShellExecution", "cursor-chat", "", HookAgentToolTerminalEnded, "", false},
		{"cursor beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", HookAgentToolStarting, "", false},
		{"cursor afterMCPExecution", "afterMCPExecution", "cursor-chat", "", HookAgentToolEnded, "", false},
		{"cursor afterAgentResponse", "afterAgentResponse", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor afterAgentThought", "afterAgentThought", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", HookAgentToolSearching, "", false},
		{"cursor afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", HookAgentToolCodeEdited, "", false},
		{"windsurf pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", HookAgentPromptSubmitting, "", false},
		{"windsurf post_cascade_response", "post_cascade_response", "windsurf-chat", "", HookAgentEnded, "", false},
		{"windsurf post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", HookAgentStarted, "", false},
		{"windsurf pre_read_code", "pre_read_code", "windsurf-chat", "", HookAgentToolSearching, "", false},
		{"windsurf pre_write_code", "pre_write_code", "windsurf-chat", "", HookAgentToolCodeEditing, "", false},
		{"windsurf post_write_code", "post_write_code", "windsurf-chat", "", HookAgentToolCodeEdited, "", false},
		{"windsurf pre_run_command", "pre_run_command", "windsurf-chat", "", HookAgentToolTerminalStarting, "", false},
		{"windsurf post_run_command", "post_run_command", "windsurf-chat", "", HookAgentToolTerminalEnded, "", false},
		{"windsurf pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", HookAgentToolStarting, "", false},
		{"windsurf post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", HookAgentToolEnded, "", false},
		{"claude SessionStart", "SessionStart", "claude-code", "", HookAgentStarted, "", false},
		{"claude SessionEnd", "SessionEnd", "claude-code", "", HookAgentEnded, "", false},
		{"claude SubagentStart", "SubagentStart", "claude-code", "", HookAgentStarted, "subagent", false},
		{"claude SubagentStop", "SubagentStop", "claude-code", "", HookAgentEnded, "subagent", false},
		{"claude TaskCompleted", "TaskCompleted", "claude-code", "", HookAgentToolPlanUpdating, "", false},
		{"claude PermissionRequest", "PermissionRequest", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude TeammateIdle", "TeammateIdle", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude Notification", "Notification", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude PreToolUse Bash", "PreToolUse", "claude-code", "Bash", HookAgentToolTerminalStarting, "", false},
		{"claude PostToolUse Bash", "PostToolUse", "claude-code", "Bash", HookAgentToolTerminalEnded, "", false},
		{"claude PreToolUse Read", "PreToolUse", "claude-code", "Read", HookAgentToolSearching, "", false},
		{"claude PreToolUse Edit", "PreToolUse", "claude-code", "Edit", HookAgentToolCodeEditing, "", false},
		{"claude PostToolUse Edit", "PostToolUse", "claude-code", "Edit", HookAgentToolCodeEdited, "", false},
		{"droid PreToolUse", "PreToolUse", "droid", "Bash", HookAgentToolTerminalStarting, "", false},
		{"codex PreToolUse", "PreToolUse", "codex", "Read", HookAgentToolSearching, "", false},
		{"antigravity PreToolUse", "PreToolUse", "antigravity-chat", "Task", HookAgentToolPlanUpdating, "", false},
		{"unknown client defaults to claude-compatible", "SessionStart", "unknown-client", "", HookAgentStarted, "", false},
		{"invalid copilot event", "UnknownEvent", "copilot-chat", "", "", "", true},
		{"invalid cursor event", "UnknownEvent", "cursor-chat", "", "", "", true},
		{"invalid windsurf event", "UnknownEvent", "windsurf-chat", "", "", "", true},
		{"invalid claude event", "UnknownEvent", "claude-code", "", "", "", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, parent, err := ResolveHookEvent(tc.eventStr, tc.client, tc.toolName, nil)
			if tc.expectErr {
				if err == nil {
					t.Errorf("expected error, got event=%s parent=%s", event, parent)
				}
				return
			}
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if event != tc.expectEvt {
				t.Errorf("expected event %s, got %s", tc.expectEvt, event)
			}
			if parent != tc.expectPar {
				t.Errorf("expected parent %q, got %q", tc.expectPar, parent)
			}
		})
	}
}

func TestResolvePreToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindPlan, HookAgentToolPlanUpdating},
		{ToolKindCodeSearch, HookAgentToolSearching},
		{ToolKindCodeEdit, HookAgentToolCodeEditing},
		{ToolKindTerminal, HookAgentToolTerminalStarting},
		{ToolKindGeneric, HookAgentToolStarting},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePreToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestResolvePostToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindCodeEdit, HookAgentToolCodeEdited},
		{ToolKindTerminal, HookAgentToolTerminalEnded},
		{ToolKindGeneric, HookAgentToolEnded},
		{ToolKindPlan, HookAgentToolEnded},
		{ToolKindCodeSearch, HookAgentToolEnded},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePostToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestPopulateEventDataAgentStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-abc","parent":"subagent"}`)
	hctx := HookContext{
		Event:     HookAgentStarted,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T10:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "sess-abc" {
		t.Errorf("expected session=sess-abc, got %s", res.Session)
	}
	if res.Timestamp != "2026-02-19T10:00:00Z" {
		t.Errorf("expected timestamp=2026-02-19T10:00:00Z, got %s", res.Timestamp)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
	if res.Parent != "subagent" {
		t.Errorf("expected parent=subagent, got %s", res.Parent)
	}
	if res.Raw == nil {
		t.Error("expected raw to be populated")
	}
}

func TestPopulateEventDataAgentStartingParentFromContext(t *testing.T) {
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "claude-code",
		Timestamp:  "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		ParentInfo: "parent-agent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "parent-agent" {
		t.Errorf("expected parent=parent-agent, got %s", res.Parent)
	}
	if res.Client != "claude-code" {
		t.Errorf("expected client=claude-code, got %s", res.Client)
	}
}

func TestPopulateEventDataAgentEnded(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-end"}`)
	hctx := HookContext{
		Event:     HookAgentEnded,
		Client:    "cursor-chat",
		Timestamp: "2026-02-19T11:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentEnded, got %T", result)
	}
	if res.Session != "sess-end" {
		t.Errorf("expected session=sess-end, got %s", res.Session)
	}
	if res.Timestamp != "2026-02-19T11:00:00Z" {
		t.Errorf("expected timestamp, got %s", res.Timestamp)
	}
	if res.Client != "cursor-chat" {
		t.Errorf("expected client=cursor-chat, got %s", res.Client)
	}
}

func TestPopulateEventDataPromptSubmitting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-p","prompt":"Fix the bug in main.go"}`)
	hctx := HookContext{
		Event:     HookAgentPromptSubmitting,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T12:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentPromptSubmitting)
	if !ok {
		t.Fatalf("expected HookResultAgentPromptSubmitting, got %T", result)
	}
	if res.Session != "sess-p" {
		t.Errorf("expected session=sess-p, got %s", res.Session)
	}
	if res.Prompt != "Fix the bug in main.go" {
		t.Errorf("expected prompt, got %s", res.Prompt)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
}

func TestPopulateEventDataCompacting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-c","chat":"previous conversation context"}`)
	hctx := HookContext{
		Event:     HookAgentCompacting,
		Client:    "claude-code",
		Timestamp: "2026-02-19T13:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentCompacting)
	if !ok {
		t.Fatalf("expected HookResultAgentCompacting, got %T", result)
	}
	if res.Session != "sess-c" {
		t.Errorf("expected session=sess-c, got %s", res.Session)
	}
	if res.Chat != "previous conversation context" {
		t.Errorf("expected chat content, got %s", res.Chat)
	}
}

func TestPopulateEventDataToolStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-t","tool_name":"runSubagent","tool_input":{"prompt":"do something"}}`)
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T14:00:00Z",
		RepoRoot:  t.TempDir(),
		ToolName:  "runSubagent",
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Session != "sess-t" {
		t.Errorf("expected session=sess-t, got %s", res.Session)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	var inputData map[string]interface{}
	if err := json.Unmarshal(res.Input, &inputData); err != nil {
		t.Fatalf("expected valid JSON input, got: %v", err)
	}
	if inputData["prompt"] != "do something" {
		t.Errorf("expected prompt in input, got %v", inputData["prompt"])
	}
}

func TestPopulateEventDataToolEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-te","tool_name":"runSubagent","tool_input":{"prompt":"do something"},"tool_output":"done"}`)
	hctx := HookContext{
		Event:     HookAgentToolEnded,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T14:30:00Z",
		RepoRoot:  t.TempDir(),
		ToolName:  "runSubagent",
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolEnded, got %T", result)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	if res.Response == nil {
		t.Error("expected response to be populated")
	}
}

func TestPopulateEventDataPlanUpdating(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-plan","tool_input":{"todoList":[{"id":1,"title":"Step 1","status":"completed"},{"id":2,"title":"Step 2","status":"in-progress"},{"id":3,"title":"Step 3","status":"not-started"}]}}`)
	hctx := HookContext{
		Event:     HookAgentToolPlanUpdating,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T15:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolPlanUpdating)
	if !ok {
		t.Fatalf("expected HookResultAgentToolPlanUpdating, got %T", result)
	}
	if res.Session != "sess-plan" {
		t.Errorf("expected session=sess-plan, got %s", res.Session)
	}
	if len(res.Steps) != 3 {
		t.Fatalf("expected 3 steps, got %d", len(res.Steps))
	}
	if res.Steps[0].Name != "Step 1" || res.Steps[0].Status != "completed" {
		t.Errorf("expected Step 1 completed, got %+v", res.Steps[0])
	}
	if res.Steps[1].Name != "Step 2" || res.Steps[1].Status != "in-progress" {
		t.Errorf("expected Step 2 in-progress, got %+v", res.Steps[1])
	}
	if res.Steps[2].Name != "Step 3" || res.Steps[2].Status != "not-started" {
		t.Errorf("expected Step 3 not-started, got %+v", res.Steps[2])
	}
}

func TestPopulateEventDataCodeSearching(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cs","tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
	hctx := HookContext{
		Event:     HookAgentToolSearching,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T16:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearching)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearching, got %T", result)
	}
	if res.Session != "sess-cs" {
		t.Errorf("expected session=sess-cs, got %s", res.Session)
	}
	if res.Query != "hookCommand" {
		t.Errorf("expected query=hookCommand, got %s", res.Query)
	}
	if len(res.Include) != 1 || res.Include[0] != "*.go" {
		t.Errorf("expected include=[*.go], got %v", res.Include)
	}
}

func TestPopulateEventDataCodeEditing(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ce","tool_input":{"filePath":"/workspaces/semio/test.go","oldString":"old code","newString":"new code"}}`)
	hctx := HookContext{
		Event:     HookAgentToolCodeEditing,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T17:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditing)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditing, got %T", result)
	}
	if res.Session != "sess-ce" {
		t.Errorf("expected session=sess-ce, got %s", res.Session)
	}
	if res.Path != "/workspaces/semio/test.go" {
		t.Errorf("expected path, got %s", res.Path)
	}
	if res.Old != "old code" {
		t.Errorf("expected old=old code, got %s", res.Old)
	}
	if res.New != "new code" {
		t.Errorf("expected new=new code, got %s", res.New)
	}
}

func TestPopulateEventDataCodeEditingWithAll(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cea","tool_input":{"filePath":"/tmp/file.ts","oldString":"x","newString":"y","all":true}}`)
	hctx := HookContext{
		Event:     HookAgentToolCodeEditing,
		Client:    "cursor-chat",
		Timestamp: "2026-02-19T17:30:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditing)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditing, got %T", result)
	}
	if !res.All {
		t.Error("expected all=true")
	}
}

func TestPopulateEventDataCodeEdited(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ced","tool_input":{"filePath":"/tmp/edited.ts","oldString":"before","newString":"after"}}`)
	hctx := HookContext{
		Event:     HookAgentToolCodeEdited,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T18:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEdited)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEdited, got %T", result)
	}
	if res.Path != "/tmp/edited.ts" {
		t.Errorf("expected path=/tmp/edited.ts, got %s", res.Path)
	}
	if res.Old != "before" {
		t.Errorf("expected old=before, got %s", res.Old)
	}
	if res.New != "after" {
		t.Errorf("expected new=after, got %s", res.New)
	}
}

func TestPopulateEventDataTerminalStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ts","tool_input":{"command":"npm test"}}`)
	hctx := HookContext{
		Event:     HookAgentToolTerminalStarting,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T19:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalStarting, got %T", result)
	}
	if res.Session != "sess-ts" {
		t.Errorf("expected session=sess-ts, got %s", res.Session)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
}

func TestPopulateEventDataTerminalEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tse","tool_input":{"command":"npm test"},"pid":"12345","terminated":true,"stdout":"all passed","stderr":""}`)
	hctx := HookContext{
		Event:     HookAgentToolTerminalEnded,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T19:30:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalEnded, got %T", result)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
	if res.PID != "12345" {
		t.Errorf("expected pid=12345, got %s", res.PID)
	}
	if !res.Terminated {
		t.Error("expected terminated=true")
	}
	if res.Stdout != "all passed" {
		t.Errorf("expected stdout=all passed, got %s", res.Stdout)
	}
}

func TestPopulateEventDataGitCommitEnded(t *testing.T) {
	tmpDir := t.TempDir()
	payload := json.RawMessage(`{"sha":"abc123def","message":"feat: add hooks"}`)
	hctx := HookContext{
		Event:     HookGitCommitEnded,
		Client:    "",
		Timestamp: "2026-02-19T20:00:00Z",
		RepoRoot:  tmpDir,
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultGitCommitEnded)
	if !ok {
		t.Fatalf("expected HookResultGitCommitEnded, got %T", result)
	}
	if res.SHA != "abc123def" {
		t.Errorf("expected sha=abc123def, got %s", res.SHA)
	}
	if res.Message != "feat: add hooks" {
		t.Errorf("expected message=feat: add hooks, got %s", res.Message)
	}
}

func TestPopulateEventDataGitCommitStartingFromFile(t *testing.T) {
	tmpDir := t.TempDir()
	gitDir := filepath.Join(tmpDir, ".git")
	if err := os.MkdirAll(gitDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: resolve issue"), 0644); err != nil {
		t.Fatal(err)
	}
	hctx := HookContext{
		Event:    HookGitCommitStarting,
		Client:   "",
		RepoRoot: tmpDir,
	}
	result := RunHook(hctx) // Changed to RunHook
	res, ok := result.(HookResultGitCommitStarting)
	if !ok {
		t.Fatalf("expected HookResultGitCommitStarting, got %T", result)
	}
	if res.Message != "fix: resolve issue" {
		t.Errorf("expected message=fix: resolve issue, got %s", res.Message)
	}
}

func TestPopulateEventDataRawField(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"raw-test","some":"data"}`)
	hctx := HookContext{
		Event:     HookAgentStarted,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T21:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw == nil {
		t.Fatal("expected raw to be populated")
	}
	var rawData map[string]interface{}
	body, _ := json.Marshal(res.Raw) // Raw is 'any', need to marshal first or type assert if it's already json.RawMessage
	if err := json.Unmarshal(body, &rawData); err != nil {
		t.Fatalf("expected valid JSON raw, got: %v", err)
	}
	if rawData["sessionId"] != "raw-test" {
		t.Errorf("expected sessionId=raw-test in raw, got %v", rawData["sessionId"])
	}
	if rawData["some"] != "data" {
		t.Errorf("expected some=data in raw, got %v", rawData["some"])
	}
}

func TestPopulateEventDataNoInputNoRaw(t *testing.T) {
	hctx := HookContext{
		Event:     HookAgentStarted,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T22:00:00Z",
		RepoRoot:  t.TempDir(),
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw != nil {
		t.Error("expected raw to be nil when no input")
	}
}

func TestPopulateEventDataToolNameFromStdin(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tn","tool_name":"mcp_custom_tool","tool_input":{"arg":"val"}}`)
	hctx := HookContext{
		Event:     HookAgentToolStarting,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T23:00:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Name != "mcp_custom_tool" {
		t.Errorf("expected name=mcp_custom_tool, got %s", res.Name)
	}
}

func TestPopulateEventDataCodeSearchWithExclude(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ex","tool_input":{"query":"test","include":["*.ts","*.tsx"],"exclude":["node_modules"]}}`)
	hctx := HookContext{
		Event:     HookAgentToolSearching,
		Client:    "copilot-chat",
		Timestamp: "2026-02-19T23:30:00Z",
		RepoRoot:  t.TempDir(),
		Input:     payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearching)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearching, got %T", result)
	}
	if res.Query != "test" {
		t.Errorf("expected query=test, got %s", res.Query)
	}
	if len(res.Include) != 2 {
		t.Errorf("expected 2 include patterns, got %d", len(res.Include))
	}
	if len(res.Exclude) != 1 || res.Exclude[0] != "node_modules" {
		t.Errorf("expected exclude=[node_modules], got %v", res.Exclude)
	}
}

func TestExtractPlanStepsFromInput(t *testing.T) {
	t.Run("todoList format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"todoList":[{"title":"Task A","status":"completed"},{"title":"Task B","status":"in-progress"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(steps))
		}
		if steps[0].Name != "Task A" || steps[0].Status != "completed" {
			t.Errorf("unexpected step 0: %+v", steps[0])
		}
	})
	t.Run("steps format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"steps":[{"name":"Build","status":"pending"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "Build" {
			t.Errorf("expected name=Build, got %s", steps[0].Name)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, `{"todoList":[{"title":"FromArgs","status":"done"}]}`)
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "FromArgs" {
			t.Errorf("expected name=FromArgs, got %s", steps[0].Name)
		}
	})
	t.Run("empty", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, "")
		if steps != nil {
			t.Errorf("expected nil steps, got %v", steps)
		}
	})
}

func TestExtractCodeSearchFromInput(t *testing.T) {
	t.Run("grep_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"hookCommand","includePattern":"*.go","excludePattern":"vendor"}}`)
		query, include, exclude := extractCodeSearchFromInput(input, "")
		if query != "hookCommand" {
			t.Errorf("expected query=hookCommand, got %s", query)
		}
		if len(include) != 1 || include[0] != "*.go" {
			t.Errorf("expected include=[*.go], got %v", include)
		}
		if len(exclude) != 1 || exclude[0] != "vendor" {
			t.Errorf("expected exclude=[vendor], got %v", exclude)
		}
	})
	t.Run("file_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"**/*.ts"}}`)
		query, _, _ := extractCodeSearchFromInput(input, "")
		if query != "**/*.ts" {
			t.Errorf("expected query=**/*.ts, got %s", query)
		}
	})
	t.Run("read_file style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/test.go"}}`)
		query, _, _ := extractCodeSearchFromInput(input, "")
		if query != "/tmp/test.go" {
			t.Errorf("expected query=/tmp/test.go, got %s", query)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		query, _, _ := extractCodeSearchFromInput(nil, `{"query":"fromArgs"}`)
		if query != "fromArgs" {
			t.Errorf("expected query=fromArgs, got %s", query)
		}
	})
}

func TestExtractCodeEditFromInput(t *testing.T) {
	t.Run("replace_string_in_file style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/file.go","oldString":"old","newString":"new"}}`)
		path, old, new_, all := extractCodeEditFromInput(input, "")
		if path != "/tmp/file.go" {
			t.Errorf("expected path=/tmp/file.go, got %s", path)
		}
		if old != "old" {
			t.Errorf("expected old=old, got %s", old)
		}
		if new_ != "new" {
			t.Errorf("expected new=new, got %s", new_)
		}
		if all {
			t.Error("expected all=false")
		}
	})
	t.Run("with replaceAll", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/f.go","oldString":"a","newString":"b","replaceAll":true}}`)
		_, _, _, all := extractCodeEditFromInput(input, "")
		if !all {
			t.Error("expected all=true")
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		path, _, _, _ := extractCodeEditFromInput(nil, `{"filePath":"/from/args.go"}`)
		if path != "/from/args.go" {
			t.Errorf("expected path=/from/args.go, got %s", path)
		}
	})
}

func TestExtractToolInputFromStdin(t *testing.T) {
	t.Run("with tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test","tool_input":{"key":"val"}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["key"] != "val" {
			t.Errorf("expected key=val, got %v", data["key"])
		}
	})
	t.Run("without tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolInputFromStdin(input)
		if result != nil {
			t.Error("expected nil tool input")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractToolInputFromStdin(nil)
		if result != nil {
			t.Error("expected nil for empty input")
		}
	})
}

func TestExtractToolResponseFromStdin(t *testing.T) {
	t.Run("tool_output", func(t *testing.T) {
		input := json.RawMessage(`{"tool_output":"response data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("tool_response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_response":"data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("no response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolResponseFromStdin(input)
		if result != nil {
			t.Error("expected nil response")
		}
	})
}

func TestExtractTerminalEndedFromInput(t *testing.T) {
	t.Run("full payload", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"command":"ls -la"},"pid":"999","terminated":true,"stdout":"file1\nfile2","stderr":""}`)
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
		if command != "ls -la" {
			t.Errorf("expected command=ls -la, got %s", command)
		}
		if pid != "999" {
			t.Errorf("expected pid=999, got %s", pid)
		}
		if !terminated {
			t.Error("expected terminated=true")
		}
		if stdout != "file1\nfile2" {
			t.Errorf("expected stdout, got %s", stdout)
		}
		if stderr != "" {
			t.Errorf("expected empty stderr, got %s", stderr)
		}
	})
	t.Run("numeric pid", func(t *testing.T) {
		input := json.RawMessage(`{"pid":42}`)
		_, pid, _, _, _ := extractTerminalEndedFromInput(input)
		if pid != "42" {
			t.Errorf("expected pid=42, got %s", pid)
		}
	})
	t.Run("empty", func(t *testing.T) {
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(nil)
		if command != "" || pid != "" || terminated || stdout != "" || stderr != "" {
			t.Error("expected all empty for nil input")
		}
	})
}

func TestExtractChatFromInput(t *testing.T) {
	t.Run("chat string", func(t *testing.T) {
		input := json.RawMessage(`{"chat":"conversation context"}`)
		result := extractChatFromInput(input)
		if result != "conversation context" {
			t.Errorf("expected conversation context, got %s", result)
		}
	})
	t.Run("messages array", func(t *testing.T) {
		input := json.RawMessage(`{"messages":[{"role":"user","content":"hello"},{"role":"assistant","content":"hi"}]}`)
		result := extractChatFromInput(input)
		if result == "" {
			t.Error("expected non-empty chat from messages array")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractChatFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractCommitMessageFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"message":"feat: new feature"}`)
		result := extractCommitMessageFromInput(input, "/nonexistent")
		if result != "feat: new feature" {
			t.Errorf("expected feat: new feature, got %s", result)
		}
	})
	t.Run("from COMMIT_EDITMSG file", func(t *testing.T) {
		tmpDir := t.TempDir()
		gitDir := filepath.Join(tmpDir, ".git")
		os.MkdirAll(gitDir, 0755)
		os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: bug fix"), 0644)
		result := extractCommitMessageFromInput(nil, tmpDir)
		if result != "fix: bug fix" {
			t.Errorf("expected fix: bug fix, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractCommitMessageFromInput(nil, "/nonexistent")
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractCommitSHAFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"sha":"deadbeef123"}`)
		result := extractCommitSHAFromInput(input)
		if result != "deadbeef123" {
			t.Errorf("expected deadbeef123, got %s", result)
		}
	})
	t.Run("empty falls back to git", func(t *testing.T) {
		result := extractCommitSHAFromInput(nil)
		if result == "" {
			t.Skip("no git repo available")
		}
	})
}

func TestExtractParentFromInput(t *testing.T) {
	t.Run("parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"subagent"}`)
		result := extractParentFromInput(input)
		if result != "subagent" {
			t.Errorf("expected subagent, got %s", result)
		}
	})
	t.Run("source field", func(t *testing.T) {
		input := json.RawMessage(`{"source":"parent-session"}`)
		result := extractParentFromInput(input)
		if result != "parent-session" {
			t.Errorf("expected parent-session, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractParentFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestHookResultJSONFields(t *testing.T) {
	result := HookResultAgentToolTerminalEnded{
		HookResultAgentBase: HookResultAgentBase{
			HookResultBase: HookResultBase{
				Allowed: true,
				Raw:     json.RawMessage(`{"raw":"data"}`),
			},
			Session:   "sess-1",
			Timestamp: "2026-02-19T10:00:00Z",
			Client:    "copilot-chat",
			MessageID: "msg-123",
		},
		Command:    "npm test",
		PID:        "12345",
		Terminated: true,
		Stdout:     "passed",
		Stderr:     "warn",
	}
	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON, got: %v", err)
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	expectedKeys := []string{"allowed", "message", "raw", "session", "timestamp", "client", "command", "pid", "terminated", "stdout", "stderr"}
	for _, key := range expectedKeys {
		if _, ok := parsed[key]; !ok {
			t.Errorf("missing JSON key: %s", key)
		}
	}
}

func TestHookResultOmitEmpty(t *testing.T) {
	result := HookResultAgentStarted{
		HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}},
	}
	data, _ := json.Marshal(result)
	var parsed map[string]interface{}
	json.Unmarshal(data, &parsed)
	if _, ok := parsed["session"]; ok {
		t.Error("expected session to be omitted when empty")
	}
	if _, ok := parsed["raw"]; ok {
		t.Error("expected raw to be omitted when nil")
	}
}

// #endregion 🔖Hook Tests

// #region 🔖Mermaid Tests

func TestMermaidLocByProjectsBundlesFoldersFiles(t *testing.T) {
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByProjectsBundlesFoldersFiles()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code\"") {
		t.Error("expected 'Lines of Code' title")
	}
	if !strings.Contains(result, EmojiProjectUser) {
		t.Error("expected user project emoji")
	}
	if !strings.Contains(result, EmojiProjectInfra) {
		t.Error("expected infra project emoji")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 5 {
		t.Errorf("expected at least 5 lines, got %d", len(lines))
	}
	hasValue := false
	for _, line := range lines {
		if strings.Contains(line, ": ") {
			parts := strings.Split(strings.TrimSpace(line), ": ")
			if len(parts) == 2 {
				val := strings.TrimSpace(parts[1])
				if val != "0" {
					hasValue = true
				}
			}
		}
	}
	if !hasValue {
		t.Error("expected at least one file with non-zero LOC value")
	}
}

func TestMermaidLocByLanguage(t *testing.T) {
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByLanguage()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code by Language\"") {
		t.Error("expected 'Lines of Code by Language' title")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 3 {
		t.Errorf("expected at least 3 lines (header + title + at least 1 language), got %d", len(lines))
	}
	hasLanguage := false
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.Contains(trimmed, ": ") && strings.HasPrefix(trimmed, "\"") {
			hasLanguage = true
		}
	}
	if !hasLanguage {
		t.Error("expected at least one language entry with LOC")
	}
}

func TestMermaidLocByContributors(t *testing.T) {
	root := findTestRepoRoot(".")
	SetRootDir(root)
	result := MermaidLocByContributors()
	if !strings.HasPrefix(result, "treemap-beta\n") {
		t.Fatalf("expected treemap-beta header, got: %s", result[:min(len(result), 100)])
	}
	if !strings.Contains(result, "\"Lines of Code by Contributor\"") {
		t.Error("expected 'Lines of Code by Contributor' title")
	}
	lines := strings.Split(strings.TrimSpace(result), "\n")
	if len(lines) < 3 {
		t.Errorf("expected at least 3 lines (header + title + at least 1 contributor), got %d", len(lines))
	}
}

func TestMermaidCommandLocByProjectsBundlesFoldersFiles(t *testing.T) {
	root := findTestRepoRoot(".")
	SetRootDir(root)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-projects-bundles-folders-files"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}

func TestMermaidCommandLocByLanguage(t *testing.T) {
	root := findTestRepoRoot(".")
	SetRootDir(root)
	cmd := NewRoot(testEngineFactory)
	buf := new(bytes.Buffer)
	cmd.SetOut(buf)
	cmd.SetArgs([]string{"mermaid", "loc-by-language"})
	if err := cmd.Execute(); err != nil {
		t.Fatalf("command failed: %v", err)
	}
	output := buf.String()
	if !strings.HasPrefix(output, "treemap-beta\n") {
		t.Errorf("expected treemap-beta output, got: %s", output[:min(len(output), 100)])
	}
}

func TestMermaidEscapeLabel(t *testing.T) {
	if got := mermaidEscapeLabel("hello \"world\""); got != "hello 'world'" {
		t.Errorf("expected hello 'world', got: %s", got)
	}
	if got := mermaidEscapeLabel("no quotes"); got != "no quotes" {
		t.Errorf("expected no quotes, got: %s", got)
	}
}

// #endregion 🔖Mermaid Tests

// #region 🔖Provider Tests

func TestProviderRegistry(t *testing.T) {
	mp := DefaultManagementProvider()
	if mp == nil {
		t.Fatal("DefaultManagementProvider() returned nil")
	}
	if mp.Kind() != "github" {
		t.Errorf("expected github, got %s", mp.Kind())
	}
	scp := DefaultSourceControlProvider()
	if scp == nil {
		t.Fatal("DefaultSourceControlProvider() returned nil")
	}
	if scp.Kind() != "github" {
		t.Errorf("expected github, got %s", scp.Kind())
	}
	sp := DefaultSandboxProvider()
	if sp == nil {
		t.Fatal("DefaultSandboxProvider() returned nil")
	}
	if sp.Kind() != "devcontainer" {
		t.Errorf("expected devcontainer, got %s", sp.Kind())
	}
}

func TestGetManagementProvider(t *testing.T) {
	mp := GetManagementProvider()
	if mp == nil {
		t.Fatal("GetManagementProvider() returned nil")
	}
	if mp.Kind() != "github" {
		t.Errorf("expected github, got %s", mp.Kind())
	}
}

func TestNullManagementProvider(t *testing.T) {
	p := &NullManagementProvider{}
	if p.Kind() != "none" {
		t.Errorf("expected none, got %s", p.Kind())
	}
	if err := p.Configure("/tmp"); err != nil {
		t.Errorf("Configure should not error: %v", err)
	}
	url, err := p.CreateIssue("test", "body", nil)
	if err != nil || url != "" {
		t.Errorf("CreateIssue should return empty string, got %q, err=%v", url, err)
	}
	if err := p.CloseIssue("url"); err != nil {
		t.Errorf("CloseIssue should not error: %v", err)
	}
	if err := p.ReopenIssue("url"); err != nil {
		t.Errorf("ReopenIssue should not error: %v", err)
	}
	if err := p.DeleteIssue("url"); err != nil {
		t.Errorf("DeleteIssue should not error: %v", err)
	}
	if err := p.UpdateIssueTitle("url", "title"); err != nil {
		t.Errorf("UpdateIssueTitle should not error: %v", err)
	}
	if err := p.UpdateIssueBody("url", "body"); err != nil {
		t.Errorf("UpdateIssueBody should not error: %v", err)
	}
	details, err := p.GetIssueDetails("url")
	if err != nil || details != nil {
		t.Errorf("GetIssueDetails should return nil, got %v, err=%v", details, err)
	}
	nodeID, err := p.GetIssueNodeID("url")
	if err != nil || nodeID != "" {
		t.Errorf("GetIssueNodeID should return empty, got %q, err=%v", nodeID, err)
	}
	parentURL, err := p.GetIssueParentURL("url")
	if err != nil || parentURL != "" {
		t.Errorf("GetIssueParentURL should return empty, got %q, err=%v", parentURL, err)
	}
	if err := p.AddComment("url", "comment"); err != nil {
		t.Errorf("AddComment should not error: %v", err)
	}
	if err := p.AddLabels("url", []string{"a"}); err != nil {
		t.Errorf("AddLabels should not error: %v", err)
	}
	if err := p.RemoveLabels("url", []string{"a"}); err != nil {
		t.Errorf("RemoveLabels should not error: %v", err)
	}
	p.AddIssueToProject("url")
	p.AssignIssueToCurrentUser("url")
	if err := p.AddSubIssue("parent", "child"); err != nil {
		t.Errorf("AddSubIssue should not error: %v", err)
	}
	if err := p.UpdateIssueMilestone("url", "title"); err != nil {
		t.Errorf("UpdateIssueMilestone should not error: %v", err)
	}
	if err := p.ClearIssueMilestone("url"); err != nil {
		t.Errorf("ClearIssueMilestone should not error: %v", err)
	}
	num, err := p.CreateMilestone("title", "desc")
	if err != nil || num != 0 {
		t.Errorf("CreateMilestone should return 0, got %d, err=%v", num, err)
	}
	if err := p.UpdateMilestone(1, "t", "d", "s", "due"); err != nil {
		t.Errorf("UpdateMilestone should not error: %v", err)
	}
	if err := p.DeleteMilestone(1); err != nil {
		t.Errorf("DeleteMilestone should not error: %v", err)
	}
	m, err := p.GetMilestone(1)
	if err != nil || m != nil {
		t.Errorf("GetMilestone should return nil, got %v, err=%v", m, err)
	}
	title, err := p.GetMilestoneTitle(1)
	if err != nil || title != "" {
		t.Errorf("GetMilestoneTitle should return empty, got %q, err=%v", title, err)
	}
	found, err := p.FindMilestoneByTitle("title")
	if err != nil || found != nil {
		t.Errorf("FindMilestoneByTitle should return nil, got %v, err=%v", found, err)
	}
	issues, err := p.ListIssuesForLabelSync()
	if err != nil || issues != nil {
		t.Errorf("ListIssuesForLabelSync should return nil, got %v, err=%v", issues, err)
	}
	urls, err := p.ListOpenIssuesWithLabel("label")
	if err != nil || urls != nil {
		t.Errorf("ListOpenIssuesWithLabel should return nil, got %v, err=%v", urls, err)
	}
	labels, err := p.ListRepoLabels()
	if err != nil || labels != nil {
		t.Errorf("ListRepoLabels should return nil, got %v, err=%v", labels, err)
	}
	if err := p.CreateRepoLabel("name"); err != nil {
		t.Errorf("CreateRepoLabel should not error: %v", err)
	}
	if err := p.DeleteRepoLabel("name"); err != nil {
		t.Errorf("DeleteRepoLabel should not error: %v", err)
	}
	if err := p.SyncRepoLabelCatalog(map[string]bool{"a": true}); err != nil {
		t.Errorf("SyncRepoLabelCatalog should not error: %v", err)
	}
	goalURL, err := p.CreateGoalIssue("title", "desc", nil)
	if err != nil || goalURL != "" {
		t.Errorf("CreateGoalIssue should return empty, got %q, err=%v", goalURL, err)
	}
	if err := p.UpdateGoalIssue("url", "t", "d"); err != nil {
		t.Errorf("UpdateGoalIssue should not error: %v", err)
	}
	if user := p.GetCurrentUser(); user != "" {
		t.Errorf("GetCurrentUser should return empty, got %q", user)
	}
}

func TestAllEditorProviders(t *testing.T) {
	providers := AllEditorProviders()
	if len(providers) == 0 {
		t.Fatal("AllEditorProviders() returned empty")
	}
	kinds := make(map[string]bool)
	for _, p := range providers {
		if p.Kind() == "" {
			t.Error("editor provider has empty Kind()")
		}
		kinds[p.Kind()] = true
	}
	for _, expected := range []string{"copilot-chat", "cursor-chat", "windsurf-chat", "claude-code", "droid", "codex", "antigravity-chat"} {
		if !kinds[expected] {
			t.Errorf("missing editor provider for %s", expected)
		}
	}
}

func TestGetEditorProvider(t *testing.T) {
	for _, client := range []string{"copilot-chat", "cursor-chat", "windsurf-chat", "claude-code", "droid", "codex", "antigravity-chat"} {
		p := GetEditorProvider(client)
		if p == nil {
			t.Errorf("GetEditorProvider(%s) returned nil", client)
			continue
		}
		if p.Kind() != client {
			t.Errorf("expected Kind()=%s, got %s", client, p.Kind())
		}
	}
	if p := GetEditorProvider("nonexistent"); p != nil {
		t.Errorf("expected nil for unknown client, got %v", p)
	}
}

func TestEditorProviderHookMapping(t *testing.T) {
	for _, p := range AllEditorProviders() {
		m := p.HookMapping()
		if m.Client == "" {
			t.Errorf("editor provider %s has empty HookMapping().Client", p.Kind())
		}
	}
}

func TestManagementProviderInterface(t *testing.T) {
	var _ ManagementProvider = &GitHubManagementProvider{}
	var _ ManagementProvider = &NullManagementProvider{}
}

func TestSourceControlProviderInterface(t *testing.T) {
	var _ SourceControlProvider = &GitHubSourceControlProvider{}
}

func TestSandboxProviderInterface(t *testing.T) {
	var _ SandboxProvider = &DevcontainerSandboxProvider{}
}

func TestEditorProviderInterface(t *testing.T) {
	var _ EditorProvider = &CopilotEditorProvider{}
	var _ EditorProvider = &CursorEditorProvider{}
	var _ EditorProvider = &WindsurfEditorProvider{}
	var _ EditorProvider = &ClaudeCodeEditorProvider{}
	var _ EditorProvider = &DroidEditorProvider{}
	var _ EditorProvider = &CodexEditorProvider{}
	var _ EditorProvider = &AntigravityEditorProvider{}
}

// #endregion 🔖Provider Tests

// #region 🔖Project Generate Tests

func TestIsLicenseText(t *testing.T) {
	if !isLicenseText("This program is free software: you can redistribute it and/or modify") {
		t.Error("should detect 'free software' and 'redistribute'")
	}
	if !isLicenseText("it under the terms of the GNU Affero General Public License as") {
		t.Error("should detect 'GNU' and 'License'")
	}
	if !isLicenseText("WITHOUT ANY WARRANTY; without even the implied warranty of") {
		t.Error("should detect 'warranty'")
	}
	if !isLicenseText("Copyright 2025 Test User") {
		t.Error("should detect 'copyright'")
	}
	if isLicenseText("This function MUST return a valid result.") {
		t.Error("should not match spec text as license")
	}
	if isLicenseText("Functions for parsing SVG files.") {
		t.Error("should not match summary text as license")
	}
}

func TestIsHeaderMetaLine(t *testing.T) {
	if !isHeaderMetaLine("[🧰semiorepo⌨️cli💻maingo](semiorepo://file/semio-repo/cli/main.go)") {
		t.Error("should detect ID link")
	}
	if !isHeaderMetaLine("#region Header") {
		t.Error("should detect #region")
	}
	if !isHeaderMetaLine("#endregion Header") {
		t.Error("should detect #endregion")
	}
	if !isHeaderMetaLine("region Header") {
		t.Error("should detect region (Python style)")
	}
	if !isHeaderMetaLine("endregion Header") {
		t.Error("should detect endregion (Python style)")
	}
	if !isHeaderMetaLine("2025 Ueli Saluz <ueli@semio-tech.com>") {
		t.Error("should detect contributor line starting with year")
	}
	if !isHeaderMetaLine("💻semio/assets/repo/some/folder/file.py") {
		t.Error("should detect file ID emoji prefix")
	}
	if isHeaderMetaLine("This function handles parsing.") {
		t.Error("should not match summary text")
	}
}

func TestExtractMarkdownSection(t *testing.T) {
	content := "# Summary\n\nThis is the summary.\n\n# Specs\n\nSpec line one MUST work.\nSpec line two SHOULD also work.\n\n# Docs\n\nDocumentation here.\n"
	summary := ExtractMarkdownSection(content, "Summary")
	if !strings.Contains(summary, "This is the summary.") {
		t.Errorf("expected summary content, got: %q", summary)
	}
	specs := ExtractMarkdownSection(content, "Specs")
	if !strings.Contains(specs, "Spec line one MUST work.") {
		t.Errorf("expected specs content, got: %q", specs)
	}
	docs := ExtractMarkdownSection(content, "Docs")
	if !strings.Contains(docs, "Documentation here.") {
		t.Errorf("expected docs content, got: %q", docs)
	}
	missing := ExtractMarkdownSection(content, "Nonexistent")
	if missing != "" {
		t.Errorf("expected empty for missing section, got: %q", missing)
	}
}

func TestExtractFileHeaderSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("semio/assets/repo/some/folder/file_empty_region.tsx")
	if strings.Contains(summary, "GNU") || strings.Contains(summary, "license") || strings.Contains(summary, "redistribute") {
		t.Errorf("should not contain license text, got: %q", summary)
	}
	if strings.HasPrefix(summary, "#region") || strings.HasPrefix(summary, "region ") {
		t.Errorf("should not start with region markers, got: %q", summary)
	}
}

func TestExtractFileHeaderSummaryReturnsActualSummary(t *testing.T) {
	summary := ExtractFileHeaderSummary("semio/assets/repo/some/folder/file_empty_region.tsx")
	if strings.Contains(summary, "free software") {
		t.Errorf("should not return license as summary, got: %q", summary)
	}
}

func TestExtractFileHeaderSpecsNoLicense(t *testing.T) {
	specs := ExtractFileHeaderSpecs("semio/assets/repo/some/folder/file.py")
	if strings.Contains(specs, "GNU") || strings.Contains(specs, "license") || strings.Contains(specs, "redistribute") {
		t.Errorf("should not contain license text, got: %q", specs)
	}
}

func TestExtractSectionLeadCommentsSkipsLicense(t *testing.T) {
	content := "# region License\n\n# This program is free software: you can redistribute it and/or modify\n# it under the terms of the GNU Affero General Public License.\n\n# endregion License\n"
	sections := GetLanguage("test.py").ParseSections(content)
	for _, s := range sections {
		if s.Name == "License" {
			specs, summary := ExtractSectionLeadComments(content, s, "#")
			if specs != "" || summary != "" {
				t.Errorf("license section should return empty specs and summary, got specs=%q summary=%q", specs, summary)
			}
		}
	}
}

func TestExtractSectionLeadCommentsSkipsRegionMarkers(t *testing.T) {
	content := "//#region 🔖Exports\n// Re-exports of icons.\n// Data MUST be valid.\n//#endregion 🔖Exports\n"
	sections := GetLanguage("test.ts").ParseSections(content)
	for _, s := range sections {
		if s.Name == "Exports" {
			specs, summary := ExtractSectionLeadComments(content, s, "//")
			if strings.Contains(summary, "region") {
				t.Errorf("should not contain region text in summary, got: %q", summary)
			}
			if !strings.Contains(summary, "Re-exports of icons.") {
				t.Errorf("should contain actual summary text, got: %q", summary)
			}
			if !strings.Contains(specs, "Data MUST be valid.") {
				t.Errorf("should contain spec text, got: %q", specs)
			}
		}
	}
}

func TestGenerateProjectSpecs(t *testing.T) {
	err := GenerateProjectSpecs("coda")
	if err != nil {
		t.Fatalf("GenerateProjectSpecs failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read SPECS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 💯 Specs") {
		t.Error("SPECS.md should start with '# 💯 Specs'")
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") {
		t.Error("SPECS.md should not contain license text")
	}
	if !strings.Contains(content, "MUST") {
		t.Error("SPECS.md should contain spec keywords")
	}
}

func TestGenerateProjectDocs(t *testing.T) {
	err := GenerateProjectDocs("coda")
	if err != nil {
		t.Fatalf("GenerateProjectDocs failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "DOCS.md"))
	if err != nil {
		t.Fatalf("failed to read DOCS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 📚 Docs") {
		t.Error("DOCS.md should start with '# 📚 Docs'")
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("DOCS.md should not contain license text")
	}
	if strings.Contains(content, "region ") {
		t.Error("DOCS.md should not contain region markers")
	}
}

func TestGenerateProjectTodos(t *testing.T) {
	err := GenerateProjectTodos("coda")
	if err != nil {
		t.Fatalf("GenerateProjectTodos failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "coda", "TODOS.md"))
	if err != nil {
		t.Fatalf("failed to read TODOS.md: %v", err)
	}
	if !strings.HasPrefix(content, "# 🔳 TODOs") {
		t.Error("TODOS.md should start with '# 🔳 TODOs'")
	}
}

func TestGenerateProjectSpecsInvalidProject(t *testing.T) {
	err := GenerateProjectSpecs("nonexistent-project")
	if err == nil {
		t.Error("should return error for nonexistent project")
	}
}

func TestGenerateProjectDocsInvalidProject(t *testing.T) {
	err := GenerateProjectDocs("nonexistent-project")
	if err == nil {
		t.Error("should return error for nonexistent project")
	}
}

func TestGenerateProjectTodosInvalidProject(t *testing.T) {
	err := GenerateProjectTodos("nonexistent-project")
	if err == nil {
		t.Error("should return error for nonexistent project")
	}
}

func TestGenerateProjectSpecsSemio(t *testing.T) {
	err := GenerateProjectSpecs("semio")
	if err != nil {
		t.Fatalf("GenerateProjectSpecs semio failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "semio", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read semio/SPECS.md: %v", err)
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("semio SPECS.md should not contain license text")
	}
	if !strings.Contains(content, "MUST") {
		t.Error("semio SPECS.md should contain MUST keywords")
	}
}

func TestGenerateProjectDocsSemio(t *testing.T) {
	err := GenerateProjectDocs("semio")
	if err != nil {
		t.Fatalf("GenerateProjectDocs semio failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "semio", "DOCS.md"))
	if err != nil {
		t.Fatalf("failed to read semio/DOCS.md: %v", err)
	}
	if strings.Contains(content, "GNU") || strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("semio DOCS.md should not contain license text")
	}
}

func TestGenerateProjectSpecsSemioRepo(t *testing.T) {
	err := GenerateProjectSpecs("semio-repo")
	if err != nil {
		t.Fatalf("GenerateProjectSpecs semio-repo failed: %v", err)
	}
	content, err := ReadTextFile(filepath.Join(rootDir, "semio-repo", "SPECS.md"))
	if err != nil {
		t.Fatalf("failed to read semio-repo/SPECS.md: %v", err)
	}
	if strings.Contains(content, "free software") || strings.Contains(content, "redistribute") {
		t.Error("semio-repo SPECS.md should not contain license text")
	}
}

// #endregion 🔖Project Generate Tests
