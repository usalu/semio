# Framework Config Incoming Deltas


PATH 🧰️framework/📦️packages/🟦️typescript/📋️project.json HEAD 4003 INCOMING 4488
--- BASE
+++ STASH
@@ -9,4 +9,12 @@
   },
   "targets": {
+    "window-utility-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": { "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🟦️typescript -- bun ./📜️script.ts window-utility-check", "cwd": "🧰️framework" },
+      "metadata": { "semio": { "workspaceCommand": ["window-utility-check"] } }
+    },
     "test-artifact-reference": {
       "executor": "repo:script",

PATH 🧰️framework/📦️packages/🟦️typescript/📜️script.ts HEAD 32744 INCOMING 33820
--- BASE
+++ STASH
@@ -1,3 +1,4 @@
 #!/usr/bin/env bun
+
 import{createIssuedResourceControlV1}from"../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
 import { typecheckRepositoryProjectV1 } from "../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🟦️typescript/🟦️.ts";
@@ -423,4 +424,15 @@
 }
 
+/** 🪟️ Verifies shared utility routing and window ownership against the neutral context corpus. */
+class WindowUtilityScript extends BundleScript {
+  async run(args: string[]): Promise<void> {
+    if (args.length) throw Error("window-utility-check accepts no arguments");
+    const source = fileURLToPath(new URL("../../🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts", import.meta.url));
+    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source], this.root, "framework:window-utility-types", 120_000, { signal: this.invocation.control.signal });
+    const { testWindowViewContext } = await import("../../🔨️modules/🛂️manifest/🧪️tests/🔬️window-view-context/🟦️.ts");
+    testWindowViewContext();
+  }
+}
+
 class TestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
@@ -447,5 +459,5 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript).register("installation-identity-check", InstallationIdentityScript).register("test-script-boundary", ScriptBoundaryScript).register("test-artifact-reference", ArtifactReferenceScript).register("test-io-ports", IoPortsScript).register("test-source-text", SourceTextScript).register("test-document-scope", DocumentScopeScript).register("test-polygon-mesh", PolygonMeshScript).register("test-sdk-surface", SdkSurfaceScript).register("test-module-package-surface", ModulePackageSurfaceScript);
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript).register("installation-identity-check", InstallationIdentityScript).register("test-script-boundary", ScriptBoundaryScript).register("test-artifact-reference", ArtifactReferenceScript).register("test-io-ports", IoPortsScript).register("test-source-text", SourceTextScript).register("test-document-scope", DocumentScopeScript).register("test-polygon-mesh", PolygonMeshScript).register("test-sdk-surface", SdkSurfaceScript).register("test-module-package-surface", ModulePackageSurfaceScript).register("window-utility-check", WindowUtilityScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/📦️packages/🦀️rust/📋️project.json HEAD 12619 INCOMING 16069
--- BASE
+++ STASH
@@ -3,4 +3,52 @@
   "$schema": "../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json",
   "targets": {
+    "test-staged-payload-funding": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust --manifest 📦️packages/🦀️rust/Cargo.toml --test-manifest 🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml --test-manifest 🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-staged-payload-funding"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-staged-payload-funding"
+          ]
+        }
+      }
+    },
+    "test-retained-pack-encoding": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust --manifest 📦️packages/🦀️rust/Cargo.toml --test-manifest 🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-retained-pack-encoding"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-pack-encoding"
+          ]
+        }
+      }
+    },
+    "test-window-utility-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust --manifest 📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-window-utility-native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-window-utility-native"
+          ]
+        }
+      }
+    },
     "test-source-projection": {
       "executor": "repo:script",
@@ -225,4 +273,32 @@
         "forwardAllArgs": true
       }
+    },
+    "test-original-factory-admission": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default",
+        "^default",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🎯️action-bus/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/**/*"
+      ],
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust --manifest 📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-original-factory-admission"
+      }
+    },
+    "test-original-factory-admission-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default",
+        "^default",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🎯️action-bus/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/**/*"
+      ],
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 📦️packages/🦀️rust -- bun ./📜️script.ts test-original-factory-admission source"
+      }
     }
   }

PATH 🧰️framework/📦️packages/🦀️rust/📜️script.ts HEAD 29026 INCOMING 35341
--- BASE
+++ STASH
@@ -1,3 +1,4 @@
 #!/usr/bin/env bun
+
 import {runRepositoryCargoCommandV1} from "../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import {runRepositoryCargoTestsV1 as runCargoTestsV1,runRepositoryCargoLintV1 as runCargoLintV1} from "../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
@@ -150,4 +151,16 @@
 }
 
+
+/** ⛽️ Proves original shared staged payload source, page birth and commit funding against Buffer and the system allocator. */
+class StagedPayloadFundingTestScript extends BundleScript {
+ async run(segments: string[]): Promise<void> {
+  const { rest } = resolveTestLevel(segments);
+  await runBudgetedTestCommand(process.execPath, ["test", resolve(this.repoRoot, "🔨️modules/🧵️job/🧪️tests/⛽️staged-projection/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });
+  await testCargo(this.repoRoot, "🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml", ["semio-framework-job"], rest.length ? rest : ["--lib", "-E", "test(original_staged_projection_admits_source_and_writer_frontiers_independently) | test(original_staged_projection_reuses_its_retained_refused_backing_without_allocation) | test(original_boxed_staged_payload_initializes_and_projects_under_fixed_small_copy_grants) | test(original_in_place_payload_builder_preserves_fixed_grants_source_pages_and_all_cancellation_frontiers) | test(original_payload_authority_admits_once_and_borrowed_contexts_preserve_original_ledger_and_receipts) | test(original_payload_pages_preserve_narrow_capacity_exact_extents_and_native_conservation) | test(original_session_frame_return_has_exact_custody) | test(payload_retirement::tests)", "--status-level", "pass", "--final-status-level", "all", "--", "--nocapture"]);
+  await runBudgetedTestCommand(process.execPath, ["test", resolve(this.repoRoot, "🔨️modules/⏯️tool-run/🎞️tick/🛫️encode/🧪️tests/🟦️.ts"), resolve(this.repoRoot, "🔨️modules/⏯️tool-run/🎞️tick/⏭️successor/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/📨️payload/🌊️stream/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/📨️payload/🌊️stream/📡️wire/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true });
+  await testCargo(this.repoRoot, "🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml", ["semio-framework-tool-run"], ["--lib", "-E", "test(original_tick_wire_preserves_writer_columns_and_fixed_grants) | test(original_tick_successor_preserves_original_native_columns_and_funded_metadata_receipt) | test(original_payload_stream)", "--status-level", "pass", "--final-status-level", "all", "--", "--nocapture"]);
+ }
+}
+
 class CoreModulesTestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
@@ -159,4 +172,7 @@
 
 /** 🗜️ Verifies admitted physical compression against the independent zlib and miniz oracles. */
+class RetainedPackEncodingTestScript extends BundleScript{
+ async run(segments:string[]):Promise<void>{const{rest}=resolveTestLevel(segments);await runBudgetedTestCommand(process.execPath,["test",resolve(this.repoRoot,"🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});await testCargo(this.repoRoot,"🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml",["semio-framework-pack"],rest.length?rest:["--lib","-E","test(borrowed_projected_pack_section_replay_keeps_original_symbols_and_text_views) | test(borrowed_projected_pack_original_expression_ast_matches_independent_corpus_and_cancellation) | test(borrowed_projected_pack_standard_map_statements_wire_match_portable_oracle) | test(borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages) | test(borrowed_pack_document_default_codec_exact_original_output_and_cancellation) | test(borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec) | test(borrowed_pack_document_multiple_original_frames_match_canonical_oracle) | test(borrowed_pack_document_raw_body_limit_retains_refusal_receipt_and_output_custody) | test(borrowed_pack_document_source_depth_refuses_explicitly_without_grant_stall) | test(borrowed_pack_document_standard_chunks_preserve_original_and_independent_hash) | test(borrowed_pack_document_chunk_frontiers_cancel_with_exact_original_retirement) | test(required_chunked_forgery_with_valid_header_and_footer_crc_is_refused) | test(record_borrowed_schema_hash_retains_exact_paged_work_and_cancellation) | test(retained_pack_required_chunked_matches_actual_custody_and_independent_crc)","--status-level","pass","--final-status-level","all","--","--nocapture"]);}
+}
 class DeflateEncodingTestScript extends BundleScript{
   async run(segments:string[]):Promise<void>{
@@ -311,8 +327,21 @@
  }
 }
+/** 🧰️ Runs the shared exact-instance utility laws through the original framework owner. */
+class WindowUtilityNativeScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{
+  if(segments.length)throw Error("test-window-utility-native accepts no arguments");
+  await testCargo(this.repoRoot,"📦️packages/🦀️rust/Cargo.toml",["semio-framework"],["--lib","manifest::window_view_context_tests","--","--nocapture"]);
+ }
+}
 const workspace=process.env.SEMIO_COMMAND_WORKSPACE;
 if(!workspace||!isAbsolute(workspace))throw Error("Explicit General command workspace required");
-const router = new ScriptRouter(import.meta.dir,workspace).register("test-view-model-original",ViewModelOriginalScript).register("test-source-projection",SourceProjectionScript).register("test-directory-discovery",DirectoryDiscoveryScript).register("test-command-ingress-consumer",CommandIngressConsumerScript).register("test-artifact-kind-source", ArtifactKindSourceScript).register("test", TestScript).register("test-fixture-ownership-source", FixtureOwnershipSourceScript).register("test-action-choices", ActionChoicesTestScript).register("test-tool-run-actions", ToolRunActionsTestScript).register("test-history-edit-actions", HistoryEditActionsTestScript).register("test-history-progress", HistoryProgressTestScript).register("test-mutation-inputs", MutationInputsTestScript).register("test-host-effect-invocation", HostEffectInvocationTestScript).register("test-snapshot-sqlite", SnapshotSqliteTestScript).register("test-core-modules", CoreModulesTestScript).register("test-deflate-encoding",DeflateEncodingTestScript).register("test-package-descriptor-value-codec", PackageDescriptorValueCodecTestScript).register("test-wire-retirement-source", WireRetirementSourceScript).register("test-wire-retirement-native", WireRetirementNativeScript).register("test-argument-retirement-native",ArgumentRetirementNativeScript).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript).register("lint", LintScript);
+
+/** 🎟️ Exercises portable admission decisions and actual original-source factory capabilities. */
+class OriginalFactoryAdmissionScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length>1||(args.length===1&&args[0]!=="source"))throw Error("test-original-factory-admission [source]");await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework"],cwd:this.root,extraArgs:["--lib","action_bus::","--","--nocapture"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);}}
+
+const router = new ScriptRouter(import.meta.dir,workspace).register("test-view-model-original",ViewModelOriginalScript).register("test-source-projection",SourceProjectionScript).register("test-directory-discovery",DirectoryDiscoveryScript).register("test-command-ingress-consumer",CommandIngressConsumerScript).register("test-artifact-kind-source", ArtifactKindSourceScript).register("test", TestScript).register("test-fixture-ownership-source", FixtureOwnershipSourceScript).register("test-action-choices", ActionChoicesTestScript).register("test-tool-run-actions", ToolRunActionsTestScript).register("test-history-edit-actions", HistoryEditActionsTestScript).register("test-history-progress", HistoryProgressTestScript).register("test-mutation-inputs", MutationInputsTestScript).register("test-host-effect-invocation", HostEffectInvocationTestScript).register("test-snapshot-sqlite", SnapshotSqliteTestScript).register("test-core-modules", CoreModulesTestScript).register("test-deflate-encoding",DeflateEncodingTestScript).register("test-retained-pack-encoding",RetainedPackEncodingTestScript).register("test-staged-payload-funding",StagedPayloadFundingTestScript).register("test-package-descriptor-value-codec", PackageDescriptorValueCodecTestScript).register("test-wire-retirement-source", WireRetirementSourceScript).register("test-wire-retirement-native", WireRetirementNativeScript).register("test-argument-retirement-native",ArgumentRetirementNativeScript).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check", CheckScript).register("lint", LintScript);
 
 router.register("test-snapshot-receiving-source",SnapshotReceivingSourceScript);
+router.register("test-window-utility-native",WindowUtilityNativeScript);
+router.register("test-original-factory-admission",OriginalFactoryAdmissionScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📋️project.json HEAD 3296 INCOMING 3553
--- BASE
+++ STASH
@@ -31,4 +31,11 @@
       }
     },
+    "test-original-payload": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-payload"
+      }
+    },
     "test-original-progress": {
       "executor": "repo:script",
@@ -36,4 +43,18 @@
         "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
         "command": "bun ./📜️script.ts test-original-progress"
+      }
+    },
+    "check": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts check"
+      }
+    },
+    "test-original-progress-receiving": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-progress-receiving"
       }
     },
@@ -72,18 +93,4 @@
         "command": "bun ./📜️script.ts test-original-entity-column"
       }
-    },
-    "check": {
-      "executor": "repo:script",
-      "options": {
-        "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
-        "command": "bun ./📜️script.ts check"
-      }
-    },
-    "test-original-progress-receiving": {
-      "executor": "repo:script",
-      "options": {
-        "cwd": "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
-        "command": "bun ./📜️script.ts test-original-progress-receiving"
-      }
     }
   }

PATH 🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts HEAD 13373 INCOMING 15709
--- BASE
+++ STASH
@@ -3,5 +3,5 @@
 import{runBudgetedTestCommand}from"../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
 import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
-import {runRepositoryCargoCommandV1} from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
+import { runRepositoryCargoCommandV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -27,5 +27,5 @@
   async run(args: string[]): Promise<void> {
     if (args.length) throw Error("Expected test-source");
-    const tests = [join(this.root, "../../🧪️tests/🧪️conformance/🟦️.ts"), join(this.root, "../../🏛️ownership/🧪️tests/🟦️.ts"), join(this.root, "../../🎞️tick/🛫️encode/🧪️tests/🟦️.ts")];
+    const tests = [join(this.root, "../../🧪️tests/🧪️conformance/🟦️.ts"), join(this.root, "../../🏛️ownership/🧪️tests/🟦️.ts"), join(this.root, "../../🎞️tick/🛫️encode/🧪️tests/🟦️.ts"),join(this.root,"../../📨️payload/🌊️stream/🧪️tests/🟦️.ts")];
     const options = { signal: this.invocation.control.signal, onProgress: () => advanceScriptInvocation(this.invocation, "test-source", "running") };
     await advanceScriptInvocation(this.invocation, "test-source", "running");
@@ -51,5 +51,5 @@
     const cargoPhase=createIssuedResourceControlV1(this.invocation,"cargo:script-entry",0);
   if(args.length>1||(args.length===1&&args[0]!=="source"))throw Error("Expected test-original-progress [source]");
-  await runBudgetedTestCommand(process.execPath,["test","--test-name-pattern","schema oracle|tick codec and step ring",join(this.root,"../../🧪️tests/🧪️conformance/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs()});
+  await runOwnedCommand(process.execPath,["test","--test-name-pattern","schema oracle|tick codec and step ring",join(this.root,"../../🧪️tests/🧪️conformance/🟦️.ts")],this.repoRoot,"tool-run:original-source",scriptInvocationBudget(this.invocation,testLevelBudgetMs()),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"original-source","running")});
   await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",join(this.root,"../../📊️progress/📸️clone/🟦️.ts"),join(this.root,"../../📊️progress/💍️steps/➕️insert/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🧬️update/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🌱️seed/🟦️.ts"),join(this.root,"../../📊️progress/🪟️presentation/🗃️slot/🟦️.ts"),join(this.root,"../../📽️tick/🔐️source/🟦️.ts")],this.repoRoot,"tool-run:original-progress-clone:types",120000);
   if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[COMPANION,PRESENTATION],cwd:this.root,extraArgs:["--lib","original_tool_run_progress_metadata","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
@@ -62,5 +62,5 @@
   if(args.length>1||(args.length===1&&args[0]!=="source"))throw Error("Expected test-original-entities [source]");
   const file=join(this.root,"../../👥️entities/🧪️tests/🟦️.ts");
-  await runBudgetedTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs()});
+  await runOwnedCommand(process.execPath,["test",file],this.repoRoot,"tool-run:original-source",scriptInvocationBudget(this.invocation,testLevelBudgetMs()),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"original-source","running")});
   await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"tool-run:entity-edit:types",120000);
   if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[ENTITIES],cwd:this.root,extraArgs:["--lib","original_tool_run_entity_edit","--","--nocapture"]},readCargoTestPolicyV1(process.env), this.repoRoot, this.invocation,cargoPhase);
@@ -123,5 +123,18 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-original-progress", OriginalProgressTestScript).register("test-original-entities",OriginalEntitiesTestScript).register("test-original-receiving-limits",OriginalReceivingLimitsTestScript).register("test-original-identity-receiving",OriginalIdentityReceivingTestScript).register("test-original-step-receiving",OriginalStepReceivingTestScript).register("test-original-native-tick",OriginalNativeTickTestScript).register("test-original-entity-column",OriginalEntityColumnTestScript).register("test-original-progress-receiving",OriginalProgressReceivingTestScript).register("test-source", SourceScript).register("check", CheckScript);
+
+
+/** 🌊️ Runs original native directory custody and independent Buffer range laws through the owner. */
+class OriginalPayloadTestScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length>1||(args.length===1&&args[0]!=="source"))throw Error("Expected test-original-payload [source]");
+  const files=[join(this.root,"../../📨️payload/🌊️stream/🧪️tests/🟦️.ts"),join(this.root,"../../📨️payload/🌊️stream/📡️wire/🧪️tests/🟦️.ts"),join(this.root,"../../🎞️tick/🛬️receive/🧪️tests/🟦️.ts")];
+  const options={signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-original-payload","running")};
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...files],this.repoRoot,"tool-run:original-payload:types",scriptInvocationBudget(this.invocation,120000),options);
+  await runOwnedCommand(process.execPath,["test",...files],this.repoRoot,"tool-run:original-payload:source",scriptInvocationBudget(this.invocation,30000),options);
+  if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:[PACKAGE],cwd:this.root,extraArgs:["--lib","original_payload_stream","--","--nocapture"]},readCargoTestPolicyV1(process.env));
+ }
+}
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-original-progress", OriginalProgressTestScript).register("test-original-entities",OriginalEntitiesTestScript).register("test-original-receiving-limits",OriginalReceivingLimitsTestScript).register("test-original-identity-receiving",OriginalIdentityReceivingTestScript).register("test-original-step-receiving",OriginalStepReceivingTestScript).register("test-original-native-tick",OriginalNativeTickTestScript).register("test-original-entity-column",OriginalEntityColumnTestScript).register("test-original-progress-receiving",OriginalProgressReceivingTestScript).register("test-source", SourceScript).register("check", CheckScript).register("test-original-payload",OriginalPayloadTestScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📋️project.json HEAD 2638 INCOMING 2736
--- BASE
+++ STASH
@@ -5,5 +5,6 @@
     "default": [
       "{projectRoot}/**/*",
-      "{workspaceRoot}/🧰️framework/🔨️modules/⏳️async/🧪️tests/🎚️config/🟦️.ts"
+      "{workspaceRoot}/🧰️framework/🔨️modules/⏳️async/🧪️tests/🎚️config/🟦️.ts",
+      "{workspaceRoot}/🧰️framework/🔨️modules/⏳️async/🔁️jittered-backoff/**/*"
     ]
   },

PATH 🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📋️project.json HEAD 630 INCOMING 1572
--- BASE
+++ STASH
@@ -10,4 +10,37 @@
         "forwardAllArgs": true
       }
+    },
+    "value-admission-check": {
+      "cache": false,
+      "dependsOn": [],
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "value-admission-check"
+          ]
+        }
+      },
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts value-admission-check"
+      },
+      "executor": "repo:script"
+    },
+    "value-admission-native-check": {
+      "cache": false,
+      "dependsOn": [],
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "value-admission-check",
+            "--native"
+          ]
+        }
+      },
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts value-admission-check --native"
+      },
+      "executor": "repo:script"
     }
   },

PATH 🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📜️script.ts HEAD 1289 INCOMING 4013
--- BASE
+++ STASH
@@ -1,3 +1,9 @@
 #!/usr/bin/env bun
+import assert from "node:assert/strict";
+import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
+import { scriptInvocationBudget } from "../../../../🏃️process/🧭️routing/🟦️.ts";
+import { configuredExactCargoLawPolicyV1, runExactCargoLaws } from "../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
+import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
+import { resolveTestLevel } from "../../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
 import { executeCargoRequestV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -14,5 +20,22 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
+/** 🚪️ Checks schema-authored semantic admission through independent source and native laws. */
+class ValueAdmissionScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.length > 1 || segments.length === 1 && segments[0] !== "--native") throw Error("value-admission-check accepts only --native");
+    const source = resolve(this.root, "../../🧬️admission/🧪️tests/🟦️.ts");
+    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source], this.repoRoot, "value:semantic-admission:types", scriptInvocationBudget(this.invocation, 120000), { signal: this.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
+    (await import("../../🧬️admission/🧪️tests/🟦️.ts")).testValueSemanticAdmission();
+    if (!segments.includes("--native")) return;
+    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(this.root, "Cargo.toml");
+    assert.equal(resolve(policy.manifestPath), manifest, "Value admission laws require the original selected manifest");
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: configuredExactCargoLawPolicyV1(), cwd: this.repoRoot, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-value-derive": manifest }, cargoTargetDir: policy.targetDirectory, groups: [{ package: "semio-framework-value-derive", target: { kind: "test", name: "value_admission" }, cargoArgs: ["--locked"], laws: ["semantic_admission_matches_neutral_schema", "controlled_admission_is_metered_and_refusals_retire_once"] }] });
+    assert.equal(receipts.length, 1);
+    assert.equal(receipts[0]!.assertions, 2);
+    console.log(`[DEBUG] Value semantic admission passed 2 exact native laws; executable ${receipts[0]!.sha256}`);
+  }
+}
+
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("value-admission-check", ValueAdmissionScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/📋️project.json HEAD 1646 INCOMING 2287
--- BASE
+++ STASH
@@ -3,5 +3,8 @@
   "projectType": "library",
   "sourceRoot": "🧰️framework/🔨️modules/🌱️value",
-  "tags": ["lang:typescript", "role:framework"],
+  "tags": [
+    "lang:typescript",
+    "role:framework"
+  ],
   "targets": {
     "test-numeric-api": {
@@ -10,5 +13,9 @@
       "dependsOn": [],
       "outputs": [],
-      "inputs": ["{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*", "{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🌱️value/**/*", "sharedGlobals"],
+      "inputs": [
+        "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
+        "{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🌱️value/**/*",
+        "sharedGlobals"
+      ],
       "options": {
         "command": "bun 📜️script.ts test-numeric-api",
@@ -21,5 +28,8 @@
       "dependsOn": [],
       "outputs": [],
-      "inputs": ["{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/🧾️dictionary/**/*", "sharedGlobals"],
+      "inputs": [
+        "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/🧾️dictionary/**/*",
+        "sharedGlobals"
+      ],
       "options": {
         "command": "bun 📜️script.ts test-dictionary",
@@ -32,7 +42,26 @@
       "dependsOn": [],
       "outputs": [],
-      "inputs": ["{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*", "{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🌱️value/**/*", "sharedGlobals"],
+      "inputs": [
+        "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
+        "{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🌱️value/**/*",
+        "sharedGlobals"
+      ],
       "options": {
         "command": "bun 📜️script.ts test-scalars-api",
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript"
+      }
+    },
+    "test-scratch-hash": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "inputs": [
+        "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
+        "{workspaceRoot}/🧰️framework/📦️packages/🟦️typescript/🌱️value/**/*",
+        "sharedGlobals"
+      ],
+      "options": {
+        "command": "bun 📜️script.ts test-scratch-hash",
         "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript"
       }

PATH 🧰️framework/🔨️modules/🌱️value/📦️packages/🟦️typescript/📜️script.ts HEAD 2689 INCOMING 3775
--- BASE
+++ STASH
@@ -28,3 +28,13 @@
 }
 
-if (import.meta.main) await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(new ScriptRouter(import.meta.dir).register("test-numeric-api", NumericApiScript).register("test-scalars-api", ScalarsApiScript).register("test-dictionary", DictionaryScript), { invocation }));
+/** 🗺️ Verifies actual fixed hash ownership against independent dictionary and JSON Patch oracles. */
+class ScratchHashScript extends BundleScript {
+  async run(args: string[]): Promise<void> {
+    if (args.length) throw Error("Scratch hash checks accept no arguments");
+    const test = resolve(this.root, "../../🔑️hashed/🧮️scratch/🧪️tests/🟦️.ts");
+    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun,node", test], this.repoRoot, "value:scratch-hash:types", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
+    await runOwnedCommand(process.execPath, ["test", test], this.repoRoot, "value:scratch-hash:source", scriptInvocationBudget(this.invocation, 30_000), { signal: this.invocation.control.signal });
+  }
+}
+
+if (import.meta.main) await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(new ScriptRouter(import.meta.dir).register("test-numeric-api", NumericApiScript).register("test-scalars-api", ScalarsApiScript).register("test-dictionary", DictionaryScript).register("test-scratch-hash", ScratchHashScript), { invocation }));

PATH 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📋️project.json HEAD 9132 INCOMING 36288
--- BASE
+++ STASH
@@ -9,4 +9,306 @@
   },
   "targets": {
+    "test-native-payload-enum-source": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun ./📜️script.ts test-native-payload-enum source",
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
+      }
+    },
+    "test-native-payload-enum-native": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun ./📜️script.ts test-native-payload-enum native",
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
+      }
+    },
+    "test-immutable-root-clone-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-immutable-root-clone source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-immutable-root-clone",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-immutable-root-clone-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-immutable-root-clone native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-immutable-root-clone",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-close-demand-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-close-demand source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-close-demand",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-close-demand-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-close-demand native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-close-demand",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-slice-equality-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-slice-equality source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-slice-equality",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-slice-equality-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-slice-equality native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-slice-equality",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-named-edit-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-named-edit source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-edit",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-named-edit-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-named-edit native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-edit",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-exact-key-lookup-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-exact-key-lookup source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-exact-key-lookup",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-exact-key-lookup-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-exact-key-lookup native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-exact-key-lookup",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-indexed-edit-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-indexed-edit source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-indexed-edit",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-indexed-edit-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-indexed-edit native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-indexed-edit",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-fixed-request-owners-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-fixed-request-owners source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-fixed-request-owners",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-fixed-request-owners-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-fixed-request-owners native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-fixed-request-owners",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-fixed-key-slots-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-fixed-key-slots source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-fixed-key-slots",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-fixed-key-slots-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-fixed-key-slots native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-fixed-key-slots",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-owned-projection-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-owned-projection source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-owned-projection",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-owned-projection-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-owned-projection native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-owned-projection",
+            "native"
+          ]
+        }
+      }
+    },
     "test-native-literal-refusal-source": {
       "executor": "repo:script",
@@ -331,4 +633,440 @@
         "forwardAllArgs": false
       }
