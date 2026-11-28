#!/usr/bin/env python3
"""H75 landing patch: `patch.py check|apply`. Every edit is an exact single-anchor replacement; `check` verifies all anchors on the live files without writing, `apply` writes each file atomically (tmp + rename) after all anchors were verified."""
import os, sys
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
C = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching"
M = "[DEB" + "UG]"

edits = []
def edit(path, old, new, count=1):
    edits.append((path, old, new, count))

P = B + "/⏱️profile/🟨️.mjs"
edit(P, 'import {createRequire} from "node:module";\n',
 'import {createRequire} from "node:module";\nimport {diagnoseV1,diagnosticsV1} from "../🩺️diagnostics/🟨️.mjs";\n')
edit(P, 'export function createRepoProfileV1({mode="off",now=()=>performance.now(),pid=process.pid,write=text=>{try{writeSync(2,text);}catch{}}}={}){\n const enabled=mode!=="off",',
 'export function createRepoProfileV1({mode="off",now=()=>performance.now(),pid=process.pid,write=text=>{try{writeSync(2,text);}catch{}},onPhase}={}){\n const enabled=mode!=="off"||onPhase!==undefined,')
edit(P, ' const close=record=>{if(Number.isNaN(record.end))record.end=now();};',
 ' const close=record=>{if(Number.isNaN(record.end)){record.end=now();onPhase?.(record);}};')
edit(P, '   const record=open(phase,detail),end=now();record.start=Math.max(0,end-durationMilliseconds);record.end=end;\n',
 '   const record=open(phase,detail),end=now();record.start=Math.max(0,end-durationMilliseconds);record.end=end;onPhase?.(record);\n')
edit(P, '   if(!enabled||flushed)return;flushed=true;',
 '   if(!enabled||flushed||mode==="off")return;flushed=true;')
edit(P, '/** 🌐️ Returns the process wide profile, created on first use from the environment and flushed when the process exits. */\nexport function repoProfileV1(){\n if(shared===undefined){\n  shared=createRepoProfileV1({mode:readRepoProfileModeV1(process.env)});\n  if(shared.enabled){process.on("exit",()=>shared.flush());installRepoProfileSpawnCensusV1(shared);}\n }\n return shared;\n}',
 '/** 🌐️ Returns the process wide profile, created on first use from the environment and flushed when the process exits; with `SEMIO_DIAGNOSTICS=timing` every finished phase is also printed at once as a `[diagnostic:timing]` line, so a long running service worker shows its phases while it works instead of only at exit.\n * @see ../🩺️diagnostics/🧬️schema/🔣️.json */\nexport function repoProfileV1(){\n if(shared===undefined){\n  const mode=readRepoProfileModeV1(process.env),onPhase=diagnosticsV1().enabled("timing")?record=>diagnoseV1("timing",()=>({event:"phase",phase:record.phase,depth:record.depth,start:round(record.start),duration:round(record.end-record.start),...(record.detail===undefined?{}:{detail:record.detail})})):undefined;\n  shared=createRepoProfileV1({mode,onPhase});\n  if(shared.enabled){if(mode!=="off")process.on("exit",()=>shared.flush());installRepoProfileSpawnCensusV1(shared);}\n }\n return shared;\n}')
edit(P, '/** 📶️ Says what the phase started last',
 '/** 🔁️ Counts one round trip of a kind (`graph-service`, `cache-lookup`) as a zero length `rt.<kind>` row of the process wide profile without touching the current phase name; free while the profile is off. */\nexport function profileRoundTripV1(kind){repoProfileV1().mark("rt."+kind);}\n/** 📶️ Says what the phase started last')

D = B + "/⏱️profile/🟨️.d.mts"
edit(D, 'export interface RepoProfileOptionsV1{mode?:RepoProfileModeV1;now?:()=>number;pid?:number;write?:(text:string)=>unknown}',
 'export interface RepoProfileOptionsV1{mode?:RepoProfileModeV1;now?:()=>number;pid?:number;write?:(text:string)=>unknown;onPhase?:(record:RepoProfileRecordV1)=>void}')
edit(D, 'export function profileMarkV1(phase:string,detail?:string):void;',
 'export function profileMarkV1(phase:string,detail?:string):void;\nexport function profileRoundTripV1(kind:string):void;')

