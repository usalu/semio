import { Readable } from "node:stream";
import type { RuntimeByteStreamV1 } from "../../📋️contract/🟦️.ts";

/** 📥️ Reads all of standard input as UTF-8 text over `node:stream`. */
export async function stdinTextV1(): Promise<string> {
  const chunks: Buffer[] = [];
  for await (const chunk of process.stdin) chunks.push(Buffer.from(chunk as Uint8Array));
  return Buffer.concat(chunks).toString("utf8");
}

/** 📥️ Exposes standard input as a web byte stream. */
export function stdinStreamV1(): RuntimeByteStreamV1 { return Readable.toWeb(process.stdin) as unknown as RuntimeByteStreamV1; }
