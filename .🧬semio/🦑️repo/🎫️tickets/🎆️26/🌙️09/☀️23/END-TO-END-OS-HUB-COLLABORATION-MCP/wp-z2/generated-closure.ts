/** Z2 probe: every gitignored `🤖️generated` module in the static import closure of the given entries (Bun scan). */
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
const transpiler = new Bun.Transpiler({ loader: "tsx" });
const seen = new Set<string>(), generated = new Map<string, string>();
const queue = process.argv.slice(2).map((entry) => resolve(entry));
while (queue.length) {
  const file = queue.shift()!;
  if (seen.has(file)) continue;
  seen.add(file);
  if (!/\.(m?[jt]sx?)$/u.test(file)) continue;
  let imports: { path: string; kind: string }[] = [];
  try { imports = transpiler.scan(readFileSync(file, "utf8")).imports as any; } catch { continue; }
  for (const { path: raw, kind } of imports) {
    const path = Buffer.from(raw, "latin1").toString("utf8");
    if (kind === "dynamic-import") continue;
    const candidate = path.startsWith(".") ? resolve(dirname(file), path) : null;
    if (candidate && /🤖️generated|🔤️tokens\/🦀️/u.test(candidate) && !generated.has(candidate)) generated.set(candidate, file);
    let next: string;
    try { next = realpathSync(Bun.resolveSync(path, dirname(file))); } catch { continue; }
    if (next.includes("/node_modules/") || !existsSync(next)) continue;
    queue.push(next);
  }
}
for (const [target, from] of generated) console.log(relative(process.cwd(), target), "<=", relative(process.cwd(), from));
console.log("modules scanned", seen.size);
