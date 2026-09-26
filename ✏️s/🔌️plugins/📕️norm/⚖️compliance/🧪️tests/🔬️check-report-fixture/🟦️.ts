import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const fixture = JSON.parse(readFileSync(join(here, "../../🎫️fixtures/🔬️check-report/🔣️.json"), "utf8"));
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
});
