/** 🔎️ Ticket-local audit of every catalog asset and fixture asset: Ajv conformance against `🧬️schema/🔣️.json` (`#/$defs/Asset`) and the owned `validateAsset`.
 * Usage: bun catalog-audit.ts [--out <file>]  -> JSON list `{ source, asset, kind: "ajv"|"semantic", pointer, rule, message, volume }`. */
import Ajv from "ajv";
import { readdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { CATALOG, STAFFAGE, GENERATED } from "./volume-lib.ts";

const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });

const targets: { source: string; asset: any }[] = [];
for (const category of readdirSync(CATALOG)) {
  const dir = join(CATALOG, category);
  let entries: string[] = [];
  try { entries = readdirSync(dir); } catch { continue; }
  for (const slug of entries) {
    const file = join(dir, slug, "🔣️.json");
    if (!existsSync(file)) continue;
    const doc = JSON.parse(readFileSync(file, "utf8"));
    if (doc.asset) targets.push({ source: `catalog/${category}/${slug}`, asset: doc.asset });
  }
}
const FIXTURES = join(STAFFAGE, "🧫️fixtures");
for (const name of readdirSync(FIXTURES)) {
  const file = join(FIXTURES, name, "🔣️.json");
  if (!existsSync(file)) continue;
  const doc = JSON.parse(readFileSync(file, "utf8"));
  const asset = doc.asset && doc.asset.rig ? doc.asset : doc.rig && doc.representations ? doc : null;
  if (asset) targets.push({ source: `fixture/${name}`, asset });
}

type AjvError = NonNullable<typeof check.errors>[number];
/** Ajv reports every branch of a failed `oneOf`; drop the branches whose `kind` or `status` discriminator already mismatches, and a `oneOf` row that has a surviving branch error beneath it. */
function relevant(errors: readonly AjvError[]): AjvError[] {
  const dead: { schema: string; instance: string }[] = [];
  for (const e of errors) {
    const m = /^(.*)\/properties\/(kind|status)\/const$/.exec(e.schemaPath);
    if (e.keyword === "const" && m) dead.push({ schema: m[1]!, instance: e.instancePath.slice(0, -`/${m[2]}`.length) });
  }
  const alive = errors.filter((e) => !dead.some((d) => (e.instancePath === d.instance ? e.schemaPath.startsWith(d.schema + "/") : e.instancePath.startsWith(d.instance + "/") && e.schemaPath.startsWith(d.schema + "/properties/"))));
  return alive.filter((e) => e.keyword !== "oneOf" || !alive.some((o) => o !== e && o.keyword !== "oneOf" && o.schemaPath.startsWith(e.schemaPath.replace(/oneOf$/, "oneOf/")) && (o.instancePath === e.instancePath || o.instancePath.startsWith(e.instancePath + "/"))));
}
const at = (root: any, pointer: string): unknown => pointer.split("/").slice(1).reduce((n, k) => (n === undefined || n === null ? undefined : n[k]), root);
const rows: any[] = [];
const seen = new Set<string>();
const push = (row: any): void => {
  const key = JSON.stringify([row.source, row.kind, row.pointer, row.rule, row.message]);
  if (!seen.has(key)) { seen.add(key); rows.push(row); }
};
const out = process.argv.includes("--out") ? process.argv[process.argv.indexOf("--out") + 1]! : join(GENERATED, "catalog-fix", "audit.json");
for (const { source, asset } of targets) {
  check(asset);
  for (const e of relevant(check.errors ?? [])) {
    const pointer = e.instancePath || "";
    const found = at(asset, pointer);
    const shown = e.keyword === "pattern" || e.keyword === "type" || e.keyword === "minimum" || e.keyword === "exclusiveMinimum" || e.keyword === "const" ? ` (value ${JSON.stringify(found)?.slice(0, 80)})` : "";
    push({ source, asset: asset.id, kind: "ajv", pointer, rule: e.keyword, message: `${e.message}${e.params && Object.keys(e.params).length ? " " + JSON.stringify(e.params) : ""}${shown}`, volume: pointer.startsWith("/volume") });
  }
  for (const i of validateAsset(asset)) push({ source, asset: asset.id, kind: "semantic", pointer: i.path, rule: i.code, message: i.message.en, volume: String(i.code).startsWith("volume-") || String(i.path).startsWith("volume") });
}
writeFileSync(out, JSON.stringify({ assets: targets.length, rows }, null, 1));
console.log(`${targets.length} assets, ${rows.length} findings -> ${out}`);
