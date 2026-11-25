test("a held head that carries no retention manifest of this code is offered again with unchanged documents, a declared one is not",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:4,records}),fleet=makeFleet(),sealed=await a.read(undefined),client=fleet.client("a"),key=selectedHeadKeyV1({scope,platform});
 const first=await publishSelectedGenerationV1({client,scope,platform,host:a.host,sealed,ownerOf:ownerOfV1});expect(first.stored).toBe(true);
 const current=await client.getResult(key) as Uint8Array,decoded=decodeProtobufV1(REAPI_V1.actionResult,current),files=(decoded.output_files as {path:string;digest:{hash:string;size_bytes:number}}[]);
 const older=encodeProtobufV1(REAPI_V1.actionResult,{output_files:files.map(file=>file.path===policy.retentionManifest?{...file,digest:{hash:files[0]!.digest.hash,size_bytes:files[0]!.digest.size_bytes}}:file),stdout_raw:decoded.stdout_raw});
 expect((await client.putResult(key,older,sha("slot"),sha(current))).stored).toBe(true);
 const offered=await publishSelectedGenerationV1({client,scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,held:first.head});
 expect(offered.stored,"the head without the manifest of this code is replaced").toBe(true);
 const settled=await publishSelectedGenerationV1({client,scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,held:first.head});
 expect(settled).toMatchObject({stored:false,reason:"current"});
});

