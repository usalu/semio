/** 🌿 Project individual willow foliage groups and exposed switch tips into each authored paper view. */
import {frames,toWorld} from './volumes-base.ts'
import {project,toDrawing,outline,pathOf,nearness} from './r3-weeping-willow-kit.ts'
function convex(points){
 const sorted=points.sort((a,b)=>a.x-b.x||a.y-b.y)
 function side(ps){const out=[];for(const p of ps){while(out.length>=2){const a=out.at(-2),b=out.at(-1);if((b.x-a.x)*(p.y-a.y)-(b.y-a.y)*(p.x-a.x)>1e-8)break;out.pop()}out.push(p)}return out.slice(0,-1)}
 return [...side(sorted),...side(sorted.reverse())]
}
export function drawWillow(a){
 const fr=frames(a)
 for(const [key,rep] of Object.entries(a.representations)){
  if(rep.status!=='authored')continue
  if(+key===500)rep.pens.detail=.10
  const k=+key,v=a.volumes[k],n=k<=100?4:3,h={50:4,100:3,200:3,500:3}[k],feature=.00015*k
  for(const [view,ink] of Object.entries(rep.geometry)){
   const solidInk=ink.filter(q=>!/^(tress-|inner-|fringe-|spray-)/.test(q.id)),groups=new Map(),wood=new Map()
   for(const q of v.body){
    if(!/^(tress-|inner-|fringe-|blade-|twig-)/.test(q.id))continue
    const id=q.id.replace(/^twig-/,'').replace(/^blade-(.*)-\d+$/,'$1'),group=groups.get(id)??{bone:q.bone,tone:q.tone,rings:[],depth:0}
    const world=q.vertices.map(v=>toWorld(fr[q.bone],v)),pts=world.map(p=>project(view,p))
    if(q.id.startsWith('blade-')){group.rings.push(convex(pts))}
    else if(q.id.startsWith('twig-')){
     const bark=wood.get(q.bone)??[],axis=[]
     for(let j=0;j<pts.length-2;j+=4){const ring=pts.slice(j,j+4);axis.push({x:ring.reduce((sum,p)=>sum+p.x,0)/4,y:ring.reduce((sum,p)=>sum+p.y,0)/4})}
     bark.push(axis);wood.set(q.bone,bark)
    }else{
     for(let j=0;j+n*h<=pts.length-2;j+=n*h){
      for(let r=0;r<h-1;r++)group.rings.push(convex(pts.slice(j+r*n,j+(r+2)*n)))
     }
     group.depth=world.reduce((sum,p)=>sum+nearness(view,p),0)/world.length;group.tone=q.tone
    }
    groups.set(id,group)
   }
   for(const [bone,strokes] of wood){
    solidInk.push({id:'switches-'+bone,bone,role:'detail',kind:'path',filled:false,z:1.38,tone:.34,region:'limb-bark',material:'willow-bark',d:pathOf(toDrawing(view,fr[bone],strokes),false)})
   }
   const layers=new Map()
   for(const [id,g] of groups){
    const key=k===500&&id.includes('-spray-')?g.bone+'-'+id.split('-spray-')[0]:id
    if(layers.has(key))layers.get(key).rings.push(...g.rings)
    else layers.set(key,g)
   }
   let z=1.4
   for(const [id,g] of [...layers].sort((x,y)=>x[1].depth-y[1].depth)){
    const rings=outline(g.rings.filter(r=>r.length>=3),feature*.20,feature**2*2);if(!rings.length)continue
    solidInk.push({id,bone:g.bone,role:'body',kind:'path',filled:true,z:+(z+=.001).toFixed(3),region:'leaf-curtain',material:'willow-leaf',tone:g.tone,d:pathOf(toDrawing(view,fr[g.bone],rings))})
   }
   rep.geometry[view]=solidInk.sort((a,b)=>a.z-b.z)
  }
 }
}
