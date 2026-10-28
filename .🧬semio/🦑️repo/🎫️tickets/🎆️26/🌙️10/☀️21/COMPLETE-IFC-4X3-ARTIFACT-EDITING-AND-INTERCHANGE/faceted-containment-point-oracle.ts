import {BoxGeometry,Mesh,MeshBasicMaterial,DoubleSide,Raycaster,Vector3,REVISION} from 'three';
import {writeFileSync} from 'node:fs';
const box=new Mesh(new BoxGeometry(2,1,3),new MeshBasicMaterial({side:DoubleSide}));box.position.set(1,.5,1.5);box.updateMatrixWorld(true);
const direction=new Vector3(1,.173,.317).normalize();
const points=[{id:'inside-cavity-centre',point:[1,.5,1.5],inside:true},{id:'outside-cavity-centre',point:[3,.5,1.5],inside:false},{id:'crossing-cavity-centre',point:[1.8,.5,1.5],inside:true},{id:'crossing-cavity-exterior-corner',point:[2.3,.75,2],inside:false}];
const output=points.map(row=>{const hits=new Raycaster(new Vector3(...row.point),direction).intersectObject(box);const distances=hits.map(h=>h.distance).filter((v,i,a)=>i===0||Math.abs(v-a[i-1])>1e-10);if((distances.length%2===1)!==row.inside)throw Error(row.id);return{...row,uniqueForwardHits:distances.length};});
writeFileSync(import.meta.dir+'/🗑️generated/faceted-containment-point-oracle.json',JSON.stringify({library:'Three '+REVISION,rows:output,scope:'Simple finite box parity corroboration; centre alone does not prove complete cavity containment'},null,2));console.log('[DEBUG] four simple box parity observations match');
