import { closeSync, lstatSync, mkdirSync, openSync, readdirSync, readFileSync, readSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import { createHash, hash, randomBytes } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

/** 🗃️ The on-disk index format version; see `./🧬️schema/🔣️.json`. */
export const RECORD_STORE_FORMAT = 1;

const RECORD_KEY = /^[0-9a-f]{64}$/u;
const PACK_NAME = /^[0-9a-z]+-[0-9]+-[0-9a-f]{16}$/u;
const PACK_TEMPORARY = /^[0-9a-z]+-[0-9]+-[0-9a-f]{16}\.(?:pack|idx)\.[0-9a-f]{8}\.tmp$/u;
const ORPHAN_MS = schema.definitions.Limits.const.orphanMilliseconds;
const PENDING_FLUSH_BYTES = schema.definitions.Limits.const.pendingFlushBytes;

/** 🔌️ Packs keep at most this many read descriptors open across every store of the process: the least recently read one is closed first, so a graph that visits thousands of project stores never exhausts the descriptor table. */
export const RECORD_STORE_DESCRIPTOR_LIMIT = schema.definitions.Limits.const.descriptorLimit;
const openDescriptors = new Map<string, () => void>();

/** 📦️ A store holding more packs than this is compacted into one pack once its writer flushed, so every process that adds records no longer leaves a pack and an index behind for good. */
export const RECORD_STORE_PACK_LIMIT = 4;

/** 📹️ Counts the pack descriptors this process currently keeps open (tests and diagnostics). */
export function openRecordStoreDescriptorCount(): number { return openDescriptors.size; }

/** 📇️ One pack index as persisted next to its pack: every record's key, byte range and value digest prefix. */
export interface RecordStoreIndex {
  readonly format: typeof RECORD_STORE_FORMAT;
  readonly pack: string;
  readonly packBytes: number;
  readonly records: readonly (readonly [key: string, offset: number, length: number, digest: string])[];
}

/** 📊️ Counters a caller reports to prove the store's effect. */
export interface RecordStoreStats { readonly hits: number; readonly misses: number; readonly puts: number; readonly packs: number; readonly records: number; readonly accessed: number }

/**
 * 🗄️ A content-addressed, append-only record store: values are opaque bytes under 64-hex keys that the caller derives from every input
 * of the computation that produced them, so a hit is indistinguishable from recomputing. Records are written in immutable packs whose
 * index is published by rename after the pack, so concurrent processes only ever see complete packs; a damaged or vanished record reads
 * as a miss, never as wrong bytes (each value is verified against its recorded digest).
 */
export interface RecordStore {
  has(key: string): boolean;
  get(key: string): Uint8Array | undefined;
  put(key: string, value: Uint8Array): void;
  flush(): void;
  retainAccessed(): void;
  compact(): void;
  close(): void;
  readonly stats: RecordStoreStats;
}

interface Location { readonly pack: string; readonly offset: number; readonly length: number; readonly digest: string }

/** 🧾️ The first 32 hex digits of a value's SHA-256, enough to reject damaged bytes. */
function valueDigest(value: Uint8Array): string {
  return hash("sha256", value, "hex").slice(0, 32);
}

/** ✅️ Parses one persisted index, or `null` when it is not exactly a well-formed index of a complete pack named like it. */
export function parseRecordStoreIndex(text: string, name: string, packBytes: number | null): RecordStoreIndex | null {
  let value: unknown;
  try { value = JSON.parse(text); } catch { return null; }
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const index = value as Record<string, unknown>;
  if (Object.keys(index).sort().join() !== "format,pack,packBytes,records" || index.format !== RECORD_STORE_FORMAT || index.pack !== name || !Number.isSafeInteger(index.packBytes) || index.packBytes !== packBytes || !Array.isArray(index.records)) return null;
  let end = 0;
  for (const record of index.records as unknown[]) {
    if (!Array.isArray(record) || record.length !== 4) return null;
    const [key, offset, length, digest] = record as unknown[];
    if (typeof key !== "string" || !RECORD_KEY.test(key) || offset !== end || !Number.isSafeInteger(length) || (length as number) < 0 || typeof digest !== "string" || !/^[0-9a-f]{32}$/u.test(digest)) return null;
    end += length as number;
  }
  return end === packBytes ? index as unknown as RecordStoreIndex : null;
}

/** 🏷️ A fresh pack name: creation time, process and randomness, so concurrent writers never collide. */
function packName(): string {
  return `${Date.now().toString(36)}-${process.pid}-${randomBytes(8).toString("hex")}`;
}

/** ✍️ Publishes one pack: the bytes first, then the index by rename, so a reader that sees the index sees the whole pack. */
/** 🎚️ Admits complete optional pack peaks before any bytes are published. */
export interface RecordStorePublicationV1 {readonly peakBytes:number;readonly retainedDeltaBytes:number}
export interface RecordStoreOptionsV1 {readonly publication?:(plan:RecordStorePublicationV1,write:()=>void)=>boolean}

function writePack(directory:string,records:Iterable<readonly[string,Uint8Array]>,options:RecordStoreOptionsV1,retiredBytes=0,retire=()=>{}):RecordStoreIndex|undefined {
 const values=[...records],name=packName(),packPath=join(directory,`${name}.pack`),temporary=`${packPath}.${randomBytes(4).toString("hex")}.tmp`,rows:[string,number,number,string][]=[];let offset=0;
 for(const[key,value]of values){rows.push([key,offset,value.byteLength,valueDigest(value)]);offset+=value.byteLength;}
 const index:RecordStoreIndex={format:RECORD_STORE_FORMAT,pack:name,packBytes:offset,records:rows},image=JSON.stringify(index),indexPath=join(directory,`${name}.idx`),indexTemporary=`${indexPath}.${randomBytes(4).toString("hex")}.tmp`,bytes=offset+Buffer.byteLength(image);
 const write=()=>{const fd=openSync(temporary,"wx");try{for(const[,value]of values)for(let written=0;written<value.byteLength;)written+=writeSync(fd,value,written,value.byteLength-written);}finally{closeSync(fd);}renameSync(temporary,packPath);writeFileSync(indexTemporary,image,{flag:"wx"});renameSync(indexTemporary,indexPath);retire();};
 if(options.publication?!options.publication({peakBytes:bytes,retainedDeltaBytes:bytes-retiredBytes},write):(write(),false))return undefined;
 return index;
}

/** 🧹️ Removes one file, tolerating it being gone or held open (a later compaction retries). */
function removeQuietly(path: string): void {
  try { rmSync(path, { force: true }); } catch {}
}

/** 📂️ Opens the store rooted at `directory` (created on demand), loading every complete pack index present. */
export function openRecordStore(directory: string, options:RecordStoreOptionsV1={}): RecordStore {
  const root = resolve(directory);
  mkdirSync(root, { recursive: true });
  const locations = new Map<string, Location>(), packs = new Set<string>(), descriptors = new Map<string, number | null>();
  const pending = new Map<string, Uint8Array>(), accessed = new Set<string>(), damaged = new Set<string>();
  let hits = 0, misses = 0, puts = 0, pendingBytes = 0;
  const now = Date.now();
  for (const entry of readdirSync(root).sort()) {
    if (entry.endsWith(".tmp")) {
      if(options.publication&&!PACK_TEMPORARY.test(entry))continue;
      const path=join(root,entry);let original;try{original=lstatSync(path);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;continue;}
      if(now-original.mtimeMs>ORPHAN_MS){
        if(options.publication){
          if(!original.isFile()||original.isSymbolicLink()||original.nlink!==1)throw Error("Original optional record temporary leaf required");
          options.publication({peakBytes:0,retainedDeltaBytes:-original.size},()=>{const current=lstatSync(path);if(!current.isFile()||current.isSymbolicLink()||current.nlink!==1||current.dev!==original.dev||current.ino!==original.ino||current.size!==original.size||current.mtimeMs!==original.mtimeMs)throw Error("Original optional record temporary identity changed");rmSync(path);});
        }else removeQuietly(path);
      }
      continue;
    }
    if (!entry.endsWith(".idx")) continue;
    const name = entry.slice(0, -4);
    if (!PACK_NAME.test(name)) continue;
    let packBytes: number | null = null;
    try { packBytes = statSync(join(root, `${name}.pack`)).size; } catch {}
    let text: string;
    try { text = readFileSync(join(root, entry), "utf8"); } catch { continue; }
    const index = parseRecordStoreIndex(text, name, packBytes);
    if (!index) continue;
    packs.add(name);
    for (const [key, offset, length, digest] of index.records) locations.set(key, { pack: name, offset, length, digest });
  }
  const release = (pack: string): void => {
    const fd = descriptors.get(pack);
    openDescriptors.delete(`${root}\0${pack}`);
    descriptors.delete(pack);
    if (typeof fd === "number") { try { closeSync(fd); } catch {} }
  };
  const descriptor = (pack: string): number | null => {
    const key = `${root}\0${pack}`, held = descriptors.get(pack);
    if (typeof held === "number") { openDescriptors.delete(key); openDescriptors.set(key, () => release(pack)); return held; }
    if (held === null) return null;
    while (openDescriptors.size >= RECORD_STORE_DESCRIPTOR_LIMIT) { const [oldest, evict] = openDescriptors.entries().next().value!; openDescriptors.delete(oldest); evict(); }
    try { const fd = openSync(join(root, `${pack}.pack`), "r"); descriptors.set(pack, fd); openDescriptors.set(key, () => release(pack)); return fd; }
    catch { descriptors.set(pack, null); return null; }
  };
  const read = (key: string): Uint8Array | undefined => {
    const staged = pending.get(key);
    if (staged) return staged;
    const location = locations.get(key);
    if (!location) return undefined;
    const fd = descriptor(location.pack), bytes = Buffer.allocUnsafe(location.length);
    let filled = 0;
    try { while (fd !== null && filled < location.length) { const count = readSync(fd, bytes, filled, location.length - filled, location.offset + filled); if (count === 0) break; filled += count; } } catch {}
    if (filled === location.length && valueDigest(bytes) === location.digest) return bytes;
    damaged.add(key);
    return undefined;
  };
  const rewrite = (keys: Iterable<string>): void => {
    store.flush();
    const live = function* (): Generator<readonly [string, Uint8Array]> { for (const key of [...keys].sort()) { const value = read(key); if (value !== undefined) yield [key, value]; } };
    const retired=[...packs],retiredBytes=retired.reduce((sum,pack)=>sum+statSync(join(root,`${pack}.idx`)).size+statSync(join(root,`${pack}.pack`)).size,0);
    const index=writePack(root,live(),options,retiredBytes,()=>{for(const pack of retired){release(pack);for(const suffix of["idx","pack"])rmSync(join(root,`${pack}.${suffix}`));packs.delete(pack);}});
    if(!index)return;
    locations.clear();
    for (const [key, offset, length, digest] of index.records) locations.set(key, { pack: index.pack, offset, length, digest });
    packs.add(index.pack);
  };
  const store: RecordStore = {
    has(key) { return pending.has(key) || locations.has(key) && !damaged.has(key); },
    get(key) {
      const value = read(key);
      if (value === undefined) misses++;
      else { hits++; accessed.add(key); }
      return value;
    },
    put(key, value) {
      if (!RECORD_KEY.test(key)) throw new Error(`Record key must be 64 lowercase hex digits: ${key}`);
      accessed.add(key);
      if (locations.has(key) && !damaged.has(key) || pending.has(key)) return;
      pending.set(key, value);
      puts++;
      pendingBytes += value.byteLength;
      if (pendingBytes >= PENDING_FLUSH_BYTES) store.flush();
    },
    flush() {
      if (pending.size === 0) return;
      const index = writePack(root, [...pending],options);
      if(!index){pending.clear();pendingBytes=0;return;}
      packs.add(index.pack);
      for (const [key, offset, length, digest] of index.records) { locations.set(key, { pack: index.pack, offset, length, digest }); damaged.delete(key); }
      pending.clear();
      pendingBytes = 0;
    },
    retainAccessed() { rewrite(accessed); },
    compact() { rewrite(locations.keys()); },
    close() {
      try{store.flush();}finally{
      for (const pack of [...descriptors.keys()]) release(pack);
      descriptors.clear();}
    },
    get stats() { return { hits, misses, puts, packs: packs.size, records: locations.size + pending.size, accessed: accessed.size }; },
  };
  return store;
}

/** 🔑️ The record key of one computation: SHA-256 over its namespace and every input, each length-prefixed so no two input lists collide. */
export function recordKey(namespace: string, ...inputs: readonly (string | Uint8Array)[]): string {
  const parts = [namespace, ...inputs];
  if (parts.every((part) => typeof part === "string")) {
    let text = "";
    for (const part of parts as string[]) text += `${Buffer.byteLength(part)}:${part}`;
    return hash("sha256", text, "hex");
  }
  const chained = createHash("sha256");
  for (const part of parts) chained.update(`${typeof part === "string" ? Buffer.byteLength(part) : part.byteLength}:`).update(part);
  return chained.digest("hex");
}

const STATIC_IMPORT = /(?:^|[\s;])(?:import|export)\s[^"'`;]*?\bfrom\s*["']([^"']+)["']|(?:^|[\s;])import\s*["']([^"']+)["']|\bimport\s*\(\s*["']([^"']+)["']\s*\)/gu;

/**
 * 🧬️ A digest of one module's source closure: its own bytes plus, recursively, every relative module it imports statically or through a
 * literal dynamic `import()`, in path order. Records keyed by it are invalidated by any edit to the code that could have produced them.
 */
export function moduleSourceClosureDigest(entry: string): string {
  const seen = new Map<string, Buffer>(), pending = [resolve(entry)];
  while (pending.length > 0) {
    const file = pending.pop()!;
    if (seen.has(file)) continue;
    const bytes = readFileSync(file);
    seen.set(file, bytes);
    for (const match of bytes.toString("utf8").matchAll(STATIC_IMPORT)) {
      const specifier = match[1] ?? match[2] ?? match[3];
      if (specifier?.startsWith(".")) pending.push(resolve(dirname(file), specifier));
    }
  }
  const base = dirname(resolve(entry));
  return recordKey("module-source-closure-v1", ...[...seen].map(([file, bytes]) => [relative(base, file).replaceAll("\\", "/"), bytes] as const).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).flatMap(([file, bytes]) => [file, bytes]));
}

/** 🧺️ Removes sibling store namespaces of `root` other than `keep` that nobody touched for an hour (older code revisions). */
export function pruneRecordStoreNamespaces(root: string, keep: string): void {
  let names: string[];
  try { names = readdirSync(root); } catch { return; }
  const now = Date.now();
  for (const name of names) {
    if (name === keep) continue;
    const path = join(root, name);
    try { const stat = statSync(path); if (stat.isDirectory() && now - stat.mtimeMs > ORPHAN_MS) rmSync(path, { recursive: true, force: true }); } catch {}
  }
}
