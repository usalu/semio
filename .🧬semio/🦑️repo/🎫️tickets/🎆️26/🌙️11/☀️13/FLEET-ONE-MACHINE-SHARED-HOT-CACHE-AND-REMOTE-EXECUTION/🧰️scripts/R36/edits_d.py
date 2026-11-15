"""R36d: the process that really reads the graph (the sealed implementation child of a graph-reading command) gets the collector cap and the allocator purge like every other graph child; without it the runtime sized its garbage by the 32 GB of the host and one synchronous stretch of the inference allocated 25 GB before its first full collection."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
X=B+"🏛️sealed/📥️execution/🟦️.ts"
add(X,'import{readPartitionPinV1}from"../🔀️partition/📌️retention/🟦️.ts";\n','''import{readPartitionPinV1}from"../🔀️partition/📌️retention/🟦️.ts";
import{repositoryCommandReadsGraphV1}from"../../⌨️command/🧭️selection/🟦️.ts";
import{graphChildEnvironmentV1}from"../../🧠️memory/🐏️child-environment/🟨️.mjs";

/** 🐏️ The environment of the process that runs one command of the build tool: a command that reads the project graph gets the collector target and the allocator purge of a graph child (`graphChildEnvironmentV1`), exactly as the trampoline gives them to its door; without them the runtime sizes its garbage by the memory of the host (32 GB: one synchronous stretch of the inference allocated 25 GB before its first full collection) and keeps freed pages in the footprint. */
export function implementationProcessEnvironmentV1(args:readonly string[],base:Readonly<Record<string,string|undefined>>,environment:Readonly<Record<string,string|undefined>>=base):Record<string,string|undefined>{
 let reads=false;try{reads=repositoryCommandReadsGraphV1(args,environment);}catch{}
 return{...base,...(reads?graphChildEnvironmentV1():{}),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"};
}
''')
add(X,'env:{...scriptResourceProcessEnvironmentV1(control,environment),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},auxiliaryInput','env:implementationProcessEnvironmentV1(args,scriptResourceProcessEnvironmentV1(control,environment),environment),auxiliaryInput')
TF=B+"🧠️memory/🦶️footprint/🧪️tests/🟦️.ts"
add(TF,'import{spawnSync}from"node:child_process";\n','import{spawnSync}from"node:child_process";\nimport{implementationProcessEnvironmentV1}from"../../../🏛️sealed/📥️execution/🟦️.ts";\nimport{graphChildEnvironmentV1}from"../../🐏️child-environment/🟨️.mjs";\n')
add(TF,'//R36b\n','''//R36b
test("the process that runs a graph-reading command of the build tool carries the collector cap and the allocator purge, a command that never loads the graph does not",()=>{
 const base={PATH:"/bin",KEEP:"1"},cap=graphChildEnvironmentV1();
 for(const args of[["show","projects"],["build","some-project"],["test","some-project"]]){
  const row=implementationProcessEnvironmentV1(args,base,{});
  expect(row).toMatchObject({...cap,PATH:"/bin",KEEP:"1",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"});
 }
 for(const args of[["doctor"],["tools","list"],["capacity"]]){
  const row=implementationProcessEnvironmentV1(args,base,{});
  for(const key of Object.keys(cap))expect(row[key]).toBeUndefined();
  expect(row.BUN_RUNTIME_TRANSPILER_CACHE_PATH).toBe("0");
 }
 expect(cap.BUN_JSC_forceRAMSize).toBe(String(512*1048576));
});
''')
