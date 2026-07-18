#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  BundleScript,
  ScriptRouter,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  probeWgpuDevPort,
  stopTrunkDevPort,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runVitest,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
} from "../../../../repo/lib/js/index.ts";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, backboneKindFromUri } from "@semio-tech/framework-os-core";
import { generatePluginRegistry, isStudioPluginFilter, type PluginRegistryEntry } from "../../../plugin/registry/script.ts";

const repoRoot = getWorkspaceRoot();
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";

//#region 🔖PlaygroundVariantResolution
/** @emoji 📚 Generated playground catalog (variant -> crate pluginId + optional app id), loaded once for this process via `@semio-tech/repo-lib`'s `loadFrameworkOsPlaygroundCatalog` (backed by `framework/plugin/registry/generated/playgrounds.ts`). */
const playgroundCatalog = loadFrameworkOsPlaygroundCatalog();

/** @emoji 🧭 A resolved playground filter: the crate pluginId to build/load, plus the app id to inject when the filter matched a catalog variant row. */
type ResolvedPlaygroundFilter = {
  readonly pluginId: string;
  readonly appId?: string;
};

/**
 * 🧭 Resolves `filterPlugin` (a playground variant id like "puzzle5d", or already a bare crate
 * pluginId like "note") against the generated playground catalog: a matching variant row yields
 * its crate pluginId and app id, otherwise `filterPlugin` is treated as already being a bare
 * pluginId (existing behavior for single-app crates where variant === pluginId).
 */
function resolvePlaygroundFilter(filterPlugin: string): ResolvedPlaygroundFilter {
  const row = playgroundCatalog.find((entry) => entry.variant === filterPlugin);
  return row ? { pluginId: row.pluginId, appId: row.app } : { pluginId: filterPlugin };
}

/** @emoji 🎯 Resolves a raw filter to the crate pluginId `generatePluginRegistry`'s `filterPlaygroundPlugin` option expects, or `undefined` for the unfiltered/studio case. */
function resolveCatalogFilterPluginId(filterPlugin?: string): string | undefined {
  return filterPlugin && !isStudioPluginFilter(filterPlugin) ? resolvePlaygroundFilter(filterPlugin).pluginId : undefined;
}
//#endregion 🔖PlaygroundVariantResolution

//#region BackboneVitePlugin
/** Lazily imports `bun:sqlite` — a static top-level import breaks Vite's config bundler, which loads this module's exports under Node before the dev server (and its Bun runtime) exists. */
let backboneDatabaseCtor: typeof import("bun:sqlite").Database | undefined;
async function backboneDatabaseCtorLazy(): Promise<typeof import("bun:sqlite").Database> {
  if (!backboneDatabaseCtor) ({ Database: backboneDatabaseCtor } = await import("bun:sqlite"));
  return backboneDatabaseCtor;
}

/** @emoji 🗂️ Same convention as `vcs::FolderSqliteStorage` (`.semio/documents.db`, a `document(id,
 * schema, json, updated_at)` table) so a folder-bound studio opened by the browser dev path and a
 * native (wgpu) reader agree on the same file. `documentId` defaults to the studio's own
 * single-document convention (mirrors os-core's `STUDIO_FOLDER_DOCUMENT_ID`) when the caller doesn't
 * pass one — app documents (per `OsDocumentRef`) always pass their own id explicitly. */
const STUDIO_FOLDER_DOCUMENT_ID = "studio";

async function readBackbonePayload(uri: string, documentId: string | null): Promise<string | null> {
  const kind = backboneKindFromUri(uri);
  if (kind === "file") {
    const path = uri.slice("file://".length);
    if (!existsSync(path)) return null;
    return readFileSync(path, "utf8");
  }
  if (kind === "folder") {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "documents.db");
    if (!existsSync(dbPath)) return null;
    const Database = await backboneDatabaseCtorLazy();
    const db = new Database(dbPath);
    db.run("CREATE TABLE IF NOT EXISTS document (id TEXT PRIMARY KEY, schema TEXT, json TEXT NOT NULL, updated_at INTEGER NOT NULL)");
    const row = db.query("SELECT json FROM document WHERE id = ?1").get(documentId ?? STUDIO_FOLDER_DOCUMENT_ID) as { json?: string } | null;
    return row?.json ?? null;
  }
  return null;
}

async function writeBackbonePayload(uri: string, documentId: string | null, schema: string | null, payload: string): Promise<void> {
  const kind = backboneKindFromUri(uri);
  if (kind === "file") {
    const path = uri.slice("file://".length);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, payload);
    return;
  }
  if (kind === "folder") {
    const folder = uri.slice("folder://".length);
    const dbPath = join(folder, ".semio", "documents.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    const Database = await backboneDatabaseCtorLazy();
    const db = new Database(dbPath);
    db.run("CREATE TABLE IF NOT EXISTS document (id TEXT PRIMARY KEY, schema TEXT, json TEXT NOT NULL, updated_at INTEGER NOT NULL)");
    db.run("INSERT INTO document (id, schema, json, updated_at) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(id) DO UPDATE SET schema = excluded.schema, json = excluded.json, updated_at = excluded.updated_at", [
      documentId ?? STUDIO_FOLDER_DOCUMENT_ID,
      schema ?? "",
      payload,
      Date.now(),
    ]);
    return;
  }
  throw new Error(`unsupported backbone uri: ${uri}`);
}

/** 👁️ Per-folder-uri debounced watchers feeding every subscribed SSE response for that uri — one
 * `node:fs.watch` per folder regardless of subscriber count. Mirrors `framework/sync`'s native
 * `notify` watcher (200ms debounce) so both the dev-browser and native paths agree on cadence. */
const folderWatchSubscribers = new Map<string, Set<{ write: (chunk: string) => void }>>();
const folderWatchHandles = new Map<string, ReturnType<typeof watch>>();
const FOLDER_WATCH_DEBOUNCE_MS = 200;

function subscribeFolderWatch(uri: string, subscriber: { write: (chunk: string) => void }): () => void {
  if (!folderWatchSubscribers.has(uri)) folderWatchSubscribers.set(uri, new Set());
  const subscribers = folderWatchSubscribers.get(uri)!;
  subscribers.add(subscriber);
  if (!folderWatchHandles.has(uri) && backboneKindFromUri(uri) === "folder") {
    const folder = uri.slice("folder://".length);
    mkdirSync(join(folder, ".semio"), { recursive: true });
    let debounceTimer: ReturnType<typeof setTimeout> | undefined;
    const handle = watch(join(folder, ".semio"), { persistent: false }, () => {
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        for (const sub of folderWatchSubscribers.get(uri) ?? []) sub.write("data: changed\n\n");
      }, FOLDER_WATCH_DEBOUNCE_MS);
    });
    folderWatchHandles.set(uri, handle);
  }
  return () => {
    subscribers.delete(subscriber);
    if (subscribers.size === 0) {
      folderWatchHandles.get(uri)?.close();
      folderWatchHandles.delete(uri);
      folderWatchSubscribers.delete(uri);
    }
  };
}

type BackboneServerRequest = { method?: string; url?: string; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BackboneServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; write: (chunk: string) => void; end: (body?: string) => void };

