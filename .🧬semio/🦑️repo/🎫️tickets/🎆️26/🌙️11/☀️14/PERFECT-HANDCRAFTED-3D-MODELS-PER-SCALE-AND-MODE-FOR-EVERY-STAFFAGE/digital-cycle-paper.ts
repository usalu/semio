/** 🚲 Draft the coarse cargo frame, rear rack and paired tyre outlines at their measured centres. */
export function draftCargo(a,base){
 const v=a.volumes['500'],rep=a.representations['500'],ink=rep.geometry.elevation
 for(const k of Object.keys(a.volumes)){
  const vol=a.volumes[k],spindle=base.structure.find(q=>q.id==='steerer-r'),old=vol.body.findIndex(s=>s.id===spindle.id)
  if(old>=0)vol.body.splice(old,1)
  const {member,part,...geometry}=spindle
  vol.body.push({...structuredClone(geometry),role:'detail',texture:true})
  vol.structure=vol.structure.filter(s=>s.id!==spindle.id)
  vol.structure.push({id:spindle.id,of:spindle.id,member,part})
  if(!vol.thermal.some(q=>q.of===spindle.id))vol.thermal.push({id:'th-steerer-r',of:spindle.id,region:'frame'})
  if(+k<200)continue
  for(const id of ['drum-r','riser','stem','bar-clamp']){
   const q=base.body.find(s=>s.id===id),prev=vol.body.findIndex(s=>s.id===id);if(prev>=0)vol.body.splice(prev,1)
   vol.body.push(structuredClone(q))
   if(!vol.thermal.some(s=>s.of===id))vol.thermal.push({id:'th-'+id,of:id,region:'frame'})
  }
 }
 for(const q of v.body){
  if(/^tyre-|^carrier-(rail|leg)-|^seatstay-/.test(q.id))q.slender=true
 }
 for(const end of ['r','f']){
  const q=ink.find(s=>s.id==='wheel-'+end),r=end==='r'?.305:.23
  delete q.d;Object.assign(q,{kind:'ellipse',cx:0,cy:0,rx:r,ry:r,filled:false,penMm:.15})
 }
 const carrier=ink.find(q=>q.id==='carrier')
 Object.assign(carrier,{closed:false,filled:false,points:[[-.30,.748],[.235,.748]],penMm:.13})
 const leg=ink.find(q=>q.id==='carrier-leg-r')
 leg.points=[[-.04,.748],[-.004,.346]]
 ink.push({...leg,id:'carrier-leg-rear-r',points:[[-.24,.748],[-.004,.346]]})
 for(const q of ink.filter(q=>(q.bone==='frame'||q.id==='riser')&&q.kind==='member'&&q.section?.kind==='circle')){
  const {x,y}=q.from,to=q.to
  delete q.from;delete q.to;delete q.section
  Object.assign(q,{kind:'poly',closed:false,filled:false,points:[[x,y],[to.x,to.y]],penMm:.13})
 }
}
