import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
const ticket=dirname(import.meta.dir),root=resolve(ticket,"../../../../../../.."),[command,epoch="1"]=process.argv.slice(2),sha=(s:string)=>createHash("sha256").update(s).digest("hex"),engine="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine",path=engine+"/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs",out=join(ticket,"🗑️generated/browser-appearance-source-law");
mkdirSync(out,{recursive:true});assert.equal(command,"stage");const destination=join(out,"source-"+epoch+".json");assert.equal(existsSync(destination),false);const before=readFileSync(join(root,path),"utf8"),lines=before.split("\n"),start=lines.findIndex(line=>line.includes('for needle in ["resolveWgpuHostAppearance(window)"')),end=start+3;
assert.ok(start>=0);assert.ok(lines[start+1]!.includes("assert!(page.contains(needle)"));assert.equal(lines[start+2],"    }");
const to=[
'    assert!(page.contains("from \\"../🌐️browser-host/🟦️.ts\\""), "the page imports the canonical browser host");',
'    assert!(page.contains("await mountWgpuBrowserHost(root,"), "the page mounts the canonical browser host");',
'    assert!(page.contains("pageBindings: true"), "the page enables live browser bindings");',
'    let host = std::fs::read_to_string(engine_root().join("🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts")).expect("browser host source");',
lines[start]!,
'        assert!(host.contains(needle), "the browser host makes the reads and keeps them live: {needle}");',
'    }'
];const after=[...lines.slice(0,start),...to,...lines.slice(end)].join("\n"),contextPaths=["🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs","🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts","🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts","🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts","🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"],contexts=contextPaths.map(suffix=>{const path=engine+"/"+suffix,source=readFileSync(join(root,path),"utf8");return{path,source,sha256:sha(source)};});
for(const row of contexts)assert.equal(readFileSync(join(root,row.path),"utf8"),row.source);assert.equal(readFileSync(join(root,path),"utf8"),before);writeFileSync(destination,JSON.stringify({producer:{path:import.meta.path,source:readFileSync(import.meta.path,"utf8")},pairs:[{path,before,after,beforeHash:sha(before),afterHash:sha(after)}],contexts,sourceWrites:0,nativeExecuted:false,wholeRootAccepted:false,scope:"Original failing browser source-chain law follows the actual canonical browser-host module; all original hook/worker/preferences/live-listener assertions retained plus three explicit boot-to-host link assertions. No runtime alias or browser behavior changed."}));console.log("[DEBUG] Browser appearance source law staged one exact owning cut");
