# Bitmap Final Correctness

The 52-case retained decision corpus previously compared decisions, diffs, diagnostics and inverse ordering without applying the retained delta. Each admitted case now executes central retained preflight and commit through the actual BitmapApply, takes mutable post authority only after retained aliases close, closes the staged plan and source owners, and compares the complete resulting BitmapSnapshot with the cold apply_mutation oracle confined to this test. No runtime cold fallback was added.

Added actual BitmapCommandWork cancellation and close coverage at three frontiers (before reserve, after reserve, after pixel materialization), using all portable cursor extents including 512×512. The same OriginalWorkFixture that pays for cursor advancement cancels and retires the production command work. Per-step heap observations compare every allocation/release with reported retained effects; inline ControlledRetirement handoff must allocate zero bytes, and total release must exactly match the actual pin backing. The preexisting close_original_granted_step RetainedWorkOutcome repair and inline handoff are preserved.

Recovered three corrupted working source inputs. Removed exactly 3187 trailing NUL bytes after the complete retained command source. Both preview JSON and editor Cargo manifest were wholly NUL in the working tree and staged index. The editor Cargo manifest was reconstructed from read-only HEAD content with its allocator testing dependency restored. The new preview fixture is hand-authored from its consumer contract because HEAD has no such file: fresh keep, unavailable/empty/contradictory/stale/extent/padding/trailing-byte/palette refusals, work limits, bounded completion, maximum cancellation and prior-order canonicalization. No git mutation was performed.

The actual diff application helper requires explicit native-owned test dependencies, added to the inference and artifact-root package manifests. The allocator remains test-only.

Validation: pinned nightly-2026-07-20 rustfmt syntax-only checks passed for retained commands, retained corpus tests, pin cursor and pin unit tests. Native execution remains owned by the consolidated native verifier and has not been claimed as passing. No native queue was added by this slice. Ticket lifecycle remains with parent.

Files: bitmap retained commands; retained mutation tests; preview JSON fixture; editor/inference/artifact-root Cargo manifests; this report.

Final recheck: scoped diff --check HEAD passed. The bitmap Python/Pillow oracle rerun passed 14 committed vectors, 9 pin-batch cases and 52 retained cases, zero failures. Native remains pending the consolidated verifier.

## Test Dependency And Input Audit

Read-only final audit confirms all three changed bitmap manifests keep the newly used helpers under dev-dependencies: artifact-root native-owned and allocator, editor allocator, inference native-owned. The inference value test dependency enables testkit; kernel protocol-laws/mutation-testing and plugin artifact-app-testing are enabled only in test dependency sections. Every helper path resolves to an existing framework package. These framework package manifests have no dependency back to a WFC artifact, so the new test edges do not create a direct artifact/framework production cycle. New helper functions remain private test code; no public signature exposes allocator, OriginalWorkFixture or native-owned test control types.

The bitmap, graph2D and graph3D Rust project nativeSources each explicitly include the shared WFC test tree, covering the new preview-revisions and graph-fill-envelope fixtures. Artifact source JSON/Rust inputs remain covered by their existing source collection. Their package runners enumerate component packages; the native verifier independently confirmed bitmap:test covers all ten component crates, including editor allocator/close laws, inference retained52 application laws and snapshot BitmapApply laws. This audit does not replace native compilation.

No source change or new build queue was needed for this audit. Compiler diagnostics remain delegated to this agent through the consolidated native verifier.

## Lock Carriers

Manually aligned only confirmed missing dependency rows with current manifests:
- bitmap/Cargo.lock: bitmap root, editor, inference and snapshot package rows now include their required projection/factory/native-owned/allocator edges. Added the already authoritative local semio-framework-value-testing-allocator 0.1.0 package row from repository Cargo.lock; its trace dependencies already existed.
- WFC/🏭️bridge/Cargo.lock: six runtime dependency rows now include bitmap projection/factory and all three transient projection edges, including both graph editors.
- 🌎️hub/Cargo.lock: three bitmap runtime projection/factory dependency edges.

Hub and bridge consume artifacts as external path dependencies, so artifact dev-dependencies were deliberately excluded from those consumer lock rows. Only the bitmap owning workspace lock acquires the allocator test package. Root and WFC plugin-root locks contain none of these artifact package rows and needed no modification. Both graph owning workspace locks already matched their relevant projection manifests. The native verifier's separate grid3d-editor bridge edit was preserved.

Nine root/hub/WFC/artifact lock carriers parse with Python's builtin TOML reader. Scoped diff --check HEAD passes for all three edited carriers. No package version was changed, Cargo command was run or git state was modified. No lock copy was placed in the ticket.

