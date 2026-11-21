/** 🛞 Keep each rear wheel attached to its rigid mounting plate and bearing receiver at every scale. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,source=a.volumes['50']
for(const scale of ['100','200','500']){
 const vol=a.volumes[scale]
 for(const side of ['l','r'])for(const part of ['axle-plate','axle-receiver']){
  const id=part+'-'+side
  if(vol.body.some(s=>s.id===id))continue
  const plate=structuredClone(source.body.find(s=>s.id===id)),surface=structuredClone(source.thermal.find(s=>s.of===id))
  vol.body.push(plate);vol.thermal.push(surface)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
