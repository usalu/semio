# 🧬 W-C — Four in-place update mutations for the FEM 2D artifact (2026-09-16)

Schema-first addition of `ReplaceNode`, `ReplaceLoad`, `ChangeLoadCaseName` and `ReplaceCombination`
to `semio-s-artifact-fem-2d`, mirroring `🔁️replace-support` and `⚖️change-load-case-self-weight` on
every surface those two appear on. The vocabulary goes from **25 to 29 kinds**.

Nothing under `…/🪆️subsets/🌐️any/✏️editor/**` was touched, and the 2d/3d twin crate
(`🗿️artifacts/🧊️3d`) was left alone — see §7.

## 1. The contract as shipped

| Kind | Folder | Keyword / wire tag | Payload | Record |
|---|---|---|---|---|
| `ReplaceNode` | `🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔁️replace-node/` | `replace-node` / `replaceNode` | `{ id: String, new_node: FemNode }` | `ReplacedNode` |
| `ReplaceLoad` | `🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-load/` | `replace-load` / `replaceLoad` | `{ case_id: String, load_id: String, new_load: Box<FemLoad> }` | `ReplacedLoad` |
| `ChangeLoadCaseName` | `🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🏷️change-load-case-name/` | `change-load-case-name` / `changeLoadCaseName` | `{ case_id: String, new_name: String }` | `ChangedLoadCaseName` |
| `ReplaceCombination` | `🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-combination/` | `replace-combination` / `replaceCombination` | `{ id: String, new_combination: FemCombination }` | `ReplacedCombination` |

The four variants are appended, in that order, at the END of `Fem2dMutation` and of `KINDS`.

### ⚠️ Two deliberate deviations from the brief

1. **`ReplaceLoad::new_load` is `Box<FemLoad>`, not a bare `crate::FemLoad`.** A bare enum field does
   not compile: the value codec implements `DslField` for `Box<T>` but not for a `DslEnum`, so
   `dsl::DslRecord` fails with `the trait bound FemLoad: DslField is not satisfied`. The field also
   carries `#[dsl(statements)]`. This is exactly what `AddLoad::load` and `ReplaceElement::new_element`
   already do — see `…/➕️add-load/🦀️.rs:17`. **Callers must write `new_load: Box::new(load)`.** The
   JSON/wire shape is unaffected (`newLoad` is still the plain internally tagged load object).
   The editor wave already writes it this way (`✏️editor/🎮️commands/🩹️patch-load/🦀️.rs:53`) and the
   whole `--features component-app-assembly --tests` check is green against these leaves, so the four
   `🩹️patch-*` commands other agents wrote compile unchanged.
2. **`ChangeLoadCaseName`'s no-op inverse is ONE step, not empty.** The brief says "warned no-op with
   empty inverse (mirror how `change-load-case-self-weight` treats an unchanged value)" and then
   "Inverse = the old name". Those two halves disagree: `change-load-case-self-weight`'s inverse is
   BASE-derived and always emits exactly one step whenever the case exists, including for its no-op
   branch. I mirrored the precedent literally — empty **diff** + `mutation.no-op` Warning, and a
   one-step inverse carrying the name `base` holds. An inverse that collapsed to `Vec::new()` on a
   no-op would also break `protocol`'s inverse law for that branch.

### Guards, in the order each diff builder runs them

* `replace-node` — target-missing (Error) → `identity_matches` (Fatal) → `node_geometry` (Fatal) → no-op (Warning). Moving a referenced node is allowed on purpose: every referrer addresses it by the id a replacement may not change.
* `replace-load` — case target-missing → load target-missing → `identity_matches` on `load_id` → `load_reference` (the same per-variant resolution `add-load` runs) → `load_magnitudes` → no-op.
* `change-load-case-name` — case target-missing (Error) → no-op (Warning). No Fatal branch at all.
* `replace-combination` — target-missing → `identity_matches` → `combination_term_references` → `combination_factors` → no-op.

> 🪞️ **Followed up.** `combination_term_references` as this wave first wrote it resolved a term
> against `base.combinations`, and under `replace-` the selected combination IS in the base — so a
> self-weighting term applied. A follow-up wave threaded the identity into the guard and made that
> branch `mutation.invariant` (Fatal), adding a fourth refusal vector
> (`reject-replace-combination-4` / `🧿️self-term-0f54d1`). See
> `📓️fix-2026-09-16-combination-self-reference.md`. The load lane therefore now carries **19**
> refusal vectors, not 18, and `REFUSALS["replace-combination"]` is 4.

