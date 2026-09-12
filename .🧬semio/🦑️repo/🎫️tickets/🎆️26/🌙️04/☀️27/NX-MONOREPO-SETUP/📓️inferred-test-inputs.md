# Inferred Native Test Input Coverage

## Confirmed Native Hash Gap

The native Node/Nx hash probe completed in 35.9s without executing any test or compiler. It selected the inferred DOCX transitional mutation subject target. Native Nx collected **556 file inputs** for hash `5554831827136213098`. None of the **35** checked compilation manifests were included: the workspace `Cargo.toml`, `rust-toolchain.toml`, the subject manifest and 32 additional local dependency manifests. The collected paths were normalized to workspace-relative paths; sample inputs include `.gitignore`, `.nxignore` and `nx.json`.

The test runner selects the nearest ancestor `📦️packages/🦀️rust/Cargo.toml` in `rustSutCrate`, then links it from its generated host. For this case that subject is the DOCX artifact package, outside the narrower transitional subset owner. Its local dependency closure was computed by the same Cargo resolver already qualified against native Cargo metadata in the cache contracts. The probe establishes a missing-input defect; it does not claim a real application test cold/warm run or a compiler execution.

Probe: `🔬️native-inventory/🦀️test-inputs/📜️script.ts`. Logs/data: `🗑️generated/native-inferred-rust-test-inputs-node.log` and `🗑️generated/native-inferred-rust-test-inputs.json`. Run native Nx graph/hash APIs with Node inside a selected Nx target; Bun graph construction failed because its worker could not require the core plugin's top-level-await ESM module.

## Required Repair

Share subject/host/contributed-package selection between runtime execution and Nx inference. Add the complete selected native source and toolchain inputs, preserve phase distinctions (oracle-only work must not build the subject), attach existing generator prerequisites in the outer task graph, and expose project dependency edges for affected selection. Native `nativeSources` named inputs and `nativeDependencyRoots`/`nativePreparation` already provide most Cargo dependency logic. Do not recursively invoke the core project normalizer or another repository-wide scheduler from the test plugin.

The test plugin currently hashes only the case, subset owner, contribution globs, testing-domain files and shared globals. It adds no Cargo dependency edges or prerequisites. The runtime also creates standalone native host manifests and builds without `--locked`; immutable host dependency resolution and artifact ownership need qualification before claiming complete test caching.

## Implementation Notes

