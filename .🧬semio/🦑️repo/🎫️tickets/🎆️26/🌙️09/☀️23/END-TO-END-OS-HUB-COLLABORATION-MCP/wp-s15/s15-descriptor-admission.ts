/** 🔎️ S15 — replays `parseVerifiedPackageDescriptorV1` (browser worker) field by field on one hub execution target:
 * the lease manifest (`generated/s15-<x>-exec-manifest.json`) and the catalog package's `descriptor.semio`. */
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { decodePackValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
const [manifestPath, descriptorPath] = process.argv.slice(2);
const fields = JSON.parse(readFileSync(manifestPath!, "utf8"));
const bytes = new Uint8Array(readFileSync(descriptorPath!));
const plain = (value: unknown): unknown => JSON.parse(JSON.stringify(value, (_k, v) => (typeof v === "bigint" ? String(v) : v && typeof v === "object" && v.kind === "uint" ? String(v.value) : v)));
const d = plain(decodePackValue(bytes)) as any;
const app = (d.manifest?.apps ?? []).find((entry: any) => entry.id === fields.surface.appId);
const window = (app?.windowKinds ?? []).find((entry: any) => entry.id === fields.surface.windowKindId);
const checks: [string, unknown, unknown][] = [
  ["descriptor sha256", createHash("sha256").update(bytes).digest("hex"), fields.descriptor.sha256],
  ["packageId", d.packageId, fields.package.packageId],
  ["pluginId", d.manifest?.pluginId, fields.package.pluginId],
  ["version", d.manifest?.version, fields.package.version],
  ["hashes.wasmSha256", d.hashes?.wasmSha256, fields.component.sha256],
  ["app found", app !== undefined, true],
  ["app.id == surfaceId", app?.id, fields.surface.surfaceId],
  ["app.role", app?.role, fields.surface.role],
  ["dialect.artifactKind", app?.dialect?.artifactKind, fields.parentDialect.artifactKind],
  ["dialect.standard", app?.dialect?.standard, fields.parentDialect.standard],
  ["dialect.subset", app?.dialect?.subset, fields.parentDialect.subset],
  ["window found", window !== undefined, true],
  ["window.bodyKey", typeof window?.bodyKey, "string"],
  ["artifactKinds has kind+schema", (d.manifest?.artifactKinds ?? []).some((k: any) => k.id === fields.artifact.kind && k.schema === fields.artifact.schema), true],
];
for (const [name, actual, expected] of checks) console.log(`${actual === expected ? "ok  " : "FAIL"} ${name}: ${JSON.stringify(actual)} vs ${JSON.stringify(expected)}`);
console.log("apps:", (d.manifest?.apps ?? []).map((entry: any) => `${entry.id}[${(entry.windowKinds ?? []).map((w: any) => w.id).join(",")}]`).join(" "));
console.log("artifactKinds:", JSON.stringify((d.manifest?.artifactKinds ?? []).map((k: any) => [k.id, k.schema])));
