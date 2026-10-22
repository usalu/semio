import { join } from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const GEN = join(W, ".cache/b/g/m");
const gen = await import(join(LIB, "🏗️build/🕸️graph/🕰️generations/🟨️.mjs"));
const engine = gen.createGenerationStoreV1({ root: join(GEN, "generations"), holds: async () => true, resources: { facts: "bench" } });
const current = await engine.read();
const assembled = await engine.assemble(current, { compare: (a, b) => a.localeCompare(b), progress: async () => {} });
const sum = {}, add = (k, v) => { sum[k] = (sum[k] ?? 0) + JSON.stringify(v ?? null).length; };
let n = 0, tt = 0;
const targetKeys = {};
for (const node of Object.values(assembled.projects)) {
  n++;
  for (const [k, v] of Object.entries(node)) add("node." + k, v);
  for (const [name, t] of Object.entries(node.targets ?? {})) { tt++; for (const [k, v] of Object.entries(t)) { targetKeys[k] = (targetKeys[k] ?? 0) + JSON.stringify(v ?? null).length; } }
  for (const [k, v] of Object.entries(node.metadata?.semio ?? {})) add("meta.semio." + k, v);
}
console.log(JSON.stringify({ projects: n, targets: tt, node: sum, targetKeys }, null, 1));
