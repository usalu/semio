import{readFileSync}from"node:fs";
import{join}from"node:path";
import{encodeGenerationPinV1}from"../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧷️pin/🟨️.mjs";
const root=process.argv[2]!,owner=Number(process.argv[3]),store=join(root,".cache/b/g/m/generations"),pointer=JSON.parse(readFileSync(join(store,"current.json"),"utf8"));
const manifest=JSON.parse(readFileSync(join(store,"m",pointer.manifest+".json"),"utf8")),owners=Object.keys(manifest.owners).sort((a:string,b:string)=>a<b?-1:a>b?1:0);
process.stdout.write(encodeGenerationPinV1({owner,generation:pointer.generation,manifest:pointer.manifest,owners,covered:process.argv[4]?new Set(process.argv[4].split(",")):undefined,atMs:Date.now()}));
