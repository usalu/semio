/** 🔍️ Cross-checks the build owner registry rows against the manifests, packages and paths of a workspace tree, and syncs them. */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { basename, dirname, join } from "node:path";

export interface OwnerRow { name: string; root: string; manifest: string; packages: string[]; nativeRoots?: string[] }
export interface Registry { version: number; owners: OwnerRow[]; catalogProviders: { ownerRoot: string; sourcePath: string; consumerRoots: string[] }[] }
export interface Finding { kind: string; owner?: string; path?: string; detail: string }
export interface Policy { collections: string[]; moduleMember: string; generated: string[] }

const PROJECT = "📋️project.json";
const LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const byteOrder = (left: string, right: string) => Buffer.compare(Buffer.from(left), Buffer.from(right));

/** 📜️ Reads the same example-collection and generated-directory policy the owner discovery applies. */
export function readPolicy(root: string): Policy {
  const ownership = JSON.parse(readFileSync(join(root, LIBRARY, "🗂️workspaces/🔣️policy.json"), "utf8")).collections;
  const caching = JSON.parse(readFileSync(join(root, LIBRARY, "⚡️caching/🔣️policy.json"), "utf8"));
  return { collections: ownership.names, moduleMember: ownership.moduleMember, generated: caching.generatedDirectories };
}

/** 🚫️ Mirrors the discovery exclusions: ticket tree, generated directories and example collections. */
export function excluded(path: string, policy: Policy): boolean {
  const parts = path.split("/");
  return path.startsWith(".🧬semio/") || parts.some((part, index) => policy.generated.includes(part) || (policy.collections.includes(part) && parts[index - 1] !== policy.moduleMember));
}

/** 📄️ Lists the working-tree files git knows (tracked and untracked, not ignored) that still exist. */
export function workspaceFiles(root: string, policy: Policy): string[] {
  const run = spawnSync("git", ["ls-files", "-z", "-c", "-o", "--exclude-standard"], { cwd: root, maxBuffer: 1 << 29 });
  if (run.status !== 0) throw Error("git ls-files failed");
  return run.stdout.toString("utf8").split("\0").filter((path) => path && !excluded(path, policy) && existsSync(join(root, path)));
}

/** 🦀️ Reads `[package] name` of a Cargo manifest, or undefined for virtual manifests. */
export function cargoPackageName(text: string): string | undefined {
  const section = /^\[package\]\s*$([\s\S]*?)(?=^\[|(?![\s\S]))/m.exec(text);
  return section ? /^name\s*=\s*"([^"]+)"/m.exec(section[1] ?? "")?.[1] : undefined;
}

const join2 = (root: string, name: string) => (root === "." ? name : `${root}/${name}`);

/** 🪪️ Reads the declared name of a manifest file, or undefined when unreadable. */
export function manifestName(root: string, manifest: string): string | undefined {
  try {
    const text = readFileSync(join(root, manifest), "utf8");
    return manifest.endsWith("Cargo.toml") ? cargoPackageName(text) : (JSON.parse(text) as { name?: string }).name;
  } catch { return undefined; }
}

/** 📦️ Reads the npm package name beside an owner root, if any. */
export function packageNameAt(root: string, directory: string): string | undefined {
  try { return (JSON.parse(readFileSync(join(root, join2(directory, "package.json")), "utf8")) as { name?: string }).name; } catch { return undefined; }
}

/** ✅️ Returns every inconsistency between registry rows and the tree. */
export function checkRegistry(root: string, registry: Registry, files: readonly string[], policy: Policy): Finding[] {
  const out: Finding[] = [];
  const add = (kind: string, detail: string, owner?: string, path?: string) => out.push({ kind, detail, owner, path });
  const fileSet = new Set(files);
  const names = new Map<string, string>();
  const covered = new Set<string>();
  for (const owner of registry.owners) {
    if (names.has(owner.name)) add("duplicate-name", `also at ${names.get(owner.name)}`, owner.name, owner.root);
    names.set(owner.name, owner.root);
    covered.add(owner.root);
    for (const native of owner.nativeRoots ?? []) {
      covered.add(native);
      if (!existsSync(join(root, native, "Cargo.toml"))) add("missing-native-root", `${native}/Cargo.toml absent`, owner.name, native);
    }
    if (!existsSync(join(root, owner.root))) { add("missing-root", "root directory absent", owner.name, owner.root); continue; }
    if (!existsSync(join(root, owner.manifest))) { add("missing-manifest", "manifest absent", owner.name, owner.manifest); continue; }
    if (owner.manifest.endsWith("Cargo.toml") && existsSync(join(root, join2(owner.root, PROJECT)))) add("cargo-row-shadows-project", `${PROJECT} beside the Cargo row is ignored by admission`, owner.name, join2(owner.root, PROJECT));
    if (excluded(owner.manifest, policy)) add("row-in-excluded-tree", "manifest sits in an excluded tree", owner.name, owner.manifest);
    const actual = manifestName(root, owner.manifest);
    if (actual !== owner.name) add("name-mismatch", `manifest declares ${actual ?? "(unreadable)"}`, owner.name, owner.manifest);
    const declared = packageNameAt(root, owner.root);
    if (declared && !owner.packages.includes(declared)) add("package-missing", `package.json declares ${declared}`, owner.name, join2(owner.root, "package.json"));
    for (const name of owner.packages) if (name !== declared) add("package-stale", `${name} not declared by package.json`, owner.name, join2(owner.root, "package.json"));
  }
  for (const provider of registry.catalogProviders) {
    if (!existsSync(join(root, provider.ownerRoot))) add("provider-owner-root", "absent", undefined, provider.ownerRoot);
    if (!existsSync(join(root, provider.sourcePath))) add("provider-source", "absent", undefined, provider.sourcePath);
    for (const consumer of provider.consumerRoots) if (!existsSync(join(root, consumer))) add("provider-consumer-root", "absent", undefined, consumer);
  }
  const projectDirs = new Set(files.filter((path) => basename(path) === PROJECT).map(dirname));
  for (const path of files) {
    const directory = dirname(path);
    if (basename(path) === PROJECT) {
      if (!registry.owners.some((owner) => owner.root === directory && owner.manifest === path)) add("project-without-row", "project manifest has no owner row", undefined, path);
    } else if (basename(path) === "Cargo.toml") {
      if (projectDirs.has(directory) || covered.has(directory) || cargoPackageName(readFileSync(join(root, path), "utf8")) === undefined) continue;
      add("cargo-without-row", "package manifest has no owner row or native root", undefined, path);
    }
  }
  return out;
}

