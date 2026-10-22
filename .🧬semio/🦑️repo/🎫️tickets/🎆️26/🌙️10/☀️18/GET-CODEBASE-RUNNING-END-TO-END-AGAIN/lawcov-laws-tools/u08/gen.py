import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
K = "app-plugin/"


def b(i, kind, line, prio="medium"):
    return [i, K + kind, prio, line]


def case(id_, rule, source, expect, **kw):
    row = {"id": id_, "rule": rule, "source": source, "expect": expect}
    row.update(kw)
    cases.append(row)


SC = "crate/a"
case("region-markers-format-labels-and-pairs", "policyRegionFormatBreaches", [
    "//#region 🔧Foo", "// #region Spaced", "//#endregion 🔧Foo", "//#endregion", "//#region", "//#region Tests", "//#endregion Tests", "//#region 🧪Tests", "//#endregion 🧪Tests"],
    [b("region-format-space-%s-2" % SC, "region-marker-format", 2), b("region-label-mismatch-%s-2" % SC, "region-marker-format", 3), b("region-format-missing-label-%s-4" % SC, "region-marker-format", 4), b("region-format-missing-label-%s-5" % SC, "region-marker-format", 5), b("region-tests-label-%s-6" % SC, "region-tests-label", 6)], scope=SC)
case("region-markers-clean-source", "policyRegionFormatBreaches", ["//#region 🔧Foo", "fn a() {}", "//#endregion 🔧Foo"], [], scope=SC)
case("app-builders-live-in-the-manifest-region", "policyManifestRegionBreaches", [
    "//#region 🔖Manifest", 'let a = App::builder("x");', "//#endregion 🔖Manifest", 'let b = App::builder("y");', "#[cfg(test)]", "mod tests {", '    let c = App::builder("z");', "}"],
    [b("manifest-region-%s-4" % SC, "manifest-region", 4)], scope=SC)
case("document-app-structs-are-named-after-their-id", "policyStructNamingBreaches", [
    'const A_APP_ID: &str = "my-app";', 'const B_APP_ID: &str = "other";', "impl DocumentApp for Wrong {", "    const ID: &str = A_APP_ID;", "}", "impl DocumentApp for OtherApp {", "    const ID: &str = B_APP_ID;", "}",
    "impl DocumentApp for Mystery {", "    const ID: &str = C_APP_ID;", "}"],
    [b("struct-naming-%s-Wrong" % SC, "struct-naming", 3)], scope=SC)
case("several-document-apps-need-their-own-modules", "policyModLayoutBreaches", [
    "pub mod one {", "impl DocumentApp for A {", "}", "}", "impl DocumentApp for B {", "}"], [b("mod-layout-%s-5" % SC, "mod-layout", 5, "high")], scope=SC)
case("a-single-document-app-needs-no-module", "policyModLayoutBreaches", ["impl DocumentApp for A {", "}"], [], scope=SC)
case("selection-ids-must-delegate-to-the-sdk", "policySelectionIdsBreaches", ["fn selection_ids(a: u8) -> u8 { 1 }", "fn selection_ids(b: u8) -> u8 { semio_framework_plugin::selection_ids(b) }"], [b("sdk-selection-ids-%s-1" % SC, "sdk-selection-ids", 1)], scope=SC)
case("selection-ids-delegating-through-an-import-alias", "policySelectionIdsBreaches", ["use semio_framework_plugin::selection_ids as sel;", "fn selection_ids(x: u8) -> u8 { sel(x) }"], [], scope=SC)
case("artifact-app-laws-delegates", "policyArtifactAppLawDelegateBreaches", [
    "fn new_app_with_registry<T>() -> X { artifact_app_laws::new_app_with_registry::<T>(a) }", "fn new_app() -> X { build() }", "fn meta() -> M { artifact_app_laws::meta() }", "fn meta2() -> M { 1 }", "fn meta () -> M { 2 }"],
    [b("sdk-artifact-app-laws-delegate-%s-2" % SC, "sdk-artifact-app-laws-delegate", 2), b("sdk-artifact-app-laws-delegate-%s-5" % SC, "sdk-artifact-app-laws-delegate", 5)], scope=SC)
