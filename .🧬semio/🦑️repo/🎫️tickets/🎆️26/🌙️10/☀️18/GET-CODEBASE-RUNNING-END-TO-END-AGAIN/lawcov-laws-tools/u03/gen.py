import sys, os, hashlib
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
oracle = open(os.path.join(HERE, "..", "pylib.py"), encoding="utf8").read() + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
HI = "high"


def case(id_, rule, files, expect=None, **kw):
    row = {"id": id_, "rule": rule, "files": files}
    if expect is not None:
        row["expect"] = expect
    row.update(kw)
    cases.append(row)


def b(i, kind, line=0, prio=HI):
    return [i, kind, prio, line]


P3 = "policyHandcraftedSpecP3Breaches"
h1 = hashlib.sha256("grammar __NAME__\nstart __START__\nbody = IDENT\n".encode()).hexdigest()[:12]
h2 = hashlib.sha256("protocol __NAME__\nbody = IDENT\n".encode()).hexdigest()[:12]
DK = "handcrafted-grammar/spec-distinctness"
case("distinctness-normalizes-names-and-start", P3, {
    "✏s/📚lib/a.grammar.semio": ["grammar alpha", "start root", "body = IDENT"],
    "✏s/📚lib/b.grammar.semio": ["grammar beta", "start top", "body = IDENT"],
    "🧰framework/📚lib/c.protocol.semio": ["protocol gamma", "body = IDENT"],
    "🧰framework/📚lib/d.protocol.semio": ["protocol delta", "body = IDENT"],
    "🧰framework/📚lib/e.protocol.semio": ["protocol eps", "body = OTHER"],
}, [b("spec-distinctness-%s-✏s/📚lib/a.grammar.semio-✏s/📚lib/b.grammar.semio" % h1, DK), b("spec-distinctness-%s-🧰framework/📚lib/c.protocol.semio-🧰framework/📚lib/d.protocol.semio" % h2, DK)])
GK = "handcrafted-grammar/generic-spec"
case("generic-spec-shapes", P3, {
    "✏s/📚lib/g1.grammar.semio": ["grammar g1", 'prop = IDENT "=" (value)'],
    "✏s/📚lib/g2.grammar.semio": ["grammar g2", "list = item*", "prop = other"],
    "✏s/📚lib/g3.grammar.semio": ["grammar g3", "field-json = STRING"],
    "✏s/📚lib/g4.grammar.semio": ["grammar g4", "stmt = IDENT assign* block?"],
    "✏s/📚lib/g5.grammar.semio": ["grammar g5", "node = IDENT", "list = node*"],
    "🧰framework/📚lib/g6.grammar.semio": ["grammar g6", "field-json = OTHER"],
}, [b("generic-spec-✏s/📚lib/g%d.grammar.semio" % i, GK) for i in (1, 2, 3, 4)])
FAM = "🧰framework/🛍products/💻os/🔨modules/🗣dsl/👪family/📖family-core.grammar.semio"
fam_lines = ["grammar family-core", "core-rule = IDENT", "start x"]
DU = "handcrafted-grammar/declared-use"
case("declared-use-missing-and-unreferenced-family", P3, {
    FAM: fam_lines,
    "✏s/📚lib/u1.grammar.semio": ["grammar u1", "use family-nope", "start m"],
    "✏s/📚lib/u2.grammar.semio": ["grammar u2", "use family-core", "m = other"],
}, [b("declared-use-missing-✏s/📚lib/u1.grammar.semio-family-nope", DU), b("declared-use-✏s/📚lib/u2.grammar.semio-family-core", DU)])
case("declared-use-referenced-production-is-clean", P3, {
    FAM: fam_lines,
    "✏s/📚lib/u3.grammar.semio": ["grammar u3", "use family-core", "m = core-rule"],
}, [], defect="🟦.ts:177 builds the word-boundary RegExp inside a template literal, where the backslash-b escape is a backspace character, so a referenced production is never found")
A1 = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a"
A2 = "✏s/🔌plugins/🔭p/🗿artifacts/🔸b"
WI, WR = "handcrafted-grammar/spec-wiring-include", "handcrafted-grammar/spec-wiring-register"
case("spec-wiring-include-and-register", P3, {
    A1 + "/🗣dsl/📖.grammar.semio": ["grammar a1", "w1 = IDENT"], A1 + "/🗣dsl/🦀.rs": ["pub fn x() {}"],
    A1 + "/🔧op/📖.grammar.semio": ["grammar a1op", "start mutation", "w2 = IDENT"], A1 + "/🔧op/📡.protocol.semio": ["protocol a1op", "w3 = IDENT"],
    A1 + "/🔧op/🦀.rs": ['const G: &str = include_str!("📖.grammar.semio");'],
    A1 + "/📡spr/📡.protocol.semio": ["protocol a1spr", "w4 = IDENT"],
    A2 + "/🗣dsl/📖.grammar.semio": ["grammar b1", "w5 = IDENT"],
    A2 + "/🗣dsl/🦀.rs": ['include_str!("📖.grammar.semio"); register_language("b");'],
    "✏s/🔌plugins/🔭p/🗿artifacts/🔸c/🦀.rs": ["pub fn nothing() {}"],
}, [b("spec-wiring-include-%s/🗣dsl/🦀.rs-📖.grammar.semio" % A1, WI), b("spec-wiring-include-%s/🔧op/🦀.rs-📡.protocol.semio" % A1, WI), b("spec-wiring-register-" + A1, WR)])
EX = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/📚examples/🖼assets"
EK = "handcrafted-grammar/empty-example"
case("empty-example-assets-at-most-64-bytes", P3, {
    EX + "/tiny.semio": ["x"], EX + "/exact64.semio": ["a" * 63], EX + "/big.semio": ["a" * 64],
    "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/📚examples/other/tiny.semio": ["x"], "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/notes/🖼assets/tiny.semio": ["x"],
    "📚examples/🖼assets/root.semio": ["y"],
}, [b("empty-example-" + EX + "/tiny.semio", EK), b("empty-example-" + EX + "/exact64.semio", EK), b("empty-example-📚examples/🖼assets/root.semio", EK)])
CK = "handcrafted-grammar/generic-codec-derive"
CR = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🦀.rs"
case("generic-codec-runtime-calls", P3, {
    CR: ["use dsl::__rt::parse_document_record;", "let a = dsl::op_rt::encode(x);", "let b = store::op_rt::decode(y); let c = dsl::__rt::print_inline_record(z);", "let fine = dsl::typed::decode(y);"],
    "✏s/🔌plugins/🔭p/other/🦀.rs": ["let a = dsl::op_rt::encode(x);"],
}, [b("generic-codec-runtime-%s-1" % CR, CK, 1), b("generic-codec-runtime-%s-2" % CR, CK, 2), b("generic-codec-runtime-%s-3" % CR, CK, 3), b("generic-codec-runtime-%s-3" % CR, CK, 3)])
case("empty-example-direct", "policyEmptyExampleBreaches", {EX + "/tiny.semio": ["x"], EX + "/big.semio": ["a" * 70]}, [b("empty-example-" + EX + "/tiny.semio", EK)])

