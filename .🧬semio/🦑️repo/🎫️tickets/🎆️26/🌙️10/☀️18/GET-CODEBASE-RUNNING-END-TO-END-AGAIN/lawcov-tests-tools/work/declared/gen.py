"""📝 Authoring aid: writes the declared target input corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
P = lambda input, root, expected: {"input": input, "projectRoot": root, "expected": expected}
patterns = [
    P("{projectRoot}/a/b", "pkg", "pkg/a/b"), P("{projectRoot}/a/b", ".", "a/b"), P("{workspaceRoot}/x/y", "pkg", "x/y"), P("{workspaceRoot}/./x//y/", "pkg", "x/y"),
    P("{projectRoot}/../sib/x", "pkg", "sib/x"), P("{projectRoot}/../../out", "pkg", None), P("{workspaceRoot}/../x", "pkg", None), P("{workspaceRoot}/", "pkg", ""), P("{projectRoot}/", ".", ""),
    P("plain/path", "pkg", None), P(5, "pkg", None), P(None, "pkg", None), P({"runtime": "x"}, "pkg", None), P("{projectRoot}", "pkg", None), P("{workspaceRoot}", "pkg", None), P("!{projectRoot}/x", "pkg", None),
    P("{projectRoot}/a/../b", "pkg", "pkg/b"), P("{projectRoot}/a/**/*.ts", "pkg", "pkg/a/**/*.ts"), P("{workspaceRoot}/a/..", "pkg", ""), P("{workspaceRoot}/a/../..", "pkg", None),
    P("{projectRoot}/é/📜️script.ts", "pkg/é", "pkg/é/é/📜️script.ts"), P("{projectRoot}/a\\b", "pkg", "pkg/a\\b"), P("{projectRoot}/.hidden/x", "pkg", "pkg/.hidden/x"), P("{projectRoot}/./", "pkg", "pkg"),
    P("{projectRoot}/../x", ".", None), P("{projectRoot}/a/../../x", "p/q", "p/x"), P("{unknown}/x", "pkg", None), P("{projectRoot}x/y", "pkg", None), P("{workspaceRoot}x/y", "pkg", None),
]
tree = {
    "pkg/test/a.test.ts": "", "pkg/test/b.test.ts": "", "pkg/test/sub/c.test.ts": "", "pkg/test/sub/deep/d.test.mjs": "", "pkg/test/data.json": "{}",
    "pkg/test/node_modules/x/e.test.ts": "", "pkg/src/m.ts": "", "pkg/src/.hidden.ts": "", "pkg/target/t.ts": "", "pkg/.git/g.ts": "", "pkg/test/sub/f.test.cjs": "", "pkg/test/sub/g.test.tsx": "", "root.test.ts": "", "other/z.test.ts": "", "other/sub/y.js": "",
}
G = lambda name, inputs, root, expected: {"name": name, "inputs": inputs, "projectRoot": root, "expected": expected}
globs = [
    G("a test directory glob finds the modules below it", ["{projectRoot}/test/**/*.test.ts"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts", "pkg/test/sub/c.test.ts"]),
    G("a module suffix of any flavour is admitted", ["{projectRoot}/test/**/*.test.{ts,mjs}"], "pkg", []),
    G("module flavours by their own extension patterns", ["{projectRoot}/test/**/*.mjs", "{projectRoot}/test/**/*.cjs"], "pkg", ["pkg/test/sub/deep/d.test.mjs", "pkg/test/sub/f.test.cjs"]),
    G("an extension pattern that does not end in a literal suffix is ignored", ["{projectRoot}/test/**/*.*js"], "pkg", []),
    G("a tsx pattern", ["{projectRoot}/test/**/*.tsx"], "pkg", ["pkg/test/sub/g.test.tsx"]),
    G("an explicit file is not a glob", ["{projectRoot}/src/m.ts"], "pkg", []),
    G("a glob that does not end in a module suffix is ignored", ["{projectRoot}/test/**"], "pkg", []),
    G("a json glob is ignored", ["{projectRoot}/test/**/*.json"], "pkg", []),
    G("an exclusion removes the files it matches", ["{projectRoot}/test/**/*.test.ts", "!{projectRoot}/test/sub/**"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts"]),
    G("an exclusion for a single name", ["{projectRoot}/test/**/*.test.ts", "!{projectRoot}/test/a.test.ts"], "pkg", ["pkg/test/b.test.ts", "pkg/test/sub/c.test.ts"]),
    G("an exclusion that names nothing inside the workspace is ignored", ["{projectRoot}/test/*.test.ts", "!/etc/**", "!plain/**"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts"]),
    G("node_modules, .git and target are never entered", ["{projectRoot}/**/*.ts"], "pkg", ["pkg/src/.hidden.ts", "pkg/src/m.ts", "pkg/test/a.test.ts", "pkg/test/b.test.ts", "pkg/test/sub/c.test.ts"]),
    G("the whole workspace from the workspace root", ["{workspaceRoot}/**/*.ts"], "pkg", ["other/z.test.ts", "pkg/src/.hidden.ts", "pkg/src/m.ts", "pkg/test/a.test.ts", "pkg/test/b.test.ts", "pkg/test/sub/c.test.ts", "root.test.ts"]),
    G("the walk starts at the literal directory prefix", ["{workspaceRoot}/other/*.test.ts"], "pkg", ["other/z.test.ts"]),
    G("a glob under a missing directory finds nothing", ["{projectRoot}/nowhere/**/*.ts"], "pkg", []),
    G("two overlapping globs list a file once, sorted", ["{projectRoot}/test/**/*.test.ts", "{projectRoot}/test/*.test.ts", "{workspaceRoot}/pkg/test/sub/*.ts"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts", "pkg/test/sub/c.test.ts"]),
    G("inputs that are not strings are ignored", [5, None, {"runtime": "x"}, "{projectRoot}/test/*.test.ts"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts"]),
    G("a glob with a question mark", ["{projectRoot}/test/?.test.ts"], "pkg", ["pkg/test/a.test.ts", "pkg/test/b.test.ts"]),
    G("a glob that escapes the workspace is ignored", ["{projectRoot}/../../x/*.ts"], "pkg", []),
    G("no inputs", [], "pkg", []),
]
json.dump({"version": 1, "patterns": patterns, "tree": tree, "globs": globs}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(patterns), len(globs))
