import fs from 'node:fs/promises';
import {stripTypeScriptTypes} from 'node:module';
import {pathToFileURL} from 'node:url';
export async function run(root,ticket){
const folder=ticket+'/🗑️generated/boolean-peer';await fs.mkdir(folder,{recursive:true});
const source=root+'/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📐️geometry/🧊️boolean/🟦️.ts';
await fs.writeFile(folder+'/boolean.mjs',stripTypeScriptTypes(await fs.readFile(source,'utf8'),{mode:'transform'}));
const {subtractSolid}=await import(pathToFileURL(folder+'/boolean.mjs').href+'?v='+Date.now());
const box=(a,b)=>({vertices:[[a[0],a[1],a[2]],[b[0],a[1],a[2]],[b[0],b[1],a[2]],[a[0],b[1],a[2]],[a[0],a[1],b[2]],[b[0],a[1],b[2]],[b[0],b[1],b[2]],[a[0],b[1],b[2]]],triangles:[[0,2,1],[0,3,2],[4,5,6],[4,6,7],[0,1,5],[0,5,4],[1,2,6],[1,6,5],[2,3,7],[2,7,6],[3,0,4],[3,4,7]]});
const join=(a,b)=>({vertices:[...a.vertices,...b.vertices],triangles:[...a.triangles,...b.triangles.map(t=>t.map(i=>i+a.vertices.length))]});
const a=box([0,0,0],[2,2,2]),far=box([4,4,4],[5,5,5]),inner=box([.5,.5,.5],[1.5,1.5,1.5]),results=[];
const probe=(name,left,right,options={})=>{try{const m=subtractSolid(left,right,options);results.push({name,accepted:true,vertices:m.vertices.length,triangles:m.triangles.length});}catch(error){results.push({name,accepted:false,error:String(error)});}};
probe('inverted',{...a,triangles:a.triangles.map(([i,j,k])=>[i,k,j])},far);
probe('zero-volume',box([0,0,0],[2,2,0]),far);probe('duplicate-sheets',join(a,a),far);
probe('edge-tangent',a,box([2,2,0],[3,3,2]));probe('point-tangent',a,box([2,2,2],[3,3,3]));
probe('face-contact-components',join(a,box([2,0,0],[3,2,2])),far);probe('disconnected',join(a,box([3,0,0],[4,1,1])),far);
probe('maxpolygons-disjoint',a,far,{maxPolygons:1});probe('maxpolygons-contained',a,inner,{maxPolygons:1});probe('maxpolygons-emptyright',a,{vertices:[],triangles:[]},{maxPolygons:1});
const flipped=structuredClone(a);flipped.triangles[0].reverse();probe('one-flipped-face',flipped,far);
const crossing=structuredClone(a);crossing.vertices[6]=[-1,-1,-1];probe('self-crossing-deformed-cube',crossing,far);
const cavity=subtractSolid(a,inner);probe('cavity-disjoint',cavity,far);probe('cavity-identical',cavity,cavity);probe('disconnected-crosscut',join(a,box([3,0,0],[4,1,1])),box([1,-1,-1],[3.5,3,3]));
return results;
}
