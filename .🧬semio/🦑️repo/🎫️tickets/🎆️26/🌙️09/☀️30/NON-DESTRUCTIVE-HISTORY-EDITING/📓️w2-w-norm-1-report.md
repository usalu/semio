# 📓️ W2-W-norm-1 — Wire-witness conversion for EN 1991 and EN 1990

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-norm-1, 2026-09-30. Contract: `📋️design.md` §6 and §11 (incl. the
negative-witness addendum), `🧭️plan.md` "W2-W brief", `📓️w2-s-report.md` F10/F11/F16/F17, `📓️w2-s-norm-report.md`,
`📓️w1-d-report.md` §8. Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991` and `…/⚖️en1990`.

## 1. Outcome

| Check (scope `--under` the artifact) | before | after |
|---|---|---|
| `schema mutation-payloads` EN 1991 | 81 findings: 80 `unwitnessed` leaves + the aggregate; 0/80 witnessed | **0 findings**, 80/80 fixtures clean, **80/80 leaves witnessed** |
| `schema mutation-payloads` EN 1990 | 31 findings: 30 `unwitnessed` + the aggregate; 0/30 witnessed | **0 findings**, 30/30 fixtures clean, **30/30 leaves witnessed** |
| `schema mutation-inputs` EN 1991 / EN 1990 | 0 / 0 | **0 / 0** (90/90 and 36/36 inputs) |
| `cargo test --lib` en1991 / en1990 (private target, `CARGO_INCREMENTAL=0`) | 80/80 / 140/142 | **160/160 / 140/140**, incl. `semio_payload_law_en1991_mutation` and `…_en1990_mutation` over the 110 committed ops and the 110 new vector cases (run before the §4.4 recoding; see §6) |
| `cargo check --target wasm32-wasip2` both crates | — | ok (before §4.4; see §6) |
| Case `🏋️mutate-en1991-1`: oracle / parity (`exhaustive`) | dead (vectors deleted by 652) | **oracle 161/161, parity 322/322 executed, 161/161 compared** |
| Case `⚖️mutate-en1990-1`: oracle / parity | dead (Python pointed at deleted vectors; the Rust file was no adapter) | **oracle 61/61, parity 122/122 executed, 61/61 compared** |
| Contract, scoped (`🧪️w2-w-norm-1-contract.ts`: both case contracts + registry breaches naming either subset) | `Unknown mutation catalog`, 32 unresolved fixtures, 81 `contribution-manifest-invalid`, 108 `mutation-without-fixture`, `unregistered-mutation-vocabulary` | **case contracts 0; vector registry 0; fixture law 2 (`mutation-without-fixture`) 0.** Remaining: 137 inventory findings from the stale runtime-inventory cache (§5.1) |
| EN 1990 failing lib tests | `default_snapshot_dsl_roundtrips`, `every_editable_leaf_changes_a_check…` | both root-caused and fixed (§3) |

Every leaf is witnessed by one committed specification vector (design §11 fixture-quintet form), not by payload-only
witnesses: the norm plugin's cases, contract gate (law 2 needs a fixture-backed vector per mutation) and Python reference are
built around vectors, and a vector witnesses the wire AND the semantics. The mutation file of every vector is the aggregate's
Rust wire, decoded generically (`FromValue`) by both adapters; no hand mapping exists anywhere.

## 2. What was built

### 2.1 Vectors (110)

`<✳️any>/🧫️fixtures/🧬️mutations/<leaf>/<scenario>/{🦠️mutation, 📸️snapshot/⬅️before, 📸️snapshot/➡️after, 🔺️diff, 🎯️outcome}/🔣️.json`,
one per leaf (80 EN 1991, 30 EN 1990), all `applied`, each moving the document.

- Payloads are hand-authored against the leaf schemas in `🧪️w2-w-norm-1-vectors.py` (realistic engineering values, valid
  enum spellings taken from the field meta, e.g. `formwork`, `bridge2`, `shopping`, `gr1b`, `tank`).
- Before-snapshot: EN 1991 = the committed `⚠️multi-fail-noncompliant` example (the one EN 1991 document with a member in every
  collection, incl. an impact case); EN 1990 = `accidental-seismic-compliant::reference_snapshot()` (every action collection
  populated).
- After/diff/outcome were written once by a temporary `[DEBUG]` generator test from production dispatch, then removed.
  They are judged independently by the Python reference (oracle phase) and held by the permanent Rust law (§2.2).
- Catalogs (`🔮️oracles/🔣️.json`): `mutationCatalogs[0].kinds` (declaration order) and `vectors` rewritten to the vectors;
  v2 manifest `payloadSchema` → `🧬️schema/🔣️.json`; EN 1990 manifest `outcomes` aligned with the leaf descriptors
  (27 rows), orphan `semanticKinds` removed; oracle rationale/`fixtureCoverage` counts updated (80 / 30).

### 2.2 Rust vector law

`🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs` (now wired in EN 1991, rewritten in EN 1990) defines `assert_vector`: the mutation file is
the canonical Rust wire (`store::os_store::test_support::assert_wire_witness`) of an op of the stated kind; before/after/diff are
canonical wires; production dispatch yields exactly the committed diff under the committed outcome and lands on AFTER; the
committed diff alone carries BEFORE→AFTER; applied ⇔ moved ⇔ non-empty inverse; the inverse restores BEFORE. One canonical case
per leaf at `<leaf>/🧪️tests/<scenario>/🦀️.rs` (required by the contract's vector registry) calls it with `include_str!`s.
82 superseded stub cases were deleted (48 EN 1991 `applies-<kind>` smoke stubs, 32 EN 1990 stubs incl. two pre-652 cases).

### 2.3 Feature cases

- `🏋️mutate-en1991-1` and `⚖️mutate-en1990-1`: features rewritten to the current vocabularies (80 / 30 kinds, mutate + inverse
  Examples = the catalog vectors) and to the real identity carriers (`🏢de-office-compliant` DSL + `📦️.pack.semio` twin;
  `🏢️high-consequence-office` DSL + `🎒️.pack.semio` twin).
- Python adapters (`🐍️.py`): the stale transcribed `KINDS`/`VECTORS` (pre-652, pointing at deleted vectors) are gone; both are
  now READ from the subset's own catalog. Still one top-level `adapter()` (the oracle-source-ownership contract).
- Rust adapters (`🦀️.rs`): EN 1990's file was a unit test, not an adapter — now a real subject adapter; EN 1991's 32-kind
  `include_str!` table is gone. Both register every kind of the aggregate's own `KINDS`, read the vector through the scenario's
  declared URIs (`ctx.step_fixture_uris()`), decode generically and assert the laws in role.
- `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts`: the pinned EN 1991 plan size 65 → 161.

### 2.4 Shared Python engine (`🔮️oracles/🏃️execution/🐍️.py`)

Index-addressed setters now resolve generically: `indexed_collection` (the collection the kind's noun names, else the one
collection whose every member carries the named field — `change-self-weight-assumed-gk` → `selfWeightElements`) and
`member_field` (the shallowest field of that name inside the member — `change-accidental-assumed-force` → the case's impact
record), in both `apply_verb` and `inverse_mutation`. Only former refusals change behaviour. A sibling landed the same
nested-path exclusion in `derived_view` concurrently (a case's own impact list was mistaken for a mirror of the case list);
I did not duplicate it.

## 3. The two failing EN 1990 lib tests — root cause

Both are test drift from commit `🚩️653` (2026-09-26), which changed `En1990Snapshot::default()` (altitude 0 → 1200 m, a third
variable action `Q-snow` with its effect) without updating two tests that pinned the old default. The model is correct.

1. `default_snapshot_dsl_roundtrips` asserted `variables.len() == 2`; the default now has 3. Fixed: the round trip keeps the exact
   equality and asserts the intent (multi-row action tables), not a magic count.
2. `every_editable_leaf_changes_a_check_when_perturbed_in_applicable_scope` perturbs `variables[Q-snow].category` `snow → snow_high`.
   Under the DE annex above 1000 m `resolve_psi_category` already maps `snow` to `snow_high` (DIN EN 1990/NA), so the
   perturbation was an identity and no check moved. Fixed: `snow → wind`, whose ψ differs from both snow rows under both annexes
   and at every altitude (the old mapping was also an identity under the EN annex).

## 4. Model defects the witnesses exposed (fixed, same crates)

1. **`En1991Diff` dropped 22 fields.** `apply_to_artifact`, `MutationDiff::apply` and `absorb` ignored `t_max`, `t_min`, `t_0`,
   `thermal_element_type`, `thermal_bridge_type`, `delta_t_m`, `storey_count`, the 12 fire fields, `assumed_bridge_lm3/lm4` and
   `bridge_load_group`: 22 mutation kinds were silent no-ops in production (the first generator run reported them `no-op`). The
   `🔖️Apply` region is now generated from the diff struct (`🧪️w2-w-norm-1-diff-apply.py`, 70 fields).
2. **`En1990Diff::absorb` dropped `structure_kind`, `altitude_m`, `k_fi_declared`, `bridge_sls`; both applies dropped
   `k_fi_declared`.** Coalesced diffs (history editing composes them) lost altitude and bridge-SLS edits. Fixed.
3. **Stale carriers.** EN 1991: the `🎒️.pack.semio` root twins could not decode (pre-652 field order) and the in-dir
   `📦️.pack.semio` twins encoded another document (height 12 m vs the DSL's 20 m); the `multi_fail_noncompliant()` subject still
   said `z = 12 m` after 653 hand-edited both DSLs to `z = 8 m`. Fixed the subject, regenerated both `📦️` twins with the crate's
   own `regenerate_example_assets_once` (DSLs byte-identical), deleted the two orphan `🎒️` twins. EN 1990: every example DSL
   wrote `importance-class:TEXT` where the printer emits `ENUM` since `ImportanceClass` became an enum, so the carrier could not
   re-emit byte-exactly; both `🎒️.pack.semio` twins were undecodable. All 8 DSLs canonicalised (only that header token changes)
   and both twins re-encoded from them.
4. **Refusal codes (design §11).** The 21 state-dependent refusals (index past the end in every EN 1991 insert/remove/indexed
   change and EN 1990 remove, and EN 1991's missing impact record) raised `mutation.invariant` (Fatal). They now raise
   `mutation.target-missing` at Error with the index as target — the store's level table for that code, the time-travel
   editor's "Target missing / Ziel fehlt" label, and the en1998 precedent (`🧪️w2-w-norm-1-target-missing.py`).

## 5. Open items (outside this WP or not verifiable here)

> Items 1–5 and the stray file in 8 were resolved in the follow-up. The §6 re-run owed after §4.4 is also done (see §8). Items 6 and 7 still stand.

1. **Runtime inventory cache is stale for all 15 norm subsets (2026-09-25).** The 137 remaining scoped findings
   (`runtime-only`/`manifest-only`/`mutation-outcome-mismatch`) compare the manifests with pre-652 dispatch. Not refreshed: the
   bridge (`🏭️bridge`) is a standalone workspace whose `Cargo.lock` differs from the root in 49 versions, i.e. a full native
   rebuild of the kernel and all 15 norm crates. Statically the manifests now equal the leaf descriptors that feed
   `DESCRIPTORS` (ids, variants, outcomes). Owner: norm plugin / whoever refreshes `test inventory` for norm.
2. **`binary-protocol-drift`** (both subsets): `💾️binary/📡️.protocol.semio` (+ `.ksy/.spicy/.abnf/.ts`) still lists the pre-652
   kinds; EN 1991's `OpBinary` is the op text as bytes (no tags), EN 1990's uses hand constants 0–29. Out of the witness scope.
3. **Descriptor `outcomeClasses` are not truthful**: EN 1991 claims `[applied, no-op, rejected]` for all 80 (scalar changes cannot
   be rejected, insert/remove cannot be no-ops); EN 1990 claims `[applied, no-op]` for all 30 (removes can be refused, inserts never
   no-op). Changing them regenerates the plugin-wide `📇️mutation-leaf-taxonomy-v1` fixture — routed, not done.
4. **EN 1990 subset declaration** (`🪆️subsets/🔣️.json`) still describes the pre-652 `combination`/`variable-actions` split with
   deleted kinds; the manifest keeps every row on `combination` (as W2-S-A left it). Needs an owner decision (single policy like
   EN 1991, or a real split).
5. **17 EN 1991 leaf directories lost their emoji** (`️change-snow-zone`, …; descriptor `emoji` is a bare U+FE0F). The new
   fixture directories mirror them because the vector registry keys fixtures by the leaf directory. Renaming touches the lib-root
   `#[path]`s, descriptors and the plugin taxonomy fixture — routed.