+    },
+    "test-factory-owned-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-factory-owned-source"
+      }
+    },
+    "test-factory-owned-source-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test --lib factory_owned_source_preserves_actual_payload_and_custom_issuer --no-fail-fast --success-output immediate"
+      }
+    },
+    "test-compound-demands": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-compound-demands"
+      }
+    },
+    "test-compound-demands-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test --lib retained_clone_normal_demand_generic_compounds --no-fail-fast --success-output immediate"
+      }
+    },
+    "test-bulk-vec-retained-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-bulk-vec-retained native"
+      }
+    },
+    "test-original-retirement-demand": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-retirement-demand source"
+      }
+    },
+    "test-original-retirement-demand-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-retirement-demand native"
+      }
+    },
+    "test-persistent-traversal": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-persistent-traversal source"
+      }
+    },
+    "test-persistent-traversal-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-persistent-traversal native"
+      }
+    },
+    "test-immutable-projection": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-immutable-projection"
+      }
+    },
+    "test-immutable-projection-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test quick --lib quick_immutable_snapshot_projection --no-fail-fast --success-output immediate"
+      }
+    },
+    "test-retained-utf8-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-utf8 source",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-retained-utf8-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-utf8 native",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-sealed-frontier-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-sealed-frontier source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-sealed-frontier",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-sealed-frontier-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-sealed-frontier native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-sealed-frontier",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-original-vector-concat": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-original-vector-concat source",
+        "cwd": null
+      }
+    },
+    "test-original-vector-concat-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-original-vector-concat native",
+        "cwd": null
+      }
+    },
+    "test-borrowed-observations-source": {
+      "options": {
+        "command": "bun ./📜️script.ts test-borrowed-observations source",
+        "cwd": "{projectRoot}"
+      },
+      "cache": false,
+      "executor": "repo:script"
+    },
+    "test-borrowed-observations-native": {
+      "options": {
+        "command": "bun ./📜️script.ts test-borrowed-observations native",
+        "cwd": "{projectRoot}"
+      },
+      "cache": false,
+      "executor": "repo:script"
+    },
+    "test-native-receiving-original-source": {
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-receiving-original-source"
+          ]
+        }
+      },
+      "cache": false,
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-native-receiving-original-source"
+      }
+    },
+    "test-persistent-build-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-persistent-build source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-persistent-build",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-persistent-build-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-persistent-build native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-persistent-build",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-retained-equality-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-retained-equality source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-equality",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-retained-equality-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-retained-equality native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-equality",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-native-persistent-receive-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-native-persistent-receive source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-persistent-receive",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-native-persistent-receive-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-native-persistent-receive native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-persistent-receive",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-native-shared-receive-source": {
+      "options": {
+        "command": "bun ./📜️script.ts test-native-shared-receive source",
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
+      },
+      "cache": false,
+      "executor": "repo:script",
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-shared-receive",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-native-shared-receive-native": {
+      "options": {
+        "command": "bun ./📜️script.ts test-native-shared-receive native",
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust"
+      },
+      "cache": false,
+      "executor": "repo:script",
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-shared-receive",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-named-root-receive-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-named-root-receive source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-root-receive",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-named-root-receive-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-named-root-receive native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-root-receive",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-named-key-index-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-named-key-index source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-key-index",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-named-key-index-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-named-key-index native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-named-key-index",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-native-retirement-loan-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-native-retirement-loan source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-retirement-loan",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-native-retirement-loan-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust --manifest 🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-native-retirement-loan native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-native-retirement-loan",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-plain-element-retirement-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🌱️value/📦️packages/🦀️rust -- bun ./📜️script.ts test-plain-element-retirement-source",
+        "forwardAllArgs": true
+      },
+      "metadata": {
+        "semio": {
+          "execution": {
+            "version": 1,
+            "kind": "source"
+          }
+        }
+      }
+    },
+    "test-bulk-vec-retained-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-bulk-vec-retained source"
+      }
     }
   }

PATH 🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts HEAD 15201 INCOMING 81921
--- BASE
+++ STASH
@@ -1,4 +1,9 @@
 #!/usr/bin/env bun
 import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
+import assert from "node:assert/strict";
+import { readFileSync } from "node:fs";
+import { semioSchemaAjvV1 } from "../../../🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
+import { configuredExactCargoLawPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
+import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
 import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -24,4 +29,7 @@
 }
 /** 🔐️ Executes original detached recipient identity and real retained physical custody laws. */
+class NativeReceivingOriginalSourceScript extends BundleScript{
+ async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-native-receiving-original-source");const tests=[resolve(this.root,"../../🛬️decode/🏭️construction/🧾️result/🧪️tests/🟦️.ts"),resolve(this.root,"../../🛬️decode/🧳️owned/🧳️original/🧪️tests/🟦️.ts"),resolve(this.root,"../../🛬️decode/🧳️owned/📥️default/🧪️tests/🟦️.ts")];for(const test of tests){await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",test],this.repoRoot,"value:receiving-original:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});await runOwnedCommand(process.execPath,["test",test],this.repoRoot,"value:receiving-original:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});}}
+}
 class NativeRecipientTestScript extends BundleScript{
  async run(args:string[]):Promise<void>{
@@ -122,3 +130,378 @@
 }
 
-await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-shared-factory-retirement",SharedFactoryRetirementTestScript).register("test-native-literal-refusal-source",NativeLiteralRefusalSourceScript).register("test-native-recipient-source",NativeRecipientSourceTestScript).register("test-native-recipient",NativeRecipientTestScript).register("test-decode-ownership", DecodeOwnershipTestScript).register("test-graphql-int64-source",GraphqlInt64SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript).register("test-refusal-portable", RefusalPortableTestScript).register("test-refusal-codec",RefusalCodecTestScript).register("test-ordered-retirement-source",OrderedRetirementSourceTestScript), { invocation: original, ...({ defaultCommand: "test" }) }));
+/** 🔗️ Validates exact original issuer custody through an independent SQLite ownership journal. */
+class FactoryOwnedSourceCheckScript extends BundleScript {
+ async run(args:string[]):Promise<void> {
+  if(args.length)throw Error("test-factory-owned-source accepts no arguments");
+  const {Database}=await import("bun:sqlite");
+  const root=resolve(this.root,"../../♻️retirement/🏭️factory/📦️owned");
+  const law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔗️source/🔣️.json"),"utf8")) as {cases:Array<{name:string;text:string;capacity:number;route:string;refuseOnce:boolean;expected:{payloadRetireCalls:number;refusals:number}}>;sourceRefusals:string[];projectionRefusals:string[];transferRefusals:string[]};
+  const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔗️source/🔣️.json"),"utf8"));
+  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
+  const database=new Database(":memory:");
+  try {
+   database.exec("CREATE TABLE custody(id TEXT PRIMARY KEY,payload TEXT,issuer TEXT,retained INTEGER,calls INTEGER,refusals INTEGER,refuse_once INTEGER)");
+   for(const row of law.cases) {
+    assert(Buffer.byteLength(row.text)<=row.capacity);
+    database.query("INSERT INTO custody VALUES(?,?,?,1,0,0,?)").run(row.name,row.text,row.name,Number(row.refuseOnce));
+    const original=database.query("SELECT payload,issuer FROM custody WHERE id=?").get(row.name);
+    if(row.refuseOnce) {
+     database.query("UPDATE custody SET calls=calls+1,refusals=refusals+1,refuse_once=0 WHERE id=? AND retained=1 AND refuse_once=1").run(row.name);
+     assert.deepEqual(database.query("SELECT payload,issuer FROM custody WHERE id=? AND retained=1").get(row.name),original);
+    }
+    database.query("UPDATE custody SET calls=calls+1,retained=0 WHERE id=? AND retained=1 AND refuse_once=0").run(row.name);
+    const actual=database.query("SELECT calls AS payloadRetireCalls,refusals FROM custody WHERE id=? AND retained=0").get(row.name);
+    assert.deepEqual(actual,row.expected,row.name);
+    assert.deepEqual(database.query("SELECT payload,issuer FROM custody WHERE id=?").get(row.name),original);
+   }
+   for(const [scope,axes] of [["source",law.sourceRefusals],["projection",law.projectionRefusals],["transfer",law.transferRefusals]] as const) {
+    for(const axis of axes) {
+     const values=[Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="capacity"),Number(axis!=="depth")];
+     const row=database.query("SELECT CASE WHEN ?=1 AND ?=1 AND (?=1 OR ?!='source') AND ?=1 THEN 1 ELSE 0 END AS visits").get(values[0],values[1],values[2],scope,values[3]) as {visits:number};
+     assert.equal(row.visits,0,`${scope} denied ${axis} must preserve custody before provider or callback work`);
+    }
+   }
+   console.log("[DEBUG] factory-owned-source: Ajv validates3 transfer/cancel/provider-refusal vectors; SQLite retains exact source/issuer identity through refusal and verifies original payload issuer invocation counts plus constructor4/projection3/transfer3 denied axes");
+  } finally { database.close(); }
+ }
+}
+
+/** 🔮️ Validates generic clone fixture shapes and independently rebuilds their values through SQLite JSON. */
+class CompoundDemandCheckScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length)throw Error("test-compound-demands accepts no arguments");const{Database}=await import("bun:sqlite");
+  const root=resolve(this.root,"../../🧬️retained-clone/🔮️demands");const law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8")) as {cases:Array<{name:string;kind:string;source:unknown}>};const schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8"));const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(schema);assert(validate(law),JSON.stringify(validate.errors));
+  const database=new Database(":memory:");
+  try{for(const row of law.cases){
+   const input=JSON.stringify(row.source);let sql:string;
+   switch(row.kind){
+    case"vec-scalar":case"vec-string":case"array-string":sql="SELECT json_group_array(value) AS value FROM json_each(?)";break;
+    case"record":sql="SELECT json_object('name',json_extract(?1,'$.name'),'values',json(json_extract(?1,'$.values'))) AS value";break;
+    case"enum":sql="SELECT CASE WHEN json_extract(?1,'$.kind')='unit' THEN json_object('kind','unit') ELSE json_object('kind','record','label',json_extract(?1,'$.label'),'values',json(json_extract(?1,'$.values'))) END AS value";break;
+    case"option-box":sql="SELECT json_quote(json_extract(?,'$')) AS value";break;
+    case"tuple":sql="SELECT json_array(json_extract(?1,'$[0]'),json(json_extract(?1,'$[1]'))) AS value";break;
+    default:throw Error(`Unknown compound fixture ${row.kind}`);
+   }
+   const reference=database.query(sql).get(input) as {value:string};assert.deepEqual(JSON.parse(reference.value),row.source,row.name);
+  }console.log("[DEBUG] compound-demands: strict Ajv validates8 generic vector/array/record/enum/optional-box/tuple fixtures; SQLite JSON independently reconstructs every expected native clone value");}finally{database.close();}
+ }
+}
+
+/** 📽️ Validates immutable root custody with strict schema and an independent SQLite alias journal. */
+class ImmutableProjectionCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length)throw Error("test-immutable-projection accepts no arguments");const{Database}=await import("bun:sqlite");
+  const root=resolve(this.root,"../../📽️projection");const law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8")) as{cases:Array<{name:string;items:number}>;cancelCuts:number[];expected:{normalTurns:number;rootBirths:number;payloadClones:number;sourceIdentityPreserved:boolean;transferRequiresGrant:boolean}};const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));assert(validate(law),JSON.stringify(validate.errors));
+  const db=new Database(":memory:");try{
+   db.exec("PRAGMA foreign_keys=ON;CREATE TABLE roots(id INTEGER PRIMARY KEY,payload BLOB);CREATE TABLE owners(kind TEXT PRIMARY KEY,root INTEGER REFERENCES roots(id))");
+   for(const row of law.cases){for(const cut of law.cancelCuts){
+    db.exec("DELETE FROM owners;DELETE FROM roots");db.query("INSERT INTO roots VALUES(1,zeroblob(?))").run(row.items);db.query("INSERT INTO owners VALUES('source',1)").run();
+    const original=db.query("SELECT id,length(payload) AS items FROM roots").get();let turns=0;
+    if(cut>0){db.query("INSERT INTO owners SELECT 'binding',root FROM owners WHERE kind='source'").run();turns++;}
+    if(cut>1){db.query("INSERT INTO owners SELECT 'candidate',root FROM owners WHERE kind='source'").run();turns++;}
+    for(const axis of ["items","copy","depth"]){const admit=db.query("SELECT CASE WHEN ?=1 AND ?=1 AND ?=1 THEN 1 ELSE 0 END AS admit").get(Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="depth")) as{admit:number};assert.equal(admit.admit,0);assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots").get(),original);}
+    if(cut>2)db.query("UPDATE owners SET kind='output' WHERE kind='candidate'").run();
+    const births=db.query("SELECT count(*)-1 AS count FROM roots").get()as{count:number};assert.equal(births.count,law.expected.rootBirths);assert.equal(turns,Math.min(cut,law.expected.normalTurns));assert.equal((db.query("SELECT count(*) AS count FROM owners WHERE kind='source' AND root=1").get()as{count:number}).count,1);
+    if(cut>1)assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots JOIN owners ON roots.id=owners.root WHERE kind=?").get(cut>2?"output":"candidate"),original);
+    db.query("DELETE FROM owners WHERE kind!='source'").run();assert.deepEqual(db.query("SELECT id,length(payload) AS items FROM roots").get(),original);
+   }}
+   const extended=law as typeof law&{refusalCases:Array<{excessCopyBytes:number;ready:boolean;aliasesAfterCapture:number;aliasesAfterClose:number}>;composition:{fields:string[];epoch:number;maximumTurns:number;payloadClones:number;sourceRootIdentityPreserved:boolean}};
+   db.exec("DELETE FROM owners;DELETE FROM roots;CREATE TABLE admission(ready INTEGER);INSERT INTO admission VALUES(0)");
+   for(const row of extended.refusalCases){db.query("INSERT INTO roots VALUES(1,zeroblob(4096))").run();db.query("INSERT INTO owners VALUES('source',1)").run();db.query("INSERT INTO owners SELECT 'unadmitted',root FROM owners WHERE kind='source'").run();db.query("UPDATE admission SET ready=1 WHERE ?<=0").run(row.excessCopyBytes);assert.equal((db.query("SELECT ready FROM admission").get()as{ready:number}).ready,Number(row.ready));assert.equal((db.query("SELECT count(*) AS count FROM owners").get()as{count:number}).count,row.aliasesAfterCapture);db.query("DELETE FROM owners WHERE kind='unadmitted'").run();assert.equal((db.query("SELECT count(*) AS count FROM owners").get()as{count:number}).count,row.aliasesAfterClose);db.exec("DELETE FROM owners;DELETE FROM roots");}
+   db.exec("CREATE TABLE records(scope TEXT,field TEXT,root INTEGER REFERENCES roots(id),scalar INTEGER,PRIMARY KEY(scope,field))");
+   for(const row of law.cases){db.exec("DELETE FROM records;DELETE FROM roots");db.query("INSERT INTO roots VALUES(1,zeroblob(?)),(2,zeroblob(?))").run(row.items,row.items);db.query("INSERT INTO records VALUES('source','epoch',NULL,?),('source','primary',1,NULL),('source','secondary',2,NULL)").run(extended.composition.epoch);const original=db.query("SELECT field,root,scalar FROM records WHERE scope='source' ORDER BY field").all();db.query("INSERT INTO records SELECT 'candidate',field,root,scalar FROM records WHERE scope='source'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='candidate' ORDER BY field").all(),original);assert.equal((db.query("SELECT count(*) AS count FROM roots").get()as{count:number}).count,2);assert.equal((db.query("SELECT sum(length(payload)) AS bytes FROM roots").get()as{bytes:number}).bytes,row.items*2);for(const axis of["items","copy","depth"]){db.query("UPDATE records SET scope='output' WHERE scope='candidate' AND ?=1 AND ?=1 AND ?=1").run(Number(axis!=="items"),Number(axis!=="copy"),Number(axis!=="depth"));assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='candidate' ORDER BY field").all(),original);}db.query("UPDATE records SET scope='output' WHERE scope='candidate'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='output' ORDER BY field").all(),original);db.query("DELETE FROM records WHERE scope='output'").run();assert.deepEqual(db.query("SELECT field,root,scalar FROM records WHERE scope='source' ORDER BY field").all(),original);}
+   console.log("[DEBUG] immutable-projection: strict Ajv validates4 neutral cardinalities and16 cancellation frontiers; independent SQLite root aliases preserve source identity and payload size with zero payload copies/root births and separately denied transfer axes; composed3-field snapshots preserve both immutable payload roots, and invalid receipt custody remains unadmitted until original close");
+  }finally{db.close();}
+ }
+}
+
+/** 📦️ Checks explicit owned field projection against SQLite copying before genuine native custody laws. */
+class OwnedProjectionCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-owned-projection requires source or native");
+  const root=resolve(this.root,"../../📽️projection"),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/📦️owned/🔣️.json"),"utf8"))as{nestedBodyQuotes:Array<{body:number;headers:number[];payload:number;spent:number}>;cases:Array<{id:string;schema:string;count:number;octet:number;optionalPayload:boolean}>;cancelCuts:number[]};
+  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(resolve(root,"🧬️schema/📦️owned.json"),"utf8")));assert(validate(law),JSON.stringify(validate.errors));
+  const{Database}=await import("bun:sqlite"),db=new Database(":memory:");try{
+   db.exec("CREATE TABLE original(schema TEXT,bytes BLOB,optional BLOB);CREATE TABLE candidate(schema TEXT,bytes BLOB,optional BLOB)");
+   for(const row of law.cases){db.exec("DELETE FROM original;DELETE FROM candidate");db.query("INSERT INTO original VALUES(?,?,?)").run(row.schema,Buffer.alloc(row.count,row.octet),row.optionalPayload?Buffer.alloc(row.count,row.octet):null);db.exec("INSERT INTO candidate SELECT schema,bytes,optional FROM original");const original=db.query("SELECT schema,bytes,optional FROM original").get()as{schema:string;bytes:Uint8Array;optional:Uint8Array|null};const candidate=db.query("SELECT schema,bytes,optional FROM candidate").get()as{schema:string;bytes:Uint8Array;optional:Uint8Array|null};assert.equal(candidate.schema,row.schema);assert.deepEqual([...candidate.bytes],Array(row.count).fill(row.octet));assert.deepEqual(candidate.optional===null?null:[...candidate.optional],row.optionalPayload?Array(row.count).fill(row.octet):null);assert.deepEqual(original,candidate);db.exec("DELETE FROM candidate");assert.deepEqual(db.query("SELECT schema,bytes,optional FROM original").get(),original);}
+  }finally{db.close();}
+  console.log("[DEBUG] Explicit owned String/Vec/Option projection corpus matches independent SQLite copying and source-preserving cancellation");
+  for(const row of law.nestedBodyQuotes){const header=row.headers.reduce((sum,width)=>sum+width,0);const db=new Database(":memory:");try{const expected=db.query("SELECT max(0,?1-?2) payload").get(row.body,header)as{payload:number};assert.equal(expected.payload,row.payload);assert.equal(row.spent,row.payload+header);assert(row.spent<=row.body);}finally{db.close();}}
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:p.buildBudgetMs,lawMilliseconds:p.assertionBudgets[p.level]},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["value::snapshot_projection::tests::owned::owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody"]}]});}
+ }
+}
+
+/** 🔎️ Validates immutable ranked table traversal and record output against original SQLite rows. */
+class PersistentTraversalCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-persistent-traversal requires source or native");
+  const root=resolve(this.root,"../../📋️list/🔗️persistent/🔎️traversal");
+  const law:typeof import("../../📋️list/🔗️persistent/🔎️traversal/🧫️fixtures/🔣️.json")=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"));
+  const{default:Ajv2020}=await import("ajv/dist/2020"),ajv=new Ajv2020({strict:true}),validate=ajv.compile<typeof law>(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));
+  assert(validate(law),JSON.stringify(validate.errors));assert(!validate({...law,stride:0}));assert(!validate({...law,cases:[]}));
+  const{Database}=await import("bun:sqlite"),db=new Database(":memory:");let assertions=3;
+  try{
+   db.exec("CREATE TABLE original(position INTEGER PRIMARY KEY,rank INTEGER,label TEXT)");
+   for(const row of law.cases){
+    const count=row.fullLeaves*16+row.tailLeaves.reduce((sum:number,length:number)=>sum+length,0);assert.equal(count,row.rowCount);let pages=row.fullLeaves+row.tailLeaves.length,all=pages;while(pages>1){pages=Math.ceil(pages/16);all+=pages;}assert.equal(all,row.pageCount);
+    db.exec("DELETE FROM original");db.transaction(()=>{for(let index=0;index<count;index++)db.query("INSERT INTO original VALUES(?,?,?)").run(index,index*law.stride,law.prefix+index)})();
+    const output=db.query("SELECT rank,label FROM original ORDER BY position").all();assert.deepEqual(output,Array.from({length:count},(_,index)=>({rank:index*law.stride,label:law.prefix+index})));
+    let position=0;for(const skip of law.queries){const requested=position+skip;const actual=db.query("SELECT rank,label FROM original ORDER BY position LIMIT 1 OFFSET ?").get(requested);assert.deepEqual(actual,requested<count?{rank:requested*law.stride,label:law.prefix+requested}:null);position=requested<count?requested+1:count;assertions++;}
+    assertions+=3;
+   }
+  }finally{db.close();}
+  console.log("[DEBUG] Persistent traversal schema and independent SQLite oracle passed "+law.cases.length+" shapes, "+assertions+" assertions; original ranked order and exact exhaustion");
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["value::list::persistent::tests::traversal::persistent_table_traversal_preserves_original_rows_and_linear_zero_heap_scan","value::list::persistent::tests::traversal::persistent_table_traversal_projects_borrowed_records_and_controlled_wire_without_payload_clone"]}]});}
+ }
+}
+/** 🚚️ Checks bounded bitwise clone work at its defining split owner under the original process. */
+class BulkVecRetainedCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-bulk-vec-retained requires source or native");
+  const source=resolve(this.root,"../../🧬️retained-clone/🚚️bulk-vec/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:bulk-clone:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:bulk-clone:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value-retained-clone":resolve(this.root,"../../🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:"semio-framework-value-retained-clone",target:{kind:"lib",name:"semio_framework_value_retained_clone"},laws:["retained_clone::bulk_vec_tests::bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape", "retained_clone::bulk_vec_tests::bulk_vector_clone_turn_count_is_bytes_over_page", "retained_clone::bulk_vec_tests::bulk_vector_clone_capacity_accounting_is_exact", "retained_clone::bulk_vec_tests::bulk_vector_clone_cancels_at_every_turn_and_releases_everything", "retained_clone::bulk_vec_tests::bulk_run_elements_obeys_every_grant_axis", "retained_clone::bulk_vec_tests::bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes", "retained_clone::bulk_vec_tests::bulk_clone_logical_body_survives_expanded_physical_headers"]}]});}
+ }
+}
+
+/** 🧮️ Verifies plain derived retirement widths against the neutral SQLite corpus. */
+class PlainElementRetirementSourceScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length)throw Error("test-plain-element-retirement-source accepts no arguments");
+  const source=resolve(this.root,"../../♻️retirement/🧪️tests/🧮️plain-elements/🟦️.ts");
+  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:plain-element-retirement:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:plain-element-retirement:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+ }
+}
+/** 🎟️ Counts original nested owner quotes independently of SQLite path leaves. */
+class OriginalRetirementDemandScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-original-retirement-demand requires source or native");
+  const root=resolve(this.root,"../../♻️retirement/🎟️demand"),law=JSON.parse(readFileSync(resolve(root,"🧫️fixtures/🔣️.json"),"utf8"))as typeof import("../../♻️retirement/🎟️demand/🧫️fixtures/🔣️.json");const{default:Ajv}=await import("ajv/dist/2020"),v=new Ajv({strict:true}).compile<typeof law>(JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")));assert(v(law),JSON.stringify(v.errors));assert(!v({...law,expected:{...law.expected,leafVisitsPerQuote:4}}));
+  const{Database}=await import("bun:sqlite"),db=new Database(":memory:");let assertions=2;try{db.exec("create table nodes(family integer,level integer,parent integer)");for(const[index,family]of law.families.entries()){for(let level=0;level<=family.sealedDepth;level++)db.query("insert into nodes values(?,?,?)").run(index,level,level===0?null:level-1);for(const body of law.bodyHints){const leaves=db.query("with recursive path(level) as(select level from nodes where family=? and parent is null union all select n.level from nodes n join path p on n.parent=p.level where n.family=?) select count(*) as visits from path where level=?").get(index,index,family.sealedDepth)as{visits:number};assert.equal(leaves.visits,law.expected.leafVisitsPerQuote);assert(Number.isSafeInteger(body));assertions+=2;}}}finally{db.close();}
+  console.log("[DEBUG] Original retirement demand strict schema and SQLite path oracle passed "+law.families.length+" families, "+assertions+" assertions");
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["retirement::original_demand_tests::original_retirement_combined_quote_visits_terminal_leaf_once_with_actual_body_hint"]}]});}
+ }
+}
+/** ⚖️ Verifies borrowed immutable Debug and value equality against SQLite. */
+class BorrowedObservationScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-borrowed-observations source|native");
+  const oracle=await import("../../📋️list/🔗️persistent/⚖️observation/🧪️tests/🟦️.ts");oracle.borrowedObservationOracle();
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["value::list::persistent::tests::borrowed_observations::borrowed_immutable_observations_preserve_original_rows_without_clone_or_heap"]}]});}
+ }
+}
+/** 🔗️ Verifies paid original child receiving, immutable sealing and every controlled cancellation frontier. */
+class NativeSharedReceivingCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-shared-receive requires source or native");
+  const source=resolve(this.root,"../../🛬️decode/🧳️owned/🔗️shared/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:native-shared-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-shared-receive:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["native_decoding::owned::shared::tests::native_shared_original_receive_exact_prefix_cancellation_and_transfer","native_decoding::owned::shared::tests::native_shared_original_callback_refusal_retains_paid_child_authority","native_decoding::owned::shared::tests::native_record32_single_active_child_fits4096_and_preserves_every_original_prefix","native_decoding::owned::shared::tests::native_flattened_original_record_preserves_source_schema_and_all_partial_custody"]}]});}
+ }
+}
+/** 📇️ Verifies source-bound immutable key admission and sparse original ranked reuse. */
+class NamedRootReceivingCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-root-receive requires source or native");
+  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/📇️index/🌳️root/🛬️receive/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-root-receive:source",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:named-root-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level])},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["named_edit::index::tests::transfer::receiving::original_named_receiving_publishes_independent_roots_and_closes_every_paid_cut","named_edit::index::tests::transfer::receiving::original_named_receiving_checks_full_utf8_under_original_decoder_quantum"]}]});}
+ }
+}
+/** 📇️ Original immutable key admission and controlled directory transfer share the actual public planner. */
+class NamedKeyIndexCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-key-index requires source or native");
+  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/📇️index/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-key-index:source",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:named-key-index:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(this.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["named_edit::index::tests::transfer::key_preserving_persistent_producer_transfers_original_directory_and_all_paid_cuts","named_edit::index::tests::transfer::key_preserving_persistent_producer_checks_complete_original_utf8_and_foreign_sources","named_edit::index::tests::admitted_named_index_preserves_original_opaque_keys_ranked_sources_and_cancelled_custody","named_edit::index::tests::admitted_named_index_reuses_persistent_originals_without_base_allocation_or_resorting","named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
+ }
+}
+
+
+
+/** 🔀️ Checks schema-declared FEM record and tuple variant receiving laws against independent validators. */
+class NativePayloadEnumScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-payload-enum requires source or native");const source=resolve(this.root,"../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:payload-enum:types",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});(await import("../../✨️derive/🛬️receive/🔀️enum/🧪️tests/🟦️.ts")).nativePayloadEnumOracle();if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["native_decoding::owned::payload_enum_tests::native_payload_enum_receives_exact_declared_variants_and_closes_every_original_frontier"]}]});}
+ }
+}
+
+
+/** ♻️ Verifies original decoder retirement custody after semantic cancellation under exact caller grants. */
+class NativeRetirementLoanCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-retirement-loan requires source or native");
+  const source=resolve(this.root,"../../🛬️decode/♻️retirement/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-retirement-loan:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:native-retirement-loan:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["native_decoding::retirement_loan::tests::native_retirement_loan_preserves_original_cancelled_ledger_and_grant","native_decoding::retirement_loan::tests::native_retirement_loan_closes_original_returned_payload_after_cancellation"]}]});}
+ }
+}
+
+
+/** 🧳️ Verifies original typed child receiving into fixed persistent pages without a cold array conversion. */
+class NativePersistentReceivingCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-native-persistent-receive requires source or native");
+  const source=resolve(this.root,"../../🛬️decode/🧳️owned/📋️list/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:native-persistent-receive:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:native-persistent-receive:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["native_decoding::owned::list_tests::native_persistent_list_receives_original_rows_and_preserves_all_cancelled_custody","native_decoding::owned::list_tests::native_persistent_list_later_row_refusal_retains_original_partial_owner","native_decoding::owned::list_tests::native_persistent_list_original_callback_refusal_preserves_partial_receiving","native_decoding::owned::list_tests::native_persistent_list_receiving_fits_original_interaction_quantum","native_decoding::owned::list_tests::native_persistent_list_original_vector_owner_fits_quantum_and_keeps_custody","native_decoding::owned::original::tests::native_receiving_original_keeps_cancellation_and_denied_transfers_owned","native_decoding::owned::original::tests::native_receiving_original_shell_requires_funded_input_and_receiver_custody"]}]});}
+ }
+}
+
+
+/** 📥️ Verifies original streamed row adoption, fixed page carries and funded persistent cancellation. */
+class PersistentBuildCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-persistent-build requires source or native");
+  const source=resolve(this.root,"../../📋️list/🔗️persistent/📥️build/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:persistent-build:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:persistent-build:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["list::persistent::build_tests::persistent_owned_builder_preserves_ranked_originals_and_every_receiving_frontier","list::persistent::build_tests::persistent_owned_builder_refuses_missing_and_excess_without_consuming_input","list::persistent::build_tests::persistent_owned_builder_cancels_every_preparation_action_frontier","list::persistent::build_tests::persistent_owned_builder_constructor_fits_authored_interaction_quantum"]}]});}
+ }
+}
+
+
+/** 🚚️ Verifies original typed sealed children use their caller's funded cleanup frontier. */
+class SealedFrontierCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-sealed-frontier requires source or native");
+  const source=resolve(this.root,"../../♻️retirement/🔗️shared/🔒️sealed/🚚️frontier/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:sealed-frontier:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:sealed-frontier:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["retirement::shared::sealed::frontier_tests::original_sealed_owned_payload_hands_native_child_to_caller_frontier","retirement::shared::sealed::tests::sealed_shared_original_aliases_preserve_exact_payload_and_funded_header","retirement::shared::sealed::tests::sealed_shared_original_alias_return_slots_transfer_parallel_custody_without_heap","retirement::shared::sealed::tests::sealed_shared_stale_slot_refuses_a_distinct_original_issuance_at_same_address"]}]});}
+ }
+}
+
+/** 🔗️ Verifies immutable list root cloning through actual original pointer and physical retirement witnesses. */
+class ImmutableRootCloneCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-immutable-root-clone requires source or native");
+  const source=resolve(this.root,"../../🧬️retained-clone/🔗️shared/📋️list/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:immutable-root-clone:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:immutable-root-clone:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+   if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["retained_clone::shared::list_tests::persistent_list_retained_clone_preserves_original_root_without_payload_copy","retained_clone::source_custody_tests::retained_clone_owned_source_shared_alias_keeps_original_identity_in_every_close_order"]}]});}
+ }
+}
+
+/** 🌳️ Verifies single-visit original close quotes and genuine paid nested owner retirement. */
+class CloseDemandCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-close-demand requires source or native");
+  const source=resolve(this.root,"../../🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:close-demand:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:close-demand:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["retained_clone::close_demand_tests::original_nested_close_preserves_paid_source_and_cancellation","retained_clone::close_demand_tests::derived_original_record_and_enum_preflight_all_axes_and_close_exact_custody"]}]});}
+ }
+}
+
+/** 🟰️ Verifies immutable scalar-buffer equality under original paged copy grants. */
+class TypedEqualityCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-equality requires source or native");
+  const source=resolve(this.root,"../../🌳️tree/🟰️equal/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:typed-equality:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:typed-equality:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["value::retained_equality::tests::retained_typed_equality_matches_portable_original_fields","value::retained_equality::tests::retained_typed_equality_cancellation_keeps_original_leases_and_child_frontiers"]}]});}
+ }
+}
+
+class SliceEqualityCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-slice-equality requires source or native");
+  const source=resolve(this.root,"../../📋️list/🟰️equal/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:slice-equality:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:slice-equality:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["slice_equality::tests::retained_slice_equality_matches_portable_bytes_without_allocations","slice_equality::tests::retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded"]}]});}
+ }
+}
+
+/** 🗂️ Verifies neutral named edits, exact paged keys and genuine original directory custody. */
+class OriginalVectorConcatCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-original-vector-concat requires source or native");
+  const source=resolve(this.root,"../../📋️list/🪢️concat/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:original-vector-concat:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:original-vector-concat:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["original_vector_concat::tests::original_vector_concat_preserves_original_rows_and_cancelled_custody","original_vector_concat::tests::original_vector_concat_wide_payload_and_chunk_directory_remain_bounded"]}]});}
+ }
+}
+
+class NamedEditCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-named-edit requires source or native");
+  const source=resolve(this.root,"../../📋️list/🔺️edit/🔑️named/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:named-edit:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:named-edit:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
+ }
+}
+
+/** 🔑️ Checks original UTF8 key pages against SQLite and genuine zero-heap native cursors. */
+class ExactKeyLookupCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-exact-key-lookup requires source or native");
+  const source=resolve(this.root,"../../🔑️lookup/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:exact-key:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:exact-key:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["exact_key_lookup::tests::exact_key_lookup_matches_portable_opaque_identity_without_heap_work","exact_key_lookup::tests::exact_key_lookup_long_prefixes_and_cancellation_preserve_originals"]}]});}
+ }
+}
+
+/** 🧮️ Verifies shared strict indexed admission, header streaming and original directory custody. */
+class IndexedEditCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-indexed-edit requires source or native");
+  const source=resolve(this.root,"../../📋️list/🔺️edit/🧪️tests/🟦️.ts");
+  const sources=[source,resolve(this.root,"../../📋️list/🔺️edit/🚚️commit/🧪️tests/🟦️.ts"),resolve(this.root,"../../📋️list/🔺️edit/🔑️named/🧪️tests/🟦️.ts")];
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",...sources],this.repoRoot,"value:indexed-edit:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",...sources],this.repoRoot,"value:indexed-edit:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody","indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded","indexed_collection_commit::tests::indexed_collection_commit_preserves_original_payloads_and_exact_cancellation","indexed_collection_commit::tests::indexed_collection_commit_copies_large_authored_fields_without_cloning_survivors","named_edit::tests::named_edit_plan_matches_portable_keys_and_physical_custody","named_edit::tests::named_edit_plan_long_prefixes_and_cancellation_are_bounded"]}]});}
+ }
+}
+
+
+/** 🎟️ Executes exact registry admission, reuse and foreign-owner laws under original process authority. */
+class FixedKeySlotsCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-fixed-key-slots requires source or native");
+  const source=resolve(this.root,"../../📋️registry/🎫️slots/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:fixed-slots:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:fixed-slots:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){
+   const policy=readCargoTestPolicyV1(process.env);
+   await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release"]}]});
+  }
+ }
+}
+
+
+/** 🎫️ Exercises shared scoped request owners with portable and independent consumer laws. */
+class FixedRequestOwnersCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-fixed-request-owners requires source or native");
+  const source=resolve(this.root,"../../📋️registry/🎫️requests/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:request-owners:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:request-owners:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},laws:["fixed_key_slots::tests::fixed_key_slots_match_portable_admission_and_stale_release","fixed_request_owners::tests::fixed_request_owners_match_portable_consumer_transitions","fixed_request_owners::tests::request_text_and_terminals_match_portable_boundaries","fixed_request_owners::tests::request_selector_preserves_original_binding_until_verified_promotion","fixed_request_owners::tests::original_source_lookup_matches_portable_reordered_operands","fixed_request_owners::tests::provisional_request_consumers_match_portable_live_and_pending_outputs","indexed_edit::tests::indexed_edit_plan_matches_portable_coordinates_and_original_custody","indexed_edit::tests::indexed_edit_plan_wide_order_and_cancellation_remain_bounded"]}]});}
+ }
+}
+
+
+/** 🚶️ Shared UTF8 scalar traversal and original buffers retain explicit caller work authority. */
+class RetainedUtf8CheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-utf8 requires source or native");const source=resolve(this.root,"../../📝️text/🚶️utf8/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"value:utf8:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"value:utf8:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000),lawMilliseconds:policy.assertionBudgets[policy.level]},nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,manifestPaths:{"semio-framework-value":manifest},groups:[{package:"semio-framework-value",target:{kind:"lib",name:"semio_framework_value"},cargoArgs:["--locked"],laws:["retained_utf8::tests::original_utf8_cursor_pages_match_portable_scalar_offsets_without_allocations","retained_utf8::tests::original_utf8_buffer_funds_transfer_and_every_cancellation_frontier"]}]});}
+ }
+}
+
+await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test-native-payload-enum",NativePayloadEnumScript).register("test-native-retirement-loan",NativeRetirementLoanCheckScript).register("test-native-shared-receive",NativeSharedReceivingCheckScript).register("test-native-persistent-receive",NativePersistentReceivingCheckScript).register("test-retained-equality",TypedEqualityCheckScript).register("test-persistent-build",PersistentBuildCheckScript).register("test-borrowed-observations",BorrowedObservationScript).register("test-retained-utf8",RetainedUtf8CheckScript).register("test-sealed-frontier",SealedFrontierCheckScript).register("test-immutable-root-clone",ImmutableRootCloneCheckScript).register("test-close-demand",CloseDemandCheckScript).register("test-bulk-vec-retained",BulkVecRetainedCheckScript).register("test-plain-element-retirement-source",PlainElementRetirementSourceScript).register("test-original-retirement-demand",OriginalRetirementDemandScript).register("test-persistent-traversal",PersistentTraversalCheckScript).register("test-slice-equality",SliceEqualityCheckScript).register("test-original-vector-concat",OriginalVectorConcatCheckScript).register("test-named-root-receive",NamedRootReceivingCheckScript).register("test-named-key-index",NamedKeyIndexCheckScript).register("test-named-edit",NamedEditCheckScript).register("test-exact-key-lookup",ExactKeyLookupCheckScript).register("test-indexed-edit",IndexedEditCheckScript).register("test-fixed-request-owners",FixedRequestOwnersCheckScript).register("test-fixed-key-slots",FixedKeySlotsCheckScript).register("test", TestScript).register("test-owned-projection",OwnedProjectionCheckScript).register("test-immutable-projection",ImmutableProjectionCheckScript).register("test-compound-demands",CompoundDemandCheckScript).register("test-factory-owned-source", FactoryOwnedSourceCheckScript).register("test-shared-factory-retirement",SharedFactoryRetirementTestScript).register("test-native-literal-refusal-source",NativeLiteralRefusalSourceScript).register("test-native-recipient-source",NativeRecipientSourceTestScript).register("test-native-receiving-original-source",NativeReceivingOriginalSourceScript).register("test-native-recipient",NativeRecipientTestScript).register("test-decode-ownership", DecodeOwnershipTestScript).register("test-graphql-int64-source",GraphqlInt64SourceTestScript).register("test-type-ownership", TypeOwnershipTestScript).register("test-controlled-construction", ControlledValueTestScript).register("test-controlled-encoding",ControlledEncodingTestScript).register("test-refusal-portable", RefusalPortableTestScript).register("test-refusal-codec",RefusalCodecTestScript).register("test-ordered-retirement-source",OrderedRetirementSourceTestScript), { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/📋️project.json HEAD 0 INCOMING 511
--- BASE
+++ STASH
@@ -0,0 +1,7 @@
+{
+ "name":"@semio-tech/framework-record-projection","tags":["framework","value"],
+ "targets":{
+  "verify-source":{"executor":"repo:script","cache":false,"options":{"cwd":"🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record","command":"bun ./📜️script.ts verify source"}},
+  "verify-native":{"executor":"repo:script","cache":false,"options":{"cwd":"🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record","command":"bun ./📜️script.ts verify native"}}
+ }
+}

