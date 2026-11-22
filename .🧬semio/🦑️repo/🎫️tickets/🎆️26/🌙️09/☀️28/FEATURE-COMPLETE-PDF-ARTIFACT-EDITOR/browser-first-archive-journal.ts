/** 📜 Reads and writes the shared folder journal and admits a private paged first-genesis capture. */
import { createHash } from "node:crypto";
import { type FileHandle, mkdir, mkdtemp, open, realpath, rm } from "node:fs/promises";
import { dirname, join } from "node:path";

const MAGIC = Buffer.from("SEMIOEL1");
const ARCHIVE_PUT = 4;
const FNV_OFFSET = 0xcbf29ce484222325n;
const FNV_PRIME = 0x100000001b3n;
const HEADER = 24;
const PAGE = 4096;
const MAXIMUM_ARCHIVE = 4 * 1024 * 1024;
const MAXIMUM_RECORD = 16 * 1024 * 1024 * 1024;
const TAIL = Buffer.alloc(8);

type Live = () => void;
type ArchiveRange = { start: number; length: number; matchingSchema: boolean };
type Scan = { validEnd: number; latest: ArchiveRange | "pair" | undefined; archive: ArchiveRange | undefined };
type Writer = <T>(path: string, write: () => Promise<T>) => Promise<T>;
type Frontier = Readonly<{ documentId: string; generation: bigint; extent: number; revision: Uint8Array }>;
type ArchiveReceipt = Readonly<{ frontier: Frontier; extent: number; digest: Uint8Array }>;
export type FolderFirstArchiveCapture = Readonly<{
  write(offset: number, page: Uint8Array): Promise<{ offset: number; length: number }>;
  publish(digest: Uint8Array, signal: AbortSignal): Promise<ArchiveReceipt>;
  retire(): Promise<void>;
}>;

const writers = new Map<string, Promise<void>>();
const active = () => {};

/** 📍 Names the canonical event log for the current folder. */
export function eventLogPath(folder: string): string {
  return join(folder, ".semio", "events.semio");
}

/** 🔐 Serializes all dev-provider appends through the resolved physical journal directory. */
export async function withFolderEventWriter<T>(path: string, work: () => Promise<T>): Promise<T> {
  const directory = dirname(path);
  await mkdir(directory, { recursive: true });
  const key = await realpath(directory);
  const previous = writers.get(key) ?? Promise.resolve();
  const result = previous.then(work);
  const ready = result.then(active, active);
  writers.set(key, ready);
  try { return await result; }
  finally { if (writers.get(key) === ready) writers.delete(key); }
}

function fail(detail: string): Error {
  return new Error(`folder event log: ${detail}`);
}

function errorCode(error: unknown): unknown {
  return error instanceof Error && "code" in error ? error.code : undefined;
}

async function inputFile(path: string): Promise<FileHandle | null> {
  try { return await open(path, "r"); }
  catch (error) { if (errorCode(error) === "ENOENT") return null; throw error; }
}

async function writerFile(path: string): Promise<FileHandle> {
  try { return await open(path, "r+"); }
  catch (error) { if (errorCode(error) !== "ENOENT") throw error; }
  try { return await open(path, "wx+"); }
  catch (error) { if (errorCode(error) !== "EEXIST") throw error; return open(path, "r+"); }
}

async function syncDirectory(path: string): Promise<void> {
  if (process.platform === "win32") return;
  const directory = await open(path, "r");
  try { await directory.sync(); }
  finally { await directory.close(); }
}

async function readExact(file: FileHandle, output: Uint8Array, position: number): Promise<void> {
  for (let cursor = 0; cursor < output.length;) {
    const { bytesRead } = await file.read(output, cursor, output.length - cursor, position + cursor);
    if (bytesRead === 0) throw fail("unexpected end of file");
    cursor += bytesRead;
  }
}

async function writeExact(file: FileHandle, bytes: Uint8Array, position: number): Promise<void> {
  for (let cursor = 0; cursor < bytes.length;) {
    const { bytesWritten } = await file.write(bytes, cursor, bytes.length - cursor, position + cursor);
    if (bytesWritten === 0) throw fail("unable to advance write");
    cursor += bytesWritten;
  }
}

async function readPages(
  file: FileHandle, start: number, length: number, page: Buffer, live: Live,
  visit: (bytes: Buffer, offset: number) => void | Promise<void>,
): Promise<void> {
  for (let offset = 0; offset < length;) {
    live();
    const bytes = page.subarray(0, Math.min(page.length, length - offset));
    await readExact(file, bytes, start + offset);
    await visit(bytes, offset);
    offset += bytes.length;
  }
  live();
}

