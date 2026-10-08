import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock")) || !existsSync(join(root, "📋️project.json"))) { const parent = dirname(root); if (parent === root) throw Error("Repository root missing"); root = parent; }
const command = process.argv[2], rest = process.argv.slice(3);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const kernel = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const repositoryEnv = { ...process.env, NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, REPO_ROOT: root };
if (command === "test-owner-command" && (rest[1] === "registry-guarded" || rest[0] === "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" || rest[0] === "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts")) {
  if (!repositoryEnv.SEMIO_TEST_ARTIFACT_DIR) throw Error("Explicit ticket artifact storage required for root Nx publication");
  repositoryEnv.NX_WORKSPACE_DATA_DIRECTORY = join(repositoryEnv.SEMIO_TEST_ARTIFACT_DIR, "oct7-plugin-root-nx");
}
const uiOwner = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts";
if (command === "test-owner-command" && [uiOwner, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts", "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts"].includes(rest[0]!) && !repositoryEnv.SEMIO_VITEST_POLICY) {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket artifact storage required for UI verification");
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd: dirname(join(root, rest[0]!)), toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "ui-vitest-cache"), coverageDirectory: join(artifactRoot, "ui-vitest-coverage"), budgetMs: 300_000 });
}

if (command === "root-service-oracles" || command === "root-service-current") {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket artifact storage required for service verification");
  const owner = join(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript");
  const policy = { version: 1 as const, cwd: owner, toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "service-vitest-cache"), coverageDirectory: join(artifactRoot, "service-vitest-coverage"), budgetMs: 300_000 };
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify(policy);
  const { runVitestV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"));
  const args = command === "root-service-current" ? [] : ["--testNamePattern", "retains the preceding inference job when a successor opening is refused|refreshes Shell session authority serially and cancels stale callbacks|retains the exact bootstrap owner across hash completion and progress callbacks"];
  await runVitestV1(policy, args, "../../💡️inference/🧪️tests/🎚️config/🟦️.ts", repositoryEnv);
  process.exit(0);
}

if (command === "root-current-corpus-tests") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts",
    "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts",
    "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/🧪️tests/🧩️conformance/🟦️.ts",
  ];
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts"), "-t", "borrowed clone authority|paged native ownership"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-story-inspect") {
  const { readFileSync } = await import("node:fs"), { relative, resolve } = await import("node:path");
  const { default: fg } = await import("fast-glob"), { loadCsf } = await import("storybook/internal/csf-tools");
  const scopes = await import(join(root, ".storybook/📖️stories/🧭️coordination/🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(root, library, "🧫️fixtures/🧫️storybook-discovery/🔣️.json"), "utf8"));
  const paths = (await fg(scopes.buildScopeStoryGlobs(scopes.resolveActiveScopes("")), { cwd: join(root, ".storybook"), onlyFiles: true, unique: true })).map(path => relative(root, resolve(root, ".storybook", path)).replaceAll("\\", "/")).sort();
  for (const path of paths) {
    const parsed = loadCsf(readFileSync(join(root, path), "utf8"), { fileName: path, makeTitle: title => title }).parse();
    const row = { path, title: parsed.meta.title, stories: Object.entries(parsed._stories).map(([exportName, story]) => ({ exportName, id: story.id, name: story.name })) };
    if (JSON.stringify(row) !== JSON.stringify(fixture.stories.find(entry => entry.path === path))) console.log("[DEBUG] story expectation " + JSON.stringify(row));
  }
  console.log("[DEBUG] retired story expectations " + JSON.stringify(fixture.stories.filter(row => !paths.includes(row.path)).map(row => row.path)));
  process.exit(0);
}

if (command === "root-node-oracles") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const groups = [
    ["🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/💬️capability-description/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🎚️config/🟦️.ts"],
    [".storybook/🧪️tests/🧪️scope-resolution/🟦️.ts", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"],
  ];
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Caller-owned test output required");
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd: dirname(join(root, uiOwner)), toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "vitest-cache"), coverageDirectory: join(artifactRoot, "coverage"), budgetMs: 120_000 });
  for (const [path, config] of groups) await runBudgetedTestCommand("node", [join(root, "node_modules/vitest/vitest.mjs"), "run", join(root, path), "--config", join(root, config)], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-store-scalar-wide") {
  const child = Bun.spawn([process.env.SEMIO_PYTHON_EXECUTABLE ?? (process.platform === "win32" ? "python" : "python3"), "-c", "from pathlib import Path\nimport json,sys\nfrom jsonschema import Draft7Validator\nroot=Path(sys.argv[1])\nstore=root/\"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store\"\nschema=json.loads((store/\"🧪️testing/🧬️schema/🔣️.json\").read_text())\nexamples=json.loads((store/\"🧪️testing/🧫️fixtures/🔢️payload-contracts/🔣️.json\").read_text())\nDraft7Validator.check_schema(schema)\ncount=0\nfor row in examples[\"scalars\"]:\n value={\"$schema\":schema[\"$schema\"],\"$defs\":schema[\"$defs\"],\"$ref\":\"#/$defs/\"+row[\"export\"]}\n assert Draft7Validator(value).is_valid(row[\"value\"])==row[\"accepted\"],row\n count+=1\nfor row in examples[\"wideUnsigned\"]:\n value={\"$schema\":schema[\"$schema\"],\"$defs\":schema[\"$defs\"],\"$ref\":\"#/$defs/U64\"}\n assert Draft7Validator(value).is_valid(json.loads(row[\"wire\"]))==row[\"accepted\"],row\n count+=1\nprint(\"[DEBUG] actual jsonschema scalar verdicts=\"+str(count)+\"; exact native u64 maximum=\"+str(schema[\"$defs\"][\"U64\"][\"maximum\"]))\n", root], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  process.exit(await child.exited);
}

if (command === "parser-python") {
  const child = Bun.spawn([process.env.SEMIO_PYTHON_EXECUTABLE ?? (process.platform === "win32" ? "python" : "python3"), join(root, "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🐍️.py")], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
  process.exit(await child.exited);
}

if (command === "draft07-oracle") {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket artifact storage required for draft-07 verification");
  process.env.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd: root, toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "draft07-vitest-cache"), coverageDirectory: join(artifactRoot, "draft07-vitest-coverage"), budgetMs: 300_000 });
  const { runVitest } = await import(join(root, library, "📦️packages/🟦️typescript/🟦️.ts"));
  await runVitest(root, ["🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts"], "🧰️framework/🔨️modules/🧬️schema/🧪️tests/🎚️config/🟦️.ts");
  process.exit(0);
}

