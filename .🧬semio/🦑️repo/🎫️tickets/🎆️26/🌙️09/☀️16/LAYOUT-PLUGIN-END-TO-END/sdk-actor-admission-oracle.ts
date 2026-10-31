import {readFile} from "node:fs/promises";
import Ajv from "ajv";
import patch from "fast-json-patch";
const base=new URL("./",import.meta.url);
const fixture=JSON.parse(await readFile(new URL("sdk-actor-admission.json",base),"utf8"));
const schema=JSON.parse(await readFile(new URL("sdk-actor-admission.schema.json",base),"utf8"));
if(!new Ajv().compile(schema)(fixture))throw Error("invalid neutral actor admission fixture");
function model(actorBytes:number,baselineCapacity:number,frameCapacity:number){
 const quotedCapacity=baselineCapacity+actorBytes+frameCapacity;
 const denied={grant:quotedCapacity-1,required:quotedCapacity,tag:true,emission:true,receipt:0,output:false};
 try{patch.applyPatch(denied,[{op:"test",path:"/grant",value:quotedCapacity},{op:"replace",path:"/tag",value:false}],true,true);throw Error("underquote accepted");}catch(error){if(!(error instanceof Error)||error.name!=="TEST_OPERATION_FAILED")throw error;}
 const accepted={tag:true,emission:true,receipt:0,output:false};
 patch.applyPatch(accepted,[{op:"replace",path:"/tag",value:false},{op:"replace",path:"/output",value:true},{op:"replace",path:"/receipt",value:quotedCapacity}],true,true);
 return {quotedCapacity,actualCapacity:accepted.receipt,deniedUnchanged:denied.tag&&denied.emission&&denied.receipt===0&&!denied.output,sharedFrameQuoted:quotedCapacity-baselineCapacity-actorBytes===frameCapacity,oneItem:true};
}
const expected=fixture.cases.map((row:any)=>({name:row.name,...model(Buffer.byteLength(fixture.actor),row.baselineCapacity,row.frameCapacity)}));
let count=0;
if(process.argv[2]){
 const rows=(await readFile(process.argv[2],"utf8")).split("\n").filter(line=>line.startsWith("sdk-actor-admission=")).map(line=>JSON.parse(line.slice("sdk-actor-admission=".length)));
 const expectedKeys=new Set(["press:false:0","press:true:0","typing:false:0","typing:false:1","typing:true:0","typing:true:1"]);const seen=new Set<string>();for(const row of rows){const key=row.name+":"+row.cancelled+":"+row.index;if(seen.has(key)||!expectedKeys.has(key))throw Error("unknown or duplicate native actor case");seen.add(key);const reference=model(Buffer.byteLength(row.actor),row.baselineCapacity,row.frameCapacity);for(const [name,value]of Object.entries(reference))if(row[name]!==value)throw Error("native actor admission differs: "+name);if(row.frameCapacity<=0)throw Error("missing native shared frame");}
 if(seen.size!==expectedKeys.size||[...expectedKeys].some(key=>!seen.has(key)))throw Error("missing native actor admission records");count=rows.length;
}
console.log(JSON.stringify({referenceCases:expected,nativeCompared:!!process.argv[2],nativeRecords:count}));
