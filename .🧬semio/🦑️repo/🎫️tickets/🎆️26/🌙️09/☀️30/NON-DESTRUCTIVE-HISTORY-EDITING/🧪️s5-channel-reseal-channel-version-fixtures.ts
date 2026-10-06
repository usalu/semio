#!/usr/bin/env bun
/**
 * 🧾️ S5-CHANNEL: re-derives the `derived` channel-version consumers that `channel-version generate --guest` refuses, for any
 * bump `--from <n> --to <m>` (design §22.4: 21 → 22 with wave B). Successor of `🧪️s4-bump-reseal-channel-v21-fixtures.ts`, same
 * oracles, but every file is computed first and written only with `--write` after ALL old values re-verified under `--from`:
 * - openable-document catalog generation = SHA-256 of `semio/hub/openable-document-catalog/v1\0 | u32 rows | (u64 BE length-prefixed
 *   fields)*` — plan fixture `expectedHex`/`expectedGenerationId`, browser fixture `installedTarget`, lease `manifest` row;
 * - the lease descriptor is a first-party Pack whose `appChannelVersion` is an f64 at its exact key, its SHA-256 is quoted by every
 *   descriptor authority; compiled-dependencies raw descriptor cases carry the same Pack f64;
 * - GIS Map frozen binding digest = SHA-256(`semio.hub.gis-map-frozen-binding/v1\0` + compact JSON), quoted by two fixtures;
 * - trusted-catalog two-package / generation-stage / stdio-gis-bootstrap carry literals only here (the bootstrap profile
 *   generation is not re-derivable outside its hub oracle and is reported as a coordinator action).
 * Usage: bun 🧪️s5-channel-reseal-channel-version-fixtures.ts --from 21 --to 22 [--write]
 */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const args = process.argv.slice(2);
const numberAfter = (flag: string): number => {
  const value = Number(args[args.indexOf(flag) + 1]);
  if (!args.includes(flag) || !Number.isInteger(value) || value < 1) throw new Error(`usage: --from <n> --to <m> [--write] (${flag} missing or not a positive integer)`);
  return value;
};
const FROM = numberAfter("--from");
const TO = numberAfter("--to");
if (TO === FROM) throw new Error("--from and --to are equal");
const WRITE = args.includes("--write");
const sha256 = (bytes: Uint8Array | string): string => createHash("sha256").update(bytes).digest("hex");
const read = (path: string): string => readFileSync(join(ROOT, path), "utf8");
const planned = new Map<string, string>();
const plan = (path: string, text: string): void => {
  if (planned.has(path)) throw new Error(`${path}: planned twice`);
  planned.set(path, text);
};

function replaceExact(text: string, from: string, to: string, count: number, label: string): string {
  const found = text.split(from).length - 1;
  if (found !== count) throw new Error(`${label}: ${JSON.stringify(from.slice(0, 80))} matched ${found}x, expected ${count}`);
  return text.split(from).join(to);
}

const literal = (version: number): string => `"appChannelVersion": ${version}`;
const packVersion = (version: number): string => {
  const bytes = Buffer.alloc(8);
  bytes.writeDoubleLE(version);
  return `6170704368616e6e656c56657273696f6e05${bytes.toString("hex")}`;
};

function lengthPrefixed(bytes: Uint8Array): Buffer {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
}

function catalogEncoding(rows: readonly Record<string, any>[], version: number): Buffer {
  const count = Buffer.alloc(4);
  count.writeUInt32BE(rows.length);
  const encodedVersion = Buffer.alloc(4);
  encodedVersion.writeUInt32BE(version);
  return Buffer.concat([
    Buffer.from("semio/hub/openable-document-catalog/v1\0"),
    count,
    ...rows.map((row) =>
      Buffer.concat(
        [
          Buffer.from(row.package.pluginId, "utf8"),
          Buffer.from(row.package.packageId, "utf8"),
          Buffer.from(row.package.version, "utf8"),
          Buffer.from(row.package.componentSha256, "hex"),
          Buffer.from(row.package.componentBlake3, "hex"),
          Buffer.from(row.package.descriptorByteSha256, "hex"),
          encodedVersion,
          Buffer.from(row.artifact.kind, "utf8"),
          Buffer.from(row.artifact.schema, "utf8"),
          Buffer.from(row.artifact.packSchemaHash, "hex"),
          Buffer.from(row.parentDialect.artifactKind, "utf8"),
          Buffer.from(row.parentDialect.standard, "utf8"),
          Buffer.from(row.parentDialect.subset, "utf8"),
          Buffer.from(row.surface.surfaceId, "utf8"),
          Buffer.from(row.surface.appId, "utf8"),
          Buffer.from(row.surface.windowKindId, "utf8"),
          Buffer.from(row.surface.role, "utf8"),
          Buffer.from(row.surface.rendererTarget, "utf8"),
          Buffer.from([row.grant.read ? 1 : 0, row.grant.write ? 1 : 0, row.grant.observe ? 1 : 0]),
        ].map(lengthPrefixed),
      ),
    ),
  ]);
}

const report: string[] = [];

