/** 🧪 Exports each catalog entry in presentation pose and independently reads the GLB scene graph and material variants. */
import { catalogEntries, loadAsset } from "./volumes-lib.ts";
import { BUDGET, auditGeometry, checkGlb, expectedScenes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts";
import { writeGlb } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";
const ids=process.argv.slice(2);
const entries=ids[0]==="--file" ? [{path:ids[1]!}] : catalogEntries().filter(e=>!ids.length||ids.includes(e.category)||ids.includes(e.slug));
for(const e of entries){
  const asset=loadAsset(e.path);
  try{
    const geometry=await auditGeometry(asset);
    const bytes=await writeGlb(asset,{pose:asset.presentation.defaultPose});
    const result=await checkGlb(bytes,asset.id+".glb",asset.id,expectedScenes(asset));
    const findings=[...geometry.findings,...result.findings];
    if(bytes.length>BUDGET.glbBytes) findings.push({check:"budget",item:asset.id,message:`GLB is ${bytes.length} bytes (budget ${BUDGET.glbBytes})`});
    console.log(JSON.stringify({id:asset.id,scenes:geometry.facts.scenes.length,bytes:bytes.length,findings}));
  }catch(err){console.log(JSON.stringify({id:asset.id,error:String(err)}));}
}
