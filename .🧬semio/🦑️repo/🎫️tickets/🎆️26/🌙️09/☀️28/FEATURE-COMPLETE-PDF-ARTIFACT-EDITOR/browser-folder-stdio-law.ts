/** 🧪 Exercises the exact Bun-to-Rust journal bridge and a competing OS writer. */
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { once } from "node:events";
import { mkdir, readFile, readdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { nativeFolderJournal, type NativeFolderJournalScope } from "./browser-folder-stdio.ts";
import { eventLogPath, readLatestArchive } from "./browser-first-archive-journal.ts";

const directory = join(import.meta.dir, "🗑️generated", "browser-first-archive");
const envBinary = process.argv[2];
const binary = envBinary || join(directory, process.platform === "win32" ? "semio-folder-journal.exe" : "semio-folder-journal");
const output = join(directory, "native-law");
const folder = join(output, "first");
await rm(output, { recursive: true, force: true });
await mkdir(folder, { recursive: true });
const pack = await readFile(join(import.meta.dir, "🗑️generated", "paged-jpeg-assets", "large.dct-placement.pack"));
const seed = Buffer.from("schema=stdio.pdf.1.7\ndocument=pdf-document");
function varint(value: number) {
  const result: number[] = [];
  for (; value >= 128; value >>= 7) result.push((value & 127) | 128);
  return Buffer.from([...result, value]);
}
const archive = Buffer.concat([Buffer.of(1), varint(pack.length), pack, varint(seed.length), seed, Buffer.of(0)]);
const digest = createHash("sha256").update(archive).digest();
const schema = "stdio.pdf.1.7";
const scope: NativeFolderJournalScope = {
  schema,
  frontier: { documentId: "pdf-document", generation: 9007199254740993n, revision: new Uint8Array(32).fill(7), extent: archive.length },
  source: {
    instanceId: 42, operationId: "0020000000000002", generation: "0020000000000003", sequence: 9007199254740997n, extent: 9437877,
    declaration: { formatKind: "s.stdio.pdf@1.7", extension: ".pdf", mediaType: "application/pdf", binary: true },
  },
};
const native = nativeFolderJournal(binary);
const signal = new AbortController();
const ordinary = Uint8Array.of(1, 1, 9, 1, 8, 0);
await native.put(folder, "neighbor", schema, ordinary);
let capture = await native.capture(folder, scope, signal.signal, () => true);
const fill = async () => {
  for (let offset = 0; offset < archive.length; offset += 4096) {
    const page = archive.subarray(offset, offset + 4096);
    const receipt = { offset, length: page.length };
    assert.deepEqual(await capture.write(offset, page), receipt);
    if (offset === 0) assert.deepEqual(await capture.write(offset, page), receipt);
  }
};
await fill();
const durable = await capture.publish(digest, signal.signal);
assert.deepEqual(durable, { extent: archive.length, digest: Uint8Array.from(digest), frontier: scope.frontier });
assert.deepEqual(await readLatestArchive(eventLogPath(folder), "pdf-document"), Uint8Array.from(archive));
assert.deepEqual(await readLatestArchive(eventLogPath(folder), "neighbor"), ordinary);
await capture.retire();
const baseline = await readFile(eventLogPath(folder));
assert.deepEqual(await capture.publish(digest, signal.signal), durable);
capture = await native.capture(folder, scope, signal.signal, () => true);
await fill();
assert.deepEqual(await capture.publish(digest, signal.signal), durable);
assert((await readFile(eventLogPath(folder))).equals(baseline));

const cancelled = new AbortController();
capture = await native.capture(folder, { ...scope, frontier: { ...scope.frontier, documentId: "abandoned" } }, cancelled.signal, () => true);
await capture.write(0, archive.subarray(0, 4096));
cancelled.abort(Error("revoke owner"));
await assert.rejects(capture.write(4096, archive.subarray(4096, 8192)), /revoke owner/u);
await capture.retire();
assert((await readFile(eventLogPath(folder))).equals(baseline));

const holder = spawn(binary, ["hold", eventLogPath(folder)], { stdio: ["pipe", "pipe", "inherit"] });
try {
  const [ready] = await once(holder.stdout, "data");
  assert.equal(ready.toString(), "ready");
  await assert.rejects(native.put(folder, "locked", schema, ordinary), /folder event writer busy/u);
  capture = await native.capture(folder, { ...scope, frontier: { ...scope.frontier, documentId: "locked" } }, signal.signal, () => true);
  await fill();
  await assert.rejects(capture.publish(digest, signal.signal), /lock|busy|unavailable|temporarily/iu);
} finally {
  holder.stdin.end(Buffer.of(1));
  assert.deepEqual(await once(holder, "close"), [0, null]);
}
assert((await readFile(eventLogPath(folder))).equals(baseline));
assert.deepEqual(await readdir(join(folder, ".semio")), ["events.semio"]);
assert.equal(await readLatestArchive(eventLogPath(folder), "locked"), null);
await Bun.write(join(output, "record.sha256"), digest.toString("hex"));
console.log(`[DEBUG] Node issued an exact ${archive.length}-byte durable first-genesis receipt through the locked Rust companion.`);