6. `bun ./📜️script.ts verify taxonomy report --scope …` aborts on a peer fixture (`📚️library/🧫️fixtures/📐️cad-draw-path-projection`
   digest mismatch), so it could not be run; the contract's layout/taxonomy rules report nothing for the new directories.
7. `🔮️oracle-source-ownership` (`bun ./📜️script.ts oracle-source`): 3 pass (incl. the EN 1991 native-host execution, 161/161),
   3 fail on peers: `🌍️mutate-en1997-1/🐍️.py` defines no `adapter()`, a new `🏭️vdi3805/…/⚖️compliance-vdi3805-1/🐍️.py` is not in
   the control fixture, and `.vscode/launch.json` lacks the `norm-js:test-oracle-source` entry.
8. Dead code seen, not touched: EN 1991 `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` is unwired; stray
   `…/🧬️schema/💡️inferences/_new_check_part1.rs.txt`.

## 6. Verification record (all foreground, gated)

- Lints: `schema mutation-payloads` / `mutation-inputs --under` each artifact → 0 / 0 (final state).
- `cargo test -p semio-s-artifact-norm-en1991 -p semio-s-artifact-norm-en1990 --lib` (target `target-nde-w2w-norm-1`) → 160/160,
  140/140 on the state before §4.4; `cargo check --target wasm32-wasip2` of both → ok on that state.
