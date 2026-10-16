// 🧵️ Read-only: every carved crate of an artifact must be reachable from the artifact's facade crate through the artifact lock graph (a carved crate nobody depends on is a half-wired carve).
import {readFileSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
let bad = 0;
for (const artifact of readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean)) {
  const facade = (Bun.TOML.parse(readFileSync(join(ROOT, artifact, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as any).package.name as string;
  const ws = Bun.TOML.parse(readFileSync(join(ROOT, artifact, "Cargo.toml"), "utf8")) as any;
  const carved: string[] = [];
  for (const m of ws.workspace.members as string[]) {
    const name = (Bun.TOML.parse(readFileSync(join(ROOT, artifact, m, "Cargo.toml"), "utf8")) as any).package?.name as string;
    if (name && name !== facade && name.startsWith(facade + "-")) carved.push(name);
  }
  const g = new Map<string, string[]>();
  for (const block of readFileSync(join(ROOT, artifact, "Cargo.lock"), "utf8").split(/\n(?=\[\[package\]\]\n)/u)) {
    const n = /name = "([^"]+)"/u.exec(block)?.[1]; if (!n) continue;
    const d = /\ndependencies = \[([\s\S]*?)\n\]/u.exec(block);
    g.set(n, d ? [...d[1]!.matchAll(/"([^"\s]+)/gu)].map(m => m[1]!) : []);
  }
  const reach = new Set<string>(); const st = [facade];
  while (st.length) { const n = st.pop()!; if (reach.has(n)) continue; reach.add(n); for (const x of g.get(n) ?? []) st.push(x); }
  const unreached = carved.filter(c => !reach.has(c));
  const dependsDirectly = carved.filter(c => (g.get(facade) ?? []).includes(c)).length;
  if (unreached.length) bad++;
  console.log(`${artifact.split("/").slice(-1)[0]}\tcarved=${carved.length}\tfacadeDirect=${dependsDirectly}\tunreached=${unreached.length}${unreached.length ? " " + unreached.join(",") : ""}`);
}
console.log("artifacts with unreached carved crates:", bad);
