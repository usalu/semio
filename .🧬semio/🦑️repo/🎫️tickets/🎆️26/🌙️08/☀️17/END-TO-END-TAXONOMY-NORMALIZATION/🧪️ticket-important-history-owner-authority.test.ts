import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

//#region 🧪️TicketImportantHistoryOwnerAuthority
type HistoryVector = Readonly<{
  id: string;
  ownerClaimed: boolean;
  manifestState: "closed" | "invalid" | "missing" | "open";
  sourceByteLength: number;
  expectedDisposition: "project" | "unclaimed";
  expectedDestinationSuffix: "📓️important/📝️.md" | null;
}>;

const fixturePath = resolve(import.meta.dir, "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️ticket-important-history-owner-authority/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Readonly<{ schemaVersion: number; cases: readonly HistoryVector[] }>;
const schema = {
  type: "object",
  additionalProperties: false,
  required: ["schemaVersion", "cases"],
  properties: {
    schemaVersion: { const: 1 },
    cases: {
      type: "array",
      minItems: 7,
      maxItems: 7,
      items: {
        type: "object",
        additionalProperties: false,
        required: ["id", "ownerClaimed", "manifestState", "sourceByteLength", "expectedDisposition", "expectedDestinationSuffix"],
        properties: {
          id: { type: "string", minLength: 1 },
          ownerClaimed: { type: "boolean" },
          manifestState: { enum: ["closed", "invalid", "missing", "open"] },
          sourceByteLength: { type: "integer", minimum: 0 },
          expectedDisposition: { enum: ["project", "unclaimed"] },
          expectedDestinationSuffix: { anyOf: [{ const: "📓️important/📝️.md" }, { type: "null" }] },
        },
      },
    },
  },
} as const;

describe("ticket important history owner authority", () => {
  test("keeps the language-neutral matrix exact", () => {
    expect(fixture.schemaVersion).toBe(1);
    expect(fixture.cases.map((entry) => entry.id)).toEqual([
      "closed-nonzero",
      "invalid-manifest-nonzero",
      "missing-manifest-nonzero",
      "missing-manifest-zero",
      "open-nonzero",
      "closed-zero",
      "counterfeit-owner",
    ]);
    expect(new Set(fixture.cases.map((entry) => entry.id)).size).toBe(7);
    expect(fixture.cases.filter((entry) => entry.expectedDisposition === "project")).toHaveLength(4);
    expect(fixture.cases.filter((entry) => entry.expectedDisposition === "unclaimed")).toHaveLength(3);
    for (const entry of fixture.cases) expect(entry.expectedDestinationSuffix === null).toBe(entry.expectedDisposition === "unclaimed");
  });

  test("matches an independent JSON Schema implementation", () => {
    const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const forged = structuredClone(fixture) as { cases: { expectedDestinationSuffix: string | null }[] };
    forged.cases[0].expectedDestinationSuffix = "📌️important/📝️.md";
    expect(validate(forged)).toBe(false);
  });
});
//#endregion 🧪️TicketImportantHistoryOwnerAuthority
