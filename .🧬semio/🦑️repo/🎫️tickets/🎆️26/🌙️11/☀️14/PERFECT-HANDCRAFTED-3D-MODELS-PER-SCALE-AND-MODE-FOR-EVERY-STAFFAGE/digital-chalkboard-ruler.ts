/** 📏 Put the one-metre bevelled teaching ruler on its tray in the fine orthographic drawing. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset
const ruler=a.volumes['50'].body.find(s=>s.id==='ruler'),rep=a.representations['50']
const q=rep.geometry.elevation.find(s=>s.id==='ruler')
Object.assign(q,{x:ruler.from,w:ruler.to-ruler.from,y:.00125,h:.0075})
writeFileSync(process.argv[3],JSON.stringify(doc))
