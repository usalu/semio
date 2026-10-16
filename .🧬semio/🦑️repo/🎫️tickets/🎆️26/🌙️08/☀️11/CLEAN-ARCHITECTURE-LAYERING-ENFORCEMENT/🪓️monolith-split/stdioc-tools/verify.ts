// ✅️ Static checks of the stdio-contract carve: manifests, workspace membership, owner rows, launch routes, path literals, DAG, single ownership, facade shape.
import {existsSync, readFileSync, readdirSync} from "node:fs";
import {dirname, join, relative, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const STDIO = join(ROOT, "✏️s/🔌️plugins/🗄️stdio");
const C = join(STDIO, "📇️registry/🧬️contract");
const nodes: Record<string, string> = {
  "semio-s-artifact-stdio-contract": "",
  "semio-s-artifact-stdio-contract-wire": "🧾️payload-wire",
  "semio-s-artifact-stdio-contract-window-kit": "🪟️window-kit",
  "semio-s-artifact-stdio-contract-assembly": "🏗️assembly",
  "semio-s-artifact-stdio-contract-editing": "✏️editing",
  "semio-s-artifact-stdio-contract-editing-patch": "✏️editing/🩹️patch",
  "semio-s-artifact-stdio-contract-editing-details": "✏️editing/🪟️details",
  "semio-s-artifact-stdio-contract-part21": "📐️part21",
  "semio-s-artifact-stdio-contract-list-delta": "🪡️list-delta",
  "semio-s-artifact-stdio-contract-media-export": "🎬️media-export",
  "semio-s-artifact-stdio-contract-native-print": "🖨️native-print",
  "semio-s-artifact-stdio-contract-testing": "🧰️testing",
};
const projects: Record<string, string> = Object.fromEntries(Object.keys(nodes).filter(name => name !== "semio-s-artifact-stdio-contract").map(name => [name, "@semio-tech/stdio-artifact-contract-" + name.slice("semio-s-artifact-stdio-contract-".length) + "-rs"]));
const problems: string[] = [];
const check = (condition: boolean, message: string) => { if (!condition) problems.push(message); };
const pkgDir = (node: string) => join(C, node, "📦️packages/🦀️rust");

const workspace = Bun.TOML.parse(readFileSync(join(STDIO, "Cargo.toml"), "utf8")) as any;
const members: string[] = workspace.workspace.members;
check(new Set(members).size === members.length, "duplicate workspace members");
check(members.every((value, index) => index === 0 || Buffer.compare(Buffer.from(members[index - 1]!), Buffer.from(value)) < 0), "workspace members are not byte-sorted");

const edges = new Map<string, Set<string>>();
for (const [name, node] of Object.entries(nodes)) {
  const dir = pkgDir(node), manifest = join(dir, "Cargo.toml");
  check(existsSync(manifest), `${name}: manifest missing`);
  const toml = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  check(toml.package.name === name, `${name}: package name ${toml.package.name}`);
  check(resolve(dir, toml.package.workspace) === STDIO, `${name}: workspace ${resolve(dir, toml.package.workspace)}`);
  check(existsSync(resolve(dir, toml.lib.path)), `${name}: lib ${toml.lib.path}`);
  check(resolve(dir, toml.lib.path) === join(C, node, "🦀️.rs"), `${name}: lib path is not the node root`);
  check(toml.package.metadata?.semio?.role === "s-module", `${name}: role`);
  check(members.includes(relative(STDIO, dir)), `${name}: not a stdio workspace member (${relative(STDIO, dir)})`);
  const tables = [toml.dependencies, toml["dev-dependencies"]];
  const out = new Set<string>();
  for (const table of tables) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (spec?.path) {
      const target = join(resolve(dir, spec.path), "Cargo.toml");
      check(existsSync(target), `${name}: dependency ${key} -> ${spec.path}`);
      if (existsSync(target)) check((Bun.TOML.parse(readFileSync(target, "utf8")) as any).package.name === key, `${name}: dependency key ${key} differs from the package name at ${spec.path}`);
      if (key.startsWith("semio-s-artifact-stdio-contract")) out.add(key);
    }
  }
  edges.set(name, out);
  for (const test of toml.test ?? []) check(existsSync(resolve(dir, test.path)), `${name}: [[test]] ${test.path}`);
  if (projects[name]) {
    const project = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8"));
    check(project.name === projects[name] && project.tags.join() === "language:rust,role:contract,family:stdio" && existsSync(resolve(dir, project.$schema)), `${name}: project.json`);
  }
  check(Buffer.byteLength(relative(ROOT, dir)) < 200, `${name}: path length`);
  check(!("rust-source-boundary" in (toml.package.metadata?.semio ?? {})), `${name}: unexpected boundary keys`);
}
const facade = Bun.TOML.parse(readFileSync(join(pkgDir(""), "Cargo.toml"), "utf8")) as any;
check(JSON.stringify(facade.features) === JSON.stringify({testing: ["dep:semio-s-artifact-stdio-contract-testing"]}), "facade testing feature");
check(facade.dependencies["semio-s-artifact-stdio-contract-testing"]?.optional === true, "facade testing dependency optional");

const state = new Map<string, number>();
const cycles: string[] = [];
const visit = (name: string, stack: string[]) => {
  if (state.get(name) === 2) return;
  if (state.get(name) === 1) { cycles.push([...stack, name].join(" -> ")); return; }
  state.set(name, 1);
  for (const next of edges.get(name) ?? []) visit(next, [...stack, name]);
  state.set(name, 2);
};
for (const name of Object.keys(nodes)) visit(name, []);
check(cycles.length === 0, "dependency cycle: " + cycles.join(" | "));
check(![...edges.entries()].some(([name, deps]) => name !== "semio-s-artifact-stdio-contract" && deps.has("semio-s-artifact-stdio-contract")), "a producer depends on the facade");

