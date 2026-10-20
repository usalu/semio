import { readFileSync, statSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { rustFunctions, tsFunctions, type FnRow } from "./census.ts";

const ROOT = process.cwd();
const ts = createRequire(import.meta.url)(process.env.TYPESCRIPT_PATH ?? "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript") as typeof import("typescript");
const census1 = readFileSync(process.argv[2]!, "utf8").split("\n").filter((l) => l.startsWith("| ") && /^\| \d+ \|/.test(l)).map((l) => {
  const c = l.split("|").map((x) => x.trim());
  const fs = c[4]!.replace(/`/g, "");
  const ix = fs.lastIndexOf(":");
  return { lines: Number(c[1]), kind: c[2]!, name: c[3]!.replace(/`/g, ""), file: fs.slice(0, ix), start: Number(fs.slice(ix + 1)) };
}).filter((r) => r.lines > 600);
const census4 = (JSON.parse(readFileSync(process.argv[3]!, "utf8")) as FnRow[]).filter((r) => !r.nested && r.lines > 600);
const SINCE = Date.parse("2026-10-18T00:00:00");
const tracked = execFileSync("git", ["ls-files", "-co", "--exclude-standard", "-z"], { cwd: ROOT, maxBuffer: 1 << 28 }).toString("utf8").split("\0").filter((p) => /\.(rs|ts|tsx)$/.test(p));
const recent = (p: string) => { try { return statSync(join(ROOT, p)).mtimeMs >= SINCE; } catch { return false; } };
const fns = (file: string): FnRow[] => {
  const text = readFileSync(join(ROOT, file), "utf8");
  return file.endsWith(".rs") ? rustFunctions(file, text) : tsFunctions(ts, file, text);
};
const outermost = (rows: FnRow[]) => rows.filter((r) => !rows.some((o) => o !== r && o.start <= r.start && o.end >= r.end && (o.start < r.start || o.end > r.end)));
const nested = (r: (typeof census1)[number]) => census1.some((o) => o !== r && o.file === r.file && o.start <= r.start && o.start + o.lines >= r.start + r.lines && (o.start < r.start || o.lines > r.lines));
const out: string[] = ["| function | before | after root | after largest helper | state |", "| --- | ---: | ---: | ---: | --- |"];
for (const r of census1) {
  if (nested(r)) continue;
  const still = census4.find((c) => c.file === r.file && c.name === r.name);
  if (still) { out.push(`| \`${r.name}\` (\`${r.file.split("/").slice(-3).join("/")}\`) | ${r.lines} | ${still.lines} | - | remaining |`); continue; }
  let rootLines = "?";
  try {
    const rows = fns(r.file);
    const same = rows.filter((x) => x.name === r.name).sort((a, b) => b.lines - a.lines)[0];
    rootLines = same ? String(same.lines) : String(Math.max(...outermost(rows).map((x) => x.lines)));
  } catch { rootLines = "n/a"; }
  const dir = dirname(r.file);
  const kids = tracked.filter((p) => p.startsWith(dir + "/") && p !== r.file && recent(p));
  let max = 0;
  for (const k of kids) { try { for (const x of outermost(fns(k))) max = Math.max(max, x.lines); } catch {} }
  out.push(`| \`${r.name}\` (\`${r.file.split("/").slice(-3).join("/")}\`) | ${r.lines} | ${rootLines} | ${max || "-"} | split (${kids.length} new files) |`);
}
console.log(out.join("\n"));
