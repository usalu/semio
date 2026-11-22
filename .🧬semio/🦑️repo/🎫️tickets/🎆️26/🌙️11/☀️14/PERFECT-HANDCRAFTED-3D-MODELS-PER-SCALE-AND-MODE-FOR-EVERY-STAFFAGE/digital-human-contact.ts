/** 🖼️ Tile four complete human sheets at a time, with the subject and pose identified in each quadrant. */
import {readFileSync} from 'node:fs'
import {join} from 'node:path'
import {createHash} from 'node:crypto'
import sharp from 'sharp'
import {Canvas} from './volumes-raster.ts'
import {catalogEntries} from './volumes-lib.ts'
const [which,group]=process.argv.slice(2),g=join(import.meta.dir,'🗑️generated'),dir=join(g,'living-current-views'),names=Object.keys(JSON.parse(readFileSync(join(import.meta.dir,'digital-human-gestures.json'),'utf8'))).slice(+group*4,+group*4+4),parts=[]
for(const [i,name] of names.entries()){
 const entry=catalogEntries().find(e=>e.slug===name),raw=readFileSync(entry.path),a=JSON.parse(raw).asset,h=createHash('sha256').update(raw).digest('hex')
 const pose=which==='second'?JSON.parse(readFileSync(join(import.meta.dir,'digital-human-gestures.json'),'utf8'))[name]:a.presentation.defaultPose
 const options=which==='second'?'-opposite-presentation-shaded-temperature':'-presentation-shaded-temperature',ideal=join(dir,name+'-'+pose+'-all'+options+'-'+h.slice(0,10)+'.png')
 const title=new Canvas(560,19,[230,233,230]);title.text(4,4,name.toUpperCase()+' / '+pose.toUpperCase(),[20,25,20],1)
 const x=i%2*560,y=Math.floor(i/2)*566
 parts.push({input:title.png(),left:x,top:y})
 parts.push({input:await sharp(ideal).resize(560,547).png().toBuffer(),left:x,top:y+19})
}
await sharp({create:{width:1120,height:1132,channels:3,background:'#f7f7f4'}}).composite(parts).png().toFile(join(g,'human-quad-'+which+'-'+group+'.png'))
