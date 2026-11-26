import { copyFileSync, readFileSync, statfsSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";

const root = process.cwd();
const registry = Bun.spawnSync(["repo", "tools", "list", "--json"], { stderr: "inherit" });
if (registry.exitCode) process.exit(registry.exitCode);
const tools = JSON.parse(registry.stdout.toString());
const rustup = tools.find((tool: { id: string }) => tool.id === "rustup");
const toolchain = readFileSync(resolve(root, "rust-toolchain.toml"), "utf8").match(/^channel\s*=\s*"([^"]+)"/m)?.[1];
if (!rustup?.command || !toolchain) throw new Error("Pinned local Rust toolchain is missing");
const cacheHome = resolve(root, ".cache/b/r/m/cargo/home");
const artifactRoot = resolve(import.meta.dir, "🗑️generated", process.argv.includes("--isolated-gateway") ? "native-host" : "native-mcp");
const profile = readFileSync(resolve(root, "Cargo.toml"), "utf8").includes("[profile.prototype]") ? "prototype" : "dev";
const compiler = Bun.spawnSync([rustup.command, "which", "--toolchain", toolchain, "rustc"], { stderr: "inherit" });
if (compiler.exitCode) process.exit(compiler.exitCode);
const storage = statfsSync(root);
console.error("[DEBUG] Native cargo:", rustup.command, toolchain, profile, Math.floor(storage.bavail * storage.bsize / 2 ** 30), "GiB available");
if (process.argv.includes("--inspect")) process.exit(0);
mkdirSync(artifactRoot, { recursive: true });
const env = { ...process.env, SEMIO_BUILD_MODE: "prototype", RUSTC: compiler.stdout.toString().trim(), RUSTC_WRAPPER: "", RUSTFLAGS: "-Zthreads=1", CARGO_HOME: cacheHome, CARGO_TARGET_DIR: artifactRoot, CARGO_BUILD_BUILD_DIR: artifactRoot, CARGO_INCREMENTAL: "0", CARGO_PROFILE_DEV_DEBUG: "0", CARGO_PROFILE_PROTOTYPE_DEBUG: "0" };
delete env.CARGO_BUILD_TARGET_DIR;
delete env.RUSTC_WORKSPACE_WRAPPER;
const bitmapWorkspace = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap";
if (process.argv.includes("--bitmap-lock-refresh")) {
  const workspaceManifest = resolve(bitmapWorkspace, "Cargo.toml");
  copyFileSync(resolve(bitmapWorkspace, "Cargo.lock"), resolve(import.meta.dir, "🗑️generated", "bitmap-previous.lock"));
  const inspection = Bun.spawnSync([rustup.command, "run", toolchain, "cargo", "metadata", "--offline", "--format-version", "1", "--manifest-path", workspaceManifest], { cwd: root, stdout: "ignore", stderr: "inherit", env });
  process.exit(inspection.exitCode);
}
const catalogTest = process.argv.includes("--catalog-test");
const folderTest = process.argv.includes("--folder-test");
const folderReopenTest = process.argv.includes("--folder-reopen-test");
const exportCancellationTest = process.argv.includes("--export-cancellation-test");
const artifactReplyTest = process.argv.includes("--artifact-reply-test");
const producerRefusalTest = process.argv.includes("--producer-refusal-test") || artifactReplyTest;
const contextTest = process.argv.includes("--actor-context-test");
const responseReserveTest = process.argv.includes("--response-reserve-test");
const dispatchReceiptTest = process.argv.includes("--dispatch-receipt-test");
const pollFailureRetirementTest = process.argv.includes("--poll-failure-retirement-test");
const mediaWireTest = process.argv.includes("--media-wire-test");
const pagedPngTest = process.argv.includes("--paged-png-test");
const pollRefusalTest = process.argv.includes("--poll-refusal-test");
const interpreterScalarTest = process.argv.includes("--interpreter-scalar-test");
const coreGlobalsTest = process.argv.includes("--core-globals-test");
const sourceClockReplyTest = process.argv.includes("--source-clock-reply-test");
const actorFailureSelectorTest = process.argv.includes("--actor-failure-selector-test");
const actorFailureCoreSelectorTest = process.argv.includes("--actor-failure-core-selector-test");
const actorInputTest = process.argv.includes("--actor-input-test");
const inferenceSliceTest = process.argv.includes("--inference-slice-test");
const inferenceGroupTest = process.argv.includes("--inference-group-test");
const bitmapWorkerTest = process.argv.includes("--bitmap-worker-test");
const hostEscrowTest = process.argv.includes("--host-escrow-test");
const postTurnTest = process.argv.includes("--post-turn-test");
const hostWakeTest = process.argv.includes("--host-wake-test");
const guardReclaimTest = process.argv.includes("--guard-reclaim-test");
const packageName = process.argv.includes("--describe") ? "semio-plugin-describe" : "semio-os-mcp";
const testing = catalogTest || folderTest || folderReopenTest || exportCancellationTest || producerRefusalTest || contextTest || responseReserveTest || dispatchReceiptTest || pollFailureRetirementTest || mediaWireTest || pagedPngTest || pollRefusalTest || interpreterScalarTest || coreGlobalsTest || sourceClockReplyTest || actorFailureSelectorTest || actorFailureCoreSelectorTest || actorInputTest || inferenceSliceTest || inferenceGroupTest || bitmapWorkerTest || hostEscrowTest || hostWakeTest || postTurnTest || guardReclaimTest;
const args = [testing ? "test" : "build", "--locked", "--profile", profile, "-j", testing || process.argv.includes("--serial") ? "1" : "2", "-p",
  ...(catalogTest ? ["semio-os-mcp-catalog", "--lib", "model_manifest_binds_the_exact_mode_crate_and_plugin"] :
    folderTest ? ["semio-os-store-sync-native", "--lib", "folder_durability_tests"] :
    folderReopenTest ? ["semio-os-mcp", "--test", "folder_reopen"] :
    exportCancellationTest ? ["semio-os-mcp", "--test", "export_cancellation"] :
    artifactReplyTest ? ["semio-os-mcp-dispatch", "--lib", "every_fault_code_maps_to_the_right_gateway_error_code"] :
    producerRefusalTest ? ["semio-plugin-host-routers", "-p", "semio-os-mcp-plugin-runtime", "--lib", "producer_refusal"] :
    contextTest ? ["semio-plugin-guest-contract", "--lib", "original_core_wire_publisher_is_ready_in_the_first_paid_bootstrap_borrow"] :
    responseReserveTest ? ["semio-plugin-guest-contract", "--lib", "first_response_quote_refuses_an_insufficient_scope_before_source_or_producer_effects"] :
    dispatchReceiptTest ? ["semio-plugin-guest-contract", "--lib", "fixed_receipt_prices_this_dispatch_against_its_resident_baseline"] :
    pollFailureRetirementTest ? ["semio-plugin-guest-contract", "--lib", "close_output_retires_only_accepted_poll_causes_before_reentry"] :
    mediaWireTest ? ["semio-plugin-host-wit", "--lib", "original_media_control_preserves_all_wire_fields"] :
    pagedPngTest ? ["semio-framework-pixels-tests", "--lib", "png_decode_moves_paged_original_across_chunk_seams_under_exact_grants"] :
    pollRefusalTest ? ["semio-plugin-host-poll", "--lib"] :
    interpreterScalarTest ? ["semio-plugin-interpreter-runtime", "--test", "original-scalar-call"] :
    coreGlobalsTest ? ["semio-plugin-interpreter", "--lib", "inline_globals_require_the_actual_funded_backing_return"] :
    actorFailureSelectorTest ? ["semio-plugin-host-actor-failure", "--lib", "actor_failure_selector_"] :
    actorFailureCoreSelectorTest ? ["semio-plugin-interpreter", "--lib", "actor_failure_selector_"] :
    sourceClockReplyTest ? ["semio-plugin-interpreter-runtime", "--lib", "original_source_clock_same_callback_matches_paid_staging"] :
    actorInputTest ? ["semio-plugin-actor-input", "--lib"] :
    inferenceSliceTest ? ["semio-plugin-reactor-jobs", "--lib", "original_inference_job_batches_paid_slices_and_preserves_exact_wire"] :
    inferenceGroupTest ? ["semio-plugin-reactor-jobs", "--lib", "infer::tests::"] :
    bitmapWorkerTest ? ["semio-s-artifact-wfc-bitmap-tests-shrinks-output-pin", "--features", "testkit", "--lib", "bitmap_worker_cancellation_accounts_for_owned_solver_storage"] :
    hostEscrowTest ? ["semio-plugin-host-wake", "--lib", "escrow"] :
    postTurnTest ? ["semio-plugin-host-post-turn-relay", "--lib", "capture_tests"] :
    hostWakeTest ? ["semio-plugin-host-wake", ...(process.argv.includes("--unit-only") ? ["--lib"] : [])] :
    guardReclaimTest ? ["semio-framework-job-return", "--lib", "full_escrow_reclaims_one_acknowledged_row_before_minting_its_guard"] :
    [packageName, "--bin", packageName])];
