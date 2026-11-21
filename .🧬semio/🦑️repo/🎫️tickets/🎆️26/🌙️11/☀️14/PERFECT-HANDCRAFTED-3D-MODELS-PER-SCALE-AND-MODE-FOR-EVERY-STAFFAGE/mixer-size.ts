/** 📦 Attribute the indexed GLB storage of the mixer to its physical part names and primitive attributes. */
import {entryOf} from "./volumes-lib.ts";
import {writeGlb} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const {asset}=entryOf("machinery/concrete-mixer-truck");
const bytes=await writeGlb(asset,{pose:asset.presentation.defaultPose});
const view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength);
const jsonLength=view.getUint32(12,true),j=JSON.parse(new TextDecoder().decode(bytes.slice(20,20+jsonLength)));
const categories=new Map<string,number>(),attrs=new Map<string,number>(),used=new Set<number>();
for(const m of j.meshes)for(const p of m.primitives){
 for(const [attr,a] of Object.entries({...p.attributes,indices:p.indices})){
  const id=j.accessors[a as number].bufferView;
  if(used.has(id))continue;
  used.add(id);
  const key=m.name.replace(/^s\d+-(body|thermal|structure)-/,"").replace(/^(th-|member-)/,"").replace(/(-[0-9]+|-[lr])(?:-|$).*/,"");
  const size=j.bufferViews[id].byteLength;
  categories.set(key,(categories.get(key)??0)+size);
  attrs.set(attr,(attrs.get(attr)??0)+size);
 }
}
console.log(JSON.stringify({bytes:bytes.length,jsonLength,meshes:j.meshes.length,views:j.bufferViews.length,groups:[...categories].sort((a,b)=>b[1]-a[1]).slice(0,36),attrs:[...attrs]},null,2));
