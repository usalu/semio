/** 🌀 Test-only independent IFC reader for the solarium's occupied wall and tapered crest. */
import {createRequire} from "node:module";
import {join} from "node:path";

/** 🧮 Integrate oriented IFC triangles after their native placements, retaining plain values only. */
export async function verifyRoofSteps(config,samples) {
  const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();
  await api.Init(name=>join(config.wasmDirectory,name));
  const results=[];
  try {
    for(const sample of samples) {
      const id=api.OpenModel(new TextEncoder().encode(sample.text));
      if(id<0)throw new Error("Native roof STEP rejected");
      try {
        const element=Number(sample.ids.element.slice(1)),product=api.GetLine(id,element);
        if(product.type!==library.IFCWALL||product.Name.value!==sample.name)throw new Error("Native localized wall identity changed");
        const tapered=Object.keys(sample.ids).filter(k=>/^solid\d+$/.test(k)).map(k=>api.GetLine(id,Number(sample.ids[k].slice(1)))).filter(s=>s.type===library.IFCEXTRUDEDAREASOLIDTAPERED);
        if(tapered.length!==sample.tapered)throw new Error("Tapered cap entities flattened");
        const mesh=api.GetFlatMesh(id,element),minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
        let volume=0,triangles=0;
        for(let i=0;i<mesh.geometries.size();i++) {
          const part=mesh.geometries.get(i),geometry=api.GetGeometry(id,part.geometryExpressID),m=part.flatTransformation;
          const vertices=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
          const point=index=>{
            const x=vertices[index*6],y=vertices[index*6+1],z=vertices[index*6+2];
            const p=[m[0]*x+m[4]*y+m[8]*z+m[12],-(m[2]*x+m[6]*y+m[10]*z+m[14]),m[1]*x+m[5]*y+m[9]*z+m[13]];
            for(let a=0;a<3;a++){minimum[a]=Math.min(minimum[a],p[a]);maximum[a]=Math.max(maximum[a],p[a]);}
            return p;
          };
          const origin=point(indices[0]);let signed=0;
          for(let j=0;j<indices.length;j+=3){
            const [a,b,c]=[0,1,2].map(k=>point(indices[j+k]).map((v,axis)=>v-origin[axis]));
            signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);
          }
          volume+=Math.abs(signed/6);triangles+=indices.length/3;geometry.delete();
        }
        if(Math.abs(volume-sample.volume)>Math.max(1e-6,sample.volume*1e-5))throw new Error("Native roof volume "+volume+" != "+sample.volume);
        for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)if(Math.abs([minimum,maximum][side][axis]-sample.bounds[side][axis])>1e-5)throw new Error("Native roof bounds changed");
        results.push({language:sample.language,parts:mesh.geometries.size(),tapered:tapered.length,triangles,volume,bounds:[minimum,maximum]});
      } finally {api.CloseModel(id);}
    }
  } finally {api.Dispose();}
  return results;
}
