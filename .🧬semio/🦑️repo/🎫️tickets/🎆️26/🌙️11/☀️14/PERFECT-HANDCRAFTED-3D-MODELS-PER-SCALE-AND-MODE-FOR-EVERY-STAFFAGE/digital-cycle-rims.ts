/** 🚲 Build the J-bend spokes and bored nipple seats of both cycles and the electric scooter's cartridge pockets. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {draftCargo} from './digital-cycle-paper.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-cycle-wheel-case.json',import.meta.url),'utf8'))[a.id],v50=structuredClone(a.volumes['50']),cargo=a.id==='cargo-bike',kick=a.id==='electric-kick-scooter'
const snap=x=>x.toArray().map(n=>+n.toFixed(7)),V=p=>new Vector3(...p)
const angle=Math.PI*2,metal=kick?'anodised':cargo?'alloy-rim':'alloy',field=kick?'rear-rim':'rims-spokes'
function ringShell(rings,axis,n,closed){
 const vertices=[],triangles=[]
 let prior
 for(const [j,[center,r,flats]] of rings.entries()){
  const along=closed?axis:rings[Math.min(j+1,rings.length-1)][0].clone().sub(rings[Math.max(j-1,0)][0]).normalize()
  const reference=prior??(Math.abs(along.x)<.8?new Vector3(1,0,0):new Vector3(0,1,0))
  const u=reference.clone().addScaledVector(along,-reference.dot(along)).normalize(),v=along.clone().cross(u);prior=u
  for(let i=0;i<n;i++){
   const phi=angle*i/n,rr=flats?r/Math.max(Math.abs(Math.cos(phi)),Math.abs(Math.sin(phi))):r
   vertices.push(snap(center.clone().addScaledVector(u,Math.cos(phi)*rr).addScaledVector(v,Math.sin(phi)*rr)))
  }
 }
 const bands=closed?rings.length:rings.length-1
 for(let j=0;j<bands;j++)for(let i=0;i<n;i++){
  const k=(j+1)%rings.length*n,A=j*n+i,B=j*n+(i+1)%n,C=k+(i+1)%n,D=k+i;triangles.push([A,B,C],[A,C,D])
 }
 if(!closed)for(let i=1;i<n-1;i++){const last=(rings.length-1)*n;triangles.push([0,i+1,i],[last,last+i,last+i+1])}
 return {vertices,triangles}
}
function lacing(q,scale){
 const A=V(q.p0),B=V(q.p1),headSide=Math.sign(A.z)*(q.id.endsWith('1')||q.id.endsWith('4')?1:-1),head=A.clone().add(new Vector3(0,0,-headSide*f.jBendM)),elbow=A.clone().add(new Vector3(0,0,headSide*.0013)),axis=B.clone().sub(elbow).normalize(),distance=B.distanceTo(elbow)
 const pos=t=>elbow.clone().addScaledVector(axis,t),normal=new Vector3(0,0,headSide)
 const entry=elbow.clone().addScaledVector(normal,-.0033),exit=pos(.0033)
 const bend=[0,.25,.5,.75,1].map(t=>[entry.clone().multiplyScalar((1-t)**2).addScaledVector(elbow,2*t*(1-t)).addScaledVector(exit,t*t),.001])
 const rings=[[head,.00185],[head.clone().addScaledVector(normal,.0006),.0019],...bend,[pos(.018),.001],[pos(.026),.0009],[pos(distance-.026),.0009],[pos(distance-.017),.001],[B,.001]]
 const base={kind:'polyhedron',bone:q.bone,material:'spoke-wire',tone:.64,role:'detail',slender:true,texture:true,repeat:q.repeat}
 const simplified=scale<=100?rings:rings.filter((_,i)=>i!==3&&i!==5&&(scale<500||i!==7))
 const wire={...base,id:q.id,...ringShell(simplified,axis,scale===50?8:scale===100?7:scale===200?6:5,false)}
 const P=t=>B.clone().addScaledVector(axis,t),length=f.nippleM
 const nipple={...base,id:q.id.replace('spoke-','nipple-'),material:metal,tone:.54,...ringShell([[P(-length+.002),.00108],[P(-length+.002),.00175,true],[P(-.003),.00175,true],[P(-.001),.0023],[P(.0003),.00315],[P(.002),.0029],[P(.002),.00108]].filter((_,i)=>scale<=100||i!==3),axis,8,true)}
 delete nipple.slender
 return [wire,nipple]
}
const chassis=q=>{
 const id=q.id.replace(/^(th-|st-)/,'')
 return cargo?/^(beam|head-tube-[fr]|down-tube|seat-tube|chainstay-[lr]|seatstay-[lr]|chainstay-bridge|carrier(?:-.+)?)$/.test(id):!kick&&/^rack[-]/.test(id)
}
const wheel=q=>q.bone?.startsWith('wheel-')
const chosen=q=>wheel(q)||chassis(q)
if(!kick){
 a.materials.push({id:'spoke-wire',title:{en:'Cold-drawn stainless spoke wire',de:'Kaltgezogener rostfreier Speichendraht'},densityKgM3:7900,conductivityWmK:16,solarAbsorptivity:.38,emissivity:.28,source:'authored-estimate'})
 if(cargo){a.materials.find(q=>q.id===metal).title={en:'Aluminium wheel rims and nipples',de:'Aluminiumfelgen und Speichennippel'}
  a.thermal.regions.push({id:field,title:{en:'Rims, hubs and tension spokes',de:'Felgen, Naben und Zugspeichen'},kind:'surface',model:'passive',material:metal,areaM2:f.wheelMetalAreaM2,areaBasis:'total',characteristicLengthM:.55,geometry:'cylinder',skyViewFactor:.38,layers:[],heatCapacityJM2K:7400,source:'authored-estimate'})
  for(const p of a.poses)p.thermal.regions[field]={exposedFraction:f.wheelMetalExposedFraction[p.id],sunlitFraction:.4}
 }
}
for(const [den,v] of Object.entries(a.volumes)){
 const k=Number(den),old=new Set(v.body.filter(chosen).map(s=>s.id)),master=v50.body.filter(chosen)
 const affected=q=>old.has(q.of)||wheel(q)||chassis(q)
 v.body=v.body.filter(q=>!chosen(q))
 v.thermal=v.thermal.filter(q=>!affected(q))
 v.structure=v.structure.filter(q=>!affected(q)&&!q.member?.startsWith('wheel-'))
 v.structure.push(...v50.structure.filter(q=>q.member?.startsWith('wheel-')&&!q.of).map(q=>structuredClone(q)))
 for(const source of master){
  const q=structuredClone(source)
  let stock=q.id.startsWith('spoke-')?lacing(q,k):[q]
  if(kick&&q.id==='hub-r'){
   q.profile=[[0,.016],[.009,.016],[.009,.009],[.071,.009],[.071,.016],[.08,.016],[.08,.02],[0,.02],[0,.016]]
   for(const [side,z] of [['l',-.04],['r',.031]])stock.push({id:'bearing-r-'+side,kind:'revolve',bone:'wheel-r',material:'steel-disc',role:'detail',tone:.41,origin:[0,0,z],axis:[0,0,1],profile:[[0,.007],[.009,.007],[.009,.016],[0,.016],[0,.007]]})
  }
  for(const s of stock){
   v.body.push(s)
   const original=v50.thermal.find(t=>t.of===s.id)
   const fallback=wheel(s)?s.id.startsWith('rotor-')?'brakes':s.id==='disc-r'?'brake-disc':s.id.startsWith('tyre-')?'tyres':s.id==='motor-f'?'hub-motor':field:cargo?'frame':'frame-tubes'
   if(!(k>50&&chassis(s)&&s.id==='carrier-tail'))v.thermal.push(original?structuredClone(original):{id:'th-'+s.id,of:s.id,region:fallback})
   const structural=v50.structure.find(t=>t.of===s.id)
   if(structural)v.structure.push(structuredClone(structural))
   else if(wheel(s)&&!s.id.startsWith('nut-')||chassis(s)&&s.role!=='texture')v.structure.push({id:'st-'+s.id,of:s.id,member:s.bone,part:s.id.startsWith('bearing-')?'joint':'member'})
  }
 }
 v.decisions.push(kick?{en:'The rear hub runs on two 14 × 32 × 9 mm cartridges in shouldered bearing seats. Its five windows and the independent front hub motor remain visible.',de:'Die Hinterradnabe läuft auf zwei Lagern 14 × 32 × 9 mm in abgesetzten Sitzen. Die fünf Fenster und der separate Vorderradnabenmotor bleiben sichtbar.'}:{en:'Each wheel carries 36 crossed tension spokes from the two hub flanges. The bent ends and headed nipples sit on the actual flange and rim. The carrier rails and forked stays keep their separate sections.',de:'Jedes Rad trägt 36 gekreuzte Zugspeichen zwischen den beiden Nabenflanschen und der Felge. Die gekröpften Enden und Speichennippel sitzen an Flansch und Felge. Die Gepäckträgerrohre und gegabelten Streben behalten ihre getrennten Querschnitte.'})
}
if(cargo)for(const rep of Object.values(a.representations))if(rep.status==='authored')for(const shapes of Object.values(rep.geometry))for(const q of shapes){
 if(!q.bone?.startsWith('wheel-'))continue
 if(q.id.includes('spoke')||q.id.includes('rim')){delete q.regions;q.region=field}
 else if(q.id.includes('wheel')||q.id.includes('hub')){const before=q.regions??(q.region?[q.region]:[]);delete q.region;q.regions=[...new Set([...before,field])]}
}
if(cargo)draftCargo(a,v50)
writeFileSync(process.argv[3],JSON.stringify(doc))
