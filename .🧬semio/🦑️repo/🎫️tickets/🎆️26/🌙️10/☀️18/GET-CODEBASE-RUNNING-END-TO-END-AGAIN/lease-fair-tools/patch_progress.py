import json,sys
root=sys.argv[1]
sch=root+'/🧬️schema/📈️progress.json'
fx=root+'/🧪️tests/📈️progress/🔣️.json'
tst=root+'/🧪️tests/📈️progress/🟦️.ts'
src=root+'/🟦️.ts'
def load(p): return json.loads(open(p,encoding='utf8').read())
def save(p,d): open(p,'w',encoding='utf8').write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
s=load(sch)
assert 'queueWait' not in s['const']
s['const']['queueWait']={
 "resourcePrefix":"lease-queue:",
 "maximumResourceCharacters":1024,
 "maximumCommandCharacters":256,
 "commandPattern":"^[A-Za-z0-9:_/.-]+$",
 "labels":{
  "en":{"template":"{command} · Waiting for access to {resource} ({mode}): position {position} of {queued}, behind the request of process {holderPid} ({holderMode}, queued {holderSeconds} s ago)","modes":{"shared":"shared","exclusive":"exclusive"}},
  "de":{"template":"{command} · Wartet auf Zugriff auf {resource} ({mode}): Position {position} von {queued}, hinter der Anfrage von Prozess {holderPid} ({holderMode}, seit {holderSeconds} s eingereiht)","modes":{"shared":"geteilt","exclusive":"exklusiv"}}
 }}
s['definitions']['QueueWait']={"type":"object","additionalProperties":False,"required":["version","position","queued","holderMode","holderPid","holderSeconds","resource"],"properties":{
 "version":{"const":1},
 "position":{"type":"integer","minimum":2,"maximum":999999},
 "queued":{"type":"integer","minimum":2,"maximum":999999},
 "holderMode":{"enum":["shared","exclusive"]},
 "holderPid":{"type":"integer","minimum":1,"maximum":9999999999},
 "holderSeconds":{"type":"integer","minimum":0,"maximum":31536000},
 "resource":{"type":"string","minLength":1,"maxLength":401}}}
s['definitions']['QueueCases']=s['definitions']['ParserCases']
save(sch,s)
f=load(fx)
def row(name,env,payload,command,mode,expected,elapsed=0):
    return {"name":name,"environment":env,"resource":"lease-queue:"+json.dumps(payload,separators=(',',':'),ensure_ascii=False),"command":command,"mode":mode,"elapsedMs":elapsed,"expected":expected}
