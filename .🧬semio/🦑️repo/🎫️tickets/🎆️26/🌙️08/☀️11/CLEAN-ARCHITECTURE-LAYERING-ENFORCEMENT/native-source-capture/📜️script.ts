import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, rmdirSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import { createHash, randomUUID } from "node:crypto";

const census = process.argv[2] === "census";
const planning = process.argv[2] === "projection-plan";
const materializing = process.argv[2] === "projection-materialize";
const root = resolve(process.argv[census || planning || materializing ? 3 : 2] ?? process.cwd());
const output = resolve(import.meta.dir, "../🗑️generated/goal-stdio");
if (planning) {
  if (process.argv.length !== 4) throw Error("projection-plan requires one current physical source root");
  physicalParents(root);
  const info = lstatSync(root);
  if (info.isSymbolicLink() || !info.isDirectory()) throw Error("Projection source root is not a physical directory");
  const contract = projectionContract();
  const paths = contract.projections.flatMap((row: { adapter?: string; feature?: string; manifest?: string }) => [row.adapter, row.feature, row.manifest].filter((path): path is string => typeof path === "string"));
  const inputs = paths.map((path: string) => {
    const full = resolve(root, path);
    if (relative(root, full).startsWith("..") || full === root) throw Error("Projection input escapes its source owner");
    const bytes = readPhysical(full);
    const sha256 = digest(bytes);
    if (!bytes.equals(readPhysical(full))) throw Error("Projection input changed during admission");
    return { path, bytes: bytes.length, sha256 };
  });
  console.log(JSON.stringify({ contract, inputs, materialized: false, nativeExecuted: false }));
  process.exit(0);
}
if (census) {
  const discovery = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts"));
  const rows = discovery.findPluginCargoFiles(root).map((manifest: string) => {
    const owner = discovery.parseComponentSourceOwnerV1(manifest, root);
    const project = JSON.parse(readFileSync(join(dirname(manifest), "📋️project.json"), "utf8"));
    if (typeof project.name !== "string" || !project.name) throw Error(`Missing actual registered project identity: ${manifest}`);
    return { ...owner, project: project.name, manifest: relative(root, manifest) };
  }).sort((left: { project: string }, right: { project: string }) => left.project.localeCompare(right.project));
  if (!rows.length || new Set(rows.map((row: { project: string }) => row.project)).size !== rows.length) throw Error("Actual deployed component census is empty or repeats a project");
  writeFileSync(join(output, "current-deployed-component-census.json"), JSON.stringify(rows, null, 2));
  console.log(JSON.stringify({ count: rows.length, projects: rows.map((row: { project: string }) => row.project) }));
  process.exit(0);
}
const captureId = randomUUID();
const projection = materializing ? projectionContract().projections.find((row: { id: string }) => row.id === process.argv[4]) : undefined;
if (materializing && (process.argv.length !== 5 || !projection)) throw Error("Unknown authored deletion projection");
const target = materializing ? join(output, "deletion-projections", projection.id + "-" + captureId) : join(output, "parent-avi-removal");
const roots = ["🧰️framework", "🌎️hub", "✏️s"];
const absent = materializing ? join(...projection.absent[0].split("/")) : join("✏️s", "🔌️plugins", "🗄️stdio", "🗿️artifacts", "📼️avi");
if (materializing && projection.absent.length !== 1) throw Error("The authored materializer requires one physical deletion owner");
const selectedInputs = materializing ? [projection.adapter, projection.feature, projection.manifest].filter((path): path is string => typeof path === "string").map(path => {
  const bytes = readPhysical(join(root, path));
  return { path, bytes: bytes.length, sha256: digest(bytes) };
}) : [];
const skip = new Set(["AGENTS.md", ".git", ".nx", ".venv", "node_modules", "target", "dist", "build", "storybook-static", "temp", "coverage", "🗑️generated", "__pycache__"]);
const report = { captureId, startedAt: new Date().toISOString(), completedAt: "", sourceRoot: root, targetRoot: target, projectionId: projection?.id ?? null, nativeExecuted: false, selectedInputs, roots, visited: 0, updated: [] as string[], currentAbsentRemoved: [] as string[], symlinksSkipped: [] as string[], absent, privateCompilerTargetRetained: !materializing, sourceWitnesses: [] as { path: string; bytes: number; sha256: string }[] };
const files = new Map<string, string>();
const directories = new Map<string, { children: string[]; sourceIno: number; sourceDev: number; destinationIno: number; destinationDev: number }>();
const extras = ["Cargo.toml", "Cargo.lock", "package.json", "bun.lock", "nx.json", "tsconfig.json", "📜️script.ts", ".cargo", ".config", ".vscode/🧩️launch.seed.jsonc", ...(materializing ? ["📋️project.json", "rust-toolchain.toml", "rustfmt.toml", "bunfig.toml"] : [])];
const extraPresence = extras.map(path => ({ path, present: physicalExists(join(root, path)) }));
if (materializing) {
  physicalParents(target);
  const workspace = Bun.TOML.parse(readPhysical(join(root, "Cargo.toml")).toString()) as { workspace?: { members?: unknown } };
  physicalParents(join(root, absent));
  const removed = lstatSync(join(root, absent));
  if (!Array.isArray(workspace.workspace?.members) || removed.isSymbolicLink() || !removed.isDirectory()) throw Error("Projection requires an actual Cargo workspace and current physical deleted-owner source");
  if (!physicalExists(join(root, "nx.json")) || physicalExists(target)) throw Error("Projection requires a current source workspace and a fresh caller-owned destination");
  mkdirSync(target, { recursive: true });
} else if (!existsSync(join(root, "nx.json")) || !existsSync(join(target, "nx.json")) || physicalExists(join(target, absent))) throw Error("Expected the existing physically AVI-absent copied workspace");
for (const path of [root, output, target]) if (lstatSync(path).isSymbolicLink() || !lstatSync(path).isDirectory()) throw Error(`Capture root is not a physical directory: ${path}`);

