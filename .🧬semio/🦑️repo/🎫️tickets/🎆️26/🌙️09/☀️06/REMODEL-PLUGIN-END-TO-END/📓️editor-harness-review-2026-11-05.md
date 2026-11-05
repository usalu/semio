# Editor Harness Ownership Review — 2026-11-05

Read-only audit of current context app/dispatch/render/run/host-turn/settle/close helpers and six-window runtime persistence law. No builds/tests or production/test source edits.

## Concrete Reopened-App Failure Cleanup Gap

In Model window config `🧪️tests/🔬️window-ownership/🦀️.rs`, the six-window runtime law creates a raw boxed reopened VcsArtifactApp and a new OriginalFixtureScope inside its async Result block. It then uses fallible load_document, each load_window_config_pack and six fallible scene renders with `?`, before reaching explicit close_registered_fixture_app.

A failure in any of those operations returns from the block and drops reopened without its paid close. The existing outer app explicitly captures outcome and closes before outcome.expect; context::RemodelingApp similarly provides funded RAII close. The reopened branch does neither on early refusal. This can replace a useful primary load/render refusal with a retained Store drop witness and obstruct diagnosis of genuine persistence failures. Root and surface owner were notified immediately. Recommended repair: capture a nested reopen outcome, always close reopened under its own live scope, then propagate the saved outcome; alternatively reuse an appropriately constructed self-closing fixture wrapper. No funding caps or framework changes are required by this finding.

## Current Driver Facts

The common RemodelingApp holds one admitted OriginalFixtureScope and paid-closes its actual VcsArtifactApp during normal Drop; the scope stays live until after that close. Dispatch sends the real typed command through public fixture authority, settles retained publication, folds returned effects into the immediate result and applies LoadDocument with the public loader. View verbs select exact window kinds; renders use the matching single instance. No direct unregistered production factory or fabricated StepContext was found in this context.

host_turn funds maintenance, advances publication, rejects Fault result pages, acknowledges every exact page token and drains typed effects/events/UI/completions/query replies. It redispatches framework tool-run actions with the same admitted caller scope. This is a specialized reconstruction host driver: it does not implement every arbitrary shell effect, so its passing tests are not evidence of an actual native file picker or browser download. The ordinary command helper separately implements LoadDocument.

settle has a finite 100000-turn guard. pump_run has a finite 50000000-turn guard; it may return at a requested nontransitional run state while typed operations remain. The finalized reconstruction tests explicitly call settle after terminal finalized before inspecting persistence; their successful completion is therefore stronger than terminal presence alone. The abort law currently snapshots durable state immediately after terminal aborted, then paid-closes app. This checks pre-close unchanged state, but does not deliberately drive extra post-abort turns and assert it stays unchanged; the separate original-caller session cancellation law supplies that local job assertion, while browser late-publication acceptance remains separate.

run_arguments recursively reads runId/generation from the real rendered run panel. Its numeric fallback converts through f64, which is adequate for these small fixture generations but is not a general exact-u64 parser. No current fixture-range failure was established and no refactor is requested from that theoretical limit.

## Six-Window Happy-Path Scope

The law dispatches six exact config updates, checks exactly six WindowConfig publications and unchanged document pack/spr, captures independent Model cameras/layers, checks Frames and Report render isolation, persists exactly six packs, reloads the original document and those packs into a distinct instance, and compares all six rendered scenes. It additionally rejects stale/wrong-kind window addressing. These are actual runtime assertions once executed; this source audit does not establish their execution. Apart from the explicit early-error cleanup gap above, no additional concrete current API or deterministic result mismatch was established.

## Reopen Cleanup Repair Review

Read the surface owner's final live repair. Both document/config-pack loads and all six reopened scene renders now execute inside a saved reopened_outcome Result. The exact reopened app is paid-closed under its same OriginalFixtureScope before `reopened_outcome?` propagates any primary refusal. If close itself refuses, its panic diagnostic includes the saved primary failure when present. The six successful rendered-scene comparisons are unchanged. This resolves the concrete early-return cleanup gap identified above for these Result failure paths. No additional concrete helper issue was found; surface reports rustfmt parse success, while native runtime validation remains pending its owner.
