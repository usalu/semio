/** 🧾 Registers framework Rust crates lacking canonical build owners: project, owner row and six launch routes. Idempotent. */
import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const dry = process.argv.includes("--dry");
const listFile = process.argv.find((a) => a.endsWith(".json") && !a.startsWith("--"))!;
const rowOnly = new Set(["@semio-tech/framework-io-codec-rs", "@semio-tech/ui-wgpu-render-rs"]);
const REGISTRY = join(root, "📋️build-owners.json");
const LAUNCH = join(root, ".vscode/launch.json");
const SEED = join(root, ".vscode/🧩️launch.seed.jsonc");
const ANCHOR = '    {\n      "name": "🦀️ semio-framework-repo-dashboard-core 🏗️build 🛠️prototype"';
const ENV = '"env": {"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0", "NODE_DISABLE_COMPILE_CACHE": "1"}';

const read = (p: string) => readFileSync(p, "utf8");
const atomic = (p: string, text: string) => { const tmp = p + ".owners-tmp"; writeFileSync(tmp, text); renameSync(tmp, p); };

const targets: string[] = JSON.parse(read(listFile)).missing.filter((m: string) => m.startsWith("🧰️framework/"));
interface Plan { dir: string; crate: string; project: string; create: boolean; routes: boolean }
const plans: Plan[] = [];
for (const dir of targets) {
  const cargo = Bun.TOML.parse(read(join(root, dir, "Cargo.toml"))) as any;
  const crate: string = cargo.package.name, id: string | undefined = cargo.package.metadata?.semio?.id;
  const projectFile = join(root, dir, "📋️project.json");
  const exists = existsSync(projectFile);
  const project: string = exists ? JSON.parse(read(projectFile)).name : `@semio-tech/${id}-rs`;
  if (!exists && !id) throw Error("metadata.semio.id required: " + dir);
  plans.push({ dir, crate, project, create: !exists, routes: !rowOnly.has(project) });
}

const registryText = read(REGISTRY), registry = JSON.parse(registryText);
if (registryText !== JSON.stringify(registry, null, 2) + "\n") throw Error("registry not canonical");
const names = new Set<string>(registry.owners.map((o: any) => o.name)), roots = new Set<string>();
for (const o of registry.owners) for (const r of [o.root, ...(o.nativeRoots ?? [])]) roots.add(r);
const fresh = plans.filter((p) => !roots.has(p.dir));
for (const p of fresh) if (names.has(p.project)) throw Error("owner name collision: " + p.project);
const dup = new Set<string>();
for (const p of fresh) { if (dup.has(p.project)) throw Error("duplicate plan name " + p.project); dup.add(p.project); }

const launchText = read(LAUNCH), seedText = read(SEED);
const used = new Set<number>();
for (const t of [launchText, seedText]) for (const m of t.matchAll(/"order": ([0-9.]+)/g)) used.add(Number(m[1]));
const pad = (n: number) => String(n).padStart(3, "0");
const kinds = [
  { label: "🏗️build", group: "4_build", target: "build", base: "900.97", cfgs: ["🛠️prototype", "🚀️release"] },
  { label: "🔎️check", group: "4_gate", target: "check", base: "922.097", cfgs: ["🛠️prototype", "🚀️release"] },
  { label: "🧪️test", group: "4_test", target: "test", base: "903.597", cfgs: ["🛠️prototype", "🚀️release"] },
];
let seq = 0;
const blocks: string[] = [];
const routed = fresh.filter((p) => p.routes && !launchText.includes(`repo run ${p.project}:build `));
for (const p of routed) {
  seq++;
  for (const k of kinds) k.cfgs.forEach((cfg, i) => {
    const order = `${k.base}${pad(seq)}${i + 1}`;
    if (used.has(Number(order))) throw Error("order collision " + order);
    used.add(Number(order));
    const config = i === 0 ? "prototype" : "release";
    blocks.push(
      `    {\n      "name": "🦀️ ${p.crate} ${k.label} ${cfg}",\n      "type": "node-terminal",\n      "request": "launch",\n      "command": "bun repo run ${p.project}:${k.target} --configuration=${config} --parallel=1 -- --lib --offline",\n      "cwd": "\${workspaceFolder}",\n      "presentation": {\n        "group": "${k.group}",\n        "order": ${order}\n      },\n      ${ENV}\n    },\n`,
    );
  });
}
const routeText = blocks.join("");

const out = { fresh: fresh.map((p) => [p.project, p.dir]), createProjects: fresh.filter((p) => p.create).length, routedCrates: routed.length, routeBlocks: blocks.length };
console.log(JSON.stringify(out, null, 1));
if (dry) process.exit(0);

for (const p of fresh) if (p.create) {
  const file = join(root, p.dir, "📋️project.json");
  if (existsSync(file)) continue;
  writeFileSync(file, JSON.stringify({ name: p.project, tags: ["scope:framework", "language:rust"] }, null, 2) + "\n");
}

const live = read(REGISTRY);
if (live !== registryText) throw Error("registry changed during planning; rerun");
const owners: any[] = registry.owners;
for (const p of fresh.sort((a, b) => (a.project < b.project ? -1 : 1))) {
  const row = { name: p.project, root: p.dir, manifest: `${p.dir}/📋️project.json`, packages: [] as string[] };
  let at = owners.findIndex((o) => o.name > row.name);
  if (at < 0) at = owners.length;
  owners.splice(at, 0, row);
}
atomic(REGISTRY, JSON.stringify(registry, null, 2) + "\n");

if (routeText) for (const file of [LAUNCH, SEED]) {
  const text = read(file);
  if (text.split(ANCHOR).length !== 2) throw Error("anchor not unique in " + file);
  if (blocks.some((b) => text.includes(b.split("\n")[1]!))) throw Error("routes already present in " + file);
  atomic(file, text.replace(ANCHOR, () => routeText + ANCHOR));
}
JSON.parse(read(LAUNCH));
Bun.JSONC.parse(read(SEED));
JSON.parse(read(REGISTRY));
console.log("applied");
