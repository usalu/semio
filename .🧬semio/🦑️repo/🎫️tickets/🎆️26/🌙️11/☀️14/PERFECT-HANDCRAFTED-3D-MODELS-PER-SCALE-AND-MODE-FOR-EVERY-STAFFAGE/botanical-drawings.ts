/** ✏️ Render the product's three orthographic paper drawings at each authored scale with the default pose. */
import {readFileSync,writeFileSync} from 'node:fs'
import {createHash} from 'node:crypto'
import {join} from 'node:path'
import sharp from 'sharp'
import {drawingSvg} from './r3-2d.ts'
import {paperInput} from './living-paper-input.ts'
const raw=readFileSync(process.argv[2],'utf8'),a=paperInput(raw),hash=createHash('sha256').update(raw).digest('hex').slice(0,10),doc={kind:'pose',pose:a.presentation.defaultPose},w=370,h=260,parts=[]
for(const [row,k] of [50,100,200,500].entries()){
 for(const [col,view] of ['elevation','front','plan'].entries()){
  const svg=drawingSvg(a,k,doc,'light-solid',view),render=await sharp(Buffer.from(svg),{density:140}).resize(w-8,h-22,{fit:'contain',background:'#fff'}).flatten({background:'#fff'}).png().toBuffer()
  const label=Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="370" height="20"><text x="7" y="15" font-family="Helvetica" font-size="12" fill="#242421">1:'+k+' '+view+'</text></svg>')
  parts.push({input:render,left:col*w+4,top:row*h+19},{input:label,left:col*w,top:row*h})
 }
 Bun.gc(true)
}
const file=join(import.meta.dir,'🗑️generated',a.id+'-paper-'+hash+'.png')
await sharp({create:{width:3*w,height:4*h,channels:3,background:'#fff'}}).composite(parts).png().toFile(file)
console.log('[DEBUG]',file)