/** @emoji 💾 Vite middleware for browser file/folder backbone IO: `GET|PUT ${BACKBONE_ENDPOINT_PATH}?uri=&documentId=&schema=`
 * for read/write, plus `GET ${BACKBONE_ENDPOINT_PATH}/watch?uri=` (SSE) for external-edit notification —
 * `backbone-worker.ts`'s folder transport degrades to polling if this endpoint isn't reachable. */
export function semioBackboneVitePlugin() {
  return {
    name: "semio-backbone",
    configureServer(server: { middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BACKBONE_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        const uri = requestUrl.searchParams.get("uri");
        if (!uri) {
          res.statusCode = 400;
          res.end("missing uri");
          return;
        }
        if (requestUrl.pathname === `${BACKBONE_ENDPOINT_PATH}/watch`) {
          if (req.method !== "GET") {
            res.statusCode = 405;
            res.end("method not allowed");
            return;
          }
          res.statusCode = 200;
          res.setHeader("content-type", "text/event-stream");
          res.setHeader("cache-control", "no-cache");
          res.setHeader("connection", "keep-alive");
          res.write(": connected\n\n");
          const unsubscribe = subscribeFolderWatch(uri, res);
          req.on("close", unsubscribe);
          return;
        }
        const documentId = requestUrl.searchParams.get("documentId");
        const schema = requestUrl.searchParams.get("schema");
        if (req.method === "GET") {
          readBackbonePayload(uri, documentId)
            .then((payload) => {
              if (payload == null) {
                res.statusCode = 404;
                res.end("");
                return;
              }
              res.statusCode = 200;
              res.setHeader("content-type", "application/json");
              res.end(payload);
            })
            .catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          return;
        }
        if (req.method === "PUT") {
          let body = "";
          req.on("data", (chunk) => {
            body += String(chunk);
          });
          req.on("end", () => {
            writeBackbonePayload(uri, documentId, schema, body)
              .then(() => {
                res.statusCode = 200;
                res.setHeader("content-type", "application/json");
                res.end("{}");
              })
              .catch((error) => {
                res.statusCode = 500;
                res.end(String(error));
              });
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BackboneVitePlugin

//#region 🔖Blake3
/** 🧬 Self-contained BLAKE3 (default 32-byte hash mode, no key/context) so the dev-only blob endpoint
 * hashes bytes exactly like `framework/hash/rs`'s `hash_bytes` (Rust `blake3` crate) does natively —
 * content-addressing must agree across the browser dev path and the native/hub `BlobStore`s. No npm
 * `blake3` package exists to lean on (native bindings, and this shared workspace's `bun add` currently
 * can't resolve at all — an unrelated in-progress workspace member), and Bun's built-in `CryptoHasher`
 * doesn't support the algorithm, so this ports the reference tree-hash construction directly from the
 * BLAKE3 spec (https://github.com/BLAKE3-team/BLAKE3/blob/master/reference_impl/reference_impl.rs).
 * Verified byte-for-byte against `semio-framework-hash::hash_bytes` across chunk-boundary inputs
 * (0/1/1023/1024/1025/2048/2049/100000 bytes) before landing — see ticket scratch notes. */
const BLAKE3_IV = new Uint32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
const BLAKE3_MSG_PERMUTATION = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
const BLAKE3_CHUNK_START = 1;
const BLAKE3_CHUNK_END = 2;
const BLAKE3_PARENT = 4;
const BLAKE3_ROOT = 8;
const BLAKE3_CHUNK_LEN = 1024;
const BLAKE3_BLOCK_LEN = 64;
const BLAKE3_OUT_LEN = 32;

function blake3Rotr(x: number, n: number): number {
  return ((x >>> n) | (x << (32 - n))) >>> 0;
}

function blake3G(state: Uint32Array, a: number, b: number, c: number, d: number, mx: number, my: number): void {
  state[a] = (state[a]! + state[b]! + mx) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 16);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 12);
  state[a] = (state[a]! + state[b]! + my) >>> 0;
  state[d] = blake3Rotr(state[d]! ^ state[a]!, 8);
  state[c] = (state[c]! + state[d]!) >>> 0;
  state[b] = blake3Rotr(state[b]! ^ state[c]!, 7);
}

function blake3RoundFn(state: Uint32Array, m: Uint32Array): void {
  blake3G(state, 0, 4, 8, 12, m[0]!, m[1]!);
  blake3G(state, 1, 5, 9, 13, m[2]!, m[3]!);
  blake3G(state, 2, 6, 10, 14, m[4]!, m[5]!);
  blake3G(state, 3, 7, 11, 15, m[6]!, m[7]!);
  blake3G(state, 0, 5, 10, 15, m[8]!, m[9]!);
  blake3G(state, 1, 6, 11, 12, m[10]!, m[11]!);
  blake3G(state, 2, 7, 8, 13, m[12]!, m[13]!);
  blake3G(state, 3, 4, 9, 14, m[14]!, m[15]!);
}

function blake3Permute(m: Uint32Array): Uint32Array {
  const out = new Uint32Array(16);
  for (let i = 0; i < 16; i++) out[i] = m[BLAKE3_MSG_PERMUTATION[i]!]!;
  return out;
}

function blake3Compress(cv: Uint32Array, block: Uint32Array, counter: number, blockLen: number, flags: number): Uint32Array {
  const state = new Uint32Array(16);
  state.set(cv, 0);
  state.set(BLAKE3_IV.subarray(0, 4), 8);
  state[12] = counter >>> 0;
  state[13] = Math.floor(counter / 2 ** 32) >>> 0;
  state[14] = blockLen;
  state[15] = flags;
  let m = block;
  for (let r = 0; r < 7; r++) {
    blake3RoundFn(state, m);
    if (r < 6) m = blake3Permute(m);
  }
  for (let i = 0; i < 8; i++) {
    state[i] = (state[i]! ^ state[i + 8]!) >>> 0;
    state[i + 8] = (state[i + 8]! ^ cv[i]!) >>> 0;
  }
  return state;
}

function blake3WordsFromBytes(bytes: Uint8Array, offset: number): Uint32Array {
  const words = new Uint32Array(16);
  for (let i = 0; i < 16; i++) {
    const o = offset + i * 4;
    words[i] = (bytes[o]! | (bytes[o + 1]! << 8) | (bytes[o + 2]! << 16) | (bytes[o + 3]! << 24)) >>> 0;
  }
  return words;
}

type Blake3ChunkOutput = { inputCv: Uint32Array; block: Uint32Array; counter: number; blockLen: number; flags: number };

function blake3OutputChainingValue(output: Blake3ChunkOutput): Uint32Array {
  return blake3Compress(output.inputCv, output.block, output.counter, output.blockLen, output.flags).subarray(0, 8);
}

function blake3RootOutputBytes(output: Blake3ChunkOutput, outLen: number): Uint8Array {
  const out = new Uint8Array(outLen);
  let outputBlockCounter = 0;
  let written = 0;
  while (written < outLen) {
    const words = blake3Compress(output.inputCv, output.block, outputBlockCounter, output.blockLen, output.flags | BLAKE3_ROOT);
    for (let i = 0; i < 16 && written < outLen; i++) {
      const w = words[i]!;
      out[written++] = w & 0xff;
      if (written < outLen) out[written++] = (w >>> 8) & 0xff;
      if (written < outLen) out[written++] = (w >>> 16) & 0xff;
      if (written < outLen) out[written++] = (w >>> 24) & 0xff;
    }
    outputBlockCounter++;
  }
  return out;
}

class Blake3ChunkState {
  cv: Uint32Array;
  chunkCounter: number;
  block = new Uint8Array(BLAKE3_BLOCK_LEN);
  blockLen = 0;
  blocksCompressed = 0;
  flags: number;

  constructor(key: Uint32Array, chunkCounter: number, flags: number) {
    this.cv = key.slice();
    this.chunkCounter = chunkCounter;
    this.flags = flags;
  }

  len(): number {
    return this.blocksCompressed * BLAKE3_BLOCK_LEN + this.blockLen;
  }

  startFlag(): number {
    return this.blocksCompressed === 0 ? BLAKE3_CHUNK_START : 0;
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.blockLen === BLAKE3_BLOCK_LEN) {
        const words = blake3WordsFromBytes(this.block, 0);
        this.cv = blake3Compress(this.cv, words, this.chunkCounter, BLAKE3_BLOCK_LEN, this.flags | this.startFlag()).subarray(0, 8);
        this.blocksCompressed++;
        this.block = new Uint8Array(BLAKE3_BLOCK_LEN);
        this.blockLen = 0;
      }
      const take = Math.min(BLAKE3_BLOCK_LEN - this.blockLen, input.length - offset);
      this.block.set(input.subarray(offset, offset + take), this.blockLen);
      this.blockLen += take;
      offset += take;
    }
  }

  output(): Blake3ChunkOutput {
    const words = blake3WordsFromBytes(this.block, 0);
    return { inputCv: this.cv, block: words, counter: this.chunkCounter, blockLen: this.blockLen, flags: this.flags | this.startFlag() | BLAKE3_CHUNK_END };
  }
}

function blake3ParentOutput(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Blake3ChunkOutput {
  const block = new Uint32Array(16);
  block.set(leftCv, 0);
  block.set(rightCv, 8);
  return { inputCv: key, block, counter: 0, blockLen: BLAKE3_BLOCK_LEN, flags: flags | BLAKE3_PARENT };
}

function blake3ParentCv(leftCv: Uint32Array, rightCv: Uint32Array, key: Uint32Array, flags: number): Uint32Array {
  return blake3OutputChainingValue(blake3ParentOutput(leftCv, rightCv, key, flags));
}

/** 🧮 Streaming hasher: chunk (1024B) → 16 blocks (64B) chained, chunks merged pairwise into a binary
 * Merkle tree via a "trailing-zero-bits" stack (a Merkle-mountain-range), root-finalized on `digest`. */
class Blake3Hasher {
  private key = BLAKE3_IV;
  private chunkState = new Blake3ChunkState(BLAKE3_IV, 0, 0);
  private cvStack: Uint32Array[] = [];
  private flags = 0;

  private addChunkChainingValue(newCvIn: Uint32Array, totalChunksIn: number): void {
    let newCv = newCvIn;
    let totalChunks = totalChunksIn;
    while ((totalChunks & 1) === 0) {
      const left = this.cvStack.pop()!;
      newCv = blake3ParentCv(left, newCv, this.key, this.flags);
      totalChunks >>>= 1;
    }
    this.cvStack.push(newCv);
  }

  update(input: Uint8Array): void {
    let offset = 0;
    while (offset < input.length) {
      if (this.chunkState.len() === BLAKE3_CHUNK_LEN) {
        const chunkCv = blake3OutputChainingValue(this.chunkState.output());
        const totalChunks = this.chunkState.chunkCounter + 1;
        this.addChunkChainingValue(chunkCv, totalChunks);
        this.chunkState = new Blake3ChunkState(this.key, totalChunks, this.flags);
      }
      const take = Math.min(BLAKE3_CHUNK_LEN - this.chunkState.len(), input.length - offset);
      this.chunkState.update(input.subarray(offset, offset + take));
      offset += take;
    }
  }

  digest(outLen = BLAKE3_OUT_LEN): Uint8Array {
    let output = this.chunkState.output();
    let parentNodesRemaining = this.cvStack.length;
    while (parentNodesRemaining > 0) {
      parentNodesRemaining--;
      output = blake3ParentOutput(this.cvStack[parentNodesRemaining]!, blake3OutputChainingValue(output), this.key, this.flags);
    }
    return blake3RootOutputBytes(output, outLen);
  }
}

/** 🔗 Hex-encoded BLAKE3 hash of `bytes`, matching `semio_framework_hash::hash_bytes`'s output format. */
function blake3Hex(bytes: Uint8Array): string {
  const hasher = new Blake3Hasher();
  hasher.update(bytes);
  return Buffer.from(hasher.digest()).toString("hex");
}
//#endregion 🔖Blake3

//#region BlobVitePlugin
let blobDatabaseSingleton: InstanceType<typeof import("bun:sqlite").Database> | undefined;

/** 🗄️ Lazily opens the dev-session-wide content-addressed blob store at `<repoRoot>/.semio/blobs.db` —
 * unlike backbone documents, blobs aren't scoped to a per-uri folder (there's no folder in the
 * `write-blob`/`read-blob` WIT signature), so this is one shared table for the whole dev server. */
async function blobDatabase(): Promise<InstanceType<typeof import("bun:sqlite").Database>> {
  if (!blobDatabaseSingleton) {
    const Database = await backboneDatabaseCtorLazy();
    const dbPath = join(repoRoot, ".semio", "blobs.db");
    mkdirSync(dirname(dbPath), { recursive: true });
    blobDatabaseSingleton = new Database(dbPath);
    blobDatabaseSingleton.run("CREATE TABLE IF NOT EXISTS blob (hash TEXT PRIMARY KEY, media_type TEXT NOT NULL, size INTEGER NOT NULL, bytes BLOB NOT NULL)");
  }
  return blobDatabaseSingleton;
}

type BlobServerRequest = { method?: string; url?: string; on: (event: string, handler: (chunk?: unknown) => void) => void };
type BlobServerResponse = { statusCode: number; setHeader: (name: string, value: string) => void; end: (body?: string | Buffer) => void };

/** @emoji 📦 Vite middleware for the dev-only content-addressed blob store: `PUT ${BLOB_ENDPOINT_PATH}?mediaType=`
 * (raw bytes body, BLAKE3-hashed above, returns `{"hash":...}`, idempotent via `INSERT OR IGNORE`) and
 * `GET ${BLOB_ENDPOINT_PATH}/:hash` (raw bytes response, 404 if absent). The browser host-shim's
 * `writeBlob`/`readBlob` (see `hostShimSource`) and `backbone-worker.ts`'s IndexedDB cache both talk to
 * this. Mirrors `vcs::FolderSqliteStorage`'s `blobs(hash, media_type, size, bytes)` table/shape. */
export function semioBlobVitePlugin() {
  return {
    name: "semio-blob",
    configureServer(server: { middlewares: { use: (handler: (req: BlobServerRequest, res: BlobServerResponse, next: () => void) => void) => void } }) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith(BLOB_ENDPOINT_PATH)) return next();
        const requestUrl = new URL(req.url, "http://127.0.0.1");
        if (req.method === "PUT") {
          const chunks: Buffer[] = [];
          req.on("data", (chunk) => {
            chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk as ArrayBuffer));
          });
          req.on("end", () => {
            void (async () => {
              const bytes = Buffer.concat(chunks);
              const mediaType = requestUrl.searchParams.get("mediaType") ?? "application/octet-stream";
              const hash = blake3Hex(new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength));
              const db = await blobDatabase();
              db.run("INSERT OR IGNORE INTO blob (hash, media_type, size, bytes) VALUES (?1, ?2, ?3, ?4)", [hash, mediaType, bytes.length, bytes]);
              res.statusCode = 200;
              res.setHeader("content-type", "application/json");
              res.end(JSON.stringify({ hash }));
            })().catch((error) => {
              res.statusCode = 500;
              res.end(String(error));
            });
          });
          return;
        }
        if (req.method === "GET") {
          const hash = requestUrl.pathname.slice(`${BLOB_ENDPOINT_PATH}/`.length);
          if (!hash) {
            res.statusCode = 400;
            res.end("missing hash");
            return;
          }
          void (async () => {
            const db = await blobDatabase();
            const row = db.query("SELECT media_type, bytes FROM blob WHERE hash = ?1").get(hash) as { media_type?: string; bytes?: Uint8Array } | null;
            if (!row) {
              res.statusCode = 404;
              res.end("");
              return;
            }
            res.statusCode = 200;
            res.setHeader("content-type", row.media_type ?? "application/octet-stream");
            res.end(Buffer.from(row.bytes ?? new Uint8Array()));
          })().catch((error) => {
            res.statusCode = 500;
            res.end(String(error));
          });
          return;
        }
        res.statusCode = 405;
        res.end("method not allowed");
      });
    },
  };
}
//#endregion BlobVitePlugin

