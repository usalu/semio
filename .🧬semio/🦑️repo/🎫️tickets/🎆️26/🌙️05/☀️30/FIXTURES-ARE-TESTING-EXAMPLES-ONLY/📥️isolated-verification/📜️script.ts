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
  await runBudgetedTestCommand(process.execPath, ["test", ...paths.map(path => join(root, path)), ...rest], { cwd: root, env: { ...repositoryEnv, SEMIO_TICKET_DIR: dirname(import.meta.dir) }, budgetMs: 180_000, throwOnFailure: true });
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
if (command === "root-resumed-corpus-tests") {
  if (rest[0] === "--runtime-schema-only" && rest.length === 1) {
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    await runBudgetedTestCommand(process.execPath, ["test", join(root, library, "🔍️discovery/🕸️runtime/🧪️tests/🟦️.ts"), "-t", "^canonical runtime domain schemas"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    process.exit(0);
  }
  if (rest[0] === "--current-corpus-only" && rest.length === 1) {
    const { default: assert } = await import("node:assert/strict");
    const { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = await import("node:fs");
    const { inventorySchemaScopes } = await import(join(root, library, "🔍️discovery/🟦️.ts"));
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    const owners = ["🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🌱️paged-origin", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/♻️retire", "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🌱️authored-paged-source", "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/♻️source-currencies", "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity", "🧰️framework/🔨️modules/🎒️pack/🌱️value/🧪️tests/🧫️fixtures/🔗️paged-source-custody", "🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🧫️fixtures/🧹️metadata", "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/♻️child-currencies", "🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️shared/🏭️factory"];
    const retiredWholeLawSchemas = ["✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧬️schema/🧫️witness/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🧫️fixtures/♻️inline-source-ownership/🧬️schema/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧬️schema/🎟️source-authority/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧬️schema/🎟️preparation-birth/🔣️.json", "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📝️drafts/🧬️schema/🔣️.json"];
    retiredWholeLawSchemas.push(...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🎟️admission/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push("🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/♻️retirement/🧬️schema/🔣️.json");
    retiredWholeLawSchemas.push(...["🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/🎯️ray-segment-closest/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧬️schema/🎯️zero-index/🔣️.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🔨️modules/🌱️value/🔗️read/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧬️schema/🎟️owned-birth/🔣️.json", "🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🔗️authority/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🗺️ordered-map/♻️custody/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🧬️schema/🔣️.json", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/♻️close/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push(...["✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧫️fixtures/🧊️validation/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️raster-lease-close/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🧬️schema/📦️prepared-close/🔣️.json", "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✅validation/🧪️tests/🧫️fixtures/🧊️cold-planning/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧭️tree/🧬️schema/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🔨️modules/🖱️ui/♻️retirement/🧬️schema/🖱️physical-job-close/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/♻️retirement/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖼️raster-ownership/♻️retirement/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🔤️text/🧬️schema/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧬️schema/♻️ownership/🔣️.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/📐️declaration-query.schema.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/📐️schema.json", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/📄️entry/📐️schema.json"]);
    retiredWholeLawSchemas.push(...["🧰️framework/🔨️modules/📐️geometry/🧭️placement/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/➰️loops/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/🕸️mesh/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/🌙️bulge/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/🔪️section/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/🦴️skeleton/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/📐️geometry/🔺️triangulation/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/◻️2d/🧱️regions/🧬️schema/🔣️.json"]);
    retiredWholeLawSchemas.push("🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🎟️admission/🧬️schema/🔣️.json");
    retiredWholeLawSchemas.push(...["🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🧬️schema/🔣️.json", "🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧬️schema/🧹️retirement/🔣️.json", "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧬️schema/🌱️entry/🔣️.json"]);
    assert.deepEqual(retiredWholeLawSchemas.filter(path => existsSync(join(root, path))), [], "Whole testing laws cannot define separate schema authority");
    const { default: ts } = await import("typescript");
    const inlineCorpusOwners = ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts"];
    inlineCorpusOwners.push(...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"]);
    inlineCorpusOwners.push("🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️prepared-close/🟦️.ts");
    inlineCorpusOwners.push("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📐️geometry/🧪️tests/🟦️.ts");
    const inlineAuthorities = inlineCorpusOwners.map(path => {
      const source = ts.createSourceFile(path, readFileSync(join(root, path), "utf8"), ts.ScriptTarget.Latest, true);
      let count = 0;
      const inspect = (node: import("typescript").Node): void => {
        if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "compile" && node.arguments[0] && (path.includes("🧪️tests/📦️prepared-close/") || ts.isObjectLiteralExpression(node.arguments[0]) || (path.includes("📜️space-history/") || path.includes("🧊️generation3d/")) && ts.isIdentifier(node.arguments[0]) && node.arguments[0].text === "contract")) count++;
        ts.forEachChild(node, inspect);
      };
      inspect(source);
      return { path, count };
    }).filter(row => row.count > 0);
    assert.deepEqual(inlineAuthorities, [], "Known whole-corpus validators cannot retain copied inline schema authority");
    const wholeReaderOwners = ["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🎟️admission/🧪️tests/🔬️unit/🟦️.ts", "🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️tests/🔬️wire-retirement/🟦️.ts", "🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧪️semio-tech-framework-3d-js/🟦️.ts"];
    wholeReaderOwners.push("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🔬️unit/🟦️.ts");
    wholeReaderOwners.push(...["🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🌱️entry/🟦️.ts"]);
    const wholeReaderAuthorities = wholeReaderOwners.map(path => {
      const source = ts.createSourceFile(path, readFileSync(join(root, path), "utf8"), ts.ScriptTarget.Latest, true);
      let count = 0;
      const inspect = (node: import("typescript").Node): void => {
        if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "compile" && node.arguments[0] && (!path.includes("🧊️3d/") || !ts.isObjectLiteralExpression(node.arguments[0]))) count++;
        ts.forEachChild(node, inspect);
      };
      inspect(source);
      return { path, count };
    }).filter(row => row.count > 0);
    assert.deepEqual(wholeReaderAuthorities, [], "Reviewed whole testing examples cannot retain independent validator authority");

    const strokeSource = ts.createSourceFile("stroke.ts", readFileSync(join(root, "🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧪️tests/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    let strokeWholeAuthorities = 0;
    const inspectStroke = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "compile" && node.arguments[0] && ts.isIdentifier(node.arguments[0]) && node.arguments[0].text === "retirementSchema") strokeWholeAuthorities++;
      ts.forEachChild(node, inspectStroke);
    };
    inspectStroke(strokeSource);
    assert.equal(strokeWholeAuthorities, 0, "Stroke whole trials cannot retain a separate schema; produced input/progress validators remain");
    const spatialRasterRetired = ["✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧫️fixtures/🧹️retirement/🧬️schema/🔣️.json", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️raster/🧬️schema/🧹️retirement/🔣️.json", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️raster/🧬️schema/🧹️flatten/🔣️.json", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️raster/🧬️schema/🧹️stroke/🔣️.json", "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️raster/🧬️schema/🧹️coverage/🔣️.json"];
    assert.deepEqual(spatialRasterRetired.filter(path => existsSync(join(root, path))), [], "Reviewed Spatial and Raster whole trials cannot own schemas");
    const rasterReader = ts.createSourceFile("raster.ts", readFileSync(join(root, "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧮️geometry/📷️raster/🧪️tests/🔬️unit/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
    let rasterWholeAuthorities = 0;
    const inspectRaster = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "compile" && node.arguments[0] && ts.isIdentifier(node.arguments[0]) && ["retirementSchema", "flattenSchema", "strokeSchema", "coverageSchema"].includes(node.arguments[0].text)) rasterWholeAuthorities++;
      ts.forEachChild(node, inspectRaster);
    };
    inspectRaster(rasterReader);
    assert.equal(rasterWholeAuthorities, 0, "Raster trial validators must retire while produced input/progress contracts remain");
    assert(!readFileSync(join(root, "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts"), "utf8").includes("🧫️fixtures/🧹️retirement/🧬️schema/"), "Spatial retirement examples cannot retain separate schema reads");
    const output = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
    if (!output) throw Error("Explicit ticket output required for physical corpus inventory");
    mkdirSync(output, { recursive: true });
    const scopeRoot = mkdtempSync(join(output, "current-corpus-"));
    try {
      for (const owner of owners) cpSync(join(root, owner), join(scopeRoot, owner), { recursive: true });
      const inventory = inventorySchemaScopes(scopeRoot, JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8")));
      const owned = (path: string) => owners.some(owner => path === owner || path.startsWith(owner + "/"));
      const authorities = inventory.modules.filter(row => owned(row.modulePath));
      const diagnostics = [...inventory.diagnostics, ...inventory.placement].filter(row => owned(row.path) && row.code === "schema-fixture-defines-schema");
      writeFileSync(join(output, "current-corpus-placement.json"), JSON.stringify({ owners, authorities, diagnostics }, null, 2) + "\n");
      assert.deepEqual(diagnostics, [], "Physical or adjacent test-corpus authority must retire independent of filename");
      assert.deepEqual(authorities, []);
    } finally { rmSync(scopeRoot, { recursive: true, force: true }); }
    const value = JSON.parse(readFileSync(join(root, owners[0]!, "🔣️.json"), "utf8"));
    const plugin = JSON.parse(readFileSync(join(root, owners[3]!, "🔣️.json"), "utf8"));
    const payload = Buffer.alloc(value.textBytes, value.textByte);
    const body = Buffer.concat([Buffer.from(value.bodyPrefix), payload]);
    const operation = Buffer.concat([Buffer.from(value.operationPrefix), payload]);
    const frame = Buffer.concat([Buffer.from(plugin.framePrefix), Buffer.alloc(plugin.textBytes, plugin.textByte)]);
    assert.equal(body.length, value.bodyBytes);
    assert.equal(operation.length, value.operationBytes);
    assert.equal(frame.length, plugin.frameBytes);
    assert.deepEqual(frame, operation);
    assert.deepEqual(plugin.modes, value.modes);
    for (const text of value.unicode) {
      const encoded = Buffer.from(text, "utf8"), crossing = Buffer.from(payload);
      encoded.copy(crossing, value.maximumBytes - 1);
      assert.equal(crossing.subarray(value.maximumBytes - 1, value.maximumBytes - 1 + encoded.length).toString("utf8"), text);
    }
    const currencies = JSON.parse(readFileSync(join(root, owners[4]!, "🔣️.json"), "utf8"));
    assert.equal(currencies.sourceBytes, payload.length);
    assert.equal(currencies.maximumReleaseBytes, value.maximumBytes);
    const custody = JSON.parse(readFileSync(join(root, owners[6]!, "🔣️.json"), "utf8"));
    const child = JSON.parse(readFileSync(join(root, owners[10]!, "🔣️.json"), "utf8"));
    assert.deepEqual(Buffer.concat([Buffer.from(custody.bodyPrefix), Buffer.alloc(custody.sourceBytes, custody.textByte)]), body);
    assert.deepEqual(Buffer.alloc(child.semanticBytes, child.semanticByte), payload);
    const metadata = JSON.parse(readFileSync(join(root, owners[7]!, "🔣️.json"), "utf8"));
    const { BufferGeometry, Float32BufferAttribute } = await import("three");
    const mesh = new BufferGeometry();
    try {
      mesh.setAttribute("position", new Float32BufferAttribute(metadata.mesh.positions, 3));
      mesh.setAttribute("normal", new Float32BufferAttribute(metadata.mesh.normals, 3));
      assert.deepEqual(Array.from(mesh.getAttribute("position").array), metadata.mesh.positions);
      assert.deepEqual(Array.from(mesh.getAttribute("normal").array), metadata.mesh.normals);
      assert.deepEqual(JSON.parse(JSON.stringify(metadata.mesh.attributes)), metadata.mesh.attributes);
      assert.deepEqual(JSON.parse(JSON.stringify(metadata.mesh.materials)), metadata.mesh.materials);
      assert.deepEqual(Array.from(Buffer.from(metadata.mesh.textures.ink.bytes)), metadata.mesh.textures.ink.bytes);
    } finally { mesh.dispose(); }
    const cold = JSON.parse(readFileSync(join(root, owners[9]!, "🧫️fixtures/⏱️cold-preview/🔣️.json"), "utf8"));
    assert.deepEqual(structuredClone(cold), JSON.parse(JSON.stringify(cold)));
    const identities = JSON.parse(readFileSync(join(root, owners[5]!, "🧫️fixtures/🔣️.json"), "utf8"));
    const { blake3 } = await import("@noble/hashes/blake3.js");
    const { blake3Hex } = await import(join(root, "🧰️framework/🔨️modules/🔏️hash/🟦️.ts"));
    const identityRows = [];
    for (const row of identities.cases) {
      const zero = Buffer.alloc(1), text = (input: string) => Buffer.from(input, "utf8");
      const unsigned = (input: string) => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(input)); return bytes; };
      let bytes: Buffer;
      switch (row.kind) {
        case "entity": bytes = Buffer.concat([text(row.prefix), zero, Buffer.from(row.input.payload)]); break;
        case "scoped": bytes = text(row.input.editId + ":" + row.input.ordinal); break;
        case "edit": { const sequence = Buffer.alloc(4); sequence.writeInt32LE(row.input.sequence); bytes = Buffer.concat([text(row.prefix), zero, unsigned(row.input.replica), zero, sequence, zero, Buffer.from(row.input.fingerprint)]); break; }
        case "change": bytes = Buffer.concat([text(row.prefix), zero, text(row.input.editIds.join("\0")), zero, text(row.input.description ?? "")]); break;
        case "alternative": bytes = Buffer.concat([text(row.prefix), zero, text(row.input.name), zero, text(row.input.checkpointIds.join("\0"))]); break;
        case "mutation": bytes = Buffer.concat([text(row.prefix), zero, Buffer.from(row.input.operation), ...row.input.stamp.map(unsigned)]); break;
        default: throw Error("Unknown identity specimen");
      }
      assert.equal(bytes.toString("hex"), row.preimageHex);
      const independent = Buffer.from(blake3(bytes)).toString("hex");
      assert.equal(blake3Hex(bytes), independent);
      identityRows.push({ kind: row.kind, preimageHex: row.preimageHex, hash: independent });
    }
    writeFileSync(join(output, "current-corpus-identity-oracle.json"), JSON.stringify({ rows: identityRows, nativeInvocationCount: 0 }, null, 2) + "\n");
    writeFileSync(join(output, "current-corpus-neutral-bytes.json"), JSON.stringify({ bodyBytes: body.length, operationBytes: operation.length, frameBytes: frame.length, utf8Crossings: value.unicode.length, plainModes: value.modes, nativeInvocationCount: 0 }, null, 2) + "\n");
    await runBudgetedTestCommand(process.execPath, ["test", join(root, owners[1]!, "🧪️tests/🟦️.ts"), join(root, owners[2]!, "🧪️tests/🔬️unit/🟦️.ts"), join(root, owners[8]!, "🔬️unit/🟦️.ts"), join(root, owners[11]!, "🧪️tests/🟦️.ts"), ...["✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧪️tests/🎟️source-authority/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️tests/🎟️preparation-birth/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/📝️drafts/🧪️tests/🟦️.ts"].map(path => join(root, path)), ...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🎟️admission/🧪️tests/🔬️unit/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/⚔️conflict/♻️retirement/🧪️tests/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🔨️modules/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts"), "-t", "^(typed read leases|erased read portable|owned source birth|funded read ownership|read closure order)"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", ...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts", "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/♻️close/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/📦️accumulate/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", ...["✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🎒️mesh-session/🟦️.ts", "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📦️prepared-close/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", ...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧭️tree/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🎟️admission/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", ...["🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🔤️text/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧪️tests/♻️ownership/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪆️child/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/📄️entry/🧪️tests/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🎟️admission/🧪️tests/🔬️unit/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    const { testWireRetirementFixture } = await import(join(root, "🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🧪️tests/🔬️wire-retirement/🟦️.ts"));
    testWireRetirementFixture();
    await runBudgetedTestCommand(process.execPath, ["test", ...["🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📦️owned/🧪️tests/🟦️.ts", "🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🌱️entry/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    console.log("[DEBUG] current test corpora: physical authorities0; independent Node bytes/UTF8, Three and RFC6902; native invocation0");
    process.exit(0);
  }
  if (rest[0] === "--durable-maintenance-only" && rest.length === 1) {
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/📦️owned/🧪️tests/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    console.log("[DEBUG] durable maintenance source/domain neutral laws; Cargo/native producer invocation0");
    process.exit(0);
  }
  if (rest[0] === "--post-session-corpus-only" && rest.length === 1) {
    const { default: assert } = await import("node:assert/strict");
    const { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = await import("node:fs");
    const { inventorySchemaScopes } = await import(join(root, library, "🔍️discovery/🟦️.ts"));
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    const owners = ["✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot", "🌎️hub/🧩️compositions/🗄️stdio/🧫️fixtures/🚢️shipped-fleet/🪶️sqlite", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands"];
    const output = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
    if (!output) throw Error("Explicit ticket output required for bounded physical collection inventory");
    mkdirSync(output, { recursive: true });
    const scopeRoot = mkdtempSync(join(output, "post-session-corpus-"));
    try {
      for (const owner of owners) cpSync(join(root, owner), join(scopeRoot, owner), { recursive: true });
      const inventory = inventorySchemaScopes(scopeRoot, JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8")));
      const owned = (path: string) => owners.some(owner => path === owner || path.startsWith(owner + "/"));
      const modules = inventory.modules.filter(row => owned(row.modulePath)), diagnostics = inventory.diagnostics.filter(row => owned(row.path));
      writeFileSync(join(output, "post-session-corpus-placement.json"), JSON.stringify({ owners, modules, diagnostics, observedAncestorModules: inventory.modules.filter(row => !owned(row.modulePath)) }, null, 2) + "\n");
      assert.deepEqual(diagnostics, [], "All selected collection authorities must retire independent of nested filename or folder");
      assert.deepEqual(modules, []);
    } finally { rmSync(scopeRoot, { recursive: true, force: true }); }
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧱️content/🟦️.ts"), ...["✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts", "🌎️hub/🧩️compositions/🗄️stdio/🧪️tests/🚢️shipped-fleet/🪶️sqlite/🟦️.ts", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/🟦️.ts"].map(path => join(root, path))], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    console.log("[DEBUG] Four current corpus authorities retired; normal selected collection placement0; actual SQLite/domain Ajv/command owner laws; Cargo/native producers0");
    process.exit(0);
  }
  if (rest[0] === "--post-corpus-only" && rest.length === 1) {
    const { default: assert } = await import("node:assert/strict");
    const { cpSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } = await import("node:fs");
    const { inventorySchemaScopes } = await import(join(root, library, "🔍️discovery/🟦️.ts"));
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    const owners = ["✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages", "🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation", "🧰️framework/🔨️modules/📡️replication/🎮️mutation/📨️messages/🧵️compose"];
    const output = repositoryEnv.SEMIO_TEST_ARTIFACT_DIR;
    if (!output) throw Error("Explicit ticket output required for bounded physical corpus inventory");
    mkdirSync(output, { recursive: true });
    const scopeRoot = mkdtempSync(join(output, "post-corpus-"));
    try {
      for (const owner of owners) cpSync(join(root, owner), join(scopeRoot, owner), { recursive: true });
      const inventory = inventorySchemaScopes(scopeRoot, JSON.parse(readFileSync(join(root, library, "🔣️taxonomy.json"), "utf8")));
      const owned = (path: string) => owners.some(owner => path === owner || path.startsWith(owner + "/"));
      const modules = inventory.modules.filter(row => owned(row.modulePath)), diagnostics = inventory.diagnostics.filter(row => owned(row.path));
      writeFileSync(join(output, "post-corpus-placement.json"), JSON.stringify({ owners, modules, diagnostics, observedAncestorModules: inventory.modules.filter(row => !owned(row.modulePath)) }, null, 2) + "\n");
      assert.deepEqual(diagnostics, [], "All selected physical collection authorities must retire independent of nested filename or folder");
      assert.deepEqual(modules, []);
    } finally { rmSync(scopeRoot, { recursive: true, force: true }); }
    await runBudgetedTestCommand(process.execPath, ["test", join(root, owners[0]!, "🧪️tests/🟦️.ts"), join(root, owners[1]!, "🧪️tests/🔬️unit/🟦️.ts"), join(root, owners[2]!, "🧪️tests/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    console.log("[DEBUG] current selected three corpus owners: normal placement modules0 diagnostics0; actual SQLite/Three/JSON/UTF8/RFC6902 laws; Cargo/native producers0");
    process.exit(0);
  }
  const { runMutationInventoryProviderChecksV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/🔌️providers/🧪️tests/🟦️.ts"));
  console.log(`[DEBUG] actual mutation inventory provider: ${runMutationInventoryProviderChecksV1()} independent admission/selection/discovery laws`);
  const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
  await runBudgetedTestCommand(process.execPath, ["test", join(root, "✏️s/🧪️tests/🗂️native-ownership/🟦️.ts"), "-t", "Variable (Neutral Owner|Complete Ownership)"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  await runBudgetedTestCommand(process.execPath, ["test", join(root, library, "🧪️tests/🔬️workspace-contract/🟦️.ts"), "-t", "schema scope catalog"], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
  process.exit(0);
}
if (command === "root-canonical-process-preview") {
  if (rest[0] === "--session-only" && rest.length === 1) {
    const { runBudgetedTestCommand } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/🧪️testing/🎛️execution/🟦️.ts"));
    await runBudgetedTestCommand(process.execPath, ["test", join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🎮️playground-session/🧪️tests/🟦️.ts")], { cwd: root, env: repositoryEnv, budgetMs: 120_000, throwOnFailure: true });
    process.exit(0);
  }
  const { default: mountedAssert } = await import("node:assert/strict");
  const { readFileSync: readMountedSource } = await import("node:fs");
  const { default: ts } = await import("typescript");
  const { default: Ajv } = await import("ajv");
  const { default: mountedParseArgv } = await import("yargs-parser");
  const { ensureDevServe, devServeCommandV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts"));
  const source = ts.createSourceFile("mounted-owner.ts", readMountedSource(import.meta.filename, "utf8"), ts.ScriptTarget.Latest, true);
  const calls: import("typescript").CallExpression[] = [];
  const collect = (node: import("typescript").Node): void => { if (ts.isCallExpression(node) && ts.isIdentifier(node.expression) && node.expression.text === "ensureDevServe" && node.arguments[0]?.getText(source).includes("input.variant")) calls.push(node); ts.forEachChild(node, collect); };
  collect(source); mountedAssert.equal(calls.length, 1);
  const mountedOptions = new Function("input", "root", "port", "cancel", "logPath", `return (${calls[0]!.arguments[0]!.getText(source)});`);
  const dev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
  const vectors = JSON.parse(readMountedSource(join(dev, "🧫️fixtures/🚀️local-hub.json"), "utf8")).serves.spawns;
  const schema = JSON.parse(readMountedSource(join(dev, "🧬️schema/🔣️.json"), "utf8"));
  const admit = new Ajv({ strict: false }).compile({ ...schema, $ref: "#/$defs/DevServeSpawnRequestV1" });
  for (const row of vectors) {
    mountedAssert(admit(row.request));
    mountedAssert.deepEqual(devServeCommandV1(row.request), row.command);
    let spawned = false, stopped = 0, observed: unknown;
    const world = { answers: async () => spawned, portInUse: async () => false, spawnServe: (request: Parameters<typeof devServeCommandV1>[0]) => { mountedAssert(admit(request)); observed = devServeCommandV1(request); spawned = true; return { pid: 123, exited: () => false }; }, terminate: () => { stopped++; }, now: () => 0, sleep: async () => {} };
    const options = mountedOptions(row.request, root, row.request.port, new AbortController(), row.request.logPath);
    const serve = await ensureDevServe({ ...options, world });
    mountedAssert.equal(serve.reused, false); await serve.stop(); mountedAssert.equal(stopped, 1);
    const expected = devServeCommandV1({ ...row.request, hubUrl: null });
    mountedAssert.deepEqual(observed, expected);
    const parseOptions = { configuration: { "populate--": true, "camel-case-expansion": false, "parse-numbers": false } };
    mountedAssert.deepEqual(mountedParseArgv((observed as typeof expected).args, parseOptions), mountedParseArgv(row.command.args, parseOptions));
  }
  mountedAssert.equal(vectors.length, 4);
  const helper = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "mountedRuntimeCompositionPathV1");
  mountedAssert(helper);
  const helperJs = new Bun.Transpiler({ loader: "ts" }).transformSync(helper!.getText(source));
  const admitComposition = new Function("root", "library", "join", helperJs + ";return mountedRuntimeCompositionPathV1;")(root, library, join);
  const wgpu = vectors.find((row: any) => row.request.renderer === "wgpu").request;
  mountedAssert.equal(await admitComposition(wgpu), wgpu.compositionConfigPath);
  for (const compositionConfigPath of [undefined, "../outside.ts", "/absolute.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🚀️local-hub.json"]) await mountedAssert.rejects(() => admitComposition({ ...wgpu, compositionConfigPath }));
  mountedAssert.equal(await admitComposition({ renderer: "react", variant: "s" }), undefined);
  mountedAssert.throws(() => devServeCommandV1({ ...vectors.find((row: any) => row.request.renderer === "wgpu").request, compositionConfigPath: undefined }), /explicit bounded composition/u);
  console.log("[DEBUG] actual mounted ensureDevServe adapter: four canonical spawn vectors, independent Ajv/yargs-parser, owned fake-world stop; servers/native producers=0");
  if (rest[0] === "--mounted-only" && rest.length === 1) process.exit(0);
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
/** 🧊️ Checks the declared physical composition owner before an actual mounted serve starts. */
async function mountedRuntimeCompositionPathV1(input: { variant: string; renderer: string; compositionConfigPath?: unknown }): Promise<string | undefined> {
  if (input.renderer !== "wgpu") return undefined;
  const { playgroundCompositionPathV1 } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟦️.ts"));
  const { PLAYGROUND_BUILD_TARGETS } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts"));
  const { runtimeFixturePathV1 } = await import(join(root, library, "🔍️discovery/🕸️runtime/🟦️.ts"));
  const { realpathSync } = await import("node:fs");
  const { isAbsolute, relative, resolve } = await import("node:path");
  const path = playgroundCompositionPathV1(input.compositionConfigPath);
  if (PLAYGROUND_BUILD_TARGETS.find(row => row.variant === input.variant)?.compositionConfigPath !== path) throw Error("Mounted composition must match its current generated playground owner");
  const lexical = resolve(root, path), physical = realpathSync(lexical);
  for (const current of [lexical, physical]) {
    const coordinate = relative(root, current).replaceAll("\\", "/");
    if (!coordinate || isAbsolute(coordinate) || /^[A-Za-z]:/u.test(coordinate) || coordinate === ".." || coordinate.startsWith("../") || runtimeFixturePathV1(coordinate)) throw Error("Mounted composition must retain a genuine repository production owner");
  }
  return path;
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
  await mountedRuntimeCompositionPathV1(input);
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
    serve = await ensureDevServe({ repoRoot: root, port, variant: input.variant, renderer: input.renderer, profile: input.profile, compositionConfigPath: input.compositionConfigPath, locale: "en", signal: cancel.signal, logPath, onProgress: (_status, line) => console.log("[DEBUG] " + line) });
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
