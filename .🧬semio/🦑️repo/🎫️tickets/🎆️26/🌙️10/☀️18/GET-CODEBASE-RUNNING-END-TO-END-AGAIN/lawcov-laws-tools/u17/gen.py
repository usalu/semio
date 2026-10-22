import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("foundation.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
cases = []
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts"
RULE = "policySchemaOverhaulPCBreaches"
K = "stdio-artifacts/"
x = []


def case(id_, files, expect, only=None):
    row = {"id": id_, "rule": RULE, "files": files, "expect": expect}
    if only:
        row["kindFilter"] = K + only
    cases.append(row)


def b(i, kind, prio="medium"):
    return [i, K + kind, prio, 0]


def sub(art, std="🔖2", name="✳any"):
    return "%s/%s/🏅standards/%s/🪆subsets/%s" % (ST, art, std, name)


S = sub("🔸foo")
own = {S + "/🧬schema/📸snapshot/x": x}
real_g = ["dialect grammar", "grammar g", "start s", "rule = x"]
case("grammar-dialects-must-be-parseable", {
    **own, S + "/🧬schema/📸snapshot/📝text/📖.grammar.semio": real_g, S + "/🧬schema/🔺diff/📝text/📖.grammar.semio": ["; abnf comment", "dialect grammar", "grammar g", "start s"],
    S + "/🧬schema/🧬mutations/📝text/📖.grammar.semio": ["# note", "grammar g", "start s"],
    "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🏅standards/🔖2/🪆subsets/✳any/🧬schema/📸snapshot/x": x, "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🏅standards/🔖2/🪆subsets/✳any/🧬schema/🔺diff/📝text/📖.grammar.semio": ["bad"],
    sub("🔸bar") + "/🧬schema/🔺diff/📝text/📖.grammar.semio": ["bad"],
}, [b("grammar-parseability-%s/🧬schema/🔺diff/📝text/📖.grammar.semio" % S, "grammar-parseability"), b("grammar-parseability-%s/🧬schema/🧬mutations/📝text/📖.grammar.semio" % S, "grammar-parseability")], only="grammar-parseability")
case("protocol-dialects-must-be-parseable", {
    **own, S + "/🧬schema/📸snapshot/💾binary/📡.protocol.semio": ["dialect protocol", "protocol p", "start s"], S + "/🧬schema/🔺diff/💾binary/📡.protocol.semio": ["dialect protocol", "protocol p", "start s", "x = %x41"],
    S + "/🧬schema/🧬mutations/💾binary/📡.protocol.semio": ["dialect grammar", "grammar p", "start s"],
}, [b("protocol-parseability-%s/🧬schema/🔺diff/💾binary/📡.protocol.semio" % S, "protocol-parseability"), b("protocol-parseability-%s/🧬schema/🧬mutations/💾binary/📡.protocol.semio" % S, "protocol-parseability")], only="protocol-parseability")
demo = lambda art: ST + "/" + art + "/📚examples/🎬demo/🖼assets/"
case("stdio-fixtures-are-honest-demos", {
    demo("🔸mine") + "🗣.dsl.semio": ["semio stdio.mine v1"], demo("🔸mine") + "🎒.pack.semio": x,
    demo("🔸bad") + "🎒.pack.semio": x, demo("🔸badtwo") + "🗣.dsl.semio": ["semio stdio.other"], demo("🔸badtwo") + "🎒.pack.semio": x,
    ST + "/🔸avi/x": x, demo("🔸html") + "🗣.dsl.semio": ["semio stdio.html v1"], demo("🔸html") + "🎒.pack.semio": x,
}, [b("fixture-honesty-%s/🔸bad" % ST, "fixture-honesty"), b("fixture-honesty-%s/🔸badtwo" % ST, "fixture-honesty"), b("fixture-honesty-stale-%s/🔸html" % ST, "fixture-honesty", "low")], only="fixture-honesty")
calls = lambda n: ["register_language(%d);" % i for i in range(n)]
case("every-stdio-standard-registers-its-languages", {
    sub("🔸lang", "🔖3") + "/🧬schema/📸snapshot/x": x, sub("🔸lang", "🔖3", "✳other") + "/🧬schema/📸snapshot/x": x, ST + "/🔸lang/🏅standards/🔖3/⚙engine/🦀.rs": calls(4),
    sub("🔸ok", "🔖1") + "/🧬schema/📸snapshot/x": x, ST + "/🔸ok/🏅standards/🔖1/⚙engine/🦀.rs": calls(5) + calls(2),
    sub("🔸avi", "🔖1.0") + "/🧬schema/📸snapshot/x": x, ST + "/🔸avi/🏅standards/🔖1.0/⚙engine/🦀.rs": calls(5),
    sub("🔸epw", "🔖energyplus") + "/🧬schema/📸snapshot/x": x, sub("🔸none", "🔖9") + "/🧬schema/📸snapshot/x": x,
}, [b("language-registration-stdio/lang/standards#3", "language-registration"), b("language-registration-stale-stdio/avi/standards#1.0", "language-registration", "low"), b("language-registration-stdio/none/standards#9", "language-registration")], only="language-registration")
JB = ST + "/🔸j"
case("stdio-codecs-do-not-transfer-through-json", {
    JB + "/⚙engine/🦀.rs": ["impl ArtifactPack for X {", "    fn pack(&self) -> Vec<u8> { serde_json::to_vec(self).unwrap() }", "}"],
    JB + "/⚙engine/ok.rs": ["impl OpBinary for Y {", "    fn a(&self) { binary() }", "}", "fn free() { serde_json::to_vec(1); }"],
    JB + "/🚪io/🦀.rs": ["fn x() { serde_json::to_string(&1); }"], JB + "/🚪io/clean.rs": ["fn x() { binary(); }"],
    "✏s/🔌plugins/🔭p/⚙engine/🦀.rs": ["impl ArtifactPack for X {", "    fn pack(&self) { serde_json::to_vec(self) }", "}"],
}, [b("stdio-json-transfer-ban-%s/⚙engine/🦀.rs" % JB, "json-transfer-ban"), b("stdio-json-transfer-ban-%s/🚪io/🦀.rs" % JB, "json-transfer-ban")], only="json-transfer-ban")
case("unfiltered-aggregate-of-one-schema-owning-artifact", {**own, S + "/🧬schema/🔺diff/📝text/📖.grammar.semio": ["bad"]}, [b("grammar-parseability-%s/🧬schema/🔺diff/📝text/📖.grammar.semio" % S, "grammar-parseability"), b("fixture-honesty-%s" % (ST + "/🔸foo"), "fixture-honesty"), b("language-registration-stdio/foo/standards#2", "language-registration")])

AVI = sub("🔸avi", "🔖1.0")
cases.append({"id": "allowlisted-avi-diff-grammar-is-exempt", "rule": RULE, "kindFilter": K + "grammar-parseability", "oracle": False, "files": {AVI + "/🧬schema/📸snapshot/x": x, AVI + "/🧬schema/🔺diff/📝text/📖.grammar.semio": ["garbage"]}, "expect": [],
              "defect": "🟦.ts:70-131 and 175-236 and 437-468 key their allowlists on the legacy leaf name ending in -component.* while the checked leaf normalises to a bare trailing dash, so no allowlist entry can ever match or go stale"})

schema = std_schema("Schema Overhaul Plugin Consistency Corpus", [RULE])
ts = std_ts(
    "📐 The stdio schema-overhaul law reports exactly the grammar, protocol, fixture, language-registration and JSON-transfer breaches its language-agnostic corpus names over synthetic stdio trees, agrees with an independent Python model and rejects a malformed corpus.",
    'import { policySchemaOverhaulPCBreaches } from "../../🟦.ts";',
    "  policySchemaOverhaulPCBreaches: (root) => policySchemaOverhaulPCBreaches(root),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/📐schema-overhaul-pc", "📐overhaul", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
