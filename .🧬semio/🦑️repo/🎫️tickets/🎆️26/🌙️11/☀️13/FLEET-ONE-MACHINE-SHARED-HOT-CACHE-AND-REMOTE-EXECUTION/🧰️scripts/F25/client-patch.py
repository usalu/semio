import sys,hashlib
R="/Users/ueli/Documents/semio/"
B=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/"
def edit(path,pairs):
    t=open(path,encoding="utf8").read();h0=hashlib.sha256(t.encode()).hexdigest()[:12]
    for old,new in pairs:
        if t.count(old)!=1: sys.exit(f"anchor {t.count(old)} {path}: {old[:80]!r}")
        t=t.replace(old,new)
    open(path,"w",encoding="utf8").write(t);print(path[-40:],h0,"->",hashlib.sha256(t.encode()).hexdigest()[:12])
edit(B+"🚪️client/🛰️plan/🟨️.mjs",[
("function startOutputV1(path,from,since,lost=false){","""/** 🌱️ The phase the starting service reports in its log (`[repo] still working after N s; last phase started: <phase>`), with the files it walked, or nothing while it has said none.
 * @see ./🧬️schema/🔣️.json */
function bootPhaseV1(path,from){
 let file;
 try{
  file=openSync(path,"r");const facts=fstatSync(file),start=Math.max(from<=facts.size?from:0,facts.size-8192),buffer=Buffer.alloc(facts.size-start);
  if(buffer.length===0)return undefined;
  const text=buffer.subarray(0,readSync(file,buffer,0,buffer.length,start)).toString("utf8"),found=[...text.matchAll(/^\\[repo\\] still working after \\d+ s; last phase started: (\\S+)(?: \\(walked (\\d+) files)?/gmu)].at(-1);
  return found===undefined?undefined:found[1]+(found[2]===undefined?"":" ("+found[2]+" files)");
 }catch{return undefined;}finally{if(file!==undefined)try{closeSync(file);}catch{}}
}

function startOutputV1(path,from,since,lost=false){"""),
('if(elapsed-announced>=2000){announced=elapsed;onProgress({phase:"start",done:0,total:0,message:say("host-wait",{elapsed})});}',
 'if(elapsed-announced>=2000){announced=elapsed;const booting=bootPhaseV1(runtime.log,initialLogOffset);onProgress({phase:"start",done:0,total:0,message:booting===undefined?say("host-wait",{elapsed}):say("host-wait-phase",{elapsed,phase:booting})});}'),
])
edit(B+"🧬️schema/🔣️.json",[
('      "host-wait": "Waiting for the repo graph service ({elapsed} ms).",\n','      "host-wait": "Waiting for the repo graph service ({elapsed} ms).",\n      "host-wait-phase": "Waiting for the repo graph service ({elapsed} ms); it is in {phase}.",\n'),
('      "host-wait": "Warte auf den Repo-Graphdienst ({elapsed} ms).",\n','      "host-wait": "Warte auf den Repo-Graphdienst ({elapsed} ms).",\n      "host-wait-phase": "Warte auf den Repo-Graphdienst ({elapsed} ms); er befindet sich in {phase}.",\n'),
])
