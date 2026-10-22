#!/usr/bin/env python3
"""Applies the boot-fast set on top of round 6 (repo-launcher and hot-cache sets applied; perf-slo applied for group `slo`).

Groups: `door` (front door module, policy, schema, tests, startup law, zero-touch closure row), `manifest` (the root package.json `repo` script runs the door; anchored on the line manifest-neutral writes, so it lands in the lock and sync wave after the door exists), `launcher` (the launcher registry starts the door instead of the build script; the Go entry-contract test follows), `caller` (the build caller loads the Cargo execution contract only for native owner commands, and its declared-selection check consumes the same global options as the invocation parser, so `--lang`, `--service` and `--graph-policy` no longer end in `Undeclared original Repo build selection`), `validator` (the shared relative path pattern of the build owner registry validates in one pass: 619 ms -> 25-40 ms per process for every `repo run <project:target>`), `taxonomy` (the test phase lists of the taxonomy are parsed once per file identity), `slo` (objective `cli-help-p50` of the performance table and its source value), `all` (default).
Dry run by default (`--dry-run` is the explicit spelling); `--write` applies all or nothing with rollback. A create refuses when its target exists with other content; an edit refuses when its anchor or its JSON path is missing or ambiguous (drift) and reports `already` when the new state is present; every JSON result is parsed before anything is written; every file is read again right before it is replaced so that a concurrent editor is never overwritten; a destination path longer than 255 characters is refused.
"""
import argparse,json,os,re,sys,tempfile

here=os.path.dirname(os.path.abspath(__file__))
ROOT=os.path.abspath(os.path.join(here,*[".."]*9))
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B=L+"/🏗️build"
D=B+"/🚪️door"
LAUNCHER="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher"
BENCH=B+"/📏️incremental/🏎️bench"
VALIDATOR="🧰️framework/🔨️modules/🧬️schema/✅️validator"
CREATES={
 "door":{
  "door-readme.md":D+"/README.md","door.ts":D+"/🟦️.ts","door-script.ts":D+"/📜️script.ts","door-policy.json":D+"/🔣️.json","door-schema.json":D+"/🧬️schema/🔣️.json",
  "door-test.ts":D+"/🧪️tests/🟦️.ts","door-fixture.json":D+"/🧪️tests/🧫️fixtures/🔣️.json","startup-law.ts":D+"/🧪️tests/🚦️startup/🟦️.ts","startup-law.json":D+"/🧪️tests/🚦️startup/🔣️.json",
 },
 "validator":{"validator-patterns-test.ts":VALIDATOR+"/🧪️tests/🏎️patterns/🟦️.ts","validator-patterns.json":VALIDATOR+"/🧪️tests/🏎️patterns/🧫️fixtures/🔣️.json","validator-oracle.py":VALIDATOR+"/🧪️tests/🐍️oracle/🐍️.py"},
 "caller":{"caller-globals-test.ts":B+"/📥️caller/🧪️tests/🌐️globals/🟦️.ts"},
 "taxonomy":{"taxonomy-test.ts":B+"/🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts"},
}

class Drift(Exception):pass

def replace(old,new,what,present=None):
    def edit(text):
        if present is not None and text.count(present)==1:return text
        if new in text:
            if text.count(new)!=1:raise Drift(f"{what}: new text occurs {text.count(new)} times, expected 1")
            return text
        if text.count(old)!=1:raise Drift(f"{what}: anchor occurs {text.count(old)} times, expected 1")
        return text.replace(old,new)
    return edit

def json_edit(change,what,indent):
    def edit(text):
        try:data=json.loads(text)
        except ValueError as error:raise Drift(f"{what}: not JSON: {error}")
        if json.dumps(data,indent=indent,ensure_ascii=False)!=text.rstrip("\n"):raise Drift(f"{what}: file is not in its canonical JSON layout (indent {indent})")
        changed=change(data)
        return text if not changed else json.dumps(data,indent=indent,ensure_ascii=False)+("\n" if text.endswith("\n") else "")
    return edit

