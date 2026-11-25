/** 🩺️ Lists the catalog findings a built site carries in its asset modules and whether each module still has the revision of the authored file. Usage: node artifact-findings.mjs <dist-directory> */
import { createHash } from "node:crypto";
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { inflateSync } from "node:zlib";

const output = resolve(process.argv[2]);
const catalog = resolve(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const manifest = JSON.parse(readFileSync(join(catalog, "🔣️.json"), "utf8"));
const folders = new Map(manifest.categories.map(({ id, folder }) => [id, folder]));
const modules = new Map();
for (const file of readdirSync(join(output, "assets")).filter((name) => /^🔣️\.json-.+\.js$/u.test(name))) {
  const { default: payload } = await import(pathToFileURL(join(output, "assets", file)).href);
  const { id } = JSON.parse(inflateSync(Buffer.from(payload.header, "base64")).toString("utf8"));
  modules.set(id, payload);
}
const missing = manifest.entries.filter(({ id }) => !modules.has(id)).map(({ id }) => id);
const invalid = [...modules].filter(([, payload]) => payload.issues?.length).map(([id, payload]) => ({ id, issues: payload.issues.map(({ code, path }) => code + " " + path) }));
const drift = manifest.entries.filter(({ category, id }) => modules.has(id) && modules.get(id).revision !== "sha256:" + createHash("sha256").update(readFileSync(join(catalog, folders.get(category), id, "🔣️.json"), "utf8").replace(/\r\n/gu, "\n")).digest("hex")).map(({ id }) => id);
console.log(JSON.stringify({ modules: modules.size, entries: manifest.entries.length, missing, invalid, changedSinceBuild: drift }, null, 2));
