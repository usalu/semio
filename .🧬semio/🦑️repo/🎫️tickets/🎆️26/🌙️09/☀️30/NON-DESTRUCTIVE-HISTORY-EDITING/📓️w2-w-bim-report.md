# 📓️ W2-W-bim — wire-witness conversion: ifc (2x3, 4) and step (ap214)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-W-bim, 2026-09-30. Contract: `📋️design.md` §6 and §11,
`🧭️plan.md` "W2-W brief", `📓️w2-s-report.md` F10, `📓️w1-d-report.md` §8.

## 1. Outcome

| check | before | after |
|---|---|---|
| `schema mutation-payloads --under …/🏗️ifc` | 36/93 clean, 145 findings (43 `invalid`, 88 `undescribed`, 14 `unmapped`) | **79/79 clean, 30/30 leaves witnessed, 0 findings** |
| `schema mutation-payloads --under …/📐️step` | 49/91 clean, 42 findings (28 `invalid`, 14 `unmapped`) | **77/77 clean, 38/38 leaves witnessed, 0 findings** |
| `schema mutation-inputs --under …/🏗️ifc` | 0 | **0** (47/47 inputs) |
| `schema mutation-inputs --under …/📐️step` | 0 | **0** (50/50 inputs; the 3 new `argIndex` inputs are labelled) |
| repo cases, `parity exhaustive` (oracle + subject + parity) | ifc/step subjects did not compile (§2.1) | **14/14 cases green**, §4 |

Every leaf is witnessed by a wire-form feature row; no extra `🧾️wire-witness` fixture was needed.

## 2. What was wrong

1. **The subjects of all 14 cases could not compile.** Every Rust adapter named `semio_s_artifact_stdio_contract::part21`
   directly, but a generated test host links only the owner's crate (`semio-s-artifact-stdio-ifc`/`-step`), the oracle crate
   and the test host. The part-21 codec moved into the contract crate on 09-23; the adapters were never re-pointed.
2. **Hand grammar.** Rows spoke a private `{"t": …, "v": …}` value grammar that is none of the three real wires
   (`Part21Value` `{kind, value|values|typeName}` with `Part21Decimal` reals for ifc 2x3; adjacently tagged `IfcValue`
   `{kind, value}` for ifc 4; externally tagged `StepValue` `"unset"`/`{"real": 2.5}` for step), and `upsert-instance`
   spelled `Part21Instance` entities `{name, args}` instead of `{typeName, arguments}`.
3. **`set-snapshot` rows were not snapshots.** Every subset's row was `{"fileSchema": [...]}` (cc1..cc6 also
   `productIdentity`), which each adapter and oracle turned into a hand-built snapshot.
4. **`no-mutation` sentinel** rows (differential tables) and `@id-no-mutation-baseline-*` scenarios, plus a `"no-mutation"`
   arm in every oracle dispatcher.
5. **step base leaves were the Rust outlier.** `set-file-description`, `set-file-name`, `set-file-schema`, `set-entity-arg`,
   `insert-entity-arg`, `remove-entity-arg` emitted snake_case (`file_description`, `arg_index`) while the TS twin, graphql
   and every feature row spelled camelCase.

## 3. What changed

### 3.1 Production (Rust)

- **Contract crate** (`📇️registry/🧬️contract/🦀️.rs`, region `🧾️PayloadWire`): three generic bridges over the kernel
  `Mutation` trait — `mutation_from_payload_json::<P, M>(kind, json)` (derive-generated `from_payload_value` on the parsed
  wire), `apply_mutation_checked` (diff → apply, a rejection is an `Err`), `mutation_inverse` (the production inverse). They
  replace per-aggregate "reachability" wrappers; any stdio artifact can use them (W2-W peers: re-export them from your crate
  root instead of writing per-aggregate decoders).
- **ifc and step crate roots** re-export `{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, part21}` so an
  adapter that links only the owner crate reaches them (fixes §2.1).
- **step base leaves**: `#[value(rename_all = "camelCase")]` on the six leaves of §2.5; their leaf schemas renamed
  (`fileDescription`, `fileName`, `fileSchema`, `argIndex`) and `argIndex` gained `x-semio-ui` (stepper, en/de label and
  description, `precision 0`, `group target`). TS twin and graphql already matched.
