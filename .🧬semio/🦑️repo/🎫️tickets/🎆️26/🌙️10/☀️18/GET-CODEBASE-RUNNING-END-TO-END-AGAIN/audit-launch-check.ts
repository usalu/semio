import { readFileSync, existsSync } from "node:fs";
import { dirname, join, basename } from "node:path";
import { spawnSync } from "node:child_process";

const ROOT = process.cwd();
const gitList = (args: string[]) => spawnSync("git", args, { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const files = [...new Set([...gitList(["ls-files", "-z"]), ...gitList(["ls-files", "-z", "--others", "--exclude-standard"])])].filter((f) => !f.includes("node_modules/") && !f.includes("/target/") && existsSync(join(ROOT, f)));
const isProjectJson = (f: string) => { const b = basename(f); return b === "project.json" || b === "📋️project.json"; };
const inTickets = (f: string) => f.startsWith(".🧬semio/");
const readJson = (f: string) => { try { return JSON.parse(readFileSync(join(ROOT, f), "utf8")); } catch { return null; } };

type Proj = { name: string; root: string; targets: Set<string>; declared: boolean; source: string[] };
const projects = new Map<string, Proj>();
const byRoot = new Map<string, Proj>();
const upsert = (name: string, root: string, source: string) => {
  let p = projects.get(name);
  if (!p) { p = { name, root, targets: new Set(), declared: false, source: [] }; projects.set(name, p); }
  p.source.push(source);
  if (!byRoot.has(root)) byRoot.set(root, p);
  return p;
};

const ticketProjects = new Map<string, string[]>();
for (const f of files.filter(isProjectJson)) {
  const json = readJson(f);
  if (!json?.name) continue;
  if (inTickets(f)) { ticketProjects.set(json.name, [...(ticketProjects.get(json.name) ?? []), f]); continue; }
  const root = dirname(f) === "." ? "." : dirname(f);
  const p = upsert(json.name, root, "project.json");
  p.declared = true;
  for (const t of Object.keys(json.targets ?? {})) p.targets.add(t);
}
for (const f of files.filter((x) => basename(x) === "package.json" && !inTickets(x))) {
  const json = readJson(f);
  if (!json?.name) continue;
  const root = dirname(f) === "." ? "." : dirname(f);
  if (byRoot.has(root) && byRoot.get(root)!.declared) { for (const s of Object.keys(json.scripts ?? {})) byRoot.get(root)!.targets.add(s); continue; }
  const p = upsert(json.name, root, "package.json");
  for (const s of Object.keys(json.scripts ?? {})) p.targets.add(s);
}
const cargoPlaygrounds: any[] = [];
for (const f of files.filter((x) => basename(x) === "Cargo.toml" && !inTickets(x))) {
  let toml: any;
  try { toml = Bun.TOML.parse(readFileSync(join(ROOT, f), "utf8")); } catch { continue; }
  if (!toml.package) continue;
  const root = dirname(f) === "." ? "." : dirname(f);
  const p = byRoot.get(root) ?? upsert(toml.package.name, root, "Cargo.toml");
  for (const t of ["build", "check", "test"]) p.targets.add(t);
  if (toml.bin || existsSync(join(ROOT, root, "src/main.rs"))) p.targets.add("run");
  const meta = toml.package.metadata;
  if (meta?.component?.package && ["plugin", "extension"].includes(meta.semio?.["component-kind"])) {
    p.targets.add("component");
    for (const row of meta.semio.playground ?? []) cargoPlaygrounds.push({ ...row, pluginId: meta.component.package.slice(6), cratePath: root, project: p.name });
    for (const row of meta.semio.playground ?? []) p.targets.add(`build-${row.variant}-site`);
    if ((meta.semio.playground ?? []).length) p.targets.add("build");
  }
}
const osdev = projects.get("@semio-tech/framework-os-dev");
if (osdev) for (const pg of cargoPlaygrounds) {
  const v = pg.variant;
  for (const profile of ["prototype", "release"]) {
    for (const engine of ["react", "wgpu"]) for (const op of ["prepare", "activate", "serve", "dev"]) osdev.targets.add(`${op}-${v}-${engine}-${profile}`);
    for (const op of ["prepare", "run", "smoke"]) osdev.targets.add(`${op}-${v}-native-${profile}`);
    if (pg.mcpHost) for (const t of ["stdio", "http"]) osdev.targets.add(`mcp-${v}-${t}-${profile}`);
  }
  osdev.targets.add(`build-${v}-react-release`); osdev.targets.add(`build-${v}-react`);
  for (const engine of ["react", "wgpu"]) osdev.targets.add(`prepare-${v}-${engine}`);
}
if (osdev && cargoPlaygrounds.length) osdev.targets.add("build-all-playground-cdn-sites");
const reg = projects.get("@semio-tech/plugin-registry");
if (reg) for (const pg of cargoPlaygrounds) reg.targets.add(`session-${pg.variant}`);
const rootProject = projects.get("workspace");
if (rootProject && existsSync(join(ROOT, "📜️script.ts"))) {
  const src = readFileSync(join(ROOT, "📜️script.ts"), "utf8");
  for (const m of src.matchAll(/\.register\(\s*["'`]([^"'`]+)["'`]/g)) if (!["nx", "repo"].includes(m[1])) rootProject.targets.add(m[1]);
}

const inputs = (file: string) => {
  const j = readJson(file);
  return new Map<string, any>((j?.inputs ?? []).map((i: any) => [i.id, i]));
};

type Route = { file: string; config: string; kind: string; project: string; target: string | null; raw: string };
const extractRoutes = (file: string, cfgs: any[]) => {
  const out: Route[] = [];
  for (const c of cfgs) {
    const cmd: string = typeof c.command === "string" ? c.command : "";
    const name: string = c.name ?? "(unnamed)";
    for (const m of cmd.matchAll(/\bbun repo(?:\s+--tool=\S+)?\s+(run-many|run|exec)\b([^\n]*)/g)) {
      if (m[1] === "run-many") { const toks = m[2].trim().split(/\s+/).map((t) => t.replace(/^["']|["']$/g, "")); const pr = toks.find((t) => t.startsWith("--projects=") || t.startsWith("-p=")) ?? (toks.indexOf("-p") >= 0 ? toks[toks.indexOf("-p") + 1] : undefined) ?? (toks.indexOf("--projects") >= 0 ? toks[toks.indexOf("--projects") + 1] : undefined); const tg = toks.find((t) => t.startsWith("--target=")) ?? (toks.indexOf("-t") >= 0 ? "--target=" + toks[toks.indexOf("-t") + 1] : undefined) ?? (toks.indexOf("--target") >= 0 ? "--target=" + toks[toks.indexOf("--target") + 1] : undefined); const target = tg ? tg.slice("--target=".length) : null; const projs = (pr ?? "").replace(/^--projects=/, "").replace(/^-p=/, "").split(",").filter(Boolean); if (!projs.length) out.push({ file, config: name, kind: "run-many-noprojects", project: "", target, raw: m[0].slice(0, 160) }); for (const p of projs) { if (p.includes("*")) { out.push({ file, config: name, kind: "run-many-glob", project: p, target, raw: m[0].slice(0, 160) }); continue; } out.push({ file, config: name, kind: "run-many", project: p, target, raw: m[0].slice(0, 160) }); } continue; }
      const rest = m[2].trim().split(/\s+/).map((t) => t.replace(/^["']|["']$/g, ""));
      if (m[1] === "exec") {
        const pr = rest.find((t) => t.startsWith("--projects="));
        if (pr) for (const p of pr.slice("--projects=".length).split(",")) out.push({ file, config: name, kind: "exec", project: p, target: null, raw: m[0].slice(0, 160) });
        else out.push({ file, config: name, kind: "exec-noprojects", project: "", target: null, raw: m[0].slice(0, 160) });
        continue;
      }
      const ref = rest.find((t) => t && !t.startsWith("-"));
      if (!ref) { out.push({ file, config: name, kind: "run-noref", project: "", target: null, raw: m[0].slice(0, 160) }); continue; }
      const i = ref.lastIndexOf(":");
      if (i < 0) { out.push({ file, config: name, kind: "run-noTarget", project: ref, target: null, raw: m[0].slice(0, 160) }); continue; }
      out.push({ file, config: name, kind: "run", project: ref.slice(0, i), target: ref.slice(i + 1), raw: m[0].slice(0, 160) });
    }
    for (const m of cmd.matchAll(/\bnx(?:\.js)?"?\s+(run|exec)\s+(\S+)/g)) {
      const ref = m[2].replace(/^["']|["']$/g, "");
      if (m[1] === "run") { const i = ref.lastIndexOf(":"); if (i > 0) out.push({ file, config: name, kind: "nx-run", project: ref.slice(0, i), target: ref.slice(i + 1), raw: m[0].slice(0, 160) }); }
    }
    for (const m of cmd.matchAll(/\bnx(?:\.js)?"?\s+exec\b[^\n]*?--projects=(\S+)/g)) out.push({ file, config: name, kind: "nx-exec", project: m[1].replace(/"/g, ""), target: null, raw: m[0].slice(0, 160) });
  }
  return out;
};

const expand = (s: string | null, ins: Map<string, any>): { values: string[]; dynamic: boolean } => {
  if (s === null) return { values: [], dynamic: false };
  const m = s.match(/\$\{input:([^}]+)\}/);
  if (!m) return { values: [s], dynamic: false };
  const inp = ins.get(m[1]);
  if (!inp || !Array.isArray(inp.options)) return { values: [], dynamic: true };
  return { values: inp.options.map((o: string) => s.replace(m[0], o)), dynamic: false };
};

const resolveRoute = (r: Route, ins: Map<string, any>) => {
  const pj = expand(r.project, ins);
  const tj = expand(r.target, ins);
  if (pj.dynamic || tj.dynamic) return [{ ...r, status: "dynamic-unexpanded" }];
  const outs: any[] = [];
  for (const p0 of pj.values) for (const t0 of (tj.values.length ? tj.values : [null])) {
    const p = projects.get(p0);
    const status = !p && ticketProjects.has(p0) ? "ticket-project-only" : !p ? "project-missing" : t0 === null ? "ok-noTarget" : p.targets.has(t0) ? "ok" : "target-missing";
    outs.push({ ...r, project: p0, target: t0, status, resolvedRoot: p?.root ?? null, inferredOnly: p ? !p.declared : null });
  }
  return outs;
};

const report: any = { projects: projects.size, ticketProjects: ticketProjects.size, files: {} };
for (const file of [".vscode/launch.json", ".vscode/🧩️launch.seed.jsonc"]) {
  const j = readJson(file);
  const ins = inputs(file);
  const cfgs = (j.configurations ?? []).filter((c: any) => c && typeof c === "object");
  const routes = extractRoutes(file, cfgs).flatMap((r) => resolveRoute(r, ins));
  const counts: Record<string, number> = {};
  for (const r of routes) counts[r.status] = (counts[r.status] ?? 0) + 1;
  const kinds: Record<string, number> = {};
  for (const r of routes) kinds[r.kind] = (kinds[r.kind] ?? 0) + 1;
  const bad = routes.filter((r) => r.status !== "ok" && r.status !== "ok-noTarget");
  const groups = new Map<string, any>();
  for (const r of bad) {
    const k = `${r.status} ${r.project}${r.target ? ":" + r.target : ""}`;
    const g = groups.get(k) ?? { key: k, status: r.status, kind: r.kind, count: 0, configs: [] as string[], raw: r.raw };
    g.count++; if (g.configs.length < 3) g.configs.push(r.config);
    groups.set(k, g);
  }
  const compoundsMissing = (j.compounds ?? []).flatMap((c: any) => (c.configurations ?? []).filter((n: string) => !cfgs.some((x: any) => x.name === n)).map((n: string) => ({ compound: c.name, missing: n })));
  const names = new Map<string, number>(); for (const c of cfgs) names.set(c.name, (names.get(c.name) ?? 0) + 1);
  const dups = [...names].filter(([, n]) => n > 1).map(([n, k]) => ({ name: n, count: k }));
  const dupIdentical = dups.map((d) => ({ ...d, identical: new Set(cfgs.filter((c: any) => c.name === d.name).map((c: any) => JSON.stringify(c))).size === 1 }));
  report.files[file] = { configurations: cfgs.length, routes: routes.length, kinds, counts, unresolvedDistinct: groups.size, unresolved: [...groups.values()].sort((a, b) => a.status.localeCompare(b.status) || b.count - a.count), compoundsMissing, duplicateNames: dupIdentical, inputs: ins.size };
}
const allPkgNamesInTickets = [...ticketProjects.keys()].slice(0, 50);
report.ticketProjectNames = allPkgNamesInTickets;
report.runTargetsOf = { "@semio-tech/repo-cli-rs": [...(projects.get("@semio-tech/repo-cli-rs")?.targets ?? [])] };
console.log(JSON.stringify(report, null, 1));
