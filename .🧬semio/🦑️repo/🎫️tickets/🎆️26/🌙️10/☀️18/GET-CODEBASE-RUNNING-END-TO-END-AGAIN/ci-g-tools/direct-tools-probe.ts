import{scanDirectToolDeclarationsV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧰️direct-tool-declarations/🟦️.ts";
import{readdirSync}from"node:fs";import{join}from"node:path";
const root="/Users/ueli/Documents/semio",walk=(d:string):string[]=>readdirSync(join(root,d),{withFileTypes:true}).flatMap(e=>e.isDirectory()?walk(join(d,e.name)):[join(d,e.name)]);
const files=[...walk(".github"),".devcontainer/devcontainer.json"];
const r=scanDirectToolDeclarationsV1(root,files);
console.log("scanned",r.scanned,"findings",r.findings.length);
for(const f of r.findings)console.log(f.allowed===null?"LIVE":"allowed",f.path,f.line,f.tool,JSON.stringify(f.command).slice(0,120));
