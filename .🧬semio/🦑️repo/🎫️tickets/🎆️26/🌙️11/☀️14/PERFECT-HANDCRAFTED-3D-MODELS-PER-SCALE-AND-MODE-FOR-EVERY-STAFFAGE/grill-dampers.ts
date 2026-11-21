/** 🌬️ Cuts the real vent routes through the kettle wall and through each independently rotating damper. */
import Module from 'manifold-3d'
import {readFileSync,writeFileSync} from 'node:fs'
const kit=await Module();kit.setup();const M=kit.Manifold,p=process.argv[2],doc=JSON.parse(readFileSync(p,'utf8')),a=doc.asset
for(const [d,n]of [[50,64],[100,48],[200,36],[500,24]]){
 const v=a.volumes[d],body=Object.fromEntries(v.body.map(q=>[q.id,q]))
 const centres=body['vent-dial'].holes.map(h=>h.reduce(([x,y],v)=>[x+v[0]/h.length,y+v[1]/h.length],[0,0]))
 function cut(q,axis,centres,r){
  let profile=q.profile.map(([h,rad])=>[rad*1000,h*1000])
  if(profile.at(-1)[0]!==0&&profile[0][0]!==0&&profile[0][1]!==profile.at(-1)[1])throw Error('open lathe '+q.id)
  const area=profile.reduce((s,[x,y],i)=>{const next=profile[(i+1)%profile.length];return s+x*next[1]-y*next[0]},0)
  if(area<0)profile.reverse()
  const segments=['vent-skirt','damper-plate'].includes(q.id)?64:n
  let m=M.revolve(profile,segments).rotate(axis==='y'?[-90,0,0]:[0,90,0]).translate(q.origin.map(k=>1000*k))
  const holes=centres.map(([x,z])=>M.cylinder(2200,r,r,Math.max(12,segments/2)).rotate(axis==='y'?[-90,0,0]:[0,90,0]).translate(axis==='y'?[x*1000,-500,z*1000]:[-500,x*1000,z*1000]))
  m=m.subtract(M.union(holes));if(m.isEmpty()||m.status()!=='NoError')throw Error(q.id+' '+m.status())
  const g=m.getMesh(),verts=[],tri=[],positions=g.vertProperties,faces=g.triVerts
  for(let i=0;i<positions.length;i+=3)verts.push([0,1,2].map(k=>Math.round(positions[i+k]*10000)/1e7))
  for(let i=0;i<faces.length;i+=3)tri.push([faces[i],faces[i+1],faces[i+2]])
  for(const k of ['profile','axis','origin'])delete q[k]
  Object.assign(q,{kind:'polyhedron',vertices:verts,triangles:tri})
 }
 const low=Array.from({length:3},(_,i)=>[.040*Math.cos(2*Math.PI*i/3),.040*Math.sin(2*Math.PI*i/3)]),top=centres.map(([y,z])=>[.2925-y,z])
 cut(body['bowl-shell'],'y',low,10);cut(body['damper-plate'],'x',low,10);cut(body['lid-shell'],'y',top,6);cut(body['vent-skirt'],'x',centres,6)
 for(const q of v.thermal){
  if(q.id==='th-bowl-envelope'||q.id==='th-lid-envelope'){
    const profile=q.profile;if(!profile)continue
    if(profile.at(-1)[1]>0)profile.push([profile.at(-1)[0],0]);if(profile[0][1]>0)profile.unshift([profile[0][0],0])
    cut(q,'y',q.id==='th-bowl-envelope'?low:top,q.id==='th-bowl-envelope'?10:6)
  }
 }
 v.decisions.push({en:'Three 20 mm ash and inlet holes pass through the bottom of the kettle. Three aligned 12 mm passages through the lid, skirt and rotating dial let hot gas escape. Both shutters reduce the opening when twisted.',de:'Drei 20-mm-Öffnungen im Kesselboden lassen Asche fallen und Luft einströmen. Drei fluchtende 12-mm-Bohrungen durch Deckel, Manschette und Drehscheibe führen Heißgas ab. Beide Schieber drosseln den Querschnitt beim Drehen.'})
}
writeFileSync(p,JSON.stringify(doc))
