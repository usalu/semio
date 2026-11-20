import {catalogEntries,loadAsset} from "./volumes-lib.ts"
import {auditGeometry} from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts"
for(const id of process.argv.slice(2)){
    const entry=catalogEntries().find(e=>e.slug===id)!
    const {facts,findings}=await auditGeometry(loadAsset(entry.path))
    console.log(id,JSON.stringify(findings),JSON.stringify(facts.scenes.map(s=>s.triangles)))
}
