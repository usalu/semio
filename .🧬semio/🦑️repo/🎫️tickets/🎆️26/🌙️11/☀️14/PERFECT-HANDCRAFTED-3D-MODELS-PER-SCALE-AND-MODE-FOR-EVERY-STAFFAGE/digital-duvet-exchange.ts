/** 🌡️ Measure the upper quilt and upper pyjama cloth exposed outside its footprint at the default pose. */
import {readFileSync} from 'node:fs'
import {Vector3,Triangle,Raycaster,Mesh,MeshBasicMaterial,BufferGeometry,Float32BufferAttribute,DoubleSide} from 'three'
import {posedMesh} from './living-world-mesh.ts'
import {qualityOf} from '../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts'
const a=JSON.parse(readFileSync(process.argv[2],'utf8')).asset,m=posedMesh(a,'supine'),q=qualityOf(a,50),vol=a.volumes[50],D=new Vector3(0,-1,0),ray=new Raycaster(),mat=new MeshBasicMaterial({side:DoubleSide})
const cover=vol.body.filter(s=>s.id.startsWith('baffle-')).map(s=>{const {world,indices}=m(s,q),g=new BufferGeometry();g.setAttribute('position',new Float32BufferAttribute(world,3));g.setIndex(Array.from(indices));g.computeBoundingSphere();return new Mesh(g,mat)})
let garment=0
for(const s of vol.body.filter(p=>vol.thermal.some(t=>t.of===p.id&&t.layer==='pyjama-top'))){
 const {world,indices}=m(s,q),v=id=>new Vector3().fromArray(world,3*id);let visible=0
 for(let j=0;j<indices.length;j+=3){
  const tri=new Triangle(v(indices[j]),v(indices[j+1]),v(indices[j+2])),c=tri.getMidpoint(new Vector3())
  ray.set(new Vector3(c.x,2,c.z),D);const hits=ray.intersectObjects(cover,false)
  if((!hits.length||c.y>=hits[0].point.y-.001)&&c.y>=.462)visible+=tri.getArea()
 }
 garment+=visible;console.log(s.id,visible)
}
console.log('Uncovered pyjama area in square metres',garment)
