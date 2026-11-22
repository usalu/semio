/** 🦮 Sculpt seven canine vertebral bodies and their open neural arches in the lumbar member frame. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),asset=doc.asset
const levels=JSON.parse(readFileSync(new URL('./digital-lab-lumbar.json',import.meta.url),'utf8')).levels
const metres=(x:number)=>+(x*.001).toFixed(7)
function vertebra(row){
 const [level,centre,length,width,depth,foramenWidth,foramenHeight,spine,reach,sweep,facet]=row
 const vertices=[],triangles=[]
 const crossSections=[-1,-.8,-.48,-.19,.04,.18,.36,.59,.82,1]
 const rings=[]
 for(const u of crossSections){
  const mid=Math.max(0,1-Math.abs((u+.06)/1.06)),wing=Math.max(0,1-((u-.12)/.92)**2)**.72,arch=Math.max(0,1-Math.abs(u)/.98)
  const end=Math.abs(u)**2,ankle=.5*width*(.86+.14*end),lateral=ankle+wing*(reach-ankle)
  const yBase=4.2,yFloor=8.95,canalTop=yFloor+foramenHeight,roof=canalTop+3.3
  const root=centre+u*length*.5
  const right=[
   [root+mid*3.8,roof+mid*(spine-roof),0],
   [root+mid*2.3,roof+mid*(spine-roof)*.91,1.6],
   [root+arch,roof+.5,3.5],
   [root+Math.sign(u)*end*2.2,roof+end*(facet-roof),7.8],
   [root+Math.sign(u)*end*2.3,roof-1+end*(facet-roof)*.80,10.0],
   [root,13.0,10.6],
   [root+wing*sweep*.48,7.9-wing*3.0,ankle+wing*(reach-ankle)*.56],
   [root+wing*sweep,5.4-wing*8.5,lateral+.5],
   [root+wing*sweep*.92,3.0-wing*8.0,lateral-.2],
   [root,1.4,ankle],
   [root,yBase-depth*.6,ankle*.96],
   [root,yBase-depth*.92,ankle*.68],
   [root,yBase-depth,0]
  ]
  const outer=[...right,...right.slice(1,-1).reverse().map(([x,y,z])=>[x,y,-z])]
  const n=outer.length,inner=outer.map((_,i)=>{
   const theta=i*2*Math.PI/n
   return [root,(canalTop+yFloor)*.5+(foramenHeight*.5)*(1+.06*end)*Math.cos(theta),.5*foramenWidth*(1+.055*end)*Math.sin(theta)]
  })
  const base=vertices.length
  vertices.push(...outer.concat(inner).map(v=>v.map(metres)))
  rings.push({base,n})
 }
 const quad=(a,b,c,d)=>triangles.push([a,b,c],[a,c,d])
 for(let j=0;j<rings.length;j++){
  const {base,n}=rings[j]
  for(let i=0;i<n;i++){
   const next=(i+1)%n,a=base+i,b=base+next,c=a+n,d=b+n
   if(j+1<rings.length){quad(a,b,b+2*n,a+2*n);quad(d,c,c+2*n,d+2*n)}
   if(j===0)quad(b,a,c,d)
   if(j===rings.length-1)quad(a,b,d,c)
  }
 }
 return {id:'lumbar-'+level,bone:'lumbar',part:'member',member:'lumbar',material:'bone',role:'body',tone:.9,kind:'polyhedron',vertices,triangles}
}
for(const vol of Object.values(asset.volumes)){
 const remove=new Set(['lumbar-column','lumbar-spines','lumbar-processes',...levels.map(([id])=>'lumbar-'+id)])
 const first=vol.structure.findIndex(s=>remove.has(s.id))
 if(first<0)throw Error('Missing lumbar column')
 vol.structure=vol.structure.filter(s=>!remove.has(s.id))
 vol.structure.splice(first,0,...levels.map(vertebra))
}
writeFileSync(process.argv[3],JSON.stringify(doc))
