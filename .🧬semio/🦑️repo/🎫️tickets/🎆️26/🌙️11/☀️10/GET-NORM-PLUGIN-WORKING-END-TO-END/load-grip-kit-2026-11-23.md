# Load-grip kit convention and the en1999 package test crate (2026-11-23/24)

Norm root: `✏️s/🔌️plugins/📕️norm/`. Run script of the checks: `load-grip-kit-chain.sh` (sequential, retries on other writers' compile breaks).

## 1. Shared load-grip kit

### Cause
The framework reads a `ScalarAlong` grip at the absolute parameter `rest_parameter = anchor . axis` (accessibility tree, keyboard, value field, `HandleDragTool`). `handle_kit::load_arrow` built the map relative to the grip (`Affine { base: value, k }`, limits `(-reach, 100)`), so at the rest parameter it wrote `value + k * (anchor . axis)` (-742500 N instead of 2000 N for `purlin-1:load-G:n`) and the limits did not contain the rest, so the typed entry clamped the field to the limit end (`nK = 0.0`).
`din18599` (`u-value` grip: `base = -rest/scale`, limits from `rest`) already followed the absolute convention; `en1994` has its own absolute `load_arrow`.

### Fix (shared, `🎬️structural-scene/🧰️projection-kit/🕹️handle-kit/🦀️.rs`)
- `load_arrow` is absolute: `rest = anchor . axis`, map `value + k * (s - rest)`, limits `(rest - reach, rest + 100)` (lower limit = glyph point = value 0).
- New `rest_of(kind, anchor)` (the one definition of the rest parameter of an along-axis or angular grip), used by `with_rest_in_limits` and `load_arrow`.
- New `rebased(handle)` converts a grip built relative to its anchor (map `value + k * s`, limits around 0) to the absolute parameter (base `- k * rest`, limits `+ rest`). Used by the en1999 gauge rack.
- Unit test `load_arrows_rest_at_their_value_in_the_absolute_parameter_of_the_framework` (`🧰️projection-kit/🧪️tests/🔬️unit/🦀️.rs`): three axes, signs and a zero value; value at rest equals the stored value, value at the lower limit is 0, `rest_of` agrees with `rest_parameter`.

### Users of the kit and what changed per family
| Family | Use | Change |
| --- | --- | --- |
| en1993 | `action_arrow` -> `load_arrow` (scene-members) | none in code; test `nested_action_rows_get_magnitude_grips_on_their_glyphs` drags from the rest parameter now |
| en1998 | `variables` `qk` grip | none in code; test `...grips on the glyphs` asserts the value at the rest parameter instead of `map.base == qk` |
| en1999 | `action_grips` (n, vz, my, udl) and the gauge rack | UDL grip overwrote the map with a relative base: now `q_k_line - k * rest`; gauge rack wraps each gauge in `rebased` (the next failure of the grip law after the load grip was `support-weld:gauge-hole-stagger`, a relative gauge whose limits did not contain the rest); tests read at `rest_of` |
| en1997 | own `Grip::build`, design-case kit, no `load_arrow` | **no conversion exists in the current file**: `Grip::build` only widens the limits to the rest (`limits.min(rest)/max(rest)`), maps are still relative (`base = current value`). The grip law passes with it (the typed entry inverts into the widened limits), but the value read at the rest parameter is not the field value. Not touched (other agent's file); `rebased` is the conversion if the owner wants the absolute values. `with_rest_in_limits` of the shared design-case forms is not redundant for the same reason (relative maps), so it stays. |
| en1994 | own absolute `load_arrow` | none |

Redundant workarounds removed: none were needed; the en1999 UDL override was the only per-family relative base on a kit grip and is now absolute.

### Changed files
- `🎬️structural-scene/🧰️projection-kit/🕹️handle-kit/🦀️.rs` (shared kit), `🎬️structural-scene/🧰️projection-kit/🧪️tests/🔬️unit/🦀️.rs` (kit test).
- en1999 scene: `🎬️scene/🏗️members/🦀️.rs` (UDL map, `handle_kit` import), `🎬️scene/🕹️handles/🎚️gauges/🦀️.rs` (`rebased`), `🎬️scene/🧪️tests/🔬️unit/🦀️.rs` (rest-parameter limits, `binding` -> `utilisation`, `Series`, section token), `🎬️scene/🧪️tests/🎨️visual-pass/🦀️.rs` (gauge test at the rest), `🎬️scene/🧪️tests/🔭️wave3/🦀️.rs` (section token, `Affine` fields).
- en1998 scene unit test, en1993 scene unit test (one assertion each, see table).
- No file of the en1997 agent, the schema-label agent or the mutation-vector agent was edited.

### Test lines
(`repo cargo --store=shared --configuration=prototype -- test -p <pkg> --offline ...` from the family dir; `--lib` for editors)
- kit: `semio-s-plugin-norm-structural-scene load_arrows`: `test result: ok. 1 passed; 0 failed`.
- en1999 editor lib (all, incl. `every_grip_of_every_entity_kind_...`): `ok. 30 passed; 0 failed; 1 ignored` (the ignored one is `latency_bench`, release-only). Before: `0 passed; 1 failed` at `purlin-1:load-G:n:magnitude`, then `support-weld:gauge-hole-stagger`.
- en1999 scene: `gauges_write_the_current_value_move_by_steps_and_the_dragged_document_re_evaluates`: `ok. 1 passed`; `nested_action_rows_get_magnitude_grips_on_their_glyphs` passes in the full run. The full scene run is `73 passed; 13 failed`; none of the 13 is a grip test or caused by this change (python oracle missing, `ring-1` missing from the default scene, golden bytes absent, HAZ/class/diagram content, `fireInsulations[].conductivity has no viewport grip`, drag budget 313 ms > 240 ms in a debug build). The scene test crate did not compile before (7 stale tests, fixed: `Series`, `SceneEntity.binding` -> `utilisation`, `SectionKind` Display, `Affine` move).
- en1997 editor lib (all): `ok. 30 passed; 0 failed; 1 ignored`.
- **Not run** (blocked, not caused by this change):
  - en1993 editor: `Rust crate semio-s-artifact-norm-en1993-editor exceeds its source boundary before the compiler: 33 of 32 files, 230225 of 262144 bytes`; en1993 scene: `8 of 32 files, 396338 of 262144 bytes`. The crate is unchanged against HEAD (only `Cargo.lock`), so the limit already blocks its tests; it needs a split by the family owner.
  - en1998 editor: does not compile (`ChangeBridge...`, `InsertJoint`, ... `: serde::Serialize is not satisfied`), the mutation payload derives in flight with the mutation-vector agent. en1998 `scene-structures`, en1993 `scene-members` and en1997 `scene` unit tests were queued behind the shared compiler and were not reached.
  - These families are covered by the kit unit test (same function, three axes and signs) and by their changed assertions, which were not executed.

## 2. en1999 package test crate (`semio-s-artifact-norm-en1999`)

### Compile (10 errors -> 0)
- `PlateElement.weld_process`/`WeldRun.direction` from `&str`: local matches in `📐️worked-example-vectors` and `🧵️weld-runs-and-outlines` (the typed enums have no `From<&str>`). The `"diagonal"` direction of `a_run_with_unknown_member_element_or_geometry_fails_with_a_remedy` cannot exist any more (typed enum), the run now uses `longitudinal`; the member, throat and geometry failures it asserts are unchanged.
- `export_sqlite`/`import_sqlite` take `&ArtifactSqliteSnapshotCodec` (4 call sites), `error.kind` of `OriginalSnapshotRefusal(String)` -> `error.0.contains("Canceled")` (`🚪️io/🪶️sqlite/📸️snapshot/🧪️tests`).
- `section_interaction_exponents(SectionKind::ExtrudedI)` and `shell_quality_q(FabricationClass::B)` in `🔬️resistance-worked-examples`.

### Tests: `99 passed; 23 failed` (0 passed of 0 before: did not compile)
Fixed besides the compile errors: `explanations_en_de_not_identical_except_numbers` (the fire insulation explanation was a symbol string identical in both languages: now `Insulation: ...` / `Bekleidung: ...`, `🎯️entity-checks`), `e2_12b_net_width...` (expected values rounded to 6 digits against a `1e-6` relative tolerance: exact values), `facet_diff_matches_rust` (`artifact` is no diff field), `facet_field_names_match_snapshot_json_schema` (schema uses `$defs`), `facet_mutations_match_kinds` (219 kinds counted from the schema folders, compared case- and separator-insensitively with TS/GraphQL/proto; written, **not re-run**), spec trace `A-61` / `the_full_showcase_alone_reaches_every_check_fragment` (the showcase fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` had no sheet profile: one row added, it is a test fixture, not an asset).

Still failing, with cause (none is a compile error):
- Environment, 6: `python_oracle...` x3, `third_party_cross_check...`, `example_snapshot_validates_against_json_schema`, `third_party_libraries...` (`no Python interpreter: SEMIO_PYTHON is unset ... run repo dependencies ensure`). Not run: that command downloads packages.
- SQLite native snapshot, 11 (`sqlite_snapshot_en1999_*`): `SQLite row column count`, `norm native text required`, `artifact identity is not owned by the declaring plugin`, `left: 25 right: 23`: the relational codec of `🚪️io/🪶️sqlite/📸️snapshot` and the independent contract fixture lag behind the model (new columns/rows) and the framework native-snapshot owner API; needs the owner of the codec.
- Evaluation content, 6: `compliant_example_passes` and `bundled_dsl_parses_and_evaluates_compliant` (the `aluminium-roof-purlin` evaluates with every member check `UnverifiedBasis`, so `complies()` is false; a registry/NDP verification state, `fails=[]`, worst u 0.914); `every_editable_leaf_influences_a_check` (`betaW`, `fillerAlloy`, `localLoadK`, `bearingLength`, `weldTJunction`); `e2_02_classification` (web with stress gradient: Class3, test says Class1); `the_channel_critical_moment...` (z_j); `a_run_haz_extent_override...` (strip loss not linear); `a_member_on_a_free_i_outline...` (V utilisation 0.0329 against 0.0387).
- `gate_parity_every_fail_remedy_clears_status`: `en1999.1-2.fire.fire-hot`. The beam of the asset fails at 20 degrees C (mu0 > 1), the theta remedy lowers u 1e6 -> 167 only. A clearing sequence needs the load remedy, which the editor remedy law forbids at u = 1e6 (every remedy must lower the check alone). The two laws conflict for this check; needs a decision of the remedy-law owner (not resolved here).

### Asset regenerate test
`regen_example_assets_when_env_set` with `REGEN_EN1999_ASSETS=1`: `ok. 1 passed`. It rewrote `aluminium-roof-purlin` and `noncompliant-multi-fail` (pack, DSL, JSON) and a byte diff of the whole `🖼️assets` folder against a copy taken before the run is empty: the assets are not stale. `aluminium-roof-purlin` untouched.

### Changed files (package tests)
`🧬️schema/🧪️tests/⚖️compliance/`: `📐️worked-example-vectors`, `🧵️weld-runs-and-outlines`, `🔬️resistance-worked-examples`, `🧬️facet-and-asset-parity`, `✅️compliance-gates` (assertion messages list the unverified checks); `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs`; engine `🧬️schema/💡️inferences/⚖️checks/🎯️entity-checks/🦀️.rs`; fixture `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`.

No `[DEBUG]` left. Logs and per-run scripts in `🗑️generated` of this task were deleted; `load-grip-kit-chain.sh` is kept.
