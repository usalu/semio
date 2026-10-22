import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
PL = "✏s/🔌plugins/🔭p"
ART = PL + "/🗿artifacts/🔸a"
STD = ART + "/🏅standards/🔖1"
SUB = STD + "/🪆subsets/✳any"
GLUE = PL + "/📦packages/🦀rust/🦀.rs"
RULE = "policyCleanArtifactStandardSubsetMechanismBreaches"
K = "taxonomy/owner-shape/"
cases = []


def base():
    return {
        GLUE: ['#[path = "../../🗿artifacts/🔸a/🦀.rs"]', "mod a;"],
        ART + "/🦀.rs": ['#[path = "🏅standards/🔖1/🦀.rs"]', "mod s;"],
        STD + "/🦀.rs": ['#[path = "🪆subsets/✳any/🦀.rs"]', "mod u;"],
        SUB + "/🦀.rs": ['#[path = "🧬schema/🦀.rs"]', "mod schema;"],
        SUB + "/🧬schema/🦀.rs": ["pub struct ASnapshot {}"],
    }


def row(i, sub, line=0):
    return [i, K + sub, "medium", line]


def add(id_, files, expect, drop=()):
    tree = base()
    for d in drop:
        tree.pop(d)
    tree.update(files)
    cases.append({"id": id_, "rule": RULE, "files": tree, "expect": expect})


add("clean-base-tree", {}, [])
add("missing-owner-root-files", {}, [row("owner-mounts-children-missing-root-" + STD, "owner-mounts-children"), row("owner-mounts-children-missing-root-" + SUB, "owner-mounts-children")], drop=[STD + "/🦀.rs", SUB + "/🦀.rs"])
add("owner-with-children-must-mount-them", {STD + "/🦀.rs": ["pub fn nothing() {}"]}, [row("owner-mounts-children-not-self-mounting-" + STD, "owner-mounts-children")])
add("plugin-glue-scope-and-shim", {GLUE: ['#[path = "../../🗿artifacts/🔸a/🏅standards/🔖1/🦀.rs"]', "mod deep;", "pub use crate::artifacts::a::standards::v1;"]},
    [row("owner-mounts-children-glue-scope-" + PL, "owner-mounts-children"), row("owner-mounts-children-glue-shim-" + PL, "owner-mounts-children")])
rs = ["use crate::artifacts::a::standards::v1::subsets::other::X;", "use crate::artifacts::a::standards::v2::thing::Y;", "use crate::artifacts::a::standards::v1::subsets::any::Z;"]
ts = ['import x from "../../🧱x/y";', 'import m from "../../🔨modules/y";', 'import d from "./local";', 'import u from "../up";']
add("subset-isolation-across-subsets-standards-and-ts-climbs", {SUB + "/⚙engine/🦀.rs": rs, SUB + "/⚙engine/🟦.ts": ts}, [
    row("subset-isolation-cross-subset-%s/⚙engine/🦀.rs-%d" % (SUB, "\n".join(rs).index("artifacts::")), "subset-isolation", 1),
    row("subset-isolation-cross-standard-%s/⚙engine/🦀.rs-%d" % (SUB, len(rs[0]) + 1 + rs[1].index("artifacts::")), "subset-isolation", 2),
    row("subset-isolation-ts-climb-%s/⚙engine/🟦.ts-%d" % (SUB, ts[0].index("from")), "subset-isolation", 1),
])
add("module-units-need-two-consumers", {
    PL + "/🔨modules/🔸util/🦀.rs": [], PL + "/🔨modules/🔸lonely/🦀.rs": [], PL + "/🔨modules/🔸once/🦀.rs": [], SUB + "/🔨modules/🔸inner/🦀.rs": [],
    SUB + "/⚙engine/🦀.rs": ["use modules::util::a;", "use modules::once::b;"], STD + "/⚙runtime/🦀.rs": ["use modules::util::c;"],
}, [row("module-consumer-count-" + PL + "/🔨modules/🔸once", "module-consumer-count"), row("module-consumer-count-" + PL + "/🔨modules/🔸lonely", "module-consumer-count"),
    row("module-consumer-subset-level-" + SUB + "/🔨modules/🔸inner", "module-consumer-count")])
