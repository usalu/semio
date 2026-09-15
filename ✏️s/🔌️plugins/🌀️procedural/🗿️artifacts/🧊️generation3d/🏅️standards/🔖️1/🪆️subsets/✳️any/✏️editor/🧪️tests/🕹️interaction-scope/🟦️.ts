import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { resolveUiDirtyScope, uiDirtyScopeWantsCatalogue, uiDirtyScopeWantsPanelBody, uiDirtyScopeWantsSection, uiDirtyScopeWantsWindowBody, type UiDirtyScope } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";

type InteractionScopeRow = { readonly verb: string; readonly windowBodies: readonly string[]; readonly panelBodies: readonly string[]; readonly measures: boolean };
type InteractionScopeRole = {
  readonly id: string;
  readonly declaredWindowBodies: readonly string[];
  readonly declaredPanelBodies: readonly string[];
  readonly publishesHover: readonly string[];
  readonly publishesSelection: readonly string[];
  readonly quiet: readonly string[];
  readonly verbs: readonly InteractionScopeRow[];
};
type InteractionScopeFixture = { readonly format: string; readonly version: number; readonly domain: string; readonly undeclaredDomain: string; readonly roles: readonly InteractionScopeRole[] };

const SELECTION_VERBS = ["interactionSelect", "clearSelection", "selectAll"];
const RESERVED_VERBS = ["interactionSelect", "interactionHover", "clearSelection", "selectAll", "setSelectionMode", "setInteractionGranularity"];

/** 🕹️ Independent re-derivation of the Rust `interaction_declared_refresh_scope`, written from the
 * fixture's own vocabulary rather than from the implementation under test: the windows that declare
 * the touched domain, every declared panel body for the verbs that MOVE a selection, and the Select
 * chrome for every verb but the pointer-transient hover. */
function deriveScope(role: InteractionScopeRole, verb: string, graphWindowBodies: readonly string[]): UiDirtyScope {
  return {
    kind: "partial",
    windowBodies: [...graphWindowBodies],
    panelBodies: SELECTION_VERBS.includes(verb) ? [...role.declaredPanelBodies] : [],
    utilities: false,
    tools: false,
    engagements: false,
    measures: verb !== "interactionHover",
    labels: false,
  };
}

/** 🐢️ What ONE batched `refresh-ui` pass would fetch for this scope — the same four predicates
 * `buildUiRefreshRequest` (`🛠️ShellHelpers/🟦️.tsx`) filters its request with, so the sections counted
 * here are the sections the host actually asks the guest to render. */
function refreshPass(scope: UiDirtyScope, role: InteractionScopeRole): { readonly windows: readonly string[]; readonly panels: readonly string[]; readonly sections: readonly string[]; readonly catalogue: boolean } {
  const resolved = resolveUiDirtyScope(scope);
  return {
    windows: role.declaredWindowBodies.filter((body) => uiDirtyScopeWantsWindowBody(resolved, body)),
    panels: role.declaredPanelBodies.filter((body) => uiDirtyScopeWantsPanelBody(resolved, body)),
    sections: (["utilities", "tools", "engagements", "measures", "labels"] as const).filter((section) => uiDirtyScopeWantsSection(resolved, section)),
    catalogue: uiDirtyScopeWantsCatalogue(resolved),
  };
}

/** ⚖️ Third-party twin of the interaction-scope fixture: the table is re-derived here from the app's
 * declared surfaces alone, then read back through the KERNEL predicates the host filters a
 * `refresh-ui` request with — so both halves of the round trip (what the guest publishes, what the
 * host then fetches) are held against the same table the Rust law scores
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function testGeneration3dInteractionScopeContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🕹️interaction-scope.json`, "utf8")) as InteractionScopeFixture;
  assert.equal(fixture.format, "semio.generation3d.interaction-scope");
  assert.equal(fixture.version, 1);
  assert.deepEqual(
    fixture.roles.map((role) => role.id),
    ["editor", "viewer"],
  );

  for (const role of fixture.roles) {
    assert.deepEqual([...role.verbs.map((row) => row.verb)].sort(), [...RESERVED_VERBS].sort(), `role ${role.id} must score every reserved verb`);
    const graphWindowBodies = role.verbs.find((row) => row.verb === "interactionHover")!.windowBodies;
    assert.ok(
      graphWindowBodies.every((body) => role.declaredWindowBodies.includes(body)),
      `role ${role.id} hover names a window this app does not declare`,
    );

    for (const row of role.verbs) {
      assert.deepEqual(deriveScope(role, row.verb, graphWindowBodies), { kind: "partial", windowBodies: row.windowBodies, panelBodies: row.panelBodies, utilities: false, tools: false, engagements: false, measures: row.measures, labels: false }, `role ${role.id} verb ${row.verb} derivation`);

      const pass = refreshPass({ kind: "partial", windowBodies: row.windowBodies, panelBodies: row.panelBodies, measures: row.measures }, role);
      assert.deepEqual(pass.windows, row.windowBodies, `role ${role.id} verb ${row.verb} fetched windows`);
      assert.deepEqual(pass.panels, row.panelBodies, `role ${role.id} verb ${row.verb} fetched panels`);
      assert.deepEqual(pass.sections, row.measures ? ["measures"] : [], `role ${role.id} verb ${row.verb} fetched sections`);
      assert.equal(pass.catalogue, false, `role ${role.id} verb ${row.verb} must never re-fetch the app-static catalogue`);

      const owed = row.verb === "interactionHover" || !SELECTION_VERBS.includes(row.verb) ? role.publishesHover : role.publishesSelection;
      for (const body of owed) assert.ok(pass.windows.includes(body) || pass.panels.includes(body), `role ${role.id} verb ${row.verb} dropped ${body}, which publishes its lane`);
      if (row.verb === "interactionHover") for (const body of role.quiet) assert.ok(!pass.windows.includes(body) && !pass.panels.includes(body), `role ${role.id} hover must not reach ${body}`);
    }

    /** 🌀️ The counter-model: the answer this replaces asked for every body of every section, plus the
     * app-static catalogue, on every pointer move. */
    const full = refreshPass({ kind: "full" }, role);
    assert.deepEqual(full.windows, role.declaredWindowBodies);
    assert.deepEqual(full.panels, role.declaredPanelBodies);
    assert.deepEqual(full.sections, ["utilities", "tools", "engagements", "measures", "labels"]);
    assert.equal(full.catalogue, true);

    const hover = refreshPass({ kind: "partial", windowBodies: graphWindowBodies, panelBodies: [], measures: false }, role);
    assert.ok(hover.windows.length + hover.panels.length + hover.sections.length < full.windows.length + full.panels.length + full.sections.length, `role ${role.id} hover must cost strictly less than the full pass it replaces`);
    console.log(`[SCOPE] ${role.id} hover ${hover.windows.length + hover.panels.length + hover.sections.length} sections vs full ${full.windows.length + full.panels.length + full.sections.length} + catalogue`);
  }
}
