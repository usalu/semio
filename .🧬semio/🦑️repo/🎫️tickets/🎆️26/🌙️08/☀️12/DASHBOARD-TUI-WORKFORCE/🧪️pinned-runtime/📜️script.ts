import assert from "node:assert/strict";
import {readFileSync,writeFileSync,renameSync} from "node:fs";
import {join,resolve} from "node:path";
const root=process.cwd(),generated=resolve(import.meta.dir,"../🗑️generated"),boot="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap",mode=process.argv[2];
function edit(path:string,transform:(text:string)=>string):void{const file=join(root,path),before=readFileSync(file,"utf8"),after=transform(before);assert.notEqual(after,before);const staged=join(generated,`runtime-stage-${Date.now()}`);writeFileSync(staged,after);assert.equal(readFileSync(file,"utf8"),before);renameSync(staged,file);}
if(mode==="test")edit(`${boot}/📦️dependencies/🧪️tests/🟦️.ts`,text=>text+readFileSync(join(import.meta.dir,"📓️test.ts.md"),"utf8"));
if(mode==="fix"||mode==="fix-tools"){
  if(mode==="fix")
  edit(`${boot}/📜️script.ts`,text=>{
    text=text.replace('import { join, resolve } from "node:path";','import { join, resolve, win32, posix } from "node:path";');
    const anchor='/** 🧭️ Loads domain selection helpers';assert.ok(text.includes(anchor));
    text=text.replace(anchor,readFileSync(join(import.meta.dir,"📓️environment.ts.md"),"utf8")+'\n'+anchor);
    text=text.replace('let tooling: { cli: string; modulePath: string };','let tooling: { cli: string; modulePath: string; bun: string };');
    const before='const env = devToolingEnv({ ...invocation.env,';assert.ok(text.includes(before));text=text.replace(before,'const env = pinnedNxRuntimeEnvironment(devToolingEnv({ ...invocation.env,');
    return text.replace('npm_lifecycle_script: undefined });','npm_lifecycle_script: undefined }), tooling.bun);');
  });
  edit(`${boot}/🛠️tools/📜️script.ts`,text=>{
    text=text.replace('readonly modulePath: string };','readonly modulePath: string; readonly bun: string };');
    const before='  const patches = manifest.semio?.toolPatches;';assert.ok(text.includes(before));text=text.replace(before,'  const bun = await prepareBun(root,manifest.packageManager.slice(4),signal);\n'+before);
    text=text.replace('return { cli: require.resolve("nx/bin/nx.js"), modulePath };','return { cli: require.resolve("nx/bin/nx.js"), modulePath, bun };');
    const duplicate=/    const installer = await prepareBun\([^\n]+\);\r?\n/;assert.ok(duplicate.test(text));return text.replace(duplicate,'').replace('await runTool(installer,','await runTool(bun,');
  });
}
