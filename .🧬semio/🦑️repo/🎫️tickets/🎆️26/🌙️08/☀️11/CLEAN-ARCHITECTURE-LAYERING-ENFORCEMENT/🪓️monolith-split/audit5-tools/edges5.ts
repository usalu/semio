// 🧭️ Read-only rank check of path dependencies among new crates against the plans' rank tables (kernel RANK, renderer frozen rank, plugin-host DAG).
import {readFileSync, existsSync} from "node:fs";
import {dirname, join, normalize, relative} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const GEN = process.argv[2]!;
const TOOLS = process.argv[3]!;
const newSet = new Set(readFileSync(join(GEN, "audit5r-new-cargo.txt"), "utf8").split("\n").filter(Boolean).map(p => dirname(p)));
const kernelRank = (readFileSync(join(TOOLS, "../okwp0-tools/okcrates.py"), "utf8").match(/RANK = \[([\s\S]*?)\]\n/) ?? [])[1] ?? "";
const kernelKeys = [...kernelRank.matchAll(/'([^']+)'/g)].map(m => m[1]!);
const kernelIdx = new Map(kernelKeys.map((k, i) => ["semio-framework-" + k, i]));
const frozen = JSON.parse(readFileSync(join(GEN, "rwp0-crates.json"), "utf8"));
const rendererRank = new Map<string, number>();
for (const v of Object.values(frozen) as any[]) rendererRank.set(normalize(join(ROOT, v.pkg)), v.rank);
const pluginDag = [
  ["wit", "operation"], ["operation", "guest-runtime"], ["guest-runtime", "shard"], ["guest-runtime", "post-turn-relay"],
  ["shard", "post-turn-relay"], ["shard", "shard-executor"], ["shard", "routers"], ["post-turn-relay", "shard-executor"], ["post-turn-relay", "routers"],
  ["engine", "guest-runtime"], ["engine", "process-transport"], ["process-transport", "shard"], ["effects", "imports"], ["wake", "post-turn-relay"],
];
const plugBelow = new Set(pluginDag.map(([a, b]) => a + ">" + b));
let changed = true;
while (changed) { changed = false; for (const x of [...plugBelow]) { const [a, b] = x.split(">"); for (const y of [...plugBelow]) { const [c, d] = y.split(">"); if (b === c && !plugBelow.has(a + ">" + d)) { plugBelow.add(a + ">" + d); changed = true; } } } }
const manifests = (await Bun.$`git --no-optional-locks -c core.quotepath=false -c core.fsmonitor=false ls-files -- '*Cargo.toml'`.cwd(ROOT).text()).split("\n").filter(p => p && !p.startsWith(".🧬semio/"));
const pkgOf = new Map<string, {name: string; deps: {name: string; dir: string; dev: boolean}[]}>();
const wsDeps = new Map<string, string>();
const rootToml = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
for (const [k, v] of Object.entries(rootToml.workspace?.dependencies ?? {}) as [string, any][]) if (v.path) wsDeps.set(k, normalize(join(ROOT, v.path)));
for (const m of manifests) {
  const dir = normalize(join(ROOT, dirname(m)));
  const parsed = Bun.TOML.parse(readFileSync(join(ROOT, m), "utf8")) as any;
  if (!parsed.package) continue;
  const deps: {name: string; dir: string; dev: boolean}[] = [];
  const tables: [any, boolean][] = [[parsed.dependencies, false], [parsed["dev-dependencies"], true]];
  for (const [t, dev] of tables) for (const [key, raw] of Object.entries(t ?? {}) as [string, any][]) {
    const spec = typeof raw === "string" ? {} : raw;
    let p = spec.path as string | undefined;
    if (spec.workspace === true) { p = wsDeps.get(spec.package ?? key); if (p) { deps.push({name: spec.package ?? key, dir: p, dev}); continue; } }
    if (p) deps.push({name: spec.package ?? key, dir: normalize(join(dir, p)), dev});
  }
  pkgOf.set(dir, {name: parsed.package.name, deps});
}
const rows: string[] = [];
const viol: string[] = [];
let checked = 0, kernelChecked = 0, rendChecked = 0, plugChecked = 0;
for (const [dir, info] of pkgOf) {
  const rel = relative(ROOT, dir);
  for (const d of info.deps) {
    const target = pkgOf.get(d.dir);
    if (!target) continue;
    const a = info.name, b = target.name;
    let verdict = "", family = "";
    if (kernelIdx.has(a) && kernelIdx.has(b)) {
      family = "kernel"; kernelChecked++;
      verdict = (kernelIdx.get(b)! < kernelIdx.get(a)!) ? "ok" : "VIOLATION";
    } else if (rendererRank.has(dir) && rendererRank.has(d.dir)) {
      family = "renderer"; rendChecked++;
      verdict = (rendererRank.get(d.dir)! < rendererRank.get(dir)!) ? "ok" : "VIOLATION";
    } else if (a.startsWith("semio-framework-plugin-host-") && b.startsWith("semio-framework-plugin-host-") || (a === "semio-framework-plugin-host" && b.startsWith("semio-framework-plugin-host-"))) {
      const sa = a.replace("semio-framework-plugin-host-", "").replace("semio-framework-plugin-host", "facade");
      const sb = b.replace("semio-framework-plugin-host-", "").replace("semio-framework-plugin-host", "facade");
      family = "plugin-host"; plugChecked++;
      if (sb === "facade") verdict = "ok-facade";
      else if (sa === "facade") verdict = "ok-facade-top";
      else if (plugBelow.has(sa + ">" + sb)) verdict = "VIOLATION";
      else if (plugBelow.has(sb + ">" + sa)) verdict = "ok";
      else verdict = "unordered-by-DAG";
    } else continue;
    checked++;
    const row = [family, a, b, rel, d.dev ? "dev" : "normal", verdict].join("\t");
    rows.push(row);
    if (verdict.startsWith("VIOLATION")) viol.push(row);
  }
}
await Bun.write(join(GEN, "audit5r-edges.tsv"), rows.join("\n") + "\n");
console.log("checked", checked, "kernel", kernelChecked, "renderer", rendChecked, "plugin-host", plugChecked, "violations", viol.length, "unordered-plugin", rows.filter(r => r.endsWith("unordered-by-DAG")).length);
console.log("violations:"); for (const v of viol.slice(0, 40)) console.log(v.slice(0, 300));
