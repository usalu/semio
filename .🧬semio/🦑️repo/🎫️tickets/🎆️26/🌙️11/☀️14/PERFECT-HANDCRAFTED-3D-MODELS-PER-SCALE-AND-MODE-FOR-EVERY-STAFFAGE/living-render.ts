import { entryOf, scaleMeshes, type ListName } from "./volumes-lib.ts"
import { renderScale } from "./volumes-render.ts"
import { join } from "node:path"
import { mkdirSync } from "node:fs"
const out = join(import.meta.dir, "🗑️generated", "living")
mkdirSync(out, {recursive:true})
for (const id of ["hosta-clump", "domestic-cat", "red-fox", "hedgehog", "red-squirrel", "herring-gull", "mallard-duck", "pigeon"]) {
    const {asset} = entryOf((id==="hosta-clump" ? "vegetation/" : "animals/")+id)
    for (const scale of [200,500]) {
        if (asset.representations[scale].status==="authored") continue
        const v = asset.volumes[scale]
        const rig = {...asset, representations:{...asset.representations,[scale]:{...asset.representations[100],denominator:scale}}}
        const meshes = scaleMeshes(rig,scale,v)
        for (const mode of ["body","thermal","structure"] as ListName[]) {
            if (meshes[mode].length !== v[mode].length || meshes[mode].some(m=>!m.indices.length)) throw new Error(id+" "+scale+" "+mode)
            renderScale(rig, scale, {}, [mode], {az:30,el:17,width:360,cut:mode==="thermal"}, () => join(out,id+"-"+scale+"-"+mode+".png"))
        }
    }
}
