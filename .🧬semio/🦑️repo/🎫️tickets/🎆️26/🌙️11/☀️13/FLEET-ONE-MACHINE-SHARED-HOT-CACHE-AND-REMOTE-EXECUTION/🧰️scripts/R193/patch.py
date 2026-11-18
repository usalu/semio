import sys,re,json
S=sys.argv[1]
def sub(text,old,new,count=1):
    assert text.count(old)==count,(old[:60],text.count(old))
    return text.replace(old,new)
def rw(name,fn):
    t=open(f"{S}/base/{name}",encoding="utf8").read()
    open(f"{S}/stage/{name}","w",encoding="utf8").write(fn(t))

def h(t):
    t=sub(t,'import{tmpdir}from"node:os";','import{loadavg,tmpdir}from"node:os";')
    t=sub(t,'import{PROVIDER_CLOSURE_POLICY_V1}','import{hostProfileV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪞️profile/🟦️.ts";\nimport{MACHINE_SPEED_POLICY_V1,localMachineSpeedFactorV1,processCpuMillisecondsV1}from"../../../../🐢️speed/🟨️.mjs";\nimport{PROVIDER_CLOSURE_POLICY_V1}')
    a=t.index('const ALIVE=')
    b=t.index('/** 🪆️ Asks one heavy derived question')
    new='''/** 🐢️ How many times longer than on the reference machine a child of a heavy question may show no sign of life before it counts as stuck: the speed factor of the machine times the one-minute load per core where that is above one, never above `maximumScale`; a machine at load 36 on 10 cores waits 3.6 times as long, not the same minute as an idle one.
 * @see ../../../../🐢️speed/🟨️.mjs `localMachineSpeedFactorV1` */
export function heavyQuestionScaleV1(policy=HEAVY_QUESTION_POLICY_V1,environment=process.env,load=loadavg()[0]){
 const cores=Math.max(1,hostProfileV1(environment).cores);
 return Math.min(policy.maximumScale,Math.max(1,localMachineSpeedFactorV1(environment))*Math.max(1,load/cores));
}

const BEAT=/^\\u0001alive(?: (\\S+))?(?: (\\d+))?$/u;
const questionOfV1=request=>request.prime?"prime":request.preflight?"preflight":String(request.row?.kind??"derived")+" "+String(request.row?.path??"").slice(0,120);
const lanes={questions:Promise.resolve(),prime:Promise.resolve()};
const inLane=(name,task)=>{const run=lanes[name].then(task,task);lanes[name]=run.then(()=>undefined,()=>undefined);return run;};
const running=new Set();
process.once("exit",()=>{for(const child of running)try{child.kill("SIGKILL");}catch{}});

const answerIn=(request,workspaceRoot,signal,policy=HEAVY_QUESTION_POLICY_V1,script=SCRIPT,ports={})=>new Promise((resolve,reject)=>{
 signal?.throwIfAborted();
 const directory=mkdtempSync(join(tmpdir(),"semio-heavy-question-")),output=join(directory,"answer.json");
 const{scale=heavyQuestionScaleV1,cpuOf=processCpuMillisecondsV1}=ports,factor=scale(policy),window=Math.ceil(policy.silenceMilliseconds*factor),look=Math.min(policy.livenessMilliseconds,window>>1),question=questionOfV1(request);
 let silent=false,phase="boot",lag=0,cpuSeen,partial="",tail="",lastLife=Date.now(),lookedAt=lastLife;
 const done=(error,value)=>{signal?.removeEventListener("abort",abort);clearTimeout(timer);clearInterval(watch);running.delete(child);rmSync(directory,{recursive:true,force:true});error?reject(error):resolve(value);};
 const child=spawn(process.execPath,[script,workspaceRoot,output],{stdio:["pipe","ignore","pipe"],windowsHide:true,env:{...process.env,[policy.environment]:String(policy.memoryLimitMegabytes)}});
 running.add(child);
 child.stderr.on("data",chunk=>{
  lastLife=Date.now();cpuSeen=undefined;partial+=chunk;
  const lines=partial.split("\\n");partial=lines.pop();
  for(const line of lines){const beat=BEAT.exec(line);if(beat===null){tail=(tail+line+"\\n").slice(-2000);continue;}phase=beat[1]??phase;lag=Math.max(lag,Number(beat[2]??0));}
 });
 const abort=()=>{child.kill();};signal?.addEventListener("abort",abort,{once:true});
 const timer=setTimeout(()=>child.kill(),Math.ceil(policy.timeoutMilliseconds*factor));
 const watch=setInterval(()=>{
  const now=Date.now();
  if(now-lastLife>=look&&now-lookedAt>=look){
   lookedAt=now;const spent=cpuOf(child.pid);
   if(spent!==undefined){if(cpuSeen!==undefined&&spent-cpuSeen>=MACHINE_SPEED_POLICY_V1.livenessCpuMilliseconds)lastLife=now;cpuSeen=spent;}
  }
  if(now-lastLife>=window){silent=true;child.kill("SIGKILL");}
 },Math.max(10,Math.min(1000,window>>2)));
 child.once("error",error=>done(error));
 child.once("close",code=>{
  tail+=partial;
  if(signal?.aborted)return done(signal.reason);
  if(silent)return done(refuseBuildV1("failed","graph.heavy-question-silent",{seconds:String(Math.round((Date.now()-lastLife)/1000)),question,phase,lag:String(Math.round(lag/1000)),detail:tail.trim().split("\\n").slice(-2).join(" | ").slice(-300)||"-"}));
  if(code!==0)return done(failureOfV1(code,tail,output));
  try{done(undefined,JSON.parse(readFileSync(output,"utf8")));}catch(error){done(error);}
 });
 child.stdin.end(JSON.stringify(request));
});

'''
    t=t[:a]+new+t[b:]
    t=sub(t,'A child that writes nothing for `silenceMilliseconds` is stopped with a refusal that names the silence. */\nexport function askHeavyQuestionV1(row,{workspaceRoot,signal,policy,script}){return inLane("questions",()=>answerIn({row},workspaceRoot,signal,policy,script));}',
      'A child that shows no sign of life (no line on its standard error, no processor time) for `silenceMilliseconds` times the scale of the machine is stopped with a refusal that names the question, its last phase and the silent seconds; a child that computes without printing, or waits on a lease with its loop turning, is never stopped. */\nexport function askHeavyQuestionV1(row,{workspaceRoot,signal,policy,script,ports}){return inLane("questions",()=>answerIn({row},workspaceRoot,signal,policy,script,ports));}')
    t=sub(t,'export function primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal,policy,script}){return inLane("prime",()=>answerIn({prime:{storeDirectory}},workspaceRoot,signal,policy,script));}','export function primeSharedClosuresInChildV1({workspaceRoot,storeDirectory,signal,policy,script,ports}){return inLane("prime",()=>answerIn({prime:{storeDirectory}},workspaceRoot,signal,policy,script,ports));}')
    t=sub(t,'const run=inLane("prime",()=>answerIn({preflight:true},workspaceRoot,signal));','const run=inLane("prime",()=>answerIn({preflight:true},workspaceRoot,signal,undefined,undefined,ports));')
    t=sub(t,'export function preflightToolClosuresInChildV1({workspaceRoot,signal}){','export function preflightToolClosuresInChildV1({workspaceRoot,signal,ports}){')
    return t
