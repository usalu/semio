test("the command words parse like the catalogue says and refuse everything else, in both languages",()=>{
 expect(parseRemoteCacheArgumentsV1(["serve","--port","0","--host=127.0.0.1","--directory","cache","--max-bytes=1048576","--read-only","--tls-certificate","c.pem","--tls-key=k.pem"])).toMatchObject({operation:"serve",values:{"--port":"0","--host":"127.0.0.1","--directory":"cache","--max-bytes":"1048576","--tls-certificate":"c.pem","--tls-key":"k.pem"}});
 expect(parseRemoteCacheArgumentsV1(["gc","--dry-run","--json"]).flags).toEqual(new Set(["--dry-run","--json"]));
 for(const bad of [[],["clear"],["remote","status"],["serve","--port"],["serve","--port","--host"],["serve","--unknown","1"],["serve","--port="],["serve","--dry-run"],["gc","--read-only"],["status","--max-bytes","1"],["gc","--dry-run","--dry-run"],["serve","--read-only=true"]])expect(()=>parseRemoteCacheArgumentsV1(bad),bad.join(" ")).toThrow();
 let message="";try{parseRemoteCacheArgumentsV1(["serve","--bogus"]);}catch(error){message=(error as Error).message;}
 expect(message).toContain("Unknown option for remote-cache serve: --bogus");expect(message).toContain("Unbekannte Option für remote-cache serve: --bogus");
 expect(remoteCacheLanguagesV1("de",{})).toEqual(["de"]);expect(remoteCacheLanguagesV1(undefined,{SEMIO_LANGUAGE:"en"})).toEqual(["en"]);expect(remoteCacheLanguagesV1(undefined,{LANG:"de_DE.UTF-8"})).toEqual(["de"]);
 expect(remoteCacheLanguagesV1(undefined,{LANG:"C"}),"no default language: both are printed").toEqual(["en","de"]);
 expect(()=>remoteCacheLanguagesV1("fr",{})).toThrow("Unbekannte Sprache");
 expect(remoteCacheTextV1("de","serve.stopped")).toBe("Der Remote-Cache wurde beendet.");
});

test("every message exists in English and German with the same placeholders",()=>{
 const names=(value:string)=>[...value.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const [key,row] of Object.entries(texts.messages as Record<string,{en:string;de:string}>)){
  expect(Object.keys(row).sort(),key).toEqual(["de","en"]);expect(names(row.de),key).toEqual(names(row.en));
  if(key!=="gc.mode.apply")expect(row.en.length>0&&row.de.length>0&&row.en!==row.de||key.startsWith("serve.transport"),key).toBe(true);
 }
});

const toolsFor=async()=>[nxRemoteCacheToolV1(),bazelRemoteCacheToolV1()];

