import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { execSync } from "node:child_process";
const root = "/Users/ueli/Documents/semio";
const C = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const cargos = readFileSync(C + "/🗑️generated/law4-out/plugin-cargos.txt", "utf8").split("\n").filter(Boolean);
export const libOwner = new Map<string, string[]>();
for (const c of cargos) {
  const f = join(root, c); if (!existsSync(f)) continue;
  for (const m of readFileSync(f, "utf8").matchAll(/^\s*path\s*=\s*"([^"]*\.rs)"/gmu)) { const t = resolve(dirname(f), m[1]!); (libOwner.get(t) ?? libOwner.set(t, []).get(t)!).push(c); }
}
const rsList = execSync(`git ls-files '✏️s/🔌️plugins' | grep '🦀️.rs$' || true`, { cwd: root, maxBuffer: 1 << 28, shell: "/bin/bash" }).toString().split("\n").filter(Boolean);
export const parents = new Map<string, string[]>();
for (const r of rsList) {
  const f = join(root, r); if (!existsSync(f)) continue;
  const s = readFileSync(f, "utf8");
  for (const m of s.matchAll(/#\[\s*path\s*=\s*"([^"]+)"\s*\]/gu)) { const t = resolve(dirname(f), m[1]!); (parents.get(t) ?? parents.set(t, []).get(t)!).push(f); }
}
export const crateOf = (file: string, seen = new Set<string>()): string[] => {
  if (libOwner.has(file)) return libOwner.get(file)!;
  if (seen.has(file)) return [];
  seen.add(file);
  const out = new Set<string>();
  for (const p of parents.get(file) ?? []) for (const c of crateOf(p, seen)) out.add(c);
  return [...out];
};
if (import.meta.main) for (const a of process.argv.slice(2)) console.log(a.slice(-80), crateOf(join(root, a)));
