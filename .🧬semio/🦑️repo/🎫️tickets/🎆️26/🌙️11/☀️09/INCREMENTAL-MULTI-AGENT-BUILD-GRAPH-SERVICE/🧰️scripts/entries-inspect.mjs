import{brotliDecompressSync}from"node:zlib";import{readdirSync,readFileSync,statSync}from"node:fs";import{join}from"node:path";
const dir=process.argv[2];
for(const name of readdirSync(join(dir,"e"))){if(!name.endsWith(".br"))continue;const text=brotliDecompressSync(readFileSync(join(dir,"e",name))).toString(),r=JSON.parse(text);
 const kinds={};for(const row of r.rows)kinds[row.kind]=(kinds[row.kind]??0)+1;
 console.log(r.recipe.fn,(r.recipe.projectRoot??r.recipe.name??r.recipe.port??""),"value",JSON.stringify(r.value).length,"rows",r.rows.length,"layers",r.layers.length,JSON.stringify(kinds).slice(0,150),statSync(join(dir,"e",name)).mtime.toISOString().slice(11,19));}
