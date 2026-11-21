/** 🖼️ Arrange current-source preview panels for a side-by-side inspection of four complete assets. */
import {readFileSync,readdirSync,writeFileSync} from 'node:fs'
import {join} from 'node:path'
import {inflateSync} from 'node:zlib'
import {createHash} from 'node:crypto'
import {Canvas} from './volumes-raster.ts'
import {catalogEntries} from './volumes-lib.ts'
const params=process.argv.slice(2),temperature=params.includes('--temperature'),[page,...names]=params.filter(v=>v!=='--temperature')
const out=join(import.meta.dir,'🗑️generated/living-current-views')
const panel=new Canvas(960,Math.ceil(names.length/2)*490,[244,244,240])
for(const [n,name] of names.entries()){
 const entry=catalogEntries().find(e=>e.slug===name)!,sha=createHash('sha256').update(readFileSync(entry.path)).digest('hex').slice(0,10)
 const file=readdirSync(out).find(p=>p.startsWith(name+'-')&&p.endsWith((temperature?'-all-temperature-':'-all-')+sha+'.png'))
 if(!file){console.log('[DEBUG] queued',name);continue}
 const data=readFileSync(join(out,file)),w=data.readUInt32BE(16),h=data.readUInt32BE(20),parts=[]
 for(let i=8;i<data.length;){const size=data.readUInt32BE(i);if(data.toString('ascii',i+4,i+8)==='IDAT')parts.push(data.subarray(i+8,i+8+size));i+=12+size}
 const pixels=inflateSync(Buffer.concat(parts)),stride=w*3+1,cx=n%2*480,cy=Math.floor(n/2)*490
 panel.text(cx+4,cy+2,name,[59,63,55],2)
 for(let y=0;y<h;y+=2){
  if(pixels[y*stride]!==0||pixels[(y+1)*stride]!==0)throw Error('Unexpected filter')
  for(let x=0;x<w;x+=2){const i=y*stride+x*3+1;const rgb=[0,1,2].map(k=>(pixels[i+k]+pixels[i+3+k]+pixels[i+stride+k]+pixels[i+stride+3+k])/4)
    panel.set(cx+x/2,cy+19+y/2,rgb as [number,number,number])
  }
 }
}
writeFileSync(join(out,'contact-'+page+'.png'),panel.png())
