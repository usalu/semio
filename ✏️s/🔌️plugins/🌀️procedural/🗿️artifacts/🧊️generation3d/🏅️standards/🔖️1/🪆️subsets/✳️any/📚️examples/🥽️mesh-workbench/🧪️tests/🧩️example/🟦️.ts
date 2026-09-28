import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
const text = readFileSync(new URL("../../🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio", import.meta.url), "utf8");
test("mesh workbench wires creation, editing, analysis and geometry preview", () => {
  for (const kind of ["box", "inset", "extrude", "analyze", "toBrep"]) expect(text).toContain(`neuron-kind=brep.mesh.${kind}`);
  for (const wire of ["box@meshOut->inset@mesh", "inset@meshOut->extrude@mesh", "extrude@meshOut->analysis@mesh", "extrude@meshOut->preview@mesh"]) expect(text).toContain(wire);
  expect(text).toContain('neuron-kind=brep.mesh.extrude preview=true');
  expect(text).toContain('neuron-kind=brep.mesh.toBrep preview=false');
});
