import{readFileSync,mkdirSync,copyFileSync,existsSync,writeFileSync}from"node:fs";import{join}from"node:path";
const SRC="/Users/ueli/Documents/semio/.cache/b/g/m/generations",DEST=process.argv[2],MAN=process.argv[3],GEN=Number(process.argv[4]);
for(const d of["m","f","x","o"])mkdirSync(join(DEST,d),{recursive:true});
const manifest=JSON.parse(readFileSync(join(SRC,"m",MAN+".json"),"utf8"));
const cp=(dir,name)=>{const from=join(SRC,dir,name);if(!existsSync(from))throw Error("missing "+from);copyFileSync(from,join(DEST,dir,name));};
cp("m",MAN+".json");cp("f",manifest.frontier+".json.br");cp("x",manifest.factIndex+".facts.br");cp("o",manifest.global+".json.br");
for(const row of Object.values(manifest.owners))cp("o",row.fragment+".json.br");
writeFileSync(join(DEST,"current.json"),JSON.stringify({generation:GEN,manifest:MAN,parent:manifest.parent??null,version:1}));
console.log(JSON.stringify({gen:GEN,owners:Object.keys(manifest.owners).length}));