PD = "plugin-dependency/parity"
OA, OB = "✏s/🔌plugins/🔭alpha", "✏s/🔌plugins/🔭beta"
case("plugin-dependency-parity-between-code-and-metadata", "policyPluginDependencyParityBreaches", {
    OA + "/🦀.rs": ['.depends_on("beta")', '.depends_on("gamma")'], OA + "/🗿artifacts/🔸x/🦀.rs": ['.depends_on("beta")'],
    OA + "/📦packages/🦀rust/Cargo.toml": ["[package]", 'name = "alpha"', "", "[package.metadata.semio]", 'extends = "gamma"', 'depends-on = ["beta", "delta", "alpha"]'],
    OB + "/🦀.rs": ['.depends_on("zeta")'],
    OA + "/🧩extensions/🔸ext/🦀.rs": ['.depends_on("alpha")'],
    OA + "/🧩extensions/🔸ext/📦packages/🦀rust/Cargo.toml": ["[package.metadata.semio]", "depends-on = []"],
    "✏s/🔌plugins/.hidden/🦀.rs": ['.depends_on("ghost")'],
}, [b("plugin-dependency-undeclared-runtime-%s-delta" % OA, PD), b("plugin-dependency-undeclared-metadata-%s-zeta" % OB, PD), b("plugin-dependency-undeclared-metadata-%s/🧩extensions/🔸ext-alpha" % OA, PD)])
CT = "plugin-dependency/contribution-target"
OC = "✏s/🔌plugins/🔭gamma"
case("contribution-targets-must-be-declared-and-canonical", "policyContributionTargetBreaches", {
    OC + "/🦀.rs": ['.depends_on("alpha")', 'ArtifactContribution::builder("s.alpha.kind")', 'ArtifactContribution::builder("s.zzz.kind")', 'ArtifactContribution::builder("nodots")'],
}, [b("contribution-target-undeclared-%s-s.zzz.kind" % OC, CT), b("contribution-target-ungrammatical-%s/🦀.rs-nodots" % OC, CT)])
SV = "mutation-migration/semantic-vocabulary"
VP = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/"
case("semantic-vocabulary-banned-and-bare-set", "policySemanticVocabularyBreaches", {
    VP + "🧬mutations/🔹m/🦀.rs": ["pub enum E { SetSnapshot(S), NoMutation }"],
    VP + "🧬mutations/🔹n/🦀.rs": ["pub enum E { SetColor, SetSize }"],
    VP + "🎮commands/🔹c/🦀.rs": ["CollectionMutation<T> and SetSnapshotX"],
    VP + "🧬mutations/🔹m/notes.txt": ["NoMutation"],
    "🌎hub/🎮commands/x/🦀.rs": ["NoMutation"],
    "other/🎮commands/x/🦀.rs": ["NoMutation"],
}, [b("semantic-vocabulary-banned-" + VP + "🧬mutations/🔹m/🦀.rs", SV), b("semantic-vocabulary-bare-set-" + VP + "🧬mutations/🔹n/🦀.rs", SV, 0, "medium"),
    b("semantic-vocabulary-banned-" + VP + "🎮commands/🔹c/🦀.rs", SV), b("semantic-vocabulary-bare-set-" + VP + "🎮commands/🔹c/🦀.rs", SV, 0, "medium"), b("semantic-vocabulary-banned-🌎hub/🎮commands/x/🦀.rs", SV)])
