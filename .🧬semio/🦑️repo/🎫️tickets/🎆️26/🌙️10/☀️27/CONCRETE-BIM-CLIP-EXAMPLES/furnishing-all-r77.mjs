/** 🪑 Decode every real furnishing and return builtin native cap triangles for independent occupied-union integration. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
export async function verifyAll(ticket,samples,verifyNative){
 const verified=await verifyNative(ticket,samples),config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();await api.Init(name=>join(config.wasmDirectory,name));const decoded=[];
 try{for(const sample of samples){const id=api.OpenModel(new TextEncoder().encode(sample.text));
 try{const mesh=api.GetFlatMesh(id,Number(sample.ids.element.slice(1))),parts=[];
 for(let i=0;i<mesh.geometries.size();i++){
  const placed=mesh.geometries.get(i),geometry=api.GetGeometry(id,placed.geometryExpressID),m=placed.flatTransformation;
  const vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
  const points=Array.from({length:vertices.length/6},(_,index)=>{const x=vertices[6*index],y=vertices[6*index+1],z=vertices[6*index+2];return [m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];});
  const z0=Math.min(...points.map(p=>p[2])),z1=Math.max(...points.map(p=>p[2])),polygons=[];
  for(let j=0;j<indices.length;j+=3){const triangle=[points[indices[j]],points[indices[j+1]],points[indices[j+2]]];
   if(triangle.every(p=>p[2]===z1)){
    const [a,b,c]=triangle,cross=(b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0]);
    if(cross!==0){const ring=triangle.map(p=>[p[0],p[1]]);polygons.push([ring.concat([ring[0]])]);}
   }
  }
  if(!(z1>z0)||!polygons.length)throw Error(sample.tag+': no positive native prism top cap');parts.push({polygons,z0,z1});geometry.delete();
 }
 decoded.push({tag:sample.tag,language:sample.language,parts,source_owned_volume:sample.net_volume});
 }finally{api.CloseModel(id);}}}finally{api.Dispose();}return {...verified,decoded};
}