function excluded(path: string): boolean {
  return path === absent || path.startsWith(absent + sep);
}

function entries(path: string, base = root): string[] {
  physicalParents(join(base, path));
  const info = lstatSync(join(base, path));
  if (info.isSymbolicLink() || !info.isDirectory()) throw Error(`Capture inventory is not a physical directory: ${path}`);
  return readdirSync(join(base, path)).filter(entry => !skip.has(entry) && !excluded(join(path, entry))).sort();
}

function digest(bytes: Uint8Array): string {
  return createHash("sha256").update(bytes).digest("hex");
}

function projectionContract(): { projections: { id: string; absent: string[]; adapter?: string; feature?: string; manifest?: string }[] } {
  const contract = JSON.parse(readPhysical(join(import.meta.dir, "🧫️fixtures/🚮️projection/🔣️.json")).toString());
  const schema = JSON.parse(readPhysical(join(import.meta.dir, "🧬️schema/🚮️projection/🔣️.json")).toString());
  if (JSON.stringify(contract) !== JSON.stringify(schema.const)) throw Error("Projection contract differs from the closed authored schema");
  return contract;
}

function physicalExists(path: string): boolean {
  try { lstatSync(path); return true; } catch (error) { if ((error as { code?: string }).code === "ENOENT") return false; throw error; }
}

function physicalParents(path: string): void {
  for (let parent = dirname(path);; parent = dirname(parent)) {
    if (physicalExists(parent)) {
      const info = lstatSync(parent);
      if (info.isSymbolicLink() || !info.isDirectory()) throw Error(`Capture ancestor is not a physical directory: ${parent}`);
    }
    if (dirname(parent) === parent) break;
  }
}

function readPhysical(path: string): Buffer {
  physicalParents(path);
  const before = lstatSync(path);
  if (before.isSymbolicLink() || !before.isFile()) throw Error(`Capture file is not physical: ${path}`);
  const bytes = readFileSync(path), after = lstatSync(path);
  physicalParents(path);
  if (after.isSymbolicLink() || !after.isFile() || before.ino !== after.ino || before.dev !== after.dev || before.size !== after.size || before.mtimeMs !== after.mtimeMs || bytes.length !== after.size) throw Error(`Capture file changed during read: ${path}`);
  return bytes;
}

