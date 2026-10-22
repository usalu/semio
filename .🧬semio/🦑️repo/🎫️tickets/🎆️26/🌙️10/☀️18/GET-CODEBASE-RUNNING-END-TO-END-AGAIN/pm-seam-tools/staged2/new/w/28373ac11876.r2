import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import {parse as parseTomlThirdParty} from "smol-toml";
import {parse as parseJsoncThirdParty} from "jsonc-parser";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,statSync,lstatSync} from "node:fs";
import {readdir,lstat} from "node:fs/promises";
import {tmpdir} from "node:os";
import {createHash} from "node:crypto";
import {dirname,join,relative,sep} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import * as core from "../🟦️.ts";
import {parseJsoncV1,parseTomlV1} from "../📖️formats/🟦️.ts";
import {builtInPackageManagersV1,currentHostV1,packageManagerRegistryV1,selectPackageManagerV1} from "../📚️catalog/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};
import ratchet from "./⚖️ratchet/🔣️.json" with {type:"json"};
import owned from "../🔣️owned.json" with {type:"json"};
import catalogDocument from "../🔣️catalog.json" with {type:"json"};
import {createRequire} from "node:module";
import installBase from "../../💾️storage/🧬️schema/📥️install.json" with {type:"json"};
import generationBase from "../../🧾️generation/🧬️schema/🔣️.json" with {type:"json"};
import originalInstall from "./🧫️fixtures/📥️install.original.json" with {type:"json"};
import originalGeneration from "./🧫️fixtures/🧾️generation.original.json" with {type:"json"};
import {materializeSchemaV1,schemaViewV1} from "../🧩️materialize/🟦️.ts";
import {projectBunLockV1} from "../🍞️bun/🟦️.ts";
import cachingPolicy from "../../../../🔣️policy.json" with {type:"json"};
import {javascriptInstallSchemaV1 as installSchema} from "../../💾️storage/🧱️schemas/🟦️.ts";

type Json=any;
const protocol=core.packageManagerProtocolV1,here=dirname(fileURLToPath(import.meta.url)),managersRoot=dirname(here),libraryRoot=join(here,"..","..","..","..","..");
const ajv=new Ajv({strict:false,allErrors:true});
const compile=(name:string)=>ajv.compile({...schema,$ref:"#/$defs/"+name});
const bytes=(text:string)=>new TextEncoder().encode(text);
const host=(row:Json)=>({platform:row.host.platform,arch:row.host.arch,versions:row.host.versions as Record<string,string>,probe:row.probe===undefined||row.probe===null?undefined:()=>row.probe as string});
const failure=(run:()=>unknown):string|undefined=>{try{run();return undefined;}catch(error){return (error as Error).message;}};
const vectorsOf=(id:string):Json=>(vectors.managers as Record<string,Json>)[id];