rw("h.mjs",h)

def s(t):
    t=sub(t,'import{HEAVY_QUESTION_POLICY_V1}from"./🟨️.mjs";\nimport{openPlannerDeriverV1}from"../../../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts";\nimport{isRepoErrorV1,repoErrorDataV1}from"../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";\n',
     'import{Worker}from"node:worker_threads";\n\ntry{new Worker(new URL("../../📦️project-nodes/🏊️project-farm/🐕️watchdog/🟨️.mjs",import.meta.url),{workerData:{parentPid:process.ppid}}).unref();}catch{}\nconst{HEAVY_QUESTION_POLICY_V1}=await import("./🟨️.mjs"),{openPlannerDeriverV1}=await import("../../../../🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"),{isRepoErrorV1,repoErrorDataV1}=await import("../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts");\n')
    t=sub(t,'stops at its resident memory cap;','stops at its resident memory cap; its first act is the parent-gone watchdog thread of the farm workers (a child whose parent was killed ends within a second, whatever it computes), and every `heartbeatMilliseconds` its event loop writes one line of life with its phase and the lateness of that turn (`\\u0001alive <phase> <lag ms>`), so a long synchronous stretch shows as lag and a stretch that waits on a lease keeps its line;')
    t=sub(t,'setInterval(()=>{process.stderr.write("\\u0001alive\\n");},HEAVY_QUESTION_POLICY_V1.heartbeatMilliseconds).unref();',
     'let phase="boot",beatAt=Date.now();\nsetInterval(()=>{const now=Date.now();process.stderr.write("\\u0001alive "+phase+" "+Math.max(0,now-beatAt-HEAVY_QUESTION_POLICY_V1.heartbeatMilliseconds)+"\\n");beatAt=now;},HEAVY_QUESTION_POLICY_V1.heartbeatMilliseconds).unref();')
    t=sub(t,'const request=JSON.parse(await Bun.stdin.text()),derive=await openPlannerDeriverV1(root!);\n','phase="read";\n const request=JSON.parse(await Bun.stdin.text());phase="open";\n const derive=await openPlannerDeriverV1(root!);phase=request.prime?"prime":request.preflight?"preflight":"question";\n')
    return t
