#!/usr/bin/env bun
/**
 * 🔗️ S4-BUMP wave B re-seal of the canonical-edit fixtures after `Edit.description` left the edit (design §20.7).
 * - `🔗️edit-digest-chains.json`: first proves the TS twin's digest (with the old description slot) reproduces every committed
 *   `expectedDigest`, then drops `description` from every edit/header and recomputes the digest without the slot.
 * - `🔏️canonical-edit-sealer.json` / `🗺️canonical-borrowed-map.json`: first proves `expectedJson`/`expectedDigest` against the
 *   committed edit, then drops `description` and re-derives both; `📖️canonical-reader.json` follows the borrowed map's bytes.
 * Usage: bun 🧪️s4-bump-reseal-edit-description.ts [--write]
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";

const BASE = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures/";
const write = process.argv.includes("--write");
const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
const record = (domain: string, parts: Buffer[]) => {
  const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
  for (const part of parts) hash.update(u64(part.length)).update(part);
  return hash.digest();
};
const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
const text = (edit: Record<string, unknown>, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
const editDigest = (edit: Record<string, unknown>, described: boolean) => {
  const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
  if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(JSON.stringify(edit))]);
  const sequence = Buffer.alloc(4);
  sequence.writeInt32BE(edit.sequenceNumber as number);
  const chained = record("edit-chained", [
    Buffer.from(String(edit.id)),
    ...text(edit, "actor"),
    ...(described ? text(edit, "description") : []),
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
const save = (name: string, value: unknown) => { if (write) writeFileSync(BASE + name, `${JSON.stringify(value, null, 2)}\n`); };

type Row = { name: string; edit?: Record<string, unknown>; header?: Record<string, unknown>; generatedOperations?: number; expectedDigest: string };
const materialize = (row: Row) => row.edit ?? {
  ...row.header,
  forwards: Array.from({ length: row.generatedOperations ?? 0 }, (_, index) => ({ SetN: { n: index + 1 } })),
  inverse: Array.from({ length: row.generatedOperations ?? 0 }, (_, index) => ({ SetN: { n: index } })),
};
const chains = JSON.parse(readFileSync(BASE + "🔗️edit-digest-chains.json", "utf8")) as { cases: Row[] };
for (const row of chains.cases) {
  const before = editDigest(materialize(row), true).toString("hex");
  if (before !== row.expectedDigest) throw new Error(`[DEBUG] ${row.name}: the oracle does not reproduce the committed digest`);
  delete (row.edit ?? row.header ?? {}).description;
  row.expectedDigest = editDigest(materialize(row), false).toString("hex");
  console.log(`[DEBUG] chains ${row.name}: ${before} -> ${row.expectedDigest}`);
}
save("🔗️edit-digest-chains.json", chains);

const sealDigest = (edit: Record<string, unknown>, json: string) => record("edit", [Buffer.from(String(edit.id)), Buffer.from(json)]).toString("hex");
const reseal = (name: string) => {
  const fixture = JSON.parse(readFileSync(BASE + name, "utf8")) as { edit: Record<string, unknown>; expectedJson: string; expectedDigest: string };
  if (fixture.expectedJson !== JSON.stringify(fixture.edit) || sealDigest(fixture.edit, fixture.expectedJson) !== fixture.expectedDigest) throw new Error(`[DEBUG] ${name}: the oracle does not reproduce the committed seal`);
  const before = fixture.expectedDigest;
  delete fixture.edit.description;
  fixture.expectedJson = JSON.stringify(fixture.edit);
  fixture.expectedDigest = sealDigest(fixture.edit, fixture.expectedJson);
  console.log(`[DEBUG] ${name}: ${before} -> ${fixture.expectedDigest} (${Buffer.byteLength(fixture.expectedJson)} bytes)`);
  save(name, fixture);
  return fixture;
};
reseal("🔏️canonical-edit-sealer.json");
const map = reseal("🗺️canonical-borrowed-map.json");
const reader = JSON.parse(readFileSync(BASE + "📖️canonical-reader.json", "utf8")) as { expectedByteLength: number; expectedJsonSha256: string };
const bytes = Buffer.from(map.expectedJson);
console.log(`[DEBUG] reader: ${reader.expectedByteLength}/${reader.expectedJsonSha256} -> ${bytes.length}/${createHash("sha256").update(bytes).digest("hex")}`);
reader.expectedByteLength = bytes.length;
reader.expectedJsonSha256 = createHash("sha256").update(bytes).digest("hex");
save("📖️canonical-reader.json", reader);
