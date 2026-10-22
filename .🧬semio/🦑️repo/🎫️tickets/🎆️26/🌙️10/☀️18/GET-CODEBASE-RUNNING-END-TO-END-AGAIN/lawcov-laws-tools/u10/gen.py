import sys, os
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
snapshot = json.load(open("/private/tmp/lawcov-laws/p10.json", encoding="utf8"))
cases = [{"id": name, "rule": "allowlist", "args": [name], "expect": value} for name, value in sorted(snapshot.items())]
schema = val_schema("Plugin Consistency Allowlist Corpus", ["allowlist"])
extra = '''
test("every exported allowlist is pinned by the corpus and no list repeats an entry", () => {
  expect(Object.keys(allowlists).sort()).toEqual((corpus.cases as any[]).map((row) => row.args[0]).sort());
  for (const row of corpus.cases as any[]) expect(new Set(row.expect.values).size, row.id).toBe(row.expect.values.length);
});
'''
ts = val_ts(
    "📋 The plugin-consistency allowlists equal the entries their language-agnostic corpus pins, agree with an independent Python parse of the source text, and every exported list is covered so a new exemption must be added to the corpus on purpose.",
    'import * as allowlists from "../../🟦.ts";',
    '''  allowlist: (_root, name: string) => { const value = (allowlists as Record<string, unknown>)[name]; return value instanceof Set ? { kind: "set", values: [...value].sort() } : Array.isArray(value) ? { kind: "array", values: value } : { kind: "string", values: [value] }; },''',
    extra,
)
folder, plen = write_test("⚖laws/🧩plugin-consistency/📋allowlists", "📋allowlists", ts, {"version": 1, "cases": cases}, schema, oracle, fixdeep=False)
print(folder, plen, len(cases))
