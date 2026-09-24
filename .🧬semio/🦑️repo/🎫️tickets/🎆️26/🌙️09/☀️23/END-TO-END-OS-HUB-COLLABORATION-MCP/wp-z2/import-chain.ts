/** Z2 probe: shortest static import chain from an entry module to a target module (Bun transpiler scan). */
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, resolve } from "node:path";
const [entry, target] = process.argv.slice(2).map((p) => resolve(p));
const transpiler = new Bun.Transpiler({ loader: "tsx" });
const parent = new Map<string, string | null>([[entry, null]]);
const queue = [entry];
while (queue.length) {
  const file = queue.shift()!;
  if (file === target) break;
  let imports: { path: string }[] = [];
  try { imports = transpiler.scan(readFileSync(file, "utf8")).imports; } catch { continue; }
  for (const { path: raw } of imports) {
    const path = Buffer.from(raw, "latin1").toString("utf8");
    let next: string;
    try { next = realpathSync(Bun.resolveSync(path, dirname(file))); } catch { continue; }
    if (next.includes("/node_modules/") || !existsSync(next) || parent.has(next)) continue;
    parent.set(next, file);
    queue.push(next);
  }
}
if (!parent.has(target)) { console.log("no chain"); process.exit(0); }
const chain: string[] = [];
for (let at: string | null = target; at; at = parent.get(at) ?? null) chain.unshift(at.replace(process.cwd() + "/", ""));
console.log(chain.join("\n  -> "));
