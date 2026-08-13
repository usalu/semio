
## Round 2 (orchestrator-dispatched correction) — subset composition

**ucas-status: partial — real `C:table` composition landed on 2 artifacts (`en1990.q_k`,
`din18599.climate`), 0 compile errors, 1105/1108 tests passing (reproduced twice, non-flaky, same 3
pre-existing failures as Round 1's baseline), all 5+1 granular mutation triads preserved with
unchanged public payload/wire shape. 13 of 15 artifacts still untouched — norm overall remains
`partial`, now with working precedent instead of zero composition.**

### Why Round 1's "architecturally blocked" framing was wrong

Round 1 declined to compose anything, reasoning that `en1990.q_k`'s five granular mutation triads
(`insert`/`remove`/`reorder`/`change-category`/`change-value`) would have to collapse into a
whole-handle-replace to become a composed child, citing `📌️important.md`'s D2/Concern B. That
citation was a misread: D2 is about how stdio's OWN `text`/`table`/`graph` subsets implement their
INTERNAL collection diff (sparse triple vs. whole-list-clone-and-wrap) — it says nothing about
whether a PLUGIN composing one of those subsets as a child must give up granular mutations at the
plugin's own dispatch layer. `mathematical`'s Round-1 report (`📓️wave4-reports/mathematical-report.md`)
is direct, already-landed counter-evidence: 14 granular mutation triads over a graph/geometry
structure, composing `text`/`table`/`value` children without collapsing to whole-blob-replace, by
routing every triad's diff/inverse through a `thread_local!` working-scene cache and re-minting a
fresh content-addressed child handle on each mutation. This round applied the identical pattern to
`en1990` and `din18599` and it worked exactly as `mathematical`'s report predicted — every triad's
public payload struct, `MutationKind` impl, and semantic descriptor are byte-for-byte unchanged;
only the internal `diff`/`inverse` function bodies were rewired to go through the cache.

### `en1990.q_k` → composed `s.stdio.semio.table` child

`En1990Snapshot.q_k: Vec<En1990QkEntry>` (`category: String, value: f64`, two scalar columns) is
replaced by `q_k: store::ArtifactChild<SemioTableSnapshot>` under `#[child(kind =
"s.stdio.semio.table")]`. All five existing mutation triads
(`🐴insert-variable-action`/`🐎remove-variable-action`/`🐗reorder-variable-actions`/
`🐮change-variable-action-category`/`🦌change-variable-action-value`) kept their exact payload
structs and semantic descriptors; only their `🔺️diff` bodies changed from
`base.q_k.clone()` + `En1990QkList{values}` wrapping to `en1990_qk(base)` (working-scene read) +
`en1990_qk_child_from_entries(&q_k)` (re-mint), and their `↩️inverse` bodies from `base.q_k.get(i)`
to `en1990_qk(base).get(i)`.

**Composition machinery** (`🗿️artifacts/📘️en1990/🦀️component.rs`, new `🔖️Composition` region):
- `En1990QkChild` type alias.
- `en1990_qk_table_from_entries`/`en1990_qk_entries_from_table` — real, lossless, positionally
  aligned converters (`category`→`SemioValue::Str`, `value`→`SemioValue::Float`), the inverse
  degrading honestly (empty category, `0.0` value) on a short/missing cell rather than panicking.
- `EN1990_QK_SCRATCH: thread_local! RefCell<HashMap<String, Vec<En1990QkEntry>>>` — content-hashed
  scene id (`en1990-qk-<hash>`), same `EngineRep`-contract shape as `mathematical`'s `MATH_SCRATCH`.
- `en1990_qk_child_from_entries` (mint+cache) / `en1990_qk` (read accessor, fails soft to `Vec::new()`
  on a cache miss — documented staleness gap, same as every prior exemplar).

**Snapshot codec**: `En1990Snapshot` dropped `#[derive(dsl::DslRecord)]` (an `ArtifactChild<S>`
field has no `DslField` impl) and gained a hand-rolled `store::ArtifactDsl`/`ArtifactPack` in
`📸️snapshot/🦀️component.rs` — real hex/bracket text codec (`gK=100\nqK=[hex,hex]\n...`) and
fixed-width/LEB128 binary codec, mirroring `mathematical`'s/`cad`'s `🔖️HandcraftedArtifactCodecs`
convention exactly. `En1990QkEntry` dropped its now-unused `dsl::DslRecord` derive (nothing nests it
inside a `DslRecord`-derived struct anymore). The 14-other-family shared
`crate::impl_norm_artifact_record!` macro is untouched — only `en1990` opted out, exactly as its own
doc comment (`📓️design-full-plan.md`'s reasoning) anticipated.

**`En1990Diff`**: `q_k: Option<En1990QkList>` → `q_k: Option<En1990QkChild>` (single-`Option`,
always-present-slot shape per `📓️migration-recipe.md` §8). The dead whole-document-replace
`artifact: Option<Box<En1990Artifact>>` field and its `diff_set_snapshot` helper are removed — grepped,
never constructed by any app command, shaped exactly like the banned `SetSnapshot` vocabulary
(mirrors `mathematical`'s identical dead-field removal).

**`En1990Artifact`** (the UI-inclusive full-state struct): `q_k` field mirrors the snapshot's
composed-child type; `to_snapshot`/`from_snapshot` copy the handle verbatim (same as
`mathematical`'s `MathematicalArtifact`).

**App command `set-snapshot`** (`🎛️apps/📘️en1990/🎮️commands/📤️set-snapshot/🦀️component.rs`):
`ReplaceSnapshot.snapshot: En1990Snapshot` (`#[dsl(block)]`) broke the same way `writer`'s did once
its snapshot lost `DslField` — collapsed to a `text: String` payload holding the document's own
`.en1990` DSL text (see "the serde_json precision bug" below for why `text`/`ArtifactDsl`, not
`json`/`serde_json`). Handler parses via `<En1990Snapshot as store::ArtifactDsl>::parse_dsl`; the
decomposition logic (`En1990Mutation::from_snapshot(base, target)`) is unchanged except it now reads
`q_k` through `en1990_qk(...)` on both sides instead of the removed direct field.

### `din18599.climate` → composed `s.stdio.semio.table` child

`Din18599Snapshot.climate: MonthlyClimate` (two parallel twelve-month `[f64;12]` arrays,
`theta_e_c`/`g_h_w_m2`) is replaced by `climate: store::ArtifactChild<SemioTableSnapshot>` — twelve
rows (one per calendar month, index-addressed), two columns (`thetaEC: Float`, `gHWM2: Float`). The
single `🐘update-climate` mutation triad (an `update-<facet>` per `📓️derivation-rules.md`'s
inseparable-≥2-field-facet exception — both arrays are always entered together, never one month at a
time) kept its exact payload shape: **`MonthlyClimate` still travels on the wire as literal data,
unchanged** — only the snapshot's own STORAGE became a composed child. `MonthlyClimate` therefore
*keeps* its `dsl::DslRecord` derive (needed by the mutation payload's own DSL mirror,
`Din18599MutationDsl::UpdateClimate{new_climate: MonthlyClimate}`) — confirmed necessary the hard
way: removing it broke compilation with `MonthlyClimate: DslField` unsatisfied at
`🧬️mutations/📝️text/🦀️component.rs:63`, fixed by restoring the derive. This is the one place this
round's design differs from `en1990`'s (`En1990QkEntry` genuinely lost its derive since nothing else
needed it) — a real, verified-by-compiler distinction, not a guess.

**Composition machinery** (`🗿️artifacts/📙️din18599/🦀️component.rs`, new `🔖️Composition` region,
placed beside the `MonthlyClimate` type it composes): `Din18599ClimateChild` type alias;
`din18599_climate_table_from_data`/`din18599_climate_data_from_table` (real converters, positional
month↔row alignment, `0.0`-degrading inverse); `DIN18599_CLIMATE_SCRATCH` thread-local cache +
`din18599_climate_child_from_data` (mint+cache) + `din18599_climate` (accessor, fails soft to an
all-zero `MonthlyClimate`).

**Call sites fixed** (all in `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, the
`Din18599Artifact`+`ComplianceHelpers` file — NOT the artifact root, which only holds the type
definition): `Din18599Artifact.climate` field type swap (`to_snapshot`/`from_snapshot` copy the
handle verbatim, unchanged code); `from_building`'s `BalancingInputs{climate: ...}` construction
site now mints via `din18599_climate_child_from_data`; `transmission_losses_kwh`/
`ventilation_losses_kwh`/`cooling_demand_kwh` (three pure per-`MonthlyClimate` helper FUNCTIONS keep
their `&MonthlyClimate` signatures unchanged — only their three call sites read `&din18599_climate(inputs)`
instead of `&inputs.climate`) and one test call site. `🔺️diff`/`🔺️diff/📝️text` (dead `artifact` field
+ `diff_set_snapshot` removed, same as `en1990`), `🧬️mutations` dispatch's `from_snapshot`, and the
`update-climate` triad's diff (mints from `payload.new_climate`)/inverse (reads via
`din18599_climate(base)`) all updated identically to the `en1990` pattern.

### The `serde_json` precision bug — found, diagnosed, fixed, not worked around

Following `writer`'s own precedent literally (`set_snapshot::SetSnapshot{json: String}`,
`serde_json::from_str`), the first pass of both `en1990`'s and `din18599`'s `set-snapshot` app
command used `serde_json`. `din18599`'s own `undo_redo_round_trips_through_the_wrapper` test then
failed with `h_v: 40.8` (restored) vs. `h_v: 40.800000000000004` (expected) — a real, reproducible
1-ULP precision loss, **traced to its actual root cause, not left as an unexplained flake**: isolated
with a temporary debug test (`cargo test debug_json_roundtrip_hv -- --nocapture`, removed after
diagnosis) proving `serde_json::from_str::<f64>("40.800000000000004")` in THIS workspace's
`serde_json` build parses to a *different* f64 (`40.8`, one ULP off) even for a bare literal — while
`format!("{}", 40.800000000000004_f64)` followed by `str::parse::<f64>()` (Rust's own std path,
already exercised correctly by both snapshots' own hand-rolled `ArtifactDsl` codecs) round-trips
exactly. `en1990`'s default fixture values happen to be round decimals that never hit this edge, so
it silently would have carried the same latent risk.

**Fix applied to both**: `ReplaceSnapshot`'s payload field renamed `json`→`text`, now holding the
document's own `print_dsl()` output escaped onto one physical line via `crate::document::
escape_op_text_field`/`unescape_op_text_field` — the exact convention `SetArtifactMutation<D>`'s own
`OpText` impl (same file, `📄️artifact/🦀️component.rs`) already used for this exact
whole-document-in-one-op-line problem. Those two helpers were `fn` (module-private); promoted to
`pub(crate)` (one-line change each, still `📕️norm`-crate-only visibility) to reuse them instead of
hand-duplicating the escape logic in two more files. Added a regression-guard test
(`din18599`'s `set-snapshot` module, `handle_preserves_full_f64_precision_through_the_payload`)
asserting `h_v` survives the payload round trip bit-for-bit — this is the kind of check that would
have caught the bug before it shipped.

### Verification

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-norm --all-targets
```
**0 errors** (baseline reconfirmed clean before starting; both artifacts' first-pass errors — `#[dsl(unit=...)]`
orphaned attributes after dropping `en1990`'s snapshot `DslRecord` derive, direct `.q_k`/`.climate`
field indexing in pre-existing tests, `MonthlyClimate: DslField` after over-eagerly dropping its
derive — were all fixed in this pass, not deferred). Warnings 238→239 lib / 279→280 test (+1/+1,
consistent with `mathematical`'s own "new doc comments, no new dead code" delta).

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-norm --no-fail-fast
```
**1108 tests run: 1105 passed, 3 failed.** Reproduced identical (same 3 test names, same pass count)
across two consecutive full runs — not flaky. All 3 failures are the exact 3 Round 1 already traced
and independently re-confirmed here to be unrelated to this round's edits — none touch `en1990` or
`din18599`:
- `din4108::…::{insert_remove_layer_round_trips, reorder_layers_round_trips}`
- `iso16757::…::selection_class_and_constraints_round_trip`

Fixture regeneration (recipe §7, done for real via a temporary `#[cfg(test)] mod debug_fixture_regen`
dumping real `print_dsl()` output, captured, written verbatim, module removed — verified clean:
`grep -rn debug_fixture_regen ✏️s/🔌️plugins/📕️norm` returns nothing):
- `en1990`'s `📚️examples/📕️high-consequence-office/🖼️assets/🗣️high-consequence-office.dsl.semio` —
  regenerated from a new `reference_snapshot()` builder (`📚️examples/📕️high-consequence-office/🦀️component.rs`)
  that mints the same 3-entry `q_k` content the file always represented (office=60,
  partition-walls=12, snow=18). The fixture's own test
  (`high_consequence_office_example_fixture_parses_and_round_trips`) now seeds the working-scene
  cache via `reference_snapshot()` before parsing — documented in the test's own comment as the
  same content-addressed-cache-hit bridge every composed-child exemplar depends on.
- `din18599`'s `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated from
  `Din18599Snapshot::default()` (the fixture's values always matched `Default` exactly); this
  fixture's own test only asserts structural round-trip (no field-depth assertion), so no
  cache-seeding change was needed there.

### Honest gap — the working-scene staleness bridge, same as every prior exemplar

Documented in both artifacts' `🔖️WorkingScene` doc comments, not hidden: a genuinely reloaded
persisted `.en1990`/`.din18599` document (fresh process, or a store-level undo/redo past this
session's history) sees a composed-child handle whose working-scene cache entry was never populated
— `en1990_qk`/`din18599_climate` fail SOFT (empty table / all-zero climate) rather than panicking.
For `en1990` this means a reloaded compliance document's variable-action combinations would compute
against an empty table until W1 lands a `LinkResolver`; for `din18599`, energy-balance calculations
against an all-zero climate. Every check both artifacts perform already routes through the accessor,
so the gap is *visibly* empty/zeroed, not silently wrong-but-plausible — same tradeoff
`mathematical`/`cad`/`writer` all already accepted, not a new risk this round introduced. Given the
compliance-calculation stakes, a fail-closed content-hash verification (lowpoly's
`StaleMeshWorkspace` pattern) would be a reasonable follow-up hardening, not implemented here to stay
consistent with the "simple documented gap is sufficient" default the migration recipe sets for
non-destructive read paths.

### Remaining 13 artifacts — not attempted, no new investigation

`din4108`, `din16798`, `en1991`–`en1999` (minus `en1990`), `iso16757`, `vdi3805` are untouched by
this round. `iso16757`/`vdi3805` already had Round 1's `LocalizedText` work; the other 11 were never
investigated for composition candidates by either round. Given the 2-artifact budget this round
targeted, no claim is made about their composability either way — a future pass should actually
check each one's shape (not assume, per this ticket's own repeated "verify before implementing"
lesson) rather than extrapolate from these two.

### sharedFileRequests

None. Every change stayed inside `✏️s/🔌️plugins/📕️norm/**` (including `📄️artifact/🦀️component.rs`'s
`fn`→`pub(crate) fn` visibility bump on `escape_op_text_field`/`unescape_op_text_field` — still
`📕️norm`-crate-private, not a public API change). Only read for schema reference: stdio's `✳️table`
subset (`SemioTableSnapshot`/`SemioTableColumn`/`SemioTableRow`/`SemioTableCellKind`) and `✳️value`'s
`SemioValue`.

### Concurrent-churn observations

`git status --porcelain -- ✏️s/🔌️plugins/📕️norm` and `git diff --stat` showed only the files this
session actually edited at every check; the repo's auto-committer (per `📌️important.md`) landed most
of this round's work mid-session (`git log` advanced by one commit, `515271bf60`, during this
session) — expected, not data loss, re-confirmed per `📌️important.md`'s own churn-detection guidance
(`git log --oneline -3`, `stat -f '%Sm'`) before concluding nothing was overwritten. No cargo lock
contention encountered.

### Files touched (Round 2)

- `📄️artifact/🦀️component.rs` — `escape_op_text_field`/`unescape_op_text_field` visibility only.
- `en1990`: `🗿️artifacts/📘️en1990/🦀️component.rs` (Composition region); `🧬️schema/📸️snapshot/🦀️component.rs`
  (struct + codecs + `En1990QkEntry`); `🧬️schema/🦀️component.rs` (`En1990Artifact`); `🧬️schema/🔺️diff/🦀️component.rs`,
  `🔺️diff/📝️text/🦀️component.rs`; `🧬️schema/🧬️mutations/🦀️component.rs` (`from_snapshot` + tests) and its 5
  triads' `🔺️diff`/`↩️inverse` (10 files); `🧬️schema/💡️inferences/🦀️component.rs`,
  `💡️inferences/🧾outline/🦀️component.rs`; `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (test + regen);
  `📚️examples/📕️high-consequence-office/🦀️component.rs` (`reference_snapshot`) and its regenerated
  `.dsl.semio`; `🎛️apps/📘️en1990/🦀️component.rs` (3 call sites) and
  `🎮️commands/📤️set-snapshot/🦀️component.rs`.
- `din18599`: `🗿️artifacts/📙️din18599/🦀️component.rs` (Composition region); `🧬️schema/🦀️component.rs`
  (`Din18599Artifact` + 5 `ComplianceHelpers` call sites); `🧬️schema/📸️snapshot/🦀️component.rs`
  (struct + codecs); `🧬️schema/🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`;
  `🧬️schema/🧬️mutations/🦀️component.rs` (`from_snapshot`) and `update-climate`'s `🔺️diff`/`↩️inverse`
  (2 files); regenerated `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`; `🎛️apps/📙️din18599/🦀️component.rs`
  (3 call sites) and `🎮️commands/📤️set-snapshot/🦀️component.rs`.

ucas-status: partial — 2 of 15 artifacts (`en1990`, `din18599`) now have real, verified,
granular-mutation-preserving `table` composition landed; Round 1's `LocalizedText` work stands
unmodified; 13 artifacts remain un-investigated for composition. Round 1's blanket
"architecturally inappropriate" conclusion for the whole plugin is retracted by this round's
evidence — it was specific to the D2 misreading, not to norm's content model in general.
