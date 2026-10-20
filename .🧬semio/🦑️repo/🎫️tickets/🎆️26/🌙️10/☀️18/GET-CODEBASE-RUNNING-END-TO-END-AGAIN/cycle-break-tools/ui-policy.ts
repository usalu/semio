/** 🖱️ File-level evaluator of the ui boundary rules (ui-no-framework-packages, ui-foundation-closure) on the source graph with glue resolved to module sources. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { scan } from "../exports-stage-tools/scan.ts";
import { model } from "../exports-stage-tools/model.ts";
import { GENERATED, ROOT } from "./lib.ts";
const FW = "🧰️framework/";
const TAX = FW + "🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const taxonomy = JSON.parse(readFileSync(resolve(process.argv.includes("--mirror") ? resolve(GENERATED, "mirror") : ROOT, TAX), "utf8"));
const roles = taxonomy.dependencyDirections.roles as Record<string, { ownerPaths: string[] }>;
const BOOT = /(^|\/)(?:📜️script\.ts|🏗️builder\/🌐️vite\/🟦️\.ts|🧪️tests\/🎚️config\/🟦️\.ts|(?:⚙️|🧪️)?(?:vite|vitest)\.config\.[cm]?[jt]s)$/u;
const UITEST = /(^|\/)🧪️(?:tests|testing)\/|\.(?:test|spec|story)\.[cm]?[jt]sx?$/u;
const BUILDER = /(^|\/)🏗️builder\//u;
const under = (f: string, ps: string[]): boolean => ps.some(p => f === p || f.startsWith(p + "/"));

const delta = JSON.parse(readFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), "utf8")) as { fileOps: { op: string; path: string; to?: string }[]; manifestOps: { op: string; field: string; package: string; key: string }[] };
export const DELETED = new Set(delta.fileOps.filter(o => o.op === "delete").map(o => o.path));
export const MOVES = new Map(delta.fileOps.filter(o => o.op === "move").map(o => [o.path, o.to!]));
const mirrorOf = (f: string): string => f.replace(FW + "📦️packages/🟦️typescript/", FW + "🔨️modules/").replace("🛂️manifest/🎚️ui-axes", "🛂️manifest/🤖️generated/🎚️ui-axes");

const M = FW + "🔨️modules/";
export const PROPOSAL = process.argv.includes("--proposal") ? { foundation: [M + "🛂️manifest/📋️context-menu", M + "🎓️onboarding/🗺️window-layout", M + "🔏️hash"] } : {};
export function evaluate(opts: { foundation?: string[]; drop?: string[]; post?: boolean } = {}) {
  const found = (roles["ui-foundation"]!.ownerPaths.filter(p => !(opts.drop ?? []).includes(p))).concat(opts.foundation ?? []);
  const ui = roles["ui"]!.ownerPaths, support = roles["ui-test-support"]!.ownerPaths;
  const { edges, bare } = scan();
  const glue = new Map<string, string[]>();
  for (const e of edges) (glue.get(e.file) ?? glue.set(e.file, []).get(e.file)!).push(e.target);
  const mv = (f: string): string => (opts.post ? MOVES.get(f) : undefined) ?? f;
  const pairs: { from: string; to: string }[] = [];
  for (const e of edges) pairs.push({ from: e.file, to: opts.post && DELETED.has(e.target) ? (glue.get(e.target)?.[0] ?? mirrorOf(e.target)) : e.target });
  for (const b of bare) {
    const P = model.byName.get(b.pkg); const v = P?.exports[b.sub];
    if (!P || typeof v !== "string" || !v.startsWith("./")) continue;
    const file = P.dir + "/" + v.slice(2);
    if (opts.post && DELETED.has(file)) { const ts = glue.get(file); if (ts?.length) for (const t of ts) pairs.push({ from: b.file, to: t }); else pairs.push({ from: b.file, to: mirrorOf(file) }); } else pairs.push({ from: b.file, to: file });
  }
  const r1 = new Map<string, Set<string>>(), r2 = new Map<string, Set<string>>();
  for (const p of pairs) {
    const from = mv(p.from), to = mv(p.to);
    if (!to.startsWith(FW) || under(to, ui) || under(to, found)) continue;
    if (under(from, ui) && !BOOT.test(from) && !under(to, support)) (r1.get(to) ?? r1.set(to, new Set()).get(to)!).add(from);
    if (under(from, found) && !BOOT.test(from) && !UITEST.test(from) && !BUILDER.test(from)) (r2.get(to) ?? r2.set(to, new Set()).get(to)!).add(from);
  }
  return { r1, r2, found };
}
if (import.meta.main) {
  const { r1, r2 } = evaluate({ post: process.argv.includes("--post"), ...PROPOSAL });
  const dir = (f: string): string => f.split("/").slice(0, 7).join("/").replace(FW, "");
  const show = (name: string, m: Map<string, Set<string>>) => { const by = new Map<string, number>(); for (const [t, fs] of m) by.set(dir(t), (by.get(dir(t)) ?? 0) + fs.size); console.log(name, "targets", m.size, "edges", [...m.values()].reduce((a, s) => a + s.size, 0)); for (const [d, n] of [...by].sort((a, b) => b[1] - a[1]).slice(0, 25)) console.log("  ", String(n).padStart(4), d); };
  show("ui-no-framework-packages", r1); show("ui-foundation-closure", r2);
}
if (import.meta.main && process.argv.includes("--list")) {
  const { r1, r2 } = evaluate({ post: process.argv.includes("--post"), ...PROPOSAL });
  for (const [n, m] of [["R1", r1], ["R2", r2]] as const) for (const [t, fs] of m) for (const f of fs) console.log(n, f.replace(FW, ""), "->", t.replace(FW, ""));
}
