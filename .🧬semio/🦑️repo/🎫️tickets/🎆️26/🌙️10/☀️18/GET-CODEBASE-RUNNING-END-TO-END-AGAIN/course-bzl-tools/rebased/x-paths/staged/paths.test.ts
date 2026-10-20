/** 📏️ Contract: every path the Bazel backend creates for a task stays within the repository's 256 unit rule and the host's own limit, because source labels name a short content-addressed directory alias instead of the repository path; the budget is computed from the real closure per OS and an independent Python implementation, the closed fixture schema, iconv-lite and the real Bazel binary cross-check it. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import iconv from "iconv-lite";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,readlinkSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join,relative} from "node:path";
import component from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema.json";
import {bazelPathBudgetV1,bazelPathClosureV1,bazelPlatformProfileV1,bazelPlatformRulesV1} from "../../🌐️platform/🟦️.ts";
import {bazelExecutionPlatformV1} from "../../🏃️execution/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "../../../../📥️caller/🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const taxonomy=JSON.parse(readFileSync(join(import.meta.dir,"../../../../../🔣️taxonomy.json"),"utf8")) as {collisionPolicy:{maxPathUtf16Units:number;maxComponentUtf8Bytes:number}},api=await import("../../📜️script.ts"),python=Bun.which("python3"),rule=component.definitions.Actions.const.sources,budget=component.definitions.PathBudget.const,label=new RegExp(rule.label);
const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});
const units=(text:string)=>iconv.encode(text,"utf16-le").length/2,bytes=(text:string)=>iconv.encode(text,"utf8").length;

test("the fixture and the authored alias and budget constants agree with their closed schemas and the repository path rule",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(fixtureSchema,{...fixture,extra:1})).toBe(false);expect(ajv.validate(fixtureSchema,{...fixture,alias:{link:"a"}})).toBe(false);
 expect(fixture.alias).toEqual(rule.alias);expect(rule.alias.link).not.toBe(rule.link);
 expect(budget.repositoryUnits).toBe(taxonomy.collisionPolicy.maxPathUtf16Units);expect(taxonomy.collisionPolicy.maxComponentUtf8Bytes).toBe(255);
 const admit=new Ajv({strict:true}).addSchema(component,"component"),actions=admit.compile({$ref:"component#/definitions/ActionPolicy"}),closure=admit.compile({$ref:"component#/definitions/PathClosure"});
 expect(actions(component.definitions.Actions.const),JSON.stringify(actions.errors)).toBe(true);
 expect(actions({...component.definitions.Actions.const,sources:{...rule,alias:{link:"A",hexLength:10}}})).toBe(false);expect(actions({...component.definitions.Actions.const,sources:{...rule,alias:{link:"a",hexLength:3}}})).toBe(false);
 const valid={alias:{units:1,bytes:1},real:{units:0,bytes:0}},measurement=admit.compile({$ref:"component#/definitions/PathMeasurement"});expect(closure(valid),JSON.stringify(closure.errors)).toBe(true);expect(closure({...valid,alias:{units:-1,bytes:1}})).toBe(false);expect(closure({...valid,extra:1})).toBe(false);expect(measurement({version:1,closure:valid}),JSON.stringify(measurement.errors)).toBe(true);expect(measurement({version:2,closure:valid})).toBe(false);expect(measurement({closure:valid})).toBe(false);
});

test.skipIf(!python)("directory aliases are the first hex digits of the SHA-256 of the slash-separated repository directory, like hashlib",()=>{
 const rows=fixture.directories,result=run("import hashlib,json,sys\nlength,directories=json.loads(sys.stdin.read())\nprint(json.dumps([hashlib.sha256(d.encode('utf-8')).hexdigest()[:length] for d in directories]))",[rule.alias.hexLength,rows.map(row=>row.directory)]);
 expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual(rows.map(row=>row.alias));
 for(const row of rows){expect(api.bazelSourceAliasV1(row.directory),row.directory).toBe(row.alias);expect(row.alias).toHaveLength(rule.alias.hexLength);}
 expect(new Set(rows.map(row=>row.alias)).size).toBe(rows.length);
 expect(api.bazelSourceAliasV1("x",12)).toHaveLength(12);expect(()=>api.bazelSourceAliasV1("x",0)).toThrow();expect(()=>api.bazelSourceAliasV1("x",65)).toThrow();
});

test.skipIf(!python)("alias labels are short, checkout-independent, accepted by the Bazel label rule and equal an independent Python rendering",()=>{
 const rows=fixture.labels,result=run("import hashlib,json,sys\nlink,length,paths=json.loads(sys.stdin.read())\nout=[]\nfor p in paths:\n    cut=p.rfind('/')\n    d='' if cut<0 else p[:cut]\n    out.append(link+'/'+hashlib.sha256(d.encode('utf-8')).hexdigest()[:length]+'/'+p[cut+1:])\nprint(json.dumps(out))",[rule.alias.link,rule.alias.hexLength,rows.map(row=>row.path)]);
 expect(result.exitCode,result.stderr.toString()).toBe(0);expect(JSON.parse(result.stdout.toString())).toEqual(rows.map(row=>row.label));
 for(const row of rows){
  expect(api.bazelSourceAliasLabelV1(row.path),row.path).toBe(row.label);expect(label.test(row.label),row.label).toBe(true);expect(row.label.split("/")).toHaveLength(3);
  expect(units(row.label)).toBe(row.label.length);expect(row.label.length).toBe(rule.alias.link.length+1+rule.alias.hexLength+1+row.path.slice(row.path.lastIndexOf("/")+1).length);
  for(const part of row.label.split("/"))expect(bytes(part)).toBeLessThanOrEqual(255);
 }
 expect(api.bazelSourceLabelV1("a/b.ts")).toBe(rule.link+"/a/b.ts");
});

test("the measured closure sizes are the longest alias and legacy labels in UTF-16 units and UTF-8 bytes, like iconv-lite",()=>{
 const files=fixture.closure.files,alias=files.map(path=>api.bazelSourceAliasLabelV1(path)),legacy=files.map(path=>api.bazelSourceLabelV1(path));
 expect(bazelPathClosureV1(alias,[])).toEqual({alias:{units:Math.max(...alias.map(units)),bytes:Math.max(...alias.map(bytes))},real:{units:0,bytes:0}});
 expect(bazelPathClosureV1(alias,[]).alias).toEqual(fixture.closure.alias);expect(bazelPathClosureV1(legacy,[]).alias).toEqual(fixture.closure.legacy);
 expect(bazelPathClosureV1([],legacy).real).toEqual(fixture.closure.legacy);expect(bazelPathClosureV1([],[])).toEqual({alias:{units:0,bytes:0},real:{units:0,bytes:0}});
 expect(fixture.closure.legacy.units).toBeGreaterThan(fixture.closure.alias.units*3);
 for(const row of fixture.budgets)if(row.derive!==null)expect(row.input.closure.alias,row.name).toEqual(fixture.closure[row.derive as "alias"|"legacy"]);
});

test("the path budget of every host matches the fixture: per OS limits, per family totals and the refusal",()=>{
 for(const row of fixture.budgets){
  const actual=bazelPathBudgetV1(row.input as never),expected=row.expect;
  expect(Object.fromEntries(actual.families.map(family=>[family.name,{units:family.units,bytes:family.bytes}])),row.name).toEqual(expected.families as Record<string,{units:number;bytes:number}>);
  expect(actual.limitUnits,row.name).toBe(expected.limitUnits);expect(actual.limitBytes,row.name).toBe(expected.limitBytes);expect(actual.slackUnits,row.name).toBe(expected.slackUnits);expect(actual.slackBytes,row.name).toBe(expected.slackBytes);expect(actual.fits,row.name).toBe(expected.fits);
  expect(actual.outputBase).toEqual({units:units(row.input.outputBase),bytes:bytes(row.input.outputBase)});
  const profile=bazelPlatformProfileV1({...row.input,container:row.input.platform==="linux"&&row.name.startsWith("devcontainer"),bun:row.input.platform==="win32"?"C:\\Users\\dev\\.bun\\bin\\bun.exe":"/usr/local/bin/bun"});
  expect(profile.pathBudget,row.name).toEqual(actual);expect(profile.refusals.filter(text=>text.includes("path budget")).length,row.name).toBe(expected.refusals);
 }
});

test.skipIf(!python)("the path budget agrees with an independent Python evaluation of the authored families",()=>{
 const rows=fixture.budgets,program="import json,sys\nbudget,runfiles,rows=json.loads(sys.stdin.read())\nu16=lambda s: len(s.encode('utf-16-le'))//2\nu8=lambda s: len(s.encode('utf-8'))\nout=[]\nfor r in rows:\n    i=r['input'];p=budget['platforms'][i['platform']];win=i['platform']=='win32'\n    fill={'configuration':p['configuration'][i['arch']],'task':budget['task'],'file':budget['file'],'sandbox':max(p['sandboxes'],key=len) if p['sandboxes'] else '','sandboxId':'0'*budget['sandboxDigits']}\n    fam={}\n    for f in budget['families']:\n        if f.get('needs')=='runfiles' and not runfiles[i['platform']]: continue\n        if f.get('needs')=='sandbox' and not p['sandboxes']: continue\n        v=f.get('view');s=i['closure'][v] if v else {'units':0,'bytes':0}\n        if v and s['units']==0: continue\n        t=f['template'].replace('{source}','')\n        for k,x in fill.items(): t=t.replace('{'+k+'}',x)\n        fam[f['name']]={'units':u16(i['outputBase'])+1+len(t)+s['units'],'bytes':u8(i['outputBase'])+1+len(t)+s['bytes']}\n    osl=(budget['platforms']['win32']['extendedMax']-1 if i['longPaths']=='enabled' else p['pathMax']-1) if win else None\n    lu=min(budget['repositoryUnits'],osl) if osl is not None else budget['repositoryUnits']\n    lb=p['pathMax']-1 if p['unit']=='utf8' else None\n    su=lu-max(x['units'] for x in fam.values());sb=None if lb is None else lb-max(x['bytes'] for x in fam.values())\n    out.append({'families':fam,'limitUnits':lu,'limitBytes':lb,'slackUnits':su,'slackBytes':sb,'fits':su>=0 and (sb is None or sb>=0)})\nprint(json.dumps(out))";
 const runfiles=Object.fromEntries(["darwin","linux","win32"].map(name=>[name,!bazelPlatformRulesV1(name).command.includes("--noenable_runfiles")])),result=run(program,[budget,runfiles,rows]);
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const independent=JSON.parse(result.stdout.toString()) as {families:Record<string,{units:number;bytes:number}>;limitUnits:number;limitBytes:number|null;slackUnits:number;slackBytes:number|null;fits:boolean}[];
 rows.forEach((row,index)=>{const actual=bazelPathBudgetV1(row.input as never);expect(independent[index],row.name).toEqual({families:Object.fromEntries(actual.families.map(family=>[family.name,{units:family.units,bytes:family.bytes}])),limitUnits:actual.limitUnits,limitBytes:actual.limitBytes,slackUnits:actual.slackUnits,slackBytes:actual.slackBytes,fits:actual.fits});});
});

test("the course closure fits every environment with aliases and exceeded the 256 unit rule on macOS, Linux and Windows with repository paths",()=>{
 const pair=(prefix:string)=>{const alias=fixture.budgets.find(row=>row.name===prefix+": alias layout")!,legacy=fixture.budgets.find(row=>row.name===prefix+": legacy repository-path layout")!;return{alias:alias.expect.fits,legacy:legacy.expect.fits};};
 expect(pair("macOS arm64 under /Users/ueli/Documents/semio")).toEqual({alias:true,legacy:false});
 expect(pair("Linux x64 under /home/ueli/Documents/semio")).toEqual({alias:true,legacy:false});
 expect(pair("devcontainer Linux arm64 under /workspaces/semio")).toEqual({alias:true,legacy:false});
 expect(pair("Windows x64 under C:\\Users\\ueli\\Documents\\semio without long paths")).toEqual({alias:true,legacy:false});
 expect(pair("Windows x64 under C:\\Users\\ueli\\Documents\\semio with long paths")).toEqual({alias:true,legacy:false});
 for(const row of fixture.budgets.filter(entry=>entry.derive==="alias"&&!entry.name.includes("semio-workspace")))expect(row.expect.slackUnits,row.name).toBeGreaterThanOrEqual(25);
 const windows=fixture.budgets.filter(row=>row.input.platform==="win32"&&row.derive==="alias");for(const row of windows)expect(row.expect.families).not.toHaveProperty("runfiles-source");
 const refused=bazelPlatformProfileV1({...fixture.budgets.find(row=>row.derive==="legacy"&&row.input.platform==="win32")!.input,container:false,bun:"C:\\bun.exe"});
 expect(refused.refusals).toHaveLength(1);expect(refused.refusals[0]).toContain("259");expect(refused.refusals[0]).toContain("C:\\semio");expect(refused.refusals[0]).toContain("source");
 const bytesOnly=bazelPlatformProfileV1({...fixture.budgets.find(row=>row.name.includes("byte limit"))!.input,container:false,bun:"/usr/local/bin/bun"});
 expect(bytesOnly.refusals).toHaveLength(1);expect(bytesOnly.refusals[0]).toContain("1023 bytes");
});

test("a Windows host never needs machine-wide long paths for the aliased closure",()=>{
 const input=fixture.budgets.find(row=>row.input.platform==="win32"&&row.derive==="alias"&&row.input.longPaths==="disabled")!.input;
 const profile=bazelPlatformProfileV1({...input,container:false,bun:"C:\\Users\\dev\\.bun\\bin\\bun.exe"});
 expect(profile.refusals).toEqual([]);expect(profile.pathBudget.fits).toBe(true);expect(profile.pathBudget.limitUnits).toBe(256);
 expect(JSON.stringify(profile.refusals)+JSON.stringify(profile.warnings)).not.toMatch(/LongPathsEnabled|elevated|machine-wide/u);
});

const prepare=(directory:string,paths:string[])=>{const checkout=join(directory,"checkout");for(const path of paths){mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),path);}return checkout;};

test("the alias farm links each directory once, idempotently, prunes stale aliases and refuses an alias collision",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-alias-")),space=join(directory,"workspace");
 try{
  const paths=fixture.closure.files.filter(path=>!path.includes("ü")||process.platform!=="win32"),checkout=prepare(directory,paths);mkdirSync(space);
  const directories=[...new Set(paths.map(path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):""))].sort(),farm=join(space,rule.alias.link);
  const linked=await api.linkBazelSourceAliasesV1(checkout,space,directories);
  expect(new Set(readdirSync(farm))).toEqual(new Set(directories.map(name=>api.bazelSourceAliasV1(name))));expect(linked.size).toBe(directories.length);
  for(const name of directories){const alias=api.bazelSourceAliasV1(name);expect(linked.get(alias)).toBe(name);expect(realpathSync(join(farm,alias))).toBe(realpathSync(join(checkout,name)));}
  for(const path of paths)expect(readFileSync(join(space,api.bazelSourceAliasLabelV1(path)),"utf8"),path).toBe(path);
  const kind=bazelPlatformRulesV1(process.platform).sourceLink,first=api.bazelSourceAliasV1(directories[0]!),inode=lstatSync(join(farm,first)).ino;
  expect(lstatSync(join(farm,first)).isSymbolicLink()).toBe(true);if(kind==="dir")expect(readlinkSync(join(farm,first))).toBe(join(checkout,directories[0]!));
  await api.linkBazelSourceAliasesV1(checkout,space,directories);expect(lstatSync(join(farm,first)).ino).toBe(inode);
  const keep=directories.slice(0,2);await api.linkBazelSourceAliasesV1(checkout,space,keep);expect(new Set(readdirSync(farm))).toEqual(new Set(keep.map(name=>api.bazelSourceAliasV1(name))));
  writeFileSync(join(farm,"stale.txt"),"x");await api.linkBazelSourceAliasesV1(checkout,space,keep);expect(existsSync(join(farm,"stale.txt"))).toBe(false);
  const other=join(directory,"other");mkdirSync(join(other,keep[0]!),{recursive:true});await api.linkBazelSourceAliasesV1(other,space,keep);expect(realpathSync(join(farm,api.bazelSourceAliasV1(keep[0]!)))).toBe(realpathSync(join(other,keep[0]!)));
  const many=Array.from({length:40},(_value,index)=>"d"+index);for(const name of many)mkdirSync(join(checkout,name));
  await expect(api.linkBazelSourceAliasesV1(checkout,space,many,1)).rejects.toThrow("alias collision");
  await api.linkBazelSourceAliasesV1(checkout,space,[]);expect(readdirSync(farm)).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test.skipIf(!python)("alias source groups render alias labels, keep repository labels for the hermetic view and never share a group name between views",()=>{
 const paths=fixture.labels.map(row=>row.path).sort(),alias=api.bazelSourceCoverV1(paths,"alias"),real=api.bazelSourceCoverV1(paths),all=Array.from({length:paths.length},()=>1);
 const aliasCover=alias.cover(all),realCover=real.cover(all),aliasName=alias.compose(aliasCover,"i")!,realName=real.compose(realCover,"i")!;
 expect(aliasName).not.toBe(realName);expect(aliasName).toMatch(/^(?:a|i|k)_[a-f0-9]{16}$/u);for(const name of alias.groups.keys())expect(real.groups.has(name),name).toBe(false);
 for(const group of alias.groups.values())expect(group.view).toBe("alias");for(const group of real.groups.values())expect(group.view).toBeUndefined();
 const build=api.renderBazelBuildV1({actions:[],groups:new Map([...alias.groups,...real.groups]),bun:"/opt/bun",sourceIdentity:"a".repeat(64),extension:""}).replace(/^load\(.*$/mu,"");
 const parsed=run("import ast,json,sys\nout={}\nfor node in ast.parse(json.loads(sys.stdin.read())).body:\n    call=node.value\n    name=[k.value.value for k in call.keywords if k.arg=='name'][0]\n    out[name]=[e.value for k in call.keywords if k.arg=='srcs' for e in k.value.elts]\nprint(json.dumps(out))",build);
 expect(parsed.exitCode,parsed.stderr.toString()).toBe(0);
 const rendered=JSON.parse(parsed.stdout.toString()) as Record<string,string[]>,expand=(name:string,out:Set<string>):Set<string>=>{for(const entry of rendered[name]!)if(entry.startsWith(":"))expand(entry.slice(1),out);else out.add(entry);return out;};
 expect(expand(aliasName,new Set())).toEqual(new Set(paths.map(path=>api.bazelSourceAliasLabelV1(path))));
 expect(expand(realName,new Set())).toEqual(new Set(paths.map(path=>api.bazelSourceLabelV1(path))));
});

test.skipIf(!python)("the generated workspace names aliased sources for host-state actions and repository paths for hermetic ones, links the farm, records the closure and every label resolves",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bazel-gen-")),checkout=join(directory,"checkout"),workspace=join(directory,"workspace"),host=bazelExecutionPlatformV1(process.platform,process.arch);
 const deep="deep/"+Array.from({length:9},(_value,index)=>"🏗️native-build-"+index+"-unit").join("/"),files:Record<string,string>={
  "lib/src/a.txt":"alpha\n","lib/gen.ts":"export {};\n","app/build.ts":"export {};\n","app/src/c.txt":"gamma\n",["app/"+deep+"/🟦️.ts"]:"export {};\n"
 };
 const target=(command:string,inputs:string[],outputs:string[],dependsOn:string[]=[],metadata:object={})=>({execution:{kind:"command",command},outputs,inputs,cache:true,dependsOn,metadata});
 const graph={version:1 as const,projects:{
  lib:{name:"lib",root:"lib",targets:{generate:target("bun {projectRoot}/gen.ts",["{projectRoot}/src/**/*","{projectRoot}/gen.ts"],["{projectRoot}/gen"],[],{semio:{hermetic:{version:1,tools:["bun"]}}})}},
  app:{name:"app",root:"app",targets:{build:target("bun {projectRoot}/build.ts",["{projectRoot}/**/*"],["{projectRoot}/dist"],["lib:generate"])}}},dependencies:[]} as any;
 try{
  for(const[path,text]of Object.entries(files)){mkdirSync(join(checkout,path,".."),{recursive:true});writeFileSync(join(checkout,path),text);}
  const request={tasks:["app:build"]},planned=api.planBazelTasksV1(graph,request),implementation={version:1 as const,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"0".repeat(64),loaderSha256:"1".repeat(64)};
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:paths-law",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(checkout,["run","app:build"],"bazel",{}),Date.now()),async original=>{
   await api.generateBazelWorkspaceV1(checkout,graph,request,planned,workspace,original,createIssuedResourceControlV1(original,"bazel:paths-law",0),implementation,()=>{},undefined,{host:host.id,tools:{[host.id]:process.execPath}});
  });
  const build=readFileSync(join(workspace,"BUILD.bazel"),"utf8"),labels=[...build.matchAll(/"((?:a|s)\/[^"]+)"/gu)].map(match=>match[1]!),deepFile="app/"+deep+"/🟦️.ts",aliasLabel=api.bazelSourceAliasLabelV1(deepFile);
  expect(labels).toContain(aliasLabel);expect(labels).toContain(api.bazelSourceLabelV1("lib/src/a.txt"));expect(labels).toContain(api.bazelSourceAliasLabelV1("app/src/c.txt"));
  expect(build).not.toContain(api.bazelSourceLabelV1(deepFile));expect(labels.filter(label=>label.startsWith("s/")).every(label=>label.startsWith("s/lib/"))).toBe(true);
  for(const label of labels)expect(existsSync(join(workspace,label)),label).toBe(true);
  const sources=JSON.parse(readFileSync(join(workspace,"sources.json"),"utf8")) as {version:number;link:string;hexLength:number;directories:Record<string,string>};
  expect(sources.link).toBe(rule.alias.link);expect(sources.hexLength).toBe(rule.alias.hexLength);expect(sources.directories[api.bazelSourceAliasV1("app/"+deep)]).toBe("app/"+deep);expect(realpathSync(join(workspace,aliasLabel))).toBe(realpathSync(join(checkout,deepFile)));
  for(const[alias,name]of Object.entries(sources.directories))expect(alias).toBe(api.bazelSourceAliasV1(name));
  const measured=JSON.parse(readFileSync(join(workspace,"paths.json"),"utf8")),admit=new Ajv({strict:true}).addSchema(component,"component").compile({$ref:"component#/definitions/PathMeasurement"});
  expect(admit(measured),JSON.stringify(admit.errors)).toBe(true);
  const aliasLabels=labels.filter(label=>label.startsWith("a/")),repositoryLabels=labels.filter(label=>label.startsWith("s/lib/"));
  expect(measured.closure.alias.units).toBeGreaterThanOrEqual(Math.max(...aliasLabels.map(units)));expect(measured.closure.real).toEqual(bazelPathClosureV1([],repositoryLabels).real);
  expect(measured.closure.alias.units).toBeLessThan(units(api.bazelSourceLabelV1(deepFile)));
  const fit=bazelPathBudgetV1({platform:process.platform as "darwin",arch:process.arch as "arm64",outputBase:join(directory,"base"),longPaths:"unprobed",closure:measured.closure});expect(fit.families.length).toBeGreaterThan(0);
  const outputs=readFileSync(join(workspace,"BUILD.bazel"),"utf8");expect(outputs).toContain("repo_hermetic_task");expect(outputs).toContain("repo_task");
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);

