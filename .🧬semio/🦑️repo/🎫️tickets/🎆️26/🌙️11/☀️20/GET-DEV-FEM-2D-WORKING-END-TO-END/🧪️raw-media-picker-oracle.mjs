import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { assign, createActor, createMachine } from "xstate";

const { picker } = JSON.parse(await readFile(process.argv[2], "utf8"));

const machine = createMachine({
  initial: "queued",
  context: { published: [], terminal: null },
  states: {
    queued: {
      on: {
        DRAIN: { target: "held", actions: assign({ published: ["picker-before"] }) },
      },
    },
    held: { on: { ADOPT: "adopted" } },
    adopted: {
      on: {
        DRAIN: {
          target: "offered",
          actions: assign({
            published: ({ context }) => [...context.published, "picker", "picker-after"],
          }),
        },
      },
    },
    offered: {
      on: {
        CANCEL: { target: "closed", actions: assign({ terminal: "accepted" }) },
      },
    },
    closed: { type: "final" },
  },
});

const actor = createActor(machine).start();
actor.send({ type: "DRAIN" });
actor.send({ type: "DRAIN" });
assert.deepEqual(actor.getSnapshot().context.published, ["picker-before"]);
actor.send({ type: "ADOPT" });
actor.send({ type: "ADOPT" });
actor.send({ type: "DRAIN" });
assert.deepEqual(actor.getSnapshot().context.published, picker.effect_order);
actor.send({ type: "CANCEL" });
assert.equal(actor.getSnapshot().status, "done");
assert.equal(actor.getSnapshot().context.terminal, "accepted");

console.log(JSON.stringify(actor.getSnapshot().context));
