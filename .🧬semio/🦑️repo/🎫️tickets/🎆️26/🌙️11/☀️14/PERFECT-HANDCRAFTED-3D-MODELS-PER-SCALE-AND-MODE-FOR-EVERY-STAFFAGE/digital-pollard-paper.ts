/** 🌿 Draw the grouped sprays attached to each pollard head in its own limb frame. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {entryOf,meshesAt,poseState,catalogEntries} from './volumes-lib.ts'
const library=join(import.meta.dir,'r3-goat-draw2d.ts')
Bun.plugin({name:'read-only-contour-tools',setup(build){build.onLoad({filter:/r3-goat-draw2d\.ts$/},args=>{
 const original=readFileSync(args.path,'utf8'),cut=original.indexOf('const STROKES:')
 if(cut<0)throw Error('Missing isolated contour declarations')
 return {contents:original.slice(0,cut)+'\nexport {outline}',loader:'ts'}
})}})
const {outline}=await import(library)

const source=process.argv[2],doc=JSON.parse(readFileSync(source,'utf8')),a=doc.asset
for(const [d,bark] of [[50,true],[50,false],[200,false],[500,false]]){
 const v=a.volumes[d],r=a.representations[d],body=v.body.filter(s=>!s.texture && (bark?!s.slender&&!s.material.includes('plane-leaf'):s.material.includes('plane-leaf'))),ms=meshesAt(a,d,body)
 const regions=new Map(v.thermal.filter(s=>s.of).map(s=>[s.of,s.region]))
 const groups=new Map()
 for(const [i,s] of body.entries()){
  const key=s.bone,group=groups.get(key)??{bone:s.bone,region:regions.get(s.id),material:s.material,meshes:[]}
  group.meshes.push(ms[i]);groups.set(key,group)
 }
 for(const view of (d===50?['elevation']:['elevation','front','plan'])){
  const original=r.geometry[view],foliage=original.filter(s=>bark? s.filled && s.role==='body' && !s.material?.includes('plane-leaf'):s.material?.includes('plane-leaf'))
  const oldRegion=new Set(foliage.flatMap(s=>s.regions??[s.region]))
  const retained=original.filter(s=>!foliage.includes(s))
  const drawings=Array.from(groups.values(),(g,i)=>({id:'crown-'+g.bone,bone:g.bone,role:'body',z:g.region==='leaf-shade'?2:3+i*.01,tone:g.region==='leaf-shade'?.35:.56+i*.013,kind:'poly',points:[[0,0],[.1,0],[0,.1]],filled:true,closed:true,material:g.material,region:g.region,penMm:.20}))
  if(bark)for(const s of drawings){
   const old=foliage.find(q=>q.bone===s.bone);if(old){s.id=old.id;s.z=old.z;s.tone=old.tone;s.region=old.region??s.region;s.material=old.material??s.material}
  }
  const probe={...a,representations:{...a.representations,[d]:{...r,geometry:{...r.geometry,[view]:drawings}}}}
  const projected=poseState(probe,d)[view]
  for(const s of drawings){
   const [o,x,y]=projected.find(p=>p.shape.id===s.id).rings[0];
   const ax=(x.x-o.x)*10,ay=(x.y-o.y)*10,bx=(y.x-o.x)*10,by=(y.y-o.y)*10,det=ax*by-ay*bx
   const pieces=outline(view,groups.get(s.bone).meshes,bark?.018:d===50?.033:d===200?.080:.16,undefined,true).rings
   const paths=pieces.filter(ring=>ring.length>=3).map(ring=>ring.map(([u,v])=>[+((by*(u-o.x)-bx*(v-o.y))/det).toFixed(5),+((ax*(v-o.y)-ay*(u-o.x))/det).toFixed(5)]))
   if(paths.length){
    const q={...s,kind:'path',d:paths.map(ring=>'M '+ring.map(p=>p.join(' ')).join(' L ')+' Z').join(' ')};delete q.points;delete q.closed
    if(!bark&&oldRegion.has('shoot-bark')&&s.region==='leaf-sun'){delete q.region;q.regions=['leaf-sun','shoot-bark']}
    retained.push(q)
   }
  }
  r.geometry[view]=retained.sort((l,r)=>l.z-r.z)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
