import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { inflateRawSync, inflateSync } from "node:zlib";

function inflateBody(body: Buffer): Buffer {
  try {
    return inflateSync(body);
  } catch {
    return inflateRawSync(body);
  }
}

function stabilize(pdf: string): string {
  const inflated = pdf.replace(/stream\r?\n([\s\S]*?)\r?\nendstream/g, (_all, body: string) => {
    try {
      const out = inflateBody(Buffer.from(body, "latin1"));
      return `stream\n${out.toString("latin1")}\nendstream`;
    } catch {
      return `stream\n${body}\nendstream`;
    }
  });
  return inflated
    .replace(/\/CreationDate\s*\([^)]*\)/g, "")
    .replace(/\/ModDate\s*\([^)]*\)/g, "")
    .replace(/\/ID\s*\[[^\]]*\]/g, "")
    .replace(/\(D:[0-9+\-'Z]+\)/g, "")
    .replace(/\/Producer\s*\([^)]*\)/g, "")
    .replace(/\/Creator\s*\([^)]*\)/g, "")
    .replace(/<x:xmpmeta[\s\S]*?<\/x:xmpmeta>/g, "");
}

const dir = import.meta.dir;
const a = readFileSync(join(dir, "viz-api-a.pdf")).toString("latin1");
const b = readFileSync(join(dir, "viz-api-b.pdf")).toString("latin1");
const sa = stabilize(a);
const sb = stabilize(b);
console.log("raw", createHash("sha256").update(a, "latin1").digest("hex").slice(0, 12), createHash("sha256").update(b, "latin1").digest("hex").slice(0, 12));
console.log("stable", createHash("sha256").update(sa, "latin1").digest("hex").slice(0, 12), createHash("sha256").update(sb, "latin1").digest("hex").slice(0, 12));
console.log("stableEqual", sa === sb, "len", sa.length, sb.length);
if (sa !== sb) {
  const diffs: number[] = [];
  for (let i = 0; i < Math.min(sa.length, sb.length); i++) if (sa[i] !== sb[i]) diffs.push(i);
  console.log("diffs", diffs.length, diffs.slice(0, 8));
  for (const i of diffs.slice(0, 3)) {
    console.log("A", JSON.stringify(sa.slice(i - 40, i + 80)));
    console.log("B", JSON.stringify(sb.slice(i - 40, i + 80)));
  }
}
