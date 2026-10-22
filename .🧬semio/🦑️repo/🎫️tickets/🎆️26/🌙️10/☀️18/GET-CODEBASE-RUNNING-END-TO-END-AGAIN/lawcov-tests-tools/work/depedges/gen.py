"""📝 Authoring aid: writes the dependency-edges resolution corpus (corpus.json)."""
import json, os

W = os.path.dirname(os.path.abspath(__file__)) + "/"
projects = {
    "app": {"root": "apps/app"}, "lib": {"root": "libs/lib"}, "libx": {"root": "libs/lib/x"}, "root": {"root": "."},
    "native": {"root": "crates/n", "metadata": {"nativeRoot": "native/n"}},
    "alpha": {"root": "twin/dir"}, "beta": {"root": "other/beta", "metadata": {"nativeRoot": "twin/dir"}},
}
by_package = {"@scope/lib": "lib", "@scope/app": "app", "lodash": "lodash-fork"}
locked = {"keys": {"left-pad": "left-pad@1.0.0", "ghost": "ghost@2.0.0", "@types/node": "@types/node@22.0.0"}, "external": ["npm:left-pad@1.0.0", "npm:@types/node@22.0.0"]}
MAIN = "apps/app/src/main.ts"


def row(name, text, imports, expected, file=MAIN, queried=None):
    return {"name": name, "file": file, "text": text, "imports": imports, "expected": expected, **({"queried": queried} if queried is not None else {})}


rows = [
    row("a relative import resolves to the deepest owner", 'import {a} from "../../../libs/lib/src/a.ts";', ["../../../libs/lib/src/a.ts"], ["lib"]),
    row("a nested owner wins over its parent", 'import {a} from "../../../libs/lib/x/y.ts";', ["../../../libs/lib/x/y.ts"], ["libx"]),
    row("an import inside the same project names the project", 'import {a} from "./util.ts";', ["./util.ts"], ["app"]),
    row("a file owned by the workspace root names nobody", 'import {a} from "../../../tools/x.ts";', ["../../../tools/x.ts"], []),
    row("a path outside the workspace names nobody", 'import {a} from "../../../../outside/x.ts";', ["../../../../outside/x.ts"], []),
    row("a declared native root is an owner root", 'import {a} from "../../../native/n/glue.ts";', ["../../../native/n/glue.ts"], ["native"]),
    row("two projects on one directory resolve to the lower name", 'import {a} from "../../../twin/dir/a.ts";', ["../../../twin/dir/a.ts"], ["alpha"]),
    row("a root name is not a prefix match", 'import {a} from "../../../libs/library/a.ts";', ["../../../libs/library/a.ts"], []),
    row("a directory import names its project", 'import {a} from "..";', [".."], ["app"]),
    row("the current directory names its project", 'import {a} from ".";', ["."], ["app"]),
    row("a bare workspace package", 'import {a} from "@scope/lib";', ["@scope/lib"], ["lib"], queried=[]),
    row("a deep path into a scoped workspace package", 'import {a} from "@scope/lib/sub/path.js";', ["@scope/lib/sub/path.js"], ["lib"], queried=[]),
    row("a deep path into an unscoped workspace package", 'import {a} from "lodash/fp/get";', ["lodash/fp/get"], ["lodash-fork"], queried=[]),
    row("a package of the lock file", 'import pad from "left-pad";', ["left-pad"], ["npm:left-pad@1.0.0"], queried=[[MAIN, "left-pad"]]),
    row("a deep path into a locked package asks for the package", 'import pad from "left-pad/lib/x";', ["left-pad/lib/x"], ["npm:left-pad@1.0.0"], queried=[[MAIN, "left-pad"]]),
    row("a locked package that has no external node is dropped", 'import g from "ghost";', ["ghost"], [], queried=[[MAIN, "ghost"]]),
    row("an unknown package is dropped", 'import g from "nobody";', ["nobody"], [], queried=[[MAIN, "nobody"]]),
    row("a scoped locked package", 'import type {T} from "@types/node/fs";', ["@types/node/fs"], ["npm:@types/node@22.0.0"], queried=[[MAIN, "@types/node"]]),
    row("a bare scope alone is not a package", 'import g from "@scope";', ["@scope"], [], queried=[[MAIN, "@scope"]]),
    row("core modules are never packages", 'import a from "node:fs";\nimport b from "fs";\nimport c from "fs/promises";\nimport d from "node:path";\nimport e from "path";', ["node:fs", "fs", "fs/promises", "node:path", "path"], [], queried=[]),
    row("dynamic imports, require and re-exports count", 'const m = await import("./b.ts");\nconst c = require("./c.ts");\nexport * from "../../../libs/lib/d.ts";\nexport {z} from "@scope/lib";', ["./b.ts", "./c.ts", "../../../libs/lib/d.ts", "@scope/lib"], ["app", "lib"], queried=[]),
    row("type only imports count for project ownership", 'import type {T} from "../../../libs/lib/types.ts";\nimport {type U} from "../../../libs/lib/more.ts";', ["../../../libs/lib/types.ts", "../../../libs/lib/more.ts"], ["lib"]),
    row("duplicates collapse and the result is sorted", 'import a from "@scope/lib";\nimport b from "./b.ts";\nimport c from "../../../libs/lib/c.ts";\nimport d from "@scope/app";\nimport e from "left-pad";', ["@scope/lib", "./b.ts", "../../../libs/lib/c.ts", "@scope/app", "left-pad"], ["app", "lib", "npm:left-pad@1.0.0"], queried=[[MAIN, "left-pad"]]),
    row("a declaration file imports like any module", 'import {A} from "../../../libs/lib/a";', ["../../../libs/lib/a"], ["lib"], file="apps/app/types/shape.d.ts"),
    row("a json file has no imports", '{"import": "x", "from": "./y"}', [], [], file="apps/app/data.json"),
    row("an empty file has no imports", "", [], []),
    row("the lock file is asked about the importing file", 'import pad from "left-pad";', ["left-pad"], ["npm:left-pad@1.0.0"], file="libs/lib/src/deep/f.ts", queried=[["libs/lib/src/deep/f.ts", "left-pad"]]),
    row("a relative import climbs from a nested file", 'import {a} from "../../a.ts";', ["../../a.ts"], ["lib"], file="libs/lib/x/z/q.ts"),
]

