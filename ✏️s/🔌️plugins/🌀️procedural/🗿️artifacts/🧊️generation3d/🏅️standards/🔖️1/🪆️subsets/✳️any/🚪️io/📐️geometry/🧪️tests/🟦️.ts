/** 🧪️ Neutral capability ownership and explicit inference-context laws. */
import {test,expect} from "bun:test";
import {readdirSync} from "node:fs";
import {fileURLToPath} from "node:url";
const geometry=new URL("../../../🧬️schema/💡️inferences/📐️geometry/",import.meta.url);
const native=new URL("../",import.meta.url);
const fixture=await Bun.file(new URL("./🧫️fixtures/🔣️.json",import.meta.url)).json() as {cases:{id:string,owner:"semantic"|"native"}[]};
const source=await Bun.file(new URL("🗃️registry/🦀️.rs",geometry)).text();
test("geometry compute capabilities have independent neutral ownership witnesses",async()=>{
  const primitive=await Bun.file(new URL("🥽️mesh-primitive/🦀️.rs",geometry)).text();
  const admitted=[await Bun.file(new URL("💾️brep-interchange/🦀️.rs",native)).text(),await Bun.file(new URL("📼️mesh-interchange/🦀️.rs",native)).text(),await Bun.file(new URL("🥽️mesh-source/🦀️.rs",native)).text()].join("\n");
  const folders=new Map(readdirSync(fileURLToPath(geometry)).map(name=>[name.replace(/^[^a-z]+/u,"").replaceAll("-","_"),name]));
  const categories=[...source.matchAll(/semio_s_artifact_procedural_3d_([a-z0-9_]+)::COMPUTES/g)].map(match=>folders.get(match[1]!)!);
  expect(categories.length).toBeGreaterThan(0);
  expect(categories.every(Boolean)).toBe(true);
  const pure=(await Promise.all(categories.map(folder=>Bun.file(new URL(`${folder}/🦀️.rs`,geometry)).text()))).join("\n").replaceAll(" ","");
  for(const item of fixture.cases){expect(pure.includes(`id:"${item.id}"`)).toBe(item.owner==="semantic");expect(item.owner==="semantic"?admitted.includes(`id: "${item.id}"`):admitted.replaceAll(" ","").includes(`id:"${item.id}"`)).toBe(item.owner==="native");}
  expect(source).not.toMatch(/mod (?:brep_interchange|mesh_interchange);/);
  expect(primitive).not.toMatch(/PolygonSourcePreparation|fn construct\(/);
});
test("semantic inference and engine forward their explicit compute capability",async()=>{
  const field=await Bun.file(new URL("🦀️.rs",geometry)).text(),engine=await Bun.file(new URL("🚂️engine/🦀️.rs",geometry)).text();
  expect(field).toContain("snapshot.computes.start(kind, inputs)");
  expect(field).toContain("dyn semio_s_artifact_procedural_3d_geometry_compute::GeometryComputeContext");
  expect(engine).toContain("self.computes.as_ref()");
  expect(engine).not.toContain("io::geometry::context()");
});
