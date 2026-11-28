#!/usr/bin/env python3
"""H46: exact-match atomic patch (run once at the landing slot; re-run is a no-op/refusal)."""
import os,sys,tempfile
R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=R+"🏗️build/"
RES=B+"🏡️resident/"
edits=[]
def E(path,old,new,count=1):edits.append((path,old,new,count))

# 1 consent: bounded retry window instead of a day
C=RES+"🔐️consent/🟨️.mjs"
E(C,'const REFUSAL_FILE_V1="launchd-refused.json",REFUSAL_MAXIMUM_AGE_V1=24*3600_000,BOOT_TOLERANCE_V1=60_000;',
'''const REFUSAL_FILE_V1="launchd-refused.json",BOOT_TOLERANCE_V1=60_000;

/** ⏳️ How long a remembered refusal of the launchd route holds: afterwards the next service start probes the route again (a start that is refused again costs one failed start and is remembered for another window), so a consent granted later, a volume that came back or a launcher that was rebuilt is found without a reboot. */
export const LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1=10*60_000;''')
E(C,'/** 🧠️ The refusal of the launchd route remembered for this checkout, or `undefined`: only a record written after the machine booted, for the same checkout and the same launcher bytes, not older than a day. */',
'/** 🧠️ The refusal of the launchd route remembered for this checkout, or `undefined`: only a record written after the machine booted, for the same checkout and the same launcher bytes, not older than `LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1` (then the route is probed again). */')
E(C,'at-value.at>REFUSAL_MAXIMUM_AGE_V1','at-value.at>LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1')
E(RES+"🔐️consent/🧬️schema/🔣️.json",'written after the machine booted, at most a day old.','written after the machine booted, at most ten minutes old (afterwards the route is probed again).')

# 2 host composition: bilingual reason with time and next probe
H=RES+"🏗️composition/🌐️host/🟨️.mjs"
E(H,'import {ACCESS_PROOF_VERSION_V1,accessDirectoryV1,consentIdentityV1,createMountReproofV1,launchdRefusalV1,','import {ACCESS_PROOF_VERSION_V1,LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1,accessDirectoryV1,consentIdentityV1,createMountReproofV1,launchdRefusalV1,')
E(H,'''  if(remembered)process.stderr.write("The graph service runs under the access of the calling app: the launchd route was refused for "+(remembered.place||cwd)+" earlier since this machine started / Der Graphdienst läuft mit dem Zugriff der aufrufenden App: die launchd-Route wurde für "+(remembered.place||cwd)+" seit dem Start dieses Rechners bereits abgewiesen\\n");''',
'''  if(remembered){const place=remembered.place||cwd,clock=new Date(remembered.at).toTimeString().slice(0,5),minutes=Math.max(1,Math.ceil((remembered.at+LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1-Date.now())/60_000));process.stderr.write("The graph service runs under the access of the calling app: the launchd route was refused for "+place+" at "+clock+"; it is probed again in "+minutes+" min / Der Graphdienst läuft mit dem Zugriff der aufrufenden App: die launchd-Route wurde für "+place+" um "+clock+" abgewiesen; sie wird in "+minutes+" Min. erneut geprüft\\n");}''')

# 3 resident index export
E(RES+"🟨️.mjs",'rememberLaunchdRefusalV1,launchdRefusalV1,','rememberLaunchdRefusalV1,launchdRefusalV1,LAUNCHD_REFUSAL_RETRY_MILLISECONDS_V1,')

# 4 client: a service lost while it starts is started again, bounded
P=RES+"🚪️client/🛰️plan/🟨️.mjs"
E(P,'attempts=3,deadlineMs,ignorePids,silenceMilliseconds,identify}=options;','attempts=3,deadlineMs,ignorePids,silenceMilliseconds,identify,lostStarts=HOST_LOST_START_RESTARTS_V1}=options;')
E(P,'let last,reattached=0,keyWaits=0,keyRestarted=false,previous;','let last,reattached=0,keyWaits=0,keyRestarted=false,previous,lostRestarts=0;')
E(P,'''   const{socket,runtime,implementation:connected}=await connectHostV1({implementation,...identify===undefined?{}:{identify},environment,platform,spawnService,signal,onProgress,languages,...ignorePids?{ignorePids}:{}});''',
'''   let started;
   try{started=await connectHostV1({implementation,...identify===undefined?{}:{identify},environment,platform,spawnService,signal,onProgress,languages,...ignorePids?{ignorePids}:{}});}
   catch(error){
    if(!(error instanceof HostUnavailableErrorV1&&error.reason==="lost"&&spawnService!==undefined&&lostRestarts<lostStarts.maximum))throw error;
    last=error;lostRestarts++;attempt--;
    onProgress({phase:"start",done:0,total:0,message:{en:"The graph service ended while it started ("+error.message.replace(/^Host graph service unavailable \\(lost\\): /u,"").split("\\n")[0].slice(0,300)+"); starting it again, "+lostRestarts+" of "+lostStarts.maximum+" / Der Graphdienst endete beim Start ("+error.message.replace(/^Host graph service unavailable \\(lost\\): /u,"").split("\\n")[0].slice(0,300)+"); er wird erneut gestartet, "+lostRestarts+" von "+lostStarts.maximum}});
    await sleep(lostStarts.milliseconds*lostRestarts,signal);
    continue;
   }
   const{socket,runtime,implementation:connected}=started;''')
