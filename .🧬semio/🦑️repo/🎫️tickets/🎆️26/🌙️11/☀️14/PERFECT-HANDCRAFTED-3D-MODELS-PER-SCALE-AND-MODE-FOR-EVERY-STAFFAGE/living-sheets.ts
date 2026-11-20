import { entryOf, scaleMeshes } from "./volumes-lib.ts"
import { fit, drawAxon, bodyItems, thermalItems, structureItems } from "./volumes-render.ts"
import { Canvas } from "./volumes-raster.ts"
import { writeFileSync,mkdirSync,readdirSync } from "node:fs"
import { spawnSync } from "node:child_process"
import { join } from "node:path"
const out = join(import.meta.dir,"🗑️generated","living")
mkdirSync(out,{recursive:true})
if (!process.argv[2]) {
    const base = join(process.cwd(),"🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog")
    for (const [dir,group] of [["🌳️vegetation","vegetation"],["🐕️animals","animals"]]) for (const d of readdirSync(join(base,dir),{withFileTypes:true})) {
        if (!d.isDirectory()) continue
        const child = spawnSync(process.execPath,[import.meta.filename,group+"/"+d.name],{stdio:"inherit"})
        if (child.status!==0) process.exit(child.status || 1)
    }
} else {
    const {asset} = entryOf(process.argv[2])
    const sheet = new Canvas(650, 350)
    const cam = {r:[0.866,0,-0.5],u:[-0.137,0.962,-0.238],e:[0.481,0.275,0.833]} as Parameters<typeof fit>[1]
    for (const [i,scale] of [50,100,200,500].entries()) {
        const rig = asset.representations[scale].status==="authored" ? asset : {...asset,representations:{...asset.representations,[scale]:{...asset.representations[100],denominator:scale}}}
        const v = asset.volumes[scale]
        const sm = scaleMeshes(rig,scale,v)
        const panels = [bodyItems(rig,sm,scale),thermalItems(rig,sm,scale),structureItems(rig,sm,scale)]
        for (const [j,mode] of (["body","thermal","structure"] as const).entries()) {
            if (sm[mode].length !== v[mode].length || sm[mode].some(m=>!m.indices.length || !m.world.every(Number.isFinite))) throw Error(asset.id+" "+scale+" "+mode)
            const panel = new Canvas(160,100,[247,247,244])
            const ax = fit(sm.body,cam,160,100,0.07)
            drawAxon(panel,ax,panels[j]!,{cut:mode==="thermal"})
            sheet.blit(panel,i*162,30+j*103)
        }
        sheet.text(i*162,15,String(scale))
    }
    sheet.text(0,0,asset.id)
    writeFileSync(join(out,asset.id+".png"),sheet.png())
    console.log(asset.id)
}