function checksum(hash: bigint, bytes: Uint8Array): bigint {
  for (const byte of bytes) hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * FNV_PRIME);
  return hash;
}

async function hashRange(file: FileHandle, range: ArchiveRange, page: Buffer, live: Live): Promise<Buffer> {
  const hash = createHash("sha256");
  await readPages(file, range.start, range.length, page, live, (bytes) => { hash.update(bytes); });
  return hash.digest();
}

async function scan(file: FileHandle, size: number, key: Buffer, schema: Buffer, page: Buffer, live: Live, from = 0): Promise<Scan> {
  if (!Number.isSafeInteger(size)) throw fail("journal extent exceeds exact offsets");
  const header = Buffer.alloc(HEADER);
  let latest: Scan["latest"], archive: ArchiveRange | undefined;
  let validEnd = from;
  while (validEnd < size) {
    live();
    const available = Math.min(HEADER, size - validEnd);
    await readExact(file, header.subarray(0, available), validEnd);
    const magicLength = Math.min(MAGIC.length, available);
    if (!header.subarray(0, magicLength).equals(MAGIC.subarray(0, magicLength))) throw fail("invalid magic");
    if (available < HEADER) break;
    const length = Number(header.readBigUInt64LE(8));
    if (length > MAXIMUM_RECORD) throw fail("record exceeds boundary");
    const start = validEnd + HEADER;
    if (length > size - start) break;
    const end = start + length;
    let hash = FNV_OFFSET;
    await readPages(file, start, length, page, live, (bytes) => { hash = checksum(hash, bytes); });
    if (hash !== header.readBigUInt64LE(16)) {
      if (end === size) break;
      throw fail("checksum mismatch");
    }
    let cursor = start;
    const consume = (length: number): number => {
      if (length > end - cursor) throw fail("truncated record");
      const at = cursor;
      cursor += length;
      return at;
    };
    const text = async (expected: Buffer): Promise<boolean> => {
      await readExact(file, header.subarray(0, 4), consume(4));
      const length = header.readUInt32LE(0);
      const at = consume(length);
      let matches = length === expected.length;
      const utf8 = new TextDecoder("utf-8", { fatal: true });
      await readPages(file, at, length, page, live, (bytes, offset) => {
        utf8.decode(bytes, { stream: true });
        matches &&= bytes.equals(expected.subarray(offset, offset + bytes.length));
      });
      utf8.decode();
      return matches;
    };
    const data = async (): Promise<{ start: number; length: number }> => {
      await readExact(file, header.subarray(0, 8), consume(8));
      const length = Number(header.readBigUInt64LE(0));
      return { start: consume(length), length };
    };
    await readExact(file, header.subarray(0, 9), consume(9));
    const kind = header[0]!;
    if (kind < 1 || kind > 4) throw fail("unknown record kind");
    const matchingKey = await text(key);
    const matchingSchema = await text(schema);
    const primary = await data();
    await data();
    if (cursor !== end) throw fail("record has trailing bytes");
    if (matchingKey && kind === ARCHIVE_PUT) latest = archive = { ...primary, matchingSchema };
    else if (matchingKey && kind === 1) latest = "pair";
    validEnd = end;
  }
  return { validEnd, latest, archive };
}

function prefix(key: Buffer, schema: Buffer, extent: number): Buffer {
  const output = Buffer.alloc(9 + 4 + key.length + 4 + schema.length + 8);
  output[0] = ARCHIVE_PUT;
  output.writeBigUInt64LE(BigInt(Date.now()), 1);
  let cursor = 9;
  for (const text of [key, schema]) {
    output.writeUInt32LE(text.length, cursor);
    output.set(text, cursor + 4);
    cursor += 4 + text.length;
  }
  output.writeBigUInt64LE(BigInt(extent), cursor);
  return output;
}

