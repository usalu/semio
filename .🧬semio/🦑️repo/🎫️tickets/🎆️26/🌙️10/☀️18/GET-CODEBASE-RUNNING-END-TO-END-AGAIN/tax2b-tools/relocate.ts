// 🚚 Moves lifted modules to a new directory: rebases the module's own relative specifiers and re-points its importers.
// usage: bun relocate.ts <pairs.tsv> [--write]   (rows: fromFile<TAB>toFile, repo-relative)
import {existsSync, mkdirSync, readFileSync, unlinkSync, writeFileSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
const REPO = "/Users/ueli/Documents/semio";
const write = process.argv.includes("--write");
const abs = (p: string) => join(REPO, p);
const spec = (fromDir: string, target: string) => { const r = relative(fromDir, target); return r.startsWith(".") ? r : "./" + r; };
const quoted = /(["'`])(\.{1,2}\/(?:(?!\1)[^\n\\$])*)\1/gu;
const SOURCES = ["", ".ts", ".tsx", "/🟦️.ts", "/🟦️.tsx"];
const exists = (p: string) => SOURCES.some((s) => existsSync(p + s));
for (const row of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const [from, to] = row.split("\t") as [string, string];
  if (!existsSync(abs(from)) || existsSync(abs(to))) { console.log("skip", from, existsSync(abs(from)) ? "occupied" : "missing"); continue; }
  const oldDir = dirname(abs(from)), newDir = dirname(abs(to));
  const moved = readFileSync(abs(from), "utf8").replace(quoted, (m, q: string, v: string) => { const t = resolve(oldDir, v); return exists(t) ? q + spec(newDir, t) + q : m; });
  const importers = ((): string[] => { const out: string[] = []; for (const f of [join(dirname(from), "📜️script.ts")]) if (existsSync(abs(f))) out.push(f); return out; })();
  const edits = importers.map((f) => { const text = readFileSync(abs(f), "utf8"); const next = text.replace(quoted, (m, q: string, v: string) => resolve(dirname(abs(f)), v) === abs(from) ? q + spec(dirname(abs(f)), abs(to)) + q : m); return {f, next, changed: next !== text}; });
  console.log(write ? "move" : "plan", from, "->", to, edits.map((e) => `${e.f.split("/").pop()}:${e.changed}`).join(","));
  if (!write) continue;
  mkdirSync(newDir, {recursive: true});
  writeFileSync(abs(to), moved);
  for (const e of edits) if (e.changed) writeFileSync(abs(e.f), e.next);
  unlinkSync(abs(from));
}
