import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const dir=process.argv[2],needle=process.argv[3];
const c=JSON.parse(readFileSync(dir+"/current.json","utf8")),m=JSON.parse(readFileSync(dir+"/m/"+c.manifest+".json","utf8"));
const idx=G.unpackFactIndexV1(readFileSync(dir+"/x/"+m.factIndex+".facts.br"),1<<29);
const holders=new Map();for(const cl of idx.classes)for(const r of cl.r)holders.set(r,(holders.get(r)??0)+cl.o.length);
const hits=idx.rows.map((r,i)=>[r,i]).filter(([r])=>r.path.includes(needle));
console.log("generation",c.generation,"rows matching",hits.length);
for(const[r,i]of hits.slice(0,8)){const owners=[];for(const cl of idx.classes)if(cl.r.includes(i))for(const o of cl.o)owners.push(idx.owners[o]);console.log(r.kind,holders.get(i),owners.slice(0,3).join(","));}
const tagged=Object.entries(m.owners).filter(([,r])=>r.tag).map(([n])=>idx.owners.indexOf(n));
const sizes=tagged.slice(0,3).map(id=>{let n=0;for(const cl of idx.classes)if(cl.o.includes(id))n+=cl.r.length;return n;});
console.log("rows of the first tagged owners",sizes);
