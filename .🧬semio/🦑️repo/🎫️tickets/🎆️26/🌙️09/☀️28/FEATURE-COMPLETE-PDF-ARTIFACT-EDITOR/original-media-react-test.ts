/** 🧪️ Verifies the staged raw picker using a real File, codec, and repository selection helper. */
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { join, resolve, basename, dirname } from "node:path";
import { pathToFileURL } from "node:url";
import { createHash } from "node:crypto";
import Ajv from "ajv/dist/2020";
import { readActorBytePage } from "@semio-tech/framework-actor/page";

const ticket = import.meta.dirname;
const output = join(ticket, "🗑️generated");
const modules = join(output, "react-raw-picker");
await mkdir(modules, { recursive: true });
const toUrl = (path: string) => pathToFileURL(resolve(path)).href;
const actor = "🧰️framework/🔨️modules/🎭️actor";
const guards: [string, string | null, string][] = await Bun.file(join(output, "original-media-react-overlay.json")).json();
for (const [path, before, text] of guards) {
  new Bun.Transpiler({ loader: "ts" }).transformSync(text);
  if (before) continue;
  const body = text.replace("@semio-tech/framework-actor/page", toUrl(actor + "/📃️page/🟦️.ts"))
    .replace("@semio-tech/framework/actor/shard-client/wire", toUrl(actor + "/📮️shard-client/📜️wire/🟦️.ts"));
  await Bun.write(join(modules, basename(path)), body);
}

const selectionSource = await Bun.file("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts").text();
const selectionStart = selectionSource.indexOf("export function requestFileSelectionV1(");
const selectionEnd = selectionSource.indexOf("\nconst readBlobSliceV1", selectionStart);
assert(selectionStart >= 0 && selectionEnd > selectionStart);
await Bun.write(join(modules, "select.ts"), selectionSource.slice(selectionStart, selectionEnd));
const { requestFileSelectionV1 } = await import(toUrl(join(modules, "select.ts")));
const { originalMediaRequest } = await import(toUrl(join(modules, "original-media-wire.ts"))) as typeof import("./original-media-react-wire");
const { createOriginalMediaExchange } = await import(toUrl(join(modules, "original-media-exchange.ts"))) as typeof import("./original-media-react-exchange");
const { pickOriginalMedia } = await import(toUrl(join(modules, "original-media-pick.ts"))) as typeof import("./original-media-react-pick");
const neutral = await Bun.file(join(ticket, "original-media-guest-cases.json")).json();
const cases = await Bun.file(join(ticket, "original-media-react-cases.json")).json();
const fixture = Bun.file(join(output, "paged-media-receiver/raw-picker.png"));
assert(await fixture.exists());
const request = originalMediaRequest(cases.request, cases.request.loan.instanceId);
assert.equal(request.loan.baseRevision.length, 64);
const law = new Ajv({ strict: true }).addSchema(await Bun.file(join(ticket, "🔣️original-media-ingress.schema.json")).json(), "urn:semio:original-media-ingress").compile(await Bun.file(join(ticket, "original-media-react.schema.json")).json());
assert(law(request), JSON.stringify(law.errors));
const bridge = guards.find(([path]) => path.includes("component-bridge-source"));
assert(bridge);
const [bridgePath, , bridgeBody] = bridge;
const kindSchema = /import \{ COMMAND_INGRESS_KINDS \} from "([^"]+)";/.exec(bridgeBody)?.[1];
assert(kindSchema);
const { COMMAND_INGRESS_KINDS } = await import(toUrl(resolve(dirname(bridgePath), kindSchema))) as { COMMAND_INGRESS_KINDS: readonly string[] };
const start = bridgeBody.indexOf("function normalizeCommandIngress(");
const end = bridgeBody.indexOf("\nfunction uiPatchReceipt", start);
assert(start >= 0 && end > start);
const normalizerSource =
  'const optionValue = (value) => value?.tag === "some" ? value.val : value?.tag === "none" ? undefined : value;\n' +
  `const commandIngressKinds = new Map(${JSON.stringify(COMMAND_INGRESS_KINDS.map((tag, kind) => [kind, tag]))});\n` +
  bridgeBody.slice(start, end).replaceAll("\\`", "`").replaceAll("\\${", "${") +
  "\nexport { normalizeCommandIngress };\n";
