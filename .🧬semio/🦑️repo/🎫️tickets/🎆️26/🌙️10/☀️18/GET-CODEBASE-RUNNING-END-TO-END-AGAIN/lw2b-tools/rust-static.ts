import TOML from "@iarna/toml";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const root = process.cwd();
const F = "🧰️framework";
const A = `${F}/🔨️modules/🧬️schema/🗿️artifact-reference`;
const crates = [`${A}/🔗️link/🧬️schema/📦️packages/🦀️rust`, `${A}/📦️blob/🧬️schema/📦️packages/🦀️rust`];
const dependents = [
  `${F}/🛍️products/💻️os/📦️packages/🦀️rust`,
  `${F}/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🖥️native/📦️packages/🦀️rust`,
  `${F}/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/👥️space-member/📦️packages/🦀️rust`,
  `${F}/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/📦️packages/🦀️rust`,
  `${F}/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/📦️packages/🦀️rust`,
];
let failures = 0;
const fail = (message: string): void => { failures += 1; console.log("FAIL", message); };
const manifest = (dir: string): any => TOML.parse(readFileSync(join(root, dir, "Cargo.toml"), "utf8"));

for (const dir of [...crates, ...dependents]) {
  const m = manifest(dir);
  const base = join(root, dir);
  const workspaceKey = m.package?.workspace;
  if (typeof workspaceKey === "string") {
    const target = resolve(base, workspaceKey, "Cargo.toml");
    if (!existsSync(target)) fail(`${dir}: workspace ${workspaceKey} has no Cargo.toml`);
    else {
      const w = TOML.parse(readFileSync(target, "utf8")) as any;
      if (!w.workspace) fail(`${dir}: ${target} declares no [workspace]`);
      else if (crates.includes(dir)) {
        const members: string[] = w.workspace.members ?? [];
        const rel = resolve(base).slice(resolve(dirname(target)).length + 1);
        if (!members.includes(rel)) fail(`${dir}: not a member of ${target} (${rel})`);
      }
    }
  }
  for (const section of ["dependencies", "dev-dependencies", "build-dependencies"]) {
    for (const [name, spec] of Object.entries<any>(m[section] ?? {})) {
      if (spec?.path === undefined) continue;
      const depDir = resolve(base, spec.path);
      const depManifest = join(depDir, "Cargo.toml");
      if (!existsSync(depManifest)) { fail(`${dir}: [${section}] ${name} -> ${spec.path} has no Cargo.toml`); continue; }
      const actual = (TOML.parse(readFileSync(depManifest, "utf8")) as any).package?.name;
      const declared = spec.package ?? name;
      if (actual !== declared) fail(`${dir}: [${section}] ${name} -> ${spec.path} is package ${actual}`);
    }
  }
  if (m.lib?.path && !existsSync(resolve(base, m.lib.path))) fail(`${dir}: lib path ${m.lib.path} missing`);
  for (const t of m.test ?? []) if (!existsSync(resolve(base, t.path))) fail(`${dir}: test ${t.name} path ${t.path} missing`);
}

const rootManifest = TOML.parse(readFileSync("Cargo.toml", "utf8")) as any;
for (const dir of crates) if ((rootManifest.workspace.members as string[]).includes(dir)) fail(`root Cargo.toml still lists ${dir}`);
for (const member of rootManifest.workspace.members as string[]) if (!existsSync(join(root, member, "Cargo.toml"))) fail(`root member without manifest: ${member}`);
const frameworkManifest = TOML.parse(readFileSync(`${F}/Cargo.toml`, "utf8")) as any;
for (const member of frameworkManifest.workspace.members as string[]) if (!existsSync(join(root, F, member, "Cargo.toml"))) fail(`framework member without manifest: ${member}`);

const lock = readFileSync(`${F}/Cargo.lock`, "utf8");
const packages = lock.split("\n[[package]]\n").slice(1);
const names = packages.map((block) => /^name = "([^"]+)"/u.exec(block)?.[1] ?? "");
const sorted = [...names].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
if (names.join("\n") !== sorted.join("\n")) fail("framework Cargo.lock package blocks are not sorted by name");
const known = new Set(names);
for (const block of packages) {
  const name = /^name = "([^"]+)"/u.exec(block)?.[1];
  if (!name?.startsWith("semio-framework-os-store-")) continue;
  const deps = /dependencies = \[\n([\s\S]*?)\n\]/u.exec(block)?.[1]?.split("\n").map((l) => l.trim().replace(/^"|",?$/gu, "")) ?? [];
  if (deps.join("\n") !== [...deps].sort().join("\n")) fail(`lock deps unsorted for ${name}`);
  for (const dep of deps) if (!known.has(dep)) fail(`lock dependency ${dep} of ${name} missing`);
  console.log("lock", name, deps.length, "deps ok");
}

const includeRe = /include_(?:str|bytes)!\(\s*"([^"]+)"\s*\)/gu;
const rsFiles = (process.argv.slice(2).length ? process.argv.slice(2) : []).filter((f) => f.endsWith(".rs"));
let includes = 0;
for (const file of rsFiles) {
  const text = readFileSync(join(root, file), "utf8");
  for (const m of text.matchAll(includeRe)) {
    includes += 1;
    if (!existsSync(resolve(root, dirname(file), m[1]!))) fail(`${file}: include path ${m[1]} missing`);
  }
}
console.log("include_str/bytes checked", includes, "in", rsFiles.length, "files");

const limits = { activeLines: 4000, files: 32, bytes: 262144 };
for (const dir of [...crates, `${F}/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/📦️packages/🦀️rust`]) {
  const m = manifest(dir);
  const lib = resolve(root, dir, m.lib?.path ?? "src/lib.rs");
  const inputs = await readRustSourceClosureV1([lib], root, [], async (path) => { try { return new Uint8Array(readFileSync(path)); } catch { return null; } });
  const total = { files: inputs.length, activeLines: inputs.reduce((s, i) => s + i.activeLines, 0), bytes: inputs.reduce((s, i) => s + i.bytes, 0) };
  const declared = m.package?.metadata?.semio?.["rust-source-boundary"];
  console.log("closure", dir.slice(-70), JSON.stringify(total), "declared", JSON.stringify(declared ?? null));
  if (total.files > limits.files || total.activeLines > limits.activeLines || total.bytes > limits.bytes) fail(`${dir}: closure exceeds global source boundary`);
  if (declared && (total.files > declared.maxSourceFiles || total.activeLines > declared.maxActiveLines || total.bytes > declared.maxSourceBytes)) fail(`${dir}: closure exceeds declared source boundary`);
}
console.log(failures === 0 ? "RUST-STATIC OK" : `RUST-STATIC FAILURES ${failures}`);
process.exit(failures === 0 ? 0 : 1);
