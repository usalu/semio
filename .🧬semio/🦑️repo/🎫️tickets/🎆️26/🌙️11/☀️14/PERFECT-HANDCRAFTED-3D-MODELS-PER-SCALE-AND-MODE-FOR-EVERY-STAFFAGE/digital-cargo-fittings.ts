/** 🚲 Fit the cargo cycle's swept handlebars, touring saddle, cranks and folding centre stand to the existing chassis. */
import {readFileSync,writeFileSync} from 'node:fs'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,base=a.volumes['50']
const selected=id=>/^(?:handlebar-|bar-clamp|grip-|lever-|stand-|crank-|pedal-|saddle|seat-post)/.test(id)
for(const [scale,v] of Object.entries(a.volumes)){
 if(scale==='50')continue
 const byBody=new Set([...base.body,...v.body].filter(q=>selected(q.id)).map(q=>q.id))
 const includes=(q,list)=>list==='body'?selected(q.id):byBody.has(q.of)||selected(q.id.replace(/^th-/,''))||list==='structure'&&q.member==='stand'&&q.part==='member'
 for(const list of ['body','thermal','structure']){
  v[list]=[...v[list].filter(q=>!includes(q,list)),...base[list].filter(q=>includes(q,list)).map(q=>structuredClone(q))]
 }
 v.decisions.push({en:'The 25.4 mm swept handlebar, 19 mm centre-stand legs, seat post and crankset keep their physical sections. The stand folds on its pin under the front box.',de:'Der gekröpfte Lenker mit 25,4 mm Durchmesser, die 19-mm-Zweibeinstütze, die Sattelstütze und die Kurbelgarnitur behalten ihre realen Querschnitte. Der Ständer klappt unter der Transportbox um seinen Lagerbolzen.'})
}
writeFileSync(process.argv[3],JSON.stringify(doc))