function pluginWorkerSource(): string {
  return `/** @generated semio plugin web worker */
let pluginApi = null;

async function loadPlugin(moduleUrl) {
  if (pluginApi) return pluginApi;
  const module = await import(moduleUrl);
  if (module.createPluginApi) {
    pluginApi = await module.createPluginApi();
    return pluginApi;
  }
  throw new Error("plugin module missing createPluginApi export");
}

function reply(requestId, type, payload) {
  self.postMessage({ requestId, type, ...payload });
}

function replyError(requestId, message) {
  self.postMessage({ requestId, type: "error", message });
}

self.addEventListener("message", async (event) => {
  const msg = event.data ?? {};
  const { type, requestId } = msg;
  // 🔗 Backbone relay passthrough (main thread ⇄ host-shim): inbound messages from the sync actor
  // (\`backbone-worker.ts\`) land in the shared queue the host-shim's \`backbonePoll\` drains; the shim's
  // \`backboneSend\` posts \`backboneOutbound\` straight up to the main thread, so there is nothing to do
  // for it here. These carry no requestId, so they must be handled before the request/response guard.
  if (type === "backboneInbound") {
    const queues = (globalThis.__semioBackboneInbound ??= new Map());
    const queue = queues.get(msg.uri) ?? [];
    for (const message of msg.messages ?? []) queue.push(message);
    queues.set(msg.uri, queue);
    return;
  }
  if (!requestId || !type) return;
  try {
    if (type === "init") {
      await loadPlugin(msg.moduleUrl);
      reply(requestId, "init", { ok: true });
      return;
    }
    const api = pluginApi;
    if (!api) throw new Error("worker not initialized");
    switch (type) {
      case "manifest":
        reply(requestId, "manifest", { value: await api.manifest() });
        break;
      case "createApp":
        reply(requestId, "createApp", { instanceId: await api.createApp(msg.appId) });
        break;
      case "destroy":
        await api.destroyApp?.(msg.instanceId);
        reply(requestId, "destroy", { ok: true });
        break;
      case "handleAction":
        reply(requestId, "handleAction", {
          value: await api.handleAction(msg.instanceId, msg.actionJson, msg.contextJson ?? msg.viewStateJson),
        });
        break;
      case "handleCommand":
        reply(requestId, "handleCommand", {
          value: await api.handleCommand(msg.instanceId, msg.commandJson, msg.contextJson ?? msg.viewStateJson),
        });
        break;
      case "render":
        reply(requestId, "render", {
          value: msg.documentJson && api.renderWithDocument
            ? await api.renderWithDocument(msg.instanceId, msg.bodyKey, msg.viewStateJson, msg.documentJson)
            : await api.render(msg.instanceId, msg.bodyKey, msg.viewStateJson),
        });
        break;
      case "refreshUi":
        reply(requestId, "refreshUi", {
          value: await api.refreshUi(msg.instanceId, msg.requestJson),
        });
        break;
      default:
        throw new Error(\`unknown worker message type: \${type}\`);
    }
  } catch (error) {
    replyError(requestId, error instanceof Error ? error.message : String(error));
  }
});
`;
}

