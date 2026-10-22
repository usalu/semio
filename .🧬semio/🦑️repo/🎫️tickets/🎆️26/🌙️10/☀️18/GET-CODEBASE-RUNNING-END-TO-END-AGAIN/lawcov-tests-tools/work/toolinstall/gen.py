"""📝 Authoring aid: writes the pinned tooling install image corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
P1 = "diff --git a/index.js b/index.js\n--- a/index.js\n+++ b/index.js\n@@ -1,2 +1,2 @@\n keep\n-old\n+new\n"
P2 = "diff --git a/a.js b/a.js\n--- a/a.js\n+++ b/a.js\n@@ -1 +1 @@\n-x\n+y\ndiff --git a/lib/b.js b/lib/b.js\n--- a/lib/b.js\n+++ b/lib/b.js\n@@ -2,1 +2,2 @@\n z\n+w\n"
BAD_FILE = "diff --git a/x b/y\n"
BAD_HUNK = "diff --git a/x b/x\n@@ nope @@\n"
REPEAT = "diff --git a/x b/x\n@@ -1 +1 @@\n-a\n+b\ndiff --git a/x b/x\n@@ -1 +1 @@\n-a\n+b\n"
ABS = "diff --git a//etc/x b//etc/x\n"
UP = "diff --git a/../x b/../x\n"
recipe = lambda deps, patches: {"name": "tools", "private": True, "packageManager": "bun@1.3.14", "dependencies": deps, "semio": {"toolPatches": patches}}
R = "Pinned tooling patch authority changed"
cases = [
    {"name": "no patches", "recipe": recipe({"a": "1.0.0", "@s/b": "2.0.0"}, {}), "lock": {"lockfileVersion": 1, "workspaces": {}, "packages": {"a": ["a@1.0.0"]}, "patchedDependencies": {}}, "sources": {}},
    {"name": "one patch", "recipe": recipe({"z": "2.0.0"}, {"z@2.0.0": "patches/z.patch"}), "lock": {"lockfileVersion": 1, "patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": P1}},
    {"name": "first party links are not installed", "recipe": recipe({"a": "1.0.0", "mine": "workspace:*", "@s/mine": "workspace:*"}, {}), "lock": {"patchedDependencies": {}}, "sources": {}},
    {"name": "two patches keep the authored order", "recipe": recipe({}, {"b@1.0.0": "patches/b.patch", "a@1.0.0": "patches/a.patch"}), "lock": {"patchedDependencies": {"b@1.0.0": "patches/b.patch", "a@1.0.0": "patches/a.patch"}}, "sources": {"patches/a.patch": P1, "patches/b.patch": P2}},
    {"name": "equal patch bytes share one artifact", "recipe": recipe({}, {"a@1.0.0": "patches/a.patch", "b@1.0.0": "patches/b.patch"}), "lock": {"patchedDependencies": {"a@1.0.0": "patches/a.patch", "b@1.0.0": "patches/b.patch"}}, "sources": {"patches/a.patch": P1, "patches/b.patch": P1}},
    {"name": "the lock keeps its other fields in place and the patched table is replaced where it stands", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"lockfileVersion": 1, "patchedDependencies": {"z@2.0.0": "patches/z.patch"}, "workspaces": {"": {"name": "tools"}}, "packages": {"z": ["z@2.0.0", "", {}, "sha512-abc"]}}, "sources": {"patches/z.patch": P1}},
    {"name": "unicode names survive", "recipe": {**recipe({"é": "1.0.0"}, {}), "name": "tööls"}, "lock": {"patchedDependencies": {}, "note": "ü"}, "sources": {}},
    {"name": "a lock that is not an object is refused", "recipe": recipe({}, {}), "lockRaw": "null", "sources": {}, "error": "Pinned tooling lock must be an object"},
    {"name": "a lock that is an array is refused", "recipe": recipe({}, {}), "lockRaw": "[]", "sources": {}, "error": "Pinned tooling lock must be an object"},
    {"name": "a lock that is a string is refused", "recipe": recipe({}, {}), "lockRaw": "\"x\"", "sources": {}, "error": "Pinned tooling lock must be an object"},
    {"name": "a lock without a patch table is refused when the recipe has an empty one", "recipe": recipe({}, {}), "lock": {"lockfileVersion": 1}, "sources": {}, "error": R},
    {"name": "a lock with another patch path is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/other.patch"}}, "sources": {"patches/z.patch": P1}, "error": R},
    {"name": "a lock with an extra patch is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch", "y@1.0.0": "patches/y.patch"}}, "sources": {"patches/z.patch": P1}, "error": R},
    {"name": "the patch table order is part of the authority", "recipe": recipe({}, {"a@1": "patches/a.patch", "b@1": "patches/b.patch"}), "lock": {"patchedDependencies": {"b@1": "patches/b.patch", "a@1": "patches/a.patch"}}, "sources": {"patches/a.patch": P1, "patches/b.patch": P1}, "error": R},
    {"name": "missing patch bytes are refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {}, "error": "Pinned tooling patch bytes required"},
    {"name": "empty patch bytes are not a patch", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": ""}, "error": "Original tooling patch sections required"},
    {"name": "a patch for two different files in one diff pair is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": BAD_FILE}, "error": "Original tooling patch file required"},
    {"name": "a malformed hunk header is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": BAD_HUNK}, "error": "Original tooling patch hunk required"},
    {"name": "a repeated file section is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": REPEAT}, "error": "Tooling patch file repeated: x"},
    {"name": "an absolute patch file is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": ABS}, "error": "Relative tooling patch file required"},
    {"name": "a parent segment in a patch file is refused", "recipe": recipe({}, {"z@2.0.0": "patches/z.patch"}), "lock": {"patchedDependencies": {"z@2.0.0": "patches/z.patch"}}, "sources": {"patches/z.patch": UP}, "error": "Relative tooling patch file required"},
]
json.dump({"version": 1, "cases": cases}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(cases))
