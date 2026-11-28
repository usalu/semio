/** 🧠️ Contract: the analysis of a source is a function of (parser identity, parser revision, relative path, content digest) alone, so after one edit in a tree of N analysed files exactly one file is analysed again, in any later process or selection, and the graph of the answers equals a cold run. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import * as observation from "../../🟨️.mjs";

const {captureObservedSourceInputsV1,observedSourceAnalysisV1,registerSourceAnalysisV1,releaseInferenceMemoryV1,useSourceAnalysisStoreV1}=observation;
const COUNT=24,PROVIDER="test:file-memo:imports",REVISION="3".repeat(64);
const body=(index:number,edit:number)=>`import {a${index}} from "./f${(index+1)%COUNT}.ts";\nexport const v${index}=${edit};\n`+"// padding\n".repeat(2000);

test("after one edit in a tree of analysed files exactly that file is analysed again and the graph equals a cold run and the CPython model",async()=>{
  const root=realpathSync(mkdtempSync(join(tmpdir(),"file-memo-"))),store=join(root,".store"),calls:string[]=[];
  try{
    mkdirSync(join(root,"src"));
    const names=Array.from({length:COUNT},(_,index)=>join(root,"src","f"+index+".ts"));
    names.forEach((name,index)=>writeFileSync(name,body(index,0)));
    const provider=registerSourceAnalysisV1({id:PROVIDER,revision:REVISION,analyze:(path:string,bytes:Buffer)=>{calls.push(path);return {from:path.slice(root.length),imports:[...bytes.toString("utf8").matchAll(/from "([^"]+)"/gu)].map(match=>match[1]),values:bytes.toString("utf8").match(/export const v\d+=(\d+)/u)?.[1]};}});
    const graph=async()=>{releaseInferenceMemoryV1();return (await captureObservedSourceInputsV1(()=>names.map(name=>observedSourceAnalysisV1(provider,name)),undefined,false)).value as unknown[];};
    const shas=()=>Object.fromEntries(names.map(name=>[name.slice(root.length+1),createHash("sha256").update(readFileSync(name)).digest("hex")]));
    useSourceAnalysisStoreV1({root,directory:store});
    const before=shas(),first=await graph();
    expect(calls.length).toBe(COUNT);
    calls.length=0;expect(await graph()).toEqual(first);expect(calls.length).toBe(0);
    writeFileSync(names[7],body(7,1));
    const after=shas(),warm=await graph(),reanalysed=[...calls].map(path=>path.slice(root.length+1)).sort();
    calls.length=0;useSourceAnalysisStoreV1(undefined);
    const cold=await graph();
    expect(calls.length).toBe(COUNT);expect(warm).toEqual(cold);expect(warm).not.toEqual(first);
    const input=join(root,"case.json");writeFileSync(input,JSON.stringify({provider:PROVIDER,revision:REVISION,before,after}));
    const model=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),input],{encoding:"utf8"});
    expect(model.status,model.stderr).toBe(0);
    expect(reanalysed).toEqual(JSON.parse(model.stdout));expect(reanalysed).toEqual(["src/f7.ts"]);
  }finally{useSourceAnalysisStoreV1(undefined);rmSync(root,{recursive:true,force:true});}
});