- Test plugin: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs`.
- Runtime selection and host materialization: the same domain's `📜️script.ts`, `rustSutCrate` and `materializeRustHost`.
- Core Cargo dependency helpers and native input groups: library `🟨️.mjs`. Generator contracts are source data in taxonomy `generatorContracts`.
- `oracleHostPackagesFor` includes contributions from **all matching ancestors**, despite its nearest-contributor docstring. Contribution directories support owner-specific taxonomy overrides. A per-owner ancestry walk can resolve relevant package declarations without scanning the entire repository. Preserve this behavior when sharing selection code.
- Core project identifiers come from an authored `📋️project.json` name, otherwise the Cargo package name. Referencing native projects' named inputs must use those identifiers, not crate aliases.

## Checked Manifests

- `Cargo.toml` — missing
- `rust-toolchain.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📦️packages/🦀️rust/Cargo.toml` — missing
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/Cargo.toml` — missing
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml` — missing


## 2026-09-12 — Inferred Rust Closure Implementation

The current test plugin already added the nearest subject package directory and ancestor oracle package directories. It still lacked Cargo’s transitive local package inputs, the source files mounted outside those package directories, compiler inputs, native generator prerequisites, and affected-project dependency edges.

A schema-backed fixture now models a nested owner, a subject whose Rust entry point is outside its Cargo package, one transitive local crate, a test host and a local oracle crate. The focused check failed before the plugin repair: no `nativeSources` project references were present (expected four). After the repair it passed against native Cargo metadata, existing native source discovery, and explicit generator/phase assertions. The first passing isolated Nx run took 768 ms; this is a small fixture measurement, not a monorepo benchmark.

The test plugin now resolves shared subject/oracle selection, references the existing Cargo projects’ `nativeSources`, adds compiler inputs and owned generator prerequisites, and contributes static project edges for affected selection. Contract phases do not acquire native prerequisites; oracle-only phases omit the subject’s native closure. Existing broad owner/contribution inputs remain conservative and are not claimed to be optimally narrow.

Native Nx cold/warm, deleted-output restoration and source-invalidation checks are being added to this fixture. Full current monorepo hashing and the broader suite remain pending. The initial current-worktree probe waited over ten minutes amid repeated native graph construction, so only this task’s verified client processes were stopped; no shared daemon or cache was reset. Two incorrectly scoped/private-data diagnostic launches were stopped before their test target ran. The corrected fixture pins both workspace-root variables and private workspace-data/cache locations.


The expanded native fixture passed in 12.7 seconds on macOS arm64. A real Cargo consumer returned `7`; its identical second Nx invocation ran neither fixture generator nor compiler handler. Removing both generator and consumer output directories restored both without rerunning either handler. An unrelated file preserved that cache hit. Changing a transitive crate changed the returned value to `8` and reran only the consumer. This proves the inferred input/prerequisite configuration in the private fixture, not full application execution or another operating system.

The regression is registered in the repository cache-contract suite. That broader suite is currently running with private Nx graph state. Existing broad test-owner/contribution inputs, generated standalone Cargo host lock ownership, command-import coverage, and test-plugin daemon reload behavior still require separate assessment.


The current native monorepo graph contains 704 projects. The same DOCX transitional case now hashes 1614 files and all 36 checked Cargo/workspace/toolchain manifests: **zero missing**. Its native hash is `9247065352388108470`. The resolved graph records 36 outgoing edges for this inferred case. This hash-only diagnostic does not execute the actual DOCX compiler/test or claim its cache reuse. The native file-map references were hydrated from Nx's workspace context without rerunning project discovery.

The first broader suite run failed after 1m04s of target execution on three stale browser-distribution imports in the production component materializer. Their existing modules had moved to the plugin's `🌐️browser-bundle` domain. The imports were corrected to those existing files; a full suite rerun is in progress.


Native Nx affected selection also passed for a simulated whole-file change to `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`: the inferred DOCX case is selected. No repository source was mutated for this diagnostic. Seven existing test-plugin selection checks passed (26 assertions).

The full suite rerun passed the repaired browser component materializer, including native warm and deleted-output restoration. It then stopped in existing `testNativePreparation` while the repository's global taxonomy validator rejected three print-related declarations: `print-graph-macro-source.role`, `packageGlueGrammar.tex.analyzer`, and `print-latex-semio-graph` source-format classification. This is not a passing full suite. Those taxonomy declarations have not been altered by this task.


## Shared Router Inputs and Resident Plugin Refresh

A second schema-backed regression failed because inferred targets had no shared command-source input group. The plugin now computes the testing router's relative import closure once per graph invocation and exposes it as `testCommandSources` on each case. All phases hash that group, together with the JavaScript toolchain contract. The fixture compares the closure with esbuild's metafile and verifies that changing a helper outside the testing domain reruns the native consumer. The expanded native cache test passed in 25.2 seconds.

A resident-plugin regression then failed: changing the private dependency helper to invalid JavaScript did not reject the next graph callback, demonstrating stale loaded code. The plugin refresh implementation is now being verified against that regression; no shared daemon/cache reset is used.


The complete focused regression passed in 12.4 seconds after moving the resident-plugin check to Node, the runtime used by native Nx. It verifies native Cargo/Nx cache behavior, esbuild command imports, and invalid-helper rejection/correction through the same resident plugin instance. A retained-module identity guard prevents recursive reload when a caller's ESM loader returns stale code. The earlier Bun-hosted reload attempt was stopped after it retained the old module and looped; no shared process was stopped.

Fresh native graph construction also succeeded: 704 projects, 0 graph errors. The sampled inferred case exposes 51 command input declarations. Broader suite status remains the recorded taxonomy failure; this graph success does not turn that failed suite into a pass.


Final current-case native hash after router coverage: `18418207222661578617`, 1641 input files, all 36 checked manifests present, and the inferred case remains selected by native affected calculation for the transitive source change. This is 27 more files than the previous 1614-file native hash, not a repository-wide source glob expansion.
