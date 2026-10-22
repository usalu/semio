import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
unit = real_unit("⚖laws/🧪self-test-lanes")
src = open(os.path.join(unit, fix("🟦.ts")), encoding="utf8").read()
ns = {}
code = open(os.path.join(HERE, "oracle.py"), encoding="utf8").read().split("cases = json.load")[0]
code = re.sub(r"^SOURCE = .*$", "", code, flags=re.M)
ns = {"re": re, "os": os, "json": json, "sys": sys, "SOURCE": os.path.join(unit, fix("🟦.ts"))}
exec(code, ns)
snapshot = ns["members"]()
cases = [{"id": "self-test-lane-signatures", "rule": "lanes", "expect": snapshot}]
schema = val_schema("Self Test Lanes Corpus", ["lanes"])
ts_test = val_ts(
    "🧪 The self-test lane contract lists exactly the lane signatures its language-agnostic corpus pins, as read by both an independent Python parse and the TypeScript compiler API, so a lane cannot be added, renamed or re-typed unnoticed.",
    'import { readFileSync } from "node:fs";\nimport ts from "typescript";',
    '  lanes: () => compilerLanes(),',
)
ts_test = ts_test.replace("const rules:", '''const compilerLanes = () => {
  const source = ts.createSourceFile("lanes.ts", readFileSync(resolve(import.meta.dir, "../../🟦.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const rows: { name: string; params: string[][]; returns: string }[] = [];
  source.forEachChild((node) => {
    if (!ts.isInterfaceDeclaration(node) || node.name.text !== "SelfTestLanes") return;
    for (const member of node.members) {
      if (!ts.isPropertySignature(member) || !member.type || !ts.isFunctionTypeNode(member.type)) continue;
      rows.push({ name: member.name.getText(source), params: member.type.parameters.map((p) => [p.name.getText(source), p.type?.getText(source) ?? ""]), returns: member.type.type.getText(source) });
    }
  });
  return rows;
};
const rules:''', 1)
folder, plen = write_test("⚖laws/🧪self-test-lanes", "🧪lanes", ts_test, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(snapshot))
