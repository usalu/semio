import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { parseChangeSliderValue } from "../../🎚️change-slider-value/🦠️mutation/🟦️.ts";
import { parseMoveNodes } from "../../🚚️move-nodes/🦠️mutation/🟦️.ts";
import{binary64}from"../../../🟦️.ts";

/** 🎚️ One generation2d gesture leaf as its twin sees it: the leaf directory, the aggregate variant its wire witness is
 * tagged with, the twin parser, and payloads past a hard bound the schema states. */
type GestureLeaf = { readonly dir: string; readonly variant: string; readonly parse: (value: unknown) => unknown; readonly outOfBounds: readonly Record<string, unknown>[] };

const LEAVES: readonly GestureLeaf[] = [
  { dir: "🎚️change-slider-value", variant: "ChangeSliderValue", parse: parseChangeSliderValue, outOfBounds: [{ id: "", value: 1 }, { id: "slider" }, { id: "slider", value: 1, extra: true }] },
  { dir: "🚚️move-nodes", variant: "MoveNodes", parse: parseMoveNodes, outOfBounds: [{ ids: [], dx: 1, dy: 0 }, { ids: ["a", "a"], dx: 1, dy: 0 }, { ids: ["a"], dx: 1 }] },
];

/** 🧪️ The TypeScript twins of the generation2d gesture leaves against a third-party validator (Ajv, strict): every
 * committed wire witness meets its leaf schema and parses to the same payload, and every payload past a hard bound is
 * refused by both. */
export function generation2dGestureLeafTwinSelfTests(): number {
  const mutations = resolve(import.meta.dir, "../..");
  const fixtures = resolve(mutations, "../../🧫️fixtures/🧬️mutations");
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-ui", metaSchema: { type: "object" } }).addKeyword({ keyword: "x-semio-invariant", metaSchema: { type: "array", items: { type: "object" } } });
  ajv.addKeyword({ keyword: "x-semio-inverse-rows", metaSchema: { type: "object", properties: { perTarget: { type: "object", additionalProperties: { type: "integer", minimum: 0 } } }, required: ["perTarget"], additionalProperties: false } });
  let checks = 0;
  for (const leaf of LEAVES) {
    const validate = ajv.compile(JSON.parse(readFileSync(resolve(mutations, leaf.dir, "🧬️schema/🔣️.json"), "utf8")));
    const witness = JSON.parse(readFileSync(resolve(fixtures, leaf.dir, "🧾️wire-witness/🦠️mutation/🔣️.json"), "utf8"));
    const payload = witness[leaf.variant];
    assert.ok(payload, `${leaf.dir}: the witness is tagged ${leaf.variant}`);
    assert.ok(validate(payload), `${leaf.dir}: ${JSON.stringify(validate.errors)}`);
    const accepted=payload as {id:string;value:number;ids:string[];dx:number;dy:number};
    const expected=leaf.variant==="ChangeSliderValue"?{id:accepted.id,value:binary64(accepted.value)}:{ids:accepted.ids,dx:binary64(accepted.dx),dy:binary64(accepted.dy)};
    assert.deepEqual(leaf.parse(payload), expected, `${leaf.dir}: declared numeric JSON binds to exact canonical words`);
    checks += 3;
    for (const hostile of leaf.outOfBounds) {
      assert.equal(validate(hostile), false, `${leaf.dir}: Ajv refuses ${JSON.stringify(hostile)}`);
      assert.throws(() => leaf.parse(hostile), TypeError, `${leaf.dir}: the twin refuses ${JSON.stringify(hostile)}`);
      checks += 2;
    }
  }
  return checks;
}
