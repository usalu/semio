const memo=JSON.parse(await Bun.file(process.argv[2]).text());
const prefix="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const files=["🕸️graph/📥️load/🟦️.ts","👁️observation/🟨️.mjs","🕸️graph/📥️load/🧭️registry-follow/🟦️.ts","🔣️taxonomy.json"];
const byKey=new Map<string,Set<number>>();
for(const f of files)byKey.set(f,new Set());
memo.owners.forEach((o:any,i:number)=>{for(const at of o.rows){const r=memo.observations[at];for(const f of files)if(r.path===prefix+f||r.path.endsWith("/"+f))byKey.get(f)!.add(i);}});
const buildRows=memo.owners.map((o:any)=>o.rows.filter((at:number)=>memo.observations[at].path.startsWith(prefix)).length);
console.log(JSON.stringify({owners:memo.owners.length,readers:Object.fromEntries([...byKey].map(([k,v])=>[k,v.size])),ownersReadingBuildTree:buildRows.filter((n:number)=>n>0).length,medianBuildRows:buildRows.sort((a:number,b:number)=>a-b)[buildRows.length>>1]}));
const kinds:Record<string,number>={};memo.owners.forEach((o:any)=>{for(const at of o.rows){const r=memo.observations[at];if(r.path.endsWith("/🕸️graph/📥️load/🟦️.ts"))kinds[r.kind]=(kinds[r.kind]??0)+1;}});console.log(kinds);
