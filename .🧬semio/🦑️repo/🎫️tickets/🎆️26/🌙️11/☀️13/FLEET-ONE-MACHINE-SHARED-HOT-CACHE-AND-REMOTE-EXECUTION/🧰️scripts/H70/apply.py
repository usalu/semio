import json,os,sys,tempfile
ROOT=os.environ.get("H70_ROOT","/Users/ueli/Documents/semio")
R=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
HERE=os.path.dirname(os.path.abspath(__file__))
def edit(path,pairs):
    text=open(path,encoding="utf8").read()
    for old,new in pairs:
        if text.count(old)!=1:sys.exit("anchor count %d: %s in %s"%(text.count(old),old[:100],path))
        text=text.replace(old,new)
    return text
def write(path,text):
    os.makedirs(os.path.dirname(path),exist_ok=True)
    d=tempfile.NamedTemporaryFile("w",encoding="utf8",dir=os.path.dirname(path),delete=False,suffix=".tmp")
    d.write(text);d.close();os.replace(d.name,path)

TREE_J=R+"/🌲️process-tree/🧬️schema/🔣️.json"
TREE_T=R+"/🌲️process-tree/🟦️.ts"
MAIN_J=R+"/🧬️schema/🔣️.json"
MAIN_T=R+"/🟦️.ts"
WRAP=R+"/🏃️wrapper/🟦️.ts"

SENDER_TEXTS='''    "SenderTexts": {
      "const": {
        "en": {
          "supervisorSelf": "Stopped by this readiness supervisor (process {supervisor}) at {time}: {reason}. The reason was recorded in the stage record before the signal was sent.",
          "supervisorOther": "Stopped by another readiness supervisor (process {supervisor}, not the one that watched the end) at {time}: {reason}. The reason was recorded in the stage record before the signal was sent.",
          "deadline": "Ended by the own deadline of the stage wrapper ({signal} at {time}); the wrapper recorded it before it sent the signal.",
          "warden": "Ended by the host warden at {time}: it cancelled the {kind} ({reason}, {megabytes} MB).",
          "runner": "Ended by a signal that the stage runner recorded in its kill record (runner process {killer}, {signal} at {time}).",
          "outsideWrapper": "{signal} reached the stage wrapper at {time} and no record of this machine explains it: no supervisor stop in the stage record, no warden cancel, no wrapper deadline and no kill record of the stage runner ({rows} other signal(s) of the runner during the run). The sender is outside the stage runner (another program, the end of a session, the operating system). The platform does not tell its pid.",
          "outsideChild": "The stage wrapper received no signal, but its command ended by {signal}: the process tree below the wrapper was signalled directly, either by code inside the stage (its own cancellation) or by a program outside the stage runner. No supervisor stop, warden cancel, wrapper deadline or kill record of the stage runner explains it ({rows} other signal(s) of the runner during the run). The platform does not tell the pid of the sender."
        },
        "de": {
          "supervisorSelf": "Von diesem Bereitschafts-Aufseher (Prozess {supervisor}) um {time} gestoppt: {reason}. Der Grund wurde vor dem Senden des Signals im Stufenprotokoll vermerkt.",
          "supervisorOther": "Von einem anderen Bereitschafts-Aufseher (Prozess {supervisor}, nicht der, der das Ende beobachtete) um {time} gestoppt: {reason}. Der Grund wurde vor dem Senden des Signals im Stufenprotokoll vermerkt.",
          "deadline": "Durch die eigene Frist des Stufen-Wrappers beendet ({signal} um {time}); der Wrapper hat es vor dem Senden des Signals vermerkt.",
          "warden": "Vom Host-Wächter um {time} beendet: er brach {kind} ab ({reason}, {megabytes} MB).",
          "runner": "Durch ein Signal beendet, das der Stufenläufer in seinem Kill-Protokoll vermerkt hat (Läuferprozess {killer}, {signal} um {time}).",
          "outsideWrapper": "{signal} erreichte den Stufen-Wrapper um {time}, und kein Protokoll dieser Maschine erklärt es: kein Aufseher-Stopp im Stufenprotokoll, kein Wächter-Abbruch, keine Wrapper-Frist und kein Kill-Protokoll des Stufenläufers ({rows} andere(s) Signal(e) des Läufers während des Laufs). Der Absender liegt außerhalb des Stufenläufers (anderes Programm, Ende einer Sitzung, Betriebssystem). Die Plattform nennt seine pid nicht.",
          "outsideChild": "Der Stufen-Wrapper erhielt kein Signal, aber sein Befehl endete durch {signal}: der Prozessbaum unter dem Wrapper wurde direkt angesprochen, entweder von Code in der Stufe (die eigene Abbruchlogik) oder von einem Programm außerhalb des Stufenläufers. Kein Aufseher-Stopp, Wächter-Abbruch, keine Wrapper-Frist und kein Kill-Protokoll des Stufenläufers erklärt es ({rows} andere(s) Signal(e) des Läufers während des Laufs). Die Plattform nennt die pid des Absenders nicht."
        }
      }
    },
'''

