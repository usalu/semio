#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router — Rust plugin OS dev host. */
import { createWriteStream, copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, watch, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
  BundleScript,
  ScriptRouter,
  buildBudgetMs,
  daemonBudgetOpts,
  describeDevPortOccupant,
  devServerUrl,
  getWorkspaceRoot,
  isDevPortInUse,
  loadFrameworkOsPlaygroundCatalog,
  probeWgpuDevPort,
  stopTrunkDevPort,
  wgpuDevPlayUrl,
  runBundleScriptMain,
  runCmd,
  runCmdStatus,
  runProbe,
  runVitest,
  spawnDaemon,
  type SpawnDaemonHandle,
  runViteBunxDev,
  frameworkOsPlaygroundDefaultPort,
  frameworkOsLockedPrefsEnv,
  resolveTestLevel,
} from "../../../../repo/lib/js/index.ts";
import { BACKBONE_ENDPOINT_PATH, BLOB_ENDPOINT_PATH, backboneKindFromUri } from "@semio-tech/framework-os-core";
import { generatePluginRegistry, isStudioPluginFilter, writePlaygroundSession, type PluginRegistryEntry } from "../../../plugin/registry/script.ts";
import { PNG } from "pngjs";
import pixelmatch from "pixelmatch";

const repoRoot = getWorkspaceRoot();
const pluginOutRoot = join(repoRoot, "framework/product/os/dev/plugin-modules");
const playgroundSessionPath = join(repoRoot, "framework/product/os/dev/generated/session.ts");

const PLUGIN_WASM_TARGET = "wasm32-wasip2";

//#region 🔖PlaygroundVariantResolution
/** @emoji 📚 Generated playground catalog (variant -> crate pluginId + optional app id), loaded once for this process via `@semio-tech/repo-lib`'s `loadFrameworkOsPlaygroundCatalog` (backed by `framework/plugin/registry/generated/playgrounds.ts`). */
const playgroundCatalog = loadFrameworkOsPlaygroundCatalog();

/** @emoji 🧭 A resolved playground filter: the crate pluginId to build/load, plus the app id and shell brand id to inject when the filter matched a catalog variant row. */
type ResolvedPlaygroundFilter = {
  readonly pluginId: string;
  readonly appId?: string;
  readonly brand?: string;
};

/**
 * 🧭 Resolves `filterPlugin` (a playground variant id like "puzzle5d", or already a bare crate
 * pluginId like "note") against the generated playground catalog: a matching variant row yields
 * its crate pluginId, app id, and brand id, otherwise `filterPlugin` is treated as already being a
 * bare pluginId (existing behavior for single-app crates where variant === pluginId).
 */
