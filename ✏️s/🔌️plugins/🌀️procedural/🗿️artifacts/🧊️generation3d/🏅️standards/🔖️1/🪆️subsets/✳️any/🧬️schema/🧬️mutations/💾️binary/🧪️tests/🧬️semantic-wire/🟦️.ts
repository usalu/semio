import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { strict as assert } from "node:assert";
import Ajv from "ajv";

/** 🧬️ Compares the authored semantic records with independent JSON-schema and protocol-tag oracles. */
export function assertGeneration3dSemanticWire(): number {
  const root = resolve(import.meta.dir, "../..");
  const read = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
  const corpus = read("🧫️fixtures/🧬️semantic-wire/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword({ keyword: "x-semio-ui", metaSchema: { type: "object" } }).addKeyword({ keyword: "x-semio-invariant", metaSchema: { type: "array", items: { type: "object" } } });
  assert(ajv.compile(read("🧬️schema/🧬️semantic-wire/🔣️.json"))(corpus));
  const tags = new Map([...readFileSync(resolve(root, "📡️.protocol.semio"), "utf8").matchAll(/^record (\S+) tag=(\d+)$/gm)].map(match => [match[1], Number(match[2])]));
  for (const row of corpus.cases) {
    assert.equal(tags.get(row.keyword), row.tag, row.keyword);
    const validate = ajv.compile(JSON.parse(readFileSync(resolve(root, "..", row.source, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(row.mutation), JSON.stringify(validate.errors));
    const hostile = { ...row.mutation, mutation: "forged" };
    assert.equal(validate(hostile), false, row.keyword);
  }
  assert.equal(new Set(corpus.cases.map((row: any) => row.keyword)).size, 5);
  assert.equal(new Set(corpus.cases.map((row: any) => row.tag)).size, 5);
  return 12;
}
