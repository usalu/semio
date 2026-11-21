/** 🔍 Assemble model-labelled sheets of the exact rendered 1:50 bodies for the visual survey. */
import {readFileSync,readdirSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {inflateSync} from 'node:zlib'
import {createHash} from 'node:crypto'
import {catalogEntries} from './volumes-lib.ts'
import {Canvas} from './volumes-raster.ts'
const dir=join(import.meta.dir,'🗑️generated/living-current-views'),names=readdirSync(dir)
const items=catalogEntries().sort((p,q)=>p.path.localeCompare(q.path))
for(let at=0;at<items.length;at+=32){
 const page=new Canvas(1280,Math.ceil(Math.min(32,items.length-at)/8)*137,[248,248,246])
 for(const [k,item] of items.slice(at,at+32).entries()){
  const h=createHash('sha256').update(readFileSync(item.path)).digest('hex').slice(0,10)
  const path=names.find(n=>n.startsWith(item.slug+'-')&&n.endsWith('-all-temperature-'+h+'.png'))
  if(!path)continue
  const raw=readFileSync(join(dir,path)),w=raw.readUInt32BE(16),chunks=[]
  for(let i=8;i<raw.length;){const n=raw.readUInt32BE(i);if(raw.toString('ascii',i+4,i+8)==='IDAT')chunks.push(raw.subarray(i+8,i+8+n));i+=12+n}
  const pixels=inflateSync(Buffer.concat(chunks)),cx=(k%8)*160,cy=Math.floor(k/8)*137,stride=w*3+1
  for(let y=0;y<234;y+=2)for(let x=0;x<320;x+=2){const j=y*stride+3*x+1;page.set(cx+x/2,cy+y/2,[0,1,2].map(c=>(pixels[j+c]+pixels[j+c+3]+pixels[j+c+stride]+pixels[j+c+stride+3])/4))}
  page.text(cx+3,cy+119,item.slug,[40,40,40],1)
 }
 writeFileSync(join(dir,'survey-'+(at/32+1)+'.png'),page.png())
}
