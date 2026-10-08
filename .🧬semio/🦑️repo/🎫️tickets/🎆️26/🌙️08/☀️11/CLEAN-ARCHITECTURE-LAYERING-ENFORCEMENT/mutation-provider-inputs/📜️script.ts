/** 🧾️ Executes the defining provider laws and conserves their selected physical source bodies. */
import assert from "node:assert/strict";
import { readFile, writeFile, mkdir } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";

const ticket = dirname(import.meta.dir), root = resolve(ticket, "../../../../../../.."), [command, epoch] = process.argv.slice(2);
const base = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers";
if (command === "worker") {
  assert.equal(process.argv.length, 3);
  const { runMutationInventoryProviderChecksV1 } = await import(join(root, base, "🧪️tests/🟦️.ts"));
  console.log(`[DEBUG] Defining mutation provider laws=${runMutationInventoryProviderChecksV1()}`);
} else {
  assert.ok(command === "law" || command === "target");
  assert.match(epoch ?? "", /^\d+$/u);
  const directory = join(ticket, "🗑️generated/mutation-provider", command + "-" + epoch), producer = await readFile(import.meta.path, "utf8");
  await mkdir(directory, { recursive: true });
  const paths = ["🟦️.ts", "🧬️schema/🔣️.json", "🧫️fixtures/🔣️schema.json", "🧫️fixtures/🔣️.json", "🧪️tests/🟦️.ts"].map(path => base + "/" + path);
  const rows = await Promise.all(paths.map(async path => ({ path, source: await readFile(join(root, path), "utf8") })));
  await writeFile(join(directory, "admission.json"), JSON.stringify({ producer, rows, atomicSnapshotClaimed: false, entireImportClosureClaimed: false }));
  const { executeCommandV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/🟦️.ts"));
  let cancelled = false, result: unknown, error: string | undefined;
  const stop = () => { cancelled = true; };
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  try {
    const args = command === "law" ? [import.meta.path, "worker"] : [join(root, "node_modules/nx/dist/bin/nx.js"), "run", "@semio-tech/repo-test-domain:inventory-provider-check", "--outputStyle=stream", "--excludeTaskDependencies"];
    const observed = await executeCommandV1({ version: 1, cwd: root, command: process.execPath, args, manifests: [], preparation: "none" }, {
      version: 1, context: { version: 1, cwd: root, cacheRoot: join(directory, "cache"), leaseDirectory: join(directory, "leases") }, budgetMs: 300000, maximumOutputBytes: 16777216,
      artifactDirectory: join(directory, "command"), retainArtifacts: true, cargoPolicies: [], vitestPolicy: null, cargoArtifactPolicy: null,
    }, { environment: { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false", NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(directory, "nx/workspace-data"), NX_CACHE_DIRECTORY: join(directory, "nx/cache"), SEMIO_TEST_ARTIFACT_DIR: join(directory, "receiver") }, cancelled: () => cancelled, onProgress: event => process.stderr.write(`[DEBUG] Mutation provider ${event.phase} elapsedMs=${event.elapsedMs}\n`) });
    result = observed; process.stdout.write(observed.stdout); process.stderr.write(observed.stderr);
    await writeFile(join(directory, "stdout.txt"), observed.stdout); await writeFile(join(directory, "stderr.txt"), observed.stderr);
    assert.equal(observed.reason, "exit"); assert.equal(observed.status, 0);
  } catch (failure) { error = String(failure); }
  finally {
    const joins = await Promise.all(rows.map(async row => ({ ...row, after: await readFile(join(root, row.path), "utf8") })));
    const sourceExact = joins.every(row => row.source === row.after) && producer === await readFile(import.meta.path, "utf8");
    await writeFile(join(directory, "terminal.json"), JSON.stringify({ command, result, error, sourceExact, rows: joins, producer, allPlatformsAccepted: false, deletionAccepted: false, entireImportClosureClaimed: false, atomicSnapshotClaimed: false }));
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (error || !sourceExact) { process.stderr.write(`[DEBUG] ${error ?? "Selected provider source advanced"}\n`); process.exitCode = 1; }
  }
}