const binary=process.env.SEMIO_TEST_BAZEL_BINARY;

test.skipIf(!binary)("real Bazel: aliased sources of a deep emoji checkout build, hit the disk cache and keep every created path within the computed budget",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"bzlpath-")),base=join(directory,"base"),install=join(directory,"install"),disk=join(directory,"disk"),space=join(directory,"workspace");
 const bazel=(args:string[])=>spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc",`--bazelrc=${join(process.cwd(),".bazelrc")}`,"--max_idle_secs=60",...args],{cwd:space,encoding:"utf8",timeout:240000,env:{...process.env,...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})}});
 try{
  const paths=fixture.closure.files.filter(path=>process.platform!=="win32"||!path.includes("ü")),checkout=prepare(directory,paths);mkdirSync(space);
  const directories=[...new Set(paths.map(path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):""))],sources=paths.map(path=>api.bazelSourceAliasLabelV1(path));
  writeFileSync(join(space,"MODULE.bazel"),'module(name="paths")\n');await api.linkBazelSourceAliasesV1(checkout,space,directories);
  writeFileSync(join(space,"BUILD.bazel"),`genrule(\n    name = "gen",\n    srcs = ${JSON.stringify(sources)},\n    outs = ["out.txt"],\n    cmd = "cat $(SRCS) > $@",\n)\n`);
  const flags=["--lockfile_mode=off",`--disk_cache=${disk}`,"--sandbox_debug"],cold=bazel(["build",...flags,"//:gen"]);expect(cold.status,cold.stderr).toBe(0);
  const bin=bazel(["info","bazel-bin"]).stdout.trim().split("\n").at(-1)!,output=readFileSync(join(bin,"out.txt"),"utf8");for(const path of paths)expect(output).toContain(path);
  expect(bazel(["clean"]).status).toBe(0);const warm=bazel(["build",...flags,"//:gen"]);expect(warm.status,warm.stderr).toBe(0);expect(warm.stderr).toMatch(/[1-9]\d* disk cache hit/u);
  const longest=(root:string):number=>{let best=0;const visit=(path:string)=>{for(const entry of readdirSync(path,{withFileTypes:true})){const full=join(path,entry.name);best=Math.max(best,units(relative(base,full)));if(entry.isDirectory()&&!entry.isSymbolicLink())visit(full);}};visit(root);return best;};
  const closure=bazelPathClosureV1(sources,sources),computed=bazelPathBudgetV1({platform:process.platform as "darwin",arch:process.arch as "arm64",outputBase:base,longPaths:"unprobed",closure}),sandboxed=computed.families.find(family=>family.name==="sandbox-source");
  const created=longest(base)+1+units(base);expect(created).toBeLessThanOrEqual(Math.max(...computed.families.map(family=>family.units)));if(sandboxed&&existsSync(join(base,"sandbox")))expect(created).toBeLessThanOrEqual(sandboxed.units);
 }finally{spawnSync(binary!,[`--output_base=${base}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:space,timeout:120000});rmSync(directory,{recursive:true,force:true});}
},900000);
