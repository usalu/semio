/** 🧺️ Every native mutation leaf remains reachable through its strict schema aggregate. */
import { describe, expect, test } from "bun:test";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { semioSchemaAjvV1 } from "@semio-tech/framework/schema/oracles/validator";
import vectors from "../🧫️fixtures/🧬️mutation-aggregate/🔣️.json" with { type: "json" };

const read = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
const branchOracle = semioSchemaAjvV1({ strict: true, allErrors: true }).compile(vectors.externalBranch);
const schemaOracle = semioSchemaAjvV1({ strict: true, allErrors: true });

function requiredProperties(node: any, path: string): void {
  if (node === null || typeof node !== "object") return;
  if (node.additionalProperties === false && node.patternProperties === undefined) {
    for (const field of node.required ?? []) expect(Object.hasOwn(node.properties ?? {}, field), `${path}/required/${field}`).toBe(true);
  }
  for (const [field, value] of Object.entries(node)) if (typeof value === "object") requiredProperties(value, `${path}/${field}`);
}

describe("norm mutation aggregates", () => {
  for (const artifact of vectors.artifacts) {
    test(`${artifact} facets have satisfiable required properties`, () => {
      const root = join(import.meta.dir, "../../🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
      for (const facet of vectors.facets) {
        const schema = read(join(root, facet));
        expect(schemaOracle.validateSchema(schema), `${artifact}/${facet}: ${JSON.stringify(schemaOracle.errors)}`).toBe(true);
        requiredProperties(schema, `${artifact}/${facet}`);
      }
    });
    test(`${artifact} exposes every leaf exactly once`, () => {
      const root = join(import.meta.dir, "../../🗿️artifacts", artifact, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");
      const leaves = readdirSync(root, { withFileTypes: true }).filter(entry => entry.isDirectory()).flatMap(entry => {
        try { return [read(join(root, entry.name, "🧬️schema/🔣️.json"))]; }
        catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return []; throw error; }
      });
      const aggregate = read(join(root, "🔣️.json"));
      const refs = aggregate.oneOf.map((branch: any) => {
        if (vectors.internallyTagged.includes(artifact)) {
          expect(typeof branch.$ref).toBe("string");
          const leaf = leaves.find(leaf => leaf.$id === branch.$ref);
          expect(typeof leaf?.properties?.mutation?.const).toBe("string");
          expect(leaf?.required).toContain("mutation");
          return branch.$ref;
        }
        expect(branchOracle(branch), JSON.stringify(branchOracle.errors)).toBe(true);
        expect(branch.required).toEqual(Object.keys(branch.properties));
        const [name, payload] = Object.entries(branch.properties)[0] as [string, { $ref: string }];
        expect(leaves.find(leaf => leaf.$id === payload.$ref)?.title).toBe(name);
        return payload.$ref;
      });
      expect(refs.length).toBeGreaterThan(0);
      expect(new Set(refs).size).toBe(refs.length);
      expect([...refs].sort()).toEqual(leaves.map(leaf => leaf.$id).sort());
    });
  }
});
