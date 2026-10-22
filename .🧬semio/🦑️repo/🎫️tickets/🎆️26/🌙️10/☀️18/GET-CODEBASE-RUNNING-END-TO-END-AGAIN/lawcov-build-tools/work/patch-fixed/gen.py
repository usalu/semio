"""📝 Authoring aid: writes the extra patch corpus (corpus.json) beside the unit's existing apply corpus."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
H = "diff --git a/f.txt b/f.txt\n--- a/f.txt\n+++ b/f.txt\n"


def v(name, patch, error=None):
    return {"name": name, "patch": patch, "accept": error is None, "error": error or ""}


c = {"schemaVersion": 1}
c["validate"] = [
    v("one hunk", H + "@@ -1,2 +1,2 @@\n a\n-b\n+c\n"),
    v("counts default to one", H + "@@ -1 +1 @@\n-a\n+b\n"),
    v("a rename is not an original file", "diff --git a/x b/y\n--- a/x\n+++ b/y\n@@ -1 +1 @@\n-a\n+b\n", "Original tooling patch file required"),
    v("a parent segment is refused", "diff --git a/../x b/../x\n@@ -1 +1 @@\n-a\n+b\n", "Relative tooling patch file required"),
    v("an absolute path is refused", "diff --git a//x b//x\n@@ -1 +1 @@\n-a\n+b\n", "Relative tooling patch file required"),
    v("a backslash is refused", "diff --git a/x\\y b/x\\y\n@@ -1 +1 @@\n-a\n+b\n", "Relative tooling patch file required"),
    v("a drive colon is refused", "diff --git a/C:x b/C:x\n@@ -1 +1 @@\n-a\n+b\n", "Relative tooling patch file required"),
    v("a dot segment is refused", "diff --git a/./x b/./x\n@@ -1 +1 @@\n-a\n+b\n", "Relative tooling patch file required"),
    v("a file may appear once", H + "@@ -1 +1 @@\n-a\n+b\n" + H + "@@ -1 +1 @@\n-a\n+b\n", "Tooling patch file repeated: f.txt"),
    v("two files are fine", H + "@@ -1 +1 @@\n-a\n+b\n" + H.replace("f.txt", "g.txt") + "@@ -1 +1 @@\n-a\n+b\n"),
    v("a hunk needs a file", "@@ -1 +1 @@\n-a\n+b\n", "Original tooling patch hunk required"),
    v("a malformed hunk header is refused", H + "@@ -x +1 @@\n-a\n+b\n", "Original tooling patch hunk required"),
    v("the old count must match the body", H + "@@ -1,2 +1,2 @@\n a\n-b", "Tooling patch hunk counts changed"),
    v("the new count must match the body", H + "@@ -1,1 +1,2 @@\n-a\n+b", "Tooling patch hunk counts changed"),
    v("a truncated hunk that ends in a newline reports its counts", H + "@@ -1,2 +1,2 @@\n a\n-b\n", "Tooling patch hunk counts changed"),
    v("a hunk with a surplus line reports its counts", H + "@@ -1,1 +1,1 @@\n-a\n+b\n+c\n", "Tooling patch hunk counts changed"),
    v("a hunk with a missing added line reports its counts", H + "@@ -1,1 +1,2 @@\n-a\n+b\n", "Tooling patch hunk counts changed"),
    v("a stray blank row in the middle of a hunk is still an unprefixed line", H + "@@ -1,2 +1,2 @@\n-a\n\n+b\n+c\n", "Tooling patch unprefixed hunk line"),
    v("a hunk with only a header and a final newline reports its counts", H + "@@ -1,1 +1,1 @@\n", "Tooling patch hunk counts changed"),
    v("an unprefixed line is refused", H + "@@ -1,1 +1,1 @@\n-a\n+b\nxyz\n", "Tooling patch unprefixed hunk line"),
    v("a blank context line stripped of its space is refused", H + "@@ -1,3 +1,3 @@\n a\n\n c\n", "Tooling patch unprefixed hunk line"),
    v("a second hunk must keep the cumulative coordinates", H + "@@ -1,1 +1,2 @@\n-a\n+a\n+b\n@@ -5,1 +5,1 @@\n-e\n+f\n", "Tooling patch cumulative coordinates changed"),
    v("a second hunk with shifted coordinates is accepted", H + "@@ -1,1 +1,2 @@\n-a\n+a\n+b\n@@ -5,1 +6,1 @@\n-e\n+f\n"),
    v("overlapping hunks are refused", H + "@@ -1,2 +1,2 @@\n a\n-b\n+c\n@@ -2,1 +2,1 @@\n-b\n+d\n", "Tooling patch cumulative coordinates changed"),
    v("an empty patch has no sections", "", "Original tooling patch sections required"),
    v("a file without a hunk has no sections", H, "Original tooling patch sections required"),
    v("the no-newline witness is not counted", H + "@@ -1 +1 @@\n-a\n\\ No newline at end of file\n+b\n\\ No newline at end of file\n"),
    v("a hunk that inserts at the very start", H + "@@ -0,0 +1,1 @@\n+a\n"),
    v("coordinates beyond the safe integers are refused", H + "@@ -99999999999999999999 +1 @@\n-a\n+b\n", "Bounded tooling patch coordinates required"),
]
G = H.replace("f.txt", "g.txt")
c["additions"] = [
    {"name": "added lines of two files", "patch": H + "@@ -1 +1,2 @@\n a\n+one\n" + G + "@@ -1 +1,2 @@\n a\n+two\n", "expected": {"f.txt": ["one"], "g.txt": ["two"]}, "error": ""},
    {"name": "a later removal cancels an earlier addition", "patch": H + "@@ -1 +1,3 @@\n a\n+gone\n+kept\n@@ -9 +11,1 @@\n-gone\n", "expected": {"f.txt": ["kept"]}, "error": ""},
    {"name": "blank additions are ignored and indentation is kept", "patch": H + "@@ -1 +1,3 @@\n a\n+   \n+  keep\n", "expected": {"f.txt": ["  keep"]}, "error": ""},
    {"name": "a file with only context owns no additions", "patch": H + "@@ -1 +1 @@\n a\n", "expected": {"f.txt": []}, "error": ""},
    {"name": "an escaping path is refused", "patch": "--- a/x\n+++ b/../x\n+a\n", "expected": None, "error": "Relative tool patch file required"},
    {"name": "a patch with no target has no owner", "patch": "+orphan\n", "expected": None, "error": "Owned tool patch file required"},
]
c["receipt"] = [
    {"name": "applied additions and context are receipted by the sha256 of the real bytes", "files": {"a.js": "x\nnew\nz\n"}, "patch": "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1,2 +1,3 @@\n x\n+new\n z\n", "error": ""},
    {"name": "two owned files", "files": {"a.js": "x\nnew\nz\n", "b/c.js": "q\nr\n"}, "patch": "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1,2 +1,3 @@\n x\n+new\n z\ndiff --git a/b/c.js b/b/c.js\n--- a/b/c.js\n+++ b/b/c.js\n@@ -1 +1,2 @@\n q\n+r\n", "error": ""},
    {"name": "a requested line that is missing is refused", "files": {"a.js": "x\nz\n"}, "patch": "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1,2 +1,3 @@\n x\n+new\n z\n", "error": "Requested tool patch was not applied: a.js"},
    {"name": "changed context after the hunk is refused", "files": {"a.js": "x\nnew\nQ\n"}, "patch": "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1,2 +1,3 @@\n x\n+new\n z\n", "error": "Exact SDK post-hunk context changed: a.js"},
    {"name": "context that moved to another position is refused", "files": {"a.js": "z\nx\nnew\n"}, "patch": "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1,2 +1,3 @@\n x\n+new\n z\n", "error": "Exact SDK post-hunk context changed: a.js"},
    {"name": "an invalid patch is refused before any file is read", "files": {}, "patch": "diff --git a/x b/y\n", "error": "Original tooling patch file required"},
]
c["apply"] = [
    {"name": "carriage returns are content and survive", "before": "a\r\nb\r\n", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -2,1 +2,1 @@\n-b\r\n+B\r\n", "after": "a\r\nB\r\n", "accept": True, "library": False},
    {"name": "an insertion into an empty file", "before": "", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -0,0 +1,2 @@\n+a\n+b\n", "after": "a\nb\n", "accept": True, "library": True},
    {"name": "a deletion of every line", "before": "a\nb\n", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -1,2 +0,0 @@\n-a\n-b\n", "after": "", "accept": True, "library": True},
    {"name": "a hunk that starts beyond the file is refused", "before": "a\n", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -7,1 +7,1 @@\n-z\n+y\n", "after": "", "accept": False, "library": True},
    {"name": "a removed line with different newline state is refused", "before": "a\nb", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -2,1 +2,1 @@\n-b\n+B\n", "after": "", "accept": False, "library": False},
    {"name": "an insertion after the last line", "before": "a\nb\n", "patch": "diff --git a/source.js b/source.js\n--- a/source.js\n+++ b/source.js\n@@ -2,0 +3,1 @@\n+c\n", "after": "a\nb\nc\n", "accept": True, "library": True},
]
json.dump(c, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
