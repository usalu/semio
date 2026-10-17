/** 🔁️ Finds one import cycle through a file (static imports and re-exports only, no dynamic or type-only imports), printing its path. Usage: bun cycle.ts <file>... */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, scanSource } from "../sdkb-tools/lib.ts";
import { nfc } from "./model.ts";
const overlay = new Map<string, string | null>();
for (const snap of (process.env.OVERLAY ?? "").split(",").filter(Boolean)) for (const [rel, text] of Object.entries(JSON.parse(readFileSync(snap, "utf8")) as Record<string, string | null>)) if (!overlay.has(nfc(rel))) overlay.set(nfc(rel), text);
const cache = new Map<string, string[]>();
const deps = (f: string): string[] => {
  if (cache.has(f)) return cache.get(f)!;
  let out: string[] = [];
  try {
    const o = overlay.get(f);
    if (o === null) return (cache.set(f, []), []);
    const text = o ?? readFileSync(join(ROOT, f), "utf8");
    out = scanSource(f, text).filter(r => r.resolved && /\.(ts|tsx)$/u.test(r.resolved)).map(r => nfc(r.resolved!));
  } catch {}
  cache.set(f, out);
  return out;
};
for (const start of process.argv.slice(2).map(nfc)) {
  const seen = new Set<string>(), stack: string[] = [];
  let found: string[] | null = null;
  const dfs = (f: string): void => {
    if (found) return;
    for (const d of deps(f)) {
      if (found) return;
      if (d === start) { found = [...stack, f, d]; return; }
      if (seen.has(d)) continue;
      seen.add(d); stack.push(f); dfs(d); stack.pop();
    }
  };
  seen.add(start); dfs(start);
  console.log("##", start.slice(-80));
  if (found) for (const f of found) console.log("   ", f.replace("🧰️framework/", "F/").slice(-120));
  else console.log("    no static cycle found");
}
