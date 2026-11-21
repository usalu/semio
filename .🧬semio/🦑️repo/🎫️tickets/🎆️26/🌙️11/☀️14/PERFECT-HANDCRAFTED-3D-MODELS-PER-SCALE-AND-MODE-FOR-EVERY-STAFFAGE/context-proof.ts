/** 🔗 Compare the scoped alternative loader to the complete set of catalog IDs and a cross-reference control. */
import {strict as assert} from "node:assert";
import {catalogEntries,loadAsset} from "./volumes-lib.ts";
import {referencedAssets} from "./r3-validate.ts";
const docs=catalogEntries();
let reads=0,links=0;
for(const entry of docs){
  const asset=loadAsset(entry.path);
  assert.equal(asset.id,entry.slug);
  const ids=new Set(Object.values(asset.representations).flatMap((r:any)=>r.alternatives??[]).filter(a=>a.kind==="asset").map(a=>a.asset));
  const map=referencedAssets(asset,()=>docs,p=>{reads++;return loadAsset(p)});
  assert.deepEqual([...map.keys()].sort(),[...new Set([entry.slug,...docs.filter(e=>ids.has(e.slug)).map(e=>e.slug)])].sort());
  links+=ids.size;
}
const [from,to]=docs;
const asset=loadAsset(from!.path);
asset.representations["500"]={status:"unavailable",alternatives:[{kind:"asset",asset:to!.slug},{kind:"asset",asset:"missing-test-asset"},{kind:"asset",asset:to!.slug},{kind:"asset",asset:from!.slug}]};
let controlled=0;
const result=referencedAssets(asset,()=>docs,p=>{assert.equal(p,to!.path);controlled++;return loadAsset(p)});
assert.deepEqual([...result.keys()].sort(),[from!.slug,to!.slug].sort());assert.equal(controlled,1);
console.log(JSON.stringify({documents:docs.length,externalAlternatives:links,extraReads:reads,crossReferenceReads:controlled}));
