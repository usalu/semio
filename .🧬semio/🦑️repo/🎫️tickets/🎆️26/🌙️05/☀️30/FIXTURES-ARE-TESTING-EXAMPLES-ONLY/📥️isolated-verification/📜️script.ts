import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
let root = import.meta.dir;
while (!existsSync(join(root, "bun.lock")) || !existsSync(join(root, "📋️project.json"))) { const parent = dirname(root); if (parent === root) throw Error("Repository root missing"); root = parent; }
const command = process.argv[2], rest = process.argv.slice(3);
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const kernel = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const repositoryEnv = { ...process.env, NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, REPO_ROOT: root };
if (command === "test-owner-command" && (rest[1] === "registry-guarded" || rest[0] === "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts" || rest[0] === "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts" || rest[0] === "🌎️hub/📦️packages/🦀️rust/📜️script.ts")) {
  if (!repositoryEnv.SEMIO_TEST_ARTIFACT_DIR) throw Error("Explicit ticket artifact storage required for root Nx publication");
  repositoryEnv.NX_WORKSPACE_DATA_DIRECTORY = join(repositoryEnv.SEMIO_TEST_ARTIFACT_DIR, "oct7-plugin-root-nx");
}
const uiOwner = "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts";
if (command === "test-owner-command" && [uiOwner, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts", "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts"].includes(rest[0]!) && !repositoryEnv.SEMIO_VITEST_POLICY) {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket artifact storage required for UI verification");
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify({ version: 1, cwd: dirname(join(root, rest[0]!)), toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "ui-vitest-cache"), coverageDirectory: join(artifactRoot, "ui-vitest-coverage"), budgetMs: 300_000 });
}

if (command === "root-corpus-port-boot-source") {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket output required for Surface and boot verification");
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🗺️surface/🧪️tests/🧩️suite/🟦️.ts"), "-t", "surface (public refusal|vector tile|neutral interaction)"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  const owner = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript");
  const policy = { version: 1 as const, cwd: owner, toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "surface-boot-vitest-cache"), coverageDirectory: join(artifactRoot, "surface-boot-vitest-coverage"), budgetMs: 300_000 };
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify(policy);
  const { runVitestV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"));
  await runVitestV1(policy, ["🧪️tests/🧩️package-integration/🟦️.ts", "--testNamePattern", ["hands the shell its injected boot plan in dependency order", "resolves injected artifact activation with an independent Map oracle", "validates the current package catalog with Ajv and independent WebCrypto integrity vectors", "validates explicit browser entry identities against neutral vectors and independent Ajv/emoji parsing"].join("|")], "../../🧪️tests/🎚️config/🟦️.ts", repositoryEnv);
  process.exit(0);
}

if (command === "root-reintroduced-retirement-source") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts"), join(root, "🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🧱️ownership/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, [join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-async-dsl-ghosts") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  for (const stage of ["worker-maintenance-check", "worker-parking-check", "worker-deferred-wake-check", "worker-pool-use-check"]) await runBudgetedTestCommand(process.execPath, [join(root, "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts"), stage], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, [join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🚪️diff-codecs/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-play-pane-ghost") {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket output required for Play verification");
  const owner = join(root, "🏢️semio-tech/🎡️play");
  const policy = { version: 1 as const, cwd: owner, toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "play-vitest-cache"), coverageDirectory: join(artifactRoot, "play-vitest-coverage"), budgetMs: 300_000 };
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify(policy);
  const { runVitestV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"));
  await runVitestV1(policy, ["--testNamePattern", "retains every pane's editor and viewer acceptance flow"], "🧪️tests/🎚️config/🟦️.ts", repositoryEnv);
  process.exit(0);
}

