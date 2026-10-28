// 🔬️ Tests of the graphql domain, split out of the pre-split godfile suite.

package graphql

import (
	context "context"
	json "encoding/json"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	codebase "github.com/usalu/semio/repo/codebase"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func execCommandWithTimeout(t *testing.T, timeout time.Duration, dir string, env []string, name string, args ...string) []byte {
	t.Helper()
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	cmd := exec.CommandContext(ctx, name, args...)
	if dir != "" {
		cmd.Dir = dir
	}
	if env != nil {
		cmd.Env = env
	}
	output, err := cmd.CombinedOutput()
	if ctx.Err() == context.DeadlineExceeded {
		t.Fatalf("%s %v timed out after %s:\n%s", name, args, timeout, output)
	}
	if err != nil {
		t.Fatalf("%s %v failed: %v\n%s", name, args, err, output)
	}
	return output
}

// 🧫️statutesFixtureDir is the surviving statute fixture tree. The godfile-era
// `repo/asset/fixture/some/folder` was deleted with the legacy tree.
const statutesFixtureDir = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/🧪️tests/🧫️fixtures/📁️some/📁️folder"

func readStatutesFixture(t *testing.T, folder string) string {
	t.Helper()
	abs := sandbox.OriginPath("..", "..", "..", "📜️statutes", "🧪️tests", "🧫️fixtures", "📁️some", "📁️folder", folder, "🟦️.tsx")
	content, err := os.ReadFile(abs)
	if err != nil {
		t.Fatalf("failed to read fixture %s: %v", abs, err)
	}
	return string(content)
}

// 🏗️withMonorepoFixture builds a throwaway monorepo in `t.TempDir()` and binds the
// workspace to it as a repository of its own, so the walk-backed queries assert against a tree this file owns
// instead of against the developer's own checkout.
func withMonorepoFixture(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	write := func(rel, content string) {
		abs := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
			t.Fatalf("mkdir %s: %v", rel, err)
		}
		if err := os.WriteFile(abs, []byte(content), 0o644); err != nil {
			t.Fatalf("write %s: %v", rel, err)
		}
	}
	write("compose/README.md", "---\nname: compose\nkind: user\n---\n\n# compose\n")
	write("repo/README.md", "---\nname: repo\nkind: infrastructure\n---\n\n# repo\n")
	write("coda/README.md", "---\nname: coda\nkind: research\n---\n\n# coda\n")
	write("compose/js/AGENTS.md", "---\nbundle:\n  emoji: 📜️\n---\n")
	write("compose/js/index.ts", fixtureSectionedTypeScript)
	write("compose/go/AGENTS.md", "---\nbundle:\n  emoji: 🐹️\n---\n")
	write("compose/go/main.go", fixtureSectionedGo)
	write("repo/client/AGENTS.md", "---\nbundle:\n  emoji: ⌨️\n---\n")
	write("repo/client/main.go", fixtureSectionedGo)
	t.Cleanup(codebase.InvalidateTechnologyCache)
	sandbox.BindRoot(t, root)
	codebase.InvalidateTechnologyCache()
	return root
}

// 🩹️seedFixableTechnology writes a one-technology repository holding the fixable statute fixture into `root`, binds the workspace to it as a repository of its own and answers the repository-relative path of the fixture copy, so the autofix cases fix a tree this file owns instead of the developer's own checkout.
func seedFixableTechnology(t *testing.T, root string, content string) string {
	t.Helper()
	relative := "compose/js/🟦️.tsx"
	for path, body := range map[string]string{
		"compose/README.md":    "---\nname: compose\nkind: user\n---\n\n# compose\n",
		"compose/js/AGENTS.md": "---\nbundle:\n  emoji: 📜️\n---\n",
		relative:               content,
	} {
		abs := filepath.Join(root, filepath.FromSlash(path))
		if err := os.MkdirAll(filepath.Dir(abs), 0o755); err != nil {
			t.Fatalf("mkdir %s: %v", path, err)
		}
		if err := os.WriteFile(abs, []byte(body), 0o644); err != nil {
			t.Fatalf("write %s: %v", path, err)
		}
	}
	t.Cleanup(codebase.InvalidateTechnologyCache)
	sandbox.BindRoot(t, root)
	codebase.InvalidateTechnologyCache()
	return relative
}

func fixtureExecutor(t *testing.T) *Executor {
	t.Helper()
	root := withMonorepoFixture(t)
	built, err := NewExecutorWithContext(root, NewRepoContext(root))
	if err != nil {
		t.Fatalf("failed to create fixture executor: %v", err)
	}
	return built
}

