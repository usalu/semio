/** 🔬️ WG7 (ticket-local) — decodes a probe's captured document-socket frames with the product's own codecs
 * (`decodeServerFrame` / `decodeClientFrame`, `@semio-tech/framework-replication`). Usage: bun s12-decode-frames.ts <frames.json> [limit] */
import { readFileSync } from "node:fs";
import { decodeClientFrame, decodeServerFrame } from "@semio-tech/framework-replication";
const [file, limit = "60"] = process.argv.slice(2);
const frames = JSON.parse(readFileSync(file!, "utf8")) as { at: string; direction: string; bytes: number; base64?: string }[];
const summary = (value: unknown): string => JSON.stringify(value, (_key, inner) => (typeof inner === "bigint" ? `${inner}n` : inner instanceof Uint8Array ? `<${inner.length} bytes>` : Array.isArray(inner) && inner.length > 16 && inner.every((x) => typeof x === "number") ? `<${inner.length} numbers>` : inner)).slice(0, 360);
for (const frame of frames.filter((entry) => entry.base64).slice(0, Number(limit))) {
  const bytes = Uint8Array.from(Buffer.from(frame.base64!, "base64"));
  let decoded: unknown;
  try {
    decoded = frame.direction === "received" ? await decodeServerFrame(bytes) : await decodeClientFrame(bytes);
  } catch (error) {
    decoded = `undecodable: ${error instanceof Error ? error.message : String(error)}`;
  }
  console.log(`${frame.at} ${frame.direction} ${frame.bytes}B ${summary(decoded)}`);
}
