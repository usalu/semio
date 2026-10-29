/** 🧪 Independent test-only IFC schema reader and geometry oracle; no library objects leave this interface. */
import {createRequire} from "node:module";
import {dirname,join} from "node:path";

function demand(condition,message) {
  if (!condition) throw new Error(message);
}

function volume(api,model,mesh) {
  let total=0;
  const minimum=[Infinity,Infinity,Infinity],maximum=[-Infinity,-Infinity,-Infinity];
  for(let i=0;i<mesh.geometries.size();i++) {
    const part=mesh.geometries.get(i),geometry=api.GetGeometry(model,part.geometryExpressID);
    const v=api.GetVertexArray(geometry.GetVertexData(),geometry.GetVertexDataSize());
    const indices=api.GetIndexArray(geometry.GetIndexData(),geometry.GetIndexDataSize());
    const m=part.flatTransformation;
    function point(index) {
      const x=v[index*6],y=v[index*6+1],z=v[index*6+2];
      const native=[m[0]*x+m[4]*y+m[8]*z+m[12],m[1]*x+m[5]*y+m[9]*z+m[13],m[2]*x+m[6]*y+m[10]*z+m[14]];
      const p=[native[0],-native[2],native[1]];
      for(let axis=0;axis<3;axis++){minimum[axis]=Math.min(minimum[axis],p[axis]);maximum[axis]=Math.max(maximum[axis],p[axis]);}
      return p;
    }
    const origin=point(indices[0]);
    let signed=0;
    for(let j=0;j<indices.length;j+=3) {
      const [a,b,c]=[0,1,2].map(k=>point(indices[j+k]).map((value,axis)=>value-origin[axis]));
      signed+=a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]);
    }
    total+=Math.abs(signed/6);
    geometry.delete();
  }
  return {volume:total,bounds:[minimum,maximum]};
}

/** 📦 Read native product/type/constituent entities and reconstruct geometry through an independent IFC engine. */
export async function verifyNativeFillingSteps(config,samples,cases) {
  const require=createRequire(import.meta.url),library=require(config.modulePath);
  const api=new library.IfcAPI();
  await api.Init(name=>join(config.wasmDirectory||dirname(config.modulePath),name));
  const results=[];
  try {
    for(const sample of samples) {
      const model=api.OpenModel(new TextEncoder().encode(sample.text));
      demand(model>=0,sample.tag+": IFC model rejected");
      try {
        const fixture=cases.cases.find(c=>c.tag===sample.tag);
        const expectedParts=Object.keys(sample.ids).filter(key=>/^solid\d+$/.test(key)).length;
        demand(expectedParts>=fixture.minimum_parts,sample.tag+": native construction part lower bound");
        const get=key=>api.GetLine(model,Number(sample.ids[key].slice(1)));
        const product=get("element"),type=get("type");
        demand(product.type===library[fixture.entity],sample.tag+": native product type");
        demand(type.type===library[fixture.type],sample.tag+": native type entity");
        demand(product.Name.value===sample.name,sample.tag+": localized name round trip");
        if(fixture.entity!=="IFCCURTAINWALL") {
          demand(Math.abs(product.OverallWidth.value-sample.quantities.rough_width)<1e-8,sample.tag+": native width");
          demand(Math.abs(product.OverallHeight.value-sample.quantities.rough_height)<1e-8,sample.tag+": native height");
          demand(product.PredefinedType===null,sample.tag+": occurrence/type override");
        }
        const aspectKeys=Object.keys(sample.ids).filter(key=>/^aspect\d+$/.test(key));
        const constituents=get("constituentset").MaterialConstituents.map(ref=>api.GetLine(model,ref.value));
        const covered=[];
        for(const key of aspectKeys) {
          const aspect=get(key);
          const constituent=constituents.find(c=>c.Name.value===aspect.Name.value);
          demand(Boolean(constituent),sample.tag+": missing material aspect");
          demand(api.GetLine(model,constituent.Material.value).type===library.IFCMATERIAL,sample.tag+": constituent material");
          demand(aspect.PartOfProductDefinitionShape.value===Number(sample.ids.shape.slice(1)),sample.tag+": aspect owner");
          for(const ref of aspect.ShapeRepresentations) covered.push(...api.GetLine(model,ref.value).Items.map(item=>item.value));
        }
        demand(covered.length===expectedParts&&new Set(covered).size===expectedParts,sample.tag+": solid material coverage");
        const mesh=api.GetFlatMesh(model,Number(sample.ids.element.slice(1)));
        demand(mesh.geometries.size()===expectedParts,sample.tag+": native mesh part count");
        const actual=volume(api,model,mesh),actualVolume=actual.volume,expectedVolume=sample.quantities.volume;
        for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)demand(Math.abs(actual.bounds[side][axis]-sample.bounds[side][axis])<1e-5,sample.tag+": native placement/bounds");
        demand(Math.abs(actualVolume-expectedVolume)<Math.max(1e-6,expectedVolume*1e-5),sample.tag+": IFC mesh volume "+actualVolume+" != "+expectedVolume);
        results.push({tag:sample.tag,language:sample.language,parts:expectedParts,nativeClass:product.constructor.name,volume:actualVolume});
      } finally {api.CloseModel(model);}
    }
  } finally {api.Dispose();}
  return results;
}

