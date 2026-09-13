import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type RowVerb = { readonly verb: string; readonly affordance: string; readonly onlyWhenSelected: boolean };
type InteractionWindow = { readonly id: string; readonly bodyKey: string; readonly surfaceKind: string; readonly actions: readonly string[]; readonly utilities: readonly string[]; readonly rowVerbs: readonly RowVerb[] };
type NodeGraphOperation = { readonly operation: string; readonly dispatchedBy: string; readonly required: readonly string[] };
type CommandTrigger = { readonly command: string; readonly triggers: readonly string[]; readonly keys: string | null };
type RemovedCommand = { readonly command: string; readonly reason: string };
type GenerateInteractionsFixture = {
  readonly format: string;
  readonly version: number;
  readonly windows: readonly InteractionWindow[];
  readonly previewParity: readonly string[];
  readonly gumballVerbs: readonly string[];
  readonly nodeGraphEditOperations: readonly NodeGraphOperation[];
  readonly commandTriggers: readonly CommandTrigger[];
  readonly removedCommands: readonly RemovedCommand[];
  readonly renameCommitArgument: { readonly authored: readonly string[]; readonly fromTrigger: string; readonly guestReads: readonly string[] };
};

/** 🎬️ Independent re-implementation of the framework's intent→args merge (`uiIntentPayload`,
 * `🛠️ShellHelpers/🟦️.tsx`): a node's AUTHORED args with the gesture's own payload merged over them, a
 * scalar payload NAMED by its trigger. Written here from the rule rather than imported, so the
 * fixture's `renameCommitArgument` row is validated against a second implementation. */
function mergeIntentPayload(authored: Record<string, unknown>, trigger: string, input: unknown): Record<string, unknown> {
  if (input === null || input === undefined) return { ...authored };
  const scalar = typeof input === "string" || typeof input === "number" || typeof input === "boolean" || typeof input === "bigint";
  const named = scalar ? { [trigger === "delta" ? "delta" : "value"]: input } : (input as Record<string, unknown>);
  return { ...authored, ...named };
}

/** 🕹️ Independent re-implementation of `World3dHost`'s gumball gate
 * (`gumballVisible = selection.gumballActive && transformGumballMode`), where `gumballActive` is
 * itself `selectedIds.length > 0 && activeUtility !== ""` and the utility has to be a transform one
 * the WINDOW offers. */
function gumballVisible(window: InteractionWindow, selectedIds: readonly string[], activeUtility: string): boolean {
  const transform = window.utilities.includes(activeUtility) && ["move", "rotate", "scale"].includes(activeUtility);
  return selectedIds.length > 0 && activeUtility !== "" && transform;
}

/** ⚖️ Third-party twin of the generate-mode interaction table: the invariants the Rust law
 * (`🦀️.rs` beside this file) asserts against the live `AppDefinition`, re-derived from the table
 * alone (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, gaps #3/#5/#7/#8/#10). */
