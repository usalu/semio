# Re-verify B2 — Norm App Surface

Read-only adversarial re-verification against `📓️spec-core-assessment-model.md` (v1.2 id-path selectors), `📓️verify-b2-app-surface.md` (5 blocking + non-blocking), `📓️impl-b2-app-surface.md` (fix claims). Code inspected 2026-09-26.

**VERDICT: PASS**

## Check matrix

| # | Requirement | Verdict | Evidence |
|---|-------------|---------|----------|
| B1 | din4108 routes B2 verbs via `norm_command_from_action!` | **PASS** | `🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs:136` — `semio_s_artifact_norm_contract::norm_command_from_action!(Din4108Command, …decode_din4108_snapshot_json)` replaces the hand-written match. Macro covers `setField`/`insertItem`/`removeItem`/`applyRemedy` (`🖥️app-surface/🦀️.rs:1551`, `:1440–1531`). |
| B2 | din4108 inspection panel passes `locale` + `controller_id` | **PASS** | `🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs:157` — `inspection_panel::render(&host, …selected_check_index, view_state.locale, Some(CONTROLLER_ID))`. |
| B3 | Cancellable `evaluate` job off paint path; progress; checkpoint; revision-keyed report cache | **PASS** | Paint path: `⚖️compliance/🦀️.rs:911–914` `NormHost::from_artifact` reads `cached_report_for` only (no `F::evaluate`). Cache: `:854–892` `document_revision_key` + `(family_id, revision)` map. Job: `🖥️app-surface/🦀️.rs:1608–1731` `NormEvaluateWorkState` (progress stages, `begin_close` cancel, checkpoint/restore); `:1693–1702` runs `F::evaluate` in `NormEvaluateCommandWork::step` and `store_cached_report_for`. Routing: `:1939–1944` `build_norm_tool_job` selects `NormEvaluateCommandWork` for `evaluate` with `maximum_work_items = 3`. Checkpoint wire accepted: `:1835–1837` (no longer rejected). Per-family `evaluate::handle` still `Emit::default()` (`🧱️din4108/…/🧮️evaluate/🦀️.rs:24–25`) — intentional; evaluation happens in retained job, not synchronous handle. |
| B4 | Results rows show localized subject label + path | **PASS** | `🖥️app-surface/🦀️.rs:740–758` `check_row_label` resolves `check.subject.label` and appends `@ {path}`; `:797` used in `build_check_tree_item`. Test: `🧪️tests/🔬️unit/🦀️.rs:257` asserts subject in rendered JSON. |
| B5 | Field-meta longest-prefix + `[]` wildcard; wired at all 15 inputs | **PASS** | Lookup: `🖥️app-surface/🦀️.rs:508–572` `lookup_norm_field_meta` (exact, `[]`-template, longest prefix) + `resolve_field_meta` (hook + prefix walk). Tests: `🧪️tests/🔬️unit/🦀️.rs:371–381`, `:463–470`. All 15 `📥️inputs/🦀️.rs` pass `Some(…)` — 9 family tables (`en1990`, `en1999`, `en1991`, `en1997`, `iso16757`, `en1994`, `en1998`, `en1995`, `din18599`), 6 `empty_field_meta` stubs (`vdi3805`, `en1996`, `din4108`, `en1993`, `en1992`, `din16798`). |
| E1 | OneOf remedies applicable | **PASS** | `⚖️compliance/🦀️.rs:216–225` `Remedy::one_of` sets `applicable: true`. `🖥️app-surface/🦀️.rs:640–645` `apply_remedy_edit` writes option string; `:813–833` results UI offers per-option `setField` actions. Tests: `🧪️tests/🔬️unit/🦀️.rs:303–314`. |
| E2 | List virtualization/paging (inputs + results) | **PASS** | Threshold `NORM_LIST_VIRTUALIZE_THRESHOLD = 64` (`🖥️app-surface/🦀️.rs:881`). Inputs: `:917–944` `tree_window_section_or_placeholder` for large arrays. Results: `:859–877` `PanelTreeBuilder::window_section_or_placeholder`. Window laws: `🧪️tests/🔬️unit/🦀️.rs:166–193` (300-check report, offset/rows). |
| E3 | `[id=…]` + `[index]` path parse/get/set/insert/remove/applyRemedy/field-meta | **PASS** | Parser: `🖥️app-surface/🦀️.rs:262–473` (`parse_path`, `get_value_at_path`, `set_value_at_path`, `insert_value_at_path`, `remove_value_at_path`, `resolve_list_index` with unknown/duplicate/malformed errors). Editor id paths: `:328–338` `list_element_path` / `list_element_selector`. Tests: `🧪️tests/🔬️unit/🦀️.rs:384–431`, `:435–459`, `:474–478`. |
| E4 | Inputs editor emits `[id=…]` for lists with `id` field | **PASS** | `🖥️app-surface/🦀️.rs:328–338`, `:927`, `:952` — `list_element_path` prefers `[id=…]` when element has valid string `id`. Test: `🧪️tests/🔬️unit/🦀️.rs:474–478`. |
| E5 | Hardcoded English UI chrome | **NOTE** | See §Hardcoded English below — 5 strings remain unlocalized; not among the original 5 blocking fixes. |
| E6 | Contract tests | **PASS** | `bun nx run @semio-tech/norm-artifact-contract-rs:test` → **45 executed, 45 passed, 0 failed** (saved: `🗑️generated/reverify-b2/norm-artifact-contract-rs-test.txt`). |
| E7 | No stubs in app-surface + compliance | **PASS** | `rg` under `🖥️app-surface` + `⚖️compliance`: only API names `window_section_or_placeholder` / test name `render_report_falls_back_to_a_placeholder…` — no `todo!`/`unimplemented!`/`TODO`/`stub` code. |

