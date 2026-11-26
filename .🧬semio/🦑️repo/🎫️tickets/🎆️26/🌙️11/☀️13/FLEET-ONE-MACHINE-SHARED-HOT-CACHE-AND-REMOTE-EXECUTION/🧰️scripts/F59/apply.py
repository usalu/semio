#!/usr/bin/env python3
"""F59: the launcher of the readiness sidecar ends and restarts a silent child (schema first, en/de), the daemon-level kill of the supervisor is removed, fleet status says "stale readiness beat". Dry run without --apply. Every write is temp + rename, every anchor must be unique."""
import os,re,sys
ROOT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B=ROOT+"📚️library/🏗️build/"
READY=B+"🌐️remote-execution/🛰️sidecar/🚦️ready/"
SIDE=B+"🌐️remote-execution/🛰️sidecar/🟦️.ts"
SUP=ROOT+"🤝️shared-worktree/🔁️replica/🛟️supervisor/🟦️.ts"
FS=B+"🚢️fleet-status/🧬️schema/🔣️.json"
apply="--apply" in sys.argv
def load(p): return open(p,encoding="utf8").read()
def save(p,t):
    tmp=p+".f59.tmp"
    open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,p)
def once(t,old,new,label):
    n=t.count(old)
    if n!=1: sys.exit(f"ANCHOR {label}: {n} matches")
    return t.replace(old,new)
done=[]
def edit(p,fn):
    t=load(p);u=fn(t)
    if u==t: sys.exit("no change "+p)
    done.append(p)
    if apply: save(p,u)

def schema(t):
    if '"silenceChildGraceMilliseconds"' in t: sys.exit("schema already landed")
    t=once(t,'        "silenceRestartCooldownMilliseconds": 300000,\n','        "silenceRestartCooldownMilliseconds": 300000,\n        "silenceChildGraceMilliseconds": 180000,\n',"cooldown")
    t=once(t,'"heartbeatNames": ["pool.json", "attest.json"],','"heartbeatNames": ["pool.json", "attest.json", "head.json"],',"heartbeat")
    texts='    "SilenceTexts": {"const": {"en": {"childRestart": "the readiness supervisor wrote nothing for {seconds} s ({beats} missed beats) while its machine lives: the launcher ends it and starts it again"}, "de": {"childRestart": "der Bereitschafts-Supervisor schrieb seit {seconds} s nichts ({beats} verpasste Schläge), obwohl seine Maschine lebt: der Starter beendet ihn und startet ihn neu"}}},\n'
    return once(t,'    "Inflight": {"type": "object"',texts+'    "Inflight": {"type": "object"',"inflight")
edit(READY+"🧬️schema/🔣️.json",schema)

SIL=READY+"💔️silence/🟦️.ts"
def silence(t):
    if "readySilenceChildWatchV1" in t: sys.exit("silence already landed")
    t=once(t,'import schema from "../🧬️schema/🔣️.json" with {type:"json"};\n','import {join} from "node:path";\nimport schema from "../🧬️schema/🔣️.json" with {type:"json"};\n',"import")
    t=once(t,'const P=schema.definitions.Policy.const;\n','const P=schema.definitions.Policy.const,T=schema.definitions.SilenceTexts.const;\n\n/** 📐️ The closed policy of the readiness record (beats, silence limits, heartbeat files), the one source of the launcher watchdog and the fleet readers. */\nexport const readySilencePolicyV1=P;\n',"policy")
    add='''
/** 🔪️ The watchdog of the launcher that owns the readiness child: like {@link readySilenceWatchV1}, but a child younger than `silenceChildGraceMilliseconds` is never judged (the record it finds is the one of the child before it) and the cooldown holds across the children of one launcher, so a wedged child is ended once and a healthy machine never loops. A pure function of the records seen so far.
 * @see ./🧪️tests/🟦️.ts */
export function readySilenceChildWatchV1(policy:typeof P=P):Readonly<{observe(status:unknown,nowMs:number,childStartedMs:number):"wait"|"restart"}>{
 const watch=readySilenceWatchV1(policy);
 return Object.freeze({observe(status,nowMs,childStartedMs){return nowMs-childStartedMs<policy.silenceChildGraceMilliseconds?"wait":watch.observe(status,nowMs);}});
}

/** 🔪️ The bilingual line the launcher says when it ends a silent readiness child (see `SilenceTexts` of the readiness schema). */
export function readySilenceChildNoticeV1(silence:Readonly<{missedBeats:number;ageMs:number}>):string{
 const fill=(language:"en"|"de"):string=>T[language].childRestart.replace(/\\{(\\w+)\\}/gu,(_all,name:string)=>String(name==="seconds"?Math.round(silence.ageMs/1000):silence.missedBeats));
 return `${fill("en")} / ${fill("de")}`;
}

/** 🗂️ The `ready.json` of the machine a launcher runs on (`<store>/v1/origins/<origin>/ready.json`) from its environment (`SEMIO_FLEET_STORE_ROOT`, `SEMIO_FLEET_ORIGIN`), `undefined` without a fleet store or with an origin that is not a plain name. */
export function readySilenceStatusFileV1(environment:Readonly<Record<string,string|undefined>>):string|undefined{
 const store=environment.SEMIO_FLEET_STORE_ROOT,origin=environment.SEMIO_FLEET_ORIGIN;
 return store===undefined||store===""||origin===undefined||!/^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/u.test(origin)?undefined:join(store,"v1","origins",origin,P.statusName);
}
'''
    return once(t,'\n/** 💔️ The bilingual line a restart says: how long the sidecar was silent. */',add+'\n/** 💔️ The bilingual line a restart says: how long the sidecar was silent. */',"notice")
