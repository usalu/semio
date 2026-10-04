# ValueType Portable Ownership Original Audit Capture

Read-only independently retained exact inputs before Root ownership split. These original bytes are audit authority; they are not runtime results.

## 🧰️framework/🔨️modules/🌱️value/🏷️type/🧪️tests/🟦️.ts

Bytes 5989; SHA-256 `1a6640cde955db9bb831ab30189abd4bfebf6d065fe2c162b82a73d7ac258f81`.

````text
import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";

const owner = resolve(import.meta.dir, ".."), root = resolve(owner, "../../../..");
const read = (path: string): string => readFileSync(resolve(owner, path), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/🔣️.json")), schema = JSON.parse(read("🧬️schema/🔣️.json"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const typeValid = ajv.getSchema(schema.$id + "#/$defs/valueType")!;

test("canonical type corpus closes every wire shape and classification pair", () => {
  expect(ajv.getSchema(schema.$id)!(fixture)).toBe(true);
  expect(new Set(fixture.cases.map((row: { type: number; kind: number }) => row.type + ":" + row.kind)).size).toBe(63);
  for (const row of fixture.wire) expect(typeValid(row)).toBe(true);
  for (const row of fixture.refused) expect(typeValid(row)).toBe(false);
});

test("actual neutral types match an independent schema validator", async () => {
  expect(existsSync(resolve(owner, "🟦️.ts"))).toBe(true);
  const subject = await import("../🟦️.ts");
  const control = { checkpoint() {} };
  for (const row of fixture.wire) expect(subject.readValueType(row, control)).toEqual(row);
  for (const row of fixture.refused) expect(() => subject.readValueType(row, control)).toThrow();
  for (const row of fixture.types) expect(subject.valueTypeId(row.type)).toBe(row.id);
  for (const row of fixture.cases) {
    const type = fixture.types[row.type].type, kind = fixture.kinds[row.kind];
    const accepted = type.kind === "any" ? ["boolean", "integer", "decimal", "text", "dictionary"] : type.kind === "decimal" ? ["boolean", "integer", "decimal"] : [type.kind];
    const reference = ajv.compile(type.kind === "list" || type.kind === "schema"
      ? { type: "object", properties: { kind: { const: "dictionary" }, schema: { const: type.kind === "list" ? "list" : type.of } }, required: ["kind", "schema"], additionalProperties: false }
      : { type: "object", properties: { kind: { enum: accepted }, schema: { type: "string" } }, required: ["kind"], additionalProperties: false });
    expect(reference(kind), row.name).toBe(row.accepted);
    expect(subject.valueTypeMatches(type, kind), row.name).toBe(row.accepted);
  }
});

test("typed reader refuses inherited fields, accessors and cycles with caller cancellation", async () => {
  const subject = await import("../🟦️.ts"), control = { checkpoint() {} };
  const inherited = Object.create({ kind: "boolean" }, { of: { value: null, enumerable: true } });
  const concealed = Object.defineProperty({ of: null }, "kind", { value: "boolean" });
  for (const row of [inherited, concealed]) {
    expect(typeValid(JSON.parse(JSON.stringify(row)))).toBe(false);
    expect(() => subject.readValueType(row, control)).toThrow();
  }
  let calls = 0;
  const accessor = Object.defineProperty({}, "kind", { get() { calls++; return "boolean"; }, enumerable: true });
  expect(() => subject.readValueType(accessor, control)).toThrow();
  expect(calls).toBe(0);
  const cycle: { kind: "list"; of?: unknown } = { kind: "list" };
  cycle.of = cycle;
  expect(() => subject.readValueType(cycle, control)).toThrow();
  let nested: unknown = { kind: "text" };
  for (let index = 0; index < 300; index++) nested = { kind: "list", of: nested };
  const progress: string[] = [];
  subject.readValueType(nested, { checkpoint(completed, phase) { progress.push(phase + ":" + completed); } });
  expect(progress.length).toBe(601);
  expect(progress[300]).toBe("read:300");
  expect(progress.at(-1)).toBe("construct:299");
  let checked = 0;
  expect(() => subject.readValueType(nested, { checkpoint() { if (++checked === 5) throw Error("cancelled"); } })).toThrow("cancelled");
  expect(checked).toBe(5);
});

test("Graph and Neural bind the one neutral type without a product forwarding API", () => {
  expect(existsSync(resolve(owner, "🦀️.rs"))).toBe(true);
  const neutral = read("🦀️.rs");
  expect(neutral).toContain("pub enum ValueType");
  expect(neutral).toContain("pub enum ValueKind");
  expect(neutral).not.toMatch(/neural_engine|semio_framework_os|serde::/u);
  const neural = readFileSync(resolve(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs"), "utf8");
  const graph = readFileSync(resolve(root, "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs"), "utf8");
  expect(neural).not.toContain("pub enum ValueType");
  expect(neural).not.toMatch(/pub use[^;]*ValueType/u);
  expect(graph).not.toMatch(/neural_engine|property_value_to_neural/u);
  expect(graph).toContain("if matches!(expected, ValueType::Any)");
  expect(graph).toContain("PropertyValue::Object(_) if matches!(expected, ValueType::Schema(_)) => true");
  const manifest = toml.parse(readFileSync(resolve(root, "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml"), "utf8"));
  expect((manifest.dependencies as Record<string, unknown>).neural_engine).toBeUndefined();
  expect(read("../📦️packages/🦀️rust/🦀️.rs")).toContain("pub use types::{ValueKind, ValueType}");
});

test("Graph retains its explicit property acceptance in an independent schema corpus", () => {
  expect(new Set(fixture.graphCases.map((row: { type: number; value: number }) => row.type + ":" + row.value)).size).toBe(63);
  for (const row of fixture.graphCases) {
    const type = fixture.types[row.type].type;
    const reference = ajv.compile(type.kind === "any" ? {} : type.kind === "boolean" ? { type: "boolean" } : type.kind === "decimal" ? { anyOf: [{ type: "boolean" }, { type: "number" }] } : type.kind === "text" ? { type: "string" } : type.kind === "schema" ? { type: "object" } : false);
    expect(reference(fixture.graphValues[row.value]), row.name).toBe(row.accepted);
  }
});

````

## 🧰️framework/🔨️modules/🌱️value/🏷️type/🧫️fixtures/🔣️.json

Bytes 18773; SHA-256 `7907552c5d1232813cf047d6625794ff2f2a68499a28c45aca6b4da37e98a398`.

````text
{
  "schemaVersion": 1,
  "types": [
    {
      "id": "boolean",
      "type": {
        "kind": "boolean"
      }
    },
    {
      "id": "integer",
      "type": {
        "kind": "integer"
      }
    },
    {
      "id": "number",
      "type": {
        "kind": "decimal"
      }
    },
    {
      "id": "text",
      "type": {
        "kind": "text"
      }
    },
    {
      "id": "list",
      "type": {
        "kind": "list",
        "of": {
          "kind": "boolean"
        }
      }
    },
    {
      "id": "point",
      "type": {
        "kind": "schema",
        "of": "point"
      }
    },
    {
      "id": "value",
      "type": {
        "kind": "any"
      }
    }
  ],
  "kinds": [
    {
      "kind": "null"
    },
    {
      "kind": "boolean"
    },
    {
      "kind": "integer"
    },
    {
      "kind": "decimal"
    },
    {
      "kind": "text"
    },
    {
      "kind": "dictionary"
    },
    {
      "kind": "dictionary",
      "schema": "list"
    },
    {
      "kind": "dictionary",
      "schema": "point"
    },
    {
      "kind": "dictionary",
      "schema": ""
    }
  ],
  "cases": [
    {
      "name": "boolean-null-0",
      "type": 0,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "boolean-boolean-1",
      "type": 0,
      "kind": 1,
      "accepted": true
    },
    {
      "name": "boolean-integer-2",
      "type": 0,
      "kind": 2,
      "accepted": false
    },
    {
      "name": "boolean-decimal-3",
      "type": 0,
      "kind": 3,
      "accepted": false
    },
    {
      "name": "boolean-text-4",
      "type": 0,
      "kind": 4,
      "accepted": false
    },
    {
      "name": "boolean-dictionary-5",
      "type": 0,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "boolean-dictionary-6",
      "type": 0,
      "kind": 6,
      "accepted": false
    },
    {
      "name": "boolean-dictionary-7",
      "type": 0,
      "kind": 7,
      "accepted": false
    },
    {
      "name": "boolean-dictionary-8",
      "type": 0,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "integer-null-0",
      "type": 1,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "integer-boolean-1",
      "type": 1,
      "kind": 1,
      "accepted": false
    },
    {
      "name": "integer-integer-2",
      "type": 1,
      "kind": 2,
      "accepted": true
    },
    {
      "name": "integer-decimal-3",
      "type": 1,
      "kind": 3,
      "accepted": false
    },
    {
      "name": "integer-text-4",
      "type": 1,
      "kind": 4,
      "accepted": false
    },
    {
      "name": "integer-dictionary-5",
      "type": 1,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "integer-dictionary-6",
      "type": 1,
      "kind": 6,
      "accepted": false
    },
    {
      "name": "integer-dictionary-7",
      "type": 1,
      "kind": 7,
      "accepted": false
    },
    {
      "name": "integer-dictionary-8",
      "type": 1,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "number-null-0",
      "type": 2,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "number-boolean-1",
      "type": 2,
      "kind": 1,
      "accepted": true
    },
    {
      "name": "number-integer-2",
      "type": 2,
      "kind": 2,
      "accepted": true
    },
    {
      "name": "number-decimal-3",
      "type": 2,
      "kind": 3,
      "accepted": true
    },
    {
      "name": "number-text-4",
      "type": 2,
      "kind": 4,
      "accepted": false
    },
    {
      "name": "number-dictionary-5",
      "type": 2,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "number-dictionary-6",
      "type": 2,
      "kind": 6,
      "accepted": false
    },
    {
      "name": "number-dictionary-7",
      "type": 2,
      "kind": 7,
      "accepted": false
    },
    {
      "name": "number-dictionary-8",
      "type": 2,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "text-null-0",
      "type": 3,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "text-boolean-1",
      "type": 3,
      "kind": 1,
      "accepted": false
    },
    {
      "name": "text-integer-2",
      "type": 3,
      "kind": 2,
      "accepted": false
    },
    {
      "name": "text-decimal-3",
      "type": 3,
      "kind": 3,
      "accepted": false
    },
    {
      "name": "text-text-4",
      "type": 3,
      "kind": 4,
      "accepted": true
    },
    {
      "name": "text-dictionary-5",
      "type": 3,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "text-dictionary-6",
      "type": 3,
      "kind": 6,
      "accepted": false
    },
    {
      "name": "text-dictionary-7",
      "type": 3,
      "kind": 7,
      "accepted": false
    },
    {
      "name": "text-dictionary-8",
      "type": 3,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "list-null-0",
      "type": 4,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "list-boolean-1",
      "type": 4,
      "kind": 1,
      "accepted": false
    },
    {
      "name": "list-integer-2",
      "type": 4,
      "kind": 2,
      "accepted": false
    },
    {
      "name": "list-decimal-3",
      "type": 4,
      "kind": 3,
      "accepted": false
    },
    {
      "name": "list-text-4",
      "type": 4,
      "kind": 4,
      "accepted": false
    },
    {
      "name": "list-dictionary-5",
      "type": 4,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "list-dictionary-6",
      "type": 4,
      "kind": 6,
      "accepted": true
    },
    {
      "name": "list-dictionary-7",
      "type": 4,
      "kind": 7,
      "accepted": false
    },
    {
      "name": "list-dictionary-8",
      "type": 4,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "point-null-0",
      "type": 5,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "point-boolean-1",
      "type": 5,
      "kind": 1,
      "accepted": false
    },
    {
      "name": "point-integer-2",
      "type": 5,
      "kind": 2,
      "accepted": false
    },
    {
      "name": "point-decimal-3",
      "type": 5,
      "kind": 3,
      "accepted": false
    },
    {
      "name": "point-text-4",
      "type": 5,
      "kind": 4,
      "accepted": false
    },
    {
      "name": "point-dictionary-5",
      "type": 5,
      "kind": 5,
      "accepted": false
    },
    {
      "name": "point-dictionary-6",
      "type": 5,
      "kind": 6,
      "accepted": false
    },
    {
      "name": "point-dictionary-7",
      "type": 5,
      "kind": 7,
      "accepted": true
    },
    {
      "name": "point-dictionary-8",
      "type": 5,
      "kind": 8,
      "accepted": false
    },
    {
      "name": "value-null-0",
      "type": 6,
      "kind": 0,
      "accepted": false
    },
    {
      "name": "value-boolean-1",
      "type": 6,
      "kind": 1,
      "accepted": true
    },
    {
      "name": "value-integer-2",
      "type": 6,
      "kind": 2,
      "accepted": true
    },
    {
      "name": "value-decimal-3",
      "type": 6,
      "kind": 3,
      "accepted": true
    },
    {
      "name": "value-text-4",
      "type": 6,
      "kind": 4,
      "accepted": true
    },
    {
      "name": "value-dictionary-5",
      "type": 6,
      "kind": 5,
      "accepted": true
    },
    {
      "name": "value-dictionary-6",
      "type": 6,
      "kind": 6,
      "accepted": true
    },
    {
      "name": "value-dictionary-7",
      "type": 6,
      "kind": 7,
      "accepted": true
    },
    {
      "name": "value-dictionary-8",
      "type": 6,
      "kind": 8,
      "accepted": true
    }
  ],
  "wire": [
    {
      "kind": "boolean"
    },
    {
      "kind": "integer"
    },
    {
      "kind": "decimal"
    },
    {
      "kind": "text"
    },
    {
      "kind": "list",
      "of": {
        "kind": "boolean"
      }
    },
    {
      "kind": "schema",
      "of": "point"
    },
    {
      "kind": "any"
    },
    {
      "kind": "list",
      "of": {
        "kind": "list",
        "of": {
          "kind": "text"
        }
      }
    }
  ],
  "refused": [
    null,
    true,
    0,
    "boolean",
    {},
    [],
    {
      "kind": "wrong"
    },
    {
      "kind": "list"
    },
    {
      "kind": "schema"
    },
    {
      "kind": "schema",
      "of": 0
    },
    {
      "kind": "boolean",
      "of": "wrong"
    },
    {
      "kind": "any",
      "extra": true
    },
    {
      "kind": "schema",
      "of": "point",
      "extra": true
    }
  ],
  "ownership": {
    "originalInputs": [
      {
        "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs",
        "sha256": "d960ab80d8b545649f24ec23cfb60b061249c3b032a139c473991efc9d737e4b"
      },
      {
        "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🦀️.rs",
        "sha256": "9aad5bf0d548a03b678c597623ec8d631a47b58b782c93088b2854014601748c"
      },
      {
        "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/Cargo.toml",
        "sha256": "26da977ee82c0ad87eab27f3935f048fba826cce9f8fdcc2e9c684fba4ee34d0"
      },
      {
        "path": "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml",
        "sha256": "ba2b9dc265c46784098448e55c3f6fbadc357e820288938bff75052650863cd2"
      },
      {
        "path": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs",
        "sha256": "7215a515fd2d8fdbbd9ca50a6590b56b522d57a87fc7b00015a141cb56d59c7e"
      },
      {
        "path": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts",
        "sha256": "671eb4fa2443d9c8f08e8ece07be127833cb158d1c873a1947b343f729f1014b"
      },
      {
        "path": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json",
        "sha256": "1c80a474365ea37e0259c3b034ca5e8434efb41ce17bcc8c09160609e8c95515"
      },
      {
        "path": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/package.json",
        "sha256": "0ffdc52dd3fadc49e05fa87d5d99f163542eead6137a7a742582bc845f5d7f24"
      },
      {
        "path": "🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs",
        "sha256": "cbaf1577e77e9661ccc8b6d724b084841cfd13c095856d5755f2d7b269b76f87"
      },
      {
        "path": "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs",
        "sha256": "64552f21a7ed7f1f9fdf654646874a19007b4350f62586c5eb16c1148737f484"
      },
      {
        "path": "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs",
        "sha256": "f48b9eb010e7da598eeef60c8dd663bfc08af66d0eaf2df58050cad0f2fbba2d"
      },
      {
        "path": "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️.rs",
        "sha256": "484003e849bdfae3f62237095b2ba1c947d02da0ed22285e99b631de4ac07d6b"
      },
      {
        "path": "✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️.rs",
        "sha256": "8291e22a93d197dba63591b6b30117e307e337b238736528abeef14ca6998558"
      },
      {
        "path": "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🦀️.rs",
        "sha256": "ef5a898caa3c5ed33b64365cbdc7cc06ae92925e8510a5b6cd7fdf6785063b77"
      },
      {
        "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs",
        "sha256": "a266704d641a8628cb83b80bdb9be9968b7ccc25461a6ae351d2d92cd5a4eb81"
      }
    ],
    "originalTypeBlockSha": "9b7081835e8304c9d3e51b222b61323698287be37a05683eb876c031b2670b3c",
    "neuralRemainderSha": "1ffe951b324bf966d407d201ab4fc6f508c13be9ae01286dd6cfcdb1a9a2fb1f"
  },
  "graphValues": [
    null,
    false,
    true,
    1.5,
    2,
    "text",
    [],
    {},
    {
      "$schema": "foreign"
    }
  ],
  "graphCases": [
    {
      "name": "boolean-graph-0",
      "type": 0,
      "value": 0,
      "accepted": false
    },
    {
      "name": "boolean-graph-1",
      "type": 0,
      "value": 1,
      "accepted": true
    },
    {
      "name": "boolean-graph-2",
      "type": 0,
      "value": 2,
      "accepted": true
    },
    {
      "name": "boolean-graph-3",
      "type": 0,
      "value": 3,
      "accepted": false
    },
    {
      "name": "boolean-graph-4",
      "type": 0,
      "value": 4,
      "accepted": false
    },
    {
      "name": "boolean-graph-5",
      "type": 0,
      "value": 5,
      "accepted": false
    },
    {
      "name": "boolean-graph-6",
      "type": 0,
      "value": 6,
      "accepted": false
    },
    {
      "name": "boolean-graph-7",
      "type": 0,
      "value": 7,
      "accepted": false
    },
    {
      "name": "boolean-graph-8",
      "type": 0,
      "value": 8,
      "accepted": false
    },
    {
      "name": "integer-graph-0",
      "type": 1,
      "value": 0,
      "accepted": false
    },
    {
      "name": "integer-graph-1",
      "type": 1,
      "value": 1,
      "accepted": false
    },
    {
      "name": "integer-graph-2",
      "type": 1,
      "value": 2,
      "accepted": false
    },
    {
      "name": "integer-graph-3",
      "type": 1,
      "value": 3,
      "accepted": false
    },
    {
      "name": "integer-graph-4",
      "type": 1,
      "value": 4,
      "accepted": false
    },
    {
      "name": "integer-graph-5",
      "type": 1,
      "value": 5,
      "accepted": false
    },
    {
      "name": "integer-graph-6",
      "type": 1,
      "value": 6,
      "accepted": false
    },
    {
      "name": "integer-graph-7",
      "type": 1,
      "value": 7,
      "accepted": false
    },
    {
      "name": "integer-graph-8",
      "type": 1,
      "value": 8,
      "accepted": false
    },
    {
      "name": "number-graph-0",
      "type": 2,
      "value": 0,
      "accepted": false
    },
    {
      "name": "number-graph-1",
      "type": 2,
      "value": 1,
      "accepted": true
    },
    {
      "name": "number-graph-2",
      "type": 2,
      "value": 2,
      "accepted": true
    },
    {
      "name": "number-graph-3",
      "type": 2,
      "value": 3,
      "accepted": true
    },
    {
      "name": "number-graph-4",
      "type": 2,
      "value": 4,
      "accepted": true
    },
    {
      "name": "number-graph-5",
      "type": 2,
      "value": 5,
      "accepted": false
    },
    {
      "name": "number-graph-6",
      "type": 2,
      "value": 6,
      "accepted": false
    },
    {
      "name": "number-graph-7",
      "type": 2,
      "value": 7,
      "accepted": false
    },
    {
      "name": "number-graph-8",
      "type": 2,
      "value": 8,
      "accepted": false
    },
    {
      "name": "text-graph-0",
      "type": 3,
      "value": 0,
      "accepted": false
    },
    {
      "name": "text-graph-1",
      "type": 3,
      "value": 1,
      "accepted": false
    },
    {
      "name": "text-graph-2",
      "type": 3,
      "value": 2,
      "accepted": false
    },
    {
      "name": "text-graph-3",
      "type": 3,
      "value": 3,
      "accepted": false
    },
    {
      "name": "text-graph-4",
      "type": 3,
      "value": 4,
      "accepted": false
    },
    {
      "name": "text-graph-5",
      "type": 3,
      "value": 5,
      "accepted": true
    },
    {
      "name": "text-graph-6",
      "type": 3,
      "value": 6,
      "accepted": false
    },
    {
      "name": "text-graph-7",
      "type": 3,
      "value": 7,
      "accepted": false
    },
    {
      "name": "text-graph-8",
      "type": 3,
      "value": 8,
      "accepted": false
    },
    {
      "name": "list-graph-0",
      "type": 4,
      "value": 0,
      "accepted": false
    },
    {
      "name": "list-graph-1",
      "type": 4,
      "value": 1,
      "accepted": false
    },
    {
      "name": "list-graph-2",
      "type": 4,
      "value": 2,
      "accepted": false
    },
    {
      "name": "list-graph-3",
      "type": 4,
      "value": 3,
      "accepted": false
    },
    {
      "name": "list-graph-4",
      "type": 4,
      "value": 4,
      "accepted": false
    },
    {
      "name": "list-graph-5",
      "type": 4,
      "value": 5,
      "accepted": false
    },
    {
      "name": "list-graph-6",
      "type": 4,
      "value": 6,
      "accepted": false
    },
    {
      "name": "list-graph-7",
      "type": 4,
      "value": 7,
      "accepted": false
    },
    {
      "name": "list-graph-8",
      "type": 4,
      "value": 8,
      "accepted": false
    },
    {
      "name": "point-graph-0",
      "type": 5,
      "value": 0,
      "accepted": false
    },
    {
      "name": "point-graph-1",
      "type": 5,
      "value": 1,
      "accepted": false
    },
    {
      "name": "point-graph-2",
      "type": 5,
      "value": 2,
      "accepted": false
    },
    {
      "name": "point-graph-3",
      "type": 5,
      "value": 3,
      "accepted": false
    },
    {
      "name": "point-graph-4",
      "type": 5,
      "value": 4,
      "accepted": false
    },
    {
      "name": "point-graph-5",
      "type": 5,
      "value": 5,
      "accepted": false
    },
    {
      "name": "point-graph-6",
      "type": 5,
      "value": 6,
      "accepted": false
    },
    {
      "name": "point-graph-7",
      "type": 5,
      "value": 7,
      "accepted": true
    },
    {
      "name": "point-graph-8",
      "type": 5,
      "value": 8,
      "accepted": true
    },
    {
      "name": "value-graph-0",
      "type": 6,
      "value": 0,
      "accepted": true
    },
    {
      "name": "value-graph-1",
      "type": 6,
      "value": 1,
      "accepted": true
    },
    {
      "name": "value-graph-2",
      "type": 6,
      "value": 2,
      "accepted": true
    },
    {
      "name": "value-graph-3",
      "type": 6,
      "value": 3,
      "accepted": true
    },
    {
      "name": "value-graph-4",
      "type": 6,
      "value": 4,
      "accepted": true
    },
    {
      "name": "value-graph-5",
      "type": 6,
      "value": 5,
      "accepted": true
    },
    {
      "name": "value-graph-6",
      "type": 6,
      "value": 6,
      "accepted": true
    },
    {
      "name": "value-graph-7",
      "type": 6,
      "value": 7,
      "accepted": true
    },
    {
      "name": "value-graph-8",
      "type": 6,
      "value": 8,
      "accepted": true
    }
  ]
}

````

## 🧰️framework/🔨️modules/🌱️value/🏷️type/🧬️schema/🔣️.json

Bytes 5536; SHA-256 `6221dfc8b8b4f1fcb205e0e9f3453b644cf7f75841c02874e322befe2e438465`.

````text
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://json.schemas.assets.semio-tech.com/framework/value/type/fixture.json",
  "type": "object",
  "properties": {
    "schemaVersion": {
      "const": 1
    },
    "types": {
      "type": "array",
      "minItems": 7,
      "maxItems": 7,
      "items": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string"
          },
          "type": {
            "$ref": "#/$defs/valueType"
          }
        },
        "required": [
          "id",
          "type"
        ],
        "additionalProperties": false
      }
    },
    "kinds": {
      "type": "array",
      "minItems": 9,
      "maxItems": 9,
      "items": {
        "$ref": "#/$defs/valueKind"
      }
    },
    "cases": {
      "type": "array",
      "minItems": 63,
      "maxItems": 63,
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string"
          },
          "type": {
            "type": "integer",
            "minimum": 0,
            "maximum": 6
          },
          "kind": {
            "type": "integer",
            "minimum": 0,
            "maximum": 8
          },
          "accepted": {
            "type": "boolean"
          }
        },
        "required": [
          "name",
          "type",
          "kind",
          "accepted"
        ],
        "additionalProperties": false
      }
    },
    "wire": {
      "type": "array",
      "minItems": 8,
      "items": {
        "$ref": "#/$defs/valueType"
      }
    },
    "refused": {
      "type": "array",
      "minItems": 10
    },
    "ownership": {
      "type": "object",
      "properties": {
        "originalInputs": {
          "type": "array",
          "minItems": 15,
          "maxItems": 15,
          "items": {
            "type": "object",
            "properties": {
              "path": {
                "type": "string"
              },
              "sha256": {
                "type": "string",
                "pattern": "^[0-9a-f]{64}$"
              }
            },
            "required": [
              "path",
              "sha256"
            ],
            "additionalProperties": false
          }
        },
        "originalTypeBlockSha": {
          "type": "string",
          "pattern": "^[0-9a-f]{64}$"
        },
        "neuralRemainderSha": {
          "type": "string",
          "pattern": "^[0-9a-f]{64}$"
        }
      },
      "required": [
        "originalInputs",
        "originalTypeBlockSha",
        "neuralRemainderSha"
      ],
      "additionalProperties": false
    },
    "graphValues": {
      "type": "array",
      "minItems": 9,
      "maxItems": 9
    },
    "graphCases": {
      "type": "array",
      "minItems": 63,
      "maxItems": 63,
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string"
          },
          "type": {
            "type": "integer",
            "minimum": 0,
            "maximum": 6
          },
          "value": {
            "type": "integer",
            "minimum": 0,
            "maximum": 8
          },
          "accepted": {
            "type": "boolean"
          }
        },
        "required": [
          "name",
          "type",
          "value",
          "accepted"
        ],
        "additionalProperties": false
      }
    }
  },
  "required": [
    "schemaVersion",
    "types",
    "kinds",
    "cases",
    "wire",
    "refused",
    "ownership",
    "graphValues",
    "graphCases"
  ],
  "additionalProperties": false,
  "$defs": {
    "valueType": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "enum": [
                "boolean",
                "integer",
                "decimal",
                "text",
                "any"
              ]
            }
          },
          "required": [
            "kind"
          ],
          "additionalProperties": false
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "list"
            },
            "of": {
              "$ref": "#/$defs/valueType"
            }
          },
          "required": [
            "kind",
            "of"
          ],
          "additionalProperties": false
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "schema"
            },
            "of": {
              "type": "string"
            }
          },
          "required": [
            "kind",
            "of"
          ],
          "additionalProperties": false
        }
      ]
    },
    "valueKind": {
      "oneOf": [
        {
          "type": "object",
          "properties": {
            "kind": {
              "enum": [
                "null",
                "boolean",
                "integer",
                "decimal",
                "text"
              ]
            }
          },
          "required": [
            "kind"
          ],
          "additionalProperties": false
        },
        {
          "type": "object",
          "properties": {
            "kind": {
              "const": "dictionary"
            },
            "schema": {
              "type": "string"
            }
          },
          "required": [
            "kind"
          ],
          "additionalProperties": false
        }
      ]
    }
  }
}