Three guards are NEW and live with the others in `…/🌐️any/🧬️schema/🧬️mutations/🦀️.rs` `mod guards`:
`combination_term_references`, `combination_factors`, `load_magnitudes`. `create-combination`'s diff
builder was refactored to call the first two, so the `create-`/`replace-` twins now provably agree
(that was the whole point of hoisting the guard rather than transcribing it).

## 2. Files added

### Leaves (4 × 5 files)

```
🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🔁️replace-node/{🦀️.rs,🔺️diff/🦀️.rs,↩️inverse/🦀️.rs,🔣️.json,🧬️schema/🔣️.json}
🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-load/{同上}
🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🏷️change-load-case-name/{同上}
🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-combination/{同上}
```

### Scenarios — 18 leaf test modules + 18 fixture bundles

Naming follows the existing `<emoji><short-slug>-<6hex>` convention, where the six hex digits are
`sha1(<full-kebab-module-name>)[:6]` (verified against `🔩️pins-the-left-base-7891ec` and
`🪪️denies-rename-63ec90`).

| Kind | Scenario id | Directory | Model | Outcome |
|---|---|---|---|---|
| replace-node | `spec-vector-replace-node` | `🕹️raises-the-ridge-e53b00` | timber portal frame | applied |
| replace-node | `frame-vector-replace-node` | `📍️widens-the-canopy-553d69` | braced steel frame | applied |
| replace-node | `reject-replace-node-1` | `⛔️rejects-a-missing-334de5` | steel | `mutation.target-missing` Error |
| replace-node | `reject-replace-node-2` | `🪪️denies-rename-e69720` | steel | `mutation.id-mismatch` Fatal |
| replace-load | `spec-vector-replace-load` | `🏋️retunes-the-live-f6fd49` | timber | applied |
| replace-load | `frame-vector-replace-load` | `💨️strengthens-the-wind-21b88a` | steel | applied |
| replace-load | `reject-replace-load-1` | `⛔️rejects-a-missing-7a1e1f` | steel | target-missing (`lw9`) |
| replace-load | `reject-replace-load-2` | `🪪️denies-rename-732de0` | steel | id-mismatch |
| replace-load | `reject-replace-load-3` | `👻️dangling-node-b95678` | steel | target-missing (`n9`) |
| change-load-case-name | `spec-vector-change-load-case-name` | `🏷️renames-the-live-7dce39` | timber | applied |
| change-load-case-name | `frame-vector-change-load-case-name` | `✏️renames-the-wind-61757b` | steel | applied |
| change-load-case-name | `reject-change-load-case-name-1` | `⛔️rejects-a-missing-fc0343` | steel | target-missing |
| change-load-case-name | `reject-change-load-case-name-2` | `🔁️keeps-the-name-4fa89f` | steel | applied + `mutation.no-op` Warning |
| replace-combination | `spec-vector-replace-combination` | `⚖️reweights-the-uls-8b17b7` | timber | applied |
| replace-combination | `frame-vector-replace-combination` | `🔗️adds-a-wind-term-6525d5` | steel | applied |
| replace-combination | `reject-replace-combination-1` | `⛔️rejects-a-missing-b32308` | steel | target-missing |
| replace-combination | `reject-replace-combination-2` | `🪪️denies-rename-a96d3c` | steel | id-mismatch |
| replace-combination | `reject-replace-combination-3` | `👻️dangling-case-65b8a8` | steel | target-missing (`seismic`) |

Every `spec-vector-*` happy path is authored on the **derived timber portal frame demo model** (the
same `🏗️timber-portal-frame.snapshot.json` all twelve fem2d cases share, byte-identical across all
copies); every `frame-vector-*` and every refusal is authored on the two-storey braced steel frame.
Each bundle carries `📸️snapshot/{⬅️before,➡️after}/🔣️.json`, `🦠️mutation/🔣️.json`,
`🎯️outcome/🔣️.json` and either `🔺️diff/🔣️.json` or `🔺️diff/🚫️.absent`.

**No non-finite scenario exists for `node_geometry`, `load_magnitudes` or `combination_factors`** —
JSON cannot carry NaN or Infinity, so those branches are covered by Rust unit tests in the aggregate
`🧪️tests/🔬️unit` instead (`replace_load_non_finite_magnitude_is_fatal`,
`replace_combination_non_finite_factor_is_fatal`). This matches how `create-node` already handles it.