if (command === "root-admin-graph" || command === "root-schema-harness") {
  if (command === "root-admin-graph") {
    const { verifyAdminEntryGraph, verifyAdminStylesheetGraph } = await import(join(root, "🌎️hub/🔨️modules/🛡️admin/🧪️tests/🕸️build-graph/🟦️.ts"));
    const owner = join(root, "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript");
    verifyAdminEntryGraph(owner);
    verifyAdminStylesheetGraph(owner);
    console.log("[DEBUG] actual Hub admin HTML/package entry and CSS dependency/source graph assertions passed");
  } else {
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧬️schema-invariants/🟦️.ts"), ...(rest[0] === "--full" ? [] : ["-t", "parity with the catalog generator"])], { cwd: root, env: repositoryEnv, budgetMs: 180_000, throwOnFailure: true });
  }
  process.exit(0);
}

if (command === "root-final-corpus") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [
  "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🏷️symbols/🎮️decode/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts",
  "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/📋️native-owner/🟦️.ts",
  "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/💾️binary/🧬️mutations/🫳️borrowed/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/📡️replication/🔗️causal/📦️slots/🧪️tests/🟦️.ts",
  "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧊️feature-ownership/🟦️.ts",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️unit/🟦️.ts"
];
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 180_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-sequence-oracles") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  for (const path of [
    "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js",
    "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js",
  ]) await runBudgetedTestCommand(process.execPath, [join(root, path)], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  console.log("[DEBUG] actual Sequence retained-route/dagre and browser binary-protocol oracles executed");
  process.exit(0);
}

if (command === "root-remaining-corpus") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [
    "🏢️semio-tech/🎡️play/🧪️tests/🧪️playrelease/🟦️.ts",
    "🧰️framework/🔨️modules/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/📏️retirement/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🧪️tests/📬️test-body/🟦️.ts",
  ];
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map(path => join(root, path))], { cwd: root, env: { ...repositoryEnv, SEMIO_TICKET_DIR: dirname(import.meta.dir) }, budgetMs: 180_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts"), "-t", "recursive structural depth"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}

if (command === "root-core-current") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const paths = [
    "🧰️framework/🔨️modules/🧵️job/🧪️tests/📏️close-demand/🟦️.ts",
    "🧰️framework/🔨️modules/🧵️job/⏱️context/📦️owner/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/🧪️tests/📏️close/🟦️.ts",
    "🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list/🟦️.ts",
    "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🟦️.ts",
    "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🟦️.ts",
  ];
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts"), "-t", "Cargo retains failed build stdout"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🧪️tests/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  const job = await import(join(root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🟦️.ts"));
  job.testJobPayloadPhysicalClose();
  const panel = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts"));
  panel.testHostPanelStateSchema();
  console.log("[DEBUG] actual exported Job physical-close and Shell panel-state behavior laws passed");
  process.exit(0);
}

