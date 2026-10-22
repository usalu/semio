import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts"
GH = "stdio-artifacts/grammar-honesty"
cases = []
leaves = [("🅰.g4", "DOCUMENT: 'schema' [ ]+"), ("🔤.ebnf", "header = 'schema', space,"), ("📖.grammar.semio", "payload = *OCTET"), ("🔠.abnf", "payload = *OCTET"), ("📡.protocol.semio", "payload = *OCTET"), ("🥋.ksy", "size-eos: true"), ("🌶.spicy", "payload: bytes &eod;")]
files, expect = {}, []
for i, (leaf, marker) in enumerate(leaves):
    placeholder = "%s/🔸a%d/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/📝text/%s" % (ST, i, leaf)
    honest = "%s/🔸b%d/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/📝text/%s" % (ST, i, leaf)
    files[placeholder] = ["header", marker, "footer"]
    files[honest] = ["a real grammar body"]
    expect.append([ "grammar-honesty-" + placeholder, GH, "low", 0])
files[ST + "/🔸c/notes.txt"] = ["payload = *OCTET"]
files["✏s/🔌plugins/🔭p/🔠.abnf"] = ["payload = *OCTET"]
cases.append({"id": "placeholder-grammar-leaves-are-reported", "rule": "policyGrammarHonestyBreaches", "files": files, "expect": expect})
AVI = ST + "/🔸bcf/🏅standards/🔖2.1/🪆subsets/✳any/🧬schema/🔺diff/💾binary/🔠.abnf"
cases.append({"id": "a-grammar-allowlisted-by-intent-is-exempt", "rule": "policyGrammarHonestyBreaches", "files": {AVI: ["payload = *OCTET"]}, "expect": [], "oracle": False,
              "defect": "🟦.ts:30-384 keys 353 allowlist entries on the legacy leaf naming ending in -component.<ext>, while the walked leaves normalise to a bare trailing dash, so no allowlist entry can ever match or go stale"})
schema = std_schema("Grammar Honesty Law Corpus", ["policyGrammarHonestyBreaches"])
ts = std_ts(
    "🧱 The grammar-honesty law reports exactly the placeholder dialect leaves its language-agnostic corpus names, agrees with an independent Python model, rejects a malformed corpus, and one recorded dead-allowlist defect is pinned as a failing row.",
    'import { policyGrammarHonestyBreaches } from "../../🟦.ts";',
    "  policyGrammarHonestyBreaches: (root) => policyGrammarHonestyBreaches(root),",
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧬schema-overhaul-s2/🧱grammar-honesty", "🧱honesty", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
