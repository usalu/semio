import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []


def case(rule, id_, args, expect, **kw):
    row = {"id": id_, "rule": rule, "args": args, "expect": expect}
    row.update(kw)
    cases.append(row)


for rel, expect in [("x/🎛apps/🔭demo/🦀rust", "demo"), ("no/apps/here", ""), ("a/🎛apps", ""), ("a/🎛apps/🔭d-e/x", "d-e")]:
    case("policyAppIdFromCrateDir", "app-id-" + (expect or "none") + "-" + str(len(cases)), [rel], expect)
P = "✏s/🔌plugins/🔭alpha"
norm = [
    ("legacy-module-with-lib-suffix", P + "/🔨modules/🔸render/⚡implementations/🦀rust/📦lib.rs", "alpha/render#lib"),
    ("legacy-module-with-distinct-app", P + "/🎛apps/🔸ui/🔨modules/🔸panel/⚡implementations/🦀rust/🦀.rs", "alpha/ui/panel"),
    ("legacy-module-with-app-named-like-the-plugin", P + "/🎛apps/🔸alpha/🔨modules/🔸panel/⚡implementations/🦀rust/🦀.rs", "alpha/panel"),
    ("legacy-framework-module", "🧰framework/🔨modules/🔸x/⚡implementations/🦀rust/🦀.rs", "framework/x"),
    ("artifact-layout-engine", P + "/🗿artifacts/🔸doc/⚙engine/🦀.rs", "alpha/doc/engine"),
    ("artifact-layout-protocol-alias", P + "/🗿artifacts/🔸doc/📡protocol/🦀.rs", "alpha/doc/spr"),
    ("artifact-layout-same-name-as-plugin", P + "/🗿artifacts/🔸alpha/⚙engine/🦀.rs", "alpha/engine"),
    ("artifact-layout-deeper-tail", P + "/🗿artifacts/🔸doc/⚙engine/🔸sub/🦀.rs", "alpha/doc/engine#sub-"),
    ("unrecognised-path-is-unchanged", "plain/path.txt", "plain/path.txt"),
    ("dot-slash-prefix-is-dropped", "./plain/x", "plain/x"),
]
for id_, rel, expect in norm:
    case("policyNormalizeRelPath", "normalize-" + id_, [rel], expect)
case("policyNormalizeRelPath", "normalize-hidden-directory-is-not-a-dot-slash-prefix", [".hidden/file"], ".hidden/file", defect="🟦.ts:96 drops two characters when the path merely starts with a dot, so .hidden/file becomes idden/file")
case("policyNewSurfacePriority", "priority-of-plugin-crate-is-kept", [{"pluginId": "alpha"}, "medium"], "medium")
case("policyNewSurfacePriority", "priority-outside-plugins-is-low", [{"pluginId": ""}, "high"], "low")
lines = ["//#region 🔧A", "  // #region  Spaced label  ", "//#endregion", "// #endregion Spaced label", "//#regionX", "// #region", "code // #region no"]
case("policyParseRegionEvents", "region-events-record-spacing-and-labels", [lines], [
    {"kind": "open", "line": 1, "spaceAfterSlashes": False, "label": "🔧A"}, {"kind": "open", "line": 2, "spaceAfterSlashes": True, "label": "Spaced label"},
    {"kind": "close", "line": 3, "spaceAfterSlashes": False, "label": ""}, {"kind": "close", "line": 4, "spaceAfterSlashes": True, "label": "Spaced label"},
    {"kind": "open", "line": 6, "spaceAfterSlashes": True, "label": ""},
])
ev = lambda k, l, label="": {"kind": k, "line": l, "spaceAfterSlashes": False, "label": label}
case("policyPairRegionSpans", "region-pairing-nests-and-ignores-unmatched-closes", [[ev("close", 1, "orphan"), ev("open", 2, "outer"), ev("open", 3, "inner"), ev("close", 4, "inner"), ev("close", 5, "outer"), ev("open", 6, "dangling")]],
     [{"label": "inner", "closeLabel": "inner", "startLine": 3, "endLine": 4}, {"label": "outer", "closeLabel": "outer", "startLine": 2, "endLine": 5}])
mods = ["pub mod outer {", "    pub mod inner {", "        fn x() { let s = \"}\"; }", "    }", "}", "pub mod flat;", "mod private {", "}"]
spans = [{"name": "inner", "startLine": 2, "endLine": 4}, {"name": "outer", "startLine": 1, "endLine": 5}]
case("policyParseModSpans", "pub-mod-spans-ignore-braces-in-strings", [mods], spans)
case("policyModAtLine", "innermost-module-wins", [spans, 3], "inner")
case("policyModAtLine", "outer-module-at-its-own-line", [spans, 5], "outer")
case("policyModAtLine", "no-module-outside-spans", [spans, 7], "")
for id_, label, expect in [("emoji-prefix", "🔖Foo bar", "Foo bar"), ("digits-and-symbols-prefix", "12-- Baz ", "Baz"), ("already-clean", "Plain", "Plain"), ("accented-letter-is-kept", "🔖Écran", "Écran")]:
    case("policyLabelName", "label-" + id_, [label], expect)