CR = {"dir": "x/🎛apps/🔭demo/🦀rust", "pluginId": "puzzle"}
case("tree-item-redefinitions-and-wrappers", "policyTreeItemBreaches", [
    "fn tree_item_with_action(a: u8) {}", "fn tree_item_plain(a: &ActionDescriptor) -> T { build(a) }", "fn tree_item_ok(a: &ActionDescriptor) -> T { tree_item_with_action(a) }", "fn tree_item_noaction(a: u8) {}"],
    [b("sdk-tree-item-redefinition-%s-1" % CR["dir"], "sdk-tree-item", 1), b("sdk-tree-item-wrapper-%s-2" % CR["dir"], "sdk-tree-item", 2)], crate=CR)
case("tree-item-redefinition-allowlisted-by-module", "policyTreeItemBreaches", ["pub mod 3d {", "fn tree_item_with_action() {}", "}"], [], crate=CR)
LB = {"dir": "crate/l", "pluginId": "demo"}
case("labels-structs-use-the-sdk-macro", "policyLabelsStructBreaches", [
    "struct DemoLabels {", "}", "app_labels!(Foo);", "/// doc", "/// doc", "struct FooLabels {", "}", "//", "//", "//", "//", "//", "//", "//", "struct FarLabels {", "}"],
    [b("sdk-labels-struct-%s-DemoLabels" % LB["dir"], "sdk-labels-struct", 1), b("sdk-labels-struct-%s-FarLabels" % LB["dir"], "sdk-labels-struct", 15)], crate=LB)
case("labels-struct-in-the-two-axis-allowlist-is-low", "policyLabelsStructBreaches", ["struct CadLabels {", "}"], [b("sdk-labels-struct-crate/c-CadLabels", "sdk-labels-struct", 1, "low")], crate={"dir": "crate/c", "pluginId": "cad"})

props = {"scope": {"type": "string"}, "source": {"type": "array", "items": {"type": "string"}}, "crate": {"type": "object", "additionalProperties": False, "required": ["dir", "pluginId"], "properties": {"dir": {"type": "string"}, "pluginId": {"type": "string"}}}}
rules = ["policyRegionFormatBreaches", "policyManifestRegionBreaches", "policyStructNamingBreaches", "policyModLayoutBreaches", "policySelectionIdsBreaches", "policyArtifactAppLawDelegateBreaches", "policyTreeItemBreaches", "policyLabelsStructBreaches"]
schema = pure_schema("Crate Shape Law Corpus", rules, props, ["source"])
ts = pure_ts(
    "🏗 The crate-shape laws report exactly the breaches their language-agnostic Rust-source corpus names (regions, manifest, naming, layout, SDK delegation), agree with an independent Python model and reject a malformed corpus.",
    'import { policyArtifactAppLawDelegateBreaches, policyLabelsStructBreaches, policyManifestRegionBreaches, policyModLayoutBreaches, policyRegionFormatBreaches, policySelectionIdsBreaches, policyStructNamingBreaches, policyTreeItemBreaches } from "../../🟦.ts";',
    '''  policyRegionFormatBreaches: (row) => policyRegionFormatBreaches(row.scope, row.source),
  policyManifestRegionBreaches: (row) => policyManifestRegionBreaches(row.scope, row.source),
  policyStructNamingBreaches: (row) => policyStructNamingBreaches(row.scope, text(row.source)),
  policyModLayoutBreaches: (row) => policyModLayoutBreaches(row.scope, row.source),
  policySelectionIdsBreaches: (row) => policySelectionIdsBreaches(row.scope, text(row.source)),
  policyArtifactAppLawDelegateBreaches: (row) => policyArtifactAppLawDelegateBreaches(row.scope, text(row.source)),
  policyTreeItemBreaches: (row) => policyTreeItemBreaches({ dir: row.crate.dir, libRelPath: "", shape: "taxonomy", role: "", ownerRel: "", pluginId: row.crate.pluginId }, text(row.source), row.source),
  policyLabelsStructBreaches: (row) => policyLabelsStructBreaches({ dir: row.crate.dir, libRelPath: "", shape: "taxonomy", role: "", ownerRel: "", pluginId: row.crate.pluginId }, text(row.source)),''',
)
ts = ts.replace("const rules:", 'const text = (lines: readonly string[]) => lines.join("\\n") + "\\n";\nconst rules:', 1)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🏗crate-shape", "🏗shape", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
