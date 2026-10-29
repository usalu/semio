/** 🧱 Validate native product identity, material coverage and occupied body with an independent IFC engine. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
export async function verifyProducts(ticket,samples){
 const config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));
 const results=[];
 try{for(const s of samples){
 const id=api.OpenModel(new TextEncoder().encode(s.text));
 try{
 const get=k=>api.GetLine(id,Number(s.ids[k].slice(1))),product=get('element');
 let predefined=product.PredefinedType?.value;
 if(predefined===undefined&&s.ids.rel_type){
 const relation=get('rel_type'),elementId=Number(s.ids.element.slice(1));
 if(relation.type!==library.IFCRELDEFINESBYTYPE||!relation.RelatedObjects.some(ref=>ref.value===elementId))throw Error(s.tag+': native type relationship');
 const assigned=api.GetLine(id,relation.RelatingType.value);
 if(assigned.type!==library[s.entity+'TYPE'])throw Error(s.tag+': native assigned type class');
 predefined=assigned.PredefinedType?.value;
 }
 if(product.type!==library[s.entity]||product.GlobalId?.value!==s.guid||product.Name?.value!==s.name||predefined!==s.predefined)throw Error(s.tag+': native product identity '+JSON.stringify({entity:product.type,guid:product.GlobalId?.value,name:product.Name?.value,predefined,expected:s.predefined}));
 if(get('representation').RepresentationType.value!==s.representation_type)throw Error(s.tag+': native body representation');
 const mesh=api.GetFlatMesh(id,Number(s.ids.element.slice(1))),minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];let volume=0,triangles=0;
 for(let i=0;i<mesh.geometries.size();i++){
 const part=mesh.geometries.get(i),geometry=api.GetGeometry(id,part.geometryExpressID),m=part.flatTransformation;
 const vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
 const point=index=>{const x=vertices[6*index],y=vertices[6*index+1],z=vertices[6*index+2],p=[m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];for(let a=0;a<3;a++){minimum[a]=Math.min(minimum[a],p[a]);maximum[a]=Math.max(maximum[a],p[a]);}return p;};
 const origin=point(indices[0]);let signed=0;
 for(let j=0;j<indices.length;j+=3){const[a,b,c]=[0,1,2].map(k=>point(indices[j+k]).map((x,i)=>x-origin[i]));signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);}
 volume+=Math.abs(signed/6);triangles+=indices.length/3;geometry.delete();
 }
 if(mesh.geometries.size()!==s.parts)throw Error(s.tag+': physical part count');
 if(!Number.isFinite(volume)||Math.abs(volume-s.volume)>Math.max(1e-7,s.volume*1e-5))throw Error(s.tag+': occupied volume '+volume+' != '+s.volume);
 for(let side=0;side<2;side++)for(let a=0;a<3;a++)if(Math.abs([minimum,maximum][side][a]-s.bounds[side][a])>1e-5)throw Error(s.tag+': placed bounds');
 results.push({tag:s.tag,language:s.language,parts:mesh.geometries.size(),triangles,volume,bounds:[minimum,maximum]});
 }finally{api.CloseModel(id);}
 }}finally{api.Dispose();}return results;
}
