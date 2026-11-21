/** 📐 Exercises the production input lane with equivalent PDF page-boundary spellings. */
import assert from "node:assert/strict";
import { InputCommitLaneV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️commit-lane/🟦️.ts";
import { inputAppliedV1, inputCommitReceiptV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";

const owner = {active: true, subscribeRetirement: () => () => undefined} as ConstructorParameters<typeof InputCommitLaneV1>[0];
const boundary = {target: "crop-box", value: "0 0 200 300", revision: "1"};
const submit = (expected: string) => new InputCommitLaneV1(owner).submit({
  read: () => ({...boundary}),
  expected,
  active: () => true,
  subscribe: () => () => undefined,
  send: async () => {
    boundary.value = "-10 20 170 280";
    boundary.revision = "2";
    return inputAppliedV1(1, inputCommitReceiptV1(1n, 2n));
  },
});
const canonical = await submit("-10 20 170 280");
assert.equal(canonical.kind, "applied");
boundary.value = "0 0 200 300";
boundary.revision = "1";
await assert.rejects(submit("-10  20 170 280"), /Input publication changed the requested value/);
assert.equal(boundary.value, "-10 20 170 280");
console.log("Production input lane acknowledges canonical spelling and rejects equivalent noncanonical spelling after the native edit applied.");
