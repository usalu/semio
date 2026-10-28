// [farmid] which verb kernels reach a module through literal imports (static + dynamic), the sites the snapshot capture follows. usage: bun reach.ts <root> <target-suffix>
import { readFileSync, existsSync } from "node:fs";
import { dirname, resolve } from "node:path";
const [root, target] = process.argv.slice(2), K = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/";
const kernels = { preflight: "🩺️preflight/🟦️.ts", dependencies: "📦️dependencies/🟦️.ts", cache: "💾️cache/🟦️.ts", ledger: "💾️cache/📋️ledger/🟦️.ts", runtime: "⚙️runtime/🟦️.ts" };
for (const [kind, rel] of Object.entries(kernels)) {
  const seen = new Set<string>(), stack = [K + rel]; let hit: string | undefined;
  while (stack.length) {
    const f = stack.pop()!; if (seen.has(f)) continue; seen.add(f); if (f.endsWith(target)) { hit = f; break; }
    if (!/\.[cm]?[jt]sx?$/.test(f)) continue;
    let src: string; try { src = readFileSync(f, "utf8"); } catch { continue; }
    const t = new Bun.Transpiler({ loader: /\.[cm]?tsx?$/.test(f) ? "ts" : "js" });
    let imps: { path: string }[] = []; try { imps = t.scan(src).imports; } catch {}
    for (const i of imps) if (i.path.startsWith(".")) { const p = resolve(dirname(f), i.path); if (existsSync(p)) stack.push(p); }
  }
  console.log(kind, seen.size, hit ? "REACHES" : "-");
}
