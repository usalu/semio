// 🛠️ Tests of the fix mutation: the served schema, the recorded payloads and the real autofix run.

package graphql

import (
	context "context"
	encoding "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func readGraphQLFixture(t *testing.T, relative string) []byte {
	t.Helper()
	content, err := os.ReadFile(sandbox.OriginPath("..", "..", "🧪️tests", "🧫️fixtures", filepath.FromSlash(relative)))
	if err != nil {
		t.Fatalf("failed to read fixture %s: %v", relative, err)
	}
	return content
}

func TestServedSchemaRendersTheCommittedSdl(t *testing.T) {
	committed := string(readGraphQLFixture(t, "📜️served-schema/🔗️.graphql"))
	if rendered := RenderSDL(BuildSchema()); rendered != committed {
		t.Fatalf("the schema built in code differs from the committed SDL.\nrendered:\n%s", rendered)
	}
}

func TestFixMutationOverRecordedBreachs(t *testing.T) {
	recording, err := NewRecordingContext(readGraphQLFixture(t, "🔣️repo-records.json"))
	if err != nil {
		t.Fatalf("failed to build the recording context: %v", err)
	}
	executor, err := NewExecutorWithContext(recording.GetRootDir(), recording)
	if err != nil {
		t.Fatalf("failed to build the executor: %v", err)
	}
	type fixPayload struct {
		Fix struct {
			Fixed     int `json:"fixed"`
			Remaining int `json:"remaining"`
			Breachs   []struct {
				ID          string `json:"id"`
				Autofixable bool   `json:"autofixable"`
			} `json:"breachs"`
		} `json:"fix"`
	}
	run := func(source string) fixPayload {
		raw, err := executor.ExecuteJSON(context.Background(), source, nil)
		if err != nil {
			t.Fatalf("%s failed: %v", source, err)
		}
		var payload fixPayload
		if err := encoding.Unmarshal([]byte(raw), &payload); err != nil {
			t.Fatalf("failed to decode %s: %v", raw, err)
		}
		return payload
	}
	scoped := run(`mutation { fix(scope: "repo/go") { fixed remaining breachs { id autofixable } } }`)
	if scoped.Fix.Fixed != 1 || scoped.Fix.Remaining != 0 || len(scoped.Fix.Breachs) != 0 {
		t.Fatalf("the scope names one autofixable breach, got %+v", scoped.Fix)
	}
	everything := run(`mutation { fix { fixed remaining breachs { id autofixable } } }`)
	if everything.Fix.Fixed != 1 || everything.Fix.Remaining != 0 {
		t.Fatalf("the other autofixable breach is fixed by the unscoped run, got %+v", everything.Fix)
	}
	nothing := run(`mutation { fix { fixed remaining breachs { id } } }`)
	if nothing.Fix.Fixed != 0 || nothing.Fix.Remaining != 0 {
		t.Fatalf("a fixed breach is not fixed twice, got %+v", nothing.Fix)
	}
	kinds := []string{}
	for _, event := range recording.Events() {
		kinds = append(kinds, event.(map[string]interface{})["kind"].(string))
	}
	if strings.Join(kinds, ",") != "fix,fix,fix" {
		t.Fatalf("every fix leaves an event behind, got %v", kinds)
	}
}

func TestFixMutationAppliesAutofixesOnDisk(t *testing.T) {
	original := readStatutesFixture(t, "🧪️file-fixable")
	expected := readStatutesFixture(t, "🧪️file-fixable-expected")

	root := t.TempDir()
	sandbox.BindRoot(t, root)

	relative := statutesFixtureDir + "/🧪️file-fixable/🟦️.tsx"
	absolute := filepath.Join(root, filepath.FromSlash(relative))
	if err := os.MkdirAll(filepath.Dir(absolute), 0o755); err != nil {
		t.Fatalf("failed to create fixture dir: %v", err)
	}
	if err := os.WriteFile(absolute, []byte(original), 0o644); err != nil {
		t.Fatalf("failed to write fixture copy: %v", err)
	}

	var repoContext model.RepoContext = NewRepoContext(root)
	scope := relative
	result, err := repoContext.Fix(&scope)
	if err != nil {
		t.Fatalf("Fix failed: %v", err)
	}
	if result.Fixed == 0 {
		t.Fatalf("expected the fixable fixture to lose breaches, got %+v", result)
	}
	content, err := workspace.ReadTextFile(absolute)
	if err != nil {
		t.Fatalf("failed to read the fixed file: %v", err)
	}
	if strings.TrimSpace(content) != strings.TrimSpace(expected) {
		t.Errorf("fixed content does not match expected.\nGot:\n%s\n\nExpected:\n%s", content, expected)
	}
	again, err := repoContext.Fix(&scope)
	if err != nil {
		t.Fatalf("second Fix failed: %v", err)
	}
	if again.Fixed != 0 {
		t.Errorf("a fixed file must not be fixed again, got %+v", again)
	}
}

func TestNodeResolvesBundlesOfTheSharedCorpus(t *testing.T) {
	recording, err := NewRecordingContext(readGraphQLFixture(t, "🔣️repo-records.json"))
	if err != nil {
		t.Fatalf("failed to build the recording context: %v", err)
	}
	executor, err := NewExecutorWithContext(recording.GetRootDir(), recording)
	if err != nil {
		t.Fatalf("failed to build the executor: %v", err)
	}
	var corpus struct {
		Queries []struct {
			ID        string                 `json:"id"`
			Source    string                 `json:"source"`
			Variables map[string]interface{} `json:"variables"`
		} `json:"queries"`
	}
	if err := encoding.Unmarshal(readGraphQLFixture(t, "▶️query-execution/🔣️queries.json"), &corpus); err != nil {
		t.Fatalf("failed to decode the corpus: %v", err)
	}
	expected := map[string]string{
		"node-resolves-a-bundle":              "Bundle",
		"node-ignores-presentation-selectors": "Bundle",
		"node-bundle-by-variable":             "Bundle",
		"node-unknown-bundle-is-null":         "",
	}
	seen := 0
	for _, entry := range corpus.Queries {
		want, ok := expected[entry.ID]
		if !ok {
			continue
		}
		seen++
		raw, err := executor.ExecuteJSON(context.Background(), entry.Source, entry.Variables)
		if err != nil {
			t.Fatalf("%s failed: %v", entry.ID, err)
		}
		var payload struct {
			Node *struct {
				Typename string `json:"__typename"`
			} `json:"node"`
		}
		if err := encoding.Unmarshal([]byte(raw), &payload); err != nil {
			t.Fatalf("%s: failed to decode %s: %v", entry.ID, raw, err)
		}
		got := ""
		if payload.Node != nil {
			got = payload.Node.Typename
		}
		if got != want {
			t.Errorf("%s: node resolved to %q, want %q", entry.ID, got, want)
		}
	}
	if seen != len(expected) {
		t.Fatalf("the shared corpus carries %d of the %d node entries", seen, len(expected))
	}
}
