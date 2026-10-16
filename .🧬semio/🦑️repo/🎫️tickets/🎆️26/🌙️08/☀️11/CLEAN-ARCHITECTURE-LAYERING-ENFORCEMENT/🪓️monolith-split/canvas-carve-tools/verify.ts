// ✅️ Static checks of the canvas carve: manifests, workspace membership, owner rows, launch routes, path literals, taxonomy of names.
import {existsSync, readFileSync, readdirSync, statSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const C = join(ROOT, "🧰️framework/🔨️modules/🖼️canvas");
const nodes = {"semio-framework-canvas": "📦️packages/🦀️rust", "semio-framework-canvas-assets": "🔌️runtime-assets/📦️packages/🦀️rust", "semio-framework-canvas-camera": "📷️camera/📦️packages/🦀️rust", "semio-framework-canvas-icons": "🪪️icon-codec/📦️packages/🦀️rust", "semio-framework-canvas-scene": "🎬️scene/📦️packages/🦀️rust"} as Record<string, string>;
const projects = {"semio-framework-canvas-assets": "@semio-tech/framework-canvas-assets-rs", "semio-framework-canvas-camera": "@semio-tech/framework-canvas-camera-rs", "semio-framework-canvas-icons": "@semio-tech/framework-canvas-icons-rs", "semio-framework-canvas-scene": "@semio-tech/framework-canvas-scene-rs"} as Record<string, string>;
const problems: string[] = [];
const check = (condition: boolean, message: string) => { if (!condition) problems.push(message); };

const frameworkToml = Bun.TOML.parse(readFileSync(join(ROOT, "🧰️framework/Cargo.toml"), "utf8")) as any;
const members: string[] = frameworkToml.workspace.members;
const names = new Map<string, string>();
for (const [name, rel] of Object.entries(nodes)) {
  const dir = join(C, rel), manifest = join(dir, "Cargo.toml");
  check(existsSync(manifest), `${name}: manifest missing`);
  const toml = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  check(toml.package.name === name, `${name}: package name ${toml.package.name}`);
  check(resolve(dir, toml.package.workspace) === join(ROOT, "🧰️framework"), `${name}: workspace ${resolve(dir, toml.package.workspace)}`);
  check(existsSync(resolve(dir, toml.lib.path)), `${name}: lib ${toml.lib.path}`);
  const boundary = toml.package.metadata.semio["rust-source-boundary"];
  check(boundary && Object.keys(boundary).sort().join() === "maxActiveLines,maxSourceBytes,maxSourceFiles" && boundary.maxSourceBytes === 262144 && boundary.maxSourceFiles === 32 && boundary.maxActiveLines === 4000, `${name}: boundary keys`);
  const tables = [toml.dependencies, toml["dev-dependencies"], ...Object.values(toml.target ?? {}).flatMap((t: any) => [t.dependencies, t["dev-dependencies"]])];
  for (const table of tables) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) if (spec?.path) { const target = join(resolve(dir, spec.path), "Cargo.toml"); check(existsSync(target), `${name}: dependency ${key} -> ${spec.path}`); }
  for (const test of toml.test ?? []) check(existsSync(resolve(dir, test.path)), `${name}: [[test]] ${test.path}`);
  names.set(name, dir);
  const member = (dir.slice(join(ROOT, "🧰️framework/").length));
  check(members.includes(member), `${name}: not a framework workspace member (${member})`);
  if (projects[name]) {
    const project = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8"));
    check(project.name === projects[name] && project.tags.join() === "scope:framework,language:rust", `${name}: project.json`);
  }
  check(Buffer.byteLength(dir.slice(ROOT.length + 1)) < 200, `${name}: path length`);
}
const sortedSlice = members.slice(members.indexOf("🔨️modules/🖼️canvas/🎬️scene/📦️packages/🦀️rust"), members.indexOf("🔨️modules/🖼️canvas/🪪️icon-codec/📦️packages/🦀️rust") + 1);
check(sortedSlice.length === 6 && sortedSlice.every((value, index) => index === 0 || Buffer.compare(Buffer.from(sortedSlice[index - 1]!), Buffer.from(value)) < 0), `members not byte-sorted: ${sortedSlice.join(" | ")}`);
check(new Set(members).size === members.length, "duplicate workspace members");

