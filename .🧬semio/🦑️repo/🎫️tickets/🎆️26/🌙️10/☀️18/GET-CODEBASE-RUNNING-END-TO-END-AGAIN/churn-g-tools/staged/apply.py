#!/usr/bin/env python3
"""Lands the churn-g batch in ONE pass: every target is checked against the hash it had when the batch was prepared (a peer edit since then stops that file, never overwrites it), anchored patches must match exactly once, new files must not exist yet (or equal).
usage: apply.py <scratchpad churn-g dir> [--check]"""
import hashlib
import json
import os
import shutil
import sys

SCRATCH, CHECK = sys.argv[1], "--check" in sys.argv
ROOT = os.getcwd()
F = "🧰️framework"
CHURN = F + "/🔨️modules/📁️filesystem/🧾️observation/🔁️churn"
L = F + "/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
HERE = os.path.dirname(os.path.abspath(__file__))
sha = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
problems, done = [], []


def base_ok(path):
    before = os.path.join(SCRATCH, "before", path)
    return os.path.exists(before) and sha(path) == sha(before)


MINE = {CHURN + "/🌊️races/🟦️.ts", CHURN + "/🧬️schema/🌊️races.json", CHURN + "/🧪️tests/🎲️fuzz/🟦️.ts"}


def copy_over(path, source=None):
    source = source or os.path.join(SCRATCH, "after", path)
    if path in MINE:
        if not os.path.exists(path) or sha(path) != sha(source):
            done.append("mine " + path)
            if not CHECK:
                shutil.copyfile(source, path)
        return
    if not os.path.exists(path):
        done.append("new  " + path)
        if not CHECK:
            os.makedirs(os.path.dirname(path), exist_ok=True)
            shutil.copyfile(source, path)
        return
    if sha(path) == sha(source):
        return
    if not base_ok(path):
        problems.append("moved since prepared (not touched): " + path)
        return
    done.append("copy " + path)
    if not CHECK:
        shutil.copyfile(source, path)


def patch(path, edits):
    text = open(path, encoding="utf-8").read()
    for old, new in edits:
        if new in text and old not in text:
            continue
        if text.count(old) != 1:
            problems.append("anchor x%d in %s: %s" % (text.count(old), path, old[:70]))
            return
        text = text.replace(old, new)
    done.append("edit " + path)
    if not CHECK:
        open(path, "w", encoding="utf-8").write(text)


for path in [CHURN + "/🟦️.ts", CHURN + "/🧾️refusals/🟦️.ts", CHURN + "/🌊️races/🟦️.ts", CHURN + "/🧬️schema/🌊️races.json", CHURN + "/📸️snapshot/🟦️.ts", CHURN + "/🧬️schema/📸️snapshot.json", L + "/📥️inputs/🔏️revision/🟨️.mjs", L + "/🧷️format-stamps/🔄️reexec/🟨️.mjs", L + "/🧷️format-stamps/🧬️schema/🔣️.json"]:
    copy_over(path)

copy_over(L + "/🚪️door/🔄️restart/🟦️.ts", os.path.join(HERE, "door-restart.ts"))
for path in [CHURN + "/🧪️tests/📸️snapshot/🟦️.ts", CHURN + "/🧪️tests/🌊️races/🔁️layers/🟦️.ts"]:
    copy_over(path)
copy_over(CHURN + "/🧪️tests/🎲️fuzz/🟦️.ts", os.path.join(HERE, "fuzz-test.ts"))

