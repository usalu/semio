import { createHash, randomUUID } from "node:crypto";
import { constants, closeSync, existsSync, fstatSync, lstatSync, mkdirSync, openSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { spawn, type ChildProcess } from "node:child_process";
import { Readable } from "node:stream";
import { pipeline } from "node:stream/promises";
const root = process.cwd();
const ticket = resolve(import.meta.dir, "..");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const load = (path: string) => JSON.parse(readFileSync(join(root, path), "utf8"));
const imported = (path: string) => import(pathToFileURL(join(root, path)).href);
if (["self-test", "capture", "imports"].includes(process.argv[2] ?? "")) { await privateSource(process.argv[2]!); process.exit(0); }
if (process.argv[2] === "source") {
  const { loadTaxonomy } = await imported(`${library}/🧹️normalization/🔣️taxonomy/🟦️.ts`);
  const { gitRows, untrackedGitPaths } = await imported(`${library}/🧹️normalization/📸️repository-capture/🟦️.ts`);
  const { cacheInternals } = await imported(`${library}/🏗️build/🕸️graph/📥️inference/🟨️.mjs`);
  const taxonomy = loadTaxonomy({ repoRoot: root }), indexed = gitRows(root, taxonomy), untracked = untrackedGitPaths(root, taxonomy);
  const prior = load(relative(root, join(ticket, "🗑️generated/private-source-metadata.json")));
  const paths = new Set([...indexed.map((row: any) => row.path), ...untracked]);
  for (const [name, rows] of Object.entries(prior.groups) as [string, string[]][]) if (name !== "conservative-domain-source-payload") for (const path of rows) paths.add(path);
  const cli = new Set<string>();
  for (const path of await cacheInternals.relativeScriptInputs([join(root, library, "🏗️build/📜️script.ts")], root, undefined, true)) if (path.startsWith("{workspaceRoot}/")) cli.add(path.slice(16));
  const schemas = (path: string) => {
    if (!existsSync(join(root, path))) return;
    for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
      const child = path + "/" + entry.name;
      if (entry.isDirectory() && !entry.isSymbolicLink()) schemas(child);
      else if (entry.isFile() && child.endsWith(".json")) cli.add(child);
    }
  };
  for (const path of [...cli]) if (/\.[cm]?[jt]sx?$/u.test(path)) schemas(dirname(path) + "/🧬️schema");
  for (const path of cli) paths.add(path);
  const excluded = new Set(["node_modules", ".git", ".cache", ".venv", ".🧬semio", "target", "dist", "🗑️generated", "📤️dist", "coverage", "__pycache__"]);
  const files: any[] = [], missing: string[] = [], links: string[] = [], omitted: string[] = [];
  for (const path of [...paths].sort()) {
    if (path.split("/").some(part => excluded.has(part))) { omitted.push(path); continue; }
    const facts = lstatSync(join(root, path), { throwIfNoEntry: false });
    if (!facts) { missing.push(path); continue; }
    if (facts.isSymbolicLink()) { links.push(path); continue; }
    if (facts.isFile()) files.push({ path, bytes: facts.size, relativeUnits: path.length, guestUnits: join("/repo", path).length, pathBytes: Buffer.byteLength(path), depth: path.split("/").length });
  }
  const summarize = (rows: any[]) => ({ files: rows.length, bytes: rows.reduce((sum, row) => sum + row.bytes, 0), maximumRelativeUnits: Math.max(...rows.map(row => row.relativeUnits)), maximumGuestUnits: Math.max(...rows.map(row => row.guestUnits)), maximumPathBytes: Math.max(...rows.map(row => row.pathBytes)), maximumDepth: Math.max(...rows.map(row => row.depth)), maximumFileBytes: Math.max(...rows.map(row => row.bytes)) });
  const value = { version: 1, authority: false, sourceSnapshot: false, measuredAt: new Date().toISOString(), ownershipMeasuredAt: prior.capturedAt, indexed: indexed.length, untracked: untracked.length, cli: summarize(files.filter(row => cli.has(row.path))), cliPaths: [...cli].sort(), summary: summarize(files), files, missing, links, omitted, pathViolations: files.filter(row => row.guestUnits > 256 || row.pathBytes > 1024 || row.depth > 64) };
  writeFileSync(join(ticket, "🗑️generated/private-source-complete-metadata.json"), JSON.stringify(value, null, 2) + "\n");
  console.log("[DEBUG] private Source ordinary physical indexed + untracked + required provider union", { indexed: value.indexed, untracked: value.untracked, cli: value.cli, summary: value.summary, missing: missing.length, links: links.length, omitted: omitted.length, pathViolations: value.pathViolations.length });
  process.exit(0);
}
if (process.argv[2] === "registry") {
  const { default: Ajv } = await import("ajv/dist/2020");
  const { readBuildOwnerRegistryV1 } = await imported(`${library}/🏗️build/🕸️graph/🚪️admission/🟦️.ts`);
  const value = load("📋️build-owners.json");
  const validate = new Ajv({ strict: false }).compile(load(`${library}/🏗️build/🧬️schema/📋️owners.json`));
  if (!validate(value)) throw new Error(JSON.stringify(validate.errors));
  const registry = readBuildOwnerRegistryV1(value);
  const missing = registry.owners.flatMap((owner: any) => { const facts = lstatSync(join(root, owner.manifest), { throwIfNoEntry: false }); return facts?.isFile() && !facts.isSymbolicLink() ? [] : [{ name: owner.name, manifest: owner.manifest }]; });
  console.log("[DEBUG] canonical registry owned parser + Ajv shape accepted", { owners: registry.owners.length, missing });
  if (missing.length) throw new Error("Canonical registry contains missing Source manifests");
  console.log("[DEBUG] canonical registry ordinary physical Source manifest parity PASS");
  process.exit(0);
}
if (process.argv[2] === "payload") {
  const prior = JSON.parse(readFileSync(join(ticket, "🗑️generated/private-source-metadata.json"), "utf8"));
  const policy = load(`${library}/🏗️build/📥️inputs/🧮️digest/📥️read/🧬️schema/🔣️.json`).definitions.Policy.const;
  const files = new Map<string, { path: string; bytes: number; relativeUnits: number; guestUnits: number }>();
  const exclusions = new Set(["node_modules", ".git", ".cache", ".venv", ".🧬semio", "target", "dist", "build", "🗑️generated", "🤖️generated", "📤️dist", "coverage", "__pycache__"]);
  const links: string[] = [], missing: string[] = [], nativeClosureBlocks: { crate: string; detail: string }[] = [], groups: Record<string, string[]> = {};
  const add = (group: string, path: string) => {
    const absolute = join(root, path), facts = lstatSync(absolute, { throwIfNoEntry: false });
    if (!facts) { missing.push(path); return; }
    if (facts.isSymbolicLink()) { links.push(path); return; }
    if (!facts.isFile()) return;
    if (!files.has(path)) files.set(path, { path, bytes: facts.size, relativeUnits: path.length, guestUnits: join("/repo", path).length });
    if (files.size > policy.maximumUniqueSourceFiles || facts.size > policy.maximumSourceFileBytes) throw Error("Schema-bounded payload metadata exceeded");
    (groups[group] ??= []).push(path);
  };
  const walk = (group: string, path: string) => {
    for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
      if (exclusions.has(entry.name)) continue;
      const child = path + "/" + entry.name;
      if (entry.isDirectory()) walk(group, child);
      else add(group, child);
    }
  };
  for (const path of prior.ownership.packages) walk("complete-physical-package-payload", path);
  const { cacheInternals } = await imported(`${library}/🏗️build/🕸️graph/📥️inference/🟨️.mjs`);
  for (const crate of prior.proctorCrates) {
    const directory = dirname(crate.manifest), document = Bun.TOML.parse(readFileSync(crate.manifest, "utf8")) as any;
    const entries = [document.lib?.path ?? "src/lib.rs", "src/main.rs", ...(document.bin ?? []).map((target: any) => target.path), ...(document.package?.build === false ? [] : [typeof document.package?.build === "string" ? document.package.build : "build.rs"])].filter(Boolean).map((path: string) => resolve(directory, path)).filter(existsSync);
    add("proctor-native-source-closure", relative(root, crate.manifest));
    try { for (const path of cacheInternals.rustSourceFiles(entries, directory)) add("proctor-native-source-closure", relative(root, path)); }
    catch (error) { nativeClosureBlocks.push({ crate: crate.name, detail: String(error) }); const owner = dirname(directory).split("/").at(-1) === "📦️packages" ? dirname(dirname(directory)) : directory; walk("proctor-explicit-owner-fallback", relative(root, owner)); }
  }
  const sourcePayload = prior.groups["conservative-domain-source-payload"].filter((path: string) => !path.split("/").some((part: string) => exclusions.has(part)));
  for (const path of sourcePayload) add("declared-domain-payload-without-environments", path);
  const summaries = Object.fromEntries(Object.entries(groups).map(([name, paths]) => {
    const rows = [...new Set(paths)].map(path => files.get(path)!);
    return [name, { files: rows.length, bytes: rows.reduce((sum, row) => sum + row.bytes, 0), maximumRelativeUnits: Math.max(...rows.map(row => row.relativeUnits)), maximumGuestUnits: Math.max(...rows.map(row => row.guestUnits)) }];
  }));
  const value = { version: 1, purpose: "complete-private-source-payload-metadata", authority: false, sourceSnapshot: false, measuredAt: new Date().toISOString(), ownershipMeasuredAt: prior.capturedAt, groups, summaries, files: [...files.values()], missing, links, nativeClosureBlocks, schemaPolicy: policy };
  writeFileSync(join(ticket, "🗑️generated/private-source-payload-metadata.json"), JSON.stringify(value, null, 2) + "\n");
  console.log("[DEBUG] complete package and proctor payload metadata", { summaries, missing, links, nativeClosureBlocks });
  process.exit(0);
}
const groups = new Map<string, Set<string>>();
const blocks: { stage: string; detail: string }[] = [];
const add = (group: string, path: string) => { const files = groups.get(group) ?? new Set(); files.add(path.replaceAll("\\", "/")); groups.set(group, files); };
const policy = load(`${library}/🏗️build/📥️source/🔏️closure/🧬️schema/🔣️.json`).definitions.Policy.const;
const projectionPolicy = load(`${library}/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json`).$defs.ProjectionPolicy.const;
const budget = new AbortController();
const deadline = setTimeout(() => budget.abort(new Error("bounded private Source metadata deadline")), 60000);
const observed = new Set<string>();
let workspaceOperations = 0;
let ownership: any;
try {
  const api = await imported(`${library}/🗂️workspaces/🟦️bun/🟦️.ts`);
  ownership = await api.inspectBunWorkspaceOwnership(root, { kind: "all" }, { signal: budget.signal, advance: async (event: any) => {
    budget.signal.throwIfAborted();
    workspaceOperations++;
    if (event.operation === "readText") observed.add(event.path);
  } });
  for (const path of observed) if (path.startsWith(root + "/")) {
    const facts = lstatSync(path, { throwIfNoEntry: false });
    if (facts?.isFile() && !facts.isSymbolicLink()) add("dependency-provider-observed-files", relative(root, path));
  }
  for (const scope of ownership.scopes) {
    add("dependency-install-inputs", scope.manifest);
    const owner = dirname(scope.manifest);
    for (const name of ["bun.lock", "bunfig.toml"]) if (existsSync(join(root, owner, name))) add("dependency-install-inputs", join(owner, name));
    const paths = [scope.manifest, ...scope.packages.map((path: string) => `${path}/package.json`)];
    for (const path of paths) {
      add("dependency-install-inputs", path);
      const manifest = load(path);
      for (const patch of Object.values(manifest.patchedDependencies ?? {})) add("dependency-install-inputs", join(dirname(path), patch as string));
    }
  }
  console.log("[DEBUG] current Bun ownership metadata", { scopes: ownership.scopes.length, packages: ownership.packages.length, observedManifestPaths: observed.size, workspaceOperations });
} catch (error) { blocks.push({ stage: "current Bun workspace ownership", detail: String(error) }); console.log("[DEBUG] Bun ownership metadata blocked", String(error)); }
finally { clearTimeout(deadline); }
const { cacheInternals } = await imported(`${library}/🏗️build/🕸️graph/📥️inference/🟨️.mjs`);
const cli = `${library}/🏗️build/📜️script.ts`;
const registry = load("📋️build-owners.json");
const ownerNames = ["@teaching/architecture-course", "@semio-tech/assets", "@semio-tech/ui-styling-tokens", "@teaching/proctor"];
const owners = registry.owners.filter((owner: any) => ownerNames.includes(owner.name));
const entries = [cli, `${library}/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts`, ...owners.map((owner: any) => `${owner.root}/📜️script.ts`), "🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts"];
for (const [group, selected] of [["cli-defining-closure", [cli]], ["selected-build-command-closure", entries.slice(1)]] as const) {
  try {
    const closure = await cacheInternals.relativeScriptInputs(selected.map(path => join(root, path)), root, undefined, true);
    for (const path of closure) if (path.startsWith("{workspaceRoot}/")) add(group, path.slice("{workspaceRoot}/".length));
    console.log("[DEBUG] defining Source metadata", { group, files: groups.get(group)?.size });
  } catch (error) { blocks.push({ stage: group, detail: String(error) }); console.log("[DEBUG] defining Source metadata blocked", group, String(error)); }
}
const includeSchemas = (group: string, path: string) => {
  const facts = lstatSync(join(root, path), { throwIfNoEntry: false });
  if (!facts) return;
  if (!facts.isDirectory() || facts.isSymbolicLink()) throw new Error("Ordinary no-follow schema directory required");
  for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
    const child = path + "/" + entry.name;
    if (entry.isDirectory()) includeSchemas(group, child);
    else if (entry.isFile() && child.endsWith(".json")) add(group, child);
  }
};
for (const group of ["cli-defining-closure", "selected-build-command-closure"]) for (const path of [...groups.get(group) ?? []]) if (/\.[cm]?[jt]sx?$/u.test(path)) includeSchemas(group, dirname(path) + "/🧬️schema");
for (const path of ["package.json", "bun.lock", "bunfig.toml", "📋️project.json", "📋️build-owners.json", "Cargo.toml", "Cargo.lock", ".cargo/config.toml", "rust-toolchain.toml", "tsconfig.json", "🎓️teaching/Cargo.toml", "🎓️teaching/Cargo.lock"]) if (existsSync(join(root, path))) add("root-and-workspace-policy", path);
for (const owner of registry.owners) add("all-graph-owner-manifests", owner.manifest);
for (const provider of registry.catalogProviders) add("graph-catalog-providers", provider.sourcePath);
const sourceEntries = ["🎓️teaching/🏛️architecture/📚️course/🟦️.ts"];
try {
  for (const path of await cacheInternals.relativeScriptInputs(sourceEntries.map(path => join(root, path)), root, undefined, true)) if (path.startsWith("{workspaceRoot}/")) add("course-runtime-import-closure", path.slice("{workspaceRoot}/".length));
} catch (error) { blocks.push({ stage: "course-runtime-import-closure", detail: String(error) }); }
const nativeManifests = new Set<string>();
let proctorCrates: any[] = [];
try {
  const { cargoPackageClosure } = await imported("🎓️teaching/🛂️proctor/🏗️bootstrap/🕸️manifest-closure/🟦️.ts");
  proctorCrates = await cargoPackageClosure(join(root, "🎓️teaching/Cargo.toml"), "teaching-proctor", { signal: AbortSignal.timeout(60000), onManifest: (path: string) => { if (existsSync(path)) { nativeManifests.add(relative(root, path)); add("proctor-native-manifest-reads", relative(root, path)); } } });
} catch (error) { blocks.push({ stage: "proctor-owned-manifest-closure", detail: String(error) }); }
const roots = ["🎓️teaching/🏛️architecture", "🎓️teaching/🛂️proctor", "🧰️framework/🛍️products/📚️course", "🧰️framework/🛍️products/🐾️pets", "🧰️framework/🛍️products/🖥️server", "🧰️framework/🔨️modules/🖱️ui", "🧰️framework/🔨️modules/🖼️assets"];
const omitted = new Set([".git", ".cache", ".🧬semio", "node_modules", "target", "dist", "build", "🗑️generated", "🤖️generated", "📤️dist", "coverage", "storybook-static", "__pycache__"]);
const walk = (path: string) => {
  for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
    if (omitted.has(entry.name)) continue;
    const child = path + "/" + entry.name;
    if (entry.isSymbolicLink()) { blocks.push({ stage: "conservative declared domain payload", detail: "Skipped symlink: " + child }); continue; }
    if (entry.isDirectory()) walk(child);
    else if (entry.isFile()) add("conservative-domain-source-payload", child);
  }
};
for (const path of roots) walk(path);
const facts = new Map<string, any>();
const all = [...new Set([...groups.values()].flatMap(paths => [...paths]))].sort();
for (const path of all) {
  const absolute = resolve(root, path);
  const stat = lstatSync(absolute, { throwIfNoEntry: false });
  if (!stat?.isFile() || stat.isSymbolicLink()) { blocks.push({ stage: "metadata roster", detail: `Not an ordinary current file: ${path}` }); continue; }
  const bytes = readFileSync(absolute);
  const after = lstatSync(absolute);
  if (["dev", "ino", "size", "mtimeMs", "ctimeMs"].some(key => stat[key] !== after[key])) { blocks.push({ stage: "metadata roster", detail: `Source changed while measuring: ${path}` }); continue; }
  facts.set(path, { path, bytes: bytes.length, sha256: createHash("sha256").update(bytes).digest("hex"), relativeUnits: path.length, guestUnits: join("/repo", path).length, hostUnits: absolute.length, utf8Bytes: Buffer.byteLength(path) });
}
const summaries = Object.fromEntries([...groups].map(([name, paths]) => {
  const rows = [...paths].flatMap(path => facts.has(path) ? [facts.get(path)] : []);
  return [name, { files: rows.length, bytes: rows.reduce((sum, row) => sum + row.bytes, 0), maximumRelativeUnits: Math.max(0, ...rows.map(row => row.relativeUnits)), maximumGuestUnits: Math.max(0, ...rows.map(row => row.guestUnits)), maximumFileBytes: Math.max(0, ...rows.map(row => row.bytes)) }];
}));
const value = { version: 1, purpose: "private-source-metadata-proposal", capturedAt: new Date().toISOString(), authority: false, sourceSnapshot: false, implementationCapturePolicy: policy, projectionPolicy, originalRoot: root, prospectiveGuestRoot: "/repo", ownership, workspaceOperations, proctorCrates, owners, entries, groups: Object.fromEntries([...groups].map(([name, paths]) => [name, [...paths].sort()])), summaries, union: { files: facts.size, bytes: [...facts.values()].reduce((sum, row) => sum + row.bytes, 0), maximumRelativeUnits: Math.max(...[...facts.values()].map(row => row.relativeUnits)), maximumGuestUnits: Math.max(...[...facts.values()].map(row => row.guestUnits)) }, files: [...facts.values()], blocks };
writeFileSync(join(ticket, "🗑️generated/private-source-metadata.json"), JSON.stringify(value, null, 2) + "\n");
console.log("[DEBUG] bounded private Source metadata proposal", { summaries, union: value.union, blocks });