if (process.argv.includes("--with-descriptor")) args.push("-p", "semio-plugin-describe", "--bin", "semio-plugin-describe");
if (pagedPngTest || actorInputTest || actorFailureSelectorTest) args.push("--manifest-path", "🧰️framework/Cargo.toml");
if (bitmapWorkerTest) args.push("--manifest-path", resolve(bitmapWorkspace, "Cargo.toml"));
if (!process.argv.includes("--online")) args.push("--offline");
const optimizeNativeHotPath = process.argv.includes("--optimize-native-hot-path");
if (optimizeNativeHotPath || process.argv.includes("--optimize-interpreter")) {
  const source = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json");
  const policy = JSON.parse(readFileSync(source, "utf8"));
  const native: { package: string; optLevel: string }[] = policy.groups[policy.owners["Cargo.toml"]].native;
  const selected = native.filter(row => row.package.startsWith(optimizeNativeHotPath ? "semio-" : "semio-plugin-interpreter"));
  for (const row of selected) {
    args.push("--config", `profile.${profile}.package.${row.package}.opt-level=${row.optLevel}`);
  }
  console.error(`[DEBUG] Native optimization (${profile}) ${selected.map(row => `${row.package}=${row.optLevel}`).join(", ")}`);
}
if (process.argv.includes("--optimize-poll-ledger")) {
  for (const name of ["semio-framework-job-context", "semio-framework-job-payload", "semio-framework-job-return-budget"]) {
    args.push("--config", `profile.${profile}.package.${name}.opt-level=2`);
  }
}
if (process.argv.includes("--physical-trace")) args.push("-p", "semio-plugin-interpreter", "--features", "semio-plugin-interpreter/testkit");
if (process.argv.includes("--unit-graph")) args.push("-Zunstable-options", "--unit-graph");
const testArguments = process.argv.indexOf("--");
if (testing && testArguments >= 0) args.push(...process.argv.slice(testArguments));
const child = Bun.spawn([rustup.command, "run", toolchain, "cargo", ...args], { cwd: root, stdout: "inherit", stderr: "inherit", env });
for (const signal of ["SIGINT", "SIGTERM"] as const) process.on(signal, () => child.kill(signal));
process.exit(await child.exited);