rw("s.ts",s)

def pj(t):
    return sub(t,'"heartbeatMilliseconds": 2000,\n  "silenceMilliseconds": 60000,','"heartbeatMilliseconds": 2000,\n  "livenessMilliseconds": 5000,\n  "silenceMilliseconds": 60000,\n  "maximumScale": 8,')
rw("p.json",pj)

def ps(t):
    t=sub(t,'"heartbeatMilliseconds", "silenceMilliseconds", "kinds"]','"heartbeatMilliseconds", "livenessMilliseconds", "silenceMilliseconds", "maximumScale", "kinds"]')
    a=t.index('    "silenceMilliseconds"');b=t.index('    "kinds"')
    new='''    "livenessMilliseconds": {"type": "integer", "minimum": 100, "description": "EN: A child that has been quiet this long is looked at: the parent reads its processor time (where the platform can say) every this often, and at least the progress step of the machine speed policy of new processor time counts as a sign of life, so a long synchronous stretch is never taken for a stall. DE: Ein Kind, das so lange still war, wird angesehen: die Elternseite liest (wo die Plattform es kann) so oft seine Prozessorzeit, und mindestens der Fortschrittsschritt der Geschwindigkeitsrichtlinie an neuer Prozessorzeit zählt als Lebenszeichen, sodass ein langer synchroner Abschnitt nie für einen Stillstand gehalten wird."},
    "silenceMilliseconds": {"type": "integer", "minimum": 1000, "description": "EN: A window without any sign of life on the reference machine (no line on the standard error of the child, no new processor time); it is multiplied by the scale of the machine (speed factor times load per core, at most maximumScale) and a child that stays inside it nowhere is stopped with a refusal that names the question, its last phase and the silent seconds; the wall limit above only bounds a child that keeps living. DE: Ein Fenster ohne jedes Lebenszeichen auf der Referenzmaschine (keine Zeile auf der Fehlerausgabe des Kindes, keine neue Prozessorzeit); es wird mit dem Maßstab der Maschine multipliziert (Geschwindigkeitsfaktor mal Last pro Kern, höchstens maximumScale), und ein Kind, das nirgends darin bleibt, wird mit einer Ablehnung gestoppt, die die Frage, ihre letzte Phase und die stillen Sekunden nennt; die Zeitgrenze oben begrenzt nur ein Kind, das weiterlebt."},
    "maximumScale": {"type": "number", "minimum": 1, "description": "EN: The most the windows and the wall limit of a child are lengthened for a slow or loaded machine. DE: Das Meiste, um das die Fenster und die Zeitgrenze eines Kindes für eine langsame oder belastete Maschine verlängert werden."},
'''
    return t[:a]+new+t[b:]
rw("ps.json",ps)

