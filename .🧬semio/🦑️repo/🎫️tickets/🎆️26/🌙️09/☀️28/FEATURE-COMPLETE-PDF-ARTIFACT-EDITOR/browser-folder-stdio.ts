/** 🚪 Owns the developer server's bounded exchange with the native folder journal writer. */
import { spawn } from "node:child_process";

const MAGIC = Buffer.from("SEMIOFJ1");
const START = 1, BYTES = 2, SEAL = 3, CANCEL = 4;
const PAGE = 4096, LIMIT = 4 * 1024 * 1024;
export type NativeFolderJournalScope = Readonly<{
  schema: string;
  frontier: Readonly<{ documentId: string; generation: bigint; revision: Uint8Array; extent: number }>;
  source: Readonly<{
    instanceId: number; operationId: string; generation: string; sequence: bigint; extent: number;
    declaration: Readonly<{ formatKind: string; extension: string; mediaType: string; binary: boolean }>;
  }>;
}>;
export type FolderFirstArchiveCapture = Readonly<{
  write(offset: number, page: Uint8Array): Promise<{ offset: number; length: number }>;
  publish(digest: Uint8Array, signal: AbortSignal): Promise<{
    frontier: NativeFolderJournalScope["frontier"]; extent: number; digest: Uint8Array;
  }>;
  retire(): Promise<void>;
}>;

function failure(detail: string): Error { return new Error(`native folder journal: ${detail}`); }
function exactToken(value: string): bigint {
  if (!/^[0-9a-f]{16}$/u.test(value) || value === "0000000000000000") throw failure("invalid loan coordinate");
  return BigInt(`0x${value}`);
}
function extent(value: number, maximum: number): number {
  if (!Number.isInteger(value) || value < 1 || value > maximum) throw failure("invalid extent");
  return value;
}
function scopeWire(scope: NativeFolderJournalScope): Buffer {
  const { frontier, source } = scope;
  if (!source.declaration.binary || frontier.revision.length !== 32 || source.sequence < 1n ||
      source.sequence > 0xffffffffffffffffn || frontier.generation < 0n || frontier.generation > 0xffffffffffffffffn) {
    throw failure("invalid source or frontier");
  }
  const fields = [source.declaration.formatKind, source.declaration.extension, source.declaration.mediaType].map((field) => {
    const bytes = Buffer.from(field);
    extent(bytes.length, 512);
    if (bytes.includes(0)) throw failure("invalid declaration coordinate");
    return bytes;
  });
  const header = Buffer.alloc(84 + fields.reduce((size, bytes) => size + bytes.length + 2, 0));
  header.set(MAGIC, 0);
  header.writeUInt32LE(extent(frontier.extent, LIMIT), 8);
  header.writeUInt32LE(extent(source.extent, 0xffffffff), 12);
  header.writeUInt32LE(extent(source.instanceId, 0xffffffff), 16);
  header.writeBigUInt64LE(exactToken(source.operationId), 20);
  header.writeBigUInt64LE(exactToken(source.generation), 28);
  header.writeBigUInt64LE(source.sequence, 36);
  header.writeBigUInt64LE(frontier.generation, 44);
  header.set(frontier.revision, 52);
  let position = 84;
  for (const bytes of fields) {
    header.writeUInt16LE(bytes.length, position);
    header.set(bytes, position + 2);
    position += bytes.length + 2;
  }
  return header;
}

function journalChild(binary: string, mode: "first" | "put", folder: string, document: string, schema: string) {
  const child = spawn(binary, [mode, folder, document, schema], { stdio: ["pipe", "pipe", "pipe"], windowsHide: true });
  let pending: { buffer: Buffer; filled: number; resolve: (value: Buffer) => void; reject: (reason: unknown) => void } | undefined;
  let refused: Error | undefined;
  let closed = false;
  let stderr = "";
  let finish: (success: boolean) => void = () => {};
  const settled = new Promise<boolean>((resolve) => { finish = resolve; });
  const reject = (error: Error) => { refused ??= error; pending?.reject(refused); pending = undefined; };
  child.stderr.on("data", (bytes: Buffer) => { if (stderr.length < 1024) stderr += bytes.subarray(0, 1024 - stderr.length).toString(); });
  child.stdout.on("data", (bytes: Buffer) => {
    if (!pending || bytes.length > pending.buffer.length - pending.filled) { reject(failure("unexpected receipt length")); child.stdin.end(); return; }
    pending.buffer.set(bytes, pending.filled);
    pending.filled += bytes.length;
    if (pending.filled === pending.buffer.length) {
      const message = pending;
      pending = undefined;
      message.resolve(message.buffer);
    }
  });
  child.stdin.on("error", (error) => { refused ??= error; });
  child.once("error", reject);
  child.once("close", (status) => {
    closed = true;
    const success = status === 0 && refused === undefined && pending === undefined;
    if (!success) reject(refused ?? failure(stderr.trim() || "writer closed before acknowledgement"));
    finish(success);
  });
  return {
    exchange(bytes: Uint8Array, responseLength: number, finalInput = false): Promise<Buffer> {
      if (closed || refused || pending || child.stdin.writableEnded) return Promise.reject(refused ?? failure("writer is unavailable"));
      const result = new Promise<Buffer>((resolve, reject) => {
        pending = { buffer: Buffer.alloc(responseLength), filled: 0, resolve, reject };
      });
      if (finalInput) child.stdin.end(bytes);
      else child.stdin.write(bytes);
      return result;
    },
    cancel() {
      if (!closed && !child.stdin.writableEnded) child.stdin.end(Buffer.of(CANCEL));
    },
    async close(success: boolean): Promise<void> {
      if (!await settled && success) throw refused ?? failure("writer did not exit durably");
    },
  };
}

