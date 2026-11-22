import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { basename } from "node:path";
import { fileURLToPath } from "node:url";
import { acquireSlotV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts";

const root = await mkdtemp(fileURLToPath(new URL("./🗑️generated/unit-slots-", import.meta.url)));
const bounded = AbortSignal.timeout(10_000);
const first = await acquireSlotV1(root, 2, { signal: bounded });
const stop = new AbortController();
let waiter: Promise<unknown> | undefined;
let following: ReturnType<typeof acquireSlotV1> | undefined;
try {
  const independent = await acquireSlotV1(root, 2, { signal: bounded });
  assert.notEqual(independent.path, first.path);
  independent.release();

  const occupied = Promise.withResolvers<void>();
  waiter = acquireSlotV1(root, 1, {
    signal: AbortSignal.any([bounded, stop.signal]),
    onWait: () => occupied.resolve(),
  });
  await Promise.race([occupied.promise, waiter.then(() => assert.fail("the one-slot waiter bypassed the held slot"))]);

  const queued = Promise.withResolvers<void>();
  following = acquireSlotV1(root, 2, { signal: bounded, onWait: () => queued.resolve() });
  await Promise.race([queued.promise, following.then(() => assert.fail("the second lane bypassed the oldest shared ticket"))]);
  stop.abort();
  await assert.rejects(waiter);
  const admitted = await following;
  assert.notEqual(admitted.path, first.path);
  assert.equal(basename(admitted.path), "1.lock");
  admitted.release();
  console.log("[DEBUG] Host-wide slots admit independent lanes, but a one-slot FIFO head blocks the other free slot.");
} finally {
  stop.abort();
  first.release();
  if (following) await following.then(claim => claim.release(), () => {});
  if (waiter) await waiter.catch(() => {});
  await rm(root, { recursive: true, force: true });
}
