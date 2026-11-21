/** 🐐 Trace an invalid profile back to its catalog solid and scale. */
import {catalogEntries,loadAsset} from "./volumes-lib.ts"
import {modelOfAsset} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts"
import {validateModel} from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts"
for(const id of process.argv.slice(2)) {
 const entry=catalogEntries().find(e=>e.slug===id)!
 const doc=await modelOfAsset(loadAsset(entry.path),{poses:'all',clips:[]})
 for(const i of validateModel(doc).filter(i=>i.severity==='error')) {
 const s=doc.solids[Number(i.path.split('/')[2])]
 console.log('[DEBUG]',s?.id,i.code,JSON.stringify(s?.kind==='extrusion'?s.profile:s).slice(0,950))
 }
}
