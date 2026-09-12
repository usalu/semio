import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { runCmd } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runCargo, runVitest } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const root = fileURLToPath(new URL("../../../../../../../../", import.meta.url));
const args = process.argv.slice(2);
process.env.CARGO_TARGET_DIR ??= `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/cargo/${args[0] ?? "validation"}`;
process.env.CARGO_INCREMENTAL = "0";
if (args[0] === "verify-abstraction-ownership") {
  runCmd("bun", [root + "/📜️script.ts", "verify", "abstraction-ownership", ...args.slice(1)], { cwd: root });
} else if (args[0] === "framework-viewport-ownership") {
  const testRoot = root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪟️poses";
  const { testViewportOwnership } = await import(`${testRoot}/🟦️.ts`);
  testViewportOwnership();
  runCmd("bun", [root + "/node_modules/prettier/bin/prettier.cjs", "--check", ...["◻️2d", "🧊️3d"].map((dimension) => root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/" + dimension + "/🧬️schema/🔗️.graphql")], { cwd: root });
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: root });
  if (args[1] === "native") {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-viewport", "--lib", "viewport_ownership_", "--", "--nocapture"], root);
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "viewport_ownership_", "--", "--nocapture"], root);
  }
} else if (args[0] === "framework-viewport-projection") {
  if (args.length > 2 || (args[1] !== undefined && args[1] !== "native")) throw new Error("framework-viewport-projection accepts only the optional native phase");
  const testRoot = root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection";
  const { testViewport3dProjectionValues } = await import(`${testRoot}/🟦️.ts`);
  testViewport3dProjectionValues();
  runCmd("bun", [root + "/node_modules/prettier/bin/prettier.cjs", "--check", root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔗️.graphql"], { cwd: root });
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: root });
  if (args[1] === "native") {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-viewport", "--lib", "viewport_projection_", "--", "--nocapture"], root);
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-scene", "--lib", "viewport_projection_pack_", "--", "--nocapture"], root);
  }
} else if (args[0] === "framework-ui-protocol-ownership") {
  if (args.length !== 1) throw new Error("framework-ui-protocol-ownership accepts no arguments");
  const testPath = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🧬️schema-owner/🟦️.ts";
  const { testRetainedCommandSchemaOwnership } = await import(testPath);
  testRetainedCommandSchemaOwnership();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", testPath], { cwd: root });
  await runVitest(root + "/🧰️framework/📦️packages/🟦️typescript", ["-t", "organizeContextMenu"], "vitest.config.ts");
  process.env.SEMIO_TEST_LEVEL = "long";
  await runVitest(root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript", ["-t", "world-3d paged scene carrier"], "vitest.config.ts");
  runCmd("bun", [root + "/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts", "interactive-job-catalog-check"], { cwd: root });
} else if (args[0] === "framework-job-physical-close") {
  const testRoot = root + "/🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close";
  const { testJobPayloadPhysicalClose } = await import(`${testRoot}/🟦️.ts`);
  testJobPayloadPhysicalClose();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: root });
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-job", "--lib", "--", "--nocapture", "--test-threads=1"], root);
} else if (args[0] === "framework-paged-list-ownership") {
  const testRoot = root + "/🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list";
  const { testPagedListOwnership } = await import(`${testRoot}/🟦️.ts`);
  testPagedListOwnership();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: root });
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], root);
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-contract", "--lib", "fixed_list_", "--", "--nocapture"], root);
} else if (args[0] === "framework-retained-pack-ownership") {
  runCmd("bun", [root + "/📜️script.ts", "verify", "framework-retained-pack-ownership", ...args.slice(1)], { cwd: root });
} else if (args[0] === "window-config-pack-identity") {
  runCmd("bun", [root + "/📜️script.ts", "verify", "window-config-pack-identity", ...args.slice(1)], { cwd: root });
} else if (args[0] === "node-graph-viewport-ownership") {
  runCmd("bun", [root + "/📜️script.ts", "verify", "node-graph-viewport-ownership", ...args.slice(1)], { cwd: root });
} else if (args[0] === "framework-variant-field-casing") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-value-derive", "--test", "variant_field_casing", "--", "--nocapture"], root);
} else if (args[0] === "framework-empty-state-contract") {
  const testRoot = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🚫️empty-state";
  const { testFrameworkEmptyStateContract } = await import(`${testRoot}/🟦️.ts`);
  testFrameworkEmptyStateContract();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "framework_empty_state_contract_", "--", "--nocapture"], root);
} else if (args[0] === "fem-scalar-owners-native") {
  runCmd("bun", [root + "/📜️script.ts", "verify", ...args], { cwd: root });
} else if (args[0] === "fem3d-numerical-child-native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-3d", "--features", "component-app-assembly", "--lib", "live_visual::tests::", "--", "--nocapture"], root);
} else if (args[0] === "fem2d-window-config-contract" || args[0] === "fem3d-window-config-contract") {
  const dimension = args[0].startsWith("fem2d") ? "2d" : "3d";
  const testRoot = root + "/✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract";
  const mountedStiffnessOracle = root + "/✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts";
  if (dimension === "3d") await runVitest(root + "/✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript", ["-t", "story window ownership|story document replacement"], "vitest.config.ts");
  const contract = await import(`${testRoot}/🟦️.ts`);
  if (dimension === "2d") contract.testFem2dWindowConfigContract();
  else {
    contract.testFem2dWindowConfigContract();
    contract.testFem3dWindowConfigContract();
    const { testFem3dMountedStiffnessOracle } = await import(mountedStiffnessOracle);
    testFem3dMountedStiffnessOracle();
  }
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${testRoot}/🟦️.ts`, ...(dimension === "3d" ? [mountedStiffnessOracle, root + "/✏️s/🔌️plugins/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts"] : [])], { cwd: root });
  if (args[1] === "native") {
    if (dimension === "2d") {
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--features", "component-app-assembly", "--lib", "fem2d_window_config_", "--", "--nocapture"], root);
    } else {
      const failures: unknown[] = [];
      for (const [packageName, filters] of [
        ["semio-s-artifact-fem-2d", ["fem2d_window_config_", "mesh_edge_authority_", "mounted_3d_element_interfaces_", "assembly_triplet_pages_", "pcg_job_", "subspace_"]],
        ["semio-s-artifact-fem-3d", ["fem3d_window_config_", "live_visual::tests::"]],
      ] as const) {
        try {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--features", "component-app-assembly", "--lib", "--", "--nocapture", "--test-threads=1", ...filters], root);
        } catch (error) {
          failures.push(error);
        }
      }
      if (failures.length > 0) throw new AggregateError(failures, "FEM native package validations failed");
    }
  }
} else if (args[0] === "composition-policy") {
  const ownerRoot = root + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript";
  runCmd("bun", [ownerRoot + "/📜️script.ts", "test", "composition-policy"], { cwd: ownerRoot });
} else if (args[0] === "process3d-diff-laws") {
  const ownerRoot = root + "/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust";
  runCmd("bun", [ownerRoot + "/📜️script.ts", "test", "--lib", "--filter-expr", "test(schema::diff::component::tests::)", "--ignore-default-filter"], { cwd: ownerRoot });
} else if (args[0] === "window-view" && args[1] === "test") {
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
} else if (args[0] === "recursive-document-archive" && args[1] === "typescript") {
  process.env.SEMIO_TEST_BUDGET_MS = "60000";
  await runVitest(`${root}/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript`, ["-t", "document archive|DocumentArchive"], "vitest.config.ts");
} else if (args[0] === "recursive-document-archive" && args[1] === "storage") {
  const ownerRoot = `${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript`;
  process.env.SEMIO_TEST_ARTIFACT_DIR = `${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/🗑️generated/archive-storage`;
  runCmd("bun", [`${ownerRoot}/📜️script.ts`, "canonical-bootstrap-folder-mirror-check", "process"], { cwd: ownerRoot });
} else if (args[0] === "recursive-document-archive" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "document_archive_", "--", "--nocapture"], root);
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
} else if (args[0] === "curation-document-contract") {
  const schemaRoot = root + "/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema";
  const { testCurationDocumentContractOracle } = await import(schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts");
  testCurationDocumentContractOracle();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", schemaRoot + "/🧪️tests/🪪️document-contract/🟦️.ts"], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sourcing-curation", "--lib", "curation_document_contract", "--", "--nocapture"], root);
} else if (args[0] === "snapshot-read-retirement") {
  const { testSnapshotReadRetirement } = await import(root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts");
  testSnapshotReadRetirement();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts"], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "snapshot_read_", "--", "--nocapture"], root);
} else if (args[0] === "semio-envelope-identity") {
      const ownerRoot = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio";
      const { testSemioEnvelopeIdentity } = await import(ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts");
      testSemioEnvelopeIdentity();
      runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts"], { cwd: root });
      if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "semio_envelope_identity", "--", "--nocapture"], root);
} else if (args[0] === "forms-document-contract") {
  const schemaRoot = `${root}/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  const { testFormsDocumentContractOracle } = await import(`${schemaRoot}/🧪️tests/🪪️document-contract/🟦️.ts`);
  testFormsDocumentContractOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-forms-forms", "--lib", ...(args[2] === "all" ? [] : ["forms_document_contract_"]), "--", "--nocapture"], root);
} else if (args[0] === "dag-document-contract") {
  const schemaRoot = `${root}/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema`;
  const { testDagDocumentContractOracle } = await import(`${schemaRoot}/🧪️tests/🪪️document-contract/🟦️.ts`);
  testDagDocumentContractOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ...["🟦️.ts", "📸️snapshot/🟦️.ts", "🔺️diff/🟦️.ts", "🧪️tests/🪪️document-contract/🟦️.ts"].map((file) => `${schemaRoot}/${file}`)], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-dag-dag", "--lib", ...(args[2] === "all" ? [] : ["dag_document_contract_"]), "--", "--nocapture"], root);
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
  const { testRecursiveOwnedDocumentReplacementOracle } = await import(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🟦️.ts`);
  testRecursiveOwnedDocumentReplacementOracle();
  runCmd("bun", [root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--resolveJsonModule", "--skipLibCheck", root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🟦️.ts"], { cwd: root });
  if (args[1] !== "oracle") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", args[2] ?? "retained_window_input", "--", "--nocapture"], root);
} else if (args[0] === "flow-window-ownership") {
  const { testFlowWindowOwnershipOracle } = await import(`${root}/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testFlowWindowOwnershipOracle();
  if (args[1] === "check") {
    await runCargo(["check", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-flow-flow", "--tests"], root);
  } else if (args[1] === "native") {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-flow-flow", "--lib", "flow_window_ownership_", "--", "--nocapture"], root);
  }
} else if (args[0] === "recursive-archive-history" && args[1] === "native") {
  await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "retained_history_decode_", "--", "--nocapture"], root);
} else if (args[0] === "forms-try-window-ownership") {
  const configRoot = `${root}/✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try/🎚️config`;
  const { testFormsTryWindowOwnership } = await import(`${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testFormsTryWindowOwnership();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${configRoot}/🧬️schema/🟦️.ts`, `${configRoot}/../🫧️transient/🧬️schema/🟦️.ts`, `${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-forms-forms", "--lib", "forms_try_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "note-empty-config-ownership") {
  const windowRoot = `${root}/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window`;
  const oracle = `${windowRoot}/🧪️tests/🔬️ownership/🟦️.ts`;
  const { testNoteEmptyConfigOwnership } = await import(oracle);
  testNoteEmptyConfigOwnership();
  const { toolJobMountedDispatchOneTurnExact } = await import(`${root}/📜️script.ts`);
  const workerSource = readFileSync(`${root}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, "utf8");
  const initialDrive = "let _ = active.drive_worker_step(&pool)?;";
  if (!toolJobMountedDispatchOneTurnExact(workerSource)
    || toolJobMountedDispatchOneTurnExact(workerSource.replace(initialDrive, ""))
    || toolJobMountedDispatchOneTurnExact(workerSource.replace(initialDrive, `${initialDrive} ${initialDrive}`))) throw new Error("Note worker dispatch must perform exactly one initial drive");
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${windowRoot}/🧬️schema/🟦️.ts`, oracle], { cwd: root });
  if (args[1] === "native") {
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-note-note", "--lib", "note_empty_config_owner_", "--", "--nocapture"], root);
  }
} else if (args[0] === "layout-window-ownership") {
  const windowsRoot = `${root}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows`;
  const configRoot = `${windowsRoot}/📐️blueprint/🎚️config`;
  const { testLayoutWindowOwnershipOracle } = await import(`${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testLayoutWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${configRoot}/🧬️schema/🟦️.ts`, `${windowsRoot}/📐️blueprint/🫧️transient/🧬️schema/🟦️.ts`, `${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-layout-layout", "--lib", "layout_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "sequence-window-ownership") {
  const windowsRoot = `${root}/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows`;
  const configRoot = `${windowsRoot}/📽️main/🎚️config`;
  const { testSequenceWindowOwnershipOracle } = await import(`${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testSequenceWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${configRoot}/🧬️schema/🟦️.ts`, `${windowsRoot}/📜️script/🫧️transient/🧬️schema/🟦️.ts`, `${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-sequence-sequence", "--lib", "sequence_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "gis-map-window-ownership") {
  const schemaRoot = `${root}/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗺️map/⚙️config`;
  const { testGisMapWindowOwnershipOracle } = await import(`${schemaRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`);
  testGisMapWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${schemaRoot}/🧬️schema/🟦️.ts`, `${schemaRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-gis-gismap", "--features", "component-app-assembly", "--lib", "gis_map_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "remodel-window-ownership") {
  const schemaRoot = `${root}/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes`;
  const testRoot = `${schemaRoot}/🧊️model/🪟️windows/🧊️model/🎚️config/🧪️tests/🔬️window-ownership`;
  const { testRemodelWindowOwnershipOracle } = await import(`${testRoot}/🟦️.ts`);
  testRemodelWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${schemaRoot}/🧊️model/🪟️windows/🧊️model/🎚️config/🧬️schema/🟦️.ts`, `${schemaRoot}/📷️capture/🪟️windows/🖼️frames/🎚️config/🧬️schema/🟦️.ts`, `${schemaRoot}/🔍️analyze/🪟️windows/📊️report/🎚️config/🧬️schema/🟦️.ts`, `${testRoot}/🟦️.ts`], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-remodel-remodeling", "--lib", "remodel_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "norm-results-window-ownership") {
  const configRoot = `${root}/✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config`;
  const oracle = `${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`;
  const { testNormResultsWindowOwnershipOracle } = await import(oracle);
  testNormResultsWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${configRoot}/🧬️schema/🟦️.ts`, `${configRoot}/🧬️schema/🧬️mutations/🟦️.ts`, oracle], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-norm-en1996", "--lib", "norm_results_window_ownership_", "--", "--nocapture"], root);
} else if (args[0] === "drawing-canvas-window-ownership") {
  const configRoot = `${root}/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🎚️config`;
  const transientRoot = `${root}/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🫧️transient`;
  const presenceRoot = `${root}/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence`;
  const oracle = `${configRoot}/🧪️tests/🔬️window-ownership/🟦️.ts`;
  const { testDrawingCanvasWindowOwnershipOracle } = await import(oracle);
  testDrawingCanvasWindowOwnershipOracle();
  runCmd("bun", [`${root}/node_modules/typescript/bin/tsc`, "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${configRoot}/🧬️schema/🟦️.ts`, `${transientRoot}/🧬️schema/🟦️.ts`, `${presenceRoot}/🧬️schema/🟦️.ts`, oracle], { cwd: root });
  if (args[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-draw-drawing", "--lib", "drawing_canvas_window_ownership_", "--", "--nocapture"], root);
} else {
  const { VerifyScript } = await import("../../../../../../../../📜️script.ts");
  await new VerifyScript(root, root).run(args);
}
