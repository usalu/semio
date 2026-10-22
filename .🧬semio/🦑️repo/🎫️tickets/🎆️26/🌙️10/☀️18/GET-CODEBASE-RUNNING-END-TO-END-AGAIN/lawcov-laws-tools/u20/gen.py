import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts"
cases = []
x = []


def case(id_, rule, files, expect):
    cases.append({"id": id_, "rule": rule, "files": files, "expect": expect})


def b(i, kind, prio="high"):
    return [i, kind, prio, 0]


CU = "stdio-artifacts/codec-id-uniqueness"
A, Bf, C = ST + "/🔸a/🦀.rs", ST + "/🔸b/🦀.rs", ST + "/🔸c/🦀.rs"
case("stdio-codec-ids-are-unique", "policyStdioCodecIdUniquenessBreaches", {
    A: ['const ID_A: &str = "codec.a";', "fn reg() {", "    register_document_codec(store::artifact_codec::<T>(ID_A));", "}"],
    Bf: ['pub const ID_B: &str = "codec.a";', 'register_document_codec(store::artifact_codec::<U<V>>(crate::ids::ID_B));', 'register_document_codec(store::artifact_codec::<W>("codec.b"));'],
    C: ['register_document_codec(store::artifact_codec::<X>("codec.c"));', "register_document_codec(store::artifact_codec::<Y>(UNKNOWN));", 'register_document_codec(store::artifact_codec::<Z>("codec.b"));'],
    "✏s/🔌plugins/🔭p/🦀.rs": ['register_document_codec(store::artifact_codec::<T>("codec.a"));'],
}, [b("stdio-codec-id-duplicate-codec.a-%s-3" % A, CU), b("stdio-codec-id-duplicate-codec.a-%s-2" % Bf, CU), b("stdio-codec-id-duplicate-codec.b-%s-3" % Bf, CU), b("stdio-codec-id-duplicate-codec.b-%s-3" % C, CU)])
case("codec-ids-need-the-stdio-plugin", "policyStdioCodecIdUniquenessBreaches", {"✏s/🔌plugins/🔭p/🦀.rs": ['register_document_codec(store::artifact_codec::<T>("a"));', 'register_document_codec(store::artifact_codec::<T>("a"));']}, [])
RT = "artifact-io/round-trip-test"
E1 = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸a/🏅standards/🔖1/⚙engine/🦀.rs"
E2 = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸b/🏅standards/🔖1/⚙engine/🦀.rs"
E3 = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸c/🏅standards/🔖1/⚙engine/🦀.rs"
E4 = "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸d/🏅standards/🔖1/⚙engine/🦀.rs"
case("engines-carry-a-round-trip-test", "policyRoundTripTestBreaches", {
    E1: ["pub fn parse() {}"], E2: ["#[cfg(test)]", "mod tests {", "    fn round_trip_is_lossless() {}", "}"], E3: ["fn roundtrip() {}"], E4: ["#[cfg(test)]", "mod t { fn LOSSLESS() {} }"],
    "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸e/🏅standards/🔖1/📚misc/🦀.rs": ["pub fn parse() {}"], "✏s/🔌plugins/🗄stdio/🗿artifacts/🔸f/⚙engine/🦀.rs": ["pub fn parse() {}"], "other/🏅standards/🔖1/⚙engine/🦀.rs": ["pub fn parse() {}"],
}, [b("round-trip-test-" + E1, RT), b("round-trip-test-" + E3, RT)])
MV = "artifact-io/mutation-vocabulary"
M = lambda art: "%s/%s/🏅standards/🔖1/🪆subsets/✳any/🧬schema/🧬mutations/🦀.rs" % (ST, art)
case("flagship-mutations-need-real-variants", "policyMutationVocabularyBreaches", {
    M("🎨svg"): ["pub enum SvgMutation {", "    NoMutation,", "    SetSnapshot(S),", "}"],
    M("🧊gltf"): ["pub enum GltfMutation {", "    #[serde(rename = \"x\")]", "    AddNode { id: u32, name: String },", "    NoMutation,", "}"],
    M("🌳pdf"): ["// pub enum PdfMutation { Real }", "pub struct Other;"],
    M("🎞gif"): ["pub enum GifMutation { Frame(u8), NoMutation }"],
    M("🔸other"): ["pub enum OtherMutation { NoMutation }"],
}, [b("mutation-vocabulary-" + M("🎨svg"), MV), b("mutation-vocabulary-" + M("🌳pdf"), MV)])
rules = ["policyStdioCodecIdUniquenessBreaches", "policyRoundTripTestBreaches", "policyMutationVocabularyBreaches"]
schema = std_schema("Dialects And Round Trip Law Corpus", rules)
ts = std_ts(
    "🔁 The stdio codec-id, round-trip-test and flagship mutation-vocabulary laws report exactly the breaches their language-agnostic corpus names over synthetic stdio trees, agree with an independent Python model and reject a malformed corpus.",
    'import { policyMutationVocabularyBreaches, policyRoundTripTestBreaches, policyStdioCodecIdUniquenessBreaches } from "../../🟦.ts";',
    "\n".join("  %s: (root) => %s(root)," % (r, r) for r in rules),
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🚪artifact-io/🔁dialects-and-round-trip", "🔁dialects", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
