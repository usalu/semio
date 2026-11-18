import{plugin}from"bun";
import{existsSync,readFileSync}from"node:fs";
import{resolve,dirname}from"node:path";
const map:Record<string,string>=JSON.parse(readFileSync(process.env.R183_MAP!,"utf8")),back=new Map(Object.entries(map).map(([live,staged])=>[staged,live]));
plugin({name:"r183",setup(b){b.onResolve({filter:/.*/},a=>{if(a.path.startsWith("/")&&map[a.path])return{path:map[a.path]};if(!a.importer||!a.path.startsWith("."))return;const imp=back.get(a.importer.replace(/^file:\/\//,""))??a.importer.replace(/^file:\/\//,""),abs=resolve(dirname(imp),a.path),m=map[abs];if(m)return{path:m};if(back.has(a.importer.replace(/^file:\/\//,""))&&existsSync(abs))return{path:abs};});}});
