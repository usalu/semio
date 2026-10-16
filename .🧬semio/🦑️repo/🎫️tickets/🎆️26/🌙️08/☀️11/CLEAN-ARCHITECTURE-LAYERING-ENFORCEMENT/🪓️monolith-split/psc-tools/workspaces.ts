/** 🗂️ Inserts the new schema TypeScript package into the generated `workspaces` lists of the six Bun scopes, at its byte-order position (what `workspaces --write` derives). Dry-run by default. Usage: bun workspaces.ts [--write] */
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const NEW = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🟦️typescript";
const SCOPES = [".", "✏️s", "🌎️hub", "♻️mit-bestand", "🏢️semio-tech", "🎓️teaching"];
const write = process.argv.includes("--write");
const cmp = (a: string, b: string) => Buffer.from(a).compare(Buffer.from(b));
for (const dir of SCOPES) {
  const path = join(ROOT, dir, "package.json");
  const text = readFileSync(path, "utf8");
  const doc = JSON.parse(text);
  const list: string[] = doc.workspaces;
  const entry = dir === "." ? NEW : relative(join(ROOT, dir), join(ROOT, NEW)).normalize("NFC");
  const sorted = [...list].sort(cmp).every((v, i) => v === list[i]);
  if (list.includes(entry)) { console.log(dir.padEnd(14), "already listed"); continue; }
  const validator = (dir === "." ? "" : "../") + "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript";
  if (!list.includes(validator)) { console.log(dir.padEnd(14), "SKIP: scope does not list the schema validator package"); continue; }
  const next = [...list, entry].sort(cmp);
  const roundtrip = JSON.stringify(doc, null, 2) + (text.endsWith("\n") ? "\n" : "");
  console.log(dir.padEnd(14), `sortedBefore=${sorted} roundtrip=${roundtrip === text} insertAt=${next.indexOf(entry)}/${next.length}`);
  if (!sorted || roundtrip !== text) continue;
  doc.workspaces = next;
  if (write) writeFileSync(path, JSON.stringify(doc, null, 2) + (text.endsWith("\n") ? "\n" : ""));
}