- After §4.4 (21 `fatal("mutation.invariant", …)` → `error("mutation.target-missing", …, [index])`): **WRITTEN BUT UNVERIFIED
  (peer build breaks).** Three re-runs (18:22–18:31) could not build: first a peer's in-flight edit in `semio-framework-plugin`
  (`🔌️plugin/🦀️.rs:23729`, `ViewWindowInstance` not found), then the kernel itself (`🏪️store/🦀️.rs:21036` and
  `📡️spr/📜️history/🦀️.rs:256` "takes 4 arguments but 3 were supplied", `🏪️store/♻️retirement/🦀️.rs:67`). The recoding uses the
  same `MutationOutcome::error(code, message, [index.to_string()])` form as en1998; no test in either crate or in the norm
  plugin asserts the old code or level, and every committed vector is `applied`, so no vector or feature row is affected.
  Re-run owed: `cargo test -p semio-s-artifact-norm-en1991 -p semio-s-artifact-norm-en1990 --lib` and the two `parity` runs.
- Feature phases (before §4.4): `oracle exhaustive` 161/161 and 61/61 (re-run after §4.4 too: unchanged, Python only);
  `parity exhaustive` 322/322 (161/161) and 122/122 (61/61).
- Scoped contract: `bun 🧪️w2-w-norm-1-contract.ts` → 0 case breaches; only the inventory findings of §5.1.

