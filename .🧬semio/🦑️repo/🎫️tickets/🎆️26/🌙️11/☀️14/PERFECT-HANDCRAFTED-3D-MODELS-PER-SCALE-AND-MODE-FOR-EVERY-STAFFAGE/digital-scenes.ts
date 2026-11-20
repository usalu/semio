/** 🧪 Exports each catalog entry in presentation pose and independently reads the GLB scene graph and material variants. */
import { catalogEntries, loadAsset } from "./volumes-lib.ts";
import { auditGeometry, checkGlb, expectedScenes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";
import { writeGlb } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const ids=process.argv.slice(2);
for(const e of catalogEntries()){
  if(ids.length&&!ids.includes(e.category)&&!ids.includes(e.slug))continue;
  const asset=loadAsset(e.path);
  try{
    const geometry=await auditGeometry(asset);
    const bytes=await writeGlb(asset,{pose:asset.presentation.defaultPose});
    const result=await checkGlb(bytes,asset.id+".glb",asset.id,expectedScenes(asset));
    console.log(JSON.stringify({id:asset.id,scenes:geometry.facts.scenes.length,bytes:bytes.length,findings:[...geometry.findings,...result.findings]}));
  }catch(err){console.log(JSON.stringify({id:asset.id,error:String(err)}));}
}
