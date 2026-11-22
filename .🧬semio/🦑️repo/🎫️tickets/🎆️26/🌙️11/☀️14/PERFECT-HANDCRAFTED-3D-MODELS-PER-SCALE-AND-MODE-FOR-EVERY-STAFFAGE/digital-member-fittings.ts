/** 🔩 Fit the crane heel, crank forgings and pollard heartwood to their physical stock. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
if(a.id==='cyclist'){
 const vol=a.volumes['500'],fine=a.volumes['50']
 for(const side of ['l','r']){
  const id='crank-arm-'+side
  vol.body.push(structuredClone(fine.body.find(q=>q.id===id)))
  vol.thermal.push(structuredClone(fine.thermal.find(q=>q.of===id)))
  const i=vol.structure.findIndex(q=>q.id==='st-crank-'+side)
  vol.structure[i]={id:vol.structure[i].id,of:id,member:'crank-'+side,part:'member'}
 }
}else if(a.id==='tower-crane'){
 for(const k of ['50','100','200']){
  const vol=a.volumes[k]
  for(const [joint,stock] of [['joint-jib-heel','jib-heel-cross'],['joint-jib-heel-top','jib-heel-pin']]){
   if(!vol.body.some(q=>q.id===stock))continue
   const member=vol.body.find(q=>q.id===stock),i=vol.structure.findIndex(q=>q.id===joint)
   const length=member.profile.at(-1)[0],radius=member.profile[0][1],width=1.6*radius
   const endCollar={...structuredClone(member),id:joint,part:'joint',member:'jib',profile:[[0,radius],[width,radius]]}
   const other=structuredClone(endCollar);other.id=joint+'-opposite';other.origin=member.origin.map((x,k)=>+(x+(length-width)*member.axis[k]).toFixed(7))
   vol.structure.splice(i,1,endCollar,other)
  }
 }
}else if(a.id==='london-plane-pollarded'){
 for(const k of ['50','100','200']){
  const vol=a.volumes[k],core=vol.structure.find(q=>q.id==='member-k-c'),shell=vol.body.find(q=>q.id==='knuckle-c'),ring=shell.triangles[0][2]
  const vertices=[],count=(shell.vertices.length-2)/ring
  for(let row=0;row<count;row++){
   const points=shell.vertices.slice(row*ring,(row+1)*ring)
   const centre=[0,1,2].map(i=>points.reduce((sum,q)=>sum+q[i],0)/ring)
   for(const q of points)vertices.push(q.map((v,i)=>+(centre[i]+(v-centre[i])*.62).toFixed(8)))
  }
  core.vertices=[...vertices,...shell.vertices.slice(-2)];core.triangles=structuredClone(shell.triangles)
 }
}else throw Error(a.id)
writeFileSync(process.argv[3],JSON.stringify(doc))
