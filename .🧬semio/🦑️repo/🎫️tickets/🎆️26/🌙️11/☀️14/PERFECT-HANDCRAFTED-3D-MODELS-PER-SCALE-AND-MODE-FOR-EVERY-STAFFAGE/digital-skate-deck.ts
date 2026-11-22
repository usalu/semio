/** 🛹 Retain the skateboard's concave maple stock, two bolted trucks and four rotating wheels at each scale. */
import {readFileSync,writeFileSync} from 'node:fs'
import {meshesAt,simplifyRing,area} from './volumes-lib.ts'
import {projectMask,contours,inverse} from './r3-project2d.ts'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,fine=a.volumes['50']
const bones=new Set(['board','wheel-f','wheel-r']),regions=new Set(['deck','trucks','wheels'])
const stations={
 '100':[0,1,2,3,4,5,6,8,10,12,13,14,16,18,20,21,22,23,24,25,26],
 '200':[0,1,2,4,6,8,10,13,16,18,20,22,24,25,26],
 '500':[0,2,4,6,9,13,17,20,22,24,26]
}
for(const [k,take] of Object.entries(stations)){
 const v=a.volumes[k],parts=fine.body.filter(q=>bones.has(q.bone)&&(!q.id.startsWith('grip')||k==='100')&&(!q.id.startsWith('pivot')||k!=='500'))
 const revised=parts.map(q=>{
  const s=structuredClone(q)
  if(s.id.startsWith('deck')||s.id.startsWith('grip'))s.sections=take.map(i=>s.sections[i])
  return s
 })
 const retained=new Set(revised.map(q=>q.id))
 v.body=[...v.body.filter(q=>!bones.has(q.bone)),...revised]
 v.thermal=[...v.thermal.filter(q=>!regions.has(q.region)),...structuredClone(fine.thermal.filter(q=>retained.has(q.of)))]
 v.structure=[...v.structure.filter(q=>!bones.has(q.member)),...structuredClone(fine.structure.filter(q=>bones.has(q.member)&&(!q.of||retained.has(q.of))))]
 const form=v.decisions[0],suffix={en:'The deck retains an 11.5 mm maple layup, a concave section and two kicks. Each truck supports two 54 mm road wheels on a 171 mm track.',de:'Das konkave Ahorndeck bleibt 11,5 mm stark und steigt an Nose und Tail an. Jede Achse trägt zwei 54-mm-Rollen bei 171 mm Spurweite.'}
 for(const lang of ['en','de']){
  const original=form[lang]
  const start=({en:['; the deck',', the deck','; the board',', the board'],de:['; das Deck',', das Deck','; das Brett',', das Brett']})[lang].map(q=>original.indexOf(q)).filter(i=>i>=0)
  form[lang]=(start.length?original.slice(0,Math.min(...start))+'.':original)+' '+suffix[lang]
 }
 if(k==='500'){
  const meshes=meshesAt(a,500,revised)
  for(const [view,shapes] of Object.entries(a.representations[k].geometry)){
   if(view==='plan')continue
   const meta=shapes.find(q=>q.id==='deck'),mask=projectMask(view,meshes,.0018,10)
   const source=mask.cells.slice(),radius=6
   for(let y=radius;y<mask.height-radius;y++)for(let x=radius;x<mask.width-radius;x++)if(source[y*mask.width+x])
    for(let v=-radius;v<=radius;v++)for(let u=-radius;u<=radius;u++)if(u*u+v*v<=radius*radius&&(view!=='elevation'||Math.abs(u)<=1))mask.cells[(y+v)*mask.width+x+u]=1
   const rings=contours(mask.cells,mask.width,mask.height,mask.box,mask.cell).sort((x,y)=>Math.abs(area(y))-Math.abs(area(x)))
   const target=simplifyRing(rings[0],.003)
   const projected=inverse(a,500,view,[{meta,target}])[0]
   if(projected.residualM>.00001)throw Error(view)
   meta.points=projected.points.map(p=>[+p.x.toFixed(6),+p.y.toFixed(6)])
  }
  v.decisions[1]={en:'The heat field follows the deck, hangers, axles and four wheel surfaces. The rider retains the face and garment surfaces and respiratory outlets.',de:'Das Temperaturfeld folgt Deck, Achskörpern, Achsstiften und den vier Rollen. Gesicht, Kleidung und Atemauslässe bleiben dem Fahrer zugeordnet.'}
  v.decisions[2]={en:'The rider retains separate spine, open ribs, shoulder girdle, limb, joint and mass geometry. The board transfers load through the deck, baseplates, kingpins, bushings, hangers, axles and wheels.',de:'Wirbelsäule, offene Rippen, Schultergürtel, Gliedmaßen, Gelenke und Massen sind getrennt modelliert. Das Brett überträgt die Last über Deck, Grundplatten, Königsbolzen, Lenkgummis, Achskörper, Achsstifte und Rollen.'}
 }
}
writeFileSync(process.argv[3],JSON.stringify(doc))
