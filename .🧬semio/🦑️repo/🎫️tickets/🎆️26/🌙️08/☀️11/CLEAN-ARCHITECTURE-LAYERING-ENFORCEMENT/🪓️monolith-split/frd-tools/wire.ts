// 🧲️ Gives every crate whose sources use the shared fixture reader its dependency row: dev-dependency, normal dependency (test-support crates) or optional dependency behind the feature that gates the sources.
// usage: bun wire.ts [--apply] [--out=<prefix>]   (needs frd-owners.json and frd-gating.json from owners.ts and gating.ts)
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative, resolve } from "node:path";
import { FIXTURE_READER_CRATE, FIXTURE_READER_PACKAGE_DIR } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧪️test-closure/🟦️.ts";
import { insertDevDependencyRow } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const F = join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split");
const G = join(F, "🗑️generated");
const ORIG = join(F, "frd-tools/orig-manifests");
const args = process.argv.slice(2);
const flag = (name: string) => args.find(a => a.startsWith(`--${name}=`))?.slice(name.length + 3);
const apply = args.includes("--apply");
const outPrefix = flag("out") ?? "frd-wire";
mkdirSync(ORIG, { recursive: true });
const SELF = join(FIXTURE_READER_PACKAGE_DIR, "Cargo.toml");
const owners = JSON.parse(readFileSync(join(G, "frd-owners.json"), "utf8")) as Record<string, { manifest: string; cls: string }[]>;
const classes = JSON.parse(readFileSync(join(G, "frd-dep-classes.json"), "utf8")) as { normal: string[]; feature: string[] };
const FEATURE_NAME: Record<string, string> = { "component-codecs": "integration-fixtures", "instance": "artifact-app-testing" };

const used = new Set<string>();
for (const [file, rows] of Object.entries(owners)) {
  const abs = join(ROOT, file);
  if (!existsSync(abs) || !readFileSync(abs, "utf8").includes("semio_framework_test_fixtures")) continue;
  for (const row of rows) used.add(row.manifest);
}
used.delete(SELF);

const insertRow = (text: string, section: "dependencies" | "dev-dependencies", row: string): string => {
  if (section === "dev-dependencies") return insertDevDependencyRow(text, row);
  const key = /^\s*([A-Za-z0-9_\-]+)\s*=/u.exec(row)![1]!;
  const lines = text.split("\n");
  const header = lines.findIndex(line => /^\[dependencies\]\s*$/u.test(line));
  if (header < 0) throw new Error("no [dependencies]");
  let end = header + 1;
  while (end < lines.length && !/^\[/u.test(lines[end]!)) end += 1;
  if (lines.slice(header + 1, end).some(line => new RegExp(`^\\s*${key}\\s*=`, "u").test(line))) return text;
  let last = header;
  for (let at = header + 1; at < end; at += 1) {
    const line = lines[at]!;
    if (line.trim() === "" || line.trim().startsWith("#")) continue;
    const existing = /^\s*([A-Za-z0-9_\-]+)\s*=/u.exec(line)?.[1];
    if (existing !== undefined && existing > key) { lines.splice(at, 0, row); return lines.join("\n"); }
    last = at;
  }
  while (last + 1 < end && lines[last + 1]!.trim() !== "" && !/^\s*[A-Za-z0-9_\-]+\s*=/u.test(lines[last + 1]!)) last += 1;
  lines.splice(last + 1, 0, row);
  return lines.join("\n");
};
const addToFeature = (text: string, feature: string): string => {
  const entry = '"dep:semio-framework-test-fixtures"';
  const pattern = new RegExp(`^(${feature}\\s*=\\s*\\[)([^\\]]*)(\\])`, "mu");
  const match = pattern.exec(text);
  if (!match) throw new Error(`feature ${feature} is not a one-line array`);
  if (match[2]!.includes("dep:semio-framework-test-fixtures")) return text;
  const body = match[2]!.trim();
  return text.replace(pattern, (_all, open, _body, close) => `${open}${body ? `${body}, ` : ""}${entry}${close}`);
};

type Outcome = { manifest: string; status: string; kind?: string; detail?: string };
const outcomes: Outcome[] = [];
for (const manifest of [...used].sort()) {
  const abs = join(ROOT, manifest);
  if (!existsSync(abs)) { outcomes.push({ manifest, status: "missing" }); continue; }
  const text = readFileSync(abs, "utf8");
  const path = relative(dirname(abs), resolve(ROOT, FIXTURE_READER_PACKAGE_DIR));
  const plain = `${FIXTURE_READER_CRATE} = { path = "${path}" }`;
  let next = text, kind = "dev";
  try {
    if (classes.normal.includes(manifest)) { kind = "normal"; next = insertRow(text, "dependencies", plain); }
    else if (classes.feature.includes(manifest)) {
      kind = "feature";
      const parsed = Bun.TOML.parse(text) as any;
      const name = Object.keys(parsed.features ?? {}).find(n => n === "testkit") ?? FEATURE_NAME[Object.keys(FEATURE_NAME).find(k => manifest.includes(`/${k}/`) || manifest.includes(`-${k}`)) ?? ""] ?? Object.keys(parsed.features ?? {}).find(n => /testing|fixtures/u.test(n));
      if (!name) throw new Error("no gating feature found");
      next = insertRow(text, "dependencies", `${FIXTURE_READER_CRATE} = { path = "${path}", optional = true }`);
      next = addToFeature(next, name);
      if (classes.feature.includes(manifest)) next = insertRow(next, "dev-dependencies", plain);
    } else next = insertRow(text, "dev-dependencies", plain);
  } catch (error) { outcomes.push({ manifest, status: "error", kind, detail: String((error as Error).message).slice(0, 120) }); continue; }
  if (next === text) { outcomes.push({ manifest, status: "already", kind }); continue; }
  try { Bun.TOML.parse(next); } catch (error) { outcomes.push({ manifest, status: "toml-error", kind, detail: String(error).slice(0, 120) }); continue; }
  if (!apply) { outcomes.push({ manifest, status: "plan", kind }); continue; }
  if (readFileSync(abs, "utf8") !== text) { outcomes.push({ manifest, status: "raced", kind }); continue; }
  const key = createHash("sha1").update(manifest).digest("hex");
  if (!existsSync(join(ORIG, key + ".toml"))) writeFileSync(join(ORIG, key + ".toml"), text);
  writeFileSync(abs, next);
  outcomes.push({ manifest, status: "applied", kind });
}
const counts: Record<string, number> = {};
for (const o of outcomes) counts[`${o.status}/${o.kind ?? ""}`] = (counts[`${o.status}/${o.kind ?? ""}`] ?? 0) + 1;
writeFileSync(join(G, `${outPrefix}.json`), JSON.stringify(outcomes, null, 1));
console.log(JSON.stringify(counts), "manifests", used.size);
