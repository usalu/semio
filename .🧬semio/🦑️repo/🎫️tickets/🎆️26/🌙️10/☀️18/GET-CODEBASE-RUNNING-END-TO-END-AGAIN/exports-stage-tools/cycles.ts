/**
 * 🔁 Workspace dependency graph (declared workspace edges) and cycle checks for export adoption.
 */
import { model } from "./model.ts";
import { loadPolicy, ownerOfFile, type Policy } from "./rules.ts";

export const graph = new Map<string, Set<string>>();
for (const p of model.all) {
  const set = graph.get(p.name) ?? graph.set(p.name, new Set()).get(p.name)!;
  for (const f of ["dependencies", "peerDependencies", "optionalDependencies"] as const) for (const [k, v] of Object.entries<string>(p.json[f] ?? {})) if (String(v).startsWith("workspace:") && model.byName.has(k)) set.add(k);
}
export const reaches = (from: string, to: string): boolean => {
  const seen = new Set<string>(); const stack = [from];
  while (stack.length) { const n = stack.pop()!; if (n === to) return true; if (seen.has(n)) continue; seen.add(n); for (const m of graph.get(n) ?? []) stack.push(m); }
  return false;
};
let policy: Policy | null = null;
const ownerPkgCache = new Map<string, string | null>();
export function packageOfFile(file: string): string | null {
  if (ownerPkgCache.has(file)) return ownerPkgCache.get(file)!;
  policy ??= loadPolicy();
  const sel = ownerOfFile(policy, file);
  const pk = sel ? sel.names[0]! : null;
  ownerPkgCache.set(file, pk);
  return pk;
}