def dm(t):
    t=sub(t,'heartbeat','heartbeat') if False else t
    t=sub(t,'pollMilliseconds:number;timeoutMilliseconds:number;kinds','pollMilliseconds:number;timeoutMilliseconds:number;heartbeatMilliseconds:number;livenessMilliseconds:number;silenceMilliseconds:number;maximumScale:number;kinds')
    t=sub(t,'/** 🪆️ Whether a derived row must be answered in a child process. */','/** 🐢️ How many times longer than on the reference machine a child may show no sign of life. */\nexport function heavyQuestionScaleV1(policy?:typeof HEAVY_QUESTION_POLICY_V1,environment?:Record<string,string|undefined>,load?:number):number;\n/** 🪆️ The ports of a law: the scale of the machine and the reader of the processor time of a child. */\nexport type HeavyQuestionPortsV1=Readonly<{scale?:(policy:typeof HEAVY_QUESTION_POLICY_V1)=>number;cpuOf?:(pid:number)=>number|undefined}>;\n/** 🪆️ Whether a derived row must be answered in a child process. */')
    t=sub(t,'options:{workspaceRoot:string;signal?:AbortSignal}):Promise<{fact:unknown;rows:readonly unknown[]}>','options:{workspaceRoot:string;signal?:AbortSignal;policy?:typeof HEAVY_QUESTION_POLICY_V1;script?:string;ports?:HeavyQuestionPortsV1}):Promise<{fact:unknown;rows:readonly unknown[]}>')
    t=sub(t,'options:{workspaceRoot:string;storeDirectory:string;signal?:AbortSignal}','options:{workspaceRoot:string;storeDirectory:string;signal?:AbortSignal;policy?:typeof HEAVY_QUESTION_POLICY_V1;script?:string;ports?:HeavyQuestionPortsV1}')
    t=sub(t,'options:{workspaceRoot:string;signal?:AbortSignal}):Promise<{checked:number}>','options:{workspaceRoot:string;signal?:AbortSignal;ports?:HeavyQuestionPortsV1}):Promise<{checked:number}>')
    return t
rw("h.d.mts",dm)

def tx(t):
    old_en='"A heavy derived question was stopped: its child process wrote nothing for {seconds} s (stuck on a lock, a dead peer or a blocked loop): {detail}"'
    new_en='"A heavy derived question was stopped: its child process showed no sign of life for {seconds} s, no output and no processor time (question {question}, last phase {phase}, longest event-loop stall {lag} s; a dead peer, a lock nobody releases or a frozen process): {detail}"'
    old_de='"Eine schwere abgeleitete Frage wurde gestoppt: ihr Kindprozess hat {seconds} s lang nichts geschrieben (hängt an einer Sperre, einem toten Partner oder einer blockierten Schleife): {detail}"'
    new_de='"Eine schwere abgeleitete Frage wurde gestoppt: ihr Kindprozess hat {seconds} s lang kein Lebenszeichen gezeigt, weder Ausgabe noch Prozessorzeit (Frage {question}, letzte Phase {phase}, längster Stillstand der Ereignisschleife {lag} s; ein toter Partner, eine Sperre, die niemand freigibt, oder ein eingefrorener Prozess): {detail}"'
    t=sub(t,old_en,new_en);t=sub(t,old_de,new_de);return t
rw("texts.json",tx)
print("ok")

def si(t):
    t=sub(t,'''export async function preflightToolClosuresV1(workspaceRoot, scripts) {
  const refused = [], names = Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(), view = Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0], ports = Object.keys(providerDesignationV1(scripts).providers).sort();
  const attempt = async (name, run) => { try { await run(); } catch (error) { if (!isRepoErrorV1(error)) throw error; refused.push({ name, ...repoErrorDataV1(error) }); } };''','''export async function preflightToolClosuresV1(workspaceRoot, scripts, { progress } = {}) {
  const refused = [], names = Object.keys(PROVIDER_CLOSURE_POLICY_V1.tools.closures).sort(), view = Object.keys(PROVIDER_CLOSURE_POLICY_V1.views)[0], ports = Object.keys(providerDesignationV1(scripts).providers).sort(), total = names.length + ports.length;
  let done = 0;
  const attempt = async (name, run) => { try { await run(); } catch (error) { if (!isRepoErrorV1(error)) throw error; refused.push({ name, ...repoErrorDataV1(error) }); } await progress?.({ done: ++done, total }); };''')
    t=sub(t,'a closure that can never be derived (it imports a generated output, names an unknown port) is known before a read derives anything. `refused` holds','a closure that can never be derived (it imports a generated output, names an unknown port) is known before a read derives anything. `progress({done,total})` is awaited after each closure, so a caller can let its event loop turn between the closures (the capped child of the heavy questions reports its phase and its heartbeat there). `refused` holds')
    return t
rw("si.mjs",si)

