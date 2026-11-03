import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const catalog = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
const manifest = JSON.parse(readFileSync(join(catalog, "🔣️.json"), "utf8"));
const ids = new Map<string, number>();
for (const entry of manifest.entries) ids.set(entry.id, (ids.get(entry.id) ?? 0) + 1);
const found: string[] = [];
const walk = (dir: string): void => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) walk(path);
    else if (name === "🔣️.json" && path !== join(catalog, "🔣️.json")) found.push(path);
  }
};
walk(catalog);
const seen = new Map<string, string[]>();
for (const path of found) {
  const document = JSON.parse(readFileSync(path, "utf8"));
  const id = document.asset?.id ?? document.id;
  seen.set(id, [...(seen.get(id) ?? []), path]);
}
console.log("manifest", manifest.entries.length, "files", found.length);
for (const [id, paths] of seen) if (!ids.has(id) || paths.length > 1) console.log("ORPHAN/DUP", id, paths);
for (const id of ids.keys()) if (!seen.has(id)) console.log("MISSING", id);
for (const [id, n] of ids) if (n > 1) console.log("DUP manifest", id);