src = 'fn a() { let s = "}"; if x { y } } trailing'
case("policyExtractFnBody", "body-balances-braces-and-masks-strings", [src, 0], '{ let s = "}"; if x { y } }')
case("policyExtractFnBody", "body-starts-at-the-first-brace-after-the-index", [src, 12], '{ y }')
case("policyExtractFnBody", "no-brace-gives-empty-body", ["fn a();", 0], "")
case("policyExtractFnBody", "unterminated-body-runs-to-the-end", ["fn a() { x", 0], "{ x")
for id_, i, expect in [("kebab", "my-app", "MyAppApp"), ("snake-and-kebab", "my-app_x", "MyAppXApp"), ("single", "demo", "DemoApp"), ("empty-segments", "--a--b", "ABApp")]:
    case("policyPascalAppStructName", "pascal-" + id_, [i], expect)
LIBP = "🧰framework/🛍products/🦑repo/🔨modules/📚library/"
for id_, p, expect in [("root-script", "📜script.ts", True), ("owned-directory-file", LIBP + "⚖laws/🧩plugin-consistency/🏗crate-shape/🟦.ts", True), ("owned-directory-itself-not", LIBP + "⚖laws/🧩plugin-consistency", False), ("other-library-file", LIBP + "🚨policy/x/🟦.ts", False)]:
    case("policyIsRootScriptSource", "root-script-source-" + id_, [p], expect)
case("policyRootScriptModulePaths", "root-script-modules-are-the-owned-leaf-modules", [], ["📜script.ts", LIBP + "⚖laws/🧩plugin-consistency/🔸b/🟦.ts", LIBP + "🎛commands/🔸a/🟦.ts"],
     files={LIBP + "🎛commands/🔸a/🟦.ts": [], LIBP + "🎛commands/🔸a/🧪tests/🟦.ts": [], LIBP + "🎛commands/🔸a/other.ts": [], LIBP + "⚖laws/🧩plugin-consistency/🔸b/🟦.ts": [], LIBP + "🚨policy/🔸c/🟦.ts": [], LIBP + "🎛commands/node_modules/🟦.ts": []})
PK = "🧰framework/🛍products/💻os/📦packages/🦀rust/🎛commands/"
case("policyPackageScriptModulePaths", "package-script-modules-are-the-os-commands", [], [PK + "🔸x/🟦.ts"], files={PK + "🔸x/🟦.ts": [], PK + "🔸x/🧪tests/🟦.ts": [], "🧰framework/🛍products/💻os/📦packages/🦀rust/🔸y/🟦.ts": []})
case("policyAllRustFiles", "rust-files-skip-generated-and-dependency-directories", [], ["a/b.rs", "c.rs", "z/y.rs"],
     files={"a/b.rs": [], "c.rs": [], "z/y.rs": [], "target/t.rs": [], "node_modules/n.rs": [], "a/c.txt": [], "compose/x.rs": []})

rules = ["policyAppIdFromCrateDir", "policyNormalizeRelPath", "policyNewSurfacePriority", "policyParseRegionEvents", "policyPairRegionSpans", "policyParseModSpans", "policyModAtLine", "policyLabelName", "policyExtractFnBody", "policyPascalAppStructName", "policyIsRootScriptSource", "policyRootScriptModulePaths", "policyPackageScriptModulePaths", "policyAllRustFiles"]
schema = val_schema("Plugin Consistency Foundation Corpus", rules)
rmap = "\n".join("  %s: (root, ...args) => (%s as any)(%s)," % (r, r, "root, ...args" if r in ("policyRootScriptModulePaths", "policyPackageScriptModulePaths", "policyAllRustFiles") else "...args") for r in rules)
ts = val_ts(
    "🏛 The plugin-consistency foundation helpers (path normalisation, region and module span parsing, brace extraction, script-owner discovery) return exactly what their language-agnostic corpus says, agree with an independent Python model, and one recorded dot-prefix defect is pinned as a failing row.",
    "import { policyAllRustFiles, policyAppIdFromCrateDir, policyExtractFnBody, policyIsRootScriptSource, policyLabelName, policyModAtLine, policyNewSurfacePriority, policyNormalizeRelPath, policyPackageScriptModulePaths, policyPairRegionSpans, policyParseModSpans, policyParseRegionEvents, policyPascalAppStructName, policyRootScriptModulePaths } from \"../../🟦.ts\";",
    rmap,
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🏛foundation", "🏛foundation", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
