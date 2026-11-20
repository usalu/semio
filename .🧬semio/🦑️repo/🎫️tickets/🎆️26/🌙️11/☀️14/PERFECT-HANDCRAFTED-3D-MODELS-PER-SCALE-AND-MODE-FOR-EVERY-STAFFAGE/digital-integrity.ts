import {validateModel} from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import {modelOfAsset} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import {catalogEntries,loadAsset} from "./volumes-lib.ts";
for (const e of catalogEntries()) {
  const args=process.argv.slice(2);
  if(args.length && !args.includes(e.slug) && !args.includes(e.category))continue;
  const model=await modelOfAsset(loadAsset(e.path),{poses:"all",clips:[]});
  const issues=validateModel(model).filter(e=>e.severity==="error").map(e=>({code:e.code,part:model.solids[Number(e.path.split("/")[2])]?.id,path:e.path}));
  console.log(JSON.stringify({id:e.slug,issues}));
}
