/** 🍁 Re-evaluate the original tier-specific drawing construction, keeping each current authored solid. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
const model=join(import.meta.dir,'r5-japanese-maple-model.ts')
Bun.plugin({name:'maple-drawing-only',setup(build){build.onLoad({filter:/r5-japanese-maple-model\.ts$/},async args=>{
 const original=readFileSync(args.path,'utf8')
 const code=original.replace(/writeFileSync\(join\(SOURCES, "vegetation", "japanese-maple.json"\),[^\n]+/,'writeFileSync(join(GENERATED,"living-maple-lobes.json"), JSON.stringify(Object.fromEntries(models.map(m=>[m.scale,m.lobes.map(l=>({id:l.id,world:l.c,axes:l.axes,radii:l.radii,pad:l.pad??false,kind:l.kind,frame:F[l.bone],raw:solidOf(l,F)}))]))));').replace(/writeFileSync\(join\(out, "🔣️.json"\),[^\n]+/,'writeFileSync(join(GENERATED, "living-maple-paper.json"),JSON.stringify({rig:asset.rig,anchors:asset.anchors,representations:asset.representations,symbols:asset.symbols}));')
 if(code.includes('writeFileSync(join(SOURCES')||!code.includes('living-maple-paper.json'))throw Error('Unexpected construction output')
 return {loader:'ts',contents:code}
})}})
await import(model)
