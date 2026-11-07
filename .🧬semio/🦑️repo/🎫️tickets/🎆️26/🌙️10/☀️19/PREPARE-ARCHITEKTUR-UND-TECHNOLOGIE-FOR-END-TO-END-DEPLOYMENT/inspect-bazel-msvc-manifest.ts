import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const url = "https://download.visualstudio.microsoft.com/download/pr/fdc37f6e-59f6-4054-838a-b476eeaa6ec3/1d82370739911457e0a2f6be15d8b5f569531b352b2eabb961429eea1e99e356/VisualStudio.vsman";
const expected = "qOhU+p8/uurCfXME4pfxZg1xN0ATid61fPeYPzPBb+8=";
const response = await fetch(url, { signal: AbortSignal.timeout(60_000) });
if (!response.ok || new URL(response.url).protocol !== "https:" || Number(response.headers.get("content-length")) > 32_000_000) throw Error("Manifest request refused");
const bytes = new Uint8Array(await response.arrayBuffer());
const digest = createHash("sha256").update(bytes).digest("base64");
if (bytes.length > 32_000_000 || digest !== expected) throw Error(`Manifest integrity mismatch: ${bytes.length} bytes ${digest}`);
const manifest = JSON.parse(new TextDecoder().decode(bytes));
writeFileSync(join(process.env.SEMIO_TICKET_DIR!, "🗑️generated", "bazel-msvc-manifest.json"), bytes);
console.log(JSON.stringify({ length: bytes.length, sha256: Buffer.from(digest, "base64").toString("hex"), info: manifest.info }));
