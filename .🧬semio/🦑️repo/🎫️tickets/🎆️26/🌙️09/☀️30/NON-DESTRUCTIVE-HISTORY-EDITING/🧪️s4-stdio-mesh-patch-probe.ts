/**
 * 🔺️ Runs the `🔺️mutate-semio-mesh` TypeScript oracle's `patch-snapshot` arm on the real derived model for ticket
 * 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING: the committed payload must move the model, its own computed inverse must restore
 * it exactly (member order included), the peer's `DeleteTexture` inverse (an indexed array insert) must still restore, and
 * an absent pointer and `splice` must be refused. Exits 1 on the first failure.
 *
 * usage: bun 🧪️s4-stdio-mesh-patch-probe.ts
 */
import { readFileSync } from "node:fs";
import { applyMutation, inverseMutation, parseDsl } from "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧪️tests/🔺️mutate-semio-mesh/🟦️.ts";

const fixtures = new URL("../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧫️fixtures/🔺️mutate-semio-mesh/", import.meta.url);
const model = parseDsl(readFileSync(new URL("🗣️.dsl.semio", fixtures), "utf8"));
const payload = JSON.parse(readFileSync(new URL("🩹️patch-snapshot/🦠️mutation/🔣️.json", fixtures), "utf8"));
let passed = 0;
const law = (name: string, holds: boolean) => {
  if (!holds) {
    console.error(`FAILED ${name}`);
    process.exit(1);
  }
  passed += 1;
};
const restoredBy = (mutation: Record<string, Record<string, unknown>>) => {
  let current = applyMutation(model, mutation);
  const moved = JSON.stringify(current) !== JSON.stringify(model);
  for (const step of inverseMutation(model, mutation)) current = applyMutation(current, step);
  return [moved, JSON.stringify(current) === JSON.stringify(model)] as const;
};
const refused = (mutation: Record<string, Record<string, unknown>>) => {
  try {
    applyMutation(model, mutation);
    return false;
  } catch {
    return true;
  }
};
const [moved, restored] = restoredBy(payload);
law("the committed patch moves the model", moved);
law("its own computed inverse restores the model exactly", restored);
law("the patched roughness is the payload's value", applyMutation(model, payload).materials[0]!.roughness === 0.5);
if (model.textures.length > 0) law("the DeleteTexture inverse still restores through an indexed array insert", restoredBy({ DeleteTexture: { id: model.textures[0]!.id } })[1]);
law("an absent pointer is refused", refused({ PatchSnapshot: { patch: { operation: "set", path: "/materials/999999/roughness", value: 0 } } }));
law("splice is refused", refused({ PatchSnapshot: { patch: { operation: "splice", path: "/materials", offset: 0, remove: 0, value: [] } } }));
law("an object member insert keeps its index", Object.keys(applyMutation(model, { PatchSnapshot: { patch: { operation: "insert", path: "/materials/0/extra", value: 1, index: 1 } } }).materials[0]!)[1] === "extra");
console.log(`mesh TypeScript oracle patch-snapshot arm: ${passed} passed / 0 failed`);