test("remote-cache status reports the contract, both renderings, a probe per protocol, the server and the store, redacts the token, and obeys the contract's token and cancellation",async()=>{
 const folder=directory("status-store"),server=await serve({directory:folder,tokens:{read:"read-only-token-9",write:"write-token-7"}}),written:string[]=[];
 const context=(environment:Record<string,string>,signal:AbortSignal=new AbortController().signal):RemoteCacheCommandContextV1=>({root:directory("root"),environment,signal,progress:async()=>{},write:line=>written.push(line),tools:toolsFor});
 try{
  expect((await fetch(server.url+"/v1/cache/777",{method:"PUT",body:"payload",headers:{authorization:"Bearer write-token-7"}})).status).toBe(200);
  const on={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"write-token-7"};
  const off=await remoteCacheStatusV1([],context({}));
  expect(off).toEqual({version:1,configured:false,contract:null,effectiveUpload:false,token:null,renderings:null,probes:null,server:null,store:null});expect(validator("Status")(off),JSON.stringify(validator("Status").errors)).toBe(true);
  const status=await remoteCacheStatusV1(["--directory",folder],context(on)) as {configured:boolean;probes:{protocol:string;reachable:boolean;authorized:boolean;status:number}[];server:{readOnly:boolean;validation:string;entries:number};store:{entries:number;bytes:number}};
  expect(validator("Status")(status),JSON.stringify(validator("Status").errors)).toBe(true);
  expect(status).toMatchObject({configured:true,effectiveUpload:true,token:{present:true},renderings:{nx:{mechanism:"nx-environment"},bazel:{mechanism:"bazel-flags"}},server:{readOnly:false,validation:"off",entries:1},store:{entries:1,bytes:7}});
  expect(status.probes.map(probe=>probe.protocol)).toEqual(["nx-self-hosted","bazel-http"]);for(const probe of status.probes)expect(probe).toMatchObject({reachable:true,authorized:true,status:404});
  expect(JSON.stringify(status),"never prints the token").not.toContain("write-token-7");
  const denied=await remoteCacheStatusV1([],context({...on,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"})) as {probes:{authorized:boolean;status:number}[];server:null};
  for(const probe of denied.probes)expect(probe).toMatchObject({authorized:false,status:401});expect(denied.server).toBeNull();
  const custom=await remoteCacheStatusV1([],context({...on,SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT:"CI_CACHE_TOKEN",CI_CACHE_TOKEN:"read-only-token-9"})) as {token:{environment:string};probes:{authorized:boolean}[];server:{entries:number}};
  expect(custom.token.environment).toBe("CI_CACHE_TOKEN");for(const probe of custom.probes)expect(probe.authorized).toBe(true);expect(custom.server.entries).toBe(1);
  const unreachable=await remoteCacheStatusV1([],context({SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"2"})) as {probes:{reachable:boolean;status:null}[]};
  for(const probe of unreachable.probes)expect(probe).toMatchObject({reachable:false,status:null});
  const cancelled=new AbortController();cancelled.abort();await expect(remoteCacheStatusV1([],context(on,cancelled.signal))).rejects.toThrow();
  expect(await runRemoteCacheCommandV1(["status"],context({}))).toEqual({exitCode:0,result:expect.objectContaining({configured:false})});
  await expect(runRemoteCacheCommandV1(["remote","clear"],context(on))).rejects.toThrow("Expected repo remote-cache serve|status|gc.");
 }finally{await server.stop();}
},60000);

async function listening(lines:string[]):Promise<string>{
 return new Promise<string>((resolve,reject)=>{const started=Date.now(),timer=setInterval(()=>{const line=lines.find(candidate=>candidate.startsWith("[remote-cache] listening "));if(line){clearInterval(timer);resolve(line.slice(25));}else if(Date.now()-started>20000){clearInterval(timer);reject(Error("no listening line"));}},20);});
}

test("remote-cache serve opens the store, announces its address in the chosen language, publishes progress and stops when cancelled",async()=>{
 const stages:string[]=[],lines:string[]=[],controller=new AbortController(),folder=directory("serve");
 const context:RemoteCacheCommandContextV1={root:folder,environment:{SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN:"write-token-7"},signal:controller.signal,progress:async stage=>{stages.push(stage);},write:line=>lines.push(line),tools:toolsFor};
 const running=serveRemoteCacheV1(["--port","0","--directory","store","--max-bytes","1048576","--lang","en"],context);
 const address=await listening(lines);
 expect(address).toMatch(/^http:\/\/127\.0\.0\.1:[0-9]+$/u);
 expect((await fetch(address+"/healthz")).status).toBe(200);expect((await fetch(address+"/status")).status).toBe(401);
 expect((await fetch(address+"/v1/cache/abc",{method:"PUT",body:"x",headers:{authorization:"Bearer write-token-7"}})).status).toBe(200);
 controller.abort();expect(await running).toEqual({exitCode:0});
 expect(stages).toEqual(expect.arrayContaining(["remote-cache:open","remote-cache:scan","remote-cache:ready","remote-cache:serving"]));
 expect(lines.some(line=>line.startsWith("Serving the remote cache on "+address+" (read-write, plain HTTP, result validation complete)"))).toBe(true);
 expect(lines.some(line=>line.includes("Clients set SEMIO_REMOTE_CACHE_ENDPOINT="+address))).toBe(true);expect(lines.some(line=>line.includes("No token is set"))).toBe(false);
 expect(lines.slice(-2)).toEqual(["[remote-cache] stopped","The remote cache stopped."]);
 await expect(fetch(address+"/healthz")).rejects.toThrow();
 const german:string[]=[],second=new AbortController(),again=serveRemoteCacheV1(["--port","0","--directory","store","--lang","de","--read-only"],{...context,environment:{},signal:second.signal,write:line=>german.push(line)});
 await listening(german);second.abort();await again;
 expect(german.some(line=>line.startsWith("Der Remote-Cache läuft auf ")&&line.includes("nur lesend")&&line.includes("einfaches HTTP"))).toBe(true);expect(german.some(line=>line.includes("Kein Token gesetzt"))).toBe(true);expect(german.at(-1)).toBe("Der Remote-Cache wurde beendet.");
 const both:string[]=[],third=new AbortController(),neutral=serveRemoteCacheV1(["--port","0","--directory","store"],{...context,environment:{LANG:"C"},signal:third.signal,write:line=>both.push(line)});
 await listening(both);third.abort();await neutral;
 expect(both.filter(line=>line==="The remote cache stopped."||line==="Der Remote-Cache wurde beendet.").length,"without a language both are printed").toBe(2);
 expect(both.some(line=>line.includes("result validation complete"))&&both.some(line=>line.includes("Ergebnisprüfung vollständig")),"every part of a line follows its language").toBe(true);
 const quiet=new AbortController();
 await expect(serveRemoteCacheV1(["--port","70000"],context)).rejects.toThrow("--port must be a whole number from 0 to 65535");
 await expect(serveRemoteCacheV1(["--host","0.0.0.0","--port","0"],{...context,environment:{},signal:quiet.signal})).rejects.toThrow("A token is required to listen beyond loopback.");
 await expect(serveRemoteCacheV1(["--tls-certificate","only.pem"],context)).rejects.toThrow("TLS needs both a certificate and a key.");
 await expect(serveRemoteCacheV1(["--tls-certificate","missing.pem","--tls-key","missing.key"],context)).rejects.toThrow("TLS file required");
 const holder=new AbortController(),held:string[]=[],first=serveRemoteCacheV1(["--port","0","--directory","busy"],{...context,signal:holder.signal,write:line=>held.push(line)});
 await listening(held);await expect(serveRemoteCacheV1(["--port","0","--directory","busy"],context)).rejects.toThrow("Ein anderer Remote-Cache-Server benutzt");
 holder.abort();await first;
},90000);

test("remote-cache gc plans first, then removes exactly the planned entries, in the chosen language, and reports JSON",async()=>{
 const folder=directory("gc"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**22,maximumEntryBytes:2**20,maximumAgeMilliseconds:0}),lines:string[]=[];
 for(let index=0;index<12;index++){await store.put("main","nx",`key${index}`,(async function*(){yield new Uint8Array(100_000).fill(index);})(),{mode:"keep",declaredLength:100_000});}
 await store.close();
 const context=(write:(line:string)=>void,environment:Record<string,string>={}):RemoteCacheCommandContextV1=>({root:directory("root"),environment,signal:new AbortController().signal,progress:async()=>{},write,tools:toolsFor});
 expect(await collectRemoteCacheV1(["--directory",join(folder,"missing")],context(line=>lines.push(line),{SEMIO_LANGUAGE:"en"}))).toEqual({exitCode:0});expect(lines.at(-1)).toContain("does not exist; nothing to collect.");
 const plan=await collectRemoteCacheV1(["--directory",folder,"--max-bytes","1048576","--dry-run","--json"],context(()=>{})) as {exitCode:number;result:{dryRun:boolean;scanned:{entries:number};removed:{entries:number;budget:number;bytes:number};remaining:{entries:number}}};
 expect(plan.result).toMatchObject({dryRun:true,scanned:{entries:12},removed:{budget:plan.result.removed.entries},remaining:{entries:12-plan.result.removed.entries}});expect(plan.result.removed.entries).toBeGreaterThan(0);
 const mixed:string[]=[];await collectRemoteCacheV1(["--directory",folder,"--dry-run"],context(line=>mixed.push(line),{LANG:"C"}));
 expect(mixed.some(line=>line.includes("(plan only)"))&&mixed.some(line=>line.includes("(nur Plan)"))).toBe(true);
 const store2=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});expect(store2.usage().entries,"a plan removes nothing").toBe(12);await store2.close();
 const german:string[]=[],applied=await collectRemoteCacheV1(["--directory",folder,"--max-bytes","1048576","--lang","de"],context(line=>german.push(line)));
 expect(applied).toEqual({exitCode:0});expect(german.at(-1)).toContain(`${plan.result.removed.entries} von 12 Einträgen entfernt`);expect(german[0]).toContain("Sammle");
 const store3=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});expect(store3.usage().entries).toBe(plan.result.remaining.entries);expect(store3.usage().bytes).toBeLessThanOrEqual(1048576*0.9);
 expect(store3.has("main","nx","key11"),"the newest entry survives").toBe(true);expect(store3.has("main","nx","key0"),"the oldest entry goes first").toBe(false);await store3.close();
 const cancelling=new AbortController();cancelling.abort();
 expect(await collectRemoteCacheV1(["--directory",folder,"--dry-run"],{...context(()=>{},{SEMIO_LANGUAGE:"en"}),signal:cancelling.signal})).toEqual({exitCode:130});
});