OLD_ENTRY=L+"/🏗️build/📜️script.ts"
NEW_ENTRY=D+"/📜️script.ts"
def registry(data):
    entry=data.get("implementation",{}).get("entries",{}).get("build")
    if not isinstance(entry,dict) or "script" not in entry:raise Drift("registry: implementation.entries.build.script is missing")
    if entry["script"]==NEW_ENTRY:return False
    if entry["script"]!=OLD_ENTRY:raise Drift(f"registry: build entry is {entry['script']!r}, expected {OLD_ENTRY!r}")
    entry["script"]=NEW_ENTRY;return True

def closures(data):
    rows=data["closures"]
    if any(row["id"]=="repo-door" for row in rows):
        row=[row for row in rows if row["id"]=="repo-door"][0]
        if row["entry"]!=NEW_ENTRY:raise Drift("zero-touch closures: repo-door names another entry")
        return False
    at=[index for index,row in enumerate(rows) if row["id"]=="repo-cli"]
    if len(at)!=1:raise Drift("zero-touch closures: row repo-cli is missing")
    rows.insert(at[0]+1,{"id":"repo-door","entry":NEW_ENTRY,"mode":"full","reason":"repo boots without node_modules for help, refusals and the light commands leases, capacity and doctor through the front door, and hands every other command to the build script"})
    return True

def slo_schema(data):
    changed=False
    for enum in (data["definitions"]["Source"]["enum"],data["definitions"]["Policy"]["const"]["sources"]):
        if "boot-fast" not in enum:enum.append("boot-fast");changed=True
    return changed

SLO_ROW={"id":"cli-help-p50","title":{"en":"Command line help, median","de":"Kommandozeilenhilfe, Median"},"probe":"cli-help","metric":"wallMs","statistic":"p50","direction":"lower","goal":100,"gate":250,"tools":["neutral"],"enforcement":"blocking","tolerance":{"relative":0.2,"absolute":25},"source":"boot-fast"}
def slo_rows(data):
    rows=data["slos"]
    if any(row["id"]==SLO_ROW["id"] for row in rows):
        if [row for row in rows if row["id"]==SLO_ROW["id"]][0]!=SLO_ROW:raise Drift("slo: cli-help-p50 exists with other content")
        return False
    at=[index for index,row in enumerate(rows) if row["id"]=="cli-help-p95"]
    if len(at)!=1:raise Drift("slo: row cli-help-p95 is missing")
    rows.insert(at[0],SLO_ROW);return True

CALLER=B+"/📥️caller/🟦️.ts"
WORKERS_LOOP='if(argument==="--workers"){index++;continue;}if(argument.startsWith("--workers=")){continue;}selected.push(argument);}request.push(...selected);'
GRAPH_POLICY_LOOP='if(argument==="--graph-policy"){index++;continue;}if(argument.startsWith("--graph-policy=")){continue;}selected.push(argument);}request.push(...buildOperationArgumentsV1(selected));'
CALLER_FN='const issueCargoExecutionBackendV1=(...call:Parameters<typeof import("../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts").issueRepositoryCargoExecutionBackendV1>)=>(createRequire(import.meta.url)("../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts") as typeof import("../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts")).issueRepositoryCargoExecutionBackendV1(...call);\n'
STORE_IMPORT='import {isStoreRepairSelectionV1} from "../🧾️store-receipt/🟨️.mjs";\n'
def caller_edit(text):
    text=replace('import {issueRepositoryCargoExecutionBackendV1} from "../../🏃️process/🦀️cargo/🏃️execution/🟦️.ts";\n','import {createRequire} from "node:module";\nimport {buildOperationArgumentsV1} from "../⚙️operations/🔤️options/🟦️.ts";\n',"caller: static cargo import")(text)
    text=replace(STORE_IMPORT,STORE_IMPORT+"/** 🪶️ Loads the Cargo execution contract only when a native owner command asks for it, so no other command pays for the Cargo test harness, its sandbox and the native build storage. */\n"+CALLER_FN,"caller: lazy loader",CALLER_FN)(text)
    text=replace("cargoExecutionBackend=issueRepositoryCargoExecutionBackendV1(root,command,capabilities.native.transport)","cargoExecutionBackend=issueCargoExecutionBackendV1(root,command,capabilities.native.transport)","caller: use")(text)
    return replace(WORKERS_LOOP,WORKERS_LOOP.replace("selected.push(argument);}request.push(...selected);",GRAPH_POLICY_LOOP),"caller: declared selection")(text)