PATH 🧰️framework/🔨️modules/🌱️value/📽️projection/🧳️record/📜️script.ts HEAD 0 INCOMING 2832
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1} from '../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {runRepositoryExactCargoLaws} from '../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts';
+/** 🧳️ Verifies original wide record output, fixed native frontiers and physical partial-field cancellation. */
+class OriginalRecordProjectionScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-record-projection requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-record:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalRecordProjectionOracle();if(args[0]==='source')return;
+  const packageName='semio-framework-value',manifest=resolve(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml');
+  const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_value'},laws:['value::snapshot_projection::tests::record::original_record_projection_stages_one_paid_field_and_preserves_all_cancellation_prefixes']}]});
+  console.log('[DEBUG] Original record projection native '+receipts[0]!.assertions+' exact custody law');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',OriginalRecordProjectionScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/📋️project.json HEAD 0 INCOMING 544
--- BASE
+++ STASH
@@ -0,0 +1 @@
+{"name":"@semio-tech/framework-native-tuple-receiving","tags":["framework","value"],"targets":{"verify-source":{"executor":"repo:script","cache":false,"options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple","command":"bun ./📜️script.ts verify source"}},"verify-native":{"executor":"repo:script","cache":false,"options":{"cwd":"🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple","command":"bun ./📜️script.ts verify native"}}}}
PATH 🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/👥️tuple/📜️script.ts HEAD 0 INCOMING 2850
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../../../🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1} from '../../../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {runRepositoryExactCargoLaws} from '../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts';
+/** 🧳️ Verifies original heterogeneous values, fixed native frontiers and physical partial-child cancellation. */
+class OriginalTupleReceivingScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-tuple-receiving requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-tuple:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalTupleOracle();if(args[0]==='source')return;
+  const packageName='semio-framework-value',manifest=resolve(this.repoRoot,'🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml');
+  const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_value'},laws:['value::native_decoding::owned::tree::tuple::tests::original_heterogeneous_tuple_receiving_preserves_every_paid_prefix_and_wire_kind']}]});
+  console.log('[DEBUG] Original tuple receiving native '+receipts[0]!.assertions+' exact custody law');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',OriginalTupleReceivingScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/📋️project.json HEAD 0 INCOMING 669
--- BASE
+++ STASH
@@ -0,0 +1,25 @@
+{
+  "name": "@semio-tech/framework-original-slot",
+  "tags": [
+    "framework",
+    "value"
+  ],
+  "targets": {
+    "verify-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot",
+        "command": "bun ./📜️script.ts verify source"
+      }
+    },
+    "verify-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot",
+        "command": "bun ./📜️script.ts verify native"
+      }
+    }
+  }
+}

PATH 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/📜️script.ts HEAD 0 INCOMING 3161
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1} from '../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {runRepositoryExactCargoLaws} from '../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts';
+/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
+class OriginalSlotScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-slot requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:original-slot:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalSlot();
+  if(args[0]==='source')return;
+  const manifest=resolve(this.repoRoot,"🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml");const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-value',target:{kind:'lib',name:'semio_framework_value'},laws:['retained_clone::original_slot::tests::original_slot_preserves_native_payload_and_waits_for_every_projected_reader','retained_clone::original_slot::tests::original_slot_generation_exhaustion_refuses_before_native_exchange','retained_clone::original_slot::tests::original_slot_exchange_matches_all_independent_portable_outputs','retained_clone::original_slot::tests::original_slot_retirement_returns_all_readers_then_uses_same_captured_payload_issuer']}]});
+  console.log('[DEBUG] Original slot native custody '+receipts[0]!.assertions+' exact laws');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',OriginalSlotScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📋️project.json HEAD 2208 INCOMING 4801
--- BASE
+++ STASH
@@ -11,4 +11,20 @@
   },
   "targets": {
+    "test-original-receiving-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-receiving source"
+      }
+    },
+    "test-original-receiving-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-receiving native"
+      }
+    },
     "test-read-policy-source": {
       "executor": "repo:script",
@@ -44,4 +60,58 @@
       }
     },
+    "test-persistent-canonical-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust -- bun ./📜️script.ts test-persistent-canonical source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-persistent-canonical",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-persistent-canonical-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust --manifest 🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml -- bun ./📜️script.ts test-persistent-canonical native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-persistent-canonical",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-immutable-tree-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-immutable-tree source"
+      }
+    },
+    "test-immutable-tree-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-immutable-tree native"
+      }
+    },
     "test-tree-wire-source": {
       "executor": "repo:script",

PATH 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📜️script.ts HEAD 9359 INCOMING 14522
--- BASE
+++ STASH
@@ -2,4 +2,5 @@
 import{createIssuedResourceControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
+import { runOwnedCommand } from "../../../../🏃️process/🎛️owned-execution/🟦️.ts";
 import {runRepositoryExactCargoLaws as runExactCargoLaws,configuredRepositoryCargoPreparationPolicyV1} from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
 import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
@@ -17,4 +18,14 @@
 import { scriptInvocationBudget } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 
+/** 📋️ Verifies original persistent canonical arrays and sealed payloads through actual native traversal. */
+class PersistentCanonicalCheckScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-persistent-canonical requires source or native");
+  const source=resolve(this.root,"../../🛫️encode/🧭️tree/📋️persistent/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"pack:persistent-canonical:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"pack:persistent-canonical:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-pack-json":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-pack-json",target:{kind:"lib",name:"semio_framework_pack_json"},laws:["native_tree::persistent_tests::original_persistent_canonical_tree_preserves_native_rows_and_exact_wire","native_tree::immutable_tests::canonical_immutable_table_views_preserve_original_rows_and_zero_heap","native_tree::immutable_tests::canonical_immutable_table_every_frontier_matches_wire_oracle_and_closes"]}]});}
+ }
+}
 /** 🧩️ Runs strict schema, canonical ownership and mandatory JSON source-policy laws. */
 class ReadPolicyScript extends BundleScript {
@@ -69,4 +80,27 @@
 /** 📨️ Differential neutral laws for the original typed Tree wire source. */
 class TreeWireSourceScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("test-tree-wire-source accepts no arguments");const file=resolve(this.root,"../../🛫️encode/🧭️tree/📨️wire/🧪️tests/🟦️.ts");for(const arguments_ of [[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",file],["test",file]])await runBudgetedTestCommand(process.execPath,arguments_,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal,throwOnFailure:true})}}
-const router = new ScriptRouter(import.meta.dir).register("test-tree-wire-source",TreeWireSourceScript).register("test-read-policy-source", ReadPolicyScript).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
+
+
+
+/** 🔒️ Validates original immutable canonical views and every funded output frontier. */
+class ImmutableTreeScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{
+  if(segments.length!==1||!["source","native"].includes(segments[0]!))throw Error("test-immutable-tree source|native");
+  const {immutableCanonicalOracle}=await import("../../🛫️encode/🧭️tree/🔒️immutable/🧪️tests/🟦️.ts");immutableCanonicalOracle();
+  if(segments[0]==="source")return;
+  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-pack-json"],cwd:this.root,extraArgs:["--lib","canonical_immutable_","--no-fail-fast","--success-output","immediate"],signal:this.invocation.control.signal,remainingMilliseconds:()=>this.invocation.control.remainingMilliseconds()},readCargoTestPolicyV1(process.env));
+ }
+}
+
+/** 🧳️ Runs actual original JSON native receiving and canonical operation-input custody. */
+class OriginalReceivingScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-original-receiving source|native");
+  const source=resolve(this.root,"../../📥️decode/🧳️receiving/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"pack:original-receiving:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"pack:original-receiving:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPaths:{"semio-framework-pack-json":resolve(this.root,"Cargo.toml")},cargoTargetDir:policy.targetDirectory,env:process.env,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,groups:[{package:"semio-framework-pack-json",target:{kind:"lib",name:"semio_framework_pack_json"},cargoArgs:["--locked"],laws:["original_receiving::tests::original_json_receiving_keeps_native_source_and_exact_physical_custody"]}]});}
+ }
+}
+const router = new ScriptRouter(import.meta.dir).register("test-tree-wire-source",TreeWireSourceScript).register("test-read-policy-source", ReadPolicyScript).register("test-reference", ReferenceScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript).register("test-immutable-tree",ImmutableTreeScript).register("test-persistent-canonical",PersistentCanonicalCheckScript).register("test-original-receiving",OriginalReceivingScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test-ownership" }));

PATH 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json HEAD 5805 INCOMING 6657
--- BASE
+++ STASH
@@ -88,4 +88,37 @@
       }
     },
+    "test-operation-diagnostics-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-operation-diagnostics source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-operation-diagnostics",
+            "source"
+          ]
+        }
+      },
+      "dependsOn": []
+    },
+    "test-operation-diagnostics-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-operation-diagnostics native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-operation-diagnostics",
+            "native"
+          ]
+        }
+      }
+    },
     "test-original-owned-async-source": {
       "executor": "repo:script",

PATH 🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts HEAD 9763 INCOMING 12459
--- BASE
+++ STASH
@@ -1,4 +1,6 @@
 #!/usr/bin/env bun
 import {verifyProducedWasmPackageV1} from "../../../🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
+import { runRepositoryCommand } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
+import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
 import {executeCargoRequestV1} from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
 import { buildBudgetMs } from '../../../🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts';
@@ -56,5 +58,5 @@
     await executeCargoRequestV1({version:1,kind:"exact-laws",manifest:relative(this.repoRoot,resolve(this.root,"Cargo.toml")).replaceAll("\\","/"),cwd:".",groups: [
       { package: "semio-framework-actor-retained-turn", target: { kind: "lib" }, laws: ["retained_turn::original_loan_tests::original_actor_semantic_loan_preserves_exact_source_total_and_cutoff"] },
-      { package: "semio-framework-actor-retained-context", target: { kind: "lib" }, laws: ["tests::original_actor_epoch_is_exact_and_cannot_be_rebound_by_a_child", "tests::original_actor_context_preserves_input_and_paid_authorities_across_admission_loans_and_close", "tests::original_actor_retirement_binding_preserves_private_source_on_refusal"] },
+      { package: "semio-framework-actor-retained-context", target: { kind: "lib" }, laws: ["tests::original_decoder_tests::original_actor_decoder_keeps_native_candidate_ledger_receipt_and_root_close_order", "tests::original_actor_epoch_is_exact_and_cannot_be_rebound_by_a_child", "tests::original_actor_context_preserves_input_and_paid_authorities_across_admission_loans_and_close", "tests::original_actor_retirement_binding_preserves_private_source_on_refusal"] },
       { package: "semio-framework-actor", target: { kind: "test", name: "original-retained-receiving" }, laws: ["original_paired_admission::original_public_actor_paired_admission_retains_exact_host_input_and_native_guest_births", "original_paired_admission::original_public_actor_guest_call_returns_actual_failed_body_receipt_and_paid_same_context_close"] },
     ]},this.invocation);
@@ -76,4 +78,16 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-ownership",OwnershipScript).register("test-original-receiving-source", OriginalReceivingSourceScript).register("test-original-owned-async",OriginalOwnedAsyncScript).register("test-original-context-native",OriginalContextNativeScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("wasm", WasmScript);
+
+
+/** 📬️ Checks shared original diagnostic publication with independent wire oracles and native custody laws. */
+class OperationDiagnosticPublicationScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-operation-diagnostics requires source or native");
+  const file=resolve(this.root,"../../🖼️wire-turn/⚠️diagnostic/🧪️tests/🟦️.ts"),receiving=resolve(this.root,"../../🖼️wire-turn/📬️typed-operation/📥️receiving/🧪️tests/🟦️.ts"),policy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
+  await runRepositoryCommand(this.invocation,process.execPath,["test",file,receiving],this.repoRoot,"operation-diagnostics-source",policy);
+  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file,receiving],this.repoRoot,"operation-diagnostics-types",policy);
+  if(args[0]==="native")await executeCargoRequestV1({version:1,kind:"exact-laws",manifest:relative(this.repoRoot,resolve(this.root,"Cargo.toml")).replaceAll("\\","/"),cwd:".",groups:[{package:"semio-framework-actor",target:{kind:"lib"},laws:["component::typed_operation_diagnostics::tests::operation_diagnostics_original_pages_preserve_ack_custody_and_physical_receipts","component::typed_operation_diagnostics::tests::operation_diagnostics_cancel_every_original_frontier","component::typed_operation_diagnostics::tests::operation_diagnostics_original4096_capture_demand_fits_production_driver","component::typed_operation_page::receiving::tests::original_host_wire_receiver_preserves_golden_bytes_and_physical_receipts","component::typed_operation_page::receiving::tests::original_host_wire_receiver_cancels_every_actual_action_prefix"]}]},this.invocation);
+ }
+}
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-ownership",OwnershipScript).register("test-original-receiving-source", OriginalReceivingSourceScript).register("test-original-owned-async",OriginalOwnedAsyncScript).register("test-original-context-native",OriginalContextNativeScript).register("typegen", TypegenScript).register("preview-generated", PreviewGeneratedScript).register("wasm", WasmScript).register("test-operation-diagnostics",OperationDiagnosticPublicationScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/🏃️process/📋️project.json HEAD 14762 INCOMING 15169
--- BASE
+++ STASH
@@ -498,5 +498,12 @@
       "executor": "repo:script",
       "cache": false,
