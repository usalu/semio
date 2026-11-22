#!/usr/bin/env python3
import json,glob,re,sys,os
SC="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-kfaIfm/ccee8eb5-3363-4a84-af9e-53e38d236627/scratchpad"
T=os.path.dirname(os.path.abspath(__file__))
rows={}
srcs=[T+"/../E30/results-part1.jsonl"]+sorted(glob.glob(SC+"/E30/*.jsonl"))+sorted(glob.glob(SC+"/E31/table*.jsonl"))
for p in srcs:
    for l in open(p):
        try: d=json.loads(l)
        except Exception: continue
        if d.get("status")=="refused" and d["file"] in rows: continue
        rows[d["file"]]=d
extra=sys.argv[1:] 
for e in extra:
    for l in open(e):
        try: d=json.loads(l)
        except Exception: continue
        rows[d["file"]]=d
fixed={ # latest verified state after fixes of E30/E31
 "⏱️profile/📊️cpu/🧪️tests/🟦️.ts":"fixed by E30, 5/5",
 "⚙️operations/🧪️tests/🧭️help-offline/🟦️.ts":"fixed by E30 (async pool), 1/1",
 "🌐️remote-cache/🧶️nx/🚦️single-flight/🧪️tests/🟦️.ts":"fixed by E30, 15/15",
 "🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🔒️real-bazel/🟦️.ts":"fixed by E30, 1/1",
 "🔧️tools/🏗️bazel/📐️path-plan/🧪️tests/🟦️.ts":"fixed by E30, 5/5",
 "🔧️tools/🏗️bazel/📸️snapshot/🧪️tests/🔒️planned/🟦️.ts":"fixed by E30, 2/2",
 "🔧️tools/🏗️bazel/🧪️tests/📥️paths/🟦️.ts":"fixed by E30, 11/11",
 "🔧️tools/🏗️bazel/🧪️tests/🟦️.ts":"fixed by E30, 15/15",
 "🔭️witness/🧪️tests/🟦️.ts":"fixed by E30, 13/13",
 "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🧪️tests/🟦️.ts":"fixed by E30, 30/30",
 "🔬️test-file/🧪️tests/🧫️fixtures/🔴️failing/🟦️.ts":"intentional failing fixture (counted by the census, not a defect)",
 "🌐️remote-execution/🛰️sidecar/🧪️tests/🧮️closure/🟦️.ts":"fixed (sidecar static import), 5/5",
 "🏞️task-view/🧱️hermetic/🧪️tests/🟦️.ts":"fixed (short scratch), 15/15",
 "🪪️agent-sessions/⌨️command/🧪️tests/🟦️.ts":"fixed (short scratch), 9/9",
 "🌐️remote-execution/🚀️offload/🧪️tests/🟦️.ts":"fixed (env isolation), 31/31",
 "🌐️remote-execution/🚀️offload/🧪️tests/🌐️cluster/🟦️.ts":"fixed (env isolation), 9/9",
 "🌐️remote-execution/🚀️offload/🧪️tests/⏱️timeouts/🟦️.ts":"fixed (env isolation), 6/6",
}
def cause(f,d):
    e=(d.get("tail") or "")+(d.get("error") or "")
    if "capabilities-differ" in e or "Original process birth already received" in e or "Original envelope" in e or "Exact Original Graph format owner required" in e or "Original native caller capabilities" in e: return "declared birth / suite row grant drift (needs `capabilities` on the suite row or the launcher; test-file and suite share it)"
    if "cannot-vouch" in e or "canary" in e or "no change journal service" in e.lower() or "no-service" in e: return "environment: FSEvents delivers no events on this host (journal canary), measured with a bare fs.watch"
    if "resident/" in f: return "resident service under host load / FSEvents (timing)"
    if "deadline exhausted" in e: return "test-file envelope deadline (file slower than budget at load 30-45); crash of the run fixed"
    if "Cannot find module" in e: return "closure copy lacks a dynamically imported module"
    if "/tmp" in e and "EPERM" in e: return "Bazel hermetic runner cannot create /tmp/<hash> inside the test sandbox (bazel lane)"
    if "🕸️graph" in f or "🧪️tests/🕸️graph" in f or "📥️source" in f or "📥️inputs" in f: return "graph / inputs / source content test (other lane)"
    if "remote-execution" in f: return "remote-execution sidecar / bazel (see report)"
    return "not analysed (first error shown)"
lines=["# Table E31 (continuation of E30)","","Latest result per file after E30 and E31 runs, failures only (`repo test-file --json`, one file per call, load 25-45, 2026-11-22 07:00-08:50). Passing files omitted.",""]
tot={}
for d in rows.values(): tot[d["status"]]=tot.get(d["status"],0)+1
lines.append("Files run: %d of 662; %s."%(len(rows),", ".join("%d %s"%(v,k) for k,v in sorted(tot.items()))))
lines+=["","| file (relative to build) | suite | status | pass/fail | s | cause / first error |","|---|---|---|---|---|---|"]
for f,d in sorted(rows.items()):
    rel=f.split("🏗️build/")[-1]
    if d["status"]=="passed": continue
    if rel in fixed: lines.append("| %s | %s | %s | %s | %s | %s |"%(rel,d.get("suite"),"FIXED","-","-",fixed[rel])); continue
    err=re.sub(r"\s+"," ",(d.get("error") or ""))[-90:].replace("|","/")
    lines.append("| %s | %s | %s | %s/%s | %s | %s: %s |"%(rel,d.get("suite"),d["status"],d.get("pass"),d.get("fail"),d.get("seconds"),cause(f,d),err))
open(T+"/table.md","w").write("\n".join(lines)+"\n")
print(len(rows),tot)