async function appendRecord(
  file: FileHandle, path: string, fileSize: number, start: number, head: Buffer, extent: number, sum: bigint, live: Live,
  archive: (position: number) => Promise<void>, verify: (expected: ArchiveRange, end: number) => Promise<void>,
): Promise<void> {
  const header = Buffer.alloc(HEADER);
  const length = head.length + extent + TAIL.length;
  header.set(MAGIC);
  header.writeBigUInt64LE(BigInt(length), 8);
  header.writeBigUInt64LE(sum, 16);
  const archiveStart = start + HEADER + head.length;
  live();
  if ((await file.stat()).size !== fileSize) throw fail("writer frontier changed");
  if (start !== fileSize) await file.truncate(start);
  try {
    live();
    await writeExact(file, header, start);
    await writeExact(file, head, start + HEADER);
    await archive(archiveStart);
    live();
    await writeExact(file, TAIL, archiveStart + extent);
    await file.sync();
    await verify({ start: archiveStart, length: extent, matchingSchema: true }, start + HEADER + length);
    await syncDirectory(dirname(path));
    await syncDirectory(dirname(dirname(path)));
    live();
  } catch (error) {
    try { await file.truncate(start); await file.sync(); }
    catch (rollback) { throw new AggregateError([error, rollback], "folder event log rollback failed"); }
    throw error;
  }
}

/** 📖 Returns the latest archive through the existing ordinary read interface. */
export async function readLatestArchive(path: string, documentId: string): Promise<Uint8Array | null> {
  try { await realpath(dirname(path)); }
  catch (error) { if (errorCode(error) === "ENOENT") return null; throw error; }
  return withFolderEventWriter(path, async () => {
    const file = await inputFile(path);
    if (!file) return null;
    try {
      const page = Buffer.alloc(PAGE);
      const { archive } = await scan(file, (await file.stat()).size, Buffer.from(documentId), Buffer.alloc(0), page, active);
      if (!archive) return null;
      if (archive.length > MAXIMUM_ARCHIVE) throw fail("archive exceeds format boundary");
      const output = new Uint8Array(archive.length);
      await readPages(file, archive.start, archive.length, page, active, (bytes, offset) => output.set(bytes, offset));
      return output;
    } finally { await file.close(); }
  });
}

/** ✍ Appends the ordinary caller’s already assembled archive through the shared writer queue. */
export async function appendArchive(path: string, documentId: string, schema: string, archive: Uint8Array): Promise<void> {
  await withFolderEventWriter(path, async () => {
    const file = await writerFile(path);
    try {
      const key = Buffer.from(documentId);
      const metadata = Buffer.from(schema);
      const fileSize = (await file.stat()).size;
      const page = Buffer.alloc(PAGE);
      const { validEnd } = await scan(file, fileSize, key, metadata, page, active);
      const head = prefix(key, metadata, archive.length);
      const sum = checksum(checksum(checksum(FNV_OFFSET, head), archive), TAIL);
      await appendRecord(file, path, fileSize, validEnd, head, archive.length, sum, active, async (position) => {
        for (let offset = 0; offset < archive.length; offset += PAGE) {
          await writeExact(file, archive.subarray(offset, offset + PAGE), position + offset);
        }
      }, async () => {});
    } finally { await file.close(); }
  });
}

async function validateFirstArchive(file: FileHandle, extent: number, page: Buffer, live: Live): Promise<void> {
  await readExact(file, page.subarray(0, 1), 0);
  if (page[0] !== 1) throw fail("unexpected archive version");
  let cursor = 1;
  const varint = async (): Promise<number> => {
    let length = 0;
    for (let shift = 0; shift <= 21 && cursor < extent; shift += 7) {
      live();
      await readExact(file, page.subarray(0, 1), cursor++);
      const byte = page[0]!;
      length += (byte & 127) * 2 ** shift;
      if (byte < 128 && (shift === 0 || byte !== 0)) return length;
      if (byte === 0) break;
    }
    throw fail("invalid archive length");
  };
  for (let field = 0; field < 2; field++) {
    const length = await varint();
    if (length === 0 || length >= extent - cursor) throw fail("invalid parent archive field");
    cursor += length;
  }
  if (await varint() !== 0 || cursor !== extent) throw fail("first archive must contain exactly its parent pair");
}

