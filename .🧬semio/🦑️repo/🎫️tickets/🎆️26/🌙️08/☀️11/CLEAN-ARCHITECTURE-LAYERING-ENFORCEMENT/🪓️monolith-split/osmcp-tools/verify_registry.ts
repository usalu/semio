// ✅️ Static registry verification for the os-mcp carve: manifests, workspace, owners, launch routes, DAG.
import {readFileSync, existsSync} from "node:fs";
import {resolve, dirname, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const MCP = join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp");
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {nodes: Record<string, string | null>};
let failures = 0;
const fail = (m: string) => { failures++; console.log("FAIL", m); };
const names = Object.entries(plan.nodes);
const root = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const members: string[] = root.workspace.members;
const sorted = [...members].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
if (JSON.stringify(sorted) !== JSON.stringify(members)) fail("workspace members are not byte-sorted");
const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[];
const ownerNames = new Set(owners.map((o) => o.name));
if (ownerNames.size !== owners.length) fail("duplicate owner names");
const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seedText = readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8");
const seed = JSON.parse(seedText.replace(/^\s*\/\/.*$/gm, "")).configurations as any[];
const edges = new Map<string, string[]>();
const pkgOf = new Map<string, string>();
const manifests = new Map<string, any>();
for (const [n, nd] of names) {
  const dir = nd ? join(MCP, nd, "📦️packages/🦀️rust") : join(MCP, "📦️packages/🦀️rust");
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { fail(`${n}: manifest missing`); continue; }
  const m = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  manifests.set(n, m);
  const pkg = nd ? `semio-framework-os-mcp-${n}` : "semio-framework-os-mcp";
  pkgOf.set(n, pkg);
  if (m.package.name !== pkg) fail(`${n}: package name ${m.package.name}`);
  if (resolve(dir, m.package.workspace) !== ROOT) fail(`${n}: workspace does not resolve to the repo root`);
  const b = m.package.metadata?.semio?.["rust-source-boundary"];
  if (!b || Object.keys(b).sort().join() !== "maxActiveLines,maxSourceBytes,maxSourceFiles" || b.maxActiveLines !== 4000 || b.maxSourceFiles !== 32 || b.maxSourceBytes !== 262144) fail(`${n}: boundary keys`);
  if (!existsSync(resolve(dir, m.lib.path))) fail(`${n}: lib path ${m.lib.path} missing`);
  if (!m.lints?.workspace) fail(`${n}: lints.workspace`);
  if (nd) {
    const rel = dir.slice(ROOT.length + 1);
    if (!members.includes(rel)) fail(`${n}: not a workspace member`);
    const project = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8"));
    const owner = owners.find((o) => o.name === project.name);
    if (!owner || owner.root !== rel || owner.manifest !== rel + "/📋️project.json") fail(`${n}: owner row`);
    for (const verb of ["build", "check", "test"]) for (const cfg of ["prototype", "release"]) {
      const cmd = `bun repo run ${project.name}:${verb} --configuration=${cfg} --parallel=1 -- --lib --offline`;
      const a = launch.filter((c) => c.command === cmd).length, s = seed.filter((c) => c.command === cmd).length;
      if (a !== 1 || s !== 1) fail(`${n}: launch route ${verb} ${cfg} launch=${a} seed=${s}`);
    }
  }
  const deps: string[] = [];
  const tables = [m.dependencies, m["dev-dependencies"], ...Object.values(m.target ?? {}).flatMap((t: any) => [t.dependencies])];
  for (const t of tables) for (const [key, spec] of Object.entries(t ?? {}) as [string, any][]) {
    if (typeof spec === "object" && typeof spec.path === "string") {
      const target = resolve(dir, spec.path, "Cargo.toml");
      if (!existsSync(target)) fail(`${n}: dependency ${key} path ${spec.path} missing`);
      else if (key.startsWith("semio-framework-os-mcp")) {
        const tm = Bun.TOML.parse(readFileSync(target, "utf8")) as any;
        if (tm.package.name !== key) fail(`${n}: dependency ${key} resolves to ${tm.package.name}`);
        deps.push(key);
        if (spec.features?.includes("testkit") && !(tm.features && "testkit" in tm.features)) fail(`${n}: testkit requested from ${key} which declares none`);
      }
    }
  }
  edges.set(pkg, deps);
}
// cycle check
const state = new Map<string, number>();
const visit = (p: string, trail: string[]): void => {
  if (state.get(p) === 1) { fail("cycle " + [...trail, p].join(" -> ")); return; }
  if (state.get(p) === 2) return;
  state.set(p, 1);
  for (const d of edges.get(p) ?? []) visit(d, [...trail, p]);
  state.set(p, 2);
};
for (const p of edges.keys()) visit(p, []);
const dup = (a: any[]) => { const s = new Set(); for (const c of a) { if (s.has(c.name)) return c.name; s.add(c.name); } return null; };
console.log("crates:", names.length, "owners:", owners.length, "launch configs:", launch.length, "seed configs:", seed.length, "dup names launch:", dup(launch.filter((c) => (c.name ?? "").startsWith("🦀️ semio-framework-os-mcp"))), "seed:", dup(seed.filter((c) => (c.name ?? "").startsWith("🦀️ semio-framework-os-mcp"))));
console.log(failures === 0 ? "ALL REGISTRY CHECKS PASSED" : `${failures} FAILURES`);
