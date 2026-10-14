import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️.json";
import schema from "./🧬️.json";

test("original widget output has deterministic ports and a single header donation", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  expect([...new Map(fixture.ports.map(port => [port, fixture.text])).keys()].sort()).toEqual(fixture.expectedPorts);
  const value = await Bun.file(new URL("../🦀️.rs", import.meta.url)).text();
  const compute = await Bun.file(new URL("../../⚙️compute/🦀️.rs", import.meta.url)).text();
  expect(value).toContain("pub type Outputs = RetainedOrderedMap<String, GeometryValue>");
  expect(compute).toContain("self.evaluation.take().map(WidgetStep::Done)");
  expect(compute).not.toContain("self.evaluation.clone()");
  expect(compute).toContain("cx.original_source_parent()?.source_identity()?");
  expect(JSON.parse(JSON.stringify(fixture.cancellation))).toEqual({text:"Retained Through Cancellation Ä",selectionIds:[3,7,11],wirePoint:[1,2,3]});
  expect(fixture.handoff).toEqual({items: 1, copyBytes: 0, capacityBytes: 0, releaseBytes: 0});
});
