/** 🧪️ Exercises the actual kind-241 browser sender with the independently decoded source fixture. */
import assert from "node:assert/strict";
import { openSync, closeSync, readSync } from "node:fs";
import { createHash } from "node:crypto";
import { join } from "node:path";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { ACTOR_BYTE_PAGE_BYTES } from "@semio-tech/framework-actor/page";
import { SHARD_COMMAND_MAXIMUM_PAGES } from "@semio-tech/framework/actor/shard-client/wire";
import { sendNaturalSource, type NaturalSourceActor } from "./🗑️generated/natural-source-browser/natural-source.ts";
import { normalizeCommandIngress } from "./🗑️generated/natural-source-browser/normalize-command-ingress.js";
import laws from "./natural-pdf-browser-source.json";
import schema from "./natural-pdf-browser-source.schema.json";
import pdf from "./🔣️segmented-pdf-source.json";
import admission from "./natural-pdf-admission.json";

const validate = new Ajv({ strict: true }).compile(schema);
assert(validate(laws), JSON.stringify(validate.errors));
const db = new Database(":memory:");
const ranges = db.query("select ?1 * ?2 as start, min((?1 + 1) * ?2, ?3) as stop");
const count = db.query("select (?1 + ?2 - 1) / ?2 as pages");
const { pages: totalPages } = count.get(pdf.sourceBytes, ACTOR_BYTE_PAGE_BYTES) as { pages: number };
assert(totalPages > SHARD_COMMAND_MAXIMUM_PAGES);
assert.equal(laws[0]!.pages, totalPages);
const fd = openSync(join(import.meta.dir, "🗑️generated", "pdf-segmented-source.pdf"), "r");
const { declaration } = admission.offer;
const instanceId = admission.trusted.instance;
const extent = pdf.sourceBytes;
const seq = BigInt("0x" + admission.begin.seq);
assert(seq > BigInt(Number.MAX_SAFE_INTEGER));
const offer = { instanceId, extent, seq, declaration };
const loan = { operationId: admission.trusted.operation, generation: admission.trusted.generation, instanceId, declaration, extent };
const sourceHash = () => createHash("sha256");
try {
  for (const row of laws) {
    let reads = 0, pages = 0, retirements = 0, retries = 0, completionPolls = 0;
    let prepared = false, begun = false, completing = false, settled = false;
    let retried: Parameters<NaturalSourceActor["turn"]>[0];
    const abort = new AbortController();
    const cancelled = new Error("selection cancelled");
    const digest = sourceHash();
    const forAck = (phase: string, result: string, extra?: Record<string, unknown>) => {
      const body = { loan, owner: BigInt(instanceId), seq, kind: 241, extent, phase, result, pageIndex: undefined, ...extra };
      const option = (value: unknown) => value === undefined ? { tag: "none" } : { tag: "some", val: value };
      return { ingress: normalizeCommandIngress({ kind: 0, naturalSourceAck: option({
        ...body, loan: option(body.loan), pageIndex: option(body.pageIndex),
      }) }) };
    };
    const actor: NaturalSourceActor = {
      retired: () => false, assertOriginalActivation() {}, serialize: run => run(), now: () => 1,
      yieldTurn: async () => {}, settle: async () => { settled = completing; },
      async turn(crossing) {
        if (completing) {
          assert.equal(completionPolls++, 0);
          assert.equal(crossing, undefined);
          return forAck("complete", "accepted");
        }
        assert(crossing);
        const event = crossing.events[0];
        if (event?.kind === "natural-source-prepare") {
          assert.deepEqual(event.payload, offer);
          assert.equal(reads, 0);
          if (row.case === "prepare-refused") return forAck("prepare", "refused", { loan: undefined });
          if (row.case === "foreign-preparation") return forAck("prepare", "accepted", { loan: { ...loan, instanceId: instanceId + 1 } });
          prepared = true;
          if (row.case === "prepare-cancelled") abort.abort(cancelled);
          return forAck("prepare", "accepted");
        }
        if (event) {
          const body = event.payload as Record<string, unknown>;
          assert.equal(event.kind, "natural-source-control");
          assert.deepEqual(body, { loan, owner: BigInt(instanceId), seq, kind: 241, extent, control: body.control });
          if (body.control === "begin") {
            assert(prepared && !begun && !reads);
            begun = true;
            return forAck("begin", "accepted");
          }
          if (body.control === "cancel") {
            retirements++;
            return forAck("cancel", "accepted");
          }
          assert.equal(body.control, "complete");
          assert.equal(pages, totalPages);
          completing = true;
          return forAck("complete", "pending");
        }
        assert(begun);
        const page = crossing.page!;
        const {start, stop} = ranges.get(pages, ACTOR_BYTE_PAGE_BYTES, extent) as {start:number; stop:number};
        assert.equal(page.bytes.length, stop - start);
        assert.deepEqual(page.cursor, { owner: BigInt(instanceId), instance: instanceId, commandIndex: 0, commandCount: 1,
          kind: 241, seq, generation: BigInt("0x"+loan.generation), pageIndex: pages, pageCount: totalPages, itemCount: 1, metadata: extent });
        if (row.case === "source-complete" && pages === 0 && !retried) {
          retried = crossing;
          retries++;
          return { ingress: { tag: "backpressure", val: page.cursor } };
        }
        if (row.case === "source-complete" && pages === 0) assert.equal(crossing, retried);
        if (row.case === "foreign-page-ack") return forAck("page", "accepted", { pageIndex: pages, seq: seq + 1n });
        digest.update(page.bytes);
        return forAck("page", "accepted", { pageIndex: pages++ });
      },
    };
    const source = { name: "fixture.pdf", type: "", size: extent, slice(start: number, end: number) {
      assert(prepared && begun);
      const expected = ranges.get(reads++, ACTOR_BYTE_PAGE_BYTES, extent);
      assert.deepEqual({start, stop:end}, expected);
      assert.equal(reads, pages + 1);
      return { async arrayBuffer() {
        if (row.case === "pending-read-cancelled" && reads === 2) {
          queueMicrotask(() => abort.abort(cancelled));
          return new Promise<ArrayBuffer>(() => {});
        }
        const bytes = new Uint8Array(end - start - Number(row.case === "short-read"));
        assert.equal(readSync(fd, bytes, 0, bytes.length, start), bytes.length);
        return bytes.buffer;
      } };
    } };
    let result = "accepted";
    try {
      const accepted = await sendNaturalSource(offer, source, actor, {
        signal: abort.signal, current: () => true,
        progress(done, total) {
          assert.equal(total, extent);
          const limit = ranges.get(pages-1, ACTOR_BYTE_PAGE_BYTES, extent) as {stop:number};
          assert.equal(done, limit.stop);
        },
      });
      assert.deepEqual(accepted, { loan, owner: BigInt(instanceId), seq, kind: 241, extent, phase: "complete", result: "accepted" });
    } catch (error) {
      result = (error as Error).message;
    }
    assert.deepEqual({case: row.case, result, reads, pages, retirements, retries, completionPolls}, row);
    if (result === "accepted") {
      assert(settled);
      assert.equal(digest.digest("hex"), pdf.sourceSha256);
    }
  }
} finally {
  closeSync(fd);
  db.close();
}
console.log(`[DEBUG] Browser staged sender passed ${laws.length} laws, ${totalPages} exact source slices, and SHA-256 ${pdf.sourceSha256}`);

