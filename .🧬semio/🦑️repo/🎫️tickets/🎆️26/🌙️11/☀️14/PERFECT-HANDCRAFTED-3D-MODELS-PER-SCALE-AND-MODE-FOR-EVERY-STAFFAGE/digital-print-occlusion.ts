/** 🖨 Locate an undersized visible fragment in the unmodified complete projection. */
import {plugin} from 'bun'
import {readFileSync} from 'node:fs'
plugin({name:'paper-occluders',setup(b){
 b.onLoad({filter:/🖨️representation\/🟦️\.ts$/},a=>{
  const src=readFileSync(a.path,'utf8'),key='const previous = featureWorst.get(p.shape.id);'
  if(!src.includes(key))throw Error('Print measure source changed.')
  return {loader:'ts',contents:src.replace(key,'console.error("[DEBUG]",pose.id,viewpoint,p.shape.id,width,filled.filter(s=>s.shape.z>p.shape.z).map(s=>s.shape.id)); '+key)}
 })
}})
