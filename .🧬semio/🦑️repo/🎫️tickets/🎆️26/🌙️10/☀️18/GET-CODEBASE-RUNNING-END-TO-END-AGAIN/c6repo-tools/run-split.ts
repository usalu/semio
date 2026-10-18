import { existsSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { analyze, ROOT } from "./ast.ts";
import { definerIndex, exportMap } from "./exports.ts";
import { assign, build, moduleGraph, surfaceOf, write, type Plan } from "./split.ts";
import { repoint } from "./repoint.ts";
import { execFileSync } from "node:child_process";

const S = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad";
const planPath = process.argv[2];
const mode = process.argv[3] ?? "--dry";
const mod = await import(planPath);
const plan: Plan = mod.plan;
const patches: { dir: string; from: string; to: string }[] = mod.patches ?? [];
const a = analyze(plan.source);
const built = build(plan, a);
for (const p of patches) {
  const b = built.find((x) => x.mod.dir === p.dir)!;
  if (b.text.split(p.from).length !== 2) throw new Error(`patch anchor not unique: ${p.from}`);
  b.text = b.text.replace(p.from, p.to);
}
const lim = built.filter((b) => b.text.split("\n").length >= 1000);
for (const b of built) console.log(String(b.text.split("\n").length).padStart(5), b.file.slice(plan.baseDir.length + 1));
console.log("cycles", JSON.stringify(moduleGraph(built).cycles));
if (lim.length) console.log("OVER 1000:", lim.map((b) => b.file).join(", "));
const { map, warnings } = surfaceOf(plan, a, built, exportMap);
for (const w of warnings) console.log("WARN", w);
const keepsSource = built.some((b) => b.file === plan.source);
if (mode === "--write") {
  write(built, false, plan.source);
  writeFileSync(S + "/last-new-files.txt", built.map((b) => b.file).join("\n") + "\n");
}
if (mode === "--importers" || mode === "--write") {
  execFileSync("python3", ["-I", join(ROOT, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/c6repo-tools/edges.py"), S + "/edges.json"]);
  const edges = JSON.parse(readFileSync(S + "/edges.json", "utf8"));
  const old = new Set([plan.source]);
  const files = Object.entries<any>(edges).filter(([, rows]) => rows.some(([, r]: any) => r && old.has(r))).map(([f]) => f).sort();
  const skipSet = new Set(built.map((b) => b.file));
  const resolver = (n: string) => {
    const f = map.get(n);
    if (!f) return undefined;
    return f;
  };
  if (keepsSource) {
    console.log("source retained; importers resolved per symbol:", files.length);
  }
  const { changed, report } = repoint(files, { oldTargets: old, resolver, dry: mode !== "--write", skip: (f) => skipSet.has(f) || f === plan.source });
  console.log("importers", files.length, "changed", changed.length, mode === "--write" ? "(written)" : "(dry)");
  for (const r of report) console.log(r.kind, "|", r.file.slice(-100), "|", r.detail);
  writeFileSync(S + "/last-importers.txt", changed.join("\n") + "\n");
  if (mode === "--write" && !keepsSource) {
    const remaining = Object.entries<any>(JSON.parse(readFileSync(S + "/edges.json", "utf8"))).filter(([f, rows]) => !changed.includes(f) && f !== plan.source && rows.some(([, r]: any) => r === plan.source));
    console.log("remaining importers (not rewritten):", remaining.map(([f]) => f).join(", ") || "none");
  }
}
