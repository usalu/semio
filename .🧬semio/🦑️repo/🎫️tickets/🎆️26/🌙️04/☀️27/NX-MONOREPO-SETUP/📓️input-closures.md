# Task Input Closure Refactor

## Observed Remaining Overreach

The current repository plugin gives every task the entire root script, every TypeScript/JavaScript/JSON file under the repository library, and the complete caching directory. Those are shared implementation bytes, but the root script contains unrelated commands and the caching script includes its full test suite. Editing a contract test consequently changes production task hashes. Exact Rust reference discovery already follows source references and include assets, yet Cargo package owners additionally include every TS/JS/JSON asset below the domain owner. The root framework Rust package therefore receives browser implementation changes from across the framework subtree. A frontend edit can invalidate native consumers through dependency defaults even when rustc never consumes that file.

The shared script library also imports the entire framework public module only to obtain ephemeralBox for a process-local Cargo-name index. That framework module reexports multiple product-neutral runtime subsystems; the kernel module providing ephemeralBox itself imports actor/shard machinery. Command-only imports need a smaller domain boundary before input closure can be narrowed honestly.

## Required Implementation Boundaries

1. Separate executable command/test implementation files by domain using only `📜️script.ts` filenames for routers. A native leaf must not import repository contract tests or the root product command router.
2. Retain the native Cargo source closure and explicit build-script/config/asset inputs, replacing broad sibling frontend globs with proven consumed sources or declared asset dependencies.
3. Build a source-import closure for each script leaf, including package exports, dynamic imports whose paths are statically resolvable, and explicit declarations for dynamic filesystem/tool consumers. Unknown behavior must remain visible, rather than quietly disappearing from task hashes.
4. Give test and production targets distinct source inputs. Avoid assigning every toolchain/environment variable to every operation merely because a project happens to contain multiple languages.
5. Verify real Nx task reuse across frontend-only, test-only, native source, shared schema, toolchain and deleted-output cases. Keep a language-neutral fixture and independent compiler/import-resolution oracle for each inference boundary.

Native preparation currently in flight is useful baseline evidence. Avoid deliberately mutating its broad shared input set while it is compiling. The long-term gate remains no unrelated native work after a frontend leaf edit; the present broad contracts do not meet that gate.

## Other Discovered Test Debt

The Flow output-boundary library test still passes a removed skipEnvVar option to runWasmPackWebBuild and expects to avoid compilation. That shortcut no longer exists. Replace the obsolete test seam with direct output-path/contract validation before the full repository-library suite is claimed. The focused Unicode suite intentionally does not qualify that unrelated test.

The threaded runWasmPackWebBuild branch maps the Cargo dev profile to --dev, which Cargo does not accept. Its wasm-pack --dev mapping is valid. Add profile argument cases and an actual Cargo oracle, then correct only the Cargo invocation. Current Note prerequisites use the non-threaded path; this defect remains outstanding.

## Implemented Native Command Boundary

Artifact staging moved into a dependency-free domain module. The Cargo producer now has its own script and imports only staging, process execution, workspace discovery and the shared command router. The router classes were extracted from the large repository library and remain reexported through that library. Thirteen producer imports and generic native target declarations were updated together; no forwarding compatibility command remains in the old audit/test router. Independent esbuild bundles verified one artifact input, two router inputs and five native producer inputs (`command-boundaries-green.log`). The initial red test refused the not-yet-existing artifact entry.

Cargo projects now expose separate `nativeSources` and `nativeTestSources`. Statically mounted Rust source and embedded assets are tracked directly; separate test/example/benchmark entry points are included only in the test source set. Generic native tasks and WASI component tasks use these dependency source sets plus the native command's literal import closure and compiler/toolchain inputs. Generic native hashing excludes unused renderer, plugin-selection and JavaScript test-budget environment variables. The source set for crates with unparsed module syntax or build scripts remains conservative over that crate's domain; it no longer inherits unrelated shared command/test implementation through `production`.

Language-neutral vectors passed an independent rustc dependency-file check for included binary assets and excluded frontend/separate-test files. The TypeScript tooling boundary's literal command-import closure matched esbuild's independent input inventory (`native-inputs-corrected.log`). The first compiler-oracle attempt required an explicit ASCII crate name for its emoji source filename; the corrected run passed. Real Nx mutation/reuse/restoration for these new source sets still needs qualification.

## Remaining Native Discovery Limits

Thirteen concrete Cargo projects currently need conservative source fallback. Several use build.rs to launch Bun generation on existence/mtime checks (schema, graph and UI). These remain hidden scheduling and cache correctness problems. Other fallbacks come from Rust macro/module forms and require their exact parser diagnostics before narrowing. Browser-oriented custom scripts still use the original broad source/toolchain inputs; the new closure is deliberately applied only to the isolated generic Cargo producer and component tasks so far.

## Native Runtime Qualification

Twelve real Nx scenarios passed (`native-inputs-cache-linked.log`, retained `📓️native-input-restoration.md`). Warm, frontend-only, separate-test-only, audit/test-only and variant-environment runs left the producer completion count at one. Deleting the complete staged output while the compiler store was absent restored identical file hashes without executing the producer. An independent Rust consumer linked and ran using the restored rlib and rmeta. Native source, embedded asset, compiler flags, workspace compiler contract and producer implementation mutations each caused the expected additional execution, ending at six. Earlier consumer attempts incorrectly supplied only one of the required rlib/rmeta pair; the producer already staged both, and the final consumer supplied both explicitly.

The source/bundler vectors are now permanent repository fixtures executed by `repo:test`. The threaded WASM helper's Cargo dev profile now uses `--profile dev`, while wasm-pack retains its own `--dev` flag. An actual Cargo CLI red test rejected the old `--dev`; language-neutral profile rows and each installed CLI's argument parser validate the corrected distinction. The obsolete Flow test no longer uses a removed compiler skip environment variable: it tests the extracted output-directory validator directly. Its focused suite passed one test and 53 assertions (`flow-output-boundary-corrected.log`). These checks do not claim an end-to-end threaded browser build.

The permanent `repo:test` suite completed with exit 0 after adding the command/source vectors and profile parser oracles (`native-inputs-permanent-contracts.log`, 1m33s). This includes real rustc dependency-file and esbuild input comparisons in the public repository contract route.

## Scoped Native Discovery Reuse

Profiling exposed repeated command-source discovery in the graph builder: two graph passes invoked nativeCommandInputs/relativeScriptInputs 1,962 times. One per-graph command input snapshot now serves all native leaves, reducing those calls to two, while production/test Rust closures are each computed once per project (728 cargoSourceInputs calls versus 994 in the comparison run). The snapshots are scoped to one graph construction and rebuilt after source changes. Wall-clock samples were affected by simultaneous full Stdio compilations, so call counts—not an uncontended speedup—are the confirmed comparison. The optimized generic-native fixture and non-Cargo cache verifier are being rerun.
