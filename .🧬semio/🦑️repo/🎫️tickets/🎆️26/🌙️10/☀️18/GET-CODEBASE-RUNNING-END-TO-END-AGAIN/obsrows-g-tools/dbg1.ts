import {mkdirSync,writeFileSync,mkdtempSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";import {realpathSync} from "node:fs";
const B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/";
const api=await import(B+"🟨️.mjs");
const fixture=JSON.parse(readFileSync(B+"🧪️tests/🧫️fixtures/🔣️.json","utf8"));
const root=mkdtempSync(join(realpathSync(tmpdir()),"dbg-"));
for(const [p,c] of Object.entries(fixture.files)){mkdirSync(join(root,p,".."),{recursive:true});writeFileSync(join(root,p),c as string);}
Bun.sleepSync(2200);
const probe=()=>({one:api.observedFs.readFileSync(join(root,"a/one.txt"),"utf8"),missing:api.observedFs.existsSync(join(root,"c/missing.txt")),listing:api.observedFs.readdirSync(join(root,"a")).sort(),bIsDirectory:api.observedFs.statSync(join(root,"b")).isDirectory()});
const cachePath=join(root,".cache","inference.json");
await api.observeInference({cachePath,key:["hold","x"]},()=>probe());
const header=JSON.parse(api.inferenceHeaderLine(cachePath));
console.log(Object.keys(header),JSON.stringify(header).slice(0,1500));
console.log(process.memoryUsage().rss/1e6,"MB");
