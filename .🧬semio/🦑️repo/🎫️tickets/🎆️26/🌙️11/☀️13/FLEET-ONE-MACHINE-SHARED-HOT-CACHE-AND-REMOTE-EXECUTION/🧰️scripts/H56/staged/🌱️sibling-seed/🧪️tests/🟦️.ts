/** 🌱️ Contract: a read without a usable memo of its own is seeded with the owner facts of the newest admission memos of other selections of the same format (newest wins a shared owner, never its own scope, never a foreign format, never an oversized memo, never a forgotten row), equal to an independent CPython model over the memo files.
 * @see ../🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {packAdmissionMemoV1,writeAdmissionMemoV1} from "../../🧠️admission-memo/🟦️.ts";
import {seedFromSiblingMemosV1} from "../🟦️.ts";

const format="f".repeat(64),other="e".repeat(64),sid=(letter:string)=>letter.repeat(64);
const base=mkdtempSync(join(tmpdir(),"sibling-seed-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const row=(path:string)=>({kind:"file",path,fact:{version:1}});
const memo=async(directory:string,scope:string,memoFormat:string,owners:string[],at:number)=>{
 const rows=owners.map(name=>row("/w/"+name+"/project.json"));
 const value=packAdmissionMemoV1({format:memoFormat,scope,admission:{owners:owners,files:rows.map(each=>each.path),targets:{},inputs:{},catalogProviders:[]},observations:rows,owners:owners.map((name,index)=>({suffix:name+"\0[[],[]]",facts:{from:scope[0],name},rows:[rows[index]!]}))});
 const file=join(directory,scope+".admission.json");
 expect(await writeAdmissionMemoV1(file,value,1<<24)).toBe(true);utimesSync(file,at/1000,at/1000);return file;
};

test("the seeded owners equal the newest-first union of the sibling memos of the same format (oracle: CPython over the files)",async()=>{
 const directory=join(base,"a");require("node:fs").mkdirSync(directory);
 await memo(directory,sid("1"),format,["a","b","c"],1_000_000);
 await memo(directory,sid("2"),format,["b","c","d"],2_000_000);
 await memo(directory,sid("3"),other,["x","y"],3_000_000);
 await memo(directory,sid("4"),format,["e"],4_000_000);
 await memo(directory,sid("9"),format,["own"],5_000_000);
 writeFileSync(join(directory,sid("5")+".admission.json"),"{not json");
 const seeded=new Map<string,string>(),known=new Set<string>(["a\0[[],[]]"]);
 const counts=await seedFromSiblingMemosV1({directory,scope:sid("9"),format,known,seed:(suffix,facts)=>{seeded.set(suffix.split("\0")[0]!,(facts as {from:string}).from);}});
 const model=spawnSync("python3",["-I","-c",`
import json,os,sys
d=json.load(sys.stdin);out={}
files=[]
for n in os.listdir(d["dir"]):
    if not n.endswith(".admission.json") or n.startswith(d["own"]): continue
    p=os.path.join(d["dir"],n)
    try: v=json.load(open(p))
    except Exception: continue
    if v.get("format")!=d["format"]: continue
    files.append((os.stat(p).st_mtime,n,v))
files.sort(key=lambda t:(-t[0],t[1]))
for _,n,v in files[:3]:
    for o in v["owners"]:
        s=o["suffix"].split("\\0")[0]
        if s in d["known"] or s in out: continue
        out[s]=n[0]
print(json.dumps(out,sort_keys=True))
`],{input:JSON.stringify({dir:directory,own:sid("9"),format,known:["a"]}),encoding:"utf8"});
 const expected=JSON.parse(model.stdout) as Record<string,string>;
 expect(Object.fromEntries([...seeded].sort(([a],[b])=>a<b?-1:1))).toEqual(expected);
 expect(expected).toEqual({e:"4",b:"2",c:"2",d:"2"});
 expect(counts).toEqual({memos:3,owners:4});
 expect(known.has("e\0[[],[]]")).toBe(true);
});

test("an oversized memo, a missing directory and an aborted read seed nothing and never throw except the abort",async()=>{
 const directory=join(base,"b");require("node:fs").mkdirSync(directory);
 await memo(directory,sid("1"),format,["a"],1_000_000);
 expect(await seedFromSiblingMemosV1({directory:join(base,"absent"),scope:sid("9"),format,known:new Set(),seed:()=>{throw Error("never");}})).toEqual({memos:0,owners:0});
 const controller=new AbortController();controller.abort();
 await expect(seedFromSiblingMemosV1({directory,scope:sid("9"),format,known:new Set(),seed:()=>undefined,signal:controller.signal})).rejects.toBeDefined();
});
