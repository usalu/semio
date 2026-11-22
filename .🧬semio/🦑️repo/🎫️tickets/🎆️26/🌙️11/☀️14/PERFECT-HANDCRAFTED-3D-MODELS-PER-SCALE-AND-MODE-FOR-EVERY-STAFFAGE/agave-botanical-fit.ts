/** 🌵 Keep the tapered fibrous midrib and water-storing leaf section independent at 1:500. */
import {readFileSync,writeFileSync} from 'node:fs'
import {Vector3} from 'three'
const doc=JSON.parse(readFileSync(process.argv[2],'utf8')),a=doc.asset,v=a.volumes[500],fine=a.volumes[50],medium=a.volumes[200]
function sample(source,target){
 const sn=source.triangles[0][2],tn=target.triangles[0][2],sr=(source.vertices.length-2)/sn,tr=(target.vertices.length-2)/tn
 const center=j=>source.vertices.slice(j*sn,(j+1)*sn).reduce((v,p)=>v.add(new Vector3(...p)),new Vector3()).divideScalar(sn)
 const centers=Array.from({length:sr},(_,j)=>center(j)),travel=[0]
 for(let j=1;j<sr;j++)travel.push(travel.at(-1)+centers[j].distanceTo(centers[j-1]))
 const at=(j,t)=>{const r=t*sn,l=Math.floor(r)%sn;return new Vector3(...source.vertices[j*sn+l]).lerp(new Vector3(...source.vertices[j*sn+(l+1)%sn]),r-Math.floor(r))}
 const vertices=[]
 for(let j=0;j<tr;j++){
  const t=travel.at(-1)*j/(tr-1),h=Math.min(sr-2,Math.max(0,travel.findIndex((r,i)=>i+1<sr&&travel[i+1]>=t))),frac=(t-travel[h])/(travel[h+1]-travel[h])
  for(let i=0;i<tn;i++)vertices.push(at(h,i/tn).lerp(at(h+1,i/tn),frac).toArray().map(x=>+x.toFixed(7)))
 }
 vertices.push(...[centers[0],centers.at(-1)].map(v=>v.toArray().map(x=>+x.toFixed(7))))
 target.vertices=vertices
}
for(const q of v.body.filter(s=>s.id.startsWith('leaf-')))sample(medium.body.find(s=>s.id===q.id),q)
for(const q of v.structure.filter(s=>s.id.startsWith('m-leaf-'))){
 sample(fine.structure.find(s=>s.id===q.id),q);q.slender=true
}
for(const s of v.thermal.filter(s=>s.tags?.includes('emitter'))){
 const surface=v.body.find(q=>q.id===s.bone)
 if(!surface)continue
 const {id,region,tags}=s
 for(const k of Object.keys(s))delete s[k]
 Object.assign(s,{id,region,tags,of:surface.id})
}
v.decisions.unshift({en:'Each of the 28 leaves keeps its own width, fleshy section and narrower fibrous midrib. The central support follows the measured taper from the blade base to its tip.',de:'Jedes der 28 Blätter behält seine Breite, seinen fleischigen Querschnitt und seine schmalere Faserrippe. Der zentrale Träger folgt der Verjüngung vom Blattgrund bis zur Spitze.'})
writeFileSync(process.argv[3],JSON.stringify(doc))
