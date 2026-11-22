/** 🕸️ Drives the staged Vite route over HTTP into the actual native journal companion. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { once } from "node:events";
import { createServer, type IncomingMessage } from "node:http";
import { mkdir, readFile, readdir, rm } from "node:fs/promises";
import { join } from "node:path";
import { createDevFirstArchiveCaptureV1 } from "./browser-first-archive-client.ts";
import { nativeFolderJournal } from "./browser-folder-stdio.ts";
import { eventLogPath, readLatestArchive } from "./browser-first-archive-journal.ts";
import { firstArchiveFolderRoute } from "./🗑️generated/browser-first-archive/proof-route.ts";

const base = "/semio-backbone/first-archive";
const work = join(import.meta.dir, "🗑️generated", "browser-first-archive");
const root = join(work, "http-law");
await rm(root, { recursive: true, force: true });
await mkdir(root, { recursive: true });
const exe = join(work, process.platform === "win32" ? "semio-folder-journal.exe" : "semio-folder-journal");
const native = nativeFolderJournal(exe);
const take = (request: IncomingMessage, maximum: number, complete: (body: Uint8Array | null) => unknown) => {
  const chunks: Buffer[] = [];
  let count = 0;
  request.on("data", (bytes: Buffer) => {
    count += bytes.length;
    if (count <= maximum) chunks.push(bytes);
  });
  request.on("end", () => complete(count <= maximum ? Buffer.concat(chunks) : null));
};
const route = firstArchiveFolderRoute(base, root, native, take);
const server = createServer((req, res) => {
  if (!route(req, res, new URL(req.url!, `http://${req.headers.host}`))) { res.statusCode = 404; res.end(); }
});
server.listen(0, "127.0.0.1");
await once(server, "listening");
const address = server.address();
assert(address && typeof address !== "string");
const origin = `http://127.0.0.1:${address.port}`;
const pack = await readFile(join(import.meta.dir, "🗑️generated", "paged-jpeg-assets", "large.dct-placement.pack"));
const spr = Buffer.from("schema=stdio.pdf.1.7\ndocument=pdf-document");
function varint(value: number) {
  const result: number[] = [];
  for (; value >= 128; value >>= 7) result.push((value & 127) | 128);
  return Buffer.from([...result, value]);
}
const archive = Buffer.concat([Buffer.of(1), varint(pack.length), pack, varint(spr.length), spr, Buffer.of(0)]);
const digest = createHash("sha256").update(archive).digest();
const folder = (id: string) => join(root, "os", "local-documents", createHash("sha256").update(id).digest("hex"));
const doc = "pdf-document";
const scope = (id = doc, ownerEpoch = 1, clientInstanceId = "trusted-browser-worker") => ({
  binding: { kind: "folder", dataClass: "persistedLocalOnly", path: folder(id) },
  actor: "local-actor", clientInstanceId, ownerEpoch, schema: "stdio.pdf.1.7",
  frontier: { documentId: id, generation: "9007199254740993", revision: "07".repeat(32), extent: archive.length },
  source: { instanceId: 42, operationId: "0020000000000002", generation: "0020000000000003", sequence: "9007199254740997",
    extent: 9437877, declaration: { formatKind: "s.stdio.pdf@1.7", extension: ".pdf", mediaType: "application/pdf", binary: true } },
});
const request = (action: string, method: string, body?: Uint8Array | string, headers: Record<string, string> = {}) =>
  fetch(`${origin}${base}/${action}`, { method, body, headers: { origin, "sec-fetch-site": "same-origin", ...headers } });
const reserve = (body: object, headers: Record<string, string> = {}) =>
  request("reserve", "POST", JSON.stringify(body), { "content-type": "application/json", ...headers });
try {
  const original = scope();
  const outside = { ...original, binding: { ...original.binding, path: root } };
  assert.equal((await reserve(outside)).status, 403);
  assert.equal((await reserve(original, { "sec-fetch-site": "cross-site" })).status, 403);
  const reserved = await reserve(original);
  assert.equal(reserved.status, 201);
  const { capability, epoch } = await reserved.json() as { capability: string; epoch: number };
  assert(Number.isSafeInteger(epoch) && epoch > 0 && /^[0-9a-f]{64}$/u.test(capability));
  assert.deepEqual(await (await reserve(original)).json(), { capability, epoch });
  assert.equal((await reserve(scope(doc, 2))).status, 409);
  assert.equal((await reserve(scope("parallel", 2))).status, 409);
  const control = { authorization: `SemioFirstArchive ${capability}`, "x-semio-folder-epoch": String(epoch) };
  const binary = { ...control, "content-type": "application/octet-stream" };
  assert.equal((await request("page?offset=0", "PUT", archive.subarray(0, 4096), { ...binary, "x-semio-folder-epoch": (epoch + 1).toString() })).status, 409);
  assert.equal((await request("page?offset=0", "PUT", archive.subarray(0, 4097), binary)).status, 413);
  assert.equal((await request("retire", "POST", undefined, control)).status, 204);
  assert.equal((await reserve(scope("stale", 1))).status, 409);

  const browserFetch = ((input: string | URL | Request, init?: RequestInit) => {
    const headers = new Headers(init?.headers);
    headers.set("origin", origin);
    headers.set("sec-fetch-site", "same-origin");
    return fetch(new URL(String(input), origin), { ...init, headers });
  }) as typeof fetch;
  const captureArchive = createDevFirstArchiveCaptureV1(browserFetch);
  const signal = new AbortController().signal;
  const { schema, actor, binding, frontier, source } = original;
  const captureRequest = {
    schema, actor,
    binding: { ...binding, kind: "folder", dataClass: "persistedLocalOnly" } as const,
    frontier: { ...frontier, generation: BigInt(frontier.generation), revision: Uint8Array.from(Buffer.from(frontier.revision, "hex")) },
    source: { ...source, sequence: BigInt(source.sequence) },
  };
  const archiveCapture = await captureArchive(captureRequest, signal);
  for (let offset = 0; offset < archive.length; offset += 4096) {
    const page = archive.subarray(offset, offset + 4096);
    assert.deepEqual(await archiveCapture.write(offset, page), { offset, length: page.length });
    if (offset === 0) assert.deepEqual(await archiveCapture.write(offset, page), { offset, length: page.length });
  }
  const published = await archiveCapture.publish(digest, signal);
  assert.deepEqual(published, { extent: archive.length, digest: Uint8Array.from(digest), frontier: captureRequest.frontier });
  await archiveCapture.retire();
  await archiveCapture.retire();
  assert.equal((await request("publish", "POST", digest, binary)).status, 409);
  assert.deepEqual(await readLatestArchive(eventLogPath(folder(doc)), doc), Uint8Array.from(archive));
  assert.deepEqual(await readdir(join(folder(doc), ".semio")), ["events.semio"]);
  const another = await reserve(scope("abandoned", 2));
  assert.equal(another.status, 201);
  const next = await another.json() as { capability: string; epoch: number };
  assert.equal((await request("retire", "POST", undefined, { authorization: `SemioFirstArchive ${next.capability}`, "x-semio-folder-epoch": String(next.epoch) })).status, 204);
  assert.deepEqual(await readdir(join(root, "os", "local-documents")), [folder(doc).split(/[\\/]/u).at(-1)]);
  await Bun.write(join(work, "http-record.sha256"), digest.toString("hex"));
} finally {
  server.closeAllConnections();
  await new Promise<void>((resolve) => server.close(() => resolve()));
}
console.log(`[DEBUG] Scoped Vite HTTP published and reopened ${archive.length} bytes through the locked native writer.`);
