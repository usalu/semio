const G=process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const gen=await import(G+"🟨️.mjs"),codec=await import(G+"🧮️fact-codec/🟨️.mjs"),sp=await import(G+"🌊️stream-patch/🟨️.mjs");
let seed=7;const rnd=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const facts=[{size:1},{size:2,h:"a".repeat(64)},"x",null,[1,2,{k:"v"}],{nested:{a:[true,false,-3,1.5]}},42];
const pool=Array.from({length:60},(_,i)=>({kind:["file","exists","analysis"][i%3],path:"p/"+(i%40)+(i>40?"é":""),fact:facts[i%facts.length]&&i%5===0?facts[i%facts.length]:{n:i%13}}));
const pick=(k)=>pool.filter(()=>rnd()<k);
let bad=0;
for(let world=0;world<200;world++){
 const owners=1+Math.floor(rnd()*10),table=new Map();for(let o=0;o<owners;o++)table.set("o"+o,pick(0.3));
 let index=gen.createFactIndexV1(table);let raw=codec.encodeFactIndexV1(index,{maximumBytes:1<<26});
 for(let step=0;step<6;step++){
  const view=codec.openFactIndexV1(raw);
  const kind=rnd(),name="o"+Math.floor(rnd()*(owners+2)),patch={drop:[],upserts:new Map(),append:new Map()};
  if(kind<0.45){const rows=pick(0.3);table.set(name,rows);patch.upserts.set(name,rows);}
  else if(kind<0.7){table.delete(name);patch.drop.push(name);}
  else if(kind<0.9&&table.has(name)){const extra=pick(0.2);table.set(name,[...table.get(name),...extra]);patch.append.set(name,extra);}
  const ref=gen.patchFactIndexV1(codec.decodeFactIndexV1(raw),patch);
  raw=await sp.patchFactIndexBytesV1(view,patch);
  const dec=codec.decodeFactIndexV1(raw);
  const again=codec.encodeFactIndexV1(dec,{maximumBytes:1<<26});
  const same=Buffer.compare(again,raw)===0;
  const refBytes=codec.encodeFactIndexV1(ref,{maximumBytes:1<<26});
  const cons=i=>JSON.stringify(gen.factConsumersV1(i).filter(([,n])=>n.length).sort((a,b)=>a[0]<b[0]?-1:1));
  if(!same||cons(dec)!==cons(ref)){bad++;if(bad<4)console.log("MISMATCH world",world,"step",step,"canonical",same,"consumers",cons(dec)===cons(ref),"rows",dec.rows.length,ref.rows.length);}
  if(Buffer.compare(refBytes,raw)!==0&&bad<4&&same&&cons(dec)===cons(ref)){}
 }
}
console.log("done bad",bad);
