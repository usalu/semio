/** 🔍️ Wave B56 standalone runner for the host-side alias-retirement laws — the react vitest lane is
 * mid-relocation by a peer (`📜️script.ts test` points at `🧪️tests/🎚️config/🟦️.ts`, which does not exist
 * yet), so the two suites are driven here directly. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import assert from "node:assert/strict";
import { windowHostContextBindings } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import fixture from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️window-host-context/🔣️.json";

const actual = windowHostContextBindings(fixture.instanceId, fixture.windows, fixture.view);
assert.deepEqual(actual, fixture.expected);
assert.equal(fixture.view.windowInstances.length > 0, true);
assert.deepEqual(actual.filter((binding) => binding.surface.surface === "window"), []);
assert.deepEqual([...new Set(actual.map((binding) => binding.surface.surface))], actual.map((binding) => binding.surface.surface));
console.log(`[DEBUG] b56 window-host-context bindings=${actual.length} surfaces=${JSON.stringify(actual.map((binding) => binding.surface.surface))} synthetic=0`);