````

## 🧰️framework/🔨️modules/🌱️value/🏷️type/🧪️tests/🦀️.rs

Bytes 2326; SHA-256 `f252bd1d967c2165510f0e81ced7799e60edba52dadca034bba52286bf9c0e48`.

````text
//! 🧪️ Language-neutral type classifications and canonical wire laws.

use super::{ValueKind, ValueType};
use crate::{DslValue, FromValue, ToValue};

fn corpus() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}

#[test]
fn value_type_all_owned_classifications_match_the_closed_corpus() {
    let fixture = corpus();
    for row in fixture["cases"].as_array().unwrap() {
        let type_row = &fixture["types"][row["type"].as_u64().unwrap() as usize];
        let value_type = ValueType::from_value(DslValue::from(type_row["type"].clone())).unwrap();
        let kind = &fixture["kinds"][row["kind"].as_u64().unwrap() as usize];
        let classification = match kind["kind"].as_str().unwrap() {
            "null" => ValueKind::Null,
            "boolean" => ValueKind::Boolean,
            "integer" => ValueKind::Integer,
            "decimal" => ValueKind::Decimal,
            "text" => ValueKind::Text,
            "dictionary" => ValueKind::Dictionary(kind.get("schema").and_then(serde_json::Value::as_str)),
            other => panic!("unknown kind {other}"),
        };
        assert_eq!(value_type.id(), type_row["id"].as_str().unwrap(), "{}", row["name"]);
        assert_eq!(value_type.matches(classification), row["accepted"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[test]
fn value_type_wire_round_trips_every_original_variant_and_nested_list() {
    for row in corpus()["wire"].as_array().unwrap() {
        let subject = ValueType::from_value(DslValue::from(row.clone())).unwrap();
        let emitted = serde_json::Value::from(subject.to_value());
        assert_eq!(&emitted, row);
        assert_eq!(ValueType::from_value(subject.to_value()).unwrap(), subject);
    }
}

#[test]
fn value_type_wire_refuses_closed_hostile_objects_and_duplicate_fields() {
    for row in corpus()["refused"].as_array().unwrap() {
        assert!(ValueType::from_value(DslValue::from(row.clone())).is_err(), "{row}");
    }
    for fields in [
        vec![("kind".into(), "boolean".to_value()), ("kind".into(), "boolean".to_value())],
        vec![("kind".into(), "list".to_value()), ("of".into(), "boolean".to_value()), ("of".into(), "boolean".to_value())],
    ] {
        assert!(ValueType::from_value(DslValue::Object(fields)).is_err());
    }
}

````

## 🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🏷️type/🦀️.rs

Bytes 1362; SHA-256 `61dbcd79c8a923c9a124b474537da372b50b48e2ce41a78c47ed74522325a666`.

````text
//! 🎯️ Original Graph property admission against the lower language-neutral corpus.

use super::{dsl_value_to_property_value, property_value_matches_type};
use semio_framework_value::{FromValue, ValueType};

#[test]
fn graph_properties_preserve_all_original_type_classifications() {
    let fixture = dsl_core::json::to_dsl_value(&dsl_core::json::parse(include_str!("../../../../🌱️value/🏷️type/🧫️fixtures/🔣️.json")).unwrap());
    let cases = fixture.get("graphCases").and_then(dsl_core::DslValue::as_array).unwrap();
    let types = fixture.get("types").and_then(dsl_core::DslValue::as_array).unwrap();
    let values = fixture.get("graphValues").and_then(dsl_core::DslValue::as_array).unwrap();
    for row in cases {
        let type_index = u64::from_value(row.get("type").unwrap().clone()).unwrap() as usize;
        let value_index = u64::from_value(row.get("value").unwrap().clone()).unwrap() as usize;
        let value_type = ValueType::from_value(types[type_index].get("type").unwrap().clone()).unwrap();
        let property = dsl_value_to_property_value(&values[value_index]);
        let expected = bool::from_value(row.get("accepted").unwrap().clone()).unwrap();
        assert_eq!(property_value_matches_type(&property, &value_type), expected, "{}", String::from_value(row.get("name").unwrap().clone()).unwrap());
    }
}

````