if (command === "root-service-oracles" || command === "root-service-current") {
  const artifactRoot = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Explicit ticket artifact storage required for service verification");
  const owner = join(root, "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript");
  const policy = { version: 1 as const, cwd: owner, toolPath: join(root, "node_modules/vitest/vitest.mjs"), runtime: "node", coverageRuntime: "node", cacheRoot: join(artifactRoot, "service-vitest-cache"), coverageDirectory: join(artifactRoot, "service-vitest-coverage"), budgetMs: 300_000 };
  repositoryEnv.SEMIO_VITEST_POLICY = JSON.stringify(policy);
  const { runVitestV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🧪️vitest/🟦️.ts"));
  const args = command === "root-service-current" ? rest : ["--testNamePattern", "retains the preceding inference job when a successor opening is refused|refreshes Shell session authority serially and cancels stale callbacks|retains the exact bootstrap owner across hash completion and progress callbacks"];
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
if (command === "runtime-store-corpus-source") {
  const { readFileSync } = await import("node:fs");
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  const ticket = dirname(import.meta.dir);
  const codec = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts"));
  codec.testNativeCodecSendFixture();
  const retirement = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts"));
  retirement.testSnapshotReadRetirement();
  const visibility = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/👁️group-visibility/🟦️.ts"));
  visibility.testGroupVisibilityFixtures();
  console.log("[DEBUG] actual Store codec, snapshot-read-retirement and group-visibility helpers completed");
  const ledger = JSON.parse(readFileSync(join(ticket, "📥️oct8-runtime-store-corpus-ledger.json"), "utf8"));
  const paths = [...ledger.updated, ...(ledger.tests ?? [])].filter((path: string) => !path.includes("📦️native-codec-send") && !path.includes("♻️snapshot-read-retirement") && !path.includes("👁️group-visibility"));
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map((path: string) => join(root, path))], { cwd: root, env: { ...repositoryEnv, SEMIO_TICKET_DIR: ticket }, budgetMs: 300_000, throwOnFailure: true });
  process.exit(0);
}
if (command === "root-canonical-process-preview") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, library, "🧪️tests/🔬️workspace-contract/🟦️.ts"), "-t", "routes native generator previews through their declared owner"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  const { default: assert } = await import("node:assert/strict");
  const { readFileSync } = await import("node:fs");
  const { default: parseArgv } = await import("yargs-parser");
  const { loadTaxonomy, generatorPreviewExecution } = await import(join(root, library, "🔍️discovery/🟦️.ts"));
  const contract = loadTaxonomy().generatorContracts!["framework-manifest"]!;
  const project = JSON.parse(readFileSync(join(root, contract.ownerPath!, "📋️project.json"), "utf8"));
  const target = project.targets[contract.previewTarget!.slice(contract.previewTarget!.lastIndexOf(":") + 1)];
  const route = generatorPreviewExecution(contract, target);
  assert.equal(route.cwd, "🧰️framework");
  assert.deepEqual(parseArgv([route.command, ...route.args], { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }), parseArgv(target.options.command, { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } }));
  console.log("[DEBUG] actual current Framework Process preview metadata admitted with exact config/package/manifest/script argv; full taxonomy inventory retained");
  process.exit(0);
}
if (command === "plugin-oct8-protected-sweep-tests") {
  const { testProtectedActorPublicationV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🗂️hub-document-sweep/🧾️publication/🧪️tests/🟦️.ts"));
  await testProtectedActorPublicationV1(root);
  console.log("[DEBUG] actual protected actor publication neutral join/refusal laws completed");
  process.exit(0);
}
if (command === "plugin-oct8-hub-provenance-tests") {
  const output = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Explicit ticket output required for Hub provenance verification");
  const failures: unknown[] = [];
  const operations = [
    async () => { const { createFreshComponentTests } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧪️tests/🆕️fresh-component/🟦️.ts")); await createFreshComponentTests().testFreshComponentProcessV1(root); },
    async () => { const { testClosedBrowserActorProducerV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts")); await testClosedBrowserActorProducerV1(root); },
    async () => { const { createTrustedCatalogProvenanceTests } = await import(join(root, "🌎️hub/🏗️bootstrap/🧾️provenance/🧪️tests/🟦️.ts")); await createTrustedCatalogProvenanceTests().testTrustedCatalogProvenanceV1(root); },
  ];
  for (const operation of operations) try { await operation(); } catch (error) { failures.push(error); }
  if (failures.length) throw new AggregateError(failures, "Actual Hub production provenance laws failed");
  console.log("[DEBUG] actual fresh component, closed actor and trusted catalog provenance laws completed");
  process.exit(0);
}
if (command === "mounted-runtime-http") {
  const { createServer } = await import("node:net");
  const { readFileSync, writeFileSync, mkdirSync, realpathSync } = await import("node:fs");
  const { isAbsolute, relative, resolve } = await import("node:path");
  const { createHash } = await import("node:crypto");
  const { default: assert } = await import("node:assert/strict");
  const { ensureDevServe } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts"));
  const { PLAYGROUND_BUILD_TARGETS } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts"));
  const { runtimeFixturePathV1 } = await import(join(root, library, "🔍️discovery/🕸️runtime/🟦️.ts"));
  const output = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!output || rest.length !== 1) throw Error("mounted-runtime-http needs one retained actual mount input and explicit ticket output");
  const plan = JSON.parse(readFileSync(resolve(root, rest[0]!), "utf8"));
  const contexts = Array.isArray(plan) ? plan : [plan];
  assert(contexts.length > 0);
  assert.equal(new Set(contexts.map(input => `${input.variant}/${input.renderer}/${input.profile}`)).size, contexts.length, "Mounted contexts must have distinct receipt owners");
  for (const input of contexts) {
  assert(["react", "wgpu"].includes(input.renderer) && ["dev", "release"].includes(input.profile) && PLAYGROUND_BUILD_TARGETS.some(row => row.variant === input.variant) && Array.isArray(input.mounts) && input.mounts.length > 0);
  const allocation = createServer();
  await new Promise<void>((resolve, reject) => { allocation.once("error", reject); allocation.listen(0, "127.0.0.1", resolve); });
  const port = (allocation.address() as { port: number }).port;
  await new Promise<void>((resolve, reject) => allocation.close(error => error ? reject(error) : resolve()));
  const cancel = new AbortController(), interrupt = () => cancel.abort();
  process.once("SIGINT", interrupt); process.once("SIGTERM", interrupt);
  mkdirSync(output, { recursive: true });
  const logPath = join(output, `actual-${input.variant}-${input.renderer}-${input.profile}-serve.log`), rows: unknown[] = [];
  const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
  let serve: Awaited<ReturnType<typeof ensureDevServe>> | undefined;
  try {
    serve = await ensureDevServe({ repoRoot: root, port, variant: input.variant, renderer: input.renderer, profile: input.profile, locale: "en", signal: cancel.signal, logPath, onProgress: (_status, line) => console.log("[DEBUG] " + line) });
    assert.equal(serve.reused, false, "This verification requires its own freshly selected serve owner");
    const request = async (route: string, bound: number) => {
      assert(route.startsWith("/") && !route.startsWith("//") && bound > 0 && bound <= 128 * 1024 * 1024);
      const url = new URL(route, serve!.url);
      assert(!route.includes("\\") && url.origin === new URL(serve!.url).origin, "Mounted request must retain its owned loopback origin");
      const response = await fetch(url, { signal: AbortSignal.any([cancel.signal, AbortSignal.timeout(60_000)]), redirect: "error" });
      const reader = response.body?.getReader(), chunks: Uint8Array[] = []; let length = 0;
      if (reader) try { while (true) { const { done, value } = await reader.read(); if (done) break; length += value.byteLength; assert(length <= bound, "Mounted response exceeded its actual byte bound"); chunks.push(value); } } finally { await reader.cancel(); }
      const bytes = Buffer.concat(chunks), sha256 = hash(bytes), mime = response.headers.get("content-type") ?? "";
      assert.equal(sha256, new Bun.CryptoHasher("sha256").update(bytes).digest("hex"));
      rows.push({ route, status: response.status, mime, byteLength: bytes.length, sha256 });
      return { response, bytes, mime, sha256 };
    };
    const entry = await request("/", 1024 * 1024); assert(entry.response.ok && entry.mime.includes("text/html"));
    for (const row of input.mounts) {
      cancel.signal.throwIfAborted(); const path = resolve(root, row.path), physical = realpathSync(path);
      for (const current of [path, physical]) {
        const coordinate = relative(root, current).replaceAll("\\", "/");
        assert(coordinate && !isAbsolute(coordinate) && !/^[A-Za-z]:/u.test(coordinate) && coordinate !== ".." && !coordinate.startsWith("../") && !runtimeFixturePathV1(coordinate), "Positive mounted byte owner must remain in a genuine repository production owner");
      }
      const before = readFileSync(path);
      assert.equal(hash(before), row.sha256); assert.equal(before.length, row.byteLength);
      const actual = await request(row.route, row.byteLength + 1); assert.equal(actual.response.status, 200);
      assert(row.contentType ? actual.mime.split(";")[0]!.trim().toLowerCase() === row.contentType : row.route.endsWith(".wasm") ? actual.mime.includes("application/wasm") : /javascript|application\/octet-stream/u.test(actual.mime));
      assert.equal(actual.sha256, row.sha256); assert.equal(actual.bytes.length, row.byteLength); assert.equal(realpathSync(path), physical); assert.equal(hash(readFileSync(path)), row.sha256);
      console.log(`[DEBUG] actual mounted byte match ${row.route}: ${row.byteLength} bytes`);
    }
    for (const route of input.retiredRoutes ?? []) {
      const actual = await request(route, 1024 * 1024);
      assert([404, 410].includes(actual.response.status) || actual.response.status === 200 && actual.mime.includes("text/html") && actual.sha256 === entry.sha256, "Retired fixture mount returned an asset response");
      console.log(`[DEBUG] actual retired fixture route refused ${route}: ${actual.response.status} ${actual.mime}`);
    }
    writeFileSync(join(output, `actual-${input.variant}-${input.renderer}-${input.profile}-mounted-http.json`), JSON.stringify({ input, url: serve.url, reused: serve.reused, logPath, rows }, null, 2) + "\n");
    console.log(`[DEBUG] actual ${input.variant}/${input.renderer}/${input.profile} mounted HTTP proof completed: ${input.mounts.length} current mounted byte owners`);
  } finally { await serve?.stop(); process.removeListener("SIGINT", interrupt); process.removeListener("SIGTERM", interrupt); }
  }
  process.exit(0);
}
if (command === "runtime-callsite-source") {
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, library, "🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts"), "-t", "runtime graph (dynamic|rust-resource-read)|runtime resource-read owners|dynamic import owners|Bun import edges|production test exclusion"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}
if (command === "ticket-mcp") {
  if (rest.length !== 1 || !["prepare", "probe", "close-current"].includes(rest[0]!)) throw Error("One explicit current ticket MCP operation required");
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, [join(import.meta.dir, "../🧑‍💻coordination/🔌️ticket-mcp/📜️script.ts"), rest[0]!], { cwd: root, env: { ...repositoryEnv, SEMIO_FIXTURE_REPO_ROOT: root }, budgetMs: 300_000, throwOnFailure: true });
  process.exitCode = 0;
} else {
const args = command === "test-owner-command" ? [join(root, rest[0]!), ...rest.slice(1)] : command === "owner-command" ? [join(root, library, "⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", join(dirname(rest[0]!), "Cargo.toml"), "--cwd", dirname(rest[0]!), "--", process.execPath, join(root, rest[0]!), ...rest.slice(1)] : command === "structural-catalog-test" ? ["test", join(root, library, "🧪️tests/🔬️workspace-contract/🟦️.ts"), "-t", "schema scope catalog"] : command === "kernel-test" ? [join(root, kernel, "📜️script.ts"), "test", "os_store::tests::", "--", "--nocapture"] : command === "kernel-command" ? [join(root, library, "⚡️caching/🦀️cargo/📜️script.ts"), "native", "owner-command", "--manifest", join(kernel, "Cargo.toml"), "--cwd", kernel, "--", process.execPath, join(root, kernel, "📜️script.ts"), ...rest] : command === "schema" ? [join(root, "📜️script.ts"), "schema", ...rest] : command === "verify" ? [join(root, "📜️script.ts"), "verify", ...rest] : command === "tests" ? ["test", ...rest.map(path => join(root, path))] : command === "parity" ? [join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts"), "parity", "quick", "--owner", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test", "--case", "🖥️host-protocol-parity"] : null;
if (!args) throw Error("Unknown verification command");
console.log("[DEBUG] isolated Nx verification " + command);
const child = Bun.spawn([process.execPath, ...args], { cwd: root, env: repositoryEnv, stdin: "inherit", stdout: "inherit", stderr: "inherit" });
const interrupt = () => child.kill("SIGINT"), terminate = () => child.kill("SIGTERM");
process.once("SIGINT", interrupt); process.once("SIGTERM", terminate);
process.exitCode = await child.exited;

}
