import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs"),discovery=await import(L+"🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs"),observation=await import(L+"👁️observation/🟨️.mjs"),reads=await import(L+"🕸️graph/🧾️registry-reads/🟨️.mjs");
test("recorded wasm-index digest equals the resolver digest on the real registry",async()=>{
 expect(inference).toBeDefined();
 const registry=JSON.parse(readFileSync(root+"/📋️build-owners.json","utf8"));
 const t0=performance.now();
 const captured=await observation.captureObservedSourceInputsV1(async()=>discovery.readBuildProducedCompileOutputV1(root+"/zz/none.js",root,{},registry,{}));
 const t1=performance.now();
 const rows=(captured.observations as any[]).filter(r=>r.kind==="global"&&JSON.parse(r.path)[2][0]==="wasm-index");
 const resolver=reads.answerOwnerRegistryQuestionV1(registry,["wasm-index"],root),t2=performance.now();
 console.log(JSON.stringify({rows:rows.length,recorded:rows[0]?.fact,resolver,observedMs:Math.round(t1-t0),plainMs:Math.round(t2-t1)}));
 expect(rows.length).toBeGreaterThan(0);expect(JSON.parse(rows[0].fact)).toBe(resolver);
});
