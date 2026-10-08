import assert from "node:assert/strict";
import { readFile, writeFile, mkdir, access, readdir } from "node:fs/promises";
import { createHash } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import { createRequire } from "node:module";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), [command, epoch] = process.argv.slice(2);
assert.ok(["verify", "target", "publish", "exclude", "metadata", "resolve", "resolve-all", "check"].includes(command)); assert.match(epoch ?? "", /^\d+$/);
const directory = join(ticket, "🗑️generated/s-native-ownership", command + "-" + epoch), producer = await readFile(import.meta.path, "utf8");
await mkdir(directory, { recursive: true });
const publicationPath = join(import.meta.dir, "📥️publication.json"), publicationSource = await readFile(publicationPath, "utf8"), publication = JSON.parse(publicationSource), lockSeedPath = join(import.meta.dir, "📥️lock-seed.toml"), lockSeed = await readFile(lockSeedPath, "utf8"), require = createRequire(join(root, "package.json")), parse = require("@iarna/toml").parse;
const policy = { budgetMs: 900000, maximumUnits: 8000000000, maximumOwnedBytes: command === "resolve-all" ? 33554432 : 16777216, maximumCaptureBytes: 268435456 }, started = performance.now(), controller = new AbortController(), stop = () => controller.abort(), runs: any[] = [], rows: any[] = [], nativeManifests: string[] = [];
process.once("SIGINT", stop); process.once("SIGTERM", stop);
let units = 0, capturedBytes = 0, code = 1, error: string | undefined;
const checkpoint = async () => { controller.signal.throwIfAborted(); assert.ok(performance.now() - started < policy.budgetMs); assert.ok(++units <= policy.maximumUnits); await new Promise<void>(accept => setImmediate(accept)); };
const exists = async (path: string) => { try { await access(path); return true; } catch (failure) { if ((failure as NodeJS.ErrnoException).code !== "ENOENT") throw failure; return false; } };
const capture = async () => {
  for (const path of [...new Set([...publication.rows.filter((row: any) => !row.path.endsWith("Cargo.lock")).map((row: any) => row.path), ...publication.originalMembers.map((row: any) => row.path), "Cargo.toml", "🧰️framework/Cargo.toml", "rust-toolchain.toml", "✏️s/🧪️tests/🗂️native-ownership/🟦️.ts", "✏️s/🧪️tests/🗂️native-ownership/🧫️fixtures/🔣️.json", "✏️s/🧪️tests/🗂️native-ownership/🧫️fixtures/🔣️schema.json"])] as string[]) {
    await checkpoint(); rows.push({ path, source: await exists(join(root, path)) ? await readFile(join(root, path), "utf8") : null });
  }
  for (const path of ["✏️s/🧪️tests/🗂️native-ownership/📜️script.ts", "✏️s/🧪️tests/🗂️native-ownership/📋️project.json"]) rows.push({ path, source: await readFile(join(root, path), "utf8") });
  const captured = new Set(rows.map(row => row.path)), omitted = new Set(["node_modules", "target", "dist", "🗑️generated"]);
  const visit = async (directory: string): Promise<void> => {
    for (const entry of await readdir(join(root, directory), { withFileTypes: true })) {
      await checkpoint(); if (omitted.has(entry.name)) continue; assert.ok(!entry.isSymbolicLink());
      const path = join(directory, entry.name);
      if (entry.isDirectory()) await visit(path);
      else if (entry.isFile() && (/\.(rs|json)$/.test(entry.name) || entry.name === "Cargo.toml") && !captured.has(path)) { rows.push({ path, source: await readFile(join(root, path), "utf8") }); captured.add(path); }
    }
  };
  await visit("✏️s/🔨️modules");
  const manifests = async (directory: string): Promise<void> => {
    for (const entry of await readdir(join(root, directory), { withFileTypes: true })) {
      await checkpoint(); if (["node_modules", "target", ".git", ".nx", "🗑️generated", ".venv", "__pycache__"].includes(entry.name)) continue; assert.ok(!entry.isSymbolicLink(), "Symbolic source entry: " + join(directory, entry.name));
      const path = join(directory, entry.name);
      if (entry.isDirectory()) await manifests(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") { nativeManifests.push(path); if (!captured.has(path)) { rows.push({ path, source: await readFile(join(root, path), "utf8") }); captured.add(path); } }
    }
  };
  await manifests("✏️s"); nativeManifests.sort();
};
const joinSources = async () => {
  const changes: any[] = [];
  for (const row of rows) { await checkpoint(); const after = await exists(join(root, row.path)) ? await readFile(join(root, row.path), "utf8") : null; if (after !== row.source) changes.push({ path: row.path, before: row.source, after }); }
  await writeFile(join(directory, "source-advances.json"), JSON.stringify({ changes, atomicSnapshotClaimed: false }));
  if (changes.length) throw Error("Source advanced in " + changes.length + " selected bodies: " + changes.map(row => row.path).join(", "));
  assert.equal(await readFile(import.meta.path, "utf8"), producer); assert.equal(await readFile(publicationPath, "utf8"), publicationSource); assert.equal(await readFile(lockSeedPath, "utf8"), lockSeed);
  const current: string[] = [], visit = async (directory: string): Promise<void> => {
    for (const entry of await readdir(join(root, directory), { withFileTypes: true })) {
      await checkpoint(); if (["node_modules", "target", ".git", ".nx", "🗑️generated", ".venv", "__pycache__"].includes(entry.name)) continue; assert.ok(!entry.isSymbolicLink(), "Symbolic source entry: " + join(directory, entry.name)); const path = join(directory, entry.name);
      if (entry.isDirectory()) await visit(path); else if (entry.isFile() && entry.name === "Cargo.toml") current.push(path);
    }
  };
  await visit("✏️s"); assert.deepEqual(current.sort(), nativeManifests, "Current S manifest roster advanced");
};
try {
  assert.equal(publication.version, 1);
  if (command === "publish" || command === "exclude") {
    const sourceRows = command === "publish" ? publication.rows : JSON.parse(await readFile(join(import.meta.dir, "📥️exclusions.json"), "utf8")).rows;
    if (command === "publish") for (const original of publication.originalMembers) assert.equal(await readFile(join(root, original.path), "utf8"), original.source);
    for (const row of sourceRows) { await checkpoint(); assert.equal(await exists(join(root, row.path)) ? await readFile(join(root, row.path), "utf8") : null, row.before, "Publication preimage advanced: " + row.path); if (row.path.endsWith("Cargo.toml")) assert.deepEqual(Bun.TOML.parse(row.after), parse(row.after)); }
    await writeFile(join(directory, "publication.json"), JSON.stringify({ phase: "prepared", rows: sourceRows, lockSeed, atomicBatchClaimed: false }));
    for (const row of sourceRows) { await checkpoint(); const path = join(root, row.path), after = row.afterSource ? lockSeed : row.after; assert.equal(await exists(path) ? await readFile(path, "utf8") : null, row.before); await mkdir(dirname(path), { recursive: true }); await writeFile(path, after); assert.equal(await readFile(path, "utf8"), after); }
    await writeFile(join(directory, "publication.json"), JSON.stringify({ phase: "completed", rows: sourceRows, lockSeed, atomicBatchClaimed: false }));
    console.log("[DEBUG] S actual native ownership " + command + " published rows=" + sourceRows.length + " preserved packages=" + publication.originalMembers.length);
  } else {
    await capture(); await writeFile(join(directory, "admission.json"), JSON.stringify({ producer, publicationSource, rows, policy, allPlatformsAccepted: false, atomicSnapshotClaimed: false }));
    const { MutationInventoryProcessWorkspace, runMutationInventoryProcess, readMutationInventoryCapture } = await import(join(root, "🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🟦️.ts"));
    const requests = command === "verify" ? [{ name: "law", executable: process.execPath, argv: [join(root, "✏️s/🧪️tests/🗂️native-ownership/📜️script.ts"), "test"] }] : command === "metadata" || command === "resolve-all" ? publication.rows.filter((row: any) => row.owner).map((row: any, index: number) => ({ name: "owner-" + index, executable: "cargo", argv: ["metadata", "--offline", ...(command === "metadata" ? ["--no-deps"] : []), "--format-version", "1", "--manifest-path", join(root, row.path)], owner: row.owner })) : command === "resolve" ? [{ name: "s-full-graph", executable: "cargo", argv: ["metadata", "--offline", "--format-version", "1", "--manifest-path", join(root, "✏️s/Cargo.toml")], owner: "✏️s" }] : [{ name: "s-all-native-targets", executable: "cargo", argv: ["check", "--offline", "--locked", "--workspace", "--all-targets", "--manifest-path", join(root, "✏️s/Cargo.toml")] }];
    if (command === "target") { requests.length = 0; requests.push({ name: "permanent-nx-law", executable: process.execPath, argv: [join(root, "node_modules/nx/dist/bin/nx.js"), "run", "@semio-tech/s-native-ownership:test", "--outputStyle=stream"] }); }
    const target = join(ticket, "🗑️generated/current-native-canonical-6/target");
    for (const request of requests) {
      await checkpoint(); const workspace = new MutationInventoryProcessWorkspace(), operation = { signal: controller.signal, maximumUnits: policy.maximumUnits - units, maximumOwnedBytes: policy.maximumOwnedBytes, workspace, onProgress: (value: any) => { if (value.stage === "spawn") console.log("[DEBUG] S native ownership " + request.name); }, yieldContinuation: () => new Promise<void>(accept => setImmediate(accept)) };
      const compiler = request.executable === "cargo" ? { buildDirectory: target, leaseDirectory: join(ticket, "🗑️generated/s-native-ownership/leases") } : null;
      const lockPath = command === "metadata" || command === "resolve-all" ? join(root, request.owner, "Cargo.lock") : join(root, "✏️s/Cargo.lock"), lockBefore = await readFile(lockPath, "utf8");
      const result = await runMutationInventoryProcess({ command: request.executable, argv: request.argv, cwd: root, environment: { ...process.env, ...(command === "target" ? { NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(directory, "nx/workspace-data"), NX_CACHE_DIRECTORY: join(directory, "nx/cache") } : {}), SEMIO_TEST_ARTIFACT_DIR: join(directory, request.name + "-command"), CARGO_TARGET_DIR: target, CARGO_BUILD_BUILD_DIR: target, CARGO_BUILD_JOBS: "2", CARGO_PROFILE_DEV_DEBUG: "0", CARGO_INCREMENTAL: "0", RUST_MIN_STACK: "134217728" }, captureDirectory: join(directory, request.name), budgetMs: Math.max(1, Math.floor(policy.budgetMs - (performance.now() - started))), maximumCaptureBytes: policy.maximumCaptureBytes - capturedBytes, compiler }, operation);
      const stdout = await readMutationInventoryCapture(result.stdout, operation), stderr = await readMutationInventoryCapture(result.stderr, operation);
      await writeFile(join(directory, request.name + ".stdout.txt"), stdout); await writeFile(join(directory, request.name + ".stderr.txt"), stderr);
      units += workspace.completed; capturedBytes += Buffer.byteLength(stdout) + Buffer.byteLength(stderr); const lockAfter = await readFile(lockPath, "utf8"), normalizationAuthorized = command === "resolve" || command === "resolve-all"; await writeFile(join(directory, request.name + ".lock-journal.json"), JSON.stringify({ path: lockPath, before: lockBefore, after: lockAfter, normalizationAuthorized })); if (!normalizationAuthorized) assert.equal(lockAfter, lockBefore); runs.push({ name: request.name, result, completed: workspace.completed });
      assert.equal(result.reason, "exit"); assert.equal(result.code, 0, request.name + " failed");
      if (command === "metadata") { const graph = JSON.parse(stdout); assert.equal(graph.workspace_root, join(root, request.owner)); const definition = Bun.TOML.parse(await readFile(join(root, request.owner, "Cargo.toml"), "utf8")) as any; assert.equal(graph.workspace_members.length, definition.workspace.members.length); const owned = new Set(definition.workspace.members.map((member: string) => join(root, request.owner, member, "Cargo.toml"))); for (const pkg of graph.packages) assert.ok(owned.has(pkg.manifest_path)); console.log("[DEBUG] Actual S child native owner=" + request.owner + " members=" + graph.workspace_members.length); }
      if (command === "resolve-all") { const graph = JSON.parse(stdout), selected = new Set(graph.workspace_members), definition = Bun.TOML.parse(await readFile(join(root, request.owner, "Cargo.toml"), "utf8")) as any; assert.equal(graph.workspace_root, join(root, request.owner)); assert.equal(selected.size, definition.workspace.members.length); const owned = new Set(definition.workspace.members.map((member: string) => join(root, request.owner, member, "Cargo.toml"))); for (const pkg of graph.packages) if (selected.has(pkg.id)) assert.ok(owned.has(pkg.manifest_path)); const before = Bun.TOML.parse(lockBefore) as any, after = Bun.TOML.parse(lockAfter) as any, key = (pkg: any) => [pkg.name, pkg.version, pkg.source].join("\u0000"), previous = new Map(before.package.filter((pkg: any) => pkg.source).map((pkg: any) => [key(pkg), pkg.checksum])); assert.deepEqual(after, parse(lockAfter)); for (const pkg of after.package.filter((pkg: any) => pkg.source)) assert.equal(previous.get(key(pkg)), pkg.checksum, "Foreign lock version/source/checksum advanced: " + pkg.name); console.log("[DEBUG] Full actual S native graph=" + request.owner + " packages=" + graph.packages.length + " owned=" + selected.size); }
      if (command === "resolve") { const graph = JSON.parse(stdout), members = new Set(graph.workspace_members); assert.equal(graph.workspace_root, join(root, "✏️s")); assert.equal(members.size, (Bun.TOML.parse(await readFile(join(root, "✏️s/Cargo.toml"), "utf8")) as any).workspace.members.length); for (const pkg of graph.packages) assert.ok(!pkg.manifest_path.startsWith(join(root, "✏️s/🔌️plugins") + "/") && !pkg.manifest_path.startsWith(join(root, "✏️s/🧑‍💻dev") + "/")); console.log("[DEBUG] Actual complete S Cargo graph packages=" + graph.packages.length + " neutral members=" + members.size); }
    }
    await joinSources();
  }
  assert.equal(await readFile(import.meta.path, "utf8"), producer); assert.equal(await readFile(publicationPath, "utf8"), publicationSource); code = 0;
} catch (failure) { error = String(failure); } finally {
  await writeFile(join(directory, "terminal.json"), JSON.stringify({ code, error, command, policy, runs, rows, producerSha256: createHash("sha256").update(producer).digest("hex"), units, capturedBytes, elapsedMs: performance.now() - started, sourceExact: code === 0 && command !== "publish", allPlatformsAccepted: false, pluginDeletionAccepted: false, atomicSnapshotClaimed: false }));
  if (error) console.error("[DEBUG] " + error); process.off("SIGINT", stop); process.off("SIGTERM", stop); process.exitCode = code;
}