### Carrier fixtures (4 new pairs)

`🕸️mesh/🧫️fixtures/🔁️replace-node/`, `🏋️load/🧫️fixtures/{🔁️replace-load,🏷️change-load-case-name,🔁️replace-combination}/`
each with `⏮️before.json` + `⏭️after.json`, plus their `carrier-<kind>` entries in
`🌐️any/🔮️oracles/🔣️.json` `fixtureManifests` (34 → 38).

## 3. Files modified

**Aggregate (`🪆️subsets/🌐️any/🧬️schema/🧬️mutations/`)**
* `🦀️.rs` — 4 enum variants, 4 leaf imports, 4 `KINDS` entries, 3 new guards, `FemCombination` import.
* `🟦️.ts` — 4 interfaces + 4 union arms.
* `🔣️.json` — 4 `oneOf` `$ref`s, "twenty-five" → "twenty-nine".
* `🧪️tests/🔬️unit/🦀️.rs` — kinds count 25 → 29; 4 round-trip tests; 4 op-text round-trips; 4 `assert_missing_target_is_error` laws; 6 guard laws (two rename laws, the create/replace dangling-term agreement, two non-finite Fatals, the rename no-op).
* `📖️.grammar.semio` — **unchanged**: it declares a text dialect, not a kind enumeration, and its conformance test only checks the dialect header.

**Crate root** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🦀️.rs` — four `#[path = "."] pub mod …` blocks
mounting component/diff/inverse plus the 18 `#[cfg(test)] #[path]` scenario modules.

**Leaf siblings** `🏋️load/🧬️schema/🧬️mutations/🔗️create-combination/🔺️diff/🦀️.rs` — now calls the two
hoisted guards instead of an inline transcription.

**Subset differential cases** (`🕸️mesh` and `🏋️load`, plus their `🌐️any/🧪️tests/*-any-*` twins):
`🦀️.rs` (KINDS, COMMITTED, REFUSED, `vector()` arms, `touches_one`), `🥒️.feature` (rows in the
`@id-mutate`, `@id-inverse`, `@id-spec-vector`, `@id-frame-vector` and `@id-reject` Outlines), `🐍️.py`
(KINDS, REFUSALS, `COLLECTIONS["combination"]` gains `newCombination`, `apply_mutation`,
`inverse_mutation`, `touches_one`, new `check_load`/`check_combination` bounds).

The ten `🐍️.py` copies are byte-identical templates apart from KINDS/REFUSALS/docstring/URI prefix,
so **all ten** got the generic engine changes — keeping them in sync is what makes the template
readable, and the four subsets that do not own the new kinds simply never dispatch to them.

**Catalogs**
* `🕸️mesh/🔮️oracles/🔣️.json` — `replace-node` vector (4 scenarios) + kind.
* `🏋️load/🔮️oracles/🔣️.json` — three vectors (14 scenarios) + kinds.
* `🌐️any/🔮️oracles/🔣️.json` — `fem2d-1-any-mesh`/`fem2d-1-any-load` kind lists, four `mutationManifests.mutations` entries (25 → 29), four `carrier-*` `fixtureManifests`, prose counts.

**Generator** `🌐️any/🏭️generator/` — `📜️script.ts` `CARRIER_FIXTURES`, `🧩️json/🦀️.rs` `KINDS` + `apply()`,
`🧩️json/🏗️generate/🦀️.rs` `fixture_directory()`.

**Repo library** — `🔣️taxonomy.json` (`members-of-schema` and `members-of-fixtures` `memberNames` gain
the four directory names), `🔣️schema-catalog.json` and `📓️schema-catalog.md` regenerated with the
sanctioned `bun ./📜️script.ts schema generate` / `schema docs`.

## 4. Test results

| Run | Result |
|---|---|
| `cargo check -p semio-s-artifact-fem-2d --tests --message-format short` | **0 errors** |
| `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --tests --message-format short` | **0 errors** (the editor other agents are building compiles as of this run) |
| `cargo nextest run … --profile fundamental -- replace_node replace_load change_load_case_name replace_combination kinds semio_grammar mutate_fem2d` | **131 tests run: 131 passed**, 901 skipped → `🗑️generated/w-c-nextest-mutations.txt` |
| `bun nx run @semio-tech/fem-2d-rs:test --skip-nx-cache` | **1038 tests run: 1038 passed, 1 skipped**, `NX_EXIT=0` → `🗑️generated/w-c-nx-test.txt` |

