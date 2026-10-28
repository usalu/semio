// [gram2-g] exploratory equality check of the chunked index against the tree's createFactIndexV1 on seeded structured corpora (shared blocks, private rows, duplicates, empty owners, equal-key rows as different objects).
import{createFactIndexChunkedV1}from"./chunked.mjs";
const g=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs");
const rng=seed=>{let s=seed>>>0;return()=>{s=(Math.imul(s,1664525)+1013904223)>>>0;return s/4294967296;};};
function corpus(seed){
 const r=rng(seed),pool=Array.from({length:200+Math.floor(r()*3000)},(_,i)=>({kind:["exists","file","readdir","resolve"][i%4],path:"/w/p"+(i%977)+"/"+"x".repeat(i%7),fact:i%5===0?{size:i,mtimeMs:1,ctimeMs:1,ino:1,sha256:"h"+(i%13)}:"v"+(i%11)})),blocks=Array.from({length:1+Math.floor(r()*5)},()=>pool.slice(Math.floor(r()*pool.length*.5)).slice(0,20+Math.floor(r()*600)));
 const owners=new Map(),count=1+Math.floor(r()*120);
 for(let o=0;o<count;o++){const list=[];if(r()<.9)for(const b of blocks)if(r()<.7)for(const row of b)list.push(r()<.02?{...row}:row);const extra=Math.floor(r()*40);for(let k=0;k<extra;k++)list.push(pool[Math.floor(r()*pool.length)]);if(r()<.1)list.length=0;if(r()<.2)list.push(...list.slice(0,10));owners.set("o"+String(Math.floor(r()*1e6)).padStart(6,"0")+"-"+o,list);}
 return owners;
}
let bad=0,total=0;
for(let seed=1;seed<=300;seed++){const input=corpus(seed),a=g.createFactIndexV1(input),b=createFactIndexChunkedV1(input,{rowKeyV1:g.rowKeyV1,compareCodePointsV1:g.compareCodePointsV1});total++;
 const same=JSON.stringify(a.owners)===JSON.stringify(b.owners)&&a.rows.length===b.rows.length&&a.rows.every((row,i)=>row===b.rows[i])&&JSON.stringify(a.classes)===JSON.stringify(b.classes);if(!same){bad++;if(bad<4)console.log("DIFF seed",seed,a.classes.length,b.classes.length);}}
console.log("seeds",total,"different",bad);