## 7. Files

- Ticket scripts: `🧪️w2-w-norm-1-vectors.py` (`plan|sources|stubs`), `🧪️w2-w-norm-1-diff-apply.py`,
  `🧪️w2-w-norm-1-target-missing.py`, `🧪️w2-w-norm-1-contract.ts`. Scratch: `🗑️generated/w2w-norm-1/`.
- Shared: `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`, `✏️s/🔌️plugins/📕️norm/🧪️tests/🔮️oracle-source-ownership/🟦️.ts`.
- EN 1991 (`🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/`): `🧫️fixtures/🧬️mutations/**` (80 bundles, 400 files, new);
  `🧬️schema/🧬️mutations/<leaf>/🧪️tests/<scenario>/🦀️.rs` (80 new, 48 stubs deleted); `🧬️schema/🧬️mutations/🦀️.rs` (wires
  `fixture_tests`); `🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`; `🧬️schema/🔺️diff/📝️text/🦀️.rs`; 16
  `🧬️schema/🧬️mutations/*/🔺️diff/🦀️.rs`; `🔮️oracles/🔣️.json`; `🧪️tests/🏋️mutate-en1991-1/{🥒️.feature,🐍️.py,🦀️.rs}`;
  `📚️examples/🧬️subjects/🦀️.rs`; `🖼️assets/*/*/📦️.pack.semio` (2, regenerated); `🖼️assets/*/🎒️.pack.semio` (2, deleted).