p={"version":1,"position":3,"queued":15,"holderMode":"exclusive","holderPid":12528,"holderSeconds":8040,"resource":"native-fixed-image:/repo/.cache/b/d"}
f['queueCases']=[
 row("english-behind-exclusive",{"LANG":"en_US.UTF-8"},p,"bun:dependency-storage","shared","bun:dependency-storage · Waiting for access to native-fixed-image:/repo/.cache/b/d (shared): position 3 of 15, behind the request of process 12528 (exclusive, queued 8040 s ago)"),
 row("german-behind-shared",{"SEMIO_DASHBOARD_LOCALE":"de"},{**p,"holderMode":"shared","position":2,"queued":2,"holderPid":4620,"holderSeconds":0},"repository:command","exclusive","repository:command · Wartet auf Zugriff auf native-fixed-image:/repo/.cache/b/d (exklusiv): position".replace("position","Position")+" 2 von 2, hinter der Anfrage von Prozess 4620 (geteilt, seit 0 s eingereiht)"),
 row("fallback-language-is-english",{"LANG":"C"},p,"compiler","exclusive","compiler · Waiting for access to native-fixed-image:/repo/.cache/b/d (exclusive): position 3 of 15, behind the request of process 12528 (exclusive, queued 8040 s ago)"),
 row("position-one-is-not-a-queue-wait",{"LANG":"en"},{**p,"position":1},"compiler","shared",None),
 row("position-beyond-queue-refused",{"LANG":"en"},{**p,"position":16},"compiler","shared",None),
 row("unknown-field-refused",{"LANG":"en"},{**p,"extra":True},"compiler","shared",None),
 row("elapsed-must-be-zero",{"LANG":"en"},p,"compiler","shared",None,1),
 row("command-outside-pattern-refused",{"LANG":"en"},p,"bad command","shared",None),
]
f['queueCases'][1]['resource']=f['queueCases'][1]['resource']
save(fx,f)
t=open(tst,encoding='utf8').read()
assert 'queue wait' not in t
t+='''

test("queue wait names the head request and position in the selected language at the actual writer",async()=>{
 const declaration=contract.const.queueWait,locale=contract.const.sourceAdmission,database=new Database(":memory:"),{const:_declaration,...caseSchema}=contract,validate=new Ajv({strict:true}).compile({...caseSchema,$ref:"#/definitions/QueueCases"});expect(validate(fixture.queueCases)).toBe(true);expect(compileJsonSchemaSubset({...caseSchema,$ref:"#/definitions/QueueCases"}).validate(fixture.queueCases)).toEqual([]);
 try{for(const row of fixture.queueCases){
  const progress:ScriptProgress={owner:"process-progress-fixture",command:row.command,stage:"running",activity:{resource:row.resource,mode:row.mode as "shared"|"exclusive",elapsedMs:row.elapsedMs}},language=(database.query("select coalesce((select lower(substr(json_extract(?1,'$.'||value),1,2)) from json_each(?2) where lower(substr(json_extract(?1,'$.'||value),1,2)) in ('en','de') and (length(json_extract(?1,'$.'||value))=2 or substr(json_extract(?1,'$.'||value),3,1) in ('_','-','.','@')) order by cast(key as integer) limit 1),json_extract(?3,'$[0]')) value").get(JSON.stringify(row.environment),JSON.stringify(locale.systemVariables),JSON.stringify(locale.fallback))as{value:"en"|"de"}).value,payload=row.resource.slice(declaration.resourcePrefix.length),labels=declaration.labels[language];
  const oracle=database.query("SELECT CASE WHEN substr(?1,1,length(?2))=?2 AND length(?1)<=?3 AND length(?4) BETWEEN 1 AND ?5 AND ?4 NOT GLOB '*[^A-Za-z0-9:_/.-]*' AND ?6=0 AND json_valid(?7) AND json_type(?7)='object' AND (SELECT count(*) FROM json_each(?7))=7 AND json_type(?7,'$.version')='integer' AND json_extract(?7,'$.version')=1 AND json_type(?7,'$.position')='integer' AND json_extract(?7,'$.position') BETWEEN 2 AND 999999 AND json_type(?7,'$.queued')='integer' AND json_extract(?7,'$.queued') BETWEEN 2 AND 999999 AND json_extract(?7,'$.position')<=json_extract(?7,'$.queued') AND json_extract(?7,'$.holderMode') IN ('shared','exclusive') AND json_type(?7,'$.holderPid')='integer' AND json_extract(?7,'$.holderPid') BETWEEN 1 AND 9999999999 AND json_type(?7,'$.holderSeconds')='integer' AND json_extract(?7,'$.holderSeconds') BETWEEN 0 AND 31536000 AND json_type(?7,'$.resource')='text' AND length(json_extract(?7,'$.resource')) BETWEEN 1 AND 401 AND ?8 IN ('shared','exclusive') THEN replace(replace(replace(replace(replace(replace(replace(replace(json_extract(?9,'$.template'),'{command}',?4),'{resource}',json_extract(?7,'$.resource')),'{mode}',json_extract(?9,'$.modes.'||?8)),'{position}',json_extract(?7,'$.position')),'{queued}',json_extract(?7,'$.queued')),'{holderPid}',json_extract(?7,'$.holderPid')),'{holderMode}',json_extract(?9,'$.modes.'||json_extract(?7,'$.holderMode'))),'{holderSeconds}',json_extract(?7,'$.holderSeconds')) ELSE NULL END message").get(row.resource,declaration.resourcePrefix,declaration.maximumResourceCharacters,row.command,declaration.maximumCommandCharacters,row.elapsedMs,payload,row.mode,JSON.stringify(labels))as{message:string|null};
  expect(oracle.message,row.name).toBe(row.expected);expect(formatScriptProcessActivityV1(progress,row.environment),row.name).toBe(row.expected);
  const writes:string[]=[],before=Object.fromEntries(locale.systemVariables.map(name=>[name,process.env[name]])),spy=spyOn(process.stderr,"write").mockImplementation(((text:unknown)=>{writes.push(String(text));return true;})as typeof process.stderr.write);
  try{for(const name of locale.systemVariables)delete process.env[name];Object.assign(process.env,row.environment);await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"queue-progress-fixture",maximumElapsedMilliseconds:0},{fixture:"queue-wait"},Date.now()),async invocation=>{const signal=invocation.control.signal;await invocation.control.publish(progress);await invocation.control.yieldContinuation();requireIssuedScriptInvocationV1(invocation);expect(invocation.control.signal).toBe(signal);expect(signal.aborted).toBe(false);});}finally{spy.mockRestore();for(const[name,value]of Object.entries(before))if(value===undefined)delete process.env[name];else process.env[name]=value;}
  expect(writes,row.name).toEqual([`[script] ${JSON.stringify(progress)}\\n`,...(row.expected===null?[]:[`[script] ${row.expected}\\n`])]);
 }}finally{database.close();}
 console.log("[DEBUG] Actual Original writer localizes the queue wait and preserves every structured event");
});
'''
open(tst,'w',encoding='utf8').write(t)
x=open(src,encoding='utf8').read()
anchor=' if(activity.resource.startsWith(declaration.resourcePrefix)){\n  if(activity.elapsedMs!==0||activity.mode!=="shared"||activity.resource.length>declaration.maximumResourceCharacters'
assert x.count(anchor)==1
block=''' const queue=progressSchema.const.queueWait;
 if(activity.resource.startsWith(queue.resourcePrefix)){
  if(activity.elapsedMs!==0||activity.resource.length>queue.maximumResourceCharacters||progress.command.length>queue.maximumCommandCharacters||!new RegExp(queue.commandPattern,"u").test(progress.command))return null;
  let value:unknown;try{value=JSON.parse(activity.resource.slice(queue.resourcePrefix.length));}catch{return null;}
  if(validateJsonSchemaSubset(progressSchema.definitions.QueueWait,value).length)return null;
  const row=value as {position:number;queued:number;holderMode:"shared"|"exclusive";holderPid:number;holderSeconds:number;resource:string};if(row.position>row.queued)return null;
  const labels=queue.labels[progressLanguage(environment)],fields:Record<string,string>={command:progress.command,resource:row.resource,mode:labels.modes[activity.mode],position:String(row.position),queued:String(row.queued),holderPid:String(row.holderPid),holderMode:labels.modes[row.holderMode],holderSeconds:String(row.holderSeconds)};
  return labels.template.replace(/\\{([A-Za-z]+)\\}/gu,(_token,key:string)=>fields[key]!);
 }
'''
x=x.replace(anchor,block+anchor)
open(src,'w',encoding='utf8').write(x)
print("ok")