test("the public JSON of the remote cache is schema-first: status, gc results and the message catalogue validate against the contract schema, and drifted documents are refused",async()=>{
 const server=await serve({tokens:{}}),status=await (await fetch(server.url+"/status")).json() as Record<string,any>;
 try{
  expect(validator("ServerStatus")(status),JSON.stringify(validator("ServerStatus").errors)).toBe(true);requireRemoteCacheShapeV1("ServerStatus",status,"x");
  const drifted:[string,(value:Record<string,any>)=>void][]=[["extra key",value=>{value.extra=1;}],["missing counter",value=>{delete value.counters.nx;}],["negative count",value=>{value.counters.ac.hits=-1;}],["extra counter field",value=>{value.counters.cas.errors=0;}],["unknown validation",value=>{value.validation="weird";}],["protocol order",value=>{value.protocols.reverse();}],["start time",value=>{value.startedAt="yesterday";}],["version",value=>{value.version=2;}]];
  for(const [name,mutate] of drifted){const copy=structuredClone(status);mutate(copy);expect(validator("ServerStatus")(copy),name).toBe(false);expect(()=>requireRemoteCacheShapeV1("ServerStatus",copy,"x"),name).toThrow();}
  expect(validator("TextCatalogue")(texts),JSON.stringify(validator("TextCatalogue").errors)).toBe(true);
  for(const bad of [{...texts,languages:["de","en"]},{...texts,messages:{...texts.messages,"x.y":{en:"only"}}},{...texts,messages:{...texts.messages,"Bad Key":{en:"a",de:"b"}}},{...texts,extra:true}])expect(validator("TextCatalogue")(bad)).toBe(false);
  const folder=directory("json"),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**22,maximumEntryBytes:2**20,maximumAgeMilliseconds:0});
  for(let index=0;index<6;index++)await store.put("main","nx",`k${index}`,(async function*(){yield new Uint8Array(1000);})(),{mode:"keep",declaredLength:1000});await store.close();
  const context:RemoteCacheCommandContextV1={root:directory("root"),environment:{},signal:new AbortController().signal,progress:async()=>{},write:()=>{},tools:toolsFor};
  const results:unknown[]=[];
  for(const args of [["--directory",join(folder,"none"),"--json"],["--directory",folder,"--json","--dry-run"],["--directory",folder,"--json","--max-bytes","1048576"]]){const outcome=await collectRemoteCacheV1(args,context) as {result:unknown};results.push(outcome.result);expect(validator("GcResult")(outcome.result),JSON.stringify(validator("GcResult").errors)).toBe(true);}
  expect(results[0]).toMatchObject({present:false});expect(results[1]).toMatchObject({dryRun:true,scanned:{entries:6}});
  for(const bad of [{...(results[0] as object),present:true},{...(results[1] as object),extra:1},{version:1,dryRun:true,directory:"x"}])expect(validator("GcResult")(bad)).toBe(false);
 }finally{await server.stop();}
 for(const bad of [{maximumBytes:0,maximumEntryBytes:1,maximumAgeMilliseconds:0},{maximumBytes:10,maximumEntryBytes:1.5,maximumAgeMilliseconds:0},{maximumBytes:10,maximumEntryBytes:1,maximumAgeMilliseconds:-1},{maximumBytes:10,maximumEntryBytes:1,maximumAgeMilliseconds:0,extra:1},{maximumBytes:10,maximumEntryBytes:1}]){
  expect(validator("StoreBounds")(bad)).toBe(false);if(!("extra" in bad))await expect(RemoteCacheStoreV1.open({directory:directory("bounds"),...bad} as never)).rejects.toThrow("Finite remote cache bounds required");
 }
 expect(validator("StoreBounds")({maximumBytes:10,maximumEntryBytes:10,maximumAgeMilliseconds:0})).toBe(true);
});
