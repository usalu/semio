/** 🕵️ Validate the graph-free grouped receiver corpus with Ajv and the native JSON codec. */
import { readFileSync } from "node:fs";
import { equal, deepEqual } from "node:assert/strict";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";

const readRows = (file: string) =>
  readFileSync(new URL(file, import.meta.url), "utf8").trim().split("\n").map(row => row.split("\t"));

const routes = readRows("roster.tsv");
const cases = readRows("cases.tsv");
equal(routes.length, cases.length);
const projection = routes.map(([, number, owner, local, tag]) =>
  `    Descriptor { tag: ${JSON.stringify(tag)}, number: ${number}, route: Group::${owner}(${local}) },`,
);
const rust = readFileSync(new URL("src/routes.rs", import.meta.url), "utf8");
deepEqual(rust.split("\n").filter(line => line.startsWith("    Descriptor ")), projection);
deepEqual(routes.map(row => Number(row[1])).sort((a, b) => a - b), [1, 2, 3, 4]);
const external = semioSchemaAjvV1({ strict: true }).compile({
  oneOf: routes.map(([, , , , tag]) => ({
    type: "object", additionalProperties: false, required: [tag],
    properties: {
      [tag]: {
        type: "object", additionalProperties: false, required: ["newValue"],
        properties: { newValue: { type: "integer", minimum: 0, maximum: 65535 } },
      },
    },
  })),
});

for (const [index, [tag, wire, number, json]] of cases.entries()) {
  const [native, wireNumber, , , rosterTag] = routes[index];
  deepEqual([native, wireNumber, rosterTag], [String(index), wire, tag]);
  const value = JSON.parse(json);
  equal(external(value), true);
  equal(JSON.stringify({ [tag]: { newValue: Number(number) } }), json);
}
equal(external({ Deck: { ChangeDeckSpan: { newValue: 17 } } }), false);
equal(external({ ChangeDeckSpan: { newValue: 17 }, ChangeDeckDeckWidth: { newValue: 19 } }), false);
console.log(`${cases.length} interleaved routes agree with the Ajv and JSON oracles`);