describe("package manager seam: registry and schema",()=>{
 test("the vectors cover exactly the registered managers",()=>{
  expect(Object.keys(vectors.managers).sort()).toEqual(packageManagerRegistryV1.all().map(manager=>manager.id));
  expect(packageManagerRegistryV1.all().length).toBe(builtInPackageManagersV1.length);
 });
 test("every descriptor, the policy, and every produced value satisfy the schema under two independent validators",()=>{
  const descriptor=compile("Descriptor"),selection=compile("Selection"),graph=compile("LockGraph"),receipt=compile("Receipt");
  for(const manager of packageManagerRegistryV1.all()){
   expect(descriptor(manager.descriptor)).toBe(true);
   expect(validateJsonSchemaSubset({...schema,$ref:"#/$defs/Descriptor"} as never,manager.descriptor)).toEqual([]);
   for(const row of vectorsOf(manager.id).lock){expect(graph(row.expected)).toBe(true);expect(validateJsonSchemaSubset({...schema,$ref:"#/$defs/LockGraph"} as never,row.expected)).toEqual([]);}
   for(const row of vectorsOf(manager.id).receipt){expect(receipt(row.expected)).toBe(true);}
  }
  expect(selection(policy)).toBe(true);
  expect(validateJsonSchemaSubset({...schema,$ref:"#/$defs/Selection"} as never,policy)).toEqual([]);
 });
 test("the descriptors are pinned by digest so a change is a reviewed vector change",()=>{
  for(const manager of packageManagerRegistryV1.all())expect(core.sha256HexV1(core.canonicalJsonV1(manager.descriptor))).toBe(vectorsOf(manager.id).descriptorSha256);
 });
 test("managers of one ecosystem agree on toolchain and generation directory, and every toolchain is in the caching policy",()=>{
  for(const ecosystem of packageManagerRegistryV1.ecosystems()){
   const managers=packageManagerRegistryV1.forEcosystem(ecosystem);
   expect(new Set(managers.map(manager=>manager.generationDirectory())).size).toBe(1);
   expect(new Set(managers.map(manager=>manager.descriptor.toolchain)).size).toBe(1);
   expect(Object.keys(cachingPolicy.toolchains)).toContain(managers[0]!.descriptor.toolchain);
  }
  expect(failure(()=>new core.PackageManagerRegistryV1().register(builtInPackageManagersV1[0]!).register(builtInPackageManagersV1[0]!))).toContain("Unique package manager id required");
 });
 test("no two managers claim the same lock file name",()=>{
  const names=builtInPackageManagersV1.flatMap(manager=>manager.lockFileNames());
  expect(new Set(names).size).toBe(names.length);
  expect(packageManagerRegistryV1.lockFileNames()).toEqual([...new Set(names)].sort());
 });
 test("the bun keys equal the formulas the live code used before the seam (no cold re-install from the swap itself)",()=>{
  const bun=packageManagerRegistryV1.get("bun"),recipe=installSchema.$defs.ProducerRecipeProtocol.const,hostNow=currentHostV1(),manifest={dependencies:{a:"1"},devDependencies:{b:"2"},patchedDependencies:{"c@1":"p"}},lock=bytes("lock\n");
  const liveBootstrap=createHash("sha256").update(JSON.stringify([manifest.dependencies,manifest.devDependencies,undefined,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])).update(lock).digest("hex");
  expect(core.bootstrapKeyV1(bun,{manifest,host:hostNow,lock})).toBe(liveBootstrap);
  const members=[{path:"w/0/package.json"}],material=[{path:"/r/package.json",bytes:bytes("{}")},{path:"/r/bun.lock",bytes:bytes("x")}],modes=installSchema.$defs.SourceOwnerInputGroupProtocol.const.modeIdentity;
  const live=createHash("sha256").update(JSON.stringify([recipe,"/r",process.versions.bun,process.platform,process.arch,modes.sync,members]));
  for(const row of material)live.update(JSON.stringify([row.path,row.bytes.length])).update(row.bytes);
  expect(core.generationKeyV1(bun,{root:"/r",host:hostNow,mode:modes.sync,members,material})).toBe(live.digest("hex"));
  const rendered=(mode:"sync"|"lock")=>bun.render(mode,{platform:process.platform,cache:"/cache"}).arguments;
  expect(rendered("sync")).toEqual(["install","--frozen-lockfile","--"+recipe.lifecycle,"--linker="+recipe.linker,"--backend="+(recipe.backends as Record<string,string>)[process.platform],"--cache-dir","/cache"]);
  expect(rendered("lock")[1]).toBe("--lockfile-only");
 });
 test("selection follows the policy and the neutral environment override, and refuses a manager of another ecosystem",()=>{
  expect(Object.keys(policy.selection).sort()).toEqual([...packageManagerRegistryV1.ecosystems()]);
  for(const [ecosystem,id] of Object.entries(policy.selection))expect(selectPackageManagerV1(ecosystem,{}).id).toBe(id);
  expect(selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"npm"}).id).toBe("npm");
  expect(failure(()=>selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"cargo"}))).toContain("selected ecosystem");
  expect(failure(()=>selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"missing"}))).toContain("Registered package manager required");
  expect(failure(()=>selectPackageManagerV1("elixir",{}))).toContain("Selected package manager required");
 });
});

describe("package manager seam: language-agnostic contract vectors",()=>{
 for(const manager of builtInPackageManagersV1){
  const vector=vectorsOf(manager.id);
  test(manager.id+" renders every operation exactly as the vectors say",()=>{
   for(const row of vector.render){
    const actual=(()=>{try{return JSON.parse(JSON.stringify(manager.render(row.operation,row.request)));}catch(error){return {error:(error as Error).message};}})();
    expect(actual).toEqual(row.expected);
   }
   expect(failure(()=>manager.render("lock",{platform:"linux",extra:"x"} as never))).toContain("Closed package manager request required");
  });
  test(manager.id+" selects the declared backend and refuses aliasing ones",()=>{
   for(const row of vector.backend)expect(manager.backend(row.platform)??null).toBe(row.expected);
   for(const [name,kind] of Object.entries(manager.descriptor.backendKinds)){
    const probe=core.createPackageManagerV1({...manager.descriptor,recipe:{...manager.descriptor.recipe,backends:{darwin:name}}} as never,()=>({packages:[],members:[]}));
    if(kind==="alias")expect(failure(()=>probe.backend("darwin"))).toContain("Supported non-aliasing install backend required");
    else expect(probe.backend("darwin")).toBe(name);
   }
  });
  test(manager.id+" renders the isolated environment",()=>{
   for(const row of vector.environment)expect(manager.environment(row.root,row.ambient,row.separator,row.overrides)).toEqual(row.expected);
  });
  test(manager.id+" resolves its runtime identity from the host or its own probe",()=>{
   for(const row of vector.runtime){
    const outcome=(()=>{try{return manager.runtimeIdentity(host({host:{platform:"linux",arch:"x64",versions:row.versions},probe:row.probe}));}catch(error){return {error:(error as Error).message};}})();
    expect(outcome).toEqual(row.expected);
   }
  });
  test(manager.id+" derives generation and bootstrap keys byte for byte",()=>{
   for(const row of vector.key)expect(core.generationKeyV1(manager,{root:row.root,host:host(row),mode:row.mode,members:row.members,material:row.material.map((item:Json)=>({path:item.path,bytes:bytes(item.text)}))})).toBe(row.expected);
   for(const row of vector.bootstrapKey)expect(core.bootstrapKeyV1(manager,{manifest:row.manifest,host:host(row),lock:bytes(row.lock)})).toBe(row.expected);
   const [first,second]=vector.key;expect(first.expected).not.toBe(second.expected);
  });
  test(manager.id+" reads its lock fixtures into the neutral graph",()=>{
   for(const row of vector.lock)expect(JSON.parse(JSON.stringify(manager.parseLock((fixtures as Json)[manager.id][row.fixture].text)))).toEqual(row.expected);
   expect(failure(()=>manager.parseLock(new Uint8Array([0xff,0xfe])))).toBeDefined();
  });
  test(manager.id+" walks the providers its lock pins",async()=>{
   for(const row of vector.providers){
    const graph=manager.parseLock((fixtures as Json)[manager.id][row.fixture].text);
    expect(core.expectedProvidersV1(manager,graph)).toEqual(row.providers);
    expect(await core.walkProvidersV1(manager,graph,memoryPort(manager,row.listing))).toEqual(row.expected);
   }
  });
  test(manager.id+" seals receipts identically",()=>{
   for(const row of vector.receipt){expect(core.sealV1(row.body)).toEqual(row.expected);expect(core.openSealedV1(row.expected)).toEqual(row.body);}
   expect(failure(()=>core.openSealedV1({...vector.receipt[0].expected,scope:"tampered"}))).toContain("Sealed package manager value required");
  });
 }
 test("canonical JSON and the digest agree with the oracle",()=>{
  for(const row of vectors.canonical){expect(core.canonicalJsonV1(row.value)).toBe(row.json);expect(core.sha256HexV1(row.json)).toBe(row.sha256);}
 });
 test("the independent Python oracle re-derives every vector",()=>{
  const run=spawnSync(testPythonV1()??"python3",["-I",join(here,"🐍️oracle.py"),"check",join(here,"🔣️.json")],{encoding:"utf8"});
  expect(run.stderr).toBe("");
  expect(run.stdout).toContain("vectors equal the oracle");
  expect(run.status).toBe(0);
 });
});