Follow-up: on native verifier request, added the missing grid3d-editor snapshot-projection runtime edge in hub/Cargo.lock. The same edge was already handled by the native verifier in its owning grid3d and bridge carriers. Final three-carrier diff --check HEAD passed.

## Cohesive Pin Command Component

The legitimate native gate reached WFC Source Admission and refused the bitmap editor at 4037 active lines against its unchanged 4000 limit, before Rust compilation. Added a real pin-solution command component at the existing command's 📦️packages/🦀️rust directory. The existing command source, cursor and portable unit tests remain in their canonical locations; no tracked source was copied.

The new semio-s-artifact-wfc-bitmap-pin-solution package owns the command/cursor and pure pin validation, complete materialization, cancellation and inverse laws. The editor imports this component and no longer mounts those unit tests into its own compilation unit. The actual BitmapCommandWork close/allocator/cancel law remains editor-owned. Removed the pin command's unrelated ownership by the shrink-output test package. The command accepts the expected output window kind from its editor caller, avoiding any dependency back to the parent editor. The leaf's unit tests use a small explicit reference scene instead of depending on an example-owning test package; there is no dev dependency cycle. Its public root explicitly exports the command and cursor interface.

Updated bitmap workspace members and the explicit artifact package test runner to execute the new component's library tests. Updated bitmap-owning, WFC bridge and hub Cargo.lock carriers with the new local package and root/editor edges. Consumer package rows contain runtime dependencies only; job/JSON test dependencies occur only in the owning workspace lock row.

Checks passed: pinned syntax-only parser over eight affected Rust files; builtin TOML parsing of 13 manifests and three lock carriers; manifest path existence; no checked new path exceeds 160 UTF-16 code units; scoped diff --check HEAD. No source policy override, line minification or test removal was used.

Public analyzer preflight input is retained as bitmap-pin-source-preflight.ts. With explicit cfg=[test], readRustSourceClosureV1 reports editor 3867 active lines, 236377 bytes and 25 files, excluding the pin unit-test mount; independent pin owner 441 active lines, 31328 bytes and five files, including its tests. These are explicitly non-authoritative preflight measurements and are not an upper bound: the failed unit's exact compiler cfg was not persisted. The sole actual full native gate must establish Source Admission and runtime correctness. The native verifier has been notified to retry; this agent starts no native queue.

## Late Completion Cache Preservation

A late old completion could previously overwrite a newer fresh app cache even though render filtering would subsequently hide the stale value. Bitmap CommitFillSolve and both graph CommitFill retained work paths now compare the completion source against current command authority before producing SetSolve. Stale internal callbacks settle quietly with no transient rows or cache overwrite. Graph completion also requires done=true.

Actual owner Work::step regressions receive authentic OriginalWorkFixture turns, collect the returned transient rows, apply them to an existing complete cache and check fresh completion changes it while stale completion emits zero rows and preserves every prior field. Shared six-case source fixtures cover exact/mismatched/malformed sources. The unbound case is checked at an optional-authority admission helper: a real retained Work input itself requires AppOperationContext and is not forged for that case. Tests therefore verify row effects and cache preservation rather than fault wording.

Pinned parser checks pass ten affected Rust files. The qualified public cfg=[test] preflight now measures bitmap editor 3906 active lines, 244364 bytes,26 files; pin leaf455 lines,36675 bytes,six files. These remain non-authoritative and are not an upper bound. The next actual native Source Admission and compiler run remain required.

Actual native source receipt: verifier reports gate 77440 completed Source Admission for all 448/448 selected units, including the repaired editor and new pin component, under unchanged source policy. Real compiler entry was observed at proc-macro2/quote/unicode-ident/syn. This establishes the native source-boundary admission, not WFC Rust type correctness or a test pass.

## Current Original Caller API Alignment

All bitmap, graph2d, graph3d and grid3d fill unit drivers, bitmap/grid3d inference fixture closures, and grid3d inline producer law now reuse the shared TestJobDriver from birth/step through outcome acknowledgement and original caller closure. Removed old self-funded closure and raw test_step_context calls in those four artifact trees. Bitmap and graph fill/inference trait callbacks now accept OriginalJobReturnClaim and return InteractiveJobStep::from_callback; owned Publication polling propagates OriginalTurnFailure without flattening. Output writer/payload close receipts pass through receive_original_retained_work. All six parents use the shared close_publication_step/publication_retirement_demands helpers so boxed publication headers close in a separate paid turn. Child and restore metadata removal also takes a separate one-item/one-CPU turn; boxed graph child headers quote their actual struct release. Completed fill child and inference restore owners remain on their parent original close ladders, without runtime close_job. Syntax checks passed; actual compiler/type gates remain pending with the native owner.
