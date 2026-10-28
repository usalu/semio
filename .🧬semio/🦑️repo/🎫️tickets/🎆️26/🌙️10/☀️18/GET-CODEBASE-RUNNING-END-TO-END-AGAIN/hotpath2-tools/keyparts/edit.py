"""🗝️ Edits of batch `keyparts` on a root, starting from the live files: python3 -I edit.py <root> [--tests-from DIR].
Diagnosis switch `SEMIO_REPO_PLAN_KEY_PARTS=<file>`: every plan reuse decision appends one JSON line {key, parts: name -> first 12 hex of the sha256 of the canonical part} to the file, so two runs can be compared part by part when the plan is planned again ('selection, graph, implementation or environment changed')."""
import json, os, re, shutil, sys
root = sys.argv[1]
B = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
live = os.path.dirname(os.path.abspath(__file__))
while not os.path.isdir(os.path.join(live, "🧰️framework")) and live != "/": live = os.path.dirname(live)
FILES = ("🔧️tools/🏗️bazel/♻️plan-reuse/🟦️.ts", "🔧️tools/🏗️bazel/📜️script.ts", "🔭️witness/🧬️schema/🔣️.json")
if os.path.abspath(live) != os.path.abspath(root):
    for rel in FILES:
        os.makedirs(os.path.dirname(B + rel), exist_ok=True)
        shutil.copyfile(live + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/" + rel, B + rel)
def edit(rel, pairs):
    path = B + rel
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:90], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)
R = "🔧️tools/🏗️bazel/♻️plan-reuse/🟦️.ts"
edit(R, [
 ('import {createReadStream} from "node:fs";', 'import {appendFileSync,createReadStream} from "node:fs";'),
 ("/** 🧾️ One thing the planner left in the root of the workspace", '''/** 🔬️ The parts of a plan key, each as the first twelve hex digits of the sha256 of its canonical text: two keys that differ differ in exactly the parts whose digests differ. */
export function bazelPlanKeyPartsV1(parts:Readonly<Record<string,unknown>>):Record<string,string>{return Object.fromEntries(Object.keys(parts).sort().map(name=>[name,createHash("sha256").update(JSON.stringify(sortKeys(parts[name]))).digest("hex").slice(0,12)]));}
/** 📝️ Appends the key and its part digests of one reuse decision to the file the switch `SEMIO_REPO_PLAN_KEY_PARTS` names; nothing without the switch, never an error. */
export function appendBazelPlanKeyPartsV1(file:string|undefined,key:string,parts:Readonly<Record<string,unknown>>,nowMs:number=Date.now()):void{
 if(file===undefined||file==="")return;
 try{appendFileSync(file,JSON.stringify({at:nowMs,pid:process.pid,key,parts:bazelPlanKeyPartsV1(parts)})+"\\n",{mode:0o600});}catch{}
}

/** 🧾️ One thing the planner left in the root of the workspace'''),
])
S = "🔧️tools/🏗️bazel/📜️script.ts"
path = B + S
s = open(path, encoding="utf8").read()
# the plan-reuse import may be one line with many names: add the new names into it
m = re.search(r'^(import \{bazelPlanFilesV1,[^}]*)\}( from "\./♻️plan-reuse/🟦️\.ts";)', s, re.M)
assert m, "import"
s = s.replace(m.group(0), m.group(1) + ",appendBazelPlanKeyPartsV1}" + m.group(2), 1)
k = re.search(r' const key=bazelPlanKeyV1\((\{version:1,root,workspace,.*?preload:hash\(preload\)\})\);', s)
assert k, "key"
s = s.replace(k.group(0), ' const keyParts=' + k.group(1) + ',key=bazelPlanKeyV1(keyParts);appendBazelPlanKeyPartsV1(process.env.SEMIO_REPO_PLAN_KEY_PARTS,key,keyParts);', 1)
open(path, "w", encoding="utf8").write(s)
W = "🔭️witness/🧬️schema/🔣️.json"
edit(W, [('"SEMIO_REPO_PLAN_REUSE",', '"SEMIO_REPO_PLAN_REUSE", "SEMIO_REPO_PLAN_KEY_PARTS",')])
print("edited keyparts", root[-30:])
