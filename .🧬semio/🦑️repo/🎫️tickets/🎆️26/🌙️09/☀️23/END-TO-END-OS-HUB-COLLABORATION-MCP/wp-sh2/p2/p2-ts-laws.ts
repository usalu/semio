/** 🧪️ SH2 P2: runs the two new TypeScript laws' assertions with bun against the overlay's modules (the vitest suites that
 * carry them load through the tree's node_modules, which a scratch overlay cannot serve): the directory twin folds the
 * language-neutral activity cases exactly, and the host cuts 1 000 check-ins into bounded, contiguous batches. */
import assert from "node:assert/strict";
import { Buffer } from "node:buffer";
import { readFileSync } from "node:fs";

const overlay = process.argv[2]!;
const os = `${overlay}/🧰️framework/🛍️products/💻️os`;
const { emptyDirectoryReadModel, foldAll } = await import(`${os}/🔨️modules/📇️directory/🟦️.ts`);
const { SPACE_DIRECTORY_BATCH_MAX_BYTES, SPACE_DIRECTORY_BATCH_MAX_EVENTS, SpaceDirectoryHistoryV1, spaceDirectoryBatchesV1 } = await import(`${os}/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📇️space-directory/🟦️.ts`);

const fixture = JSON.parse(readFileSync(`${os}/🧫️fixtures/📇️directory/🕒️document-activity-v1.json`, "utf8"));
const model = foldAll(emptyDirectoryReadModel(), fixture.events);
assert.equal(model.cursor, fixture.expected.cursor);
for (const [spaceId, rows] of Object.entries(fixture.expected.documentActivity)) assert.deepEqual(model.spaces.get(spaceId)?.documentActivity, rows);
assert.equal(model.spaces.has("space-unknown"), false);
assert.deepEqual(foldAll(model, fixture.events), model);
const golden = JSON.parse(readFileSync(`${os}/🧫️fixtures/📇️directory/⚡️events.json`, "utf8"));
assert.equal(foldAll(emptyDirectoryReadModel(), golden.events).spaces.get("sp-studio-fabrication")?.documentActivity.length, 0);

const hash = (byte: number) => Array.from({ length: 32 }, () => byte);
const checkIn = (seq: number) => ({
  seq, id: `e${seq}`, hlc: { physicalMs: seq, logical: 0 }, actor: { kind: "user", id: "user:u1#s1" }, spaceId: "space-a", recordedAtMs: seq,
  body: { kind: "artifact.checkpoint-published", checkpoint: { scope: { spaceId: "space-a", documentId: "doc" }, checkpointId: hash(seq % 251), descriptorDigestV1: hash(7), baselineFrontier: { documentId: "doc", headEditOrdinal: seq, headEditId: `edit:${seq}`, lastCommitSeq: seq, chainHash: hash(2) }, pack: { sha256: hash(3), byteLength: 2 }, spr: { sha256: hash(4), byteLength: 15 }, aggregateSha256: hash(5), publishedAtMs: seq } },
});
const history = new SpaceDirectoryHistoryV1("space-a");
history.add(Array.from({ length: 1_000 }, (_, index) => checkIn(index + 1)));
const batches = spaceDirectoryBatchesV1(history, 0);
assert(batches.length > 1);
let after = 0;
const fed: number[] = [];
for (const batch of batches) {
  const events = JSON.parse(batch.eventsJson) as { seq: number }[];
  assert(Buffer.byteLength(batch.eventsJson, "utf8") <= SPACE_DIRECTORY_BATCH_MAX_BYTES);
  assert(events.length <= SPACE_DIRECTORY_BATCH_MAX_EVENTS);
  assert.equal(batch.afterSeqExclusive, after);
  assert.equal(batch.throughSeqInclusive, events[events.length - 1]!.seq);
  after = batch.throughSeqInclusive;
  fed.push(...events.map((event) => event.seq));
}
assert.deepEqual(fed, Array.from({ length: 1_000 }, (_, index) => index + 1));
assert.deepEqual(spaceDirectoryBatchesV1(history, 1_000), []);
history.add([checkIn(1_001)]);
const [latest, ...rest] = spaceDirectoryBatchesV1(history, 1_000);
assert.deepEqual(rest, []);
assert.deepEqual((JSON.parse(latest.eventsJson) as { seq: number }[]).map((event) => event.seq), [1_001]);
console.log(`p2-ts-laws: activity twin ok; ${batches.length} bounded batches for 1 000 check-ins (largest ${Math.max(...batches.map((batch: { eventsJson: string }) => Buffer.byteLength(batch.eventsJson, "utf8")))} bytes); one new check-in = one batch`);
