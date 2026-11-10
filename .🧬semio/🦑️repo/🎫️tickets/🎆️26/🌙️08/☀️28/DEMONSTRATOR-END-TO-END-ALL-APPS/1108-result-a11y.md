# 1108 a11y result

## Hand edits (step 9), done
- Host workflow emit: `os_workflow_to_node_graph_payload(graph, terminology, locale)` resolves port labels (was emitting the matrix object into a `String` field of `NodeGraphPortRecord`, which would have decoded to an empty graph); hub space window `workflow::render` takes `view_state` and passes `terminology`/`locale`; standalone host test updated.
- TS tests: `renderer-types-and-forms` (matrix cells, en and de assertions), demonstrator `demonstratorpanebranding` (both locales, matrix breadcrumbs).
- Schema JSON: `connect-ports` and `add-node` mutation schemas: `LocalizedLabel`/`LocalizedLabelRow` defs, `MediaPortSpec.label` refs them. Rows: `rows-a-h` `terminologyBreadcrumbs: Array<unknown>` + docstring.
- Persisted fixtures: 24 committed component descriptors (`hub/*/component/json`): port labels and reuse breadcrumbs rewritten to the typed matrix with `1108-a11y/descriptors.py` (same tables as `localize.py`; the later `describe` run must reproduce them byte-identically).
- Already done by core.py (verified in tree): workflow record + sqlite columns + snapshot schema, norm plugin, `workflow_node_for_app`/`add_workflow_node` label argument, sqlite and mutation fixtures.

