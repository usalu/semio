/** 🛝️ Independent native ramp product, aggregation, placement and scope validation. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
export async function verifyRampSteps(ticket,samples){
 const config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();
 await api.Init(name=>join(config.wasmDirectory,name));
 const result=[];
 try {for(const s of samples){
  const id=api.OpenModel(new TextEncoder().encode(s.text));
  try {
   const get=key=>api.GetLine(id,Number(s.ids[key].slice(1)));
   const product=get('element');
   if(product.type!==library[s.entity]||product.GlobalId.value!==s.guid||product.Name.value!==s.name)throw Error(s.tag+': native identity');
   if(get('ownership_pset').Name.value!=='Pset_MaterialOwnership'||get('ownership_BodyScope').NominalValue.value!=='exclusive_owned_material')throw Error(s.tag+': body scope');
   for(const [name,value] of [['FullMemberVolumeM3',s.full_volume],['OwnedVolumeM3',s.volume],['SharedVolumeM3',s.shared_volume],['ChildProductVolumeM3',s.child_volume]])if(Math.abs(get('ownership_'+name).NominalValue.value-value)>1e-9)throw Error(s.tag+': semantic volume '+name);
   if(get('ownership_SharedComponentReferences').NominalValue.value!==s.shared_refs||get('ownership_ThicknessDirection').NominalValue.value!=='vertical'||get('ownership_ThicknessDirectionBasis').NominalValue.value!=='inferred')throw Error(s.tag+': registered geometric convention');
   if(s.children.length){
    const relation=get('aggregation');
    if(relation.GlobalId.value!==s.relation_guid||relation.RelatingObject.value!==Number(s.ids.element.slice(1)))throw Error(s.tag+': native parent identity');
    const refs=relation.RelatedObjects.map(r=>r.value).sort((a,b)=>a-b),expected=s.children.map(c=>Number(c.ids.element.slice(1))).sort((a,b)=>a-b);
    if(JSON.stringify(refs)!==JSON.stringify(expected))throw Error(s.tag+': aggregate children');
   }
   let total=0,bodyParts=0;
   for(const p of [s,...s.children]){
    const identity=api.GetLine(id,Number(p.ids.element.slice(1)));
    if(identity.type!==library[p.entity]||identity.GlobalId.value!==p.guid)throw Error(p.tag+': child product identity');
   const mesh=api.GetFlatMesh(id,Number(p.ids.element.slice(1))),minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
   let volume=0,triangles=0;
   for(let i=0;i<mesh.geometries.size();i++){
    const part=mesh.geometries.get(i),geometry=api.GetGeometry(id,part.geometryExpressID),m=part.flatTransformation;
    const vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
    const point=index=>{
     const x=vertices[6*index],y=vertices[6*index+1],z=vertices[6*index+2];
     const p=[m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];
     for(let a=0;a<3;a++){minimum[a]=Math.min(minimum[a],p[a]);maximum[a]=Math.max(maximum[a],p[a]);}
     return p;
    };
    const origin=point(indices[0]);let signed=0;
    for(let j=0;j<indices.length;j+=3){
     const [a,b,c]=[0,1,2].map(k=>point(indices[j+k]).map((x,i)=>x-origin[i]));
     signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);
    }
    volume+=Math.abs(signed/6);triangles+=indices.length/3;geometry.delete();
   }
   if(Math.abs(volume-p.volume)>Math.max(1e-7,p.volume*1e-5))throw Error(p.tag+': volume '+volume+' != '+p.volume);
   for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)if(Math.abs([minimum,maximum][side][axis]-p.bounds[side][axis])>1e-5)throw Error(p.tag+': placed bounds');
    if(mesh.geometries.size()!==p.parts)throw Error(p.tag+': duplicate or missing owned body');
    total+=volume;bodyParts+=mesh.geometries.size();
   }
   if(Math.abs(total-s.volume-s.child_volume)>1e-6)throw Error(s.tag+': assembled native inventory');
   result.push({tag:s.tag,case:s.case,language:s.language,bodyParts,children:s.children.length,volume:total,fullVolume:s.full_volume,sharedVolume:s.shared_volume});
  }finally{api.CloseModel(id);}
 }}finally{api.Dispose();}
 return result;
}
