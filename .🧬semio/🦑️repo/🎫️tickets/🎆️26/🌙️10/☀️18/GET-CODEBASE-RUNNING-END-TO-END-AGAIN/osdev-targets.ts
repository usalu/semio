import { readFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";

const ROOT = process.cwd();
const run = (args: string[]) => spawnSync("git", args, { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 }).stdout.split("\0").filter(Boolean);
const list = (pattern: string) => [...new Set([...run(["ls-files", "-z", "--", pattern]), ...run(["ls-files", "-z", "--others", "--exclude-standard", "--", pattern])])]
  .filter((p) => !p.includes(".🧬semio") && !p.includes("node_modules") && existsSync(join(ROOT, p)));

type Target = { dependsOn?: unknown[]; options?: { command?: string }; [k: string]: unknown };
type Project = { name: string; root: string; targets: Record<string, Target>; declared: boolean };

const projects = new Map<string, Project>();
const byRoot = new Map<string, Project>();
const add = (p: Project) => { if (projects.has(p.name)) { const q = projects.get(p.name)!; Object.assign(q.targets, p.targets); return q; } projects.set(p.name, p); byRoot.set(p.root, p); return p; };

for (const file of list("*project.json")) {
  if (!file.endsWith("📋️project.json") && !file.endsWith("/project.json") && file !== "project.json") continue;
  let json: any;
  try { json = JSON.parse(readFileSync(join(ROOT, file), "utf8")); } catch { continue; }
  if (!json.name) continue;
  const root = dirname(file) === "." ? "." : dirname(file);
  add({ name: json.name, root, targets: { ...(json.targets ?? {}) }, declared: true });
}

const components: any[] = [];
const playgrounds: any[] = [];
for (const file of list("*Cargo.toml")) {
  let toml: any;
  try { toml = Bun.TOML.parse(readFileSync(join(ROOT, file), "utf8")); } catch { continue; }
  if (!toml.package) continue;
  const root = dirname(file) === "." ? "." : dirname(file);
  let project = byRoot.get(root);
  if (!project) project = add({ name: toml.package.name, root, targets: {}, declared: false });
  for (const t of ["build", "check", "test"]) project.targets[t] ??= { inferred: "cargo" };
  const meta = toml.package.metadata;
  if (meta?.component?.package && ["plugin", "extension"].includes(meta.semio?.["component-kind"])) {
    project.targets.component ??= { inferred: "component" };
    components.push({ project: project.name, root, meta });
    for (const row of meta.semio.playground ?? []) playgrounds.push({ ...row, pluginId: meta.component.package.slice(6), cratePath: root, project: project.name });
    for (const row of meta.semio.playground ?? []) { project.targets[`build-${row.variant}-site`] = { inferred: "site" }; }
    if ((meta.semio.playground ?? []).length) project.targets.build ??= { inferred: "site" };
  }
}

const osdev = projects.get("@semio-tech/framework-os-dev")!;
for (const pg of playgrounds) {
  const v = pg.variant;
  for (const profile of ["prototype", "release"]) {
    for (const engine of ["react", "wgpu"]) for (const op of ["prepare", "activate", "serve", "dev"]) osdev.targets[`${op}-${v}-${engine}-${profile}`] = { inferred: "playground" };
    for (const op of ["prepare", "run", "smoke"]) osdev.targets[`${op}-${v}-native-${profile}`] = { inferred: "playground" };
    if (pg.mcpHost) for (const t of ["stdio", "http"]) osdev.targets[`mcp-${v}-${t}-${profile}`] = { inferred: "playground" };
  }
  osdev.targets[`build-${v}-react-release`] = { inferred: "playground" };
  osdev.targets[`build-${v}-react`] = { inferred: "playground" };
  for (const engine of ["react", "wgpu"]) osdev.targets[`prepare-${v}-${engine}`] = { inferred: "playground" };
  const reg = projects.get("@semio-tech/plugin-registry");
  if (reg) reg.targets[`session-${v}`] = { inferred: "session" };
}
if (playgrounds.length) osdev.targets["build-all-playground-cdn-sites"] = { inferred: "playground" };

const rootProject = projects.get("workspace");
if (rootProject && existsSync(join(ROOT, "📜️script.ts"))) {
  const src = readFileSync(join(ROOT, "📜️script.ts"), "utf8");
  for (const m of src.matchAll(/\.register\(\s*["'`]([^"'`]+)["'`]/g)) if (!["nx", "repo"].includes(m[1])) rootProject.targets[m[1]] ??= { inferred: "root-command" };
}

export { projects, playgrounds, components };

const declaredDeps = () => {
  const out: { from: string; target: string; dep: string; missing: string }[] = [];
  for (const [name, project] of projects) {
    if (!project.declared) continue;
    for (const [tname, t] of Object.entries(project.targets)) {
      for (const d of (t.dependsOn ?? []) as any[]) {
        const refs: [string, string][] = [];
        if (typeof d === "string") {
          const s = d.startsWith("^") ? d.slice(1) : d;
          if (d.startsWith("^")) continue;
          const i = s.lastIndexOf(":");
          if (i > 0 && !s.startsWith(":")) refs.push([s.slice(0, i), s.slice(i + 1)]); else refs.push([name, s]);
        } else if (d && typeof d === "object") {
          const target = d.target;
          if (typeof target !== "string") continue;
          if (Array.isArray(d.projects)) for (const p of d.projects) refs.push([p, target]);
          else if (typeof d.projects === "string") refs.push([d.projects, target]);
          else if (d.dependencies) continue;
          else refs.push([name, target]);
        }
        for (const [p, t2] of refs) {
          const q = projects.get(p);
          if (!q) out.push({ from: name, target: tname, dep: p + ":" + t2, missing: "project" });
          else if (!(t2 in q.targets)) out.push({ from: name, target: tname, dep: p + ":" + t2, missing: "target" });
        }
      }
    }
  }
  return out;
};

const launchRefs = (file: string) => {
  const text = readFileSync(join(ROOT, file), "utf8");
  const out: { line: number; ref: string; missing: string }[] = [];
  const lines = text.split("\n");
  lines.forEach((ln, i) => {
    const m = ln.match(/"command": "bun repo (?:--tool=\w+ )?run ([^ "]+)/);
    if (!m) return;
    const ref = m[1];
    const idx = ref.lastIndexOf(":");
    if (idx < 0) return;
    const p = ref.slice(0, idx), t = ref.slice(idx + 1);
    const q = projects.get(p);
    if (!q) out.push({ line: i + 1, ref, missing: "project" });
    else if (!(t in q.targets)) out.push({ line: i + 1, ref, missing: "target" });
  });
  return out;
};
export { declaredDeps, launchRefs };

const admission = async () => {
  const { runtimeInputAdmissionV1 } = await import(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs"));
  const rows = components.map((c) => ({ ...c.meta.semio, pluginId: c.meta.component.package.slice(6), dependsOn: [...(c.meta.semio.extends ? [c.meta.semio.extends] : []), ...(c.meta.semio["depends-on"] ?? [])] }));
  const refused: any[] = [];
  const engineErrors: any[] = [];
  for (const pg of playgrounds) {
    const engines = (pg.engines ?? []).map((e: string) => join(e, "📋️project.json").replace(/^\.\//, ""));
    const hosts = [pg.nativeHost, pg.mcpHost].filter(Boolean).flatMap((h: any) => [h.cratePath + "/Cargo.toml", h.cratePath + "/📋️project.json"]);
    const sources = [...engines, ...hosts, ...(pg.devContribution ? [pg.devContribution] : []), ...(pg.compositionConfigPath ? [pg.compositionConfigPath] : [])];
    const a = runtimeInputAdmissionV1(rows, [{ id: pg.pluginId, appScoped: pg.app !== undefined }], sources, (path: string) => existsSync(join(ROOT, path)));
    if (a.status !== "admitted") refused.push({ variant: pg.variant, ...a.missing });
    for (const e of pg.engines ?? []) {
      const root = e.replace(/^\.\//, "").replace(/\/$/, "");
      const pr = byRoot.get(root);
      if (!pr || !pr.targets.wasm) engineErrors.push({ variant: pg.variant, engine: e, project: pr?.name ?? null });
    }
  }
  return { refused, engineErrors };
};
export { admission };

if (import.meta.main) {
  const mode = process.argv[2];
  if (mode === "dump") console.log(JSON.stringify(Object.fromEntries([...projects].map(([n, p]) => [n, { root: p.root, targets: Object.keys(p.targets) }])), null, 0));
  else if (mode === "deps") console.log(JSON.stringify(declaredDeps()));
  else if (mode === "launch") console.log(JSON.stringify({ launch: launchRefs(".vscode/launch.json"), seed: launchRefs(".vscode/🧩️launch.seed.jsonc") }));
  else if (mode === "admission") console.log(JSON.stringify(await admission()));
  else if (mode === "variants") console.log(JSON.stringify(playgrounds.map((p) => p.variant).sort()));
}
