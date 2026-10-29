/** 🏗️ Independent native RC column, structural slab and shared-member ownership proof. */
import {createRequire} from 'node:module';
import {join} from 'node:path';
import {writeFile} from 'node:fs/promises';
import {setupNativeIfcEngine} from './native-ifc-setup-r18.mjs';
export async function verifyRoofBearing(ticket,samples){
 const config=await setupNativeIfcEngine(join(ticket,'🗑️generated','joint-ifc-r30'));
 const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();
 await api.Init(name=>join(config.wasmDirectory,name));
 const results=[],products=[];
 try{for(const s of samples){
  const id=api.OpenModel(new TextEncoder().encode(s.text),{CIRCLE_SEGMENTS:192});
  try{
   const get=key=>api.GetLine(id,Number(s.ids[key].slice(1))),read=ref=>api.GetLine(id,ref.value),coords=ref=>read(ref).Coordinates.map(x=>x.value);
   const product=get('element');
   if(product.type!==library[s.entity]||product.GlobalId.value!==s.guid||product.Name.value!==s.name)throw Error(s.tag+': native identity');
   const prisms=[];
   for(let i=0;i<s.parts;i++){
    const solid=get('solid'+i),profile=read(solid.SweptArea),axis=read(solid.Position),position=coords(axis.Location),depth=solid.Depth.value;
    const direction=read(solid.ExtrudedDirection).DirectionRatios.map(v=>v.value);
    if(JSON.stringify(direction)!=='[0,0,1]'||axis.Axis)throw Error(s.tag+': unexpected native sweep placement');
    let outline,kind='polygon',radius;
    if(profile.type===library.IFCARBITRARYCLOSEDPROFILEDEF){
     const curve=read(profile.OuterCurve);outline=curve.Points.map(ref=>coords(ref));
     if(JSON.stringify(outline[0])===JSON.stringify(outline.at(-1)))outline.pop();
    }else{
     const plane=read(profile.Position),centre=coords(plane.Location),d=plane.RefDirection?read(plane.RefDirection).DirectionRatios.map(x=>x.value):[1,0];
     if(profile.type===library.IFCRECTANGLEPROFILEDEF){
      const x=profile.XDim.value/2,y=profile.YDim.value/2;
      outline=[[-x,-y],[x,-y],[x,y],[-x,y]].map(([x,y])=>[centre[0]+d[0]*x-d[1]*y,centre[1]+d[1]*x+d[0]*y]);
     }else if(profile.type===library.IFCCIRCLEPROFILEDEF){
      kind='circle';radius=profile.Radius.value;
      outline=Array.from({length:96},(_,i)=>[centre[0]+radius*Math.cos(2*Math.PI*i/96),centre[1]+radius*Math.sin(2*Math.PI*i/96)]);
     }else throw Error(s.tag+': unexpected native profile');
    }
    outline=outline.map(p=>[p[0]+position[0],p[1]+position[1]]);
    prisms.push({outline,z0:position[2],z1:position[2]+depth,kind,radius});
   }
   const area=ring=>Math.abs(ring.reduce((sum,a,i)=>{const b=ring[(i+1)%ring.length];return sum+a[0]*b[1]-b[0]*a[1];},0))/2;
   const analytic=prisms.reduce((sum,p)=>sum+(p.kind==='circle'?Math.PI*p.radius*p.radius:area(p.outline))*(p.z1-p.z0),0);
   if(Math.abs(analytic-s.volume)>Math.max(1e-8,s.volume*1e-9))throw Error(s.tag+': native profile/depth volume '+analytic+' != '+s.volume);
   if(s.entity==='IFCCOLUMN'){
    const materials=Object.keys(s.ids).filter(k=>k==='material'||/^component_material[0-9]+$/.test(k)).map(k=>get(k).Name.value);
    if(!materials.length||materials.some(name=>name!==s.material_name))throw Error(s.tag+': native RC material identity');
    if(!get('historical_ReconstructionUncertainty').NominalValue.value.includes(s.language==='en'?'inferred embedded junction':'abgeleiteter eingebetteter Knoten'))throw Error(s.tag+': inferred construction scope');
   }else{
    const scope=s.entity==='IFCSLAB'?'exclusive_owned_structural_material':'exclusive_owned_material';
    if(get('ownership_BodyScope').NominalValue.value!==scope)throw Error(s.tag+': body scope');
    for(const [key,value] of [['FullMemberVolumeM3',s.full_volume],['OwnedVolumeM3',s.volume],['SharedVolumeM3',s.shared_volume]])if(Math.abs(get('ownership_'+key).NominalValue.value-value)>1e-9)throw Error(s.tag+': native material quantity');
    if(get('ownership_SharedOwnerTags').NominalValue.value!==s.owners.join('; '))throw Error(s.tag+': native shared owner references');
    if(s.entity==='IFCSLAB')for(const [key,value] of [['FullFinishVolumeM3',s.finish_volume],['OwnedFinishVolumeM3',s.owned_finish_volume],['SharedFinishVolumeM3',s.shared_finish_volume]])if(Math.abs(get('ownership_'+key).NominalValue.value-value)>1e-9)throw Error(s.tag+': finish scope');
   }
   const mesh=api.GetFlatMesh(id,Number(s.ids.element.slice(1))),minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
   let volume=0,curveBound=0,triangles=0;const circlesSeen=new Set();
   for(let i=0;i<mesh.geometries.size();i++){
    const part=mesh.geometries.get(i),geometry=api.GetGeometry(id,part.geometryExpressID),m=part.flatTransformation;
    const v=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
    const point=index=>{
     const x=v[6*index],y=v[6*index+1],z=v[6*index+2],p=[m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];
     for(let a=0;a<3;a++){minimum[a]=Math.min(minimum[a],p[a]);maximum[a]=Math.max(maximum[a],p[a]);}
     return p;
    };
    const origin=point(indices[0]);let signed=0;
    for(let j=0;j<indices.length;j+=3){
     const [a,b,c]=[0,1,2].map(k=>point(indices[j+k]).map((x,i)=>x-origin[i]));
     signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);
    }
    for(const circle of s.circle_components){
     const boundary=new Set();
     for(const index of indices){const p=point(index);if(Math.abs(Math.hypot(p[0]-circle.axis[0],p[1]-circle.axis[1])-circle.radius)<1e-5)boundary.add(p.slice(0,2).map(v=>v.toFixed(7)).join(','));}
     const n=boundary.size;
     if(n===0)continue;
     const t=2*Math.PI/n;circlesSeen.add(circle.part);
     if(n<12)throw Error(s.tag+': insufficient independent circle mesh '+n);
     curveBound+=Math.PI*circle.radius**2*circle.height*(1-Math.sin(t)/t);
    }
    volume+=Math.abs(signed/6);triangles+=indices.length/3;geometry.delete();
   }
   if(circlesSeen.size!==s.circle_components.length)throw Error(s.tag+': missing native circular component');
   if(Math.abs(volume-s.volume)>curveBound+Math.max(1e-7,s.volume*1e-6))throw Error(s.tag+': native placed mesh volume '+volume+' != '+s.volume);
   for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)if(Math.abs([minimum,maximum][side][axis]-s.bounds[side][axis])>1e-5)throw Error(s.tag+': actual placed bounds');
   if(mesh.geometries.size()!==s.parts)throw Error(s.tag+': native body count');
   results.push({tag:s.tag,language:s.language,parts:mesh.geometries.size(),triangles,volume,analytic,curveBound});
   if(s.language==='en')products.push({tag:s.tag,parts:prisms});
  }finally{api.CloseModel(id);}
 }}finally{api.Dispose();}
 await writeFile(join(ticket,'🗑️generated','roof-bearing-native-profiles-r43.json'),JSON.stringify(products));
 return results;
}