## Original 5 blocking fixes — re-check

| # | Original blocking item | Status | Evidence |
|---|------------------------|--------|----------|
| 1 | din4108 `command_from_action` missing B2 verbs | **Fixed** | `🧱️din4108/…/✏️editor/🦀️.rs:136` |
| 2 | din4108 inspection `render` arity | **Fixed** | `🧱️din4108/…/✏️editor/🦀️.rs:157` |
| 3 | No evaluate progress/cancellation/checkpoint; sync eval on paint | **Fixed** | `⚖️compliance/🦀️.rs:911–914`, `:854–892`; `🖥️app-surface/🦀️.rs:1608–1731`, `:1939–1944`, `:1835–1837` |
| 4 | Results rows omit subject label/path | **Fixed** | `🖥️app-surface/🦀️.rs:740–758` |
| 5 | Field-meta exact-only lookup; no family hook wired | **Fixed** | `🖥️app-surface/🦀️.rs:508–572`; all 15 `📥️inputs/🦀️.rs` pass `Some(meta_fn)` |

## Blocking fix list

*(none — all five original blocking items are resolved)*

## Hardcoded English UI chrome (remaining)

| Location | String | Notes |
|----------|--------|-------|
| `🖥️app-surface/🦀️.rs:1096` | `"Id"` | Inspection field key — no `chrome(en, de, …)` |
| `🖥️app-surface/🦀️.rs:1005` | `"null"` | Null leaf display in inputs editor |
| `🖥️app-surface/🦀️.rs:1350` | `"null"` | `value_arg_json` default wire literal |
| `🖥️app-surface/🦀️.rs:1155` | `"Unknown body: {body_key}"` | Unknown-body fallback (English only) |
| `🖥️app-surface/🦀️.rs:1231`, `:1240` | `"Model"`, `"Report"` | `norm_io` media port labels (not `LocalizedLabel`) |

Additionally: enum/select choice labels without `NormFieldMeta::choices` use the raw choice string as both value and label (`🖥️app-surface/🦀️.rs:995`) — acceptable when choices are domain codes (e.g. annex `de`/`en`).

## Test run

```
Command: bun nx run @semio-tech/norm-artifact-contract-rs:test
Executed: 45
Passed: 45
Failed: 0
```

Output file: `🗑️generated/reverify-b2/norm-artifact-contract-rs-test.txt`

### Test coverage vs required assertions

| Assertion | Covered | Test |
|-----------|---------|------|
| `applyRemedy` flips failing check | Yes | `dispatch_apply_remedy_at_least_flips_failing_check_to_pass` (`🧪️tests/🔬️unit/🦀️.rs:287–299`) |
| OneOf application | Yes | `one_of_remedy_application_writes_selected_option` (`:303–314`) |
| `render_document_editor` emits setField/insert/remove | Yes | `render_document_editor_emits_set_field_and_list_verbs` (`:317–328`) |
| Locale switch en→de | Yes | `locale_switch_en_to_de_changes_chrome_and_check_copy` (`:331–351`); `report_and_summary_localize_en_and_de` (`:205–216`) |
| Evaluate job progress + cancellation | Yes | `evaluate_job_progress_and_cancellation` (`:355–367`) |
| Id-path cases | Yes | `parse_path_accepts_index_and_id_selectors`, `set_value_at_path_resolves_id_selectors`, `apply_remedy_with_id_path_target_flips_failing_check`, `field_meta_wildcard_matches_id_path`, `list_element_path_prefers_id_when_present` (`:384–478`) |
| Report virtualization | Yes | `an_oversized_report_stamps_its_total…` etc. (`:166–193`) |
| Retained verb cohort × 15 apps | Yes | `🧪️tests/🖥️app-surface/🦀️.rs:158–214` |
| 30-app surface render smoke | Yes | `🧪️tests/🖥️app-surface/🦀️.rs:66–89` |

## Stubs (`todo!`, `unimplemented!`, `placeholder`, `TODO`, `stub`)

`rg -i` under `🖥️app-surface` and `⚖️compliance` (`*.rs`): **no code stubs**. Matches are only `tree_window_section_or_placeholder` (framework API) and test name `render_report_falls_back_to_a_placeholder_when_nothing_was_computed`.

## Non-blocking observations

- `en1992` schema exposes `field_metadata()` (`🧬️schema/🦀️.rs:1134`) but inputs still wire `empty_field_meta` (`📥️inputs/🦀️.rs:20`) — table exists, adapter not yet connected (Wave C).
- Six families use `empty_field_meta` until per-family tables land (`vdi3805`, `en1996`, `din4108`, `en1993`, `en1992`, `din16798`) — call sites are wired; tables are empty.
- Inspection panel applies OneOf remedies via generic `applyRemedy` (option index 0 only); results expanded rows expose full per-option `setField` UX (`🖥️app-surface/🦀️.rs:813–833` vs `:1122–1145`).
- Stale comment at `🖥️app-surface/🦀️.rs:1561` still says evaluate derives report on every read — contradicts cache + retained job design.
- `norm_bounded_contract` last bound is `7_999` (`:1592`), not `15_000` cited in `📓️impl-b2-app-surface.md` — minor doc drift only.
- `impl-b2` claimed 33 tests; contract crate now runs **45** (additional id-path, OneOf, evaluate-job, field-meta tests).
