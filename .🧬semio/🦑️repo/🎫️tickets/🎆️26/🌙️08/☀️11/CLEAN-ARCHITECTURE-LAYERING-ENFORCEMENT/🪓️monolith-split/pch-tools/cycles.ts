// 🔁️ Read-only: for every shared plugin test-support file, would turning it into a dev-dependency crate create a package cycle with its consumers? (Reachability over the artifact lock graph from the packages the file names.)
import {readFileSync, existsSync} from "node:fs";
import {dirname, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const tsv = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map(l => l.split("\t"));
const lockCache = new Map<string, Map<string, string[]>>();
const nearestWs = (path: string): string => { let d = dirname(path); while (d && d !== ".") { if (existsSync(join(ROOT, d, "Cargo.toml")) && /\[workspace\]/.test(readFileSync(join(ROOT, d, "Cargo.toml"), "utf8"))) return d; d = dirname(d); } throw new Error("no ws " + path); };
const graph = (ws: string) => {
  if (lockCache.has(ws)) return lockCache.get(ws)!;
  const g = new Map<string, string[]>();
  const p = join(ROOT, ws, "Cargo.lock");
  if (existsSync(p)) for (const block of readFileSync(p, "utf8").split(/\n(?=\[\[package\]\]\n)/u)) {
    const name = /name = "([^"]+)"/u.exec(block)?.[1]; if (!name) continue;
    const deps = /\ndependencies = \[([\s\S]*?)\n\]/u.exec(block);
    g.set(name, deps ? [...deps[1]!.matchAll(/"([^"\s]+)(?: [^"]*)?"/gu)].map(m => m[1]!) : []);
  }
  lockCache.set(ws, g);
  return g;
};
const nameOfManifest = (m: string) => (Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any).package?.name as string;
for (const r of tsv) {
  const [file, , pkgs] = r;
  if (!file!.startsWith("✏️s/") || file!.includes("🚦️native")) continue;
  const text = readFileSync(join(ROOT, file!), "utf8");
  const consumers = pkgs!.split(" ; ").map(p => `${p}/Cargo.toml`).filter(m => existsSync(join(ROOT, m)));
  if (!consumers.length) continue;
  const ws = nearestWs(consumers[0]!);
  const g = graph(ws);
  const idents = new Set([...text.matchAll(/(?<![\w:.$])([a-z][a-z0-9_]*)::/g)].map(m => m[1]!).concat([...text.matchAll(/^\s*(?:pub\s+)?use\s+([a-z][a-z0-9_]*)/gm)].map(m => m[1]!)));
  const used = [...idents].map(i => i.replaceAll("_", "-")).filter(n => g.has(n));
  const consumerNames = consumers.map(nameOfManifest);
  const reach = new Set<string>(); const stack = [...used];
  while (stack.length) { const n = stack.pop()!; if (reach.has(n)) continue; reach.add(n); for (const d of g.get(n) ?? []) stack.push(d); }
  const cyc = consumerNames.filter(c => reach.has(c));
  console.log(`${cyc.length ? "CYCLE" : "safe "} ${consumers.length} consumers  ${file!.split("/").slice(-4).join("/")}  uses=${used.length}${cyc.length ? "  reaches " + cyc.slice(0, 3).join(",") : ""}`);
}
