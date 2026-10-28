import{readdirSync,statSync}from"node:fs";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");
const D="/Users/ueli/Documents/semio/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const names=readdirSync(D).filter(n=>/^[0-9a-f]{24}\.json$/.test(n));
const sample=names.filter((_,i)=>i%Number(process.argv[2]??25)===0);
const tot={};let n=0,fails=0;const rowsPer=[];const newest=[];
for(const name of sample){try{const h=obs.readInferenceHeaderV1(D+name);n++;const k={};for(const r of h.observations)k[r.kind]=(k[r.kind]??0)+1;rowsPer.push(h.observations.length);for(const[a,b]of Object.entries(k))tot[a]=(tot[a]??0)+b;newest.push(statSync(D+name).mtimeMs);}catch(e){fails++;if(fails<3)console.log(String(e.message).slice(0,300));}}
rowsPer.sort((a,b)=>a-b);
console.log("records",n,"fails",fails,"rows/record median",rowsPer[rowsPer.length>>1],"max",rowsPer.at(-1),"kinds per record avg",JSON.stringify(Object.fromEntries(Object.entries(tot).map(([k,v])=>[k,Math.round(v/n)]))));