function pluginComponentBridgeSource(componentBase: string, wasmFileName: string): string {
  return `/** @generated semio plugin jco component bridge */
import { plugin } from "./${componentBase}.js";

const apps = new Set();
let tail = Promise.resolve();
let pluginApiPromise = null;

function runSerialized(fn) {
  const job = tail.then(async () => {
    for (let attempt = 0; attempt < 8; attempt += 1) {
      try {
        return await fn();
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        if (!message.includes("plugin instance busy") && !message.includes("plugin busy")) throw error;
        await new Promise((resolve) => setTimeout(resolve, attempt + 1));
      }
    }
    return fn();
  }, async () => fn());
  tail = job.then(
    () => undefined,
    () => undefined,
  );
  return job;
}

async function createPluginApiInner() {
  const core = {
    async manifest() {
      return (await plugin.manifest()).json;
    },
    async createApp(appId) {
      const instanceId = await plugin.instantiateApp(appId, appId);
      apps.add(instanceId);
      return instanceId;
    },
    async destroyApp(instanceId) {
      apps.delete(instanceId);
    },
    async handleAction(instanceId, actionJson, contextJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleAction(instanceId, { json: actionJson }, { json: context });
      return response.json;
    },
    async handleCommand(instanceId, commandJson, contextJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const context =
        contextJson && contextJson.trim().startsWith("{")
          ? contextJson
          : JSON.stringify({ viewState: JSON.parse(contextJson), actor: "local" });
      const response = await plugin.handleCommand(instanceId, { json: commandJson }, { json: context });
      return response.json;
    },
    async render(instanceId, bodyKey, viewStateJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson) }),
      });
      return response.json;
    },
    async renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const response = await plugin.updateWindow(instanceId, {
        json: JSON.stringify({ bodyKey, viewState: JSON.parse(viewStateJson), documentJson }),
      });
      return response.json;
    },
    async refreshUi(instanceId, requestJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const response = await plugin.refreshUi(instanceId, { json: requestJson });
      return response.json;
    },
  };
  return {
    manifest: () => runSerialized(() => core.manifest()),
    createApp: (appId) => runSerialized(() => core.createApp(appId)),
    destroyApp: (instanceId) => runSerialized(() => core.destroyApp(instanceId)),
    handleAction: (instanceId, actionJson, contextJson) =>
      runSerialized(() => core.handleAction(instanceId, actionJson, contextJson)),
    handleCommand: (instanceId, commandJson, contextJson) =>
      runSerialized(() => core.handleCommand(instanceId, commandJson, contextJson)),
    render: (instanceId, bodyKey, viewStateJson) =>
      runSerialized(() => core.render(instanceId, bodyKey, viewStateJson)),
    renderWithDocument: (instanceId, bodyKey, viewStateJson, documentJson) =>
      runSerialized(() => core.renderWithDocument(instanceId, bodyKey, viewStateJson, documentJson)),
    refreshUi: (instanceId, requestJson) => runSerialized(() => core.refreshUi(instanceId, requestJson)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
`;
}

