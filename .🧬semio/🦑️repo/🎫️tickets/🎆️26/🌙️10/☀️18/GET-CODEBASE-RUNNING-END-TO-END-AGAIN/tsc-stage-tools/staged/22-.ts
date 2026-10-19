import { describe, expect, test } from "bun:test";
import iconv from "iconv-lite";
import fixture from "../🧫️fixtures/🧮️path-budget/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { pathBudgetCodes, pathBudgetFitsWithReserve, pathLongestComponentUtf8Bytes, pathUtf16Units } from "../../🔍️discovery/📏️path-budget/🟦️.ts";

const render = (parts: readonly Readonly<{ text: string; count: number }>[]): string => parts.map(({ text, count }) => text.repeat(count)).join("");
const thirdPartyUnits = (value: string): number => iconv.encode(value, "utf16-le").length / 2;
const thirdPartyComponentBytes = (value: string): number => Math.max(0, ...value.split(/[\\/]/u).map((component) => iconv.encode(component, "utf8").length));

describe("path budget (language-agnostic fixture)", () => {
  test("the fixture policy is the taxonomy collision policy", () => {
    const policy = loadCatalogTaxonomy().collisionPolicy;
    expect({ maxPathUtf16Units: policy.maxPathUtf16Units, maxComponentUtf8Bytes: policy.maxComponentUtf8Bytes }).toEqual(fixture.policy);
  });

  test("the policy is the AGENTS.md path rule: 256 UTF-16 units, 255 UTF-8 bytes per component", () => {
    expect(fixture.policy).toEqual({ maxPathUtf16Units: 256, maxComponentUtf8Bytes: 255 });
  });

  for (const row of fixture.cases) {
    test(`${row.id}: ${row.note}`, () => {
      const path = render(row.parts);
      expect(pathUtf16Units(path)).toBe(row.utf16Units);
      expect(pathLongestComponentUtf8Bytes(path)).toBe(row.longestComponentUtf8Bytes);
      expect<readonly string[]>(pathBudgetCodes(path, fixture.policy)).toEqual(row.codes);
    });

    test(`${row.id}: iconv-lite measures the same units and bytes`, () => {
      const path = render(row.parts);
      expect(thirdPartyUnits(path)).toBe(row.utf16Units);
      expect(thirdPartyComponentBytes(path)).toBe(row.longestComponentUtf8Bytes);
    });
  }

  for (const row of fixture.reserveCases) {
    test(`${row.id}: root plus reserved suffix`, () => {
      const root = render(row.parts);
      expect(pathUtf16Units(root)).toBe(row.rootUtf16Units);
      expect(thirdPartyUnits(root)).toBe(row.rootUtf16Units);
      expect(pathBudgetFitsWithReserve(root, row.reserveUtf16Units, fixture.policy)).toBe(row.fits);
    });
  }

  test("the byte budget of the previous policy would have refused a path that fits the real limits", () => {
    const row = fixture.cases.find(({ id }) => id === "old-byte-budget-regression")!;
    const path = render(row.parts);
    expect(Buffer.byteLength(path, "utf8")).toBeGreaterThan(240);
    expect(pathBudgetCodes(path, fixture.policy)).toEqual([]);
  });
});
