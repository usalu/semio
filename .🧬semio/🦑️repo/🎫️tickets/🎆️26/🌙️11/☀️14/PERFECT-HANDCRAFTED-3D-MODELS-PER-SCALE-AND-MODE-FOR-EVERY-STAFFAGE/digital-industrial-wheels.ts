/** 🚚 Preserve the mixer duals, industrial solid tyres and scooter cast wheel and match their load paths. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=structuredClone(a.volumes['50']),truck=a.id==='concrete-mixer-truck',fork=a.id==='forklift',scoot=a.id==='motor-scooter'
if(!truck&&!fork&&!scoot)throw Error(a.id)
const wheel=q=>/^wheel-/.test(q.bone??''),stock=q=>wheel(q)&&(!truck||!/^(axle|differential)/.test(q.id)),master=fine.body.filter(stock)
const round=x=>+x.toFixed(6)
function nut(q){
 const [x,y,z]=q.origin,sgn=q.axis[2],rr=q.profile[0][1],h=q.profile.at(-1)[0],vertices=[],triangles=[]
 for(const [d,r] of [[0,rr*.88],[.0014,rr],[h-.002,rr],[h,rr*.84]])for(let j=0;j<6;j++){
  const b=sgn*2*Math.PI*j/6+Math.PI/6;vertices.push([round(x+r*Math.cos(b)),round(y+r*Math.sin(b)),round(z+sgn*d)])
 }
 for(let i=0;i<3;i++)for(let j=0;j<6;j++){const u=i*6+j,v=i*6+(j+1)%6;triangles.push([u,v,v+6],[u,v+6,u+6])}
 for(let i=1;i<5;i++)triangles.push([0,i+1,i],[18,18+i,19+i])
 return {id:q.id,kind:'polyhedron',bone:q.bone,material:q.material,role:q.role,tone:q.tone,vertices,triangles}
}
const names={en:truck?'The steering axles carry singles and the two driven axles carry separate inner and outer tyres. Each tyre, disc and rim joins the axle load path.':fork?'Four compact industrial tyres carry the truck through their steel hubs. The wheel retainers have six real wrench flats and a chamfered head.':'The cast wheels retain the five open windows, their hubs, separate tyres and front brake disc. The rear hub and front steering bearing stay on the original wheel axes.',de:truck?'Die Lenkachsen tragen Einzelreifen, die beiden Antriebsachsen getrennte innere und äußere Reifen. Reifen, Radscheibe und Felge leiten die Last zur Achse.':fork?'Vier kompakte Industriereifen tragen den Stapler über ihre Stahlnaben. Die Radmuttern haben sechs Schlüsselflächen und angefaste Köpfe.':'Die Gussräder behalten ihre fünf offenen Fenster, Naben, getrennten Reifen und die vordere Bremsscheibe. Hinterradnabe und vorderes Lenklager bleiben auf den ursprünglichen Radachsen.'}
for(const [k,v] of Object.entries(a.volumes)){
 const old=new Set(v.body.filter(stock).map(q=>q.id))
 const selected=master.filter(q=>!truck||+k<=100||!q.id.startsWith('wheel-nuts-')).map(q=>fork&&q.id.startsWith('nut-')?nut(q):structuredClone(q))
 const del=q=>old.has(q.of)||stock(q)
 v.body=v.body.filter(q=>!stock(q)).concat(selected)
 v.structure=v.structure.filter(q=>!(q.part==='member'&&(/^wheel-/.test(q.member))&&(del(q)||/^(member-(wheel|rim|tyre|hub)|hub-|rim-|m-tyre-|m-rim-|wheel-)/.test(q.id))))
 v.thermal=v.thermal.filter(q=>!del(q))
 for(const q of selected){
  let region=fine.thermal.find(s=>s.of===q.id)?.region
  if(!region)region=q.id.startsWith('tyre')?'tyres':truck?q.id.startsWith('brake-')?'brakes':'running-gear':fork?'chassis-paint':q.material==='cast-iron'?'brake-disc':'rims'
  v.thermal.push({id:'th-'+q.id,of:q.id,region})
  if(!/^(nut-|wheel-nuts-|brake-|disc-f$|cap-r$)/.test(q.id))v.structure.push({id:'wheel-member-'+q.id,of:q.id,member:q.bone,part:'member'})
 }
 if(scoot){
  const steer=q=>q.bone==='steer',original=new Set(v.body.filter(steer).map(q=>q.id)),own=new Set(fine.body.filter(steer).map(q=>q.id))
  const bound=q=>steer(q)||original.has(q.of)
  const steering=fine.body.filter(steer).map(q=>structuredClone(q))
  for(const grip of steering.filter(q=>q.id.startsWith('grip-'))){
   const profile=[[0,.01125],[.129,.01125],[.131,.013],[.131,.018],[.127,.0195],[.115,.0195],[.111,.0166]]
   for(let d=.103;d>.016;d-=.014)profile.push([round(d+.002),.0166],[round(d+.001),.0155],[round(d-.001),.0155],[round(d-.002),.0166])
   profile.push([.014,.0166],[.007,.019],[.002,.019],[0,.017],[0,.01125])
   grip.profile=profile
  }
  const bar={id:'steering-tube',kind:'revolve',bone:'steer',material:'exhaust-steel',role:'detail',tone:.43,origin:[-.00529,.00759,-.336],axis:[0,0,1],profile:[[0,.009],[.672,.009],[.672,.011],[0,.011],[0,.009]]}
  v.body=v.body.filter(q=>!steer(q)).concat(steering,bar)
  v.thermal=v.thermal.filter(q=>!bound(q)).concat(fine.thermal.filter(q=>(steer(q)||own.has(q.of))&&(!q.id.startsWith('th-grip-')||q.of)&&(k==='50'||q.of||q.region!=='steer-assembly')).map(q=>{
   if(q.of?.startsWith('grip-'))return {id:q.id,of:q.of,region:'handle-grips'}
   const mapped=structuredClone(q);if(k!=='50'&&mapped.region==='steer-assembly')delete mapped.layer
   return mapped
  }),{id:'th-steering-tube',of:bar.id,region:'steer-assembly'})
  v.structure=v.structure.filter(q=>!bound(q)&&q.member!=='steer').concat(fine.structure.filter(q=>q.id!=='handlebar'&&(steer(q)||own.has(q.of)||q.member==='steer')).map(q=>structuredClone(q)),{id:'handlebar',of:bar.id,member:'steer',part:'member'})
 }
 v.decisions.push(names)
}
if(scoot){
 a.thermal.regions.push({id:'handle-grips',title:{en:'Left grip and twist throttle',de:'Linker Griff und Drehgasgriff'},kind:'surface',model:'passive',material:'rubber',areaBasis:'total',areaM2:0.0503203,characteristicLengthM:.131,geometry:'cylinder',skyViewFactor:.46,layers:[],heatCapacityJM2K:8310,source:'authored-estimate'})
 for(const p of a.poses)p.thermal.regions['handle-grips']={exposedFraction:0.594113,sunlitFraction:.45}
 for(const rep of Object.values(a.representations))if(rep.status==='authored')for(const shapes of Object.values(rep.geometry))for(const q of shapes){
  if(q.id.includes('grip')&&q.region==='steer-assembly')q.region='handle-grips'
  if((q.id==='bar'||q.id==='headset'||q.id.endsWith('-bar')||q.id.endsWith('-headset'))&&q.regions?.includes('steer-assembly'))q.regions.push('handle-grips')
 }
}
if(fork)a.thermal.regions.find(r=>r.id==='chassis-paint').areaM2=14.921
writeFileSync(process.argv[3],JSON.stringify(doc))