async function privateSource(mode: string): Promise<void> {
  const maximumBytes = 4 * 1024 ** 3, maximumFiles = 262144, maximumMilliseconds = 15 * 60000;
  const image = "sha256:04ed9880ecae29f4b9954f72f498b2da9b3b58e4fc4b482d193b64e4a38118e0";
  const generated = join(ticket, "🗑️generated"), digest = (bytes: Uint8Array | string) => createHash("sha256").update(bytes).digest("hex");
  const stop = new AbortController(), started = Date.now();
  let active: ReturnType<typeof Bun.spawn> | undefined;
  let streamingChild: ChildProcess | undefined;
  const cancel = () => { stop.abort(Error("Private Source capture cancelled")); active?.kill(); streamingChild?.kill(); };
  const timer = setTimeout(cancel, maximumMilliseconds);
  process.on("SIGINT", cancel); process.on("SIGTERM", cancel);
  const check = () => { stop.signal.throwIfAborted(); if (Date.now() - started > maximumMilliseconds) throw Error("Private Source deadline exceeded"); };
  const progress = (phase: string, facts: unknown) => console.log("[DEBUG] private Source " + phase + " " + JSON.stringify(facts));
  const run = async (args: string[], input?: Uint8Array, quiet = false) => {
    check(); active = Bun.spawn(args, { stdin: input ? "pipe" : "ignore", stdout: quiet ? "pipe" : "inherit", stderr: "inherit" });
    if (input) { await active.stdin.write(input); await active.stdin.end(); }
    const output = quiet ? await new Response(active.stdout).text() : "", status = await active.exited; active = undefined;
    check(); if (status !== 0) throw Error("Private Source child refused: " + args[0] + " status " + status); return output.trim();
  };
  const stream = async (args: string[], chunks: Iterable<Buffer>) => {
    check(); const child = streamingChild = spawn(args[0]!, args.slice(1), { stdio: ["pipe", "inherit", "inherit"], signal: stop.signal });
    const terminal = new Promise<number>((resolve, reject) => { child.once("error", reject); child.once("close", (status, signal) => status === null ? reject(Error("Private Source stream terminated: " + signal)) : resolve(status)); });
    try { await pipeline(Readable.from(chunks, { objectMode: false }), child.stdin!, { signal: stop.signal }); const status = await terminal; check(); if (status !== 0) throw Error("Private Source stream child refused: " + status); }
    catch (error) { child.kill(); await terminal.catch(() => {}); throw error; }
    finally { streamingChild = undefined; }
  };
  const safe = (path: string) => {
    if (!path || path.startsWith("/") || /[:\\\0\r\n]/u.test(path) || path.split("/").some(part => !part || part === "." || part === "..") || join("/repo", path).length > 256 || Buffer.byteLength(path) > 1024 || path.split("/").length > 64) throw Error("Bounded private Source path required: " + path);
    return path;
  };
  const identity = (facts: any) => [facts.dev, facts.ino, facts.mode, facts.size, facts.mtimeNs, facts.ctimeNs].join(":");
  const read = (base: string, path: string, limit: number) => {
    check(); safe(path); const absolute = resolve(base, path), parents: { path: string; identity: string }[] = [];
    for (let current = dirname(absolute);; current = dirname(current)) {
      const facts = lstatSync(current, { bigint: true });
      if (!facts.isDirectory() || facts.isSymbolicLink()) throw Error("No-follow Source ancestry required: " + current);
      parents.push({ path: current, identity: [facts.dev, facts.ino].join(":") });
      if (current === base) break;
      if (dirname(current) === current) throw Error("Private Source escaped root");
    }
    const descriptor = openSync(absolute, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
    try {
      const before = fstatSync(descriptor, { bigint: true });
      if (!before.isFile() || before.size > BigInt(limit)) throw Error("Bounded ordinary private Source required: " + path);
      const bytes = readFileSync(descriptor), after = fstatSync(descriptor, { bigint: true }), named = lstatSync(absolute, { bigint: true });
      if (named.isSymbolicLink() || identity(before) !== identity(after) || identity(before) !== identity(named) || bytes.length !== Number(before.size)) throw Error("Private Source changed while reading: " + path);
      for (const row of parents) { const facts = lstatSync(row.path, { bigint: true }); if (!facts.isDirectory() || facts.isSymbolicLink() || [facts.dev, facts.ino].join(":") !== row.identity) throw Error("Private Source ancestry changed"); }
      return { path, bytes, sha256: digest(bytes), identity: identity(before), executable: Boolean(Number(before.mode) & 0o111) };
    } finally { closeSync(descriptor); }
  };
  const unchanged = (base: string, row: ReturnType<typeof read>) => {
    const current = read(base, row.path, row.bytes.length);
    if (current.sha256 !== row.sha256 || current.identity !== row.identity) throw Error("Private Source cohort changed: " + row.path);
  };
  const tarHeader = (path: string, bytes: number, type = "0", executable = false) => {
    const header = Buffer.alloc(512), octal = (value: number, width: number) => value.toString(8).padStart(width - 1, "0") + "\0";
    header.write(path, 0, 100, "utf8"); header.write(octal(executable ? 0o555 : 0o444, 8), 100); header.write(octal(0, 8), 108); header.write(octal(0, 8), 116); header.write(octal(bytes, 12), 124); header.write(octal(0, 12), 136); header.fill(32, 148, 156); header.write(type, 156); header.write("ustar\0", 257); header.write("00", 263);
    header.write(header.reduce((sum, byte) => sum + byte, 0).toString(8).padStart(6, "0") + "\0 ", 148); return header;
  };
  const pax = (path: string) => {
    const text = " path=" + path + "\n"; let length = Buffer.byteLength(text) + 1;
    while (String(length).length + Buffer.byteLength(text) !== length) length = String(length).length + Buffer.byteLength(text);
    return Buffer.from(String(length) + text);
  };
  function* archive(rows: readonly { path: string; bytes: Buffer; executable?: boolean }[]) {
    for (const [index, row] of rows.entries()) {
      safe(row.path); const metadata = pax(row.path);
      yield tarHeader("PaxHeaders/" + index, metadata.length, "x"); yield metadata;
      if (metadata.length % 512) yield Buffer.alloc(512 - metadata.length % 512);
      yield tarHeader("Source/" + index, row.bytes.length, "0", row.executable);
      for (let offset = 0; offset < row.bytes.length; offset += 65536) yield row.bytes.subarray(offset, offset + 65536);
      if (row.bytes.length % 512) yield Buffer.alloc(512 - row.bytes.length % 512);
    }
    yield Buffer.alloc(1024);
  }
  let volume: string | undefined, container: string | undefined, retained = false;
  try {
    if (mode === "self-test") {
      const assert = await import("node:assert/strict"), fixture = join(generated, "private-source-fixture"); rmSync(fixture, { recursive: true, force: true }); mkdirSync(fixture, { recursive: true });
      const vectors = JSON.parse(readFileSync(join(import.meta.dir, "🧫️capture.json"), "utf8")) as { valid: string[]; invalid: string[] };
      for (const path of vectors.valid) assert.doesNotThrow(() => safe(path));
      for (const path of vectors.invalid) assert.throws(() => safe(path));
      writeFileSync(join(fixture, "plain"), "original"); const row = read(fixture, "plain", 1024); unchanged(fixture, row);
      assert.throws(() => read(fixture, "plain", 1)); writeFileSync(join(fixture, "plain"), "changed!"); assert.throws(() => unchanged(fixture, row));
      symlinkSync("plain", join(fixture, "link")); assert.throws(() => read(fixture, "link", 1024));
      const rows = [{ path: vectors.valid[0]!, bytes: Buffer.alloc(0) }, { path: vectors.valid[1]!, bytes: Buffer.from("authored unicode\n"), executable: true }];
      const packed = Buffer.concat([...archive(rows)]), oracle = String.raw`import sys,tarfile,io,json,hashlib
expected=json.loads(sys.argv[1]); seen=[]
with tarfile.open(fileobj=io.BytesIO(sys.stdin.buffer.read()),mode='r:') as archive:
 for entry in archive:
  data=archive.extractfile(entry).read(); seen.append({'path':entry.name,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),'executable':bool(entry.mode&0o111)}); assert entry.isfile()
assert seen==expected,(seen,expected)
print('[DEBUG] Independent Python tarfile PAX/unicode/empty/mode/hash oracle PASS')`;
      await run(["python3", "-c", oracle, JSON.stringify(rows.map(row => ({ path: row.path, bytes: row.bytes.length, sha256: digest(row.bytes), executable: Boolean(row.executable) })))], packed);
      const block = Buffer.alloc(256 * 1024, 91), bulk = [...Array.from({ length: 1024 }, (_, index) => ({ path: "stream/" + index, bytes: block })), { path: "stream/large-binary", bytes: Buffer.alloc(96 * 1024 ** 2, 93) }, { path: "stream/large-metadata", bytes: Buffer.alloc(32 * 1024 ** 2, 123) }];
      const streamOracle = String.raw`import sys,tarfile,hashlib,io,time
data=bytearray()
while chunk:=sys.stdin.buffer.read(65536): data.extend(chunk); time.sleep(0.00005)
assert len(data)==404230144,(len(data),404230144)
assert data[-1024:]==bytes(1024)
count=0; total=0
with tarfile.open(fileobj=io.BytesIO(data),mode='r:') as archive:
 for entry in archive:
  source=archive.extractfile(entry); size=0; digest=hashlib.sha256()
  while chunk:=source.read(65536): size+=len(chunk); digest.update(chunk)
  expected=('stream/'+str(count),262144,91) if count<1024 else ('stream/large-binary',100663296,93) if count==1024 else ('stream/large-metadata',33554432,123)
  assert (entry.name,size)==expected[:2] and digest.hexdigest()==hashlib.sha256(bytes([expected[2]])*expected[1]).hexdigest(),(count,entry.name,size)
  count+=1; total+=size
assert count==1026 and total==402653184,(count,total)
print('[DEBUG] Independent Python sustained PAX pipe oracle PASS '+str(total))`;
      await stream(["python3", "-c", streamOracle], archive(bulk));
      progress("sustained pipe", { method: "node Writable pipeline, completion and child close awaited", sourceBytes: 402653184 });
      progress("direct regression PASS", { vectors: vectors.valid.length + vectors.invalid.length, descriptor: "ordinary, oversize, symlink, changed cohort", independent: "Python tarfile + hashlib" }); rmSync(fixture, { recursive: true }); return;
    }
    if (mode === "imports") {
      const receipt = JSON.parse(readFileSync(join(generated, "private-source-capture-receipt.json"), "utf8"));
      const code = String.raw`const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto'),builtin=require('node:module').isBuiltin,hash=b=>crypto.createHash('sha256').update(b).digest('hex'),bytes=fs.readFileSync('/repo/.semio-private/source.json');if(hash(bytes)!==process.argv[1])throw Error('Frozen manifest identity mismatch');const m=JSON.parse(bytes),rows=new Map(m.files.map(row=>[row.path,row])),pending=[process.argv[2]],seen=new Set(),edges=[];while(pending.length){const name=pending.pop();if(seen.has(name))continue;seen.add(name);const row=rows.get(name);if(!row)throw Error('Import requires uncaptured Source '+name);const data=fs.readFileSync('/repo/'+name);if(hash(data)!==row.sha256)throw Error('Frozen imported Source changed '+name);if(name.endsWith('.json')){JSON.parse(data.toString());continue;}const text=data.toString().replace(/^#![^\r\n]*/u,''),imports=new Bun.Transpiler({loader:/\.[cm]?tsx?$/u.test(name)?'ts':'js',target:'bun',define:{'import.meta.vitest':'undefined'}}).scan(text).imports;for(const item of imports){let specifier=item.path;if(/[\u0080-\u00ff]/u.test(specifier)&&!/[\u0100-\uffff]/u.test(specifier)){const decoded=Buffer.from(specifier,'latin1').toString('utf8');if(Buffer.from(decoded).toString('latin1')===specifier)specifier=decoded;}if(specifier.startsWith('node:')||specifier.startsWith('bun:')||builtin(specifier))continue;const target=path.relative('/repo',Bun.resolveSync(specifier,path.dirname('/repo/'+name))).replaceAll('\\','/');if(target.startsWith('../')||target.split('/').some(part=>part==='node_modules'||part==='.cache')||!rows.has(target))throw Error('Frozen CLI imported Source not closed '+name+' -> '+specifier);edges.push([name,specifier,target]);pending.push(target);}}console.log('[DEBUG] frozen guest CLI import closure/hash proof '+JSON.stringify({files:seen.size,edges:edges.length,sha256:hash(JSON.stringify(edges.sort())),manifestSha256:hash(bytes),platform:process.platform,architecture:process.arch}));`;
      const output = await run(["docker", "run", "--rm", "--pull=never", "--platform", "linux/amd64", "--network=none", "--memory=768m", "--pids-limit=128", "--cpus=2", "--mount", "type=volume,source=" + receipt.volume + ",target=/repo,readonly", "--workdir", "/repo", receipt.image, "/repo/.semio-private/bun", "-e", code, receipt.manifestSha256, library + "/🏗️build/📜️script.ts"], undefined, true);
      console.log(output);
      const capturedBytes = readFileSync(join(generated, "private-source-capture-manifest.json"));
      if (digest(capturedBytes) !== receipt.manifestSha256) throw Error("Receipt source manifest identity mismatch");
      const captured = JSON.parse(capturedBytes.toString("utf8")), marker = "[DEBUG] frozen guest CLI import closure/hash proof ";
      const proof = JSON.parse(output.split("\n").find(line => line.startsWith(marker))!.slice(marker.length));
      writeFileSync(join(generated, "private-source-capture-receipt.json"), JSON.stringify({ ...receipt, atomicHostSnapshot: false, captureModel: "independently-frozen-owner-provider-chunk-epochs", epochs: captured.epochs.length, epochNegativePaths: captured.epochNegativePaths.length, laterHostDivergence: captured.laterHostDivergence, frozenCliImports: proof }, null, 2) + "\n");
      return;
    }
    if (!process.argv.includes("transport")) throw Error("Explicit capture transport mode required");
    const { inspectBunWorkspaceOwnership } = await imported(`${library}/🗂️workspaces/🟦️bun/🟦️.ts`);
    const ownership = async () => { let operations = 0; const result = await inspectBunWorkspaceOwnership(root, { kind: "all" }, { signal: stop.signal, advance: async () => { check(); if (++operations % 65536 === 0) progress("ownership", { operations }); } }); return JSON.stringify(result); };
    const beforeOwnership = await ownership();
    const roster = async () => { await run([process.execPath, import.meta.path, "source"]); return JSON.parse(readFileSync(join(generated, "private-source-complete-metadata.json"), "utf8")); };
    const before = await roster();
    if (before.files.length > maximumFiles || before.summary.bytes > maximumBytes || before.links.length || before.pathViolations.length) throw Error("Bounded ordinary Source roster required");
    const binary = read(ticket, "🗑️generated/builder-probe/bun", 256 * 1024 ** 2);
    if (binary.sha256 !== "a83d263767d839e4d2649ca8e35d07159c7afc99afdc96d731ced29e056dda0c") throw Error("Pinned official Linux Bun required");
    const captureMemoryBound = before.summary.bytes + before.summary.maximumFileBytes + binary.bytes.length + before.files.length * 8192 + 128 * 1024 ** 2;
    if (captureMemoryBound > maximumBytes) throw Error("Conservative private capture memory bound exceeded before reading Source");
    const files: (ReturnType<typeof read> & { epoch: number })[] = [], epochs: any[] = [], epochNegativePaths: { path: string; epoch: number; observedAt: string }[] = []; let total = 0;
    const captureGroup = async (name: string, paths: string[]) => {
      for (let attempt = 1; attempt <= 3; attempt++) {
        check(); const openedAt = new Date().toISOString();
        try {
          const rows: ReturnType<typeof read>[] = [], absent: string[] = []; let bytes = 0;
          for (const path of paths) { if (!lstatSync(join(root, path), { throwIfNoEntry: false })) { absent.push(path); continue; } const row = read(root, path, Math.min(512 * 1024 ** 2, maximumBytes - total - bytes)); rows.push(row); bytes += row.bytes.length; if (rows.length % 256 === 0) await Bun.sleep(0); }
          for (const row of rows) unchanged(root, row);
          for (const path of absent) if (lstatSync(join(root, path), { throwIfNoEntry: false })) throw Error("Negative Source observation changed: " + path);
          const epoch = epochs.length;
          epochs.push({ epoch, name, attempt, openedAt, sealedAt: new Date().toISOString(), files: rows.length, bytes, sha256: digest(JSON.stringify(rows.map(row => [row.path, row.sha256, row.identity]))) });
          epochNegativePaths.push(...absent.map(path => ({ path, epoch, observedAt: new Date().toISOString() })));
          files.push(...rows.map(row => ({ ...row, epoch }))); total += bytes;
          if (epochs.length % 16 === 0 || name === "manifest-provider-control") progress("epoch sealed", { epoch, name, files: files.length, bytes: total });
          return;
        } catch (error) { progress("epoch retry", { name, attempt, detail: String(error) }); if (attempt === 3) throw error; await Bun.sleep(0); }
      }
    };
    const prior = JSON.parse(readFileSync(join(generated, "private-source-metadata.json"), "utf8")), present = new Set<string>(before.files.map((row: any) => row.path));
    const critical = new Set<string>((Object.entries(prior.groups) as [string, string[]][]).filter(([name]) => name !== "conservative-domain-source-payload" && name !== "course-runtime-import-closure").flatMap(([, paths]) => paths).filter(path => present.has(path)));
    for (const path of before.cliPaths) critical.add(path);
    let sealedOwnership = beforeOwnership, registry: any;
    for (let attempt = 1; attempt <= 3; attempt++) {
      try {
        const currentRegistry = JSON.parse(read(root, "📋️build-owners.json", 16 * 1024 ** 2).bytes.toString("utf8"));
        for (const owner of currentRegistry.owners) critical.add(owner.manifest);
        for (const scope of JSON.parse(sealedOwnership).scopes) { critical.add(scope.manifest); for (const path of scope.packages) critical.add(path + "/package.json"); }
        await captureGroup("manifest-provider-control", [...critical].sort());
        const capturedCritical = new Set(files.map(row => row.path));
        for (const path of critical) if (!capturedCritical.has(path)) throw Error("Mandatory critical Source disappeared: " + path);
        const controlOwnership = await ownership();
        if (sealedOwnership !== controlOwnership) { sealedOwnership = controlOwnership; throw Error("Physical workspace ownership changed across manifest/provider epoch"); }
        for (const row of files) unchanged(root, row);
        registry = JSON.parse(files.find(row => row.path === "📋️build-owners.json")!.bytes.toString("utf8"));
        const missing = registry.owners.filter((owner: any) => !capturedCritical.has(owner.manifest));
        if (missing.length) { for (const owner of missing) critical.add(owner.manifest); throw Error("Registry dependency closure expanded: " + missing.map((owner: any) => owner.manifest).join(", ")); }
        break;
      } catch (error) { progress("critical cohort retry", { attempt, detail: String(error) }); files.length = 0; epochs.length = 0; epochNegativePaths.length = 0; total = 0; if (attempt === 3) throw error; await Bun.sleep(0); }
    }
    const boundaries = registry.owners.map((owner: any) => ({ name: owner.name, root: owner.root, manifest: owner.manifest })).sort((a: any, b: any) => b.root.length - a.root.length), boundaryRoots = new Map(boundaries.map((owner: any) => [owner.root, owner.name]));
    const boundary = (path: string): string => { for (let current = dirname(path); current !== "."; current = dirname(current)) if (boundaryRoots.has(current)) return boundaryRoots.get(current) as string; return "repository-source"; };
    const groups = new Map<string, string[]>();
    for (const row of before.files) if (!critical.has(row.path)) { const name = boundary(row.path), paths = groups.get(name) ?? []; paths.push(row.path); groups.set(name, paths); }
    for (const [name, paths] of groups) {
      let chunk: string[] = [], bytes = 0, part = 0;
      for (const path of paths) { const size = lstatSync(join(root, path), { throwIfNoEntry: false })?.size ?? 0; if (chunk.length && (chunk.length >= 128 || bytes + size > 32 * 1024 ** 2)) { await captureGroup(name + ":" + part++, chunk); chunk = []; bytes = 0; } chunk.push(path); bytes += size; }
      if (chunk.length) await captureGroup(name + ":" + part, chunk);
    }
    const after = await roster(), afterOwnership = await ownership(), captured = new Map(files.map(row => [row.path, row]));
    const divergence = { measuredAt: new Date().toISOString(), ownershipChanged: sealedOwnership !== afterOwnership, added: after.files.filter((row: any) => !captured.has(row.path)).map((row: any) => row.path), missing: files.filter(row => !existsSync(join(root, row.path))).map(row => row.path), changedIdentity: files.filter(row => { const facts = lstatSync(join(root, row.path), { bigint: true, throwIfNoEntry: false }); return !facts || identity(facts) !== row.identity; }).map(row => row.path), negativePaths: after.missing };
    writeFileSync(join(generated, "private-source-capture-host-divergence.json"), JSON.stringify(divergence, null, 2) + "\n");
    progress("later host divergence", { added: divergence.added.length, missing: divergence.missing.length, changedIdentity: divergence.changedIdentity.length, ownershipChanged: divergence.ownershipChanged });
    const manifest = { version: 1, capturedAt: new Date().toISOString(), authority: false, sourceSnapshot: true, atomicHostSnapshot: false, method: "independently frozen canonical-owner/provider epochs; per-epoch ordinary no-follow descriptor and before/after identity/hash fence, bounded retries; later host divergence recorded", image, root: "/repo", epochs, boundaries, boundaryMeaning: "Longest actual canonical owner package-root containment; finite repository Source chunks outside package roots. Transport boundaries do not substitute for unchanged guest managed admission.", files: files.map(({ path, bytes, sha256, identity, executable, epoch }) => ({ path, bytes: bytes.length, sha256, identity, executable, epoch, boundary: boundary(path) })), totalBytes: total, ownership: JSON.parse(sealedOwnership), ownershipSha256: digest(sealedOwnership), rosterSha256: digest(JSON.stringify(before.files)), negativePaths: before.missing, epochNegativePaths, laterHostDivergence: divergence, maximumBytes, maximumFiles };
    const manifestBytes = Buffer.from(JSON.stringify(manifest)), manifestSha256 = digest(manifestBytes);
    const overhead = files.length * 8192 + 64 * 1024 ** 2, projected = total + manifestBytes.length + binary.bytes.length + overhead;
    if (projected > maximumBytes) throw Error("Private volume conservative disk bound exceeded");
    writeFileSync(join(generated, "private-source-capture-manifest.json"), manifestBytes);
    progress("sealed capture", { files: files.length, bytes: total, manifestSha256, projected, maximumBytes });
    const verifiedImage = JSON.parse(await run(["docker", "image", "inspect", image], undefined, true))[0]; if (verifiedImage.Id !== image || verifiedImage.Os !== "linux" || verifiedImage.Architecture !== "amd64") throw Error("Exact prepared Linux image required");
    if (files.some(row => row.path === ".semio-private" || row.path.startsWith(".semio-private/"))) throw Error("Private transport metadata namespace collision");
    volume = "semio-course-source-" + manifestSha256.slice(0, 12) + "-" + randomUUID().slice(0, 8); container = volume + "-capture";
    await run(["docker", "volume", "create", "--label", "semio.ticket=teaching-architecture-deploy-ready", volume], undefined, true);
    const verify = String.raw`const fs=require('node:fs'),crypto=require('node:crypto'),h=b=>crypto.createHash('sha256').update(b).digest('hex'),bytes=fs.readFileSync('/repo/.semio-private/source.json');if(h(bytes)!==process.argv[1])throw Error('Source manifest digest mismatch');const m=JSON.parse(bytes);let total=0;for(const[index,row]of m.files.entries()){const p='/repo/'+row.path,s=fs.lstatSync(p);if(!s.isFile()||s.isSymbolicLink()||s.size!==row.bytes)throw Error('Guest Source kind/size mismatch '+row.path);const b=fs.readFileSync(p);if(h(b)!==row.sha256)throw Error('Guest Source digest mismatch '+row.path);total+=b.length;if(index%4096===0)console.log('[DEBUG] guest Source verified '+JSON.stringify({files:index+1,total}));}if(total!==m.totalBytes)throw Error('Guest byte count mismatch');const allocated=Number(require('node:child_process').execFileSync('du',['-sk','/repo'],{encoding:'utf8'}).split(/\s+/)[0])*1024;if(!Number.isSafeInteger(allocated)||allocated>m.maximumBytes)throw Error('Actual private Source volume bound exceeded');console.log('[DEBUG] guest Source complete hash proof '+JSON.stringify({files:m.files.length,bytes:total,allocatedBytes:allocated,manifestSha256:h(bytes),platform:process.platform,architecture:process.arch}));`;
    const quote = (text: string) => "'" + text.replaceAll("'", "'\\''") + "'";
    let streamed = 0;
    function* boundedArchive() { for (const chunk of archive([...files, { path: ".semio-private/source.json", bytes: manifestBytes }, { path: ".semio-private/bun", bytes: binary.bytes, executable: true }])) { check(); streamed += chunk.length; if (streamed > maximumBytes) throw Error("Private transport byte bound exceeded"); if (streamed % (64 * 1024 ** 2) < chunk.length) progress("stream", { bytes: streamed }); yield chunk; } }
    await stream(["docker", "run", "--rm", "-i", "--pull=never", "--platform", "linux/amd64", "--name", container, "--network=none", "--memory=768m", "--pids-limit=128", "--cpus=2", "--mount", "type=volume,source=" + volume + ",target=/repo", image, "sh", "-ec", "tar -xf - -C /repo; /repo/.semio-private/bun -e " + quote(verify) + " " + quote(manifestSha256) + "; du -sk /repo"], boundedArchive());
    const receipt = { version: 1, completedAt: new Date().toISOString(), image, volume, guestRoot: "/repo", sourceFiles: files.length, sourceBytes: total, manifestSha256, binarySha256: binary.sha256, streamedBytes: streamed, conservativeVolumeBytes: projected, maximumBytes, sourceSnapshot: true, atomicHostSnapshot: false, captureModel: "independently-frozen-owner-provider-chunk-epochs", epochs: epochs.length, hostSourceBinding: false, dependenciesRun: false, compilerRun: false, gitSupplement: "separate parent-owned stream required" };
    writeFileSync(join(generated, "private-source-capture-receipt.json"), JSON.stringify(receipt, null, 2) + "\n"); retained = true; progress("transport verified", receipt);
  } finally {
    clearTimeout(timer); process.off("SIGINT", cancel); process.off("SIGTERM", cancel); active?.kill(); streamingChild?.kill();
    if (container) { const child = Bun.spawn(["docker", "rm", "-f", container], { stdout: "ignore", stderr: "ignore" }); await child.exited; }
    if (volume && !retained) { const child = Bun.spawn(["docker", "volume", "rm", volume], { stdout: "ignore", stderr: "inherit" }); await child.exited; }
  }
}