function memoryPort(manager:core.PackageManagerV1,listing:readonly string[]):core.ProviderPortV1{
 const containers=manager.descriptor.installed.containers.map(pattern=>pattern.replace(/\*/gu,"x")),container=containers[0]!,kinds=new Map<string,"directory"|"file">(),lists=new Map<string,string[]>();
 const parts=container==="."?[]:container.split("/");
 for(let at=0;at<=parts.length;at++)kinds.set(at===0?".":parts.slice(0,at).join("/"),"directory");
 const insert=(path:string)=>{const segments=path.split("/");for(let at=1;at<=segments.length;at++){const current=(container==="."?"":container+"/")+segments.slice(0,at).join("/");kinds.set(current,"directory");const parent=at===1?(container==="."?".":container):(container==="."?"":container+"/")+segments.slice(0,at-1).join("/");const list=lists.get(parent)??[];if(!list.includes(segments[at-1]!))list.push(segments[at-1]!);lists.set(parent,list);}};
 for(const name of listing)insert(name);
 for(let at=1;at<parts.length;at++){lists.set(at===1?(parts[0]!):parts.slice(0,at).join("/"),[parts[at]!]);}
 if(parts.length)lists.set(".",[parts[0]!]);
 return {kind:async path=>kinds.get(path)??"missing",list:async path=>lists.get(path)??[]};
}

