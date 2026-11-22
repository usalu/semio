/** 🍁 Project each Japanese maple pad onto the branch and printed sector it occupies. */
import {frames,toWorld} from './volumes-base.ts'
import {outline,project,toDrawing,pathOf} from './r3-japanese-maple-kit.ts'
function footprint(vertices,triangles){
 const c=vertices.reduce((p,q)=>({x:p.x+q.x/vertices.length,y:p.y+q.y/vertices.length}),{x:0,y:0})
 const edges=[],used=new Set()
 for(const t of triangles)for(let j=0;j<3;j++){
  const u=t[j],v=t[(j+1)%3],key=Math.min(u,v)*vertices.length+Math.max(u,v)
  if(used.has(key))continue
  used.add(key);const p=vertices[u],q=vertices[v];edges.push([p.x-c.x,p.y-c.y,q.x-p.x,q.y-p.y])
 }
 return Array.from({length:80},(_,j)=>{
  const theta=2*Math.PI*j/80,x=Math.cos(theta),y=Math.sin(theta);let far=0
  for(const [a,b,u,v] of edges){
   const det=x*v-y*u;if(Math.abs(det)<1e-10)continue
   const d=(a*v-b*u)/det,f=(a*y-b*x)/det
   if(f>=0&&f<=1&&d>far)far=d
  }
  return {x:c.x+far*x,y:c.y+far*y}
 })
}
export function maplePaper(a){
 const fr=frames(a),groups=new Map()
 for(const [scale,rep] of Object.entries(a.representations)){
  if(rep.status!=='authored')continue
  const vol=a.volumes[scale],pads=vol.body.filter(q=>q.kind==='polyhedron'&&q.material?.startsWith('maple-leaf')&&!q.texture)
  for(const [view,ink] of Object.entries(rep.geometry)){
   if(scale==='500'){
    const index=ink.findIndex(q=>q.id==='crown'),proto=ink[index],branches=[...new Set(pads.map(q=>q.bone))]
    const separate=branches.map((bone,i)=>{
     const group=pads.filter(q=>q.bone===bone),regions=[...new Set(group.map(q=>q.material.endsWith('sun')?'leaf-sun':'leaf-shade'))]
     const {region,regions:oldRegions,...shape}=proto
     return {...shape,id:'crown-'+bone,bone,material:group[0].material,z:proto.z+i*.002,...(regions.length===1?{region:regions[0]}:{regions})}
    })
    ink.splice(index,1,...separate)
   }
   const shapes=ink.filter(q=>q.filled===true&&q.role==='body'&&(q.id==='crown'||q.bone.startsWith('fol-')))
   const assigned=new Map(shapes.map(q=>[q,[]]))
   for(const p of pads){
    const pts=p.vertices.map(v=>toWorld(fr[p.bone],v)),mid=pts.reduce((c,v)=>c.map((x,i)=>x+v[i]/pts.length),[0,0,0])
    const peers=shapes.filter(q=>q.id==='crown'||q.bone===p.bone),angle=Math.atan2(mid[2],mid[0])*180/Math.PI
    const score=q=>{const m=q.id.match(/-(\d+)$/);return m?Math.abs((angle-Number(m[1])+540)%360-180):0}
    const shape=peers.sort((a,b)=>score(a)-score(b))[0]
    if(!shape)throw Error(p.id+' has no printed branch group')
    const projected=pts.map(p=>project(view,p))
    let ring
    if(scale==='500'&&view==='front'){
     const triangles=p.triangles.map(t=>t.map(i=>projected[i])).filter(([a,b,c])=>Math.abs((b.x-a.x)*(c.y-a.y)-(c.x-a.x)*(b.y-a.y))>1e-9)
     ring=outline(triangles,.001,.000001)[0]
    }else ring=footprint(projected,p.triangles)
    assigned.get(shape).push(ring)
   }
   for(const [shape,pieces] of assigned){
    if(!pieces.length){ink.splice(ink.indexOf(shape),1);continue}
    const feature=.00015*Number(scale),rings=outline(pieces,feature*.23,((scale==='500'?1.3:3)*feature)**2)
    if(!rings.length)throw Error('Missing projection of '+shape.id)
    shape.d=pathOf(toDrawing(view,fr[shape.bone],rings))
   }
   if(scale==='500'){
    const originals=a.representations['200'].geometry[view]
    for(const q of originals.filter(q=>q.region==='limb-bark'&&/^wood-t[123][ew]$/.test(q.id)))ink.push(structuredClone(q))
    for(const shape of shapes){
     if(shape.bone.startsWith('fol-t')&&shape.bone!=='fol-top'){
      const regions=shape.regions??[shape.region];delete shape.region;shape.regions=[...new Set([...regions,'limb-bark'])]
     }
    }
   }
   if(+scale<=100){
    for(const q of vol.body.filter(q=>q.id.startsWith('twig-fork-'))){
     const line=[]
     for(let j=0;j<q.vertices.length;j+=6){
      const ring=q.vertices.slice(j,j+6),p=ring.reduce((sum,v)=>sum.map((n,i)=>n+v[i]/6),[0,0,0])
      line.push(project(view,toWorld(fr[q.bone],p)))
     }
     const coords=toDrawing(view,fr[q.bone],[line])[0]
     if(Math.hypot(coords.at(-1).x-coords[0].x,coords.at(-1).y-coords[0].y)<.00032*Number(scale))continue
     ink.push({id:'ink-'+q.id,kind:'path',bone:q.bone,role:'detail',z:1.84,filled:false,material:q.material,region:'limb-bark',tone:.38,penMm:.18,d:pathOf([coords],false)})
    }
   }
   if(scale==='100')groups.set(view,outline([...assigned.values()].flat(),.019,.075))
  }
 }
 for(const sy of a.symbols)for(const [view,ink] of Object.entries(sy.geometry)){
  const crown=ink.find(q=>q.id==='crown')
  if(crown)crown.d=pathOf(groups.get(view))
 }
}