- EN 1990 (`⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/`): `🧫️fixtures/🧬️mutations/**` (30 bundles, 150 files, new);
  `🧬️schema/🧬️mutations/<leaf>/🧪️tests/<scenario>/🦀️.rs` (30 new, 32 stubs deleted); `🧬️schema/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`;
  `🧬️schema/🔺️diff/📝️text/🦀️.rs`; 6 `🧬️schema/🧬️mutations/*/🔺️diff/🦀️.rs`; `🔮️oracles/🔣️.json`;
  `🧪️tests/⚖️mutate-en1990-1/{🥒️.feature,🐍️.py,🦀️.rs}`; `🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`;
  `🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs`; `🖼️assets/*/*/🗣️.dsl.semio` (8, canonicalised);
  `🖼️assets/*/🎒️.pack.semio` (2, re-encoded).

## 8. Follow-up (2026-09-30 evening): §5 leftovers and the §6 re-run owed

### 8.1 Outcome

| Item | State |
|---|---|
| Point 7 / §4.4 re-run (target-missing recoding) | **Verified.** Kernel builds again; `cargo test --lib` en1990 140/140, en1991 160/160; `parity exhaustive` 322/322 (161/161) and 122/122 (61/61) — first right after the recoding, again after every edit below |
| (a) 17 EN 1991 leaves without emoji | Fixed — taxonomy names restored, every reference rewritten, scenarios shortened to the pair budget |
| (b) stale EN 1990 subset declaration | Fixed — single policy, like EN 1991; manifest `subset` field removed |
| (c) untruthful `outcomeClasses` | Fixed — derived from production diffs, and asserted from the vectors by the crate law |
| (d) binary protocols listing old kinds | Fixed — regenerated from `KINDS`; both codecs now read their tags from the protocol file |
| (e) 137 inventory findings | Cleared — bridge rebuilt, `test inventory` refreshed: 80/80 and 30/30, 0 differences; scoped contract **0 breaches** |

### 8.2 (a) Leaf names

- The 17 leaves (`change-accidental-assumed-force` 🚗, `change-bridge-lane-width` ↔️, `change-coast-or-island` 🏝️,
  `change-crane-claimed` 🏗️, `change-en-sk` ❄️, `change-floor-assumed-qk` 🏢, `change-hoisting-speed` ⏫,
  `change-mixed-terrain-distance` 📏, `change-roof-assumed-sk` 🌨️, `change-silo-bulk-density` 🌾, `change-silo-claimed` 🏭,
  `change-silo-hydraulic-radius` ⭕, `change-silo-k` ⚙️, `change-snow-zone` 🗺️, `change-structure-kind` 🌉,
  `change-terrain-category` 🏞️, `change-wind-zone` 🪁) were moved (schema leaf, fixture bundle, canonical case), and their
  descriptors' `emoji`/owner fixed (`🧪️w2-w-norm-1-rename.py`).
- References rewritten: the EN 1991 lib root (51 `#[path]`s), `📚️library/🔣️schema-catalog.json` and `📓️schema-catalog.md` (17 paths
  each), the norm `📇️mutation-leaf-taxonomy-v1` fixture (then regenerated by its registered `mutation-leaf-taxonomy-generate`
  target; `…-check` passes). Catalogs, feature Examples and generated sources are rewritten by `🧪️w2-w-norm-1-vectors.py sources`.
