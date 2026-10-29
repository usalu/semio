/** 🚰 Test-only independent native IFC service geometry and historical property oracle. */
import {createRequire} from "node:module";
import {join} from "node:path";

/** 🔎 Validate native class, historical evidence, occupied volume and actual placed bounds. */
export async function verifyServiceSteps(config,samples) {
  const library=createRequire(import.meta.url)(config.modulePath),api=new library.IfcAPI();
  await api.Init(name=>join(config.wasmDirectory,name));
  const result=[];
  try {
    for(const sample of samples) {
      const id=api.OpenModel(new TextEncoder().encode(sample.text));
      if(id<0)throw new Error(sample.tag+": native model rejected");
      try {
        const get=key=>api.GetLine(id,Number(sample.ids[key].slice(1))),element=Number(sample.ids.element.slice(1)),product=get("element");
        if(product.type!==library[sample.entity]||product.Name.value!==sample.name)throw new Error(sample.tag+": native identity");
        const historical=get("historical_pset"),state=get("historical_TargetState");
        if(historical.Name.value!=="Pset_HistoricalEvidence"||state.NominalValue.value!==sample.historical)throw new Error(sample.tag+": historical state lost");
        if(!get("historical_SourceReferences").NominalValue.value||!get("historical_ReconstructionUncertainty").NominalValue.value)throw new Error(sample.tag+": evidence lost");
        const mesh=api.GetFlatMesh(id,element),minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
        let volume=0,triangles=0;
        for(let i=0;i<mesh.geometries.size();i++) {
          const part=mesh.geometries.get(i),geometry=api.GetGeometry(id,part.geometryExpressID),m=part.flatTransformation;
          const v=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize()),indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
          const point=index=>{
            const x=v[6*index],y=v[6*index+1],z=v[6*index+2];
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
        if(Math.abs(volume-sample.volume)>Math.max(1e-7,sample.volume*1e-5))throw new Error(sample.tag+": native volume "+volume+" != "+sample.volume);
        for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)if(Math.abs([minimum,maximum][side][axis]-sample.bounds[side][axis])>1e-5)throw new Error(sample.tag+": actual axis/placement bounds");
        result.push({tag:sample.tag,language:sample.language,parts:mesh.geometries.size(),triangles,volume,historical:state.NominalValue.value});
      } finally {api.CloseModel(id);}
    }
  } finally {api.Dispose();}
  return result;
}
