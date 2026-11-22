/** 🐾 Sample the exposed forequarter and seated plan outline against the posed triangle meshes. */
import {readFileSync} from 'node:fs'
import assert from 'node:assert/strict'
import {Ray,Vector3} from 'three'
import {rigAt,meshesIn} from './r3-lib.ts'
const asset=JSON.parse(readFileSync(process.argv[2],'utf8')).asset
const V=(a,i=0)=>new Vector3().fromArray(a,i)
const f=JSON.parse(readFileSync(new URL('./lab-shoulder-case.json',import.meta.url),'utf8')),verify=!process.argv.includes('--measure')
for(const scale of f.scaleDenominators){
 const body=asset.volumes[scale].body.filter(s=>/^(neck|chest|shoulder-[nf]|upper-arm-[nf]|fore-upper-[nf])$/.test(s.id))
 for(const pose of ['standing-alert','sitting-waiting']){
  const fr=rigAt(asset,scale,{kind:'pose',pose}).frames
  const meshes=meshesIn(asset,fr,body,scale)
  const tris=meshes.map(m=>Array.from({length:m.indices.length/3},(_,i)=>[0,1,2].map(k=>V(m.world,m.indices[3*i+k]*3))))
  const widths=[]
  for(let x=fr.neck.o[0]-.095;x<fr.neck.o[0]+.161;x+=.008){
   let w=0
   for(const tt of tris)for(const tri of tt)for(let i=0;i<3;i++){
    const a=tri[i],b=tri[(i+1)%3]
    if((a.x<x)===(b.x<x))continue
    w=Math.max(w,Math.abs(a.z+(x-a.x)*(b.z-a.z)/(b.x-a.x)))
   }
   widths.push(Math.round(w*1000))
  }
  const peaks=[]
  for(let x=.398;x<.496;x+=.006)for(let y=.379;y<.49;y+=.006){
   const ray=new Ray(new Vector3(x,y,-1),new Vector3(0,0,1)),hit=new Vector3()
   const near=tris.map(tt=>{let z=1;for(const [a,b,c] of tt)if(ray.intersectTriangle(a,b,c,false,hit))z=Math.min(z,hit.z);return z})
   const base=Math.min(...near.filter((_,i)=>['chest','neck'].includes(body[i].id))),leg=Math.min(...near.filter((_,i)=>!['chest','neck'].includes(body[i].id)))
   if(base<1&&leg<1)peaks.push(Math.max(0,base-leg))
  }
  peaks.sort((a,b)=>a-b)
  const p75mm=1000*peaks[Math.floor(.75*peaks.length)],meanMm=1000*peaks.reduce((a,b)=>a+b,0)/peaks.length
  const [first,last]=f.seatedInterval,maxDropMm=Math.max(...widths.slice(first,last+1).map((w,i)=>widths[first+i-1]-w))
  if(verify&&pose==='standing-alert'){assert.ok(p75mm<=f.standingP75ProtrusionMm,kMsg());assert.ok(meanMm<=f.standingMeanProtrusionMm,kMsg())}
  if(verify&&pose==='sitting-waiting')assert.ok(maxDropMm<=f.seatedMaxWidthLossMmPer8Mm,kMsg())
  function kMsg(){return JSON.stringify({scale,pose,p75mm,meanMm,maxDropMm})}
  console.log(JSON.stringify(pose==='standing-alert'?{scale,pose,p75mm,meanMm}:{scale,pose,maxDropMm}))
 }
}
