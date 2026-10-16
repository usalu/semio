// ✅️ Static registry verification for the plugin-host carve: manifests, workspace members, owner rows, launch routes (both files), dependency DAG with dev edges, testkit forwarding.
import {readFileSync, existsSync} from "node:fs";
import {resolve, join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(process.argv[2]!, "utf8")) as {crates: {name: string; pkg: string; facade: boolean}[]};
let failures = 0;
const fail = (m: string) => { failures++; console.log("FAIL", m); };
const root = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const members: string[] = root.workspace.members;
const sorted = [...members].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
if (JSON.stringify(sorted) !== JSON.stringify(members)) fail("workspace members are not byte-sorted");
if (new Set(members).size !== members.length) fail("duplicate workspace members");
const owners = JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[];
const ownerNames = owners.map((o) => o.name);
if (new Set(ownerNames).size !== owners.length) fail("duplicate owner names");
const ownerSorted = [...ownerNames].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
if (JSON.stringify(ownerSorted) !== JSON.stringify(ownerNames)) fail("owner rows are not byte-sorted");
const launch = JSON.parse(readFileSync(join(ROOT, ".vscode/launch.json"), "utf8")).configurations as any[];
const seedText = readFileSync(join(ROOT, ".vscode/🧩️launch.seed.jsonc"), "utf8");
const seed = JSON.parse(seedText.replace(/^\s*\/\/.*$/gm, "")).configurations as any[];
const edges = new Map<string, string[]>();
const manifests = new Map<string, any>();
const familyNames = new Set(plan.crates.map((c) => c.name));
for (const c of plan.crates) {
  const dir = join(ROOT, c.pkg);
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { fail(`${c.name}: manifest missing`); continue; }
  const m = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  manifests.set(c.name, m);
  if (m.package.name !== c.name) fail(`${c.name}: package name ${m.package.name}`);
  if (resolve(dir, m.package.workspace) !== ROOT) fail(`${c.name}: workspace does not resolve to the repo root`);
  const b = m.package.metadata?.semio?.["rust-source-boundary"];
  if (!c.facade && (!b || Object.keys(b).sort().join() !== "maxActiveLines,maxSourceBytes,maxSourceFiles" || b.maxActiveLines !== 4000 || b.maxSourceFiles !== 32 || b.maxSourceBytes !== 262144)) fail(`${c.name}: boundary keys`);
  if (!existsSync(resolve(dir, m.lib.path))) fail(`${c.name}: lib path ${m.lib.path} missing`);
  if (!m.lints?.workspace) fail(`${c.name}: lints.workspace`);
  if (!members.includes(c.pkg)) fail(`${c.name}: not a workspace member`);
  const project = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8"));
  const owner = owners.find((o) => o.name === project.name);
  if (!owner || owner.root !== c.pkg || owner.manifest !== c.pkg + "/📋️project.json") fail(`${c.name}: owner row`);
  if (!c.facade) {
    if (project.name !== "@semio-tech/" + c.name.replace("semio-", "") + "-rs") fail(`${c.name}: project name ${project.name}`);
    for (const verb of ["build", "check", "test"]) for (const cfg of ["prototype", "release"]) {
      const cmd = `bun repo run ${project.name}:${verb} --configuration=${cfg} --parallel=1 -- --lib --offline`;
      const a = launch.filter((x) => x.command === cmd).length, s = seed.filter((x) => x.command === cmd).length;
      if (a !== 1 || s !== 1) fail(`${c.name}: launch route ${verb} ${cfg} launch=${a} seed=${s}`);
    }
  }
  const deps: string[] = [];
  const tables = [m.dependencies, m["dev-dependencies"]];
  for (const t of tables) for (const [key, spec] of Object.entries(t ?? {}) as [string, any][]) {
    if (typeof spec === "object" && typeof spec.path === "string") {
      const target = resolve(dir, spec.path, "Cargo.toml");
      if (!existsSync(target)) { fail(`${c.name}: dependency ${key} path ${spec.path} missing`); continue; }
      const tm = Bun.TOML.parse(readFileSync(target, "utf8")) as any;
      if ((spec.package ?? key) !== tm.package.name && !spec.package) fail(`${c.name}: dependency ${key} resolves to ${tm.package.name}`);
      if (familyNames.has(tm.package.name)) {
        deps.push(tm.package.name);
        if (spec.features?.includes("testkit") && !(tm.features && "testkit" in tm.features)) fail(`${c.name}: testkit requested from ${tm.package.name} which declares none`);
      }
    }
  }
  for (const [feat, list] of Object.entries(m.features ?? {}) as [string, string[]][]) for (const item of list) {
    const dep = item.split("/")[0]!.replace(/^dep:/, "");
    if (item.includes("/") && !(m.dependencies && dep in m.dependencies)) fail(`${c.name}: feature ${feat} forwards ${item} to a non-normal dependency`);
  }
  edges.set(c.name, deps);
}
const state = new Map<string, number>();
const visit = (p: string, trail: string[]): void => {
  if (state.get(p) === 1) { fail("cycle " + [...trail, p].join(" -> ")); return; }
  if (state.get(p) === 2) return;
  state.set(p, 1);
  for (const d of edges.get(p) ?? []) visit(d, [...trail, p]);
  state.set(p, 2);
};
for (const p of edges.keys()) visit(p, []);
console.log("crates:", plan.crates.length, "owners:", owners.length, "launch configs:", launch.length, "seed configs:", seed.length);
for (const [p, d] of edges) console.log(" ", p.replace("semio-framework-plugin-host", "host"), "->", d.map((x) => x.replace("semio-framework-plugin-host", "host")).join(", "));
console.log(failures === 0 ? "ALL REGISTRY CHECKS PASSED" : `${failures} FAILURES`);
