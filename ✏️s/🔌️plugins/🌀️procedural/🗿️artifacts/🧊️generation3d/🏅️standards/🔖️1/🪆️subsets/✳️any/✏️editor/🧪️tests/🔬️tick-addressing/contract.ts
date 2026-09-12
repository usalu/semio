import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** ⚖️ Third-party twin of the tick-addressing fixture: `JSON.parse` must see generate preview AND the
 * read-only viewer preview on the same chain as edit preview — one addressing law, three windows,
 * two surfaces. */
export function testGeneration3dPreviewTickAddressingContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🪟️tick-addressing.json`, "utf8")) as {
    format: string;
    windowKinds: Record<string, string>;
    arming: Array<{ id: string; surface: string; armedWindowIds: string[] }>;
    dispatch: Array<{ id: string; surface: string; payloadWindowId: string; payloadWindowKind?: string; admitted: boolean }>;
  };
  assert.equal(fixture.format, "semio.generation3d.tick-addressing");
  assert.equal(fixture.windowKinds.generatePreview, "generation3d-generate-preview");
  assert.equal(fixture.windowKinds.viewPreview, "procedural-view-preview");

  const generateArmed = fixture.arming.find((row) => row.id === "generate-preview-attached");
  assert.equal(generateArmed?.surface, "editor");
  assert.deepEqual(generateArmed?.armedWindowIds, ["generation3d-generate-preview"]);
  const generateAdmitted = fixture.dispatch.find((row) => row.id === "generate-preview-addressed-from-generations");
  assert.equal(generateAdmitted?.payloadWindowId, "generation3d-generate-preview");
  assert.equal(generateAdmitted?.payloadWindowKind, "generatePreview");
  assert.equal(generateAdmitted?.admitted, true);

  // 👁️ The read-only surface is part of the SAME law, not an exception to it.
  const viewerRows = fixture.arming.filter((row) => row.surface === "viewer");
  assert.ok(viewerRows.length >= 2, "the viewer surface must declare its own arming rows");
  assert.deepEqual(viewerRows.find((row) => row.id === "viewer-no-surface-mounted")?.armedWindowIds, []);
  assert.deepEqual(viewerRows.find((row) => row.id === "viewer-preview-attached")?.armedWindowIds, ["view-preview"]);
  const viewerDispatch = fixture.dispatch.filter((row) => row.surface === "viewer");
  assert.equal(viewerDispatch.find((row) => row.id === "viewer-preview-addressed")?.admitted, true);
  assert.equal(viewerDispatch.find((row) => row.id === "viewer-unaddressed")?.admitted, false);
  assert.equal(viewerDispatch.find((row) => row.id === "viewer-detached-window-addressed")?.admitted, false);
  for (const row of viewerDispatch) assert.equal(row.payloadWindowKind, "viewPreview");

  console.log(
    `generation3d tick-addressing generatePreview=${fixture.windowKinds.generatePreview} viewPreview=${fixture.windowKinds.viewPreview} ` +
      `editorRows=${fixture.arming.filter((row) => row.surface === "editor").length}/${fixture.dispatch.filter((row) => row.surface === "editor").length} ` +
      `viewerRows=${viewerRows.length}/${viewerDispatch.length}`,
  );
}

if (import.meta.main) testGeneration3dPreviewTickAddressingContract();
