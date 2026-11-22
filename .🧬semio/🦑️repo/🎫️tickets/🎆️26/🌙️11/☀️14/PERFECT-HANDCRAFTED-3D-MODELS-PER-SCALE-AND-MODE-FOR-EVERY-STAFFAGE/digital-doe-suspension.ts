/** 🐐 Suspend the two gland halves and teats from a common median support in the inguinal region. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
import {tessellateSolid} from '../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-doe-suspension.json',import.meta.url),'utf8'))
const id='mammary-suspension',region='mammary-skin',r=x=>+x.toFixed(8),V=p=>new Vector3(p.x,p.y,p.z)
const pelvis=a.rig.bones.find(b=>b.id==='pelvis'),fr=evaluateDrawing(a,{kind:'pose',pose:'standing-alert'}).bones.pelvis.frame
const local=new Vector3(...f.attachmentM).sub(V(fr.o)),x=r(local.dot(V(fr.u))),y=r(local.dot(V(fr.v))),angle=-90-Math.atan2(fr.u.y,fr.u.x)*180/Math.PI
a.materials.push({id:'suspensory-tissue',title:{en:'Suspensory connective tissue',de:'Bindegewebe der Euteraufhängung'},emissivity:.98,solarAbsorptivity:.58,densityKgM3:1050,source:'authored-estimate'})
const old=pelvis.mass.kg,m=f.massKg,theta=angle*Math.PI/180
const cx=x+Math.cos(theta)*f.comM[0]-Math.sin(theta)*f.comM[1],cy=y+Math.sin(theta)*f.comM[0]+Math.cos(theta)*f.comM[1]
pelvis.mass.comX=r((old*pelvis.mass.comX-m*cx)/(old-m));pelvis.mass.comY=r((old*pelvis.mass.comY-m*cy)/(old-m));pelvis.mass.kg=r(old-m)
a.rig.bones.push({id,parent:'pelvis',x,y,angleDeg:r(angle),lengthM:f.lengthM,title:{en:'Mammary suspensory apparatus',de:'Euteraufhängung'},joint:{type:'revolute',minDeg:-48,maxDeg:10},mass:{kg:m,comX:f.comM[0],comY:f.comM[1],distribution:'uniform',source:'authored-estimate'}})
for(const p of a.poses){
 p.thermal.regions[region]={exposedFraction:f.exposedFraction[p.id],sunlitFraction:r(.18*p.thermal.regions['coat-trunk'].sunlitFraction)}
 const pitch=p.bones.pelvis?.angleDeg??0
 if(pitch)p.bones[id]={angleDeg:r(-f.sagittalCompliance*pitch)}
}
const coat=a.thermal.regions.find(q=>q.id==='coat-trunk'),skinArea=a.thermal.body.skinAreaM2,area=f.freshSkinAreaM2
const fraction=area/skinArea,metabolic=r(coat.body.metabolicShare*fraction/coat.body.skinFraction)
coat.areaM2=r(coat.areaM2*(1-fraction/coat.body.skinFraction));coat.body.skinFraction=r(coat.body.skinFraction-fraction);coat.body.metabolicShare=r(coat.body.metabolicShare-metabolic)
a.thermal.regions.push({id:region,title:{en:'Lightly haired mammary skin',de:'Dünn behaarte Euterhaut'},kind:'skin',model:'body',material:'bare-skin',areaM2:area,areaBasis:'total',characteristicLengthM:.112,geometry:'sphere',skyViewFactor:.18,heatCapacityJM2K:15000,body:{segment:'trunk',skinFraction:r(fraction),metabolicShare:metabolic},layers:[],source:'authored-estimate'})
a.structure.assumptions.push({en:'The compliant mammary support retains forty percent of pelvic pitch. Its 0.625 kg is transferred from pelvic tissue with the standing centre of mass conserved.',de:'Die nachgiebige Euteraufhängung übernimmt vierzig Prozent der Beckenneigung. Ihre Masse von 0,625 kg stammt aus dem Beckengewebe. Der Schwerpunkt im Stand bleibt erhalten.'})
const sectionRotation=[0,.70710678,0,.70710678],hull=points=>{
 const ps=[...new Map(points.map(p=>[p.join(','),p])).values()].sort((a,b)=>a[0]-b[0]||a[1]-b[1])
 const chain=xs=>{const out=[];for(const p of xs){while(out.length>1){const [a,b]=out.slice(-2);if((b[0]-a[0])*(p[1]-a[1])-(b[1]-a[1])*(p[0]-a[0])>1e-10)break;out.pop()}out.push(p)}out.pop();return out}
 return [...chain(ps),...chain([...ps].reverse())]
}
for(const [scale,vol] of Object.entries(a.volumes)){
 const gone=new Set(['udder','teat-n','teat-f']);vol.body=vol.body.filter(q=>!gone.has(q.id));vol.thermal=vol.thermal.filter(q=>!gone.has(q.of))
 const lobes=f.lobes.filter((s,i)=>scale==='50'||scale==='100'&&i!==2||[0,2,4,6,8].includes(i)),teats=f.teatRing.filter((s,i)=>scale==='50'||[0,2,4].includes(i))
 const selected=[]
 for(const [side,sign] of [['near',-1],['far',1]]){
  for(const type of ['udder','teat']){
   const sections=(type==='udder'?lobes:teats).map(s=>({at:[s[0],s[1],r(sign*s[2])],rx:type==='udder'?s[4]:s[3],ry:s[3],exponent:type==='udder'?2.08:2,rotation:sectionRotation}))
   const body={id:type+'-'+side,bone:id,kind:'loft',material:'bare-skin',role:'body',tone:.80,sections,capStart:true,capEnd:true}
   selected.push(body);vol.body.push(body);vol.thermal.push({id:'th-'+body.id,of:body.id,region})
   if(type==='udder')vol.structure.push({id:'mass-'+side+'-gland',of:body.id,member:id,part:'mass'})
  }
 }
 const lamina={id:'median-suspensory-lamina',kind:'loft',bone:id,role:'body',tone:.65,material:'suspensory-tissue',part:'member',member:id,capStart:true,capEnd:true,sections:[[.001,.001,.007,.019],[.028,-.002,.006,.022],[.059,-.005,.005,.017],[.090,-.006,.004,.013],[.107,-.006,.003,.008]].filter((s,i)=>scale==='50'||i%2===0).map(([d,s,halfWidth,halfLength])=>({at:[d,s,0],rx:halfWidth,ry:halfLength,exponent:2.4,rotation:sectionRotation}))}
 vol.structure.push(lamina)
 const rep=a.representations[scale]
 for(const [view,shapes] of Object.entries(rep.geometry)){
  const i=shapes.findIndex(q=>q.id==='udder'),template=i<0?{role:'body',z:2.7,filled:true,tone:.80,kind:'poly',closed:true}:shapes[i]
  const index=view==='elevation'?[0,1,2]:view==='front'?[0,2,1]:[1,2,0],points=[]
  let low=Infinity,high=-Infinity
  for(const q of selected){
   const vs=tessellateSolid({...q,node:id},qualityOf(a,Number(scale))).positions
   for(let j=0;j<vs.length;j+=3){points.push([r(vs[j+index[0]]),r(vs[j+index[1]])]);low=Math.min(low,vs[j+index[2]]);high=Math.max(high,vs[j+index[2]])}
  }
  let outline=hull(points)
  if(view==='front'&&(scale==='50'||scale==='100')){
   const positive=outline.findIndex((p,j)=>p[0]>.14&&p[1]>0&&outline[(j+outline.length-1)%outline.length][1]<0)
   const negative=outline.findIndex((p,j)=>p[0]>.14&&p[1]<0&&outline[(j+outline.length-1)%outline.length][1]>0)
   const j=positive>=0?positive:negative
   if(j>=0){const sign=Math.sign(outline[j][1]);outline.splice(j,0,[.143,-sign*.024],[.112,-sign*.013],[.104,0],[.112,sign*.013],[.143,sign*.024])}
  }
  const shape={...template,id:'udder',bone:id,region,points:outline,solid:{kind:'extruded',sweep:['u','v','w'][index[2]],z0:r(low),z1:r(high)}}
  if(i>=0)shapes.splice(i,1,shape);else shapes.push(shape)
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
