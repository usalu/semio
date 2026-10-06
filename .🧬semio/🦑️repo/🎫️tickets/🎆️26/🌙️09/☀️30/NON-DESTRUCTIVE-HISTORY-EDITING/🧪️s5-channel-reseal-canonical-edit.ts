/** 🔏️ S5-CHANNEL wave C: reseals the four canonical-edit fixtures for design §22.28 — `sequenceNumber` leaves the revision digest.
 *
 * Rule (S5-STORE, 2026-10-05 10:20, `🏪️store/🦀️.rs` + `🧵️canonical-edit/🦀️.rs`): an edit's revision value is the edit in
 * declaration order WITHOUT its `sequenceNumber` member. A single-operation edit digests `record("edit", [id, JSON(revision)])`;
 * a chained edit digests `record("edit-chained", […])` without the 4-byte sequence part, then `edit-verb` / `edit-line` as before.
 * The fixtures' `edit` INPUT keeps `sequenceNumber` (the `Edit` type keeps the field); only what is derived from it moves:
 * `expectedJson` + `expectedDigest` (sealer, borrowed map), `expectedByteLength` + `expectedJsonSha256` (reader), every
 * `expectedDigest` (digest chains).
 * It first proves its own oracle against the fixtures as they are (old rule) or reports them already resealed (new rule); a
 * fixture that matches neither rule, or that does not round-trip its own formatting, stops the run before anything is written.
 * Usage: bun 🧪️s5-channel-reseal-canonical-edit.ts [--write | --restore] */
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const FIXTURES = join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧫️fixtures");
const BACKUP = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🗑️generated/s5-channel/wave-c-canonical-edit-backup");
const NAMES = ["🔏️canonical-edit-sealer.json", "🗺️canonical-borrowed-map.json", "📖️canonical-reader.json", "🔗️edit-digest-chains.json"] as const;
type Json = Record<string, any>;
type Rule = "sequenced" | "revision";

const u64 = (value: number) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64BE(BigInt(value)); return bytes; };
const record = (domain: string, parts: Buffer[]) => {
  const hash = createHash("sha256").update("semio.artifact.cursor.v2").update(u64(Buffer.byteLength(domain))).update(domain);
  for (const part of parts) hash.update(u64(part.length)).update(part);
  return hash.digest();
};
const chain = (domain: string, items: unknown[]) => items.reduce<Buffer>((state, item) => record(domain, [state, Buffer.from(JSON.stringify(item))]), Buffer.alloc(32));
const text = (edit: Json, key: string) => [Buffer.from([key in edit ? 1 : 0]), Buffer.from(String(edit[key] ?? ""))];
const revisionValue = ({ sequenceNumber: _sequenceNumber, ...revision }: Json): Json => revision;
const canonicalJson = (edit: Json, rule: Rule) => JSON.stringify(rule === "revision" ? revisionValue(edit) : edit);
const editDigest = (edit: Json, rule: Rule): string => {
  const [forwards, inverse, meta] = [edit.forwards as unknown[], edit.inverse as unknown[], (edit.mutationMeta ?? []) as unknown[]];
  if (forwards.length <= 1) return record("edit", [Buffer.from(String(edit.id)), Buffer.from(canonicalJson(edit, rule))]).toString("hex");
  const sequence = Buffer.alloc(4);
  sequence.writeInt32BE(edit.sequenceNumber as number);
  const chained = record("edit-chained", [
    Buffer.from(String(edit.id)),
    ...text(edit, "actor"),
    ...(rule === "sequenced" ? [sequence] : []),
    Buffer.from(String(edit.startedAt)),
    ...text(edit, "finishedAt"),
    u64(forwards.length), chain("edit-forward", forwards),
    u64(inverse.length), chain("edit-inverse", inverse),
    u64(meta.length), chain("edit-meta", meta),
  ]);
  const authored = "verb" in edit ? record("edit-verb", [chained, Buffer.from(String(edit.verb))]) : chained;
  return record("edit-line", [authored, Buffer.from([edit.line === null ? 0 : 1]), Buffer.from(String(edit.line ?? ""))]).toString("hex");
};
const chainEdit = (row: Json): Json => row.edit ?? {
  ...row.header,
  forwards: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index + 1 } })),
  inverse: Array.from({ length: row.generatedOperations }, (_, index) => ({ SetN: { n: index } })),
};