/** 🔧️ Applies the findings to the registry: drops rows whose root and manifest are gone, adds missing rows, renames and re-lists packages. */
export function syncRegistry(root: string, registry: Registry, findings: readonly Finding[]): { registry: Registry; removed: string[]; added: string[]; changed: string[] } {
  const removed: string[] = [], added: string[] = [], changed: string[] = [];
  const gone = new Set(findings.filter((f) => f.kind === "missing-root" || f.kind === "missing-manifest").map((f) => f.owner));
  const owners = registry.owners.filter((owner) => { if (gone.has(owner.name)) { removed.push(owner.name); return false; } return true; });
  for (const owner of owners) {
    const manifest = join2(owner.root, PROJECT), name = manifestName(root, manifest);
    if (!owner.manifest.endsWith("Cargo.toml") || !name || owners.some((other) => other !== owner && other.name === name)) continue;
    owner.name = name; owner.manifest = manifest; changed.push(name);
  }
  for (const owner of owners) {
    const declared = packageNameAt(root, owner.root);
    const packages = declared ? [declared] : [];
    if (JSON.stringify(packages) !== JSON.stringify(owner.packages)) { owner.packages = packages; changed.push(owner.name); }
    const actual = manifestName(root, owner.manifest);
    if (actual && actual !== owner.name && !owners.some((other) => other.name === actual)) { owner.name = actual; changed.push(actual); }
  }
  for (const finding of findings) {
    if (finding.kind !== "project-without-row" && finding.kind !== "cargo-without-row") continue;
    const manifest = finding.path!, directory = dirname(manifest), name = manifestName(root, manifest);
    if (!name || owners.some((owner) => owner.name === name || owner.root === directory)) continue;
    const declared = packageNameAt(root, directory);
    const row: OwnerRow = { name, root: directory, manifest, packages: declared ? [declared] : [] };
    const at = owners.findIndex((owner) => byteOrder(owner.name, name) > 0);
    owners.splice(at < 0 ? owners.length : at, 0, row);
    added.push(name);
  }
  return { registry: { ...registry, owners }, removed, added, changed };
}

if (import.meta.main) {
  const root = process.argv[2] ?? process.cwd();
  const file = join(root, "📋️build-owners.json");
  const policy = readPolicy(root);
  const files = workspaceFiles(root, policy);
  const registry = JSON.parse(readFileSync(file, "utf8")) as Registry;
  let findings = checkRegistry(root, registry, files, policy);
  if (process.argv.includes("--fix")) {
    const fresh = JSON.parse(readFileSync(file, "utf8")) as Registry;
    const result = syncRegistry(root, fresh, checkRegistry(root, fresh, files, policy));
    writeFileSync(file, JSON.stringify(result.registry, null, 2) + "\n");
    console.log(JSON.stringify({ removed: result.removed, added: result.added, changed: result.changed }, null, 1));
    findings = checkRegistry(root, result.registry, files, policy);
  }
  const counts: Record<string, number> = {};
  for (const finding of findings) counts[finding.kind] = (counts[finding.kind] ?? 0) + 1;
  console.log(JSON.stringify({ owners: registry.owners.length, findings: findings.length, counts }, null, 1));
  if (process.argv.includes("--list")) for (const finding of findings) console.log(`${finding.kind}\t${finding.owner ?? ""}\t${finding.path ?? ""}\t${finding.detail}`);
  process.exit(findings.length ? 1 : 0);
}
