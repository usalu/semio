/** 🚧 Form the open shoes, independently sprung rollers and pierced idler yokes in the two excavator tracks. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=structuredClone(a.volumes['50']),round=x=>+x.toFixed(6)
const belongs=q=>q.bone==='undercarriage'&&q.id!=='carbody'&&q.id!=='turret'
const thByBody=new Map(fine.thermal.filter(q=>q.of).map(q=>[q.of,q]))
const stock=fine.body.filter(belongs).map(q=>{
 if(!q.id.startsWith('idler-yoke-'))return q
 const z=q.transform.translation[2],w=q.size[2]/2,eyeX=1.825,ring=(r,n)=>Array.from({length:n},(_,i)=>[round(eyeX+r*Math.cos(i*2*Math.PI/n)),round(.45+r*Math.sin(i*2*Math.PI/n))])
 const nose=Array.from({length:11},(_,i)=>{const theta=-Math.PI/2+Math.PI*i/10;return [round(eyeX+.088*Math.cos(theta)),round(.45+.088*Math.sin(theta))]})
 return {id:q.id,bone:q.bone,material:q.material,role:q.role,tone:q.tone,kind:'extrude',plane:'xy',profile:[[1.34,.39],[1.43,.368],...nose,[1.43,.532],[1.34,.51]],holes:[ring(.037,20)],from:round(z-w),to:round(z+w),repeat:q.repeat}
})
const left={count:2,translation:[0,0,-2.38]},gear='undercarriage'
stock.push({id:'idler-cross-pin',kind:'revolve',bone:gear,material:'track-steel',role:'detail',tone:.48,origin:[1.825,.45,1.04],axis:[0,0,1],profile:[[0,.035],[.295,.035],[.296,.043],[.303,.043],[.305,.039]],repeat:left})
for(const [id,origin,profile] of [
 ['track-adjuster-barrel',[.995,.45,1.19],[[0,.066],[.012,.071],[.21,.071],[.22,.066],[.22,.060],[.25,.060],[.26,.054],[.26,.035],[.243,.035],[.243,.054],[.02,.054],[.02,.061],[0,.061],[0,.066]]],
 ['track-adjuster-rod',[1.231,.45,1.19],[[0,.034],[.128,.034],[.132,.030]]]
])stock.push({id,kind:'revolve',bone:gear,material:'track-steel',role:'detail',tone:.49,origin,axis:[1,0,0],profile,repeat:left})
for(const [key,v] of Object.entries(a.volumes)){
 const old=new Set(v.body.filter(belongs).map(q=>q.id)),scale=+key
 const chosen=stock.filter(q=>scale<=100||!q.id.startsWith('grouser-')||q.id.startsWith('grouser-bottom-'))
 const oldThermal=new Set(v.thermal.filter(q=>q.region==='tracks'&&(old.has(q.of)||q.bone===gear&&!/carbody|turret/.test(q.id))).map(q=>q.id))
 v.body=v.body.filter(q=>!old.has(q.id)).concat(structuredClone(chosen))
 v.thermal=v.thermal.filter(q=>!oldThermal.has(q.id))
 v.structure=v.structure.filter(q=>q.part!=='member'||!old.has(q.of)&&!/^track-beam|^member-track/.test(q.id))
 for(const q of chosen){
  const th=thByBody.get(q.id)
  if(th)v.thermal.push(structuredClone(th))
  else if(!q.id.startsWith('grouser')&&!q.id.startsWith('sprocket-teeth'))v.thermal.push({id:'th-'+q.id,of:q.id,region:'tracks'})
  if(/^(track-(shoes|rail|frame|adjuster)|roller(-shaft)?-[lr]$|carrier-roller|idler($|-yoke|-cross)|sprocket$)/.test(q.id))
   v.structure.push({id:'st-'+q.id,of:q.id,member:gear,part:q.id.includes('pin')||q.id.includes('shaft')?'joint':'member'})
 }
 v.decisions.push({en:'The open track loops retain eight lower rollers per side, the return rollers, idlers and drive sprockets. Each idler pin passes through two pierced yoke plates and the adjuster rod seats against the yoke.',de:'Die offenen Ketten behalten je Seite acht Laufrollen, die Stützrollen sowie Leit- und Antriebsrad. Die Leitradachse durchläuft zwei gebohrte Gabelbleche. Die Spannstange stützt sich an der Gabel ab.'})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
