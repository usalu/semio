import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
x = []


def case(id_, rule, files, expect=None, **kw):
    row = {"id": id_, "rule": rule, "files": files}
    if expect is not None:
        row["expect"] = expect
    row.update(kw)
    cases.append(row)


def b(i, kind, line=0, prio="high"):
    return [i, kind, prio, line]


model = [
    "#[derive(Debug, dsl::DslDocument)]", "pub struct GoodSnap {}", "#[derive(dsl::DslOps)]", "pub enum GoodMut {}",
    "impl DocumentApp for GoodApp {", "    type Snapshot = crate::GoodSnap;", "    type Mutation = GoodMut;", "}",
    "impl DocumentApp for BadApp {", "    type Snapshot = BadSnap;", "    type Mutation = Value;", "}",
    "impl DocumentApp for AliasApp {", "    type Snapshot = AliasedSnap<T>;", "    type Mutation = M2;", "}",
    "impl DocumentApp for HandApp {", "    type Snapshot = Handmade;", "    type Mutation = Handmade;", "}",
]
DK = "dsl-migration/completeness"
case("every-document-app-snapshot-and-mutation-has-a-dsl", "policyDslCompletenessBreaches", {
    "a/model.rs": model, "a/alias.rs": ["use crate::GoodSnap as AliasedSnap;"], "b/hand.rs": ["impl DocumentDsl for Handmade {}", "impl<T> vcs::OpText for Handmade {}"],
}, [b("dsl-completeness-a/model.rs-9-Snapshot", DK, 9), b("dsl-completeness-a/model.rs-13-Mutation", DK, 13)])
case("pack-round-trips-accompany-dsl-round-trips", "policyPackCompletenessBreaches", {
    "p/one.rs": ["assert_dsl_round_trip(x);"], "p/two.rs": ["assert_dsl_round_trip(x);", "assert_dsl_pack_equivalence(x);"], "p/three.rs": ["assert_document_text_round_trip(x);", "assert_document_pack_round_trip(x);"], "p/four.rs": ["fn nothing() {}"],
}, [b("pack-completeness-p/one.rs", "pack-migration/completeness")])
GIS = "✏s/🔌plugins/🌍gis/🗿artifacts/🌍gis/🗿artifact/🦀.rs"
case("command-envelopes-accompany-pack-checks", "policyCommandEnvelopeCompletenessBreaches", {
    "e/one.rs": ["assert_dsl_pack_equivalence(x);"], "e/two.rs": ["assert_document_pack_round_trip(x);", "assert_command_envelope_round_trip(x);"], GIS: ["assert_dsl_pack_equivalence(x);"], "e/three.rs": ["fn nothing() {}"],
}, [b("command-envelope-completeness-e/one.rs", "protocol-migration/command-envelope-completeness")])
FK = "dsl-migration/ts-facade-completeness"
AR = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/"
case("scaffold-stubs-belong-to-constitutional-facets", "policyTsFacadeBreaches", {
    AR + "📚misc/🟦.ts": ['throw new Error("not wired to WASM yet");'], AR + "⚙engine/🟦.ts": ['throw new Error("not wired to WASM yet");'], AR + "🗣dsl/🟦.ts": ["// wire the parser to plugin WASM"],
    AR + "🎒pack/🟦.ts": ["export const ok = 1;"], AR + "🔸other/🟦.ts": ["// wire the codec to plugin WASM"], "✏s/🔌plugins/🔭p/other/🟦.ts": ['throw new Error("WASM");'],
}, [b("ts-facade-misplaced-stub-" + AR + "📚misc/🟦.ts", FK, 0, "medium"), b("ts-facade-misplaced-stub-" + AR + "🔸other/🟦.ts", FK, 0, "medium")])
PM = "protocol-migration/vcs-shim-regression"
pm_lines = ["use vcs::Mutation;", "// vcs::MutationDiff", "let a = crate::vcs::Edit::new();", "use vcs::{Foo, OpText, Bar};", "use vcs::{Foo, Bar};", " * vcs::Edit", "use vcs::{", "    HubClientFrame,", "};", "vcs::Mutation and vcs::Edit", "vcs::Other::x();"]
case("the-vcs-shim-must-not-come-back", "policyProtocolMigrationBreaches", {"m/shim.rs": pm_lines},
     [b("protocol-migration-m/shim.rs-1", PM, 1), b("protocol-migration-use-m/shim.rs-4", PM, 4), b("protocol-migration-use-m/shim.rs-7", PM, 7), b("protocol-migration-m/shim.rs-10", PM, 10)])
case("grammar-and-protocol-file-allowlists-are-empty", "policyGrammarFileBreaches", {"x.txt": x}, [])
case("protocol-file-allowlist-is-empty", "policyProtocolFileBreaches", {"x.txt": x}, [])
case("document-app-usages-are-extracted-from-source", "policyDocumentAppUsages", {}, scope="scope/x", source=[
    "impl DocumentApp for One {", "    type Snapshot = crate::a::Snap<T>;", "    type Mutation = Mut;", "}", "impl DocumentApp for Half {", "    type Snapshot = S;", "}", "impl DocumentApp for Two {", "    type Mutation = m::M;", "    type Snapshot = S2;", "}"],
     expectList=["scope/x|1|One|Snap|Mut", "scope/x|8|Two|S2|M"])

props = {"scope": {"type": "string"}, "source": {"type": "array", "items": {"type": "string"}}}
rules = ["policyDslCompletenessBreaches", "policyPackCompletenessBreaches", "policyCommandEnvelopeCompletenessBreaches", "policyTsFacadeBreaches", "policyProtocolMigrationBreaches", "policyGrammarFileBreaches", "policyProtocolFileBreaches", "policyDocumentAppUsages"]
schema = std_schema("Completeness Law Corpus", rules, props)
ts = std_ts(
    "🧮 The completeness laws report exactly the DSL, pack, envelope, facade and vcs-shim breaches their language-agnostic corpus names, agree with an independent Python model and reject a malformed corpus.",
    'import { policyCommandEnvelopeCompletenessBreaches, policyDocumentAppUsages, policyDslCompletenessBreaches, policyGrammarFileBreaches, policyPackCompletenessBreaches, policyProtocolFileBreaches, policyProtocolMigrationBreaches, policyTsFacadeBreaches } from "../../🟦.ts";',
    '''  policyDslCompletenessBreaches: (root) => policyDslCompletenessBreaches(root),
  policyPackCompletenessBreaches: (root) => policyPackCompletenessBreaches(root),
  policyCommandEnvelopeCompletenessBreaches: (root) => policyCommandEnvelopeCompletenessBreaches(root),
  policyTsFacadeBreaches: (root) => policyTsFacadeBreaches(root),
  policyProtocolMigrationBreaches: (root) => policyProtocolMigrationBreaches(root),
  policyGrammarFileBreaches: (root) => policyGrammarFileBreaches(root),
  policyProtocolFileBreaches: (root) => policyProtocolFileBreaches(root),
  policyDocumentAppUsages: (_root, row) => policyDocumentAppUsages(row.scope, row.source.join("\\n") + "\\n").map((u) => `${u.scope}|${u.line}|${u.appType}|${u.snapshotType}|${u.mutationType}`),''',
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧮completeness", "🧮completeness", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
