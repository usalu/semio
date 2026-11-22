/** 🛞 Fit the heater’s removable shaft, wheel hubs and bored mounting plates to the base pan. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
const f=JSON.parse(readFileSync(new URL('./digital-heater-wheel-case.json',import.meta.url),'utf8'))
const [x,y]=f.axisOriginM,steel='stainless-brushed',paint='powder-steel',rubber='plastic-black'
const r=n=>+n.toFixed(6)
const stock=(id,material,role,tone)=>({id,bone:'base',material,role,tone})
const turn=(id,z,profile,material,role,tone)=>({...stock(id,material,role,tone),kind:'revolve',axis:[0,0,1],origin:[x,y,z],profile:profile[0][1]>0?[...profile,profile[0]]:profile})
const gear=/^(axle|(?:tyre|hub|wheel|bracket|bush)-(?:left|right))$/
const decision={en:'Two 90 mm transport wheels sit 5 mm above the ground while the heater rests on its pan. Their bored hubs run on a 12 mm shaft. Two flanged bushes seat the shaft in pressed steel brackets attached to the pan.',de:'Die beiden Transporträder mit 90 mm Durchmesser haben 5 mm Bodenfreiheit, wenn der Heizpilz auf der Wanne steht. Ihre durchbohrten Naben laufen auf einer 12-mm-Achse. Zwei Bundbuchsen lagern die Achse in gepressten Stahlhaltern an der Wanne.'}
for(const [d,v] of Object.entries(a.volumes)){
 const n=32,rep=a.representations[d]
 v.body=v.body.filter(s=>!gear.test(s.id));v.thermal=v.thermal.filter(s=>!gear.test(s.of));v.structure=v.structure.filter(s=>!gear.test(s.of))
 const parts=[],sr=f.shaftRadiusM,bore=f.clearanceRadiusM,bush=f.bushOuterM,seat=f.tyreSeatM
 for(const [i,side] of ['left','right'].entries()){
  const z=f.wheelCenterZM[i],bz=f.bracketCenterZM[i],h=f.tyreWidthM/2,right=i===1
  const shoulder=.011,tyre=[[-h,seat],[-h,.040],[-shoulder,.045],[shoulder,.045],[h,.040],[h,seat]]
  parts.push(turn('tyre-'+side,z,tyre,rubber,'body',.23))
  const hub=[[-.014,bore],[-.014,.012],[-.011,.014],[-.011,seat],[.011,seat],[.011,.014],[.014,.012],[.014,bore]]
  parts.push(turn('hub-'+side,z,hub,steel,'detail',.62))
  const profile=[[.016,.185],[.016,.253],[.025,.272],[.042,.281],[.058,.281],[.070,.271],[.074,.249],[.060,.185]]
  const holes=[Array.from({length:n},(_,j)=>[r(x+bush*Math.cos(j*2*Math.PI/n)),r(y+bush*Math.sin(j*2*Math.PI/n))])]
  parts.push({...stock('bracket-'+side,paint,'detail',.51),kind:'extrude',profile,holes,plane:'xy',from:r(bz-f.bracketHalfWidthM),to:r(bz+f.bracketHalfWidthM)})
  const sleeve=[[-.004,bore],[-.004,bush],[.004,bush],[.004,.012],[.006,.012],[.006,bore]]
  parts.push(turn('bush-'+side,bz,right?sleeve.map(([a,b])=>[-a,b]).reverse():sleeve,steel,'detail',.66))
 }
 const axle=[[-.180,0],[-.180,.008],[-.179,.010],[-.176,.010],[-.176,sr],[.176,sr],[.176,.010],[.179,.010],[.180,.008],[.180,0]]
 parts.push(turn('axle',0,axle,steel,'detail',.48))
 for(const s of parts){
  v.body.push(s);v.thermal.push({id:'th-'+s.id,of:s.id,region:'base-housing'})
  v.structure.push({id:'st-'+s.id,of:s.id,member:'base',part:s.id.startsWith('bush-')||s.id.startsWith('hub-')?'joint':'member'})
 }
 if(!v.decisions.some(t=>t.en===decision.en))v.decisions.push(decision)
 const description=v.decisions[0]
 if(d==='100'){
  description.en=description.en.replace('each wheel one solid tyre on a slender axle','each wheel a tyre on a bored, shouldered hub').replace('Brackets, hubs, igniter','Igniter')
  description.de=description.de.replace('jedes Rad ein massiver Reifen auf schlanker Achse','jedes Rad ein Reifen auf einer abgesetzten, durchbohrten Nabe').replace('Halter, Naben, Zünder','Zünder')
 }
 if(d==='200'){
  description.en=description.en.replace('Wheels (30 mm wide behind the pan), axle, access panel','Access panel')
  description.de=description.de.replace('Räder (30 mm breit hinter der Wanne), Achse, Zugangsklappe','Zugangsklappe')
  rep.decisions.merged={en:'The side outline joins the transport wheel to the pan. The valve housing, emitter and cap share one head outline.',de:'Der Seitenumriss verbindet das Transportrad mit der Wanne. Ventilgehäuse, Strahler und Deckel teilen sich einen Kopfumriss.'}
 }
 if(d==='500'){
  description.en=description.en.split('Pan step, wheels')[0].trim();description.de=description.de.split('Wannenstufe, Räder')[0].trim()
  rep.decisions.omitted={en:'Finial, seams and individual hardware lines.',de:'Knauf, Fugen und Einzellinien der Beschläge.'}
 }
 const elev=rep.geometry.elevation,wheel=elev.find(s=>s.id==='wheel')
 if(wheel)wheel.tone=.23
 if(+d>=200){
  const base=elev.find(s=>s.id==='base'),stop=base.points.findIndex(p=>p[0]>.6&&p[1]>.20&&p[1]<.23&&p[0]<.710)
  if(stop<0)throw Error('Heater pan outline')
  const rear=d==='500'?.225:.21,foot=d==='500'?.225:.235
  base.points=[...base.points.slice(0,stop+1),[.073,rear],[.073,.221],[.083,.230],[.092,.244],[.095,.26],[.091,.278],[.081,.293],[.065,.302],[.05,.305],[.033,.301],[.018,.292],[.008,.276],[.005,.26],[.010,.239],[.017,foot],[0,foot]]
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