describe("package manager seam: one scenario, any manager",()=>{
 const probeOf=(manager:core.PackageManagerV1):string=>vectorsOf(manager.id).runtime.find((row:Json)=>typeof row.probe==="string").probe;
 const scenario=(manager:core.PackageManagerV1,fixture:string)=>{
  const text=(fixtures as Json)[manager.id][fixture].text as string,graph=manager.parseLock(text),listing=memoryListing(manager,graph),calls:string[]=[],store=new Map<string,string>();
  const ports={
   runner:{run:async(invocation:core.ManagerInvocationV1,environment:Readonly<Record<string,string>>)=>{calls.push(invocation.executable+" "+invocation.arguments.join(" "));expect(Object.keys(environment).length).toBeGreaterThan(0);return {code:0};}},
   receipts:{read:async(name:string)=>store.get(name)??null,write:async(name:string,value:string)=>{store.set(name,value);}},
   providers:memoryPort(manager,listing),
  };
  const input={scope:".",root:"/store",host:{platform:"linux",arch:"x64",versions:{[manager.descriptor.runtime.hostVersionKey??"unused"]:"9.9.9"},probe:()=>probeOf(manager)},mode:"locked",members:[],material:[{path:"lock",bytes:bytes(text)}],lock:bytes(text),request:{platform:"linux",cache:"/store/cache",directory:"/work",manifest:"/work/"+manager.manifestFileNames()[0]},ambient:{PATH:"/bin"}};
  return {graph,calls,store,ports,input};
 };
 const eligible=builtInPackageManagersV1.map(manager=>[manager,Object.keys((fixtures as Json)[manager.id])[0]!] as const);
 for(const [manager,fixture] of eligible){
  test(manager.id+": ensure syncs once, then answers from the sealed receipt",async()=>{
   const run=scenario(manager,fixture),first=await core.ensureGenerationV1(manager,run.input,run.ports);
   expect(first.status).toBe("synced");
   expect(run.calls.length).toBe(1);
   expect(run.calls[0]).toBe(manager.descriptor.executable+" "+manager.render("sync",run.input.request).arguments.join(" "));
   expect(first.receipt.generation.location.startsWith(manager.generationDirectory()+"/")).toBe(true);
   expect(first.receipt.lock.name).toBe(manager.primaryLockName());
   const second=await core.ensureGenerationV1(manager,run.input,run.ports);
   expect(second.status).toBe("current");expect(run.calls.length).toBe(1);expect(second.receipt).toEqual(first.receipt);
   const stale=await core.ensureGenerationV1(manager,{...run.input,lock:bytes("changed"),material:[{path:"lock",bytes:bytes("changed")}]},run.ports).catch(error=>error as Error);
   expect(stale).toBeInstanceOf(Error);
   run.store.set([...run.store.keys()][0]!,"{not json");
   expect((await core.ensureGenerationV1(manager,run.input,run.ports)).status).toBe("synced");
  });
  test(manager.id+": ensure refuses a failed sync and a missing provider, and honors cancellation",async()=>{
   const run=scenario(manager,fixture);
   expect(await core.ensureGenerationV1(manager,run.input,{...run.ports,runner:{run:async()=>({code:3})}}).catch(error=>(error as Error).message)).toContain("Successful package manager sync required");
   if(run.graph.packages.length>0&&core.expectedProvidersV1(manager,run.graph).length>0)expect(await core.ensureGenerationV1(manager,run.input,{...run.ports,providers:memoryPort(manager,[])}).catch(error=>(error as Error).message)).toContain("Complete locked providers required");
   const controller=new AbortController();controller.abort();
   expect(await core.ensureGenerationV1(manager,run.input,run.ports,{signal:controller.signal}).catch(error=>(error as Error).name)).toBe("AbortError");
   expect(run.store.size).toBe(0);
  });
 }
 test("swapping the javascript manager changes lock name, command and layout but nothing in the flow",async()=>{
  const outcomes=[];
  for(const id of ["bun","npm"]){
   const manager=selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:id}),run=scenario(manager,Object.keys((fixtures as Json)[id])[0]!),result=await core.ensureGenerationV1(manager,run.input,run.ports);
   outcomes.push({id,status:result.status,lock:result.receipt.lock.name,directory:result.receipt.generation.location.split("/")[0],command:run.calls[0]!.split(" ")[0],key:result.key});
  }
  expect(outcomes.map(row=>row.status)).toEqual(["synced","synced"]);
  expect(outcomes.map(row=>row.directory)).toEqual(["j","j"]);
  expect(outcomes[0]!.lock).not.toBe(outcomes[1]!.lock);
  expect(outcomes[0]!.command).not.toBe(outcomes[1]!.command);
  expect(outcomes[0]!.key).not.toBe(outcomes[1]!.key);
 });
 test("a large lock reports bounded progress and stops on cancellation",async()=>{
  const manager=selectPackageManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"npm"}),packages=Array.from({length:600},(_,index)=>({name:"p"+index,version:"1.0.0",key:"node_modules/p"+index,source:null,integrity:null})),graph={version:1 as const,format:"package-lock",packages,members:[]};
  const port=memoryPort(manager,packages.map(row=>row.name)),seen:number[]=[];
  expect(await core.walkProvidersV1(manager,graph,port,{advance:count=>{seen.push(count);}})).toEqual({status:"match",examined:600});
  expect(seen).toEqual([256,512]);
  const controller=new AbortController();
  expect(await core.walkProvidersV1(manager,graph,port,{signal:controller.signal,advance:()=>controller.abort()}).catch(error=>(error as Error).name)).toBe("AbortError");
  const missing=await core.walkProvidersV1(manager,graph,memoryPort(manager,[]));
  expect(missing.status).toBe("mismatch");if(missing.status==="mismatch"){expect(missing.missing.length).toBe(protocol.maximumMissingReported);expect(missing.truncated).toBe(true);}
 });
 test("the provider walk agrees with a real directory tree",async()=>{
  const manager=selectPackageManagerV1("javascript",{}),graph=manager.parseLock((fixtures as Json).bun["tools-excerpt"].text),root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"pm-seam-")));
  try{
   const entries=core.expectedProvidersV1(manager,graph).map(row=>row.entry);
   for(const entry of entries)mkdirSync(join(root,"node_modules",".bun",entry+(entry.startsWith("typescript")?"+0123456789abcdef":"")),{recursive:true});
   const port:core.ProviderPortV1={kind:async path=>{try{const stat=await lstat(join(root,path));return stat.isDirectory()?"directory":stat.isSymbolicLink()?"link":stat.isFile()?"file":"other";}catch{return "missing";}},list:async path=>readdir(join(root,path))};
   expect(await core.walkProvidersV1(manager,graph,port)).toEqual({status:"match",examined:entries.length});
   rmSync(join(root,"node_modules",".bun",entries[0]!),{recursive:true});
   const broken=await core.walkProvidersV1(manager,graph,port);
   expect(broken.status).toBe("mismatch");
  }finally{rmSync(root,{recursive:true,force:true});}
 });
});

