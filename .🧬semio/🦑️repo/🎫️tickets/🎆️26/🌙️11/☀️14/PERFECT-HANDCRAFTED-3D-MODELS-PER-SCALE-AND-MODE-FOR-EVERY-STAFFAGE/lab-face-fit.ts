/** 🦮 Shape the Labrador stop, flews, wet eyelid apertures, zygomatic arches and perforated nasal planum. */
import {readFileSync,writeFileSync} from 'node:fs'
import {ShapeUtils,Vector2,Vector3} from 'three'
import {meshesAt,featureM,simplifyRing,area} from './volumes-lib.ts'
import {projectMask,contours,inverse} from './r3-project2d.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./lab-face-case.json',import.meta.url),'utf8'))
const round=n=>+n.toFixed(7),mm=q=>q.map(v=>round(v/1000)),base=(id,kind,material,tone,role='body')=>({id,kind,bone:'head',material,role,tone})
const coat='short-fur',skin='bare-skin',cornea='cornea',bone='bone'
const loft=(id,material,tone,sections)=>({...base(id,'loft',material,tone),sections,capStart:true,capEnd:true})
const section=(x,y,z,rx,ry,e=2)=>({at:mm([x,y,z]),rx:round(rx/1000),ry:round(ry/1000),exponent:e,rotation:[.5,.5,.5,.5]})
const headSection=([x,b,t,w,e])=>section(x,(b+t)/2,0,(t-b)/2,w,e)
const station=(x)=>{
 const rows=f.headStationsMm;let k=rows.findIndex(q=>q[0]>=x)
 if(k<=0)return rows[k===0?0:rows.length-1]
 const l=rows[k-1],h=rows[k],t=(x-l[0])/(h[0]-l[0]);return l.map((n,i)=>n+(h[i]-n)*t)
}
const cheek=(x,y)=>{
 const [,b,t,w,e]=station(x),h=(t-b)/2,cy=(b+t)/2
 return w*Math.max(0,1-Math.abs((y-cy)/h)**e)**(1/e)
}
const shape=(id,material,tone,vertices,triangles,role='body')=>{
 const v=vertices.map(p=>new Vector3(...p))
 if(triangles.reduce((s,[i,j,k])=>s+v[i].dot(new Vector3().crossVectors(v[j],v[k])),0)<0)for(const t of triangles)t.reverse()
 return {...base(id,'polyhedron',material,tone,role),vertices:vertices.map(mm),triangles}
}
const faceParts=[],bones=[],heat=[]
function append(s,region,layer){faceParts.push(s);heat.push({id:'th-'+s.id,of:s.id,region,...(layer?{layer}:{})})}
const holes=[]
const border=[[18,0],[17,12],[14,21],[9,26],[3,26],[-2,24],[-7,18],[-11,10],[-13,3],[-10.7,0],[-13,-3],[-11,-10],[-7,-18],[-2,-24],[3,-26],[9,-26],[14,-21],[17,-12]]
for(const sign of [-1,1])holes.push(Array.from({length:16},(_,i)=>{
 const t=2*Math.PI*i/16,cs=Math.cos(t),sn=Math.sin(t)
 return [6+3.8*sn+sign*.9*cs,sign*11+5*cs]
}))
const area2=ring=>ring.reduce((a,[y,z],i)=>{const p=ring[(i+1)%ring.length];return a+y*p[1]-z*p[0]},0)
for(const [i,ring] of [border,...holes].entries())if((area2(ring)>0)!==(i===0))ring.reverse()
function nose(){
 const loops=[border,...holes],rows=loops.flat(),n=rows.length,vs=[],tri=[]
 const caps=ShapeUtils.triangulateShape(border.map(p=>new Vector2(...p)),holes.map(q=>q.map(p=>new Vector2(...p))))
 for(const [level,scale,depth] of [[0,.89,229],[1,1,236],[2,.99,245.4]]){
  for(const [j,ring] of loops.entries())for(const [y,z] of ring){
   const blend=scale
   const x=level===2?depth+3.1*(1-(z/28)**2)-.85*((y-4)/18)**2:depth
   vs.push([x,y*blend,z*blend])
  }
 }
 for(const [i,j,k] of caps){tri.push([k,j,i],[2*n+i,2*n+j,2*n+k])}
 for(let t=0;t<2;t++){let o=0;for(const ring of loops){for(let j=0;j<ring.length;j++){
  const h=o+j,k=o+(j+1)%ring.length,lo=t*n,hi=(t+1)*n;tri.push([lo+h,lo+k,hi+k],[lo+h,hi+k,hi+h])
 }o+=ring.length}}
 return shape('nose-leather',skin,.12,vs,tri)
}
append(nose(),'nose-skin')
const X=f.eyeX*1000,Y=f.eyeY*1000,W=f.eyeHalfWidthM*1000,H=f.eyeHalfHeightM*1000
const eyeLine=u=>Y-1.6*u
const height=(u,w)=>H*w*Math.max(0,1-u*u)
const dome=(x,y)=>{
 const u=(x-X)/W,h=Math.max(.001,height(u,1)),v=(y-eyeLine(u))/h
 return .55+3.2*Math.max(0,1-u*u)*Math.max(0,1-v*v)
}
function eyeSurface(id,sign,pupil){
 const vs=[],tri=[],n=24
 for(const rad of [1,.73,.39])for(let j=0;j<n;j++){
  const t=2*Math.PI*j/n,u=rad*Math.cos(t),v=rad*Math.sin(t)
  const x=X+u*(pupil?3.2:W),y=pupil?Y+v*3.2:eyeLine(u)+v*H*(1-.45*u*u)
  vs.push([x,y,sign*(cheek(x,y)+dome(x,y)+(pupil?.18:0))])
 }
 const center=vs.length;vs.push([X,Y,sign*(cheek(X,Y)+dome(X,Y)+(pupil?.18:0))])
 const back=vs.length
 for(let i=0;i<n;i++){const [x,y,z]=vs[i];vs.push([x,y,z-sign*(pupil?.6:3)])}
 vs.push([X,Y,sign*(cheek(X,Y)-(pupil?.9:17.8))])
 for(let j=0;j<n;j++){
  const k=(j+1)%n
  for(let t=0;t<2;t++){const lo=t*n,hi=lo+n;tri.push([lo+j,lo+k,hi+k],[lo+j,hi+k,hi+j])}
  tri.push([2*n+j,2*n+k,center],[back+j,back+k,vs.length-1],[j,back+j,back+k],[j,back+k,k])
 }
 for(let i=0;i<tri.length;i++)if(tri[i][2]===vs.length-1)tri[i].reverse()
 const s=shape(id,cornea,pupil?.018:.30,vs,tri,'detail');if(pupil)s.texture=true
 return s
}
for(const [name,sign] of [['n',-1],['f',1]]){
 append(eyeSurface('eye-'+name,sign,false),'eyes')
 append(eyeSurface('pupil-'+name,sign,true),'eyes')
 for(const [label,dir] of [['upper',1],['lower',-1]]){
  const ss=[-1.08,-1,-.75,-.4,0,.4,.75,1,1.08].map(u=>{
   const x=X+W*u,y=eyeLine(u)+dir*(height(u,1)+1.75),mid=Math.max(0,1-u*u),cap=Math.abs(u)>1?.40:1
   return section(x,y,sign*(cheek(x,y)-3.5),cap*(2.65+1.25*mid),cap*(5+.5*mid),2)
  })
  append(loft('lid-'+label+'-'+name,coat,.43,ss),'coat-head','fur')
 }
 const lipStations=[[112,0],[124,.22],[141,.67],[158,.95],[179,1],[198,.85],[216,.62],[228,.20],[232,0]],angles=[2,-9,-22,-35,-48,-62,-76,-84]
 const lipV=[],lipT=[],nx=lipStations.length,ny=angles.length
 for(const back of [false,true])for(const [x,wgt] of lipStations)for(const angle of angles){
  const [,b,t,w,e]=station(x),phi=angle*Math.PI/180,h=(t-b)/2
  const drape=wgt*Math.max(0,Math.sin(Math.PI*(2-angle)/86))**1.4
  const y=(b+t)/2+h*Math.sign(Math.sin(phi))*Math.abs(Math.sin(phi))**(2/e)-(back?0:6.5*drape)
  const z=w*Math.abs(Math.cos(phi))**(2/e)-.8+(back?-3:2.8*drape)
  lipV.push([x,y,sign*z])
 }
 const wall=(i,j,k,l)=>lipT.push([i,j,k],[i,k,l]),offset=nx*ny
 for(let i=0;i<nx-1;i++)for(let j=0;j<ny-1;j++){
  const v=i*ny+j;wall(v,v+ny,v+ny+1,v+1);wall(offset+v,offset+v+1,offset+v+ny+1,offset+v+ny)
 }
 const border=[...Array.from({length:ny},(_,i)=>i),...Array.from({length:nx-1},(_,i)=>(i+1)*ny+ny-1),...Array.from({length:ny-1},(_,i)=>nx*ny-2-i),...Array.from({length:nx-2},(_,i)=>(nx-2-i)*ny)]
 for(let i=0;i<border.length;i++){const j=border[i],k=border[(i+1)%border.length];wall(j,k,offset+k,offset+j)}
 append(shape('flew-'+name,coat,.69,lipV,lipT),'coat-head','fur')
 const arch=[[40,1,37,3,4],[56,-2,48,4.6,4.7],[74,-6,53,5.1,4.5],[91,-3,52,4.8,4.5],[103,5,46,4.4,4.3],[112,10,36,3.7,4],[123,7,27,3.5,4]]
 bones.push({...loft('zygomatic-'+name,bone,.89,arch.map(([x,y,z,h,w])=>section(x,y,sign*z,h,w))),member:'head',part:'member'})
 const ring=holes[name==='n'?0:1]
 const pad={...base('naris-'+name,'extrude',skin,.025,'detail'),plane:'yz',profile:ring.map(mm),from:.237,to:.240}
 pad.texture=true;append(pad,'nose-skin')
}
const initial=a.volumes['50'],jaw=structuredClone(initial.body.find(q=>q.id==='jaw'))
for(const [k,v] of Object.entries(a.volumes)){
 v.body=v.body.filter(q=>!['nose-leather','eye-n','eye-f'].includes(q.id));v.body.push(...structuredClone(faceParts))
 v.body.find(q=>q.id==='head').sections=f.keepStations[k].map(i=>headSection(f.headStationsMm[i]))
 if(!v.body.some(q=>q.id==='jaw')){
  const lower=structuredClone(jaw);lower.sections=lower.sections.filter((_,i)=>[0,4,10,16,20,24].includes(i))
  v.body.push(lower);v.thermal.push({id:'th-jaw',of:'jaw',region:'coat-head',layer:'fur'})
 }
 const jawRows=[[29,-42,-9,32],[45,-51,-16,40],[64,-56,-19,43],[88,-57,-22,42],[115,-56,-22,40],[145,-55,-24,38],[173,-54,-26,35],[198,-51,-26,32],[217,-47,-23,29],[231,-39,-19,23],[237,-29,-17,14],[239,-24,-18,6]]
 const jawSections=rows=>rows.map(([x,b,t,width])=>{
  const y=(b+t)/2+12,dx=x-30,phi=-4.9*Math.PI/180,co=Math.cos(phi),sn=Math.sin(phi)
  return {...section(dx*co+y*sn,-dx*sn+y*co,0,(t-b)/2,width,2.35),rotation:[.47817,.52092,.52092,.47817]}
 })
 const keep={50:[0,1,2,3,4,5,6,7,8,9,10,11],100:[0,1,2,4,6,7,8,9,10,11],200:[0,2,4,6,8,9,11],500:[0,2,5,7,9,11]}[k]
 v.body.find(q=>q.id==='jaw').sections=jawSections(keep.map(i=>jawRows[i]))
 const jawSurface=jawSections(jawRows)
 const tongueRows=[[31.35,4.5,17],[81.17,4.5,19],[140.95,4.5,18],[170.84,4.3,16.5],[188.5,4.1,14],[201,3.3,9],[205,2.6,5]]
 const floor=x=>{
  const hi=jawSurface.findIndex(s=>s.at[0]>=x),lo=jawSurface[hi-1],end=jawSurface[hi],t=(x-lo.at[0])/(end.at[0]-lo.at[0])
  return (1-t)*(lo.at[1]+lo.rx)+t*(end.at[1]+end.rx)
 }
 let tongue=v.body.find(q=>q.id==='tongue')
 if(!tongue){tongue=structuredClone(initial.body.find(q=>q.id==='tongue'));v.body.push(tongue)}
 tongue.sections=tongueRows.map(([x,h,w])=>({...section(x,floor(x/1000)*1000-.75,0,h,w),rotation:[.47817,.52092,.52092,.47817]}))
 const heatIndex=v.thermal.findIndex(q=>q.id==='th-tongue')
 v.thermal[heatIndex]={id:'th-tongue',of:'tongue',region:'tongue'}
 const mouth=a.anchors.find(q=>q.id==='mouth')
 mouth.x=.202;mouth.y=round(floor(mouth.x)+.0017)
 v.thermal.find(q=>q.id==='th-breath-mouth').transform.translation=[mouth.x,mouth.y,0]
 const mandibular=[[31,-34,-17,26],[47,-44,-23,32],[69,-49,-28,33],[98,-50,-28,31],[145,-49,-29,27],[181,-47,-31,23],[210,-42,-30,19],[226,-36,-24,13],[232,-31,-23,7]]
 const mandible=v.structure.find(q=>q.id==='mandible');if(mandible.kind!=='loft'){for(const key of ['p0','p1','r0','r1'])delete mandible[key];Object.assign(mandible,{kind:'loft',capStart:true,capEnd:true})}
 mandible.sections=jawSections((k==='50'?mandibular:k==='100'?mandibular.filter((_,i)=>i!==3):mandibular.filter((_,i)=>[0,2,4,6,7,8].includes(i))))
 v.thermal=v.thermal.filter(q=>!['th-eye-n','th-eye-f','th-nose','th-nose-leather'].includes(q.id))
 v.thermal.push(...structuredClone(heat))
 for(const [name,sign] of [['n',-1],['f',1]]){
  const out=v.thermal.find(q=>q.id==='th-breath-nose-'+name)
  out.transform.translation=[.240,.006,sign*.011]
 }
 v.structure.push(...structuredClone(bones))
 v.decisions.push({en:'A defined stop separates the broad skull from the blunt muzzle. The two flews meet the articulated jaw. The corneal surface is framed by upper and lower lids above the zygomatic arch. The nose leather has two recessed nostrils.',de:'Ein deutlicher Stirnabsatz trennt den breiten Schädel vom stumpfen Fang. Die Lefzen schließen an den beweglichen Unterkiefer an. Ober- und Unterlid fassen die Hornhaut über dem Jochbogen ein. Der Nasenspiegel hat zwei vertiefte Nasenöffnungen.'})
}
const head=a.thermal.regions.find(q=>q.id==='coat-head'),eyeArea=.000701,frac=eyeArea/a.thermal.body.skinAreaM2,met=head.body.metabolicShare*frac/head.body.skinFraction
head.areaM2=.07537;head.body.skinFraction=round(head.body.skinFraction-frac);head.body.metabolicShare=round(head.body.metabolicShare-met)
const nasal=a.thermal.regions.find(q=>q.id==='nose-skin');nasal.areaM2=.003439;nasal.areaBasis='exchange'
a.thermal.assumptions.push({en:'The fine standing face exposes 0.07537 m² of head fur, 0.003439 m² of nasal skin and 0.000701 m² across the two moist eyes. The nasal outlets sit inside the vestibules.',de:'Am feinen Kopfmodell im Stand sind 0,07537 m² Fell, 0,003439 m² Nasenhaut und 0,000701 m² an beiden feuchten Augen exponiert. Die nasalen Austrittsflächen liegen in den Nasenvorhöfen.'})
a.materials.push({id:cornea,title:{en:'Moist cornea',de:'Feuchte Hornhaut'},emissivity:.96,solarAbsorptivity:.1,source:'authored-estimate',reference:'ashrae-2021'})
a.thermal.regions.push({id:'eyes',title:{en:'Exposed corneas',de:'Freie Hornhautflächen'},kind:'skin',model:'body',material:cornea,areaM2:eyeArea,areaBasis:'exchange',characteristicLengthM:.015,geometry:'sphere',skyViewFactor:.48,layers:[],heatCapacityJM2K:2500,body:{segment:'head',metabolicShare:round(met),skinFraction:round(frac)},source:'authored-estimate',reference:'ashrae-2021'})
for(const p of a.poses)p.thermal.regions.eyes={exposedFraction:1,sunlitFraction:p.thermal.regions['coat-head'].sunlitFraction}
for(const [k,v] of Object.entries(a.volumes)){
 const scale=+k,target=v.body.filter(q=>['head','jaw','nose-leather','eye-n','eye-f','flew-n','flew-f','tongue'].includes(q.id))
 const meshes=new Map(meshesAt(a,scale,target).map((m,i)=>[target[i].id,m]))
 const feature=featureM(scale,a)
 for(const [view,shapes] of Object.entries(a.representations[k].geometry)){
  const groups={head:['head','flew-n','flew-f',...(scale>=200?['nose-leather']:[]),...(scale===500?['jaw']:[])],jaw:scale>=100?['jaw','tongue']:['jaw'],tongue:['tongue'],'nose-leather':['nose-leather'],eye:['eye-n'],'eye-n':['eye-n'],'eye-f':['eye-f']}
  const jobs=shapes.filter(q=>Object.hasOwn(groups,q.id)&&q.filled).map(meta=>{
   const mask=projectMask(view,groups[meta.id].map(id=>meshes.get(id)),feature/16,3)
   let ring=contours(mask.cells,mask.width,mask.height,mask.box,mask.cell).sort((l,r)=>Math.abs(area(r))-Math.abs(area(l)))[0]
   for(let i=0;i<4;i++)ring=ring.map((p,j)=>{const l=ring[(j+ring.length-1)%ring.length],r=ring[(j+1)%ring.length];return {x:(l.x+2*p.x+r.x)/4,y:(l.y+2*p.y+r.y)/4}})
   if(meta.id.startsWith('eye'))meta.region='eyes'
   if(meta.id==='head'&&!shapes.some(q=>q.id.startsWith('eye'))){
    meta.regions=[...new Set([...(meta.regions||[meta.region]),'eyes'])];delete meta.region
   }
   return {meta,target:simplifyRing(ring,feature*.17)}
  })
  const drawn=inverse(a,scale,view,jobs)
  jobs.forEach(({meta},i)=>{if(drawn[i].residualM>1e-5)throw Error(meta.id+' '+view);meta.points=drawn[i].points.map(p=>[round(p.x),round(p.y)])})
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