const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as {name: string; root: string; manifest: string; packages: string[]}[];
check(new Set(owners.map(row => row.name)).size === owners.length, "owner names are not unique");
for (const [name, project] of Object.entries(projects)) {
  const row = owners.find(owner => owner.name === project), root = "🧰️framework/🔨️modules/🖼️canvas/" + nodes[name];
  check(!!row && row.root === root && row.manifest === root + "/📋️project.json" && row.packages.length === 0, `${project}: owner row`);
}

const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seed = JSON.parse(readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8").replace(/^\s*\/\/.*$/gmu, "")).configurations.filter((row: unknown) => typeof row === "object") as any[];
for (const [label, rows] of [["launch.json", launch], ["seed", seed]] as [string, any[]][]) {
  const orders = new Map<string, Set<number>>();
  for (const row of rows) { const group = row.presentation?.group; if (typeof row.presentation?.order === "number") { const seen = orders.get(group) ?? new Set(); orders.set(group, seen); } }
  for (const [name, project] of Object.entries(projects)) for (const [target, label2, group] of [["build", "🏗️build", "4_build"], ["check", "🔎️check", "4_gate"], ["test", "🧪️test", "4_test"]] as const) for (const [config, clabel] of [["prototype", "🛠️prototype"], ["release", "🚀️release"]] as const) {
    const wanted = `🦀️ ${name} ${label2} ${clabel}`, hits = rows.filter(row => row.name === wanted);
    check(hits.length === 1, `${label}: ${wanted} x${hits.length}`);
    const row = hits[0];
    check(row?.command === `bun repo run ${project}:${target} --configuration=${config} --parallel=1 -- --lib --offline` && row?.presentation?.group === group && row?.type === "node-terminal" && row?.request === "launch" && row?.cwd === "${workspaceFolder}" && Object.keys(row).join() === "name,type,request,command,cwd,presentation", `${label}: ${wanted} shape`);
  }
  const mine = rows.filter(row => /semio-framework-canvas-/u.test(row.name ?? ""));
  const keyed = new Map<string, number>();
  for (const row of rows) { const key = `${row.presentation?.group}:${row.presentation?.order}`; if (row.presentation?.order !== undefined) keyed.set(key, (keyed.get(key) ?? 0) + 1); }
  for (const row of mine) check(keyed.get(`${row.presentation.group}:${row.presentation.order}`) === 1, `${label}: order ${row.presentation.group}:${row.presentation.order} reused`);
}

const walk = (dir: string, out: string[] = []): string[] => { for (const entry of readdirSync(dir, {withFileTypes: true})) { if (["node_modules", "dist", "target"].includes(entry.name)) continue; const path = join(dir, entry.name); if (entry.isDirectory()) walk(path, out); else if (entry.name.endsWith(".rs")) out.push(path); } return out; };
const literals = /#\[path\s*=\s*"([^"]+)"\]|include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)/gu;
let resolved = 0;
const sources = [join(C, "🦀️.rs"), ...["🔌️runtime-assets", "📷️camera", "🪪️icon-codec", "🎬️scene"].flatMap(node => walk(join(C, node))), ...walk(join(C, "♻️retirement")), ...walk(join(C, "🧪️tests")), ...walk(join(C, "🌉️icon-name-value"))];
const pathModules = (file: string, text: string) => {
  const out: string[] = [];
  let depth = 0;
  for (const match of text.matchAll(literals)) {
    const literal = match[1] ?? match[2]!;
    const base = dirname(file);
    const candidate = resolve(base, literal);
    resolved++;
    check(existsSync(candidate), `${file.slice(ROOT.length + 1)}: ${literal} -> missing`);
    out.push(candidate);
  }
  return out;
};
for (const file of sources) pathModules(file, readFileSync(file, "utf8"));
const longest = Math.max(...sources.map(file => Buffer.byteLength(file.slice(ROOT.length + 1))));
console.log(JSON.stringify({manifests: names.size, ownerRows: Object.keys(projects).length, launchRoutes: 24, sources: sources.length, pathLiterals: resolved, longestPathBytes: longest}));
console.log(problems.length ? "PROBLEMS\n" + problems.join("\n") : "ALL STATIC CHECKS PASSED");