function capture(path: string): void {
  const source = join(root, path), destination = join(target, path);
  physicalParents(source);
  physicalParents(destination);
  const info = lstatSync(source);
  if (info.isSymbolicLink()) throw Error(`Capture source link is forbidden: ${path}`);
  if (info.isDirectory()) {
    if (physicalExists(destination) && (lstatSync(destination).isSymbolicLink() || !lstatSync(destination).isDirectory())) throw Error(`Copied source directory has conflicting ownership: ${path}`);
    mkdirSync(destination, { recursive: true });
    const children = entries(path);
    const copied = lstatSync(destination);
    directories.set(path, { children, sourceIno: info.ino, sourceDev: info.dev, destinationIno: copied.ino, destinationDev: copied.dev });
    for (const entry of children) capture(join(path, entry));
    return;
  }
  if (!info.isFile()) return;
  report.visited++;
  const bytes = readPhysical(source), witness = digest(bytes);
  files.set(path, witness);
  report.sourceWitnesses.push({ path, bytes: bytes.length, sha256: witness });
  if (report.visited % 2000 === 0) console.log(JSON.stringify({ captureId: report.captureId, phase: "copy", visited: report.visited }));
  if (physicalExists(destination)) {
    if (bytes.equals(readPhysical(destination))) {
      if (!bytes.equals(readPhysical(source))) throw Error(`Source changed during capture: ${path}`);
      return;
    }
  }
  mkdirSync(dirname(destination), { recursive: true });
  physicalParents(destination);
  writeFileSync(destination, bytes);
  if (!bytes.equals(readPhysical(source))) throw Error(`Source changed during capture: ${path}`);
  report.updated.push(path);
}

function removeAbsent(path: string): void {
  const destination = join(target, path), source = join(root, path), info = lstatSync(destination);
  physicalParents(source);
  physicalParents(destination);
  if (info.isSymbolicLink()) throw Error(`Capture destination link is forbidden: ${path}`);
  if (physicalExists(source) && lstatSync(source).isSymbolicLink()) throw Error(`Capture source link is forbidden during pruning: ${path}`);
  if (info.isDirectory()) {
    for (const entry of readdirSync(destination)) if (!skip.has(entry) && !excluded(join(path, entry))) removeAbsent(join(path, entry));
    if (!physicalExists(source)) {
      if (readdirSync(destination).length) throw Error(`Obsolete copied directory retains excluded storage: ${path}`);
      rmdirSync(destination);
    }
  } else if (info.isFile() && (!physicalExists(source) || !lstatSync(source).isFile())) {
    unlinkSync(destination);
    report.currentAbsentRemoved.push(path);
  }
}

for (const path of roots) { capture(path); removeAbsent(path); }
for (const { path, present } of extraPresence) {
  if (present) capture(path);
  if (physicalExists(join(target, path))) removeAbsent(path);
}
for (const [path, witness] of directories) {
  const source = lstatSync(join(root, path)), destination = lstatSync(join(target, path));
  if (source.ino !== witness.sourceIno || source.dev !== witness.sourceDev || destination.ino !== witness.destinationIno || destination.dev !== witness.destinationDev || JSON.stringify(entries(path)) !== JSON.stringify(witness.children) || JSON.stringify(entries(path, target)) !== JSON.stringify(witness.children)) throw Error(`Capture directory inventory or identity changed: ${path}`);
}
for (const { path, present } of extraPresence) if (physicalExists(join(root, path)) !== present || physicalExists(join(target, path)) !== present) throw Error(`Capture selected input presence changed: ${path}`);
let verified = 0;
for (const [path, witness] of files) {
  if (digest(readPhysical(join(root, path))) !== witness || digest(readPhysical(join(target, path))) !== witness) throw Error(`Capture final file witness differs: ${path}`);
  if (++verified % 2000 === 0) console.log(JSON.stringify({ captureId: report.captureId, phase: "verify", verified }));
}
for (const input of selectedInputs) if (files.get(join(...input.path.split("/"))) !== input.sha256 || digest(readPhysical(join(root, input.path))) !== input.sha256 || digest(readPhysical(join(target, input.path))) !== input.sha256) throw Error("Projection selected source changed or was omitted");
if (physicalExists(join(target, absent)) || relative(output, target).startsWith("..")) throw Error("Copied workspace containment or physical deletion changed");
report.completedAt = new Date().toISOString();
writeFileSync(join(output, materializing ? "deletion-projection-source-capture-" + projection.id + "-" + captureId + ".json" : "retained-current-coherent-source-capture.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify({ captureId, targetRoot: target, projectionId: projection?.id ?? null, nativeExecuted: false, visited: report.visited, updated: report.updated.length, currentAbsentRemoved: report.currentAbsentRemoved.length, symlinksSkipped: report.symlinksSkipped.length, absent }));
