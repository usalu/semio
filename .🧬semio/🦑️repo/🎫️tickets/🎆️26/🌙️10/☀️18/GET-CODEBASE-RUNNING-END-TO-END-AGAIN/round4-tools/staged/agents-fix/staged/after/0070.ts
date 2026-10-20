import { dispatchOwnedScriptRoute } from "../../🏃️process/🧭️routing/🧩️contributions/🟦️.ts";
import { orchestratorBudgetOpts, runCmd } from "../../🏃️process/🟦️.ts";
import { semioBuildParallelFlag } from "../../🏃️process/🌿️environment/🟦️.ts";
import { policyArtifactOwnershipFieldParity } from "../../🧬️schema/🗿️artifact/⚖️laws/🪪️ownership-field-parity/🟦️.ts";
import { abstractionOwnershipChecks } from "../../📏️ownership/🏛️abstraction/✅️verification/🟦️.ts";
import { policyAbstractionOwnershipBreaches } from "../../📏️ownership/🏛️abstraction/⚖️law/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runTypescriptVerification } from "./🔷️typescript-project/🟦️.ts";
import { runAgnosticGate, runDebugTagCensusGate, runDocstringCensusGate, runDocstringEmojiGate, runInterfaceImportGate, runMutationCapGate, runSourceCensusGate } from "./🚦️gates/🟦️.ts";
import { VerifyGateLanes } from "./🏁️gate-lanes/🟦️.ts";

/** 🧾️ Parses each viewport GraphQL schema with the reference parser and refuses the first syntax error. */
async function parseViewportGraphqlSchemasV1(paths: readonly string[]): Promise<void> {
  const { parse } = await import("graphql");
  for (const path of paths) parse(readFileSync(path, "utf8"));
}

//#region 🔖️VerifyScript

