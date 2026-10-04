import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";
import { parseChangeSliderValue } from "../../../🎚️change-slider-value/🦠️mutation/🟦️.ts";
import { parseChangeWidgetInput } from "../../../🎛️change-widget-input/🦠️mutation/🟦️.ts";
import { parseDragTransforms } from "../../../✋️drag-transforms/🦠️mutation/🟦️.ts";
import { parseMoveNodes } from "../../../🚚️move-nodes/🦠️mutation/🟦️.ts";
import { parseRotateTransforms } from "../../../🔃️rotate-transforms/🦠️mutation/🟦️.ts";
import { parseScaleTransforms } from "../../../📏️scale-transforms/🦠️mutation/🟦️.ts";
import { binary64Value } from "../../../../🟦️.ts";

/** 🚪️ Every gesture leaf's TypeScript twin parser, by wire keyword. */
const PARSERS: Readonly<Record<string, (value: unknown) => unknown>> = {
  "change-slider-value": parseChangeSliderValue,
  "drag-transforms": parseDragTransforms,
  "rotate-transforms": parseRotateTransforms,
  "scale-transforms": parseScaleTransforms,
  "move-nodes": parseMoveNodes,
  "change-widget-input": parseChangeWidgetInput,
};

/** 🚫️ One payload per leaf that breaks a hard bound its JSON Schema states — both the third-party validator and the
 * twin parser must refuse it. */
const OUT_OF_BOUNDS: Readonly<Record<string, Record<string, unknown>>> = {
  "change-slider-value": { mutation: "changeSliderValue", id: "", value: 1 },
  "drag-transforms": { mutation: "dragTransforms", targets: [], dx: 1, dy: 0, dz: 0 },
  "rotate-transforms": { mutation: "rotateTransforms", targets: ["rotate", "rotate"], ax: 0, ay: 0, az: 1, angle: 1 },
  "scale-transforms": { mutation: "scaleTransforms", targets: ["scale"], sx: 0, sy: 1, sz: 1 },
  "move-nodes": { mutation: "moveNodes", ids: ["slider"], dx: 1 },
  "change-widget-input": { mutation: "changeWidgetInput", id: "translate", channel: "", type: "number", value: 1 },
};

/** 🧬️ Compares the authored semantic records with independent JSON-schema and protocol-tag oracles, and the twin parsers
 * against the same third-party validator: every corpus record parses to its payload, a forged tag and every
 * out-of-bounds payload are refused by both, every `change-widget-input` literal type is covered, and the payload-intrinsic
 * invariant the schema cannot state (`axis-nonzero`) is declared as `x-semio-invariant` and refused by the parser alone. */
export function assertGeneration3dSemanticWire(): number {
  const root = resolve(import.meta.dir, "../..");
  const read = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
  const corpus: { cases: { keyword: string; tag: number; source: string; mutation: Record<string, unknown> }[] } = read("🧫️fixtures/🧬️semantic-wire/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-ui", metaSchema: { type: "object" } }).addKeyword({ keyword: "x-semio-invariant", metaSchema: { type: "array", items: { type: "object" } } }).addKeyword({ keyword: "x-semio-inverse-rows", metaSchema: { type: "object" } });
  assert(ajv.compile(read("🧬️schema/🧬️semantic-wire/🔣️.json"))(corpus));
  const tags = new Map([...readFileSync(resolve(root, "📡️.protocol.semio"), "utf8").matchAll(/^record (\S+) tag=(\d+)$/gm)].map(match => [match[1], Number(match[2])]));
  let checks = 0;
  for (const row of corpus.cases) {
    assert.equal(tags.get(row.keyword), row.tag, row.keyword);
    const schema = JSON.parse(readFileSync(resolve(root, "..", row.source, "🧬️schema/🔣️.json"), "utf8"));
    const validate = ajv.getSchema(schema.$id) ?? ajv.compile(schema);
    assert(validate(row.mutation), JSON.stringify(validate.errors));
    const hostile = { ...row.mutation, mutation: "forged" };
    assert.equal(validate(hostile), false, row.keyword);
    const parse = PARSERS[row.keyword];
    assert(parse, `${row.keyword} has a twin parser`);
    const { mutation: _tag, ...payload } = row.mutation;
    const parsed = parse(row.mutation);
    if (row.keyword === "move-nodes") {
      const moved = parsed as ReturnType<typeof parseMoveNodes>;
      assert.deepEqual({...moved,dx:binary64Value(moved.dx),dy:binary64Value(moved.dy)},payload,`${row.keyword} canonical binary64 values match the independent JSON witness`);
    } else assert.deepEqual(parsed, payload, `${row.keyword} parses to its payload`);
    assert.throws(() => parse(hostile), TypeError, `${row.keyword} refuses a forged tag`);
    const outOfBounds = OUT_OF_BOUNDS[row.keyword];
    assert.equal(validate(outOfBounds), false, `${row.keyword}: Ajv refuses ${JSON.stringify(outOfBounds)}`);
    assert.throws(() => parse(outOfBounds), TypeError, `${row.keyword}: the twin refuses ${JSON.stringify(outOfBounds)}`);
    checks += 5;
  }
  const rotateSchema = JSON.parse(readFileSync(resolve(root, "../🔃️rotate-transforms/🧬️schema/🔣️.json"), "utf8"));
  assert.deepEqual(rotateSchema["x-semio-invariant"].map((invariant: { id: string }) => invariant.id), ["axis-nonzero"]);
  const zeroAxis = { mutation: "rotateTransforms", targets: ["rotate"], ax: 0, ay: 0, az: 0, angle: 1 };
  assert.equal((ajv.getSchema(rotateSchema.$id) ?? ajv.compile(rotateSchema))(zeroAxis), true, "draft-07 cannot state axis-nonzero");
  assert.throws(() => parseRotateTransforms(zeroAxis), /axis-nonzero/, "the twin refuses the declared invariant");
  assert.equal(new Set(corpus.cases.map((row) => row.keyword)).size, 6);
  assert.equal(new Set(corpus.cases.map((row) => row.tag)).size, 6);
  assert.deepEqual([...new Set(corpus.cases.filter((row) => row.keyword === "change-widget-input").map((row) => row.mutation.type))], ["number", "text", "boolean", "point", "vector", "numberList", "textList", "booleanList", "pointList", "vectorList"], "every widget-input type has a vector");
  return checks + 8;
}