TREE_CODE='''/** 🗞️ The signals of the stage runner that its kill record `kills.log` lists since `sinceMs`: time, pid of the process that received the signal, the signal, the runner process that sent it and its stage, parsed from the English half of each line (the line `Process <pid> (<name>) received <signal> from the ready stage runner of this worker (runner process <killer>, stage <stage>, ...)`); lines of other forms are ignored. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export type ReadyKillRowV1=Readonly<{atMs:number;pid:number;signal:"SIGTERM"|"SIGKILL";killer:number;stage:string}>;
export function readyKillRowsV1(text:string,sinceMs:number):readonly ReadyKillRowV1[]{
 const rows:ReadyKillRowV1[]=[],expression=/^(\\S+) Process (\\d+) \\(.*?\\) received (SIGTERM|SIGKILL) from the ready stage runner of this worker \\(runner process (\\d+), stage ([^,]+),/u;
 for(const line of text.split("\\n").slice(-P.senderRecordLines)){
  const match=expression.exec(line);
  if(match===null)continue;
  const atMs=Date.parse(match[1]!);
  if(Number.isFinite(atMs)&&atMs>=sinceMs)rows.push({atMs,pid:Number(match[2]),signal:match[3] as "SIGTERM"|"SIGKILL",killer:Number(match[4]),stage:match[5]!});
 }
 return rows;
}

/** 📟️ Who stopped a stage whose command ended by a signal, answered only from what this machine recorded before the signal was sent: `supervisor` (the stage record carries `stopBy`), `deadline` (the wrapper recorded its own deadline), `warden` (a cancel row of the host warden names the stage process), `runner` (the kill record of the stage runner names the stage process), and otherwise `outside-wrapper` (the wrapper itself received the signal) or `outside-child` (only the tree below it was signalled); `none` when the stage did not end by a signal (`ended` is `null`). A record counts only within `senderMatchMilliseconds` of the end, so an old row never explains a later end. The platform does not tell the pid of a sender, so an unrecorded sender is named by what it was not. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export type ReadyStopSenderV1=Readonly<{kind:"supervisor"|"deadline"|"warden"|"runner"|"outside-wrapper"|"outside-child"|"none";line:string}>;
export type ReadyStopSenderInputV1=Readonly<{ended:string|null;stagePid:number;endedMs:number;observer:number;stopBy?:Readonly<{supervisor:number;reason:string;atMs:number}>;received?:readonly Readonly<{signal:string;atMs:number}>[];sent?:readonly Readonly<{signal:string;reason:string;atMs:number}>[];kills:readonly ReadyKillRowV1[];warden?:Readonly<{atMs:number;kind:string;reason:string;megabytes:number}>}>;
export function readyStopSenderV1(input:ReadyStopSenderInputV1,policy:typeof P=P):ReadyStopSenderV1{
 if(input.ended===null)return{kind:"none",line:""};
 const texts=schema.definitions.SenderTexts.const,near=(atMs:number):boolean=>Math.abs(input.endedMs-atMs)<=policy.senderMatchMilliseconds,time=(atMs:number):string=>new Date(atMs).toISOString();
 const say=(key:keyof typeof texts.en,values:Readonly<Record<string,string|number>>):string=>(["en","de"] as const).map(language=>texts[language][key].replace(/\\{(\\w+)\\}/gu,(_all,name:string)=>String(values[name]??""))).join(" / ");
 const rowOf=input.kills.find(row=>row.pid===input.stagePid&&near(row.atMs)),others=input.kills.filter(row=>row!==rowOf).length;
 const deadline=input.sent?.find(row=>row.reason==="deadline"&&near(row.atMs));
 if(deadline!==undefined)return{kind:"deadline",line:say("deadline",{signal:deadline.signal,time:time(deadline.atMs)})};
 const by=input.stopBy;
 if(by!==undefined&&near(by.atMs))return{kind:"supervisor",line:say(by.supervisor===input.observer?"supervisorSelf":"supervisorOther",{supervisor:by.supervisor,time:time(by.atMs),reason:by.reason})};
 const warden=input.warden;
 if(warden!==undefined&&near(warden.atMs))return{kind:"warden",line:say("warden",{time:time(warden.atMs),kind:warden.kind,reason:warden.reason,megabytes:warden.megabytes})};
 if(rowOf!==undefined)return{kind:"runner",line:say("runner",{killer:rowOf.killer,signal:rowOf.signal,time:time(rowOf.atMs)})};
 const first=input.received?.[0];
 return first!==undefined?{kind:"outside-wrapper",line:say("outsideWrapper",{signal:first.signal,time:time(first.atMs),rows:others})}:{kind:"outside-child",line:say("outsideChild",{signal:input.ended,rows:others})};
}

/** 🪧️ Where the stall clock of a stage starts that a supervisor adopts without the watchdog history of the one that started it (no record, or one the current schema does not accept): at the adoption, never at the start of the stage. The adopting supervisor cannot know how long the stage was quiet, so a stage is never stopped for lack of history; with history the recorded clock and the time nobody watched are used instead. A pure function.
 * @see ./🧪️tests/🛑️stop-sender */
export function readyAdoptionClockV1(input:Readonly<{startedMs:number;observedMs:number}>):number{return Math.max(input.startedMs,input.observedMs);}

'''

