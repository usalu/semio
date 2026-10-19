import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const roots = ["✏️s/🧑‍💻dev/", "✏️s/🧪️tests/", "♻️mit-bestand/"];
const mods = new Map<string, any>(r.modules.map((m: any) => [m.source, m]));
const edges = new Map<string, any>();
for (const x of r.summary.violations as any[]) {
  if (!roots.some((p) => x.from.startsWith(p))) continue;
  const dep = mods.get(x.from)?.dependencies.find((d: any) => d.resolved === x.to);
  const k = x.from + "\0" + x.to;
  const e = edges.get(k) ?? { from: x.from, to: x.to, spec: dep?.module, depTypes: dep?.dependencyTypes, rules: [] };
  e.rules.push(x.rule.name); edges.set(k, e);
}
console.log(edges.size);
const out = [...edges.values()];
const short = (s: string) => s.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/", "REPOLIB/").replace("🧰️framework/🛍️products/💻️os/🔨️modules/", "OSMOD/");
for (const e of out) console.log(e.rules.map((x: string) => x.replace("no-cross-package-relative-@semio-tech/", "N:").replace("no-cross-technology-", "T:")).join(","), "|", e.depTypes?.join("/"), "|", e.spec, "|", short(e.from).slice(0, 110), "->", short(e.to));
