import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import fixtures from "../../🧫️fixtures/🔣️.json";
import { parseComponentTarget, componentGroup, selectedMeshVertices } from "../../🟦️.ts";
import topology from "../../🧫️fixtures/🥽️topology/🔣️.json";
import editSchema from "../../../🎮️commands/🥽️edit-mesh-selection/🧬️schema/🔣️.json";
import knifeSchema from "../../../🎮️commands/🔪️knife-mesh-selection/🧬️schema/🔣️.json";
import knifeFixtures from "../../../🎮️commands/🔪️knife-mesh-selection/🧫️fixtures/🔣️.json";
import { knifeSelectionParameters } from "../../../🎮️commands/🔪️knife-mesh-selection/🟦️.ts";
import { Vector3 } from "three";

describe("evaluated mesh component selection", () => {
  for (const fixture of topology.valid) test(`resolves topology ${fixture.ids.join(",")}`, () => {
    const vertices = selectedMeshVertices(fixture.ids, JSON.stringify(topology.mesh));
    expect(vertices).toEqual(fixture.vertices);
    const points = vertices.map(id => new Vector3(...topology.mesh.vertices[id]));
    const center = points.reduce((sum, point) => sum.add(point), new Vector3()).divideScalar(points.length);
    expect(center.toArray()).toEqual(fixture.pivot);
  });
  for (const ids of topology.invalid) test(`rejects stale topology ${ids.join(",")}`, () => {
    expect(() => selectedMeshVertices(ids, JSON.stringify(topology.mesh))).toThrow();
  });
});

describe("selected-face knife contract", () => {
  const validate = new Ajv().compile(knifeSchema);
  for (const fixture of knifeFixtures.valid) test(`knife face target: ${fixture.ids.join(",")}`, () => {
    expect(validate(knifeFixtures.payload)).toBe(true);
    const result = knifeSelectionParameters(fixture.ids, knifeFixtures.payload);
    expect(result).toEqual({ face: fixture.face, ...knifeFixtures.payload });
    expect(new Vector3(...result.start).distanceTo(new Vector3(...result.end))).toBe(2);
  });
  for (const ids of knifeFixtures.invalidSelections) test(`rejects knife selection: ${ids.join(",")}`, () => {
    expect(() => knifeSelectionParameters(ids, knifeFixtures.payload)).toThrow();
  });
  for (const payload of knifeFixtures.invalidPayloads) test(`rejects knife points: ${JSON.stringify(payload)}`, () => {
    expect(() => knifeSelectionParameters(knifeFixtures.valid[0].ids, payload)).toThrow();
  });
  test("rejects non-finite points", () => {
    for (const value of [Infinity, NaN]) expect(() => knifeSelectionParameters(knifeFixtures.valid[0].ids, { start: [value, 0, 0], end: [0, 1, 0] })).toThrow();
  });
});

describe("mesh component selection contract", () => {
  const validate = new Ajv().compile(schema);
  const validateEdit = new Ajv().compile(editSchema);
  for (const mode of fixtures.quickActions) for (const operation of mode.operations) test(`quick action schema: ${mode.granularity}/${operation}`, () => {
    const payload = { operation, amount: 0.1, cuts: 1, dx: 0, dy: 0, dz: 0 };
    expect(validateEdit(payload)).toBe(true);
    for (const cuts of [0, 257, 1.5]) expect(validateEdit({ ...payload, cuts })).toBe(false);
  });
  for (const target of fixtures.valid) test(target.id, () => {
    expect(validate(target.id)).toBe(true);
    expect(parseComponentTarget(target.id)).toEqual(target);
  });
  for (const id of fixtures.invalid) test(`rejects ${JSON.stringify(id)}`, () => {
    expect(validate(id)).toBe(false);
    expect(parseComponentTarget(id)).toBeUndefined();
  });
  for (const group of fixtures.groups) test(`groups ${JSON.stringify(group.ids)}`, () => {
    if ("error" in group) expect(() => componentGroup(group.ids)).toThrow();
    else expect(componentGroup(group.ids)).toEqual({ instance: group.instance, granularity: group.granularity, components: group.components });
  });
});