PATTERN_USE_OLD='const pattern = typeof contract.pattern === "string" ? new RegExp(contract.pattern, "u") : undefined;'
PATTERN_USE_NEW='const pattern = typeof contract.pattern === "string" ? jsonSchemaSubsetPattern(contract.pattern) : undefined;'
CHECK_TYPE='type JsonSchemaSubsetCheck = (value: unknown, path: string) => string[];\n'
def validator_edit(text):
    block=open(os.path.join(here,"files","validator-block.txt"),encoding="utf8").read()
    text=replace(PATTERN_USE_OLD,PATTERN_USE_NEW,"validator: pattern use")(text)
    return replace(CHECK_TYPE,CHECK_TYPE+block,"validator: linear patterns")(text)

INVOCATION=B+"/🧭️invocation/🟦️.ts"
TAXONOMY_PATH='"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"'
TAXONOMY_FN='''const testTaxonomyV1=new Map<string,Readonly<{identity:string;value:Readonly<{testPhases?:readonly string[];testLevellessPhases?:readonly string[]}>}>>();

/** 🧊️ Reads the two test phase lists of the taxonomy once per file identity, so the several selections of one invocation do not parse the whole taxonomy again; a replaced or edited file has another identity and is read anew. */
function readTestTaxonomyV1(root:string):Readonly<{testPhases?:readonly string[];testLevellessPhases?:readonly string[]}> {
  const path=join(root,'''+TAXONOMY_PATH+'''),row=statSync(path,{bigint:true}),identity=[row.dev,row.ino,row.size,row.mtimeNs,row.ctimeNs].join(":"),prior=testTaxonomyV1.get(path);
  if(prior?.identity===identity)return prior.value;
  const taxonomy=JSON.parse(readFileSync(path,"utf8")) as {testPhases?:readonly string[];testLevellessPhases?:readonly string[]},value=Object.freeze({testPhases:taxonomy.testPhases,testLevellessPhases:taxonomy.testLevellessPhases});
  testTaxonomyV1.set(path,{identity,value});
  return value;
}
'''
SELECTION_DOC='/** 🧭️ Resolves public selections before shared task graph admission. */\nexport function resolveBuildSelectionV1('
def invocation_edit(text):
    text=replace("    const taxonomy = JSON.parse(readFileSync(join(root, "+TAXONOMY_PATH+'), "utf8"));\n',"    const taxonomy = readTestTaxonomyV1(root);\n","invocation: taxonomy read")(text)
    return replace(SELECTION_DOC,TAXONOMY_FN+SELECTION_DOC,"invocation: memo")(text)

ENTRY_TEST=LAUNCHER+"/🧪️tests/🏗️entry-contract/🐹️.go"
OLD_GO='"📚️library", "🏗️build", "📜️script.ts")'
NEW_GO='"📚️library", "🏗️build", "🚪️door", "📜️script.ts")'

HOST_CAPACITY=B+"/🌡️host-capacity/🟦️.ts"
HOST_CAPACITY_COMMAND=B+"/🌡️host-capacity/⌨️command/🟦️.ts"
BARREL_IMPORT=re.compile(r'(from "(?:\.\./)+🔨️modules/🏃️process/)📦️packages/🟦️typescript/(🪪️agent-sessions/🟦️\.ts";)')
def host_capacity_edit(text):
    found=BARREL_IMPORT.findall(text)
    if not found:return text
    if len(found)!=1:raise Drift("host-capacity: agent-sessions import occurs %d times"%len(found))
    return BARREL_IMPORT.sub(lambda m:m.group(1)+m.group(2),text)

