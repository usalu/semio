/** 🧾️ Checks exact browser source authority against the real compiler traversal. */
import assert from "node:assert/strict";
import { readFileSync, realpathSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const taxonomy = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const profile = taxonomy.generatorContracts["wgpu-frame-worker"].packageGeneration.browserProfile;
const cases = JSON.parse(readFileSync(join(ticket, "browser-authority-cases.json"), "utf8"));
for (const [specifier, binding] of Object.entries(profile.workspaceImports)) {
  assert.ok(profile.sourceModulePaths.includes(binding.entryPath), "bound entry is schema-owned " + specifier);
  const manifest = JSON.parse(readFileSync(join(root,binding.manifestPath),"utf8"));
  const packageName = specifier.split("/").slice(0,2).join("/");
  assert.equal(manifest.name,packageName,"binding manifest identity " + specifier);
  assert.equal(resolve(dirname(join(root,binding.manifestPath)),manifest.exports["."+specifier.slice(packageName.length)]),join(root,binding.entryPath),"binding export identity " + specifier);
}
const { parseSemanticPackageBrowserProfile } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🎁️package-projection/🟦️.ts"));
parseSemanticPackageBrowserProfile(profile,taxonomy.pathEmojiPolicy.genericEmojiIdentities);
const {default:Ajv}=await import("ajv");
const specifierSchema=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🎁️package-projection/🧬️schema/📦️workspace-import/🔣️.json"),"utf8"));
const validateSpecifier=new Ajv().compile(specifierSchema);
for (const item of cases.specifierCases) {
  assert.equal(validateSpecifier(item.specifier),item.accepted,"independent Ajv identity "+item.name);
  const candidate=structuredClone(profile); candidate.workspaceImports[item.specifier]=candidate.workspaceImports["@semio-tech/framework"];
  candidate.workspaceImports=Object.fromEntries(Object.entries(candidate.workspaceImports).sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b))));
  const check=()=>parseSemanticPackageBrowserProfile(candidate,taxonomy.pathEmojiPolicy.genericEmojiIdentities);
  if(item.accepted) assert.doesNotThrow(check,item.name); else assert.throws(check,undefined,item.name);
}
const modules = new Set(), bindings = {}, compare = (a,b) => Buffer.compare(Buffer.from(a),Buffer.from(b));
const workspace = absolute => { const path = relative(root, absolute).replaceAll("\\", "/"); assert.ok(!path.startsWith("../") && !path.includes("/node_modules/"), "owned workspace source " + path); return path; };
for (const entry of profile.entries) {
  assert.ok(cases.entries.includes(entry.id));
  const output = await Bun.build({entrypoints:[join(root,profile.ownerPath,entry.sourceRelativePath)],target:"browser",format:"esm",define:{"import.meta.vitest":"undefined"},plugins:[{name:"authority-independent-traversal",setup(builder){
    builder.onResolve({filter:/.*/}, request => {
      let absolute;
      if (isAbsolute(request.path)) absolute=request.path;
      else if(request.path.startsWith(".")) absolute=resolve(dirname(request.importer),request.path);
      else {
        const packageName=request.path.split("/").slice(0,request.path.startsWith("@")?2:1).join("/");
        const manifestAbsolute=realpathSync(join(root,"node_modules",packageName,"package.json")), manifest=JSON.parse(readFileSync(manifestAbsolute,"utf8"));
        const subpath="."+request.path.slice(packageName.length), target=manifest.exports?.[subpath];
        assert.equal(typeof target,"string","explicit export " + request.path);
        absolute=resolve(dirname(manifestAbsolute),target);
        bindings[request.path]={manifestPath:workspace(manifestAbsolute),entryPath:workspace(absolute)};
      }
      return {path:absolute,namespace:"authority-input"};
    });
    builder.onLoad({filter:/.*/,namespace:"authority-input"}, request => { modules.add(workspace(request.path)); return {contents:readFileSync(request.path,"utf8"),loader:request.path.endsWith(".json")?"json":request.path.endsWith(".tsx")?"tsx":"ts"}; });
  }}]});
  assert.equal(output.success,true,String(output.logs));
  assert.equal(output.outputs.length,1);
}
const observed={sourceModulePaths:[...modules].sort(compare),workspaceImports:Object.fromEntries(Object.entries(bindings).sort(([a],[b])=>compare(a,b)))};
writeFileSync(join(ticket,"🗑️generated/fleet-host-2026-11-04/authority-observed.json"),JSON.stringify(observed,null,2)+"\n");
for(const specifier of cases.requiredWorkspaceImports) assert.ok(observed.workspaceImports[specifier],"runtime traversal includes " + specifier);
for(const item of cases.rejectionCases) assert.equal(Boolean(profile.workspaceImports[item.specifier] || profile.sourceModulePaths.includes(item.specifier)),item.accepted,item.name);
const missing=observed.sourceModulePaths.filter(path=>!profile.sourceModulePaths.includes(path)),unused=profile.sourceModulePaths.filter(path=>!modules.has(path));
const changed=Object.keys(observed.workspaceImports).filter(specifier=>JSON.stringify(observed.workspaceImports[specifier])!==JSON.stringify(profile.workspaceImports[specifier]));
assert.deepEqual({missing,unused,changed},{missing:[],unused:[],changed:[]},"exact browser authority must equal actual Bun compiler traversal");
console.log("[DEBUG] Exact browser authority matches compiler traversal: "+modules.size+" source inputs, "+Object.keys(bindings).length+" workspace bindings");
