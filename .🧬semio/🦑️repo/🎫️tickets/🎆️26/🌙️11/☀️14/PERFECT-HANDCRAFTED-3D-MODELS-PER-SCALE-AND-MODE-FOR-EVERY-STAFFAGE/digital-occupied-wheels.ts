/** 🛞 Fit a rider's crossed wheels, wheelchair pushrims, and the rollator's five moulded webs at their actual axles. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,ride=a.id==='cyclist',chair=a.id==='wheelchair-user',walker=a.id==='person-with-rollator'
if(!ride&&!chair&&!walker)throw Error('Unknown occupied wheel set')
const fine=structuredClone(a.volumes[50]),round=n=>+n.toFixed(7),tau=2*Math.PI,snap=v=>v.toArray().map(round),V=(x,y,z)=>new Vector3(x,y,z)
const wheelBones=new Set(ride?['front-wheel','rear-wheel']:chair?['rear-wheel']:['wheel-front','wheel-rear'])
const riderControl=s=>ride&&/^(handlebar(?:-[lr])?|grip-[lr]|bar-clamp|pedal-(?:spindle-)?[lr])$/.test(s.id)
const own=s=>wheelBones.has(s.bone)&&s.id!=='rear-cog'||riderControl(s)
const forms=fine.body.filter(own).filter(s=>!s.id.includes('spoke')||walker)
function line(bone,id,p0,p1,count,steel,d){
 const a=new Vector3(...p0),b=new Vector3(...p1),dir=b.clone().sub(a).normalize(),K=d===50?6:d===100||ride&&d===200?5:4
 const cap=a.clone().addScaledVector(dir,-.0028);cap.z+=Math.sign(a.z-b.z)*.0013
 const rings=[[cap,.0017],[a,.00155],[a.clone().addScaledVector(dir,.004),.001],[b.clone().addScaledVector(dir,-.010),.001],[b.clone().addScaledVector(dir,-.009),.00165],[b,.00175]]
 const verts=[],tri=[]
 let prior=V(1,0,0)
 for(let k=0;k<rings.length;k++){
  const [c,r]=rings[k],axis=rings[Math.min(k+1,rings.length-1)][0].clone().sub(rings[Math.max(k-1,0)][0]).normalize()
  let u=prior.clone().addScaledVector(axis,-axis.dot(prior));if(u.lengthSq()<1e-8)u=V(0,1,0).addScaledVector(axis,-axis.y);u.normalize()
  const w=axis.clone().cross(u);prior=u
  for(let j=0;j<K;j++)verts.push(snap(c.clone().addScaledVector(u,r*Math.cos(tau*j/K)).addScaledVector(w,r*Math.sin(tau*j/K))))
  if(k)for(let j=0;j<K;j++){const m=(k-1)*K+j,n=(k-1)*K+(j+1)%K;tri.push([m,n,n+K],[m,n+K,m+K])}
 }
 for(let j=1;j<K-1;j++){const end=(rings.length-1)*K;tri.push([0,j+1,j],[end,end+j,end+j+1])}
 return {id,bone,kind:'polyhedron',material:steel,role:'detail',tone:.67,slender:true,texture:true,vertices:verts,triangles:tri,repeat:{count,rotationAxis:[0,0,1],rotationRad:tau/count,origin:[0,0,0]}}
}
function rimProfile(q,d){
 const s=structuredClone(q)
 if(ride&&s.id.endsWith('-hub')){
  const rear=s.bone==='rear-wheel',axle=rear?.068:.054,flange=rear?.040:.032,R=rear?.032:.030,r=rear?.016:.013
  const left=[[-axle,0],[-axle,.006],[-flange-.008,.006],[-flange-.007,r],[-flange-.002,r],[-flange-.002,R],[-flange+.002,R],[-flange+.002,r]]
  s.profile=[...left,...left.slice().reverse().map(([x,y])=>[-x,y])]
 }
 return s
}
function active(d){
 const out=forms.map(s=>rimProfile(s,d))
 if(walker)return out
 const polar=(r,angle,z)=>[round(r*Math.cos(angle)),round(r*Math.sin(angle)),z]
 if(ride){
  for(const end of ['front','rear'])for(let n=0;n<4;n++){
   const rear=end==='rear',phase=n*tau/36,side=n%2?1:-1,dir=n<2?1:-1
   out.push(line(end+'-wheel',end+'-spoke-'+n,polar(rear?.028:.026,phase+dir*tau/6,side*(rear?.040:.032)),polar(.294,phase,0),9,'steel',d))
  }
 }else{
  for(const [side,sign] of [['l',1],['r',-1]])for(let n=0;n<4;n++){
   const start=[0,30,15,45][n]*Math.PI/180,end=start+(n%2?-1:1)*tau/6,inside=n<2
   out.push(line('rear-wheel','spokes-'+(inside?'in':'out')+'-'+(n%2?'trail':'lead')+'-'+side,polar(.02465,start,sign*(inside?.25:.298)),polar(.255,end,sign*.28),6,'aluminium',d))
  }
 }
 return out
}
function path(r,start=0,end=tau,steps=40){
 return Array.from({length:steps+1},(_,i)=>{const ang=start+i*(end-start)/steps;return [round(r*Math.cos(ang)),round(r*Math.sin(ang))]})
}
const contour=pts=>'M '+pts.map(p=>p.join(' ')).join(' L ')+' Z'
function paper(scale){
 const geo=a.representations[scale].geometry
 if(!walker){
  if(scale!==500)return
  const base=a.representations[200].geometry
  for(const [view,items] of Object.entries(geo)){
   if(ride&&view==='elevation'){
    const tyre=s=>/^((rear|front)-tyre|(rear|front)-hub)$/.test(s.id),contact=s=>/^(foot|toe|pedal|hand)-[lr]$/.test(s.bone)
    const actual=base[view].filter(s=>tyre(s)||contact(s)).map(s=>({...structuredClone(s),penMm:/^pedal-[lr]$/.test(s.id)?.18:.16}))
    for(let k=items.length-1;k>=0;k--)if(tyre(items[k])||contact(items[k]))items.splice(k,1)
    items.push(...actual)
   }
   const samples=base[view].filter(s=>s.id.includes('spokes')&&(ride?/-[0235]$/:/-[1245]$/).test(s.id))
   const tags=new Set(samples.map(s=>'wheel-ink-'+s.id))
   geo[view]=items.filter(s=>!tags.has(s.id)).concat(samples.map(s=>({...structuredClone(s),id:'wheel-ink-'+s.id,role:'texture',penMm:.16})))
  }
  return
 }
 if(scale===50)return
 for(const [view,items] of Object.entries(geo)){
  const replacement=[]
  if(scale===200)for(const shoe of items.filter(s=>s.id==='shoe-l'&&s.kind==='poly')){
   const initial={elevation:.207,front:.0922,plan:.1983}[view],width=Math.max(...shoe.points.map(p=>p[0]))-Math.min(...shoe.points.map(p=>p[0]))
   if(Math.abs(width-initial)<.0001){const C=shoe.points.reduce((c,p)=>[c[0]+p[0]/shoe.points.length,c[1]+p[1]/shoe.points.length],[0,0]);shoe.points=shoe.points.map(p=>p.map((v,k)=>round(C[k]+(v-C[k])*1.012)))}
  }
  for(const end of ['rear','front'])for(const side of ['l','r']){
   const tag=end+'-'+side,bone='wheel-'+end,depth=(side==='l'?1:-1)*(end==='front'?.235:.283),z=items.find(s=>s.bone===bone&&s.id.endsWith(tag))?.z??2.1
   const style={bone,role:'body',z,filled:true,kind:'path',penMm:scale===500?.23:.18,solid:{kind:'extruded',sweep:'w',z0:round(depth-.018),z1:round(depth+.018)}}
   replacement.push({...style,id:'tyre-'+tag,tone:.10,material:'pu-tyre',region:'tyres',d:contour(path(.1))+' '+contour(path(.076,tau,0))})
   const windows=[]
   for(let k=0;k<5;k++){
    const A=k*tau/5,B=(k+1)*tau/5
    windows.push(contour([...path(.060,B-.11,A+.11,6),...path(.021,A+.31,B-.31,4)]))
   }
   replacement.push({...style,id:'moulded-'+tag,z:z+.002,tone:.62,material:'nylon-grey',region:'frame',d:contour(path(.077))+' '+windows.join(' ')})
  }
  geo[view]=items.filter(s=>!wheelBones.has(s.bone)).concat(replacement)
 }
}
const notes=ride?['Thirty-six crossed wires per wheel run in both directions on both flanges. Each swaged spoke seats at the steel rim and in the narrowed hub barrel’s flange.','Je Rad laufen sechsunddreißig gekreuzte Speichen in beiden Richtungen aus beiden Flanschen. Die verdickten Speichenenden sitzen in der Stahlfelge und am Flansch des schmalen Nabenkörpers.']:chair?['Both hand-driven wheels retain their separate tyre, rim, dished twenty-four-spoke assembly, hub and handrim with its connecting tabs.','Beide Greifräder behalten getrennte Reifen, Felgen, vierundzwanzig gekreuzte Speichen, Naben sowie Greifreifen mit Befestigungslaschen.']:['The four moulded wheels retain their polyurethane tread, separate nylon hub and rim and five integral webs with open spaces between them.','Alle vier Formräder behalten ihre Polyurethan-Lauffläche, separate Nylonnaben und Felgen sowie fünf angeformte Stege mit offenen Zwischenräumen.']
for(const [band,vol] of Object.entries(a.volumes)){
 const d=+band;paper(d)
 if(ride&&d!==50){
  const surface=s=>/^(?:hand|toe|foot)-[lr]$/.test(s.bone),named=new Set(vol.body.filter(surface).map(s=>s.id))
  vol.body=vol.body.filter(s=>!surface(s)).concat(fine.body.filter(surface).map(s=>structuredClone(s)))
  vol.thermal=vol.thermal.filter(s=>!named.has(s.of)&&!surface(s)).concat(fine.thermal.filter(s=>fine.body.some(b=>surface(b)&&s.of===b.id)).map(s=>structuredClone(s)))
 }
 if(chair&&d!==50){
  const limb=s=>/^(?:hand|forearm)-[lr]$/.test(s.bone),hand=s=>/^hand-[lr]$/.test(s.member),names=new Set(vol.body.filter(limb).map(s=>s.id))
  const external=fine.body.filter(limb),ids=new Set(external.map(s=>s.id))
  vol.body=vol.body.filter(s=>!limb(s)).concat(structuredClone(external))
  vol.thermal=vol.thermal.filter(s=>!names.has(s.of)&&!limb(s)).concat(structuredClone(fine.thermal.filter(s=>ids.has(s.of))))
  vol.structure=vol.structure.filter(s=>!hand(s)).concat(structuredClone(fine.structure.filter(hand)))
 }
 const old=new Set(vol.body.filter(own).map(s=>s.id)),newForms=active(d)
 const region=s=>s.material.includes('tyre')||s.id.includes('tyre')?'tyres':ride?'bike-metal':chair?'frame-metal':'frame'
 vol.body=vol.body.filter(s=>!old.has(s.id)).concat(newForms)
 vol.thermal=vol.thermal.filter(s=>!old.has(s.of)&&!(own(s)&&s.id.includes('spoke')))
 vol.structure=vol.structure.filter(s=>!old.has(s.of)&&!(wheelBones.has(s.member)&&s.part==='member'&&s.id.includes('spoke')))
 for(const s of newForms){
  vol.thermal.push({id:'th-'+s.id,of:s.id,region:region(s)})
  vol.structure.push({id:(walker?'member-':'st-')+s.id,of:s.id,member:s.bone,part:'member'})
 }
 if(!vol.decisions.some(x=>x.en===notes[0]))vol.decisions.push({en:notes[0],de:notes[1]})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