const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as {name: string; root: string; manifest: string; packages: string[]}[];
check(new Set(owners.map(row => row.name)).size === owners.length, "owner names are not unique");
for (const [name, project] of Object.entries(projects)) {
  const row = owners.find(owner => owner.name === project), root = relative(ROOT, pkgDir(nodes[name]!));
  check(!!row && row.root === root && row.manifest === root + "/📋️project.json" && row.packages.length === 0, `${project}: owner row`);
}
const sortedOwners = owners.map(row => row.name).filter(name => name.startsWith("@semio-tech/stdio-artifact-contract"));
check(sortedOwners.every((value, index) => index === 0 || sortedOwners[index - 1]! < value), "owner rows are not lexical");

const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seed = JSON.parse(readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8").replace(/^\s*\/\/.*$/gmu, "")).configurations.filter((row: unknown) => typeof row === "object") as any[];
let routes = 0;
for (const [label, rows] of [["launch.json", launch], ["seed", seed]] as [string, any[]][]) {
  const keyed = new Map<string, number>();
  for (const row of rows) { const key = `${row.presentation?.group}:${row.presentation?.order}`; if (row.presentation?.order !== undefined) keyed.set(key, (keyed.get(key) ?? 0) + 1); }
  for (const [name, project] of Object.entries(projects)) for (const [target, label2, group] of [["build", "🏗️build", "4_build"], ["check", "🔎️check", "4_gate"], ["test", "🧪️test", "4_test"]] as const) for (const [config, clabel] of [["prototype", "🛠️prototype"], ["release", "🚀️release"]] as const) {
    const wanted = `🦀️ ${name} ${label2} ${clabel}`, hits = rows.filter(row => row.name === wanted);
    check(hits.length === 1, `${label}: ${wanted} x${hits.length}`);
    const row = hits[0];
    check(row?.command === `bun repo run ${project}:${target} --configuration=${config} --parallel=1 -- --lib --offline` && row?.presentation?.group === group && row?.type === "node-terminal" && row?.request === "launch" && row?.cwd === "${workspaceFolder}" && Object.keys(row).join() === "name,type,request,command,cwd,presentation", `${label}: ${wanted} shape`);
    if (row) check(keyed.get(`${row.presentation.group}:${row.presentation.order}`) === 1, `${label}: order ${row.presentation.group}:${row.presentation.order} reused`);
    routes++;
  }
}

const walk = (dir: string, out: string[] = []): string[] => { for (const entry of readdirSync(dir, {withFileTypes: true})) { if (["node_modules", "dist", "target"].includes(entry.name)) continue; const path = join(dir, entry.name); if (entry.isDirectory()) walk(path, out); else if (entry.name.endsWith(".rs")) out.push(path); } return out; };
const literals = /#\[\s*path\s*=\s*"([^"]+)"\s*\]|include(?:_str|_bytes)?!\(\s*"([^"]+)"\s*\)/gu;
const owner = new Map<string, string>();
let resolved = 0, longest = 0;
const mountsOf = (file: string): string[] => {
  const text = readFileSync(file, "utf8");
  return [...text.matchAll(/#\[\s*path\s*=\s*"([^"]+)"\s*\]/gu)].map(match => resolve(dirname(file), match[1]!));
};
for (const [name, node] of Object.entries(nodes)) {
  const stack = [join(C, node, "🦀️.rs")];
  while (stack.length) {
    const file = stack.pop()!;
    if (owner.has(file)) { check(owner.get(file) === name, `${file.slice(ROOT.length + 1)} is mounted by ${owner.get(file)} and ${name}`); continue; }
    owner.set(file, name);
    check(existsSync(file), `${name}: missing ${file.slice(ROOT.length + 1)}`);
    if (!existsSync(file)) continue;
    longest = Math.max(longest, Buffer.byteLength(file.slice(ROOT.length + 1)));
    for (const mount of mountsOf(file)) stack.push(mount);
    for (const match of readFileSync(file, "utf8").matchAll(literals)) { resolved++; check(existsSync(resolve(dirname(file), (match[1] ?? match[2])!)), `${file.slice(ROOT.length + 1)}: ${(match[1] ?? match[2])} -> missing`); }
  }
}
const nodeRoots = Object.values(nodes).map(node => join(C, node, "🦀️.rs"));
const allSources = walk(C).filter(path => !path.includes("/🧪️tests/") && !path.includes("🧫️fixtures"));
const unowned = allSources.filter(path => !owner.has(path) && !path.includes("/📦️packages/"));
check(unowned.length === 0, "Rust sources owned by no producer: " + unowned.map(path => path.slice(C.length + 1)).join(", "));
const registryRoot = join(STDIO, "📇️registry/🦀️.rs");
check(owner.get(registryRoot) === "semio-s-artifact-stdio-contract-assembly", "registry root is not owned by assembly");
const facadeText = readFileSync(join(C, "🦀️.rs"), "utf8");
check(!/pub use [^;{]*::\*;/u.test(facadeText) && !facadeText.includes("#[path"), "facade has a glob re-export or a mount");
check(longest < 240, `longest path ${longest}`);
console.log(JSON.stringify({manifests: Object.keys(nodes).length, ownerRows: Object.keys(projects).length, launchRoutesPerFile: routes / 2, ownedFiles: owner.size, pathLiterals: resolved, longestPathBytes: longest, nodeRoots: nodeRoots.length}));
console.log(problems.length ? "PROBLEMS\n" + problems.join("\n") : "ALL STATIC CHECKS PASSED");
