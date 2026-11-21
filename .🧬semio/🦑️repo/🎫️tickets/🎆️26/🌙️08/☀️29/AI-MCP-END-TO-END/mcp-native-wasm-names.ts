import { createHash } from "node:crypto";
import { createReadStream, createWriteStream } from "node:fs";
import { open } from "node:fs/promises";
import { Readable } from "node:stream";
import { pipeline } from "node:stream/promises";

type Range = Readonly<{ start: number; end: number }>;
type Section = Range & Readonly<{ id: number; payload: number; name?: string }>;
type Piece = Range | Uint8Array;

async function digestRange(path: string, range: Range): Promise<string> {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path, { start: range.start, end: range.end - 1 })) hash.update(chunk);
  return hash.digest("hex");
}

function readLength(bytes: Uint8Array, cursor: number): readonly [number, number] {
  let size = 0;
  for (let index = 0; index < 5 && cursor < bytes.length; index++) {
    const byte = bytes[cursor++];
    size += (byte & 127) * (2 ** (7 * index));
    if (byte < 128 && size <= 0xffffffff) return [size, cursor];
  }
  throw Error("WebAssembly section length required");
}

function sectionHeader(id: number, size: number): Uint8Array {
  const bytes = [id];
  while (size >= 128) {
    bytes.push((size % 128) | 128);
    size = Math.floor(size / 128);
  }
  bytes.push(size);
  return Uint8Array.from(bytes);
}

/** 🧹️ Removes only optional core function names from an overlarge ticket component. */
export async function stripCoreNamesV1(source: string, destination: string, maximumBytes: number): Promise<number> {
  if (source === destination) throw Error("Distinct native fallback component paths required");
  const file = await open(source);
  const pieces: Piece[] = [{ start: 0, end: 8 }];
  let removed = 0;
  try {
    const size = (await file.stat()).size;
    const prefix = async (start: number, count: number) => {
      const bytes = Buffer.alloc(count);
      const { bytesRead } = await file.read(bytes, 0, bytes.length, start);
      return bytes.subarray(0, bytesRead);
    };
    const sections = async (start: number, end: number): Promise<Section[]> => {
      const found: Section[] = [];
      for (let cursor = start; cursor < end;) {
        const bytes = await prefix(cursor, Math.min(32, end - cursor));
        const id = bytes[0];
        const [length, offset] = readLength(bytes, 1);
        const payload = cursor + offset;
        const next = payload + length;
        if (next > end) throw Error("WebAssembly section exceeds its container");
        const name = id === 0 && length >= 5 && bytes[offset] === 4 ? bytes.toString("ascii", offset + 1, offset + 5) : undefined;
        found.push({ start: cursor, end: next, id, payload, name });
        cursor = next;
      }
      return found;
    };
    if ((await prefix(0, 8)).toString("hex") !== "0061736d0d000100") throw Error("WebAssembly component required");
    if (size > 1024 ** 3) throw Error("Finite native fallback component required");
    for (const section of await sections(8, size)) {
      if (section.id !== 1) {
        pieces.push(section);
        continue;
      }
      const coreStart = section.payload + 8;
      if ((await prefix(section.payload, 8)).toString("hex") !== "0061736d01000000") throw Error("Embedded core module required");
      const core = await sections(coreStart, section.end);
      const names = core.filter(row => row.id === 0 && row.name === "name");
      if (!names.length) {
        pieces.push(section);
        continue;
      }
      const bytes = names.reduce((count, row) => count + row.end - row.start, 0);
      removed += bytes;
      pieces.push(sectionHeader(1, section.end - section.payload - bytes), { start: section.payload, end: coreStart }, ...core.filter(row => !names.includes(row)));
    }
  } finally {
    await file.close();
  }
  const outputSize = pieces.reduce((size, piece) => size + (piece instanceof Uint8Array ? piece.byteLength : piece.end - piece.start), 0);
  if (!removed || outputSize > maximumBytes) throw Error("Core name removal does not fit the component validator");
  async function* bytes() {
    for (const piece of pieces) {
      if (piece instanceof Uint8Array) yield piece;
      else yield* createReadStream(source, { start: piece.start, end: piece.end - 1 });
    }
  }
  await pipeline(Readable.from(bytes()), createWriteStream(destination));
  let offset = 0;
  for (const piece of pieces) {
    const extent = piece instanceof Uint8Array ? piece.length : piece.end - piece.start;
    const [expected, actual] = await Promise.all([
      piece instanceof Uint8Array ? createHash("sha256").update(piece).digest("hex") : digestRange(source, piece),
      digestRange(destination, { start: offset, end: offset + extent }),
    ]);
    if (actual !== expected) throw Error("Non-name WebAssembly section bytes changed");
    offset += extent;
  }
  return removed;
}
