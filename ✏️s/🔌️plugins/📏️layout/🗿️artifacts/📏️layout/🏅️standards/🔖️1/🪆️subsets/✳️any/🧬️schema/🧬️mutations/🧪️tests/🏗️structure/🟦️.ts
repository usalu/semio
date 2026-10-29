/** 🧪️ Independent RFC 6902 and schema checks for the shared structural corpus. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { applyPatch, type Operation } from "fast-json-patch";
import vectors from "../../../../🧪️tests/🧫️fixtures/🧬️structure/🔣️.json";
import artifactSchema from "../../../🔣️.json";
import { layoutArtifactFromNativeJson } from "../../../../🚪️io/📝️text/📸️snapshot/🪪️native-json/🟦️.ts";

test("structural mutations share exact independent JSON Patch witnesses", () => {
  for (const vector of vectors.cases) {
    const input = JSON.stringify(vector.before);
    expect(applyPatch(structuredClone(vector.before), vector.patch as Operation[], true, false).newDocument).toEqual(vector.after);
    expect(JSON.stringify(vector.before)).toBe(input);
    expect(() => layoutArtifactFromNativeJson(vector.before)).not.toThrow();
    expect(() => layoutArtifactFromNativeJson(vector.after)).not.toThrow();
  }
});

test("structural payloads match their first-party JSON Schemas", () => {
  const ajv = new Ajv({ strict: false, allErrors: true });
  ajv.addSchema({ $id: artifactSchema.$id, $defs: artifactSchema.$defs });
  const folders = ["📝️create-paragraph-style","🗑️delete-paragraph-style","📖️create-parent-page","🗑️delete-parent-page","📓️create-spread","🗑️delete-spread","📓️set-page-spread","🪜️reorder-layer","📖️set-parent-page-content"];
  for (const folder of folders) {
    const schema = JSON.parse(readFileSync(new URL("../../" + folder + "/🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = ajv.compile(schema);
    for (const vector of vectors.cases.filter((row) => Object.hasOwn(row.mutation, schema.title))) {
      const payload = (vector.mutation as Record<string, object | undefined>)[schema.title]!;
      expect(validate(payload)).toBe(true);
      expect(validate({ ...payload, unexpected: true })).toBe(false);
    }
  }
});
