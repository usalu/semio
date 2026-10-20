import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { parse, buildASTSchema } from "graphql";
import { Root } from "protobufjs";
import { binary64, binary64Value, type Binary64 } from "@semio-tech/framework/value/ieee754";

const owner = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const fixture = JSON.parse(readFileSync(resolve(owner, "🧪️tests/🧫️fixtures/🧱️field-intent/🔣️.json"), "utf8")) as { cases: { dimension: "2d" | "3d"; before: Record<string, unknown>; events: { mutation: string; id: string; patch: Record<string, unknown> }[]; after: Record<string, unknown> }[]; refusedPatches: unknown[] };
const capturePatch = (patch: unknown): unknown => patch && typeof patch === "object" ? Object.fromEntries(Object.entries(patch).map(([key, value]) => [key, typeof value === "number" ? binary64(value) : value])) : patch;

/** 🧱️ Registers the material field-intent laws of one dimension against that artifact's own diff parsers. */
export function registerMaterialFieldIntent(dimension: "2d" | "3d", parsers: { parsePatch(patch: any): unknown; parseDelta(delta: any): { modified: readonly { patch: object }[] } }): void {
describe(`${dimension} material field intent mirrors`, () => {
  for (const testCase of fixture.cases.filter(row => row.dimension === dimension)) {
    const schemaOwner = resolve(owner, `🗿️artifacts/${testCase.dimension === "2d" ? "◻️2d" : "🧊️3d"}/🏅️standards/🔖️1/🪆️subsets`);
    const schema = JSON.parse(readFileSync(resolve(schemaOwner, "🧱️material/🧬️schema/🧬️mutations/🩹️change-material/🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: false, allErrors: true, validateFormats: false }).compile(schema);
    const { parsePatch, parseDelta } = parsers;

    it(`${testCase.dimension} preserves both independent field edits in either arrival order`, () => {
      for (const events of [testCase.events, [...testCase.events].reverse()]) {
        let actual = { ...testCase.before };
        let oracle = { ...testCase.before };
        for (const event of events) {
          expect(validate(event), JSON.stringify(validate.errors)).toBe(true);
          const delta = parseDelta({ removed: [], inserted: [], moved: [], modified: [{ id: event.id, patch: capturePatch(event.patch) }] });
          actual = { ...actual, ...Object.fromEntries(Object.entries(delta.modified[0]!.patch).map(([key, value]) => [key, typeof value === "string" ? value : binary64Value(value as Binary64)])) };
          oracle = applyPatch(oracle, Object.entries(event.patch).map(([key, value]) => ({ op: "replace" as const, path: `/${key}`, value })), true, false).newDocument;
          expect(actual).toEqual(oracle);
        }
        expect(actual).toEqual(testCase.after);
      }
    });

    it(`${testCase.dimension} refuses invalid sparse fields exactly as Ajv does`, () => {
      for (const patch of fixture.refusedPatches) {
        expect(validate({ mutation: "changeMaterial", id: "steel", patch })).toBe(false);
        expect(() => parsePatch(capturePatch(patch))).toThrow();
      }
      expect(parsePatch({})).toEqual({});
      expect(validate({ mutation: "changeMaterial", id: "steel", patch: {} })).toBe(true);
    });

    it(`${testCase.dimension} declares a mutation union and resolves a sparse protobuf patch`, () => {
      const mutationOwner = resolve(schemaOwner, "🌐️any/🧬️schema/🧬️mutations");
      const document = parse(readFileSync(resolve(mutationOwner, "🔗️.graphql"), "utf8"));
      const union = document.definitions.find(definition => definition.kind === "UnionTypeDefinition" && definition.name.value === `Fem${testCase.dimension}Mutation`);
      expect(union?.kind === "UnionTypeDefinition" && union.types?.some(type => type.name.value === `Fem${testCase.dimension}ChangeMaterial`)).toBe(true);
      const snapshot = readFileSync(resolve(mutationOwner, "../📸️snapshot/🔗️.graphql"), "utf8");
      const diff = readFileSync(resolve(mutationOwner, "../🔺️diff/🔗️.graphql"), "utf8");
      const schema = buildASTSchema(parse(`enum StateClass { ARTIFACT CONFIG TRANSIENT PRESENCE } directive @state(class: StateClass!) on FIELD_DEFINITION\n${snapshot}\n${diff}\n${readFileSync(resolve(mutationOwner, "🔗️.graphql"), "utf8")}`));
      expect(schema.getType(`Fem${testCase.dimension}MaterialPatch`)).toBeDefined();
      const protobuf = new Root().loadSync(resolve(mutationOwner, "🛰️.proto")).resolveAll();
      const type = protobuf.lookupType(`semio.s.fem.fem${testCase.dimension}.mutation.Fem${testCase.dimension}ChangeMaterial`);
      const wire = { id: "steel", patch: { name: "Structural Steel" } };
      expect(type.verify(wire)).toBeNull();
      expect(type.toObject(type.decode(type.encode(type.create(wire)).finish()))).toEqual(wire);
    });
  }
});
}
