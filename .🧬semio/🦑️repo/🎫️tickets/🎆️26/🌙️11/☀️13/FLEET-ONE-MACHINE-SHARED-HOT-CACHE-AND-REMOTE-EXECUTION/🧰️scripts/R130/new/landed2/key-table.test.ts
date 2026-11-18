import{expect,test}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createKeyTableV1}from"../../🔑️key-table/🟨️.mjs";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py");
const lcg=(seed:number)=>()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const text=(at:number)=>"k"+at+(at%7===0?"é𝄞":"")+"/".repeat(at%5)+"x".repeat(at%40);

test("the table answers what a Map and the Python dict answer over seeded operations, across several doublings",()=>{
 const random=lcg(11),ops:(["set",string,number]|["get",string])[]=[];
 for(let step=0;step<20000;step++){const key=text(Math.floor(random()*6000));ops.push(random()<0.45?["set",key,Math.floor(random()*1e6)]:["get",key]);}
 const table=createKeyTableV1(4),model=new Map<string,number>(),answers:(number|undefined)[]=[];
 for(const op of ops){if(op[0]==="set"){table.set(op[1],op[2]);model.set(op[1],op[2]);}else{const got=table.get(op[1]);expect(got).toBe(model.get(op[1]));answers.push(got);}}
 expect(table.size()).toBe(model.size);
 const directory=mkdtempSync(join(tmpdir(),"key-table-"));
 try{
  const path=join(directory,"ops.json");writeFileSync(path,JSON.stringify(ops));
  const done=python?Bun.spawnSync([python,"-I",oracle,path]):undefined;
  if(done){expect(done.exitCode).toBe(0);const independent=JSON.parse(done.stdout.toString());expect(independent.answers.map((value:number|null)=>value??undefined)).toEqual(answers);expect(independent.size).toBe(table.size());}
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("an entry costs a fixed number of bytes whatever the length of its text, and a number out of range is refused",()=>{
 const table=createKeyTableV1(1024);
 for(let at=0;at<20000;at++)table.set("p".repeat(at%300)+at,at);
 expect(table.size()).toBe(20000);
 expect(table.bytes()/table.size()).toBeLessThanOrEqual(84);
 expect(()=>table.set("a",-1)).toThrow();expect(()=>table.set("a",2**32)).toThrow();
 expect(table.get("never")).toBeUndefined();
 expect(table.intern("fresh",7)).toBe(7);expect(table.intern("fresh",9)).toBe(7);expect(table.get("fresh")).toBe(7);
 table.clear();expect(table.size()).toBe(0);expect(table.get("p"+0)).toBeUndefined();
});