Nothing is left red.

## 5. Filtered runs

```
cargo nextest run -p semio-s-artifact-fem-2d --profile fundamental --no-fail-fast \
  --status-level fail --final-status-level fail -- \
  replace_node replace_load change_load_case_name replace_combination kinds semio_grammar mutate_fem2d
→ Summary [0.832s] 131 tests run: 131 passed, 901 skipped
```

Every one of the 18 new leaf scenarios (each 4–8 assertions), the four new aggregate round-trip
tests, the four `assert_missing_target_is_error` laws, the six new guard laws, the op-text matrix and
`every_mutation_registers_a_semantic_descriptor` (now 29) are inside those 131.

**Python differential side, replayed standalone** (the committed twin driven directly, harness
stubbed, no Rust linked): all 18 new committed vectors reproduce the declared `after`, the declared
`🎯️outcome` code/level/address, `observable`, `touches_one` and `restores`; and all four
`mutate-<kind>` / `inverse-<kind>` payloads hold on the derived timber portal frame. So both halves
of each new differential row are green independently before the generated host ever runs them.

**Structural cross-checks** (scripted, over the repo as it now stands):
* 29 aggregate `oneOf` `$ref`s ↔ 29 leaf payload schemas ↔ 29 leaf descriptors ↔ 29 `KINDS` ↔ 29
  `mutationManifests.mutations` — all four sets equal, no symmetric difference.
* The four subset catalogs' union of `kinds` equals `KINDS` exactly.
* 117 catalogued scenarios: every one has both a `🧪️tests/<scenario>/🦀️.rs` and a fixture bundle, and
  no fixture directory on disk is uncatalogued.
* 667 `include_str!` targets across every fem2d mutation adapter and leaf test resolve.
* 118 leaf test directories on disk; all 118 (minus the two binary-facet suites mounted elsewhere)
  are mounted in the crate root.
* All ten `🐍️.py` adapters byte-compile; all ten subset `🦀️.rs` adapters parse under rustfmt (they
  are built by the generated test host, not by `cargo check -p semio-s-artifact-fem-2d`).

## 6. Left red, and why

**Nothing.** The final `nx` run is 1038/1038.

Worth recording for the next person, because it cost an hour of confusion: an intermediate
whole-crate run under a saturated build fleet failed two tests —
`analyses::tests::assembly_job_one_fuel_steps_stay_below_eight_milliseconds` (8924 µs against an
8000 µs budget) and `mesh::tests::mesh_job_large_boundary_never_runs_to_completion_in_one_step`
(13.1 ms against 8 ms). Both are **wall-clock budget assertions in the fem ENGINE**
(`✏️s/🔨️modules/🏗️fem/⚙️engine/{🧮️analyses,🕸️mesh}/🧪️tests/🔬️unit/🦀️.rs`), they read nothing from the
mutation vocabulary, and both pass once the machine is not compiling four crates at once. They are
load-sensitive, not broken — but they will keep flapping under a full fleet.

## 7. Not done, on purpose

* **The 2d/3d twin crate is untouched.** I looked for a conformance test that compares the two kind
  lists and there is none: the only 2d↔3d couplings are `FemDof`/`FemAnalysisSettings` re-exports and
  prose cross-references in docstrings (`🗿️artifacts/🧊️3d/…/🧬️mutations/🟦️.ts:11,218`). `fem3d` keeps
  its own 25-kind vocabulary and nothing asserts the two must match.
* **The carrier generator's own output drifted before this wave.** `bun …/🏭️generator/📜️script.ts carrier`
  now emits compact single-line JSON, while the 22 committed pairs (and the `sha256`/`bytes` their
  manifests record) are pretty-printed with sorted keys — a `pack::json::to_string` change that
  predates this ticket. Running the generator therefore rewrote all 22 existing pairs. I restored
  those 22 pairs from the index and authored the four NEW pairs in the committed serializer's own
  shape (renderer verified byte-for-byte against `⚪️create-node/⏮️before.json`), so the new manifests
  are consistent with the old ones and no unrelated fixture moved. **The drift is real and still
  open** — regenerating the whole carrier corpus plus its 26 manifest digests is its own change.
