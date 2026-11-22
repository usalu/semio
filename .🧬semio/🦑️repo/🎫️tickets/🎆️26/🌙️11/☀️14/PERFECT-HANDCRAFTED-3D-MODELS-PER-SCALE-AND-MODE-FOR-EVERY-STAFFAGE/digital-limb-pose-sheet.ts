/** 🦴 Arrange the coarse canonical body, heat field and skeleton for each repaired adult pose. */
import sharp from 'sharp'
import {readFileSync,readdirSync} from 'node:fs'
import {join} from 'node:path'
import {createHash} from 'node:crypto'
import {catalogEntries} from './volumes-base.ts'
const out=join(import.meta.dir,'🗑️generated'),views=join(out,'living-current-views')
const names=JSON.parse(readFileSync(join(import.meta.dir,'digital-limb-gauge.json'),'utf8')).subjects
const tiled=[]
for(const [i,name] of names.entries()){
 const path=catalogEntries().find(q=>q.slug===name).path,h=createHash('sha256').update(readFileSync(path)).digest('hex').slice(0,10)
 const file=readdirSync(views).find(f=>f.startsWith(name+'-')&&f.endsWith('-all-presentation-shaded-temperature-'+h+'.png'))
 if(!file)throw Error(name)
 const input=await sharp(join(views,file)).extract({left:0,top:705,width:960,height:235}).resize(580,142).png().toBuffer()
 tiled.push({input,left:0,top:i*162+19})
 tiled.push({input:Buffer.from('<svg width="580" height="19"><text x="8" y="15" font-family="sans-serif" font-size="13">'+name+'</text></svg>'),left:0,top:i*162})
}
await sharp({create:{width:580,height:162*names.length,channels:3,background:'#f6f6f2'}}).composite(tiled).png().toFile(join(out,'eight-adult-control-poses.png'))
