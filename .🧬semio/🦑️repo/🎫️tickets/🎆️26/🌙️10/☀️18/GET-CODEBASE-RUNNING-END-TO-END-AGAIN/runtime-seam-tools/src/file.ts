import { createReadStream } from "node:fs";
import { readFile, stat } from "node:fs/promises";
import { Readable } from "node:stream";
import type { RuntimeFileV1 } from "../../📋️contract/🟦️.ts";

/** 📄️ Describes one file lazily over `node:fs`; nothing is read until a member is called. */
export function openFileV1(path: string): RuntimeFileV1 {
  return {
    path,
    exists: async () => { try { return (await stat(path)).isFile(); } catch { return false; } },
    size: async () => (await stat(path)).size,
    text: () => readFile(path, "utf8"),
    body: () => Readable.toWeb(createReadStream(path)) as ReadableStream<Uint8Array>,
  };
}