function ensureWasmTarget(): void {
  const probe = spawnSync("rustup", ["target", "list", "--installed"], { encoding: "utf8" });
  if (!probe.stdout?.includes(PLUGIN_WASM_TARGET)) {
    spawnSync("rustup", ["target", "add", PLUGIN_WASM_TARGET], { stdio: "inherit" });
  }
}

function preview2ShimVendorDir(): string {
  return join(pluginOutRoot, "_vendor/@bytecodealliance/preview2-shim");
}

function ensurePreview2ShimVendor(): void {
  const sourceDir = join(repoRoot, "node_modules/@bytecodealliance/preview2-shim/lib/browser");
  const targetDir = preview2ShimVendorDir();
  if (!existsSync(sourceDir)) throw new Error("missing @bytecodealliance/preview2-shim browser shims; run bun install");
  mkdirSync(targetDir, { recursive: true });
  for (const entry of readdirSync(sourceDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith(".js")) continue;
    copyFileSync(join(sourceDir, entry.name), join(targetDir, entry.name));
  }
}

function rewritePreview2ShimImports(componentJsPath: string): void {
  const outDir = dirname(componentJsPath);
  const rel = relative(outDir, preview2ShimVendorDir()).replace(/\\/g, "/");
  const prefix = rel.endsWith("/") ? rel : `${rel}/`;
  let content = readFileSync(componentJsPath, "utf8");
  const bareSpecifier = /(from\s+['"])@bytecodealliance\/preview2-shim\/([\w-]+)(['"])/g;
  if (!bareSpecifier.test(content)) return;
  content = content.replace(bareSpecifier, (_match, lead, subpath, trail) => `${lead}${prefix}${subpath}.js${trail}`);
  writeFileSync(componentJsPath, content);
}

function rewriteExistingPluginShimImports(): void {
  if (!existsSync(pluginOutRoot)) return;
  for (const entry of readdirSync(pluginOutRoot, { withFileTypes: true })) {
    if (!entry.isDirectory() || entry.name.startsWith("_")) continue;
    for (const file of readdirSync(join(pluginOutRoot, entry.name))) {
      if (!file.endsWith("_component.js")) continue;
      rewritePreview2ShimImports(join(pluginOutRoot, entry.name, file));
    }
  }
}

function transpilePluginComponent(artifact: string, outDir: string, componentBase: string): void {
  const transpile = spawnSync("bunx", ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/host=./host-shim.js"], { cwd: repoRoot, stdio: "inherit" });
  if (transpile.status !== 0) throw new Error(`jco transpile failed for ${artifact}`);
  rewritePreview2ShimImports(join(outDir, `${componentBase}.js`));
}

/** @emoji 🗄️ JS implementation of the `semio:framework/host` component import. The backbone imports are
 * a pure in-memory queue exchange: `backbone-send` posts an outbound message up to the plugin worker's
 * parent (the main thread) which relays it into `backbone-worker.ts` (the real sync actor), and
 * `backbone-poll` drains an inbound queue the worker fills from `backboneInbound` postMessages. A
 * WASI-P2 plugin worker never owns a socket/fetch itself (WS-B design), so there is no localStorage or
 * synchronous XHR here anymore. */
function hostShimSource(): string {
  return `/** @generated semio plugin host shim */

export function log(level, message) {
  if (level === "error") console.error(\`[plugin] \${message}\`);
  else console.log(\`[plugin] \${message}\`);
}

export function nowMs() {
  return BigInt(Date.now());
}

export function readDocument(handle) {
  throw \`read-document unsupported: \${handle}\`;
}

export function writeDocument(handle, payloadJson) {
  throw \`write-document unsupported: \${handle}\`;
}

export function openWindow(kind, paramsJson) {
  throw \`open-window unsupported: \${kind}\`;
}

export function invokeAction(target, invocationJson) {
  throw \`invoke-action unsupported: \${target}\`;
}

export function readAsset(handle) {
  throw \`read-asset unsupported: \${handle}\`;
}

export function networkFetch(origin, path) {
  throw \`network-fetch unsupported: \${origin}\${path}\`;
}

// 📦 Must match \`framework/product/os/core/js/index.ts\`'s \`BLOB_ENDPOINT_PATH\`.
const BLOB_ENDPOINT_PATH = "/semio-blob";

/** @emoji 📦 Persists \`data\` to the dev server's content-addressed blob store, returning its hash.
 * \`write-blob\`/\`read-blob\` are declared synchronous in the WIT world (no \`async\` on the host import),
 * so this can't use \`fetch\` — a dedicated worker (unlike the main thread) still permits synchronous
 * \`XMLHttpRequest\`, which is the standard sync-bridge trick for exactly this constraint. */
export function writeBlob(data, mediaType) {
  const xhr = new XMLHttpRequest();
  xhr.open("PUT", \`\${BLOB_ENDPOINT_PATH}?mediaType=\${encodeURIComponent(mediaType)}\`, false);
  xhr.send(new Uint8Array(data));
  if (xhr.status < 200 || xhr.status >= 300) throw \`write-blob failed (\${xhr.status})\`;
  return JSON.parse(xhr.responseText).hash;
}

/** @emoji 📦 Fetches a previously written blob's bytes by hash. See \`writeBlob\` for why this is a
 * synchronous XHR rather than \`fetch\`. */
export function readBlob(hash) {
  const xhr = new XMLHttpRequest();
  xhr.open("GET", \`\${BLOB_ENDPOINT_PATH}/\${encodeURIComponent(hash)}\`, false);
  xhr.responseType = "arraybuffer";
  xhr.send();
  if (xhr.status === 404) throw \`blob not found: \${hash}\`;
  if (xhr.status < 200 || xhr.status >= 300) throw \`read-blob failed (\${xhr.status})\`;
  return new Uint8Array(xhr.response);
}

// 🔗 Per-uri inbound queues (serialized \`BackboneMessage\`s), shared on the worker global so the plugin
// worker's \`backboneInbound\` relay (see pluginWorkerSource) can fill them while this shim drains them —
// the two scripts live in the same worker realm but are separate modules.
function backboneInboundQueues() {
  return (globalThis.__semioBackboneInbound ??= new Map());
}
const backboneAttached = new Set();

/** @emoji 📤 Enqueues an outbound message to the main thread, which relays it into \`backbone-worker.ts\`
 * (the sync actor). A plugin worker can't own the socket/fetch itself, so this is postMessage-only. */
export function backboneSend(uri, messageJson) {
  backboneAttached.add(uri);
  // Only a worker's \`postMessage\` takes a single argument and reaches the parent; on the main thread
  // \`window.postMessage\` needs a targetOrigin, so the relay is a no-op there (WorkerGlobalScope is
  // defined in both classic and module workers, undefined on the main thread).
  if (typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && typeof self.postMessage === "function") {
    self.postMessage({ type: "backboneOutbound", uri, message: messageJson });
  }
}

/** @emoji 📥 Drains the inbound queue the worker filled from \`backboneInbound\` postMessages. Returns
 * serialized \`BackboneMessage\`s (never blocks — an empty queue yields \`[]\`). */
export function backbonePoll(uri) {
  backboneAttached.add(uri);
  const queues = backboneInboundQueues();
  const queue = queues.get(uri);
  if (!queue || queue.length === 0) return [];
  queues.set(uri, []);
  return queue;
}

/** @emoji 📶 Reports whether this shim has seen traffic for a uri (the real transport health lives in
 * \`backbone-worker.ts\`; the sandboxed plugin only needs attached/detached). */
export function backboneStatus(uri) {
  return backboneAttached.has(uri) ? "attached" : "detached";
}
`;
}

async function readPackageName(cratePath: string): Promise<string> {
  const content = await Bun.file(join(repoRoot, cratePath, "Cargo.toml")).text();
  const match = content.match(/^name = "([^"]+)"/m);
  if (!match) throw new Error(`missing package name in ${cratePath}/Cargo.toml`);
  return match[1]!;
}

async function buildPlugin(target: PluginRegistryEntry): Promise<void> {
  const packageName = await readPackageName(target.cratePath);
  const build = spawnSync("cargo", ["build", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--release"], { cwd: repoRoot, stdio: "inherit" });
  if (build.status !== 0) throw new Error(`plugin build failed: ${target.pluginId}`);
  const artifact = join(repoRoot, "target", PLUGIN_WASM_TARGET, "release", `${packageName.replace(/-/g, "_")}.wasm`);
  const outDir = join(pluginOutRoot, target.pluginId);
  mkdirSync(outDir, { recursive: true });
  const jsBase = target.wasmOut.replace(/\.wasm$/, "");
  const wasmOut = join(outDir, target.wasmOut);
  const componentBase = `${jsBase}_component`;
  copyFileSync(artifact, wasmOut);
  writeFileSync(join(outDir, "host-shim.js"), hostShimSource());
  transpilePluginComponent(wasmOut, outDir, componentBase);
  const jsOut = join(outDir, `${jsBase}.js`);
  writeFileSync(jsOut, pluginComponentBridgeSource(componentBase, target.wasmOut));
  writeFileSync(join(outDir, "plugin-worker.js"), pluginWorkerSource());
  const hotSwapMarker = join(pluginOutRoot, ".hot-swap");
  writeFileSync(hotSwapMarker, `${JSON.stringify({ pluginId: target.pluginId, rebuiltAt: Date.now() })}\n`);
  console.log(`[DEBUG] built plugin ${target.pluginId} (${PLUGIN_WASM_TARGET}) -> ${outDir}`);
}

async function ensurePluginRegistry(filterPlugin?: string): Promise<void> {
  const registryScript = join(repoRoot, "framework/plugin/registry/script.ts");
  const args = ["generate"];
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  if (filterPluginId) args.push(filterPluginId);
  const generate = spawnSync("bun", [registryScript, ...args], { cwd: repoRoot, stdio: "inherit" });
  if (generate.status !== 0) throw new Error("plugin registry generation failed");
}

function resolvePluginBuildTargets(entries: readonly PluginRegistryEntry[], filterPlugin?: string): readonly PluginRegistryEntry[] {
  const resolvedPluginId = filterPlugin ? resolvePlaygroundFilter(filterPlugin).pluginId : undefined;
  const targets = resolvedPluginId ? entries.filter((target) => target.pluginId === resolvedPluginId) : entries;
  if (filterPlugin && targets.length === 0) {
    throw new Error(`no plugin build targets for filter ${JSON.stringify(filterPlugin)} (resolved plugin id: ${resolvedPluginId ?? "none"})`);
  }
  return targets;
}

async function buildPlugins(filterPlugin?: string): Promise<void> {
  ensureWasmTarget();
  await ensurePluginRegistry(filterPlugin);
  const filterPluginId = resolveCatalogFilterPluginId(filterPlugin);
  const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
  mkdirSync(pluginOutRoot, { recursive: true });
  ensurePreview2ShimVendor();
  rewriteExistingPluginShimImports();
  const stalePublicPlugins = join(repoRoot, "framework/product/os/dev/public/plugin-modules");
  if (existsSync(stalePublicPlugins)) {
    rmSync(stalePublicPlugins, { recursive: true, force: true });
  }
  const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
  if (filterPlugin && !isStudioPluginFilter(filterPlugin)) {
    console.log(`[DEBUG] plugin build scope: ${targets.map((target) => target.pluginId).join(", ")}`);
  } else {
    console.log(`[DEBUG] plugin build scope: all (${targets.length} plugin crates)`);
  }
  for (const target of targets) {
    await buildPlugin(target);
  }
}

class PluginBuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
  }
}

class PluginWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
    const filterPluginId = resolveCatalogFilterPluginId(filterPlugin || undefined);
    const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
    for (const target of targets) {
      const watchRoot = join(repoRoot, target.cratePath);
      watch(watchRoot, { recursive: true }, () => {
        void buildPlugin(target).catch((error) => {
          console.error("[DEBUG] plugin watch rebuild failed", error);
        });
      });
    }
    console.log("[DEBUG] watching plugin crates for hot-swap rebuilds");
  }
}

