import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import Ajv from "ajv";
import { sumBy } from "lodash-es";

const fixture = JSON.parse(await readFile("✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📝️form/pages.json", "utf8"));
const positive = { type: "integer", minimum: 1 };
const natural = { type: "integer", minimum: 0 };
const result = { type: "object", required: ["index", "total", "first", "last", "nodes"], additionalProperties: false, properties: {
  index: natural, total: positive, first: natural, last: natural, nodes: natural,
} };
const field = { type: "object", required: ["nodes", "count"], additionalProperties: false, properties: {
  nodes: positive, count: positive, section: natural,
} };
const schema = { type: "object", required: ["cases"], additionalProperties: false, properties: { cases: {
  type: "array", minItems: 1, items: {
    type: "object", required: ["name", "budget", "requested", "fields"], additionalProperties: false,
    properties: { name: { type: "string" }, budget: positive, requested: natural, fields: { type: "array", items: field }, expect: result, rejected: natural },
    oneOf: [{ required: ["expect"] }, { required: ["rejected"] }],
  },
} } };
const valid = new Ajv({ strictRequired: false }).compile(schema);
assert.ok(valid(fixture), JSON.stringify(valid.errors));

type FieldCost = { readonly nodes: number; readonly section?: number };
type Page = { readonly first: number; readonly last: number; readonly nodes: number };

function minimumContiguousPages(fields: readonly FieldCost[], budget: number): Page[] | number {
  const used = (rows: readonly FieldCost[]) => sumBy(rows, (row) => row.nodes)
    + rows.filter((row, index) => row.section !== undefined && row.section !== rows[index - 1]?.section).length;
  const oversized = fields.findIndex((row) => used([row]) > budget);
  if (oversized !== -1) return oversized;
  const best: Page[][] = [];
  best[fields.length] = [];
  for (let first = fields.length - 1; first >= 0; first--) {
    const candidates: Page[][] = [];
    for (let last = first + 1; last <= fields.length; last++) {
      const nodes = used(fields.slice(first, last));
      if (nodes > budget) break;
      candidates.push([{ first, last, nodes }, ...best[last]]);
    }
    candidates.sort((a, b) => a.length - b.length || b[0].last - a[0].last);
    best[first] = candidates[0];
  }
  return best[0].length ? best[0] : [{ first: 0, last: 0, nodes: 0 }];
}

for (const row of fixture.cases) {
  const fields = row.fields.flatMap(({ count, ...field }) => Array.from({ length: count }, () => field));
  const pages = minimumContiguousPages(fields, row.budget);
  if (typeof pages === "number") {
    assert.equal(pages, row.rejected, row.name);
    continue;
  }
  const index = Math.min(row.requested, pages.length - 1);
  assert.deepEqual({ index, total: pages.length, ...pages[index] }, row.expect, row.name);
  assert.equal(sumBy(pages, ({ first, last }) => last - first), fields.length, row.name);
}
console.log(fixture.cases.length + " bounded form cases matched the independent partition");