function memoryListing(manager:core.PackageManagerV1,graph:core.LockGraphV1):string[]{return core.expectedProvidersV1(manager,graph).map(row=>row.entry);}

describe("package manager seam: third-party cross checks",()=>{
 const canonical=(value:unknown)=>core.canonicalJsonV1(JSON.parse(JSON.stringify(value,(_,item)=>typeof item==="bigint"?Number(item):item instanceof Date?item.toISOString():item)));
 test("the TOML subset reader equals smol-toml on every TOML lock fixture and on real repository locks",()=>{
  const texts=[(fixtures as Json).cargo["lowpoly-excerpt"].text,(fixtures as Json).cargo["repo-test-host"].text,(fixtures as Json).uv.styling.text];
  for(const real of ["Cargo.lock","uv.lock","🧰️framework/Cargo.lock","🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🐍️python/uv.lock"]){const path=join(libraryRoot,"..","..","..","..","..",real);if(existsSync(path)&&statSync(path).size<4_000_000)texts.push(readFileSync(path,"utf8"));}
  for(const text of texts)expect(canonical(parseTomlV1(text))).toBe(canonical(parseTomlThirdParty(text)));
  expect(canonical(parseTomlV1('a.b = 1\n[t]\nx = "a\\u00e9"\ny = [1, 2,\n 3, # c\n]\nz = {p = true, q = \'lit\'}\n[[r]]\nn = 1\n[[r]]\nn = 2\n[r.sub]\nk = """\nml"""\n'))).toBe(canonical(parseTomlThirdParty('a.b = 1\n[t]\nx = "a\\u00e9"\ny = [1, 2,\n 3, # c\n]\nz = {p = true, q = \'lit\'}\n[[r]]\nn = 1\n[[r]]\nn = 2\n[r.sub]\nk = """\nml"""\n')));
  expect(failure(()=>parseTomlV1("a = \n"))).toContain("Closed TOML subset required");
  expect(failure(()=>parseTomlV1("a = 1\na = 2\n"))).toContain("duplicate key");
 });
 test("the JSONC reader equals jsonc-parser on comments, trailing commas and tricky strings",()=>{
  const cases=['{"a":[1,2,],"b":{"c":"//not a comment, ]",},}','// head\n{"a":1 /* inline */,"b":"x\\"y",}\n','{\n "k": ["v", // c\n "w",\n],\n}','[]','{"u":"\\u00e9,}"}'];
  for(const text of cases)expect(canonical(parseJsoncV1(text))).toBe(canonical(parseJsoncThirdParty(text)));
  for(const manager of packageManagerRegistryV1.forEcosystem("javascript"))for(const fixture of Object.values((fixtures as Json)[manager.id]) as Json[])if(manager.id==="bun")expect(canonical(parseJsoncV1(fixture.text))).toBe(canonical(parseJsoncThirdParty(fixture.text)));
  expect(failure(()=>parseJsoncV1('{"a": /* open'))).toContain("Terminated JSONC comment required");
 });
 test("every real repository lock parses, and package counts equal the third-party parsers",()=>{
  const root=join(libraryRoot,"..","..","..","..","..");
  const live:[string,string][]=[["bun","bun.lock"],["bun","🎓️teaching/bun.lock"],["cargo","Cargo.lock"],["uv","uv.lock"]];
  for(const [id,file] of live){
   const path=join(root,file);if(!existsSync(path))continue;
   const text=readFileSync(path,"utf8"),graph=packageManagerRegistryV1.get(id).parseLock(text);
   const counted=id==="bun"?Object.entries((parseJsoncThirdParty(text) as Json).packages).filter(([,row])=>!/^(workspace|file|link|portal):/u.test(((row as string[])[0]!).slice((row as string[])[0]!.lastIndexOf("@")+1))).length
    :((parseTomlThirdParty(text) as Json).package as Json[]).filter(row=>id==="cargo"?row.source!==undefined:!["editable","virtual","directory","path"].includes(Object.keys(row.source)[0]!)).length;
   expect(graph.packages.length).toBeLessThanOrEqual(counted);
   expect(graph.packages.length).toBeGreaterThan(0);
  }
 });
 test("rendered flags are known to the real tools",()=>{
  for(const manager of builtInPackageManagersV1){
   const tool=spawnSync(manager.descriptor.executable,manager.descriptor.runtime.probe.arguments,{encoding:"utf8"});
   if(tool.error||tool.status!==0)continue;
   for(const operation of ["lock","sync","verify"] as const){
    if(!manager.descriptor.operations[operation])continue;
    const invocation=manager.render(operation,{platform:process.platform,cache:"/c",directory:"/d",manifest:"/m"}),leading:string[]=[];
    for(const word of invocation.arguments){if(word.startsWith("-")||!/^[a-z][a-z-]*$/u.test(word))break;leading.push(word);}
    const help=manager.id==="go"?spawnSync("go",["help",...leading],{encoding:"utf8"}):spawnSync(manager.descriptor.executable,[...leading,"--help"],{encoding:"utf8"}),text=help.stdout+help.stderr;
    expect(leading.length).toBeGreaterThan(0);
    expect(help.status===0||text.length>0).toBe(true);
    const options=manager.id==="npm"?spawnSync("npm",["config","ls","-l"],{encoding:"utf8"}).stdout:"";
    for(const flag of invocation.arguments.filter(word=>word.startsWith("--")).map(word=>word.split("=")[0]!))expect(text.includes(flag)||options.includes("\n"+flag.slice(2)+" = "),flag).toBe(true);
   }
  }
 });
});

