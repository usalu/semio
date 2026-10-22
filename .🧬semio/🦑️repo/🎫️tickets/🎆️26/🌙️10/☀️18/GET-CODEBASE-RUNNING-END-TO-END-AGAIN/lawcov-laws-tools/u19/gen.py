import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + read("ptree.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
ST = "✏s/🔌plugins/🗄stdio/🗿artifacts"
x = []
cases = []
sub = lambda art, std, name: "%s/%s/🏅standards/%s/🪆subsets/%s" % (ST, art, std, name)


def entry(art, std, name, art_id, slug, sid):
    return {"artRel": ST + "/" + art, "artifactId": art_id, "standardSlug": slug, "subsetRel": sub(art, std, name), "subsetId": sid}


cases.append({"id": "schema-owning-stdio-subsets-are-listed", "rule": "policyListStdioSchemaOwningEntries", "files": {
    sub("🎞avi", "🔖1.0", "✳any") + "/🧬schema/📸snapshot/x": x, sub("🎞avi", "🔖1.0", "✳fem-2d") + "/🧬schema/📸snapshot/x": x, sub("🎞avi", "🔖1.0", "✳plain") + "/🧬schema/🔺diff/x": x,
    sub("🔸wav", "🔖riff-pcm", "✳any") + "/🧬schema/📸snapshot/x": x, "✏s/🔌plugins/🔭p/🗿artifacts/🔸a/🏅standards/🔖1/🪆subsets/✳any/🧬schema/📸snapshot/x": x, sub("🔸bad", "std", "✳any") + "/🧬schema/📸snapshot/x": x,
}, "expect": sorted([entry("🎞avi", "🔖1.0", "✳any", "avi", "1.0", "*"), entry("🎞avi", "🔖1.0", "✳fem-2d", "avi", "1.0", "fem-2d"), entry("🔸wav", "🔖riff-pcm", "✳any", "wav", "riff-pcm", "*")], key=lambda e: e["subsetRel"].encode("utf-16-be"))})
cases.append({"id": "no-stdio-plugin-lists-nothing", "rule": "policyListStdioSchemaOwningEntries", "files": {"README.md": ["x"]}, "expect": []})
for id_, a, s in [("plain", "avi", "1.0"), ("slug-with-hash-free-text", "semio", "v1")]:
    cases.append({"id": "standard-key-" + id_, "rule": "policyStdioStandardKey", "args": [a, s], "expect": "stdio/%s/standards#%s" % (a, s)})
rules = ["policyListStdioSchemaOwningEntries", "policyStdioStandardKey"]
schema = val_schema("Stdio Entries Corpus", rules)
ts = val_ts(
    "📇 The stdio standard-entry listing returns exactly the schema-owning subsets its language-agnostic corpus names over synthetic stdio trees, builds the standard key its corpus names, and agrees with an independent Python model.",
    'import { policyListStdioSchemaOwningEntries, policyStdioStandardKey } from "../../🟦.ts";',
    '''  policyListStdioSchemaOwningEntries: (root) => [...policyListStdioSchemaOwningEntries(root)].sort((a, b) => (a.subsetRel < b.subsetRel ? -1 : a.subsetRel > b.subsetRel ? 1 : 0)),
  policyStdioStandardKey: (_root, artifact: string, slug: string) => policyStdioStandardKey(artifact, slug),''',
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/🧬schema-overhaul-s2/📇stdio-entries", "📇entries", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
