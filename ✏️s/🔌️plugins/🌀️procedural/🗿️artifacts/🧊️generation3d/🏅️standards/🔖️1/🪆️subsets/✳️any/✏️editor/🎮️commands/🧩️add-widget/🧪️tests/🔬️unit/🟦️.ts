/** 🧩️ Independent schema and placement oracle for the shared widget-creation cases. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";

/** 🧪️ Answers the same language-neutral fixture as the Rust creation command. */
export function generation3dWidgetCreationSelfTests(): number {
  const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const validate = new Ajv({ strict: false, allErrors: true }).compile(schema);
  assert.deepEqual(schema.properties.format.enum,fixture.exportFormats);
  let checks = 0;
  for (const [cases, expected] of [[fixture.valid, true], [fixture.invalid, false]] as const) {
    for (const value of cases) { assert.equal(validate(value), expected, JSON.stringify(value)); checks++; }
  }
  for (const row of fixture.defaults) {
    assert.equal(validate(row.payload),true);
    const descriptor = {...row.payload,format:row.payload.format ?? schema.properties.format.default};
    assert.deepEqual(descriptor,row.descriptor);
    checks += 2;
  }
  for (const row of fixture.placements) {
    let [x, y] = [row.x ?? 120, row.y ?? 120];
    if (row.x === undefined || row.y === undefined) {
      for (;;) {
        const collisions = row.occupied.filter((rect: number[]) => Math.abs(x - rect[0]!) < (row.size[0] + rect[2]!) / 2 + fixture.gap && Math.abs(y - rect[1]!) < (row.size[1] + rect[3]!) / 2 + fixture.gap);
        if (collisions.length === 0) break;
        if (row.y === undefined) y = Math.max(...collisions.map((rect: number[]) => rect[1]! + (row.size[1] + rect[3]!) / 2 + fixture.gap));
        else x = Math.max(...collisions.map((rect: number[]) => rect[0]! + (row.size[0] + rect[2]!) / 2 + fixture.gap));
      }
    }
    assert.deepEqual([x, y], row.expected, row.name); checks++;
  }
  const names = JSON.parse(readFileSync(new URL("../../../../📌️panels/🛍️catalogue/🧫️fixtures/🗣️.json", import.meta.url), "utf8"));
  const terminology = JSON.parse(readFileSync(new URL("../../../../../🧫️fixtures/🗣️terminology.json", import.meta.url), "utf8"));
  for (const row of names) {
    const field = `catalogue_${row.id.replaceAll(".", "_").replace(/([a-z])([A-Z])/gu, "$1_$2").toLowerCase()}`;
    assert.equal(terminology.labels[field].nativeEn, row.en);
    assert.equal(terminology.labels[field].nativeDe, row.de);
    checks += 2;
  }
  const inputs = JSON.parse(readFileSync(new URL("../../../../📌️panels/🛍️catalogue/🧫️fixtures/🎛️inputs.json", import.meta.url), "utf8"));
  for (const row of inputs) {
    const field = `input_name_${row.id.replace(/([a-z])([A-Z])/gu, "$1_$2").toLowerCase()}`;
    assert.equal(terminology.labels[field].nativeEn, row.en);
    assert.equal(terminology.labels[field].nativeDe, row.de);
    checks += 2;
  }
  return checks;
}
