import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** ⚖️ Third-party twin of the tick-addressing fixture: JSON.parse must see generate preview on the same chain as edit preview. */
export function testGeneration3dGeneratePreviewTickAddressingContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../🧫️fixtures/🪟️tick-addressing.json`, "utf8")) as {
    format: string;
    windowKinds: Record<string, string>;
    arming: Array<{ id: string; armedWindowIds: string[] }>;
    dispatch: Array<{ id: string; payloadWindowId: string; payloadWindowKind?: string; admitted: boolean }>;
  };
  assert.equal(fixture.format, "semio.generation3d.tick-addressing");
  assert.equal(fixture.windowKinds.generatePreview, "generation3d-generate-preview");
  const armed = fixture.arming.find((row) => row.id === "generate-preview-attached");
  assert.deepEqual(armed?.armedWindowIds, ["generation3d-generate-preview"]);
  const admitted = fixture.dispatch.find((row) => row.id === "generate-preview-addressed-from-generations");
  assert.equal(admitted?.payloadWindowId, "generation3d-generate-preview");
  assert.equal(admitted?.payloadWindowKind, "generatePreview");
  assert.equal(admitted?.admitted, true);
  console.log(`generation3d-generate-preview tick-addressing generatePreview=${fixture.windowKinds.generatePreview} armed=${armed?.armedWindowIds.join(",")} admitted=${admitted?.admitted}`);
}

if (import.meta.main) testGeneration3dGeneratePreviewTickAddressingContract();
