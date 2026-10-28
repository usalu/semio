// 🩹️ Tests of the breach autofix applicator.

package graphql

import (
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	codebase "github.com/usalu/semio/repo/codebase"
	languages "github.com/usalu/semio/repo/languages"
	model "github.com/usalu/semio/repo/model"
	statutes "github.com/usalu/semio/repo/statutes"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestFixApplyAutofixes(t *testing.T) {
	original := readStatutesFixture(t, "🧪️file-fixable")
	expectedContent := readStatutesFixture(t, "🧪️file-fixable-expected")

	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	fixtureSrc := statutesFixtureDir + "/🧪️file-fixable/🟦️.tsx"
	srcAbs := filepath.Join(tmpDir, filepath.FromSlash(fixtureSrc))
	if err := os.MkdirAll(filepath.Dir(srcAbs), 0o755); err != nil {
		t.Fatalf("failed to create fixture dir: %v", err)
	}
	if err := os.WriteFile(srcAbs, []byte(original), 0o644); err != nil {
		t.Fatalf("failed to write fixture copy: %v", err)
	}

	bundles := codebase.LoadBundles()
	scope := workspace.Scope{Kind: workspace.ScopeFile, FilePath: fixtureSrc}
	ctx := statutes.NewPolicyContextWithFiles(scope, bundles, []string{fixtureSrc})
	breachs, err := statutes.CheckPoliciesWithContext(ctx, []string{"code"})
	if err != nil {
		t.Fatalf("policy check failed: %v", err)
	}

	var autofixable []model.Breach
	for _, v := range breachs {
		if v.Autofixable() {
			autofixable = append(autofixable, v)
		}
	}
	if len(autofixable) == 0 {
		t.Fatal("expected autofixable breachs in fixture")
	}

	fixed, err := applyAutofixes(fixtureSrc, autofixable)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed == 0 {
		t.Error("expected at least one fix applied")
	}

	fixedContent, err := workspace.ReadTextFile(srcAbs)
	if err != nil {
		t.Fatalf("failed to read fixed file: %v", err)
	}

	if strings.TrimSpace(fixedContent) != strings.TrimSpace(expectedContent) {
		t.Errorf("fixed content does not match expected.\nGot:\n%s\n\nExpected:\n%s", fixedContent, expectedContent)
	}
}

func TestApplyAutofixesRunsFormatterAfterEdit(t *testing.T) {
	tmpDir := workspace.CanonicalPath(t.TempDir())
	sandbox.BindRoot(t, tmpDir)

	if err := workspace.WriteTextFile(filepath.Join(tmpDir, "package.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write package.json: %v", err)
	}
	if err := workspace.WriteTextFile(filepath.Join(tmpDir, ".prettierrc.json"), "{}\n"); err != nil {
		t.Fatalf("failed to write .prettierrc.json: %v", err)
	}
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin", "prettier")
	if err := workspace.WriteTextFile(prettierBin, "#!/usr/bin/env sh\nexit 0\n"); err != nil {
		t.Fatalf("failed to write prettier stub: %v", err)
	}
	if err := os.Chmod(prettierBin, 0755); err != nil {
		t.Fatalf("failed to chmod prettier stub: %v", err)
	}

	originalLookup := workspace.FormatterBinaryLookup
	originalRun := workspace.FormatterCommandRun
	defer func() {
		workspace.FormatterBinaryLookup = originalLookup
		workspace.FormatterCommandRun = originalRun
	}()

	var ranBinary string
	var ranArgs []string
	var ranDir string
	workspace.FormatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}
	workspace.FormatterCommandRun = func(binary string, args []string, workDir string) error {
		ranBinary = binary
		ranArgs = append([]string{}, args...)
		ranDir = workDir
		return nil
	}

	testFile := "formatted.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖️A\n\nconst x = 1; // remove\n\n// #endregion 🔖️A\n"
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	expectedBinary := filepath.Join("node_modules", ".bin", "prettier")
	if ranBinary != expectedBinary {
		t.Fatalf("expected formatter to run with %q, got %q", expectedBinary, ranBinary)
	}
	if ranDir != tmpDir {
		t.Fatalf("expected formatter work dir %q, got %q", tmpDir, ranDir)
	}
	if len(ranArgs) < 3 || ranArgs[0] != "--write" || ranArgs[len(ranArgs)-1] != testFile {
		t.Fatalf("unexpected formatter args: %v", ranArgs)
	}
}

