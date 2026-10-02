# Native Preparation Queue

The snapshot verification fleet repeatedly failed before compilation because the selected preparation process and queued lease admission both charged a 30-second deadline while another owner held the resource. This was a prerequisite failure, not a snapshot assertion.

The owned queue already provides arrival ordering, observable wait diagnostics, stale-owner cleanup and signal cancellation. The repair removes the lease admission deadline and the outer selected-preparation subprocess deadline. Each active package preparation recipe retains its existing 30-second execution limit. Native test profiles and their floors are unchanged.

A permanent language-neutral fixture holds an independent owner for 31,250 milliseconds. The actual selected Cargo invocation must wait, publish its package-owned input after admission and retain clean JSON machine stdout. Ajv independently validates the fixture, Node timers hold the competing owner, and the proof runs in a separate Bun process against a minimal physical Cargo workspace.

The registered `@semio-tech/cargo-workspaces:queued-contract-check` reproduced the defect: 0 passing laws, 1 failing assertion, aborted selected preparation, 31.38 seconds. After the repair the same registered route passed 1 law and 5 assertions in 31.84 seconds. Runtime output confirmed `kernel/Cargo.toml publish` after the wait. Generated workspaces and captured outputs remain under this ticket's generated folder until the entire snapshot goal is complete.

The route is registered in the canonical Cargo `📜️script.ts`, its `📋️project.json`, and the existing launch configuration gate group. Its scoped 45-second test process budget covers the deliberately long queue law; the existing 15-second workspace contract route remains unchanged. The route resolves its output through the existing caller-owned test artifact environment.

After removing temporary debug output, the fresh uncached registered route passed again against the final source. Nx completed in 36.0 seconds with the same independent 31,250-millisecond competing owner and five assertions. No native profile or active preparation execution deadline was changed.

BCF's artifact-owned composition receipt also retained its former `bcf` native extension after the logical codec changed to `semio`. Its exact extension and runtime capability now agree with the source factory and definition. The actual BCF structural hash assertion remains pending native execution. Concurrent VCS and store source changes may still produce compiler prerequisite failures; those remain separate from feature verification.

A new read-only native process audit found twenty-seven sleeping Cargo invocations with no rustc, clang or linker children. The older owned Generation2d application-feature build still retains per-unit Cargo build locks. A one-second native sample is recorded under generated output; the procedural owner was notified to retire only its older owned application compilation once its explicit feature-selector regression route is verified. No cache or lock file is removed, and other owners’ processes are untouched. Pending invocations are not recorded as passing tests.

The Cargo sample confirms its main queue waiting on condition variables and worker threads waiting in `prebuild_lock_exclusive` → `LockManager::lock` → `flock`; it does not show active compilation. The procedural owner has now terminated only its owned old Generation2d Cargo28105/Nextest28012, leaving all cache and lock files and other owners intact.

After the owned superseded build was retired, rustc PID66005 became active under the already queued AVI Cargo30988. This confirms renewed compilation, not completed native test execution. No global Cargo settings were changed.

A later process audit again found no compiler or linker children. Root therefore retired only its own overlapping Nextest listing/compile invocations for WAV, MP4, EPW, BCF and the initial EN1990 capability selection, validating each exact package and process identity first. AVI remains the current root native gate. These are cancelled pending runs, not runtime failures or passes; the completed public/source evidence remains separate. Each full native owner gate will be relaunched in sequence after the current root gate completes. All shared cache and lock files and all other owners’ processes remain intact.
- Owned process 37270: semio-s-artifact-stdio-epw; 1 descendant processes retired.
- Owned process 41247: semio-s-artifact-stdio-bcf; 1 descendant processes retired.
- Owned process 45188: semio-s-artifact-norm-en1990; 1 descendant processes retired.

Reconciliation with final process outcomes confirms AVI,WAV and MP4 had completed successfully before root’s overlapping-run retirement. The identity-checked retirement actually affected only EPW,BCF and the initial EN1990 capability build; their exit1 represents cancellation. Completed native evidence: AVI48/48,WAV50/50,MP4 63/63. Root will replace the cancelled initial EN1990 selection with its complete owned nine-law gate, then rerun BCF and EPW in sequence.

### Effective Intermediate Directory Verification

Read the pinned installed Cargo documentation (`nightly-2026-07-07` config.html) and repository `.cargo/config.toml`: CARGO_TARGET_DIR only overrides deliverables; CARGO_BUILD_BUILD_DIR overrides intermediate compilation units. The canonical repository Cargo directory resolver explicitly supports both independent environment overrides. Current EN1990 native lane has no assertions after more than 110 minutes of admission. A task-owned temporary intermediate/deliverable override can be tested without changing shared configuration or deleting another builder's cache; it is not a proved global lock-cycle repair and is not a permanent isolation policy.

Only the verified root-owned EN1990 Cargo70372 (parent70322) was terminated after almost two hours with no assertions. The registered replacement uses both directory environment overrides under this ticket's generated cargo-root-norm-owned. Within about twenty seconds it entered actual Rust/framework compilation, with rustc child processes and compilation output. This is real preparation progress, not a native test result or a global deadlock repair. No foreign processes or caches were stopped/removed, and shared Cargo configuration remains unchanged.

The replacement reached an owning Norm editor compiler prerequisite after 2m22s: E0004 at app-surface render_value_editor, whose exhaustive DSL match omitted intrinsic Bytes. The editor now projects a localized byte extent without copying payload bytes into UI nodes; an owning native law covers a 65,537-byte property in English and German using independent serde_json projection. Neither that new law nor the EN1990 assertions have run yet. The same paired directory is retained for the next registered native retry.

The EN1990 paired directory retry completed compilation in8m39s. The separate Nextest assertion operation then exceeded the repository fundamental15s total process budget, with no pass/fail output published before cancellation. This is a runtime-harness budget failure, not nine passing laws or a specific semantic assertion RED. The next invocation uses the existing quick level300s without changing any global budget or adding a new profile; compilation remains sequential in the same warmed paired directory.

First quick-level retry failed during project graph construction on concurrently admitted EN1997 missing root facade and existing missing test-host/renderer-producer edges, before owner execution. EN1997 facade was authored by its owner; no foreign compatibility project was added. Retrying canonical EN1990 with invocation-only plugin isolation disabled and same warmed directories.

EN1990 actual nine-law native GREEN: Nextest6f33c37a-ca2b-4403-8d8a-822ac3f34360,9/9selected pass,0.513s assertions/3m40s total with3m00 compilation after concurrent shared DSL changes. The quick-level rerun used the existing300s profile; the selected laws themselves took under1s, so the earlier15s cancellation did not establish a slow semantic law. All9include full typed and actual declaration-owned Binary/Text I/O across all raw-word vectors. No controlled parser hook coverage is inferred. Root now moves to full BCF verification sequentially in the warmed task-local directory pair.

BCF full owned native GREEN:7dc5de8c-c0a4-4527-af3e-4a9796a30103,50/50tests,0skipped,1.412s assertions/4m51s total. Authentic factory/hash and actualtyped/erasedSQLIO included; same task-owned warmed directory pair. Root next native lane is full EPW.
