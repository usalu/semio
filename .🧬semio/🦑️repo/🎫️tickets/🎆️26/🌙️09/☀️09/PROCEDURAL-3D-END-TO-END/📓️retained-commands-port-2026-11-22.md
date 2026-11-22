# Retained commands port (2026-11-22)

Crate: `semio-s-artifact-procedural-3d-retained-commands` (`✏️s/.../✏️editor/🧵️retained-commands`). The name `...-retained-commands-testkit` does not exist as a crate anywhere in `✏️s` or `🧰️framework`; the testkit surface is the `testkit` feature of `semio-s-artifact-procedural-retained-source` and the in-crate `🧪️laws` modules.

## State found
Peers had already ported nearly everything: when checked, the lib and tests had exactly 1 error (not ~150).

## Change
- `📥️provisional/🎛️input/🧬️admit/🧪️laws/🦀️.rs`: `semio_framework_trace::HeapObservation` (does not exist) replaced by `semio_framework_trace::HeapAllocationObservation`, the type returned by `observe_heap_allocations_on_this_thread` (exported at the trace crate root).

No retained API edited.

## Results
- Invocation: `repo cargo` store shared, prototype, parallelism 4, `check --manifest-path ./✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml -p semio-s-artifact-procedural-3d-retained-commands --tests`. (`-p` alone without the manifest path fails: "Unknown or ambiguous selected Cargo package".)
- Errors: 1 before, 0 after. `Finished prototype ... in 9m 13s`, 11 warnings in the crate (unused `RetireOwned` imports in 5 test modules, unused `HostScratchFixtureMutation` alias, unused `new` in input module, unnecessary qualifications).
- Tests: NOT verified. First `test` run exited 101 after 30m40s but the error line was in the truncated stderr (only dependency warnings visible). Reruns now fail before compiling with a repo tooling fault: `Closed compiler builtin-read import coverage refused: the rewrite did not count every built-in import` (`redirectCapturedBuiltinReadsV1`), a peer change in the cargo capture path, not this crate.
- Transient failures seen on the way (peers/registry): wgpu `blit.wgsl` read failure (extraction race, file present afterwards), `Undeclared original Repo build selection`, and `semio-ui-wgpu-render` E0023 mid-edit by a peer; all cleared on retry.

## Left
- Rerun `test -q ... -p semio-s-artifact-procedural-3d-retained-commands --no-fail-fast` once the builtin-read coverage fault is fixed; capture the unfiltered failing line from the earlier 101.
- Optional cleanup of the 11 warnings.
- `🗑️generated` untouched (coordinator owns it).
