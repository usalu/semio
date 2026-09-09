import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { runCmd } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runCargo, runVitest } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const root = fileURLToPath(new URL("../../../../../../../../", import.meta.url));
const args = process.argv.slice(2);
process.env.CARGO_TARGET_DIR = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/cargo-trinity`;
process.env.CARGO_INCREMENTAL = "0";
if (args[0] === "window-view" && args[1] === "test") {
  const { testWindowViewContext } = await import(`${root}/🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts`);
  testWindowViewContext();
} else if (args[0] === "trinity-rewriting" && args[1] === "test") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "--features", "component-app-assembly", ...args.slice(2)], `${root}/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust`);
} else if (args[0] === "host-ownership" && args[1] === "native") {
  for (const filter of ["concrete_window_instances_round_trip_without_kind_collapse", "context_menu_point_resolves_the_exact_concrete_window_instance", "canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript", "build_os_commands_covers_every_wired_setting"]) {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", filter, "--", "--nocapture"], root);
  }
} else if (args[0] === "plugin-host" && args[1] === "check") {
  await runCargo(["check", "--manifest-path", "Cargo.toml"], `${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust`);
} else if (args[0] === "ui-preferences" && args[1] === "fixture") {
  const mutationRoot = `${root}/🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences`;
  const fixtureRoot = `${mutationRoot}/🧪️tests/🎨️updates-every-os-ui-preference`;
  const schema = JSON.parse(readFileSync(`${mutationRoot}/🧬️schema/🔣️.json`, "utf8"));
  const mutations = JSON.parse(readFileSync(`${fixtureRoot}/🦠️mutations/🔣️.json`, "utf8"));
  const before = JSON.parse(readFileSync(`${fixtureRoot}/📸️snapshot/⬅️before/🔣️.json`, "utf8"));
  const after = JSON.parse(readFileSync(`${fixtureRoot}/📸️snapshot/➡️after/🔣️.json`, "utf8"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const mutation of mutations) assert(validate(mutation), JSON.stringify(validate.errors));
  const contract = await import(`${mutationRoot}/🟦️.ts`);
  let snapshot = structuredClone(before);
  const inverses: unknown[][] = [];
  for (const mutation of mutations) {
    inverses.push(contract.inverse(mutation, snapshot));
    snapshot = contract.diff(mutation, snapshot);
  }
  assert.deepEqual(snapshot, after);
  for (const group of inverses.reverse()) for (const mutation of group) snapshot = contract.diff(mutation, snapshot);
  assert.deepEqual(snapshot, before);
  console.log(`ui-preferences-fixture mutations=${mutations.length} schema=valid fold=valid inverse=valid`);
} else if (args[0] === "cargo" && args[1] === "check") {
  await runCargo(["check", "--manifest-path", "Cargo.toml", "-p", args[2]], root);
} else if (args[0] === "block3d-window-transient" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-block-3d", "--features", "component-app-assembly", "--lib", "preview_", "--", "--nocapture"], root);
} else if (args[0] === "returned-snapshot-read" && args[1] === "native") {
  for (const law of ["returned_snapshot_read_releases_an_alias_before_the_displaced_root_and_retires_a_unique_fallback", "store_close_releases_a_returned_read_before_its_displaced_root"]) {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", law, "--", "--nocapture"], root);
  }
} else if (args[0] === "procedural-preview" && args[1] === "native") {
  for (const packageName of ["semio-s-artifact-procedural-generation2d", "semio-s-artifact-procedural-generation3d"]) {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--features", "component-app-assembly", "--lib", "preview_", "--", "--nocapture"], root);
  }
} else if (args[0] === "window-config-channel" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "window_config_transport_matches_shared_cross_language_vectors", "--", "--nocapture"], root);
} else if (args[0] === "window-config-channel" && args[1] === "typescript") {
  process.env.SEMIO_TEST_BUDGET_MS = "60000";
  await runVitest(`${root}/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`, ["-t", "window config"], "vitest.config.ts");
} else if (args[0] === "jack-window-config" && args[1] === "oracle") {
  const { testJackGraphWindowConfigOracle } = await import(`${root}/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts`);
  testJackGraphWindowConfigOracle();
} else if (args[0] === "jack-document-contract") {
  const { testJackDocumentContract } = await import(`${root}/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testJackDocumentContract();
} else if (args[0] === "owned-document-closure") {
  const { testOwnedDocumentClosureOracle } = await import(root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️tests/🔬️unit/🟦️.ts");
  testOwnedDocumentClosureOracle();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🟦️.ts"], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "owned_document_closure", "--", "--nocapture"], root);
} else if (args[0] === "stdio-document-contract") {
  const { testSemioObjectDocumentContract } = await import(root + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
  testSemioObjectDocumentContract();
  const { testSemioKitDocumentContract } = await import(root + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts");
  testSemioKitDocumentContract();
  const { testSemioGeometryContract } = await import(root + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🧪️tests/🔬️unit/🟦️.ts");
  testSemioGeometryContract();
  const stdioSchemaRoot = root + "/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${stdioSchemaRoot}/✉️base/🧬️schema/🧮️geometry/🟦️.ts`, ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts"].map((file) => `${stdioSchemaRoot}/📦️object/🧬️schema/${file}`), ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧬️mutations/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${stdioSchemaRoot}/🧰️kit/🧬️schema/${file}`)], { cwd: root });
  if (args[1] === "native") for (const filter of ["stdio_document_contract", "subsets::object::", "subsets::kit::"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-stdio-semio", "--lib", filter, "--", "--nocapture"], root);
} else if (args[0] === "map-document-contract") {
  const { testMapDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testMapDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--lib", "map_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "terrain-document-contract") {
  const { testTerrainDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testTerrainDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gisterrain", "--lib", "terrain_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "program-document-contract") {
  const { testProgramDocumentContract } = await import(`${root}/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️document-contract/🟦️.ts`);
  testProgramDocumentContract();
  const schemaRoot = `${root}/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-architect-program", "--lib", "program_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "layout-document-contract") {
  const { testLayoutDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testLayoutDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-layout-layout", "--lib", "layout_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "cad-document-contract") {
  const { testCadDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testCadDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-cad-cad", "--lib", "cad_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "generation3d-preview-window-transient") {
  const testRoot = `${root}/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient`;
  const { testGeneration3dPreviewWindowTransientContract } = await import(`${testRoot}/🧪️tests/🔬️contract/🟦️.ts`);
  testGeneration3dPreviewWindowTransientContract();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧬️schema/🟦️.ts`, `${testRoot}/🧪️tests/🔬️contract/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-generation3d", "--features", "component-app-assembly", "--lib", "preview_eval_", "--", "--nocapture"], root);
} else if (args[0] === "gis-terrain-window-config") {
  const testRoot = `${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/⚙️config`;
  const { testGisTerrainWindowConfigContract } = await import(`${testRoot}/🧪️tests/🔬️contract/🟦️.ts`);
  testGisTerrainWindowConfigContract();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🧬️schema/🟦️.ts`, `${testRoot}/🧪️tests/🔬️contract/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gisterrain", "--features", "component-app-assembly", "--lib", "gis_terrain_window_config_", "--", "--nocapture"], root);
} else if (args[0] === "presentation-document-contract") {
  const { testPresentationDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testPresentationDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, ...["--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)]], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "presentation_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "procedure-document-contract") {
  const { testProcedureDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testProcedureDocumentContractOracle();
  const schemaRoot = `${root}/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, ...["--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)]], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-imperative-procedure", "--lib", "procedure_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "dag-document-contract") {
  const { testDagDocumentContractOracle } = await import(`${root}/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts`);
  testDagDocumentContractOracle();
} else if (args[0] === "jack-window-config" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "--features", "component-app-assembly", "--lib", "jack_graph_window_config_", "--", "--nocapture"], `${root}/✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/📦️packages/🦀️rust`);
} else if (args[0] === "writer-window-state" && args[1] === "oracle") {
  const { testWriterWindowStateOracle } = await import(`${root}/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/✒️main/🎚️config/🧪️tests/🔬️window-state-ownership/🟦️.ts`);
  testWriterWindowStateOracle();
} else if (args[0] === "writer-window-state" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "writer_window_state_", "--", "--nocapture"], `${root}/✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/📦️packages/🦀️rust`);
} else if (args[0] === "equation-window-config" && args[1] === "oracle") {
  const { testEquationGraphWindowConfigOracle } = await import(`${root}/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts`);
  testEquationGraphWindowConfigOracle();
} else if (args[0] === "equation-window-config" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "equation_graph_window_config_", "--", "--nocapture"], `${root}/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust`);
} else if (args[0] === "equation-window-config" && args[1] === "check") {
  await runCargo(["check", "--manifest-path", "Cargo.toml", "--tests"], `${root}/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/📦️packages/🦀️rust`);
} else if (args[0] === "dag-demo-ownership") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-artifact-infinite-dag", "--lib", "--", "--nocapture"], root);
} else if (args[0] === "retained-window-input") {
  const { testRetainedWindowInputOracle } = await import(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🟦️.ts`);
  testRetainedWindowInputOracle();
  if (args[1] !== "oracle") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "retained_window_input", "--", "--nocapture"], root);
} else if (args[0] === "flow-window-ownership") {
  const { testFlowWindowOwnershipOracle } = await import(`${root}/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testFlowWindowOwnershipOracle();
  if (args[1] === "check") {
    await runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-flow-flow", "--tests"], root);
  } else if (args[1] === "native") {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-flow-flow", "--lib", "flow_window_ownership_", "--", "--nocapture"], root);
  }
} else {
  const { VerifyScript } = await import("../../../../../../../../📜️script.ts");
  await new VerifyScript(root, root).run(args);
}