func TestApplyAutofixesFormatterFallbackNormalizesText(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	originalLookup := workspace.FormatterBinaryLookup
	defer func() { workspace.FormatterBinaryLookup = originalLookup }()
	workspace.FormatterBinaryLookup = func(file string) (string, error) {
		return "", exec.ErrNotFound
	}

	testFile := "fallback.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	content := "// #region 🔖️A\n\nconst x = 1;   \n\n// #endregion"
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}
	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}
	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Fatalf("expected 1 fix, got %d", fixed)
	}
	result, err := workspace.ReadTextFile(absPath)
	if err != nil {
		t.Fatalf("failed to read result: %v", err)
	}
	if strings.Contains(result, "   \n") {
		t.Fatalf("expected fallback formatter to trim trailing spaces, got %q", result)
	}
}

func TestFixSectionMissingEndName(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️MySection\n\nconst x = 1;\n\n// #endregion\n"
	expected := "// #region 🔖️MySection\n\nconst x = 1;\n\n// #endregion 🔖️MySection\n"

	testFile := "test_missing_end.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionNameMismatch(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️Alpha\n\nconst x = 1;\n\n// #endregion 🔖️Beta\n"
	expected := "// #region 🔖️Alpha\n\nconst x = 1;\n\n// #endregion 🔖️Alpha\n"

	testFile := "test_mismatch.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionNameMismatch, Scope: testFile, Line: 5},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixSectionEmpty(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️Keep\n\nconst x = 1;\n\n// #endregion 🔖️Keep\n\n// #region 🔖️Empty\n\n// #endregion 🔖️Empty\n\n// #region 🔖️Also\n\nconst y = 2;\n\n// #endregion 🔖️Also\n"
	expected := "// #region 🔖️Keep\n\nconst x = 1;\n\n// #endregion 🔖️Keep\n\n// #region 🔖️Also\n\nconst y = 2;\n\n// #endregion 🔖️Also\n"

	testFile := "test_empty.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeSectionEmpty, Scope: testFile + "#Empty", Line: 7},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixInlineComment(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️Section\n\n// inline one\n\n// inline two\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_inline.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixBlockComment(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️Section\n\n/* block comment */\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_block.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentBlock, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixJSDocComment(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️Section\n\n/** jsdoc comment */\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"
	expected := "// #region 🔖️Section\n\nconst x = 1;\n\n// #endregion 🔖️Section\n"

	testFile := "test_jsdoc.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentJSDoc, Scope: testFile, Line: 3},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 1 {
		t.Errorf("expected 1 fix, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if result != expected {
		t.Errorf("unexpected result:\nGot: %q\nWant: %q", result, expected)
	}
}

func TestFixMultipleBreachsSameFile(t *testing.T) {
	tmpDir := t.TempDir()
	sandbox.BindRoot(t, tmpDir)

	content := "// #region 🔖️A\n\n// bad comment\n\nconst a = 1;\n\n// #endregion\n\n// #region 🔖️B\n\n// another bad\n\nconst b = 2;\n\n// #endregion 🔖️Wrong\n"
	testFile := "test_multi.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	breachs := []model.Breach{
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 3},
		{Kind: model.BreachCodeSectionMissingEndName, Scope: testFile, Line: 7},
		{Kind: model.BreachCodeCommentInline, Scope: testFile, Line: 11},
		{Kind: model.BreachCodeSectionNameMismatch, Scope: testFile, Line: 15},
	}

	fixed, err := applyAutofixes(testFile, breachs)
	if err != nil {
		t.Fatalf("applyAutofixes failed: %v", err)
	}
	if fixed != 4 {
		t.Errorf("expected 4 fixes, got %d", fixed)
	}

	result, _ := workspace.ReadTextFile(absPath)
	if !strings.Contains(result, "// #endregion 🔖️A") {
		t.Error("expected missing end name to be fixed to A")
	}
	if !strings.Contains(result, "// #endregion 🔖️B") {
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
	sandbox.BindRoot(t, tmpDir)

	content := `// #region 🔖️Section
const x = 1; // trailing comment
// TODO: fix this
// this line is part of the todo description

// another normal comment
/* TODO: block todo */
const y = 2; // normal trailing
// #endregion 🎁️Fix
`

	testFile := "test_improved.tsx"
	absPath := filepath.Join(tmpDir, testFile)
	if err := workspace.WriteTextFile(absPath, content); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	ctx := &statutes.PolicyContext{}
	lang := languages.NewTypeScriptLanguage()
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

	result, _ := workspace.ReadTextFile(absPath)

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
