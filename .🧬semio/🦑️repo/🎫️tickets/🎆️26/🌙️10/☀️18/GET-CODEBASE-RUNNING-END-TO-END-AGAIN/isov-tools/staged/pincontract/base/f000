import{expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{delimiter,dirname,join}from"node:path";
import{spawnSync}from"node:child_process";
import{fileURLToPath}from"node:url";
import{guardDecisionV1,guardMessageV1,locatePinnedRuntimeV1,parseProcCmdlineV1,parseProcessArguments2V1,pinnedDistributionV1,pinnedRuntimeV1,splitWindowsCommandLineV1,type RuntimePinRegistryV1}from"../../📌️runtime-pin/🟦️.ts";
import{guardRuntimeV1,installedRuntimeV1,readRuntimePinRegistryV1,systemRuntimePinHostV1,userCacheDirectoryV1}from"../../📌️runtime-pin/⌨️command/🟦️.ts";

const root=fileURLToPath(new URL("../../../../../../../",import.meta.url)),launcher="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher",registryFile=join(root,launcher,"🗂️registry/🔣️.json");
const fixtureFile=join(root,launcher,"🧫️fixtures/📌️runtime-pin/🔣️.json"),oracleFile=join(root,launcher,"🧪️tests/📌️runtime-pin-contract/🐍️oracle/🐍️.py");
const fixture=JSON.parse(readFileSync(fixtureFile,"utf8")) as any,fixtureRegistry=fixture.registry as RuntimePinRegistryV1;
const unix=process.platform!=="win32",hex=(text:string)=>Uint8Array.from(text.match(/../gu)!.map(byte=>parseInt(byte,16)));

function scratch<T>(body:(directory:string)=>T):T{
  const directory=mkdtempSync(join(tmpdir(),"semio-pin-")),clean=()=>rmSync(directory,{recursive:true,force:true});
  let result:T;
  try{result=body(directory);}catch(error){clean();throw error;}
  if(result instanceof Promise)return result.finally(clean)as T;
  clean();return result;
}

test("the pinned archive of every host is the one the fixture names",()=>{
  for(const row of fixture.select)expect(pinnedDistributionV1(fixtureRegistry.runtimes.bun!.distributions,row.platform,row.arch,row.musl)?.archive??null,row.id).toBe(row.archive);
});

test("an install is found in the tools partition, then the user cache, and only when its receipt names the pinned archive",()=>{
  for(const row of fixture.locate)scratch(directory=>{
    const environment={SEMIO_RUNTIME_CACHE:join(directory,"cache")},host={...systemRuntimePinHostV1(directory,environment),platform:row.platform,arch:row.arch,musl:row.musl,installed:(dir:string,exe:string,sha:string)=>installedRuntimeV1(dir,exe,sha,row.platform)};
    if(pinnedDistributionV1(fixtureRegistry.runtimes.bun!.distributions,row.platform,row.arch,row.musl)===undefined){expect(()=>pinnedRuntimeV1(fixtureRegistry,host),row.id).toThrow();expect(row.origin).toBeNull();return;}
    const pin=pinnedRuntimeV1(fixtureRegistry,host),places:Record<string,string>={tools:pin.tools,user:pin.user!};
    for(const store of row.stores){
      const place=places[store.store]!;mkdirSync(place,{recursive:true});
      if(store.executable!=="absent"){const file=join(place,pin.executable);writeFileSync(file,"12345");chmodSync(file,store.executable==="present"?0o755:0o644);}
      if(store.receipt!=="absent")writeFileSync(join(place,".toolchain.json"),JSON.stringify({archive:["ok","size-mismatch","ok-sized"].includes(store.receipt)?pin.sha256:"0".repeat(64),...store.receipt==="size-mismatch"?{bytes:99}:store.receipt==="ok-sized"?{bytes:5}:{}}));
    }
    const found=locatePinnedRuntimeV1(pin,host);
    expect(found?.origin??null,row.id).toBe(row.origin);
    if(found!==undefined)expect(found.path).toBe(join(row.origin==="tools-partition"?pin.tools:pin.user!,pin.executable));
  });
});

test("the user cache is the operating system's cache directory unless SEMIO_RUNTIME_CACHE names one",()=>{
  for(const row of fixture.cache)expect(userCacheDirectoryV1(row.platform,row.environment,row.home)??null,row.id).toBe(row.directory);
});

test("every combination of running version, mode, readable command line, install, launcher and depth decides as the table says",()=>{
  expect(fixture.decide.length).toBe(64);
  for(const row of fixture.decide)expect(guardDecisionV1(row),JSON.stringify(row)).toBe(row.decision);
});

test("Windows command lines, macOS process arguments and Linux command lines split into the arguments they were made of",()=>{
  for(const row of fixture.split)expect(splitWindowsCommandLineV1(row.line),row.id).toEqual(row.expected);
  for(const row of fixture.procargs)expect(parseProcessArguments2V1(hex(row.hex)),row.id).toEqual(row.arguments);
  for(const row of fixture.cmdline)expect(parseProcCmdlineV1(hex(row.hex)),row.id).toEqual(row.arguments);
  expect(parseProcessArguments2V1(new Uint8Array(3))).toBeUndefined();expect(parseProcCmdlineV1(new Uint8Array(0))).toBeUndefined();
});

test("an independent Python implementation (standard library only) derives the same expectations from the same inputs",()=>{
  const out=spawnSync("python3",["-I",oracleFile,fixtureFile],{encoding:"utf8"});
  expect(out.status,out.stderr).toBe(0);
  const answer=JSON.parse(out.stdout);
  expect(answer.select).toEqual(fixture.select.map((row:any)=>row.archive));
  expect(answer.locate).toEqual(fixture.locate.map((row:any)=>row.origin));
  expect(answer.cache).toEqual(fixture.cache.map((row:any)=>row.directory));
  expect(answer.decide).toEqual(fixture.decide.map((row:any)=>row.decision));
  expect(answer.split.map((row:any)=>[row.line,row.arguments])).toEqual(fixture.split.map((row:any)=>[row.line,row.expected]));
  expect(answer.procargs).toEqual(fixture.procargs.map((row:any)=>row.hex));
  expect(answer.cmdline).toEqual(fixture.cmdline.map((row:any)=>row.hex));
});

test("the real registry pins the Bun of package.json, and every guard text exists in every language with the same placeholders",()=>{
  const registry=readRuntimePinRegistryV1(root),pinned=/"packageManager"\s*:\s*"bun@(\d+\.\d+\.\d+)"/u.exec(readFileSync(join(root,"package.json"),"utf8"))![1];
  expect(registry.runtimes[registry.selected]!.version).toBe(pinned);
  for(const key of["guardTest","guardMissing","guardLoop"]){
    const texts=registry.messages[key]!,placeholders=(text:string)=>(text.match(/\{[A-Za-z]+\}/gu)??[]).sort().join();
    expect(Object.keys(texts).sort()).toEqual([...registry.languages].sort());
    expect(new Set(registry.languages.map(language=>placeholders(texts[language]!))).size).toBe(1);
  }
});

test("bunfig.toml preloads the guard for scripts and for the test runner",()=>{
  const text=readFileSync(join(root,"bunfig.toml"),"utf8"),parsed=Bun.TOML.parse(text) as any,guard=`./${launcher}/📌️runtime-pin/📜️script.ts`;
  expect(parsed.preload).toEqual([guard]);expect(parsed.test.preload).toEqual([guard,"./🧰️framework/🔨️modules/🏃️process/🧪️testing/🟨️bun/🪦️orphans/📜️script.ts"]);
});

test("the Go launcher and the guard agree on where the provisioned Bun of this host is",()=>{
  const registry=readRuntimePinRegistryV1(root),pin=pinnedRuntimeV1(registry,systemRuntimePinHostV1(root)),found=locatePinnedRuntimeV1(pin,systemRuntimePinHostV1(root));
  const version=spawnSync("repo",["--version","--lang=en"],{encoding:"utf8",cwd:root});
  if(version.status!==0||found===undefined)return;
  expect(version.stdout).toContain(`runtime ${pin.name} ${pin.version}: ready, ${found.path}`);
});

const realRegistry=(directory:string):{environment:Record<string,string>;pinPath:string}=>{
  const registry=readRuntimePinRegistryV1(root),environment={SEMIO_LANGUAGE:"en",PATH:"/usr/bin:/bin",HOME:directory,USERPROFILE:directory,LocalAppData:join(directory,"LocalAppData"),XDG_CACHE_HOME:join(directory,".cache")},host=systemRuntimePinHostV1(directory,environment),pin=pinnedRuntimeV1(registry,host);
  mkdirSync(join(directory,launcher,"🗂️registry"),{recursive:true});writeFileSync(join(directory,launcher,"🗂️registry/🔣️.json"),readFileSync(registryFile));
  return{environment,pinPath:pin.tools};
};
const fake=(directory:string,name:string,code:number):string=>{
  const file=join(directory,name),log=join(directory,name+".log");
  writeFileSync(file,`#!/bin/sh\nprintf '%s\\n' "$@" >"${log}"\nprintf '%s\\n' "guard=$SEMIO_RUNTIME_GUARD" "path=$PATH" >>"${log}"\nexit ${code}\n`);chmodSync(file,0o755);return file;
};
const provision=(directory:string,place:string,executable:string)=>{
  const registry=readRuntimePinRegistryV1(root),pin=pinnedRuntimeV1(registry,systemRuntimePinHostV1(directory,{}));
  mkdirSync(place,{recursive:true});writeFileSync(join(place,".toolchain.json"),JSON.stringify({archive:pin.sha256}));
  writeFileSync(join(place,pin.executable),readFileSync(executable));chmodSync(join(place,pin.executable),0o755);
};
const outcome=async(directory:string,options:Parameters<typeof guardRuntimeV1>[0])=>{
  const written:string[]=[],exits:number[]=[];
  await guardRuntimeV1({root:directory,running:"0.0.1",write:text=>{written.push(text);},exit:code=>{exits.push(code);},...options});
  return{written,exits};
};

test.skipIf(!unix)("a Bun that is not the pinned one re-runs the exact command line under the provisioned pinned Bun and leaves with its exit code",async()=>{
  await scratch(async directory=>{
    const{environment,pinPath}=realRegistry(directory);provision(directory,pinPath,fake(directory,"pinned",7));
    const line=["/usr/local/bin/bun","test","./a b.ts","--timeout","5"],result=await outcome(directory,{environment,mode:"test",readLine:async()=>line,launcher:null});
    expect(result.exits).toEqual([7]);expect(result.written).toEqual([]);
    expect(readFileSync(join(directory,"pinned.log"),"utf8").split("\n")).toEqual(["test","./a b.ts","--timeout","5","guard=1",`path=${pinPath}:/usr/bin:/bin`,""]);
  });
});

test.skipIf(!unix)("a script start re-runs under the provisioned pinned Bun with its own arguments when the command line is unreadable",async()=>{
  await scratch(async directory=>{
    const{environment,pinPath}=realRegistry(directory);provision(directory,pinPath,fake(directory,"pinned",0));
    const result=await outcome(directory,{environment,mode:"run",argv:["bun","/x/script.ts","one"],readLine:async()=>undefined,launcher:null});
    expect(result.exits).toEqual([0]);expect(readFileSync(join(directory,"pinned.log"),"utf8").split("\n").slice(0,2)).toEqual(["/x/script.ts","one"]);
  });
});

test.skipIf(!unix)("without a provisioned Bun the repo launcher gets the script as a script route and the tests through tools exec",async()=>{
  await scratch(async directory=>{
    const{environment}=realRegistry(directory),repo=fake(directory,"repo",3);
    const script=await outcome(directory,{environment,mode:"run",argv:["bun","/x/script.ts","one"],readLine:async()=>["bun","/x/script.ts","one"],launcher:repo});
    expect(script.exits).toEqual([3]);expect(readFileSync(join(directory,"repo.log"),"utf8").split("\n").slice(0,2)).toEqual(["/x/script.ts","one"]);
    const tests=await outcome(directory,{environment,mode:"test",argv:["bun","/x/a.ts"],readLine:async()=>["bun","test","/x/a.ts","-t","name"],launcher:repo});
    expect(tests.exits).toEqual([3]);expect(readFileSync(join(directory,"repo.log"),"utf8").split("\n").slice(0,8)).toEqual(["tools","exec","--light","bun","--","test","/x/a.ts","-t"]);
  });
});

test("the guard refuses in the chosen language, or in every language when none is chosen, and exits with 2",async()=>{
  await scratch(async directory=>{
    const{environment}=realRegistry(directory);
    const unreadable=await outcome(directory,{environment,mode:"test",argv:["bun","/x/a.ts"],readLine:async()=>undefined,launcher:null});
    expect(unreadable.exits).toEqual([2]);expect(unreadable.written.join("\n")).toContain("runs this test");expect(unreadable.written.join("\n")).not.toContain("führt");expect(unreadable.written.join("\n")).toContain("repo tools exec --light bun -- test /x/a.ts");
    const missing=await outcome(directory,{environment:{},mode:"run",argv:["bun","/x/s.ts"],readLine:async()=>["bun","/x/s.ts"],launcher:null,host:{...systemRuntimePinHostV1(directory,{}),installed:()=>false}});
    expect(missing.exits).toEqual([2]);expect(missing.written.join("\n")).toContain("not provisioned here");expect(missing.written.join("\n")).toContain("nicht bereitgestellt");expect(missing.written.join("\n")).toContain(unix?"bash ":"powershell ");
    const looped=await outcome(directory,{environment:{...environment,SEMIO_RUNTIME_GUARD:"1"},mode:"run",argv:["bun","/x/s.ts"],readLine:async()=>["bun","/x/s.ts"],launcher:null});
    expect(looped.exits).toEqual([2]);expect(looped.written.join("\n")).toContain("still runs this");
    expect(guardMessageV1(readRuntimePinRegistryV1(root),"guardLoop",{runtime:"bun",version:"1",running:"2"},{SEMIO_LANGUAGE:"de"})).toContain("noch immer");
  });
});

test("the pinned Bun itself passes the guard without reading a command line",async()=>{
  await scratch(async directory=>{
    const{environment}=realRegistry(directory),version=readRuntimePinRegistryV1(root).runtimes.bun!.version;
    const result=await outcome(directory,{environment,running:version,readLine:async()=>{throw new Error("must not be read");}});
    expect(result.exits).toEqual([]);expect(result.written).toEqual([]);
  });
});

const other=(()=>{const found=Bun.which("bun",{PATH:(process.env.PATH??"").split(delimiter).filter(entry=>entry!==dirname(process.execPath)).join(delimiter)});if(found===null||found===process.execPath)return undefined;const out=spawnSync(found,["--version"],{encoding:"utf8"});return out.status===0&&out.stdout.trim()!==process.versions.bun?found:undefined;})();
test.skipIf(other===undefined||!unix)("a different Bun on the PATH started in the checkout runs the script and the tests as the pinned Bun",()=>{
  const script=spawnSync(other!,["-e","console.log(process.versions.bun)"],{cwd:root,encoding:"utf8"});
  expect(script.stdout.trim()).toBe(process.versions.bun);
  const file=join(tmpdir(),`semio-pin-${process.pid}.test.ts`);
  writeFileSync(file,'import{expect,test}from"bun:test";test("pinned",()=>expect(process.versions.bun).toBe(process.env.EXPECTED));');
  try{
    const run=spawnSync(other!,["test",file,"-t","pinned"],{cwd:root,encoding:"utf8",env:{...process.env,EXPECTED:process.versions.bun}});
    expect(run.status,run.stdout+run.stderr).toBe(0);expect(run.stdout+run.stderr).toContain("1 pass");
  }finally{rmSync(file,{force:true});}
});