tree_json=edit(TREE_J,[
 ('        "staleWaitingMilliseconds": 900000,\n','        "staleWaitingMilliseconds": 900000,\n        "senderMatchMilliseconds": 60000,\n        "senderRecordLines": 4000,\n        "senderRecordBytes": 524288,\n'),
 ('    "Fixture": {',SENDER_TEXTS+'    "Fixture": {'),
])
tree_ts=edit(TREE_T,[
 ('/** 🔐️ Whether the read of the stage is queued for the storage mapping:',TREE_CODE+'/** 🔐️ Whether the read of the stage is queued for the storage mapping:'),
])

main_json=edit(MAIN_J,[
 ('"stop": {"$ref": "#/definitions/StageStop"}, "toolsGraceDenial"','"stop": {"$ref": "#/definitions/StageStop"}, "stopBy": {"$ref": "#/definitions/StopBy"}, "toolsGraceDenial"'),
 ('    "StageStop": {"enum": ["timeout", "cancel", "stall", "overdue", "restart"]},\n','    "StageStop": {"enum": ["timeout", "cancel", "stall", "overdue", "restart"]},\n    "StopBy": {"type": "object", "additionalProperties": false, "required": ["supervisor", "reason", "atMs"], "properties": {"supervisor": {"type": "integer", "minimum": 1}, "reason": {"$ref": "#/definitions/StageStop"}, "atMs": {"type": "integer", "minimum": 1}}, "description": "Who stopped the stage and why, written into the stage record BEFORE the signal is sent (also for a stage that has no launch record): the readiness supervisor that sends it, the reason and the time. A supervisor that adopts the stage later, or any reader of the end, finds the sender here instead of an unknown SIGTERM."},\n    "ReceivedSignals": {"type": "array", "maxItems": 8, "items": {"type": "object", "additionalProperties": false, "required": ["signal", "atMs"], "properties": {"signal": {"type": "string"}, "atMs": {"type": "integer"}}}, "description": "The termination signals the stage wrapper itself received (and forwarded to its command), with the time: a command that ended by a signal without an entry here was signalled directly below the wrapper."},\n    "SentSignals": {"type": "array", "maxItems": 4, "items": {"type": "object", "additionalProperties": false, "required": ["signal", "reason", "atMs"], "properties": {"signal": {"type": "string"}, "reason": {"enum": ["deadline"]}, "atMs": {"type": "integer"}}}, "description": "The signals the stage wrapper sent to its command by its own deadline, written before they are sent."},\n'),
 ('"properties": {"exit": {"type": ["integer", "null"]}, "signal": {"type": ["string", "null"]}, "spawnFailed": {"type": "boolean"}, "endedMs": {"type": "integer"}}},','"properties": {"exit": {"type": ["integer", "null"]}, "signal": {"type": ["string", "null"]}, "spawnFailed": {"type": "boolean"}, "endedMs": {"type": "integer"}, "received": {"$ref": "#/definitions/ReceivedSignals"}, "sent": {"$ref": "#/definitions/SentSignals"}}},'),
])

