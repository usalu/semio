# Workspace Cleanup Process Ownership Correction

Workspace cleanup no longer contains a name-based process termination or zombie-parent reaping mechanism. The unsupported stray-process command is removed, and ordinary cleanup no longer invokes a sweep. Actual owned-execution cancellation and explicit service stop remain at their existing owners. No new process registry, PID adoption, whitelist, environment-wire capability or caller-supplied owner callback was introduced.

The actual cleanup command now decodes its closed entry before observing or changing resources. Unknown and removed modes, including either dry marker, fail before filesystem, cache or process work. This prevents the removed spelling from falling through to full workspace cleanup. Existing testing/taxonomy tails remain delegated to their actual owners. The skill's unsafe executable-name killing instructions and fast-path command are removed. AGENTS files are unchanged. No stray-process launch entry existed; the existing generic launcher continues to cover supported cleanup modes.

The active-build regex moved unchanged into the process-table observation module. Its remaining use is the existing conservative cache-prune check and portable parsing tests. It is not a process ownership or termination rule.

## Schema and Source Evidence

The existing fixture now embeds a closed portable schema with 11 process-table scenarios and 18 entry cases. Every census row is unowned DATA: Bun, Node, rustc, IDE/dashboard resemblance, orphan status, live ancestry, zombie state and Original-looking argv cannot select a signal. A SQLite relation independently produces the same empty signal selection, and Ajv plus the first-party schema validator agree on the fixture.

The Source tests execute the actual extracted CleanScript and entry decoder with explicitly modeled effect counters. Normal and dry cleanup perform no process sweep; removed and unknown commands produce no modeled filesystem/cache/inventory/signal effects. The first semantic RED recorded the old actual CleanScript calling `process-sweep` after normal workspace cleanup, missing closed decoding and the still-present signal module. No actual cleanup or process-removal command was invoked.

Final `const-fold/📜️script.ts --cleanup-tests`: natural zero, eight laws / 160 assertions, 1.86 seconds. Four cleanup laws plus four existing portable process-table laws ran; fifteen unrelated Windows/host/filesystem laws were intentionally outside this Source gate. Final `--cleanup-strict`: natural zero for four roots with no compiler diagnostics. Both modes are Source-only, use the existing helper, and do not grant Native capabilities.

The first post-change Source gate retained a stale Windows-test fixture path failure. The exact two fixture references now name their actual existing test directory. The second gate exposed an old test applying a single-row canonical schema to the entire process fixture; it now validates every actual fixture row against that unchanged schema. The parser and Windows command behavior were not weakened. All failed logs remain separate from the final result.

## Scope and Limits

The concrete global kill path was unsafe if executed, but this Source correction does not attribute any prior SIGKILL to a particular invocation. We did not run workspace cleanup, inspect protected Native assets, terminate a process, or claim system-wide process custody. Runtime ordinary Windows integration remains paused at its separate Source checkpoint. The final ten-input manifest includes nine current files (launch.json is read-only) and one removed runtime module; no new runtime Source file was added.

Evidence: `🗑️generated/bazel-planner/workspace-process-ownership-{red,green1,green2,green3,strict1}.log` and `workspace-process-ownership-ten-manifest.json`.
