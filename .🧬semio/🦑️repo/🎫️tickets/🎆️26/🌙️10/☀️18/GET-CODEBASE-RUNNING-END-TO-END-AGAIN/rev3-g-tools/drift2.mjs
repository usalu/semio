const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {createFactIndexV1,patchFactIndexV1}=await import(G);
const N=600,BLOCKS=3000,BR=5,PRIV=20;
const row=(kind,path,v)=>({kind,path,fact:{v}});
let seed=7;const rnd=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
const mine=Array.from({length:N},()=>[]);
for(let b=0;b<BLOCKS;b++){const rows=Array.from({length:BR},(_,i)=>row("file","/ws/b"+b+"/"+i,"b"+b));const set=new Set();while(set.size<3)set.add(Math.floor(rnd()*N));for(const o of set)mine[o].push(...rows);}
const owners=new Map();
for(let o=0;o<N;o++)owners.set("o"+o,[...mine[o],...Array.from({length:PRIV},(_,i)=>row("file","/ws/o"+o+"/"+i,"p"+o+"_"+i))]);
let index=createFactIndexV1(owners);
const total=ix=>ix.classes.reduce((s,c)=>s+c.r.length,0);
console.log("gen0 classes",index.classes.length,"sum r",total(index),"bound",2*N+1024);
for(let g=1;g<=400;g++){
  const o=Math.floor(rnd()*N);
  index=patchFactIndexV1(index,{upserts:new Map([["o"+o,[...mine[o],...Array.from({length:PRIV},(_,i)=>row("file","/ws/o"+o+"/"+i,"p"+o+"_"+i+"_"+g))]]])});
  if(g%50===0)console.log("gen",g,"classes",index.classes.length,"sum r",total(index));
}
