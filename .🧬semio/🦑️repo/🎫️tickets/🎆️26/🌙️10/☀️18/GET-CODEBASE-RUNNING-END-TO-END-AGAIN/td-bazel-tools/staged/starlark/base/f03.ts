/** 🌐️ Contract: every platform-dependent Bazel decision is a pure function of host facts, so one fixture proves all six OS and CPU pairs without running them; the real Bazel binary (SEMIO_TEST_BAZEL_BINARY) cross-checks the flag names, the Starlark escaping of Windows paths and a cached build with the rendered flags. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import component from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema.json";
import {bazelPlatformProfileV1,bazelPlatformRulesV1} from "../../🌐️platform/🟦️.ts";

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,adapter=readFileSync(join(import.meta.dir,"../../📜️script.ts"),"utf8");
const starlark=/const STARLARK_RULES=(?:bazelResourceSetsV1\([^)]*\)\+)?`([\s\S]*?)`;/u.exec(adapter)![1]!;

test("the fixture and every rendered profile validate against their closed schemas",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 const validate=new Ajv({strict:true}).compile({...component.definitions.PlatformProfile,definitions:component.definitions});
 for(const row of fixture.profiles)expect(validate(bazelPlatformProfileV1(row.input)),`${row.name} ${JSON.stringify(validate.errors)}`).toBe(true);
});

test("six OS and CPU pairs render the authored flags, environment, launcher extension and source link",()=>{
 const pairs=new Set(fixture.profiles.map(row=>row.input.platform+"/"+row.input.arch));
 for(const pair of ["darwin/arm64","darwin/x64","linux/arm64","linux/x64","win32/arm64","win32/x64"])expect(pairs.has(pair),pair).toBe(true);
 for(const row of fixture.profiles){
  const profile=bazelPlatformProfileV1(row.input),expected=row.expect;
  expect([...profile.startup],row.name).toEqual(expected.startup as string[]);expect([...profile.command],row.name).toEqual(expected.command as string[]);expect({...profile.environment},row.name).toEqual(expected.environment as Record<string,string>);
  expect(profile.launcherExtension,row.name).toBe(expected.launcherExtension);expect(profile.sourceLink,row.name).toBe(expected.sourceLink as typeof profile.sourceLink);expect(profile.emojiLabels,row.name).toBe(expected.emojiLabels as typeof profile.emojiLabels);
  expect(profile.pathBudget.fits,row.name).toBe(expected.fits);expect(profile.refusals.length,`${row.name} ${profile.refusals.join("|")}`).toBe(expected.refusals);expect(profile.warnings.length,`${row.name} ${profile.warnings.join("|")}`).toBe(expected.warnings);
  expect(profile.shell).toBe("none");
 }
 for(const name of fixture.unsupported)expect(()=>bazelPlatformRulesV1(name),name).toThrow("Unsupported Bazel platform");
 for(const row of fixture.invalid)expect(()=>bazelPlatformProfileV1(row.input),row.name).toThrow("Closed Bazel platform input required");
});


test("every Bazel action runs Bun directly: no shell, no genrule, JSON-quoted Windows paths, platform facts from one place",()=>{
 for(const token of fixture.forbiddenShell)expect(starlark.toLowerCase(),token).not.toContain(token.toLowerCase());
 const runs=starlark.split("ctx.actions.run(").length-1;expect(runs).toBeGreaterThan(0);expect(starlark.split("executable = ctx.attr.runtime").length-1).toBe(runs);
 expect(adapter).toContain("quote=(value:string)=>JSON.stringify(value)");
 expect(adapter).toContain("bazelPlatformRulesV1(process.platform).launcherExtension");
 expect(adapter).not.toContain('process.platform==="win32"?".exe":""');expect(adapter).not.toContain('process.platform==="win32"?"junction":"dir"');expect(adapter).not.toContain('process.platform==="darwin"?{LANG');
 expect(adapter).not.toContain("--output_user_root");
 for(const bun of Object.values(fixture.buildRender.bun))expect(JSON.parse(JSON.stringify(bun))).toBe(bun);
});

function run(args:string[],cwd:string,env:Record<string,string>={}){const result=spawnSync(binary!,args,{cwd,encoding:"utf8",env:{PATH:process.env.PATH,HOME:process.env.HOME,...env} as NodeJS.ProcessEnv,timeout:280000});return{code:result.status,out:(result.stdout??"")+(result.stderr??""),stdout:result.stdout??""};}
const unlock=(path:string):void=>{try{chmodSync(path,0o700);for(const entry of readdirSync(path,{withFileTypes:true}))if(entry.isDirectory())unlock(join(path,entry.name));}catch{}};
const names=(text:string)=>new Set([...text.matchAll(/--\[no\](\w+)|--(\w+)/gu)].map(match=>match[1]??match[2]!));
const known=(set:Set<string>,flag:string)=>{const match=/^--(no)?([a-z_]+)/u.exec(flag)!;return set.has(match[2]!)||(match[1]!==undefined&&set.has("no"+match[2]!));};

test.skipIf(!binary)("the real Bazel binary knows every rendered flag, parses Windows Starlark paths and serves emoji sources from its disk cache",()=>{
 const base=mkdtempSync(join(tmpdir(),"bzlplat-")),install=join(base,"install"),common=(name:string)=>["--batch",`--output_base=${join(base,name)}`,`--install_base=${install}`,"--nosystem_rc","--nohome_rc"],environment=bazelPlatformRulesV1(process.platform).environment;
 try{
  const startupHelp=run([...common("h1"),...fixture.flagOracle.startupHelp],base,environment),commandHelp=run([...common("h2"),...fixture.flagOracle.commandHelp],base,environment);
  expect(startupHelp.code,startupHelp.out.slice(-400)).toBe(0);expect(commandHelp.code,commandHelp.out.slice(-400)).toBe(0);
  const startupNames=names(startupHelp.out),commandNames=names(commandHelp.out);
  for(const platform of ["darwin","linux","win32"])for(const flag of bazelPlatformRulesV1(platform).startup)expect(known(startupNames,flag),`${platform} startup ${flag}`).toBe(true);
  for(const platform of ["darwin","linux","win32"])for(const flag of bazelPlatformRulesV1(platform).command)expect(known(commandNames,flag),`${platform} command ${flag}`).toBe(true);
  const windowsBun=fixture.buildRender.bun.win32,query=join(base,"q");mkdirSync(query,{recursive:true});
  writeFileSync(join(query,"MODULE.bazel"),'module(name = "xplat_query", version = "0.0.0")\n');writeFileSync(join(query,"tasks.bzl"),starlark);
  writeFileSync(join(query,"BUILD.bazel"),`load(":tasks.bzl", "repo_launch", "repo_task")\nrepo_task(name = "task", runtime = ${JSON.stringify(windowsBun)}, identity = "p:t", source_identity = "x")\nrepo_launch(name = "launch", runtime = ${JSON.stringify(windowsBun)}, source_identity = "x", extension = ".exe")\n`);
  const listed=run([...common("q1"),"query","//:all","--output=build","--lockfile_mode=off"],query,environment);
  expect(listed.code,listed.out.slice(-600)).toBe(0);expect(listed.out).toContain(JSON.stringify(windowsBun));expect(listed.out).toContain('extension = ".exe"');
  const profile=bazelPlatformProfileV1({platform:process.platform,arch:process.arch,container:false,outputBase:join(base,"b"),bun:process.execPath,longPaths:process.platform==="win32"?"enabled":"unprobed",closure:{alias:{units:0,bytes:0},real:{units:0,bytes:0}}});
  expect(profile.refusals).toEqual([]);
  const root=join(base,"w");mkdirSync(join(root,"src/🧰️framework"),{recursive:true});
  writeFileSync(join(root,"MODULE.bazel"),'module(name = "xplat_build", version = "0.0.0")\n');writeFileSync(join(root,"tasks.bzl"),starlark);
  writeFileSync(join(root,"src/🧰️framework/🟦️.txt"),"emoji input\n");writeFileSync(join(root,"manifest.json"),"{}\n");writeFileSync(join(root,"fingerprint.txt"),"f\n");writeFileSync(join(root,"implementation.json"),"{}\n");
  writeFileSync(join(root,"runner.ts"),'import {copyFileSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";const [,,mode,manifest,out]=process.argv.slice(2);if(mode==="birth-launcher")copyFileSync(process.execPath,out);else{mkdirSync(out+"/🧪️d",{recursive:true});writeFileSync(out+"/🧪️d/📜️result.txt",readFileSync(manifest,"utf8")+"emoji");}\n');
  symlinkSync(join(root,"src"),join(root,"s"),profile.sourceLink);
  const bun=JSON.stringify(process.execPath);
  writeFileSync(join(root,"BUILD.bazel"),`load(":tasks.bzl", "repo_launch", "repo_task")\nrepo_task(name = "task", srcs = ["s/🧰️framework/🟦️.txt"], manifest = "manifest.json", fingerprint = "fingerprint.txt", runner = "runner.ts", source = "implementation.json", source_identity = "x", runtime = ${bun}, identity = "p:t", cache = True, host_state = "task")\nrepo_launch(name = "launch", manifest = "manifest.json", runner = "runner.ts", source = "implementation.json", source_identity = "x", runtime = ${bun}, extension = ${JSON.stringify(profile.launcherExtension)}, uncached_reason = "launcher", host_state = "launcher")\n`);
  const startup=[...common("b"),...profile.startup],options=["--lockfile_mode=off","--incompatible_strict_action_env",`--disk_cache=${join(base,"cache")}`,...profile.command],build=()=>run([...startup,"build","//:task","//:launch",...options],root,profile.environment);
  const cold=build();expect(cold.code,cold.out.slice(-800)).toBe(0);expect(cold.out).toMatch(/\d+ processes?: .*\b(?:local|processwrapper-sandbox|linux-sandbox|darwin-sandbox|windows-sandbox)\b/u);
  const info=run([...startup,"info","bazel-bin"],root,profile.environment).stdout.trim().split("\n").at(-1)!;
  expect(readFileSync(join(info,"task.snapshot/🧪️d/📜️result.txt"),"utf8")).toContain("emoji");
  expect(run([...startup,"clean",...options.slice(0,2)],root,profile.environment).code).toBe(0);
  const warm=build();expect(warm.code,warm.out.slice(-800)).toBe(0);expect(warm.out).toContain("disk cache hit");
 }finally{unlock(base);rmSync(base,{recursive:true,force:true});}
},900000);
