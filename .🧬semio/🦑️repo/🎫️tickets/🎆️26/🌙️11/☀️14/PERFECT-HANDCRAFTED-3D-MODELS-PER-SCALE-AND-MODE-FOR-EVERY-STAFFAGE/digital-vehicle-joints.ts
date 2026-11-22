/** 🔩 Attach the two halves of the scooter clamp and carry each platform handrail into the structural scene. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,c=JSON.parse(readFileSync(new URL('./digital-vehicle-joint-cases.json',import.meta.url),'utf8'))[a.id]
const round=v=>+v.toFixed(7)
for(const [scale,v] of Object.entries(a.volumes)){
 if(a.id==='scissor-lift'){
  for(const q of v.body.filter(q=>['platform','deck-ext'].includes(q.bone)&&/^(?:ext-|front-|rear-)?(?:post-|top-rail)|^gate(?:-top|-stile)?$/.test(q.id))){
   if(!v.structure.some(s=>s.of===q.id))v.structure.push({id:'member-'+q.id,of:q.id,member:q.bone,part:'member'})
  }
  continue
 }
 const x=-.00529,y=.00759,r=c.saddleBoreM/2,split=.00075,n=scale==='50'?12:scale==='100'?10:8,h=c.clampWidthM/2,material='exhaust-steel'
 const add=(id,geometry,part='joint')=>{
  v.body.push({id,bone:'steer',material,role:'detail',texture:true,tone:.52,...geometry})
  v.structure.push({id:'st-'+id,of:id,member:'steer',part})
  v.thermal.push({id:'th-'+id,of:id,region:'steer-assembly'})
 }
 for(const side of [-1,1]){
  const theta=Math.acos(split/r),arc=Array.from({length:n+1},(_,i)=>{const t=-theta+2*theta*i/n;return [round(side*r*Math.cos(t)),round(r*Math.sin(t))]})
  const profile=[...arc,[side*split,.0265],[side*.013,.0265],[side*.02,.018],[side*.021,-.015],[side*.013,-.0265],[side*split,-.0265]]
  if(side<0)profile.reverse()
  add('bar-saddle-'+(side<0?'cap':'base'),{kind:'extrude',plane:'xy',profile,from:-h,to:h,transform:{translation:[x,y,0]}})
 }
 add('upper-column-socket',{kind:'revolve',origin:[.012,0,0],axis:[1,0,0],profile:[[0,.01995],[.037,.01995],[.037,.025],[.031,.027],[0,.027],[0,.01995]]})
 for(const sign of [-1,1])add('saddle-pinch-'+(sign<0?'lower':'upper'),{kind:'revolve',origin:[x-.016,y+sign*.021,0],axis:[1,0,0],profile:[[0,0],[0,.0042],[.0038,.0042],[.0038,.0024],[.030,.0024],[.030,0]]})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
