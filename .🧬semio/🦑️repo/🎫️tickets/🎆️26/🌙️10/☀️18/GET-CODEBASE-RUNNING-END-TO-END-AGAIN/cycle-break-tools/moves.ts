/** 🚚 Directory moves with relative-specifier re-pointing: every import that crosses the moved set keeps resolving to the same file. */
import { dirname, relative, posix as pp } from "node:path";
import { spawnSync } from "node:child_process";
import { scan } from "../exports-stage-tools/scan.ts";
import { listAll } from "../exports-stage-tools/lib.ts";
import { ROOT } from "./lib.ts";
import type { Plan } from "./plan.ts";

export type DirMove = { from: string; to: string };
const posixPath = (p: string): string => p.split("\\").join("/");

export function planMoves(plan: Plan, wave: string, why: string, moves: DirMove[], opts: { allowMissing?: boolean } = {}): { moved: Map<string, string>; reports: string[] } {
  const all = listAll();
  const moved = new Map<string, string>();
  for (const m of moves) {
    const files = all.filter(f => f === m.from || f.startsWith(m.from + "/")).filter(f => !f.includes("/node_modules/"));
    if (!files.length && !opts.allowMissing) plan.flag(wave, "blocker", `nothing to move under ${m.from}`);
    for (const f of files) moved.set(f, m.to + f.slice(m.from.length));
  }
  const mapped = (f: string): string => moved.get(f) ?? f;
  const { edges } = scan();
  const reports: string[] = [];
  const perFile = new Map<string, Map<string, string>>();
  for (const e of edges) {
    const X = mapped(e.file), T = mapped(e.target);
    if (X === e.file && T === e.target) continue;
    const base = e.spec.split("?")[0]!, query = e.spec.slice(base.length);
    let rel = posixPath(relative(dirname(X), T));
    if (!rel.startsWith(".")) rel = "./" + rel;
    const normalized = pp.normalize(pp.join(dirname(e.file), base));
    let spec = rel;
    if (normalized === e.target) spec = rel;
    else if (e.target.startsWith(normalized + ".")) spec = rel.slice(0, rel.length - (e.target.length - normalized.length)) ;
    else if (e.target.startsWith(normalized + "/")) spec = rel.slice(0, rel.length - (e.target.length - normalized.length));
    else plan.flag(wave, "note", `unclassified specifier style ${e.spec} in ${e.file}`);
    if (base.endsWith(".js") && !spec.endsWith(".js")) spec = spec.replace(/\.(ts|tsx)$/u, ".js");
    spec += query;
    if (spec === e.spec) continue;
    const m = perFile.get(e.file) ?? perFile.set(e.file, new Map()).get(e.file)!;
    const have = m.get(e.spec);
    if (have !== undefined && have !== spec) plan.flag(wave, "blocker", `ambiguous re-pointing in ${e.file}: ${e.spec} -> ${have} | ${spec}`);
    m.set(e.spec, spec);
  }
  for (const [file, m] of perFile) for (const [from, to] of m) plan.edit({ file, from, to, mode: "specifier", wave, why });
  for (const [from, to] of moved) plan.move(from, to, wave, why);
  const nonSource = [...new Set(moves.flatMap(m => {
    const r = spawnSync("git", ["grep", "-l", "-F", m.from, "--", ".", ":(exclude).🧬semio", ":(exclude)*bun.lock", ":(exclude)🔒️dependencies.json"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 26 });
    return r.stdout.split("\n").filter(f => f && !/\.(ts|tsx|mts|js|mjs|cjs)$/u.test(f) && !moved.has(f));
  }))];
  if (nonSource.length) reports.push(`non-source files mentioning a moved path (review): ${nonSource.join(", ")}`);
  const dynamic = [...moved.keys()].filter(f => /\.(ts|tsx|mts)$/u.test(f)).filter(f => { const t = require("node:fs").readFileSync(`${ROOT}/${f}`, "utf8"); return /new URL\(|import\.meta\.(dir|dirname|url)|__dirname|readFileSync\(/u.test(t); });
  if (dynamic.length) reports.push(`moved files with path-sensitive code (review): ${dynamic.length} e.g. ${dynamic.slice(0, 3).join(", ")}`);
  void pp;
  return { moved, reports };
}