/** 🏦 Uses the native Store journal for both ordinary dev writes and bounded first-genesis publication. */
export function nativeFolderJournal(binary: string) {
  return {
    async put(folder: string, document: string, schema: string, archive: Uint8Array): Promise<void> {
      extent(archive.length, LIMIT);
      const child = journalChild(binary, "put", folder, document, schema);
      const header = Buffer.alloc(12);
      header.set(MAGIC);
      header.writeUInt32LE(archive.length, 8);
      try {
        const request = Buffer.concat([header, archive]);
        const receipt = await child.exchange(request, 1, true);
        if (receipt[0] !== SEAL) throw failure("foreign write receipt");
        await child.close(true);
      } finally { child.cancel(); await child.close(false); }
    },
    async capture(folder: string, scope: NativeFolderJournalScope, signal: AbortSignal, current: () => boolean): Promise<FolderFirstArchiveCapture> {
      const live = () => {
        signal.throwIfAborted();
        if (!current()) throw failure("folder binding was retired");
      };
      live();
      const header = scopeWire(scope);
      const { documentId, generation, extent: archiveExtent } = scope.frontier;
      const revision = Uint8Array.from(scope.frontier.revision);
      const child = journalChild(binary, "first", folder, documentId, scope.schema);
      const cancel = () => child.cancel();
      signal.addEventListener("abort", cancel, { once: true });
      const retire = async () => {
        signal.removeEventListener("abort", cancel);
        cancel();
        await child.close(false);
      };
      let finished = false;
      const receipt = (digest: Uint8Array) => ({
        extent: archiveExtent, digest: Uint8Array.from(digest),
        frontier: { documentId, generation, revision: revision.slice(), extent: archiveExtent },
      });
      let published: Buffer | undefined;
      try {
        if ((await child.exchange(header, 1))[0] !== START) throw failure("foreign capture receipt");
        live();
      } catch (error) { await retire(); throw error; }
      return {
        async write(offset, bytes) {
          live();
          if (finished || !Number.isInteger(offset) || offset < 0 || offset >= archiveExtent || offset % PAGE !== 0 ||
              bytes.length !== Math.min(PAGE, archiveExtent - offset)) throw failure("wrong archive page");
          const request = Buffer.alloc(7 + bytes.length);
          request[0] = BYTES;
          request.writeUInt32LE(offset, 1);
          request.writeUInt16LE(bytes.length, 5);
          request.set(bytes, 7);
          const observed = await child.exchange(request, 7);
          if (!observed.equals(request.subarray(0, 7))) throw failure("foreign archive page receipt");
          live();
          return { offset, length: bytes.length };
        },
        async publish(digest, publishing) {
          live();
          publishing.throwIfAborted();
          if (digest.length !== 32) throw failure("wrong archive digest");
          if (published?.equals(digest)) return receipt(published);
          if (finished) throw failure("archive writer has finished");
          const request = Buffer.alloc(33);
          request[0] = SEAL;
          request.set(digest, 1);
          publishing.addEventListener("abort", cancel, { once: true });
          try {
            const observed = await child.exchange(request, 37);
            if (observed[0] !== SEAL || observed.readUInt32LE(1) !== archiveExtent || !observed.subarray(5).equals(request.subarray(1))) {
              throw failure("foreign durable seal");
            }
            await child.close(true);
            finished = true;
            published = Buffer.from(observed.subarray(5));
            live();
            publishing.throwIfAborted();
            return receipt(published);
          } catch (error) {
            publishing.throwIfAborted();
            signal.throwIfAborted();
            throw error;
          } finally { publishing.removeEventListener("abort", cancel); await retire(); }
        },
        retire,
      };
    },
  };
}

export type NativeFolderJournal = ReturnType<typeof nativeFolderJournal>;
