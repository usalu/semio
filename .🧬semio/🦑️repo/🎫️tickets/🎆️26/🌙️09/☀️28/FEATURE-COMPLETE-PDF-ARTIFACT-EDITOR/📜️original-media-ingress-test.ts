/** 🧪 Checks bounded file reads, captured loan, progress and cancellation against a neutral manifest. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import Ajv from "ajv/dist/2020";
import {ORIGINAL_MEDIA_PAGE_BYTES, sendOriginalMedia} from "./📜️original-media-ingress";

const json = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const cases = json("./🔣️original-media-ingress-cases.json");
const validate = new Ajv({strict: true}).compile(json("./🔣️original-media-ingress.schema.json"));

for (const fixture of cases.cases) {
  let state = 0x3167ab52;
  const original = Uint8Array.from({length: fixture.size}, () => {
    state ^= state << 13;
    state ^= state >>> 17;
    state ^= state << 5;
    return state & 0xff;
  });
  const blob = new Blob([original], {type: "image/jpeg"});
  const abort = new AbortController();
  const hash = new Bun.CryptoHasher("sha256");
  const authored = createHash("sha256").update(original).digest("hex");
  const frames: string[] = [];
  const reads: number[] = [];
  let current = true;
  let bytes = 0;
  let count = 0;
  let live = false;
  const source = {
    name: fixture.filename ?? "picked-image.jpg", type: fixture.mediaType ?? blob.type, size: blob.size,
    slice(start: number, end: number) {
      assert.equal(start, bytes);
      assert(end > start && end - start <= ORIGINAL_MEDIA_PAGE_BYTES);
      reads.push(end - start);
      return blob.slice(start, fixture.shortAt === reads.length - 1 ? end - 1 : end);
    },
  };
  const issued = {...cases.loan, documentId: fixture.documentId ?? cases.loan.documentId};
  const transfer = sendOriginalMedia(issued, source, fixture.limit, async (frame, body) => {
    assert(validate(frame), JSON.stringify(validate.errors));
    assert.deepEqual(frame.loan, issued);
    frames.push(frame.kind);
    switch (frame.kind) {
      case "begin":
        assert(!live && frames.length === 1);
        assert.equal(frame.byteLength, original.length);
        assert.equal(frame.mediaType, source.type || "application/octet-stream");
        live = true;
        break;
      case "page":
        assert(live && body);
        assert.equal(frame.index, count);
        assert.equal(body.length, Math.min(ORIGINAL_MEDIA_PAGE_BYTES, original.length - bytes));
        assert.equal(frame.byteLength, body.length);
        if (fixture.rejectAt === frame.index) throw Error("The test sink rejected its page");
        assert.deepEqual(body, original.slice(bytes, bytes + body.length));
        hash.update(body);
        count++;
        bytes += body.length;
        break;
      case "complete":
        assert(live && bytes === original.length);
        assert.equal(hash.digest("hex"), authored);
        live = false;
        break;
      case "cancel":
        assert(live);
        live = false;
        break;
    }
  }, {signal: abort.signal, current: () => current, progress(completed, total) {
    assert.equal(completed, bytes);
    assert.equal(total, original.length);
    if (fixture.cancelAfter === count) abort.abort();
    if (fixture.staleAfter === count) current = false;
  }});
  if (fixture.result === "complete") await transfer;
  else await assert.rejects(transfer);
  assert.equal(count, fixture.acknowledged);
  assert(!live);
  assert.equal(frames.at(-1) ?? "refused", fixture.result);
  if (fixture.result === "refused") assert.equal(reads.length, 0);
  console.log("[DEBUG]", fixture.name, "accepted", count, "pages via", reads.length, "bounded reads");
}

{
  const captured = {...cases.loan};
  const selected = {name: "unchanged.jpg", type: "image/jpeg", size: 8193,
    slice(start: number, end: number) { return new Blob([new Uint8Array(end - start)]); }};
  let count = 0;
  await sendOriginalMedia(captured, selected, 8193, async frame => {
    assert(validate(frame), JSON.stringify(validate.errors));
    assert.deepEqual(frame.loan, cases.loan);
    if (frame.kind === "begin") {
      selected.size = 1;
      captured.baseRevision = "0".repeat(64);
    }
    if (frame.kind === "page") count++;
  }, {signal: new AbortController().signal, current: () => true, progress(_, total) { assert.equal(total, 8193); }});
  assert.equal(count, 3);
  console.log("[DEBUG] Original picker loan and extent stayed stable across awaited transfer turns");
}

{
  const started = Promise.withResolvers<void>();
  const fileRead = Promise.withResolvers<ArrayBuffer>();
  const signal = new AbortController();
  const notifications: string[] = [];
  const failure = new Error("picker cancelled during physical read");
  const transfer = sendOriginalMedia(cases.loan, {
    name: "slow.jpg", type: "image/jpeg", size: 8193,
    slice(start, end) {
      assert.equal(start, 0);
      assert.equal(end, ORIGINAL_MEDIA_PAGE_BYTES);
      return {arrayBuffer() { started.resolve(); return fileRead.promise; }};
    },
  }, 8193, async frame => { notifications.push(frame.kind); }, {
    signal: signal.signal, current: () => true,
    progress() { throw Error("an outstanding read must not advance acknowledged progress"); },
  });
  await started.promise;
  signal.abort(failure);
  await assert.rejects(transfer, error => error === failure);
  assert.deepEqual(notifications, ["begin", "cancel"]);
  fileRead.resolve(new ArrayBuffer(ORIGINAL_MEDIA_PAGE_BYTES));
  await Promise.resolve();
  assert.deepEqual(notifications, ["begin", "cancel"]);
  console.log("[DEBUG] Cancelled an outstanding bounded physical read without advancing the source");
}
