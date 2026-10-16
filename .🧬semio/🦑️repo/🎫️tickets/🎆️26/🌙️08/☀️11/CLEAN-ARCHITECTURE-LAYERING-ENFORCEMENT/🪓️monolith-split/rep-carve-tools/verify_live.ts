// 🔎️ Static verification of the applied pack carve against the live tree (TOML, JSON, registries, launch routes).
import {readFileSync, existsSync} from "node:fs";
import {resolve, dirname, join} from "node:path";
const R = "/Users/ueli/Documents/semio";
const crates: [string, string, string][] = JSON.parse(process.env.CRATES!); // [cargoName, packageDir, project]
let bad = 0; const fail = (m: string) => { bad++; console.log("FAIL", m); };
const read = (p: string) => readFileSync(join(R, p), "utf8");
const launch = JSON.parse(read(".vscode/launch.json")), seed = Bun.JSONC.parse(read(".vscode/🧩️launch.seed.jsonc")) as any;
const owners = JSON.parse(read("📋️build-owners.json")).owners as any[];
const fwToml = Bun.TOML.parse(read("🧰️framework/Cargo.toml")) as any;
const members: string[] = fwToml.workspace.members;
const rootToml = Bun.TOML.parse(read("Cargo.toml")) as any;
console.log("launch configs", launch.configurations.length, "seed configs", seed.configurations.length, "owners", owners.length);
const names = new Map<string, number>();
for (const o of owners) names.set(o.name, (names.get(o.name) ?? 0) + 1);
for (const [name, dir, project] of crates) {
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(join(R, manifest))) { fail("missing " + manifest); continue; }
  const t = Bun.TOML.parse(read(manifest)) as any;
  if (t.package.name !== name) fail("name " + name);
  const ws = resolve(R, dir, t.package.workspace);
  if (ws !== join(R, "🧰️framework")) fail("workspace " + name + " " + ws);
  if (!existsSync(resolve(R, dir, t.lib.path))) fail("lib path " + name);
  for (const section of ["dependencies", "dev-dependencies"]) for (const [k, v] of Object.entries(t[section] ?? {}) as [string, any][]) {
    if (v.path && !existsSync(resolve(R, dir, v.path, "Cargo.toml"))) fail("dep path " + name + " " + k);
  }
  const b = t.package.metadata.semio["rust-source-boundary"];
  if (Object.keys(b).sort().join() !== "maxActiveLines,maxSourceBytes,maxSourceFiles" || b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32 || b.maxActiveLines !== 4000) fail("boundary " + name);
  const rel = dir.replace("🧰️framework/", "");
  if (!members.includes(rel)) fail("member " + name);
  const row = owners.filter(o => o.root === dir);
  if (row.length !== 1 || row[0].name !== project) fail("owner row " + name + " " + JSON.stringify(row.map(r => r.name)));
  if (names.get(project) !== 1) fail("owner name uniqueness " + project);
  const pj = JSON.parse(read(join(dir, "📋️project.json")));
  if (pj.name !== project) fail("project.json name " + name);
  for (const [file, label] of [[launch, "launch.json"], [seed, "seed"]] as [any, string][]) {
    for (const target of ["build", "check", "test"]) for (const conf of ["prototype", "release"]) {
      const cmd = `bun repo run ${project}:${target} --configuration=${conf} --parallel=1 -- --lib --offline`;
      const hits = file.configurations.filter((c: any) => c.command === cmd);
      if (hits.length !== 1) fail(`${label} route ${project}:${target}:${conf} x${hits.length}`);
    }
  }
}
const sorted = [...members].filter(m => m.startsWith("🔨️modules/📡️replication/")).map(m => Buffer.from(m));
for (let i = 1; i < sorted.length; i++) if (Buffer.compare(sorted[i - 1], sorted[i]) > 0) fail("members unsorted near " + sorted[i].toString());
for (const [file, label] of [[launch, "launch.json"], [seed, "seed"]] as [any, string][]) {
  const orders = new Map<string, string[]>();
  for (const c of file.configurations) { const k = c.presentation?.group + "|" + c.presentation?.order; orders.set(k, [...(orders.get(k) ?? []), c.name]); }
  const mine = file.configurations.filter((c: any) => crates.some(([n]) => c.name?.startsWith("🦀️ " + n + " ")));
  for (const c of mine) { const k = c.presentation.group + "|" + c.presentation.order; if (orders.get(k)!.length !== 1) fail(label + " duplicate order " + k); }
  console.log(label, "my routes", mine.length);
}
for (const f of ["🧰️framework/Cargo.lock", "Cargo.lock"]) {
  const l = Bun.TOML.parse(read(f)) as any;
  const have = new Set(l.package.map((p: any) => p.name));
  for (const [name] of crates) if (!have.has(name)) fail(f + " lacks " + name);
  const pack = l.package.find((p: any) => p.name === "semio-framework-replication");
  console.log(f, "pack deps", pack.dependencies.length);
}
console.log(bad ? "VERIFY FAILED " + bad : "ALL STATIC CHECKS PASSED");
