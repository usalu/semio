import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { parseGeneration3dPreviewWindowTransient } from "../../🧬️schema/🟦️.ts";

/** 🧪️ Compares the production parser with Ajv for the exact window-transient/config boundary. */
export function testGeneration3dPreviewWindowTransientContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/🔬️unit/🔣️.json`, "utf8")) as {
    accepted: unknown[];
    rejected: unknown[];
    configWithMisclassifiedField: unknown;
  };
  const transientSchema = JSON.parse(readFileSync(`${here}/../../🧬️schema/🔣️.json`, "utf8"));
  const configSchema = JSON.parse(readFileSync(`${here}/../../../../../../../🎚️config/🧬️schema/🔣️.json`, "utf8"));
  const artifactSchema = JSON.parse(readFileSync(`${here}/../../../../../../../../🧬️schema/🔣️.json`, "utf8"));
  const ajv = new Ajv({ strict: false, allErrors: true, formats: { double: true } });
  ajv.addSchema(artifactSchema);
  const validateTransient = ajv.compile(transientSchema);
  const validateConfig = ajv.compile(configSchema);
  const productionAccepts = (value: unknown): boolean => {
    try { parseGeneration3dPreviewWindowTransient(value); return true; } catch { return false; }
  };
  for (const value of fixture.accepted) {
    assert.equal(productionAccepts(value), true);
    assert.equal(validateTransient(value), true, JSON.stringify(validateTransient.errors));
  }
  for (const value of fixture.rejected) {
    assert.equal(productionAccepts(value), false);
    assert.equal(validateTransient(value), false);
  }
  assert.equal(validateConfig(fixture.configWithMisclassifiedField), false, "app config must reject computed preview output");
  assert.equal(transientSchema.properties.previewEvalText["x-semio-state"], "ephemeral-local-window");
  console.log(`generation3d-preview-window-transient accepted=${fixture.accepted.length} rejected=${fixture.rejected.length} configExcludesPreview=true owner=procedural-preview`);
}

if (import.meta.main) testGeneration3dPreviewWindowTransientContract();
