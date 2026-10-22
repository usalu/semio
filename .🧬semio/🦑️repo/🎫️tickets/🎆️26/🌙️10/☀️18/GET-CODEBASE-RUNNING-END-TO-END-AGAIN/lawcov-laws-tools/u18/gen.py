import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
base = read("pylib.py") + "\n" + read("ptree.py") + "\n"
cases = []


def case(rule, id_, args, expect, **kw):
    row = {"id": id_, "rule": rule, "args": args, "expect": expect}
    row.update(kw)
    cases.append(row)


FB = ["🧬schema", "⚙engine", "🚪io", "🪓decomposer"]
case("policyStdioArtifactFacets", "facets-fall-back-without-taxonomy-dirs", [{}], FB)
case("policyStdioArtifactFacets", "facets-skip-legacy-dirs-and-keep-new-ones", [{"artifactComponentDirs": ["🧬schema", "🚪io", "🗣dsl", "📚examples", "📸snapshot", "🎒pack"]}], FB + ["📚examples", "🎒pack"])
case("policyStdioSchemaChildDirs", "schema-children-from-the-taxonomy", [{"schemaChildDirs": ["🔺diff", "💡inferences"]}], ["🔺diff", "💡inferences"])
case("policyStdioSchemaChildDirs", "schema-children-fall-back-when-empty", [{"schemaChildDirs": []}], ["📸snapshot", "🔺diff", "🧬mutations"])
case("policyStdioRepresentationDirs", "representation-dirs-are-copied", [{"nativeRepresentationDirs": ["📝text", "💾binary"]}], ["📝text", "💾binary"])
case("policyStdioFormatDir", "format-directory-lookup", [{"avi": {"dir": "d/avi", "depends": []}}, "avi"], "d/avi")
case("policyStdioFormatDir", "unknown-format-has-no-directory", [{"avi": {"dir": "d/avi", "depends": []}}, "wav"], None)
case("policyStdioArtifactsDirName", "artifacts-directory-name", [], "🗿artifacts")
for id_, body, trait, expect in [
    ("plain-impl", "impl Foo for Bar {}", "Foo", True), ("generic-impl", "impl<T: X> Foo<T> for Bar<T> {}", "Foo", True), ("commented-impl", "// impl Foo for Bar {}", "Foo", False),
    ("block-commented-impl", "/* impl Foo for Bar {} */", "Foo", False), ("string-impl", 'let s = "impl Foo for Bar";', "Foo", False), ("longer-trait-name", "impl Foolish for Bar {}", "Foo", False),
    ("prefix-trait-name", "impl FooBar for X {}", "Foo", False), ("inherent-impl", "impl Bar { fn foo() {} }", "Foo", False),
]:
    case("policyRustFileHasRealTraitImpl", "real-trait-impl-" + id_, [body, trait], expect)
case("specLeaves", "spec-leaf-filenames-follow-the-file-kind-taxonomy", [], {"text": ["📖️.grammar.semio", "🔤️.ebnf", "🅰️.g4", "🔗️.graphql", "🔣️.json", "🛰️.proto", "🦀️.rs", "🟦️.ts"], "binary": ["📡️.protocol.semio", "🔠️.abnf", "🥋️.ksy", "🌶️.spicy", "🦀️.rs", "🟦️.ts"]})
rules = ["policyStdioArtifactFacets", "policyStdioSchemaChildDirs", "policyStdioRepresentationDirs", "policyStdioFormatDir", "policyStdioArtifactsDirName", "policyRustFileHasRealTraitImpl", "specLeaves"]
schema = val_schema("Stdio Owner Table Corpus", rules)
extra = '''
test("the owner table names the stdio plugin, its artifacts and the shared facet vocabulary", () => {
  expect([owner.POLICY_STDIO_OWNER_TABLE_REL, owner.POLICY_STDIO_PLUGIN_REL, owner.POLICY_STDIO_ARTIFACTS_REL]).toEqual(["✏️s/🔌️plugins/🗄️stdio/📇️registry/🔣️.json", "✏️s/🔌️plugins/🗄️stdio", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"]);
  expect([owner.POLICY_STDIO_FACET_DECOMPOSER, owner.POLICY_STDIO_FACET_TEXT, owner.POLICY_STDIO_FACET_BINARY, owner.POLICY_STDIO_FACET_DESERIALIZERS, owner.POLICY_STDIO_FACET_SERIALIZERS, owner.POLICY_STDIO_IO_IMPORT, owner.POLICY_STDIO_IO_EXPORT]).toEqual(["🪓️decomposer", "📝️text", "💾️binary", "🧩️deserializers", "🧵️serializers", "📥️import", "📤️export"]);
  expect([...owner.POLICY_STDIO_CODEC_BANNED_MARKERS]).toEqual(["SRAS", "IFCCARTOONMESH", 'b"minimal"', "stub codec", "minimal stub codec"]);
});
'''
ts = val_ts(
    "📋 The stdio owner-table helpers (facet vocabulary, schema children, trait-impl detection, spec-leaf names) return exactly what their language-agnostic corpus says and, where an independent Python model exists, agree with it; the shared constants are pinned.",
    'import * as owner from "../../🟦.ts";',
    '''  policyStdioArtifactFacets: (_root, taxonomy) => owner.policyStdioArtifactFacets(taxonomy),
  policyStdioSchemaChildDirs: (_root, taxonomy) => owner.policyStdioSchemaChildDirs(taxonomy),
  policyStdioRepresentationDirs: (_root, taxonomy) => owner.policyStdioRepresentationDirs(taxonomy),
  policyStdioFormatDir: (_root, artifacts, id) => owner.policyStdioFormatDir(artifacts, id),
  policyStdioArtifactsDirName: () => owner.policyStdioArtifactsDirName(),
  policyRustFileHasRealTraitImpl: (_root, body, trait) => owner.policyRustFileHasRealTraitImpl(body, trait),
  specLeaves: () => ({ text: [...owner.POLICY_STDIO_TEXT_SPEC_LEAVES], binary: [...owner.POLICY_STDIO_BINARY_SPEC_LEAVES] }),''',
    extra,
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚪artifact-io/📋owner-table", "📋owners", ts, {"version": 1, "cases": cases}, schema, base + open(os.path.join(HERE, "oracle_owner.py"), encoding="utf8").read())
print(folder, plen, len(cases))