/** 🕳️ Independently read native hollow column profiles and reconstruct their complete occupied solids. */
export async function verifyNativeBoreSteps(config,samples) {
  const require=createRequire(import.meta.url),library=require(config.modulePath),api=new library.IfcAPI();
  await api.Init(name=>join(config.wasmDirectory||dirname(config.modulePath),name));
  const results=[];
  try {
    for(const sample of samples) {
      const model=api.OpenModel(new TextEncoder().encode(sample.text),{CIRCLE_SEGMENTS:2048,ALLOW_INCOMPATIBLE_SCHEMA_ALIASES:false});
      demand(model>=0,sample.tag+": IFC model rejected");
      try {
        const get=key=>api.GetLine(model,Number(sample.ids[key].slice(1)));
        demand(get("element").type===library.IFCCOLUMN,sample.tag+": native column class");
        demand(get("type").type===library.IFCCOLUMNTYPE,sample.tag+": native column type");
        if(/^[24](-FIRST)?$/.test(sample.tag)) {
          const evidence=get("historical_BoreConvention").NominalValue;
          demand(evidence.constructor.name==="IfcText"&&evidence.value.includes("teaching inferences"),sample.tag+": native historical inference text");
        }
        let hollows=0;
        for(let i=0;i<sample.parts;i++) {
          const profile=get("profile"+i),expected=sample.profiles[i];
          if(profile.type===library.IFCCIRCLEHOLLOWPROFILEDEF) {
            hollows++;
            demand(Math.abs(profile.Radius.value-expected.outer_radius)<1e-10,sample.tag+": outer radius");
            demand(Math.abs(profile.WallThickness.value-(expected.outer_radius-expected.radii[0]))<1e-10,sample.tag+": actual radial material width");
          }
          if(profile.type===library.IFCARBITRARYPROFILEDEFWITHVOIDS) {
            hollows++;
            demand(profile.InnerCurves.length===1,sample.tag+": explicit inner curve");
            const inner=api.GetLine(model,profile.InnerCurves[0].value);
            demand(inner.type===library.IFCCIRCLE&&Math.abs(inner.Radius.value-expected.radii[0])<1e-10,sample.tag+": exact native inner circle");
          }
        }
        demand(hollows===sample.hollows,sample.tag+": native bore profile count");
        const mesh=api.GetFlatMesh(model,Number(sample.ids.element.slice(1)));
        const actual=volume(api,model,mesh);
        demand(Math.abs(actual.volume-sample.volume)<Math.max(1e-6,sample.volume*5e-6),sample.tag+": independent IFC volume "+actual.volume+" != "+sample.volume);
        for(let side=0;side<2;side++)for(let axis=0;axis<3;axis++)demand(Math.abs(actual.bounds[side][axis]-sample.bounds[side][axis])<1e-5,sample.tag+": placement changed");
        results.push({tag:sample.tag,language:sample.language,hollows,parts:mesh.geometries.size(),volume:actual.volume});
      } finally {api.CloseModel(model);}
    }
  } finally {api.Dispose();}
  return results;
}
