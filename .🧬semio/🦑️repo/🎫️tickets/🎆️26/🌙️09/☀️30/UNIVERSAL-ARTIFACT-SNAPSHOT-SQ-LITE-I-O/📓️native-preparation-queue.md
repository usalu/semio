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
