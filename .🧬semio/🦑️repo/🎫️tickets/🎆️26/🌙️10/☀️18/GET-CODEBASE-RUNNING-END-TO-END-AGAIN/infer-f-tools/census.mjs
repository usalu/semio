import{readFileSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
console.log(Object.keys(idx),idx.owners.length,idx.rows.length,idx.classes.length);
console.log(JSON.stringify(idx.owners.slice(0,3)),JSON.stringify(idx.rows.slice(0,2)),JSON.stringify(idx.classes.slice(0,2)).slice(0,400));
console.log(Object.keys(idx.classes[0]),JSON.stringify(idx.classes[0]).slice(-200));
const per=new Map();
for(const c of idx.classes){
  const owners=c.o??c.owners??[];
  for(const o of owners){const name=typeof o==="number"?idx.owners[o]:o;const e=per.get(name)??{rows:0,bytes:0,kinds:{}};
    for(const ri of c.r){const row=idx.rows[ri];e.rows++;e.bytes+=row.path.length+JSON.stringify(row.fact).length;e.kinds[row.kind]=(e.kinds[row.kind]??0)+1;}
    per.set(name,e);}
}
const top=[...per].sort((a,b)=>b[1].rows-a[1].rows).slice(0,25);
for(const[n,e]of top)console.log(n,e.rows,e.bytes,JSON.stringify(e.kinds));
