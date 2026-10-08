/** 🧪️ Neutral capability ownership and explicit inference-context laws. */
import {test,expect} from "bun:test";
const geometry=new URL("../../../🧬️schema/💡️inferences/📐️geometry/",import.meta.url);
const native=new URL("../",import.meta.url);
const fixture=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json() as {cases:{id:string,owner:"semantic"|"native"}[]};
const source=await Bun.file(new URL("🗃️registry/🦀️.rs",geometry)).text();
test("geometry compute capabilities have independent neutral ownership witnesses",async()=>{
  const primitive=await Bun.file(new URL("🥽️mesh-primitive/🦀️.rs",geometry)).text();
  const admitted=[await Bun.file(new URL("💾️brep-interchange/🦀️.rs",native)).text(),await Bun.file(new URL("📼️mesh-interchange/🦀️.rs",native)).text(),await Bun.file(new URL("🥽️mesh-source/🦀️.rs",native)).text()].join("\n");
  const pure=(await Promise.all([...source.matchAll(/#\[path = "\.\.\/(.+?)\/🦀️\.rs"\]/g)].map(match=>Bun.file(new URL(`${match[1]}/🦀️.rs`,geometry)).text()))).join("\n").replaceAll(" ","");
  for(const item of fixture.cases){expect(pure.includes(`id:"${item.id}"`)).toBe(item.owner==="semantic");expect(item.owner==="semantic"?admitted.includes(`id: "${item.id}"`):admitted.replaceAll(" ","").includes(`id:"${item.id}"`)).toBe(item.owner==="native");}
  expect(source).not.toMatch(/mod (?:brep_interchange|mesh_interchange);/);
  expect(primitive).not.toMatch(/PolygonSourcePreparation|fn construct\(/);
  console.log("[DEBUG] Procedural neutral capability ownership matches plain neutral examples and actual relocated native bodies");
});
test("semantic inference and engine forward their explicit compute capability",async()=>{
  const field=await Bun.file(new URL("🦀️.rs",geometry)).text(),engine=await Bun.file(new URL("🚂️engine/🦀️.rs",geometry)).text();
  expect(field).toContain("snapshot.computes.start(kind, inputs)");
  expect(field).toContain("dyn compute::GeometryComputeContext");
  expect(engine).toContain("Arc::clone(&self.computes)");
  expect(engine).not.toContain("io::geometry::context()");
  console.log("[DEBUG] Procedural production field and engine explicitly forward injected compute capabilities");
});