-      "metadata": { "semio": { "execution": { "version": 1, "kind": "source" } } },
+      "metadata": {
+        "semio": {
+          "execution": {
+            "version": 1,
+            "kind": "source"
+          }
+        }
+      },
       "dependsOn": [],
       "inputs": [
@@ -510,4 +517,25 @@
         "cwd": "🧰️framework/🔨️modules/🏃️process",
         "command": "bun ./📜️script.ts test exact-cargo-storage"
+      }
+    },
+    "test-resource-lease-waits": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "{projectRoot}/**/*"
+      ],
+      "outputs": [],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🏃️process",
+        "command": "bun ./📜️script.ts test resource-lease-waits"
+      },
+      "metadata": {
+        "semio": {
+          "execution": {
+            "version": 1,
+            "kind": "source"
+          }
+        }
       }
     }

PATH 🧰️framework/🔨️modules/🏃️process/📜️script.ts HEAD 12794 INCOMING 12069
--- BASE
+++ STASH
@@ -42,4 +42,5 @@
     }
     const suites: Readonly<Record<string, { source: string; budgetMs: number; testNamePattern?: string }>> = {
+      "resource-lease-waits": {source:"🔒️leases/⏳️wait/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.quick},
       "exact-cargo-storage": {source:"🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts",budgetMs:TEST_LEVEL_BUDGET_MS.fundamental,testNamePattern:"exact Cargo refuses missing|actual exact Cargo bounded|exact Cargo preparation|Cargo execution|Declared command tails"},
       "compiler-state": { source: "🏗️build/🟦️typescript/🧪️tests/🟦️.ts", budgetMs: TEST_LEVEL_BUDGET_MS.quick },
@@ -80,6 +81,6 @@
     if(segments[0]==="compiler-capacity")sources.push(resolve(this.root,"🔒️leases/📦️bounded/🧪️tests/🟦️.ts"));
     if(segments[0]==="native-storage-platform")sources.push(resolve(this.root,"📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts"),resolve(this.root,"💾️state/🧪️tests/🟦️.ts"));
-    if (["routing", "command", "capture", "compiler-state", "cargo-driver", "cargo-profile","exact-cargo-storage"].includes(segments[0]!)) await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", ...(segments[0] === "compiler-state" ? ["--noUncheckedIndexedAccess"] : []), "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...(segments[0]==="exact-cargo-storage"?[resolve(this.root,"🧪️testing/🦀️cargo/🎯️exact/🟦️.ts"),sources.at(-2)!,sources.at(-1)!]:sources)], this.repoRoot, "process:" + segments[0] + ":types", scriptInvocationBudget(this.invocation, suite.budgetMs), { signal: this.invocation.control.signal });
-    await runOwnedCommand(process.execPath, ["test", ...sources, ...(suite.testNamePattern ? ["--test-name-pattern", suite.testNamePattern] : [])], this.repoRoot, `process:${segments[0]}`, scriptInvocationBudget(this.invocation, suite.budgetMs), { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, signal: this.invocation.control.signal });
+    if (["routing", "command", "capture", "compiler-state", "cargo-driver", "cargo-profile","exact-cargo-laws","exact-cargo-storage"].includes(segments[0]!)) await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", ...(segments[0] === "compiler-state" ? ["--noUncheckedIndexedAccess"] : []), "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...(segments[0]==="exact-cargo-storage"?[resolve(this.root,"🧪️testing/🦀️cargo/🎯️exact/🟦️.ts"),sources.at(-2)!,sources.at(-1)!]:sources)], this.repoRoot, "process:" + segments[0] + ":types", scriptInvocationBudget(this.invocation, suite.budgetMs), { signal: this.invocation.control.signal });
+    await runOwnedCommand(process.execPath, ["test", "--timeout", String(scriptInvocationBudget(this.invocation, suite.budgetMs)), ...sources, ...(suite.testNamePattern ? ["--test-name-pattern", suite.testNamePattern] : [])], this.repoRoot, `process:${segments[0]}`, scriptInvocationBudget(this.invocation, suite.budgetMs), { env: { ...process.env, SEMIO_TEST_ARTIFACT_DIR: output }, signal: this.invocation.control.signal });
   }
 }

PATH 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️script.ts HEAD 2658 INCOMING 2818
--- BASE
+++ STASH
@@ -16,8 +16,8 @@
   if(segments.length)throw Error("physical observation test accepts no arguments");
   await runOwnedCommand(process.execPath,[resolve(this.root,"📜️script.ts"),"test-worker"],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
-  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--target","ES2022","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions",resolve(this.root,"🟦️.ts"),resolve(this.root,"📁️current/🟦️.ts"),resolve(this.root,"🧪️tests/🟦️.ts")],this.repoRoot,"tool:owner",cmdBudgetMs(),{env:process.env});
  }
 }
 /** 🧪️ Executes the complete portable fixture roster in its defining source owner. */
-class WorkerScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("physical observation worker accepts no arguments");const {runFileObservationChecksV1}=await import("./🧪️tests/🟦️.ts");await runFileObservationChecksV1();}}
+class WorkerScript extends BundleScript{async run(segments:string[]):Promise<void>{if(segments.length)throw Error("physical observation worker accepts no arguments");const {runFileObservationChecksV1,runCurrentFileRecheckChecksV1}=await import("./🧪️tests/🟦️.ts");await runCurrentFileRecheckChecksV1();await runFileObservationChecksV1();}}
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(new ScriptRouter(import.meta.dir).register("test",TestScript).register("test-worker",WorkerScript), { invocation: original, ...({defaultCommand:"test"}) }));

PATH 🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📋️project.json HEAD 2122 INCOMING 2480
--- BASE
+++ STASH
@@ -9,4 +9,14 @@
   },
   "targets": {
+    "test-selection-oracle": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": ["workspace:deps-python"],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test oracle selection",
+        "forwardAllArgs": true
+      }
+    },
     "test": {
       "executor": "repo:script",

PATH 🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📜️script.ts HEAD 3016 INCOMING 3461
--- BASE
+++ STASH
@@ -10,5 +10,5 @@
 import { resolveTestLevel, testLevelBudgetMs } from "../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
 
-const ORACLES = ["three", "shapely"] as const;
+const ORACLES = ["three", "shapely", "selection"] as const;
 
 /** 🧪️ `semio-framework-geometry` tests: `test [level]` runs the cargo suites (kurbo and parry3d oracles included); `test oracle [three|shapely]` runs the third-party oracle suites on the committed fixtures: `three` (vitest) and `shapely` (pytest, covering `semio-framework-2d` regions). @see 🧪️tests/🏙️aec-oracles */
@@ -22,5 +22,5 @@
     }
     const { rest } = resolveTestLevel(segments.slice(1));
-    const picked = rest.length ? rest : [...ORACLES];
+    const picked = rest.length ? rest : ["three", "shapely"];
     for (const name of picked) if (!(ORACLES as readonly string[]).includes(name)) throw Error(`Expected test oracle ${ORACLES.join("|")}, got ${JSON.stringify(name)}`);
     const owner = resolve(this.root, "../..");
@@ -28,4 +28,5 @@
     if (picked.includes("three")) await runBudgetedTestCommand("node", [join(this.repoRoot, "node_modules/vitest/vitest.mjs"), "run", "--config", join(owner, "🧪️tests/🎚️config/🟦️.ts")], { cwd: owner, budgetMs });
     if (picked.includes("shapely")) await runBudgetedTestCommand("uv", ["run", "--locked", "--no-sync", "python", "-m", "pytest", "-p", "no:cacheprovider", "-q", join(owner, "🧪️tests/🏙️aec-oracles/🐍️.py")], { cwd: this.repoRoot, budgetMs });
+    if (picked.includes("selection")) await runBudgetedTestCommand("uv", ["run", "--locked", "--no-sync", "python", "-m", "pytest", "-p", "no:cacheprovider", "-q", "-k", "selection", join(owner, "🧪️tests/🏙️aec-oracles/🐍️.py")], { cwd: this.repoRoot, budgetMs });
   }
 }

PATH 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📋️project.json HEAD 2317 INCOMING 3162
--- BASE
+++ STASH
@@ -56,4 +56,27 @@
         "forwardAllArgs": true
       }
+    },
+    "test-map-delta": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun 📜️script.ts test-map-delta",
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript"
+      }
+    },
+    "test-retained-mutation": {
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript",
+        "command": "bun 📜️script.ts test-retained-mutation"
+      },
+      "executor": "repo:script"
+    },
+    "test-diagnostic-transfer": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript",
+        "command": "bun ./📜️script.ts test-diagnostic-transfer",
+        "forwardAllArgs": true
+      }
     }
   }

PATH 🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts HEAD 2344 INCOMING 4031
--- BASE
+++ STASH
@@ -27,5 +27,16 @@
 /** 🎟️ Exercises independently authored original index insertion references. */
 class IndexInsertionTestScript extends BundleScript{async run():Promise<void>{const {testHistoryIndexInsertion}=await import("../../🔗️causal/🔀️transition/🔁️fold/🗂️index/🧪️tests/🎟️insertion/🟦️.ts");testHistoryIndexInsertion();}}
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-index-entry",IndexEntryTestScript).register("test-index-insertion",IndexInsertionTestScript).register("test-index-set",IndexSetTestScript);
+/** 🗂️ Runs shared map transport, composition and inverse laws against Ajv and RFC6902. */
+class MapDeltaTestScript extends BundleScript {
+ async run(args:string[]):Promise<void>{if(args.length)throw Error("test-map-delta accepts no arguments");const{testSharedMapDeltaOracle}=await import("../../🎮️mutation/🗂️map/🧪️tests/🧪️shared-map-delta-source/🟦️.ts");testSharedMapDeltaOracle();console.log("[DEBUG] shared map inverse/composition verified with Ajv and RFC6902");}
+}/** 🧳️ Runs the required retained semantic owner neutral SQLite oracle. */
+class RetainedMutationTestScript extends BundleScript {
+ async run(args:string[]):Promise<void>{if(args.length)throw Error("test-retained-mutation accepts no arguments");const{testRetainedMutationOracle}=await import("../../🎮️mutation/🧳️retained/🧪️tests/🟦️.ts");testRetainedMutationOracle();console.log("[DEBUG] required retained mutation outcomes verified with Ajv and SQLite");}
+}
+/** 📤️ Verifies original diagnostic bundle moves against an independent JSON Patch oracle. */
+class DiagnosticTransferTestScript extends BundleScript {
+ async run(args:string[]):Promise<void>{if(args.length)throw Error("test-diagnostic-transfer accepts no arguments");const{testDiagnosticTransferOracle}=await import("../../🎮️mutation/📨️messages/♻️retire/🧪️tests/📤️transfer/🟦️.ts");testDiagnosticTransferOracle();console.log("[DEBUG] diagnostic transfer neutral cases verified with RFC6902");}
+}
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("test-map-delta",MapDeltaTestScript).register("test-diagnostic-transfer",DiagnosticTransferTestScript).register("test-retained-mutation",RetainedMutationTestScript).register("test-index-entry",IndexEntryTestScript).register("test-index-insertion",IndexInsertionTestScript).register("test-index-set",IndexSetTestScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📋️project.json HEAD 7401 INCOMING 17318
--- BASE
+++ STASH
@@ -19,4 +19,97 @@
   },
   "targets": {
+    "native-diff-contract-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-native-diff-contract source"
+      }
+    },
+    "native-diff-contract-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-native-diff-contract native"
+      }
+    },
+    "inverse-admission-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts inverse-admission-check"
+      }
+    },
+    "inverse-admission-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts inverse-admission-check --native"
+      }
+    },
+    "scoped-identity-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts scoped-identity-check"
+      }
+    },
+    "scoped-identity-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts scoped-identity-check --native"
+      }
+    },
+    "interaction-set-laws-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts interaction-set-laws-check --native"
+      }
+    },
+    "interaction-set-laws-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts interaction-set-laws-check",
+        "forwardAllArgs": true
+      }
+    },
+    "history-invariants-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts history-invariants-check",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "history-invariants-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts history-invariants-check --native"
+      },
+      "outputs": []
+    },
+    "prepared-fields-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts prepared-fields-check --native"
+      },
+      "outputs": []
+    },
     "presence-peer-codec-check": {
       "executor": "repo:script",
@@ -180,4 +273,189 @@
       },
       "outputs": []
+    },
+    "prepared-fields-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts prepared-fields-check",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-diagnostic-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-diagnostic source",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-retained-diagnostic-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-diagnostic native",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-retained-patch-apply-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-patch-apply source",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-retained-patch-apply-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-patch-apply native",
+        "forwardAllArgs": true
+      },
+      "outputs": []
+    },
+    "test-retained-field-change-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-field-change source",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-field-change-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-field-change native",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-diff-sequence-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-diff-sequence source",
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"
+      }
+    },
+    "test-retained-diff-sequence-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-diff-sequence native",
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"
+      }
+    },
+    "test-retained-diff-receiving-source": {
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-diff-receiving",
+            "source"
+          ]
+        }
+      },
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-diff-receiving source",
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"
+      }
+    },
+    "test-retained-diff-receiving-native": {
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-diff-receiving",
+            "native"
+          ]
+        }
+      },
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-diff-receiving native",
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust"
+      }
+    },
+    "test-retained-record-change-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-record-change source",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-record-change-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-record-change native",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-indexed-replacement-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-indexed-replacement source",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-indexed-replacement-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-retained-indexed-replacement native",
+        "forwardAllArgs": true
+      }
+    },
+    "test-history-backing-birth-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-history-backing-birth source",
+        "forwardAllArgs": true
+      }
+    },
+    "test-history-backing-birth-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-history-backing-birth native",
+        "forwardAllArgs": true
+      }
+    },
+    "test-retained-logical-body-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-logical-body source"
+      }
+    },
+    "test-retained-logical-body-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "outputs": [],
+      "options": {
+        "command": "bun ./📜️script.ts test-retained-logical-body native"
+      }
     }
   }

PATH 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts HEAD 32603 INCOMING 63206
--- BASE
+++ STASH
@@ -19,4 +19,6 @@
 import assert from "node:assert/strict";
 import{unsignedLebOracleV1}from"../../../🎒️pack/📐️format/🧪️tests/🔮️oracles/🔢️leb/🟦️.ts";
+import {runOwnedCommand} from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
+import {scriptInvocationBudget} from "../../../🏃️process/🧭️routing/🟦️.ts";
 import { readFileSync } from "node:fs";
 import { join, resolve } from "node:path";
@@ -50,4 +52,59 @@
 }
 
+/** ⚠️ Original semantic diagnostic targets keep their declared code, row and funded custody. */
+class RetainedDiagnosticScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{
+  assert(segments.length===1&&["source","native"].includes(segments[0]!),"test-retained-diagnostic requires source or native");
+  const{testRetainedMutationOracle}=await import("../../🎮️mutation/🧳️retained/🧪️tests/🟦️.ts");testRetainedMutationOracle();
+  if(segments[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);
+  const laws=["retained_semantic_diagnostics_match_neutral_original_code_level_text_and_target","retained_semantic_diagnostic_cancellation_closes_all_original_construction_frontiers","retained_literal_diagnostic_targets_match_portable_outputs_and_partial_cancellation"].map(law=>"mutation::retained::diagnostic::tests::"+law);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs,lawMilliseconds:policy.assertionBudgets[policy.level]},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+  assert.equal(receipts[0]!.assertions,laws.length);console.log("[DEBUG] original semantic diagnostic native laws="+laws.length+" executable="+receipts[0]!.sha256);
+ }
+}
+
+/** 🎚️ Proves typed sparse field decisions and original child cancellation through their declared owner. */
+class RetainedFieldChangeScript extends BundleScript{
+ async run(segments:string[]):Promise<void>{
+  assert(segments.length===1&&["source","native"].includes(segments[0]!),"test-retained-field-change requires source or native");
+  const path=resolve(this.root,"../../🎮️mutation/🧳️retained/🪡️field/🟰️change/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",path],this.repoRoot,"replication:field-change:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  const{testFieldChangeOracle}=await import("../../🎮️mutation/🧳️retained/🪡️field/🟰️change/🧪️tests/🟦️.ts");testFieldChangeOracle();
+  if(segments[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);
+  const laws=["optional_field_sparse_changes_and_inverse_preserve_original_sources","optional_field_partial_cancellation_and_foreign_sources_remain_owned","typed_field_changes_plain_text_bytes_and_ieee_preserve_original_children"].map(law=>"mutation::retained::field::change::tests::"+law);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs,lawMilliseconds:policy.assertionBudgets[policy.level]},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+  assert.equal(receipts[0]!.assertions,laws.length);console.log("[DEBUG] original typed-field native laws="+laws.length+" executable="+receipts[0]!.sha256);
+ }
+}
+/** 🎚️ Proves one funded original record field frontier at a time. */
+class RetainedRecordChangeScript extends BundleScript{
+ async run(segments:string[]):Promise<void>{
+  assert(segments.length===1&&["source","native"].includes(segments[0]!),"test-retained-record-change requires source or native");
+  const path=resolve(this.root,"../../🎮️mutation/🧳️retained/🪡️field/🟰️change/🎚️record/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",path],this.repoRoot,"replication:field-change:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  const{testRecordFieldChangeOracle}=await import("../../🎮️mutation/🧳️retained/🪡️field/🟰️change/🎚️record/🧪️tests/🟦️.ts");testRecordFieldChangeOracle();
+  if(segments[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);
+  const laws=["record_changes_original_sparse_fields_and_paid_frontier_match_neutral_cases","record_changes_partial_original_output_and_child_frontier_close"].map(law=>"mutation::retained::field::change::record::tests::"+law);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs,lawMilliseconds:policy.assertionBudgets[policy.level]},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+  assert.equal(receipts[0]!.assertions,laws.length);console.log("[DEBUG] original typed-field native laws="+laws.length+" executable="+receipts[0]!.sha256);
+ }
+}
+/** 📋️ Proves original whole indexed replacements and paid payload transfer. */
+class RetainedIndexedReplacementScript extends BundleScript{
+ async run(segments:string[]):Promise<void>{
+  assert(segments.length===1&&["source","native"].includes(segments[0]!),"test-retained-indexed-replacement requires source or native");
+  const path=resolve(this.root,"../../🎮️mutation/🧳️retained/🪡️field/🟰️change/📋️indexed/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",path],this.repoRoot,"replication:field-change:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  const{testIndexedReplacementOracle}=await import("../../🎮️mutation/🧳️retained/🪡️field/🟰️change/📋️indexed/🧪️tests/🟦️.ts");testIndexedReplacementOracle();
+  if(segments[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(policy.manifestPath),manifest);
+  const laws=["indexed_replacement_original_rows_and_moved_payloads_match_neutral_cases","indexed_replacement_partial_original_deque_and_output_children_close"].map(law=>"mutation::retained::field::change::indexed::tests::"+law);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs,lawMilliseconds:policy.assertionBudgets[policy.level]},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+  assert.equal(receipts[0]!.assertions,laws.length);console.log("[DEBUG] original typed-field native laws="+laws.length+" executable="+receipts[0]!.sha256);
+ }
+}
 class SourceTestScript extends BundleScript {
   async run(): Promise<void> {
@@ -427,6 +484,176 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test-command-ingress-ownership", CommandIngressOwnershipScript).register("test-command-ingress-native", CommandIngressNativeScript).register("test", TestScript)
-  .register("test-original-conflict-source",OriginalConflictSourceScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript).register("retained-verification-check", RetainedVerificationScript).register("retained-record-observation-check", RetainedRecordObservationScript).register("presence-peer-codec-check", PresencePeerCodecScript).register("test-crate-partition", CratePartitionLawScript);
-
+
+/** 🧾️ Compares shared portable field publication with an independent SQLite transaction. */
+class PreparedFieldsScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.some(segment => segment !== "--native")) throw Error("prepared-fields-check accepts only --native");
+    const { Database } = await import("bun:sqlite");
+    const { default: Ajv } = await import("ajv/dist/2020");
+    const owner = join(this.root, "../../🎮️mutation");
+    const corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/📦️prepared-fields/🔣️.json"), "utf8"));
+    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/📦️prepared-fields/🔣️.json"), "utf8"));
+    const validator = new Ajv({ strict: true }).compile(schema);
+    assert(validator(corpus), JSON.stringify(validator.errors));
+    const database = new Database(":memory:");
+    try {
+      database.exec("CREATE TABLE projection (first INTEGER NOT NULL, second INTEGER NOT NULL)");
+      for (const row of corpus.cases) {
+        database.exec("DELETE FROM projection");
+        database.query("INSERT INTO projection VALUES (?, ?)").run(corpus.base.first, corpus.base.second);
+        database.exec("BEGIN");
+        if (row.diff.first !== undefined) database.query("UPDATE projection SET first = ?").run(row.diff.first);
+        if (row.diff.second !== undefined) database.query("UPDATE projection SET second = ?").run(row.diff.second);
+        database.exec(row.refused ? "ROLLBACK" : "COMMIT");
+        assert.deepEqual(database.query("SELECT first, second FROM projection").get(), row.expected, row.id);
+      }
+    } finally { database.close(); }
+    console.log("[DEBUG] Prepared-field portable atomic outcomes agree with Ajv and independent SQLite transactions");
+    if (!segments.includes("--native")) return;
+    const laws = ["unpublished_owner_applies_without_projection_clones", "unpublished_owner_refusal_is_atomic_and_retires_prepared_fields", "cold_owner_refusal_retires_scratch_and_preserves_original", "prepared_fields_follow_portable_atomic_outcomes"];
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 600_000 }, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => "mutation::unpublished_owner_tests::" + law) }] });
+    assert.equal(receipts[0]!.assertions, laws.length);
+  }
+}
+
+/** 🪪️ Checks shared history ownership and UTF-8 event order against SQLite and the native retained fold. */
+class HistoryInvariantsScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    const { testHistoryInvariants } = await import("../../🔗️causal/🧪️tests/🪪️history-invariants/🟦️.ts");
+    testHistoryInvariants();
+    if (!segments.includes("--native")) return;
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: ["causal::transition::tests::history_invariants_follow_portable_ownership_and_utf8_vectors"] }] });
+    assert.equal(receipts[0]!.assertions, 1);
+  }
+}
+
+const router = new ScriptRouter(import.meta.dir).register("history-invariants-check", HistoryInvariantsScript).register("prepared-fields-check", PreparedFieldsScript).register("test-command-ingress-ownership", CommandIngressOwnershipScript).register("test-command-ingress-native", CommandIngressNativeScript).register("test", TestScript)
+  .register("test-retained-indexed-replacement",RetainedIndexedReplacementScript).register("test-retained-record-change",RetainedRecordChangeScript).register("test-retained-field-change",RetainedFieldChangeScript).register("test-retained-diagnostic",RetainedDiagnosticScript).register("test-original-conflict-source",OriginalConflictSourceScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript).register("retained-verification-check", RetainedVerificationScript).register("retained-record-observation-check", RetainedRecordObservationScript).register("presence-peer-codec-check", PresencePeerCodecScript).register("test-crate-partition", CratePartitionLawScript);
+
+/** 🎯️ Checks ordered set selection and bounded topology traversal in both implementations. */
+class InteractionSetLawsScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.some(segment => segment !== "--native")) throw Error("interaction-set-laws-check accepts only --native");
+    const { testInteractionSetLaws } = await import("../../../🕹️interaction/🧪️tests/🎯️set-laws/🟦️.ts");
+    testInteractionSetLaws();
+    await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--esModuleInterop", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--jsx", "react-jsx", "--types", "bun,vite/client", resolve(this.root, "../../../🕹️interaction/🧪️tests/🎯️set-laws/🟦️.ts")], { cwd: this.repoRoot, budgetMs: cmdBudgetMs(), throwOnFailure: true });
+    if (!segments.includes("--native")) return;
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 600_000 }, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: ["wire::frames::interaction_tests::interaction_set_laws_follow_portable_vectors"] }] });
+    assert.equal(receipts[0]!.assertions, 1);
+  }
+}
+router.register("interaction-set-laws-check", InteractionSetLawsScript);
+
+/** 🏷️ Checks shared scoped interaction identities against the independent UTF-8 oracle. */
+class ScopedIdentityScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.some(segment => segment !== "--native")) throw Error("scoped-identity-check accepts only --native");
+    const { runOwnedCommand } = await import("../../../🏃️process/🎛️owned-execution/🟦️.ts");
+    const { scriptInvocationBudget } = await import("../../../🏃️process/🧭️routing/🟦️.ts");
+    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", resolve(this.root, "../../🧾️wire/🪪️scoped-identity/🧪️tests/🟦️.ts")], this.repoRoot, "scoped-identity:source-typecheck", scriptInvocationBudget(this.invocation, 120000), { signal: this.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
+    const { testScopedIdentities } = await import("../../🧾️wire/🪪️scoped-identity/🧪️tests/🟦️.ts");
+    testScopedIdentities();
+    if (!segments.includes("--native")) return;
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 600_000 }, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, cargoArgs: ["--locked"], laws: ["wire::scoped_identity::tests::scoped_identities_follow_portable_vectors_and_bounds", "wire::scoped_identity::tests::scoped_identity_visitor_is_atomic_ordered_and_allocation_free"] }] });
+    assert.equal(receipts.length, 1);
+    assert.equal(receipts[0]!.assertions, 2);
+    console.log(`[DEBUG] Scoped identity exact native verification passed ${receipts[0]!.assertions} laws; executable ${receipts[0]!.sha256}`);
+  }
+}
+router.register("scoped-identity-check", ScopedIdentityScript);
+
+/** ↩️ Verifies cold inverse admission and the aggregate derive against the neutral contract. */
+class InverseAdmissionScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.some(segment => segment !== "--native")) throw Error("inverse-admission-check accepts only --native");
+    const { testInverseAdmission } = await import("../../🎮️mutation/🧪️tests/↩️inverse-admission/🟦️.ts");
+    testInverseAdmission();
+    if (!segments.includes("--native")) return;
+    const deriveManifest = resolve(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml");
+    const receipts = await runExactCargoLaws({ invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: 600_000 }, artifactDir: process.env.SEMIO_TEST_ARTIFACT_DIR, manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml"), "semio-framework-os-kernel-dsl-derive": deriveManifest }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [
+      { package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, cargoArgs: ["--locked"], laws: ["mutation::inverse_admission_tests::inverse_admission_follows_portable_outcomes_and_retires_each_diff"] },
+      { package: "semio-framework-os-kernel-dsl-derive", target: { kind: "lib", name: "dsl_derive" }, cargoArgs: ["--locked"], laws: ["mandatory_mutations_tests::expanded_aggregate_matches_neutral_contract_and_syn_ast"] },
+    ] });
+    assert.deepEqual(receipts.map(receipt => receipt.assertions), [1, 1]);
+  }
+}
+router.register("inverse-admission-check", InverseAdmissionScript);
+
+
+/** 🚚️ Common sparse central application verifies portable fields and original physical cancellation. */
+class RetainedRowPatchApplyScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{
+  assert(segments.length===1&&["source","native"].includes(segments[0]!),"test-retained-patch-apply requires source or native");
+  const source=resolve(this.root,"../../🎮️mutation/🧳️retained/🩹️patch/🧪️tests/🟦️.ts");
+  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
+  const {verifyRetainedRowPatchOracle}=await import("../../🎮️mutation/🧳️retained/🩹️patch/🧪️tests/🟦️.ts");verifyRetainedRowPatchOracle();
+  if(segments[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");
+  const laws=["retained_row_patch_inverse_and_owned_exchange_match_neutral_sparse_fields","retained_row_patch_inverse_cancellation_closes_every_original_field","retained_row_patch_apply_preserves_portable_rows_and_every_cancelled_frontier","close_demand::retained_row_patch_close_quote_visits_original_child_once"].map(law=>"mutation::retained::patch::tests::"+law);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},cargoTargetDir:policy.targetDirectory,nativeEnv:{RUST_MIN_STACK:policy.rustMinStack},cwd:this.repoRoot,groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+  assert.equal(receipts[0]!.assertions,laws.length);console.log("[DEBUG] original sparse row apply exact native laws="+laws.length+" executable="+receipts[0]!.sha256);
+ }
+}
+router.register("test-retained-patch-apply",RetainedRowPatchApplyScript);
+
+/** 🪢️ Original ordered children retain intermediate source, diagnostic and candidate custody. */
+class RetainedDiffSequenceScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-diff-sequence requires source or native");
+  const source=resolve(this.root,"../../🎮️mutation/🧳️retained/🔺️diff/🪢️sequence/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"replication:retained-diff-sequence:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"replication:retained-diff-sequence:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-replication",target:{kind:"test",name:"retained_diff_sequence"},laws:["retained_diff_sequence_matches_original_intermediate_sqlite_values_and_refusal","retained_diff_sequence_cancellation_closes_original_sources_diagnostics_and_candidates","retained_diff_sequence_wide_original_projection_and_every_custody_cut_are_genuine"]}]});}
+ }
+}
+router.register("test-retained-diff-sequence",RetainedDiffSequenceScript);
+/** 🫴️ Original receiving control reaches the required central typed apply owner unchanged. */
+class RetainedDiffReceivingScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-diff-receiving requires source or native");
+  const oracle=await import("../../🎮️mutation/🧳️retained/🔺️diff/🫴️receiving/🧪️tests/🟦️.ts");oracle.retainedDiffReceivingOracle();
+  if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env);await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000)},cwd:this.repoRoot,cargoTargetDir:p.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws:["mutation::retained::diff::tests::receiving::retained_central_diff_forwards_actual_original_receiving_control_and_ledger"]}]});}
+ }
+}
+router.register("test-retained-diff-receiving",RetainedDiffReceivingScript);
+/** 🧳️ Native persisted deltas do not require synchronous receiving or unpriced coalescing. */
+class NativeDiffContractScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&['source','native'].includes(args[0]!), 'test-native-diff-contract requires source or native');
+  const source=resolve(this.root,'../../🎮️mutation/🧪️tests/🧳️native-contract/🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'replication:native-diff-contract:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const{verifyNativeDiffContractOracle}=await import('../../🎮️mutation/🧪️tests/🧳️native-contract/🧪️tests/🟦️.ts');verifyNativeDiffContractOracle();if(args[0]==='source')return;
+  const{runRepositoryExactCargoLaws}=await import('../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts');
+  const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:resolve(this.root,'Cargo.toml'),artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,groups:[{package:'semio-framework-replication',target:{kind:'lib',name:'protocol'},laws:['mutation::native_diff_contract_tests::native_delta_vocabulary_does_not_require_cold_coalescing_or_receiving']}]});
+  assert.equal(receipts[0]!.assertions,1);console.log('[DEBUG] Native delta contract exact runtime law='+receipts[0]!.assertions+' executable='+receipts[0]!.sha256);
+ }
+}
+router.register('test-native-diff-contract',NativeDiffContractScript);
+/** 🏗️ Fold backing admission retains unborn allocations and exact original cancellation frames. */
+class HistoryBackingBirthScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-history-backing-birth requires source or native");
+  const source=resolve(this.root,"../../🔗️causal/🔀️transition/🔁️fold/🏗️birth/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"replication:fold-backing-birth:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"replication:fold-backing-birth:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");
+  const laws=["granted_fold_backing_birth_waits_before_frame_and_vector_allocation","granted_fold_waiting_backing_cancellation_never_allocates_the_unborn_vector","granted_trunk_identity_matches_the_cold_blake3_wire_oracle"].map(name=>"causal::transition::retained_fold::original_backing_birth::tests::"+name);
+  await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":manifest},groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws}]});
+ }
+}
+router.register("test-history-backing-birth",HistoryBackingBirthScript);
+/** 📏️ Qualifies original logical body independently of physical protocol headers. */
+class RetainedLogicalBodyScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  assert(args.length===1&&["source","native"].includes(args[0]!),"test-retained-logical-body requires source or native");
+  const source=resolve(this.root,"../../🎮️mutation/🧳️retained/📏️body/🧪️tests/🟦️.ts"),budget=scriptInvocationBudget(this.invocation,120000);
+  await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"replication:retained-logical-body:types",budget,{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test","--timeout",String(scriptInvocationBudget(this.invocation,budget)),source],this.repoRoot,"replication:retained-logical-body:source",scriptInvocationBudget(this.invocation,budget),{signal:this.invocation.control.signal});
+  if(args[0]==="source")return;
+  const policy=readCargoTestPolicyV1(process.env);
+  await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,policy.buildBudgetMs>0?policy.buildBudgetMs:600000)},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-replication":resolve(this.root,"Cargo.toml")},groups:[{package:"semio-framework-replication",target:{kind:"lib",name:"protocol"},cargoArgs:["--locked"],laws:["mutation::retained::field::tests::retained_field_logical_body_survives_expanded_physical_headers","mutation::retained::span::tests::retained_byte_span_logical_body_survives_expanded_physical_headers"]}]});
+ }
+}
+router.register("test-retained-logical-body",RetainedLogicalBodyScript);
 if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json HEAD 13806 INCOMING 15151
