/** 🏗️ Model the rail contact, wheel flanges, reeving grooves and open truss around the crane carriage. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=a.volumes['50']
const supports=new Set(['mast','jib','counter-jib','rope-1','rope-2','rope-3','rope-4']),drawn=new Set([...supports,'trolley'])
const plate=fine.body.find(s=>s.id==='trolley-cheeks'),base=fine.body.filter(s=>s.bone==='trolley'&&s!==plate).map(q=>structuredClone(q))
const round=x=>+x.toFixed(5)
for(const wheel of base.filter(s=>s.id.startsWith('trolley-wheel'))){
 const sign=wheel.id.includes('-l-')?1:-1
 wheel.axis=[0,0,sign];wheel.origin[2]=.590*sign
 wheel.profile=[[0,.119],[.004,.125],[.068,.125],[.072,.128],[.074,.14],[.083,.14],[.086,.136]]
}
for(const wheel of base.filter(s=>s.id.startsWith('trolley-sheave'))){
 wheel.profile=[[0,.195],[.004,.2],[.017,.2],[.021,.196],[.024,.191],[.027,.1887],[.03,.188],[.033,.1887],[.036,.191],[.039,.196],[.043,.2],[.056,.2],[.06,.195]]
}
for(const sign of [-1,1]){
 const s=structuredClone(plate);s.id='trolley-cheek-'+(sign<0?'r':'l');s.size[2]=.016;s.transform.translation[2]=sign*.048;base.push(s)
}
for(const s of base.filter(q=>q.id.startsWith('trolley-wheel-')||q.id.startsWith('trolley-sheave-'))){
 const runner=s.id.includes('-wheel-'),sign=Math.sign(s.axis[2]),rear=s.origin[0]
 const origin=runner?[rear,.255,round(sign*.579)]:[rear,-.45,-.065]
 const end=runner?.158:.13
 base.push({id:s.id+'-axle',kind:'revolve',bone:'trolley',material:'steel-s355',role:'detail',tone:.57,axis:[0,0,sign],origin,profile:[[0,.029],[end,.029]]})
}
function replace(old,stock,predicate){const i=old.findIndex(predicate),out=old.filter(x=>!predicate(x));out.splice(i<0?out.length:i,0,...structuredClone(stock));return out}
const mid=a.volumes['200'],low=a.volumes['500']
const frameIds=vol=>new Set(vol.body.filter(q=>supports.has(q.bone)).map(s=>s.id)),older=frameIds(low),newer=frameIds(mid)
for(const key of ['body','thermal','structure']){
 const own=(s,ids)=>supports.has(s.bone)||ids.has(s.of)
 low[key]=replace(low[key],mid[key].filter(s=>own(s,newer)),s=>own(s,older))
}
a.representations['500'].pens.detail=.13
for(const [view,shapes] of Object.entries(a.representations['500'].geometry)){
 const stock=a.representations['200'].geometry[view].filter(s=>drawn.has(s.bone)).map(s=>{
  const q=structuredClone(s)
  if(q.penMm!==undefined)q.penMm=round(Math.max(.13,q.penMm*.4))
  if(q.id.startsWith('trolley-frame')||q.id==='trolley-sheave')q.penMm=.175
  return q
 })
 a.representations['500'].geometry[view]=replace(shapes,stock,s=>drawn.has(s.bone))
}
for(const [scale,v] of Object.entries(a.volumes)){
 const oldIds=new Set(v.body.filter(q=>q.bone==='trolley').map(q=>q.id))
 v.body=replace(v.body,base,q=>q.bone==='trolley')
 v.thermal=replace(v.thermal,base.map(q=>({id:'th-'+q.id,of:q.id,region:'trolley'})),q=>q.region==='trolley')
 v.structure=replace(v.structure,base.map(q=>({id:'st-'+q.id,of:q.id,member:'trolley',part:/-wheel-|-axle$/.test(q.id)?'joint':'member'})),q=>oldIds.has(q.of)||q.member==='trolley'&&q.part!=='mass')
 const text={en:'The carriage bears on four flanged steel wheels on the two bottom rails. Two separate cheek plates support the grooved rope sheaves on through pins.',de:'Die Laufkatze rollt auf vier Stahlrädern mit Spurkranz auf den beiden Untergurtschienen. Zwei getrennte Wangen lagern die gerillten Seilscheiben auf durchgehenden Bolzen.'}
 if(scale==='500'){
  v.decisions[0]={en:'The four mast chords and the open bracing on the mast, jib and counter-jib remain individually drawn on this scale. The slewing head, ballast, cab, load and rigging use their coarse forms.',de:'Die vier Mastgurte und die offenen Fachwerke von Mast, Ausleger und Gegenausleger bleiben in diesem Maßstab einzeln gezeichnet. Drehbühne, Ballast, Kabine, Last und Seilführung behalten ihre vereinfachten Formen.'}
 }
 v.decisions.push(text)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
