import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Box3, Vector3 } from "three";
import preview from "../🧫️fixtures/🧩️example/🔣️.json";
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

test("mesh workbench publishes the two solid buffers and bounds of the authored world-unit extrusion", () => {
  const { dimensions, center, selectedFaceNormal, units } = workbench.box;
  expect(units).toBe("world");
  const half = dimensions.map((length: number) => length / 2);
  const box = new Box3(
    new Vector3(...center.map((value: number, axis: number) => value - half[axis]) as [number, number, number]),
    new Vector3(...center.map((value: number, axis: number) => value + half[axis]) as [number, number, number]),
  );
  const faceZ = box.min.z;
  const insetHalf = half[0] - slider("amount");
  const extrusion = new Vector3(...selectedFaceNormal as [number, number, number]).multiplyScalar(slider("distance"));
  for (const x of [-insetHalf, insetHalf]) for (const y of [-insetHalf, insetHalf]) box.expandByPoint(new Vector3(x, y, faceZ).add(extrusion));
  expect(preview.preview).toEqual({ node: "extrude", channel: "meshOut", kind: "solid" });
  expect(preview.delivery.meshRoles).toEqual({ solid: 2 });
  expect(preview.delivery.meshes).toBe(2);
  expect(preview.expect.volume).toBe(workbench.volume);
  expect(preview.delivery.minTriangles).toBe(2 * (6 + 4 + 4));
  expect(preview.delivery.minTriangles).toBe(workbench.mesh.triangles);
  expect(preview.delivery.boundingBoxMin).toEqual(workbench.mesh.boundingBoxMin);
  expect(preview.delivery.boundingBoxMax).toEqual(workbench.mesh.boundingBoxMax);
  expect(box.min.toArray()).toEqual(preview.delivery.boundingBoxMin);
  expect(box.max.toArray()).toEqual(preview.delivery.boundingBoxMax);
});