E(P,'export const PLAN_KEY_WAIT_V1=Object.freeze({maximum:30,milliseconds:1000});','''export const PLAN_KEY_WAIT_V1=Object.freeze({maximum:30,milliseconds:1000});

/** 🔁️ How often a request starts the graph service again when it ended while it started (it never wrote an exit record: its sources moved under a landing, the machine was out of memory for the start, a peer ended it) and the growing pause before each start (`milliseconds` times the number of the restart): the request then waits for the new service instead of failing its stage; only a service that is lost `maximum` times in a row ends the request as `lost`. */
export const HOST_LOST_START_RESTARTS_V1=Object.freeze({maximum:3,milliseconds:5000});''')

# 5 authorizers: export membership classification
A=R+"⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"
E(A,'function membershipChangeV1(error:unknown):boolean{','export function membershipChangeV1(error:unknown):boolean{')

# 6 compiler admission: the build layer of the churn protocol retries a closure that kept moving
K=B+"📥️source/🔏️closure/🏗️compiler/🟦️.ts"
E(K,'import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";',
'import{captureNxAuthorizerSourcesV1,membershipChangeV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";\nimport{concurrentEditV1,retryOnConcurrentEditV1}from"../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";')
E(K,'captured=await profilePhaseV1("source.capture",()=>captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true,report:profileProgressV1,...(memo?{memo}:{})}));',
'captured=await profilePhaseV1("source.capture",()=>retryOnConcurrentEditV1("build",async()=>{try{return await captureNxAuthorizerSourcesV1(root,entry,control.signal,{installed:true,report:profileProgressV1,...(memo?{memo}:{})});}catch(error){control.signal.throwIfAborted();if(!membershipChangeV1(error))throw error;throw concurrentEditV1("vanished",(error as{path?:string}).path??entry,String((error as Error)?.message??error));}},{signal:control.signal,...(environment?{environment}:{})}));')


# 7 tests
T=RES+"🧪️tests/🔐️consent/🟦️.ts"
E(T,'another checkout or a day"','another checkout or ten minutes"')
E(T,'expect(at(base+23*3600_000,24*3600),"booted before the record: still the same boot").toBeDefined();','expect(at(base+9*60_000,24*3600),"booted before the record: still the same boot").toBeDefined();')
E(T,'expect(at(base+25*3600_000,30*3600),"older than a day").toBeUndefined();','expect(at(base+11*60_000,30*3600),"older than the retry window: the route is probed again").toBeUndefined();')
NEW=RES+"🧪️tests/🔁️lost-start/🟦️.ts"
if "--check" not in sys.argv and "--scratch" not in sys.argv and not os.path.exists(NEW):
    os.makedirs(os.path.dirname(NEW),exist_ok=True)
    open(NEW,"w",encoding="utf8").write(open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"lost-start.test.ts"),encoding="utf8").read())
    print("wrote",NEW[-60:])

# group + verify
by={}
for p,o,n,c in edits:by.setdefault(p,[]).append((o,n,c))
out={}
for p,rows in by.items():
    s=open(p,encoding="utf8").read()
    for o,n,c in rows:
        if c==0:continue
        if n in s and o not in s:print("already applied:",o[:60]);continue
        if s.count(o)!=1:print("MISMATCH",p[-60:],o[:80],s.count(o));sys.exit(2)
        s=s.replace(o,n)
    out[p]=s
if "--check" in sys.argv:print("ok, all anchors found");sys.exit(0)
if "--scratch" in sys.argv:
    d=sys.argv[sys.argv.index("--scratch")+1];os.makedirs(d,exist_ok=True)
    for i,(p,s) in enumerate(out.items()):open(os.path.join(d,str(i)+os.path.splitext(p)[1]),"w",encoding="utf8").write(s)
    sys.exit(0)
for p,s in out.items():
    fd,t=tempfile.mkstemp(dir=os.path.dirname(p),prefix=".h46-");os.write(fd,s.encode());os.close(fd);os.chmod(t,os.stat(p).st_mode);os.rename(t,p);print("wrote",p[-70:])
