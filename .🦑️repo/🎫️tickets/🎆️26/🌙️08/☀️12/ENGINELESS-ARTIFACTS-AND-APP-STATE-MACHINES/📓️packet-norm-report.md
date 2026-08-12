# Packet: `📕️norm` engineless-artifacts migration

Scope: all 15 `⚙️engine` dirs under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine`.

## Structural verification (compiler-independent)

```
find ✏️s/🔌️plugins/📕️norm -path "*🗿️artifacts*" -name "⚙️engine" -type d   → 0
grep -rn "::engine::|standards::v1::engine|subsets::any::engine" ✏️s/🔌️plugins/📕️norm --include="*.rs"  → 0
```

Dangling `#[path]` check on `📦️glue.rs` (mandatory per coordinator):
```python
# scans every #[path="..."] in glue.rs, resolves relative to glue.rs's dir, flags missing targets
dangling: 0
```
Zero dangling mounts. Every `⚙️engine` mount block (`#[path=...]pub mod engine;`) and every
`pub mod engine { pub use super::standards::v1::engine::*; }` / `...subsets::any::engine::*`
(en1990's trap variant) shim was removed from `📦️glue.rs`, not merely orphaned.

Remaining `engine` string hits in the plugin are all unrelated: `EngineHandles` (framework type,
`semio_framework_plugin`) and my own docstrings ("relocated verbatim from the deleted `⚙️engine`").

## Per-artifact table

| Artifact | Engine deleted | glue.rs mount removed | glue.rs shim removed | Call sites updated | Deviation |
|---|---|---|---|---|---|
| din4108 | ✅ | ✅ | ✅ | ✅ | none |
| din16798 | ✅ | ✅ | ✅ | ✅ | none |
| en1990 | ✅ | ✅ | ✅ | ✅ | **module-path trap**: mounted under `subsets::any::engine` (not `v1::engine`) in both the mount block and the shim; `io_registry` used `standards::v1::subsets::any::engine::io_registry`. Both fixed at their actual location, not pattern-copied. `na_de`/`NaDe` relocated to schema — depended on by en1991/95/96/97/98/99 (all updated). `outline/component.rs` also called `engine::evaluate` directly (not just the shim) — fixed to fully-qualified `schema::inferences::evaluate`. |
| en1991 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; one dropped assertion (`"1991-1-6"` clause-family check in `evaluate_reaches_every_part_module`) caught and restored during review |
| en1992 | ✅ | ✅ | ✅ | ✅ | `cross-fem`-gated FEM helpers (`check_rc_beam_from_fem`, `max_beam_moment_knm`, `max_beam_shear_kn`) moved to schema (pure, feature-gated); `evaluate`'s fem branch in inferences imports them under the same `#[cfg(feature = "cross-fem")]` |
| en1993 | ✅ | ✅ | ✅ | ✅ | largest Eurocode engine (1155 LOC); `parse_fire_rating`/`parse_fatigue_method` initially duplicated into both schema and inferences during scripted extraction — caught (duplicate `pub fn` definitions) and removed from schema, kept only in inferences (where they're actually used) |
| en1994 | ✅ | ✅ | ✅ | ✅ | none |
| en1995 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; **duplicate `// #region 🔖️Session` marker** in source (cosmetic, two separate regions both named Session) |
| en1996 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; `part_2` submodule imports document types (`ExposureClass`, `MortarClass`) from `crate::artifacts::en1996::part_2` (kept as-is, self-contained) |
| en1997 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; duplicate `// #region 🔖️Session` marker (same pattern as en1995) |
| en1998 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; duplicate `// #region 🔖️Session` marker; largest region split (18 tests) |
| en1999 | ✅ | ✅ | ✅ | ✅ | depends on en1990's relocated `na_de`; duplicate `// #region 🔖️Session` marker |
| din18599 | ✅ | ✅ | ✅ | ✅ | **cross-artifact dependency on din4108 AND din16798** (`total_resistance`/`u_value_from_resistance`/`R_SI_WALL_M2K_W`/`R_SE_WALL_M2K_W` from din4108's relocated schema; `residential_ventilation_rate` from din16798's relocated schema) — processed last among the dependency chain so both were already relocated; `BalancingInputs` is a `pub type` alias for `Din18599Snapshot` at the artifact root, so `part_N::check(&BalancingInputs)` reads like a snapshot-level fn but is treated as a per-metric schema helper (mirrors every other artifact's `part_N::check`), while `balance_annual`/`evaluate` (the actual whole-report composition) went to inferences; artifact-root doc comment referencing the old `engine::reference_residential` path updated; `reference_wall_layers` changed from private to `pub` (now needed by inferences' test helper) |
| iso16757 | ✅ | ✅ | ✅ | ✅ | largest/most structurally distinct engine (1184 LOC): `part_1/2/4/5` are pure catalogue/geometry/dictionary algorithms → schema; the engine's own `pub mod io { catalogue_to_json/from_json, dictionary_to_json }` → **`🚪️io`** (JSON serializers, matching region-map rule 5) alongside the pre-existing `io_registry`; `evaluate`'s helper fns `clause()`/`check_count()` moved with it to inferences. A scripted-extraction bug briefly duplicated `evaluate()` in the inferences file (parts['inferences_block'] already included it, then I manually retyped it again) — caught via `grep "^pub fn evaluate"` returning 2 hits, fixed by removing the duplicate copy. |
| vdi3805 | ✅ | ✅ | ✅ | ✅ | **structurally unique**: 99 macro-generated `part_N::check(document: &Vdi3805Snapshot)` sheet-checks take the whole snapshot directly (not decomposed scalars like the Eurocodes), so the entire `SheetParts` macro + all part modules + `all_part_checks`/`evaluate` went to **inferences** rather than schema — this is the one artifact where the schema/inferences split is asymmetric (schema is small: text parse/serialize, `validate_structure`, `linear_map`, `diagnostics_to_report`, plus the shared `clause`/`na_check`/`pass_check`/`fail_check` helpers used by both schema's own `diagnostics_to_report` and inferences' per-sheet checks). Whole-artifact JSON (de)serializers → `🚪️io` (same pattern as iso16757). |

## Region → destination map (as applied)

1. `*Engine` struct → deleted outright, all 15 confirmed zero construction sites, zero `ArtifactEngine` trait/impl remaining anywhere in the plugin.
2. Derived compute from a snapshot (`evaluate`, `check_full_*`/`balance_annual`/vdi3805's 99 `part_N::check`) → `🧬️schema/💡️inferences/🦀️component.rs`, new `//#region 🔖️ComplianceReport` + `//#region 🧪️ComplianceReportTests`.
3. Pure helpers over document types (`part_N` compute modules, `na_de`/`AnnexParams`, `check_*_member`/`check_*_beam`-style non-snapshot composites) → `🧬️schema/🦀️component.rs`, new `//#region 🔖️ComplianceHelpers` + `//#region 🧪️ComplianceHelpersTests`.
4. Nothing in this plugin's engines returned `AppIo` or referenced an app type directly (the only "app-shaped" thing was the `NormFamily` binding itself, handled by rule 7).
5. `io_registry`/`ComposerEntry` → `🚪️io/🦀️component.rs`, new `//#region 🚪️IoRegistry`. Two artifacts (iso16757, vdi3805) additionally had ad-hoc whole-artifact JSON serializers inside `⚙️engine` — also relocated to `🚪️io` under `//#region 🚪️JsonSerializers`.
6. No `register*()` wiring existed inside any `⚙️engine` itself (only inside each artifact root's pre-existing `io_registry`, which was already dead code per the `declaration()` docstring — confirmed zero call sites of `register()` — left as-is, out of scope: it's not part of `⚙️engine`).
7. `NormFamily` impl + `Host` type alias → each artifact's `🎛️apps/<app>/🦀️component.rs`, new `//#region 🧩️ComplianceFamily`, calling `standards::v1::subsets::any::schema::inferences::evaluate` fully qualified. No app state machine was invented (per instructions); this is a plain trait impl, not new stateful machinery.
8. Tests split by what they now exercise; classification cross-checked programmatically (regex-extracted test-function count per artifact matched my manual before/after classification for all 15 — see per-artifact test-count table below).

## Test relocation counts (test **functions**, not raw assertions — see honesty note below)

| Artifact | Total tests (before) | → schema | → inferences | → io | → apps |
|---|---|---|---|---|---|
| din4108 | 18 | 15 | 2 | – | 1 |
| din16798 | 27 | 25 | 2 | – | 0 |
| en1990 | 13 | 11 | 2 | – | 0 |
| en1991 | 8 | 6 | 2 | – | 0 |
| en1992 | 20 | 17 | 3 | – | 0 |
| en1993 | 27 | 25 | 2 | – | 0 |
| en1994 | 12 | 10 | 2 | – | 0 |
| en1995 | 12 | 10 | 2 | – | 0 |
| en1996 | 11 | 9 | 2 | – | 0 |
| en1997 | 11 | 9 | 2 | – | 0 |
| en1998 | 18 | 16 | 2 | – | 0 |
| en1999 | 16 | 13 | 3 | – | 0 |
| din18599 | 18 | 16 | 2 | – | 0 |
| iso16757 | 34 | 30 | 1 | 2 | 1 |
| vdi3805 | 22 | 13 | 6 | 1 | 2 |

**Honesty note on verification method**: for en1993 through vdi3805 (10 artifacts), test-function
bodies were extracted programmatically via regex from the original `⚙️engine` file and spliced
byte-for-byte into destination files, so every `assert!`/`assert_eq!` inside is preserved by
construction (not retyped). For din4108/din16798/en1990/en1991/en1992 (5 artifacts), test bodies
were copied by hand via the Edit tool; **one dropped assertion was caught** (en1991,
`evaluate_reaches_every_part_module`'s `"1991-1-6"` check) and restored. I did not do a final
line-by-line `assert!` recount across all 15 after the last edits — the coordinator's stop-work
message landed before that pass. The test-**function**-count table above (before vs. sum of
after-buckets) is the verification I completed; it is necessary but not fully sufficient for
"every assertion survives" — treat the 10 scripted artifacts as high-confidence and the 5
hand-copied ones as verified only for the specific bug already found and fixed.

## Per-error attribution

No compiler was run to completion (see below), so there is no compiler output to attribute errors
from. Everything reported above is static/textual verification only.

## Compile status

**UNVERIFIED — build-lock contention, not attempted.** At the time of this report, 23 concurrent
`cargo` processes were already running on this machine (confirmed via `ps aux | grep cargo`),
consistent with the coordinator's report of ~35 concurrent processes across this wave's sessions.
Per the coordinator's explicit instruction, I did not start a `cargo check` and did not wait on any
queued/killed run. I have **not** verified `cargo check -p semio-s-plugin-norm --all-targets`
(`RUSTC_WRAPPER=""`) reaches a `Finished`/error verdict. I did not run it and am not reporting a
result for it — green or red — because it did not run.

## What I'm not certain of

- Full assertion-level (not just test-function-level) parity for the 5 hand-copied artifacts beyond
  the one bug already fixed.
- Whether any `use` import I added is unused (would show as a warning, not caught without rustc) or
  missing (would show as a hard error) — the `check_primary_energy`/`reference_wall_layers`
  visibility fix (private → `pub`) in din18599 and the iso16757 duplicate-`evaluate` fix are the two
  concrete defects caught by manual/grep review; there may be others of the same shape (an item used
  cross-file that was left non-`pub`, or an import brought in but never used) that only rustc would
  surface.
- `📦️glue.rs` shim removal was done for all 15, but I have not re-verified that no *other* file in
  this plugin (outside the ones I explicitly `grep`'d for `::engine::`) references a shim path like
  `crate::artifacts::<x>::engine::*` indirectly through a glob re-export I didn't trace.
