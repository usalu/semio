#!/usr/bin/env bun
/**
 * 🔗️ S3-CLOSURE re-seal of `🏪️store/🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` after `Edit.coalesce_key` left the
 * edit (design §20.1). First proves the digest algorithm (the TS twin's `editDigest`) reproduces every committed digest with
 * the old `coalesceKey` slot, then drops `coalesceKey` from every edit/header and recomputes `expectedDigest` without the slot.
 * Usage: bun 🧪️s3-closure-reseal-edit-digest-chains.ts [--write]
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const FIXTURE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json";
const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
const record = (domain: string, parts: Buffer[]) => {
  const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
  for (const part of parts) hash.update(u64(part.length)).update(part);
  return hash.digest();
};
const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
const text = (edit: Record<string, unknown>, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
const editDigest = (edit: Record<string, unknown>, keyed: boolean) => {
  const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
  if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(edit))]);
  const sequence = Buffer.alloc(4);
  sequence.writeInt32BE(edit.sequenceNumber as number);
  const chained = record("edit-chained", [
    Buffer.from(String(edit.id)),
    ...text(edit, "actor"),
    ...text(edit, "description"),
    ...(keyed ? text(edit, "coalesceKey") : []),
    sequence,
    Buffer.from(String(edit.startedAt)),
    ...text(edit, "finishedAt"),
    u64(forwards.length), chain("edit-forward", forwards),
    u64(inverse.length), chain("edit-inverse", inverse),
    u64(meta.length), chain("edit-meta", meta),
  ]);
  const authored = "verb" in edit ? record("edit-verb", [chained, Buffer.from(String(edit.verb))]) : chained;
  return record("edit-line", [authored, Buffer.from([edit.line === null ? 0 : 1]), Buffer.from(String(edit.line ?? ""))]);
};
type Row = { name: string; edit?: Record<string, unknown>; header?: Record<string, unknown>; generatedOperations?: number; expectedDigest: string };
const materialize = (row: Row) => row.edit ?? {
  ...row.header,
  forwards: Array.from({ length: row.generatedOperations ?? 0 }, (_, index) => ({ SetN: { n: index + 1 } })),
  inverse: Array.from({ length: row.generatedOperations ?? 0 }, (_, index) => ({ SetN: { n: index } })),
};
const fixture = JSON.parse(readFileSync(FIXTURE, "utf8")) as { cases: Row[] };
for (const row of fixture.cases) {
  const before = editDigest(materialize(row), true).toString("hex");
  if ("coalesceKey" in (row.edit ?? row.header ?? {}) && before !== row.expectedDigest) throw new Error(`[DEBUG] ${row.name}: the oracle does not reproduce the committed digest`);
  delete (row.edit ?? row.header ?? {}).coalesceKey;
  row.expectedDigest = editDigest(materialize(row), false).toString("hex");
  console.log(`[DEBUG] ${row.name}: ${before} -> ${row.expectedDigest}`);
}
if (process.argv.includes("--write")) writeFileSync(FIXTURE, `${JSON.stringify(fixture, null, 2)}\n`);
