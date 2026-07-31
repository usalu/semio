# Wave 1 part A — vcs `.ops` header re-derived on the DSL engine — status

DONE. `cargo test -p vcs` green (72/72). `cargo build --workspace --keep-going` shows only the
same pre-existing, unrelated `ui_tui`/`ChromePalette` failure already documented in
`wave0-status.md` (concurrent theme-system refactor, nothing to do with `dsl`/`vcs`).

## Files touched

- `vcs/rs/Cargo.toml` — moved `dsl = { path = "../../dsl/rs" }` from `[dev-dependencies]` to
  `[dependencies]` (production code now derives on it, not just in-crate tests). No cycle: `dsl`'s
  own regular deps are only `dsl_core`/`dsl_schema`/`dsl_derive`; it only *dev*-depends on `vcs`.
- `vcs/rs/lib.rs`:
  - `extern crate self as vcs;` un-cfg-gated (was `#[cfg(test)]`-only) — `OpsHeaderLine`'s
    `#[derive(DslOps)]` expansion emits `impl ::vcs::OpText for OpsHeaderLine` in production code
    now, not just in test fixtures.
  - Added `use dsl::{DslOps, DslRecord};`.
  - `🔖️TextFormat` region: replaced the hand-rolled `StructuralLine`/`parse_structural_line`/
    `escape_id_component`/`unescape_id_component`/`escape_text_field`/`unescape_text_field`/
    `find_unescaped_trailing_quote`/`split_ids`/`join_ids`/`print_authors`/`parse_authors`/`field`/
    `optional_field` machinery with a new `🔖️OpsHeaderGrammar` sub-region: `struct OpsAuthor`
    (`#[derive(DslRecord)]`, 2 positional fields) + `enum OpsHeaderLine`
    (`#[derive(DslOps)]`, variants `Doc`/`Edit`/`Change`/`Checkpoint`/`Alternative`/`Active`, `id`
    positional-first on every variant). `print_edit_lines`/`print_document_text`/
    `parse_document_text` rewritten to build/match `OpsHeaderLine` values and call
    `.print_op()`/`OpsHeaderLine::parse_op(...)` instead of hand-formatting/hand-parsing strings.
  - `🧪️Tests` region (`🔖️TextFormatHelpers` subregion): rewrote every test that exercised a deleted
    helper into an `OpsHeaderLine`/`OpsAuthor` round-trip test; updated the two `parse_document_text`
    fixture strings (`@doc ...`/`@edit ...` → `doc ...`/`edit ...`) and one `print_edit_lines`
    assertion (`"@edit id="` → `"edit "`).
- `framework/plugin/rs/lib.rs` — NOT touched. Verified: its `DocumentApp` bounds only name
  `vcs::DocumentDsl`/`vcs::OpText` (trait definitions, unchanged) — `cargo build`/`cargo test -p
  semio-framework-plugin` both green except one unrelated pre-existing UI-icon-catalog test failure
  (`app::form_kit_tests::entity_detail_builds_a_stack_with_header_key_value_and_actions`, panics in
  generated `ui/asset/icon/generated/icon_name.rs` — nothing to do with vcs/dsl).

## Example — one real `.ops` document, captured via a temporary `[DEBUG]` eprintln in
`document_text_round_trips_after_apply_and_checkpoint` (removed after capture; not part of the diff)

Before (old hand-rolled grammar):
```
@doc schema=demo/v1 id=demo
@edit id=e1 actor=- started=17 finished=- key=- "resize"
@checkpoint id=c1 parent=- changes=ch1,ch2 by=u1%3Aueli:Ueli at=18 "first"
```

After (actual captured output, `#[derive(DslOps)]`-generated):
```
doc demo schema=demo/v1
edit edit-2 started="1785153565994" actor=local finished="1785153565994" description=bump
  set-n n=3
change change-3 saved="1785153565994" description=c1 edits=[ edit-2 ]
checkpoint checkpoint-4 at="1785153565994" message=c1 changes=[ change-3 ] by=[ a1 Alice ]
```

Notes: `@` sigil gone (bare lowercase keywords); `id` positional-first; numeric-shaped strings
(timestamps) print quoted per `is_bare_ident`, everything else (ids, `local`, `bump`, `c1`, `Alice`)
prints bare; absent optionals (e.g. `key=`, `finished=` when unset, `parent=`) are omitted entirely
— no more `-` placeholder; `edits=`/`changes=`/`by=` are real space-joined DSL lists in `[ ]`, not
comma+percent-encoded strings; `by=[ a1 Alice ]` — `OpsAuthor` positional `id name`, `Author.avatar`
still never serialized (matches the pre-existing behavior).

## Test commands run

- `cargo build -p vcs` — green (only pre-existing unrelated lint warnings).
- `cargo test -p vcs --no-run` — green.
- `cargo test -p vcs` — 72 passed, 0 failed.
- `cargo build -p semio-framework-plugin` — green.
- `cargo test -p semio-framework-plugin` — 99 passed, 1 failed
  (`app::form_kit_tests::entity_detail_builds_a_stack_with_header_key_value_and_actions`, panics in
  `ui/asset/icon/generated/icon_name.rs` — "invalid catalog icon name"; unrelated to this ticket's
  scope, no vcs/dsl code in its call path).
- `cargo build --workspace` / `cargo build --workspace --keep-going` — red only on `ui_tui`
  (`E0609 no field 'canvas'/'window'/'panel'/'hover_window'/'hover_panel'/'temporary' on
  &ChromePalette`, `ui/tui/rs/lib.rs`) — same pre-existing, unrelated, already-documented
  concurrent-session failure from `wave0-status.md`. Everything else in the workspace compiles.
- `cargo test -p dsl_core -p dsl_schema -p dsl` — 18 / 25 / 15 passed, 0 failed (unaffected by this
  wave's edits; re-run as a sanity check since `vcs` now regular-depends on `dsl`).
- `cargo clippy -p vcs --no-deps` — no new errors/warnings beyond the pre-existing
  `unnecessary qualification` lints already present before this change.
