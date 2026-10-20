import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import { Root } from "protobufjs";

const owner = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const read = (path: string) => JSON.parse(readFileSync(path, "utf8"));

/** 🛡️ Registers the support and snapshot-schema laws of one dimension against that artifact's own parsers. */
export function registerBoundaryFieldIntent(dimension: "2d" | "3d", parsers: { parseSupportPatch(patch: any): unknown; parseSupportRow(row: any): { fixed: unknown } }): void {
describe(`${dimension} support membership and concrete snapshot schemas`, () => {
  {
    const subsets = resolve(owner, `🗿️artifacts/${dimension === "2d" ? "◻️2d" : "🧊️3d"}/🏅️standards/🔖️1/🪆️subsets`);
    const ajv = new Ajv({ strict: false, strictNumbers: true, validateFormats: false });
    const support = ajv.compile(read(resolve(subsets, "🛡️boundary/🧬️schema/🧬️mutations/🩹️change-support/🧬️schema/🔣️.json")));
    const snapshot = ajv.compile(read(resolve(subsets, "🌐️any/🧬️schema/📸️snapshot/🔣️.json")));
    const parsePatch = parsers.parseSupportPatch;
    const parseRow = parsers.parseSupportRow;

    it(`${dimension} admits canonical fixed sets and rejects permutations or duplicates`, () => {
      for (const fixed of [[], ["Tx", "Ty", "Rz"], ["Ty", "Rz"]]) {
        expect(support({ mutation: "changeSupport", id: "s1", patch: { fixed } })).toBe(true);
        expect(parsePatch({ fixed })).toEqual({ fixed });
        expect(parseRow({ id: "s1", nodeId: "n1", fixed }).fixed).toEqual(fixed);
      }
      for (const fixed of [["Ty", "Tx"], ["Tx", "Tx"]]) {
        expect(support({ mutation: "changeSupport", id: "s1", patch: { fixed } })).toBe(false);
        expect(() => parsePatch({ fixed })).toThrow();
        expect(() => parseRow({ id: "s1", nodeId: "n1", fixed })).toThrow();
      }
      for (const patch of [{ fixed: [], tx: true }, { tx: null }, { tx: "true" }, { id: "renamed" }]) {
        expect(support({ mutation: "changeSupport", id: "s1", patch })).toBe(false);
        expect(() => parsePatch(patch)).toThrow();
      }
    });

    it(`${dimension} snapshot validates concrete domain records`, () => {
      const valid = read(resolve(subsets, "🧱️material/🧪️tests/🧫️fixtures/🧬️mutations/🩹️change-material/🩹️renames/📸️snapshot/⬅️before/🔣️.json"));
      expect(snapshot(valid), JSON.stringify(snapshot.errors)).toBe(true);
      for (const materials of [[{}], [{ ...valid.materials[0], e: 0 }], [{ ...valid.materials[0], id: null }]]) {
        expect(snapshot({ ...valid, materials })).toBe(false);
      }
      expect(snapshot({ ...valid, nodes: [{ id: "n1", x: 0 }] })).toBe(false);
      expect(snapshot({ ...valid, supports: [{ id: "s1", nodeId: "n1", fixed: ["Ty", "Tx"] }] })).toBe(false);
    });

    it(`${dimension} protobuf preserves an explicit empty fixed set`, () => {
      const root = new Root().loadSync(resolve(subsets, "🌐️any/🧬️schema/🧬️mutations/🛰️.proto")).resolveAll();
      const type = root.lookupType(`semio.s.fem.fem${dimension}.mutation.Fem${dimension}ChangeSupport`);
      const message = type.decode(type.encode(type.create({ id: "s1", patch: { fixed: { values: [] } } })).finish()) as unknown as { patch: { fixed: { values: unknown[] } } };
      expect(Object.hasOwn(message.patch, "fixed")).toBe(true);
      expect(message.patch.fixed.values).toEqual([]);
    });
  }
});
}