main_ts=edit(MAIN_T,[
 ('readyWardenCancelOfStageV1,readyGrantWaitV1,','readyWardenCancelOfStageV1,readyKillRowsV1,readyStopSenderV1,readyAdoptionClockV1,readyGrantWaitV1,'),
 ('launch?:ReadyStageLaunchV1;stop?:ReadyStageStopV1}>&ReadyStopEvidenceV1;','launch?:ReadyStageLaunchV1;stop?:ReadyStageStopV1;stopBy?:Readonly<{supervisor:number;reason:ReadyStageStopV1;atMs:number}>}>&ReadyStopEvidenceV1;'),
 ('wardenCancel?:Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"megabytes">;','wardenCancel?:Pick<HostWardenCancelV1,"reason"|"kind"|"pid"|"megabytes">;sender?:string;'),
 ('   const seen=[tracker.observe(recent),graphCounter.observe(recent)].some(Boolean);\n   if(seen)try{progressAt=stallAt=Math.max(inflight.startedMs,Math.min(observedAt,Math.floor(statSync(inflight.log).mtimeMs)));}catch{}\n',
  '   tracker.observe(recent);graphCounter.observe(recent);\n   progressAt=stallAt=readyAdoptionClockV1({startedMs:inflight.startedMs,observedMs:observedAt});\n'),
 ('const ending=(signal:string|null,why?:string):Readonly<{tail:string[];failure:string[]}>=>{const note=signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why],evidence=',
  'const ending=(signal:string|null,why?:string,extra:readonly string[]=[]):Readonly<{tail:string[];failure:string[]}>=>{const note=[...signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why],...extra],evidence='),
 ('   if(held.launch!==undefined)try{writeReadyLocalRecordV1(inflightFile,{...held,stop:reason,...evidence});}catch(error){failWatch(error);return false;}\n',
  '   const stopBy={supervisor:process.pid,reason,atMs:Date.now()};\n   if(held.launch!==undefined)try{writeReadyLocalRecordV1(inflightFile,{...held,stop:reason,...evidence,stopBy});}catch(error){failWatch(error);return false;}\n   else try{writeReadyLocalRecordV1(inflightFile,{...held,stopBy});}catch{}\n'),
 ('''    let exit:number|null=null,spawnFailed=false,signal:string|null=null,endedMs:number|undefined;
     try{
      const row=JSON.parse(readFileSync(inflight.exitFile,"utf8")) as {exit:number|null;signal?:string|null;spawnFailed:boolean;endedMs?:number};
      exit=row.exit;spawnFailed=row.spawnFailed;signal=row.signal??null;
      if(!validateJsonSchemaSubset(schema.definitions.Exit,row,schema).length)endedMs=row.endedMs;
     }catch{}
    return complete({exit,signal,timedOut,spawnFailed,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...exit===0||killed?{}:wardenEnd(),...exit===0?{tail:logTailV1(inflight.log)}:ending(signal)},killed||signal!==null,endedMs);
''','''    let exit:number|null=null,spawnFailed=false,signal:string|null=null,endedMs:number|undefined,received:readonly {signal:string;atMs:number}[]|undefined,sent:readonly {signal:string;reason:string;atMs:number}[]|undefined;
     try{
      const row=JSON.parse(readFileSync(inflight.exitFile,"utf8")) as {exit:number|null;signal?:string|null;spawnFailed:boolean;endedMs?:number;received?:{signal:string;atMs:number}[];sent?:{signal:string;reason:string;atMs:number}[]};
      exit=row.exit;spawnFailed=row.spawnFailed;signal=row.signal??null;
      if(!validateJsonSchemaSubset(schema.definitions.Exit,row,schema).length){endedMs=row.endedMs;received=row.received;sent=row.sent;}
     }catch{}
    const ended=exit===0?null:signal??(exit!==null&&(P.externalStopExits as readonly number[]).includes(exit)?`exit status ${exit}`:null);
    const sender=ended===null?undefined:stopSenderV1(ended,endedMs??Date.now(),received,sent);
    const stoppedBy=!killed&&signal!==null&&sender?.kind==="supervisor"?readInflightV1(inflightFile)?.stopBy:undefined;
    if(stoppedBy!==undefined){timedOut=stoppedBy.reason==="timeout";cancelled=stoppedBy.reason==="cancel";stalled=readyStoppedForStallV1(stoppedBy.reason);killed=true;}
    return complete({exit,signal,timedOut,spawnFailed,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...exit===0||killed?{}:wardenEnd(),...sender===undefined||sender.kind==="none"?{}:{sender:sender.kind},...exit===0?{tail:logTailV1(inflight.log)}:ending(signal,undefined,sender===undefined||sender.kind==="none"?[]:[sender.line])},killed||signal!==null,endedMs);
'''),
 ('  const wardenEnd=():Pick<ReadyRunResultV1,"wardenCancel">=>{',
  '''  const stopSenderV1=(ended:string,endedAt:number,received:readonly {signal:string;atMs:number}[]|undefined,sent:readonly {signal:string;reason:string;atMs:number}[]|undefined)=>{
   let kills:ReturnType<typeof readyKillRowsV1>=[],warden:Parameters<typeof readyStopSenderV1>[0]["warden"];
   try{const file=join(directory,contain.killRecordName),size=statSync(file).size,fd=openSync(file,"r");try{const length=Math.min(size,treePolicy.senderRecordBytes),buffer=Buffer.alloc(length);readSync(fd,buffer,0,length,size-length);kills=readyKillRowsV1(buffer.toString("utf8"),inflight.startedMs);}finally{closeSync(fd);}}catch{}
   try{const row=readyWardenCancelOfStageV1(readHostWardenCancelsSinceV1(inflight.startedMs),inflight.pid);if(row!==undefined)warden={atMs:row.atMs,kind:row.kind,reason:row.reason,megabytes:row.megabytes};}catch{}
   const stopBy=readInflightV1(inflightFile)?.stopBy;
   return readyStopSenderV1({ended,stagePid:inflight.pid,endedMs:endedAt,observer:process.pid,kills,...stopBy===undefined?{}:{stopBy},...received===undefined?{}:{received},...sent===undefined?{}:{sent},...warden===undefined?{}:{warden}},treePolicy);
  };
  const wardenEnd=():Pick<ReadyRunResultV1,"wardenCancel">=>{'''),
])

