/** 🔬 Read actual native representation meshes without treating their tessellation as analytic material authority. */
const productInterval=(values,errors)=>{let lower=Infinity,upper=-Infinity;for(let bits=0;bits<8;bits++){let value=1;for(let axis=0;axis<3;axis++)value*=values[axis]+((bits>>axis)&1?errors[axis]:-errors[axis]);lower=Math.min(lower,value);upper=Math.max(upper,value);}return[lower,upper];};

/** 🧭 Select exactly one named native representation before requesting any cached mesh. */
export function selectRepresentation(api,model,element,name){
 const product=api.GetLine(model,element),shape=api.GetLine(model,product.Representation.value),selected=shape.Representations.filter(ref=>api.GetLine(model,ref.value).RepresentationIdentifier?.value===name);
 if(selected.length!==1)throw Error('A native representation must be selected explicitly: '+name);
 shape.Representations=selected;api.WriteLine(model,shape);
}

/** 📐 Retain strict cell closure and a determinant interval for actual Float32 mesh quantization only. */
export function inspectMesh(api,model,element){
 const mesh=api.GetFlatMesh(model,element),rows=[];let total=0,minimum=0,maximum=0;
 for(let index=0;index<mesh.geometries.size();index++){
  const part=mesh.geometries.get(index),geometry=api.GetGeometry(model,part.geometryExpressID),matrix=part.flatTransformation,vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
  const point=index=>{const x=vertices[6*index],y=vertices[6*index+1],z=vertices[6*index+2];return[matrix[0]*x+matrix[4]*y+matrix[8]*z+matrix[12],-(matrix[2]*x+matrix[6]*y+matrix[10]*z+matrix[14]),matrix[1]*x+matrix[5]*y+matrix[9]*z+matrix[13]];};
  const error=index=>{const local=[vertices[6*index],vertices[6*index+1],vertices[6*index+2]].map(value=>Math.abs(value)*2**-24+2**-150);return[0,2,1].map(axis=>Math.abs(matrix[axis])*local[0]+Math.abs(matrix[4+axis])*local[1]+Math.abs(matrix[8+axis])*local[2]);};
  const edges=new Map(),origin=indices.length?point(indices[0]):[0,0,0],originError=indices.length?error(indices[0]):[0,0,0];let signed=0,lower=0,upper=0,collapsed=0;
  for(let offset=0;offset<indices.length;offset+=3){const triangle=[0,1,2].map(k=>point(indices[offset+k]));for(let side=0;side<3;side++){const a=triangle[side].join('|'),b=triangle[(side+1)%3].join('|');if(a===b)collapsed++;const key=a<b?a+'>'+b:b+'>'+a;edges.set(key,(edges.get(key)||0)+1);}
   const values=triangle.map(row=>row.map((value,axis)=>value-origin[axis])),errors=[0,1,2].map(k=>error(indices[offset+k]).map((value,axis)=>value+originError[axis]));
   for(const[axes,sign]of[[[0,1,2],1],[[0,2,1],-1],[[1,2,0],1],[[1,0,2],-1],[[2,0,1],1],[[2,1,0],-1]]){const terms=values.map((row,k)=>row[axes[k]]),bounds=productInterval(terms,errors.map((row,k)=>row[axes[k]]));signed+=sign*terms.reduce((a,b)=>a*b,1);lower+=sign>0?bounds[0]:-bounds[1];upper+=sign>0?bounds[1]:-bounds[0];}
  }
  const volume=Math.abs(signed/6),lo=(lower<=0&&upper>=0?0:Math.min(Math.abs(lower),Math.abs(upper)))/6,hi=Math.max(Math.abs(lower),Math.abs(upper))/6;total+=volume;minimum+=lo;maximum+=hi;rows.push({triangles:indices.length/3,volume,closed:indices.length>0&&collapsed===0&&[...edges.values()].every(count=>count===2),positive:volume>0,collapsed_edges:collapsed,float32_volume_interval:[lo,hi]});geometry.delete();
 }
 return{cells:rows.length,volume:total,float32_volume_interval:[minimum,maximum],rows,scope:'Native tessellation diagnostic. Float32 interval bounds numeric quantization of this mesh, not error from approximating analytic curves.'};
}