DE = "taxonomy/dead-example-leaf"
EXR = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/📚examples"
case("dead-example-leaves-follow-glue-path-declarations", "policyDeadExampleLeafBreaches", {
    "✏s/🔌plugins/🔭p/🦀.rs": ['#[path = "🗿artifacts/🔸a/📚examples/ref.rs"]', "mod ref_example;", '#[path = "🗿artifacts/🔸a/📚examples"]', "mod examples {", "    mod nested;", "}"],
    EXR + "/ref.rs": ["fn a() {}"], EXR + "/nested.rs": ["fn b() {}"], EXR + "/dead.rs": ["fn c() {}"], "🧰framework/📚examples/fw.rs": ["fn d() {}"],
}, [b("dead-example-leaf-" + EXR + "/dead.rs", DE), b("dead-example-leaf-🧰framework/📚examples/fw.rs", DE)])
PO = "✏s/🔌plugins/🔭p"
case("owner-own-component-files-skip-extension-owners", "policyOwnerOwnComponentFiles", {
    PO + "/🦀.rs": [], PO + "/🗿artifacts/🔸a/🦀.rs": [], PO + "/🧩extensions/🔸e/🦀.rs": [], PO + "/📦packages/🦀rust/lib.rs": [], PO + "/🗿artifacts/🔸a/🧩extensions/y/🦀.rs": [],
}, ownerRel=PO, expectList=[PO + "/🦀.rs", PO + "/🗿artifacts/🔸a/🦀.rs"])
OG = "mutation-migration/op-grammar-start"
CF = "taxonomy/command-folder-semantic"
AG = "policyMutationArtifactEngineBreaches"
CD = "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🎮commands"
case("op-grammar-start-and-command-folder-verbs-in-a-git-admitted-tree", AG, {
    A1 + "/🔧op/📖.grammar.semio": ["grammar g", "start operation"], A1 + "/🔧op/🔸x/📖.grammar.semio": ["grammar h"], A1 + "/🔧op/🔹ok/📖.grammar.semio": ["grammar i", "start mutation"],
    A1 + "/🗣dsl/📖.grammar.semio": ["grammar j", "start operation"],
    CD + "/🔹create-node/🦀.rs": [], CD + "/🔹frobnicate/🦀.rs": [], CD + "/🔹Create/🦀.rs": [], CD + "/plain-add/🦀.rs": [], CD + "/🔹set-title/🦀.rs": [], CD + "/🦀.rs": [],
}, [b("op-grammar-start-mutation-" + A1 + "/🔧op/📖.grammar.semio", OG), b("op-grammar-start-mutation-" + A1 + "/🔧op/🔸x/📖.grammar.semio", OG),
    b("command-folder-semantic-%s/🔹frobnicate" % CD, CF, 0, "medium"), b("command-folder-semantic-%s/🔹Create" % CD, CF, 0, "medium"), b("command-folder-semantic-%s/plain-add" % CD, CF, 0, "medium")], git=True)

