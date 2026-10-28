"""🧱️ Staged batch `go-root-marker` (proof2-g): Go tests that bind a temporary repository root fence it with a `.git` marker.

`python3 go-root-marker.py check|apply|revert|copies <dir>` (`PATCH_ROOT` overrides the checkout; `copies` writes the patched files to <dir> for a Go `-overlay`).
Defect: the official Go route issues TMPDIR / SEMIO_TEST_ARTIFACT_DIR inside the checkout and root discovery is `.git`-only, so `SetRootDir(t.TempDir())` resolves
the REAL checkout and the test reads and writes the real repository. Fix: `sandbox.BindRoot` (used by the module tests) and the three workspace tests that
bind a root themselves create the marker first; discovery then ends in the test directory, exactly as it does for a directory outside any checkout."""
import os, sys

ROOT = os.environ.get("PATCH_ROOT") or os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 9)))
W = os.path.join("🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "🏠️workspace")
FILES = {
    "sandbox": os.path.join(W, "📦️packages", "🐹️go", "🏖️sandbox", "🌱️fixture-repository", "🐹️.go"),
    "unit": os.path.join(W, "🧪️tests", "🔬️unit", "🐹️.go"),
    "selection": os.path.join(W, "🧪️tests", "🧭️root-selection", "🐹️.go"),
}
EDITS = {
    "sandbox": [
        ("\tprevious, bound := workspace.LookupRootDir()\n\tworkspace.SetRootDir(dir)\n", "\tprevious, bound := workspace.LookupRootDir()\n\tfenceRepositoryRoot(t, dir)\n\tworkspace.SetRootDir(dir)\n"),
        ("// 🌱️NewRepository creates a throwaway git repository",
         "// 🧱️fenceRepositoryRoot gives dir a `.git` marker when it has none, so root discovery ends in dir itself even where the test directory lies inside a checkout.\nfunc fenceRepositoryRoot(t T, dir string) {\n\tt.Helper()\n\tmarker := filepath.Join(dir, \".git\")\n\tif _, err := os.Lstat(marker); err == nil {\n\t\treturn\n\t}\n\tif err := os.MkdirAll(marker, 0o755); err != nil {\n\t\tt.Fatalf(\"mkdir %s: %v\", marker, err)\n\t}\n}\n\n// 🌱️NewRepository creates a throwaway git repository"),
    ],
    "unit": [
        ("var FixtureOrigin string\n", "var FixtureOrigin string\n\n// 🧱️fenceRoot gives a test directory a `.git` marker so root discovery ends in it even when the directory lies inside a checkout.\nfunc fenceRoot(t *testing.T, dir string) string {\n\tt.Helper()\n\tif err := os.MkdirAll(filepath.Join(dir, \".git\"), 0o755); err != nil {\n\t\tt.Fatal(err)\n\t}\n\treturn dir\n}\n"),
        ("func TestRepoMetaDirUsesSemioRoot(t *testing.T) {\n\troot := CanonicalPath(t.TempDir())\n", "func TestRepoMetaDirUsesSemioRoot(t *testing.T) {\n\troot := CanonicalPath(fenceRoot(t, t.TempDir()))\n"),
        ("\ttmpDirA := t.TempDir()\n\ttmpDirB := t.TempDir()\n", "\ttmpDirA := fenceRoot(t, t.TempDir())\n\ttmpDirB := fenceRoot(t, t.TempDir())\n"),
    ],
    "selection": [
        ("if owned == \"\" { t.Fatal(\"finite fixture allocation exhausted\") }; defer os.RemoveAll(owned)\n", "if owned == \"\" { t.Fatal(\"finite fixture allocation exhausted\") }; defer os.RemoveAll(owned); if err := os.MkdirAll(filepath.Join(owned, \".git\"), 0755); err != nil { t.Fatal(err) }\n"),
    ],
}


def read(name):
    return open(os.path.join(ROOT, FILES[name]), encoding="utf8").read()


def state(name):
    text = read(name)
    if all(new in text for _, new in EDITS[name]):
        return "applied"
    if all(text.count(old) == 1 for old, _ in EDITS[name]):
        return "fresh"
    return "ANCHORS MOVED"


def patched(name):
    text = read(name)
    for old, new in EDITS[name]:
        text = text.replace(old, new, 1)
    return text


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    states = {name: state(name) for name in FILES}
    if mode == "check":
        print(states)
        return
    if mode == "copies":
        os.makedirs(sys.argv[2], exist_ok=True)
        for name in FILES:
            if states[name] != "fresh":
                sys.exit(f"{name}: {states[name]}")
            open(os.path.join(sys.argv[2], name + ".go"), "w", encoding="utf8").write(patched(name))
            print(os.path.join(ROOT, FILES[name]), "=", os.path.join(os.path.abspath(sys.argv[2]), name + ".go"))
        return
    if mode == "apply":
        if set(states.values()) != {"fresh"}:
            sys.exit(f"cannot apply: {states}")
        for name in FILES:
            text = patched(name)
            open(os.path.join(ROOT, FILES[name]), "w", encoding="utf8").write(text)
    elif mode == "revert":
        if set(states.values()) != {"applied"}:
            sys.exit(f"cannot revert: {states}")
        for name in FILES:
            text = read(name)
            for old, new in reversed(EDITS[name]):
                text = text.replace(new, old, 1)
            open(os.path.join(ROOT, FILES[name]), "w", encoding="utf8").write(text)
    else:
        sys.exit("check|apply|revert|copies <dir>")
    print(mode, "ok")


main()
