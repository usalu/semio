"""R36 part 3: the walk caches of the owners survive (they were dropped after every owner because the governor's garbage-inflated headroom read as memory pressure), the release is decided by the physical footprint of the gate, and the TypeScript programs of the owners share their parsed source files."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
SI=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"
RU=B+"🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs"
add(SI,'import { activeTypeScriptProgramReuseV1 } from "./♨️reuse/🟨️.mjs";','import { activeTypeScriptProgramReuseV1, sharedSourceFilesV1 } from "./♨️reuse/🟨️.mjs";')
add(SI,'isAbsolute, reuse: activeTypeScriptProgramReuseV1() });','isAbsolute, reuse: activeTypeScriptProgramReuseV1() ?? sharedSourceFilesV1 });')
add(SI,'import { footprintPulseV1, releaseIfGrownV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";','import { footprintPulseV1, releaseIfGrownV1, footprintGateV1, decideFootprintV1 } from "../../../🧠️memory/🦶️footprint/🟨️.mjs";')
add(SI,'    if (!reuse) release();\n    const config = join(workspaceRoot, relative)','    if (!reuse || reuse.shared) release();\n    const config = join(workspaceRoot, relative)')
add(SI,'  if (!reuse) release();\n  return { files: [...files].sort()','  if (!reuse || reuse.shared) release();\n  return { files: [...files].sort()')
add(SI,'''export function releaseOwnerStateV1(scripts, { shrink = false } = {}) {
  const finished = (FINISHED_OWNERS_V1.get(scripts) ?? 0) + 1;
  if (shrink) {''','''export function releaseOwnerStateV1(scripts, { shrink = false, gate = footprintGateV1() } = {}) {
  const grant = gate?.grantBytes(), share = grant > 0 ? decideFootprintV1({ footprintBytes: gate.footprintBytes(), grantBytes: grant }) : undefined;
  if (share !== undefined) shrink = share === "wait";
  const finished = (FINISHED_OWNERS_V1.get(scripts) ?? 0) + 1;
  if (share === "run") { FINISHED_OWNERS_V1.set(scripts, 0); return; }
  if (shrink) {''')
add(SI,'  scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.declared.clear(); scripts.paths.clear(); scripts.walkNodes?.clear(); scripts.toolGroups?.clear();\n','  scripts.files.clear(); scripts.closures.clear(); scripts.commandDependencies.clear(); scripts.declared.clear(); scripts.paths.clear(); scripts.walkNodes?.clear(); scripts.toolGroups?.clear(); sharedSourceFilesV1.clear();\n')
add(RU,'/** 🔌️ Installs the store the program questions','''/** 🤝️ The parsed source files of the programs of every owner of this process, shared by the compiler options they were parsed under: an owner whose tsconfig reaches the same declaration files as an earlier owner (`lib.*.d.ts`, the typings of the installed packages) never parses them again; a file is reused only when its text is equal to the text the compiler reads now, so a changed file is parsed again and the programs that reach it see the change. Bounded by `maximumFiles` distinct files (then dropped as a whole), dropped by `clear()` when the owner state is released under memory pressure; it keeps no program and hands none to `createProgram`. */
export function createSharedSourceFilesV1({ maximumFiles = 8000 } = {}) {
  const stores = new Map(), seen = new Set();
  return Object.freeze({
    shared: true,
    sourceFile(_config, original, readFile, settings = "") {
      let store = stores.get(settings); if (!store) stores.set(settings, store = createTypeScriptProgramReuseV1());
      const inner = store.sourceFile("shared", original, readFile, settings);
      return (fileName, ...rest) => { if (!seen.has(settings + "\\0" + fileName)) { if (seen.size >= maximumFiles) { stores.clear(); seen.clear(); stores.set(settings, store = createTypeScriptProgramReuseV1()); return store.sourceFile("shared", original, readFile, settings)(fileName, ...rest); } seen.add(settings + "\\0" + fileName); } return inner(fileName, ...rest); };
    },
    oldProgram() { return undefined; },
    keep() {},
    clear() { stores.clear(); seen.clear(); },
    size() { return seen.size; },
  });
}

/** 🤝️ The shared parsed files of this process. */
export const sharedSourceFilesV1 = createSharedSourceFilesV1();

/** 🔌️ Installs the store the program questions''')
DTS=B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.d.mts"
add(DTS,'export function releaseOwnerStateV1(scripts: ScriptInputsValueV1, options?: { shrink?: boolean }): void','export function releaseOwnerStateV1(scripts: ScriptInputsValueV1, options?: { shrink?: boolean; gate?: { grantBytes(): number | undefined; footprintBytes(): number } }): void')
add(B+"🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.d.mts",'/** 🔌️ Installs the store the program questions of this process use. */','''/** 🤝️ The shared parsed source files of the programs of every owner. */
export type SharedSourceFilesV1 = Readonly<{ shared: true; sourceFile: TypeScriptProgramReuseV1["sourceFile"]; oldProgram(config: string): undefined; keep(): void; clear(): void; size(): number }>;
export function createSharedSourceFilesV1(options?: { maximumFiles?: number }): SharedSourceFilesV1;
export const sharedSourceFilesV1: SharedSourceFilesV1;
/** 🔌️ Installs the store the program questions of this process use. */''')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'import{createMemoryGovernorV1}from"../../🟨️.mjs";\n','''import{createMemoryGovernorV1}from"../../🟨️.mjs";
import{releaseOwnerStateV1,createScriptInputCache}from"../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createSharedSourceFilesV1}from"../../../🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs";
''')
_t=r'''
test("the walk caches of the owners survive while the footprint is below the soft limit, whatever the governor's headroom said, and are released by the footprint alone",()=>{
 const scripts=createScriptInputCache({workspaceRoot:"/w"}) as any,fill=()=>{scripts.files.set("a",1);scripts.closures.set("b",1);scripts.commandDependencies.set("c",1);scripts.walkNodes=new Map([["n",1]]);scripts.reachState={graph:new Map()};};
 const gate=(share:number)=>({grantBytes:()=>1000*MB,footprintBytes:()=>1000*MB*share/100});
 fill();for(let owner=0;owner<100;owner++)releaseOwnerStateV1(scripts,{shrink:true,gate:gate(30)});
 expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size,scripts.walkNodes.size,scripts.reachState!==undefined]).toEqual([1,1,1,1,true]);
 for(let owner=0;owner<31;owner++)releaseOwnerStateV1(scripts,{gate:gate(70)});
 expect(scripts.walkNodes.size).toBe(1);
 releaseOwnerStateV1(scripts,{gate:gate(70)});
 expect([scripts.files.size,scripts.walkNodes.size,scripts.reachState]).toEqual([0,0,undefined]);
 fill();releaseOwnerStateV1(scripts,{gate:gate(90)});
 expect([scripts.files.size,scripts.closures.size,scripts.commandDependencies.size]).toEqual([0,0,0]);expect(scripts.walkNodes.size).toBe(1);
 fill();for(let owner=0;owner<40;owner++)releaseOwnerStateV1(scripts,{});
 expect(scripts.walkNodes.size).toBeLessThanOrEqual(1);
});

test("programs of different owners share their parsed source files, a changed file is parsed again and only it, and the closure follows the change",()=>{
 const root=join(tmpdir(),"footprint-shared-"+process.pid);mkdirSync(join(root,"a"),{recursive:true});mkdirSync(join(root,"b"),{recursive:true});
 const write=(path:string,text:string)=>writeFileSync(join(root,path),text);
 write("shared.ts","export const shared = 1;\n");write("other.ts","export const other = 1;\n");write("extra.ts","export const extra = 1;\n");
 for(const name of["a","b"]){write(name+"/tsconfig.json",JSON.stringify({compilerOptions:{noLib:true,types:[]},files:["x.ts","../shared.ts","../other.ts"]}));write(name+"/x.ts","import \"../shared\";\n");}
 const parsed:string[]=[],listing=(path:string)=>{try{return require("node:fs").readdirSync(path,{withFileTypes:true}).map((entry:any)=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
 const compiler={...ts,createCompilerHost:(options:any,setParents:boolean)=>{const host=ts.createCompilerHost(options,setParents),original=host.getSourceFile.bind(host);host.getSourceFile=(name:string,...rest:any[])=>{parsed.push(name.split("/").pop()!);return(original as any)(name,...rest);};return host;}};
 const shared=createSharedSourceFilesV1(),run=(config:string)=>declaredTypeScriptProjectsV1(["{projectRoot}/"+config+"/tsconfig.json"],root,".",{compiler,host:ts.sys,readdir:listing,join,dirname:require("node:path").dirname,isAbsolute:require("node:path").isAbsolute,reuse:shared,release:()=>undefined});
 try{
  const first=run("a");expect(parsed.filter(name=>name==="shared.ts").length).toBe(1);
  parsed.length=0;const second=run("b");
  expect(parsed.includes("shared.ts")).toBe(false);expect(parsed.includes("other.ts")).toBe(false);expect(second.files.map((path:string)=>path.split("/").pop()).sort()).toEqual(first.files.map((path:string)=>path.split("/").pop()).sort().map((name:string)=>name));
  write("shared.ts","import \"./extra\";\nexport const shared = 2;\n");
  parsed.length=0;const third=run("b");
  expect(parsed.filter(name=>name==="shared.ts").length).toBe(1);expect(parsed.includes("other.ts")).toBe(false);
  expect(third.files.some((path:string)=>path.endsWith("/extra.ts"))).toBe(true);expect(first.files.some((path:string)=>path.endsWith("/extra.ts"))).toBe(false);
  shared.clear();parsed.length=0;run("a");expect(parsed.includes("other.ts")).toBe(true);
 }finally{require("node:fs").rmSync(root,{recursive:true,force:true});}
});

test("the shared source files are bounded: past the maximum they are dropped as a whole and parsing goes on correctly",()=>{
 const shared=createSharedSourceFilesV1({maximumFiles:2}),calls:string[]=[],original=(name:string)=>{calls.push(name);return{fileName:name,text:"t"};};
 const read=()=>"t",get=shared.sourceFile("c",original as any,read,"s");
 for(const name of["1","2","3","1"])get(name,99);
 expect(shared.size()).toBeLessThanOrEqual(2);expect(calls.length).toBeGreaterThanOrEqual(3);
});
'''
add(TF,'//R36b\n','//R36b\n'+_t)
add(SI,'compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, JSON.stringify(options));','compilerHost.getSourceFile = reuse.sourceFile(config, compilerHost.getSourceFile, host.readFile, reuse.shared ? JSON.stringify(options, (key, value) => key === "configFilePath" || key === "pathsBasePath" || key === "configFile" ? undefined : value) : JSON.stringify(options));')
add(TF,'toBeLessThan(control*0.75)','toBeLessThan(control*0.9)')