- **Deleted** the 12 per-class wrappers `apply_step_ccN_mutation_checked` / `inverse_step_ccN_mutation` (`🚪️Reachability`
  regions of cc1..cc6) — superseded by the generic bridges, no other caller.

### 3.2 Feature tables (14 files)

`🧪️w2-w-bim-features.py` (ticket root, idempotent, `--check` mode) rewrote every row to the leaf wire and deleted the
`no-mutation` rows and baseline scenarios. `set-snapshot` rows now carry a whole, self-contained snapshot record:

| family | snapshot in the row |
|---|---|
| ifc 2x3 (base, cobie, cv20, sav, differential) | the wellness header + the real `IFCPROJECT #120` with its references cleared (9 attributes) |
| ifc 4 (mutate, differential) | the Nakagin header + the real `IFCPROJECT #1` with its references cleared |
| step (base, cc1..cc6) | the ISO 10303-21 minimum header + the `PRODUCT`/formation/definition chain the cc rows used to build |

Prose updated to match (wire paragraph `🧾️`, no-mutation wording, kind counts: cc1/cc6 4 kinds, cc2..cc5 5, base 10,
ifc4 10, ifc2x3 4; differential set-snapshot paragraphs rewritten).

### 3.3 Adapters (Rust subjects, 14 files)

Every subject is now generic: `mutation_from_payload_json::<Snapshot, Aggregate>(kind, params)` → `apply_mutation_checked`
→ encode; the inverse rows apply **the mutation's own `Mutation::inverse`** (the feature says "the mutation's own inverse")
to the re-decoded forward result; the identity round trip is decode → encode. All hand mappers (`mutation_from_spec`,
`value_from_json`, `str_field`/`u64_field`/`identity_from`/`representation_from`, …) are deleted. Oracle halves use
`oracle_round_trip` for baselines and speak wire in their inverse specs; `set-snapshot`'s inverse is
`oracle_snapshot_payload(input)` — the untouched model read back by `ruststep` as a `set-snapshot` payload.

### 3.4 Reference implementations

- **ruststep oracles (Rust, oracle crate)**: new `🧾️Wire` grammars read exactly the leaf wire — ifc 2x3 at the standard
  level (`🔖️2x3/🔮️oracles`, shared by base/cobie/cv20/sav; `Part21Value` incl. `Part21Decimal` reals, `Part21Instance`,
  `Part21Header`), ifc 4 in its subset oracle (`IfcValue`, `IfcEntity` incl. `complex`, `IfcSnapshot`), step at the ap214
  standard level (`StepValue`, `StepEntity`, typed `StepHeader` records, `StepSnapshot`; shared by base and cc1..cc6). Each
  also emits the inverse direction (`snapshot_payload`). Every dispatcher lost its `"no-mutation"` arm and exports
  `oracle_round_trip`/`oracle_snapshot_payload`; cc oracles lost `minimal_document` and the `documentText` restore.
- **IfcOpenShell oracles (Python, differential 2x3 and 4)**: read the same wire (`to_ifcopenshell`/`literal` over
  `Part21Value`/`IfcValue`), `set-snapshot` builds a fresh typed model from the snapshot record, unset arguments stay unset
  (IfcOpenShell refuses an explicit null on a mandatory attribute — found on the first parity run), inverse of ifc 4
  `set-snapshot` re-reads the untouched fixture; 2x3 keeps no `set-snapshot` inverse row (rebuilding 3 464 referenced
  instances through `add` is impossible — documented in the feature).
- **Oracle unit tests** (12 files) read the case's own rows through `crate::law::feature_rows(include_str!(…🥒️.feature))`,
  so unit law and scenario run the same wire payloads.

## 4. Verification (all run, foreground, gated)

