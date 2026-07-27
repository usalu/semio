# Wave 0 — Engine — Status

DRAFT — full workspace test sweep still running (see scratch/wave0-test-workspace-3.txt). This
file is being written incrementally; final version supersedes this note once the sweep completes.

## Implemented

1. dsl/core/rs/lib.rs
   - `TokenKind::BackArrow` (`<-`) and `TokenKind::Placeholder` (lone `_`).
   - `pub fn is_bare_ident(s: &str) -> bool`.
   - Wired both new token kinds into `token_classes`.
   - Extended 🧪Tests: back-arrow lexing, placeholder vs `_foo`/`foo_bar`, `is_bare_ident` unit tests.

2. dsl/schema/rs/lib.rs (biggest step)
   - Deleted `Shape::Ident` / `FieldValue::Ident`; `Shape::Text` now parses `Ident|Text` tokens,
     prints bare via `dsl_core::is_bare_ident`, quoted+escaped otherwise. Optional positional Text
     fields accept only `Text|Placeholder` on parse (not bare Ident) to avoid keyword ambiguity.
   - Deleted `Shape::RawLines`, `FieldValue::RawLines`, `consume_raw_lines`, `Chunk::Raw`,
     `Writer::raw_lines`, the forgiving whole-document lex mode. `Cursor` no longer holds source
     text or a lifetime param.
   - Split `parse` into `parse_tokens(Vec<SpannedToken>, ...)` + thin wrapper.
   - Added `Shape::Table(fn() -> RecordSpec)` — SoA columnar collection. Bare `key [col:TYPE ...]
     { rows }` form and AoS-verbose `key=[...]` form both accepted on parse; print always emits
     SoA. `shape_type_name`, `validate_table_columns` (rejects `Tuple(_, None)`/`Statements` as
     column shapes), `parse_table_soa`.
   - `parse_wire` accepts `BackArrow`, normalizes by swapping endpoints (stored/printed value is
     always `->`/`--`). Added `pub fn parse_wire_text`.
   - `print_record` field order: keyword → positionals → scalar keyed → composite keyed → Table →
     Statements, ties by declaration order (`keyed_field_rank`).
   - Added `Writer::glue()`; used for every `key=value` fusion (scalars and composites alike),
     replacing the old last-atom-string-mutation hacks in `print_key_value`, Map printing, and
     `DslValue::Object` printing.
   - Extended 🧪Tests: bare-string printing, glue spacing (List/Value/Map/Record/Wire), wire `<-`
     normalization + `parse_wire_text`, Table SoA round-trip w/ `_` cell + WIRE column, AoS-verbose
     input canonicalizing to SoA output, header without explicit type tags, Document/Inline
     agreement on a table doc, spec-build-time rejection of a non-self-delimiting column shape.
     `canonicalization_is_idempotent` still green.

3. dsl/derive/rs/lib.rs
   - `to_kebab_or_camel` (previously an identity fn) replaced by a real `to_kebab(&str) -> String`
     (PascalCase/camelCase/snake_case → lowercase kebab-case), applied to variant keywords, record
     field keys, and `DslScalar` variant tags. `#[dsl(key = "...")]` still works as an override.
   - Deleted `#[dsl(ident)]` attribute, `FieldKind::IdentString`, its codegen arms.
   - Deleted `#[dsl(raw_lines = "...")]` attribute and its codegen.
   - Added `#[dsl(table)]` on `Vec<T: DslRecord>` → `Shape::Table(<T>::__dsl_spec as fn() -> ::dsl::RecordSpec)`;
     `to_value`/`from_value` identical to the existing `VecList` arm.

