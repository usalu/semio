import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {resolveBuildSelectionV1} from "../../🟦️.ts";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"tx-")),file=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
mkdirSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),{recursive:true});
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const select=(...selected:string[])=>resolveBuildSelectionV1(root,["run","workspace:test","--",...selected]).args;
const parses=(body:()=>void):number=>{const original=JSON.parse;let count=0;JSON.parse=((text:string,reviver?:(key:string,value:unknown)=>unknown)=>{if(typeof text==="string"&&text.includes("testPhases"))count++;return original(text,reviver);}) as typeof JSON.parse;try{body();}finally{JSON.parse=original;}return count;};

test("the test phase lists of the taxonomy are parsed once per file identity and a changed file is read anew",()=>{
 writeFileSync(file,JSON.stringify({testPhases:["alpha","beta"],testLevellessPhases:["beta"],unrelated:Array.from({length:50},(_,index)=>index)}));
 let results:string[][]=[];
 expect(parses(()=>{results=[select("alpha","x"),select("beta","x"),select("gamma"),select("alpha","y")];})).toBe(1);
 expect(results).toEqual([["run","@semio-tech/repo-test-domain:test-alpha","--","fundamental","x"],["run","@semio-tech/repo-test-domain:test-beta","--","x"],["run","workspace:test","--","gamma"],["run","@semio-tech/repo-test-domain:test-alpha","--","fundamental","y"]]);
 writeFileSync(file,JSON.stringify({testPhases:["alpha","gamma"],testLevellessPhases:[],unrelated:[]}));
 const later=new Date(Date.now()+5000);utimesSync(file,later,later);
 expect(parses(()=>{results=[select("gamma"),select("beta","x")];})).toBe(1);
 expect(results).toEqual([["run","@semio-tech/repo-test-domain:test-gamma","--","fundamental"],["run","workspace:test","--","beta","x"]]);
});

test("the selections the taxonomy does not decide never read it",()=>{
 rmSync(file);
 expect(parses(()=>{expect(resolveBuildSelectionV1(root,["run","a:b"]).args).toEqual(["run","a:b"]);expect(select()).toEqual(["run","workspace:test","--","fundamental"]);expect(select("repo-client","z")).toEqual(["run","@semio-tech/repo-client:test","--","fundamental","z"]);})).toBe(0);
});