| check | result |
|---|---|
| `schema mutation-payloads` / `schema mutation-inputs`, both scopes | 0 / 0 findings (§1) |
| `cargo check -p semio-s-artifact-stdio-{contract,ifc,step}` native and `--target wasm32-wasip2` | ok, no warnings in ifc/step; contract warnings are pre-existing (`✏️editing`) |
| `cargo test --no-fail-fast -p semio-s-artifact-stdio-ifc -p …-step --lib` (private target `target-nde-w2w-bim`) | ifc **135 pass** (2 ignored), step **148 pass**, incl. all 12 `semio_payload_law_*` (ifc2x3, cobie, cv20, sav, ifc4, step, cc1..cc6) |
| oracle crate `cargo test --features oracles --lib -- artifacts::ifc artifacts::step` | **104 pass** |
| `bun ./📜️script.ts parity exhaustive --case …` | mutate-ifc-2x3 9/9 · cobie 13/13 · cv20 11/11 · sav 11/11 · mutate-ifc-4 21/21 · differential-ifc-2x3 6/6 · differential-ifc-4 13/13 · mutate-step-ap214 21/21 · cc1 9/9 · cc2 11/11 · cc3 11/11 · cc4 11/11 · cc5 11/11 · cc6 9/9 — every scenario passed in oracle AND subject role |
| contract phase (`contract --case 🧱️mutate-ifc-2x3`) | repo-wide run exits 1 on 2 936 pre-existing breaches; **0 breaches in ifc/step scope** (catalog coverage holds after the `no-mutation` removal) |
| `test schema --under` ifc / step | no `schema-mutation-*` codes; remaining codes are pre-existing families (`schema-catalog-stale` 19/27 → central `schema generate`, `schema-owner-ineligible`, `schema-export-parser-missing`/`-incomplete` TS twins) |

## 5. Open items and hand-offs

1. **Catalog refresh** (coordinator): leaf schemas changed (step camelCase + `argIndex` labels) → `schema generate` once.
2. **Shared law module** `🗄️stdio/🔮️oracles/⚖️law` still special-cases `kind == "no-mutation"` in
   `mutation_is_observable_within`; used by other artifacts, left for the stdio-wide sweep.
3. **Peers**: the three contract bridges are ready for every other stdio artifact; re-export them from the artifact crate
   root (`pub use semio_s_artifact_stdio_contract::{…}`) — the generated host cannot name the contract crate itself.
4. Pre-existing prose drift fixed only where I touched files (kind counts in cc oracles/aggregate docs and the ap214 standard
   oracle).

## 6. Files

- **Created:** `🧪️w2-w-bim-features.py` (ticket root); scratch in `🗑️generated/w2w-bim/` (lint/test/parity logs, codemods
  `cc-adapters.py`, `cc-subject.rs.txt`, `feature-prose.py`).
- **Production:** `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`; `🗿️artifacts/🏗️ifc/🦀️.rs`; `🗿️artifacts/📐️step/🦀️.rs`;
  step base leaves `{📋️set-file-description,📛set-file-name,🏷️set-file-schema,🔧set-entity-arg,➕insert-entity-arg,➖remove-entity-arg}/{🦀️.rs,🧬️schema/🔣️.json}`;
  cc1..cc6 `🧬️schema/🧬️mutations/🦀️.rs` (wrappers removed; cc6 KINDS doc).
- **Features (14):** ifc `🧱️mutate-ifc-2x3`, `🔺️differential-ifc-2x3`, `🏢️mutate-ifc-2x3-cobie`, `🤝️mutate-ifc-2x3-cv20`,
  `🧮️mutate-ifc-2x3-sav`, `🏗️mutate-ifc-4`, `🔺️differential-ifc-4`; step `📐️mutate-step-ap214`, `🔬️{1..6}-mutate-step-ap214-ccN`.
- **Adapters (14 Rust + 2 Python):** the `🦀️.rs` beside each feature; `🐍️.py` of both differential cases.
- **Oracles:** `🏗️ifc/🏅️standards/🔖️2x3/🔮️oracles/🦀️.rs`; `🔖️2x3/🪆️subsets/{🧱️base,🏢️cobie,🤝️cv20,🧮️sav}/🔮️oracles/🦀️.rs`;
  `4️⃣4/🪆️subsets/✳️any/🔮️oracles/🦀️.rs`; `📐️step/🏅️standards/🔖️ap214/🔮️oracles/🦀️.rs` (+ its test doc);
  `🔖️ap214/🪆️subsets/{🧱️base,1️⃣cc1..6️⃣cc6}/🔮️oracles/🦀️.rs`; and each of those modules' `🧪️tests/🔬️unit/🦀️.rs`.
