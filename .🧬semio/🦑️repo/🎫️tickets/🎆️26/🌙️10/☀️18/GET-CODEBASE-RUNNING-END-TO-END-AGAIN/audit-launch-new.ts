import { readFileSync, existsSync } from "node:fs";
import { dirname, basename } from "node:path";
const launchText = readFileSync(".vscode/launch.json", "utf8") + readFileSync(".vscode/🧩️launch.seed.jsonc", "utf8");
const lines = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
const roots = new Map<string, { cargo?: string; project?: string; pkg?: string }>();
for (const f of lines) {
  const d = dirname(f);
  const r = roots.get(d) ?? {};
  if (basename(f) === "Cargo.toml") r.cargo = f;
  else r.project = f;
  roots.set(d, r);
}
const out: any[] = [];
for (const [d, r] of roots) {
  let name: string | null = null;
  let kind = "";
  if (r.cargo) { try { const t = Bun.TOML.parse(readFileSync(r.cargo, "utf8")) as any; name = t.package?.name ?? null; kind = t.package ? "cargo" : "workspace-only"; if (t.package?.metadata?.semio?.["component-kind"]) kind += "+" + t.package.metadata.semio["component-kind"]; if (t.bin) kind += "+bin"; } catch { kind = "cargo-parse-error"; } }
  let pjName: string | null = null;
  if (r.project) { try { pjName = JSON.parse(readFileSync(r.project, "utf8")).name ?? null; } catch { pjName = "PARSE_ERROR"; } }
  const n = pjName ?? name;
  const inLaunch = n ? launchText.includes(n) : false;
  const launchRoute = n ? launchText.split("\n").filter((l) => l.includes(`bun repo`) && (l.includes(`${n}:`) || l.includes(`--projects=${n}`) || l.includes(`${n} `))).length : 0;
  const dupCount = n ? lines.length && 0 : 0;
  out.push({ dir: d, cargoName: name, projectName: pjName, kind, inLaunchText: inLaunch, launchRouteLines: launchRoute, hasBinOrCompanionKind: /bin|companion|plugin|extension/.test(kind) || /companion|bin/.test(d) });
}
console.log(JSON.stringify(out));
