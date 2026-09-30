# Extension Bundle Session Retirement Gap

Read-only source review on 2026-09-30. No builds or tests were run. Coordinates refer to the current shared working tree.

## Confirmed Session Changes

In `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:805`, `close_step` checks terminal state, zero grants, and pause before `begin_close`, then rechecks under its transactional locks. A zero grant no longer seals an open authority. Lines 765–775 enforce explicit sealed authority, transferred jobs, and terminal-empty family retirement on ordinary final destruction; these are refusal guards, not synchronous cleanup.

`Session::close` at line 836 is deliberately synchronous for cold callers: it begins close and drains positive-progress slices until terminal, blocked, error, or zero progress. It cannot establish terminal family retirement while another admitted authority remains open or an actual native reader holds the lock. The producing retained owner must keep driving the family afterward. The `GeometryPort` implementation at line 1375 only calls `begin_close`; dropping a port after that does not run native garbage collection.

Cancellation and resumption at lines 801–803 affect the shared family cursor. They preserve sealed authority and retained payloads; cancellation of one child therefore pauses family retirement, including work transferred by siblings. This is family-wide pause semantics, not independent child cursor cancellation.

## Actionable Extension Lifecycle Gap

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:43787` defines `ExtensionBundle` with only a manifest and a map of boxed request handlers. The handler type at line 43784 is a synchronous `Fn(&[u8]) -> Result<Vec<u8>, Fault>`. The builder's `handler` method at line 43942 installs those closures. There is no retained owner, budgeted close callback, terminal query, or cancellation/resumption hook.

The process-local bundle is retained in `EXTENSION_BUNDLE` at line 43968. `extension_deactivate` at line 44013 only clears the active flag and explicitly preserves handlers. `install_extension_bundle` at line 43973 replaces the previous `Option` by assignment, ordinarily dropping the prior handlers. The `extension_exports!` implementation beginning at line 44052 uses `PluginRuntime<NoPluginApp>` and supplies no connection between that runtime's retirement and the captured bundle resources.

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:2065` creates a real Session in `extension_guest::bundle`. Its clones are captured by `evaluate` at line 2075 and `tessellate` at line 2079; the original is captured by `tessellateCancel` at line 2096. Request cancellation does not close the Session family. Consequently, deactivation leaves the native family retained, and replacement ordinarily destroys unclosed captured authorities, triggering the new explicit-close guard. An actor close currently has no route to drive that family to terminal-empty state.

The required integration belongs in the neutral ExtensionBundle/runtime lifecycle: admit a retained resource owner, seal invocation before retirement, drive its budgeted close slices with progress and cancellation, retain the bundle until terminal, and prevent replacement from dropping an unfinished owner. BREP can then attach its captured Session through that mechanism. A user-facing capability named `close`, a synchronous closure destructor, or hidden garbage collection would not supply this missing lifecycle ownership.

## Validation Scope

This report establishes source relationships only. It does not claim native runtime success. Parent-reported Playbook preview and renderer failures remain separate active validation work.
