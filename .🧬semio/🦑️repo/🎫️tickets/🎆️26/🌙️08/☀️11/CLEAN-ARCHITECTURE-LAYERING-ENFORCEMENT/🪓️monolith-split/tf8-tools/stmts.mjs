import { load } from "./graph.mjs";
const [file] = process.argv.slice(2);
const { tops } = load(file);
for (const t of tops) {
  if (t.isImport) continue;
  const deps = [...t.deps].map(([d, v]) => `${d.names[0] ?? d.kind}${v.typeOnly ? "(t)" : v.eager ? "(E)" : ""}`).join(",");
  console.log(`${t.codeLine}-${t.endLine}\t${t.region}\t${t.names.join("+") || t.kind}\t${deps}`);
}
