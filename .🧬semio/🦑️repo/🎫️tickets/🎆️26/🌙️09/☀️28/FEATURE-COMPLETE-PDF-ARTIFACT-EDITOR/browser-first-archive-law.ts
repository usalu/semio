/** 🧪 Exercises the actual dev provider journal with the independently produced PDF Pack fixture. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { statSync } from "node:fs";
import { mkdir, open, readFile, readdir, rm, stat, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { appendArchive, captureFirstFolderArchive, eventLogPath, readLatestArchive, withFolderEventWriter } from "./browser-first-archive-journal.ts";

const load = (path: string) => Bun.file(join(import.meta.dir, path)).json();
const fixture = await load("browser-first-archive-cases.json");
const schema = await load("browser-first-archive-cases.schema.json");
const ajv = new Ajv2020({ strict: true });
assert(ajv.validate(schema, fixture), JSON.stringify(ajv.errors));
const generated = join(import.meta.dir, "🗑️generated");
const directory = join(generated, "browser-first-archive", "law");
await rm(directory, { recursive: true, force: true });
await mkdir(directory, { recursive: true });
const pack = await readFile(join(generated, "paged-jpeg-assets", "large.dct-placement.pack"));
const spr = Buffer.from(`schema=${fixture.schema}\ndocument=${fixture.document}`);
const varint = (length: number): Buffer => {
  const bytes: number[] = [];
  for (; length >= 128; length = Math.floor(length / 128)) bytes.push(length % 128 + 128);
  return Buffer.from([...bytes, length]);
};
const archive = Buffer.concat([Buffer.of(1), varint(pack.length), pack, varint(spr.length), spr, Buffer.of(0)]);
assert.equal(archive.length, fixture.extent);
const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest();
const digest = hash(archive);
const frontier = { documentId: fixture.document, generation: BigInt(fixture.generation), revision: new Uint8Array(32).fill(5), extent: fixture.extent };
const first = eventLogPath(join(directory, "first")), corrupt = eventLogPath(join(directory, "corrupt")), rollback = eventLogPath(join(directory, "rollback"));
type Capture = Awaited<ReturnType<typeof captureFirstFolderArchive>>;
const outcome = async (work: () => Promise<unknown>) => {
  try { await work(); return "accepted"; }
  catch (error) { return (error as Error).message.replace(/^folder event log: /u, ""); }
};
const results: Record<string, string> = {};
const capture = (path = first, artifactSchema = fixture.schema, current = () => true) =>
  captureFirstFolderArchive(path, artifactSchema, frontier, current, withFolderEventWriter);
const fill = async (sink: Capture, bytes: Uint8Array, repeated = false) => {
  for (let offset = 0; offset < bytes.length; offset += fixture.page) {
    const page = bytes.subarray(offset, offset + fixture.page);
    const receipt = { offset, length: page.length };
    assert.deepEqual(await sink.write(offset, page), receipt);
    if (offset === 0 && repeated) {
      assert.deepEqual(await sink.write(offset, page), receipt);
      const wrong = Uint8Array.from(page);
      wrong[0] ^= 1;
      assert.equal(await outcome(() => sink.write(offset, wrong)), "archive retry changed bytes");
    }
  }
};
const signal = new AbortController().signal;
const ordinary = Uint8Array.of(1, 1, 4, 1, 5, 0);
await appendArchive(first, "neighbor", fixture.schema, ordinary);
const baseline = await readFile(first);
await writeFile(first, Buffer.from("SEMIOEL1"), { flag: "a" });

const testHandle = await open(first, "r");
const prototype = Object.getPrototypeOf(testHandle);
const oldRead = prototype.read, oldWrite = prototype.write;
await testHandle.close();
let maximumRead = 0, maximumWrite = 0;
prototype.read = function (bytes: Uint8Array, offset: number, length: number, position: number) {
  maximumRead = Math.max(maximumRead, length);
  return oldRead.call(this, bytes, offset, length, position);
};
prototype.write = function (bytes: Uint8Array, offset: number, length: number, position: number) {
  maximumWrite = Math.max(maximumWrite, length);
  return oldWrite.call(this, bytes, offset, length, position);
};
try {
  let sink = await capture();
  const pending = sink.write(0, archive.subarray(0, fixture.page));
  assert.equal(await outcome(() => sink.write(0, archive.subarray(0, fixture.page))), "archive page acknowledgement is pending");
  await pending;
  await fill(sink, archive, true);
  const badDigest = Buffer.from(digest);
  badDigest[0] ^= 1;
  results.seal = await outcome(() => sink.publish(badDigest, signal));
  results.commit = await outcome(async () => {
    const stored = await sink.publish(digest, signal);
    assert.deepEqual(stored, { frontier, extent: archive.length, digest: Uint8Array.from(digest) });
    stored.frontier.revision[0] ^= 1;
    stored.digest[0] ^= 1;
    assert.deepEqual(await sink.publish(digest, signal), { frontier, extent: archive.length, digest: Uint8Array.from(digest) });
  });
  const durable = await readFile(first);
  assert(durable.subarray(0, baseline.length).equals(baseline));
  sink = await capture();
  await fill(sink, archive);
  results.retry = await outcome(() => sink.publish(digest, signal));
  assert((await readFile(first)).equals(durable));
  assert.deepEqual(await readLatestArchive(first, "neighbor"), ordinary);
  assert.deepEqual(await readLatestArchive(first, fixture.document), Uint8Array.from(archive));

  sink = await capture(first, "other-schema");
  await fill(sink, archive);
  results.schema = await outcome(() => sink.publish(digest, signal));
  await sink.retire();

  sink = await capture();
  results.order = await outcome(() => sink.write(fixture.page, archive.subarray(fixture.page, 2 * fixture.page)));
  await sink.retire();
  await sink.retire();
  let live = true;
  sink = await capture(first, fixture.schema, () => live);
  live = false;
  results.stale = await outcome(() => sink.write(0, archive.subarray(0, fixture.page)));
  await sink.retire();
  results.overLimit = await outcome(() => captureFirstFolderArchive(first, fixture.schema, { ...frontier, extent: 4 * 1024 * 1024 + 1 }, () => true, withFolderEventWriter));
  sink = await capture();
  const malformed = Buffer.from(archive);
  malformed[malformed.length - 1] = 1;
  await fill(sink, malformed);
  results.format = await outcome(() => sink.publish(hash(malformed), signal));
  await sink.retire();

  await appendArchive(rollback, "neighbor", fixture.schema, ordinary);
  const rollbackBaseline = await readFile(rollback);
  const controller = new AbortController();
  let observedPartial = false;
  sink = await capture(rollback, fixture.schema, () => {
    if (statSync(rollback).size > rollbackBaseline.length + 2 * fixture.page) {
      observedPartial = true;
      controller.abort(Error("cancelled"));
    }
    return true;
  });
  await fill(sink, archive);
  results.rollback = await outcome(() => sink.publish(digest, controller.signal));
  assert(observedPartial);
  assert((await readFile(rollback)).equals(rollbackBaseline));
  assert.equal(await readLatestArchive(rollback, fixture.document), null);
  await sink.retire();

  await mkdir(dirname(corrupt), { recursive: true });
  const damaged = Buffer.from(durable);
  damaged[16] ^= 1;
  await writeFile(corrupt, damaged);
  results.interior = await outcome(() => readLatestArchive(corrupt, fixture.document));
  assert((await readFile(corrupt)).equals(damaged));
  assert.deepEqual(results, fixture.cases);
  for (const journal of [first, rollback]) assert.deepEqual(await readdir(dirname(journal)), ["events.semio"]);
  assert.equal((await stat(first)).size, durable.length);
} finally {
  prototype.read = oldRead;
  prototype.write = oldWrite;
}
assert.equal(maximumRead, fixture.page);
assert.equal(maximumWrite, fixture.page);
await writeFile(join(directory, "law.json"), JSON.stringify({ ...results, maximumRead, maximumWrite, sha256: digest.toString("hex") }));
console.log(`[DEBUG] Browser captured ${archive.length} bytes with reads and writes at most ${fixture.page} bytes.`);
