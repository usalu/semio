/** 📏 Verifies shipped digital models, including coordinates, static analyses and authored scale eligibility. */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { STAFFAGE, catalogEntries, loadAsset } from "./volumes-lib.ts";
import { checkVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { volumeIssues } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
const args=process.argv.slice(2);
const schema=JSON.parse(readFileSync(STAFFAGE+"/🧬️schema/🔣️.json","utf8"));
const validator=new Ajv({strict:true,allErrors:true}).addSchema(schema).compile({$ref:schema.$id+"#/$defs/AssetDocument"});
for (const entry of catalogEntries()) {
  if(args.length && !args.includes(entry.category) && !args.includes(entry.slug)) continue;
  const doc=JSON.parse(readFileSync(entry.path,"utf8")), asset=doc.asset;
  const valid=validator(doc);
  if(!valid) console.log(asset.id,"schema",JSON.stringify(validator.errors?.filter(e=>e.keyword!=="oneOf" && e.keyword!=="additionalProperties").slice(0,8)));
  for(const d of [50,100,200,500] as const) {
    if(asset.representations[d].status==="authored") continue;
    if(asset.volumes[d].status!=="authored") { console.log(asset.id,d,"missing"); continue; }
    try{
      const found=checkVolumes(asset,{scale:d});
      console.log(asset.id,d,found.length,found.map(f=>f.rule+" "+f.message.en).join(" | "));
    }catch(e){console.log(asset.id,d,"ERROR",String(e));}
  }
}