rules = ["policyHandcraftedSpecP3Breaches", "policyEmptyExampleBreaches", "policyPluginDependencyParityBreaches", "policyContributionTargetBreaches", "policySemanticVocabularyBreaches", "policyDeadExampleLeafBreaches", "policyOwnerOwnComponentFiles", "policyMutationArtifactEngineBreaches"]
schema = std_schema("Handcrafted Mutation Law Corpus", rules, {"ownerRel": {"type": "string"}})
ts = std_ts(
    "✍ The handcrafted-grammar and mutation-migration laws report exactly the breaches their language-agnostic corpus names, agree with an independent Python model, and one recorded word-boundary defect is pinned as a failing row.",
    'import { policyContributionTargetBreaches, policyDeadExampleLeafBreaches, policyEmptyExampleBreaches, policyHandcraftedSpecP3Breaches, policyMutationArtifactEngineBreaches, policyOwnerOwnComponentFiles, policyPluginDependencyParityBreaches, policySemanticVocabularyBreaches } from "../../🟦.ts";',
    '''  policyHandcraftedSpecP3Breaches: (root) => policyHandcraftedSpecP3Breaches(root),
  policyEmptyExampleBreaches: (root) => policyEmptyExampleBreaches(root),
  policyPluginDependencyParityBreaches: (root) => policyPluginDependencyParityBreaches(root),
  policyContributionTargetBreaches: (root) => policyContributionTargetBreaches(root),
  policySemanticVocabularyBreaches: (root) => policySemanticVocabularyBreaches(root),
  policyDeadExampleLeafBreaches: (root) => policyDeadExampleLeafBreaches(root, []),
  policyOwnerOwnComponentFiles: (root, row) => policyOwnerOwnComponentFiles(root, row.ownerRel),
  policyMutationArtifactEngineBreaches: (root) => policyMutationArtifactEngineBreaches(root).filter((b) => b.kind === "mutation-migration/op-grammar-start" || b.kind === "taxonomy/command-folder-semantic"),''',
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/✍handcrafted-mutation", "✍handcrafted", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
