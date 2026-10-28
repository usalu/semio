#!/usr/bin/env python3
"""🎚️ jobskey-tools: the granted job count (`request.parallel`, `--parallel` in the envelope capabilities) leaves the Bazel plan-reuse key, and the temporary [DEBUG] key-part lines are removed.
Usage: python3 -I edit.py <root> apply|check|revert. Anchored, all-or-nothing; backups under jobskey-tools/base/<n>."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
Z = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
KEY = "/** 🗝️ Digests everything a plan is a function of"
HELPER = ("/** 🎚️ The granted job count is an execution setting: only the worker plan after the planner reads it, so it leaves the key and a run admitted with fewer jobs reuses the plan. Drops the top-level `parallel` of the request and every `--parallel=<n>` / `--parallel <n>` from the capabilities. */\n"
 "export function bazelPlanJobFreeV1<T>(value:T):T{\n"
 " const strip=(item:unknown):unknown=>{if(Array.isArray(item)){const out:unknown[]=[];for(let index=0;index<item.length;index++){const entry=item[index];if(typeof entry===\"string\"&&/^--parallel=\\d+$/u.test(entry))continue;if(entry===\"--parallel\"&&typeof item[index+1]===\"string\"&&/^\\d+$/u.test(item[index+1] as string)){index++;continue;}out.push(strip(entry));}return out;}return item!==null&&typeof item===\"object\"?Object.fromEntries(Object.entries(item).map(([name,entry])=>[name,strip(entry)])):item;};\n"
 " return (value!==null&&typeof value===\"object\"&&!Array.isArray(value)?strip(Object.fromEntries(Object.entries(value).filter(([name])=>name!==\"parallel\"))):strip(value)) as T;\n"
 "}\n\n")
DEBUG_OLD = None
EDITS = [
 (Z + "/♻️plan-reuse/🟦️.ts", [(KEY, HELPER + KEY)]),
 (Z + "/📜️script.ts", [
  ("import {bazelPlanFilesV1,bazelPlanKeyV1,", "import {bazelPlanFilesV1,bazelPlanJobFreeV1,bazelPlanKeyV1,"),
  (" const debugParts={version:1,root,workspace,request,", " const key=bazelPlanKeyV1({version:1,root,workspace,request:bazelPlanJobFreeV1(request),"),
  ("capabilities:readIssuedScriptProcessEnvelopeV1(invocation).capabilities,runtime:", "capabilities:bazelPlanJobFreeV1(readIssuedScriptProcessEnvelopeV1(invocation).capabilities),runtime:"),
  ("preload:hash(preload)};console.error(\"[DEBUG] plan key parts \"+JSON.stringify(Object.fromEntries(Object.entries(debugParts).map(([name,value])=>[name,hash(JSON.stringify(value)).slice(0,12)]))));console.error(\"[DEBUG] plan key raw \"+JSON.stringify({request:debugParts.request,capabilities:debugParts.capabilities}).slice(0,6000));const key=bazelPlanKeyV1(debugParts);", "preload:hash(preload)});"),
 ]),
 (Z + "/♻️plan-reuse/🧪️tests/🟦️.ts", [
  ("import {bazelPlanKeyV1}from\"../🟦️.ts\";", "import {bazelPlanJobFreeV1,bazelPlanKeyV1}from\"../🟦️.ts\";"),
  ("test(\"a fresh plan in the same workspace and in an empty one are identical,",
   "test(\"the granted job count is no part of the key: requests and envelopes that differ only in --parallel give one key, any other difference another\",()=>{\n"
   " const part=(jobs:number,task=\"@s/a:build\")=>({request:bazelPlanJobFreeV1({tasks:[task],args:[],parallel:jobs,bail:false}),capabilities:bazelPlanJobFreeV1({arguments:[\"run\",task,\"--parallel=\"+jobs],launches:[[\"repo\",\"run\",task,\"--parallel\",String(jobs)]],selection:[task,\"--parallel=\"+jobs]})});\n"
   " expect(bazelPlanKeyV1(part(6))).toBe(bazelPlanKeyV1(part(2)));\n"
   " expect(bazelPlanKeyV1(part(6))).not.toBe(bazelPlanKeyV1(part(6,\"@s/b:build\")));\n"
   " expect(bazelPlanJobFreeV1({selection:[\"--parallel\",\"x\",\"--parallelism=3\"],tasks:[{parallel:false}]})).toEqual({selection:[\"--parallel\",\"x\",\"--parallelism=3\"],tasks:[{parallel:false}]});\n"
   "});\n\n"
   "test(\"a fresh plan in the same workspace and in an empty one are identical,"),
 ]),
]


def main():
    root, mode = sys.argv[1], sys.argv[2]
    plans, bad = [], []
    for index, (rel, pairs) in enumerate(EDITS):
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        new = text
        for old, replacement in pairs:
            if mode == "revert":
                continue
            count = new.count(old)
            if count == 1 and replacement not in new:
                new = new.replace(old, replacement)
            elif replacement not in new:
                bad.append("%s: anchor %r found %d times" % (rel, old[:60], count))
        plans.append((index, rel, path, text, new))
    if mode == "revert":
        for index, rel, path, _, _ in plans:
            backup = os.path.join(HERE, "base", str(index))
            if os.path.exists(backup):
                shutil.copy2(backup, path)
                print("restored", rel)
        return
    for line in bad:
        print("DRIFT", line)
    if bad:
        sys.exit(2)
    for index, rel, path, text, new in plans:
        print(("pending " if new != text else "applied ") + rel)
        if mode == "apply" and new != text:
            os.makedirs(os.path.join(HERE, "base"), exist_ok=True)
            shutil.copy2(path, os.path.join(HERE, "base", str(index)))
            open(path, "w", encoding="utf8").write(new)


main()
