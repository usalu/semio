import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {BufferGeometry,Float32BufferAttribute,Mesh,MeshStandardMaterial,Texture,Triangle,Vector3} from "three";
import {mergeGeometries} from "three/examples/jsm/utils/BufferGeometryUtils.js";
import fixture from "../🧫️fixtures/🔗️combine.json";
import assets from "../🧫️fixtures/🔗️combine-assets.json";

type Source=typeof fixture.mesh|typeof fixture.other;
function prepare(source:Source) {
  const out=new BufferGeometry();
  const row=(kind:string,field:"normal"|"uv",index:number)=>source.surface.find(sample=>sample.kind===kind&&sample.index===index)?.[field];
  const attribute=source.attributes.fixtureId;
  out.setAttribute("position",new Float32BufferAttribute(source.vertices.flat(),3));
  out.setAttribute("normal",new Float32BufferAttribute(source.vertices.flatMap((_,id)=>row("vertexNormal","normal",id)!),3));
  out.setAttribute("uv",new Float32BufferAttribute(source.vertices.flatMap((_,id)=>row("cornerUv","uv",id)!),2));
  out.setAttribute("fixtureId",new Float32BufferAttribute(source.vertices.map((_,id)=>attribute.values["indices" in attribute?attribute.indices[id]!:id]!),1));
  out.setIndex(source.faces.flat());
  return out;
}

test("Three combines two components with separate indices and authored samples",()=>{
  const joined=mergeGeometries([prepare(fixture.mesh),prepare(fixture.other)])!;
  expect(Array.from(joined.index!.array)).toEqual(fixture.expected.faces.flat());
  expect(joined.getAttribute("position").count).toBe(fixture.expected.vertices);
  expect(Array.from(joined.getAttribute("fixtureId").array)).toEqual(fixture.expected.fixtureIds);
  let area=0;
  for(let index=0;index<joined.index!.count;index+=3) {
    const points=[0,1,2].map(delta=>new Vector3().fromBufferAttribute(joined.getAttribute("position"),joined.index!.getX(index+delta)));
    area+=new Triangle(points[0],points[1],points[2]).getArea();
  }
  expect(area).toBe(fixture.expected.area);
  const native=process.env.SEMIO_TEST_COMBINED_POLYGON;
  if(!native)return;
  const output=JSON.parse(readFileSync(native,"utf8"));
  expect(output.faces).toEqual(fixture.expected.faces);
  expect(output.vertices.flat()).toEqual(Array.from(joined.getAttribute("position").array));
  expect(output.surface.filter((row:{kind:string})=>row.kind==="vertexNormal").flatMap((row:{normal:number[]})=>row.normal)).toEqual(Array.from(joined.getAttribute("normal").array));
  expect(output.surface.filter((row:{kind:string})=>row.kind==="cornerUv").flatMap((row:{uv:number[]})=>row.uv)).toEqual(Array.from(joined.getAttribute("uv").array));
  const {indices,values}=output.attributes.fixtureId;
  expect(indices.map((id:number)=>values[id])).toEqual(fixture.expected.fixtureIds);
  expect(output.surface.find((row:{kind:string})=>row.kind==="seamEdge").index).toBe(fixture.expected.seamEdge);
  expect(output.surface.find((row:{kind:string})=>row.kind==="smoothFace").index).toBe(fixture.expected.smoothFace);
});

test("Three combines material groups with shared and disjoint texture bindings",()=>{
  const materials={...assets.mesh.materials,...assets.other.materials};
  const textures={...assets.mesh.textures,...assets.other.textures};
  const images=new Map(Object.entries(textures).map(([name,source])=>{
    const texture=new Texture();
    texture.name=name;
    texture.userData={mime:source.mime,bytes:Uint8Array.from(source.bytes)};
    return [name,texture] as const;
  }));
  const panels=Object.entries(materials).map(([name,source])=>{
    const material=new MeshStandardMaterial({map:images.get(source.baseColorTexture)});
    material.name=name;
    material.color.setRGB(source.baseColor[0]!,source.baseColor[1]!,source.baseColor[2]!);
    return material;
  });
  const combined=mergeGeometries([prepare(assets.mesh),prepare(assets.other)],true)!;
  const panelsByFace=[assets.mesh,assets.other].map(source=>panels.find(panel=>panel.name===source.attributes.surfaceMaterial.values[0])!);
  const mesh=new Mesh(combined,panelsByFace);
  expect(mesh.geometry.groups.map(group=>mesh.material[group.materialIndex!]!.name)).toEqual(assets.expected.facesMaterials);
  expect(Object.keys(materials).sort()).toEqual(assets.expected.materials);
  expect([...images.keys()].sort()).toEqual(assets.expected.textures);
  const native=process.env.SEMIO_TEST_COMBINED_ASSETS;
  if(!native)return;
  const output=JSON.parse(readFileSync(native,"utf8"));
  expect(output.materials).toEqual(materials);
  expect(output.textures).toEqual(textures);
  expect(output.attributes.surfaceMaterial.values).toEqual(assets.expected.facesMaterials);
  expect(output.faces.flat()).toEqual(Array.from(mesh.geometry.index!.array));
});