function resolvePlaygroundFilter(filterPlugin: string): ResolvedPlaygroundFilter {
  const row = playgroundCatalog.find((entry) => entry.variant === filterPlugin);
  return row ? { pluginId: row.pluginId, appId: row.app, brand: row.brand } : { pluginId: filterPlugin };
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
 * `node:fs.watch` per folder regardless of subscriber count. Mirrors `store_sync`'s native
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
      case "consumeMedia":
        await api.consumeMedia(msg.instanceId, msg.portId, msg.descriptorJson, msg.data);
        reply(requestId, "consumeMedia", { ok: true });
        break;
      case "produceMedia":
        reply(requestId, "produceMedia", {
          value: await api.produceMedia(msg.instanceId, msg.portId, msg.requestJson),
        });
        break;
      default:
        throw new Error(\`unknown worker message type: \${type}\`);
    }
  } catch (error) {
    const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
    const detail = payload !== undefined ? \` payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : "";
    replyError(requestId, (error instanceof Error ? error.message : String(error)) + detail);
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
        const payload = error && typeof error === "object" && "payload" in error ? error.payload : undefined;
        const detail = payload !== undefined ? \`\${message} payload=\${(() => { try { return JSON.stringify(payload); } catch { return String(payload); } })()}\` : message;
        const busy = detail.includes("plugin instance busy") || detail.includes("plugin busy");
        const trapped = detail.includes("unreachable") || /trap|panicked/i.test(detail);
        if (busy || trapped) {
          try { plugin.clearInstanceGuard?.(); } catch { /* guard heal is best-effort */ }
        }
        if (!busy) throw error;
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
    async consumeMedia(instanceId, portId, descriptorJson, data) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      await plugin.consumeMedia(instanceId, portId, {
        descriptorJson,
        data: data instanceof Uint8Array ? data : new Uint8Array(data ?? []),
      });
    },
    async produceMedia(instanceId, portId, requestJson) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const artifact = await plugin.produceMedia(instanceId, portId, requestJson ?? "");
      return { descriptorJson: artifact.descriptorJson, data: artifact.data };
    },
    async readAppDocumentText(instanceId) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const files = await plugin.readAppDocumentText(instanceId);
      return { dsl: files.dsl, ops: files.ops };
    },
    async loadAppDocumentText(instanceId, dsl, ops) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      await plugin.loadAppDocumentText(instanceId, { dsl, ops });
    },
    async readAppDocumentPack(instanceId) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      const files = await plugin.readAppDocumentPack(instanceId);
      return { pack: files.pack, ops: files.ops };
    },
    async loadAppDocumentPack(instanceId, pack, ops) {
      if (!apps.has(instanceId)) throw new Error(\`unknown instance: \${instanceId}\`);
      await plugin.loadAppDocumentPack(instanceId, {
        pack: pack instanceof Uint8Array ? pack : new Uint8Array(pack ?? []),
        ops,
      });
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
    consumeMedia: (instanceId, portId, descriptorJson, data) =>
      runSerialized(() => core.consumeMedia(instanceId, portId, descriptorJson, data)),
    produceMedia: (instanceId, portId, requestJson) =>
      runSerialized(() => core.produceMedia(instanceId, portId, requestJson)),
    readAppDocumentText: (instanceId) => runSerialized(() => core.readAppDocumentText(instanceId)),
    loadAppDocumentText: (instanceId, dsl, ops) => runSerialized(() => core.loadAppDocumentText(instanceId, dsl, ops)),
    readAppDocumentPack: (instanceId) => runSerialized(() => core.readAppDocumentPack(instanceId)),
    loadAppDocumentPack: (instanceId, pack, ops) => runSerialized(() => core.loadAppDocumentPack(instanceId, pack, ops)),
  };
}

export async function createPluginApi() {
  if (!pluginApiPromise) pluginApiPromise = createPluginApiInner();
  return pluginApiPromise;
}
`;
}

function ensureWasmTarget(): void {
  const probe = runProbe("rustup", ["target", "list", "--installed"]);
  if (!probe.stdout.includes(PLUGIN_WASM_TARGET)) {
    runCmd("rustup", ["target", "add", PLUGIN_WASM_TARGET]);
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
  if (runCmdStatus("bunx", ["@bytecodealliance/jco", "transpile", artifact, "-o", outDir, "--name", componentBase, "--map", "semio:framework/host=./host-shim.js"], { cwd: repoRoot }) !== 0) {
    throw new Error(`jco transpile failed for ${artifact}`);
  }
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
 * (the sync actor). Inside a dedicated worker this is postMessage-only (a worker can't own the
 * socket/fetch itself); when this component is instead loaded directly on the main thread (the
 * no-\`Worker\`/component-model-load fallback in \`framework/core/js/index.ts\`), it reaches the same
 * relay through the well-known \`__semioMainThreadPluginBackboneOutbound\` global instead. */
export function backboneSend(uri, messageJson) {
  backboneAttached.add(uri);
  if (typeof WorkerGlobalScope !== "undefined" && typeof self !== "undefined" && typeof self.postMessage === "function") {
    self.postMessage({ type: "backboneOutbound", uri, message: messageJson });
  } else if (typeof globalThis.__semioMainThreadPluginBackboneOutbound === "function") {
    globalThis.__semioMainThreadPluginBackboneOutbound(uri, messageJson);
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
  if (runCmdStatus("cargo", ["build", "-p", packageName, "--target", PLUGIN_WASM_TARGET, "--release"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) {
    throw new Error(`plugin build failed: ${target.pluginId}`);
  }
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
  if (runCmdStatus("bun", [registryScript, "generate"], { cwd: repoRoot }) !== 0) throw new Error("plugin registry generation failed");
  const variant = filterPlugin ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
  writePlaygroundSession(variant, playgroundSessionPath, repoRoot);
}

function resolvePluginBuildTargets(entries: readonly PluginRegistryEntry[], filterPlugin?: string): readonly PluginRegistryEntry[] {
  if (!filterPlugin || isStudioPluginFilter(filterPlugin)) return entries;
  if (entries.length === 0) {
    throw new Error(`no plugin build targets for filter ${JSON.stringify(filterPlugin)}`);
  }
  return entries;
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

/** @emoji 👀 A plugin crate's edits alone don't cover every source that feeds its build: multi-crate
 * app families (e.g. `fem/plugin/rs` depending on `fem/2d/rs`/`fem/3d/rs`/`fem/core/rs`, or an
 * example fixture under `fem/2d/example`) live as SIBLING directories under the same top-level app
 * folder, not inside the plugin crate itself. Watching just `target.cratePath` misses them, so a
 * schema or fixture edit never triggers a hot-swap rebuild. Framework-hosted plugin crates
 * (`framework/...`) keep the narrow crate-only watch instead — widening to all of `framework/` would
 * watch the entire monorepo's shared core. Cargo's own `target/` output lives at the repo root and
 * built wasm lands in `framework/product/os/dev/plugin-modules`, so widening the watch root here
 * cannot cause a rebuild to re-trigger itself. */
function pluginWatchRoot(target: PluginRegistryEntry): string {
  const topLevel = target.cratePath.split("/")[0];
  return join(repoRoot, topLevel === "framework" ? target.cratePath : topLevel);
}

class PluginWatchScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = segments[0] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND;
    await buildPlugins(filterPlugin || undefined);
    const filterPluginId = resolveCatalogFilterPluginId(filterPlugin || undefined);
    const catalogEntries = generatePluginRegistry(repoRoot, filterPluginId ? { filterPlaygroundPlugin: filterPluginId } : {});
    const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin || undefined);
    for (const target of targets) {
      watch(pluginWatchRoot(target), { recursive: true }, () => {
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
  // Each recurses into a crate's own `wasm` script (wasm-pack/cargo build under the hood) — budgeted at
  // the build class rather than the generic command default since those inner builds can legitimately
  // approach [[buildBudgetMs]] themselves.
  const graphScript = join(repoRoot, "framework/surface/node-graph/rs/script.ts");
  if (runCmdStatus("bun", [graphScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("framework-surface-node-graph wasm build failed");
  const editorScript = join(repoRoot, "framework/editor/rs/script.ts");
  if (runCmdStatus("bun", [editorScript, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("framework-editor wasm build failed");
  const row = playgroundCatalog.find((entry) => entry.variant === variant);
  for (const engineCratePath of row?.engines ?? []) {
    const script = engineWasmScriptPath(engineCratePath);
    if (runCmdStatus("bun", [script, "wasm"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`${engineCratePath} wasm build failed`);
  }
}

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const filterPlugin = process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    if (process.env.SKIP_PLUGIN_BUILD !== "1") {
      await buildPlugins(filterPlugin);
    } else {
      await ensurePluginRegistry(filterPlugin);
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
      const serveStatus = runCmdStatus("bun", [wgpuScript, "serve"], {
        cwd: join(repoRoot, "framework/renderer/wgpu"),
        env: {
          ...process.env,
          SEMIO_PLUGIN: plugin,
          SEMIO_RENDERER: renderer,
          S_OS_PORT: String(port),
        },
        ...daemonBudgetOpts(),
      });
      if (serveStatus !== 0 && !probeWgpuDevPort(host, port)) {
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
        ...(resolvedFilter.brand && !process.env.SEMIO_BRAND ? { SEMIO_BRAND: resolvedFilter.brand } : {}),
        ...frameworkOsLockedPrefsEnv(),
      },
    });
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variantSegment = segments[0] && !segments[0].startsWith("-") ? segments[0] : undefined;
    const viteSegments = variantSegment ? segments.slice(1) : segments;
    const plugin = variantSegment ?? process.env.SEMIO_PLUGIN ?? process.env.PLAYGROUND_APP_KIND ?? "s";
    await new PluginBuildScript(this.root).run([plugin]);
    const renderer = process.env.SEMIO_RENDERER ?? "react";
    if (renderer === "wgpu" && process.env.SKIP_WGPU_BUILD !== "1") {
      const wgpuScript = join(repoRoot, "framework/renderer/wgpu/script.ts");
      if (runCmdStatus("bun", [wgpuScript, "wasm", "--release"], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error("wgpu trunk build failed");
      return;
    }
    await buildEngineWasm(plugin, renderer);
    const resolvedFilter = resolvePlaygroundFilter(plugin);
    runCmdStatus("bun", ["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], {
      cwd: this.root,
      env: {
        ...process.env,
        SEMIO_PLUGIN: plugin,
        SEMIO_RENDERER: renderer,
        VITE_SEMIO_RENDERER: renderer,
        VITE_SEMIO_PLUGIN: resolvedFilter.pluginId,
        ...(resolvedFilter.appId ? { VITE_SEMIO_APP_ID: resolvedFilter.appId } : {}),
        ...(resolvedFilter.brand && !process.env.SEMIO_BRAND ? { SEMIO_BRAND: resolvedFilter.brand } : {}),
        ...frameworkOsLockedPrefsEnv(),
      },
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
    const metadataResult = runProbe("cargo", ["metadata", "--format-version", "1", "--no-deps"], { cwd: repoRoot, budgetMs: buildBudgetMs() });
    if (metadataResult.status !== 0) {
      throw new Error(metadataResult.stderr || "cargo metadata failed");
    }
    const metadata = JSON.parse(metadataResult.stdout || "{}") as {
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
      egui: "forbidden",
      eframe: "forbidden",
      wgpu: "forbidden",
      "wgpu-core": "forbidden",
      winit: "forbidden",
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
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "vitest.config.ts");
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

async function activateStudioE2eWorkflowWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eWorkflowEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eWorkflowWindow(page);
  await page.evaluate(() => document.getElementById("framework.window.sWorkflow.search.toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eWorkflowEngagement(page);
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
  await page.waitForFunction(() => /home/i.test(document.body.innerText) && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

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
      .filter({ hasText: /checkpoint/i })
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
    await waitForStudioE2eCondition(page, ({ text }) => /Catalogue/i.test(text), "opened studio from home vfs", deadline);
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
      if (runCmdStatus("cargo", ["test", "-p", packageName], { cwd: repoRoot, budgetMs: buildBudgetMs() }) !== 0) throw new Error(`${packageName} tests failed`);
    }
    if (runCmdStatus("bunx", ["vitest", "run"], { cwd: join(repoRoot, "framework/renderer/react") }) !== 0) throw new Error("framework-renderer-react tests failed");
    await runStudioE2eVerify(studioUrl, timeoutMs);
    await new PluginCapabilityLintScript(this.root).run([]);
    console.log(`[DEBUG] s studio verify passed (${studioUrl})`);
  }
}

//#region 🔬ParityScript
/** 🔬wgpu↔React UI-parity verification harness — structural DOM/retained-tree comparison, per-region
 * pixel diffing, and a boot-triage ladder, driven per catalog playground. Ticket:
 * `.repo/🎫/26/07/11/WGPU-RENDERER-FULL-PARITY/`. */

//#region 🔖ParityTypes
type ParityRenderer = "react" | "wgpu";
type ParityRect = readonly [number, number, number, number];
type ParityColor = readonly [number, number, number, number];

type ParityNode = {
  readonly path: string;
  readonly kind: string;
  readonly rect: ParityRect;
  readonly text: string | null;
  readonly color: ParityColor | null;
  readonly bg: ParityColor | null;
  readonly fontSize: number | null;
  readonly fontWeight: number | null;
  readonly visible: boolean;
  readonly state: { readonly hovered: boolean; readonly disabled: boolean; readonly selected: boolean };
};

type ParityDump = {
  readonly viewport: { readonly w: number; readonly h: number; readonly dpr: number };
  readonly focusPath: string | null;
  readonly nodes: readonly ParityNode[];
};

type ParityMismatchAxis = "topology" | "text" | "rect" | "color" | "bg" | "fontSize" | "focus";
type ParityMismatch = { readonly path: string; readonly axis: ParityMismatchAxis; readonly react: unknown; readonly wgpu: unknown };
type StructuralResult = { readonly status: "PASS" | "FAIL"; readonly nodeCount: number; readonly mismatches: readonly ParityMismatch[] };

/** 🪜Boot-triage ladder status — evaluated before any structural/pixel comparison, never conflated with a mismatch. */
type BootStatus = "PASS" | "SERVER-FAIL" | "BOOT-TIMEOUT" | "ENV-FAIL" | "DUMP-EMPTY" | "BLANK-PAINT";

type PixelRegionResult = { readonly path: string; readonly ratio: number; readonly threshold: number; readonly diffPng?: string };

type ParityPlaygroundReport = {
  readonly variant: string;
  readonly boot: { readonly react: BootStatus; readonly wgpu: BootStatus; readonly detail?: string };
  readonly structural?: StructuralResult;
  readonly pixel?: { readonly status: "PASS" | "FAIL"; readonly regions: readonly PixelRegionResult[] };
  /** 🎬See `🔖ProbeCatalog` — behavioral (interaction-driven) parity, distinct from the static
   * `structural`/`pixel` end-state checks above. Optional: only populated once boot passed (a probe
   * can't drive a page that never finished booting). */
  readonly behavioral?: ProbeRunResult;
  readonly durationMs: number;
};
//#endregion 🔖ParityTypes

//#region 🔖StructuralDump
/** 🌳DOM-side structural walk — every element carrying `data-ui-path` (see `framework/renderer/react/index.tsx`
 * region `🔖UiInterpreter`) is one matched node. `text` is only captured for non-container kinds since
 * `textContent` on a container aggregates all descendant text, which would false-positive against wgpu's
 * per-node (non-aggregated) text field. */
const PARITY_CONTAINER_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "externalSlot"]);

const REACT_DOM_DUMP_SCRIPT = `(() => {
  const CONTAINER_KINDS = new Set(${JSON.stringify([...PARITY_CONTAINER_KINDS])});
  function parseColor(str) {
    const m = /rgba?\\(([^)]+)\\)/.exec(str || "");
    if (!m) return null;
    const parts = m[1].split(",").map((s) => parseFloat(s.trim()));
    if (parts.length < 3) return null;
    return [Math.round(parts[0]), Math.round(parts[1]), Math.round(parts[2]), parts[3] === undefined ? 1 : parts[3]];
  }
  function nearestPath(el) {
    let cur = el;
    while (cur) {
      const p = cur.getAttribute && cur.getAttribute("data-ui-path");
      if (p) return p;
      cur = cur.parentElement;
    }
    return null;
  }
  const nodes = [];
  document.querySelectorAll("[data-ui-path]").forEach((el) => {
    const rect = el.getBoundingClientRect();
    const style = getComputedStyle(el);
    const path = el.getAttribute("data-ui-path");
    const kind = (path.split("/").pop() || "").replace(/\\[.*/, "").replace(/^#.*/, "");
    nodes.push({
      path,
      kind,
      rect: [Math.round(rect.x), Math.round(rect.y), Math.round(rect.width), Math.round(rect.height)],
      text: CONTAINER_KINDS.has(kind) ? null : (el.textContent || "").replace(/\\s+/g, " ").trim() || null,
      color: parseColor(style.color),
      bg: parseColor(style.backgroundColor),
      fontSize: parseFloat(style.fontSize) || null,
      fontWeight: parseFloat(style.fontWeight) || null,
      visible: rect.width > 0 && rect.height > 0 && style.visibility !== "hidden" && style.display !== "none",
      state: {
        hovered: el.matches(":hover"),
        disabled: el.hasAttribute("disabled") || el.getAttribute("aria-disabled") === "true",
        selected: el.getAttribute("aria-selected") === "true" || el.getAttribute("aria-pressed") === "true",
      },
    });
  });
  const active = document.activeElement;
  return JSON.stringify({
    viewport: { w: window.innerWidth, h: window.innerHeight, dpr: window.devicePixelRatio },
    focusPath: active ? nearestPath(active) : null,
    nodes,
  });
})()`;

async function dumpReactStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(REACT_DOM_DUMP_SCRIPT);
  return JSON.parse(json as unknown as string) as ParityDump;
}

/** 🧊Calls the wasm-bindgen introspection hooks exposed by `framework/renderer/wgpu/rs/lib.rs` region
 * `🔬Introspection`. Reachable at `window.wasmBindings.dumpStructure()`/`dumpFrameStats()` — Trunk's
 * dev-server boot glue (`framework/renderer/wgpu/js/boot.ts`) attaches the wasm module's exports there
 * (the same path `semioRendererBoot`/`uploadIconAtlas` already use), NOT a bespoke global. Returns an
 * empty dump (never throws) when the hooks aren't present yet, so triage can distinguish "not booted"
 * from "no hooks" via `DUMP-EMPTY`. */
async function dumpWgpuStructure(page: import("playwright").Page): Promise<ParityDump> {
  const json = await page.evaluate(() => (window as unknown as { wasmBindings?: { dumpStructure?: () => string } }).wasmBindings?.dumpStructure?.());
  if (!json) return { viewport: { w: 0, h: 0, dpr: 1 }, focusPath: null, nodes: [] };
  return JSON.parse(json) as ParityDump;
}

async function dumpWgpuFrameStats(page: import("playwright").Page): Promise<{ readonly drawCalls: number; readonly quads: number; readonly glyphs: number } | null> {
  const json = await page.evaluate(() => (window as unknown as { wasmBindings?: { dumpFrameStats?: () => string } }).wasmBindings?.dumpFrameStats?.());
  if (!json) return null;
  const stats = JSON.parse(json) as { readonly drawCalls: number; readonly quadCount: number; readonly glyphCount: number };
  return { drawCalls: stats.drawCalls, quads: stats.quadCount, glyphs: stats.glyphCount };
}
//#endregion 🔖StructuralDump

//#region 🔖StructuralCompare
const PARITY_RECT_TOLERANCE_PX = 1.5;
const PARITY_COLOR_TOLERANCE = 3;
const PARITY_FONT_SIZE_TOLERANCE_PX = 0.5;
/** 🎨Scene canvases are rasterized by two different pipelines — structural comparison covers only
 * their rect (placement), never their internal text/color, which is a pixel/behavioral-probe concern. */
const PARITY_SCENE_LEAF_KINDS = new Set(["componentScene", "image"]);

function parityNormalizeText(s: string | null): string | null {
  return s === null ? null : s.normalize("NFC").replace(/\s+/g, " ").trim();
}

function parityColorClose(a: ParityColor | null, b: ParityColor | null): boolean {
  if (a === null || b === null) return a === b;
  return Math.abs(a[0] - b[0]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[1] - b[1]) <= PARITY_COLOR_TOLERANCE && Math.abs(a[2] - b[2]) <= PARITY_COLOR_TOLERANCE;
}

/** 🎨React's dump reports sRGB `rgb()` CSS values as 0–255 ints; wgpu's `Theme` colors are LINEAR-space
 * 0–1 floats (see `framework/renderer/wgpu/rs/lib.rs`'s `🔬IntrospectionVisualFields` doc comment) —
 * comparing them raw would treat every color as a mismatch. Converts wgpu's linear floats to sRGB
 * 0–255 ints so both sides land in the same space before `parityColorClose`'s byte-scale tolerance applies. */
function parityLinearToSrgbColor(c: ParityColor | null): ParityColor | null {
  if (c === null) return null;
  const toByte = (channel: number): number => {
    const clamped = Math.min(1, Math.max(0, channel));
    const srgb = clamped <= 0.0031308 ? clamped * 12.92 : 1.055 * Math.pow(clamped, 1 / 2.4) - 0.055;
    return Math.round(srgb * 255);
  };
  return [toByte(c[0]), toByte(c[1]), toByte(c[2]), c[3]];
}

function compareParityStructural(reactDump: ParityDump, wgpuDump: ParityDump): StructuralResult {
  const mismatches: ParityMismatch[] = [];
  const reactByPath = new Map(reactDump.nodes.map((n) => [n.path, n]));
  const wgpuByPath = new Map(wgpuDump.nodes.map((n) => [n.path, n]));
  const allPaths = new Set([...reactByPath.keys(), ...wgpuByPath.keys()]);
  for (const path of allPaths) {
    const r = reactByPath.get(path);
    const w = wgpuByPath.get(path);
    if (!r || !w) {
      mismatches.push({ path, axis: "topology", react: r?.kind ?? null, wgpu: w?.kind ?? null });
      continue;
    }
    const isSceneLeaf = PARITY_SCENE_LEAF_KINDS.has(r.kind);
    if (!isSceneLeaf && parityNormalizeText(r.text) !== parityNormalizeText(w.text)) {
      mismatches.push({ path, axis: "text", react: r.text, wgpu: w.text });
    }
    const [rx, ry, rw, rh] = r.rect;
    const [wx, wy, ww, wh] = w.rect;
    if (Math.abs(rx - wx) > PARITY_RECT_TOLERANCE_PX || Math.abs(ry - wy) > PARITY_RECT_TOLERANCE_PX || Math.abs(rw - ww) > PARITY_RECT_TOLERANCE_PX || Math.abs(rh - wh) > PARITY_RECT_TOLERANCE_PX) {
      mismatches.push({ path, axis: "rect", react: r.rect, wgpu: w.rect });
    }
    const wColorSrgb = parityLinearToSrgbColor(w.color);
    const wBgSrgb = parityLinearToSrgbColor(w.bg);
    if (!isSceneLeaf && !parityColorClose(r.color, wColorSrgb)) mismatches.push({ path, axis: "color", react: r.color, wgpu: wColorSrgb });
    if (!isSceneLeaf && !parityColorClose(r.bg, wBgSrgb)) mismatches.push({ path, axis: "bg", react: r.bg, wgpu: wBgSrgb });
    if (r.fontSize !== null && w.fontSize !== null && Math.abs(r.fontSize - w.fontSize) > PARITY_FONT_SIZE_TOLERANCE_PX) {
      mismatches.push({ path, axis: "fontSize", react: r.fontSize, wgpu: w.fontSize });
    }
  }
  if (parityNormalizeText(reactDump.focusPath) !== parityNormalizeText(wgpuDump.focusPath)) {
    mismatches.push({ path: "$focus", axis: "focus", react: reactDump.focusPath, wgpu: wgpuDump.focusPath });
  }
  return { status: mismatches.length === 0 ? "PASS" : "FAIL", nodeCount: allPaths.size, mismatches: mismatches.slice(0, 200) };
}
//#endregion 🔖StructuralCompare

//#region 🔖PixelCompare
/** 📐Pixel-region gate covers only structural *containers* (matches the design's "container-level
 * matched node pairs" — navbar/footer/panel-level regions are shell chrome, out of scope for this
 * pass since they're not part of the UiNode tree) plus scene/image leaves, not every leaf text node —
 * bounding pixel-diff cost to O(containers) rather than O(all nodes) per playground. */
const PARITY_PIXEL_REGION_KINDS = new Set(["stack", "field", "section", "group", "tree", "componentScene", "image"]);
const PARITY_PIXEL_THRESHOLD_DEFAULT = 0.005;
const PARITY_PIXEL_THRESHOLD_SCENE = 0.02;

function parityPixelThreshold(kind: string): number {
  return PARITY_SCENE_LEAF_KINDS.has(kind) ? PARITY_PIXEL_THRESHOLD_SCENE : PARITY_PIXEL_THRESHOLD_DEFAULT;
}

function compareParityRegion(reactPng: PNG, wgpuPng: PNG, node: ParityNode, outDir: string, variant: string): PixelRegionResult {
  const [rx, ry, rw, rh] = node.rect;
  const width = Math.max(1, Math.min(Math.round(rw), reactPng.width - Math.round(rx), wgpuPng.width - Math.round(rx)));
  const height = Math.max(1, Math.min(Math.round(rh), reactPng.height - Math.round(ry), wgpuPng.height - Math.round(ry)));
  const threshold = parityPixelThreshold(node.kind);
  if (width <= 0 || height <= 0 || rx < 0 || ry < 0) return { path: node.path, ratio: 0, threshold };
  const reactCrop = new PNG({ width, height });
  const wgpuCrop = new PNG({ width, height });
  PNG.bitblt(reactPng, reactCrop, Math.round(rx), Math.round(ry), width, height, 0, 0);
  PNG.bitblt(wgpuPng, wgpuCrop, Math.round(rx), Math.round(ry), width, height, 0, 0);
  const diff = new PNG({ width, height });
  const mismatched = pixelmatch(reactCrop.data, wgpuCrop.data, diff.data, width, height, { threshold: 0.1, includeAA: false });
  const ratio = mismatched / (width * height);
  let diffPng: string | undefined;
  if (ratio > threshold) {
    diffPng = join(outDir, `diff-${variant}-${node.path.replace(/[^a-zA-Z0-9]+/g, "_")}.png`);
    writeFileSync(diffPng, PNG.sync.write(diff));
  }
  return { path: node.path, ratio, threshold, diffPng };
}
//#endregion 🔖PixelCompare

//#region 🔖Triage
const PARITY_BOOT_TIMEOUT_MS = 45_000;

/** 🪜Boot-triage ladder — each rung is a distinct terminal status, never conflated with a structural/pixel mismatch. */
async function triageParityBoot(page: import("playwright").Page, renderer: ParityRenderer, url: string): Promise<{ readonly status: BootStatus; readonly detail?: string }> {
  try {
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 30_000 });
  } catch (e) {
    return { status: "SERVER-FAIL", detail: String(e) };
  }
  if (renderer === "react") {
    try {
      await page.waitForFunction(() => document.querySelectorAll("#root *").length > 20, { timeout: PARITY_BOOT_TIMEOUT_MS });
    } catch {
      return { status: "BOOT-TIMEOUT", detail: "react #root never populated" };
    }
    const nodeCount = await page.evaluate(() => document.querySelectorAll("[data-ui-path]").length);
    return nodeCount === 0 ? { status: "DUMP-EMPTY", detail: "no data-ui-path nodes" } : { status: "PASS" };
  }
  const consoleErrors: string[] = [];
  page.on("pageerror", (e) => consoleErrors.push(String(e)));
  try {
    await page.waitForFunction(() => document.querySelector("#semio-wgpu-canvas") != null, { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    return { status: "BOOT-TIMEOUT", detail: "wgpu canvas never mounted" };
  }
  if (consoleErrors.some((e) => /NoCompatibleDevice|WebGPU/i.test(e))) return { status: "ENV-FAIL", detail: consoleErrors.join(" | ") };
  try {
    await page.waitForFunction(() => typeof (window as unknown as { wasmBindings?: { dumpStructure?: unknown } }).wasmBindings?.dumpStructure === "function", { timeout: PARITY_BOOT_TIMEOUT_MS });
  } catch {
    return { status: "BOOT-TIMEOUT", detail: "wgpu introspection hook never appeared" };
  }
  const dump = await dumpWgpuStructure(page);
  if (dump.nodes.length === 0) return { status: "DUMP-EMPTY", detail: "wgpu structural dump empty (plugin-bridge/kernel wiring)" };
  const stats = await dumpWgpuFrameStats(page);
  if (stats && stats.drawCalls === 0) return { status: "BLANK-PAINT", detail: "zero draw calls (paint pipeline)" };
  return { status: "PASS" };
}
//#endregion 🔖Triage

//#region 🔖ProbeCatalog
/** 🎬Behavioral probe system — drives semantically-identical interactions on the react and wgpu
 * pages in lockstep (same click/type/key/drag/wheel sequence on both, each side resolving its OWN
 * click/drag/wheel coordinates from its OWN structural dump so the sequence stays semantically
 * identical even when pixel layout differs slightly) and diffs a fresh `compareParityStructural`
 * after every step. Complements `StructuralCompare`/`PixelCompare` (static end-state) and `Triage`
 * (boot) — this is the only sub-region that actually DRIVES interaction, closing the gap this
 * ticket's `verifyParityVariant` had: it previously only ever checked static boot state. */

type ProbeKeyCombo = string; // 🎹 Playwright key-combo syntax, e.g. `"Control+p"`, `"Escape"`.

/** 🔎`exists`/`absent`/`focus`/`text` match a node whose `path` equals OR case-insensitively
 * *contains* the given string (also checked against `kind`) — a probe author usually only knows the
 * semantic identifier ("search"), not the full generated structural path, and loose matching keeps
 * the DSL usable without every catalog entry hardcoding brittle exact paths. */
type ProbeExpectPredicate =
  | { readonly kind: "exists"; readonly path: string }
  | { readonly kind: "absent"; readonly path: string }
  | { readonly kind: "focus"; readonly path: string }
  | { readonly kind: "text"; readonly path: string; readonly equals: string }
  | { readonly kind: "custom"; readonly name: string; readonly check: (dump: ParityDump) => boolean };

type ProbeStep =
  | { readonly kind: "click"; readonly path: string }
  | { readonly kind: "type"; readonly text: string }
  | { readonly kind: "key"; readonly combo: ProbeKeyCombo }
  | { readonly kind: "dragTo"; readonly fromPath: string; readonly toPath: string }
  | { readonly kind: "wheel"; readonly path: string; readonly deltaY: number }
  | { readonly kind: "settle"; readonly ms: number }
  | { readonly kind: "expect"; readonly predicate: ProbeExpectPredicate };

type ProbeStepStatus = "PASS" | "FAIL" | "SKIP";
type ProbeStepResult = {
  readonly index: number;
  readonly step: ProbeStep;
  readonly status: ProbeStepStatus;
  readonly structural?: StructuralResult;
  readonly detail?: string;
};
type ProbeRunResult = { readonly status: "PASS" | "FAIL"; readonly steps: readonly ProbeStepResult[] };
type ParityProbeSuite = { readonly name: string; readonly steps: readonly ProbeStep[] };

function parityRectCenter(rect: ParityRect): readonly [number, number] {
  const [x, y, w, h] = rect;
  return [x + w / 2, y + h / 2];
}

async function parityDumpFor(page: import("playwright").Page, renderer: ParityRenderer): Promise<ParityDump> {
  return renderer === "react" ? dumpReactStructure(page) : dumpWgpuStructure(page);
}

function parityFindNodeExact(dump: ParityDump, path: string): ParityNode | null {
  return dump.nodes.find((n) => n.path === path) ?? null;
}

function parityNodeMatches(dump: ParityDump, needle: string): readonly ParityNode[] {
  const lower = needle.toLowerCase();
  return dump.nodes.filter((n) => n.path === needle || n.path.toLowerCase().includes(lower) || n.kind.toLowerCase().includes(lower));
}

/** 🕹️Executes one non-`expect` step against a single page, resolving click/drag/wheel targets from
 * a dump pulled from THAT SAME page immediately beforehand — never the other renderer's dump, and
 * never a stale one — so react/wgpu layout drift never desyncs which element gets hit. */
async function executeParityStep(
  page: import("playwright").Page,
  renderer: ParityRenderer,
  step: Exclude<ProbeStep, { readonly kind: "expect" }>,
): Promise<{ readonly ok: boolean; readonly detail?: string }> {
  switch (step.kind) {
    case "click": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `click target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.click(cx, cy);
      return { ok: true };
    }
    case "type":
      await page.keyboard.type(step.text);
      return { ok: true };
    case "key":
      await page.keyboard.press(step.combo);
      return { ok: true };
    case "dragTo": {
      const dump = await parityDumpFor(page, renderer);
      const from = parityFindNodeExact(dump, step.fromPath);
      const to = parityFindNodeExact(dump, step.toPath);
      if (!from || !to) return { ok: false, detail: `dragTo target not found: ${!from ? step.fromPath : step.toPath}` };
      const [fx, fy] = parityRectCenter(from.rect);
      const [tx, ty] = parityRectCenter(to.rect);
      await page.mouse.move(fx, fy);
      await page.mouse.down();
      await page.mouse.move(tx, ty, { steps: 8 });
      await page.mouse.up();
      return { ok: true };
    }
    case "wheel": {
      const node = parityFindNodeExact(await parityDumpFor(page, renderer), step.path);
      if (!node) return { ok: false, detail: `wheel target not found: ${step.path}` };
      const [cx, cy] = parityRectCenter(node.rect);
      await page.mouse.move(cx, cy);
      await page.mouse.wheel(0, step.deltaY);
      return { ok: true };
    }
    case "settle":
      await page.waitForTimeout(step.ms);
      return { ok: true };
  }
}

/** ✅Evaluates one `expect` predicate against BOTH sides' freshly-pulled dumps — a predicate only
 * passes the step when it holds on react AND wgpu, since the point is cross-renderer parity, not
 * either renderer in isolation. */
function evaluateParityExpect(predicate: ProbeExpectPredicate, reactDump: ParityDump, wgpuDump: ParityDump): { readonly ok: boolean; readonly detail?: string } {
  const checkOne = (dump: ParityDump): { readonly ok: boolean; readonly detail?: string } => {
    switch (predicate.kind) {
      case "exists": {
        const ok = parityNodeMatches(dump, predicate.path).length > 0;
        return { ok, detail: ok ? undefined : `no node matching "${predicate.path}"` };
      }
      case "absent": {
        const ok = parityNodeMatches(dump, predicate.path).length === 0;
        return { ok, detail: ok ? undefined : `node still present matching "${predicate.path}"` };
      }
      case "focus": {
        const focus = dump.focusPath;
        const ok = focus !== null && (focus === predicate.path || focus.toLowerCase().includes(predicate.path.toLowerCase()));
        return { ok, detail: ok ? undefined : `focusPath "${focus ?? "null"}" does not match "${predicate.path}"` };
      }
      case "text": {
        const node = parityNodeMatches(dump, predicate.path)[0];
        const ok = node !== undefined && parityNormalizeText(node.text) === parityNormalizeText(predicate.equals);
        return { ok, detail: ok ? undefined : `text at "${predicate.path}" is "${node?.text ?? "<missing>"}", expected "${predicate.equals}"` };
      }
      case "custom": {
        const ok = predicate.check(dump);
        return { ok, detail: ok ? undefined : `custom predicate "${predicate.name}" failed` };
      }
    }
  };
  const react = checkOne(reactDump);
  const wgpu = checkOne(wgpuDump);
  const ok = react.ok && wgpu.ok;
  return { ok, detail: ok ? undefined : `react: ${react.detail ?? "ok"} | wgpu: ${wgpu.detail ?? "ok"}` };
}

/** 🏃Runs `steps` on `reactPage`/`wgpuPage` in lockstep — never advances to the next step on either
 * page until the current one finished on both. Non-`expect` steps execute identically on both pages
 * then get a fresh `compareParityStructural` diff; `expect` steps take no page action and just
 * evaluate their predicate against fresh dumps from both. The first `FAIL` halts the run (remaining
 * steps marked `SKIP`) — steps are an ORDERED scenario, not a bag of independent assertions, so a
 * downstream step referencing state a failed step never reached would only add noise. Returns the
 * FULL step trail (not just a final boolean) so a failure is diagnosable by (which step, which axis)
 * — see `ParityMismatchAxis` for the axis vocabulary reused from `StructuralCompare`. */
async function runParityProbe(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, steps: readonly ProbeStep[]): Promise<ProbeRunResult> {
  const results: ProbeStepResult[] = [];
  let halted = false;
  for (let index = 0; index < steps.length; index++) {
    const step = steps[index];
    if (halted) {
      results.push({ index, step, status: "SKIP" });
      continue;
    }
    if (step.kind === "expect") {
      const reactDump = await dumpReactStructure(reactPage);
      const wgpuDump = await dumpWgpuStructure(wgpuPage);
      const outcome = evaluateParityExpect(step.predicate, reactDump, wgpuDump);
      results.push({ index, step, status: outcome.ok ? "PASS" : "FAIL", detail: outcome.detail });
      if (!outcome.ok) halted = true;
      continue;
    }
    const [reactOutcome, wgpuOutcome] = await Promise.all([executeParityStep(reactPage, "react", step), executeParityStep(wgpuPage, "wgpu", step)]);
    if (!reactOutcome.ok || !wgpuOutcome.ok) {
      results.push({ index, step, status: "FAIL", detail: [reactOutcome.detail, wgpuOutcome.detail].filter(Boolean).join(" | ") });
      halted = true;
      continue;
    }
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    results.push({ index, step, status: structural.status, structural });
    if (structural.status === "FAIL") halted = true;
  }
  const status = results.some((r) => r.status === "FAIL") ? "FAIL" : "PASS";
  return { status, steps: results };
}

async function runParityProbeSuite(reactPage: import("playwright").Page, wgpuPage: import("playwright").Page, suite: ParityProbeSuite): Promise<{ readonly name: string } & ProbeRunResult> {
  const result = await runParityProbe(reactPage, wgpuPage, suite.steps);
  return { name: suite.name, ...result };
}

/** 🐚Minimal cross-playground smoke suite — command palette open/close is the one interaction every
 * catalog playground exposes IDENTICALLY, via `useActionHotkey("mod+p", ...)` in
 * `framework/renderer/react/index.tsx` (`mod` accepts `ctrlKey || metaKey`, so `"Control+p"` works
 * regardless of host OS — no need to special-case macOS `"Meta+p"`).
 *
 * KNOWN LIMITATION (confirmed by reading `openStudioE2eCommandPalette` in `🔖StudioE2eVerify` above,
 * and `UISearch` in `framework/renderer/react/index.tsx`): the palette is FRAMEWORK CHROME, not
 * `UiNode`-declared app content — React renders it via shadcn/cmdk (`[role='dialog'] [data-slot=
 * 'command-input']`), which never carries `data-ui-path`, so `REACT_DOM_DUMP_SCRIPT` (see
 * `🔖StructuralDump`) cannot see it at all. The `exists`/`absent` checks below are therefore
 * expected to be unreliable (likely FAIL on the react side) until the structural dump is extended to
 * also tag framework-chrome overlays — a real, scoped follow-up (would also need mirroring into
 * `framework/renderer/wgpu/rs/lib.rs`'s `🔬Introspection` walk, which is a different file, out of
 * reach from this one). Flagging rather than silently "fixing" by touching either renderer's core
 * dump mechanism unverified, per this pass's own constraint of no live browser run to confirm
 * against. */
const PARITY_SHELL_PROBE_SUITE: ParityProbeSuite = {
  name: "shell",
  steps: [
    { kind: "key", combo: "Control+p" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "exists", path: "search" } },
    { kind: "key", combo: "Escape" },
    { kind: "settle", ms: 200 },
    { kind: "expect", predicate: { kind: "absent", path: "search" } },
  ],
};

/** 🗂️Starter catalog — keyed by suite name so `ParityProbeScript`/`verifyParityVariant` can look one
 * up by string. A per-playground text/dnd/scene suite (dragging dock panels, typing into a text
 * editor host, orbiting a 3d scene) is a natural follow-up once `shell` is confirmed working
 * end-to-end against a real live boot — out of scope for this pass per the ticket's own brief. */
const PARITY_PROBE_CATALOG: Readonly<Record<string, ParityProbeSuite>> = {
  shell: PARITY_SHELL_PROBE_SUITE,
};
//#endregion 🔖ProbeCatalog

//#region 🔖ServerPool
/** 🔌Harness dev-server pool — clear of the catalog's per-variant 6012–6205 ports so a sweep never
 * collides with another concurrent dev's running playground. One react+wgpu port pair per shard,
 * reused (restart-between-variants) across that shard's playground list. React bakes its plugin
 * choice at boot via `VITE_SEMIO_PLUGIN` (no runtime `?query=` switch — see `js/index.ts`), so a
 * fresh server per variant is required on both renderers, not just wgpu. */
const PARITY_PORT_BASE = 7300;

function parityPortsForShard(shardIndex: number): { readonly react: number; readonly wgpu: number } {
  const base = PARITY_PORT_BASE + shardIndex * 2;
  return { react: base, wgpu: base + 1 };
}

const PARITY_PORT_POOL_SHARDS = 49; // (7398 - 7300) / 2

/** 🔌`smoke`/`triage`/`verify` are meant to be run by multiple concurrent agents/sessions — hardcoding
 * shard 0 meant every concurrent invocation collided on the same 7300/7301 pair, producing false
 * `SERVER-FAIL`/`DUMP-EMPTY` results indistinguishable from real failures (found the hard way: several
 * parallel boot-triage agents hit exactly this). Scans the shard pool for the first pair where BOTH
 * ports are actually free right now — no coordination needed between callers. */
function findFreeParityPortPair(): { readonly react: number; readonly wgpu: number } {
  for (let shard = 0; shard < PARITY_PORT_POOL_SHARDS; shard++) {
    const candidate = parityPortsForShard(shard);
    if (!isDevPortInUse("127.0.0.1", candidate.react) && !isDevPortInUse("127.0.0.1", candidate.wgpu)) return candidate;
  }
  throw new Error(`no free parity port pair in the ${PARITY_PORT_POOL_SHARDS}-shard pool (${PARITY_PORT_BASE}-${PARITY_PORT_BASE + PARITY_PORT_POOL_SHARDS * 2})`);
}

function parityDevUrl(renderer: ParityRenderer, variant: string, port: number): string {
  return renderer === "wgpu" ? wgpuDevPlayUrl("127.0.0.1", port, variant) : devServerUrl("127.0.0.1", port);
}

type ParityServerHandle = { readonly daemon: SpawnDaemonHandle; readonly port: number };

/** ⏱️A cold `bun ./script.ts dev` boot can mean compiling the ENTIRE plugin crate catalog (33 crates)
 * plus, for wgpu, a from-scratch trunk/cargo build — many minutes with an empty `target/`, not the
 * ~40-60s a warm-cache boot takes. Default generously; `PARITY_BOOT_BUDGET_MS` overrides for CI/tuning. */
const PARITY_DEV_SERVER_BOOT_BUDGET_MS = Number(process.env.PARITY_BOOT_BUDGET_MS ?? 900_000);

async function startParityDevServer(renderer: ParityRenderer, variant: string, port: number): Promise<ParityServerHandle> {
  const devScript = join(repoRoot, "framework/product/os/dev/script.ts");
  const logPath = join(parityOutDir(), `boot-${renderer}-${variant}.log`);
  const logStream = createWriteStream(logPath);
  const daemon = spawnDaemon("bun", [devScript, "dev"], {
    cwd: join(repoRoot, "framework/product/os/dev"),
    env: { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: renderer, S_OS_PORT: String(port) },
    stdio: "pipe",
  });
  daemon.child.stdout?.pipe(logStream);
  daemon.child.stderr?.pipe(logStream);
  const deadline = Date.now() + PARITY_DEV_SERVER_BOOT_BUDGET_MS;
  while (Date.now() < deadline) {
    if (isDevPortInUse("127.0.0.1", port)) return { daemon, port };
    if (daemon.child.exitCode !== null) throw new Error(`${renderer} dev server for ${variant} exited early (code ${daemon.child.exitCode}) — see ${logPath}`);
    await Bun.sleep(500);
  }
  daemon.kill();
  throw new Error(`${renderer} dev server for ${variant} did not open port ${port} within ${PARITY_DEV_SERVER_BOOT_BUDGET_MS}ms — see ${logPath}`);
}

/** 🧹Best-effort: kills the spawned wrapper AND whatever ends up bound to the port, since vite/trunk
 * fork their own child processes that a plain wrapper-kill doesn't always reap. */
function stopParityDevServer(handle: ParityServerHandle): void {
  try {
    handle.daemon.kill();
  } catch {
    /* already gone */
  }
  const occupant = describeDevPortOccupant(handle.port);
  const pid = Number(occupant?.match(/PID (\d+)/)?.[1]);
  if (Number.isFinite(pid)) {
    try {
      process.kill(pid, "SIGTERM");
    } catch {
      /* already gone */
    }
  }
}
//#endregion 🔖ServerPool

//#region 🔖Report
function parityOutDir(): string {
  const dir = process.env.PARITY_OUT_DIR ?? join(repoRoot, ".repo/🎫/26/07/11/WGPU-RENDERER-FULL-PARITY");
  mkdirSync(dir, { recursive: true });
  return dir;
}

function writeParityReport(reports: readonly ParityPlaygroundReport[]): void {
  const outDir = parityOutDir();
  writeFileSync(join(outDir, "parity-report-v2.json"), JSON.stringify(reports, null, 2), "utf8");
  const lines = ["# Wgpu Parity Report (v2 harness)", "", `Generated: ${reports.length} playground(s)`, "", "| Variant | React Boot | Wgpu Boot | Structural | Pixel | Behavioral |", "|---|---|---|---|---|---|"];
  for (const r of reports) lines.push(`| ${r.variant} | ${r.boot.react} | ${r.boot.wgpu} | ${r.structural?.status ?? "-"} | ${r.pixel?.status ?? "-"} | ${r.behavioral?.status ?? "-"} |`);
  const failed = reports.filter((r) => r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL");
  lines.push("", `**${reports.length - failed.length}/${reports.length} PASS**`);
  writeFileSync(join(outDir, "parity-report-v2.md"), lines.join("\n"), "utf8");
}
//#endregion 🔖Report

//#region 🔖Sweep
async function verifyParityVariant(variant: string, ports: { readonly react: number; readonly wgpu: number }, opts: { readonly skipDev?: boolean } = {}): Promise<ParityPlaygroundReport> {
  const start = Date.now();
  const { chromium } = await import("playwright");
  const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
  let reactServer: ParityServerHandle | undefined;
  let wgpuServer: ParityServerHandle | undefined;
  try {
    if (!opts.skipDev) {
      reactServer = await startParityDevServer("react", variant, ports.react);
      wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
    }
    const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
    const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
    const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
    if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
      return { variant, boot: { react: reactBoot.status, wgpu: wgpuBoot.status, detail: reactBoot.detail ?? wgpuBoot.detail }, durationMs: Date.now() - start };
    }
    await reactPage.waitForTimeout(400);
    await wgpuPage.waitForTimeout(400);
    const reactDump = await dumpReactStructure(reactPage);
    const wgpuDump = await dumpWgpuStructure(wgpuPage);
    const structural = compareParityStructural(reactDump, wgpuDump);
    const outDir = parityOutDir();
    const reactPng = PNG.sync.read(await reactPage.screenshot());
    const wgpuPng = PNG.sync.read(await wgpuPage.screenshot());
    const wgpuPaths = new Set(wgpuDump.nodes.map((n) => n.path));
    const regionNodes = reactDump.nodes.filter((n) => PARITY_PIXEL_REGION_KINDS.has(n.kind) && wgpuPaths.has(n.path));
    const regions = regionNodes.map((n) => compareParityRegion(reactPng, wgpuPng, n, outDir, variant));
    const failingRegions = regions.filter((r) => r.ratio > r.threshold);
    // 🎬Runs regardless of the structural/pixel outcome above (not gated on their PASS) — behavioral
    // parity is a distinct axis (interaction-driven dynamic state vs. static end-state), and a
    // static mismatch elsewhere shouldn't hide whether the shell still opens/closes correctly. Wrapped
    // defensively: a probe-runner exception (e.g. a page closing mid-step) must not take down the
    // whole `verifyParityVariant` call, only degrade `behavioral` to a diagnosable FAIL.
    let behavioral: ProbeRunResult | undefined;
    try {
      behavioral = await runParityProbe(reactPage, wgpuPage, PARITY_SHELL_PROBE_SUITE.steps);
    } catch (e) {
      behavioral = { status: "FAIL", steps: [{ index: 0, step: { kind: "settle", ms: 0 }, status: "FAIL", detail: `probe runner threw: ${String(e)}` }] };
    }
    return {
      variant,
      boot: { react: reactBoot.status, wgpu: wgpuBoot.status },
      structural,
      pixel: { status: failingRegions.length === 0 ? "PASS" : "FAIL", regions: failingRegions },
      behavioral,
      durationMs: Date.now() - start,
    };
  } finally {
    await browser.close();
    if (reactServer) stopParityDevServer(reactServer);
    if (wgpuServer) stopParityDevServer(wgpuServer);
  }
}

class ParitySmokeScript extends BundleScript {
  async run(): Promise<void> {
    const variant = process.env.SEMIO_PLUGIN || "s";
    const report = await verifyParityVariant(variant, findFreeParityPortPair());
    console.log(JSON.stringify(report, null, 2));
    if (report.boot.react !== "PASS" || report.boot.wgpu !== "PASS") {
      throw new Error(`parity smoke FAILED: boot react=${report.boot.react} wgpu=${report.boot.wgpu}${report.boot.detail ? ` (${report.boot.detail})` : ""}`);
    }
    console.log(`[DEBUG] parity smoke PASS for ${variant}: structural=${report.structural?.status} pixel=${report.pixel?.status} behavioral=${report.behavioral?.status} (${report.durationMs}ms)`);
  }
}

class ParityTriageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const ports = findFreeParityPortPair();
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    const reactServer = await startParityDevServer("react", variant, ports.react);
    const wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
    try {
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      console.log(`[DEBUG] triage ${variant}: react=${reactBoot.status}${reactBoot.detail ? ` (${reactBoot.detail})` : ""}`);
      console.log(`[DEBUG] triage ${variant}: wgpu=${wgpuBoot.status}${wgpuBoot.detail ? ` (${wgpuBoot.detail})` : ""}`);
    } finally {
      await browser.close();
      stopParityDevServer(reactServer);
      stopParityDevServer(wgpuServer);
    }
  }
}