def s2(t):
    t=sub(t,'writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.primeSharedClosuresV1(root!,(derive as any).scripts))));','writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.primeSharedClosuresV1(root!,(derive as any).scripts,{progress}))));')
    t=sub(t,'}else writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.preflightToolClosuresV1(root!,(derive as any).scripts))));','}else writeFileSync(output!,JSON.stringify(await observeInferenceSnapshotV1(()=>scriptInputs.preflightToolClosuresV1(root!,(derive as any).scripts,{progress}))));')
    t=sub(t,'  const scriptInputs=await import("../🟨️.mjs")','  const progress=async({done,total}:{done:number;total:number})=>{phase=(request.prime?"prime:":"preflight:")+done+"/"+total;await new Promise(turn=>setImmediate(turn));};\n  const scriptInputs=await import("../🟨️.mjs")')
    return t
t=open(f"{S}/stage/s.ts",encoding="utf8").read();open(f"{S}/stage/s.ts","w",encoding="utf8").write(s2(t))

def tt(t):
    t=sub(t,'import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,isHeavyQuestionV1,','import{HEAVY_QUESTION_POLICY_V1,askHeavyQuestionV1,heavyQuestionScaleV1,isHeavyQuestionV1,')
    t=sub(t,'import{mkdtempSync,rmSync,writeFileSync}from"node:fs";','import{mkdtempSync,rmSync,writeFileSync}from"node:fs";\nimport{spawn}from"node:child_process";\nimport{fileURLToPath}from"node:url";')
    t=sub(t,'const FAST={...HEAVY_QUESTION_POLICY_V1,silenceMilliseconds:1500,timeoutMilliseconds:60000};','const FAST={...HEAVY_QUESTION_POLICY_V1,silenceMilliseconds:1500,livenessMilliseconds:1000,timeoutMilliseconds:60000},ONE={scale:()=>1};')
    t=t.replace('policy:FAST,','policy:FAST,ports:ONE,')
    t=sub(t,'expect(renderRepoErrorV1(reason,"en")).toContain("nothing for 2 s");expect(renderRepoErrorV1(reason,"de")).toContain("nichts geschrieben");','expect(renderRepoErrorV1(reason,"en")).toContain("no sign of life for 2 s");expect(renderRepoErrorV1(reason,"de")).toContain("kein Lebenszeichen");')
    t=sub(t,'test("law: a long prime does not hold','''const REAL=fileURLToPath(new URL("../📜️script.ts",import.meta.url)),ROOT=resolve(import.meta.dir,"../".repeat(11));

test("law: a child that computes without printing is not stopped, because liveness is progress and not output",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-busy-"));
 try{
  const script=fakeChild(directory,"busy.cjs","const end=Date.now()+4500;while(Date.now()<end){}fs.writeFileSync(output,JSON.stringify({fact:3,rows:[]}));process.exit(0);");
  const began=Date.now();
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script})).toEqual({fact:3,rows:[]});
  expect(Date.now()-began,"it ran longer than the silence window").toBeGreaterThan(FAST.silenceMilliseconds*2);
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: a child that is truly dead is stopped within the bound derived from the machine, and the refusal names the question, the last phase and the silent seconds in both languages",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-dead-"));
 try{
  const script=fakeChild(directory,"dead.cjs","process.stderr.write('\\\\u0001alive lease:store 0\\\\n');setTimeout(()=>Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0),100);setInterval(()=>{},1000);");
  const began=Date.now(),reason=await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:{scale:()=>2},script}).then(()=>undefined,(error:unknown)=>error as any);
  const took=Date.now()-began;
  expect(took,"the window is the authored silence times the scale").toBeGreaterThanOrEqual(2900);expect(took).toBeLessThan(15000);
  expect(reason.key).toBe("build.graph.heavy-question-silent");
  const english=renderRepoErrorV1(reason,"en"),german=renderRepoErrorV1(reason,"de");
  for(const text of[english,german]){expect(text).toContain("derived");expect(text).toContain("lease:store");expect(text).toMatch(/[34] s/u);}
  expect(english).toContain("last phase");expect(german).toContain("letzte Phase");
 }finally{rmSync(directory,{recursive:true,force:true});}
},40000);

test("law: a child that waits on a named lease with its loop turning is waiting, not stalled, however long the wait outlasts the silence window",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-lease-"));
 try{
  const script=fakeChild(directory,"lease.cjs","process.stderr.write('Process 7 (repo show) holds the dependency store since now; waiting for it\\\\n');const timer=setInterval(()=>process.stderr.write('\\\\u0001alive lease:dependency-store 0\\\\n'),300);setTimeout(()=>{clearInterval(timer);fs.writeFileSync(output,JSON.stringify({fact:4,rows:[]}));process.exit(0);},4800);");
  expect(await askHeavyQuestionV1(QUESTION,{workspaceRoot:directory,policy:FAST,ports:ONE,script})).toEqual({fact:4,rows:[]});
 }finally{rmSync(directory,{recursive:true,force:true});}
},30000);

test("law: the window is the speed factor times the load per core, never below the authored time and never above the maximum, from an independent model",()=>{
 const small={SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES:"8192",SEMIO_HOST_CAPACITY_CORES:"4"},model=(load:number)=>Math.min(HEAVY_QUESTION_POLICY_V1.maximumScale,Math.ceil(Math.max(1,8/4)*Math.sqrt(16384/8192)*100-1e-9)/100*Math.max(1,load/4));
 for(const load of[0,2,4,8,12,100])expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,load),"load "+load).toBeCloseTo(model(load),9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,small,0)).toBeCloseTo(2.83,9);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},0)).toBeGreaterThanOrEqual(1);
 expect(heavyQuestionScaleV1(HEAVY_QUESTION_POLICY_V1,{},1e6)).toBe(HEAVY_QUESTION_POLICY_V1.maximumScale);
});

test("law: the capped child turns its event loop during the preflight, names its phase and reports the lag of each turn (the 60 s of silence came from a stretch of 12 to 20 s that wrote nothing)",async()=>{
 const child=spawn(process.execPath,[REAL,ROOT,join(tmpdir(),"heavy-beat-"+process.pid+".json")],{stdio:["pipe","ignore","pipe"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:"2560"}});
 const beats:string[][]=[];let tail="";child.stderr.on("data",chunk=>{tail+=chunk;const lines=tail.split("\\n");tail=lines.pop()!;for(const line of lines){const match=/^\\u0001alive (\\S+) (\\d+)$/u.exec(line);if(match)beats.push([match[1]!,match[2]!]);}});
 child.stdin.end(JSON.stringify({preflight:true}));
 const code=await new Promise<number|null>(resolveClose=>child.once("close",resolveClose));
 expect(code).toBe(0);
 expect(beats.filter(([phase])=>/^preflight:\\d+\\/\\d+$/u.test(phase!)).length,"heartbeats named a preflight step").toBeGreaterThanOrEqual(2);
 expect(beats.some(([phase])=>phase==="preflight:39/39"||/^preflight:3[0-9]\\/39$/u.test(phase!))).toBe(true);
},600000);

test("law: a child whose parent is killed ends within seconds although it is still computing (two real processes)",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"heavy-orphan-"));
 try{
  const parent=join(directory,"parent.cjs");
  writeFileSync(parent,'const{spawn}=require("node:child_process");const c=spawn(process.execPath,[process.argv[2],process.argv[3],process.argv[4]],{stdio:["pipe","ignore","pipe"]});c.stdin.end(JSON.stringify({preflight:true}));let said=false;c.stderr.on("data",d=>{if(!said&&String(d).includes("alive preflight")){said=true;console.log("CHILD "+c.pid);}});setInterval(()=>{},1000);');
  const holder=spawn(process.execPath,[parent,REAL,ROOT,join(directory,"out.json")],{stdio:["ignore","pipe","ignore"],env:{...process.env,[HEAVY_QUESTION_POLICY_V1.environment]:"2560"}});
  const pid=await new Promise<number>((resolvePid,rejectPid)=>{let text="";holder.stdout.on("data",chunk=>{text+=chunk;const match=/CHILD (\\d+)/u.exec(text);if(match)resolvePid(Number(match[1]));});holder.once("close",()=>rejectPid(new Error("the parent ended before the child said preflight")));});
  const lives=()=>{try{process.kill(pid,0);return true;}catch{return false;}};
  expect(lives(),"the child computes while its parent is alive").toBe(true);
  holder.kill("SIGKILL");
  const began=Date.now();while(lives()&&Date.now()-began<20000)await new Promise(resolveTimer=>setTimeout(resolveTimer,200));
  expect(lives(),"the orphan outlived its parent by "+(Date.now()-began)+" ms").toBe(false);expect(Date.now()-began).toBeLessThan(6000);
 }finally{rmSync(directory,{recursive:true,force:true});}
},600000);

test("law: a long prime does not hold''')
    return t
t=open(f"{S}/base/t.ts",encoding="utf8").read();open(f"{S}/stage/t.ts","w",encoding="utf8").write(tt(t))