EDITS={
 "door":[(B+"/📥️source/🛗️zero-touch/🗂️closures/🔣️.json",json_edit(closures,"zero-touch closures",2)),(HOST_CAPACITY,host_capacity_edit),(HOST_CAPACITY_COMMAND,host_capacity_edit)],
 "launcher":[(LAUNCHER+"/🗂️registry/🔣️.json",json_edit(registry,"launcher registry",1)),(ENTRY_TEST,replace(OLD_GO,NEW_GO,"launcher entry-contract test"))],
 "validator":[(VALIDATOR+"/🟦️.ts",validator_edit)],
 "manifest":[("package.json",replace('"repo": "repo ./'+B+'/📜️script.ts repo"','"repo": "repo ./'+D+'/📜️script.ts repo"',"root package.json: repo script"))],
 "caller":[(CALLER,caller_edit)],
 "taxonomy":[(INVOCATION,invocation_edit)],
 "slo":[(BENCH+"/🧬️schema/🔣️.json",json_edit(slo_schema,"perf-slo schema",2)),(BENCH+"/🎯️slo/🔣️.json",json_edit(slo_rows,"perf-slo table",1)),(BENCH+"/🧪️tests/🟦️.ts",replace('toEqual(["cli-help-p95","cli-help-p99","noop-run-oneshot-p50","cache-second-run-build-min"])','toEqual(["cli-help-p50","cli-help-p95","cli-help-p99","noop-run-oneshot-p50","cache-second-run-build-min"])',"perf-slo test: selected rows"))],
}
GROUPS=["door","launcher","manifest","caller","validator","taxonomy","slo"]

def read(path):
    try:return open(path,encoding="utf8").read()
    except FileNotFoundError:return None

def main():
    ap=argparse.ArgumentParser();mode=ap.add_mutually_exclusive_group();mode.add_argument("--write",action="store_true");mode.add_argument("--dry-run",action="store_true");ap.add_argument("--root",default=ROOT);ap.add_argument("--group",choices=GROUPS+["all"],default="all")
    a=ap.parse_args();selected=GROUPS if a.group=="all" else [a.group]
    plan=[];problems=[]
    for group in selected:
        for flat,destination in CREATES.get(group,{}).items():
            new=open(os.path.join(here,"files",flat),encoding="utf8").read();path=os.path.join(a.root,destination);old=read(path)
            if len(path)>255:problems.append(f"path longer than 255: {path}")
            if old is None:plan.append(("create",path,None,new))
            elif old==new:plan.append(("same",path,old,new))
            elif os.path.exists(os.path.join(here,"files",flat+".pre-r7")) and old==open(os.path.join(here,"files",flat+".pre-r7"),encoding="utf8").read():plan.append(("edit",path,old,new))
            else:problems.append(f"create target exists with other content: {path}")
        for destination,edit in EDITS.get(group,[]):
            path=os.path.join(a.root,destination);old=read(path)
            if old is None:problems.append(f"edit target missing (apply its set first): {path}");continue
            try:new=edit(old)
            except Drift as error:problems.append(str(error));continue
            if path.endswith(".json"):
                try:json.loads(new)
                except ValueError as error:problems.append(f"result is not JSON: {path}: {error}");continue
            plan.append(("same" if new==old else "edit",path,old,new))
    if ("launcher" in selected or "manifest" in selected) and not any(os.path.exists(os.path.join(a.root,NEW_ENTRY)) or row[1]==os.path.join(a.root,NEW_ENTRY) for row in plan):problems.append("the launcher and manifest groups need the door script: apply group door first or together")
    for kind,path,old,new in plan:print(f"{kind:7} {os.path.relpath(path,a.root)[-150:]}")
    if problems:
        print("REFUSED (nothing written):");[print(" -",p) for p in problems];sys.exit(2)
    changes=[row for row in plan if row[0]!="same"]
    print(f"{len(changes)} change(s), {len(plan)-len(changes)} already applied")
    if not a.write:print("dry run: pass --write to apply");return
    staged=[]
    for kind,path,old,new in changes:
        os.makedirs(os.path.dirname(path),exist_ok=True)
        handle,temp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".boot-fast-");os.close(handle)
        open(temp,"w",encoding="utf8").write(new);staged.append((path,temp,old))
    swapped=[]
    try:
        for path,temp,old in staged:
            if read(path)!=old:raise SystemExit(f"drift during apply: {path}")
        for path,temp,old in staged:
            os.chmod(temp,os.stat(path).st_mode&0o7777 if os.path.exists(path) else 0o644)
            os.replace(temp,path);swapped.append((path,old))
    except BaseException:
        for path,old in reversed(swapped):
            if old is None:os.remove(path)
            else:open(path,"w",encoding="utf8").write(old)
        for path,temp,old in staged:
            if os.path.exists(temp):os.remove(temp)
        raise
    print("applied")

if __name__=="__main__":main()