- The canonical mutation pair did not fit the taxonomy budget (fixture prefix 150 B + 42 ≤ 240 ⇒ leaf + scenario ≤ 48 B), so
  every EN 1991 and EN 1990 scenario got a short slug (`🗺️zone-3`, `🪚beam-b1`, `⏳cat-5`, …; table in `🧪️w2-w-norm-1-vectors.py`).
- Taxonomy inventory (`🧪️w2-w-norm-1-taxonomy-moves.ts`, because `verify taxonomy report` still aborts on the peer
  `📐️cad-draw-path-projection` digest), before → after:
  EN 1991 `path-too-long` 522 → 2, `mutation-pair-path-budget` 77 → 1, `directory-kind-unresolved` 327 → 97;
  EN 1990 137 → 0, 26 → 0, 140 → 64. The rest is structural or pre-existing: the leaf name
  `🏔change-exceptional-snow-north-german-lowlands` alone exceeds the pair budget (a kind rename, not a scenario one); the
  unresolved directories are vector bundle leaves with no registered member name — norm-wide, en1995 shows the same (287), and
  W3-TAX is registering fixture dirs; `🧪parse-en1990-artifact.bun.ts` (kind-only basename, used by the Rust io test) and the
  `📝️text` projection member are older than this WP.

### 8.3 (b) Subsets

`⚖️en1990/…/🪆️subsets/🔣️.json` now declares one unconstrained subset (`"*"`, `subsetPolicy: single`) with a rationale naming the
current kinds; the variable actions are an inline ordered collection of the one value model, not a separately owned register.
The 30 manifest rows lost `subset: "combination"` and the manifest its `semanticKinds`. EN 1991's rationale lists current kinds.

### 8.4 (c) Outcome classes

`🧪️w2-w-norm-1-outcomes.py` derives each leaf's classes from its production diff: `applied`, plus `no-op` where the diff guards
equality (`mutation.no-op`), plus `rejected` where it refuses. Result: EN 1991 insert/remove `[applied, rejected]`, scalar
change `[applied, no-op]`, indexed change `[applied, no-op, rejected]`; EN 1990 remove `[applied, rejected]`, insert `[applied]`,
change `[applied, no-op]`. Descriptors and v2 manifest rows are written together. The crate vector law now proves it per
vector: it replays the op (applied), the op on its own after-state (no-op), and the op with `index = u32::MAX` on the
before-state (rejected, ≥ Error), and asserts that the reached classes equal the declared ones.

### 8.5 (d) Binary protocol

- New generic codec in the norm contract crate: `payload_op_binary::{encode, decode}` (`📇️registry/🧬️contract/🦀️.rs`, region
  `💾️PayloadOpBinary`). Frame = `format u8` (`OP_BINARY_FORMAT`) · `tag u8` from the subset's `📡️.protocol.semio` (found via
  `dsl::protocol_record`) · `payload_value()` as canonical JSON. Decode goes through `Mutation::from_payload_value` and refuses
  bytes that do not re-encode to themselves. It names no variant and no field.
- EN 1991's `OpBinary` was the op text as bytes (no tag); EN 1990 used 30 hand constants plus its own JSON reader. Both now
  call the generic codec. EN 1990 keeps its old tags (0–29); EN 1991 gets 0–79 in `KINDS` order (`🧪️w2-w-norm-1-protocols.py`).
- `📡️.protocol.semio`, `🥋️.ksy`, `🌶️.spicy`, `🔠️.abnf` were regenerated for both. `binaryProtocolDriftBreaches` is now in the scoped
  contract: 0. The vector law round-trips every vector through `OpBinary`.

### 8.6 (e) Runtime inventory

- The bridge (`✏️s/🔌️plugins/📕️norm/🏭️bridge`) is the sanctioned producer: `test inventory` runs its `📜️script.ts list-mutations`
  (`cargo run --offline`). Built in the foreground with the private target (6 m 58 s, all 17 norm crates at the current tree),
  then `bun ./📜️script.ts inventory --artifact s.norm.en1991 --standard 1` / `s.norm.en1990` → 80 runtime / 80 declared and 30 / 30,
  0 differences each (cache under `⚡️cache/tests/results/🏭️inventory/`, gitignored).