4. dsl/rs/lib.rs (facade)
   - Removed the `FieldValue::Ident` arm from `String`'s `DslField::from_value`.
   - Added `pub struct Wire(pub WireValue)` implementing `DslField` with `Shape::Wire`.
   - Extended 🧪Tests: `wire_field_dsl_field_impl_round_trips`, `derived_table_field_prints_compact_soa_and_round_trips`.
   - Fixed one pre-existing test assertion (`derived_newtype_tuple_variants_round_trip`) that
     hardcoded quoted `"c1"` — now correctly bare `c1` per the new bare-string law.

5. Deleted crate `dsl/codec`
   - Grepped the whole repo for `dsl_codec`/`dsl-codec` outside `dsl/codec/` itself: zero
     consumers. Deleted `dsl/codec/rs/` entirely and removed its entry from the root `Cargo.toml`
     workspace `members` list.

## Adopter compile fixes (build was red without these — minimal, as instructed)

`Shape::Ident`/`FieldValue::Ident` were referenced directly (not just via the now-harmless-no-op
`#[dsl(ident)]` attribute) in five hand-written `DslField`/`RecordSpec` impls:
- framework/product/os/core/rs/lib.rs (`media_contract_spec`/`media_contract_to_record`/`media_contract_from_record`)
- compose/client/lib/rs/lib.rs
- norm/iso/16757/rs/lib.rs
- norm/vdi/3805/rs/lib.rs
- architect/program/rs/lib.rs

All five changed mechanically: `Shape::Ident` → `Shape::Text`, `FieldValue::Ident(x)` →
`FieldValue::Text(x)`, and `FieldValue::Text(s) | FieldValue::Ident(s) => ...` match arms
collapsed to just `FieldValue::Text(s) => ...`. No other adopter-crate edits were made.

Note: the `#[dsl(ident)]`/`#[dsl(raw_lines = ...)]` *attribute* usages that remain in several norm/
imperative/layout/procedural crates (grep found them in `imperative/core`, `layout`,
`norm/din/4108`, `norm/din/en/16798`, `norm/en/1997`, `norm/en/1999`, `procedural/3d`,
`procedural/2d`) do NOT break the build: `dsl_derive`'s attribute parser silently ignores unknown
`#[dsl(...)]` meta keys (falls through to `Ok(())`), so these are inert no-ops post-deletion, not
compile errors. Left untouched per instructions (mechanical `#[dsl(...)]` cleanup is a later wave).

## cargo build --workspace

GREEN except for one pre-existing, unrelated, actively-being-edited failure: `ui_tui` fails with
6x `E0609 no field 'canvas'/'window'/'panel'/'hover_window'/'hover_panel'/'temporary' on type
&ChromePalette` at ui/tui/rs/lib.rs:149-163. Root cause: `ui/styling/rs/generated.rs` (which
defines `ChromePalette`) is mid-refactor by a concurrent session (`git status` showed it staged/
modified while `ui/wgpu/rs/lib.rs` was also transiently dirty during this session — same
concurrent theme-system refactor). This is completely unrelated to the DSL engine (`ChromePalette`
has nothing to do with `dsl`/`vcs`/adopter DSL grammars) and was NOT touched, per the "don't fix
unrelated concurrent churn" guidance. Every other crate in two full `cargo build --workspace` /
`cargo test --workspace` runs compiled with zero errors (only pre-existing lint warnings).

Exact commands run:
- `cargo build -p dsl_core` / `-p dsl_schema` / `-p dsl_derive` / `-p dsl` — all green individually.
- `cargo build --workspace` (full log at scratch/wave0-build-full.txt) — red only on `ui_tui`.

## cargo test results

- `cargo test -p dsl_core` — 18 passed, 0 failed.
- `cargo test -p dsl_schema` — 25 passed, 0 failed.
- `cargo test -p dsl` — 15 passed, 0 failed (includes the two new Wire/Table tests).
- `dsl_derive` is a proc-macro crate with no unit tests of its own (tested transitively via `dsl`'s
  derive-based tests, per existing repo convention).

Full-workspace test enumeration in progress at the time of writing this draft — see the final
version of this file / the closing report for the complete red-crate list.