await Bun.write(join(modules, "normalizer.ts"), normalizerSource);
const { normalizeCommandIngress } = await import(toUrl(join(modules, "normalizer.ts")));
assert.deepEqual(normalizeCommandIngress({kind: 0}), {tag: "idle"});
const hex = (value: bigint) => value.toString(16).padStart(16, "0");

for (const row of cases.cases) {
  const reads: number[] = [];
  class PickedFile extends File {
    override arrayBuffer(): Promise<ArrayBuffer> { throw new Error("whole File read"); }
    override slice(start = 0, end = this.size): Blob {
      reads.push(end - start);
      assert(end - start <= neutral.pageBytes);
      const page = super.slice(start, end);
      if (row.trigger === "abort-read" || row.trigger === "stale-read") {
        const read = page.arrayBuffer.bind(page);
        Object.defineProperty(page, "arrayBuffer", { value: async () => {
          if (row.trigger === "abort-read") controller.abort(new DOMException("cancelled", "AbortError"));
          else current = false;
          return read();
        }});
      }
      return page;
    }
  }
  const file = new PickedFile([fixture], "registered-picker.png", { type: "image/png" });
  assert.equal(file.size, neutral.png.pageLengths.reduce((a: number,b: number) => a+b, 0));
  let current = true, retired = false, inActor = false;
  let detached = 0, selected = 0, finished = 0;
  let hostTime = 0;
  const controller = new AbortController();
  const phase: string[] = [], transferred: number[] = [], progress: number[] = [];
  const digest = createHash("sha256");
  const window = globalThis as unknown as Record<string, unknown>;
  const previous = window.document;
  window.document = {
    createElement(tag: string) {
      assert.equal(tag, "input");
      const input = {
        type: "", accept: "", multiple: true, files: [file], onchange: null as (() => void) | null, oncancel: null as (() => void) | null,
        click() {
          ++selected;
          assert.equal(this.type, "file"); assert.equal(this.accept, "image/png,.png"); assert.equal(this.multiple, false);
          if (row.trigger === "cancel-picker") this.oncancel?.(); else this.onchange?.();
        },
        remove() { ++detached; assert.equal(this.onchange, null); assert.equal(this.oncancel, null); },
      };
      return input;
    },
  };
  type Crossing = ReturnType<typeof import("./original-media-react-wire").originalMediaCrossing>;
  let pending: Crossing | undefined;
  let retry: Crossing | undefined;
  const exchange = createOriginalMediaExchange(request, BigInt(request.loan.instanceId), BigInt(cases.seq), {
    current: () => current && !retired,
    retired: () => retired,
    assertOriginalActivation() { assert(!retired); },
    async serialize(run) {
      assert(!inActor);
      inActor = true;
      try { return await run(); }
      finally { inActor = false; }
    },
    async turn(offer) {
      assert(inActor);
      const body = offer ?? pending;
      assert(body);
      const receipt = (val: Crossing["identity"] & {result: string}) => {
        const normalized = JSON.parse(JSON.stringify({...val, owner: hex(val.owner), seq: hex(val.seq)}));
        assert(law(normalized), JSON.stringify(law.errors));
        return { ingress: normalizeCommandIngress({ kind: 0, originalMediaAck: {tag: "some", val} }) };
      };
      const id = body.identity;
      const kind = id.phase === "page" ? "page#" + id.pageIndex : id.phase;
      const mode = row.trigger;
      assert.equal(id.loan.baseRevision, request.loan.baseRevision);
      assert.equal(id.loan.operationId, request.loan.operationId);
      assert.equal(id.loan.documentId, request.loan.documentId);
      assert.equal(id.loan.generation, request.loan.generation);
      if (offer) phase.push(kind);
      if (!offer) phase.push("poll:" + kind);
      if (mode === "control-deferred" && id.phase === "begin" && offer) {
        pending = offer;
        return { ingress: { tag: "idle" }, deferredEvents: 1 };
      }
      if (mode === "retry-control" && id.phase === "begin") {
        if (!retry) {
          retry = offer;
          return { ingress: { tag: "idle" } };
        }
        assert.equal(offer, retry);
      }
      if (mode === "pending-complete" && id.phase === "complete" && offer) {
        pending = offer;
        return receipt({ ...id, result: "pending" });
      }
      if (id.phase === "page") {
        assert(body.page); assert.deepEqual(body.events, []);
        assert.equal(body.page.cursor.owner, id.owner); assert.equal(body.page.cursor.seq, id.seq);
        assert.equal(body.page.cursor.metadata, file.size); assert.equal(body.page.cursor.itemCount, 1);
        assert.equal(body.page.cursor.kind, request.kind);
        assert.equal(body.page.cursor.generation, BigInt("0x" + request.loan.generation));
        assert.equal(body.page.cursor.commandIndex, 0); assert.equal(body.page.cursor.commandCount, 1);
        assert.equal(body.page.cursor.instance, request.loan.instanceId); assert.equal(body.page.cursor.pageCount, neutral.png.pageLengths.length);
        if (mode === "retry-page" && id.pageIndex === 0 && !retry) {
          retry = offer;
          return receipt({ ...id, result: "retry" });
        }
        if (mode === "retry-page" && id.pageIndex === 0) assert.equal(offer, retry);
        if (mode === "retry-backpressure" && id.pageIndex === 0 && !retry) {
          retry = offer;
          return { ingress: normalizeCommandIngress({ kind: 2, cursor: body.page.cursor }) };
        }
        if (mode === "retry-backpressure" && id.pageIndex === 0) assert.equal(offer, retry);
        if (mode === "pending-page" && id.pageIndex === 0 && offer) {
          pending = offer;
          return receipt({ ...id, result: "pending" });
        }
        if (mode === "wrong-ack" && id.pageIndex === 0) {
          const changed = request.loan.baseRevision.slice(0, 62) + "00";
          assert.notEqual(changed, request.loan.baseRevision);
          return receipt({ ...id, loan: { ...id.loan, baseRevision: changed }, result: "accepted" });
        }
        if (mode === "guest-stale" && id.pageIndex === 1) return receipt({ ...id, result: "stale" });
        const exact = readActorBytePage(body.page.page);
        assert.deepEqual(exact, body.page.bytes);
        assert.equal(exact.length, neutral.png.pageLengths[id.pageIndex!]);
        transferred.push(exact.length);
        digest.update(exact);
      } else {
        assert.equal(body.page, undefined);
        const [event] = body.events;
        assert.equal(event!.kind, "original-media-control");
        const payload = event!.payload as { extent: number; control: { tag: string; val?: unknown } };
        assert.equal(payload.control.tag, id.phase);
        assert.equal(payload.extent, id.phase === "cancel" && phase.length === 1 ? 0 : file.size);
        if (id.phase === "begin") assert.deepEqual(payload.control.val, { name: file.name, mediaType: file.type });
      }
      pending = undefined;
      return receipt({ ...id, result: "accepted" });
    },
    async settle() { assert(inActor); },
    async yieldTurn() { assert(inActor); hostTime += 1; },
    now() { return hostTime; },
  });
  let status;
  try {
    status = await pickOriginalMedia(request, exchange, () => requestFileSelectionV1("image/png,.png"), () => current, () => ({
      signal: controller.signal,
      progress(at, total) { assert.equal(total, file.size); assert(!inActor); progress.push(at); },
      finish() { ++finished; },
    }));
  } catch (error) {
    status = error instanceof Error ? error.message : String(error);
  } finally {
    if (previous === undefined) delete window.document; else window.document = previous;
  }
  assert.equal(status, row.expected);
  assert.deepEqual(phase, row.phase);
  assert.deepEqual(reads, row.reads);
  assert.deepEqual(progress, row.progress);
  assert.equal(selected, 1);
  assert.equal(detached, 1);
  assert.equal(finished, row.trigger === "cancel-picker" ? 0 : 1);
  if (status === "complete") {
    assert.deepEqual(transferred, neutral.png.pageLengths);
    assert.equal(digest.digest("hex"), neutral.png.sha256);
  }
  console.log("[DEBUG] " + row.name + " " + status + " " + phase.join(" → "));
}