test("a machine that takes a head published by an older code declares it into the protected class without reading anything",async()=>{
 const{fleet,machines:[a,b]}=world(6,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 await run(fleet,a!);
 const client=fleet.client("a"),key=selectedHeadKeyV1({scope,platform}),current=await client.getResult(key) as Uint8Array,decoded=decodeProtobufV1(REAPI_V1.actionResult,current),files=(decoded.output_files as {path:string;digest:{hash:string;size_bytes:number}}[]);
 const older=encodeProtobufV1(REAPI_V1.actionResult,{output_files:files.filter(file=>file.path!==policy.retentionManifest),stdout_raw:decoded.stdout_raw});
 expect((await client.putResult(key,older,sha("slot"),sha(current))).stored).toBe(true);
 const outcome=await run(fleet,b!);expect(outcome.outcome).toBe("installed");
 const after=decodeProtobufV1(REAPI_V1.actionResult,await client.getResult(key) as Uint8Array),kept=retentionManifestV1();
 expect((after.output_files as {path:string;digest:{hash:string}}[]).filter(file=>file.path===policy.retentionManifest).map(file=>file.digest.hash)).toEqual([kept.hash]);
 expect(typeof await lookSelectedGenerationV1({client,scope,platform,host:b!.host}),"the head is still complete").toBe("object");
});

/** 🤝️ Contract: one machine reads a selection and seals it, every other machine on the same tree state fetches the sealed generation and reads nothing; owners that differ are read again and only those; two machines that start at once read each owner once; the keys name no machine; a corrupt or foreign head is a miss.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{createHash}from"node:crypto";
import{hostname}from"node:os";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{REAPI_V1,decodeProtobufV1,encodeProtobufV1}from"../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import{SELECTED_EXCHANGE_POLICY_V1 as policy,retentionManifestV1,buildSelectedGenerationV1,lookSelectedGenerationV1,publishSelectedGenerationV1,selectedGenerationV1,selectedHeadKeyV1,selectedOwnerDigestV1,selectedRowsByOwnerV1}from"../🟦️.ts";
import{makeFleet,makeMachine,makeRecords,ownerNameV1,ownerOfV1,sha,type FleetV1,type MachineV1,type RecordsV1}from"./🌍️world/🟦️.ts";

const scope=sha("scope-of-the-selection-and-the-registry"),platform="darwin-arm64";
const run=(fleet:FleetV1,machine:MachineV1,extra:Partial<Parameters<typeof selectedGenerationV1>[0]>={})=>selectedGenerationV1({client:fleet.client(machine.name),scope,platform,host:machine.host,owner:machine.name,ownerOf:ownerOfV1,holds:machine.holds,install:machine.install,read:machine.read,claimTtlMilliseconds:400,claimHeartbeatMilliseconds:80,waitSliceMilliseconds:60,waitMaximumMilliseconds:5000,...extra});
const world=(count:number,names:string[])=>{const records=makeRecords(),fleet=makeFleet(),machines=names.map(name=>makeMachine({name,count,records}));return{records,fleet,machines};};
const changed=(machine:MachineV1,indexes:number[])=>{for(const index of indexes)machine.edit(index,`edited-${index}`);};

test("the first machine reads and publishes, a second machine on the same tree installs the generation and reads nothing",async()=>{
 const{fleet,machines:[a,b],records}=world(12,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1];records:RecordsV1};
 const first=await run(fleet,a!);
 expect(first.outcome).toBe("read");expect(first.published).toBe(true);
 expect(a!.counters).toEqual({observations:12,inferences:12,takes:0,reads:1});
 const second=await run(fleet,b!);
 expect(second.outcome).toBe("installed");expect(second.differing).toEqual([]);expect(second.held).toBe(12);
 expect(b!.counters).toEqual({observations:0,inferences:0,takes:0,reads:0});
 expect(b!.installed).toBe(true);
 expect(b!.graph).toBe(a!.graph.replaceAll(a!.root,b!.root));
 expect([...records.inferences.values()].every(count=>count===1)).toBe(true);
});

test("the head names the module graph the reading machine offered, a machine that takes the head adopts that graph before it judges the owners, and a newer graph alone publishes the head again",async()=>{
 const{fleet,machines:[a,b]}=world(6,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 const offered=sha("graph-head-1"),adopted:string[]=[],order:string[]=[];
 const first=await run(fleet,a!,{graph:{offer:async()=>offered,adopt:async()=>{throw new Error("the reader adopts nothing");}}});
 expect(first.published).toBe(true);
 const decoded=decodeProtobufV1(REAPI_V1.actionResult,await fleet.client("a").getResult(selectedHeadKeyV1({scope,platform})) as Uint8Array) as {stdout_raw:Uint8Array};
 expect(JSON.parse(Buffer.from(decoded.stdout_raw).toString("utf8")).closureGraph).toBe(offered);
 const install=b!.install;
 const second=await run(fleet,b!,{graph:{offer:async()=>undefined,adopt:async head=>{adopted.push(head);order.push("adopt");return{outcome:"adopted",confirmed:5,refused:1};}},install:async plan=>{order.push("install");await install(plan);}});
 expect(second.outcome).toBe("installed");expect(adopted).toEqual([offered]);expect(order).toEqual(["adopt","install"]);
 const sealed=await a!.read(undefined),held=JSON.parse(Buffer.from(decoded.stdout_raw).toString("utf8"));
 expect(await publishSelectedGenerationV1({client:fleet.client("a"),scope,platform,host:a!.host,sealed:sealed!,ownerOf:ownerOfV1,held,closureGraph:offered})).toMatchObject({stored:false,reason:"current"});
 expect((await publishSelectedGenerationV1({client:fleet.client("a"),scope,platform,host:a!.host,sealed:sealed!,ownerOf:ownerOfV1,held,closureGraph:sha("graph-head-2")})).stored).toBe(true);
});

test("k changed owners: exactly those are read again, the other owners are held, and the fleet converges on the newest head",async()=>{
 const{fleet,machines:[a,b,c],records}=world(40,["a","b","c"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1,MachineV1];records:RecordsV1};
 await run(fleet,a!);
 changed(b!,[5,20]);
 const second=await run(fleet,b!);
 expect(second.outcome).toBe("repaired");expect(second.differing).toEqual(["o20","o5"]);expect(second.held).toBe(38);expect(second.published).toBe(true);
 expect(b!.counters.observations).toBe(2);expect(b!.counters.reads).toBe(1);
 expect(b!.counters.inferences).toBe(4);
 expect([...records.inferences].filter(([,count])=>count>1).map(([name])=>name).sort()).toEqual(["o20","o21","o5","o6"]);expect([...records.inferences.values()].reduce((sum,count)=>sum+count,0)).toBe(44);
 changed(c!,[5,20]);
 const third=await run(fleet,c!);
 expect(third.outcome).toBe("installed");expect(c!.counters).toEqual({observations:0,inferences:0,takes:0,reads:0});
});

test("one differing owner never rejects the whole generation",async()=>{
 const{fleet,machines:[a,b]}=world(40,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 await run(fleet,a!);changed(b!,[0]);
 const result=await run(fleet,b!);
 expect(result.held).toBe(39);expect(result.differing).toEqual(["o0"]);
});

test("a machine that holds its own digest of an owner decides without touching the tree",async()=>{
 const{fleet,machines:[a,b]}=world(8,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 await run(fleet,a!);
 const mine=new Map<string,string>();
 for(const[owner,rows]of selectedRowsByOwnerV1(await a!.read(undefined).then(sealed=>sealed.rows),a!.host,ownerOfV1))mine.set(owner,selectedOwnerDigestV1(rows));
 let touched=0;
 const counting=async(rows:readonly unknown[])=>{touched++;return b!.holds(rows as never);};
 const result=await run(fleet,b!,{own:()=>mine,holds:counting});
 expect(result.outcome).toBe("installed");expect(touched).toBe(0);
});

test("simultaneous start: one machine reads, the other waits with progress and takes the head, every owner is inferred once fleet-wide",async()=>{
 const{fleet,machines:[a,b],records}=world(16,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1];records:RecordsV1};
 const said:string[]=[],slow=async(differing:readonly string[]|undefined)=>{await Bun.sleep(300);return a!.read(differing);};
 const [first,second]=await Promise.all([run(fleet,a!,{read:slow}),run(fleet,b!,{say:(key,fields)=>said.push(key+":"+JSON.stringify(fields))})]);
 const outcomes=[first.outcome,second.outcome].sort();
 expect(outcomes).toEqual(["read","waited"]);
 expect(said.some(line=>line.startsWith("waiting:"))).toBe(true);expect(said.join("\n")).toContain("\"limit\":5");
 expect([...records.inferences.values()].reduce((sum,count)=>sum+count,0)).toBe(16);
 expect([...records.inferences.values()].every(count=>count===1)).toBe(true);
 expect(a!.counters.observations+b!.counters.observations).toBe(16);
});

test("the reading machine dies: the waiter takes over after the lease and reads with the owner records already published",async()=>{
 const{fleet,machines:[a,b],records}=world(10,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1];records:RecordsV1};
 for(let index=0;index<6;index++){const own=sha(a!.tree.get(a!.file(index))!),dep=index>0?sha(a!.tree.get(a!.file(index-1))!):"";records.records.set(sha(JSON.stringify([index,own,dep])),ownerNameV1(index));records.inferences.set(ownerNameV1(index),1);}
 const key=selectedHeadKeyV1({scope,platform});
 fleet.claims.set(key,{owner:"dead",until:Date.now()+150});
 const said:string[]=[],started=Date.now(),result=await run(fleet,b!,{say:(name)=>said.push(name)});
 expect(result.outcome).toBe("taken-over");expect(said).toContain("waiting");expect(said).toContain("taken-over");
 expect(Date.now()-started).toBeLessThan(3000);
 expect(b!.counters.takes).toBe(6);expect(b!.counters.inferences).toBe(4);
 expect([...records.inferences.values()].every(count=>count===1)).toBe(true);
 void a;
});

test("a holder that keeps renewing is waited for only up to the named bound, then this machine reads",async()=>{
 const{fleet,machines:[a]}=world(4,["a"]) as {fleet:FleetV1;machines:[MachineV1]};
 const key=selectedHeadKeyV1({scope,platform}),beat=setInterval(()=>{fleet.claims.set(key,{owner:"alive",until:Date.now()+300});},50);
 fleet.claims.set(key,{owner:"alive",until:Date.now()+300});
 const said:string[]=[],started=Date.now();
 const result=await run(fleet,a!,{waitMaximumMilliseconds:400,say:(name,fields)=>said.push(name+JSON.stringify(fields))});
 clearInterval(beat);
 expect(result.outcome).toBe("unreachable");expect(Date.now()-started).toBeLessThan(2500);expect(a!.counters.reads).toBe(1);
 expect(said.some(line=>line.includes("alive"))).toBe(true);
});

test("a machine with enough memory reads beside a renewing holder at once, a small machine waits for it",async()=>{
 const key=selectedHeadKeyV1({scope,platform});
 const beside=world(4,["a"]) as {fleet:FleetV1;machines:[MachineV1]};
 const beat=setInterval(()=>{beside.fleet.claims.set(key,{owner:"alive",until:Date.now()+300});},50);
 beside.fleet.claims.set(key,{owner:"alive",until:Date.now()+300});
 const said:string[]=[],started=Date.now(),big=await run(beside.fleet,beside.machines[0]!,{readBesideMegabytes:policy.readBesideMinimumTotalMegabytes,waitMaximumMilliseconds:60000,say:name=>said.push(name)});
 expect(big.outcome).toBe("read");expect(Date.now()-started).toBeLessThan(2000);expect(beside.machines[0]!.counters.reads).toBe(1);expect(said).toContain("reading-beside");expect(said).not.toContain("waiting");
 const small=world(4,["b"]) as {fleet:FleetV1;machines:[MachineV1]};
 small.fleet.claims.set(key,{owner:"alive",until:Date.now()+300});
 const waited:string[]=[],result=await run(small.fleet,small.machines[0]!,{readBesideMegabytes:policy.readBesideMinimumTotalMegabytes-1,waitMaximumMilliseconds:200,say:name=>waited.push(name)});
 clearInterval(beat);
 expect(waited).toContain("waiting");expect(waited).not.toContain("reading-beside");expect(result.outcome).toBe("unreachable");
});

test("a fleet that is down, denied or breaks mid-read never fails or hangs the read",async()=>{
 const{fleet,machines:[a]}=world(5,["a"]) as {fleet:FleetV1;machines:[MachineV1]};
 fleet.down=true;
 const result=await run(fleet,a!);
 expect(result.outcome).toBe("unreachable");expect(result.published).toBe(false);expect(a!.counters.inferences).toBe(5);
 const broken=makeFleet(),client=broken.client("x");
 const raising={...client,getResult:async()=>{throw Error("network");},claim:async()=>{throw Error("network");}} as typeof client;
 const second=makeMachine({name:"y",count:3,records:makeRecords()});
 const again=await selectedGenerationV1({client:raising,scope,platform,host:second.host,owner:"y",ownerOf:ownerOfV1,holds:second.holds,install:second.install,read:second.read});
 expect(again.outcome).toBe("unreachable");expect(second.counters.reads).toBe(1);
});

test("a fleet that answers nothing is waited for only up to the look bound, then this machine reads and infers every owner itself",async()=>{
 const{fleet,machines:[a]}=world(6,["a"]) as {fleet:FleetV1;machines:[MachineV1]};
 const client=fleet.client("a"),calls={count:0},silent={...client,getResult:(key:string)=>calls.count++===0?new Promise<never>(()=>undefined):client.getResult(key)} as typeof client,said:string[]=[],started=Date.now();
 const result=await selectedGenerationV1({client:silent,scope,platform,host:a!.host,owner:"a",ownerOf:ownerOfV1,holds:a!.holds,install:a!.install,read:a!.read,lookMaximumMilliseconds:150,say:key=>said.push(key)});
 expect(result.outcome).toBe("unreachable");expect(Date.now()-started).toBeLessThan(2500);expect(said).toContain("look-bound");
 expect(a!.counters.reads).toBe(1);expect(a!.counters.inferences).toBe(6);
});

test("a machine that is not Ready and has no records of its own installs the fleet generation: no read, no inference, progress told",async()=>{
 const{fleet,machines:[a,b]}=world(20,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 await run(fleet,a!);
 const said:string[]=[];
 const result=await run(fleet,b!,{say:key=>said.push(key)});
 expect(result.outcome).toBe("installed");expect(b!.counters).toEqual({observations:0,inferences:0,takes:0,reads:0});expect(said).toContain("fetching");
});

test("keys, heads and documents name no machine: two roots, two homes, two host names, different file stamps give the same bytes",async()=>{
 const records=makeRecords(),one=makeMachine({name:"one",count:9,records,stamp:111}),two=makeMachine({name:"two",count:9,records,stamp:987654321});
 expect(one.root).not.toBe(two.root);expect(one.home).not.toBe(two.home);
 const sealedOf=new Map<string,Awaited<ReturnType<typeof one.read>>>();for(const machine of[one,two])sealedOf.set(machine.name,await machine.read(undefined));
 const built0=(machine:MachineV1)=>sealedOf.get(machine.name)!;
 const built=await Promise.all([one,two].map(async machine=>{const sealed=built0(machine);return buildSelectedGenerationV1({scope,platform,host:machine.host,sealed,ownerOf:ownerOfV1,now:1});}));
 expect(JSON.stringify(built[0]!.head)).toBe(JSON.stringify(built[1]!.head));
 for(const index of built[0]!.documents.keys())expect(Buffer.from(built[0]!.documents[index]!.bytes).equals(Buffer.from(built[1]!.documents[index]!.bytes))).toBe(true);
 const under=<T>(home:string,work:()=>T):T=>{const before={HOME:process.env.HOME,USERPROFILE:process.env.USERPROFILE};process.env.HOME=home;process.env.USERPROFILE=home;try{return work();}finally{for(const[name,value]of Object.entries(before))if(value===undefined)delete process.env[name];else process.env[name]=value;}};
 expect(under(one.home,()=>selectedHeadKeyV1({scope,platform}))).toBe(under(two.home,()=>selectedHeadKeyV1({scope,platform})));
 const homeBuilt=await Promise.all([one,two].map(async machine=>under(machine.home,()=>buildSelectedGenerationV1({scope,platform,host:machine.host,sealed:{...built0(machine)},ownerOf:ownerOfV1,now:1}))));
 expect(JSON.stringify(homeBuilt[0]!.head)).toBe(JSON.stringify(homeBuilt[1]!.head));
 const key=selectedHeadKeyV1({scope,platform}),haystack=[key,JSON.stringify(built[0]!.head),...built[0]!.documents.map(document=>Buffer.from(document.bytes).toString("utf8"))].join("\n");
 for(const name of[one.root,two.root,one.home,two.home,hostname(),process.env.HOME??"\u0000"])expect(haystack.includes(name)).toBe(false);
 const fleet=makeFleet(),first=await selectedGenerationV1({client:fleet.client("one"),scope,platform,host:one.host,owner:"one",ownerOf:ownerOfV1,holds:one.holds,install:one.install,read:one.read});
 expect(first.published).toBe(true);
 const keys=[...fleet.results.keys()];expect(keys).toEqual([key]);
 for(const name of[one.root,one.home,hostname()])expect(keys.join("").includes(name)).toBe(false);
});

test("the key changes with the scope, the revision and the platform, and with nothing else",()=>{
 const base=selectedHeadKeyV1({scope,platform});
 expect(selectedHeadKeyV1({scope:sha("other"),platform})).not.toBe(base);
 expect(selectedHeadKeyV1({scope,platform:"linux-x64"})).not.toBe(base);
 expect(selectedHeadKeyV1({scope,platform,revision:policy.revision+1})).not.toBe(base);
 expect(selectedHeadKeyV1({scope,platform,revision:policy.revision})).toBe(base);
});

test("a corrupt blob, a foreign head, a head of another revision and a damaged owner digest are a miss, never an installed generation",async()=>{
 const{fleet,machines:[a,b,c,d]}=world(6,["a","b","c","d"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1,MachineV1,MachineV1]};
 await run(fleet,a!);
 fleet.corruptBlobs=true;
 expect(await lookSelectedGenerationV1({client:fleet.client("b"),scope,platform,host:b!.host})).toBe("fault");
 const corrupt=await run(fleet,b!,{claimTtlMilliseconds:200});
 expect(corrupt.outcome).toBe("unreachable");expect(b!.installed).toBe(false);expect(b!.counters.reads).toBe(1);
 fleet.corruptBlobs=false;
 expect(await lookSelectedGenerationV1({client:fleet.client("c"),scope:sha("another"),platform,host:c!.host})).toBe("miss");
 const key=selectedHeadKeyV1({scope,platform}),held=fleet.results.get(key)!;
 fleet.results.set(selectedHeadKeyV1({scope:sha("foreign"),platform}),held);
 expect(await lookSelectedGenerationV1({client:fleet.client("c"),scope:sha("foreign"),platform,host:c!.host})).toBe("miss");
 fleet.results.set(key,Buffer.from("not a protobuf"));
 expect(await lookSelectedGenerationV1({client:fleet.client("d"),scope,platform,host:d!.host})).toBe("miss");
 fleet.results.set(key,held);
 const fetched=await lookSelectedGenerationV1({client:fleet.client("d"),scope,platform,host:d!.host});
 expect(typeof fetched).toBe("object");
});

test("a valid blob served under the name of another is refused, so a swapped graph is never installed",async()=>{
 const{fleet,machines:[a,b,c]}=world(6,["a","b","c"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1,MachineV1]};
 await run(fleet,a!);changed(b!,[1]);
 const{packGraphDocumentV1}=await import("../../../../🪪️graph-state/🛰️exchange/🟦️.ts");
 const other=packGraphDocumentV1({name:"graph",bytes:Buffer.from(JSON.stringify({owners:["somewhere else"]}))})[0]!,graph=packGraphDocumentV1({name:"graph",bytes:Buffer.from(a!.graph.replaceAll(a!.root,"⟦semio-root⟧"))})[0]!;
 expect(fleet.blobs.has(sha(graph.bytes))).toBe(true);
 fleet.blobs.set(sha(graph.bytes),other.bytes);
 expect(await lookSelectedGenerationV1({client:fleet.client("c"),scope,platform,host:c!.host})).toBe("fault");
 const result=await run(fleet,b!,{claimTtlMilliseconds:200});
 expect(result.outcome).toBe("unreachable");expect(b!.installed).toBe(false);
});

test("a head whose owner digests do not equal its rows is refused",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:5,records}),fleet=makeFleet(),sealed=await a.read(undefined),built=buildSelectedGenerationV1({scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,now:1});
 const lying={...built.head,owners:{...built.head.owners,o2:sha("lie")}};
 const{encodeProtobufV1,REAPI_V1}=await import("../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
 const{packGraphDocumentV1}=await import("../../../../🪪️graph-state/🛰️exchange/🟦️.ts");
 for(const blob of built.documents.flatMap(document=>packGraphDocumentV1(document)))fleet.blobs.set(sha(blob.bytes),blob.bytes);
 fleet.results.set(selectedHeadKeyV1({scope,platform}),encodeProtobufV1(REAPI_V1.actionResult,{output_files:lying.documents.map(row=>({path:row.name,digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(lying))}));
 expect(await lookSelectedGenerationV1({client:fleet.client("x"),scope,platform,host:a.host})).toBe("fault");
});

test("the head and the owner digests satisfy the schema (Ajv) and equal an independent recomputation",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:7,records,stamp:5}),sealed=await a.read(undefined);
 const built=buildSelectedGenerationV1({scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,now:42}),ajv=new Ajv({strict:false}),validate=ajv.compile({...schema.definitions.Head,definitions:schema.definitions} as never);
 expect(validate(built.head)).toBe(true);
 expect(validate({...built.head,revision:0})).toBe(false);
 const independent=new Map<string,string[]>();
 for(const row of sealed.rows){
  const relative=String(row.path).slice(a.root.length+1),owner=ownerOfV1(relative)!,fact=row.fact as{sha256:string;size:number};
  const line=JSON.stringify(["file","⟦semio-root⟧/"+relative,{...fact,mtimeMs:0,ctimeMs:0,ino:0}]);
  (independent.get(owner)??independent.set(owner,[]).get(owner)!).push(line);
 }
 for(const[owner,lines]of independent)expect(built.head.owners[owner]).toBe(createHash("sha256").update(lines.sort().join("\n")).digest("hex"));
 expect(Object.keys(built.head.owners).sort()).toEqual([...independent.keys()].sort());
});

test("owners that consumed the same rows store them once in the admission document and the reader gets every owner's rows back",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:30,records}),sealed=await a.read(undefined),shared=sealed.rows[0]!;
 const crowded={...sealed,admission:sealed.admission.map(entry=>({...entry,rows:[...entry.rows,shared,...sealed.rows.slice(0,10)]}))};
 const built=buildSelectedGenerationV1({scope,platform,host:a.host,sealed:crowded,ownerOf:ownerOfV1,now:1}),document=JSON.parse(Buffer.from(built.documents.find(row=>row.name==="admission")!.bytes).toString("utf8").replaceAll("⟦semio-root⟧","R")) as {version:number;rows:unknown[];entries:{rows:number[]}[]};
 expect(document.version).toBe(2);expect(document.entries.length).toBe(30);expect(document.rows.length).toBeLessThanOrEqual(sealed.rows.length+30);
 expect(document.entries.every(entry=>entry.rows.every(position=>Number.isInteger(position)&&position<document.rows.length))).toBe(true);
 const fleet=makeFleet(),stored=await publishSelectedGenerationV1({client:fleet.client("a"),scope,platform,host:a.host,sealed:crowded,ownerOf:ownerOfV1});
 expect(stored.stored).toBe(true);
 const fetched=await lookSelectedGenerationV1({client:fleet.client("b"),scope,platform,host:a.host});
 expect(typeof fetched).toBe("object");const got=fetched as Exclude<typeof fetched,string>;
 expect(got.admission.length).toBe(30);for(let index=0;index<30;index++)expect(got.admission[index]!.rows.length).toBe(crowded.admission[index]!.rows.length);
 expect(got.admission[3]!.rows.map(row=>row.path)).toEqual(crowded.admission[3]!.rows.map(row=>row.path));
});

test("a held head that equals the new read is not written again, and the publisher does not throw without upload rights",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:4,records}),fleet=makeFleet(),sealed=await a.read(undefined);
 const first=await publishSelectedGenerationV1({client:fleet.client("a"),scope,platform,host:a.host,sealed,ownerOf:ownerOfV1});
 expect(first.stored).toBe(true);
 const again=await publishSelectedGenerationV1({client:fleet.client("a"),scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,held:first.head});
 expect(again.stored).toBe(false);expect(again.reason).toBe("current");
 const readonly=fleet.client("r"),denied=await publishSelectedGenerationV1({client:{...readonly,settings:{...readonly.settings,upload:false}} as typeof readonly,scope,platform,host:a.host,sealed,ownerOf:ownerOfV1});
 expect(denied).toEqual({stored:false,reason:"no-upload"});
});

test("a head written under the key of this revision by another revision is a miss",async()=>{
 const records=makeRecords(),a=makeMachine({name:"a",count:5,records}),fleet=makeFleet(),sealed=await a.read(undefined),built=buildSelectedGenerationV1({scope,platform,host:a.host,sealed,ownerOf:ownerOfV1,now:1});
 const{encodeProtobufV1,REAPI_V1}=await import("../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
 const{packGraphDocumentV1}=await import("../../../../🪪️graph-state/🛰️exchange/🟦️.ts");
 for(const blob of built.documents.flatMap(document=>packGraphDocumentV1(document)))fleet.blobs.set(sha(blob.bytes),blob.bytes);
 const future={...built.head,revision:policy.revision+1},put=(head:typeof built.head)=>fleet.results.set(selectedHeadKeyV1({scope,platform}),encodeProtobufV1(REAPI_V1.actionResult,{output_files:head.documents.map(row=>({path:row.name,digest:{hash:row.hash,size_bytes:row.size}})),stdout_raw:Buffer.from(JSON.stringify(head))}));
 put(future);
 expect(await lookSelectedGenerationV1({client:fleet.client("x"),scope,platform,host:a.host})).toBe("miss");
 put(built.head);
 expect(typeof await lookSelectedGenerationV1({client:fleet.client("x"),scope,platform,host:a.host})).toBe("object");
});

test("a machine that may not upload never claims the head: waiters would wait for a head that cannot come",async()=>{
 const{fleet,machines:[a]}=world(3,["a"]) as {fleet:FleetV1;machines:[MachineV1]},client=fleet.client("a"),readonly={...client,settings:{...client.settings,upload:false}} as typeof client;
 const result=await selectedGenerationV1({client:readonly,scope,platform,host:a!.host,owner:"a",ownerOf:ownerOfV1,holds:a!.holds,install:a!.install,read:a!.read});
 expect(result.outcome).toBe("unreachable");expect(fleet.claims.size).toBe(0);expect(a!.counters.reads).toBe(1);
});

test("a published head is declared to the cache server as an entry of the protected graph class",async()=>{
 const{fleet,machines:[a]}=world(6,["a"]) as {fleet:FleetV1;machines:[MachineV1]};
 await run(fleet,a!);
 const kept=retentionManifestV1(),client=fleet.client("a"),declared=JSON.parse(Buffer.from(kept.bytes).toString("utf8"));
 expect(await client.hasBlob(kept.hash)).toBe(true);
 expect(declared.retention).toMatchObject({version:2,kind:"graph",sources:[],externs:[]});
 expect(policy.retentionManifest).toBe("stage.json");
 expect(typeof await lookSelectedGenerationV1({client,scope,platform,host:a!.host})).toBe("object");
});

test("a head published by a machine that never held the claim ends the wait of a small machine at once, not after its wait slice",async()=>{
 const{fleet,machines:[a,b]}=world(6,["a","b"]) as {fleet:FleetV1;machines:[MachineV1,MachineV1]};
 const key=selectedHeadKeyV1({scope,platform}),beat=setInterval(()=>{fleet.claims.set(key,{owner:"slow-holder",until:Date.now()+400});},60);
 fleet.claims.set(key,{owner:"slow-holder",until:Date.now()+400});
 const said:string[]=[],started=Date.now();
 const waiting=run(fleet,b!,{waitSliceMilliseconds:5000,waitMaximumMilliseconds:20000,say:name=>said.push(name)});
 await Bun.sleep(150);
 const sealed=await a!.read(undefined),published=await publishSelectedGenerationV1({client:fleet.client("c"),scope,platform,host:a!.host,sealed:sealed!,ownerOf:ownerOfV1});
 expect(published.stored).toBe(true);
 const result=await waiting;clearInterval(beat);
 expect(said).toContain("waiting");expect(result.outcome).toBe("waited");expect(result.differing).toEqual([]);
 expect(Date.now()-started,"the wait slice is 5 s, the head arrived after 150 ms").toBeLessThan(1500);
 expect(b!.counters.reads).toBe(0);
});
