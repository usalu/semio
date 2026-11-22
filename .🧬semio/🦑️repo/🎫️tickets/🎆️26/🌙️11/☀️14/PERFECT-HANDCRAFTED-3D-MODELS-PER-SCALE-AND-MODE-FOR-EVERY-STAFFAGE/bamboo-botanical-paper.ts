/** 🎋 Project each physical axil and its blade onto the print drawing of the same branch and heat surface. */
import {Vector3,Quaternion} from 'three'
import {frames,toWorld} from './volumes-base.ts'
import {hull,outline,pathOf,project,toDrawing} from './r3-weeping-willow-kit.ts'
export function drawBamboo(a){
 const fr=frames(a)
 for(const k of [100,200,500]){
  const v=a.volumes[k],surface=new Map(v.thermal.map(q=>[q.of,q.region]))
  for(const [view,shapes] of Object.entries(a.representations[k].geometry)){
   const polygons=new Map()
   for(const q of v.body.filter(q=>q.material==='bamboo-leaf')){
    const key=q.bone+'-'+surface.get(q.id),group=polygons.get(key)??[],rotation=new Quaternion(...(q.transform?.rotation??[0,0,0,1])),translation=new Vector3(...(q.transform?.translation??[0,0,0]))
    const pt=q.vertices.map(p=>project(view,toWorld(fr[q.bone],new Vector3(...p).applyQuaternion(rotation).add(translation).toArray())))
    if(q.id.startsWith('apex')||/-f\d+-l\d+$/.test(q.id))group.push(hull(pt))
    else{
     const J=(pt.length-8)/11,rows=2*J+2
     for(let j=0;j<rows-1;j++)group.push(hull(pt.slice(3*j,3*(j+2))))
     for(let j=0;j<J;j++){
      const row=2*j+1,start=3*rows+5*j
      group.push(hull([...pt.slice(row*3,row*3+6),...pt.slice(start,start+5)]))
     }
    }
    polygons.set(key,group)
   }
   const former=shapes.filter(s=>s.region?.startsWith('leaves-')),drawn=new Set()
   for(let i=shapes.length-1;i>=0;i--){const q=shapes[i];if(q.region?.startsWith('leaves-')&&!polygons.has(q.bone+'-'+q.region))shapes.splice(i,1)}
   for(const key of polygons.keys()){
    if(shapes.some(q=>q.bone+'-'+q.region===key))continue
    const split=key.indexOf('-leaves-'),bone=key.slice(0,split),region=key.slice(split+1),sample=former.find(q=>q.bone===bone)
    if(!sample)throw Error('Missing foliage bone '+bone)
    shapes.push({...structuredClone(sample),id:key,region,z:sample.z+.001,tone:region==='leaves-sun'?.67:.39})
   }
   for(const shape of shapes.filter(s=>s.region?.startsWith('leaves-'))){
    const key=shape.bone+'-'+shape.region,group=polygons.get(key)
    const rings=outline(group.filter(p=>p.length>=3),.0008,.000006)
    if(!rings.length)throw Error('Empty projection '+key)
    delete shape.points;delete shape.closed
    shape.kind='path';shape.d=pathOf(toDrawing(view,fr[shape.bone],rings));shape.filled=true;drawn.add(key)
   }
   for(const key of polygons.keys())if(!drawn.has(key))throw Error('Missing printed branch '+view+' '+key)
  }
 }
}
