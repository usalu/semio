/** 🍏 Check optically exposed scaffold wood and real fruit in opposing views of the apple crown. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
import {posedMesh} from './living-world-mesh.ts'
import {sceneInput,eachSolid} from './living-volume-input.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const [file,dest]=process.argv.slice(2),raw=readFileSync(file,'utf8'),a=sceneInput(raw),mesh=posedMesh(a,a.presentation.defaultPose,true),w=370,h=260,sheet=new Canvas(2*w,(h+15)*4,[250,249,243])
const minWindows=JSON.parse(readFileSync(new URL('./botanical-coarse-case.json',import.meta.url),'utf8')).apple.minimumVisibleOpenings
const cameras=[[.53,.16,1],[-.69,.14,-1]].map(p=>{const e=new Vector3(...p).normalize(),r=new Vector3(0,1,0).cross(e).normalize(),u=e.clone().cross(r).normalize();return{e:e.toArray(),r:r.toArray(),u:u.toArray()}})
for(const [n,k] of [50,100,200,500].entries()){
 const items=Array.from(eachSolid(raw,String(k))('body')).map(s=>{
  const fruit=s.material==='apple-fruit',twig=s.material==='apple-bark-young'||s.material==='apple-twig',wood=s.material.includes('bark')||twig
  return {mesh:mesh(s,qualityOf(a,k)),color:fruit?[166,66,45]:wood?[104,77,47]:[78+26*s.tone,102+48*s.tone,58+15*s.tone],marker:fruit?[216,20,0]:twig?[0,30,219]:[87,108,64]}
 })
 const counts=[]
 for(const [j,cam] of cameras.entries()){
  const view=new Canvas(w,h,[245,245,235]),mask=new Canvas(w,h,[250,250,250]),ax=fit(items.map(q=>q.mesh),cam,w,h,.09)
  drawAxon(view,ax,items);for(const it of items)drawAxon(mask,ax,[{mesh:it.mesh,color:it.marker}],{unlit:true,clearDepth:false})
  const seen=new Uint8Array(w*h),wood=[],red=[]
  for(let i=0;i<w*h;i++){if(mask.data[3*i+2]===219)wood.push(i);if(mask.data[3*i]===216)red.push(i)}
  let windows=0
  for(const i of wood){
   if(seen[i])continue
   const q=[i];seen[i]=1
   for(let t=0;t<q.length;t++)for(const j of [q[t]-1,q[t]+1,q[t]-w,q[t]+w])if(j>=0&&j<w*h&&!seen[j]&&mask.data[3*j+2]===219){seen[j]=1;q.push(j)}
   if(q.length>=3)windows++
  }
  counts.push({woodPx:wood.length,windowCount:windows,fruitPx:red.length})
  const y=n*(h+15);sheet.blit(view,j*w,y);sheet.text(j*w+4,y+h+3,'1/'+k+' '+(j?'rear':'front')+' crown',[36,46,37],1)
 }
 console.log(JSON.stringify({k,counts}))
 if(k===500&&process.argv.includes('--accept')&&(Math.min(...counts.map(c=>c.windowCount))<minWindows||Math.min(...counts.map(c=>c.fruitPx))<3))process.exitCode=1
}
writeFileSync(dest,sheet.png())