export function testGeneration3dGenerateModeInteractionContract(): void {
  const here = fileURLToPath(new URL(".", import.meta.url));
  const fixture = JSON.parse(readFileSync(`${here}/../../../🧫️fixtures/🎛️generate-mode-interactions.json`, "utf8")) as GenerateInteractionsFixture;
  assert.equal(fixture.format, "semio.generation3d.generate-mode-interactions");
  assert.equal(fixture.version, 1);

  const byId = new Map(fixture.windows.map((window) => [window.id, window]));
  assert.equal(byId.size, fixture.windows.length, "one row per window kind");

  // 🗂️ The Generations window is the only one with row verbs, and all four generation verbs are there.
  const generations = byId.get("generation3d-generations")!;
  assert.deepEqual(
    [...generations.rowVerbs].map((row) => row.verb).sort(),
    ["addGeneration", "removeGeneration", "renameGeneration", "selectGeneration"],
    "every generation verb needs a row affordance",
  );
  for (const row of generations.rowVerbs) {
    assert.ok(generations.actions.includes(row.verb), `${row.verb} is offered on a row but not declared on the window`);
    assert.ok(["rowActivate", "rowActionButton", "inlineEditorCommit"].includes(row.affordance), `${row.verb} needs a real affordance, not ${row.affordance}`);
  }
  const selectionScoped = generations.rowVerbs.filter((row) => row.onlyWhenSelected).map((row) => row.verb);
  assert.deepEqual(selectionScoped, ["renameGeneration"], "only the inline rename editor is scoped to the selected row");
  assert.equal(generations.rowVerbs.find((row) => row.verb === "renameGeneration")!.affordance, "inlineEditorCommit", "rename must be an editor the user types into, not a fixed-argument button");
  for (const window of fixture.windows) {
    if (window.id === generations.id) continue;
    assert.equal(window.rowVerbs.length, 0, `${window.id} is not a tree and must declare no row verbs`);
  }

  // 📝️ The Form window's single verb is the value edit.
  assert.deepEqual([...byId.get("generation3d-generate-form")!.actions], ["updateGenerationValues"]);

  // 🕹️ Preview parity: both World3d previews offer the same actions, the same rail, and a live gumball.
  assert.ok(fixture.previewParity.length >= 2, "parity needs at least two previews to compare");
  const previews = fixture.previewParity.map((id) => byId.get(id)!);
  for (const preview of previews) {
    assert.equal(preview.surfaceKind, "world3d", `${preview.id} must be a world3d surface`);
    assert.deepEqual([...preview.actions].sort(), [...previews[0].actions].sort(), `${preview.id} diverges from ${previews[0].id}`);
    assert.deepEqual(preview.utilities, previews[0].utilities, `${preview.id} has a different utility rail`);
    for (const verb of fixture.gumballVerbs) assert.ok(preview.actions.includes(verb), `${preview.id} must own ${verb}`);
    assert.equal(gumballVisible(preview, ["extrude@solid#0"], "move"), true, `${preview.id} must show a gumball for a selection under the move utility`);
    assert.equal(gumballVisible(preview, [], "move"), false, `${preview.id} must not show a gumball with nothing selected`);
    assert.equal(gumballVisible(preview, ["extrude@solid#0"], ""), false, `${preview.id} must not show a gumball with no active utility`);
  }
  const flow = byId.get("procedural-main")!;
  assert.equal(gumballVisible(flow, ["extrude"], "move"), false, "a window with no transform rail never shows a gumball");

  // 🕸️ Wire editing: connect and disconnect are both present, and each operation names its arguments.
  const operations = new Map(fixture.nodeGraphEditOperations.map((row) => [row.operation, row]));
  for (const required of ["connect", "disconnect", "move", "deleteSelection", "setFixture"]) {
    assert.ok(operations.has(required), `nodeGraphEdit must handle ${required}`);
  }
  assert.deepEqual([...operations.get("connect")!.required].sort(), ["sourceNodeId", "sourcePortId", "targetNodeId", "targetPortId"], "a connect names both endpoints by node AND port");
  assert.deepEqual([...operations.get("disconnect")!.required], ["synapseId"], "a disconnect names the wire it cuts");
  assert.deepEqual([...operations.get("move")!.required].sort(), ["nodeId", "x", "y"], "a move names the node and where it landed");
  assert.equal(operations.get("move")!.dispatchedBy, "nodeDrag");

  // 🚫️ No dead commands: every listed command has at least one trigger, and a removed one has none.
  const listed = new Set(fixture.commandTriggers.map((row) => row.command));
  for (const row of fixture.commandTriggers) {
    assert.ok(row.triggers.length > 0, `${row.command} is listed with no trigger at all`);
    if (row.keys !== null) assert.match(row.keys, /^[a-z+]+(,[a-z+]+)*$/, `${row.command}'s chord must be one or more plain lowercase chords, got ${row.keys}`);
  }
  assert.ok(fixture.commandTriggers.find((row) => row.command === "reorganize")!.triggers.includes("keybinding"), "reorganize must be keyboard-reachable");
  for (const removed of fixture.removedCommands) {
    assert.ok(!listed.has(removed.command), `${removed.command} was removed as dead but is still listed with triggers`);
    assert.ok(removed.reason.length > 0, `${removed.command} must say why it was removed`);
  }

  // 🖊️ The rename commit's argument names — derived, not restated.
  const { authored, fromTrigger, guestReads } = fixture.renameCommitArgument;
  const merged = mergeIntentPayload(Object.fromEntries(authored.map((key) => [key, "generation-1"])), "commit", "Balcony Study");
  assert.equal(merged[fromTrigger], "Balcony Study", `a scalar commit payload must arrive under \`${fromTrigger}\``);
  assert.equal(merged["id"], "generation-1", "the authored args must survive the merge");
  assert.ok(guestReads.includes(fromTrigger), "the guest has to read the spelling the trigger actually produces");

  console.log(
    `generation3d generate-mode interactions windows=${fixture.windows.length} rowVerbs=${generations.rowVerbs.length} previews=${previews.length} graphOps=${operations.size} commands=${fixture.commandTriggers.length} removed=${fixture.removedCommands.length}`,
  );
}

if (import.meta.main) testGeneration3dGenerateModeInteractionContract();
