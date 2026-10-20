import {dirname} from "node:path";
const rows=JSON.parse(await Bun.file(process.argv[2]).text());
const out:any[]=[];
for(const r of rows){
  const m=/Cannot find module '([^']+)' from '([^']+)'/.exec(r.firstError);
  if(!m) continue;
  try{Bun.resolveSync(m[1],dirname(m[2]));out.push({file:r.file,spec:m[1],ok:true});}
  catch(e:any){out.push({file:r.file,spec:m[1],ok:false,err:String(e.message).slice(0,200)});}
}
console.log(JSON.stringify(out));