/** 🎬Standalone entry point for JUST the behavioral probe suite — boots both dev servers, triages
 * boot, then runs `PARITY_PROBE_CATALOG[suiteName]` (default `"shell"`) without paying for the
 * structural/pixel comparison `verifyParityVariant` also does. Useful for iterating on a probe suite
 * itself without re-running the (slower) full `verify`. */
class ParityProbeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variant = segments[0] || process.env.SEMIO_PLUGIN || "s";
    const suiteName = segments[1] || "shell";
    const suite = PARITY_PROBE_CATALOG[suiteName];
    if (!suite) throw new Error(`unknown probe suite: ${suiteName} (known: ${Object.keys(PARITY_PROBE_CATALOG).join(", ")})`);
    const ports = findFreeParityPortPair();
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: process.env.HEADED !== "1", args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
    const reactServer = await startParityDevServer("react", variant, ports.react);
    const wgpuServer = await startParityDevServer("wgpu", variant, ports.wgpu);
    try {
      const reactPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const wgpuPage = await browser.newPage({ viewport: { width: 1280, height: 720 } });
      const reactBoot = await triageParityBoot(reactPage, "react", parityDevUrl("react", variant, ports.react));
      const wgpuBoot = await triageParityBoot(wgpuPage, "wgpu", parityDevUrl("wgpu", variant, ports.wgpu));
      if (reactBoot.status !== "PASS" || wgpuBoot.status !== "PASS") {
        throw new Error(`parity probe FAILED: boot react=${reactBoot.status} wgpu=${wgpuBoot.status}${reactBoot.detail ?? wgpuBoot.detail ? ` (${reactBoot.detail ?? wgpuBoot.detail})` : ""}`);
      }
      const result = await runParityProbeSuite(reactPage, wgpuPage, suite);
      console.log(JSON.stringify(result, null, 2));
      console.log(`[DEBUG] probe ${variant}/${suiteName}: ${result.status} (${result.steps.length} step(s))`);
      if (result.status !== "PASS") throw new Error(`parity probe ${variant}/${suiteName} FAILED`);
    } finally {
      await browser.close();
      stopParityDevServer(reactServer);
      stopParityDevServer(wgpuServer);
    }
  }
}

class ParityVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const variants = segments.filter((s) => !s.startsWith("--"));
    if (variants.length === 0) throw new Error("usage: parity verify <variant…>");
    const skipDev = process.env.SKIP_DEV === "1";
    const ports = skipDev ? parityPortsForShard(0) : findFreeParityPortPair();
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      const report = await verifyParityVariant(variant, ports, { skipDev });
      reports.push(report);
      console.log(`[DEBUG] ${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    const failed = reports.filter((r) => r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL");
    if (failed.length > 0) throw new Error(`parity verify: ${failed.length}/${reports.length} playground(s) failed`);
  }
}

class ParitySweepScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const shardArg = segments.find((s) => s.startsWith("--shard="))?.slice("--shard=".length);
    const [shardIndex, shardCount] = shardArg ? shardArg.split("/").map(Number) : [0, 1];
    const variants = playgroundCatalog.map((r) => r.variant).filter((_, i) => i % (shardCount ?? 1) === (shardIndex ?? 0));
    const reports: ParityPlaygroundReport[] = [];
    for (const variant of variants) {
      const report = await verifyParityVariant(variant, parityPortsForShard(shardIndex ?? 0));
      reports.push(report);
      console.log(`[DEBUG] sweep ${variant}: boot=${report.boot.react}/${report.boot.wgpu} structural=${report.structural?.status ?? "-"} pixel=${report.pixel?.status ?? "-"} behavioral=${report.behavioral?.status ?? "-"}`);
    }
    writeParityReport(reports);
    const failed = reports.filter((r) => r.boot.react !== "PASS" || r.boot.wgpu !== "PASS" || r.structural?.status === "FAIL" || r.pixel?.status === "FAIL" || r.behavioral?.status === "FAIL");
    console.log(`[DEBUG] parity sweep complete: ${reports.length - failed.length}/${reports.length} PASS`);
    if (failed.length > 0) throw new Error(`parity sweep: ${failed.length}/${reports.length} playground(s) failed`);
  }
}
//#endregion 🔖Sweep
//#endregion 🔬ParityScript

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("verify", VerifyScript)
  .register(
    "parity",
    class extends BundleScript {
      async run(segments: string[]): Promise<void> {
        const sub = segments[0];
        if (sub === "smoke") return new ParitySmokeScript(this.root).run(segments.slice(1));
        if (sub === "triage") return new ParityTriageScript(this.root).run(segments.slice(1));
        if (sub === "probe") return new ParityProbeScript(this.root).run(segments.slice(1));
        if (sub === "verify") return new ParityVerifyScript(this.root).run(segments.slice(1));
        if (sub === "sweep") return new ParitySweepScript(this.root).run(segments.slice(1));
        throw new Error(`unknown parity subcommand: ${sub} (expected smoke|triage|probe|verify|sweep)`);
      }
    },
  )
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
        // 🐛`sub` here is the variant filter itself (e.g. `plugin cad`), not a subcommand to strip —
        // slicing it off silently dropped the filter and fell back to building the entire 33-crate
        // catalog for every `bun ./script.ts plugin <variant>` invocation.
        return new PluginBuildScript(this.root).run(segments);
      }
    },
  );

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
}
