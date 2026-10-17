/** 🗂️ Inserts the new TypeScript packages of the given plans into the generated `workspaces` lists of the six Bun scopes: next to the sibling Rust package, in byte order. Dry-run by default. Usage: bun workspaces.ts <plan.json>... [--write] */
import { readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const SCOPES = [".", "✏️s", "🌎️hub", "♻️mit-bestand", "🏢️semio-tech", "🎓️teaching"];
const write = process.argv.includes("--write");
const plans = process.argv.slice(2).filter(a => a.endsWith(".json"));
const news: { dir: string; sibling: string; noSibling: boolean }[] = [];
for (const p of plans) for (const np of JSON.parse(readFileSync(p, "utf8")).newPackages ?? []) news.push({ dir: np.dir.normalize("NFC"), sibling: (np.sourceOwner + "/📦️packages/🦀️rust").normalize("NFC"), noSibling: !!np.noSibling });
const cmp = (a: string, b: string) => Buffer.from(a).compare(Buffer.from(b));
for (const scope of SCOPES) {
  const path = join(ROOT, scope, "package.json");
  const text = readFileSync(path, "utf8");
  const doc = JSON.parse(text);
  const roundtrip = JSON.stringify(doc, null, 2) + (text.endsWith("\n") ? "\n" : "");
  if (roundtrip !== text) { console.log(scope.padEnd(14), "SKIP: package.json not round-trip stable"); continue; }
  const list: string[] = doc.workspaces;
  const sorted = [...list].sort(cmp).every((v, i) => v === list[i]);
  const rel = (d: string) => (scope === "." ? d : relative(join(ROOT, scope), join(ROOT, d)).normalize("NFC"));
  let next = [...list], added = 0;
  for (const n of news) {
    const entry = rel(n.dir);
    if (next.includes(entry)) continue;
    const top = n.dir.split("/")[0];
    const hasSibling = next.includes(rel(n.sibling));
    const sameTop = next.some(e => (scope === "." ? e : join(scope, e)).normalize("NFC").replace(/^(\.\.\/)+/u, "").startsWith(top + "/"));
    if (!hasSibling && !(n.noSibling && sameTop)) continue;
    next.push(entry); added++;
  }
  if (!added) { console.log(scope.padEnd(14), "nothing to add"); continue; }
  if (sorted) next.sort(cmp);
  else { console.log(scope.padEnd(14), "NOTE: list is not byte-sorted; entries appended"); }
  console.log(scope.padEnd(14), `add ${added}, sortedBefore=${sorted}, total ${list.length} -> ${next.length}`);
  doc.workspaces = next;
  if (write) writeFileSync(path, JSON.stringify(doc, null, 2) + (text.endsWith("\n") ? "\n" : ""));
}
