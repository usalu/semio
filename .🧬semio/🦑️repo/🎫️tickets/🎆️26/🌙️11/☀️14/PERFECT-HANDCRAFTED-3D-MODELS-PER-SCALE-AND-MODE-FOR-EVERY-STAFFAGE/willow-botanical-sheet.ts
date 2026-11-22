/** 🌳 Count visible wood in opposite views of the willow and inspect its final lanceolate leaves. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {sceneInput,eachSolid} from './living-volume-input.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
import {Canvas} from './volumes-raster.ts'
import {drawAxon,fit} from './volumes-render.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=sceneInput(raw),pose=process.argv[4]??'calm',mesh=posedMesh(a,pose,true),w=300,h=230,sheet=new Canvas(3*w,(h+15)*4,[250,250,245])
const cam=(eye,up)=>{const e=eye.normalize(),r=up.clone().cross(e).normalize(),u=e.clone().cross(r).normalize();return {r:r.toArray(),u:u.toArray(),e:e.toArray()}}
for(const [row,k] of [50,100,200,500].entries()){
 const q=qualityOf(a,k),visit=eachSolid(raw,String(k)),counts=[]
 for(const [col,mode] of ['side','reverse','terminal blades'].entries()){
  const items=[],subset=[],stats=new Canvas(w,h,[250,250,245]),view=new Canvas(w,h,[250,250,245])
  for(const s of visit('body')){
   if(col===2&&!s.id.includes('tress-c09-spray-3'))continue
   const m=mesh(s,q),wood=s.material==='willow-bark',mainWood=wood&&!s.id.startsWith('twig-'),color=wood?[116,85,52]:s.id.startsWith('blade-')?[86,135,74]:[90+Math.round(s.tone*32),126+Math.round(s.tone*22),75]
   items.push({mesh:m,color,mainWood})
   if(s.id.startsWith('blade-'))subset.push(m)
  }
  const cameras=[cam(new Vector3(.03,.12,1),new Vector3(0,1,0)),cam(new Vector3(-.43,.15,-1),new Vector3(0,1,0))]
  if(col===2&&!subset.length)continue
  if(col===2){
   const xyz=i=>new Vector3().fromArray(subset[0].world,i*3),up=xyz(3).sub(xyz(0)).normalize(),side=xyz(5).sub(xyz(1)).normalize()
   cameras.push(cam(side.clone().cross(up).normalize(),up))
  }
  const ax=fit(col===2?subset:items.map(i=>i.mesh),cameras[col],w,h,.10)
  drawAxon(view,ax,items)
  if(col!==2){
   for(const it of items)drawAxon(stats,ax,[{mesh:it.mesh,color:it.mainWood?[240,0,0]:[0,140,0]}],{clearDepth:false,unlit:true})
   let red=0,open=0
   for(let j=0;j<stats.data.length;j+=3){if(stats.data[j]===240&&stats.data[j+1]===0)red++;if(stats.data[j+1]===140)open++}
   if(process.argv.includes('--accept')&&red<JSON.parse(readFileSync(new URL('./willow-botanical-case.json',import.meta.url),'utf8')).minimumWoodVisiblePixels)throw Error(k+' '+mode+' exposed wood '+red)
   counts.push({woodPixels:red,foliageOrTwigs:open})
  }
  const y=row*(h+15);sheet.blit(view,col*w,y);sheet.text(col*w+3,y+h+2,'1/'+k+' '+mode,[45,45,44],1)
 }
 console.log(JSON.stringify({k,pose,counts}))
}
writeFileSync(process.argv[3],sheet.png())
