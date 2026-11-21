/** 🌳 Render a current source at one scale in body, thermal and structural view. */
import {createHash} from 'node:crypto'
import {mkdirSync,readFileSync} from 'node:fs'
import {join} from 'node:path'
import {catalogEntries} from './volumes-lib.ts'
import {renderScale} from './volumes-render.ts'
const [name,denom]=process.argv.slice(2)
const entry=catalogEntries().find(e=>e.slug===name)
if(!entry||!['50','100','200','500'].includes(denom!))throw Error('Supply an asset and denominator.')
const raw=readFileSync(entry.path,'utf8'),asset=JSON.parse(raw).asset
const mark=createHash('sha256').update(raw).digest('hex').slice(0,10)
const out=join(import.meta.dir,'🗑️generated','living-current-views')
mkdirSync(out,{recursive:true})
renderScale(asset,Number(denom),{},['body','thermal','structure'],{az:34,el:18,cut:false,width:900},mode=>join(out,name+'-'+denom+'-'+mark+'-'+mode+'.png'))

