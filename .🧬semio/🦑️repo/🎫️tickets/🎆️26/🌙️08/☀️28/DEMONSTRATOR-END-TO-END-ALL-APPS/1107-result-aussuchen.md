STATUS: UNVERIFIED — newest proof W-aussuchen-smoke-55 (1105b) RED at apply; no 1107 run finished. Waiting for LOCK of the curation workspace (claim 03:27).

- Last finished run: C/🗑️generated/1105b/W-aussuchen-smoke-55.txt rc 1. Load and runtime-b's Retiring stall passed; fails at pane-smoke-law:21 "the typed command must change the loaded document" (applied snapshot == loaded snapshot, curated: []).
- W-runtimeb-101 (runtime-b's run of this pane, 03:19) died at CargoLockRefused: curation Cargo.lock behind manifests, `semio-framework-plugin-reactor-receiving-source` unresolvable (proc fixed the dep key at 03:25). Needs `dependencies lock` (coordinator).
- No duplicate smokes: my own queued W-aussuchen-smoke-101 was cancelled; runtime/runtime-b also run this pane.
- Static note: curation-add resolves via curation_decision_for_delta -> NoOp if the id is absent from document.stock_extra. Test id comes from the boot snapshot; demo stock first id is beam-glulam-gl24h (avail 24), so NoOp is unlikely; likely the mutation is not applied/published (settle) — to confirm from the next run.

## 03:50 static trace of the apply fault (UNVERIFIED hypotheses ruled out by code reading)
- Dispatch path in the law (dispatch_original_fixture_command -> handle_action -> settle_registered_typed_operation -> app.snapshot()) is identical to sourcing unit test `dispatch` (editor/🧪️tests/🔬️unit:76-87); the handler reads the same `stores` snapshot the probe reads (composer-api snapshot()). No stale-store split.
- curation_add -> curation_decision_for_delta is NoOp only if the id is not in `stock_extra`; boot and demo stock are the same document, so Create(item) is expected; preparation (artifact-store-preparation) refuses with a Fault, not silently.
- Remaining suspect: loading the document (store replacement) before the command; unverified. Temporary test `aussuchen_apply_probe` (pane-smoke test file) prints dispatch result, settle receipt (completions/lanes/effects/revisions), document revision before/after load and after settle. Runner: 1105b-tools/aussuchen/probe.sh, watcher waits for lock-sync-1107-ok then runs as W-aussuchen-smoke-101. REMOVE the [DEBUG] test after.

## 04:15 W-aussuchen-smoke-101 (rc 1) — shared source-boundary refusal, before compiler
Log C/🗑️generated/1105b/W-aussuchen-smoke-101.txt: semio-framework-job-context over maxSourceBytes (267,439 > 262,144; park file 13,686 B). Already reported by runtime-b (claims 04:14, W-runtimeb-102) to runtime; not mine. Probe untouched, rerun W-aussuchen-smoke-102 once runtime splits the crate.

## 04:35 W-aussuchen-smoke-102 (rc 1): job-context source budget still exceeded (log 1105b/W-aussuchen-smoke-102.txt)
Refusal moved to ⏱️context/📦️return/🎟️place/🦀️.rs (8,256 B vs 2,175 B remaining, 12 files left): runtime's split is in progress, crate still over 262,144. Not aussuchen-owned. Probe still pending; rerun as W-aussuchen-smoke-103 after runtime's split.

## 04:45 a11y audit findings 17 + 18 (C/1107-audit-a11y.md) — NOT edited, cross-cutting schema
- 17: `MediaPortSpec.label` is `String` (🧰️framework/🔨️modules/🛂️manifest/🎞️media/🦀️.rs:200), ~20 construction sites + generated TS; sourcing site is editor-vocabulary/🦀️.rs:20 ("Catalog"/"Katalog"). Needs the field to become LocalizedLabel (wire shape exists) — owner runtime.
- 18: `AppDefinition.terminology_breadcrumbs: HashMap<String,Vec<String>>` (manifest/🧱️app-surface/🧬️app-definition:105; builder `terminology_document`; generated 🪪️manifest TS; consumer breadcrumbs:14). Ten plugins pass the German project name; sourcing site is editor/📋️manifest:39. Needs localized segments or a brand key.
- Sent to runtime (a8f570504dc3f4b48); sourcing call sites switch as soon as the types exist. Smoke W-aussuchen-smoke-103 and the probe remain pending on job-context source budget.

## POST-E2E DEBT (runtime decision 04:50, deferred; sourcing call sites stay as they are)
- a11y 17: LocalizedLabel type for `MediaPortSpec.label` (sourcing "Catalog"/"Katalog" at editor-vocabulary:20).
- a11y 18: localized segments + a brand key for `AppDefinition.terminology_breadcrumbs` (sourcing editor/📋️manifest:39). Runtime takes both as one typed sweep after the first green e2e.

## 07:05 W-aussuchen-smoke-103 (rc 1): compiler reached (source budget cleared); foreign crate fails
Log 1105b/W-aussuchen-smoke-103.txt: `semio-framework-value-native-owned` (lib) 60 errors (E0277 `I: NativeValueSource` unsatisfied, E0053 close_original_*_step signature) in value/🛬️decode/🧳️owned — value lane mid-edit. No sourcing-owned error. Probe + smoke still unrun. Rerun as W-aussuchen-smoke-104 when it clears.

## 07:10 W-aussuchen-smoke-104 (rc 1): foreign `semio-framework-value` (lib) 278 errors (retained-clone/paged, paged/receive, shared-utf8/clone, ordered/numeric scratch work) — value lane mid-sweep; the native-owned crate no longer the frontier. No sourcing-owned error; probe/smoke still unrun. The no-op apply stays UNEXPLAINED until a run reaches the test. Rerun as 105 later.

## 07:20 W-aussuchen-smoke-105 (rc 1): foreign only — semio-framework-value 12 errors (was 278), native-receiving-original 22. Probe/smoke unrun; rerun 106.

## 07:36 W-aussuchen-smoke-107/108 (rc 1): foreign only — semio-framework-pack-json-canonical 35 -> 25 errors (value compiles now). Probe/smoke unrun; retry 109.

## 07:41 W-aussuchen-smoke-109 (rc 1): pack-json-canonical cleared; next foreign frontier semio-framework-pack-json 171 errors. retry 110.

## 07:50 W-aussuchen-smoke-110 (rc 1): foreign semio-framework-pack-json 134 errors (was 171). Per coordinator: NO more reruns until claims show pack-json FIXED by aggregator and valuefix. Probe + smoke pending; next run W-aussuchen-smoke-111 then.

## Test-path port (addendum 09:01), source only, compile UNVERIFIED
Ported sourcing's two store-preparation modules (editor/🗄️artifact-store-preparation, editor/📬️config-store-preparation) to the current ArtifactStoreOneItemPreparation trait: `advance` takes `semio_framework_value::NativeDecodeControl`, returns `OriginalReceivingPrimary`, `commit_work(u128)`; `close_step` returns `OriginalRetirementFailure`; added the four no-pending receiving-recovery methods; replaced the non-test `.expect` with a typed InvariantViolated refusal. Other audit item (dep `semio-framework-test-original-snapshot-owners` close 1) is testdebt-b/fixture-owned. Proof: W-aussuchen-smoke-111 when dsl-record-model clears (dslowned claim 08:46 open).
