import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("cold reference builder wire values and disposal agree with independent SQL", () => {
  expect(new Ajv({ strict: true }).compile(schema)(corpus)).toBe(true);
  expect(corpus.disposal).toBe("owned-wire");
  expect(corpus.nativeOperation).toBe(false);
  const database = new Database(":memory:");
  try {
    for (const row of corpus.cases) {
      const oracle: any = database.query("SELECT CASE WHEN ? = 'absorb' AND ? THEN NULL WHEN ? OR ? THEN ? ELSE ? END AS result, (? = 'absorb' AND ?) AS error, (? = 'mutate') AS mutationsRetired, (? = 'absorb' OR ? OR ?) AS diffsRetired, (1 + (? AND NOT ?)) AS projectionsRetired").get(row.entry, row.applyRefusal, row.semanticRefusal, row.applyRefusal, corpus.base, corpus.value, row.entry, row.applyRefusal, row.entry, row.entry, row.semanticRefusal, row.applyRefusal, row.applyRefusal, row.semanticRefusal) as Record<string, unknown>;
      expect({ ...oracle, error: Boolean(oracle.error) }).toEqual({ result: row.result, error: row.error, mutationsRetired: row.mutationsRetired, diffsRetired: row.diffsRetired, projectionsRetired: row.projectionsRetired });
    }
  } finally { database.close(); }
});

