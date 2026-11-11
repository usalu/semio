# Framework API migration, remaining consumers

## Outcome
Not completed. The 🧰️framework workspace itself is still broken by the half-finished 8fdb3e10b97 migration far beyond a handful of consumers, so ✏️s and 🌎️hub were not checked (they depend on framework crates that do not compile).

## Changed
- 🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🏁️collect-publish-and-close/🦀️.rs: testkit helper `layout_tree_now` calls `worker_one(cx)` instead of the removed 2-arg-less `InteractiveJob::step` (step now needs an `OriginalJobReturnClaim`). The lib compiles past it.

## Compile status
`repo cargo ... check --all-targets --keep-going` in 🧰️framework: about 3650 errors across roughly 250 crates (log: 🗑️generated/rest-framework.log, crate list: 🗑️generated/rest-framework-failing-crates.txt). About 168 errors are in non-test sources (14 lib crates), about 3180 in test targets.

Non-test source crates still failing (first-error causes):
- value-native-schema (61): crate split (fbf4da6a5aa) not wired; the file still uses `crate::` paths of its former parent crate.
- 2d text-font (37): RetireOwned impls (`retirement` -> `RetirementCursor` + `retirement_seed`), outline cursor `work.slot` now needs an effect.
- value dictionary and dictionary/equal (14): RetainedWorkPoll/Outcome changes, lifetime of callback.
- text-splice native (12), plugin host wake (9), actor job publication (8), pack value (5), graph layout-run (5): construction admission signatures, RetainedWorkOutcome vs Poll.
- vulkan backend trait lacks `read_back`/`debug_force_device_loss`, tree equal `RetainedSequenceEquality::advance/begin_close` missing, `NativeSnapshotEncodeEffectQuote` import missing, `close_step_granted` missing on RetainedPayloadBuilder, `OriginalWorkActionIssuer::loan` missing, duplicate `retirement_fields_*` exports, private `finish_with_progress`/`apply_bootstrap_*`.

## Left
Everything above plus all framework test targets, then ✏️s/** and 🌎️hub rechecks. This needs the framework owner to finish the API (several referenced symbols no longer exist anywhere) or a per-crate split across agents.
