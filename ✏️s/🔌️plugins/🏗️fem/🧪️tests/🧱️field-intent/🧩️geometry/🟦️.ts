import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { Root } from "protobufjs";
import { binary64, binary64Value, type Binary64 } from "@semio-tech/value-ts/ieee754";
import { parseFem2dRegionPatch } from "../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";
import { parseFem3dSolidPatch } from "../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";

const owner = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const capture = (value: unknown, key = ""): unknown => typeof value === "number" ? key === "layers" ? BigInt(value) : binary64(value) : Array.isArray(value) ? value.map(item => capture(item)) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).map(([key, item]) => [key, capture(item, key)])) : value;
const query = (value: unknown): unknown => typeof value === "bigint" ? Number(value) : Array.isArray(value) ? value.map(query) : value && typeof value === "object" ? "bits" in value ? binary64Value(value as Binary64) : Object.fromEntries(Object.entries(value).map(([key, item]) => [key, query(item)])) : value;

describe("region and solid field intent mirrors", () => {
  for (const dimension of ["2d", "3d"] as const) {
    const entity = dimension === "2d" ? "region" : "solid";
    const title = dimension === "2d" ? "Region" : "Solid";
    const collection = dimension === "2d" ? "regions" : "solids";
    const subsets = resolve(owner, `🗿️artifacts/${dimension === "2d" ? "◻️2d" : "🧊️3d"}/🏅️standards/🔖️1/🪆️subsets`);
    const fixtures = resolve(subsets, `🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-${entity}`);
    const validate = new Ajv({ strict: false, strictNumbers: true, validateFormats: false }).compile(read(resolve(subsets, `🕸️mesh/🧬️schema/🧬️mutations/🩹️change-${entity}/🧬️schema/🔣️.json`)));
    const parsePatch = dimension === "2d" ? parseFem2dRegionPatch : parseFem3dSolidPatch;

    it(`${dimension} preserves name and owned empty holes in either event order`, () => {
      const before = read(resolve(fixtures, "🩹️changes/📸️snapshot/⬅️before/🔣️.json"))[collection][0];
      const events = ["🩹️changes", "📐️geometry"].map(scenario => read(resolve(fixtures, scenario, "🦠️mutation/🔣️.json")));
      for (const sequence of [events, [...events].reverse()]) {
        let actual = { ...before };
        let oracle = { ...before };
        for (const event of sequence) {
          expect(validate(event), JSON.stringify(validate.errors)).toBe(true);
          actual = { ...actual, ...query(parsePatch(capture(event.patch))) as object };
          oracle = applyPatch(oracle, Object.entries(event.patch).map(([key, value]) => ({ op: "replace" as const, path: `/${key}`, value })), true, false).newDocument;
          expect(actual).toEqual(oracle);
        }
        expect(actual.name).toBe(`Structural ${title}`);
        expect(actual.holes).toEqual([]);
        expect(actual.outline).toEqual(before.outline);
      }
    });

    it(`${dimension} admits polygon fields and refuses malformed shapes or scalars`, () => {
      const outline = [[0, 0], [4, 0], [4, 4], [0, 4]];
      expect(query(parsePatch(capture({ outline, holes: [] })))).toEqual({ outline, holes: [] });
      const invalid = [{ id: "other" }, { name: null }, { outline: null }, { outline: [[0, 0], [1, 1]] }, { outline: [[0, 0, 0], [1, 0], [1, 1]] }, { holes: null }, { meshSize: 0 }, { meshSize: Infinity }, ...(dimension === "3d" ? [{ layers: 0 }, { axis: "w" }] : [{ thickness: -1 }])];
      for (const patch of invalid) {
        expect(validate({ mutation: `change${title}`, id: "target", patch })).toBe(false);
        expect(() => parsePatch(capture(patch))).toThrow();
      }
      expect(parsePatch({})).toEqual({});
    });

    it(`${dimension} protobuf retains empty geometry-field presence`, () => {
      const root = new Root().loadSync(resolve(subsets, "🌐️any/🧬️schema/🧬️mutations/🛰️.proto")).resolveAll();
      const type = root.lookupType(`semio.s.fem.fem${dimension}.mutation.Fem${dimension}Change${title}`);
      const withEmptyHoles = type.decode(type.encode(type.create({ id: "target", patch: { holes: { rings: [] } } })).finish()) as unknown as { patch: { holes?: { rings: unknown[] }; outline?: unknown } };
      expect(Object.hasOwn(withEmptyHoles.patch, "holes")).toBe(true);
      expect(withEmptyHoles.patch.holes?.rings).toEqual([]);
      expect(Object.hasOwn(withEmptyHoles.patch, "outline")).toBe(false);
      const omitted = type.decode(type.encode(type.create({ id: "target", patch: { name: "Renamed" } })).finish()) as unknown as { patch: object };
      expect(Object.hasOwn(omitted.patch, "holes")).toBe(false);
    });
  }
});
