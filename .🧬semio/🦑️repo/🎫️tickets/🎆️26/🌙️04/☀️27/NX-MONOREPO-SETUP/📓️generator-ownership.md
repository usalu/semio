# Generator Output Ownership

The native inventory found two competing physical writers for WGPU browser artifacts: the OS package generator and the dedicated frame-worker/browser-boot targets. The package contract now identifies each browser output's physical Nx producer. The package target owns four declaration files and depends on the two browser targets; preview and freshness checking retain the complete six-artifact projection.

## Regression Evidence

A language-neutral JSON fixture and JSON Schema describe a package with three physical producers. Ajv validates the fixture; native Nx supplies the task-graph and caching oracle. The initial regression failed because the package owned all three outputs. After implementing producer inference, the graph regression passed. Native execution then passed cold build, warm reuse, restoration after deleting all outputs, and a worker-only edit: worker and package reran while browser boot retained its cache entry. The invalid producer-owner case is rejected.

Native command: private `verify:ownership` target, completed successfully in 10.5 seconds on macOS arm64. The fixture's generated workspace and logs remain under this ticket's generated folder until ticket completion.

## Runtime Qualification

Applying the producer partition to real WGPU is in progress. The canonical renderer can select one producer's outputs. Dedicated browser generation must use that renderer, and target inputs must cover its actual reads. Real WGPU cache restoration, source invalidation, package preview parity, and a fresh whole-repository ownership audit remain to be run. No Windows/Linux/container runtime claim is made.

## WGPU Runtime Checkpoint

The dedicated boot target ran successfully through native Nx with registry input discovery and registry generation scheduled first (48.2 seconds of task execution). The first selected frame-worker projection rejected two missing viewport source paths. An independent native Bun read census observed exactly 100 browser modules, with those two additions and no removed paths; the package authority and worker input contract were corrected to match.

The selected boot projection renders one 58,389-byte artifact from eight content reads. The selected frame-worker projection renders one 1,136,242-byte artifact from 101 content reads. These counts exclude the taxonomy authority read and implementation files, which are separately hashed by the Nx inputs. The four declaration outputs plus the two browser outputs exactly matched the complete canonical preview. The projection probe passed in 4.3 seconds.

The actual `@semio-tech/framework-os:generate-wgpu` task completed successfully with five prerequisites: UI axes generation, registry input discovery, registry generation, browser boot generation, and frame-worker generation. Its own writer reported exactly four artifacts and zero changed declaration files. Total task-run duration was 39.4 seconds. Native cached project metadata independently reports one output per browser target and four outputs for the package target.

The existing boot-input regression passed in 7.6 seconds, including Bun/esbuild session-independence controls and missing/stale checks preserving output bytes and mtime. Its fixture now uses a private workspace with the canonical package path and explicit workspace argument, so it exercises the same ownership validation as production.

A permanent regression now checks partition/full-preview parity, rejects an unknown producer, compares source authority with the native Bun reader, and asserts worker inputs cover every observed source or generated prerequisite. The source contract excludes Rust files; the boot source group contains its application closure, while Nx independently infers command imports. Scoped taxonomy inputs include the browser profile, output owners, path exclusions and taxonomy locator. Full repository checks and a repeated real package invocation are running; cache restoration of the real WGPU deliverables remains distinct from the successful generic native partition restoration test.

## Native Inventory and Permanent Regression

The current native inventory completed successfully in 8.0 seconds: 704 projects, 7,260 commands, 7,900 artifact/storage entries, and zero unresolved machine-checked contract findings. This clears the two observed WGPU overlap findings; it does not qualify every execution path, release promotion, platform or retention policy.

The permanent generic partition plus real WGPU projection/source regression passed in 7.8 seconds. The first broader `repo:test` retry reached these tests after passing native preparation but failed because the new test was omitted from a dynamic import destructuring. That registration error has been corrected, and the full suite is running again. A repeat actual package invocation succeeded in 33.2 seconds but reported no cache hits after intervening implementation/input edits; it is not evidence of warm reuse. A further unchanged invocation is running.

## Real Cache Reuse and Missing-Directory Regression

The unchanged real package invocation completed in 48.1 seconds and reused both browser generators from the local cache. UI axes, registry discovery/generation and the package declaration target ran; this is a measured 2/6 task cache hit result, not whole-pipeline warm reuse.

