/** 🎸 Articulate a seated player's left hand against measured strings, fret positions and a tuning button. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {evaluateDrawing} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,f=JSON.parse(readFileSync(new URL('./digital-guitar-grip-case.json',import.meta.url),'utf8'))
const round=x=>+x.toFixed(7),rad=x=>x*Math.PI/180,degrees=x=>x*180/Math.PI,v=p=>new Vector3(p.x,p.y,p.z),pt=(x,y,z)=>new Vector3(x,y,z),n=(en,de)=>({en,de})
const fine=a.volumes[50],fresh=structuredClone(fine.body),oldPalm=fresh.find(s=>s.id==='palm-l'),oldThenar=fresh.find(s=>s.id==='thenar-l'),wrist=a.rig.bones.find(b=>b.id==='hand-l'),turn=[0,-.70710678,0,.70710678]
const names=Object.keys(f.fingers),links=name=>(name==='thumb'?['metacarpal','proximal','distal']:['proximal','middle','distal']).map(type=>name+'-'+type+'-l'),belong=new Set(names.flatMap(links)),de={thumb:'Daumen',index:'Zeigefinger',middle:'Mittelfinger',ring:'Ringfinger',little:'Kleinfinger'}
a.rig.bones=a.rig.bones.filter(b=>!belong.has(b.id))
wrist.lengthM=.105
const original=new Map(a.poses.map(p=>[p.id,evaluateDrawing(a,{kind:'pose',pose:p.id}).bones]))
for(const name of names){const fng=f.fingers[name]
 for(const [i,id] of links(name).entries()){
  const [x,y,z]=i?[fng.lengths[i-1],0,0]:fng.knuckle
  a.rig.bones.push({id,parent:i?links(name)[i-1]:wrist.id,title:n('Left '+name+' '+id.split('-')[1],de[name]+' links, '+(['Grundgelenk','Mittelgelenk','Endgelenk'][i])),x,y,z,lengthM:fng.lengths[i],angleDeg:0,joint:i?{type:'revolute',minDeg:0,maxDeg:i===1?123:90}:{type:'revolute',minDeg:-180,maxDeg:180,twistMinDeg:-180,twistMaxDeg:180}})
 }
}
const anchorIds=new Set(names.map(k=>'fingerpad-'+k+'-l').concat('tuner-button'))
a.anchors=a.anchors.filter(anchor=>!anchorIds.has(anchor.id))
for(const name of names)a.anchors.push({id:'fingerpad-'+name+'-l',bone:links(name)[2],x:f.fingers[name].lengths[2],y:0,z:0,kind:'attachment',title:n('Left '+name+' fingertip',de[name]+' links, Fingerkuppe')})
const button=fresh.find(s=>s.id===f.tuner).transform.translation
a.anchors.push({id:'tuner-button',bone:'guitar',x:button[0],y:button[1],z:button[2],kind:'attachment',title:n('Bass-side middle tuning button','Mittlerer Stimmflügel auf der Bassseite')})
function relative(a,point,direction=false){
 const d=direction?point.clone():point.clone().sub(v(a.o));return pt(d.dot(v(a.u)),d.dot(v(a.v)),d.dot(v(a.w)))
}
function world(a,p,direction=false){return (direction?pt(0,0,0):v(a.o)).addScaledVector(v(a.u),p.x).addScaledVector(v(a.v),p.y).addScaledVector(v(a.w),p.z)}
function fit(base,target,outward,approach,len,desired){
 const along=target.clone().sub(base),requested=along.length();along.normalize()
 const normal=outward.clone().addScaledVector(along,-outward.dot(along)).normalize(),[l,m,n]=len
 const distance=Math.max(.008,Math.min(l+m+n-.00002,requested))
 let winner=null
 for(let ip=2;ip<=123;ip+=.5){
  const p=rad(ip),cx=l+m*Math.cos(p),cy=m*Math.sin(p),c=Math.hypot(cx,cy)
  const cos=(distance*distance-c*c-n*n)/(2*c*n);if(Math.abs(cos)>1)continue
  const d=Math.acos(cos)+Math.atan2(cy,cx)-p,dp=degrees(d);if(dp<0||dp>90)continue
  const endAngle=Math.atan2(cy+n*Math.sin(p+d),cx+n*Math.cos(p+d))
  const dir=along.clone().multiplyScalar(Math.cos(endAngle-p-d)).addScaledVector(normal,Math.sin(endAngle-p-d))
  const score=dir.dot(approach)*2-Math.abs(dp-ip*.66)/100-Math.abs(ip-desired)/170
  if(!winner||score>winner.score){
   const u=along.clone().multiplyScalar(Math.cos(endAngle)).addScaledVector(normal,Math.sin(endAngle)).normalize(),w=normal.clone().cross(along).normalize(),up=w.clone().cross(u).normalize(),basic=pt(-u.y,u.x,0).normalize()
   winner={score,p:ip,d:dp,rotation:{angleDeg:round(degrees(Math.atan2(u.y,u.x))),abductionDeg:round(degrees(Math.asin(u.z))),twistDeg:round(degrees(Math.atan2(w.dot(basic),up.dot(basic))))}}
  }
 }
 if(!winner)throw Error('Unreachable digit curl '+requested+' '+len.join(','))
 return winner
}
const stringAt=(num,x)=>{const s=fresh.find(p=>p.id==='guitar-string-'+num).sections,[p,q]=s.slice(0,2).map(k=>new Vector3(...k.at)),t=(x-p.x)/(q.x-p.x);return p.lerp(q,t)}
const tuner=fresh.find(s=>s.id===f.tuner),peg=new Vector3(...tuner.transform.translation)
for(const pose of a.poses){
 const b=original.get(pose.id),g=b.guitar.frame,h=b[wrist.id].frame,local=p=>relative(h,world(g,p)),vector=p=>relative(h,world(g,p,true),true)
 const handX=relative(g,v(h.o)).x
 for(const name of names){
  const digit=f.fingers[name],base=new Vector3(...digit.knuckle),r=digit.radii[3],grip=f.chords[pose.id]?.[name],thumb=name==='thumb'
  let p,side=pt(0,-1,.18),normal=pt(0,1,0),curl=65
  if(pose.id==='tuning'&&(thumb||name==='index')){
   p=peg.clone();p.y+=(thumb?1:-1)*(tuner.radii[1]+r);normal=pt(0,thumb?-1:1,0);side=pt(thumb?1:-.4,thumb?.35:-1,.15);curl=thumb?32:70
  }else if(grip){
   const [fret,number]=grip,x=f.neck.nutX-f.neck.scaleLength*(1-2**(-fret/12))+f.neck.pressBehindFretM
   p=stringAt(number,x);p.y-=r+.0006
  }else if(thumb){
   p=pt(handX+.035,-.036+r,f.neck.boardZ+.002);side=pt(1,.3,.45);normal=pt(0,-1,0);curl=38
  }else{
   const k=names.indexOf(name),x=handX+[.027,.009,-.011,-.032][k],tuning=pose.id==='tuning'
   p=pt(x,tuning?-.064:-.083,f.neck.boardZ+(tuning?-.031:.007-k*.003));curl=tuning?72:60
  }
  const rframe=fit(base,local(p),vector(side),vector(normal),digit.lengths,curl),ids=links(name)
  pose.bones[ids[0]]=rframe.rotation;pose.bones[ids[1]]={angleDeg:round(rframe.p)};pose.bones[ids[2]]={angleDeg:round(rframe.d)}
 }
}
const instrument=s=>s.id.startsWith('guitar-string-')||s.id.startsWith('guitar-tuner')||s.id.startsWith('fret-wire-')||['guitar-neck','guitar-fretboard','guitar-head','guitar-nut'].includes(s.id)
const hand=s=>s.bone===wrist.id||belong.has(s.bone),skin={material:'skin',role:'body',tone:.74},bone={material:'bone',role:'body',tone:.83}
const boards=Object.fromEntries(Object.entries(a.representations[50].geometry).map(([v,shapes])=>[v,structuredClone(shapes.filter(s=>instrument(s)&&!s.id.startsWith('guitar-string-')&&!s.id.startsWith('fret-wire-')&&s.id!=='guitar-nut'))]))
const home=new Map(fine.structure.filter(s=>s.bone===wrist.id&&!s.id.startsWith('phalanges-')&&!s.id.startsWith('thumb-bones')).map(s=>[s.id,structuredClone(s)]))
for(const [scale,vol] of Object.entries(a.volumes)){
 const removed=new Set(vol.body.filter(p=>hand(p)||instrument(p)).map(s=>s.id)),geo=a.representations[scale].geometry
 vol.body=vol.body.filter(p=>!removed.has(p.id));vol.thermal=vol.thermal.filter(p=>!removed.has(p.of)&&!hand(p));vol.structure=vol.structure.filter(p=>!removed.has(p.of)&&!hand(p))
 for(const view in geo)geo[view]=geo[view].filter(p=>!hand(p)&&!instrument(p))
 function part(s,region,struct=false){
  vol.body.push(s);vol.thermal.push({id:'th-'+s.id,of:s.id,region});if(struct)vol.structure.push({id:'st-'+s.id,of:s.id,member:'guitar',part:'member'})
 }
 function draw(s){
  const k=s.kind
  if(k==='loft'&&s.sections.every(q=>Math.abs(q.at[1])<.000001&&Math.abs(q.at[2])<.000001)){
   const path=s.sections.map(q=>[q.at[0],q.ry]),points=[[path[0][0],0],...path,[path.at(-1)[0],0]]
   for(const view in geo)geo[view].push({id:s.id,bone:s.bone,kind:'poly',material:s.material,region:'hands',role:'body',tone:s.tone,z:2.76,filled:true,closed:true,points,solid:{kind:'revolved',axisBone:s.bone,axis:'u',axisX:0,mirror:true}})
  }
 }
 const palm=structuredClone(oldPalm),thenar=structuredClone(oldThenar);part(palm,'hands');part(thenar,'hands')
 for(const s of home.values())vol.structure.push(structuredClone(s))
 const outline=palm.sections.map(q=>[q.at[0],q.ry]),palmPoints=[...outline,...outline.slice().reverse().map(([x,y])=>[x,-y])]
 for(const view in geo)geo[view].push({id:'palm-l',bone:wrist.id,kind:'poly',material:'skin',region:'hands',role:'body',tone:palm.tone,z:2.74,filled:true,closed:true,points:palmPoints,solid:{kind:'extruded',sweep:'w',z0:.19-.04,z1:.19+.04}})
 for(const name of names){const dig=f.fingers[name];
  for(const [i,id] of links(name).entries()){
   const l=dig.lengths[i],r0=dig.radii[i],r1=dig.radii[i+1],station=(x,r)=>({at:[round(x),0,0],rx:round(r),ry:round(r),rotation:turn,exponent:2})
   const sections=[station(-r0*.62,r0*.44),station(0,r0),station(l*.55,(r0+r1)/2),station(l,r1),station(l+r1*.72,r1*.48)]
   const s={id:'pad-'+id,bone:id,kind:'loft',...skin,sections,capStart:true,capEnd:true};part(s,'hands');draw(s)
   vol.structure.push({id:'shaft-'+id,bone:id,kind:'capsule',...bone,p0:[.0014,0,0],p1:[round(l-.0014),0,0],r0:round(Math.max(.0038,r0*.53)),r1:round(Math.max(.0038,r1*.53)),member:id,part:'member'},{id:'joint-'+id,bone:id,kind:'ellipsoid',...bone,radii:[round(Math.max(.0038,r0*.53)),round(Math.max(.0038,r0*.59)),round(Math.max(.0038,r0*.53))],member:id,part:'joint'})
  }
 }
 function etched(s,points,heights){
  for(const view in geo)geo[view].push({id:s.id,bone:'guitar',kind:'poly',material:s.material,region:'guitar',role:'texture',tone:s.tone,z:3.1,filled:true,closed:true,points,penMm:.20,solid:{kind:'extruded',sweep:'v',z0:Math.min(...heights),z1:Math.max(...heights)}})
 }
 for(const view in geo)geo[view].push(...structuredClone(boards[view]).map(s=>(+scale>=200?{...s,penMm:scale==='500'?.23:.17}:s)))
 const neck=fresh.filter(s=>instrument(s)&&!s.id.startsWith('guitar-tuner-spindle-')&&!s.id.startsWith('guitar-tuner-post-')&&!s.id.startsWith('fret-wire-')&&s.id!=='guitar-nut')
 for(const s of neck){
  const v=structuredClone(s)
  if(+scale>=100&&v.kind==='loft'&&v.sections.length>3)v.sections=v.sections.filter((_,i)=>i===0||i===v.sections.length-1||i===Math.floor(v.sections.length/2))
  part(v,'guitar',v.id!=='guitar-fretboard')

 }
 for(let j=1;j<=6;j++){
  const string=neck.find(s=>s.id==='guitar-string-'+j),end=string.sections.at(-1).at
  const button=neck.find(s=>s.id==='guitar-tuner-'+(j<=3?'b':'t')+(j<=3?j:7-j)),at=button.transform.translation
  const horizontal=[end[0],at[1],end[2]]
  for(const [kind,p0,p1,r] of [['post',end,horizontal,.0031],['spindle',horizontal,at,.0027]]){
   const id='guitar-tuner-'+kind+'-'+j;part({id,bone:'guitar',kind:'capsule',p0,p1,r0:kind==='post'?r:.0043,r1:kind==='post'?.0043:r,material:'steel',role:'detail',tone:.65},'guitar',true)
  }
 }
 for(let k=0;k<=20;k++){
  const x=f.neck.nutX-f.neck.scaleLength*(1-2**(-k/12)),hw=.0218+(.5797-x)*.0182,r=k?.00055:.0015,y=k?-.0623:-.0626,id=k?'fret-wire-'+k:'guitar-nut'
  const s={id,bone:'guitar',material:k?'steel':'rosewood',role:'texture',tone:k?.65:.77,kind:'capsule',p0:[round(x),y,round(f.neck.boardZ-hw)],p1:[round(x),y,round(f.neck.boardZ+hw)],r0:r,r1:r,...(k?{slender:true}:{})};part(s,'guitar',true);if(scale==='50'&&[3,7,12].includes(k))etched(s,[[x-r,s.p0[2]],[x+r,s.p0[2]],[x+r,s.p1[2]],[x-r,s.p1[2]]],[y-r,y+r])
 }
 const note=n('The fretting hand has separate knuckles, finger shafts and pads. Three fingertips press a C fingering, move up the neck, and release for the thumb and index to grip the tuning button. The radiused neck, six strings and fret wires run beneath the pads.','Die Griffhand besitzt einzelne Knöchel, Fingerglieder und Fingerkuppen. Drei Finger greifen einen C-Akkord, wandern auf dem Hals und lösen sich zum Stimmen. Daumen und Zeigefinger umfassen den Stimmwirbel. Unter den Fingerkuppen liegen der gerundete Hals, sechs Saiten und die Bundstäbe.')
 if(!vol.decisions.some(d=>d.en===note.en))vol.decisions.push(note)
}
writeFileSync(process.argv[3],JSON.stringify(doc))
