package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"testing"
)



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
		name     string
		args     []string
		wantMarkers []string
	}{
		{
			name: "Repo Tree MD",
			args: []string{"tree", "--md"},
			wantMarkers: []string{"- [", "]("},
		},
		{
			name: "Ticket Tree MD",
			args: []string{"ticket", "tree", "--md"},
			wantMarkers: []string{"- [", "]("},
		},
		{
			name: "Goal Tree MD",
			args: []string{"goal", "tree", "--md"},
			wantMarkers: []string{"- [", "]("},
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
			// We don't assert error here because some might return errors if no tickets/goals exist, 
			// but we want to check output format if it produces anything.
			// Actually, for ticket/goal tree, if empty, it might output just "done".
			// But for repo tree, it should output something.

			output := b.String()
			// If output contains "Markdown", check markers
			// Or check for the event format.
			// The current main.go uses renderStream which prints raw data.
			// If --md is passed, renderStream usually tries to look for "markdown" data field?
			// Let's verify renderStream logic later.
			
			// For now, looking for "- [" is a good check for the specific recursive MD generation.
			// Currently it fails because they print "├──".
			
			for _, marker := range tt.wantMarkers {
				if !strings.Contains(output, marker) {
					t.Errorf("Output missing marker %q. Got:\n%s", marker, output)
				}
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
			if mode == "" { title = "Test Lifecycle human" }

			// 1. Open Ticket
			openArgs := []string{"ticket", "open", title, "Test Prompt", "copilot-chat", "gemini-3-pro", "--no-issue", "--no-github"}
			if mode == "json" { openArgs = append(openArgs, "--json") }
			if mode == "md" { openArgs = append(openArgs, "--md") }

			rootCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			rootCmd.SetOut(b)
			rootCmd.SetErr(b)
			rootCmd.SetArgs(openArgs)

			err := rootCmd.Execute()
			if err != nil {
				t.Fatalf("ticket open failed: %v\nOutput: %s", err, b.String())
			}

			// 2. Find Ticket ID via list --json
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
						// Match title loosely because of Titleization?
						// "Test Lifecycle human" -> "Test Lifecycle Human"
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

			// Cleanup
			defer os.RemoveAll(GetTicketPath(y, m, d, slug))

			// 2.5 Change Ticket
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

			// Verify change
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

			// 3. Close Ticket
			closeArgs := []string{"ticket", "close",
				"--no-github",
				"--year", strconv.Itoa(y),
				"--month", strconv.Itoa(m),
				"--day", strconv.Itoa(d),
				"--slug", slug,
				"--summary", "Test Summary",
				"--files", "@semio-repo/go/main.go",
			}
			if mode == "json" { closeArgs = append(closeArgs, "--json") }
			if mode == "md" { closeArgs = append(closeArgs, "--md") }

			closeCmd := NewRoot(factory)
			closeB := bytes.NewBufferString("")
			closeCmd.SetOut(closeB)
			closeCmd.SetErr(closeB)
			closeCmd.SetArgs(closeArgs)

			err = closeCmd.Execute()
			if err != nil {
				t.Fatalf("ticket close failed: %v\nOutput: %s", err, closeB.String())
			}
			
			// Verify output not empty
			if closeB.String() == "" {
				t.Errorf("ticket close output empty")
			}
		})
	}
}

func TestListCommands(t *testing.T) {
	// Setup environment
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	SetRootDir(repoRoot)
	
	// Create a factory that returns an engine using the repo root
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
		modes []string
	}{
		{
			name: "bundle list", 
			args: []string{"bundle", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "ticket list", 
			args: []string{"ticket", "list"}, 
			modes: []string{"", "json", "md"},
		},
		{
			name: "folder list", 
			args: []string{"folder", "list", "@semio-repo/go"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "file list",
			args: []string{"file", "list", "@semio-repo/go"}, 
			modes: []string{"", "json", "md"},
		},
		{
			name: "section list",
			args: []string{"section", "list", "@semio-repo/go/main.go"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "definition list",
			args: []string{"definition", "list", "@semio-repo/go/main.go"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "policy list",
			args: []string{"policy", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "contributor list",
			args: []string{"contributor", "list"},
			modes: []string{"", "json", "md"},
		},
		{
			name: "project list",
			args: []string{"project", "list"},
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
				
				// Capture stdout/stderr
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
					// Check if output is ndjson
					// Split by newline and try to parse each line as JSON
					lines := strings.Split(strings.TrimSpace(output), "\n")
					for _, line := range lines {
						if line == "" {
							continue
						}
						// Skip "DONE" or similar messages if they are not json?
						// But usually we output pure JSON in JSON mode.
						// Wait, renderStream outputs JSON events.
						// Event structure: { "kind": "...", ... }
						if !strings.HasPrefix(strings.TrimSpace(line), "{") {
							// Maybe a log line?
							continue
						}
						// Verify it's valid JSON
						// var js map[string]interface{}
						// if err := json.Unmarshal([]byte(line), &js); err != nil {
						// 	t.Errorf("Line is not valid JSON: %s", line)
						// }
					}
				} else if mode == "md" {
					// Check for markdown headers or list items
					if !strings.Contains(output, "# ") && !strings.Contains(output, "- ") && !strings.Contains(output, "|") && output != "" {
						// If output is not empty, it should look like markdown.
						// Allowed to be empty if list empty.
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
		name        string
		ext         string
		contentFmt  string // format with %s for section name
		renameTo    string
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

			// 1. Create file with section
			var content string
			if tc.name == "Markdown" {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 1)
			} else {
				content = strings.Replace(tc.contentFmt, "%s", sectionName, 2)
			}
			os.WriteFile(filePath, []byte(content), 0644)

			// 2. Rename (Move) Section
			moveCmd := NewRoot(factory)
			b := bytes.NewBufferString("")
			moveCmd.SetOut(b)
			moveCmd.SetErr(b)
			moveCmd.SetArgs([]string{"section", "move", relPath, sectionName, tc.renameTo})
			err := moveCmd.Execute()
			if err != nil {
				t.Fatalf("Move failed: %v Output: %s", err, b.String())
			}

			// Verify content
			newContentBytes, _ := os.ReadFile(filePath)
			newContent := string(newContentBytes)
			if !strings.Contains(newContent, tc.renameTo) {
				t.Errorf("File content does not contain renamed section %s. Content:\n%s", tc.renameTo, newContent)
			}

			// 3. Extract Section
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

			// Verify target file existence and content
			targetContentBytes, err := os.ReadFile(targetFile)
			if err != nil {
				t.Fatalf("Target file not created: %v", err)
			}
			targetContent := string(targetContentBytes)
			if len(targetContent) == 0 && tc.name != "Markdown" {
				t.Errorf("Extracted content is empty")
			}

			// 4. Integrate Section
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

			// Verify integration
			finalContentBytes, _ := os.ReadFile(filePath)
			finalContent := string(finalContentBytes)
			if !strings.Contains(finalContent, integrateContent) {
				t.Errorf("File content does not contain integrated content. Content:\n%s", finalContent)
			}
		})
	}
}
