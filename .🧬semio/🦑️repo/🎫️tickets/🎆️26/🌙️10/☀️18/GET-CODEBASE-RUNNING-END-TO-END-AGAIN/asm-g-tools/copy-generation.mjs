// [asm-g] copies the CURRENT sealed generation (pointer, manifest, frontier, fact index, every referenced fragment) into a scratch store. usage: bun copy-generation.mjs <dest>
import{readFileSync,mkdirSync,copyFileSync,existsSync}from"node:fs";import{join}from"node:path";
const SRC=process.env.GEN??"/Users/ueli/Documents/semio/.cache/b/g/m/generations",DEST=process.argv[2];
if(!DEST)throw Error("dest required");
for(const d of["m","f","x","o"])mkdirSync(join(DEST,d),{recursive:true});
const pointer=JSON.parse(readFileSync(join(SRC,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(SRC,"m",pointer.manifest+".json"),"utf8"));
let n=0,bytes=0;const cp=(dir,name)=>{const from=join(SRC,dir,name);if(!existsSync(from))throw Error("missing "+from);copyFileSync(from,join(DEST,dir,name));n++;};
cp("m",pointer.manifest+".json");cp("f",manifest.frontier+".json.br");cp("x",manifest.factIndex+".facts.br");cp("o",manifest.global+".json.br");
for(const row of Object.values(manifest.owners))cp("o",row.fragment+".json.br");
copyFileSync(join(SRC,"current.json"),join(DEST,"current.json"));
console.log(JSON.stringify({generation:pointer.generation,owners:Object.keys(manifest.owners).length,files:n}));