/** @emoji 🔎 Resolves an `engines` crate path (from the playground registry) to its `script.ts` wasm
 * build entry point — most engine crates keep `script.ts` inside the `rs` dir itself, a few (e.g.
 * `flow/core/rs`) keep it one level up next to the crate's TS sibling, so both are tried. */
function engineWasmScriptPath(cratePath: string): string {
  const direct = join(repoRoot, cratePath, "script.ts");
  if (existsSync(direct)) return direct;
  const parent = cratePath.endsWith("/rs") ? cratePath.slice(0, -"/rs".length) : cratePath;
  const parentScript = join(repoRoot, parent, "script.ts");
  if (existsSync(parentScript)) return parentScript;
  throw new Error(`no wasm build script found for engine crate ${cratePath}`);
}

/** @emoji 🔌 Builds every wasm engine a react-renderer dev session needs: the framework node-graph +
 * editor host engines unconditionally (shared studio chrome, not any one app), then whatever the
 * active playground variant declares via `engines = […]` on its `[[…playground]]` Cargo.toml row —
 * replaces the previous hardcoded `if (pluginId === "flow" | "gis2d" | "gis3d" | "raster" | "puzzle2d")` branches. */
async function buildEngineWasm(variant: string, renderer: string): Promise<void> {
  if (renderer !== "react" || process.env.SKIP_ENGINE_BUILD === "1") return;
  const graphScript = join(repoRoot, "framework/surface/node-graph/rs/script.ts");
  const graphBuild = spawnSync("bun", [graphScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
  if (graphBuild.status !== 0) throw new Error("framework-surface-node-graph wasm build failed");
  const editorScript = join(repoRoot, "framework/editor/rs/script.ts");
  const editorBuild = spawnSync("bun", [editorScript, "wasm"], { cwd: repoRoot, stdio: "inherit" });
  if (editorBuild.status !== 0) throw new Error("framework-editor wasm build failed");
  const row = playgroundCatalog.find((entry) => entry.variant === variant);
  for (const engineCratePath of row?.engines ?? []) {
    const script = engineWasmScriptPath(engineCratePath);
    const build = spawnSync("bun", [script, "wasm"], { cwd: repoRoot, stdio: "inherit" });
    if (build.status !== 0) throw new Error(`${engineCratePath} wasm build failed`);
  }
}

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (process.env.SKIP_PLUGIN_BUILD !== "1") {
      const filterPlugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
      await buildPlugins(filterPlugin);
    }
    const renderer = process.env.SEMIO_RENDERER ?? "react";
    const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    await buildEngineWasm(plugin, renderer);
    const defaultPort = String(frameworkOsPlaygroundDefaultPort(playgroundCatalog, plugin, renderer));
    if (renderer === "wgpu") {
      const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
      const port = Number(process.env.S_OS_PORT ?? defaultPort);
      const playUrl = wgpuDevPlayUrl(host, port, plugin);
      if (isDevPortInUse(host, port)) {
        const entry = probeWgpuDevPort(host, port);
        if (entry?.entryPath === "/") {
          console.log(`[dev] Port ${port} already serving wgpu trunk at ${playUrl}`);
          return;
        }
        const occupant = describeDevPortOccupant(port);
        if (occupant?.startsWith("trunk")) {
          console.log(`[dev] Restarting stale trunk on port ${port} (${occupant})`);
          stopTrunkDevPort(port);
          for (let attempt = 0; attempt < 40 && isDevPortInUse(host, port); attempt++) {
            await Bun.sleep(250);
          }
        } else if (entry) {
          console.log(`[dev] Port ${port} already serving legacy wgpu trunk at ${wgpuDevPlayUrl(host, port, plugin, entry.entryPath)}`);
          return;
        } else {
          console.error(`[dev] Port ${port} is already in use${occupant ? ` by ${occupant}` : ""}. Stop that process or set S_OS_PORT.`);
          process.exit(1);
        }
      }
      const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
      const serve = spawnSync("bun", [wgpuScript, "serve"], {
        cwd: join(repoRoot, "framework/renderer/wgpu"),
        stdio: "inherit",
        env: {
          ...process.env,
          SEMIO_PLUGIN: plugin,
          SEMIO_RENDERER: renderer,
          S_OS_PORT: String(port),
        },
      });
      if (serve.status !== 0 && !probeWgpuDevPort(host, port)) {
        throw new Error("wgpu trunk serve failed");
      }
      console.log(`[dev] wgpu trunk serving at ${playUrl}`);
      return;
    }
    const resolvedFilter = resolvePlaygroundFilter(plugin);
    runViteBunxDev(this.root, segments, {
      portEnv: "S_OS_PORT",
      defaultPort,
      fixedPort: true,
      env: {
        SEMIO_PLUGIN: plugin,
        SEMIO_RENDERER: renderer,
        VITE_SEMIO_RENDERER: renderer,
        VITE_SEMIO_PLUGIN: resolvedFilter.pluginId,
        ...(resolvedFilter.appId ? { VITE_SEMIO_APP_ID: resolvedFilter.appId } : {}),
        ...frameworkOsLockedPrefsEnv(),
      },
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await new PluginBuildScript(this.root).run([]);
    const renderer = process.env.SEMIO_RENDERER ?? "react";
    const plugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    if (renderer === "wgpu" && process.env.SKIP_WGPU_BUILD !== "1") {
      const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
      const wgpuBuild = spawnSync("bun", [wgpuScript, "wasm", "--release"], { cwd: repoRoot, stdio: "inherit" });
      if (wgpuBuild.status !== 0) throw new Error("wgpu trunk build failed");
      return;
    }
    await buildEngineWasm(plugin, renderer);
    spawnSync("bun", ["run", "vite", "build", "--config", "vite.config.ts", ...segments], {
      cwd: this.root,
      stdio: "inherit",
    });
  }
}

const PLUGIN_HOST_MODE_SYMBOLS = ["SEMIO_PLUGIN", "PLAYGROUND_APP_KIND", "studioMode", "pluginFilter"] as const;

function walkRustSources(dir: string, out: string[]): void {
  for (const ent of readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "target" || ent.name === "node_modules") continue;
    const abs = join(dir, ent.name);
    if (ent.isDirectory()) {
      walkRustSources(abs, out);
      continue;
    }
    if (ent.name.endsWith(".rs")) out.push(abs);
  }
}

class PluginCapabilityLintScript extends BundleScript {
  async run(): Promise<void> {
    const metadataResult = spawnSync("cargo", ["metadata", "--format-version", "1"], {
      cwd: repoRoot,
      encoding: "utf8",
      maxBuffer: 64 * 1024 * 1024,
    });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout ?? "{}") as {
      packages: Array<{
        name: string;
        manifest_path: string;
        dependencies: Array<{ name: string }>;
      }>;
    };
    const registryEntries = generatePluginRegistry(repoRoot);
    const pluginPackageNames = new Map(registryEntries.map((entry) => [entry.packageName, entry.pluginId]));
    const depRules: Record<string, string> = {
      rusqlite: "localBackboneStorage",
      libloading: "forbidden",
      reqwest: "forbidden",
      "web-sys": "forbidden",
      "js-sys": "forbidden",
    };
    const failures: string[] = [];
    for (const pkg of metadata.packages) {
      if (!pkg.manifest_path.includes("/plugin/rs/Cargo.toml")) continue;
      if (pkg.manifest_path.includes("/framework/plugin/rs/")) continue;
      const manifestText = await Bun.file(pkg.manifest_path).text();
      const declared = new Set<string>();
      const metaMatch = manifestText.match(/\[package\.metadata\.semio\][\s\S]*?capabilities\s*=\s*\[([^\]]*)\]/);
      if (metaMatch?.[1]) {
        for (const entry of metaMatch[1].match(/"([^"]+)"/g) ?? []) {
          declared.add(entry.slice(1, -1));
        }
      }
      if (manifestText.includes("local_backbone_storage()") || manifestText.includes("ResourceKind::Backbone")) {
        declared.add("localBackboneStorage");
      }
      const depNames = new Set(pkg.dependencies.map((dep) => dep.name));
      for (const dep of pkg.dependencies) {
        const otherPluginId = pluginPackageNames.get(dep.name);
        if (otherPluginId && dep.name !== pkg.name) {
          failures.push(`${pkg.name}: cross-plugin dependency on ${dep.name} (${otherPluginId})`);
        }
      }
      for (const [dep, rule] of Object.entries(depRules)) {
        if (!depNames.has(dep)) continue;
        if (rule === "forbidden") {
          failures.push(`${pkg.name}: forbidden dependency ${dep}`);
          continue;
        }
        if (!declared.has(rule)) {
          failures.push(`${pkg.name}: dependency ${dep} requires capability ${rule}`);
        }
      }
      const rustSources: string[] = [];
      walkRustSources(dirname(pkg.manifest_path), rustSources);
      for (const sourcePath of rustSources) {
        const source = await Bun.file(sourcePath).text();
        if (/std::fs::|std::net::/.test(source) && !declared.has("localBackboneStorage")) {
          failures.push(`${pkg.name}: uses std::fs/std::net without localBackboneStorage capability (${relative(repoRoot, sourcePath)})`);
        }
        for (const symbol of PLUGIN_HOST_MODE_SYMBOLS) {
          if (!source.includes(symbol)) continue;
          failures.push(`${pkg.name}: plugin source references host-mode symbol ${symbol} (${relative(repoRoot, sourcePath)})`);
        }
      }
    }
    if (failures.length > 0) {
      for (const failure of failures) console.error(`[plugin-capability-lint] ${failure}`);
      throw new Error(`plugin capability lint failed (${failures.length} issues)`);
    }
    console.log("[DEBUG] plugin capability lint passed");
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "vitest.config.ts");
  }
}