add("io-calls-outside-the-io-owner", {
    ART + "/⚙engine/🦀.rs": ["fn a() { parse_dsl(x); }", "fn b() { let _ = std::fs::read(p); }", 'const D: &[u8] = include_bytes!("x");', "use semio_s_plugin_other::foo::io;", "#[cfg(test)]", "mod t {", "    fn z() { encode_pack(1); }", "}"],
    ART + "/🚪io/x.rs": ["fn a() { parse_dsl(x); }"], ART + "/🧪tests/t.rs": ["fn a() { parse_dsl(x); }"],
}, [row("io-exclusivity-%s/⚙engine/🦀.rs-%s" % (ART, label), "io-exclusivity", line) for line, label in ((1, "parse_dsl("), (2, "std::fs::"), (3, "include_bytes!"), (4, "semio_s_plugin_<other>::…::io"))])
add("io-leaves-must-be-declared-and-have-a-ts-twin", {
    SUB + "/🚪io/🦀.rs": ["mod 🔸leafa;", "mod 🔸leafc;"], SUB + "/🚪io/🔸leafa/🦀.rs": [], SUB + "/🚪io/🔸leafb/🦀.rs": [], SUB + "/🚪io/🔸leafb/🟦.ts": [],
    SUB + "/🚪io/🔸leafc/🦀.rs": [], SUB + "/🚪io/🔸leafc/🟦.ts": [],
}, [row("io-declaration-missing-ts-twin-" + SUB + "/🚪io/🔸leafa", "io-declaration"), row("io-declaration-unreachable-" + SUB + "/🚪io/🔸leafb/🦀.rs", "io-declaration")])
sa = ["pub use crate::artifacts::a::standards::v1::subsets::other::*;", "pub use crate::artifacts::a::standards::v1::subsets::any::*;"]
add("subset-schema-must-stand-alone", {SUB + "/🧬schema/🦀.rs": sa}, [
    row("subset-standalone-bare-reexport-%s/🧬schema/🦀.rs-0" % SUB, "subset-standalone", 1), row("subset-standalone-no-own-snapshot-" + SUB, "subset-standalone"),
    row("subset-isolation-cross-subset-%s/🧬schema/🦀.rs-%d" % (SUB, sa[0].index("artifacts::")), "subset-isolation", 1)])
add("declaration-tree-must-name-its-children", {
    ART + "/🦀.rs": ["pub fn artifact() -> ArtifactDeclaration {}"],
    STD + "/🦀.rs": ["pub fn standard() -> StandardDeclaration {}", '#[path = "🪆subsets/✳x/🦀.rs"]'],
    SUB + "/🦀.rs": ["pub fn subset() -> SubsetDeclaration {}", '#[path = "🧬schema/🦀.rs"]'],
}, [row("declaration-tree-artifact-missing-standard-%s-v1" % ART, "declaration-tree"), row("declaration-tree-standard-missing-subset-%s-any" % STD, "declaration-tree"), row("declaration-tree-plugin-missing-artifact-%s-a" % PL, "declaration-tree")])

schema = std_schema("Clean Mechanism Law Corpus", [RULE])
tsx = std_ts(
    "🧼 The clean artifact/standard/subset mechanism law reports exactly the owner-shape breaches its language-agnostic corpus names over synthetic plugin trees, agrees with an independent Python model and rejects a malformed corpus.",
    'import { policyCleanArtifactStandardSubsetMechanismBreaches } from "../../🟦.ts";',
    "  policyCleanArtifactStandardSubsetMechanismBreaches: (root) => policyCleanArtifactStandardSubsetMechanismBreaches(root),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧼clean-mechanism", "🧼mechanism", tsx, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
