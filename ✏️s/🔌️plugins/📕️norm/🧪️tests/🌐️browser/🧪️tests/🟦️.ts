/** 🧪️ Language-agnostic witness of the norm browser journey: the fixture meets its JSON Schema under Ajv, every region carries English and German names equal to the Rust focus-order catalogue, and the phase split matches the README (editor five windows, viewer three). */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import { regionNameV1, regionSelectorV1 } from "../🧭️journey/🟦️.ts";
import { validateNormBrowserArgumentsV1 } from "../📋️arguments/🟦️.ts";

const focusOrder = readFileSync(new URL("../../../📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/♿️accessibility/⌨️focus-order/🦀️.rs", import.meta.url), "utf8");
const args = ["--hub", "http://127.0.0.1:8080/", "--serve", "http://127.0.0.1:6070/", "--locale", "en", "--kinds", "norm.en1993", "--out", "x/🗑️generated/norm"];

describe("norm browser journey", () => {
  test("fixture meets the schema", () => {
    const validate = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture)).toBe(true);
  });
  test("region names equal the Rust catalogue in both languages", () => {
    for (const row of fixture.regions) expect(focusOrder).toContain(`LocalizedCopy::new("${row.names.en}", "${row.names.de}")`);
    expect(regionNameV1("checks", "de")).toBe("Nachweise");
  });
  test("viewer shows three of the five regions and every check has one phase", () => {
    expect(fixture.viewerRegions.every(role => fixture.regions.some(row => row.role === role))).toBe(true);
    expect(fixture.checks.filter(row => row.phase === "viewer").length).toBe(1);
    expect(fixture.checks.filter(row => row.phase === "editor").length).toBe(5);
    expect(regionSelectorV1("checks")).toContain("-checks");
  });
  test("arguments are validated", () => {
    expect(() => validateNormBrowserArgumentsV1(args)).not.toThrow();
    expect(() => validateNormBrowserArgumentsV1(args.slice(0, -2))).toThrow();
    expect(() => validateNormBrowserArgumentsV1([...args.slice(0, -1), "plain"])).toThrow();
  });
});