describe("package manager seam: neutrality laws",()=>{
 const adapterTokens=()=>{
  const tokens=new Set<string>();
  for(const manager of builtInPackageManagersV1){
   tokens.add(manager.id);
   for(const name of [...manager.lockFileNames(),...manager.configurationNames("scope"),...manager.configurationNames("home"),...manager.configurationNames("config")])tokens.add(name);
   if(manager.descriptor.runtime.hostVersionKey)tokens.add(manager.descriptor.runtime.hostVersionKey);
  }
  return [...tokens].filter(token=>token.length>1);
 };
 const mentions=(text:string,token:string)=>new RegExp("(?<![\\w.-])"+token.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")+"(?![\\w.-])","u").test(text);
 test("the neutral core, formats and schema name no manager, lock file, configuration file or host runtime",()=>{
  for(const file of [join(managersRoot,"🟦️.ts"),join(managersRoot,"📖️formats","🟦️.ts"),join(managersRoot,"🧩️materialize","🟦️.ts"),join(managersRoot,"🧬️schema","🔣️.json")]){
   const text=readFileSync(file,"utf8").replace(/"(javascript|rust|go|python|dotnet)"/gu,"");
   const hits=adapterTokens().filter(token=>!["go"].includes(token)&&mentions(text,token));
   expect({file:relative(managersRoot,file),hits}).toEqual({file:relative(managersRoot,file),hits:[]});
  }
  expect(readFileSync(join(managersRoot,"🟦️.ts"),"utf8")).not.toMatch(/process\.|Bun\.|node:fs|node:child_process/u);
 });
 test("only the catalog imports adapters, and adapters import nothing but the core, the formats and their own descriptor",()=>{
  const importsOf=(file:string)=>[...readFileSync(file,"utf8").matchAll(/from\s*"([^"]+)"/gu)].map(match=>match[1]!);
  expect(importsOf(join(managersRoot,"🟦️.ts")).filter(path=>path.startsWith("."))).toEqual(["./🧬️schema/🔣️.json","./🔣️policy.json"]);
  for(const manager of builtInPackageManagersV1){
   const folder=readdirSync(managersRoot).find(name=>name.endsWith(manager.id)&&existsSync(join(managersRoot,name,"🟦️.ts")))!;
   const allowed=new Set(["../🟦️.ts","../📖️formats/🟦️.ts","./🔣️.json"]);
   expect(importsOf(join(managersRoot,folder,"🟦️.ts")).filter(path=>!allowed.has(path))).toEqual([]);
  }
 });
 test("the bun descriptor materializes the pre-seam install and generation schemas exactly, and its recipe keeps the hashed key order",()=>{
  const bun=packageManagerRegistryV1.get("bun"),view=schemaViewV1(bun.descriptor),install=materializeSchemaV1(installBase,view) as any,generation=materializeSchemaV1(generationBase,view) as any;
  const {ManagerProtocol,...rest}=install.$defs;
  expect(core.canonicalJsonV1({...install,$defs:rest})).toBe(core.canonicalJsonV1(originalInstall));
  expect(ManagerProtocol.const).toEqual({manager:"bun",label:"Bun",lockFile:"bun.lock",scopeConfig:"bunfig.toml",generationDirectory:"j",strictProfilePrefix:"bun-",hostVersionKey:"bun",cacheVariable:"BUN_INSTALL_CACHE_DIR"});
  const expectedGeneration=structuredClone(originalGeneration) as any;
  expectedGeneration.$defs.Runtime.required=expectedGeneration.$defs.Runtime.required.map((name:string)=>name==="bun"?"manager":name);
  expectedGeneration.$defs.Runtime.properties=Object.fromEntries(Object.entries(expectedGeneration.$defs.Runtime.properties).map(([name,value])=>[name==="bun"?"manager":name,value]));
  expect(core.canonicalJsonV1(generation)).toBe(core.canonicalJsonV1(expectedGeneration));
  expect(JSON.stringify(install.$defs.ProducerRecipeProtocol.const)).toBe(JSON.stringify((originalInstall as any).$defs.ProducerRecipeProtocol.const));
  expect(JSON.stringify(bun.recipe())).toBe(JSON.stringify((originalInstall as any).$defs.ProducerRecipeProtocol.const));
  expect(installSchema.$defs.ManagerProtocol.manager).toBe(bun);
  expect(installSchema.$defs.ManagerProtocol.const.runtime).toBe(process.versions.bun);
 });
 test("every manager materializes both schema bases without a leftover token, and the digests equal the oracle",()=>{
  for(const manager of builtInPackageManagersV1){
   const view=schemaViewV1(manager.descriptor),install=materializeSchemaV1(installBase,view),generation=materializeSchemaV1(generationBase,view);
   for(const [name,value] of [["install",install],["generation",generation]] as const){
    expect(JSON.stringify(value),manager.id+" "+name).not.toMatch(/\{\{|\$manager/u);
    expect(core.sha256HexV1(core.canonicalJsonV1(value)),manager.id+" "+name).toBe(vectorsOf(manager.id).materialized[name]);
   }
  }
  expect(failure(()=>materializeSchemaV1({a:"{{missing.path}}"},{}))).toContain("Declared manager value required");
  expect(failure(()=>materializeSchemaV1({a:"{{recipe}}"},{recipe:{}}))).toContain("Declared manager text required");
 });
 test("the bun lock projection equals the pre-seam driver arrow in both directions",()=>{
  const original=(text:string,reverse:boolean,mapping:Map<string,string>,patches:Map<string,{source:string;target:string}>,scope:string)=>{
   const lock=parseJsoncThirdParty(text) as any,links=reverse?new Map([...mapping].map(([source,target])=>[target,source])):mapping;
   if(lock.patchedDependencies)lock.patchedDependencies=Object.fromEntries([...patches].map(([name,row])=>[name,reverse?relative(scope,row.source).split(sep).join("/"):row.target]));
   lock.workspaces=Object.fromEntries(Object.entries(lock.workspaces??{}).filter(([path])=>reverse||path===""||links.has(path)).map(([path,value])=>[links.get(path)??path,value]));
   for(const[key,row]of Object.entries(lock.packages??{}) as [string,any][]){if(!Array.isArray(row)||typeof row[0]!=="string")continue;const marker=row[0].indexOf("@workspace:");if(marker<0)continue;const path=row[0].slice(marker+11),target=links.get(path);if(!target){if(reverse)throw Error("Locked workspace package lacks original admitted Source");delete lock.packages[key];continue;}row[0]=row[0].slice(0,marker+11)+target;}
   return JSON.stringify(lock,null,2)+"\n";
  };
  const text=(fixtures as Json).bun["tools-excerpt"].text as string,mapping=new Map([["packages/member","w/0"]]),patches=new Map([["left-pad@1.3.0",{source:"patches/left-pad.patch",target:"patches/0.patch"}]]);
  const withPatch=text.replace('"packages": {','"patchedDependencies": {\n    "left-pad@1.3.0": "patches/left-pad.patch",\n  },\n  "packages": {');
  for(const input of [text,withPatch]){
   const projection={reverse:false,mapping,patches:[...patches].map(([name,row])=>({name,forward:row.target,reverse:relative(".",row.source).split(sep).join("/")}))};
   const forward=projectBunLockV1(input,projection);
   expect(forward).toBe(original(input,false,mapping,patches,"."));
   const back=projectBunLockV1(forward,{...projection,reverse:true});
   expect(back).toBe(original(forward,true,mapping,patches,"."));
  }
  expect(failure(()=>projectBunLockV1(text,{reverse:true,mapping:new Map(),patches:[]}))).toContain("Locked workspace package lacks original admitted Source");
 });
 test("optional argument groups and scopes render, and an undeclared scope is refused",()=>{
  const uv=packageManagerRegistryV1.get("uv");
  expect(uv.render("sync",{platform:"linux"}).arguments).toEqual(["sync","--locked"]);
  expect(uv.render("sync",{platform:"linux",cache:"/c",scope:"all"}).arguments).toEqual(["sync","--locked","--cache-dir","/c","--all-packages","--all-groups"]);
  expect(failure(()=>uv.render("sync",{platform:"linux",scope:"nothing"}))).toContain("Declared package manager scope required");
  expect(failure(()=>packageManagerRegistryV1.get("bun").render("sync",{platform:"linux",cache:"/c",scope:"all"}))).toContain("Declared package manager scope required");
  expect(packageManagerRegistryV1.get("go").environment("/r",{},"/",{GOWORK:"/w/go.work"}).GOWORK).toBe("/w/go.work");
 });
 test("production code outside the adapters and the declared repository data names no package manager lock, configuration, flag or role name (ratchet is zero)",async()=>{
  const pattern=/bun\.lock|bun\.lockb|bunfig|--frozen-lockfile|--lockfile-only|BUN_INSTALL|bun-(?:cache|global|bin|home|config|import|require)/gu,skip=new Set(["node_modules","🧪️tests","🧫️fixtures",".git","target","dist",".cache","🔌️managers"]),found:Record<string,number>={};
  const modules=builtInPackageManagersV1.flatMap(manager=>manager.descriptor.ownedModules),data=Object.keys(owned.data);
  const walk=async(directory:string)=>{
   for(const entry of await readdir(directory,{withFileTypes:true})){
    const path=join(directory,entry.name),relativePath=relative(libraryRoot,path).split(sep).join("/");
    if(entry.isDirectory()){if(!skip.has(entry.name)&&!modules.includes(relativePath))await walk(path);continue;}
    if(!/\.(ts|mjs|cjs|json)$/u.test(entry.name)||modules.includes(relativePath)||data.includes(relativePath))continue;
    const count=(readFileSync(path,"utf8").match(pattern)??[]).length;
    if(count)found[relativePath]=count;
   }
  };
  await walk(libraryRoot);
  expect(found).toEqual({});
  expect(ratchet.files).toEqual({});
 });
 test("every adapter-owned module exists and every declared repository datum still names the selected manager's files",()=>{
  const names=(ecosystem:string)=>{const manager=selectPackageManagerV1(ecosystem,{});return {lock:manager.primaryLockName(),scopeConfig:manager.configurationNames("scope")[0]!};};
  for(const manager of builtInPackageManagersV1)for(const module of manager.descriptor.ownedModules)expect(existsSync(join(libraryRoot,module)),module).toBe(true);
  for(const [file,row] of Object.entries(owned.data)){
   const text=readFileSync(join(libraryRoot,file),"utf8"),selected=names(row.ecosystem);
   for(const key of row.names)expect(text.includes(selected[key as "lock"|"scopeConfig"]),file+" "+key).toBe(true);
  }
 });
 test("the CommonJS accessor and the catalog agree with the registry for every ecosystem and manager",()=>{
  const accessor=createRequire(import.meta.url)("../🟨️.cjs");
  expect(Object.keys(catalogDocument.adapters).sort()).toEqual(packageManagerRegistryV1.all().map(manager=>manager.id));
  for(const manager of packageManagerRegistryV1.all()){
   const row=accessor.managerV1(manager.id);
   expect(row.primaryLockName).toBe(manager.primaryLockName());expect([...row.lockFileNames]).toEqual([...manager.lockFileNames()]);expect(row.generationDirectory).toBe(manager.generationDirectory());
   expect(row.scopeConfigName??null).toBe(manager.configurationNames("scope")[0]??null);expect(row.graphComponent).toBe(manager.descriptor.graphComponent);expect(catalogDocument.adapters[manager.id as keyof typeof catalogDocument.adapters]).toBeDefined();
  }
  for(const ecosystem of packageManagerRegistryV1.ecosystems()){expect(accessor.selectedManagerV1(ecosystem,{}).id).toBe(selectPackageManagerV1(ecosystem,{}).id);}
  expect(accessor.selectedManagerV1("javascript",{SEMIO_PACKAGE_MANAGER_JAVASCRIPT:"npm"}).primaryLockName).toBe("package-lock.json");
  expect(accessor.sourceFilesV1().length).toBe(2+1+packageManagerRegistryV1.all().length);
 });
 test("the cargo preparation pair runs exactly the commands the cargo manager renders",()=>{
  const source=readFileSync(join(libraryRoot,"🗂️workspaces","🦀️cargo","🛠️preparation","🟦️.ts"),"utf8"),cargo=packageManagerRegistryV1.get("cargo");
  const literal=(operation:"lock"|"sync")=>JSON.stringify(cargo.render(operation,{platform:process.platform,manifest:"MANIFEST"}).arguments).replace('"MANIFEST"',"manifest");
  expect(source).toContain("runner("+literal("lock"));
  expect(source).toContain("runner("+literal("sync"));
 });
 test("the native dependency synchronisation builds python, rust lock and go commands from the managers, never from tool literals",()=>{
  const source=readFileSync(join(libraryRoot,"⚡️caching","🚀️bootstrap","📦️dependencies","🏗️native","🟦️.ts"),"utf8");
  expect(source).toContain('render("python"');expect(source).toContain('render("rust", "lock"');expect(source).toContain('render("go", "sync")');
  expect(source).not.toMatch(/run\("uv"|run\("go", \["mod"|\["update", "--workspace"/u);
  const python=packageManagerRegistryV1.get("uv");
  expect(python.render("provision",{platform:"linux",settings:{series:"3.14"}}).arguments).toEqual(["python","install","3.14"]);
  expect(python.render("sync",{platform:"linux",settings:{project:"."},lists:{groups:["dev","test"]}}).arguments).toEqual(["sync","--locked","--project",".","--group","dev","--group","test"]);
  expect(packageManagerRegistryV1.get("go").render("sync",{platform:"linux"}).arguments).toEqual(["mod","download"]);
  expect(packageManagerRegistryV1.get("cargo").render("lock",{platform:"linux",manifest:"M"}).arguments).toEqual(["update","--workspace","--manifest-path","M"]);
 });
 test("the pinned tooling install renders the plain frozen install with the copying backend",()=>{
  expect(packageManagerRegistryV1.get("bun").render("install-pinned",{platform:"darwin",cache:"/c"}).arguments).toEqual(["install","--frozen-lockfile","--ignore-scripts","--cache-dir","/c","--backend","copyfile"]);
  expect(packageManagerRegistryV1.get("npm").render("install-pinned",{platform:"linux",cache:"/c"}).arguments).toEqual(["ci","--ignore-scripts","--cache","/c"]);
 });
});