- `🏭️bridge/Cargo.lock` changed only by resolution: `+semio-framework-{machine,machine-derive,time-travel,tool-machine}` (new
  kernel deps), `−zip` and its now-unused tree (`flate2`, `zopfli`, `crc32fast`, `thiserror 2`, `arbitrary`, …). Accepted as is.
- An earlier detached build (22:13) failed on a peer's in-flight `🔌️plugin/⏪️time-travel/🦀️.rs` edit (undefined
  `protocol::APPLY_OUTCOME_CODE_PREFIX`). The peer removed it by 22:21. It was replaced by the foreground build above.

### 8.7 Verification record (follow-up, all foreground, gated, `target-nde-w2w-norm-1`)

- `cargo test --lib`: `semio-s-artifact-norm-contract` 51/51, en1990 140/140, en1991 160/160 (last run after (d)).
- `cargo check --target wasm32-wasip2` for en1991, en1990 and the contract crate: ok.
- `schema mutation-payloads` → 0 (80/80, 30/30 witnessed); `schema mutation-inputs` → 0 (90/90, 36/36).
- `parity exhaustive`: 322/322 (161/161), 122/122 (61/61). `oracle exhaustive` re-run at 22:39 against the shared
  `🔮️oracles/🏃️execution/🐍️.py` a sibling changed at 22:39: 161/161, 61/61.
- Scoped contract (`bun 🧪️w2-w-norm-1-contract.ts`, now including binary protocol drift): 2 cases, **0 breaches** (was 137).
- `🔮️oracle-source-ownership`: still 3 pass / 3 fail on peers only (§5.7, unchanged).
- The only edit after the last cargo test run is a module docstring (`🏋️en1991/…/💾️binary/🦀️.rs`). The bridge build compiled it.

### 8.8 Files (follow-up)

- New ticket scripts: `🧪️w2-w-norm-1-rename.py`, `🧪️w2-w-norm-1-outcomes.py`, `🧪️w2-w-norm-1-protocols.py`,
  `🧪️w2-w-norm-1-taxonomy-moves.ts`. Updated: `🧪️w2-w-norm-1-vectors.py` (short slugs, probe law, binary round trip),
  `🧪️w2-w-norm-1-contract.ts` (+ binary protocol drift).
- Norm contract: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🦀️.rs`. Norm fixture:
  `✏️s/🔌️plugins/📕️norm/🧫️fixtures/📇️mutation-leaf-taxonomy-v1/🔣️.json`. Bridge: `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock`.
- Library: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/{🔣️schema-catalog.json,📓️schema-catalog.md}`.
- EN 1991: `🏋️en1991/🦀️.rs`; the `🪆️subsets/🔣️.json`; under `✳️any/`: the 17 renamed leaves (schema + fixture), every
  bundle/case moved to its short scenario, all 80 leaf `🔣️.json` (outcomes), `🧬️schema/🧬️mutations/{📝️text,💾️binary}/🦀️.rs`,
  `💾️binary/{📡️.protocol.semio,🥋️.ksy,🌶️.spicy,🔠️.abnf}`, `🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, `🔮️oracles/🔣️.json`,
  `🧪️tests/🏋️mutate-en1991-1/🥒️.feature`. Deleted: `🧬️schema/💡️inferences/_new_check_part1.rs.txt` (stale fragment, integrated
  as `check_full_actions`).
- EN 1990: `🪆️subsets/🔣️.json`; under `✳️any/`: every bundle/case moved to its short scenario, all 30 leaf `🔣️.json`,
  `🧬️schema/🧬️mutations/📝️text/🦀️.rs`, `💾️binary/{📡️.protocol.semio,🥋️.ksy,🌶️.spicy,🔠️.abnf}`,
  `🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, `🔮️oracles/🔣️.json`, `🧪️tests/⚖️mutate-en1990-1/🥒️.feature`.
- Scratch/logs: `🗑️generated/w2w-norm-1/` (left for the coordinator's sweep).