wrapper=edit(WRAP,[
 ('const end=(exit:number|null,spawnFailed:boolean,signal:string|null=null):void=>{\n writeFileSync(`${exitFile}.tmp`,JSON.stringify({exit,signal,spawnFailed,endedMs:Date.now()}));',
  'const received:{signal:string;atMs:number}[]=[],sent:{signal:string;reason:"deadline";atMs:number}[]=[];\nconst end=(exit:number|null,spawnFailed:boolean,signal:string|null=null):void=>{\n writeFileSync(`${exitFile}.tmp`,JSON.stringify({exit,signal,spawnFailed,endedMs:Date.now(),...received.length>0?{received}:{},...sent.length>0?{sent}:{}}));'),
 ('terminate:()=>{try{child.kill("SIGTERM");}catch{}},kill:()=>{try{child.kill("SIGKILL");}catch{}}});',
  'terminate:()=>{sent.push({signal:"SIGTERM",reason:"deadline",atMs:Date.now()});try{child.kill("SIGTERM");}catch{}},kill:()=>{if(sent.length<4)sent.push({signal:"SIGKILL",reason:"deadline",atMs:Date.now()});try{child.kill("SIGKILL");}catch{}}});'),
 ('for(const name of["SIGTERM","SIGINT"] as const)process.on(name,()=>{try{child.kill(name);}catch{}});',
  'for(const name of["SIGTERM","SIGINT","SIGHUP"] as const)process.on(name,()=>{if(received.length<8)received.push({signal:name,atMs:Date.now()});try{child.kill(name);}catch{}});'),
])
write(TREE_J,tree_json);write(TREE_T,tree_ts);write(MAIN_J,main_json);write(MAIN_T,main_ts);write(WRAP,wrapper)
for rel,src in(("🧪️tests/🛑️stop-sender/🟦️.ts","stop-sender.ts"),("🧪️tests/🛑️stop-sender/🐍️.py","stop-sender.py")):
    write(R+"/"+rel,open(HERE+"/"+src,encoding="utf8").read())
print("applied")