cases = [
    {"name": "the preferred file exists", "files": ["a/x.ts"], "map": {"p": ["a/x.ts", "b/x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "a/x.ts"},
    {"name": "a moved file is found by its base name", "files": ["b/x.ts"], "map": {"p": ["a/x.ts", "b/x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "b/x.ts"},
    {"name": "two existing candidates keep the preferred path", "files": ["b/x.ts", "c/x.ts"], "map": {"p": ["b/x.ts", "c/x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "a/x.ts"},
    {"name": "a candidate that does not exist does not count", "files": ["c/x.ts"], "map": {"p": ["b/x.ts", "c/x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "c/x.ts"},
    {"name": "without a file map the preferred path stays", "files": ["c/x.ts"], "map": None, "project": "p", "preferred": "a/x.ts", "expected": "a/x.ts"},
    {"name": "a project missing from the map keeps the preferred path", "files": ["c/x.ts"], "map": {"q": ["c/x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "a/x.ts"},
    {"name": "an empty file list keeps the preferred path", "files": ["c/x.ts"], "map": {"p": []}, "project": "p", "preferred": "a/x.ts", "expected": "a/x.ts"},
    {"name": "an explicit hint selects another base name", "files": ["c/y.ts"], "map": {"p": ["c/y.ts", "c/x.ts"]}, "project": "p", "preferred": "a/x.ts", "hint": "y.ts", "expected": "c/y.ts"},
    {"name": "a root level file matches its own name", "files": ["x.ts"], "map": {"p": ["x.ts"]}, "project": "p", "preferred": "a/x.ts", "expected": "x.ts"},
    {"name": "a longer base name is not a match", "files": ["a/ax.ts"], "map": {"p": ["a/ax.ts"]}, "project": "p", "preferred": "z/x.ts", "expected": "z/x.ts"},
    {"name": "the hint is the last path segment of the preferred path", "files": ["m/deep/x.ts"], "map": {"p": ["m/deep/x.ts"]}, "project": "p", "preferred": "old/place/x.ts", "expected": "m/deep/x.ts"},
]
json.dump({"version": 1, "workspaceRoot": "/ws", "projects": projects, "byPackage": by_package, "locked": locked, "rows": rows, "tracked": cases}, open(W + "corpus.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(len(rows), len(cases))
