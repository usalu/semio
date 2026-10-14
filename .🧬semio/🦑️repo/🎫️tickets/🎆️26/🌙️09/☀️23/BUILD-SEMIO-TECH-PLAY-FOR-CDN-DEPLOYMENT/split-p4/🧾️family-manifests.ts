#!/usr/bin/env bun
/**
 * 🧾️ Writes one owner-rooted `<owner>/📦️packages/🦀️rust/Cargo.toml` per crate of a cut config with exactly the packages its
 * cfg-selected lib (native ∪ wasm32) and test closures name, declares every named package once in the crate's workspace
 * root, and gives every consumer package exactly the family crates its own closure names.
 *
 * usage: bun 🧾️family-manifests.ts <cut.json> plan|apply
 *
 * The cut config carries `workspace` (the artifact workspace dir), `plugin_workspace` (the plugin-level workspace dir),
 * `crates[]` (`package`, `root`, optional `description`, optional `tests[]` of `{name, path}` integration targets) and
 * `consumer_manifests[]` (repository-relative manifests).
 *
 * A crate whose lib closure names `feature = "testing"` gets `[features] testing = [...]`: every package only that cfg names
 * becomes an optional dependency it enables, and every family dependency with its own `testing` feature is forwarded.
 * A crate with tests enables `testing` on every family dependency that declares it.
 */
import { existsSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { readRustSourceClosureV1, selectRustSourceV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const NATIVE = ["target_arch=aarch64", "target_os=macos", "target_env=", "target_pointer_width=64", "unix", "debug_assertions", "panic=unwind"];
const WASM = ["target_arch=wasm32", "target_os=wasi", "target_env=p2", "target_pointer_width=32", "debug_assertions", "panic=abort"];
const WEB = ["target_arch=wasm32", "target_os=unknown", "target_env=", "target_pointer_width=32", "debug_assertions", "panic=abort"];
const EXTERNAL: Record<string, string> = { serde: "serde", serde_json: "serde_json", wasm_bindgen: "wasm-bindgen", js_sys: "js-sys", parry3d: "parry3d", pack: "pack", machine: "machine" };
const DEV_FEATURES: Record<string, string[]> = { "semio-framework-plugin": ["artifact-app-testing"], "semio-framework-os-kernel": ["protocol-laws", "mutation-testing"] };
const [configPath, mode] = process.argv.slice(2) as [string, "plan" | "apply"];
const config = JSON.parse(readFileSync(configPath, "utf8"));
const read = async (path: string) => await readFile(path).catch(error => error.code === "ENOENT" ? null : Promise.reject(error));
const strip = (text: string) => text.replace(/\/\*[\s\S]*?\*\//g, " ").replace(/\/\/[^\n]*/g, " ").replace(/r#*"[\s\S]*?"#*/g, '""').replace(/"(?:\\.|[^"\\])*"/g, '""');
const scan = (text: string, out: Set<string>) => {
  const t = strip(text).replace(/(?<![\w>)\]])::/g, " ");
  for (const m of t.matchAll(/(?<![\w:.])([a-z_][a-z0-9_]*)\s*::/g)) out.add(m[1]!);
  for (const m of t.matchAll(/extern\s+crate\s+([a-z_][a-z0-9_]*)/g)) out.add(m[1]!);
  for (const m of t.matchAll(/\buse\s+([a-z_][a-z0-9_]*)\s*(?:;|as\b)/g)) out.add(m[1]!);
  if (/\bstatechart!/.test(t)) out.add("machine");
};
const closure = async (entry: string, cfg: string[]) => {
  const out = new Set<string>(); let tests = false, marked = false;
  for (const input of await readRustSourceClosureV1([entry], REPO, cfg, read)) {
    if (input.kind !== "source") continue;
    const raw = new TextDecoder().decode((await read(resolve(REPO, input.path)))!);
    if (/feature\s*=\s*"testing"/.test(raw.replace(/\/\/[^\n]*/g, ""))) marked = true;
    const text = selectRustSourceV1(raw, cfg);
    scan(text, out);
    if (/#\[(?:[\w:]+::)?test\b/.test(text)) tests = true;
  }
  return { out, tests, marked };
};
const manifestOf = (root: string) => join(dirname(resolve(REPO, root)), "📦️packages", "🦀️rust", "Cargo.toml");
const family = new Map<string, { package: string; root: string; manifest: string; description?: string; plugin: boolean; tests?: { name: string; path: string }[] }>();
for (const row of config.crates) family.set(row.package.replaceAll("-", "_"), { ...row, manifest: manifestOf(row.root), plugin: row.package.startsWith("semio-s-plugin-") });
let repository: Map<string, string> | undefined;
const directoryOf = (name: string) => {
  repository ??= new Map(Bun.spawnSync(["find", "🧰️framework", "✏️s/🔌️plugins/🗄️stdio", "-name", "target", "-prune", "-o", "-name", "node_modules", "-prune", "-o", "-name", "Cargo.toml", "-path", "*📦️packages/🦀️rust/Cargo.toml", "-print"], { cwd: REPO }).stdout.toString().split("\n").filter(Boolean).flatMap(path => {
    const name = readFileSync(join(REPO, path), "utf8").match(/^\[package\][^\[]*?^name\s*=\s*"([^"]+)"/ms)?.[1];
    return name ? [[name, dirname(join(REPO, path))] as [string, string]] : [];
  }));
  const directory = repository.get(name);
  if (!directory) throw Error("Unknown package " + name);
  return directory;
};
const packageOf = (ident: string) => {
  if (family.has(ident)) return family.get(ident)!.package;
  if (ident in EXTERNAL) return EXTERNAL[ident]!;
  if (/^semio_(framework|s|hub)(_|$)/.test(ident)) return ident.replaceAll("_", "-");
  return undefined;
};
type Needs = { lib: Set<string>; web: Set<string>; dev: Set<string>; testing: Set<string>; tests: boolean; feature: boolean };
const TESTING = "feature=testing";
const needsOf = async (entries: string[], self?: string, testEntries: string[] = []): Promise<Needs> => {
  const lib = new Set<string>(), web = new Set<string>(), test = new Set<string>(), testing = new Set<string>(); let tests = false, feature = false;
  for (const entry of entries) {
    for (const id of (await closure(entry, NATIVE)).out) lib.add(id);
    for (const id of (await closure(entry, WASM)).out) lib.add(id);
    for (const id of (await closure(entry, WEB)).out) if (!lib.has(id)) web.add(id);
    const t = await closure(entry, [...NATIVE, "test"]); tests ||= t.tests;
    for (const id of t.out) test.add(id);
    const g = await closure(entry, [...NATIVE, TESTING]);
    for (const id of g.out) if (!lib.has(id)) { testing.add(id); feature = true; }
    feature ||= g.marked;
  }
  for (const entry of testEntries) {
    const t = await closure(entry, [...NATIVE, "test"]); tests ||= t.tests;
    for (const id of t.out) test.add(id);
  }
  const map = (ids: Set<string>) => new Set([...ids].map(packageOf).filter((name): name is string => !!name && name !== self));
  const libs = map(lib), webs = map(web), devs = map(test), testings = map(testing);
  for (const name of libs) { webs.delete(name); devs.delete(name); testings.delete(name); }
  for (const name of webs) devs.delete(name);
  return { lib: libs, web: webs, dev: devs, testing: testings, tests, feature };
};
const row = (name: string, features?: string[]) => `${name} = { workspace = true${features?.length ? `, features = [${features.map(f => JSON.stringify(f)).join(", ")}]` : ""} }`;
const describe = (root: string, fallback: string) => {
  const head = readFileSync(resolve(REPO, root), "utf8").match(/^\/\/!\s*\S+\s+(.*)$/m)?.[1] ?? fallback;
  const clean = head.replace(/`/g, "").replace(/\s+—.*$/, "").replace(/[.:;,]+$/, "").trim();
  return clean.length > 110 ? clean.slice(0, clean.lastIndexOf(" ", 107)) : clean;
};
const workspaceDeclared = new Map<string, Set<string>>();
const declareRows = (workspaceDir: string, names: Iterable<string>) => { const set = workspaceDeclared.get(workspaceDir) ?? new Set(); for (const n of names) set.add(n); workspaceDeclared.set(workspaceDir, set); };
const writes: [string, string][] = [];
const aggregatorPackage = config.aggregator as string;
const needsByCrate = new Map<string, Needs>();
for (const crate of family.values()) needsByCrate.set(crate.package, await needsOf([resolve(REPO, crate.root)], crate.package, (crate.tests ?? []).map(t => resolve(REPO, t.path))));
const hasTesting = (name: string) => needsByCrate.get(name)?.feature === true;
for (const [ident, crate] of family) {
  const needs = needsByCrate.get(crate.package)!;
  const workspaceDir = resolve(REPO, crate.plugin ? config.plugin_workspace : config.workspace);
  const isAggregator = crate.package === aggregatorPackage;
  const libs = [...needs.lib].sort(), webs = [...needs.web].sort();
  const devs = new Set(needs.dev);
  if (needs.tests) for (const name of Object.keys(DEV_FEATURES)) if (needs.lib.has(name) || devs.has(name)) devs.add(name);
  if (needs.tests && (needs.lib.has("semio-framework-async") || devs.has("semio-framework-async"))) devs.add("semio-framework-async-macros");
  if (needs.tests) for (const name of [...needs.lib, ...needs.testing]) if (hasTesting(name)) devs.add(name);
  const devFeatures = (name: string) => [...(DEV_FEATURES[name] ?? []), ...(needs.tests && hasTesting(name) ? ["testing"] : [])];
  const optional = [...needs.testing].sort();
  const testingFeature = needs.feature ? [...optional.map(name => `dep:${name}`), ...libs.filter(hasTesting).map(name => `${name}/testing`)] : [];
  const lines = [
    "[package]",
    `workspace = "${relative(dirname(crate.manifest), workspaceDir)}"`,
    `name = "${crate.package}"`,
    "version.workspace = true",
    "edition.workspace = true",
    "rust-version.workspace = true",
    `description = "${crate.description ?? describe(crate.root, crate.package)}"`,
    "",
    "[package.metadata.semio]",
    `role = "${isAggregator ? "artifact" : "library"}"`,
    "",
    "[lints]",
    "workspace = true",
    "",
    "[lib]",
    `path = "${relative(dirname(crate.manifest), resolve(REPO, crate.root))}"`,
    "",
    "[dependencies]",
    ...libs.map(name => row(name)),
    ...(needs.feature ? optional.map(name => `${name} = { workspace = true, optional = true }`) : []),
  ];
  if (needs.feature) lines.push("", "[features]", `testing = [${testingFeature.map(f => JSON.stringify(f)).join(", ")}]`);
  if (webs.length) lines.push("", `[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]`, ...webs.map(name => row(name)));
  if (devs.size) lines.push("", "[dev-dependencies]", ...[...devs].sort().map(name => row(name, devFeatures(name))));
  for (const target of crate.tests ?? []) lines.push("", "[[test]]", `name = "${target.name}"`, `path = "${relative(dirname(crate.manifest), resolve(REPO, target.path))}"`);
  writes.push([crate.manifest, lines.join("\n") + "\n"]);
  declareRows(workspaceDir, [...libs, ...webs, ...devs, ...(needs.feature ? optional : [])]);
  console.log(`${crate.package}: lib=${libs.length} web=${webs.length} dev=${devs.size} tests=${needs.tests}`);
  void ident;
}
for (const manifest of config.consumer_manifests as string[]) {
  const path = resolve(REPO, manifest), text = readFileSync(path, "utf8");
  const lib = Bun.TOML.parse(text) as any;
  const entries = [resolve(dirname(path), lib.lib?.path ?? "src/lib.rs"), ...((lib.test ?? []) as any[]).map(t => resolve(dirname(path), t.path))].filter(existsSync);
  const needs = await needsOf(entries, lib.package?.name);
  const pick = (set: Set<string>) => [...set].filter(name => family.has(name.replaceAll("-", "_"))).sort();
  const libs = pick(needs.lib), devs = pick(needs.dev).filter(n => !libs.includes(n));
  let updated = text;
  const familyNames = new Set([...family.values()].map(c => c.package));
  const strip = (section: string) => { updated = updated.replace(new RegExp(`(^\\[${section}\\][^\\[]*?)`, "ms"), body => body.split("\n").filter(line => { const name = line.match(/^([\w-]+)\s*=/)?.[1]; return !name || !familyNames.has(name) || name === aggregatorPackage && (libs.includes(name) || devs.includes(name)); }).join("\n")); };
  strip("dependencies"); strip("dev-dependencies");
  const add = (section: string, names: string[]) => {
    const missing = names.filter(name => !new RegExp(`^\\[${section}\\][^\\[]*^${name}\\s*=`, "ms").test(updated));
    if (!missing.length) return;
    const header = new RegExp(`^\\[${section}\\]\\s*\\n`, "m");
    updated = header.test(updated) ? updated.replace(header, h => h + missing.map(n => row(n)).join("\n") + "\n") : updated.trimEnd() + `\n\n[${section}]\n` + missing.map(n => row(n)).join("\n") + "\n";
  };
  add("dependencies", libs); add("dev-dependencies", devs);
  writes.push([path, updated]);
  let directory = dirname(path), pointer = lib.package?.workspace as string | undefined;
  const workspaceDir = pointer ? resolve(directory, pointer) : directory;
  declareRows(workspaceDir, [...libs, ...devs]);
  console.log(`consumer ${manifest}: lib=${libs.join(",")} dev=${devs.join(",")}`);
}
for (const [workspaceDir, names] of workspaceDeclared) {
  const path = join(workspaceDir, "Cargo.toml");
  let text = existsSync(path) ? readFileSync(path, "utf8") : "";
  const pending = writes.find(([p]) => p === path); if (pending) text = pending[1];
  const parsed = Bun.TOML.parse(text) as any;
  const declared = parsed.workspace?.dependencies ?? {};
  const inline = /^\[workspace\.dependencies\]\s*\n[\w-]+\s*=\s*\{/m.test(text) && !/^\s*\[workspace\.dependencies\.[\w-]+\]/m.test(text);
  const added: string[] = [];
  for (const name of [...names].sort()) {
    if (name in declared) continue;
    const familyCrate = family.get(name.replaceAll("-", "_"));
    const external = ["serde", "serde_json", "wasm-bindgen", "js-sys", "parry3d"].includes(name);
    if (external) throw Error(`${path} lacks workspace dependency ${name}`);
    const directory = familyCrate ? dirname(familyCrate.manifest) : name === "pack" ? directoryOf("semio-framework-pack") : name === "machine" ? directoryOf("semio-framework-machine") : directoryOf(name);
    const rel = relative(workspaceDir, directory);
    const extra = name === "pack" ? `, package = "semio-framework-pack"` : name === "machine" ? `, package = "semio-framework-machine"` : "";
    added.push(inline ? `${name} = { path = "${rel}"${extra} }` : `\n    [workspace.dependencies.${name}]\n    path = "${rel}"${extra ? `\n    package = ${extra.split("= ")[1]}` : ""}`);
  }
  const owned = [config.workspace, config.plugin_workspace].map((d: string) => resolve(REPO, d)).includes(workspaceDir);
  if (owned) for (const name of Object.keys(declared)) {
    if (names.has(name)) continue;
    const lines = text.split("\n"), start = lines.findIndex(line => line.trim() === `[workspace.dependencies.${name}]`);
    if (start < 0) throw Error(`${path} declares ${name} inline`);
    let end = start + 1; while (end < lines.length && !/^\s*\[/.test(lines[end]!)) end++;
    while (end > start + 1 && lines[end - 1]!.trim() === "") end--;
    lines.splice(start, end - start); text = lines.join("\n").replace(/\n{3,}/g, "\n\n");
    console.log(`workspace ${relative(REPO, path)}: -${name}`);
  }
  if (!added.length) { if (owned) { if (pending) pending[1] = text; else writes.push([path, text]); } continue; }
  if (inline) text = text.replace(/^\[workspace\.dependencies\]\s*\n/m, h => h + added.join("\n") + "\n");
  else {
    const anchor = text.search(/^\[workspace\.lints/m);
    text = anchor < 0 ? text.trimEnd() + "\n" + added.join("\n") + "\n" : text.slice(0, anchor).trimEnd() + "\n" + added.join("\n") + "\n\n" + text.slice(anchor);
  }
  if (pending) pending[1] = text; else writes.push([path, text]);
  console.log(`workspace ${relative(REPO, path)}: +${added.length}`);
}
for (const [path, text] of writes) {
  if (mode !== "apply") { console.log("would write " + relative(REPO, path)); continue; }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, text);
}