--- BASE
+++ STASH
@@ -23,4 +23,13 @@
   },
   "targets": {
+    "test-tui": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test tui"
+      },
+      "inputs": ["default", "^production", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/⌨️tui/**/*", "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🧱️elements/**/*"]
+    },
     "check-command-types": {
       "executor": "repo:script",

PATH 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📋️project.json HEAD 1625 INCOMING 2203
--- BASE
+++ STASH
@@ -4,22 +4,66 @@
   "sourceRoot": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema",
   "namedInputs": {
-    "default": ["{workspaceRoot}/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/**/*", "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/**/*"]
+    "default": [
+      "{workspaceRoot}/🧰️framework/🔨️modules/🗣️dsl/🧬️schema/**/*",
+      "{workspaceRoot}/🧰️framework/🔨️modules/🏃️process/**/*"
+    ]
   },
   "targets": {
+    "test-presence": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-presence"
+      }
+    },
+    "test-ownership": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-ownership"
+      }
+    },
+    "test-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-native"
+      }
+    },
     "test-original-receiving-source": {
-      "executor":"repo:script","cache":false,"dependsOn":[],"inputs":["default"],
-      "options":{"cwd":"🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust","command":"bun ./📜️script.ts test-original-receiving source"}
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-receiving source"
+      }
     },
     "test-original-receiving-native": {
-      "executor":"repo:script","cache":false,"dependsOn":[],"inputs":["default"],
-      "options":{"cwd":"🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust","command":"bun ./📜️script.ts test-original-receiving native"}
-    },
-    "test-ownership": {
-      "executor": "repo:script", "cache": false, "inputs": ["default"],
-      "options": {"cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test-ownership"}
-    },
-    "test-native": {
-      "executor": "repo:script", "cache": false, "inputs": ["default"],
-      "options": {"cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust", "command": "bun ./📜️script.ts test-native"}
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "default"
+      ],
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-receiving native"
+      }
     }
   }

PATH 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📜️script.ts HEAD 4455 INCOMING 5110
--- BASE
+++ STASH
@@ -17,4 +17,12 @@
     await runBudgetedTestCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", test], {cwd: this.repoRoot, budgetMs: 30000, throwOnFailure: true});
     await runBudgetedTestCommand(process.execPath, ["test", test], {cwd: this.repoRoot, budgetMs: testLevelBudgetMs(), throwOnFailure: true});
+  }
+}
+
+/** 📨️ Compares the neutral presence corpus against independent JSON Schema validation. */
+class PresenceScript extends BundleScript {
+  async run(segments:string[]):Promise<void>{
+    if(segments.length)throw Error("test-presence accepts no arguments");
+    await runBudgetedTestCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/📨️presence/🟦️.ts")],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
   }
 }
@@ -41,4 +49,5 @@
  }
 }
-const router = new ScriptRouter(import.meta.dir).register("test-original-receiving",OriginalReceivingScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript);
+
+const router = new ScriptRouter(import.meta.dir).register("test-original-receiving",OriginalReceivingScript).register("test-ownership", OwnershipScript).register("test-native", NativeScript).register("test-presence",PresenceScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand: "test-ownership"}) }));

PATH 🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📋️project.json HEAD 1275 INCOMING 2467
--- BASE
+++ STASH
@@ -32,4 +32,36 @@
         "command": "bun ./📜️script.ts test-binding"
       }
+    },
+    "test-retained-equality-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust -- bun ./📜️script.ts test-retained-equality source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-equality",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-retained-equality-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework",
+        "command": "bun ./🔨️modules/🏃️process/📜️script.ts command --config ./⚙️configuration/🏃️process/🔣️.json --cwd 🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust -- bun ./📜️script.ts test-retained-equality native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-retained-equality",
+            "native"
+          ]
+        }
+      }
     }
   }

PATH 🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📜️script.ts HEAD 2981 INCOMING 5371
--- BASE
+++ STASH
@@ -1,4 +1,7 @@
 #!/usr/bin/env bun
 import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
+import { scriptInvocationBudget } from "../../../🏃️process/🧭️routing/🟦️.ts";
+import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
+import { configuredExactCargoLawPolicyV1, runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
 import { runRepositoryCargoTestsV1 as runCargoTestsV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -31,4 +34,15 @@
 }
 
-const router=new ScriptRouter(import.meta.dir).register("test-native",NativeScript).register("test-binding",BindingScript);
+/** 🟰️ Verifies exact retained semantic identity fields against SQLite and original native leases. */
+class EqualityScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-retained-equality requires source or native");
+  const source=resolve(this.root,"../../../🧬️schema/🗿️artifact-reference/🟰️equal/🧪️tests/🟦️.ts");
+  await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"artifact-reference:equality:types",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"artifact-reference:equality:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal});
+  if(args[0]==="native"){const policy=readCargoTestPolicyV1(process.env);await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:policy.buildBudgetMs},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,manifestPaths:{"semio-framework-artifact-reference":resolve(this.root,"Cargo.toml")},extraCargoArgs:["--locked"],groups:[{package:"semio-framework-artifact-reference",target:{kind:"lib",name:"semio_framework_artifact_reference"},laws:["equality_tests::original_artifact_reference_equality_preserves_all_exact_identity_fields","equality_tests::original_artifact_reference_equality_cancellation_keeps_source_and_cursor_custody"]}]});}
+ }
+}
+
+const router=new ScriptRouter(import.meta.dir).register("test-retained-equality",EqualityScript).register("test-native",NativeScript).register("test-binding",BindingScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({defaultCommand:"test-native"}) }));

PATH 🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts HEAD 9294 INCOMING 9444
--- BASE
+++ STASH
@@ -87,4 +87,5 @@
 }
 
+
 const router = new ScriptRouter(import.meta.dir).register("test-original-fixed-limits",OriginalFixedLimitsScript).register("test-original-physical",OriginalPhysicalScript).register("test-ownership",OwnershipScript).register("test-operation",OperationScript).register("test-refusal", RefusalScript).register("test-native", NativeScript).register("test-receiving-schema",ReceivingSchemaScript);
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test-refusal" }) }));

PATH 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📋️project.json HEAD 917 INCOMING 2516
--- BASE
+++ STASH
@@ -10,4 +10,6 @@
   },
   "targets": {
+    "verify-rank-source":{"executor":"repo:script","cache":false,"options":{"command":"bun ./📜️script.ts verify-rank source","cwd":"🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust"}},
+    "verify-rank-native":{"executor":"repo:script","cache":false,"options":{"command":"bun ./📜️script.ts verify-rank","cwd":"🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust"}},
     "test": {
       "executor": "repo:script",
@@ -24,4 +26,37 @@
         "command": "bun ./📜️script.ts lint"
       }
+    },
+    "verify-dense-source": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts verify-dense-source"
+      }
+    },
+    "verify-dense-native": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts verify-dense-native retained_dense",
+        "forwardAllArgs": true
+      }
+    },
+    "verify-inverse-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts verify-inverse-source",
+        "forwardAllArgs": true,
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust"
+      }
+    },
+    "verify-inverse-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts verify-inverse-native retained_inverse",
+        "forwardAllArgs": true,
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust"
+      }
     }
   }

PATH 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts HEAD 2500 INCOMING 6755
--- BASE
+++ STASH
@@ -1,4 +1,6 @@
 #!/usr/bin/env bun
 import{createIssuedResourceControlV1}from"../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
+import { runOwnedCommand } from "../../../🏃️process/🎛️owned-execution/🟦️.ts";
+import { scriptInvocationBudget } from "../../../🏃️process/🧭️routing/🟦️.ts";
 import { runRepositoryCargoTestsV1 as runCargoTestsV1, runRepositoryCargoLintV1 as runCargoLintV1 } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -29,5 +31,28 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript);
+/** 🔲️ Portable retained dense cases and independent Mathjs reference. */
+class DenseSourceScript extends BundleScript { async run(): Promise<void> { const source=resolve(this.root,"../../../🧮️math/🧮️algebra/🔲️dense/🧪️tests/🟦️.ts");await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"math:dense-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});const {verifyRetainedDenseOracle}=await import("../../../🧮️math/🧮️algebra/🔲️dense/🧪️tests/🟦️.ts");verifyRetainedDenseOracle(); } }
+
+
+/** ↔️ Original retained inverse cases, full49-order residuals and independent Mathjs reference. */
+class InverseSourceScript extends BundleScript {async run():Promise<void>{
+ const source=resolve(this.root,"../../../🧮️math/🧮️algebra/🔲️dense/↔️inverse/🧪️tests/🟦️.ts");
+ await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"math:inverse-source-typecheck",scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal,output:{maximumBytes:8388608,maximumLines:65536}});
+ const {verifyRetainedInverseOracle}=await import("../../../🧮️math/🧮️algebra/🔲️dense/↔️inverse/🧪️tests/🟦️.ts");verifyRetainedInverseOracle();
+}}
+
+/** 📶️ Verifies original rank preparation against its neutral SQLite oracle and actual native custody. */
+class RankScript extends BundleScript{async run(args:string[]):Promise<void>{
+ if(args.length>1||(args.length===1&&args[0]!=="source"))throw Error("Expected verify-rank [source]");
+ const file=resolve(this.root,"../../../🧮️math/📶️rank/🧪️tests/🟦️.ts"),options={signal:this.invocation.control.signal};
+ await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",file],this.repoRoot,"math:rank:types",scriptInvocationBudget(this.invocation,120000),options);
+ await runOwnedCommand(process.execPath,["test",file],this.repoRoot,"math:rank:source",scriptInvocationBudget(this.invocation,30000),options);
+ if(args[0]!=="source")await runCargoTestsV1({manifestPath:resolve(this.repoRoot,"🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust/Cargo.toml"),packages:["semio-framework-math-rank"],cwd:this.root,extraArgs:["--lib","original_scalar_rank","--","--nocapture"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);
+}}
+
+/** 🧮️ Native algebra gates select their actual narrow retained owner. */
+class DenseNativeScript extends BundleScript{async run(args:string[]):Promise<void>{await runCargoTestsV1({manifestPath:resolve(this.repoRoot,"🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust/Cargo.toml"),packages:["semio-framework-math-dense"],cwd:this.root,extraArgs:args},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);}}
+
+const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("lint", LintScript).register("verify-dense-source", DenseSourceScript).register("verify-inverse-source",InverseSourceScript).register("verify-rank",RankScript).register("verify-dense-native",DenseNativeScript).register("verify-inverse-native",DenseNativeScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust/📋️project.json HEAD 0 INCOMING 610
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+{
+  "name": "@semio-tech/math-rank-rs",
+  "projectType": "library",
+  "targets": {
+    "test-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-source",
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust"
+      }
+    },
+    "test": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test",
+        "cwd": "🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust"
+      }
+    }
+  }
+}

PATH 🧰️framework/🔨️modules/🧮️math/📶️rank/📦️packages/🦀️rust/📜️script.ts HEAD 0 INCOMING 1890
--- BASE
+++ STASH
@@ -0,0 +1,12 @@
+#!/usr/bin/env bun
+import { resolve } from "node:path";
+import { BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,advanceScriptInvocation } from "../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
+import { runRepositoryCargoTestsV1 } from "../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
+import { readCargoTestPolicyV1 } from "../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
+import { runOwnedCommand } from "../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
+/** 🧪️ Runs the owned neutral corpus against its independent implementation. */
+class SourceScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-source");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts")],this.repoRoot,"math-rank:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-source","running")});}}
+/** 🦀️ Runs original native math ownership laws through repository Cargo admission. */
+class TestScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test");await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-math-rank"],cwd:this.root,extraArgs:["--lib"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);}}
+const router=new ScriptRouter(import.meta.dir).register("test-source",SourceScript).register("test",TestScript);
+await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust/📋️project.json HEAD 0 INCOMING 643
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+{
+  "name": "@semio-tech/math-dense-rs",
+  "projectType": "library",
+  "targets": {
+    "test-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-source",
+        "cwd": "🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust"
+      }
+    },
+    "test": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test",
+        "cwd": "🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust"
+      }
+    }
+  }
+}

PATH 🧰️framework/🔨️modules/🧮️math/🧮️algebra/🔲️dense/📦️packages/🦀️rust/📜️script.ts HEAD 0 INCOMING 1969
--- BASE
+++ STASH
@@ -0,0 +1,12 @@
+#!/usr/bin/env bun
+import { resolve } from "node:path";
+import { BundleScript,ScriptRouter,receiveScriptProcessInvocation,runScriptMain,scriptInvocationBudget,advanceScriptInvocation } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
+import { runRepositoryCargoTestsV1 } from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";
+import { readCargoTestPolicyV1 } from "../../../../../🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
+import { runOwnedCommand } from "../../../../../🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
+/** 🧪️ Runs the owned neutral corpus against its independent implementation. */
+class SourceScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-source");await runOwnedCommand(process.execPath,["test",resolve(this.root,"../../🧪️tests/🟦️.ts"),resolve(this.root,"../../↔️inverse/🧪️tests/🟦️.ts")],this.repoRoot,"math-dense:source",scriptInvocationBudget(this.invocation,30000),{signal:this.invocation.control.signal,onProgress:()=>advanceScriptInvocation(this.invocation,"test-source","running")});}}
+/** 🦀️ Runs original native math ownership laws through repository Cargo admission. */
+class TestScript extends BundleScript {async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test");await runRepositoryCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-math-dense"],cwd:this.root,extraArgs:["--lib"]},readCargoTestPolicyV1(process.env),this.repoRoot,this.invocation);}}
+const router=new ScriptRouter(import.meta.dir).register("test-source",SourceScript).register("test",TestScript);
+await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📋️project.json HEAD 3859 INCOMING 5039
--- BASE
+++ STASH
@@ -10,18 +10,18 @@
   },
   "targets": {
-    "test-original-turn-failure-source": {
+    "test-original-receiving-source": {
       "executor": "repo:script",
       "cache": false,
       "options": {
         "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
-        "command": "bun ./📜️script.ts test-original-turn-failure source"
+        "command": "bun ./📜️script.ts test-original-receiving source"
       }
     },
-    "test-original-turn-failure-native": {
+    "test-original-receiving-native": {
       "executor": "repo:script",
       "cache": false,
       "options": {
         "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
-        "command": "bun ./📜️script.ts test-original-turn-failure native"
+        "command": "bun ./📜️script.ts test-original-receiving native"
       }
     },
@@ -104,4 +104,36 @@
         "command": "bun ./📜️script.ts test-original-retained-loan-native"
       }
+    },
+    "test-original-turn-failure-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-turn-failure source"
+      }
+    },
+    "test-original-turn-failure-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-turn-failure native"
+      }
+    },
+    "test-original-page-return-source": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-page-return source"
+      },
+      "cache": false
+    },
+    "test-original-page-return-native": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-page-return native"
+      },
+      "cache": false
     }
   }

PATH 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts HEAD 13203 INCOMING 17563
--- BASE
+++ STASH
@@ -76,5 +76,5 @@
   await runOwnedCommand(process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"job:original-failure-types",remaining,{env:process.env});
   await runOwnedCommand(process.execPath,["test",source],this.repoRoot,"job:original-failure-source",this.invocation.control.remainingMilliseconds()??0,{env:process.env});
-  if(segments[0]==="native"){const remaining=this.invocation.control.remainingMilliseconds();if(remaining===null||remaining<=0)throw Error("Original native failure qualification exhausted its caller");await runExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:remaining,listMilliseconds:remaining,lawMilliseconds:remaining},manifestPaths:{"semio-framework-job":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-framework-job",target:{kind:"test",name:"original-turn-failure"},cargoArgs:["--features","testkit"],laws:["original_turn_failure_debit_preserves_both_owned_causes","original_turn_failure_paid_retirement_preserves_denied_sources","original_turn_failure_preserves_full_diagnostic_custody","original_turn_failure_preserves_three_complete_owned_causes","original_turn_failure_domain_admission_preserves_full_source_and_paid_close","original_closure_tests::original_source_closure_full_pair_same_caller_denials_and_paid_close"]}]});}
+  if(segments[0]==="native"){const remaining=this.invocation.control.remainingMilliseconds();if(remaining===null||remaining<=0)throw Error("Original native failure qualification exhausted its caller");await runExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:remaining,listMilliseconds:remaining,lawMilliseconds:remaining},manifestPaths:{"semio-framework-job":resolve(this.root,"Cargo.toml")},cwd:this.repoRoot,groups:[{package:"semio-framework-job",target:{kind:"test",name:"original-turn-failure"},cargoArgs:["--features","testkit"],laws:["original_turn_failure_debit_preserves_both_owned_causes","original_turn_failure_paid_retirement_preserves_denied_sources","original_turn_failure_preserves_full_diagnostic_custody","original_turn_failure_preserves_three_complete_owned_causes","original_turn_failure_receiving_preserves_five_owned_causes","original_turn_failure_domain_admission_preserves_full_source_and_paid_close","original_closure_tests::original_source_closure_full_pair_same_caller_denials_and_paid_close"]}]});}
  }
 }
@@ -104,5 +104,22 @@
 }
 
-const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("canonical-architecture", CanonicalArchitectureScript).register("test-close-demand-source", CloseDemandSourceScript).register("test-retained-source", RetainedSourceScript).register("test-original-turn-failure",OriginalTurnFailureScript).register("test-original-close-context-source",OriginalCloseContextScript).register("test-original-close-context-native",OriginalCloseContextNativeScript).register("test-original-retained-loan-native",OriginalRetainedLoanNativeScript);
+
+
+/** 🛬️ Validates original Job decoder custody against the portable independent oracle and exact native law. */
+class OriginalReceivingScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{const mode=segments.shift();if(segments.length||!['source','native'].includes(mode??''))throw Error('Expected source or native');const source=resolve(this.root,'../../🛬️receiving/🧪️tests/🟦️.ts');await runOwnedCommand(process.execPath,[resolve(this.repoRoot,'node_modules/typescript/bin/tsc'),'--noEmit','--strict','--skipLibCheck','--allowImportingTsExtensions','--resolveJsonModule','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'job:original-receiving:types',Math.min(30000,this.invocation.control.remainingMilliseconds()),{signal:this.invocation.control.signal});const {originalJobReceivingOracle}=await import('../../🛬️receiving/🧪️tests/🟦️.ts');originalJobReceivingOracle();if(mode==='source')return;await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:240000,listMilliseconds:30000,lawMilliseconds:30000},manifestPaths:{'semio-framework-job':resolve(this.root,'Cargo.toml')},cargoTargetDir:readCargoTestPolicyV1(process.env).targetDirectory,cwd:this.repoRoot,groups:[{package:'semio-framework-job',target:{kind:'lib'},laws:['component::original_receiving::tests::original_job_receiving_preserves_ledger_recipient_and_cumulative_capacity','component::original_receiving::tests::original_context_charges_child_construction_and_enclosing_semantic_once','component::original_receiving::tests::original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap']}],artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR});}
+}
+/** 📮️ Executes the original supplied-page corpus and its authentic native custody law. */
+class OriginalPageReturnScript extends BundleScript {
+ async run(segments:string[]):Promise<void>{
+  const mode=segments.shift();if(segments.length||!['source','native'].includes(mode??''))throw Error('Expected source or native');
+  const sources=['../../📬️outcome/📮️return/🧪️tests/🟦️.ts','../../📬️outcome/📮️return/📦️bytes/🧪️tests/🟦️.ts'].map(source=>resolve(this.root,source));
+  await runOwnedCommand(process.execPath,['test',...sources],this.repoRoot,'job:original-page-return:source',Math.min(30000,this.invocation.control.remainingMilliseconds()),{signal:this.invocation.control.signal});
+  if(mode==='source')return;
+  const remaining=this.invocation.control.remainingMilliseconds();if(remaining===null||remaining<=0)throw Error('Original page return exhausted its caller');
+  await runExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:remaining,listMilliseconds:remaining,lawMilliseconds:remaining},manifestPaths:{'semio-framework-job':resolve(this.root,'Cargo.toml')},cwd:this.repoRoot,groups:[{package:'semio-framework-job',target:{kind:'test',name:'original-page-return'},cargoArgs:['--features','testkit'],laws:['original_supplied_page_returns_same_backing_before_original_source_close','original_supplied_pages_preserve_ordinals_and_sealed_prefix_during_partial_transfer','original_supplied_page_notification_observes_published_return_without_owner_or_closure_birth','original_contiguous_bytes_keep_exact_birth_fill_and_return_on_the_same_source']}]});
+ }
+}
+const router = new ScriptRouter(import.meta.dir).register("test-original-page-return",OriginalPageReturnScript).register("test", TestScript).register("canonical-architecture", CanonicalArchitectureScript).register("test-close-demand-source", CloseDemandSourceScript).register("test-retained-source", RetainedSourceScript).register("test-original-turn-failure",OriginalTurnFailureScript).register("test-original-close-context-source",OriginalCloseContextScript).register("test-original-close-context-native",OriginalCloseContextNativeScript).register("test-original-retained-loan-native",OriginalRetainedLoanNativeScript).register("test-original-receiving",OriginalReceivingScript);
 
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json HEAD 3162 INCOMING 3456
--- BASE
+++ STASH
@@ -5,4 +5,7 @@
     "default": [
       "{projectRoot}/**/*",
+      "{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🪂️canopies/🔣️.json",
+      "{workspaceRoot}/🎓️teaching/🏛️architecture/🐾️pets/🔣️.json",
+      "{workspaceRoot}/🎓️teaching/🏛️architecture/🐾️pets/*/🔣️.json",
       "{workspaceRoot}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🟦️.tsx",
       "{workspaceRoot}/🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css",

PATH 🧰️framework/🛍️products/🐾️pets/📦️packages/🦀️rust/📜️script.ts HEAD 2741 INCOMING 3057
--- BASE
+++ STASH
@@ -29,4 +29,5 @@
 }
 
