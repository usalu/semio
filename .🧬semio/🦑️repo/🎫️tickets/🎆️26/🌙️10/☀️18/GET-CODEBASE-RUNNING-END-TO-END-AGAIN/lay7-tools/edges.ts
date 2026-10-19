import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/", REPO = "🧰️framework/🛍️products/🦑️repo/", rend = OS + "🔨️modules/📺️renderer/";
const out: any[] = [];
for (const m of r.modules) {
  if (!m.source.startsWith(OS) || m.source.startsWith(rend)) continue;
  for (const d of m.dependencies) if (d.resolved.startsWith(REPO)) out.push({ from: m.source, spec: d.module, to: d.resolved, kind: d.dependencyTypes, tc: d.typeOnly ?? false });
}
await Bun.write(process.argv[3]!, JSON.stringify(out, null, 1));
console.log(out.length);