refusals = CHURN + "/🧬️schema/🧾️refusals.json"
table = json.load(open(refusals, encoding="utf-8"))
rows = table["definitions"]["Table"]["const"]["classes"]
if not any(row["id"] == "filesystem-race" for row in rows):
    rows.insert([row["id"] for row in rows].index("retries-exhausted") + 1, {
        "id": "filesystem-race", "disposition": "reobserve", "layer": "snapshot", "markers": [], "messages": [],
        "describe": {
            "en": "A raw filesystem error (ENOENT, ENOTDIR, EISDIR, ESTALE, ELOOP; on Windows EBUSY, EPERM, EACCES) of a READ of a workspace path that a peer deleted, renamed, replaced or held, classified once by the race table and only when a directory around the path changed after the process started: a path that is back is read again by the layer that read it, a path that is gone is cured by a new snapshot or a new process and never by reading it again. Printed at the end of a failed child's output, the same table makes the lane run the step again.",
            "de": "Ein roher Dateisystemfehler (ENOENT, ENOTDIR, EISDIR, ESTALE, ELOOP; unter Windows EBUSY, EPERM, EACCES) beim LESEN eines Arbeitsbereichspfads, den ein anderer Prozess löschte, umbenannte, ersetzte oder festhielt, einmal von der Wettlauftabelle eingeordnet und nur dann, wenn sich ein Verzeichnis um den Pfad nach dem Prozessstart änderte: ein Pfad, der wieder da ist, wird von der lesenden Schicht erneut gelesen, ein verschwundener Pfad wird durch einen neuen Schnappschuss oder einen neuen Prozess geheilt und nie durch erneutes Lesen. Steht der Fehler am Ende der Ausgabe eines gescheiterten Kindes, lässt dieselbe Tabelle die Bahn den Schritt erneut ausführen."
        }})
    done.append("json " + refusals)
    if not CHECK:
        open(refusals, "w", encoding="utf-8").write(json.dumps(table, indent=2, ensure_ascii=False) + "\n")

patch(CHURN + "/🧪️tests/🧾️refusals/🟦️.ts", [('"retries-exhausted","lockfile-frozen"', '"retries-exhausted","filesystem-race","lockfile-frozen"')])
patch(L + "/🧷️format-stamps/🧪️tests/🟦️.ts", [
    ("expect(protocol.maximumReexecutions).toBe(3);", "expect(protocol.maximumReexecutions).toBe(6);"),
    ('const door=text("🏗️build","🚪️door","📜️script.ts");expect(door).toContain("runBuildCliWithRestartV1(root,argv)");expect(door).not.toContain("runBuildCliV1(");expect(script).toContain("useProcessWideImplementationPinV1()");expect(script).toContain("export async function runBuildCliWithRestartV1(");',
     'const door=text("🏗️build","🚪️door","📜️script.ts"),restart=text("🏗️build","🚪️door","🔄️restart","🟦️.ts");expect(door).toContain("runBuildWithRestartV1(async()=>{const{runBuildCliV1}=await import(\\"../📜️script.ts\\");");expect(restart).toContain("useProcessWideImplementationPinV1()");expect(restart).toContain("runWithStaleReexecV1(run,");expect(script).toContain("runBuildWithRestartV1(()=>runBuildCliV1(root,argv,environment),argv,environment)");expect(script).toContain("export async function runBuildCliWithRestartV1(");'),
    ('expect(script).toContain("runWithStaleReexecV1(()=>runBuildCliV1(root,argv,environment)");expect(script).toContain("outermost:environment[SCRIPT_PROCESS_INVOCATION_ENV]===undefined");', 'expect(text("🏗️build","🚪️door","🔄️restart","🟦️.ts")).toContain("outermost:environment[invocationEnvironmentV1]===undefined");'),
])

patch(L + "/🚪️door/📜️script.ts", [(
    'if(await runBuildDoorV1(root,argv,process.env)==="full"){const{runBuildCliWithRestartV1}=await import("../📜️script.ts");await profilePhaseV1("cli.run",()=>runBuildCliWithRestartV1(root,argv).catch(',
    'if(await runBuildDoorV1(root,argv,process.env)==="full"){const{runBuildWithRestartV1}=await import("./🔄️restart/🟦️.ts");await profilePhaseV1("cli.run",()=>runBuildWithRestartV1(async()=>{const{runBuildCliV1}=await import("../📜️script.ts");await runBuildCliV1(root,argv);},argv).catch(')])

patch(L + "/📜️script.ts", [
    ('import { runWithStaleReexecV1 } from "./🧷️format-stamps/🔄️reexec/🟨️.mjs";\n', 'import { runBuildWithRestartV1 } from "./🚪️door/🔄️restart/🟦️.ts";\n'),
    ('''  if(!argv.slice(0,3).includes("service"))(await import("./📥️inputs/🔏️revision/🟨️.mjs")).useProcessWideImplementationPinV1();
  const{SCRIPT_PROCESS_INVOCATION_ENV}=await import("../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
  const outcome=await runWithStaleReexecV1(()=>runBuildCliV1(root,argv,environment),{environment,outermost:environment[SCRIPT_PROCESS_INVOCATION_ENV]===undefined});
  if(outcome.status!=="completed")process.exit(outcome.code);
''', '''  await runBuildWithRestartV1(()=>runBuildCliV1(root,argv,environment),argv,environment);
'''),
])

