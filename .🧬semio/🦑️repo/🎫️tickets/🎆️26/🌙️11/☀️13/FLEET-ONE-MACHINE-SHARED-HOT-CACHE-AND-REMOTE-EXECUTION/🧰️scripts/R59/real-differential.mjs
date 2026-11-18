import{readdirSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";import{join}from"node:path";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const P=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const load=store=>{const cas=join(store,"v1/main/cas"),out=[];for(const d of readdirSync(cas))for(const f of readdirSync(join(cas,d))){let x;try{x=brotliDecompressSync(readFileSync(join(cas,d,f)))}catch{continue}let j;try{j=JSON.parse(x.toString())}catch{continue}out.push({j,bytes:x.length})}return out};
const oldDocs=load(process.argv[2]).filter(x=>x.j.recipe&&!x.j.version===undefined||x.j.recipe&&x.j.layers),newDocs=load(process.argv[3]).filter(x=>x.j.version===2&&x.j.recipe);
const oldByKey=new Map(oldDocs.map(x=>[P.closureKeyV1(x.j.recipe),x])),host=P.hostOfRootV1(root);
let equal=0,missing=0,diff=0,t0=performance.now();
for(const n of newDocs){
 const key=P.closureKeyV1(n.j.recipe),o=oldByKey.get(key);if(!o){missing++;continue}
 const b=P.unpackBundleV1(n.j),want=o.j;
 const ok=JSON.stringify(b.rows)===JSON.stringify(want.rows)&&JSON.stringify(b.layers)===JSON.stringify(want.layers)&&JSON.stringify(b.value)===JSON.stringify(want.value)&&JSON.stringify(b.sidecar)===JSON.stringify(want.sidecar);
 if(ok)equal++;else{diff++;console.log("DIFF",key.slice(0,8),"rows",JSON.stringify(b.rows)===JSON.stringify(want.rows),"layers",b.layers.length,want.layers.length,JSON.stringify(b.layers)===JSON.stringify(want.layers),"value",JSON.stringify(b.value)===JSON.stringify(want.value),"sidecar",JSON.stringify(b.sidecar)===JSON.stringify(want.sidecar));}
 const local=P.localBundleV1({recipeKey:key,bundle:b},host,new Map(),{generation:undefined});
 console.log(key.slice(0,8),"old",o.bytes,"new",n.bytes,"layers",b.layers.length,"local ok",local!==undefined);
}
console.log("new docs",newDocs.length,"old docs",oldDocs.length,"equal",equal,"different",diff,"no old counterpart",missing,"ms",Math.round(performance.now()-t0));
for(const n of newDocs){const key=P.closureKeyV1(n.j.recipe),o=oldByKey.get(key);const b=P.unpackBundleV1(n.j);if(JSON.stringify(b.value)!==JSON.stringify(o.j.value)){const A=new Set(b.value),Bs=new Set(o.j.value);console.log("VALUE",key.slice(0,8),"only new",[...A].filter(x=>!Bs.has(x)).map(x=>String(x).slice(-70)),"only old",[...Bs].filter(x=>!A.has(x)).length)}}