+
 const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript);
 

PATH 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json HEAD 33544 INCOMING 40584
--- BASE
+++ STASH
@@ -10,18 +10,28 @@
   ],
   "targets": {
-    "test-typed-snapshot-file-native": {
-      "executor": "repo:script",
-      "cache": false,
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-typed-snapshot-file-native"
-      }
-    },
-    "test-typed-snapshot-file-source": {
-      "executor": "repo:script",
-      "cache": false,
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-typed-snapshot-file-source"
+    "db-error-ownership-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts db-error-ownership-check"
+      }
+    },
+    "db-error-ownership-native-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts db-error-ownership-check --native",
+        "forwardAllArgs": true
+      }
+    },
+    "stable-identity-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts stable-identity-check",
+        "forwardAllArgs": true
       }
     },
@@ -961,4 +971,221 @@
         "command": "bun ./📜️script.ts test-crate-boundaries"
       }
+    },
+    "test-diagnostic-admission": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun 📜️script.ts test-diagnostic-admission",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      }
+    },
+    "test-owned-replay": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-owned-replay"
+      }
+    },
+    "canonical-source-identity-source": {
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-canonical-source-identity source"
+      },
+      "executor": "repo:script",
+      "cache": false
+    },
+    "canonical-source-identity-native": {
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-canonical-source-identity native"
+      },
+      "executor": "repo:script",
+      "cache": false
+    },
+    "test-retained-lane-source": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun 📜️script.ts test-retained-lane source",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      }
+    },
+    "test-retained-lane-native": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun 📜️script.ts test-retained-lane native",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      }
+    },
+    "test-original-ingress-source": {
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-ingress source"
+      },
+      "executor": "repo:script"
+    },
+    "test-original-history-input-source": {
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-history-input source"
+      },
+      "executor": "repo:script"
+    },
+    "test-original-history-input-native": {
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-history-input native"
+      },
+      "executor": "repo:script"
+    },
+    "test-original-ingress-native": {
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-ingress native"
+      },
+      "executor": "repo:script"
+    },
+    "test-original-io-retirement-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-io-retirement source"
+      }
+    },
+    "test-original-io-retirement-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-io-retirement native"
+      }
+    },
+    "semantic-source-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "command": "bun 📜️script.ts semantic-source source",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      },
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:semantic-source-source"
+        }
+      }
+    },
+    "semantic-source-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "command": "bun 📜️script.ts semantic-source native",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      },
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:semantic-source-native"
+        }
+      }
+    },
+    "member-facets-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "command": "bun 📜️script.ts member-facets source",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      },
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:member-facets-source"
+        }
+      }
+    },
+    "member-facets-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "command": "bun 📜️script.ts member-facets native",
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
+      },
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:member-facets-native"
+        }
+      }
+    },
+    "test-original-vcs-command-source": {
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:test-original-vcs-command-source"
+        }
+      },
+      "outputs": [],
+      "dependsOn": [],
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun 📜️script.ts test-original-vcs-command source"
+      },
+      "cache": false
+    },
+    "test-original-vcs-command-native": {
+      "metadata": {
+        "dashboard": {
+          "workspaceCommand": "@semio-tech/framework-os-kernel:test-original-vcs-command-native"
+        }
+      },
+      "outputs": [],
+      "dependsOn": [],
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun 📜️script.ts test-original-vcs-command native"
+      },
+      "cache": false
+    },
+    "test-typed-snapshot-file-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-typed-snapshot-file-native"
+      }
+    },
+    "test-typed-snapshot-file-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-typed-snapshot-file-source"
+      }
+    },
+    "test-original-snapshot-receiving-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-snapshot-receiving source"
+      }
+    },
+    "test-original-snapshot-receiving-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-snapshot-receiving native"
+      }
     }
   },

PATH 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts HEAD 9513 INCOMING 11793
--- BASE
+++ STASH
@@ -1,3 +1,16 @@
 #!/usr/bin/env bun