## Generator (step 10)
`workspace:schema-generate` (label `W-a11y-gen-1`) rc 1: not my edits, the build environment refuses ("Every owner needs edges of the published environment: semio-framework-pack-json-output-receiving", a peer's new crate). The two catalog scopes (`add-node`, `connect-ports`) were therefore updated by hand: exports `LocalizedLabel`/`LocalizedLabelRow` inserted alphabetically, `hashes` recomputed (sha256 of the file, verified against an untouched scope). Other stale hashes in the catalog (energy model3d windows, flow vcs mutations) are peers'. Generated manifest TS (gitignored) comes from `@semio-tech/framework-rs:generate` (native cargo test `exports_typescript_bindings`), not run (needs a green native framework). Rerun when the environment is fixed: `"$C/1105b-tools/slot.sh" W-a11y-gen-2 env SEMIO_LANGUAGE=en repo run workspace:schema-generate`.

## Fixes found by compile or scan
- `workflow-graph` `parameters` (`sync_workflow_node_parameter_ports`): literal `"Parameter".into()` missed by localize.py (its regex excludes braces inside the struct) -> `LocalizedLabel::native("Parameter", "Parameter")`.
- Same miss, 6 more sites outside the checked roots: hub space unit tests (`In A`, `Out A`, `Out B`, `Scene`, `test_port`) and host `workflow-unit` (`Port`) -> `LocalizedLabel::data(..)`. `check.py` scans only framework and the plugins; a repo-wide brace-matched scan now reports 0 string labels in any `MediaPortSpec`.
- Cargo: `workflow-snapshot` +semio-ui-locale (sqlite label cells use it in production code) and `semio-os-run` +semio-ui-locale dev-dep (unit test). LOCK NEEDED logged in the claims file.
- FALLBACK-IMPORT (task 2): the host-unit tests are `include!`d into `semio-os-host` (plugin-host), which already depends on `semio-ui-locale` in `[dependencies]` -> OK, no change. Host-workflow has it as dev-dep, enough for its test use; its production code uses `semio_framework::{Locale, Terminology}`.

## check.py (step 5)
`python3 -I 1107-a11y/check.py` -> `violations: 0`, exit 0.

## Compile (step 12), wasm, labels W-a11y-<n>, log `$G/a11y-chain.log`
- GREEN: pack-json W-a11y-1, ui-locale W-a11y-2, manifest-media W-a11y-3, manifest app-surface W-a11y-4.
- RED, not mine: plugin-app-builder W-a11y-5, plugin-decl-surface W-a11y-6: `semio-plugin-ownership` fails (`RetainedOwnedProjectionAdmission`, `SharedOriginalStorage` unresolved), claimed by gen3d2 (plugin/ownership, 14:05). Everything above the plugin tier (builder, declaration, host, 8 panes, norm, wgpu, hub) is blocked behind it.
- RED, mine, fixed: workflow-graph W-a11y-7 (the "Parameter" label); re-run W-a11y-11 rc0 (GREEN).
- `workflow-snapshot` W-a11y-8 rc1: one real a11y miss fixed (`sqlite/snapshot/cells`: port label was counted and read as a plain text; now four label cells in `typed` and `borrowed`, byte model unchanged vs the +15 fixture). The other 11 errors are upstream drift, not a11y: `ArtifactSqliteSnapshotHost` is now output-slot shaped (`SnapshotReceivingOutcome`, 6-arg validate, 4-arg decode); the workflow sqlite impl (and every plugin sqlite impl, see claims line 1034 "hosts lane sweeps them") still has the old shape. Unverified after the cells fix: W-a11y-12 (snapshot) was still compiling when I stopped.
- NOT RUN: workflow, host-workflow, semio-os, 8 panes, norm, wgpu, hub (blocked by the ownership and sqlite-host red above, plus LOCK for the two new Cargo deps).

## Rerun (after gen3d2 releases plugin/ownership, the hosts sweep lands, and the coordinator locks the 2 Cargo.toml)
`for p in semio-plugin-app-builder semio-plugin-decl-surface semio-framework-artifact-workflow-workflow-snapshot semio-framework-artifact-workflow-workflow semio-os-host-workflow semio-os <8 pane crates> semio-s-plugin-norm-app-surface semio-renderer-wgpu-shell-session semio-hub-space-app; do "$C/1105b-tools/chk.sh" W-a11y-<n> Cargo.toml wasm -p $p; done`, then `1105b-tools/hub.sh W-a11y-hub-1`. Also `desc` for the 24 descriptors: `python3 -I 1108-a11y/descriptors.py` (dry run reports 0 files now) must stay byte-identical after the next `describe`.
Not checked: native/TS test runs (renderer-types-and-forms, demonstratorpanebranding) were edited but not executed; wgpu navbar callers (`Locale::parse` placeholder from core.py) are unconfirmed by compiler.

## Update after coordinator message (ownership green)
- Workflow `ArtifactSqliteSnapshotHost` ported to the output-slot shape (flow pattern: pre-checks return `NotStarted`, then `decode/encode_sqlite_snapshot_record_native(.., output)`; the two `validate_*` overrides dropped, trait defaults apply). `workflow-snapshot` W-a11y-20 rc0 GREEN (includes the label-cell fix in `cells`).
- wgpu placeholder: the 3 navbar callers (`chrome-frame-and-main-window`, `navbar-footer-and-overlays`, `keyboard-and-palette`) now use `self.prefs.active_locale()` (the shell prefs locale authority, no default, no `Option` leak) instead of `Locale::parse(..)`. Not compiled yet.
- plugin-app-builder W-a11y-21 rc1: `semio-plugin-window-config` Source refused, `window/config/owner-registry` is 30323 B > 29117 B per-file budget. Not an a11y file by content (3-line diff is extB's); extB has an active claim on window-config owner-registry (16:52). Blocked on extB; I did not raise the budget.
- Chain still queued in `scratchpad/run4.sh` (labels W-a11y-22..37: decl-surface, workflow, host-workflow, semio-os, 8 panes, norm, wgpu-shell-session, renderer-wgpu, hub-space-app); log `$G/a11y-chain.log`. TS tests: `repo run @semio-tech/mit-bestand-demonstrator:test` as W-a11y-ts-1 was queued (rc/ txt in `$G`); renderer-types-and-forms is a vitest `engineSuite` under the renderer react target, not yet run.

STATUS: BLOCKED builder/declaration/host/panes/hub behind extB (window-config owner-registry source budget, claim 16:52); verified green: pack-json, ui-locale, manifest-media, app-surface, workflow-graph, workflow-snapshot (sqlite host ported); check.py 0 violations