edit(SIL,silence)

def side(t):
    if "readySilenceChildWatchV1" in t: sys.exit("sidecar already landed")
    t=once(t,'import {rmSync} from "node:fs";','import {readFileSync,rmSync} from "node:fs";',"fs")
    t=once(t,'import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\n','import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";\nimport {readySidecarSilenceV1,readySilenceChildNoticeV1,readySilenceChildWatchV1,readySilencePolicyV1,readySilenceStatusFileV1} from "./🚦️ready/💔️silence/🟦️.ts";\n',"import")
    t=once(t,'let stopping=false,alive=false,child:Child|undefined,successor:Child|undefined;','let stopping=false,alive=false,child:Child|undefined,successor:Child|undefined,launchedMs=Date.now();',"let")
    t=once(t,'  const created:Child=Bun.spawn(','  launchedMs=Date.now();\n  const created:Child=Bun.spawn(',"spawn")
    block='''const silenceFile=role==="ready"?readySilenceStatusFileV1(environment):undefined;
 if(silenceFile!==undefined){
  const pick=(name:string,fallback:number):number=>Number(environment[name])||fallback,watch=readySilenceChildWatchV1({...readySilencePolicyV1,silenceChildGraceMilliseconds:pick("SEMIO_SIDECAR_SILENCE_GRACE_MILLISECONDS",readySilencePolicyV1.silenceChildGraceMilliseconds)});
  setInterval(()=>{
   if(stopping||!alive||child===undefined)return;
   try{
    const status=JSON.parse(readFileSync(silenceFile,"utf8")),now=Date.now();
    if(watch.observe(status,now,launchedMs)!=="restart")return;
    process.stderr.write(`${new Date().toISOString()} ${role} ${readySilenceChildNoticeV1(readySidecarSilenceV1({status,nowMs:now}))}\\n`);
    child.kill("SIGKILL");
   }catch{}
  },pick("SEMIO_SIDECAR_SILENCE_POLL_MILLISECONDS",5000)).unref();
 }
 let delay=restart.initialMilliseconds;'''
    return once(t,' let delay=restart.initialMilliseconds;',' '+block,"delay")
edit(SIDE,side)

def sup(t):
    lines=t.split("\n");out=[];i=0
    if "silenceModuleRelativeV1" not in t: sys.exit("supervisor already landed")
    skip=set()
    for n,l in enumerate(lines):
        if l.startswith("type SilenceModuleV1=") or l.startswith("const silenceModuleRelativeV1="): skip.add(n)
        if l.startswith("/** 💔️ The `ready.json` of this machine's readiness sidecar"):
            skip.update(range(n,n+7)); assert lines[n+6]=="}"
        if l.startswith("  const statusFile=readyStatusFileV1(root,environment);") or l.startswith("  if(statusFile!==undefined)void import(pathToFileURL(join(root,...silenceModuleRelativeV1))"): skip.add(n)
        if l.startswith("   if(statusFile!==undefined&&silenceRule!==undefined"):
            skip.update(range(n,n+4)); assert lines[n+3]=="   }"
    for n,l in enumerate(lines):
        if n in skip: continue
        if l.startswith("  let recording=false,ownedBeat:Beat|undefined,ownedBeatAt=startedAt,silenceRule"):
            l=re.sub(r",silenceRule:SilenceModuleV1\|undefined,silenceWatch:ReturnType<SilenceModuleV1\[\"readySilenceWatchV1\"\]>\|undefined","",l)
        out.append(l)
    u="\n".join(out)
    for w in ("silenceRule","silenceWatch","statusFile","SilenceModuleV1","readyStatusFileV1","silenceModuleRelativeV1"):
        if w in u: sys.exit("leftover "+w)
    return u
edit(SUP,sup)

def fs(t):
    t=once(t,'''    "workers.state.silent": {
     "en": "silent",
     "de": "still"
    },''','''    "workers.state.silent": {
     "en": "stale readiness beat",
     "de": "Bereitschaftsschlag veraltet"
    },''',"state")
    t=once(t,'"worker {origin} lives but wrote no readiness record for {age} although it is {recorded} in stage {stage}"','"worker {origin} lives but its readiness beat is stale: no readiness record for {age} although it is {recorded} in stage {stage}"',"finding-en")
    return once(t,'"Worker {origin} lebt, schrieb aber {age} lang keinen Bereitschaftseintrag, obwohl er {recorded} in Stufe {stage} ist"','"Worker {origin} lebt, aber sein Bereitschaftsschlag ist veraltet: {age} lang kein Bereitschaftseintrag, obwohl er {recorded} in Stufe {stage} ist"',"finding-de")
edit(FS,fs)
t=load(FS)
print("fleet-status de finding check:",'"Worker {origin} lebt, schrieb aber {age} lang keinen Bereitschaftseintrag, obwohl er {recorded} in Stufe {stage} ist"' in t)
print(("APPLIED" if apply else "DRY RUN OK"),*done,sep="\n ")