R = B + "/🏡️resident/🚪️client/🛰️plan/🟨️.mjs"
edit(R, 'import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";\n',
 'import {createFrameDecoderV1,encodeFrameV1} from "../../📨️protocol/🟨️.mjs";\nimport {profileRoundTripV1} from "../../../⏱️profile/🟨️.mjs";\n')
edit(R, '  socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:agent},languages,cap:token,implementation})+encodeFrameV1({...frame,cap:token}));',
 '  profileRoundTripV1("graph-service");\n  socket.write(encodeFrameV1({t:"hello",v:1,client:{name:"repo",pid:process.pid,lane:agent},languages,cap:token,implementation})+encodeFrameV1({...frame,cap:token}));')

POL = C + "/💾️storage/🔒️enforcement/🏗️compiler/🚪️ingress/📥️arguments/📋️policy/🟦️.ts"
edit(POL, 'import schema from "../🧬️schema/🔣️.json" with {type:"json"};\n',
 'import schema from "../🧬️schema/🔣️.json" with {type:"json"};\nimport {diagnoseV1} from "../../../../../../../🏗️build/🩺️diagnostics/🟨️.mjs";\n')
edit(POL, 'throw Error("Cargo configuration belongs to original repository admission " + M + " rejected="+JSON.stringify(configuration)+" allowed="+JSON.stringify(configurations));'.replace(' " + M + "', ' ' + M + ' ').replace('" ' + M + ' rejected=', '" '+M+' rejected=') if False else
     'throw Error("Cargo configuration belongs to original repository admission ' + M + ' rejected="+JSON.stringify(configuration)+" allowed="+JSON.stringify(configurations));',
 '{diagnoseV1("cargo-admission",()=>({event:"configuration-refused",rejected:configuration??null,allowed:configurations}));throw Error("Cargo configuration belongs to original repository admission");}')

PT = B + "/⏱️profile/🧪️tests/🟦️.ts"
APPEND_TEST = """

test("a phase callback streams every finished phase, stays silent at flush and a round trip counts without naming a phase",()=>{
 const seen:string[]=[],written:string[]=[];let time=0;
 const profile=createRepoProfileV1({mode:"off",now:()=>time,write:text=>written.push(text),onPhase:record=>seen.push(record.phase+":"+(record.end-record.start))});
 expect(profile.enabled).toBe(true);
 profile.phase("a.b",()=>{time+=5;profile.mark("rt.graph-service");});
 profile.event("spawn.x",3);
 profile.flush();
 expect(seen).toEqual(["rt.graph-service:0","a.b:5","spawn.x:3"]);
 expect(written).toEqual([]);
});

test("SEMIO_DIAGNOSTICS=timing prints each finished phase at once and a round trip leaves the current phase alone",()=>{
 const module=JSON.stringify(import.meta.dir+"/../🟨️.mjs"),program=`const m=await import(${module});m.profilePhaseV1("a.b",()=>{m.profileRoundTripV1("graph-service");});console.log(m.profileCurrentPhaseV1());`;
 const run=spawnSync(process.execPath,["-e",program],{encoding:"utf8",env:{...process.env,SEMIO_REPO_PROFILE:"off",SEMIO_DIAGNOSTICS:"timing"}});
 expect(run.status).toBe(0);expect(run.stdout.trim()).toBe("a.b");
 const lines=run.stderr.split("\\n").filter(line=>line.startsWith("[diagnostic:timing] "));
 expect(lines.map(line=>JSON.parse(line.slice(20)).phase)).toEqual(["rt.graph-service","a.b"]);
 const quiet=spawnSync(process.execPath,["-e",program],{encoding:"utf8",env:{...process.env,SEMIO_REPO_PROFILE:"off",SEMIO_DIAGNOSTICS:""}});
 expect(quiet.stderr).toBe("");
});
"""

def main(mode):
    plan = {}
    for path, old, new, count in edits:
        text = plan.get(path)
        if text is None:
            text = open(path, encoding="utf-8").read()
        found = text.count(old)
        if found != count:
            print("ANCHOR", found, "!=", count, path[-60:], repr(old[:70])); sys.exit(1)
        plan[path] = text.replace(old, new)
    plan[PT] = open(PT, encoding="utf-8").read() + APPEND_TEST
    print("anchors ok:", len(edits), "edits in", len(plan), "files")
    if mode == "apply":
        for path, text in plan.items():
            tmp = path + ".h75.tmp"
            open(tmp, "w", encoding="utf-8").write(text)
            os.replace(tmp, path)
        print("applied")

main(sys.argv[1] if len(sys.argv) > 1 else "check")