export abstract class VerifyScript extends VerifyGateLanes {
  /**
   * 🧭️ Routes one `verify` lane to its checks.
   *
   * The `media-export-encoding` lane runs `tsc --strict` over the codec twin alone: the twin is self-contained by construction, whereas the law imports `🎠️kernel/🎬️media-export/🟦️.ts` and would drag the whole renderer graph (and its pre-existing, unrelated Bun-ism diagnostics) into the verdict of the lane.
   *
   * The `file-open-import` lane runs `tsc --strict` over the PAGE half alone: the page imports nothing, is the one module allowed to name `document` on this target, and must type-check against the DOM lib without dragging the renderer graph (and its unrelated diagnostics) into the verdict of the lane.
   */
  async run(segments: string[]): Promise<void> {
    if (await dispatchOwnedScriptRoute(this.invocation,this.root, ["verify", ...segments])) return;
    if (segments[0] === "canonical-architecture") {
      if (segments.length !== 1) throw new Error("Expected verify canonical-architecture");
      this.runCanonicalArchitecture();
      return;
    }
    if (segments[0] === "taxonomy") {
      await this.runTaxonomy(segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-outcome-law") {
      this.runMutationOutcomeLaw();
      return;
    }
    if (segments[0] === "history-closure") {
      this.runHistoryClosure(segments.slice(1));
      return;
    }
    if (segments[0] === "composed-child-refs") {
      this.runComposedChildRefs(segments.slice(1));
      return;
    }
    if (segments[0] === "semantic-vocabulary") {
      this.runSemanticVocabulary();
      return;
    }
    if (segments[0] === "package-purity") {
      this.runPackagePurity();
      return;
    }
    if (segments[0] === "build-owners") {
      this.runBuildOwners(segments.slice(1));
      return;
    }
    if (segments[0] === "zero-touch") {
      this.runZeroTouch(segments.slice(1));
      return;
    }
    if (segments[0] === "generated-corruption") {
      this.runGeneratedCorruption(segments.slice(1));
      return;
    }
    if (segments[0] === "build-parity") {
      const { runBuildParityV1 } = await import("../../🏗️build/🪞️parity/⌨️command/🟦️.ts");
      await runBuildParityV1(this.root, segments.slice(1), this.invocation);
      return;
    }
    if (segments[0] === "typescript") {
      await runTypescriptVerification(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "rust-warnings") {
      await this.runRustWarnings(segments.slice(1));
      return;
    }
    if (segments[0] === "production-placeholders") {
      await runSourceCensusGate(this.root, "placeholders");
      return;
    }
    if (segments[0] === "docstrings") {
      await (segments[1] === "emoji-unique" ? runDocstringEmojiGate(this.root, segments.slice(2)) : runDocstringCensusGate(this.root, segments[1]));
      return;
    }
    if (segments[0] === "agnostic") {
      await runAgnosticGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "mutation-caps") {
      await runMutationCapGate(this.root, segments.slice(1));
      return;
    }
    if (segments[0] === "debug-tags") {
      await runDebugTagCensusGate(this.root);
      return;
    }
    if (segments[0] === "interface-owners") {
      await runInterfaceImportGate(this.root);
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "commands") {
      await runSourceCensusGate(this.root, "commands");
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "tool-jobs") {
      await this.runToolJobCoverage(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "apps") {
      await this.runInteractivityApps(segments.slice(2));
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1q-b1-b6") {
      this.runInteractivityP1qB1B6();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1w") {
      this.runInteractivityP1w();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1x") {
      this.runInteractivityP1x();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1y") {
      this.runInteractivityP1y();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p1z") {
      this.runInteractivityP1z();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5d") {
      this.runInteractivityP5d();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p5e") {
      this.runInteractivityP5e();
      return;
    }
    if (segments[0] === "interactivity" && segments[1] === "p3mn") {
      this.runInteractivityP3mn();
      return;
    }
    if (segments[0] === "interactivity") {
      await this.runInteractivityAudit();
      return;
    }
    if (segments[0] === "dependencies") {
      this.runDependencyFreeze(segments.slice(1));
      return;
    }
    if (segments[0] === "layering") {
      this.runLayering(segments.slice(1));
      return;
    }
    if (segments[0] === "browser-actor-host-context") {
      if (segments[1] === "worker") {
        const result = Bun.spawn(["bun", join(this.root, "node_modules/vitest/vitest.mjs"), "run", "--config", join(this.root, "🧰️framework/🛍️products/💻️os/🧪️tests/🎚️config/🟦️.ts"), "-t", "browser document actor transfers one verified cold pair", "--maxWorkers=1", "--testTimeout=300000", "--silent=false", "--disableConsoleIntercept"], { cwd: this.root, stdout: "inherit", stderr: "inherit" });
        if (await result.exited !== 0) throw new Error("Browser actor host context worker test failed");
        return;
      }
      const { testBrowserActorHostContext } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🌐️browser-bundle/🪟️view-context/🧪️testing/🪟️host-opening-context/🟦️.ts");
      testBrowserActorHostContext();
      return;
    }
    if (segments[0] === "resolved-host-context") {
      const { testResolvedHostContext } = await import("../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts");
      testResolvedHostContext();
      return;
    }
    if (segments[0] === "host-ownership") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "--", "--nocapture", "--test-threads=1", ...["concrete_window_instances_round_trip_without_kind_collapse", "context_menu_point_resolves_the_exact_concrete_window_instance", "canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript", "build_os_commands_covers_every_wired_setting"]], this.root);
      return;
    }
    
    if (segments[0] === "shared-dynamic-value") {
      const { testSharedDynamicValueOracle } = await import("../../../../../../🔨️modules/🌱️value/📦️packages/🟦️typescript/🧪️testing/🔣️json-projection/🟦️.ts");
      testSharedDynamicValueOracle();
      return;
    }
    if (segments[0] === "shared-artifact-addressing") {
      const { testContentIdOracle, testSharedArtifactAddressingOracle } = await import("../../../../../💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🧪️testing/🪪️artifact-addressing/🟦️.ts");
      testSharedArtifactAddressingOracle();
      testContentIdOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "artifact_addressing", "--", "--nocapture"], this.root);
      return;
    }
    
    if (segments[0] === "retained-window-input") {
      const { testRetainedWindowInputOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🪟️window/🫧️transient/🧪️testing/🪟️retained-window-input/🟦️.ts");
      testRetainedWindowInputOracle();
      const { testRecursiveOwnedDocumentReplacementOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🧪️testing/🧩️composition/🟦️.ts");
      testRecursiveOwnedDocumentReplacementOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", segments[2] ?? "retained_window_input", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "publication-retirement-authority") {
      const { testPublicationRetirementAuthorityOracle } = await import("../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🧪️testing/♻️publication-retirement-authority/🟦️.ts");
      testPublicationRetirementAuthorityOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "publication_retirement", "--", "--nocapture"], this.root);
      return;
    }
    
    if (segments[0] === "dag-demo-ownership") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-artifact-infinite-dag", "--lib", "--", "--nocapture"], this.root);
      return;
    }
    
    
    
    if (segments[0] === "snapshot-read-retirement") {
      const { testSnapshotReadRetirement } = await import("../../../../../💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🧪️testing/♻️snapshot-read-retirement/🟦️.ts");
      testSnapshotReadRetirement();
      runCmd("bun", [this.root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", this.root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/♻️snapshot-read-retirement/🟦️.ts"], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "snapshot_read_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "semio-envelope-identity") {
      const { runCmd } = await import("../../🏃️process/🟦️.ts");
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      const ownerRoot = this.root + "/🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio";
      const { testSemioEnvelopeIdentity } = await import(ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts");
      testSemioEnvelopeIdentity();
      runCmd("bun", [this.root + "/node_modules/typescript/bin/tsc", "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--skipLibCheck", ownerRoot + "/🧪️tests/🪪️envelope-identity/🟦️.ts"], { cwd: this.root });
      if (segments[1] === "native") await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "semio_envelope_identity", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "reset-document-ownership") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "-p", "semio-s-artifact-trinity-rewriting", "--lib", "reset_document_ownership", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "wires-pointer-move") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-reasoning-wires", "--lib", "wires_pointer_move_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "jack-results-window-transient") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-trinity-jack", "--lib", "results_window_large_output_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-value-canonical-json") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "shared_value_canonical_json", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-value-clone") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_value_clone", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "shared-map-delta") {
      const { testSharedMapDeltaOracle } = await import("../../../../../../🔨️modules/📡️replication/📦️packages/🟦️typescript/🎮️mutation/🗂️map/🧪️testing/🧪️shared-map-delta-source/🟦️.ts");
      testSharedMapDeltaOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "shared_map_delta", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "artifact-contract-ownership") {
      abstractionOwnershipChecks(this.root);
      const fixture = JSON.parse(readFileSync(join(this.root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧫️fixtures/🧪️abstraction-ownership/🔣️.json"), "utf8")) as { artifactSchemas: string[] };
      const packages = [...new Set(fixture.artifactSchemas.map((path) => {
        const [owner] = path.split("/🏅️standards/");
        if (owner === undefined) throw new Error("Artifact schema owner required");
        const manifest = Bun.TOML.parse(readFileSync(join(this.root, owner, "📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { package: { name: string } };
        return manifest.package.name;
      }))];
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      const packageArgs = packages.flatMap((name) => ["-p", name]);
      if (segments[1] === "test") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "--no-fail-fast", ...packageArgs, "committed_diff", "--", "--nocapture"], this.root);
        console.log(`artifact-contract-ownership: committed diff generation, canonical encoding and application checks completed for ${packages.length} selected artifact crates.`);
        return;
      }
      await runCargo(["check", "--manifest-path", "Cargo.toml", "--tests", ...packageArgs], this.root);
      console.log(`[verify artifact-contract-ownership] ${packages.length} artifact crates and their tests compile.`);
      return;
    }
    
    if (segments[0] === "store-reload-publication") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "artifact_store_reset_preserves_capacity_for_retained_batch_publication", "--", "--nocapture"], this.root);
      return;
    }
    
    
    if (segments[0] === "window-config-provisioning-lifecycle") {
      if (segments.length !== 1) throw new Error("window-config-provisioning-lifecycle accepts no arguments");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🚪️provisioning-lifecycle/🟦️.ts");
      const { testWindowConfigProvisioningLifecycleOracle } = await import(testPath);
      testWindowConfigProvisioningLifecycleOracle();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      return;
    }
    if (segments[0] === "window-config-retained-pack-load") {
      if (segments.length !== 2 || segments[1] !== "native") throw new Error("window-config-retained-pack-load requires native");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🟦️.ts");
      const { testWindowConfigRetainedPackLoadFixture } = await import(testPath);
      testWindowConfigRetainedPackLoadFixture();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "window_config_retained_pack_load_", "--", "--nocapture", "--test-threads=1"], this.root);
      return;
    }
    
    
    
    
    
    
    
    
    
    
    
    if (segments[0] === "block3d-window-transient") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-block-3d", "--lib", "preview_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "ephemeral-transfer-preparation") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "ephemeral_transfer_preparation", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "owned-document-closure") {
      const { testOwnedDocumentClosureOracle } = await import("../../../../../💻️os/📦️packages/🟦️typescript/🔨️modules/🏪️store/🧩️composition/🌳️closure/🧪️testing/🔬️unit/🟦️.ts");
      testOwnedDocumentClosureOracle();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️closure/🟦️.ts")], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "owned_document_closure", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    
    
    
    
    if (segments[0] === "framework-viewport-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🪟️poses");
      const { testViewportOwnership } = await import(`${testRoot}/🟦️.ts`);
      testViewportOwnership();
      await parseViewportGraphqlSchemasV1([...["◻️2d", "🧊️3d"].map((dimension) => this.root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/" + dimension + "/🧬️schema/🔗️.graphql")]);
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-viewport", "--lib", "viewport_ownership_", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-kernel", "--lib", "viewport_ownership_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "framework-viewport-projection") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("framework-viewport-projection accepts only the optional native phase");
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection");
      const { testViewport3dProjectionValues } = await import(`${testRoot}/🟦️.ts`);
      testViewport3dProjectionValues();
      await parseViewportGraphqlSchemasV1([this.root + "/🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔗️.graphql"]);
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-viewport", "--lib", "viewport_projection_", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-scene", "--lib", "viewport_projection_pack_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "node-graph-viewport-ownership") {
      const nativePhase = segments[2];
      if (segments.length > 3 || (segments[1] !== undefined && segments[1] !== "native") || (nativePhase !== undefined && nativePhase !== "renderer-host" && nativePhase !== "consumers")) {
        throw new Error("node-graph-viewport-ownership accepts native with the optional renderer-host or consumers phase");
      }
      const flowRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust");
      const reactRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript");
      const surfaceRoot = join(this.root, "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust");
      runCmd("bun", [join(flowRoot, "📜️script.ts"), "declarations"], { cwd: flowRoot });
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts"), { runVitest } = await import("../../🧪️testing/🧪️vitest/🟦️.ts");
      process.env.SEMIO_TEST_LEVEL = "long";
      await runVitest(reactRoot, ["--silent=false", "--reporter=verbose", "-t", "uses the live session viewport|matches the shared neutral viewport schema"], "../../🧪️tests/🎚️config/🟦️.ts");
      if (segments[1] === "native") {
        if (nativePhase === undefined || nativePhase === "renderer-host") {
          runCmd("bun", [join(surfaceRoot, "📜️script.ts"), "wasm"], { cwd: surfaceRoot, env: { ...process.env, SEMIO_WORKSPACE_ROOT: this.root, REPO_ROOT: this.root } });
          for (const [packageName, filters] of [
            ["semio-framework-ui-scene", ["typed_scene_neutral_catalog_matches_native_serde_contracts"]],
            ["semio-framework-surface-node-graph", ["scene_retained_tests::"]],
            ["semio-framework-os-renderer-wgpu", ["node_graph_attach_tests", "concrete_window_instances_round_trip_without_kind_collapse", "context_menu_point_resolves_the_exact_concrete_window_instance", "canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript", "build_os_commands_covers_every_wired_setting"]],
          ] as const) {
            for (const filter of filters) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", filter, "--", "--nocapture"], this.root);
          }
        }
        if (nativePhase === undefined || nativePhase === "consumers") {
          const consumers = [
          ["semio-s-artifact-architect-program", ["every_command_round_trips_text_and_binary_under_its_declared_wire_keyword"]],
          ["semio-s-artifact-reasoning-wires", ["every_command_round_trips_through_text_and_binary"]],
          ["semio-hub-space", ["node_graph_viewport_writes_typed_workflow_camera_config"]],
          ["semio-s-artifact-dag-dag", ["every_command_round_trips_through_text_and_binary"]],
          ["semio-s-artifact-flow-flow", ["node_graph_viewport_moves_the_camera"]],
          ["semio-s-artifact-mathematical-equation", ["node_graph_viewport_writes_config_not_mutations"]],
          ["semio-s-artifact-trinity-rewriting", ["trinity_rewriting_command_text_and_binary_round_trip"]],
          ["semio-s-artifact-trinity-jack", ["trinity_jack_command_text_and_binary_round_trip"]],
          ] as const;
          const filters = [...new Set(consumers.flatMap(([, filters]) => [...filters]))];
          await runCargo(["test", "--manifest-path", "Cargo.toml", ...consumers.flatMap(([packageName]) => ["-p", packageName]), "--lib", "--", "--nocapture", "--test-threads=1", ...filters], this.root);
          for (const packageName of ["semio-s-artifact-procedural-2d-editor", "semio-s-artifact-procedural-generation3d"]) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--test", "editor-routes", "--", "--nocapture", "--test-threads=1", "every_command_round_trips_through_text_and_binary"], this.root);
        }
      }
      return;
    }
    
    if (segments[0] === "framework-job-physical-close") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close");
      const { testJobPayloadPhysicalClose } = await import(`${testRoot}/🟦️.ts`);
      testJobPayloadPhysicalClose();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-job", "--lib", "--", "--nocapture", "--test-threads=1"], this.root);
      return;
    }
    if (segments[0] === "framework-retained-pack-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws");
      const { testRetainedPackPhysicalOwnership } = await import(`${testRoot}/🟦️.ts`);
      testRetainedPackPhysicalOwnership();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "outer-native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.root);
        return;
      }
      if (segments[1] === "catalog-native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_catalog_", "--", "--nocapture"], this.root);
        return;
      }
      if (segments[1] === "value") return;
      if (segments[1] === "diagnostic") return;
      if (segments[1] === "inflater") return;
      if (segments[1] === "inflater-native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-deflate", "--lib", "retained_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_inflater_physical_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_pipeline_diagnostic_deflate_", "--", "--nocapture", "--test-threads=1"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture", "--test-threads=1"], this.root);
        for (const packageName of ["semio-s-artifact-procedural-2d-io-binary", "semio-s-artifact-procedural-3d-io-binary"]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", "retained_mounted_laws", "--", "--nocapture", "--test-threads=1"], this.root);
        }
        return;
      }
      if (segments[1] === "diagnostic-native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        for (const filter of [
          "retained_pack_pipeline_diagnostic_",
          "retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable",
          "retained_anchor_rejects_hostile_crc_and_requires_explicit_close",
        ]) {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", filter, "--", "--nocapture"], this.root);
        }
        return;
      }
      if (segments[1] === "value-native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        for (const [packageName, filters] of [
          ["semio-framework-replication", ["retained_paged_list_capacity_admission_and_exact_release_grants"]],
          ["semio-framework-pack", ["retained_symbol_table_", "retained_pack_catalog_", "write_then_read_round_trip_with_compressed_segment_and_chunk"]],
          ["semio-framework-os-kernel", ["retained_value_", "retained_record_body_"]],
          ["semio-s-artifact-procedural-2d-host", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-2d-io-binary", ["retained_mounted_laws"]],
          ["semio-s-artifact-procedural-generation3d", ["every_fourteen_variant_decodes_through_retained_structural_grants"]],
          ["semio-s-artifact-procedural-3d-io-binary", ["retained_mounted_laws"]],
        ] as const) {
          for (const filter of filters) await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", filter, "--", "--nocapture"], this.root);
        }
        return;
      }
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "retained_pack_source_laws", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-pack", "--lib", "write_then_read_round_trip_with_compressed_segment_and_chunk", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-2d-host", "--lib", "retained_pack_outer_", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-procedural-3d-io-binary", "--lib", "retained_mounted_laws", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    if (segments[0] === "framework-paged-list-ownership") {
      const testRoot = join(this.root, "🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list");
      const { testPagedListOwnership } = await import(`${testRoot}/🟦️.ts`);
      testPagedListOwnership();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--esModuleInterop", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-replication", "--lib", "value::list::", "--", "--nocapture"], this.root);
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-ui-contract", "--lib", "fixed_list_", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "framework-variant-field-casing") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-value-derive", "--test", "variant_field_casing", "--", "--nocapture"], this.root);
      return;
    }
    if (segments[0] === "framework-empty-state-contract") {
      const testRoot = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🚫️empty-state");
      const { testFrameworkEmptyStateContract } = await import(`${testRoot}/🟦️.ts`);
      testFrameworkEmptyStateContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${testRoot}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "framework_empty_state_contract_", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "window-config-pack-identity") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("window-config-pack-identity accepts only optional native");
      const testPath = join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🪪️pack-identity/🟦️.ts");
      const { testWindowConfigPackIdentityContract } = await import(testPath);
      testWindowConfigPackIdentityContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "window_config_pack_identity_", "--", "--nocapture", "--test-threads=1"], this.root);
      }
      return;
    }
    
    
    
    
    
    
    
    
    if (segments[0] === "media-export-encoding") {
      const { testMediaExportEncodingContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/⬇️media-export-encoding/🟦️.ts");
      testMediaExportEncodingContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🔨️modules/🚪️io/🔤️base64")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-io-base64", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "media_export_encoding", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "video-render-export") {
      const { testVideoRenderProgramContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/🎞️video-render-program/🟦️.ts");
      const { testVideoRenderJobContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/🧵️video-render-job/🟦️.ts");
      const { testRasterVideoAvcPcmContract } = await import("../../../../../../📦️packages/🟦️typescript/🖌️raster/🎥️video/🧪️testing/🔬️unit/🟦️.ts");
      const { testVideoRenderHostContract } = await import("../../../../../💻️os/📦️packages/🟦️typescript/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎥️VideoRenderHost/🧪️testing/🔬️unit/🟦️.ts");
      testVideoRenderProgramContract();
      testVideoRenderJobContract();
      testRasterVideoAvcPcmContract();
      await testVideoRenderHostContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🔨️modules/🖌️raster/🎥️video")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "video_render", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-raster", "--lib", "video", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "wire_effect_round_trip", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-animate-presentation", "--lib", "export_video", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "file-open-import") {
      const { testFileOpenImportContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/📤️file-open-import/🟦️.ts");
      testFileOpenImportContract();
      runCmd("bun", [join(this.root, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--lib", "ESNext,DOM", "--module", "ESNext", "--moduleResolution", "bundler", "--allowImportingTsExtensions", "--skipLibCheck", `${join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚪️host-io")}/🟦️.ts`], { cwd: this.root });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "file_open_import", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "file_open_import", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "ui-dirty-scope") {
      const { testUiDirtyScopeContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/🐢️ui-dirty-scope/🟦️.ts");
      testUiDirtyScopeContract();
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.root);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "ui_dirty_scope", "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "spawned-job-drive") {
      const { testSpawnedJobDriveContract } = await import("../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/🧪️testing/🧵️spawned-job-drive/🟦️.ts");
      await testSpawnedJobDriveContract();
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "spawned_job_drive", "--", "--nocapture"], this.root);
      }
      return;
    }
    
    
    
    
    
    
    
    if (segments[0] === "window-action-context") {
      const { testWindowActionContextOracle } = await import("../../../../../💻️os/📦️packages/🟦️typescript/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️testing/🔬️window-action-context/🟦️.ts");
      testWindowActionContextOracle();
      if (segments[1] === "oracle") return;
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      for (const filter of ["window_action_context", "apply_drop_committed_queues_the_merged_action_into_input"]) {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", filter, "--", "--nocapture"], this.root);
      }
      return;
    }
    if (segments[0] === "surface-view-context") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", segments[1] === "menu" ? "context_menu_wire" : "surface_context", "--", "--nocapture"], join(this.root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"));
      return;
    }
    if (segments[0] === "view-context-integer-carriers") {
      const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework", "--lib", "view_context_integer_carrier", "--", "--nocapture"], this.root);
      if (segments[1] === "native") return;
      runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-react:view-state-carriage-check"], { cwd: this.root });
      return;
    }
    if (segments[0] === "window-view-context") {
      if (segments[1] === "native") {
        const { runCargo } = await import("../../🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "--lib", "window_view_context_uses_the_addressed_instance"], join(this.root, "🧰️framework/📦️packages/🦀️rust"));
        return;
      }
      const { testWindowViewContext } = await import("../../../../../../🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts");
      testWindowViewContext();
      return;
    }
    if (segments[0] === "artifact-field-parity") {
      const { testArtifactFieldParityOracle } = await import("../../📏️ownership/🧪️tests/🪪️field-parity/🟦️.ts");
      testArtifactFieldParityOracle();
      if (segments[1] === "test") return;
      const breaches = policyArtifactOwnershipFieldParity(this.root);
      for (const breach of breaches) console.log(`[verify artifact-field-parity] ${breach.path}: missing=${breach.missing.join(",")} extra=${breach.extra.join(",")}`);
      console.log(`[verify artifact-field-parity] ${breaches.length} schema representations disagree with the canonical document field sets.`);
      if (breaches.length && segments[1] !== "report") throw new Error("Artifact schema representations disagree on document fields.");
      return;
    }
    if (segments[0] === "abstraction-ownership") {
      abstractionOwnershipChecks(this.root);
      if (segments[1] === "test") return;
      const breaches = policyAbstractionOwnershipBreaches(this.root);
      for (const breach of breaches) console.log(`[verify abstraction-ownership] ${breach.scope}: ${breach.summary}`);
      console.log(`[verify abstraction-ownership] ${breaches.length} misplaced declarations.`);
      if (breaches.length && segments[1] !== "report") throw new Error("OS-owned preferences or host controls are declared by plugin surfaces.");
      return;
    }
    if (segments[0] === "policy-breach") {
      this.runPolicyBreach(segments[1]);
      return;
    }
    await this.runGate();
    if (segments[0] === "gate") return;
    runCmd("bun", ["repo", "run-many", "-t", "test", "--all", "--exclude", "workspace", ...semioBuildParallelFlag()], { cwd: this.root, ...orchestratorBudgetOpts() });
  }
}
//#endregion 🔖️VerifyScript