if (command === "parser-input-oracles") {
  const subset = await import(join(root, "🧰️framework/🔨️modules/🧬️schema/✅️validator/🧪️tests/🟦️.ts"));
  console.log("[DEBUG] schema subset Ajv/Node proofs=" + await subset.proveJsonSchemaSubsetContractV1());
  const ownership = await import(join(root, library, "📏️ownership/🏛️abstraction/✅️verification/🟦️.ts"));
  console.log("[DEBUG] actual abstraction ownership proofs=" + ownership.abstractionOwnershipChecks(root));
  process.exit(0);
}

if (command === "kernel-corpus") {
  const failed: string[] = [];
  for (const law of ["paged-history-stack-check", "wal-recovery-check", "wal-capacity-check", "wal-committed-transactions-check", "wal-writer-authority-check", "database-history-completion-check", "database-catalog-read-ownership-check", "database-capability-completion-check", "wal-committed-compaction-check", "database-shutdown-check", "document-mount-single-flight-check", "durable-owned-group-decision-check", "durable-group-journal-check", "retained-clone-check", "directory-event-page-bootstrap-check"]) {
    console.log("[DEBUG] kernel behavior oracle " + law);
    const child = Bun.spawn([process.execPath, join(root, kernel, "📜️script.ts"), law], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
    const stop = () => child.kill("SIGTERM");
    process.once("SIGINT", stop); process.once("SIGTERM", stop);
    const code = await child.exited;
    process.off("SIGINT", stop); process.off("SIGTERM", stop);
    if (code) failed.push(law);
  }
  console.log("[DEBUG] kernel behavior oracle failures=" + JSON.stringify(failed));
  process.exit(failed.length ? 1 : 0);
}
const args = command === "test-owner-command" ? [join(root, rest[0]!), ...rest.slice(1)] : command === "owner-command" ? [join(root, library, "⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", join(dirname(rest[0]!), "Cargo.toml"), "--cwd", dirname(rest[0]!), "--", process.execPath, join(root, rest[0]!), ...rest.slice(1)] : command === "structural-catalog-test" ? ["test", join(root, library, "🧪️tests/🔬️workspace-contract/🟦️.ts"), "-t", "schema scope catalog"] : command === "kernel-test" ? [join(root, kernel, "📜️script.ts"), "test", "os_store::tests::", "--", "--nocapture"] : command === "kernel-command" ? [join(root, library, "⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", join(kernel, "Cargo.toml"), "--cwd", kernel, "--", process.execPath, join(root, kernel, "📜️script.ts"), ...rest] : command === "schema" ? [join(root, "📜️script.ts"), "schema", ...rest] : command === "verify" ? [join(root, "📜️script.ts"), "verify", ...rest] : command === "tests" ? ["test", ...rest.map(path => join(root, path))] : command === "parity" ? [join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts"), "parity", "quick", "--owner", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", "--case", "🖥️host-protocol-parity"] : null;
if (!args) throw Error("Unknown verification command");
console.log("[DEBUG] isolated Nx verification " + command);
const child = Bun.spawn([process.execPath, ...args], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
const interrupt = () => child.kill("SIGINT"), terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
process.exitCode = await child.exited;
