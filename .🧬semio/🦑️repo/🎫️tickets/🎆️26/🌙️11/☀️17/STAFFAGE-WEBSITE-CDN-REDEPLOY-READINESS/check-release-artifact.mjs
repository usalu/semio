/** 📦️ Verifies a static release and reports any catalog revisions that differ from the live worktree. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const staffage = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const args = process.argv.slice(2);
const output = resolve(args.find((arg) => arg !== "--require-current") ?? join(staffage, "🌐️site/📦️packages/🟦️typescript/dist/release"));
const assets = readdirSync(join(output, "assets"));
const read = (path) => readFileSync(join(output, path), "utf8");
const sha256 = (source) => createHash("sha256").update(source);
const root = "https://staffage.semio-tech.com";

assert.equal(read("CNAME").trim(), "staffage.semio-tech.com");
assert(existsSync(join(output, ".nojekyll")));
const html = read("index.html");
assert.equal(read("404.html"), html);
const policy = html.match(/http-equiv="Content-Security-Policy" content="([^"]+)"/u)?.[1];
assert(policy);
for (const [, code] of html.matchAll(/<(?:script|style)\b[^>]*>([\s\S]*?)<\/(?:script|style)>/gu)) {
  if (code) assert(policy.includes("'sha256-" + sha256(code).digest("base64") + "'"));
}
assert(html.indexOf("__staffage_recovery-") < html.indexOf('id="staffage-entry"'));
assert(html.includes("__staffage_recovery-") && html.includes('id="staffage-entry"'));

const verify = (url) => {
  const target = new URL(url, root);
  assert.equal(target.origin, root);
  const path = join(output, decodeURIComponent(target.pathname));
  assert(existsSync(path) && statSync(path).isFile(), url);
};
for (const tag of html.matchAll(/<(?:link|script)\b[^>]*>/gu)) {
  for (const [, url] of tag[0].matchAll(/\b(?:href|src)="([^"]+)"/gu)) verify(url);
}
for (const css of assets.filter((path) => path.endsWith(".css"))) {
  for (const [, url] of read(join("assets", css)).matchAll(/url\(["']?(\/[^"')]+)["']?\)/gu)) verify(url);
}
for (const { src } of JSON.parse(read("manifest.webmanifest")).icons) verify(src);
const dwg = assets.find((path) => /^staffage_dwg-.+\.wasm$/u.test(path));
assert(dwg && WebAssembly.validate(readFileSync(join(output, "assets", dwg))), "DWG WebAssembly");

const revisions = new Map();
for (const file of assets.filter((path) => /^🔣️\.json-.+\.js$/u.test(path))) {
  const { default: payload } = await import(pathToFileURL(join(output, "assets", file)).href);
  const { id } = JSON.parse(payload.json);
  assert.match(payload.revision, /^sha256:[a-f0-9]{64}$/u, id);
  assert.deepEqual(payload.issues, [], id);
  assert(!revisions.has(id), id + " appears in more than one asset module");
  revisions.set(id, payload.revision);
}
const catalog = join(staffage, "🗂️catalog");
const manifest = JSON.parse(readFileSync(join(catalog, "🔣️.json"), "utf8"));
assert.deepEqual([...revisions.keys()].sort(), manifest.entries.map(({ id }) => id).sort());
const folders = new Map(manifest.categories.map(({ id, folder }) => [id, folder]));
const drift = manifest.entries.filter(({ category, id }) => {
  const source = readFileSync(join(catalog, folders.get(category), id, "🔣️.json"), "utf8").replace(/\r\n/gu, "\n");
  return revisions.get(id) !== "sha256:" + sha256(source).digest("hex");
}).map(({ id }) => id);
if (args.includes("--require-current")) assert.deepEqual(drift, [], "Catalog source differs from the release");
console.log("Static shell, links, CSP, DWG WebAssembly and " + revisions.size + " valid catalog modules verified");
if (drift.length) console.log("Live catalog revisions that differ from the release: " + drift.join(", "));
