// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🕳️empty-workspace proves that a directory without a repository stays one when the issued scratch lies inside a checkout.

// #endregion 🧲️Header

package launcher

import (
	"os"
	"path/filepath"
	"testing"
)

// 🕳️The scratch below a checkout resolves to that checkout, the empty workspace never does.
func TestEmptyWorkspaceLiesOutsideACheckoutThatIssuesTheScratch(t *testing.T) {
	checkout := t.TempDir()
	for name, body := range map[string]string{"📋️project.json": "{}\n", "package.json": `{"workspaces":["*"]}` + "\n"} {
		if err := os.WriteFile(filepath.Join(checkout, name), []byte(body), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	scratch := filepath.Join(checkout, "scratch")
	if err := os.MkdirAll(scratch, 0o755); err != nil {
		t.Fatal(err)
	}
	for _, variable := range []string{"TMPDIR", "TMP", "TEMP"} {
		t.Setenv(variable, scratch)
	}
	inside, err := os.MkdirTemp(os.TempDir(), "inside-")
	if err != nil {
		t.Fatal(err)
	}
	if root, err := ResolveRoot(inside); err != nil || root != checkout {
		t.Fatalf("a directory below the checkout must resolve to it, got %q %v", root, err)
	}
	empty := emptyWorkspace(t)
	if _, err := ResolveRoot(empty); !IsRootNotFound(err) {
		t.Fatalf("the empty workspace %q resolves to a root: %v", empty, err)
	}
}