+import { StableIdentityCheckScript } from "./🎛️commands/📸️snapshot-checks/🟦️.ts";
+import { DiagnosticAdmissionCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
+import { OwnedReplayCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
+import { CanonicalSourceIdentityCheckScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
+import { RetainedLaneScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
+import { OriginalIngressScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
+import { OriginalHistoryInputScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
+import { OriginalIoRetirementScript } from "./🎛️commands/🧬️retained-clone-check/🟦️.ts";
+import { SharedSemanticSourceScript } from "../../🔨️modules/🏪️store/🏗️initialization/🏠️owners/📜️script.ts";
+import { MemberFacetsScript } from "./🎛️commands/👥️member-dialect-checks/🟦️.ts";
+import { OriginalVcsCommandScript } from "./🎛️commands/🧪️source-and-native-tests/🟦️.ts";
+import { DbErrorOwnershipCheckScript } from "./🎛️commands/🗄️database-checks/🟦️.ts";
+import { OriginalSnapshotReceivingScript } from "./🎛️commands/📸️snapshot-checks/🟦️.ts";
 /** 🦀️ `@semio-tech/framework-os-kernel` task router: parses the command and dispatches it to the check and test scripts of `🎛️commands`. */
 import { receiveScriptProcessInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -122,3 +135,17 @@
 router.register("test-crate-boundaries",CrateBoundariesSourceScript);
 
+router.register("stable-identity-check", StableIdentityCheckScript);
+router.register("test-diagnostic-admission", DiagnosticAdmissionCheckScript);
+router.register("test-owned-replay", OwnedReplayCheckScript);
+router.register("test-canonical-source-identity", CanonicalSourceIdentityCheckScript);
+router.register("test-retained-lane", RetainedLaneScript);
+router.register("test-original-ingress", OriginalIngressScript);
+router.register("test-original-history-input", OriginalHistoryInputScript);
+router.register("test-original-io-retirement", OriginalIoRetirementScript);
+router.register("semantic-source", SharedSemanticSourceScript);
+router.register("member-facets", MemberFacetsScript);
+router.register("test-original-vcs-command", OriginalVcsCommandScript);
+router.register("db-error-ownership-check", DbErrorOwnershipCheckScript);
+router.register("test-original-snapshot-receiving", OriginalSnapshotReceivingScript);
+
 await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json HEAD 3448 INCOMING 3783
--- BASE
+++ STASH
@@ -84,4 +84,12 @@
         "forwardAllArgs": true
       }
+    },
+    "test-original-board-fill-source": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-board-fill-source",
+        "forwardAllArgs": true
+      }
     }
   },

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts HEAD 6464 INCOMING 7473
--- BASE
+++ STASH
@@ -1,3 +1,4 @@
 #!/usr/bin/env bun
+import { runRepositoryTestCommand } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
@@ -60,4 +61,10 @@
 }
 
-const router = new ScriptRouter(ROOT).register("fonts", FontsScript).register("test-wgpu-world-terrain", TestWgpuWorldTerrainScript);
+class OriginalBoardFillSourceScript extends BundleScript {
+  async run(args: string[]): Promise<void> {
+    if(args.length) throw Error("test-original-board-fill-source accepts no arguments");
+    await runRepositoryTestCommand(process.execPath,["test",join(this.root,"../../🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🎟️admission/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000,signal:this.invocation.control.signal});
+  }
+}
+const router = new ScriptRouter(ROOT).register("fonts", FontsScript).register("test-wgpu-world-terrain", TestWgpuWorldTerrainScript).register("test-original-board-fill-source",OriginalBoardFillSourceScript);
 if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts HEAD 45416 INCOMING 45468
--- BASE
+++ STASH
@@ -620,4 +620,5 @@
 `;
 
+
 const router = new ScriptRouter(import.meta.dir)
   .register("build", BuildScript)

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/📋️project.json HEAD 0 INCOMING 652
--- BASE
+++ STASH
@@ -0,0 +1,25 @@
+{
+  "name": "@semio-tech/framework-history-read",
+  "tags": [
+    "framework",
+    "vcs"
+  ],
+  "targets": {
+    "verify-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read",
+        "command": "bun ./📜️script.ts verify source"
+      }
+    },
+    "verify-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read",
+        "command": "bun ./📜️script.ts verify native"
+      }
+    }
+  }
+}

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🔗️read/📜️script.ts HEAD 0 INCOMING 3481
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1} from '../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {runRepositoryExactCargoLaws} from '../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts';
+/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
+class OriginalHistoryReadScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('original-history-read requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'history:original-read:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyOriginalHistoryRead();oracle.verifyOriginalProjectedSource();oracle.verifyCapturedSnapshotSourceOracle();
+  if(args[0]==='source')return;
+  const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,manifestPath:resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml'),artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-os-kernel',target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_vcs::history_read::tests::original_history_reads_keep_forward_backing_and_refuse_each_denied_currency','os_vcs::history_read::tests::original_history_reads_return_every_cancelled_birth_and_reject_foreign_generations','os_vcs::history_read::tests::original_history_reads_refuse_unpublished_groups_and_foreign_original_issuers','os_vcs::history_read::tests::original_history_store_retirement_returns_read_sources_and_waits_for_live_leases','os_vcs::history_read::tests::original_projected_source_requires_retirement_only_from_its_actual_authority','os_store::component::captured_snapshot_source::tests::captured_snapshot_source_uses_original_registry_and_installed_opaque_issuer']}]});
+  console.log('[DEBUG] Original history native custody '+receipts[0]!.assertions+' exact laws');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',OriginalHistoryReadScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/📜️script.ts HEAD 0 INCOMING 3286
--- BASE
+++ STASH
@@ -0,0 +1,23 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from '../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {readCargoTestPolicyV1} from '../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts';
+/** 🏭️ Validates one quoted concrete semantic source and both persistent native ports. */
+export class SharedSemanticSourceScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('shared-semantic-source requires source or native');
+  const source=resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'store:semantic-source:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  await runOwnedCommand(process.execPath,['test',source],this.repoRoot,'store:semantic-source:oracle',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  if(args[0]==='source')return;
+  const packageName='semio-framework-os-kernel',manifest=resolve(this.repoRoot,'🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml'),policy=readCargoTestPolicyV1(process.env);
+  const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:this.invocation.control.remainingMilliseconds()},cwd:this.repoRoot,cargoTargetDir:policy.targetDirectory,manifestPaths:{[packageName]:manifest},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:packageName,target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_store::component::tests::shared_semantic_source_tests::original_shared_semantic_source_constructs_one_paired_issuer_and_closes_all_actual_frontiers']}]});
+  console.log('[DEBUG] Shared semantic source exact native assertions='+receipts[0]!.assertions);
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',SharedSemanticSourceScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/📋️project.json HEAD 0 INCOMING 733
--- BASE
+++ STASH
@@ -0,0 +1,25 @@
+{
+  "name": "@semio-tech/framework-store-native-semantic",
+  "tags": [
+    "framework",
+    "store"
+  ],
+  "targets": {
+    "verify-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native",
+        "command": "bun ./📜️script.ts verify source"
+      }
+    },
+    "verify-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native",
+        "command": "bun ./📜️script.ts verify native"
+      }
+    }
+  }
+}

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/📜️script.ts HEAD 0 INCOMING 3461
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1,runExactCargoLaws} from '../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {readCargoTestPolicyV1} from '../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts';
+/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
+class NativeSemanticPreparationScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('native-semantic-preparation requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'store:native-semantic:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyNativeSemanticPreparation();
+  if(args[0]==='source')return;
+  const packageName="semio-framework-os-kernel",manifest=resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml"),p=readCargoTestPolicyV1(process.env);const receipts=await runExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,cargoTargetDir:p.targetDirectory,manifestPaths:{[packageName]:manifest},artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-os-kernel',target:{kind:'lib',name:'semio_framework_os_kernel'},laws:['os_store::component::snapshot_read_registry_handle::birth::tests::original_registry_birth_keeps_four_kib_quantum_fifo_and_exact_cumulative_custody','os_store::component::replay_preparation::native::tests::original_semantic_factory_matches_neutral_outputs_and_exact_physical_turns','os_store::component::derived_operation::tests::original_derived_semantic_task_admits_copy_and_capacity_as_independent_currencies']}]});
+  console.log('[DEBUG] Original semantic native custody '+receipts[0]!.assertions+' exact laws');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',NativeSemanticPreparationScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json HEAD 12113 INCOMING 13023
--- BASE
+++ STASH
@@ -17,4 +17,19 @@
   },
   "targets": {
+    "viewport-presence-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts viewport-presence-check"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "viewport-presence-check"
+          ]
+        }
+      }
+    },
     "paint2d-editing-check": {
       "executor": "repo:script",
@@ -237,4 +252,20 @@
       },
       "outputs": []
+    },
+    "operation-diagnostic-host-check": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript",
+        "command": "bun ./📜️script.ts operation-diagnostic-host-check"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "operation-diagnostic-host-check"
+          ]
+        }
+      }
     }
   },

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts HEAD 75420 INCOMING 76620
--- BASE
+++ STASH
@@ -36,4 +36,20 @@
 }
 
+/** 👥️ Verifies canonical viewport presence on actual mounted generic Canvas2d and World3d hosts. */
+class ViewportPresenceCheckScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.length) throw new Error("viewport-presence-check accepts no arguments");
+    process.env.SEMIO_TEST_LEVEL = "long";
+    await runVitest(this.root, ["../../../../🧱️elements/👕️canvas-presence/🧪️tests/✏️text-carets/🟦️.tsx", "../../../../🧱️elements/👕️canvas-presence/🧪️tests/🔬️unit/🟦️.ts", "../../../../🧱️elements/📐️Canvas2dHost/🧪️tests/👕️peer-presence/🟦️.tsx", "../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
+  }
+}
+/** 📬️ Executes original diagnostic command identity, replay, progress and before-ACK terminal laws. */
+class OperationDiagnosticHostCheckScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.length) throw Error("operation-diagnostic-host-check accepts no arguments");
+    process.env.SEMIO_TEST_LEVEL = "long";
+    await runVitest(this.root, ["../../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx", "--silent=false", "--reporter=verbose", "--testNamePattern=typed-operation diagnostic|routes a foreign typed-operation|bounds parked typed-operation"], "../../🧪️tests/🎚️config/🟦️.ts");
+  }
+}
 class TestScript extends BundleScript {
   async run(segments: string[]): Promise<void> {
@@ -582,5 +598,5 @@
     mirrorDomain: string;
     cases: readonly Case[];
-    wiring: { file: string; projection: string; callSites: readonly string[]; lockedFromSelectedIds: string };
+    wiring: { projection: { file: string; source: string }; callSites: readonly { file: string; source: string }[]; lockedFromSelectedIds: { file: string; source: string } };
   };
   let checks = 0;
@@ -607,8 +623,8 @@
     check(new Set(entry.selectedIds).size === entry.selectedIds.length, `${entry.id}: selectedIds must name every id at most once`);
   }
-  const source = readFileSync(join(repoRoot, fixture.wiring.file), "utf8");
-  check(source.includes(fixture.wiring.projection), "the ONE shared leftover-selection projection must still exist");
-  for (const site of fixture.wiring.callSites) check(source.includes(site), `every flatten site must go through the shared projection — missing ${site.trim()}`);
-  check(source.includes(fixture.wiring.lockedFromSelectedIds), "`locked` must be derived from the deduplicated selectedIds, never from a second flatten");
+  const anchored = (anchor: { readonly file: string; readonly source: string }): boolean => readFileSync(join(repoRoot, anchor.file), "utf8").includes(anchor.source);
+  check(anchored(fixture.wiring.projection), "the ONE shared leftover-selection projection must still exist");
+  for (const site of fixture.wiring.callSites) check(anchored(site), `every flatten site must go through the shared projection — missing ${site.source}`);
+  check(anchored(fixture.wiring.lockedFromSelectedIds), "`locked` must be derived from the deduplicated selectedIds, never from a second flatten");
   return checks;
 }
@@ -637,5 +653,5 @@
     wiring: {
       codec: { file: string; toWire: string; fromWire: string };
-      domainPath: { file: string; decoder: string; worldPath: string };
+      domainPath: { file: string; decoder: string; worldFile: string; worldPath: string };
       host: { file: string; selectionArgs: string; componentPick: string; deletedBuilders: readonly string[] };
     };
@@ -713,5 +729,5 @@
   const plugin = readFileSync(join(repoRoot, fixture.wiring.domainPath.file), "utf8");
   check(plugin.includes(fixture.wiring.domainPath.decoder), "the domain path must decode through the shared codec, never through its own match arms");
-  check(plugin.includes(fixture.wiring.domainPath.worldPath), "the non-domain world path must take a decoded MergeMode, so a private word is unrepresentable");
+  check(readFileSync(join(repoRoot, fixture.wiring.domainPath.worldFile), "utf8").includes(fixture.wiring.domainPath.worldPath), "the non-domain world path must take a decoded MergeMode, so a private word is unrepresentable");
   const host = readFileSync(join(repoRoot, fixture.wiring.host.file), "utf8");
   check(host.includes(fixture.wiring.host.selectionArgs), "the host's selection args builder must take a typed MergeMode");
@@ -767,9 +783,9 @@
   async run(segments: string[]): Promise<void> {
     if (segments.length !== 0) throw new Error("world3d-interaction-check accepts no arguments");
+    process.env.SEMIO_TEST_LEVEL = "long";
+    await runVitest(this.root, ["../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "../../../../🧪️tests/🎯️world3d-pick-bounds/🟦️.ts", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
     console.log(`world3d-pointer-gesture-oracle: checks=${world3dPointerGestureOracle(this.repoRoot)} clean`);
     console.log(`interaction-selection-set-oracle: checks=${interactionSelectionSetOracle(this.repoRoot)} clean`);
     console.log(`selection-merge-vocabulary-oracle: checks=${selectionMergeVocabularyOracle(this.repoRoot)} clean`);
-    process.env.SEMIO_TEST_LEVEL = "long";
-    await runVitest(this.root, ["../../../../🧪️tests/🖱️world3d-interaction/🟦️.tsx", "--silent=false", "--reporter=verbose"], "../../🧪️tests/🎚️config/🟦️.ts");
   }
 }
@@ -959,5 +975,7 @@
 
 const router = new ScriptRouter(fileURLToPath(new URL(".", import.meta.url)))
-  .register("test", TestScript)
+  .register("operation-diagnostic-host-check", OperationDiagnosticHostCheckScript)
+    .register("test", TestScript)
+  .register("viewport-presence-check", ViewportPresenceCheckScript)
   .register("lint", LintScript)
   .register("typecheck", TypecheckScript)

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts HEAD 13550 INCOMING 13657
--- BASE
+++ STASH
@@ -13,4 +13,6 @@
 import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
 import { once } from "node:events";
+import nativeLaunchDeclaration from "./🎟️launch/🔣️.json";
+import { nativeLaunchEnvironmentV1, readNativeLaunchDeclarationV1 } from "./🎟️launch/🟦️.ts";
 import { existsSync, mkdirSync, readFileSync } from "node:fs";
 import type { Server } from "node:http";
@@ -84,7 +86,6 @@
     const row = catalog.find((entry: { variant: string }) => entry.variant === variant);
     if (!row) throw new Error(`Unknown native playground: ${variant}`);
-    const controller = new AbortController(), cancel = () => controller.abort();
-    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
-    try {
+    const signal = this.invocation.control.signal;
+    {
       const binary=playgroundNativeHostArtifactV1(repo,row.nativeHost,profile)??nativeRendererBinary(resolve(import.meta.dir,"../📦️packages/🦀️rust"),profile);
       if(!existsSync(binary))throw new Error(`Missing Nx native host artifact: ${binary}`);
@@ -92,6 +93,6 @@
       // occurrence; the playground row then supplies the per-server defaults (`app`, `brand`) the wgpu
       // serve injects as `<meta name="semio-*">` for the browser.
-      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_RUNTIME_ASSETS: repo }, repo,this.invocation,createRequestedIssuedResourceControlV1(runtimeControl,0,controller.signal),row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
-    } finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
+      await runNativeSession(binary, ["--plugin", variant, ...axes, ...(row.app ? ["--app", row.app] : []), ...(row.brand ? ["--brand", row.brand] : []), ...(args.includes("--smoke") ? ["--smoke"] : [])], nativeLaunchEnvironmentV1(this.invocation, { ...process.env, SEMIO_PLUGIN: variant, SEMIO_RENDERER: "wgpu", SEMIO_BUILD_MODE: profile === "release" ? "release" : "prototype", SEMIO_PLUGIN_MODULES: runtime, SEMIO_RUNTIME_ASSETS: repo }, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()), repo,this.invocation,createRequestedIssuedResourceControlV1(runtimeControl,0,signal),row.assets?.length ? () => createAssetHttpServerV1(repo, 0, row.assets, PLAYGROUND_ASSET_PROVIDERS_V1) : undefined);
+    }
     });
     });
@@ -124,8 +125,6 @@
     for (const flag of ["--scale", "--scale-wasm", "--report"]) if (args.filter(value => value === flag).length !== 1) throw new Error(`Native scale requires exactly one ${flag}`);
     const report=args[args.indexOf("--report")+1];if(!report)throw Error("Finite scale report required");mkdirSync(dirname(resolve(this.repoRoot,report)),{recursive:true});
-    const controller = new AbortController(), cancel = () => controller.abort();
-    process.once("SIGINT", cancel); process.once("SIGTERM", cancel);
-    try { await runNativeBinary(nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile), args,process.env,this.repoRoot,this.invocation,createRequestedIssuedResourceControlV1(phaseControl,0,controller.signal)); }
-    finally { process.removeListener("SIGINT", cancel); process.removeListener("SIGTERM", cancel); }
+    const signal = this.invocation.control.signal;
+    await runNativeBinary(nativeRendererBinary(resolve(import.meta.dir, "../📦️packages/🦀️rust"), profile), args, nativeLaunchEnvironmentV1(this.invocation, process.env, readNativeLaunchDeclarationV1(nativeLaunchDeclaration), Date.now()),this.repoRoot,this.invocation,createRequestedIssuedResourceControlV1(phaseControl,0,signal));
   }
 }

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json HEAD 83756 INCOMING 58401
--- BASE
+++ STASH
@@ -377,4 +377,110 @@
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript",
         "forwardAllArgs": true
+      },
+      "metadata": {
+        "semio": {
+          "dashboard": {
+            "verb": "test",
+            "parameters": [
+              {
+                "id": "mode",
+                "kind": "choice",
+                "default": "test",
+                "values": [
+                  {
+                    "id": "test",
+                    "args": [
+                      "test"
+                    ]
+                  },
+                  {
+                    "id": "react",
+                    "args": [
+                      "run",
+                      "react"
+                    ]
+                  },
+                  {
+                    "id": "wgpu",
+                    "args": [
+                      "run",
+                      "wgpu"
+                    ]
+                  }
+                ],
+                "title": {
+                  "en": "Run mode",
+                  "de": "Ausführungsmodus"
+                },
+                "description": {
+                  "en": "Choose tests or a renderer session.",
+                  "de": "Tests oder eine Renderer-Sitzung auswählen."
+                }
+              },
+              {
+                "id": "serve",
+                "kind": "text",
+                "valuePositional": true,
+                "title": {
+                  "en": "Server address",
+                  "de": "Serveradresse"
+                },
+                "description": {
+                  "en": "Address of the server to verify.",
+                  "de": "Adresse des zu prüfenden Servers."
+                }
+              },
+              {
+                "id": "output",
+                "kind": "text",
+                "valueFlag": "--output",
+                "title": {
+                  "en": "Output directory",
+                  "de": "Ausgabeverzeichnis"
+                },
+                "description": {
+                  "en": "Directory for generated verification artifacts.",
+                  "de": "Verzeichnis für erzeugte Prüfartefakte."
+                }
+              },
+              {
+                "id": "suite",
+                "kind": "choice",
+                "values": [
+                  {
+                    "id": "docking",
+                    "args": [
+                      "--suite",
+                      "docking"
+                    ]
+                  },
+                  {
+                    "id": "widgets",
+                    "args": [
+                      "--suite",
+                      "widgets"
+                    ]
+                  }
+                ],
+                "title": {
+                  "en": "Test suite",
+                  "de": "Testsuite"
+                },
+                "description": {
+                  "en": "Choose docking or widget checks.",
+                  "de": "Andock- oder Widget-Prüfungen auswählen."
+                }
+              }
+            ],
+            "title": {
+              "en": "Verify browser docking",
+              "de": "Browser-Andocken prüfen"
+            },
+            "description": {
+              "en": "Choose docking tests or a renderer session.",
+              "de": "Andocktests oder eine Renderer-Sitzung auswählen."
+            }
+          }
+        }
       }
     },
@@ -387,4 +493,78 @@
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript",
         "forwardAllArgs": true
+      },
+      "metadata": {
+        "semio": {
+          "dashboard": {
+            "verb": "test",
+            "parameters": [
+              {
+                "id": "serve",
+                "kind": "text",
+                "required": true,
+                "valueFlag": "--serve",
+                "title": {
+                  "en": "Server address",
+                  "de": "Serveradresse"
+                },
+                "description": {
+                  "en": "Address of the server to verify.",
+                  "de": "Adresse des zu prüfenden Servers."
+                }
+              },
+              {
+                "id": "locale",
+                "kind": "choice",
+                "required": true,
+                "values": [
+                  {
+                    "id": "en",
+                    "args": [
+                      "--locale",
+                      "en"
+                    ]
+                  },
+                  {
+                    "id": "de",
+                    "args": [
+                      "--locale",
+                      "de"
+                    ]
+                  }
+                ],
+                "title": {
+                  "en": "Language",
+                  "de": "Sprache"
+                },
+                "description": {
+                  "en": "Choose the language explicitly.",
+                  "de": "Sprache ausdrücklich auswählen."
+                }
+              },
+              {
+                "id": "output",
+                "kind": "text",
+                "required": true,
+                "valueFlag": "--output",
+                "title": {
+                  "en": "Output directory",
+                  "de": "Ausgabeverzeichnis"
+                },
+                "description": {
+                  "en": "Directory for generated verification artifacts.",
+                  "de": "Verzeichnis für erzeugte Prüfartefakte."
+                }
+              }
+            ],
+            "title": {
+              "en": "Verify browser media",
+              "de": "Browser-Medien prüfen"
+            },
+            "description": {
+              "en": "Run media checks with an explicit server and language.",
+              "de": "Medienprüfungen mit explizitem Server und Sprache ausführen."
+            }
+          }
+        }
       }
     },
@@ -758,4 +938,49 @@
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript",
         "forwardAllArgs": true
+      },
+      "metadata": {
+        "semio": {
+          "dashboard": {
+            "verb": "test",
+            "parameters": [
+              {
+                "id": "configuration",
+                "kind": "text",
+                "required": true,
+                "valueFlag": "--configuration",
+                "title": {
+                  "en": "Configuration file",
+                  "de": "Konfigurationsdatei"
+                },
+                "description": {
+                  "en": "Path to the browser verification configuration.",
+                  "de": "Pfad zur Browser-Prüfkonfiguration."
+                }
+              },
+              {
+                "id": "output",
+                "kind": "text",
+                "required": true,
+                "valueFlag": "--output",
+                "title": {
+                  "en": "Output directory",
+                  "de": "Ausgabeverzeichnis"
+                },
+                "description": {
+                  "en": "Directory for generated verification artifacts.",
+                  "de": "Verzeichnis für erzeugte Prüfartefakte."
+                }
+              }
+            ],
+            "title": {
+              "en": "Verify embedded browser",
+              "de": "Eingebetteten Browser prüfen"
+            },
+            "description": {
+              "en": "Run embedded browser checks with a selected configuration.",
+              "de": "Prüfungen des eingebetteten Browsers mit ausgewählter Konfiguration ausführen."
+            }
+          }
+        }
       }
     },
@@ -806,4 +1031,12 @@
       "options": {
         "command": "bun ./📜️script.ts test-original-frame-source",
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"
+      },
+      "cache": false
+    },
+    "test-original-io-invocation-native": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun ./📜️script.ts test-original-io-invocation-native",
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"
       },

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts HEAD 38336 INCOMING 41301
--- BASE
+++ STASH
@@ -112,8 +112,13 @@
     if (args.length) throw Error("Expected test-original-frame-source");
     const { runOwnedCommand } = await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
-    const files = ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🧪️tests/🟦️.ts"].map(file=>resolve(this.repoRoot,file));
-    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...files], this.repoRoot, "wgpu:original-frame:types", this.childBudget(), { signal: this.invocation.control.signal });
+    const files = ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🎟️grant/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🎟️launch/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/📬️outcome/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📮️runtime-mailbox-core/🎟️apply/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/🎟️original/🧪️tests/🟦️.ts","🧰️framework/🔨️modules/🧵️job/📬️outcome/📮️return/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎟️native-caller/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️invocation/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️source/🧪️tests/🟦️.ts","🧰️framework/🔨️modules/🧵️job/📬️outcome/📮️return/📦️bytes/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎟️task-ingress/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🎟️source/🧪️tests/🟦️.ts","🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🐣️create-app/🧩️compile/🧪️tests/🟦️.ts"].map(file=>resolve(this.repoRoot,file));
+    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", this.root), "--noEmit", "--strict", "--noUncheckedIndexedAccess", "--skipLibCheck", "--allowImportingTsExtensions", "--resolveJsonModule", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...files], this.repoRoot, "wgpu:original-frame:types", this.childBudget(), { signal: this.invocation.control.signal });
     await runOwnedCommand(process.execPath, ["test", ...files], this.repoRoot, "wgpu:original-frame:source", this.childBudget(), { signal: this.invocation.control.signal });
   }
+}
+
+/** 📮️ Executes the mounted native invocation source, streaming and exact page-return lifecycle laws. */
+class OriginalIoInvocationNativeScript extends BundleScript {
+ async run(args:string[]):Promise<void>{if(args.length)throw Error("Expected test-original-io-invocation-native");await runRepositoryCargoTests([crateName],this.repoRoot,this.invocation,["--lib","-E","test(renderer_original_io_) | test(renderer_source_io_) | test(renderer_task_ingress_) | test(renderer_create_app_source_) | test(renderer_compiler_task_) | test(a_contended_renderer_io_poll_wakes_itself_until_the_exact_session_can_register) | test(mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact) | test(headless_entrypoint_drives_retained_io_and_matches_the_json_oracle) | test(native_app_worker_completes_io_without_a_presentation_host) | test(native_kernel_worker_completes_io_without_a_presentation_host) | test(native_component_reader_admits_local_dev_artifacts_above_catalog_network_bound)","--","--nocapture"]);}
 }
 
@@ -504,4 +509,5 @@
   .register("test-socket-snapshot-sqlite",SocketSnapshotSqliteScript)
   .register("test-original-frame-source", OriginalFrameSourceScript)
+  .register("test-original-io-invocation-native",OriginalIoInvocationNativeScript)
   .register("test-original-browser-output-source",OriginalBrowserOutputSourceScript)
   .register("test-original-browser-output-native",OriginalBrowserOutputNativeScript)

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts HEAD 78443 INCOMING 77835
--- BASE
+++ STASH
@@ -200,5 +200,5 @@
 }
 
-type BrowserActorCodegenManifest = Readonly<{ version: "1.34.0"; runtime: "bun@1.3.14"; importInterfaces: readonly string[]; files: readonly string[] }>;
+type BrowserActorCodegenManifest = Readonly<{ version: "1.34.0"; runtime: "bun@1.4.2"; importInterfaces: readonly string[]; files: readonly string[] }>;
 
 /** 🛂️ Admits exact canonical generated-file and interface names independently of the codegen subprocess. */
@@ -207,5 +207,5 @@
   if (!value || typeof value !== "object" || Array.isArray(value) || Object.keys(value).sort().join(",") !== "files,importInterfaces,runtime,version") return denied();
   const record = value as Record<string, unknown>;
-  if (record.version !== "1.34.0" || record.runtime !== "bun@1.3.14") return denied();
+  if (record.version !== "1.34.0" || record.runtime !== "bun@1.4.2") return denied();
   const strings = (value: unknown, maximum: number, valid: (entry: string) => boolean): readonly string[] => {
     if (!Array.isArray(value) || value.length > maximum || !value.every((entry, index) => typeof entry === "string" && valid(entry) && (index === 0 || value[index - 1] < entry))) return denied();
@@ -215,5 +215,5 @@
   const files = strings(record.files, 65, name => name === "browser-actor.js" || /^browser-actor\.core\d*\.wasm$/.test(name));
   if (files.length < 2 || files.filter(name => name === "browser-actor.js").length !== 1) return denied();
-  return Object.freeze({ version: "1.34.0", runtime: "bun@1.3.14", importInterfaces, files });
+  return Object.freeze({ version: "1.34.0", runtime: "bun@1.4.2", importInterfaces, files });
 }
 
@@ -390,5 +390,5 @@
 
 function observedBrowserCodegenPolicy(executable:{sha256:string;byteLength:number},compiler:Awaited<ReturnType<typeof buildBrowserCodegenModule>>,inputs:Omit<ReturnType<typeof captureBrowserCodegenPolicyInputs>,"physicalInputs">){
-  return sealBrowserCodegenPolicy({schema:"semio.os.browser-codegen-policy.v1",revision:1,runtime:{kind:"bun",version:"1.3.14",executable:{sha256:executable.sha256,byteLength:executable.byteLength}},compiler:{module:{sha256:compiler.sha256,byteLength:compiler.byteLength},inputs:compiler.inputs,cores:compiler.cores},...inputs,options:{jco:"1.34.0",entrypoint:"@bytecodealliance/jco/component",target:"browser",format:"esm",name:"browser-actor",instantiation:"async",asyncMode:"jspi",nodejsCompat:false,base64Cutoff:0,importInterfaces:browserActorInterfaces,asyncImports:browserActorAsyncImports}});
+  return sealBrowserCodegenPolicy({schema:"semio.os.browser-codegen-policy.v1",revision:1,runtime:{kind:"bun",version:"1.4.2",executable:{sha256:executable.sha256,byteLength:executable.byteLength}},compiler:{module:{sha256:compiler.sha256,byteLength:compiler.byteLength},inputs:compiler.inputs,cores:compiler.cores},...inputs,options:{jco:"1.34.0",entrypoint:"@bytecodealliance/jco/component",target:"browser",format:"esm",name:"browser-actor",instantiation:"async",asyncMode:"jspi",nodejsCompat:false,base64Cutoff:0,importInterfaces:browserActorInterfaces,asyncImports:browserActorAsyncImports}});
 }
 
@@ -400,5 +400,5 @@
   const executable=exactExecutableFingerprint(realpathSync(process.execPath),{cancelled:control.cancelled,progress:(completed,total)=>control.progress?.("policy",completed,total)});if(canonicalJson(executable)!==canonicalJson(observation.runtime))denied();
   const originalCompiler=readStableBuildFile(observation.compiler.path,browserActorMaximumBytes,{remaining:browserActorMaximumBytes},check);if(originalCompiler.byteLength!==observation.compiler.byteLength||createHash("sha256").update(originalCompiler).digest("hex")!==observation.compiler.sha256)denied();
-  const managerPath=join(browserActorRepoRoot,"package.json"),manager=readStableBuildFile(managerPath,1024*1024,{remaining:1024*1024},check);if(Bun.version!=="1.3.14"||JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(manager)).packageManager!=="bun@1.3.14")denied();
+  const managerPath=join(browserActorRepoRoot,"package.json"),manager=readStableBuildFile(managerPath,1024*1024,{remaining:1024*1024},check);if(Bun.version!=="1.4.2"||JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(manager)).packageManager!=="bun@1.4.2")denied();
   const runtime=captureBrowserActorRuntime(import.meta.dir,check),{physicalInputs,...policyInputs}=captureBrowserCodegenPolicyInputs(runtime,check),scratchRoot=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(scratchRoot,{recursive:true});const scratch=mkdtempSync(join(scratchRoot,"browser-producer-verification-"));
   try{control.progress?.("closure",0,1);const compiler=await buildBrowserCodegenModule(scratch,check,invocation,phase),current=observedBrowserCodegenPolicy(executable,compiler,policyInputs),inputs=[{logicalPath:"toolchain/package.json",path:managerPath,sha256:createHash("sha256").update(manager).digest("hex"),byteLength:manager.byteLength},...physicalInputs,...compiler.physicalInputs];
@@ -426,5 +426,5 @@
   try {
     const packageManagerPath=join(browserActorRepoRoot,"package.json"),packageManagerBytes=readStableBuildFile(packageManagerPath,1024*1024,{remaining:1024*1024},check);
-    if (typeof Bun === "undefined" || Bun.version !== "1.3.14" || JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(packageManagerBytes)).packageManager !== "bun@1.3.14") throw new Error("browser actor artifact: unqualified build runtime");
+    if (typeof Bun === "undefined" || Bun.version !== "1.4.2" || JSON.parse(new TextDecoder("utf-8",{fatal:true}).decode(packageManagerBytes)).packageManager !== "bun@1.4.2") throw new Error("browser actor artifact: unqualified build runtime");
     if (!(component instanceof Uint8Array) || component.byteLength < 8 || component.byteLength > browserActorMaximumBytes || ![0, 97, 115, 109, 13, 0, 1, 0].every((value, index) => component[index] === value)) throw new Error("browser actor artifact: component header or byte bound");
     snapshot = Uint8Array.from(component);
@@ -459,5 +459,5 @@
       import { createHash } from "node:crypto";
       import { join } from "node:path";
-      assert.equal(globalThis.Bun?.version, "1.3.14", "browser actor artifact: unqualified codegen runtime");
+      assert.equal(globalThis.Bun?.version, "1.4.2", "browser actor artifact: unqualified codegen runtime");
       assert(["NODE_OPTIONS", "NODE_PATH", "BUN_OPTIONS", "BUN_PRELOAD", "PATH"].every(name => process.env[name] === undefined), "browser actor artifact: ambient codegen environment");
       const [componentPath, outputRoot, importsJson, asyncJson, maximum, compilerPath, compilerHash, compilerLength, componentHash, componentLength] = process.argv.slice(1);

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📋️project.json HEAD 19872 INCOMING 29924
--- BASE
+++ STASH
@@ -10,126 +10,70 @@
   ],
   "targets": {
-    "test-original-fixture-close-source": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-fixture-close-source"
-      }
-    },
-    "test-original-command-receiving-source": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-command-receiving-source"
-      }
-    },
-    "test-original-command-receiving-native": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-command-receiving-native"
-      }
-    },
-    "test-original-pending-worker-source": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-pending-worker-source"
-      },
-      "metadata": {
-        "description": "Original pending worker source and Store-issued command read donation"
-      }
-    },
-    "test-original-pending-worker-native": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-pending-worker-native"
-      },
-      "metadata": {
-        "description": "Original pending worker source and Store-issued command read donation"
-      }
-    },
-    "test-original-lifecycle-source": {
-      "executor": "repo:script",
-      "options": {
-        "command": "bun ./📜️script.ts test-original-lifecycle-source",
-        "cwd": "{projectRoot}"
-      }
-    },
-    "test-original-lifecycle-native": {
-      "executor": "repo:script",
-      "options": {
-        "command": "bun ./📜️script.ts test-original-lifecycle-native",
-        "cwd": "{projectRoot}"
-      }
-    },
-    "test-original-job-native": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "inputs": [
-        "{projectRoot}/📜️script.ts",
-        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/**/*",
-        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/**/*",
-        "{workspaceRoot}/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/**/*"
-      ],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-original-job-native"
-      }
-    },
-    "test-media-turn-source": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "inputs": [
-        "{projectRoot}/📜️script.ts",
-        "{projectRoot}/Cargo.toml",
-        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*"
-      ],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-media-turn-source"
-      },
-      "metadata": {
-        "semio": {
-          "execution": {
-            "version": 1,
-            "kind": "source"
-          }
-        }
-      }
-    },
-    "test-media-turn-native": {
-      "executor": "repo:script",
-      "cache": false,
-      "dependsOn": [],
-      "inputs": [
-        "{projectRoot}/📜️script.ts",
-        "{projectRoot}/Cargo.toml",
-        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*"
-      ],
-      "outputs": [],
-      "options": {
-        "cwd": "{projectRoot}",
-        "command": "bun ./📜️script.ts test-media-turn-native"
+    "test-original-window-catalog-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-window-catalog source"
+      }
+    },
+    "test-original-window-catalog-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-original-window-catalog native"
+      }
+    },
+    "test-original-tick-payload-sources-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-tick-payload-sources source"
+      }
+    },
+    "test-original-tick-payload-sources-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-tick-payload-sources native"
+      }
+    },
+    "test-completion-rejection-close-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-completion-rejection-close source"
+      }
+    },
+    "test-completion-rejection-close-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-completion-rejection-close native"
+      }
+    },
+    "test-snapshot-builder-ownership-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-snapshot-builder-ownership source"
+      }
+    },
+    "test-snapshot-builder-ownership-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-snapshot-builder-ownership native"
       }
     },
@@ -353,5 +297,8 @@
         "cwd": "{projectRoot}",
         "forwardAllArgs": true
-      }
+      },
+      "inputs": [
+        "{workspaceRoot}/🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/**"
+      ]
     },
     "test-codec-send-source": {
@@ -471,4 +418,230 @@
       }
     },
+    "test-remote-ingest-prefix-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-remote-ingest-prefix source"
+      }
+    },
+    "test-remote-ingest-prefix-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-remote-ingest-prefix native"
+      }
+    },
+    "test-camera-presence-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-camera-presence source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-camera-presence",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-camera-presence-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-camera-presence native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-camera-presence",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-snapshot-read-preparation-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-snapshot-read-preparation source"
+      }
+    },
+    "test-snapshot-read-preparation-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-snapshot-read-preparation native"
+      }
+    },
+    "test-result-publication-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-result-publication source"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-result-publication",
+            "source"
+          ]
+        }
+      }
+    },
+    "test-result-publication-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-result-publication native"
+      },
+      "metadata": {
+        "semio": {
+          "workspaceCommand": [
+            "test-result-publication",
+            "native"
+          ]
+        }
+      }
+    },
+    "test-original-fixture-close-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-fixture-close-source"
+      }
+    },
+    "test-original-command-receiving-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-command-receiving-source"
+      }
+    },
+    "test-original-command-receiving-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-command-receiving-native"
+      }
+    },
+    "test-original-pending-worker-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-pending-worker-source"
+      },
+      "metadata": {
+        "description": "Original pending worker source and Store-issued command read donation"
+      }
+    },
+    "test-original-pending-worker-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-pending-worker-native"
+      },
+      "metadata": {
+        "description": "Original pending worker source and Store-issued command read donation"
+      }
+    },
+    "test-original-lifecycle-source": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun ./📜️script.ts test-original-lifecycle-source",
+        "cwd": "{projectRoot}"
+      }
+    },
+    "test-original-lifecycle-native": {
+      "executor": "repo:script",
+      "options": {
+        "command": "bun ./📜️script.ts test-original-lifecycle-native",
+        "cwd": "{projectRoot}"
+      }
+    },
+    "test-original-job-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "{projectRoot}/📜️script.ts",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️extension/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/💼️job/**/*"
+      ],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-original-job-native"
+      }
+    },
+    "test-media-turn-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "{projectRoot}/📜️script.ts",
+        "{projectRoot}/Cargo.toml",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*"
+      ],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-media-turn-source"
+      },
+      "metadata": {
+        "semio": {
+          "execution": {
+            "version": 1,
+            "kind": "source"
+          }
+        }
+      }
+    },
+    "test-media-turn-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "dependsOn": [],
+      "inputs": [
+        "{projectRoot}/📜️script.ts",
+        "{projectRoot}/Cargo.toml",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*"
+      ],
+      "outputs": [],
+      "options": {
+        "cwd": "{projectRoot}",
+        "command": "bun ./📜️script.ts test-media-turn-native"
+      }
+    },
     "test-original-failure-debit-source": {
       "executor": "repo:script",
@@ -477,5 +650,11 @@
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
         "command": "bun ./📜️script.ts test-original-failure-debit-source"
-      }
+      },
+      "inputs": [
+        "{projectRoot}/📜️script.ts",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs"
+      ]
     },
     "test-original-failure-debit-native": {
@@ -485,5 +664,11 @@
         "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
         "command": "bun ./📜️script.ts test-original-failure-debit-native"
-      }
+      },
+      "inputs": [
+        "{projectRoot}/📜️script.ts",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/🦀️.rs"
+      ]
     },
     "test-original-paged-output-source": {
@@ -606,4 +791,69 @@
         }
       }
+    },
+    "test-original-producer-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-original-producer-source",
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"
+      },
+      "inputs": [
+        "default",
+        "^default",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🏭️producer/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️request/♻️retirement/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🎯️action-bus/🚪️admission/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🏛️reserved/⚙️worker/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🏭️construction/**/*",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🏭️producer/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/**"
+      ]
+    },
+    "test-original-document-receiving-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-original-document-receiving source",
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"
+      },
+      "inputs": [
+        "default",
+        "^default",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/💡️inference/🫴️receiving/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🚪️io/🔤️base64/**/*"
+      ]
+    },
+    "test-original-document-receiving-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "command": "bun ./📜️script.ts test-original-document-receiving native",
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust"
+      },
+      "inputs": [
+        "default",
+        "^default",
+        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/💡️inference/🫴️receiving/**/*",
+        "{workspaceRoot}/🧰️framework/🔨️modules/🚪️io/🔤️base64/**/*"
+      ]
+    },
+    "test-reserved-registry-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-reserved-registry source",
+        "forwardAllArgs": false
+      }
+    },
+    "test-reserved-registry-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-reserved-registry native",
+        "forwardAllArgs": false
+      }
     }
   }

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts HEAD 67387 INCOMING 91038
--- BASE
+++ STASH
@@ -1,4 +1,8 @@
 #!/usr/bin/env bun
 import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../../../🔨️modules/🏃️process/🏗️build/🎟️capacity/🟦️.ts";
+import {runBudgetedTestCommand} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
+import { runRepositoryCommand } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
+import { readCargoTestPolicyV1 } from "../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
+
 import { configuredExactCargoLawPolicyV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
@@ -15,5 +19,7 @@
 
 
-import { resolve } from "node:path";
+import { join, resolve } from "node:path";
+import { readFileSync } from "node:fs";
+import { semioSchemaAjvV1 } from "../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";
 
 
@@ -64,5 +70,6 @@
     const { composedChildHistoryOracle } = await import("../../🧪️tests/🧪️composed-child-history/🟦️.ts");
     const { checkpointActorOracle } = await import("../../⚛️reactor/📸️checkpoint/🧪️tests/🔬️unit/🟦️.ts");
-    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🎞️media/⚠️turn/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧪️tests/🧪️history-edit-acceptance/🧪️tests/📥️action/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
+    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🏭️factory/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../👷️job/🏛️reserved/⚙️worker/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/♻️retirement/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/🔒️command/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/🎞️media/🏭️construction/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🔄️full-operation/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
+    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🎞️media/⚠️turn/🧪️tests/🟦️.ts"),resolve(this.root,"../../🖥️host/⚡️effects/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧪️tests/🧪️history-edit-acceptance/🧪️tests/📥️action/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
     console.log(`[DEBUG] plugin-runner-oracle cases=${await pluginTestRunnerSelfTests()}`);
     console.log(`artifact-admission-oracle cases=${artifactAdmissionOracle(this.repoRoot)} firstParty=39`);
@@ -141,5 +148,5 @@
   const {runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
   const laws=["⚛️reactor/🚪️lifetime/🏗️opening/🧪️tests/🟦️.ts","⚛️reactor/🔄️turn/📥️receiving/🧪️tests/🟦️.ts","🪟️window/🫧️transient/🔁️document-replacement/🧪️tests/🟦️.ts"].map(path=>resolve(this.root,"../..",path));
-  const remaining=()=>{const value=this.invocation.control.remainingMilliseconds();if(value===null||value<=0)throw Error("Original receiving verification requires its finite actual caller");return value};
+  const remaining=()=>{const value=scriptInvocationBudget(this.invocation,0);if(value===null||value<=0)throw Error("Original receiving verification requires its finite actual caller");return value};
   await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.repoRoot),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun,node",...laws],this.repoRoot,"plugin:original-receiving:strict",remaining(),{signal:this.invocation.control.signal});
   await runOwnedCommand(process.execPath,["test",...laws],this.repoRoot,"plugin:original-receiving:source",remaining(),{signal:this.invocation.control.signal});
@@ -430,5 +437,5 @@
   }
   const {runOwnedCommand}=await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts");
-  const remaining=()=>{const milliseconds=this.invocation.control.remainingMilliseconds();if(milliseconds!==null&&milliseconds<1)throw Error("Original snapshot source deadline exhausted");return milliseconds===null?0:Math.floor(milliseconds);};
+  const remaining=()=>{const milliseconds=scriptInvocationBudget(this.invocation,0);if(milliseconds!==null&&milliseconds<1)throw Error("Original snapshot source deadline exhausted");return milliseconds===null?0:Math.floor(milliseconds);};
   const source=resolve(this.root,"../../🧬️schema/🪶️sqlite/⚠️refusal/🧪️tests/🟦️.ts");
   await runOwnedCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],this.repoRoot,"snapshot:refusal:types",remaining(),{signal:this.invocation.control.signal});
@@ -561,5 +568,5 @@
 /** 🦀️ Owning exact original cause/debit and registered job entry laws. */
 class OriginalFailureDebitNative extends OriginalFailureDebitSource{
- async run(args:string[]):Promise<void>{await super.run(args);await runRepositoryCargoTests([],this.root,this.invocation,["--offline","--lib","-E","test(original_guest_failure_debit_) | test(original_guest_job_loan_retains_whole_value_error_and_actual_context) | test(original_extension_) | test(original_inference_gateway_failure_all_owned_causes) | test(original_plugin_fixture_caller_charges_root_finance_before_body_and_closes_same_stock) | test(original_receiving_cancelled_before_first_loan_retires_same_inputs_and_source) | test(erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority) | test(original_query_output_width_does_not_price_or_renew_its_caller_receiving_grant) | test(test_config_original_record_receiving_preserves_nullable_selection)","--","--nocapture"]);}
+ async run(args:string[]):Promise<void>{await super.run(args);await runRepositoryCargoTests([],this.root,this.invocation,["--offline","--lib","-E","test(original_guest_failure_debit_) | test(original_guest_job_loan_retains_whole_value_error_and_actual_context) | test(original_extension_) | test(original_inference_gateway_failure_all_owned_causes) | test(original_plugin_fixture_) | test(original_fixture_command_projection_matches_the_closed_unit_and_data_cases) | test(original_receiving_cancelled_before_first_loan_retires_same_inputs_and_source) | test(erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority) | test(original_query_output_width_does_not_price_or_renew_its_caller_receiving_grant) | test(test_config_original_record_receiving_preserves_nullable_selection)","--","--nocapture"]);}
 }
 /** 📬️ Checks every original Store command receiving ordinal and partial envelope custody. */
@@ -612,4 +619,22 @@
  async run(args:string[]):Promise<void>{if(args.length)throw Error("original fixture close takes no arguments");const sources=[resolve(this.root,"../../🧪️tests/🧬️mutation-fixtures-job-close/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🧪️tests/🔬️unit/⛽️fuel/🟦️.ts"),resolve(this.root,"../../📬️completion/⚠️fault/🧪️tests/🟦️.ts")];const execute=(args:string[])=>runRepositoryTestCommand(process.execPath,args,{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});await execute([Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",...sources]);await execute(["test",...sources]);}
 }
+
+
+/** 📷️ Checks canonical window presence and the existing native declared-interaction law. */
+class CameraPresenceContractScript extends BundleScript {
+  async run(segments: string[]): Promise<void> {
+    if (segments.length !== 1 || !["source", "native"].includes(segments[0]!)) throw Error("test-camera-presence requires source or native");
+    const file = resolve(this.root, "../../👥️presence/📷️camera/🧪️tests/🟦️.ts");
+    await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"presence-source-types",{version:1,maximumElapsedMilliseconds:120000,output:{maximumBytes:8388608,maximumLines:65536}});
+    const { testCameraPresenceContract } = await import("../../👥️presence/📷️camera/🧪️tests/🟦️.ts");
+    console.log(`[DEBUG] shared camera portable presence admission passed ${await testCameraPresenceContract(this.repoRoot)} assertions with Ajv`);
+    if (segments[0] === "source") return;
+    const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);const receipts=await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:native.buildBudgetMs,lawMilliseconds:native.assertionBudgets[native.level]},cwd:this.repoRoot,nativeEnv:{RUST_MIN_STACK:native.rustMinStack},manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib",name:"semio_framework_plugin"},cargoArgs:["--locked"],laws:["component::plugin_runtime::plugin_builder_contract_tests::ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs"]}]});
+    console.log(`[DEBUG] canonical presence native receipts=${receipts.length}`);
+  }
+}
+/** 🏭️ Verifies real producer/source custody corpora against independent schema and physical-effect oracles. */
+class OriginalProducerSourceScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length)throw Error("test-original-producer-source accepts no arguments");await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../👷️job/🏛️reserved/⚙️worker/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/♻️retirement/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/👷️job/🎟️request/🔒️command/🧪️tests/🟦️.ts"),resolve(this.root,"../../📜️declaration/🎞️media/🏭️construction/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🔄️full-operation/🏭️producer/🧪️tests/🟦️.ts"),resolve(this.root,"../../🧵️retained-command/🏭️factory/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🎯️action-bus/🚪️admission/🧪️tests/🟦️.ts"),resolve(this.repoRoot,"🧰️framework/🔨️modules/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});}}
+
 const router = new ScriptRouter(import.meta.dir).register("test-original-fixture-close-source",OriginalFixtureCloseSource).register("test-original-media-custody-source",OriginalMediaCustodySource).register("test-original-media-custody-native",OriginalMediaCustodyNative).register("test-original-command-receiving-source",OriginalCommandReceivingSource).register("test-original-command-receiving-native",OriginalCommandReceivingNative).register("test-original-pending-worker-source",OriginalPendingWorkerSource).register("test-original-pending-worker-native",OriginalPendingWorkerNative).register("test-original-inference-close-source",OriginalInferenceCloseSource).register("test-original-inference-close-native",OriginalInferenceCloseNative).register("test-original-paged-output-source",OriginalPagedOutputSource).register("test-original-paged-output-native",OriginalPagedOutputNative).register("test-original-lifecycle-source",OriginalLifecycleSource).register("test-original-lifecycle-native",OriginalLifecycleNative).register("test-original-failure-debit-source",OriginalFailureDebitSource).register("test-original-failure-debit-native",OriginalFailureDebitNative).register("test-window-mutation",WindowMutationScript).register("test-retained-raw",RetainedRawScript).register("test-retained-structural-refusal",RetainedStructuralRefusalScript).register("test-retained-metadata",RetainedMetadataScript).register("test-snapshot-sqlite-refusal",SnapshotSqliteRefusalScript).register("test-snapshot-sqlite-admission",SnapshotSqliteAdmissionScript).register("test-command-ingress-consumer",CommandIngressConsumerScript)
   .register("test-window-registry",WindowRegistryScript)
@@ -634,5 +659,5 @@
   .register("test-codec-send-source", CodecSendSourceScript)
   .register("artifact-admission-check", ArtifactAdmissionCheckScript)
-  .register("schema-document-authority-check", SchemaDocumentAuthorityCheckScript);
+  .register("schema-document-authority-check", SchemaDocumentAuthorityCheckScript).register("test-camera-presence", CameraPresenceContractScript);
 export async function pluginTestRunnerSelfTests(): Promise<number> {
   const { createPluginRunnerTests } = await import("../../🧪️tests/🏃️runner-self-tests/🟦️.ts");
@@ -642,4 +667,106 @@
 
 
-if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
+/** 📥️ Verifies accepted remote prefixes against SQLite and the mounted plugin runtime. */
+class RemoteIngestPrefixScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-remote-ingest-prefix source|native");
+  const owner=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/📥️remote-prefix"),law=JSON.parse(readFileSync(join(owner,"🔣️.json"),"utf8"));
+  const validate=semioSchemaAjvV1({strict:true,allErrors:true}).compile(JSON.parse(readFileSync(join(owner,"🧬️schema/🔣️.json"),"utf8")));assert.equal(validate(law),true,JSON.stringify(validate.errors));
+  const {Database}=await import("bun:sqlite"),oracle=new Database(":memory:");
+  try{oracle.exec("CREATE TABLE accepted(id TEXT PRIMARY KEY,payload TEXT NOT NULL CHECK(payload='valid'),amount INTEGER NOT NULL)");for(const row of law.cases){this.invocation.control.signal.throwIfAborted();oracle.exec("DELETE FROM accepted");let refused=false,notifications=0;for(const step of row.steps){try{const result=oracle.query("INSERT INTO accepted VALUES('original',?,1) ON CONFLICT DO NOTHING").run(step==="malformed"?"malformed":"valid");notifications+=Number(result.changes);}catch{refused=true;break;}}const value=oracle.query("SELECT COALESCE(SUM(amount),0) AS count FROM accepted").get()as{count:number};assert.equal(refused,row.refused,row.name);assert.equal(value.count,row.count,row.name);assert.equal(notifications,row.notifications,row.name);}console.log("[DEBUG] remote-prefix SQLite four neutral vectors preserve accepted prefixes and suppress duplicate notifications");}
+  finally{oracle.close();}
+  if(args[0]==="native")await runRepositoryCargoTests(["semio-framework-plugin"],this.root,this.invocation,["--lib","accepted_remote_prefix_notifies_before_later_refusal","--no-fail-fast","--success-output","immediate"]);
+ }
+}
+
+router.register("test-remote-ingest-prefix",RemoteIngestPrefixScript);
+/** 🏗️ Verifies central builder retirement with neutral SQL receipts and the native owner. */
+class SnapshotBuilderOwnershipScript extends BundleScript {
+  async run(args: string[]): Promise<void> {
+    if (args.length !== 1 || !["source", "native"].includes(args[0]!)) throw Error("test-snapshot-builder-ownership source|native");
+    await runRepositoryTestCommand(process.execPath, ["test", resolve(this.root, "../../📜️declaration/🪟️surface/🏗️snapshot-builder/🧪️tests/🟦️.ts")], { cwd: this.repoRoot, budgetMs: 15_000 });
+    if (args[0] === "native") {
+      const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");
+      assert.equal(resolve(native.manifestPath),manifest);
+      const receipts=await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0)},cwd:this.repoRoot,env:process.env,nativeEnv:{RUST_MIN_STACK:native.rustMinStack},manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"snapshot_builder_ownership"},cargoArgs:["--locked"],laws:["fixture::snapshot_builder_retires_original_owners_on_success_and_refusal"]}]});
+      assert.equal(receipts[0]!.assertions,1);
+    }
+  }
+}
+router.register("test-snapshot-builder-ownership", SnapshotBuilderOwnershipScript);
+/** 📬️ Checks returned completion owners through the neutral lifecycle and real typed close. */
+class CompletionRejectionCloseScript extends BundleScript {
+  async run(args:string[]):Promise<void>{
+    if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-completion-rejection-close source|native");
+    await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../🧵️retained-command/🧪️tests/♻️completion-rejection/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
+    const {completionRejectionOracle}=await import("../../🧪️tests/🧪️artifact-admission-and-completion-oracles/🟦️.ts");
+    console.log(`[DEBUG] original completion owner source cases=${completionRejectionOracle(this.repoRoot)}`);
+    if(args[0]==="native")await runRepositoryCargoTests(["semio-framework-plugin"],this.root,this.invocation,["--lib","retained_rejected_completion_closes_original_artifact_and_ephemeral_owners","--no-fail-fast"]);
+  }
+}
+router.register("test-completion-rejection-close",CompletionRejectionCloseScript);
+/** 🧾️ Verifies original snapshot preparation receipt admission. */
+class SnapshotReadPreparationScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-snapshot-read-preparation source|native");
+  await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../📜️declaration/👁️view/📸️preparation/🧪️tests/🟦️.ts")],{cwd:this.repoRoot,budgetMs:15000});
+  await runRepositoryTestCommand(process.execPath,["test",resolve(this.root,"../../⏪️time-travel/♻️retirement/🧪️tests/🟦️.ts"),"-t","original ZST"],{cwd:this.repoRoot,budgetMs:15000});
+  if(args[0]==="native"){const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},nativeEnv:{RUST_MIN_STACK:native.rustMinStack},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"mounted_snapshot_preparation"},cargoArgs:["--locked","--features","artifact-app-testing"],laws:["receipts::mounted_snapshot_preparation_records_original_receipts_before_readiness","mounted_snapshot_preparation_boundary_uses_original_context_and_readiness","mounted_pending_source_version_compares_complete_original_authority"]}]});}
+ }
+}
+router.register("test-snapshot-read-preparation",SnapshotReadPreparationScript);
+/** 🎞️ Verifies original writer-owned and borrowed native tick sources through paid payload publication. */
+class OriginalTickPayloadSourcesScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-original-tick-payload-sources source|native");
+  const file=resolve(this.root,"../../⏯️tool-run/🎞️tick/📦️payload/🧪️tests/🟦️.ts"),sourcePolicy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
+  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"original-tick-payload-sources",sourcePolicy);
+  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"original-tick-payload-source-types",sourcePolicy);
+  if(args[0]==="source")return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
+  await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"test",name:"original_tick_payload_sources"},cargoArgs:["--locked"],laws:["original_payload::tests::original_tick_payload_preserves_same_context_authority_and_cancellation"]}]});
+ }
+}
+router.register("test-original-tick-payload-sources",OriginalTickPayloadSourcesScript);
+
+/** 📮️ Verifies persistent Reactor original page custody with independent portable ABI and event-order oracles. */
+class ResultPublicationScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-result-publication source|native");
+  const file=resolve(this.root,"../../⚛️reactor/🏃️runtime/📮️publication/🧪️tests/🟦️.ts"),policy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
+  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"result-publication-source",policy);
+  await runRepositoryCommand(this.invocation,process.execPath,[resolve(this.repoRoot,"node_modules/typescript/bin/tsc"),"--noEmit","--strict","--skipLibCheck","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--allowImportingTsExtensions","--types","bun",file],this.repoRoot,"result-publication-source-types",policy);
+  if(args[0]==="source")return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
+  await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,0)},cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib"},cargoArgs:["--locked"],laws:["component::plugin_runtime::result_publication::tests::native_page_batch_preserves_eight_original_loans_through_funded_receiving","component::plugin_runtime::result_publication::tests::native_page_batch_cancellation_preserves_original_source_and_partial_destination_custody","component::plugin_runtime::result_publication::tests::native_page_publication_cancel_after_birth_keeps_destination_before_ack","component::plugin_runtime::plugin_builder_contract_tests::concurrent_typed_operations_hand_every_presentable_page_to_one_turn","component::plugin_runtime::plugin_builder_contract_tests::original_result_batch_retains_eight_under_separate_original_input","component::plugin_runtime::plugin_builder_contract_tests::retained_operation_continues_after_command_admission_until_publication_and_retirement","component::plugin_runtime::plugin_builder_contract_tests::retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires","component::plugin_runtime::plugin_builder_contract_tests::a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled"]}]});
+ }
+}
+router.register("test-result-publication",ResultPublicationScript);
+/** 🏠️ Validates genuine Window catalog source admission, paired issuer identity and caller-funded closure. */
+class OriginalWindowCatalogScript extends BundleScript{async run(args:string[]):Promise<void>{if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('test-original-window-catalog source|native');const {originalWindowCatalogOracle}=await import('../../🪟️window/🎚️config/🏠️catalog/🧪️tests/🟦️.ts');originalWindowCatalogOracle();if(args[0]==='source')return;const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,'Cargo.toml');await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:240000,listMilliseconds:30000,lawMilliseconds:30000},cwd:this.repoRoot,env:process.env,manifestPaths:{'semio-framework-plugin':manifest},groups:[{package:'semio-framework-plugin',target:{kind:'lib'},cargoArgs:['--locked'],laws:['component::window_config::original_catalog_tests::original_window_catalog_sources_and_tickets_match_physical_original_grants']}]});}}
+router.register('test-original-window-catalog',OriginalWindowCatalogScript);
+/** 🫴️ Validates exact original artifact document source receiving against independent codecs and native custody laws. */
+class OriginalDocumentReceivingScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-original-document-receiving source|native");
+  const file=resolve(this.root,"../../💡️inference/🫴️receiving/🧪️tests/🟦️.ts"),policy={version:1 as const,maximumElapsedMilliseconds:120000,output:{maximumBytes:1048576,maximumLines:4096}};
+  await runRepositoryCommand(this.invocation,process.execPath,["test",file],this.repoRoot,"original-document-receiving-source",policy);
+  if(args[0]==="source")return;
+  const native=readCargoTestPolicyV1(process.env),manifest=resolve(this.root,"Cargo.toml");assert.equal(resolve(native.manifestPath),manifest);
+  await runRepositoryExactCargoLaws({preparation:configuredRepositoryCargoPreparationPolicyV1(),invocation:this.invocation,policy:configuredExactCargoLawPolicyV1(),cwd:this.repoRoot,env:process.env,manifestPaths:{"semio-framework-plugin":manifest},groups:[{package:"semio-framework-plugin",target:{kind:"lib"},cargoArgs:["--locked"],laws:["component::artifact_inference::document_receiving::tests::artifact_document_original_pair_receiving_preserves_exact_sources_and_physical_receipts","component::artifact_inference::document_receiving::tests::artifact_document_original_receiving_refusal_cancellation_and_foreign_source_retain_partial_custody"]}]});
+ }
+}
+router.register("test-original-document-receiving",OriginalDocumentReceivingScript);
+if(import.meta.main) router.register("test-original-producer-source",OriginalProducerSourceScript);
+/** 🏛️ Verifies genuine Reserved registry body bytes, original CPU and physical source custody. */
+class ReservedRegistryScript extends BundleScript{
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!["source","native"].includes(args[0]!))throw Error("test-reserved-registry source|native");
+  const file=resolve(this.root,"../../⚛️reactor/💼️jobs/🏛️reserved/🧪️tests/🟦️.ts");
+  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--noUncheckedIndexedAccess","--skipLibCheck","--allowImportingTsExtensions","--resolveJsonModule","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",file],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});
+  await runBudgetedTestCommand(process.execPath,["test",file],{cwd:this.repoRoot,budgetMs:scriptInvocationBudget(this.invocation,0),signal:this.invocation.control.signal});
+  if(args[0]==="source")return;
+  await runRepositoryCargoTests(["semio-framework-plugin"],this.root,this.invocation,["--offline","--lib","-E","test(reserved_registry_original_body_bytes_and_actual_receipts_share_one_source) | test(reserved_registry_original_body_work_denials_preserve_input_and_last_unit_progress) | test(reserved_registry_original_partial_output_cancel_keeps_backings_until_same_source_close) | test(reserved_registry_original_expired_clock_closes_partial_output_without_renewal) | test(reserved_registry_original_expired_clock_closes_failed_and_bounded_slot_owners) | test(reserved_registry_original_global_close_rejects_foreign_source_and_keeps_expired_clock) | test(reserved_registry_original_child_release_denial_preserves_last_work_unit)","--","--nocapture"]);
+ }
+}
+router.register("test-reserved-registry",ReservedRegistryScript);
+await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "check" }) }));
 //#endregion 🎯️Tasks

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/📋️project.json HEAD 0 INCOMING 734
--- BASE
+++ STASH
@@ -0,0 +1,25 @@
+{
+  "name": "@semio-tech/framework-window-semantic-preview",
+  "tags": [
+    "framework",
+    "plugin"
+  ],
+  "targets": {
+    "verify-source": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview",
+        "command": "bun ./📜️script.ts verify source"
+      }
+    },
+    "verify-native": {
+      "executor": "repo:script",
+      "cache": false,
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview",
+        "command": "bun ./📜️script.ts verify native"
+      }
+    }
+  }
+}

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preview/📜️script.ts HEAD 0 INCOMING 3066
--- BASE
+++ STASH
@@ -0,0 +1,22 @@
+#!/usr/bin/env bun
+import {resolve} from 'node:path';
+import {BundleScript,ScriptRouter,scriptInvocationBudget} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts';
+import {receiveScriptProcessInvocation} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts';
+import {runScriptMain} from '../../../../../../../🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts';
+import {runOwnedCommand} from '../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts';
+import {configuredExactCargoLawPolicyV1} from '../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts';
+import {runRepositoryExactCargoLaws} from '../../../../../../🦑️repo/🔨️modules/📚️library/🟦️.ts';
+/** 🔗️ Verifies the shared original history read schema, independent semantic output and physical native custody. */
+class WindowSemanticPreviewScript extends BundleScript {
+ async run(args:string[]):Promise<void>{
+  if(args.length!==1||!['source','native'].includes(args[0]!))throw Error('window-semantic-preview requires source or native');
+  const source=resolve(this.root,'🧪️tests/🟦️.ts');
+  await runOwnedCommand(process.execPath,[Bun.resolveSync('typescript/bin/tsc',this.root),'--noEmit','--strict','--noUncheckedIndexedAccess','--skipLibCheck','--resolveJsonModule','--allowImportingTsExtensions','--esModuleInterop','--target','ESNext','--module','ESNext','--moduleResolution','bundler','--types','bun',source],this.repoRoot,'value:window-semantic-preview:types',scriptInvocationBudget(this.invocation,120000),{signal:this.invocation.control.signal});
+  const oracle=await import('./🧪️tests/🟦️.ts');oracle.verifyWindowSemanticPreview();
+  if(args[0]==='source')return;
+  const manifest=resolve(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml");const receipts=await runRepositoryExactCargoLaws({invocation:this.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(this.invocation,600000),lawMilliseconds:scriptInvocationBudget(this.invocation,300000)},cwd:this.repoRoot,manifestPath:manifest,artifactDir:process.env.SEMIO_TEST_ARTIFACT_DIR,env:{...process.env,RUST_MIN_STACK:process.env.SEMIO_BUILD_RUST_MIN_STACK??'33554432'},nativeEnv:{RUST_MIN_STACK:'268435456'},groups:[{package:'semio-framework-plugin',target:{kind:'lib',name:'semio_framework_plugin'},laws:['component::window_config::semantic_preview::tests::original_window_semantic_preview_matches_portable_outputs_and_retains_boxed_press_sources']}]});
+  console.log('[DEBUG] Original Window semantic preview custody '+receipts[0]!.assertions+' exact laws');
+ }
+}
+const router=new ScriptRouter(import.meta.dir).register('verify',WindowSemanticPreviewScript);
+if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>runScriptMain(router,{invocation:original}));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📋️project.json HEAD 4720 INCOMING 5110
--- BASE
+++ STASH
@@ -116,5 +116,17 @@
         "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧪️tests/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json"
       ]
+    },
+    "test-cold-reference": {
+      "executor": "repo:script",
+      "options": {
+        "cwd": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
+        "command": "bun ./📜️script.ts test-cold-reference",
+        "forwardAllArgs": true
+      },
+      "inputs": [
+        "production",
+        "^production"
+      ]
     }
   }
-}
+}
PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📜️script.ts HEAD 9165 INCOMING 9690
--- BASE
+++ STASH
@@ -1,3 +1,4 @@
 #!/usr/bin/env bun
+import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts";
 import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 import { resolveTestLevel } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
@@ -90,4 +91,12 @@
 }
 
+/** 🧊️ Checks explicit native/reference derive boundaries under the original selected owner policy. */
+class ColdReferenceTestScript extends BundleScript {
+  async run(segments:string[]):Promise<void>{
+    const {rest}=resolveTestLevel(segments);
+    await runCargoTestsV1({manifestPath:join(this.root,"Cargo.toml"),packages:["semio-framework-os-kernel-dsl-derive"],cwd:this.root,extraArgs:["--lib","cold_reference_",...rest]},readCargoTestPolicyV1(process.env));
+  }
+}
+
 class ExportSourceTestScript extends BundleScript {
   async run(): Promise<void> { await import("../../🧪️tests/📤️macro-exports/🟦️.ts"); }
@@ -132,5 +141,7 @@
 }
 
+
 const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("generate-schema-index", GenerateSchemaIndexScript).register("schema-index", PreviewSchemaIndexScript).register("check-schema-index", CheckSchemaIndexScript).register("test-schema-index-source", SchemaIndexSourceTestScript).register("test", TestScript).register("test-exports", ExportTestScript).register("test-exports-source", ExportSourceTestScript).register("test-source-authority-source", SourceAuthorityTestScript);
 
+router.register("test-cold-reference",ColdReferenceTestScript);
 if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json HEAD 40371 INCOMING 40586
--- BASE
+++ STASH
@@ -515,5 +515,5 @@
       "options": {
         "cwd": ".",
-        "command": "bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s3-foreign-kind-in-s.mjs",
+        "command": "bun 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts ticket-probe foreign-kind-s",
         "forwardAllArgs": true
       },
@@ -534,5 +534,5 @@
       "options": {
         "cwd": ".",
-        "command": "bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️u4-pinch-diagram-contrast-probe.mjs",
+        "command": "bun 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts ticket-probe pinch-diagram-contrast-s",
         "forwardAllArgs": true
       },
@@ -806,4 +806,126 @@
         "command": "bun ./📜️script.ts parity journey",
         "forwardAllArgs": true
+      },
+      "metadata": {
+        "semio": {
+          "dashboard": {
+            "verb": "test",
+            "parameters": [
+              {
+                "id": "renderer",
+                "kind": "choice",
+                "required": true,
+                "values": [
+                  {
+                    "id": "paired",
+                    "args": [
+                      "--renderer",
+                      "paired"
+                    ]
+                  },
+                  {
+                    "id": "react",
+                    "args": [
+                      "--renderer",
+                      "react"
+                    ]
+                  },
+                  {
+                    "id": "wgpu",
+                    "args": [
+                      "--renderer",
+                      "wgpu"
+                    ]
+                  }
+                ],
+                "title": {
+                  "en": "Renderer",
+                  "de": "Renderer"
+                },
+                "description": {
+                  "en": "Choose a paired run or one renderer.",
+                  "de": "Gepaarten Lauf oder einzelnen Renderer auswählen."
+                }
+              },
+              {
+                "id": "react-serve",
+                "kind": "text",
+                "valueFlag": "--react-serve",
+                "title": {
+                  "en": "React server",
+                  "de": "React-Server"
+                },
+                "description": {
+                  "en": "Address of the React server.",
+                  "de": "Adresse des React-Servers."
+                }
+              },
+              {
+                "id": "wgpu-serve",
+                "kind": "text",
+                "valueFlag": "--wgpu-serve",
+                "title": {
+                  "en": "WGPU server",
+                  "de": "WGPU-Server"
+                },
+                "description": {
+                  "en": "Address of the WGPU server.",
+                  "de": "Adresse des WGPU-Servers."
+                }
+              },
+              {
+                "id": "locale",
+                "kind": "choice",
+                "required": true,
+                "values": [
+                  {
+                    "id": "en",
+                    "args": [
+                      "--locale",
+                      "en"
+                    ]
+                  },
+                  {
+                    "id": "de",
+                    "args": [
+                      "--locale",
+                      "de"
+                    ]
+                  }
+                ],
+                "title": {
+                  "en": "Language",
+                  "de": "Sprache"
+                },
+                "description": {
+                  "en": "Choose the language explicitly.",
+                  "de": "Sprache ausdrücklich auswählen."
+                }
+              },
+              {
+                "id": "output",
+                "kind": "text",
+                "required": true,
+                "valueFlag": "--output",
+                "title": {
+                  "en": "Output directory",
+                  "de": "Ausgabeverzeichnis"
+                },
+                "description": {
+                  "en": "Directory for generated verification artifacts.",
+                  "de": "Verzeichnis für erzeugte Prüfartefakte."
+                }
+              }
+            ],
+            "title": {
+              "en": "Compare renderer journeys",
+              "de": "Renderer-Abläufe vergleichen"
+            },
+            "description": {
+              "en": "Compare interactive journeys across the selected renderers.",
+              "de": "Interaktive Abläufe der ausgewählten Renderer vergleichen."
+            }
+          }
+        }
       }
     },

PATH 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts HEAD 13622 INCOMING 14448
--- BASE
+++ STASH
@@ -1,3 +1,4 @@
 #!/usr/bin/env bun
+
 import { receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
 /** 🧭️ Routes OS development commands to their semantic owners. */
@@ -54,4 +55,17 @@
   }
 }
+/** 🧪️ Runs retained authored ticket probes through the OS owner's cancellable command boundary. */
+class TicketProbeScript extends BundleScript {
+  async run([probe, ...args]: string[]): Promise<void> {
+    const probes = { "foreign-kind-s": "🐍️s3-foreign-kind-in-s.mjs", "pinch-diagram-contrast-s": "🐍️u4-pinch-diagram-contrast-probe.mjs" } as const;
+    const file = probes[probe as keyof typeof probes];
+    if (!file) throw new Error("ticket-probe requires foreign-kind-s or pinch-diagram-contrast-s");
+    const { runOwnedCommand } = await import("../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts"), { join } = await import("node:path");
+    const input = join(this.repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END", file);
+    await runOwnedCommand(process.execPath, [input, ...args], this.repoRoot, `ticket-probe:${probe}`, this.invocation.control.remainingMilliseconds() ?? 0, { signal: this.invocation.control.signal });
+  }
+}
+
+
 const router = new ScriptRouter(import.meta.dir)
   .registerLazy("verify-staged", async () => (await import("../../🔎️verification/🧾️convergence/🟦️.ts")).VerifyStagedScript)
@@ -68,5 +82,5 @@
      * SIGINT/SIGTERM, then stops only what it started — the zero-touch serve provider of the repository goal gate. */
     async run(segments: string[]): Promise<void> {
-      const { devServePortV1, ensureDevServe } = await import("../../🚀️local-hub/🏃️execution/🟦️.ts");
+      const { devServePortV1, ensureDevServe } = await import("../../🚀️local-hub/🏃️execution/🟦️.ts").register("ticket-probe", TicketProbeScript);
       const flag = (name: string): string | undefined => (segments.indexOf(name) >= 0 ? segments[segments.indexOf(name) + 1] : undefined);
       const serveUrl = flag("--serve");
