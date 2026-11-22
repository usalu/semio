/** 🌳 Shape six interlocking crowns with a clipped outside face and a scalloped, leafy boundary. */
const relief=[.014,-.007,.020,.005,-.014,.010,-.002,.022,.004,-.010,.012,.003,-.013,.017,-.004,.008,.001,-.015,.016,.006,-.009,.013,-.002,.018]
const offsets=[0,11,4,18,2,9,16,5,20,7,3,17,10,1,14,21,6,19,13,8,15,23,12,4,22,9,16]
const bends=[[0,-.014,.021,-.017,.008],[0,.019,-.026,.018,-.006],[0,-.023,.007,.026,-.013],[0,.011,.028,-.021,.008],[0,.022,-.015,-.025,.01],[0,-.013,.024,.008,-.017]]
const round=x=>+x.toFixed(6)
export function clippedCrown(panel,k,scale){
 const n={'50':42,'100':28,'200':20,'500':14}[scale],steps={'50':25,'100':16,'200':10,'500':7}[scale]
 const section=panel.sections,lo=section[0].at[0],hi=section.at(-1).at[0],vertices=[],triangles=[]
 const at=x=>{
  let j=section.findIndex(s=>s.at[0]>=x);if(j<0)j=section.length-1
  const a=section[Math.max(0,j-1)],b=section[j],t=a===b?0:Math.max(0,Math.min(1,(x-a.at[0])/(b.at[0]-a.at[0]))),mix=(p,q)=>p+(q-p)*t
  return {dy:mix(a.at[1],b.at[1]),dz:mix(a.at[2],b.at[2]),ry:mix(a.ry,b.ry),rz:mix(a.rx,b.rx),pow:mix(a.exponent,b.exponent)}
 }
 const base=at((hi+lo)/2),curve=[],dists=[0];let total=0
 for(let i=0;i<=256;i++){
  const b=i*2*Math.PI/256,cs=Math.cos(b),sn=Math.sin(b),p=[Math.sign(cs)*Math.abs(cs)**(2/base.pow),Math.sign(sn)*Math.abs(sn)**(2/base.pow)]
  if(i){const q=curve.at(-1);total+=Math.hypot((p[0]-q[0])*base.ry,(p[1]-q[1])*base.rz);dists.push(total)}curve.push(p)
 }
 const direction=t=>{
  const d=((t%1+1)%1)*total,i=dists.findIndex(q=>q>=d);if(i<=0)return curve[0]
  const f=(d-dists[i-1])/(dists[i]-dists[i-1]);return curve[i].map((v,j)=>curve[i-1][j]+f*(v-curve[i-1][j]))
 }
 const xs=[lo,...Array.from({length:steps-1},(_,i)=>lo+(hi-lo)*(i+1)/steps),hi]
 const levels=[...xs.map((x,i)=>[x,1,i]),[hi+.009,.69,steps+1],[hi+.012,.35,steps+2]]
 for(const [x,s,row] of levels){
  const v=at(x),f=Math.max(0,Math.min(1,(x-lo)/(hi-lo))),phase=f*4,j=Math.min(3,Math.floor(phase)),drift=bends[k][j]+(bends[k][j+1]-bends[k][j])*(phase-j)
  for(let i=0;i<n;i++){
   const col=offsets[(row+k*3)%offsets.length],index=offsets[(i+col+k*5)%offsets.length],pitch=(index/23-.5)*.47
   const [cy,cz]=direction((i+pitch+.25*(row%2))/n),depth={'50':.68,'100':.61,'200':.51,'500':.39}[scale]
   const scallop=depth*(relief[index]+.006*Math.sin(.47*i+row*.29+k)),up=(row===0?0:row>=steps?.29:1)*relief[(index+col)%relief.length]*.66
   vertices.push([round(x+up),round(v.dy+drift+cy*s*(v.ry+scallop)),round(v.dz+.72*drift+cz*s*(v.rz+scallop))])
  }
 }
 for(let j=0;j+1<levels.length;j++)for(let i=0;i<n;i++){
  const a=j*n+i,b=j*n+(i+1)%n
  if((i+j)%2)triangles.push([a,b,b+n],[a,b+n,a+n]);else triangles.push([a,b,a+n],[b,b+n,a+n])
 }
 const bottom=vertices.length,top=bottom+1
 vertices.push([lo,section[0].at[1],section[0].at[2]],[round(hi+.015),section.at(-1).at[1],section.at(-1).at[2]])
 for(let i=0;i<n;i++){const q=(i+1)%n,last=(levels.length-1)*n;triangles.push([bottom,q,i],[top,last+i,last+q])}
 const {sections,capStart,capEnd,...body}=panel
 return {...body,kind:'polyhedron',vertices,triangles}
}
