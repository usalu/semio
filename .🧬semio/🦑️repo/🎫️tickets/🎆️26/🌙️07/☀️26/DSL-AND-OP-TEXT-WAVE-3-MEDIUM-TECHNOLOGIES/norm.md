# norm — DocumentDsl + OpText (Wave 3)

## Scope

`norm` has 15 family crates total (not 13 — the assignment undercounted; see below), all sharing
`norm_core::SetDocumentOperation<D>` as their op type via `norm/plugin/rs/lib.rs`'s
`define_norm_family_app!` macro.

## Generic OpText (norm/core/rs/lib.rs, `//#region 🔖️OpText`)

One `impl<D: vcs::DocumentDsl + Clone + Default + PartialEq + Serialize + DeserializeOwned>
vcs::OpText for SetDocumentOperation<D>` covers every family at once. Op line format:

```
set-document "<D::print_dsl() text, escaped onto one line>"
```

Escaping copies vcs's own private `escape_text_field`/`unescape_text_field` convention exactly
(`\\`, `\"`, `\n` — same three escapes, same precedence) via two new private helpers
`escape_op_text_field`/`unescape_op_text_field` in norm_core (vcs doesn't export its versions, so
they're duplicated, not reused, but behave identically). Example for `DemoDocument { value: 7.0 }`
(test-only type in norm_core's own test module):

```
set-document "value 7\n"
```

Tested in norm_core's existing `mod tests` (extended, no new file): `demo_document_dsl_round_trips`,
`set_document_operation_op_text_round_trips`, `set_document_operation_op_text_escapes_multiline_dsl_text`
(explicitly checks the `\n` escape), `document_text_round_trips_for_a_norm_family_document` (full
`DocumentVcsStore` round trip).

## Shared `dsl_kv` helper (norm_core, `//#region 🔖️DslKv`)

Every family's `Document` is a flat-ish struct of scalar fields (f64/u32/u8/bool/String/enums), so
instead of 13-15 near-identical hand-rolled tokenizers, `norm_core::dsl_kv` provides:
- `parse_lines(text) -> HashMap<String, String>` — one `key value` line per field.
- `trait DslScalar { print_scalar, parse_scalar }` with impls for numeric primitives, `bool`,
  `String` (bare unquoted token — every current string field is an identifier/enum-like tag with no
  internal whitespace), and the norm_core-shared enums `ClimateZoneDe`, `AnnexChoice`,
  `ImposedCategory`, `DesignSituation`, `OccupancyType`.
- `scalar::<T>(&fields, key)` / `line(key, &value)` for reading/printing one field.
- Family-local enums (`part_1_2::FireCurve` in en1991, `part_1_2::FireRating` + `part_3::TightnessClass`
  in en1992, `MasonryClass` + `part_2::ExposureClass` + `part_2::MortarClass` + local `UseClass` in
  en1996/din18599) get their own `impl DslScalar` written directly in that family's own `lib.rs`
  (local type, so no orphan-rule issue implementing the norm_core trait there).

Each family's `Document` DSL is then just a `key value` per line, e.g. (DIN 4108):

```
category residential
climate zone2
airtightness_n50 2.5
...
layer 0.24 0.81
layer 0.14 0.035
```

Collection fields (the only ones any family has) are hand-written outside `dsl_kv` right next to it:
- **din4108**: `Vec<LayerDocument>` → repeated `layer <thickness_m> <lambda_w_mk>` lines.
- **en1990**: `Vec<(String, f64)>` (`q_k`) → repeated `q_k <category> <value>` lines.
- **din18599** (`BalancingInputs`): nested `MonthlyClimate { theta_e_c: [f64;12], g_h_w_m2: [f64;12] }`
  → two `climate_theta_e_c <12 comma-separated values>` / `climate_g_h_w_m2 <...>` lines.

## Completed families (13/15)

| Family | Crate | Extension | Notes |
|---|---|---|---|
| DIN 4108 | norm_din_4108 | `din4108` | `Vec<LayerDocument>` handled by hand |
| DIN EN 16798 | norm_din_en_16798 | `din16798` | 62 scalar fields, pure `dsl_kv` |
| DIN V 18599 | norm_din_v_18599 | `din18599` | nested `MonthlyClimate` handled by hand + local `UseClass` |
| EN 1990 | norm_en_1990 | `en1990` | `Vec<(String,f64)>` handled by hand |
| EN 1991 | norm_en_1991 | `en1991` | local `FireCurve` DslScalar |
| EN 1992 | norm_en_1992 | `en1992` | local `FireRating`, `TightnessClass` DslScalar |
| EN 1993 | norm_en_1993 | `en1993` | 74 scalar fields, pure `dsl_kv` |
| EN 1994 | norm_en_1994 | `en1994` | pure `dsl_kv` |
| EN 1995 | norm_en_1995 | `en1995` | pure `dsl_kv` |
| EN 1996 | norm_en_1996 | `en1996` | local `MasonryClass`, `ExposureClass`, `MortarClass` DslScalar |
| EN 1997 | norm_en_1997 | `en1997` | pure `dsl_kv` |
| EN 1998 | norm_en_1998 | `en1998` | 49 scalar fields, pure `dsl_kv` (note: `annex` is `String` here, not `AnnexChoice`) |
| EN 1999 | norm_en_1999 | `en1999` | pure `dsl_kv` |

Every completed family got, in its existing `#[cfg(test)] mod tests` (no new test files):
`document_dsl_round_trips`, `set_document_op_text_round_trips`, `document_text_round_trips_through_store`
(the last builds a real `DocumentVcsStore`, applies a `SetDocument`, and calls
`vcs::test_support::assert_document_text_round_trip`).

## Left undone (2/15): ISO 16757, VDI 3805

Both are genuinely out of scope for a quick `key value` DSL:

- **ISO 16757** (`norm/iso/16757/rs/lib.rs`, 1770 lines): `Document` nests a full product
  `Catalogue`, a `Dictionary` (subjects/properties/controlled value lists/relationships), a
  `GeometryCatalogue` with a recursive `GeometryNode` enum, `HashMap<String, CatalogueValue>`, part-5
  `PartNumberRule`/`ExchangeProcess`. This is a structured-document format, not a flat record.
- **VDI 3805** (`norm/vdi/3805/rs/lib.rs`, 2723 lines): `Document` nests a `ManufacturerFile`,
  `BTreeMap<u16, EditionProfileChoice>`, a `CatalogIndex`, `BTreeMap<String, ParametricGeometry>`,
  `BTreeMap<String, CharacteristicCurve>`, native VDI record structures with `VdiValue`
  enums/units — effectively its own catalogue file format.

Designing a faithful textual DSL for either is a standalone design task (essentially a mini nested
serializer with its own grammar for lists-of-records/maps/recursive nodes), not a mechanical
`dsl_kv` wrapper like the other 13. Left for a follow-up ticket rather than rushing something that
would only be a JSON-with-different-punctuation encoding in disguise.

## Test results (all green, confirmed by running `cargo test`/`cargo check`)

- `norm_core --lib`: 8/8 pass (includes the 4 new OpText/DslKv tests).
- All 13 completed family crates, `cargo test -p <crate> --lib`, 0 failures:
  norm_din_4108 21/21, norm_en_1990 16/16, norm_en_1991 11/11, norm_en_1992 23/23, norm_en_1993
  30/30, norm_en_1994 15/15, norm_en_1995 15/15, norm_en_1996 14/14, norm_en_1997 14/14, norm_en_1998
  21/21, norm_en_1999 19/19, norm_din_en_16798 30/30, norm_din_v_18599 20/20 (each includes that
  crate's pre-existing tests plus the 3 new DSL/OpText/store tests).
- `norm-plugin --lib`: 4/4 pass (`fifteen_family_apps_are_registered`, both din4108 host tests, the
  plugin-manifest sanity test) — confirms all 15 family apps (including the 2 left undone) still
  compile and register fine together.
- `cargo check -p norm-plugin --target wasm32-unknown-unknown`: clean (only pre-existing
  `semio-framework-plugin` warnings, unrelated to this change).

## Infra note

The shared `CARGO_TARGET_DIR` under this session's scratchpad was being hit by several sibling
Wave-3 agents at once (mathematical, protocol, sourcing_curate, etc. all building concurrently into
the same generically-named `cargo-target` dir), which stalled cargo on its build-lock for 10+
minutes with 0% CPU. Fix: use a build-dir name unique to this task
(`scratchpad/norm-wave3-cargo-target`) instead of the generic `cargo-target` name every sibling
agent was independently guessing.