//#region 🔖StudioE2eVerify
/** 🎭 Playwright end-to-end workflow verification for the `s` studio shell (folded in from the former `.repo/🎫/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`). */
const STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function studioE2eAssert(condition: boolean, message: string): void {
  if (!condition) throw new Error(message);
}

function isIgnorableStudioE2ePageError(message: string): boolean {
  return STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

async function waitForStudioE2eCondition(page: import("playwright").Page, predicate: (state: { text: string; children: number }) => boolean, label: string, deadline: number): Promise<{ text: string; children: number }> {
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const children = await page.locator("#root *").count();
    if (predicate({ text, children })) return { text, children };
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for ${label}`);
}

async function openStudioE2e(page: import("playwright").Page, deadline: number): Promise<{ text: string; children: number }> {
  await page.keyboard.press("Meta+n");
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const path = await page.evaluate(() => location.pathname);
    const children = await page.locator("#root *").count();
    if (/Catalogue/i.test(text) && /Parameters/i.test(text) && path.startsWith("/studios/")) {
      return { text, children };
    }
    await page.waitForTimeout(500);
  }
  throw new Error("timeout waiting for studio workspace");
}

async function activateStudioE2eMediaGraphWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eMediaGraphEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eMediaGraphWindow(page);
  await page.evaluate(() => document.getElementById("s-media-graph-window-engagement-toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eMediaGraphEngagement(page);
  const engagementInput = page.locator("#s-media-catalogue-hint");
  await engagementInput.fill("draw draw");
  await engagementInput.press("Enter");
  await page.waitForTimeout(1500);
  return "engagement";
}

async function openStudioE2eCommandPalette(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(100);
  await page.keyboard.press("Meta+p");
  await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromPalette(page: import("playwright").Page): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("draw");
  await page.waitForTimeout(400);
  const drawSpawn = page
    .locator("[cmdk-item]")
    .filter({ hasText: /Spawn Draw/i })
    .first();
  if (await drawSpawn.count()) {
    await drawSpawn.click();
    return "palette";
  }
  await page.keyboard.press("Escape");
  return null;
}

async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  console.log(`[DEBUG] navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForFunction(() => document.body.innerText.includes("Home") && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

  const deadline = Date.now() + timeoutMs;
  const booted = await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text), "home shell with studios", deadline);
  console.log(`[DEBUG] home loaded (${booted.children} nodes)`);
  studioE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

  await openStudioE2e(page, deadline);
  const pathAfterCreate = await page.evaluate(() => location.pathname);
  console.log(`[DEBUG] studio loaded at ${pathAfterCreate}`);
  studioE2eAssert(pathAfterCreate.startsWith("/studios/"), "studio uri should be under /studios/");

  await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, { timeout: 30_000 });

  const bodyText = await page.locator("body").innerText();
  studioE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");
  studioE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");
  studioE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
  console.log("[DEBUG] three studio windows rendered");

  let spawnMode: string | null = null;
  try {
    spawnMode = await spawnStudioE2eDrawFromEngagement(page);
    console.log(`[DEBUG] spawn via ${spawnMode}`);
  } catch {
    spawnMode = await spawnStudioE2eDrawFromPalette(page);
    studioE2eAssert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
    console.log(`[DEBUG] spawn via ${spawnMode}`);
  }

  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(1500);
  console.log("[DEBUG] undo issued");

  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("undo");
  await page.waitForTimeout(300);
  studioE2eAssert((await page.locator("[cmdk-item]").filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
  await paletteInput.fill("checkpoint");
  await page.waitForTimeout(300);
  studioE2eAssert(
    (await page
      .locator("[cmdk-item]")
      .filter({ hasText: /commitCheckpoint/ })
      .count()) > 0,
    "checkpoint command should be in command palette",
  );
  console.log("[DEBUG] studio commands in palette");
  await page.keyboard.press("Escape");

  await page.keyboard.press("Meta+f");
  await page.waitForTimeout(500);
  studioE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
  console.log("[DEBUG] find palette available");
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: "← Home" }).click({ force: true });
  await waitForStudioE2eCondition(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via studio bar", deadline);
  console.log("[DEBUG] studio home bar navigation works");

  const demoStudioRow = page.locator('[data-row-id="studio:default"]');
  if (await demoStudioRow.count()) {
    await demoStudioRow.dblclick({ force: true });
    await page.waitForFunction(() => location.pathname.startsWith("/studios/"), { timeout: 15_000 });
    studioE2eAssert(/Catalogue/i.test(await page.locator("body").innerText()), "opened studio from home vfs");
    console.log("[DEBUG] home vfs open studio works");
  }

  const criticalErrors = pageErrors.filter((message) => !isIgnorableStudioE2ePageError(message));
  if (criticalErrors.length !== pageErrors.length) {
    console.log(`[DEBUG] ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}`);
  }
  studioE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

  await browser.close();
  console.log("PASS: S studio end-to-end workflows verified");
}
//#endregion 🔖StudioE2eVerify

class VerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const port = process.env.S_OS_PORT ?? "6070";
    const studioUrl = process.env.S_STUDIO_URL ?? `http://127.0.0.1:${port}/`;
    const timeoutMs = Number(process.env.S_STUDIO_E2E_TIMEOUT_MS ?? 300_000);
    if (segments[0] === "e2e") {
      await runStudioE2eVerify(studioUrl, timeoutMs);
      console.log(`[DEBUG] s studio e2e verify passed (${studioUrl})`);
      return;
    }
    for (const target of generatePluginRegistry(repoRoot)) {
      const packageName = await readPackageName(target.cratePath);
      const pluginTests = spawnSync("cargo", ["test", "-p", packageName], { cwd: repoRoot, stdio: "inherit" });
      if (pluginTests.status !== 0) throw new Error(`${packageName} tests failed`);
    }
    const rendererTests = spawnSync("bunx", ["vitest", "run"], {
      cwd: join(repoRoot, "framework/renderer/react"),
      stdio: "inherit",
    });
    if (rendererTests.status !== 0) throw new Error("framework-renderer-react tests failed");
    await runStudioE2eVerify(studioUrl, timeoutMs);
    await new PluginCapabilityLintScript(this.root).run([]);
    console.log(`[DEBUG] s studio verify passed (${studioUrl})`);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("verify", VerifyScript)
  .register(
    "plugin",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "watch") return new PluginWatchScript(this.root).run(segments.slice(1));
        if (sub === "lint") return new PluginCapabilityLintScript(this.root).run(segments.slice(1));
        if (sub === "registry") {
          await ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
          return;
        }
        return new PluginBuildScript(this.root).run(segments.slice(1));
      }
    },
  );

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
}
