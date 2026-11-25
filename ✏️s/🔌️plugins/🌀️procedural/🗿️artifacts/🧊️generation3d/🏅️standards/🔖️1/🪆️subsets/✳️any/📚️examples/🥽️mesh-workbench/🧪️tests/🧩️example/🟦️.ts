import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
const text = readFileSync(new URL("../../🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio", import.meta.url), "utf8");
test("mesh workbench wires creation, editing, analysis and geometry preview", () => {
  for (const kind of ["box", "inset", "extrude", "analyze", "toBrep"]) expect(text).toContain(`neuron-kind=brep.mesh.${kind}`);
  for (const wire of ["box@meshOut->inset@mesh", "inset@meshOut->extrude@mesh", "extrude@meshOut->analysis@mesh", "extrude@meshOut->preview@mesh"]) expect(text).toContain(wire);
  expect(text).toContain('neuron-kind=brep.mesh.extrude preview=true');
  expect(text).toContain('neuron-kind=brep.mesh.toBrep preview=false');
});
const root = new URL("../../../../../../../../../../../../../", import.meta.url);
const workbench = JSON.parse(readFileSync(new URL("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🧫️fixtures/🥽️retained-workbench/🔣️.json", root), "utf8"));
const slider = (id: string): number => Number(new RegExp(`input-slider id="${id}"[^\\n]*?\\svalue=(-?[\\d.]+)`).exec(text)![1]);
test("mesh workbench recomputes the committed area and volume of the inset and extruded unit box from its sliders", () => {
  const side = 1, amount = slider("amount"), distance = slider("distance"), inner = side - 2 * amount;
  expect(amount).toBe(workbench.amount);
  expect(distance).toBe(workbench.distance);
  expect(Math.abs(workbench.volume - (side ** 3 + inner ** 2 * distance))).toBeLessThan(1e-9);
  expect(Math.abs(workbench.area - (6 * side ** 2 + 4 * inner * distance))).toBeLessThan(1e-9);
});
