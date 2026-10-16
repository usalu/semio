/** 📦️ hr2 not-to-unlisted analyser: maps every cruiser not-to-unlisted edge to the importer's nearest package.json. Usage: bun unlisted.ts <err.txt> [--tests] */
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const [errPath, flag] = process.argv.slice(2);
const includeTests = flag === "--tests";
const lines = readFileSync(errPath!, "utf8").split("\n");
const rows = new Map<string, { manifest: string; pkg: string; files: Set<string>; declared: string | null }>();
function nearest(file: string): string | null {
  let dir = dirname(join(ROOT, file));
  while (dir.length > ROOT.length - 1) {
    const candidate = join(dir, "package.json");
    if (existsSync(candidate)) return candidate.slice(ROOT.length + 1);
    if (dir === ROOT) break;
    dir = dirname(dir);
  }
  return null;
}
let edges = 0;
for (const line of lines) {
  const m = /^\s*error not-to-unlisted: (.*) → (.*)$/u.exec(line);
  if (!m) continue;
  const from = m[1]!;
  if (!includeTests && /\/🧪️tests?\//u.test(from)) continue;
  const target = m[2]!.replace(/^.*node_modules\//u, "");
  const parts = target.split("/");
  const pkg = parts[0]!.startsWith("@") ? `${parts[0]}/${parts[1]}` : parts[0]!;
  const manifest = nearest(from) ?? "(none)";
  edges++;
  const key = `${manifest}\t${pkg}`;
  const row = rows.get(key) ?? { manifest, pkg, files: new Set<string>(), declared: null };
  row.files.add(from);
  rows.set(key, row);
}
console.log(`edges=${edges} groups=${rows.size}`);
for (const row of [...rows.values()].sort((a, b) => a.manifest.localeCompare(b.manifest) || a.pkg.localeCompare(b.pkg))) {
  console.log(`${row.manifest}\t${row.pkg}\t${row.files.size}`);
}
