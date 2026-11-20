import {validateModel} from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts"
import {modelOfAsset} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts"
import {entryOf} from "./volumes-lib.ts"
for (const id of ["goat","labrador-dog"]){
    const {asset}=entryOf("animals/"+id)
    const model=await modelOfAsset(asset,{poses:"all",clips:[]})
    for(const e of validateModel(model).filter(s=>s.severity==="error")){
        const i=Number(e.path.split("/")[2])
        console.log(id,e.code,model.solids[i].id)
    }
}