const fixtureSectionedTypeScript = `// #region 🔖️Header
// [💻️js/index.ts](repo://file/💻️index)
// #endregion 🔖️Header

// #region 🧩️State
export const state = 1;

// #region 🏪️Store
export const store = 2;
// #endregion 🏪️Store
// #endregion 🧩️State
`

const fixtureSectionedGo = `// #region 🔖️Header
// [💻️client/main.go](repo://file/💻️main)
// #endregion 🔖️Header

package main

// #region 🎖️Entry
func main() {}
// #endregion 🎖️Entry
`

// 🕸️#region 🎙️GraphQL
func TestGraphQLRepoQuery(t *testing.T) {
	result, err := fixtureExecutor(t).ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "compose") {
		t.Errorf("Expected result to contain 'compose', got: %s", result)
	}
}

func TestGraphQLBundlesQuery(t *testing.T) {
	root := withMonorepoFixture(t)
	fixtureExecutor, err := NewExecutorWithContext(root, NewRepoContext(root))
	if err != nil {
		t.Fatalf("failed to create fixture executor: %v", err)
	}
	result, err := fixtureExecutor.ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Fatalf("ExecuteGraphQL bundles returned error: %v", err)
	}
	var payload struct {
		Repo struct {
			Bundles []struct {
				ID   string `json:"id"`
				Name string `json:"name"`
				Root string `json:"root"`
			} `json:"bundles"`
		} `json:"repo"`
	}
	if err := json.Unmarshal([]byte(result), &payload); err != nil {
		t.Fatalf("failed to decode bundles payload %s: %v", result, err)
	}
	names := map[string]string{}
	for _, bundle := range payload.Repo.Bundles {
		if bundle.ID == "" {
			t.Errorf("bundle %q has an empty id in %s", bundle.Name, result)
		}
		names[bundle.Name] = bundle.Root
	}
	for _, want := range []string{"compose/js", "compose/go", "repo/client"} {
		if _, ok := names[want]; !ok {
			t.Errorf("Expected bundles to contain %q, got: %s", want, result)
		}
	}
	if len(payload.Repo.Bundles) != 3 {
		t.Errorf("expected exactly the fixture's three bundles, got %d: %s", len(payload.Repo.Bundles), result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := fixtureExecutor(t).ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := fixtureExecutor(t).ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

// 🌐️TestGraphQLEffortMutationsAndQueries tests that GraphQL mutations and queries support reasoning effort.
func TestGraphQLEffortMutationsAndQueries(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	sandbox.BindRoot(t, tmpDir)
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	schema, err := buildSchema(NewResolver(tmpDir))
	if err != nil {
		t.Fatalf("buildSchema failed: %v", err)
	}

	goalMutation := `
		mutation {
			goalCreate(input: {
				title: "GQL Goal",
				description: "GQL Goal Desc",
				prompt: "GQL Goal Prompt",
				dueDate: "2026-12-31",
				client: "copilot-chat",
				llm: "gemini-3-7-pro",
				effort: "high",
				noManagement: true
			}) {
				id
				title
				llm
				effort
			}
		}
	`
	result := Do(Params{
		Schema:        schema,
		RequestString: goalMutation,
		Context:       context.Background(),
	})
	if len(result.Errors) > 0 {
		t.Fatalf("goalCreate GraphQL errors: %v", result.Errors)
	}
	goalData := result.Data.(map[string]interface{})["goalCreate"].(map[string]interface{})
	if goalData["effort"] != "high" {
		t.Errorf("goal.effort = %v, want 'high'", goalData["effort"])
	}
	if goalData["llm"] != "gemini-3-7-pro" {
		t.Errorf("goal.llm = %v, want 'gemini-3-7-pro'", goalData["llm"])
	}

	ticketMutation := `
		mutation {
			ticketOpen(input: {
				emoji: "🎫️",
				title: "GQL Ticket",
				prompt: "GQL Ticket Prompt",
				client: COPILOT_CHAT,
				llm: "opus-4-7",
				effort: "max",
				goal: "GQL-GOAL",
				noManagement: true
			}) {
				year
				month
				day
				slug
				effort
				llm
			}
		}
	`
	tResult := Do(Params{
		Schema:        schema,
		RequestString: ticketMutation,
		Context:       context.Background(),
	})
	if len(tResult.Errors) > 0 {
		t.Fatalf("ticketOpen GraphQL errors: %v", tResult.Errors)
	}
	ticketData := tResult.Data.(map[string]interface{})["ticketOpen"].(map[string]interface{})
	if ticketData["effort"] != "max" {
		t.Errorf("ticket.effort = %v, want 'max'", ticketData["effort"])
	}
	if ticketData["llm"] != "opus-4-7" {
		t.Errorf("ticket.llm = %v, want 'opus-4-7'", ticketData["llm"])
	}
}