patch(L + "/📥️inputs/🧮️digest/📥️read/🟨️.mjs", [
    ('import{concurrentEditV1,retryOnConcurrentEditV1,staleImplementationV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";',
     'import{concurrentEditV1,retryOnConcurrentEditV1,staleImplementationV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";\nimport{filesystemShapeErrorV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🌊️races/🟦️.ts";\nimport{stableTreeSettleTextV1,stableTreeProtocolV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/📸️snapshot/🟦️.ts";'),
    ('try{await step(scope,()=>lstat(parents.absent??path,{bigint:true}));}catch(error){if(missing(error))return null;throw error;}throw concurrentEditV1("appeared",path,"Source absence changed");}',
     'try{await step(scope,()=>lstat(parents.absent??path,{bigint:true}));}catch(error){if(missing(error)){scope.observed.set(path,null);return null;}throw error;}throw concurrentEditV1("appeared",path,"Source absence changed");}'),
    ('throw concurrentEditV1("identity",parents.blocked.path,"Source absence blocker changed");return null;}', 'throw concurrentEditV1("identity",parents.blocked.path,"Source absence blocker changed");scope.observed.set(path,null);return null;}'),
    ('if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n)throw Error("Single-link regular no-follow Source input required");', 'if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1n||before.size<0n)throw filesystemShapeErrorV1(before,path,"Single-link regular no-follow Source input required");'),
    ('if(retained)await verify();return Object.freeze({sha256,byteLength:size,...(retained?{value}:{})});', 'if(retained)await verify();scope.observed.set(path,exact);return Object.freeze({sha256,byteLength:size,...(retained?{value}:{})});'),
    ('paths:new Set(),cache:new Map(),pending:new Set(),failures:[]},token=Object.freeze({});', 'paths:new Set(),cache:new Map(),observed:new Map(),pending:new Set(),failures:[]},token=Object.freeze({});'),
    ('if(failed)throw failure;check(scope);requireCurrentExactSourceReadV1();return value;});}', 'if(failed)throw failure;check(scope);await verifyObserved(scope);requireCurrentExactSourceReadV1();return value;});}'),
    ('function exactSourceReadBatchV1(root,control,run,inventoryRoots,store){',
     '''const stampTimes=exact=>exact.split(":").slice(5).map(part=>Number(BigInt(part)/1000000n));
/** 🔒️ One batch hashes its inputs from ONE tree state: before the batch is accepted, every file it read is looked at again and must carry the stamp it had when it was read (every absence must still hold), and an entry edited within the settle window is waited out and looked at once more, so an input rewritten halfway through the batch, a writer that paused, a file renamed over another all end in a concurrent edit of the `snapshot` layer and the batch runs again. */
async function verifyObserved(scope){
 const settle=stableTreeProtocolV1().settleMilliseconds,rows=[...scope.observed];
 const look=async()=>{
  let newest=0,newestPath="";
  for(const[path,exact]of rows){
   let row;try{row=await lstat(path,{bigint:true});}catch(error){if(!missing(error))throw error;if(exact!==null)throw concurrentEditV1("vanished",path,"Source vanished while the batch read it","snapshot");continue;}
   if(exact===null)throw concurrentEditV1("appeared",path,"Source appeared while the batch read it","snapshot");
   if(!row.isFile()||row.isSymbolicLink()||stamp(row)!==exact)throw concurrentEditV1(exact.split(":").slice(0,2).join(":")!==[row.dev,row.ino].join(":")?"identity":"bytes",path,"Source changed while the batch read it","snapshot");
   const edited=Math.max(...stampTimes(exact));if(edited>=newest){newest=edited;newestPath=path;}
  }
  return{newest,newestPath};
 };
 const first=await look(),wait=Math.min(settle,first.newest+settle-Date.now());
 if(wait>0){
  try{process.stderr.write(stableTreeSettleTextV1(first.newestPath,Math.max(0,Date.now()-first.newest),wait)+"\\n");}catch{}
  await new Promise(accept=>setTimeout(accept,wait));check(scope);await look();
 }
}
function exactSourceReadBatchV1(root,control,run,inventoryRoots,store){'''),
])

print("\n".join(done))
print("PROBLEMS" if problems else "no problems")
print("\n".join(problems))
sys.exit(1 if problems else 0)
