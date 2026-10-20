/** 🧰 Shared helpers of the cycle-break tools: paths, binding manifests, delta model. */
import { existsSync, readFileSync } from "node:fs";
import { relative } from "node:path";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { ROOT, loadPackages, posix, type Pkg } from "../exports-stage-tools/lib.ts";

export { ROOT, posix };
export const TICKET = resolve(import.meta.dir, "..");
export const GENERATED = resolve(TICKET, "🗑️generated/cycle-break-x");
export const model = loadPackages();
const dirs = new Map(model.all.map(p => [p.dir, p]));
const bound = (dir: string): boolean => existsSync(resolve(ROOT, dir, "node_modules"));
export const bindingOf = (file: string): Pkg => {
  let d = file;
  while (d.includes("/")) { d = d.slice(0, d.lastIndexOf("/")); const p = dirs.get(d); if (p && bound(d)) return p; }
  return dirs.get(".")!;
};
export const declares = (m: Pkg, name: string): boolean => name in m.deps || name in m.dev || name in m.peer || name in (m.json.optionalDependencies ?? {});
export const sha256 = (text: string): string => createHash("sha256").update(text).digest("hex");
export const read = (rel: string): string => readFileSync(resolve(ROOT, rel), "utf8");

export type ManifestOp = { id: string; packageJson: string; package: string; field: "exports" | "dependencies" | "devDependencies"; op: "add" | "remove"; key: string; value?: string; why: string; wave: string };
export type NewManifest = { id: string; path: string; content: string; why: string; wave: string };
export type Rewrite = { id: string; file: string; from: string; to: string; why: string; wave: string };
export type FileOp = { id: string; op: "create" | "delete" | "move"; path: string; to?: string; content?: string; sha256?: string; why: string; wave: string };
export type Delta = { schema: "cycle-break/delta@1"; stagedAt: string; note: string; manifestOps: ManifestOp[]; newManifests: NewManifest[]; rewrites: Rewrite[]; fileOps: FileOp[] };

/** Roots whose `workspaces` list contains `anchorDir` also list `newDir` (relative to each root, same list order convention). */
export function workspaceOps(newDir: string, anchorDir: string): { packageJson: string; package: string; key: string }[] {
  const out: { packageJson: string; package: string; key: string }[] = [];
  for (const p of model.all) {
    const ws = p.json.workspaces;
    if (!Array.isArray(ws)) continue;
    const dirs = new Set(ws.filter((e: unknown): e is string => typeof e === "string").map((e: string) => posix(resolve(ROOT, p.dir, e)).slice(posix(ROOT).length + 1)));
    if (!dirs.has(anchorDir)) continue;
    const rel = posix(relative(resolve(ROOT, p.dir), resolve(ROOT, newDir)));
    out.push({ packageJson: p.packageJson, package: p.name, key: rel === "" ? "." : rel });
  }
  return out;
}

/** Files reachable from the root `📜️script.ts` through relative imports: the zero-touch CLI closure that must stay relative-only. */
export function cliClosure(edges: { file: string; target: string }[]): Set<string> {
  const adj = new Map<string, string[]>();
  for (const e of edges) (adj.get(e.file) ?? adj.set(e.file, []).get(e.file)!).push(e.target);
  const seen = new Set<string>(); const stack = ["📜️script.ts"];
  while (stack.length) { const f = stack.pop()!; if (seen.has(f)) continue; seen.add(f); for (const t of adj.get(f) ?? []) stack.push(t); }
  return seen;
}
