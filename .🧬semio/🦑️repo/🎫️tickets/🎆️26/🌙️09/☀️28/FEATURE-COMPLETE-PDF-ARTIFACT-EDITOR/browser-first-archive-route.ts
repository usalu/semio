/** 🔒 Serves one short-lived first-genesis capture under a server-owned default folder. */
import { createHash, randomBytes } from "node:crypto";
import { mkdir, realpath, rmdir } from "node:fs/promises";
import { isAbsolute, join, resolve } from "node:path";
import { captureBoundBackboneFirstArchive, type BoundBackboneFirstArchive } from "../🗄️backbone-storage/🟦️.ts";
import type { NativeFolderJournal } from "../🗄️backbone-storage/📞️native-journal.ts";
import type { BackboneServerRequest, BackboneServerResponse } from "./🟦️.ts";

const TTL = 300_000;
const SLOTS = 32;
const MAXIMUM = 4 * 1024 * 1024;
const PAGE = 4096;
const TOKEN = /^[0-9a-f]{64}$/u;
const LOAN = /^[0-9a-f]{16}$/u;
type Capture = Awaited<ReturnType<typeof captureBoundBackboneFirstArchive>>;
type Body = (request: BackboneServerRequest, limit: number, complete: (bytes: Uint8Array | null) => void) => void;
type Slot = {
  capability: string; epoch: number; identity: string; fingerprint: string;
  worker: string; workerEpoch: number;
  controller: AbortController; createdFolder?: string; ready: Promise<Capture>; timer: ReturnType<typeof setTimeout>;
};

function refused(status: number, detail: string): Error { return Object.assign(Error(detail), { status }); }
function object(value: unknown, names: readonly string[]): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Object.keys(value).length !== names.length ||
      !names.every((name) => Object.hasOwn(value, name))) throw refused(400, "invalid first archive scope fields");
  return value as Record<string, unknown>;
}
function string(value: unknown, limit: number): string {
  if (typeof value !== "string" || value.length === 0 || Buffer.byteLength(value) > limit || value.includes("\0")) {
    throw refused(400, "invalid first archive scope text");
  }
  return value;
}
function integer(value: unknown, maximum: number): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 1 || value > maximum) throw refused(400, "invalid first archive scope number");
  return value;
}
function uint64(value: unknown, minimum = 0n): bigint {
  if (typeof value !== "string" || !/^(0|[1-9][0-9]{0,19})$/u.test(value)) throw refused(400, "invalid first archive sequence");
  const decoded = BigInt(value);
  if (decoded < minimum || decoded > 0xffffffffffffffffn) throw refused(400, "invalid first archive sequence");
  return decoded;
}
function loan(value: unknown): string {
  const text = string(value, 16);
  if (!LOAN.test(text) || text === "0000000000000000") throw refused(400, "invalid first archive source loan");
  return text;
}
function hex(value: unknown): Uint8Array {
  const text = string(value, 64);
  if (!TOKEN.test(text)) throw refused(400, "invalid first archive digest");
  return Uint8Array.from(Buffer.from(text, "hex"));
}
function reserve(bytes: Uint8Array): BoundBackboneFirstArchive {
  const raw = object(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes)),
    ["binding", "schema", "actor", "clientInstanceId", "ownerEpoch", "source", "frontier"]);
  const binding = object(raw.binding, ["kind", "dataClass", "path"]);
  const frontier = object(raw.frontier, ["documentId", "generation", "revision", "extent"]);
  const source = object(raw.source, ["instanceId", "operationId", "generation", "sequence", "extent", "declaration"]);
  const declaration = object(source.declaration, ["formatKind", "extension", "mediaType", "binary"]);
  if (binding.kind !== "folder" || binding.dataClass !== "persistedLocalOnly" || declaration.binary !== true) {
    throw refused(400, "invalid first archive folder or declaration");
  }
  const path = string(binding.path, 4096);
  if (!isAbsolute(path)) throw refused(400, "invalid first archive folder");
  return {
    binding: { kind: "folder", dataClass: "persistedLocalOnly", path },
    schema: string(raw.schema, 1024), actor: string(raw.actor, 512), clientInstanceId: string(raw.clientInstanceId, 128),
    ownerEpoch: integer(raw.ownerEpoch, Number.MAX_SAFE_INTEGER),
    source: {
      instanceId: integer(source.instanceId, 0xffffffff), operationId: loan(source.operationId), generation: loan(source.generation),
      sequence: uint64(source.sequence, 1n), extent: integer(source.extent, 0xffffffff),
      declaration: {
        formatKind: string(declaration.formatKind, 512), extension: string(declaration.extension, 512),
        mediaType: string(declaration.mediaType, 512), binary: true,
      },
    },
    frontier: {
      documentId: string(frontier.documentId, 512), generation: uint64(frontier.generation), revision: hex(frontier.revision),
      extent: integer(frontier.extent, MAXIMUM),
    },
  };
}