The stronger private WGPU publication test uses current canonical sources and the production renderers/package publisher under native Nx. Its initial cold run failed with ENOENT because `generateFrameWorker` did not create its output directory. The writer now creates that owned parent before writing. This targeted failure was observed before the fix; six-artifact cold/warm/deletion/source-change verification is running.

The broader suite then passed WGPU parity but failed a pre-existing command-import budget after artifact publication acquired resource leases. Independent esbuild traces found exactly three artifact-publication modules (publisher, leases, workspace-root resolver) and seven native Cargo modules (the prior six plus leases). The reviewed fixture budgets were updated from 1/6 to 3/7; command router remains 2 and Demonstrator runtime remains below its existing budget (11/12). No publisher implementation was changed for this fixture correction.

## Six-Artifact Native Restoration — Passed

The permanent combined ownership/projection/publication regression passed in 36.4 seconds after the missing-directory fix. In a private workspace, native Nx invoked the production WGPU renderer helpers and package publisher, starting with absent output directories. All six emitted artifacts matched the live canonical projection byte for byte. An unchanged run executed no producer. Removing all six output files and rerunning restored every byte without producer execution. An observable frame-worker source change reran the worker and package target while retaining the boot cache and unchanged declaration bytes. This does not delete or overwrite any active development artifacts.

The test copies exact canonical input files into its private fixture and pins its own workspace, Nx data and cache paths. Third-party controls include native Nx scheduling/storage, Bun's compiler file-read census, and the existing Ajv/esbuild boot tests. Full repository validation with this new permanent regression and reviewed publication import budgets is running.

## Empty-Checkout Discovery Correction

The broad suite passed the publication regression and reached its empty-checkout tooling test, which failed because the fixture had not copied the inferred-test dependency helper. Adding that module exposed two further assumptions: inferred test discovery eagerly read the test command router despite finding no canonical cases, and physical-producer binding validation attempted to bind contracts whose participants were all absent.

Discovery now computes command inputs only after identifying an actual canonical test case. Producer validation is scoped to contracts with a discovered participant; it still rejects a missing/mismatched producer when the package owner exists. A direct empty-project assertion was added alongside the existing invalid-producer regression. The bootstrap fixture includes the new plugin helper. Both focused tests are running before another full-suite retry.

Root setup already contains a Trunk 0.21.14 pin in its wasm dependency branch, alongside wasm-pack 0.15.0, lock-selected wasm-bindgen and both Rust wasm targets. A future Trunk prerequisite refactor should reuse that existing authority instead of inventing a separate version.


The corrected empty-checkout bootstrap test passed in 9.4 seconds, preserving foreign graph/cache storage, acquiring only pinned tooling and using the repository plugins to synchronize its private application fixture. The ownership, projection and six-artifact native restoration tests passed again in 17.1 seconds with the participant-scoped validation. A complete `repo:test` rerun is active. The remaining build-contract report now records an additional confirmed compiler-output defect: the real WGPU wasm target is cacheable with an empty output list despite writing Trunk deliverables. This is outside the completed generator partition and keeps the overall goal open.


The complete retry passed all native command, WGPU restoration, inferred Rust input/hot-reload, dependency bootstrap, cancellation and inventory checks, then failed in editor/playground verification because the registry command router no longer exports `generatePlaygroundRegistry`. The test now imports the current playground discovery and session modules directly and shares one discovered playground list across its assertions. No registry compatibility export was introduced. Another full run is active; the 3m35s preceding run is not an overall pass.

## Native Editor Coverage

The following complete run failed after 4m31s on a missing native scale-fixture build launcher. A census against the native graph found ten missing build/check/test commands: nine across the package-body oracle, WGPU renderer Rust package and viewport Rust package, plus the scale-fixture native build. The seed now contains those nine entries in its existing Rust grouping and corrects the generic scale build entry from `build-wasm` to `build`; the separate wasm launcher remains.

The focused native census passed in 1.3s with zero missing commands. Actual `@semio-tech/plugin-registry:generate` regenerated `.vscode/launch.json` successfully through Nx in 40.1s, including its discovery prerequisite. A complete `repo:test` retry now uses streamed output. Its result remains pending.