const generated = join(import.meta.dir, "🗑️generated", "natural-source-browser");
const overlay: [string, string | null, string][] = await Bun.file(join(generated,"overlay.json")).json();
const stagedShell = overlay.find(([path]) => path.includes("📤️document-transfer-tabs"))?.[2];
assert(stagedShell);
const methodStart = stagedShell.indexOf("      importInto: async (owner, picked, task) => {");
const methodStop = stagedShell.indexOf("\n      commit: async (owner, instanceId, task) => {", methodStart);
assert(methodStart >= 0 && methodStop > methodStart);
const lifecycle = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts").text();
const ownerStart = lifecycle.indexOf("export async function openNaturalFileOwnerV1(");
const ownerStop = lifecycle.indexOf("\n/** 📦️", ownerStart);
assert(ownerStart >= 0 && ownerStop > ownerStart);
const fixtureScope = [
  "let pickedName: string;",
  "const NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1 = 1024*1024;",
  lifecycle.slice(ownerStart,ownerStop),
  "export function actualPickerRoute({ naturalFileFormatV1, naturalFileMediaInputV1, readBlobBytesBoundedV1, core }) {",
  "  return {",
  stagedShell.slice(methodStart,methodStop),
  "  }.importInto;",
  "}",
].join("\n");
const shellHarness = join(generated,"shell-picker-route.ts");
await Bun.write(shellHarness,fixtureScope);
const { actualPickerRoute } = await import(shellHarness);
const signal = new AbortController().signal;
const maximumBytes = pdf.shellCommandBytes - 126;
const input = { port: "natural-file", descriptor: { wire: declaration }, maximumBytes };
for (const instrument of ["accepted", "refused", "ordinary"] as const) {
  const expected: string[] = instrument === "ordinary"
    ? ["contiguous-read","ordinary-create","contiguous-import"]
    : ["purpose-create","segmented-import",...(instrument === "refused" ? ["retire"] : [])];
  const seen: string[] = [];
  const picked = { size: instrument === "ordinary" ? 64 : extent, name: "natural-source.pdf" };
  const route = actualPickerRoute({
    naturalFileFormatV1: () => declaration,
    naturalFileMediaInputV1: () => input,
    readBlobBytesBoundedV1: async () => {
      assert.equal(instrument,"ordinary");
      seen.push("contiguous-read");
      return new Uint8Array([37,80,68,70]);
    },
    core: { pluginOpInFlightRef: {current: new Set()} },
  });
  const owner = { app: {id:"fixture"}, handle: {
    pluginId: "stdio-pdf",
    async createApp(appId: string, purpose: unknown) {
      assert.equal(appId,"fixture");
      if (instrument === "ordinary") {
        assert.equal(purpose,undefined);
        seen.push("ordinary-create");
      } else {
        assert.deepEqual(purpose,{tag:"natural-source",val:{declaration,extent}});
        seen.push("purpose-create");
      }
      return 41;
    },
    async importAppNaturalSource(id: number, declared: unknown, source: unknown, control: {signal:AbortSignal; current():boolean}) {
      assert.equal(id,41);
      assert.deepEqual(declared,declaration);
      assert.equal(source,picked);
      assert.equal(control.signal,signal);
      assert(control.current());
      seen.push("segmented-import");
      if (instrument === "refused") throw Error("fixture decoder refused");
    },
    async importAppMedia(id: number, port: string, descriptor: unknown, bytes: Uint8Array, acceptedSignal: AbortSignal) {
      assert.equal(id,41);
      assert.equal(port,input.port);
      assert.equal(descriptor,input.descriptor);
      assert.equal(bytes.byteLength,4);
      assert.equal(acceptedSignal,signal);
      seen.push("contiguous-import");
    },
    async destroyApp(id: number) {
      assert.equal(id,41);
      seen.push("retire");
    },
  } };
  const opened = route(owner,picked,{signal,progress() {}});
  if (instrument === "refused") await assert.rejects(opened,/fixture decoder refused/);
  else assert.equal(await opened,41);
  assert.deepEqual(seen,expected);
}
console.log("[DEBUG] Actual picker route preserved the ordinary open, issued the fresh purpose and retired a refused fresh owner");
