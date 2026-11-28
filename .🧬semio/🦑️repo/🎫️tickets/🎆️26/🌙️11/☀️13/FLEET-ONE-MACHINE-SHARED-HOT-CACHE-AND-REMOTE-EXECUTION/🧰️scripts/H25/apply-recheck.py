import os,sys,tempfile
U="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/"
def edit(path,pairs):
    p=U+path;s=open(p,encoding="utf8").read()
    for old,new in pairs:
        if s.count(old)!=1:sys.exit("anchor not unique/missing in "+path+": "+old[:70])
        s=s.replace(old,new)
    fd,t=tempfile.mkstemp(dir=os.path.dirname(p));os.write(fd,s.encode());os.close(fd);os.replace(t,p)
edit("🧬️schema/🔣️.json",[
('"parkAfterMilliseconds": 3000\n','"parkAfterMilliseconds": 3000,\n          "recheckAfterMilliseconds": 10000\n'),
('        "claimLostAfterPark": {','''        "recheckHit": {
          "en": "[unit-cache] {crate}: another machine stored the unit while this one waited for local admission; installed from the fleet cache instead of compiling",
          "de": "[unit-cache] {crate}: eine andere Maschine hat die Einheit gespeichert, während diese auf die lokale Zulassung wartete; aus dem Flotten-Cache installiert statt kompiliert"
        },
        "claimLostAfterPark": {''')])
edit("⏳️admit/🟦️.ts",[
('|"claimLostAfterPark";','|"claimLostAfterPark"|"recheckHit";'),
('parkAfterMilliseconds:number;headroom','parkAfterMilliseconds:number;recheckAfterMilliseconds:number;headroom'),
('parkAfterMilliseconds:UNIT_CACHE_POLICY_V1.slot.parkAfterMilliseconds,','parkAfterMilliseconds:UNIT_CACHE_POLICY_V1.slot.parkAfterMilliseconds,recheckAfterMilliseconds:UNIT_CACHE_POLICY_V1.slot.recheckAfterMilliseconds,'),
('claim:"kept"|"lost"|"none"}>;\n\n/** 🪂️','claim:"kept"|"lost"|"none";served:boolean}>;\n\n/** 🪂️'),
('hold?:Pick<FleetHoldV1,"park"|"resume">|undefined;report','hold?:Pick<FleetHoldV1,"park"|"resume">|undefined;recheck?:()=>Promise<boolean>;report'),
(' return Object.freeze({slot,waitedMs:Date.now()-started,timedOut,claim:outcome.claim});',
''' let served=false;
 if(world.recheck!==undefined&&(Date.now()-started>=bounds.recheckAfterMilliseconds||outcome.claim==="lost")){
  served=await world.recheck().catch(()=>false);
  if(served){slot?.release();slot=undefined;say("recheckHit");world.event(Date.now()-started,"recheck-hit: the unit was stored by another machine while this one waited");}
 }
 return Object.freeze({slot,waitedMs:Date.now()-started,timedOut,claim:outcome.claim,served});'''),
(' * @see ../🧪️tests/🪢️wedge/🟦️.ts\n * @see ../🛰️fleet/🟦️.ts (`FleetHoldV1`) */\nexport async function awaitCompileAdmissionV1',' * A unit that waited at least `recheckAfterMilliseconds` (or lost its claim while parked) asks `recheck` once more before it runs: another machine may have stored it meanwhile, then `served` is true, the slot is given back and the caller must not compile.\n * @see ../🧪️tests/🪢️wedge/🟦️.ts\n * @see ../🧪️tests/🔄️recheck/🟦️.ts\n * @see ../🛰️fleet/🟦️.ts (`FleetHoldV1`) */\nexport async function awaitCompileAdmissionV1'),
])
edit("📜️script.ts",[
('type Outcome=Readonly<{status:number;stdout:string;stderr:string;peakRssBytes?:number}>;','type Outcome=Readonly<{status:number;stdout:string;stderr:string;peakRssBytes?:number;served?:true}>;'),
('const compile=async(capture:boolean,localInputKey?:string):Promise<Outcome>=>{','const compile=async(capture:boolean,localInputKey?:string,recheck?:()=>Promise<boolean>):Promise<Outcome>=>{'),
('try{slot=(await awaitCompileAdmissionV1({root:settings.root,slots:settings.slots,crate,since:started,environment,facts:alone?undefined:admitFactsV1(args,environment,cwd),alone,hold,signal:stop.signal,','try{const admission=await awaitCompileAdmissionV1({root:settings.root,slots:settings.slots,crate,since:started,environment,facts:alone?undefined:admitFactsV1(args,environment,cwd),alone,hold,...recheck?{recheck}:{},signal:stop.signal,'),
('count:reason=>countFallbackV1(settings.root,reason)})).slot;}','count:reason=>countFallbackV1(settings.root,reason)});if(admission.served)return{status:0,stdout:"",stderr:"",served:true};slot=admission.slot;}'),
('  const outcome=remote??await compile(true,localInputKey);\n','''  const recheck=fleet&&breaker==="ok"?async()=>await fetchFleetUnitV1(fleet!)==="hit":undefined;
  let local=remote===undefined?await compile(true,localInputKey,recheck):undefined;
  if(local?.served){
   await hold?.release();hold=undefined;
   try{if(await serve(fleet!.localKey)){event("hit",key,Date.now()-started,waited,"after-admission-wait");lookup("fleet-hit",fleet!.key);countUnitV1(workspaceOf,"remoteHit");await awaitFleetSettlesV1();return 0;}}catch{}
   countFallbackV1(settings.root,"hit-unservable");local=await compile(true,localInputKey);
  }
  const outcome=remote??local!;
'''),
('import {awaitFleetSettlesV1,explainFleetMissV1,','import {awaitFleetSettlesV1,explainFleetMissV1,fetchFleetUnitV1,'),
])
print("ok")
