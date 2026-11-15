import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{wantedPolicyV1}from"../../🟦️.ts";
import{classifyOwnersV1,createOwnerCostRecorderV1,describeHeavyV1,emptyOwnerCostTableV1,estimateOwnerCostV1,mergeOwnerCostTablesV1,orderByCostV1,ownerCostPathV1,ownerCostPolicyV1,ownerCostTextV1,ownerGrantV1,publishOwnerCostsV1,readOwnerCostFileV1,readOwnerCostsV1,recordOwnerCostFileV1,recordOwnerCostV1,waveEvidenceV1,writeOwnerCostFileV1,type OwnerCostInputV1}from"../🟦️.ts";

/** 💰️ Contract: projects are weighed by memory, one owner at a time. The recorded cost of an owner is its own, the heavy ones are found by the grant of the machine, the order of a serving machine follows the cost, and the table survives files, merges and the fleet. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`owner-cost-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Table",definitions:schema.definitions});
const sample=(name:string,megabytes:number,atMs:number,stopped=false,kind="project"):OwnerCostInputV1=>({name,kind,megabytes,seconds:12,rows:34,stopped,atMs});
const BASE=wantedPolicyV1,SIXTEEN=ownerGrantV1(8192,BASE);

test("the grant of a 16 GB machine is the admitted half less the base, divided by the footprint factor and the margin; a serving machine has none",()=>{
 expect(SIXTEEN).toEqual({footprintMegabytes:4392,managedMegabytes:Math.floor(4392/2/1.25)});expect(SIXTEEN.managedMegabytes).toBe(1756);
 expect(ownerGrantV1(9400,BASE).managedMegabytes).toBe(Number.POSITIVE_INFINITY);
 expect(ownerGrantV1(16384,BASE).footprintMegabytes).toBe(Number.POSITIVE_INFINITY);
 expect(ownerGrantV1(1000,BASE)).toEqual({footprintMegabytes:0,managedMegabytes:0});
});

test("a recorded owner is estimated by its own newest samples, never by its kind; seeds, kinds and the unknown follow in that order",()=>{
 let table=emptyOwnerCostTableV1();
 for(const [at,megabytes] of [100,200,9000,300,250,260,270,280].entries())table=recordOwnerCostV1(table,sample("@semio-tech/a",megabytes,at+1));
 table=recordOwnerCostV1(table,sample("@semio-tech/light",90,50));
 expect(estimateOwnerCostV1(table,"@semio-tech/a")).toEqual({megabytes:300,source:"recorded"});
 expect(table.owners["@semio-tech/a"]!.samples).toHaveLength(ownerCostPolicyV1.samples);
 expect(estimateOwnerCostV1(table,"@semio-tech/light")).toEqual({megabytes:90,source:"recorded"});
 expect(estimateOwnerCostV1(table,"@semio-tech/s-dev-composition","project")).toEqual({megabytes:13976,source:"seed"});
 expect(estimateOwnerCostV1(table,"workspace","root:language:cargo")).toEqual({megabytes:3100,source:"seed"});
 expect(estimateOwnerCostV1(table,"another-root","root:language:cargo")).toEqual({megabytes:3100,source:"kind"});
 expect(estimateOwnerCostV1(table,"@semio-tech/never")).toEqual({megabytes:ownerCostPolicyV1.unknownMegabytes,source:"unknown"});
 expect(estimateOwnerCostV1(recordOwnerCostV1(table,sample("@semio-tech/s-dev-composition",900,99)),"@semio-tech/s-dev-composition")).toEqual({megabytes:900,source:"recorded"});
});

test("on a 16 GB machine the heavy owners are never local and the light ones are: a mix of the measured owners",()=>{
 const table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("@semio-tech/os-kernel",1500,5));
 const names=["@semio-tech/s-dev-composition","@semio-tech/framework-rs","@semio-tech/repo-lib","workspace","@semio-tech/os-kernel","@semio-tech/value-rs","@semio-tech/never"];
 const kinds:Record<string,string>={workspace:"root:language:cargo","@semio-tech/value-rs":"language:cargo"};
 const classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(table,name,kinds[name]),SIXTEEN);
 expect(classes.heavy.map(row=>row.name)).toEqual(["@semio-tech/s-dev-composition","@semio-tech/repo-lib","@semio-tech/framework-rs","workspace"]);
 expect(classes.light).toEqual(["@semio-tech/os-kernel","@semio-tech/value-rs","@semio-tech/never"]);
 expect(classes.heavy[0]).toEqual({name:"@semio-tech/s-dev-composition",megabytes:13976,source:"seed"});
});

test("the same owners on a 32 GB machine are all light: the grant is the only difference",()=>{
 const classes=classifyOwnersV1(["@semio-tech/s-dev-composition","workspace"],name=>estimateOwnerCostV1(undefined,name),ownerGrantV1(16384,BASE));
 expect(classes.heavy).toEqual([]);
});

test("light owners fill the grant by count (the wave rule) while the heavy ones stay out of it, whatever number of light ones there are",()=>{
 const light=Array.from({length:300},(_,at)=>`@semio-tech/l${at}`),names=[...light,"workspace"];
 const classes=classifyOwnersV1(names,name=>estimateOwnerCostV1(undefined,name,name==="workspace"?"root:language:cargo":"project"),SIXTEEN);
 expect(classes.heavy.map(row=>row.name)).toEqual(["workspace"]);expect(classes.light).toHaveLength(300);
});

test("the serving order is the heavy ones of the requesters first, then by cost, heaviest first; equal cost in the order given; every name once",()=>{
 let table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("b",500,1));
 table=recordOwnerCostV1(table,sample("a",500,1));table=recordOwnerCostV1(table,sample("c",4000,1));table=recordOwnerCostV1(table,sample("d",10,1));
 const estimate=(name:string)=>estimateOwnerCostV1(table,name);
 expect(orderByCostV1(["d","b","a","c","c","e"],estimate)).toEqual(["c","e","b","a","d"]);
 expect(orderByCostV1(["d","b","a","c"],estimate,new Set(["d"]))).toEqual(["d","c","b","a"]);
 expect(orderByCostV1(["x","y"],estimate)).toEqual(["x","y"]);
});

test("a wave proves the cost of its only project at the peak, nothing of several and nothing below the floor",()=>{
 expect(waveEvidenceV1({peakMegabytes:18814,managedMegabytes:4838,peakProjects:["@semio-tech/s-dev-composition"]})).toEqual({name:"@semio-tech/s-dev-composition",megabytes:13976});
 expect(waveEvidenceV1({peakMegabytes:8365,managedMegabytes:4332,peakProjects:["workspace","a","b"]})).toBeUndefined();
 expect(waveEvidenceV1({peakMegabytes:4300,managedMegabytes:4200,peakProjects:["a"]})).toBeUndefined();
 expect(waveEvidenceV1({peakMegabytes:4300,managedMegabytes:4200,peakProjects:[]})).toBeUndefined();
});

test("the table is a file that validates against its schema with Ajv, survives damage and foreign content as empty, and merges equal samples once",()=>{
 const directory=temp("file"),path=ownerCostPathV1(directory);
 expect(path).toBe(join(directory,".cache/f/graph-costs/owner-costs.json"));
 expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 const first=recordOwnerCostFileV1(path,sample("a",700,10)),second=recordOwnerCostFileV1(path,sample("a",9000,20,true));
 expect(validate(second)).toBe(true);expect(validate(JSON.parse(readFileSync(path,"utf8")))).toBe(true);
 expect(readOwnerCostFileV1(path).owners.a!.samples.map(row=>row.megabytes)).toEqual([700,9000]);expect(first.owners.a!.samples).toHaveLength(1);
 writeFileSync(path,"{damaged");expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 writeFileSync(path,JSON.stringify({version:2,owners:{}}));expect(readOwnerCostFileV1(path)).toEqual(emptyOwnerCostTableV1());
 const merged=mergeOwnerCostTablesV1([second,second,undefined,recordOwnerCostV1(emptyOwnerCostTableV1(),sample("a",800,15))]);
 expect(merged.owners.a!.samples.map(row=>row.megabytes)).toEqual([700,800,9000]);
 expect(validate(JSON.parse(JSON.stringify({version:1,owners:{a:{kind:"p",rows:0,seconds:0,samples:[]}}})))).toBe(false);
 writeOwnerCostFileV1(join(directory,"blocked","x","table.json"),second);
});

test("the heavy owners are worded in English and German with their physical cost, the heaviest first, and the rest as a count",()=>{
 const heavy=Array.from({length:8},(_,at)=>({name:`p${at}`,megabytes:1000*(8-at),source:"recorded" as const}));
 expect(describeHeavyV1(heavy.slice(0,2),"en")).toBe("p0 (about 16000 MB), p1 (about 14000 MB)");
 expect(describeHeavyV1(heavy.slice(0,1),"de")).toBe("p0 (etwa 16000 MB)");
 expect(describeHeavyV1(heavy,"en").endsWith("and 2 more")).toBe(true);expect(describeHeavyV1(heavy,"de").endsWith("und 2 weitere")).toBe(true);
 for(const key of Object.keys(schema.definitions.Texts.const.en) as (keyof typeof schema.definitions.Texts.const.en)[])expect(ownerCostTextV1(key,"de").length).toBeGreaterThan(3);
 expect(Object.keys(schema.definitions.Texts.const.de)).toEqual(Object.keys(schema.definitions.Texts.const.en));
});

test("the fleet carries the table of the serving machine: published, read, damaged or absent",async()=>{
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 try{
  const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"},root=temp("client"),client=graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});
  expect(await readOwnerCostsV1({client})).toBeUndefined();
  const table=recordOwnerCostV1(emptyOwnerCostTableV1(),sample("@semio-tech/s-dev-composition",13976,5));
  expect(await publishOwnerCostsV1({client,server:"g1-server",table})).toBe(true);
  expect(await readOwnerCostsV1({client})).toEqual(table);
 }finally{await server.stop(true);}
},60000);

test("the recorder keeps only notable owners, writes merged and rarely, and flushes at once when an owner was stopped",()=>{
 const directory=temp("recorder"),path=ownerCostPathV1(directory),MB=1048576;let time=1_000_000;
 const recorder=createOwnerCostRecorderV1({path,now:()=>time,rowsOf:key=>key==="heavy"?18748:0});
 recorder.record({key:"tiny",kind:"project",takenBytes:5*MB,seconds:1,stopped:false});
 recorder.record({key:"big",kind:"project",takenBytes:900*MB,seconds:30,stopped:false});
 expect(Object.keys(recorder.table().owners)).toEqual(["big"]);expect(readOwnerCostFileV1(path).owners.big?.samples[0]?.megabytes).toBe(900);
 time+=100;recorder.record({key:"big",kind:"project",takenBytes:100*MB,seconds:3,stopped:false});
 expect(readOwnerCostFileV1(path).owners.big?.samples).toHaveLength(1);
 time+=ownerCostPolicyV1.flushMilliseconds;recorder.record({key:"big",kind:"project",takenBytes:110*MB,seconds:3,stopped:false});
 expect(readOwnerCostFileV1(path).owners.big?.samples.map(sample=>sample.megabytes)).toEqual([900,100,110]);
 recordOwnerCostFileV1(path,sample("other-writer",777,5));
 recorder.record({key:"heavy",kind:"root:language:cargo",takenBytes:4000*MB,seconds:60,stopped:true});
 const file=readOwnerCostFileV1(path);expect(file.owners.heavy).toMatchObject({rows:18748,samples:[{megabytes:4000,stopped:true}]});expect(file.owners["other-writer"]?.samples[0]?.megabytes).toBe(777);
 expect(validate(file)).toBe(true);
});
