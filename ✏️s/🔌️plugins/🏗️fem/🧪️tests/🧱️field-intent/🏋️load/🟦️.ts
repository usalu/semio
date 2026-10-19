import { readFileSync, readdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import { Root } from "protobufjs";
import { buildSchema } from "graphql";
import { binary64, binary64Value, type Binary64 } from "../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import { parseFem2dLoadPatch, parseFem2dCombinationPatch } from "../../../🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";
import { parseFem3dLoadPatch, parseFem3dCombinationPatch } from "../../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🔺️diff/🟦️.ts";

const owner = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const capture = (value: unknown): unknown => typeof value === "number" ? binary64(value) : Array.isArray(value) ? value.map(capture) : value && typeof value === "object" ? Object.fromEntries(Object.entries(value).map(([key, item]) => [key, capture(item)])) : value;
const query = (value: unknown): unknown => Array.isArray(value) ? value.map(query) : value && typeof value === "object" ? "bits" in value ? binary64Value(value as Binary64) : Object.fromEntries(Object.entries(value).map(([key, item]) => [key, query(item)])) : value;

describe("load and combination field intent", () => {
  for (const dimension of ["2d", "3d"] as const) {
    const subsets = resolve(owner, `🗿️artifacts/${dimension === "2d" ? "◻️2d" : "🧊️3d"}/🏅️standards/🔖️1/🪆️subsets`);
    const fixtures = resolve(subsets, "🏋️load/🧫️fixtures/🧬️mutations");
    const parseLoad = dimension === "2d" ? parseFem2dLoadPatch : parseFem3dLoadPatch;
    const parseCombination = dimension === "2d" ? parseFem2dCombinationPatch : parseFem3dCombinationPatch;
    const ajv = new Ajv({ strict: false, strictNumbers: true, validateFormats: false });
    const validators = Object.fromEntries(["load", "combination-name", "combination-term"].map(kind => [kind, ajv.compile(read(resolve(subsets, `🏋️load/🧬️schema/🧬️mutations/🩹️change-${kind}/🧬️schema/🔣️.json`)))]));

    it(`${dimension} validates all fifteen language-neutral vectors`, () => {
      for (const kind of Object.keys(validators)) for (const scenario of readdirSync(resolve(fixtures, `🩹️change-${kind}`))) {
        const event = read(resolve(fixtures, `🩹️change-${kind}`, scenario, "🦠️mutation/🔣️.json"));
        expect(validators[kind](event), JSON.stringify(validators[kind].errors)).toBe(true);
      }
    });

    it(`${dimension} independently preserves scalar and target edits in either order`, () => {
      const before = read(resolve(fixtures, "🩹️change-load/🩹️changes/📸️snapshot/⬅️before/🔣️.json")).loadCases[0].loads[0];
      const events = ["🩹️changes", "📍️target"].map(scenario => read(resolve(fixtures, "🩹️change-load", scenario, "🦠️mutation/🔣️.json")));
      for (const sequence of [events, [...events].reverse()]) {
        let actual = { ...before }, oracle = { ...before };
        for (const event of sequence) {
          const patch = query(parseLoad(capture(event.patch))) as Record<string, unknown>;
          expect(patch.kind).toBe(actual.kind);
          actual = { ...actual, ...patch };
          oracle = applyPatch(oracle, Object.entries(event.patch).filter(([key]) => key !== "kind").map(([key, value]) => ({ op: "replace" as const, path: `/${key}`, value })), true, false).newDocument;
          expect(actual).toEqual(oracle);
        }
        expect(actual.nodeId).toBe("n2"); expect(actual.value).toBe(20); expect(actual.dof).toBe("Tx");
      }
    });

    it(`${dimension} rejects missing tags, null scalars, identity and foreign variant fields`, () => {
      for (const patch of [{ value: 1 }, { kind: "nodal", id: "x" }, { kind: "nodal", value: null }, { kind: "nodal", value: Infinity }, { kind: "nodal", wx: 1 }, { kind: "area", nodeId: "x" }]) {
        expect(validators.load({ mutation: "changeLoad", caseId: "lc1", id: "l1", patch })).toBe(false);
        expect(() => parseLoad(capture(patch))).toThrow();
      }
      expect(() => parseCombination({ name: null })).toThrow();
      for (const dof of ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]) {
        const patch = { kind: "nodal", dof };
        expect(validators.load({ mutation: "changeLoad", caseId: "lc1", id: "l1", patch })).toBe(true);
        expect(query(parseLoad(patch))).toEqual(patch);
      }
      expect(() => parseCombination({ id: "c1" })).toThrow();
      const term = { mutation: "changeCombinationTerm", id: "c1", caseId: "lc1", factor: null };
      expect(validators["combination-term"](term)).toBe(true);
      expect(validators["combination-term"]({ mutation: term.mutation, id: term.id, caseId: term.caseId })).toBe(false);
      if (dimension === "2d") expect(validators["combination-term"]({ ...term, index: null })).toBe(false);
    });

    it(`${dimension} preserves sparse nested term and explicit null codec presence`, () => {
      const diff = read(resolve(fixtures, "🩹️change-combination-term/⚖️weights/🔺️diff/🔣️.json"));
      expect(query(parseCombination(capture(diff.combinations.modified[0].patch)))).toEqual(diff.combinations.modified[0].patch);
      const root = new Root().loadSync(resolve(subsets, "🌐️any/🧬️schema/🧬️mutations/🛰️.proto")).resolveAll();
      const type = root.lookupType(`semio.s.fem.fem${dimension}.mutation.Fem${dimension}ChangeCombinationTerm`);
      const removed = type.decode(type.encode(type.create({ id: "c1", caseId: "lc1", factor: { null: {} } })).finish()) as unknown as { factor: object };
      expect(Object.hasOwn(removed.factor, "null")).toBe(true);
      expect(Object.hasOwn(removed.factor, "value")).toBe(false);
      const omitted = type.decode(type.encode(type.create({ id: "c1", caseId: "lc1" })).finish());
      expect(Object.hasOwn(omitted, "factor")).toBe(false);
      const schema = ["📸️snapshot", "🔺️diff", "🧬️mutations"].map(part => readFileSync(resolve(subsets, `🌐️any/🧬️schema/${part}/🔗️.graphql`), "utf8")).join("\n");
      expect(buildSchema(`enum StateClass { ARTIFACT CONFIG TRANSIENT PRESENCE } directive @state(class: StateClass!) on FIELD_DEFINITION\n${schema}\ntype Query { mutation: Fem${dimension}Mutation }`).getType(`Fem${dimension}ChangeCombinationTerm`)).toBeDefined();
    });
  }
});