function resealPlan(): void {
  const path = "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json";
  let text = read(path);
  const fixture = JSON.parse(text);
  const before = catalogEncoding(fixture.catalogRows, FROM);
  if (before.toString("hex") !== fixture.catalogEncoding.expectedHex || sha256(before) !== fixture.catalogEncoding.expectedGenerationId) throw new Error(`plan: v${FROM} encoding does not re-verify`);
  const after = catalogEncoding(fixture.catalogRows, TO);
  const generation = sha256(after);
  text = replaceExact(text, fixture.catalogEncoding.expectedHex, after.toString("hex"), 1, "plan expectedHex");
  text = replaceExact(text, fixture.catalogEncoding.expectedGenerationId, generation, text.split(fixture.catalogEncoding.expectedGenerationId).length - 1, "plan generation");
  text = replaceExact(text, literal(FROM), literal(TO), 3, "plan literals");
  plan(path, text);
  report.push(`${path}: generation ${fixture.catalogEncoding.expectedGenerationId} → ${generation}`);
}

function resealBrowser(): void {
  const path = "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🌐️browser-document-open-v1.json";
  let text = read(path);
  const fixture = JSON.parse(text);
  const old = fixture.installedTarget.catalog.generationId;
  if (sha256(catalogEncoding([fixture.installedTarget], FROM)) !== old || fixture.plan.catalog.generationId !== old) throw new Error(`browser: v${FROM} generation does not re-verify`);
  const generation = sha256(catalogEncoding([fixture.installedTarget], TO));
  text = replaceExact(text, old, generation, 2, "browser generation");
  text = replaceExact(text, literal(FROM), literal(TO), 2, "browser literals");
  plan(path, text);
  report.push(`${path}: generation ${old} → ${generation}`);
}

function resealLease(): void {
  const path = "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json";
  let text = read(path);
  const fixture = JSON.parse(text);
  const oldDescriptor: string = fixture.descriptorHex;
  const oldSha = sha256(Buffer.from(oldDescriptor, "hex"));
  const oldGeneration = fixture.manifest.catalog.generationId;
  if (oldSha !== fixture.manifest.descriptor.sha256 || sha256(catalogEncoding([fixture.manifest], FROM)) !== oldGeneration || fixture.expected.rotation.generationA !== oldGeneration) throw new Error(`lease: v${FROM} digests do not re-verify`);
  const newDescriptor = replaceExact(oldDescriptor, packVersion(FROM), packVersion(TO), 1, "lease descriptor Pack version");
  const newSha = sha256(Buffer.from(newDescriptor, "hex"));
  text = replaceExact(text, oldDescriptor, newDescriptor, 1, "lease descriptorHex");
  text = replaceExact(text, oldSha, newSha, text.split(oldSha).length - 1, "lease descriptor sha");
  text = replaceExact(text, literal(FROM), literal(TO), 2, "lease literals");
  const resealed = JSON.parse(text);
  const generation = sha256(catalogEncoding([resealed.manifest], TO));
  text = replaceExact(text, oldGeneration, generation, 3, "lease generation");
  plan(path, text);
  report.push(`${path}: descriptor ${oldSha} → ${newSha}; generation ${oldGeneration} → ${generation}`);
}

function resealCompiledDependencies(): void {
  const path = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json";
  plan(path, replaceExact(read(path), packVersion(FROM), packVersion(TO), 4, "compiled-dependencies Pack versions"));
  report.push(`${path}: 4 raw descriptor Pack versions ${FROM} → ${TO}`);
}

function resealLiterals(path: string, count: number): void {
  plan(path, replaceExact(read(path), literal(FROM), literal(TO), count, path));
  report.push(`${path}: ${count} literals ${FROM} → ${TO}`);
}

function resealFrozenBinding(): void {
  const path = "🌎️hub/🧫️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json";
  const digest = (binding: unknown): string =>
    createHash("sha256").update(Buffer.concat([Buffer.from("semio.hub.gis-map-frozen-binding/v1", "utf8"), Buffer.from([0])])).update(JSON.stringify(binding)).digest("hex");
  let text = read(path);
  const fixture = JSON.parse(text);
  const old = fixture.expectedDigest;
  if (digest(fixture.binding) !== old) throw new Error(`frozen binding: the committed digest does not re-verify under v${FROM}`);
  text = replaceExact(text, literal(FROM), literal(TO), 1, "frozen binding literal");
  const next = digest(JSON.parse(text).binding);
  text = replaceExact(text, old, next, 1, "frozen binding digest");
  plan(path, text);
  for (const quoting of ["🌎️hub/🧫️fixtures/🗺️gis-inference-job-v1/🔣️.json", "🌎️hub/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json"]) plan(quoting, replaceExact(read(quoting), old, next, 1, quoting));
  report.push(`${path} (+ gis-inference-job-v1, gis-map-proposal-approval-v1): digest ${old} → ${next}`);
}

resealPlan();
resealBrowser();
resealLease();
resealCompiledDependencies();
resealLiterals("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/👥️two-package/🔣️.json", 2);
resealLiterals("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧱️generation-stage/🔣️.json", 3);
resealLiterals("🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json", 2);
resealFrozenBinding();
if (planned.size === 0) throw new Error("nothing planned");
if (WRITE) for (const [path, text] of planned) writeFileSync(join(ROOT, path), text);
for (const line of report) console.log(`[reseal-v${TO}] ${line}`);
console.log(`[reseal-v${TO}] ${planned.size} files ${WRITE ? "written" : "would change (dry run)"}`);