function sameOrigin(request: BackboneServerRequest): boolean {
  const { origin, host, "sec-fetch-site": site } = request.headers ?? {};
  if (typeof origin !== "string" || typeof host !== "string" || site !== "same-origin") return false;
  try {
    const parsed = new URL(origin);
    return parsed.host === host && (parsed.protocol === "http:" || parsed.protocol === "https:");
  } catch { return false; }
}

/** 🛤 Routes scoped reserve, page, publish and retire requests without accepting arbitrary folders. */
export function firstArchiveFolderRoute(base: string, dataRoot: string, journal: NativeFolderJournal, collect: Body) {
  const byDocument = new Map<string, Slot>();
  const byCapability = new Map<string, Slot>();
  const workerEpochs = new Map<string, { epoch: number; expires: number; active: boolean }>();
  let epoch = 0;
  const retire = async (slot: Slot) => {
    if (byCapability.get(slot.capability) !== slot) return;
    slot.controller.abort(refused(409, "first archive authority closed"));
    clearTimeout(slot.timer);
    try {
      const capture = await slot.ready.catch(() => undefined);
      await capture?.retire();
    } finally {
      byCapability.delete(slot.capability);
      if (byDocument.get(slot.identity) === slot) byDocument.delete(slot.identity);
      const owner = workerEpochs.get(slot.worker);
      if (owner?.epoch === slot.workerEpoch) { owner.active = false; owner.expires = Date.now() + TTL; }
      if (slot.createdFolder) {
        await rmdir(join(slot.createdFolder, ".semio")).catch(() => undefined);
        await rmdir(slot.createdFolder).catch(() => undefined);
      }
    }
  };
  const owned = (req: BackboneServerRequest): Slot => {
    const authorization = req.headers?.authorization;
    const epoch = req.headers?.["x-semio-folder-epoch"];
    if (typeof authorization !== "string" || !authorization.startsWith("SemioFirstArchive ") || typeof epoch !== "string") {
      throw refused(403, "first archive capability required");
    }
    const capability = authorization.slice("SemioFirstArchive ".length);
    const slot = TOKEN.test(capability) && byCapability.get(capability);
    if (!slot || String(slot.epoch) !== epoch || slot.controller.signal.aborted) throw refused(409, "first archive capability retired");
    return slot;
  };
  const reply = (res: BackboneServerResponse, status: number, body?: object) => {
    res.statusCode = status;
    res.setHeader("cache-control", "no-store");
    if (body) res.setHeader("content-type", "application/json");
    res.end(body ? JSON.stringify(body) : undefined);
  };
  const discover = async (bytes: Uint8Array) => {
    const scope = reserve(bytes);
    if (!isAbsolute(dataRoot)) throw refused(503, "local document root unavailable");
    const digest = createHash("sha256").update(scope.frontier.documentId).digest("hex");
    const parent = join(resolve(dataRoot), "os", "local-documents");
    const requested = join(parent, digest);
    if (resolve(scope.binding.path) !== requested) throw refused(403, "first archive folder differs from its configured destination");
    const fingerprint = createHash("sha256").update(bytes).digest("hex");
    const retained = byDocument.get(digest);
    if (retained) {
      if (retained.fingerprint !== fingerprint || retained.controller.signal.aborted) throw refused(409, "first archive folder is already scoped");
      await retained.ready;
      return retained;
    }
    if (byDocument.size >= SLOTS || epoch >= Number.MAX_SAFE_INTEGER) throw refused(503, "first archive capacity exhausted");
    const now = Date.now();
    for (const [worker, old] of workerEpochs) if (!old.active && old.expires <= now) workerEpochs.delete(worker);
    const worker = `${scope.actor}\0${scope.clientInstanceId}`;
    const previous = workerEpochs.get(worker);
    if (previous && (previous.active || previous.epoch >= scope.ownerEpoch)) throw refused(409, "first archive worker epoch changed");
    if (!previous && workerEpochs.size >= SLOTS * 4) throw refused(503, "first archive epoch capacity exhausted");
    workerEpochs.set(worker, { epoch: scope.ownerEpoch, expires: now + TTL, active: true });
    const capability = randomBytes(32).toString("hex");
    const controller = new AbortController();
    const localEpoch = ++epoch;
    const ready = Promise.resolve().then(async () => {
      await mkdir(resolve(dataRoot), { recursive: true });
      const data = await realpath(resolve(dataRoot));
      const physicalParent = join(data, "os", "local-documents");
      await mkdir(physicalParent, { recursive: true });
      if (await realpath(physicalParent) !== physicalParent) throw refused(403, "first archive parent changed its physical folder");
      const physical = join(physicalParent, digest);
      controller.signal.throwIfAborted();
      try { await mkdir(physical, { mode: 0o700 }); slot.createdFolder = physical; }
      catch (error) {
        if (error instanceof Error && "code" in error && error.code === "EEXIST") throw refused(409, "first archive destination already exists");
        throw error;
      }
      if (await realpath(physical) !== physical) throw refused(403, "first archive destination changed its physical folder");
      const binding = { ...scope.binding, path: physical };
      return captureBoundBackboneFirstArchive({ ...scope, binding }, () =>
        !controller.signal.aborted && byCapability.get(capability)?.epoch === localEpoch, journal, controller.signal);
    });
    const slot: Slot = {
      capability, epoch: localEpoch, identity: digest, fingerprint, worker, workerEpoch: scope.ownerEpoch,
      controller, ready, timer: setTimeout(() => { void retire(slot).catch(() => undefined); }, TTL),
    };
    slot.timer.unref?.();
    byDocument.set(digest, slot);
    byCapability.set(capability, slot);
    try { await ready; }
    catch (error) {
      if (slot.createdFolder) await retire(slot);
      else {
        clearTimeout(slot.timer);
        byCapability.delete(capability);
        if (byDocument.get(digest) === slot) byDocument.delete(digest);
        if (workerEpochs.get(worker)?.epoch === scope.ownerEpoch) {
          if (previous) workerEpochs.set(worker, previous);
          else workerEpochs.delete(worker);
        }
      }
      throw error;
    }
    return slot;
  };
  return (request: BackboneServerRequest, response: BackboneServerResponse, url: URL): boolean => {
    if (!url.pathname.startsWith(`${base}/`)) return false;
    const action = url.pathname.slice(base.length + 1);
    const type = request.headers?.["content-type"];
    const reserveRequest = action === "reserve" && request.method === "POST" && type === "application/json" && url.searchParams.size === 0;
    const pageRequest = action === "page" && request.method === "PUT" && type === "application/octet-stream" &&
      url.searchParams.size === 1 && /^(0|[1-9][0-9]{0,6})$/u.test(url.searchParams.get("offset") ?? "");
    const publishRequest = action === "publish" && request.method === "POST" && type === "application/octet-stream" && url.searchParams.size === 0;
    const retireRequest = action === "retire" && request.method === "POST" && type === undefined && url.searchParams.size === 0;
    if (!sameOrigin(request)) { reply(response, 403, { error: "same-origin first archive request required" }); return true; }
    if (!reserveRequest && !pageRequest && !publishRequest && !retireRequest) {
      reply(response, 400, { error: "invalid first archive request" });
      return true;
    }
    collect(request, reserveRequest ? 8192 : pageRequest ? PAGE : publishRequest ? 32 : 0, (bytes) => {
      void (async () => {
        if (!bytes) throw refused(413, "first archive request exceeds its page limit");
        if (reserveRequest) {
          const slot = await discover(bytes);
          reply(response, 201, { epoch: slot.epoch, capability: slot.capability });
          return;
        }
        const slot = owned(request);
        if (retireRequest) { await retire(slot); reply(response, 204); return; }
        const capture = await slot.ready;
        if (pageRequest) {
          reply(response, 200, await capture.write(Number(url.searchParams.get("offset")), bytes));
          return;
        }
        if (bytes.length !== 32) throw refused(400, "invalid archive seal");
        const close = () => { void retire(slot).catch(() => undefined); };
        response.on?.("close", close);
        try {
          const result = await capture.publish(bytes, slot.controller.signal);
          response.off?.("close", close);
          reply(response, 200, {
            extent: result.extent, digest: Buffer.from(result.digest).toString("hex"),
            frontier: { ...result.frontier, generation: String(result.frontier.generation), revision: Buffer.from(result.frontier.revision).toString("hex") },
          });
        } finally { response.off?.("close", close); }
      })().catch((error: unknown) => {
        const status = error instanceof Error && "status" in error ? Number(error.status) : 409;
        reply(response, status, { error: "first archive operation was refused" });
      });
    });
    return true;
  };
}
