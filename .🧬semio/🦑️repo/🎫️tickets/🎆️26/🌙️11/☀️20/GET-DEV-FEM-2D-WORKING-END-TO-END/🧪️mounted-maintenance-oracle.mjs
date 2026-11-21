import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { assign, createActor, createMachine } from "xstate";

const { idleMaintenance: fixture } = JSON.parse(await readFile(process.argv[2], "utf8"));
const machine = createMachine({
  initial: "mounted",
  context: { outcome: null },
  states: {
    mounted: { on: {
      MAINTAIN: { actions: assign({ outcome: "complete" }) },
      CLOSE: "closed",
    } },
    closed: { type: "final", output: "complete" },
  },
  output: ({ event }) => event.output,
});
const actor = createActor(machine).start();
for (let opportunity = 0; opportunity < fixture.opportunities; opportunity++) {
  actor.send({ type: "MAINTAIN" });
  const snapshot = actor.getSnapshot();
  assert.equal(snapshot.value, "mounted");
  assert.equal(snapshot.status === "done", fixture.liveTerminal);
  assert.equal(snapshot.hasTag("maintenance-pending"), fixture.livePending);
  assert.equal(snapshot.context.outcome, fixture.liveOutcome);
}
actor.send({ type: "CLOSE" });
assert.equal(actor.getSnapshot().status, "done");
assert.equal(actor.getSnapshot().value, "closed");
assert.equal(actor.getSnapshot().output, fixture.closedOutcome);
console.log("[DEBUG] XState idle maintenance fixture passed", fixture.opportunities);
