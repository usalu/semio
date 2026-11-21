/** 🍃 Alternate double-toothed, pleated hornbeam blades on six separately clipped and wind-bent stools. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,spec=JSON.parse(readFileSync(new URL('./digital-hornbeam-flush.json',import.meta.url),'utf8')),rad=Math.PI/180,r=x=>+x.toFixed(6)
for(const pose of a.poses){
 if(!['windy','storm'].includes(pose.id))continue
 for(const [k,plant] of spec.plants.entries())for(const prefix of ['stem','block']){
  const id=prefix+'-'+(k+1),previous=pose.bones[id]?.angleDeg??0
  pose.bones[id]={...pose.bones[id],angleDeg:r(.23*previous),abductionDeg:r((pose.id==='storm'?1.78:1)*plant.lean*(prefix==='stem'?1.56:2.8))}
 }
}
for(const [scale,vol] of Object.entries(a.volumes)){
 for(const [k,plant] of spec.plants.entries()){
  const bone='block-'+(k+1),panel=vol.body.find(q=>q.id===bone)
  const skin=(height,along)=>{
   const ss=panel.sections;let j=ss.findIndex(q=>q.at[0]>=height);if(j<0)j=ss.length-1
   const lo=ss[Math.max(0,j-1)],hi=ss[j],t=lo===hi?0:Math.max(0,Math.min(1,(height-lo.at[0])/(hi.at[0]-lo.at[0])))
   const mix=(a,b)=>a+(b-a)*t,offset=mix(lo.at[1],hi.at[1]),width=mix(lo.ry,hi.ry),power=mix(lo.exponent??2,hi.exponent??2)
   return mix(lo.rx,hi.rx)*Math.pow(Math.max(.025,1-Math.pow(Math.min(.995,Math.abs(along-offset)/width),power)),1/power)
  }
  for(const [j,shoot] of plant.shoots.entries()){
   if(scale==='100'&&j===2||scale==='200'&&![0,2,4].includes(j)||scale==='500'&&j!==2)continue
   const vertices=[],triangles=[]
   const strip=rings=>{
    const start=vertices.length,n=rings[0].length;vertices.push(...rings.flat().map(p=>p.map(r)))
    for(let i=0;i+1<rings.length;i++)for(let z=0;z<n;z++){
     const a=start+n*i+z,b=start+n*i+(z+1)%n
     triangles.push([a,b,a+n],[b,b+n,a+n])
    }
    for(let z=1;z<n-1;z++){triangles.push([start,start+z+1,start+z]);const end=start+n*(rings.length-1);triangles.push([end,end+z,end+z+1])}
   }
   const flush=spec.flushes[(plant.phase+j)%3],sections=scale==='50'?spec.toothWidths.map((_,i)=>i):scale==='100'?[0,3,5,7,10,13,15,17,19]:scale==='200'?[0,5,8,13,17,19]:[0,7,13,19]
   for(const side of [-1,1]){
    const rotation=(shoot[2]*side+(side<0?9:0))*rad
    const baseY=shoot[0]*side,baseX=shoot[1]+(side<0?.055:0)
    const pointAt=s=>[baseX+Math.cos(rotation)*s,baseY+Math.sin(rotation)*s]
    const nodes=flush.map(([pos])=>pointAt(pos/1000))
    strip(nodes.map(([x,y],i)=>{
     const z=side*(skin(x,y)+.001),radius=.0018-.00095*i/(nodes.length-1)
     return [[x,y-radius,z],[x,y,z+radius],[x,y+radius,z],[x,y,z-radius]]
    }))
    for(const [i,[pos,cant,length,width]] of flush.entries()){
     if(scale==='200'&&i%2||scale==='500'&&![1,3,6].includes(i))continue
     const [x,y]=nodes[i],angle=rotation+cant*side*rad,mid=i%2===0?.18:-.14
     const rings=sections.map(idx=>{
      const t=idx/(spec.toothWidths.length-1),l=length/1000,amplitude=spec.toothWidths[idx]*width/2000
      const cx=x+Math.cos(angle)*l*t-Math.sin(angle)*mid*l*t*t*.20,cy=y+Math.sin(angle)*l*t+Math.cos(angle)*mid*l*t*t*.20
      const crest=side*(skin(cx,cy)+.001+.012*Math.sin(Math.PI*t)+.004*t)
      const meridian=side*(.00035+.0015*Math.sin(Math.PI*t))
      const edge=(sign,asym)=>{
       const half=amplitude*asym*sign,dx=-Math.sin(angle)*half,dy=Math.cos(angle)*half
       return [cx+dx,cy+dy,crest-side*(Math.abs(half)*(.18+.1*(idx%2))+meridian*side*.22)]
      }
      return [edge(-1,1.07),[cx,cy,crest],edge(1,.93),[cx,cy,crest-meridian]]
     })
     strip(rings)
    }
   }
   const id='leaf-spray-'+(k+1)+'-'+(j+1)
   vol.body.push({id,kind:'polyhedron',bone,material:'hornbeam-leaf',role:'body',tone:r(.40+.052*((plant.phase+j)%5)),vertices,triangles})
   vol.thermal.push({id:'th-'+id,of:id,region:'leaf-face'})
  }
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
