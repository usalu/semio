import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const np=process.argv[2];
const keep=i=>typeof i==="string"&&!i.startsWith("!")&&!/[*?{}[\]]/u.test(i.replace(/^\{(?:projectRoot|workspaceRoot)\}/u,""))&&/\.[cm]?[jt]sx?$/u.test(i)&&/^\{(?:projectRoot|workspaceRoot)\}\//u.test(i);
for(const f of process.argv.slice(3)){const text=brotliDecompressSync(readFileSync(join(np,f))).toString();const o=JSON.parse(text.slice(text.indexOf("\n")+1));
 const before=JSON.stringify(o.closureRecipes).length;let parts={options:0,configurations:0,inputs:0,metadata:0};
 for(const r of Object.values(o.closureRecipes)){const t=r.target;if(!t)continue;for(const k of Object.keys(parts))parts[k]+=JSON.stringify(t[k]??null).length;if(t.inputs)t.inputs=t.inputs.filter(keep);}
 console.log(f,"value",text.length,"recipes before",before,"after",JSON.stringify(o.closureRecipes).length,JSON.stringify(parts));}
