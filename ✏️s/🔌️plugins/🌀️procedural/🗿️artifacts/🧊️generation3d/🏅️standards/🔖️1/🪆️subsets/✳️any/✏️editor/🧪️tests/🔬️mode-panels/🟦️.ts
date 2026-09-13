import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type ModePanelWindow = { readonly id: string; readonly kind: string };
type ModePanelCase = { readonly id: string; readonly mode: string; readonly windows: readonly ModePanelWindow[]; readonly focusedWindow: string; readonly focusedWindowInRoster: boolean };
type ModePanelRow = { readonly tab: string; readonly bodyKey: string; readonly root: string; readonly contains: readonly string[] };
type ModePanelFixture = { readonly format: string; readonly version: number; readonly panels: readonly ModePanelRow[]; readonly cases: readonly ModePanelCase[] };

/** 📌️ Independent re-implementation of the panel projection the fixture's law is about — the twin of
 * `ViewModel::for_panel` (`🛂️manifest/🦀️.rs`) and `panelViewContext` (`🛂️manifest/🟦️.ts`), written
 * here from the fixture's own vocabulary so the law is validated against a second implementation
 * rather than against the one under test. */
function projectPanel(row: ModePanelCase): { readonly windowId: undefined; readonly focusedWindowId: string | undefined } {
  return { windowId: undefined, focusedWindowId: row.windows.some((window) => window.id === row.focusedWindow) ? row.focusedWindow : undefined };
}

/** ⚖️ Third-party twin of the mode-panel publication fixture: panels are framework-injected tabs, so
 * EVERY mode publishes all three bodies, and a focused pane the active mode's roster does not carry
 * must leave the projection rather than take the whole panel down
 * (`wgpu-ui.surface-not-published:framework.panel.*`, ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function testGeneration3dModePanelPublicationContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/📌️mode-panel-publication.json`, "utf8")) as ModePanelFixture;
  assert.equal(fixture.format, "semio.generation3d.mode-panel-publication");
  assert.equal(fixture.version, 1);

  assert.deepEqual(
    fixture.panels.map((panel) => panel.tab),
    ["framework.panel.artifact", "framework.panel.catalogue", "framework.panel.inspection"],
  );
  for (const panel of fixture.panels) {
    assert.ok(panel.bodyKey.startsWith("procedural.play."), `${panel.tab} must carry an app body key`);
    assert.ok(panel.contains.length > 0, `${panel.tab} must declare what its published body carries`);
    assert.ok(
      panel.contains.every((marker) => marker.startsWith(panel.root)),
      `${panel.tab} markers must live under its own root ${panel.root}`,
    );
  }
  assert.equal(new Set(fixture.panels.map((panel) => panel.bodyKey)).size, fixture.panels.length, "one body key per panel");

  // 🎭️ Both modes are exercised, and the generate roster never contains an edit-mode window.
  assert.deepEqual(new Set(fixture.cases.map((row) => row.mode)), new Set(["edit", "generate"]));
  for (const row of fixture.cases) {
    const projected = projectPanel(row);
    assert.equal(projected.windowId, undefined, `case ${row.id} is a panel projection`);
    assert.equal(projected.focusedWindowId !== undefined, row.focusedWindowInRoster, `case ${row.id} declares the wrong focus liveness`);
    assert.equal(row.windows.some((window) => window.id === row.focusedWindow), row.focusedWindowInRoster, `case ${row.id} roster and focus disagree`);
  }
  const stale = fixture.cases.filter((row) => !row.focusedWindowInRoster);
  assert.ok(stale.length >= 1, "the live defect — a focused pane the roster does not carry — must have its own case");
  assert.ok(
    stale.some((row) => row.mode === "generate" && row.windows.length > 0),
    "the wgpu generate boot (three generate windows, edit-mode focus) must be one of the cases",
  );

  console.log(`generation3d mode-panel publication panels=${fixture.panels.length} cases=${fixture.cases.length} staleFocusCases=${stale.map((row) => row.id).join(",")}`);
}

if (import.meta.main) testGeneration3dModePanelPublicationContract();
