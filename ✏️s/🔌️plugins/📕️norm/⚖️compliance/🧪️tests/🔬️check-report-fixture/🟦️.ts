import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(join(here, "../🧫️fixtures/🔬️check-report/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(here, "../../🧬️schema/🔣️.json"), "utf8"));

describe("Norm compliance CheckReport fixture", () => {
  test("third-party Ajv accepts the language-agnostic CheckReport fixture", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
    expect(validate({ ...fixture, checks: "nope" })).toBe(false);
    expect(fixture.summary.fail).toBe(1);
    expect(fixture.checks[0].remedies.length).toBeGreaterThan(0);
    expect(fixture.checks[0].status).toBe("Fail");
  });

  test("stations obey the law utilization == max(stations) and the schema bounds their dimension", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    const withStations = fixture.checks.filter((c: { stations: unknown[] }) => c.stations.length > 0);
    expect(withStations.length).toBeGreaterThan(0);
    for (const c of withStations) {
      expect(Math.max(...c.stations.map((s: { utilization: number }) => s.utilization))).toBeCloseTo(c.utilization, 12);
    }
    const broken = structuredClone(fixture);
    broken.checks[2].stations[0].at = [0, 1, 2, 3];
    expect(validate(broken)).toBe(false);
    broken.checks[2].stations[0].at = [0.5];
    broken.checks[2].stations[0].utilization = -1;
    expect(validate(broken)).toBe(false);
  });

  test("an unverified basis blocks compliance and cites its NDP ids", () => {
    const unverified = fixture.checks.filter((c: { status: string }) => c.status === "UnverifiedBasis");
    expect(unverified.length).toBe(fixture.summary.unverified);
    expect(unverified.every((c: { ndps: string[] }) => c.ndps.length > 0)).toBe(true);
    expect(fixture.summary.complies).toBe(false);
  });

  test("third-party Ajv accepts the committed NDP registry and refuses unknown statuses", () => {
    const registry = JSON.parse(readFileSync(join(here, "../../📜️annex-and-tables/🔖️ndp/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true }).compile({ ...schema, $ref: "#/definitions/NdpRegistry", required: undefined, properties: undefined, additionalProperties: undefined, type: undefined });
    expect(validate(registry)).toBe(true);
    const ids = registry.entries.map((e: { id: string }) => e.id);
    expect(ids).toEqual([...ids].sort());
    expect(new Set(ids).size).toBe(ids.length);
    expect(validate({ entries: [{ ...registry.entries[0], status: "maybe" }] })).toBe(false);
  });
});
