/** 🫁 Independently measure all fourteen species' authored apertures in their own face planes using Three.js. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {createHash} from 'node:crypto'
import {Box3,SphereGeometry,Vector3} from 'three'
import {catalogEntries} from './volumes-lib.ts'
const rows=[]
for(const entry of catalogEntries().filter(e=>e.path.includes('animals'))){
 const raw=readFileSync(entry.path),a=JSON.parse(raw.toString()).asset
 const emitters=a.thermal.emitters.filter(e=>e.kind==='breath')
 const samples=[]
 for(const [scale,v] of Object.entries(a.volumes)){
  for(const e of emitters){
   const solids=v.thermal.filter(s=>s.tags?.includes(e.id)&&s.tags?.includes('emitter'))
   if(!solids.length)throw Error(a.id+' '+scale+' '+e.id+' missing')
   for(const s of solids){
    let radii
    if(s.kind==='ellipsoid'){
     const sphere=new SphereGeometry(1,48,24);sphere.scale(...s.radii);sphere.computeBoundingBox()
     radii=sphere.boundingBox.getSize(new Vector3()).toArray().sort((a,b)=>a-b);sphere.dispose()
    }else if(s.kind==='extrude'){
     const face=s.profile.map(([u,v])=>new Vector3(u,v,0));let diameter=0
     for(const u of face)for(const v of face)diameter=Math.max(diameter,u.distanceTo(v))
     const bounds=new Box3().setFromPoints(face).getSize(new Vector3())
     radii=[s.to-s.from,Math.min(bounds.x,bounds.y),diameter]
    }else throw Error('Unexpected physical surface '+s.id)
    if(Math.abs(radii[2]-e.diameterM)>1e-6||radii[0]<=0||radii[0]>=radii[1])throw Error([a.id,scale,s.id,radii.join(','),e.diameterM].join(' '))
    samples.push({scale,id:s.id,expected:e.diameterM,observedM:radii[2],thicknessM:radii[0]})
   }
  }
 }
 rows.push({id:a.id,hash:createHash('sha256').update(raw).digest('hex'),samples})
}
writeFileSync(join(import.meta.dir,'🗑️generated/living-current/apertures.json'),JSON.stringify(rows))
console.log('[DEBUG]',rows.length,rows.flatMap(x=>x.samples).length)
