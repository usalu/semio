/** 🦆 Stretch two scalloped palmate webs between the mallard's three forward toes and articulate the four digital scales. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=structuredClone(a.volumes[50]),r=n=>+n.toFixed(7)
const station=(s,t)=>{
 const first=s.sections[0].at[0],last=s.sections.at(-1).at[0],x=first+(last-first)*t
 let h=s.sections.findIndex(q=>q.at[0]>=x);if(h<=0)return s.sections[h===0?0:s.sections.length-1].at
 const lo=s.sections[h-1].at,hi=s.sections[h].at,at=(x-lo[0])/(hi[0]-lo[0])
 return lo.map((v,i)=>v+(hi[i]-v)*at)
}
function membrane(id,mid,lateral){
 const vertices=[],triangles=[],tRows=[.10,.25,.41,.57,.73,.89,.985],across=[0,.18,.38,.59,.81,1],W=across.length,H=tRows.length,N=W*H
 for(const sign of [-1,1])for(const t of tRows)for(const u of across){
  const scallop=Math.sin(Math.PI*u),s=t-.09*scallop*Math.max(0,(t-.45)/.535)**2
  const m=station(mid,s),l=station(lateral,s)
  const p=m.map((v,j)=>v+(l[j]-v)*u)
  p[1]+=.0007*scallop*Math.sin(Math.PI*t)+sign*(.00057+.00036*(1-t)+.00022*(1-scallop))
  vertices.push(p.map(r))
 }
 const wall=(l,h)=>triangles.push([l,h,h+N],[l,h+N,l+N])
 for(let y=0;y<H-1;y++)for(let x=0;x<W-1;x++){const i=y*W+x,j=i+1,m=i+W,n=m+1;triangles.push([i,j,n],[i,n,m],[N+i,N+n,N+j],[N+i,N+m,N+n])}
 const edge=[...Array.from({length:W},(_,i)=>i),...Array.from({length:H-1},(_,i)=>(i+2)*W-1),...Array.from({length:W-1},(_,i)=>N-2-i),...Array.from({length:H-2},(_,i)=>(H-i-2)*W)]
 for(let i=0;i<edge.length;i++)wall(edge[(i+1)%edge.length],edge[i])
 const vs=vertices.map(p=>new Vector3(...p));if(triangles.reduce((sum,[i,j,k])=>sum+vs[i].dot(vs[j].clone().cross(vs[k])),0)<0)triangles.forEach(t=>t.reverse())
 return {id,kind:'polyhedron',bone:mid.bone,role:'body',material:mid.material,tone:.68,vertices,triangles}
}
for(const [prefix,side] of [['near','n'],['far','f']]){
 const get=k=>fine.body.find(q=>q.id===prefix+'-'+k),mid=get('toe-middle'),inner=get('toe-inner'),outer=get('toe-outer'),hallux=get('hallux')
 const end=hallux.sections.at(-1),hand=side==='n'?1:-1
 hallux.sections.push({...end,at:[-.0158,-.00405,hand*.0072],rx:.00045,ry:.00043},{...end,at:[-.0171,-.00456,hand*.00745],rx:.00010,ry:.00010})
 const paper=a.representations[50].geometry.elevation,heel=paper.find(q=>q.id===prefix+'-hallux')
 paper.push({...heel,id:prefix+'-hallux-horn',tone:.38,points:[[-.01405,-.00295],[-.01507,-.00326],[-.01624,-.00380],[-.01718,-.00458],[-.01625,-.00435],[-.01477,-.00398],[-.01387,-.00370]]})
 const surface=[mid,inner,outer,hallux,membrane(prefix+'-web-inner',mid,inner),membrane(prefix+'-web-outer',mid,outer)]
 const halluxBone={...structuredClone(hallux),id:prefix+'-digit-hallux',material:'bone',tone:.84,member:'foot-'+side,part:'member',sections:hallux.sections.map(q=>({...q,rx:r(q.rx*.48),ry:r(q.ry*.46)}))}
 const bodyIds=new Set(surface.map(q=>q.id)),names=new Set([...bodyIds,prefix+'-foot',prefix+'-web'])
 for(const [key,v] of Object.entries(a.volumes)){
  const k=+key
  v.body=v.body.filter(q=>!names.has(q.id));v.thermal=v.thermal.filter(q=>!names.has(q.of))
  for(const src of surface){
   const q=structuredClone(src)
   if(q.kind==='loft'&&k>50){const n=q.sections.length,pick=k===100?2:3;q.sections=q.sections.filter((s,i)=>i===0||i===n-1||i%pick===0)}
   v.body.push(q);v.thermal.push({id:'th-'+q.id,of:q.id,region:'web-feet'})
  }
  if(!v.body.some(q=>q.id===prefix+'-tarsus')){
   const t=structuredClone(get('tarsus'));t.sections=t.sections.filter((s,i,ss)=>i===0||i===Math.floor(ss.length/2)||i===ss.length-1)
   v.body.push(t);v.thermal.push({id:'th-'+t.id,of:t.id,region:'legs-skin'})
  }
  if(k>=200){
   const leg=prefix+'-leg';v.body=v.body.filter(q=>q.id!==leg);v.thermal=v.thermal.filter(q=>q.of!==leg)
   for(const [part,region] of [['drumstick','body-plumage'],['shank','legs-skin']]){
    const s=structuredClone(get(part)),ss=s.sections
    s.sections=k===200?[ss[0],ss[2],ss[ss.length-2],ss.at(-1)]:[ss[0],ss[Math.floor(ss.length/2)],ss.at(-1)]
    v.body.push(s);v.thermal.push({id:'th-'+s.id,of:s.id,region})
   }
  }
  for(const digit of ['inner','outer'])if(!v.structure.some(q=>q.id===prefix+'-digit-'+digit)){
   const s=structuredClone(fine.structure.find(q=>q.id===prefix+'-digit-'+digit));s.sections=s.sections.filter((q,i,ss)=>i===0||i===Math.floor(ss.length/2)||i===ss.length-1);v.structure.push(s)
  }
  v.structure.push(halluxBone)
 }
}
for(const k of [200,500]){
 const v=a.volumes[k]
 v.decisions[0]={en:'Folded wing and trunk retain their coarse volumes. Both paddles articulate at the tarsometatarsal joint. Two thin, scalloped membranes span the medial, middle and lateral toes. A free hallux projects behind the foot. Four phalangeal rays support the bare foot in the structural view.',de:'Gefaltete Flügel und Rumpf behalten ihre vereinfachten Volumen. Beide Schwimmfüße bewegen sich am Zehengrundgelenk. Zwei dünne, ausgebuchtete Häute verbinden die drei Vorderzehen. Die freie Hinterzehe ragt nach hinten. Vier Zehenstrahlen stützen den nackten Fuß in der Strukturdarstellung.'}
}
const rep=a.representations[200];rep.pens.body=.2
for(const [view,shapes] of Object.entries(rep.geometry)){
 const paper=a.representations[100].geometry[view].filter(q=>/^(near|far)-(foot|tarsus|shank|drumstick)$/.test(q.id))
 rep.geometry[view]=[...shapes.filter(q=>!/^(near|far)-(foot|tarsus|leg)$/.test(q.id)),...paper].sort((x,y)=>x.z-y.z)
}
const rest=structuredClone(a.poses.find(p=>p.id==='standing'))
rest.id='resting-ashore';rest.title={en:'Resting ashore',de:'Ruhend am Ufer'};rest.description={en:'Breast and both plantar webs carry the settled drake on a dry bank, with its shanks folded beneath the flank.',de:'Brust und beide Schwimmhäute stützen den sitzenden Erpel auf dem trockenen Ufer. Die Unterschenkel liegen unter den Flanken.'}
rest.bones.trunk={angleDeg:0};rest.bones['neck-1']={angleDeg:-27};rest.bones['neck-2']={angleDeg:2};rest.bones['neck-3']={angleDeg:-2};rest.bones.head={angleDeg:14}
for(const side of ['n','f'])for(const [id,angleDeg] of [['thigh',64],['shank',-85],['tarsus',61],['foot',-45]])rest.bones[id+'-'+side]={angleDeg}
a.anchors.push({id:'resting-breast',bone:'trunk',x:.054,y:-.089,kind:'contact',title:{en:'Ventral breast resting on the bank',de:'Brustauflage am Ufer'},support:'contact',normalDeg:90,footprintLengthM:.089,footprintWidthM:.052,frictionCoefficient:.55})
rest.contacts=['resting-breast','contact-n','contact-f'];rest.thermal.activityMet=1.1;rest.thermal.breathing='rest';rest.thermal.regions['web-feet']={exposedFraction:.20,sunlitFraction:0,contact:{fraction:.42,tempC:18}};rest.thermal.regions['legs-skin']={exposedFraction:.18,sunlitFraction:0};rest.thermal.regions['body-plumage']={exposedFraction:.83,sunlitFraction:.48,contact:{fraction:.12,tempC:18}}
const exchange=a.thermal.regions.find(r=>r.id==='web-feet')
exchange.areaM2=.00674914;exchange.areaBasis='exchange';a.poses.push(rest)
writeFileSync(process.argv[3],JSON.stringify(doc))