/** 🧮️ The four fixtures as `rule` derives them from their own `edit` inputs. */
function derive(fixtures: Record<string, Json>, rule: Rule): Record<string, Json> {
  const [sealer, borrowed, reader, chains] = NAMES.map((name) => structuredClone(fixtures[name]!));
  for (const fixture of [sealer!, borrowed!]) {
    fixture.expectedJson = canonicalJson(fixture.edit, rule);
    fixture.expectedDigest = editDigest(fixture.edit, rule);
  }
  reader!.expectedByteLength = Buffer.byteLength(borrowed!.expectedJson);
  reader!.expectedJsonSha256 = createHash("sha256").update(borrowed!.expectedJson).digest("hex");
  for (const row of chains!.cases) row.expectedDigest = editDigest(chainEdit(row), rule);
  return Object.fromEntries(NAMES.map((name, index) => [name, [sealer, borrowed, reader, chains][index]!]));
}

const format = (value: Json) => `${JSON.stringify(value, null, 2)}\n`;
const mode = process.argv.slice(2);
if (mode.some((argument) => argument !== "--write" && argument !== "--restore") || mode.length > 1) throw new Error(`[DEBUG] unknown arguments ${JSON.stringify(mode)}: refusing`);
if (!existsSync(join(ROOT, ".git")) || !existsSync(FIXTURES)) throw new Error("[DEBUG] not the repo root or no fixture folder: refusing");

if (mode[0] === "--restore") {
  if (!NAMES.every((name) => existsSync(join(BACKUP, name)))) throw new Error("[DEBUG] no complete backup: nothing restored");
  for (const name of NAMES) writeFileSync(join(FIXTURES, name), readFileSync(join(BACKUP, name)));
  rmSync(BACKUP, { recursive: true });
  console.log(`[DEBUG] ${NAMES.length} fixtures restored`);
} else {
  const sources = Object.fromEntries(NAMES.map((name) => [name, readFileSync(join(FIXTURES, name), "utf8")]));
  const fixtures: Record<string, Json> = Object.fromEntries(NAMES.map((name) => [name, JSON.parse(sources[name]!)]));
  for (const name of NAMES) if (format(fixtures[name]!) !== sources[name]) throw new Error(`[DEBUG] ${name} does not round-trip its own formatting: refusing`);
  const [sequenced, revision] = [derive(fixtures, "sequenced"), derive(fixtures, "revision")];
  const holds = (derived: Record<string, Json>) => NAMES.every((name) => format(derived[name]!) === sources[name]);
  if (holds(revision)) console.log("[DEBUG] the four fixtures already hold the §22.28 revision digests: nothing to write");
  else if (!holds(sequenced)) throw new Error("[DEBUG] the fixtures match neither the sequenced nor the revision rule: refusing (the oracle or a fixture moved)");
  else {
    console.log("[DEBUG] oracle proven: the four fixtures hold the sequenced digests as they are");
    for (const name of [NAMES[0], NAMES[1]]) console.log(`[DEBUG] ${name}: expectedJson ${Buffer.byteLength(sequenced[name]!.expectedJson)} → ${Buffer.byteLength(revision[name]!.expectedJson)} bytes, expectedDigest ${sequenced[name]!.expectedDigest.slice(0, 12)} → ${revision[name]!.expectedDigest.slice(0, 12)}`);
    console.log(`[DEBUG] ${NAMES[2]}: expectedByteLength ${sequenced[NAMES[2]]!.expectedByteLength} → ${revision[NAMES[2]]!.expectedByteLength}, expectedJsonSha256 ${sequenced[NAMES[2]]!.expectedJsonSha256.slice(0, 12)} → ${revision[NAMES[2]]!.expectedJsonSha256.slice(0, 12)}`);
    for (const [index, row] of (revision[NAMES[3]]!.cases as Json[]).entries()) console.log(`[DEBUG] ${NAMES[3]} ${row.name}: ${sequenced[NAMES[3]]!.cases[index].expectedDigest.slice(0, 12)} → ${row.expectedDigest.slice(0, 12)}`);
    if (mode[0] === "--write") {
      if (existsSync(BACKUP)) throw new Error("[DEBUG] a backup exists: --restore first");
      mkdirSync(BACKUP, { recursive: true });
      for (const name of NAMES) writeFileSync(join(BACKUP, name), sources[name]!);
      for (const name of NAMES) writeFileSync(join(FIXTURES, name), format(revision[name]!));
      console.log(`[DEBUG] ${NAMES.length} fixtures written (backup: 🗑️generated/s5-channel/wave-c-canonical-edit-backup)`);
    } else console.log(`[DEBUG] ${NAMES.length} fixtures would change (dry run)`);
  }
}
