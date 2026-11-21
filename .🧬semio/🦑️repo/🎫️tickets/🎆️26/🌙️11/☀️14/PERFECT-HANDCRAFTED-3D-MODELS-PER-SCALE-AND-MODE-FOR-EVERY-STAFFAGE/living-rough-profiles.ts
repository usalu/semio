/** 🌲 Locate large adjacent-scale contour changes with bounded, direct orthographic triangle rasterization. */
import {readFileSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {createHash} from 'node:crypto'
import {catalogEntries} from './volumes-lib.ts'
import {eachSolid,sceneInput} from './living-volume-input.ts'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {Mask,project,distanceField} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts'
const [id]=process.argv.slice(2),raw=readFileSync(catalogEntries().find(e=>e.slug===id)!.path,'utf8')
const a=sceneInput(raw),mesh=posedMesh(a,a.presentation.defaultPose),q=qualityOf(a),scales=['50','100','200','500'],views=['elevation','front','plan']
const extent=[Infinity,Infinity,Infinity,-Infinity,-Infinity,-Infinity],rows=[]
const size=Math.max(a.dimensions.widthM,a.dimensions.heightM,a.dimensions.depthM),cell=Math.max(.004,size/850)
function world(k,next){let n=0;for(const s of eachSolid(raw,k)('body')){if(s.texture)continue;next(mesh(s,q));if(++n%90===0)Bun.gc(true)}}
world('50',g=>{for(let i=0;i<g.world.length;i+=3)for(let j=0;j<3;j++){extent[j]=Math.min(extent[j],g.world[i+j]);extent[j+3]=Math.max(extent[j+3],g.world[i+j])}})
const boxes=views.map(view=>{const c=[0,1,2,3,4,5,6,7].map(i=>project(view,extent[(i&1)?3:0],extent[(i&2)?4:1],extent[(i&4)?5:2]))
 return {minX:Math.min(...c.map(p=>p.x))-cell*10,maxX:Math.max(...c.map(p=>p.x))+cell*10,minY:Math.min(...c.map(p=>p.y))-cell*10,maxY:Math.max(...c.map(p=>p.y))+cell*10}})
function outlines(m){const a=m.cells,w=m.width,e=new Uint8Array(a.length);for(let k=w+1;k<a.length-w-1;k++)if(a[k]&&(!a[k-1]||!a[k+1]||!a[k+w]||!a[k-w]))e[k]=1;return e}
let previous
for(const k of scales){
 const masks=boxes.map(b=>new Mask(b,cell))
 world(k,g=>{for(let j=0;j<3;j++){const xy=[];for(let i=0;i<g.world.length;i+=3)xy.push(project(views[j],g.world[i],g.world[i+1],g.world[i+2]))
   const ids=g.indices;for(let i=0;i<ids.length;i+=3)masks[j].fill([[xy[ids[i]],xy[ids[i+1]],xy[ids[i+2]]]],true)}})
 const current=masks.map(m=>{m.seal();return outlines(m)})
 if(previous)for(let j=0;j<3;j++){
   const dist=[];for(const [p,target] of [[previous[j],current[j]],[current[j],previous[j]]]){const df=distanceField(target,masks[j].width,masks[j].height);for(let i=0;i<p.length;i++)if(p[i])dist.push(Math.sqrt(df[i])*cell)}
   dist.sort((a,b)=>a-b);rows.push({scale:k,view:views[j],p95M:dist[Math.floor(.95*dist.length)],longestM:dist.at(-1),cellM:cell})
 }
 previous=current;Bun.gc(true)
}
writeFileSync(join(import.meta.dir,'🗑️generated/living-current',id+'-screen.json'),JSON.stringify({hash:createHash('sha256').update(raw).digest('hex'),rows}))
console.log('[DEBUG]',id,rows.map(v=>v.p95M?.toFixed(3)).join(' '))
