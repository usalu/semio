/** 🐿️ Shape the squirrel's nasal planum, corneas, limb roots and splayed climbing digits in bone coordinates. */
import {readFileSync,writeFileSync} from 'node:fs'
import {ShapeUtils,Vector2,Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./squirrel-extremities-case.json',import.meta.url),'utf8')),fine=structuredClone(a.volumes[50])
const r=n=>+n.toFixed(7),mm=p=>p.map(x=>r(x/1000)),clone=structuredClone
const sec=(x,y,z,rz,ry)=>({at:mm([x,y,z]),rx:r(rz/1000),ry:r(ry/1000),rotation:[0,-.7071068,0,.7071068],exponent:2})
const loft=(id,bone,material,tone,rows,role='body')=>({id,bone,material,tone,kind:'loft',role,capStart:true,capEnd:true,sections:rows.map(row=>sec(...row))})
const ball=(id,bone,material,tone,at,radii,role='detail')=>({id,kind:'ellipsoid',bone,material,tone,role,radii:mm(radii),transform:{translation:mm(at)}})
const head=fine.body.find(s=>s.id==='head'),headFur=a.thermal.regions.find(s=>s.id==='head-fur'),noseRegion=a.thermal.regions.find(s=>s.id==='nose')
const exposedEyes=.000209,physicalAreas={nose:.000137,'hands-skin':.001309,'feet-skin':.002574}
for(const [id,area] of Object.entries(physicalAreas)){
 const region=a.thermal.regions.find(s=>s.id===id);region.areaM2=area;region.areaBasis='exchange'
}
const eyeFraction=r(exposedEyes/a.thermal.body.skinAreaM2),eyeMet=r(headFur.body.metabolicShare*eyeFraction/headFur.body.skinFraction)
a.materials.push({id:'cornea',title:{en:'Moist cornea',de:'Feuchte Hornhaut'},emissivity:.96,solarAbsorptivity:.10,source:'authored-estimate',reference:noseRegion.reference})
const eyes={...clone(noseRegion),id:'eyes',title:{en:'Exposed corneas',de:'Freie Hornhautflächen'},material:'cornea',areaM2:exposedEyes,areaBasis:'exchange',characteristicLengthM:.009,heatCapacityJM2K:2500,body:{segment:'head',metabolicShare:eyeMet,skinFraction:eyeFraction}}
a.thermal.regions.push(eyes);headFur.body.metabolicShare=r(headFur.body.metabolicShare-eyeMet);headFur.body.skinFraction=r(headFur.body.skinFraction-eyeFraction)
for(const p of a.poses)p.thermal.regions.eyes={...p.thermal.regions['head-fur']}
function planum(){
 const border=[[3.7,0],[3.25,2.8],[2,4.4],[.3,4.9],[-1.3,4.0],[-2.2,2.3],[-3.1,.4],[-2.9,0],[-3.1,-.4],[-2.2,-2.3],[-1.3,-4],[.3,-4.9],[2,-4.4],[3.25,-2.8]],holes=[-1,1].map(sign=>Array.from({length:12},(_,i)=>{const t=i*2*Math.PI/12;return [1.65+.72*Math.sin(t)+sign*.18*Math.cos(t),sign*2.7+1.22*Math.cos(t)]}))
 const loops=[border,...holes],area=q=>q.reduce((a,p,i)=>{const h=q[(i+1)%q.length];return a+p[0]*h[1]-h[0]*p[1]},0)
 loops.forEach((q,i)=>{if((area(q)>0)!==(i===0))q.reverse()})
 const caps=ShapeUtils.triangulateShape(border.map(p=>new Vector2(...p)),holes.map(q=>q.map(p=>new Vector2(...p)))),rows=loops.flat(),n=rows.length,vertices=[],triangles=[]
 for(const [x,scale] of [[52.2,.88],[54.0,1.0],[56.2,1]])for(const [y,z] of rows)vertices.push(mm([x+(x>56?.42*(1-(z/5)**2):0),y*scale,z*scale]))
 for(const [i,j,k] of caps)triangles.push([k,j,i],[2*n+i,2*n+j,2*n+k])
 for(let step=0;step<2;step++){let off=0;for(const ring of loops){for(let i=0;i<ring.length;i++){const h=off+i,j=off+(i+1)%ring.length,lo=step*n,hi=lo+n;triangles.push([lo+h,lo+j,hi+j],[lo+h,hi+j,hi+h])}off+=ring.length}}
 const v=vertices.map(p=>new Vector3(...p));if(triangles.reduce((sum,[i,j,k])=>sum+v[i].dot(v[j].clone().cross(v[k])),0)<0)triangles.forEach(t=>t.reverse())
 return {id:'nose',kind:'polyhedron',bone:'head',material:'bare-skin',role:'detail',tone:.12,vertices,triangles}
}
Object.assign(a.anchors.find(q=>q.id==='nostrils'),{x:.0554,y:.00165,z:.0027})
Object.assign(a.anchors.find(q=>q.id==='snout-tip'),{x:.0564,y:.0002,z:0})
a.dimensions.depthM=.076
const physical=[],thermal=[],structure=[]
function add(s,region,layer){physical.push(s);thermal.push({id:'th-'+s.id,of:s.id,region,...layer&&{layer}})}
add(planum(),'nose');add(ball('nasal-recess','head','bare-skin',.055,[52.8,.9,0],[1.5,2.0,4.4]),'nose')
for(const [name,side,sign] of [['near','n',-1],['far','f',1]]){
 add(ball(name+'-eye','head','cornea',.085,[27.4,9.4,sign*16.9],[5.0,4.45,3.55]),'eyes')
 add(ball(name+'-pupil','head','cornea',.018,[27.8,9.5,sign*19.65],[2.65,2.5,.91]),'eyes')
 const lid=loft(name+'-supraorbital-lid','head','fur-red',.37,[[22.7,10.1,sign*17.3,.38,.38],[24.6,12.4,sign*17.8,1.0,.88],[27.3,13.05,sign*17.4,1.15,.91],[30.4,12.2,sign*16.65,.93,.83],[32.0,10.1,sign*16.5,.35,.35]],'detail')
 add(lid,'head-fur','head-guard')
 const cheek=loft(name+'-muzzle-pad','head','fur-red',.49,[[33.5,0,sign*11.9,1.1,1.2],[39.5,-.6,sign*10.9,3.6,4.6],[45.8,-1.9,sign*8.9,4,3.9],[50.8,-1.4,sign*6.2,2.8,2.4],[53,.0,sign*3.8,.8,.75]],'detail');add(cheek,'head-fur','head-guard')
 for(const [part,zone] of [['hand','hands-skin'],['foot','feet-skin']]){
  const bone=part+'-'+side,hind=part==='foot'
  const palm=loft(name+'-'+(hind?'sole':'fingers'),bone,'bare-skin',.51,hind?[[-6,-5.4,0,2,1],[0,-5.4,0,4.4,1.6],[15,-4.6,0,5,1.8],[25,-3.1,0,6.8,2],[32,-2.2,0,7.4,1.95],[37,-2.7,0,4.1,1.85],[40,-3.38,0,2.7,1.55],[43,-2.8,0,1.3,.65]]:[[-2,-1.4,0,2.2,1.55],[4,-1.5,0,3.5,1.65],[10,-.7,0,5.5,2.05],[14,-.5,0,5.9,1.9],[17,-.7,0,3.4,1.3]])
  add(palm,zone)
  for(const [j,[x0,y0,z0,x1,z1]] of f.digitsMm[part].entries()){
   const z=t=>sign*(z0+(z1-z0)*t),x=t=>x0+(x1-x0)*t,base=hind?-1.6:-.8
   const rows=[[x(0),base,z(0),1.75,1.65],[x(.32),base+1,z(.32),1.62,1.5],[x(.58),base+.65,z(.61),1.52,1.34],[x(.81),base-.25,z(.82),1.06,1.02],[x(1),base-.6,z(1),.84,.76]]
   const digit=loft(name+'-'+part+'-digit-'+(j+1),bone,'bare-skin',.51,rows,'detail');add(digit,zone)
   const dx=hind?2.15:2.05,tipz=z(1)+sign*(z1-z0)*.06
   const claw=loft(digit.id.replace('-digit-','-claw-'),bone,'bare-skin',.24,[[x1-.55,base-.45,z(1),.85,.82],[x1+.6,base-.52,tipz,.75,.7],[x1+1.65,base-.92,tipz,.48,.5],[x1+dx,base-1.7,tipz,.12,.15]],'detail');add(claw,zone)
   const bones=loft('phalanx-'+digit.id,bone,'bone',.82,rows.map(q=>[q[0],q[1],q[2],q[3]*.45,q[4]*.46]),'detail');structure.push({...bones,part:'member',member:bone})
  }
 }
}
const contacts=new Set(['head','neck','chest','rump',...['near','far'].flatMap(n=>['arm','thigh','forearm','shank','hand','foot'].map(p=>n+'-'+p))])
for(const [key,v] of Object.entries(a.volumes)){
 const k=Number(key),src=new Map(fine.body.filter(s=>contacts.has(s.id)).map(s=>[s.id,s]))
 for(const [name,base] of src){
  if(k>50&&['rump','neck','chest','near-forearm','far-forearm','near-shank','far-shank'].includes(name)&&v.body.some(s=>s.id===name)&&!(name==='chest'&&k===500))continue
  const s=clone(base)
  if(s.kind==='loft'){
   if(/^(near|far)-(arm|thigh)$/.test(name)){
    const sign=name.startsWith('near-')?-1:1,hip=name.endsWith('thigh')
    for(const q of s.sections){
     const ease=Math.max(0,Math.min(1,1-(q.at[0]+.002)/(hip?.036:.026)))
     q.at[2]=r(q.at[2]-sign*(hip?.0038:.0031)*ease)
     q.rx=r(q.rx*(1-(hip?.10:.13)*ease))
    }
   }
   if(name.endsWith('-hand'))s.sections=s.sections.filter(q=>q.at[0]<=.0145)
   if(name.endsWith('-foot'))s.sections=s.sections.filter(q=>q.at[0]<=.0345)
   if(k>50){const n=s.sections.length,step=k===100?2:k===200?3:4;s.sections=s.sections.filter((q,i)=>i===0||i===n-1||i%step===0)}
  }
  const old=v.body.findIndex(q=>q.id===name);if(old>=0)v.body[old]=s;else v.body.push(s)
 }
 const ids=new Set(physical.map(q=>q.id));v.body=[...v.body.filter(s=>!ids.has(s.id)),...physical]
 const own=new Set([...ids,...contacts])
 v.thermal=v.thermal.filter(s=>!own.has(s.of)&&!['th-nose','th-head-underfur'].includes(s.id))
 for(const t of fine.thermal.filter(q=>q.of&&contacts.has(q.of)))v.thermal.push(clone(t))
 if(k===50){
  v.thermal.push(clone(fine.thermal.find(s=>s.id==='th-head-underfur')))
  for(const heat of v.thermal.filter(q=>/^th-(near|far)-(arm|thigh)-underfur$/.test(q.id))){
   const sign=heat.id.includes('near')?-1:1,hip=heat.id.includes('thigh')
   for(const sec of heat.sections){
    const ease=Math.max(0,Math.min(1,1-(sec.at[0]+.002)/(hip?.036:.026)))
    sec.at[2]=r(sec.at[2]-sign*(hip?.0038:.0031)*ease)
    sec.rx=r(sec.rx*(1-(hip?.10:.13)*ease))
   }
  }
 }
 v.thermal.push(...thermal);v.structure.push(...structure)
 const nostril=v.thermal.find(s=>s.id==='th-breath-nose');if(nostril)nostril.transform.translation=mm([55.4,1.65,2.7])
 v.decisions.push({en:'The exposed corneas, pierced nasal planum, four fore digits and five hind digits retain physical size across digital scales. The distal phalanges follow the named hand and hind-foot bones.',de:'Die freiliegenden Hornhäute, der Nasenspiegel mit zwei Öffnungen sowie vier Vorder- und fünf Hinterzehen behalten ihre körperliche Größe über alle digitalen Maßstäbe. Die Endglieder folgen den jeweiligen Hand- und Fußknochen.'})
}
for(const r of Object.values(a.representations)){
 if(r.status!=='authored')continue
 for(const shapes of Object.values(r.geometry))for(const s of shapes){
  if(s.id.includes('eye')&&s.filled){s.region='eyes';delete s.regions}
  else if(s.bone==='head'&&s.filled&&s.role==='body'){s.regions=[...new Set([...(s.regions??[s.region]).filter(Boolean),'eyes'])];delete s.region}
 }
}
for(const k of [50,100]){
 const rep=a.representations[k],v=a.volumes[k]
 const targets=v.body.filter(s=>/-(digit|claw)-|-(pupil|eye|supraorbital-lid|fingers|sole|hand|foot)$/.test(s.id))
 for(const [view,shapes] of Object.entries(rep.geometry)){
  const ids=new Set(targets.map(s=>s.id)),keep=shapes.filter(s=>!ids.has(s.id))
  for(const s of targets){
   const side=s.id.startsWith('near')?-1:1
   const base=s.bone.startsWith('hand-')?.015*side:s.bone.startsWith('foot-')?.013*side:0
   const meta=shapes.find(q=>q.id===s.id)||shapes.find(q=>q.filled&&q.bone===s.bone&&q.role==='body')||{z:3}
   const region=v.thermal.find(q=>q.of===s.id)?.region
   let points,slab
   if(s.kind==='loft'){
    const ss=s.sections,first=ss[0],last=ss.at(-1)
    points=[...ss.map(q=>[q.at[0],r(q.at[1]+q.ry)]),...[...ss].reverse().map(q=>[q.at[0],r(q.at[1]-q.ry)])]
    slab={kind:'extruded',sweep:'w',z0:r(base+first.at[2]-first.rx),z1:r(base+first.at[2]+first.rx),zTip:[r(base+last.at[2]-last.rx),r(base+last.at[2]+last.rx)]}
   }else{
    const [x,y,z]=s.transform.translation,[rx,ry,rz]=s.radii
    points=Array.from({length:16},(_,i)=>[r(x+rx*Math.cos(i*Math.PI/8)),r(y+ry*Math.sin(i*Math.PI/8))])
    slab={kind:'extruded',sweep:'w',z0:r(z-rz),z1:r(z+rz)}
   }
   keep.push({id:s.id,bone:s.bone,role:'body',z:r((meta.z??3)+.001),tone:s.tone,region,kind:'poly',filled:true,closed:true,points,solid:slab,penMm:.2})
  }
  for(const [label,sign] of [['near',-1],['far',1]])for(const part of ['hand','foot']){
   const bone=part+'-'+(sign===-1?'n':'f'),hind=part==='foot',base=(hind?.013:.015)*sign
   const palm=keep.find(q=>q.bone===bone&&q.id===label+'-'+(hind?'sole':'fingers'))
   const hook=keep.find(q=>q.id===label+'-'+part+'-claw-3')
   if(!palm||!hook)throw Error('Missing terminal paper geometry '+label+part+view)
   const stations=hind?[[-6,-5,1.6],[0,-4.4,2.5],[15,-3,3],[30,-1.4,3.1],[42,-1.5,2.2],[50,-2,1.55],[54,-2.2,.92]]:[[-2,-.6,1.65],[5,-1,2.4],[13,-.5,2.85],[19,-.15,2.3],[27,-.3,1.6],[32,-.85,1.25],[35,-1.3,.95]]
   palm.points=[...stations.map(q=>mm([q[0],q[1]+q[2]])),...[...stations].reverse().map(q=>mm([q[0],q[1]-q[2]]))]
   const z=hook.solid.z0+(hook.solid.z1-hook.solid.z0)/2,wing=view==='plan'?.006:hind?.014:.010
   palm.solid={kind:'extruded',sweep:'w',z0:r(base-(hind?.0058:.0046)),z1:r(base+(hind?.0058:.0046)),zTip:[r(z-wing),r(z+wing)]}
   palm.regions=[hind?'legs-fur':'arms-fur',hind?'feet-skin':'hands-skin'];delete palm.region
   for(let i=keep.length-1;i>=0;i--)if(keep[i].bone===bone&&keep[i]!==palm&&keep[i]!==hook)keep.splice(i,1)
  }
  rep.geometry[view]=keep.sort((a,b)=>a.z-b.z)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