/** 🧾 Captures one exact frontier into a private spool, then verifies the durable kind-4 record. */
export async function captureFirstFolderArchive(
  path: string, schema: string, frontier: Frontier, current: () => boolean, withWriter: Writer,
): Promise<FolderFirstArchiveCapture> {
  const key = Buffer.from(frontier.documentId), metadata = Buffer.from(schema);
  const revision = Uint8Array.from(frontier.revision);
  const { documentId, generation, extent } = frontier;
  if (key.length === 0 || key.length > 512 || key.includes(0) || metadata.length === 0 || metadata.length > 1024 ||
      metadata.includes(0) || !Number.isInteger(extent) || extent < 1 || extent > MAXIMUM_ARCHIVE ||
      revision.length !== 32 || generation < 0n || generation > 0xffffffffffffffffn) throw fail("invalid first archive frontier");
  const check = (signal?: AbortSignal): void => {
    signal?.throwIfAborted();
    if (!current()) throw fail("first archive folder authority changed");
  };
  check();
  const directory = dirname(path);
  await mkdir(directory, { recursive: true });
  const temporary = await mkdtemp(join(directory, ".archive-"));
  let file: FileHandle;
  try { file = await open(join(temporary, "pages"), "wx+", 0o600); }
  catch (error) { await rm(temporary, { recursive: true }); throw error; }
  const page = Buffer.alloc(PAGE);
  const captured = createHash("sha256");
  let written = 0;
  let failed = false;
  let writing = false;
  let retired = false;
  let operation = Promise.resolve();
  let receipt: ArchiveReceipt | undefined;
  const enqueue = <T>(work: () => Promise<T>): Promise<T> => {
    const result = operation.then(work);
    operation = result.then(active, active);
    return result;
  };
  const dispose = async () => {
    if (!retired) { await file.close(); retired = true; }
    await rm(temporary, { recursive: true, force: true });
  };
  const result = (digest: Uint8Array): ArchiveReceipt => ({
    extent, digest: Uint8Array.from(digest), frontier: { documentId, generation, extent, revision: revision.slice() },
  });
  try { check(); }
  catch (error) { await dispose(); throw error; }
  return {
    write(offset, bytes) {
      if (!Number.isInteger(offset) || offset < 0 || offset >= extent || offset % PAGE !== 0 ||
          bytes.length !== Math.min(PAGE, extent - offset)) return Promise.reject(fail("invalid archive page"));
      if (writing) return Promise.reject(fail("archive page acknowledgement is pending"));
      writing = true;
      const owned = Buffer.from(bytes);
      return enqueue(async () => {
        check();
        if (retired || failed || receipt) throw fail("archive capture is closed");
        if (offset === written) {
          try { await writeExact(file, owned, offset); }
          catch (error) { failed = true; throw error; }
          captured.update(owned);
          written += owned.length;
        } else if (offset + owned.length === written) {
          await readExact(file, page.subarray(0, owned.length), offset);
          if (!owned.equals(page.subarray(0, owned.length))) throw fail("archive retry changed bytes");
        } else throw fail("archive page changed order");
        check();
        return { offset, length: owned.length };
      }).finally(() => { writing = false; });
    },
    publish(digest, signal) {
      if (digest.length !== 32) return Promise.reject(fail("invalid archive digest"));
      const expected = Buffer.from(digest);
      return enqueue(async () => {
        const live = () => check(signal);
        live();
        if (receipt && expected.equals(receipt.digest)) return result(expected);
        if (retired || failed || receipt || written !== extent || !captured.copy().digest().equals(expected)) throw fail("archive seal differs from capture");
        if ((await file.stat()).size !== extent) throw fail("archive spool extent changed");
        await validateFirstArchive(file, extent, page, live);
        const head = prefix(key, metadata, extent);
        const reread = createHash("sha256");
        let sum = checksum(FNV_OFFSET, head);
        await readPages(file, 0, extent, page, live, (bytes) => {
          sum = checksum(sum, bytes);
          reread.update(bytes);
        });
        if (!reread.digest().equals(expected)) throw fail("archive spool digest changed");
        await withWriter(path, async () => {
          live();
          const journal = await writerFile(path);
          try {
            const size = (await journal.stat()).size;
            const { validEnd, latest } = await scan(journal, size, key, metadata, page, live);
            if (latest) {
              if (latest === "pair" || !latest.matchingSchema || latest.length !== extent ||
                  !(await hashRange(journal, latest, page, live)).equals(expected)) throw fail("different first archive already exists");
              await journal.sync();
              await syncDirectory(directory);
              await syncDirectory(dirname(directory));
              live();
              return;
            }
            await appendRecord(journal, path, size, validEnd, head, extent, checksum(sum, TAIL), live,
              (position) => readPages(file, 0, extent, page, live,
                (bytes, offset) => writeExact(journal, bytes, position + offset)),
              async (expectedRange, end) => {
                const observed = await scan(journal, end, key, metadata, page, live, validEnd);
                const found = observed.latest;
                if ((await journal.stat()).size !== end || observed.validEnd !== end || !found || found === "pair" ||
                    !found.matchingSchema || found.start !== expectedRange.start || found.length !== extent ||
                    !(await hashRange(journal, found, page, live)).equals(expected)) throw fail("durable readback differs from archive seal");
              });
          } finally { await journal.close(); }
        });
        receipt = result(expected);
        await dispose();
        return result(expected);
      });
    },
    retire() { return enqueue(dispose); },
  };
}
