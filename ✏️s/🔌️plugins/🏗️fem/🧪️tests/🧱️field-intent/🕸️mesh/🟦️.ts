import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { Root } from "protobufjs";
import { parse, buildASTSchema } from "graphql";
import { binary64, binary64Value, type Binary64 } from "@semio-tech/value-ts/ieee754";
import { parseFem2dNodePatch, parseFem2dSectionPatch } from "../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";
import { parseFem3dNodePatch, parseFem3dSectionPatch } from "../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";

const owner = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const capture = (patch: Record<string, unknown>) => Object.fromEntries(Object.entries(patch).map(([key, value]) => [key, typeof value === "number" ? binary64(value) : value]));

describe("node and section field intent mirrors", () => {
  for (const dimension of ["2d", "3d"] as const) {
    const subsets = resolve(owner, `🗿️artifacts/${dimension === "2d" ? "◻️2d" : "🧊️3d"}/🏅️standards/🔖️1/🪆️subsets`);
    for (const entity of ["node", "section"] as const) {
      const title = entity === "node" ? "Node" : "Section";
      const parsePatch = entity === "node" ? dimension === "2d" ? parseFem2dNodePatch : parseFem3dNodePatch : dimension === "2d" ? parseFem2dSectionPatch : parseFem3dSectionPatch;
      const schema = read(resolve(subsets, `🕸️mesh/🧬️schema/🧬️mutations/🩹️change-${entity}/🧬️schema/🔣️.json`));
      const validate = new Ajv({ strict: false, strictNumbers: true, validateFormats: false }).compile(schema);
      const fixtures = resolve(subsets, `🕸️mesh/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-${entity}`);
      const scenarios = ["🩹️changes", "📐️coordinates"];

      it(`${dimension} ${entity} preserves independent edits in either arrival order`, () => {
        const collection = entity === "node" ? "nodes" : "sections";
        const before = read(resolve(fixtures, "🩹️changes/📸️snapshot/⬅️before/🔣️.json"))[collection][0];
        const events = scenarios.map(scenario => read(resolve(fixtures, scenario, "🦠️mutation/🔣️.json")));
        const expected = Object.assign({}, before, ...events.map(event => event.patch));
        for (const sequence of [events, [...events].reverse()]) {
          let actual = { ...before };
          let oracle = { ...before };
          for (const event of sequence) {
            expect(validate(event), JSON.stringify(validate.errors)).toBe(true);
            const patch = parsePatch(capture(event.patch));
            actual = { ...actual, ...Object.fromEntries(Object.entries(patch).map(([key, value]) => [key, typeof value === "string" ? value : binary64Value(value as Binary64)])) };
            oracle = applyPatch(oracle, Object.entries(event.patch).map(([key, value]) => ({ op: "replace" as const, path: `/${key}`, value })), true, false).newDocument;
            expect(actual).toEqual(oracle);
          }
          expect(actual).toEqual(expected);
        }
      });

      it(`${dimension} ${entity} rejects unknown, identity, null and invalid scalar fields`, () => {
        const field = entity === "node" ? "x" : "area";
        const invalid = [{ id: "renamed" }, { unknown: 1 }, { [field]: null }, { [field]: Infinity }, ...(entity === "section" ? [{ area: 0 }, { iy: -1 }] : [])];
        for (const patch of invalid) {
          expect(validate({ mutation: `change${title}`, id: "target", patch })).toBe(false);
          expect(() => parsePatch(capture(patch))).toThrow();
        }
        expect(parsePatch({})).toEqual({});
        expect(validate({ mutation: `change${title}`, id: "target", patch: {} })).toBe(true);
      });

      it(`${dimension} ${entity} resolves typed GraphQL and sparse protobuf fields`, () => {
        const source = resolve(subsets, "🌐️any/🧬️schema");
        const graph = ["📸️snapshot", "🔺️diff", "🧬️mutations"].map(part => readFileSync(resolve(source, part, "🔗️.graphql"), "utf8")).join("\n");
        const schema = buildASTSchema(parse(`enum StateClass { ARTIFACT CONFIG TRANSIENT PRESENCE } directive @state(class: StateClass!) on FIELD_DEFINITION\n${graph}`));
        expect(schema.getType(`Fem${dimension}${title}Patch`)).toBeDefined();
        const root = new Root().loadSync(resolve(source, "🧬️mutations/🛰️.proto")).resolveAll();
        const type = root.lookupType(`semio.s.fem.fem${dimension}.mutation.Fem${dimension}Change${title}`);
        const patch = entity === "node" ? { x: 4 } : { name: "Structural Section" };
        const wire = { id: "target", patch };
        expect(type.verify(wire)).toBeNull();
        expect(type.toObject(type.decode(type.encode(type.create(wire)).finish()))).toEqual(wire);
      });
    }
  }
});
