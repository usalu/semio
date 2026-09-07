# 🏢️ W10 — exhaustive fixture cases for every fem2d mutation kind

Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/*/🧬️schema/🧬️mutations/<kind>/🧪️tests/`.
Before this wave each of the 25 fem2d kinds carried **exactly one** committed vector. It now carries
**three**: the pre-existing happy path, a second happy path on a **new real-world model**, and an
**edge case** encoding the refusal (or the provable no-op) the implementation actually performs.

**50 new case directories, 300 new files, 50 new crate-entry mounts, 50 new oracle scenarios,
50 new `members-of-tests` entries. No compiler was run** (host swap exhausted; the coordinator
compiles). Every claim in §5 is exactly the static resolution, the `rustfmt` parse, the third-party
`jsonschema` validation or the Python replay whose output is quoted there — nothing else.

---

## 1. The new real-world model — a two-storey braced steel frame

Authored in the snapshot schema `🌐️any/🧬️schema/📸️snapshot/🔣️.json` (nine members,
`additionalProperties: false`) and committed identically as the `📸️snapshot/⬅️before/🔣️.json` of
all 50 new cases. It is the **second** real fem2d model in the repository; the first is the
`🏗️timber-portal-frame.snapshot.json` that the subset-level differential cases share.

One 6.0 m bay, storey heights 3.5 m, plus a 2.0 m canopy tip at roof level. All SI base units
(m, m², m⁴, Pa, kg/m³, N, N/m, Pa).

| member | content |
|---|---|
| `nodes` (7) | `n1..n2` bases at y=0, `n3..n4` first floor at y=3.5, `n5..n6` roof at y=7.0, `n7` canopy tip at (8.0, 7.0) |
| `elements` (7) | `c1..c4` HEB 200 columns (`beam`), `b1` IPE 270 floor beam, `b2` IPE 240 roof beam, `br1` CHS 88.9×4.0 lower-storey tension brace (`bar`) |
| `regions` (2) | `wall1` 200 mm C30/37 infill panel over the lower bay with a 1.2 × 1.4 m window opening as a hole; `panel_spare` 150 mm side panel |
| `materials` (3) | `steel_s355` (E = 210 GPa, ν = 0.3, ρ = 7850), `concrete_c30` "Concrete C30/37" (E_cm = 33 GPa, ν = 0.2, ρ = 2500), `steel_s235_spare` |
| `sections` (5) | HEB 200 (A = 78.08 cm², I_y = 5696 cm⁴), IPE 270 (45.95 cm², 5790 cm⁴), IPE 240 (39.12 cm², 3892 cm⁴), CHS 88.9×4.0 (10.669 cm², 96.34 cm⁴), IPE 200 spare |
| `supports` (3) | `sup1`/`sup2` fixed column bases (`Tx,Ty,Rz`), `sup_tie` roof-level lateral tie (`Tx`) |
| `loadCases` (4) | `dead` (self-weight on; 18.5 kN/m on `b1`, 9.2 kN/m on `b2`), `live` (15.0 kN/m on `b1`), `wind` (12.5 kN nodal `Tx` at `n5`, 640 Pa area pressure on `wall1`), `snow_spare` (4.8 kN/m on `b2`) |
| `combinations` (3) | `uls1` "ULS 6.10b" (1.35 dead + 1.5 live), `sls1` "SLS characteristic" (1.0/1.0/0.6), `uls_spare` |
| `analysis` | `modalCount 6`, `bucklingCount 4`, `deformationScale 120.0` |

`n7`, `steel_s235_spare`, `ipe200_spare`, `sup_tie`, `panel_spare`, `snow_spare` and `uls_spare` are
deliberate **trailing spares**, the same device the timber portal frame uses: no `create-` verb in
this vocabulary carries an index, so a `delete-` is exactly invertible only for a trailing record.
Every one of them is referenced by nothing else, so the happy-path deletes leave no dangling id.

The model is authored once, in `🔨️w10-fem2d-cases.py`'s `MODEL`, and emitted per case.

---

## 2. Kind × cases

| kind | subset | (a) pre-existing | (b) steel-frame happy path | (c) edge case | what the code does on (c) |
|---|---|---|---|---|---|
| `create-node` | 🕸️mesh | `📍️appends-node-n3` | `🏢️appends-the-canopy-226a20` | `🚫️rejects-a-duplicate-eb0df0` | `mutation.duplicate-id` (Fatal) |
| `delete-node` | 🕸️mesh | `🚫️removes-node-n3-without-6eab3f` | `🗑️drops-the-spare-6c285d` | `⛔️rejects-a-missing-429801` | `mutation.target-missing` (Error) |
| `create-element` | 🕸️mesh | `➖️appends-bar-e2-between-fc1c09` | `📐️braces-the-upper-d96634` | `🚫️rejects-a-dangling-b9e64c` | `mutation.target-missing` (Error) |
| `delete-element` | 🕸️mesh | `🚫️removes-bar-e2-and-3c0260` | `✂️cuts-the-lower-e4a250` | `⛔️rejects-a-missing-611215` | `mutation.target-missing` (Error) |
| `replace-element` | 🕸️mesh | `♻️converts-beam-e1-into-a-5d21f5` | `🔧️regrades-the-roof-fb20eb` | `⛔️rejects-a-missing-bd448c` | `mutation.target-missing` (Error) |
| `create-material` | 🧱️material | `🧱️appends-concrete-c30` | `🏗️adds-the-c25-slab-11d8df` | `🚫️rejects-a-duplicate-f3220b` | `mutation.duplicate-id` (Fatal) |
| `delete-material` | 🧱️material | `🚫️removes-the-30f7a2` | `🗑️drops-the-spare-00e964` | `⛔️rejects-a-missing-d5b18f` | `mutation.target-missing` (Error) |
| `replace-material` | 🧱️material | `🏗️restates-steel-7c22bc` | `📉️cracks-the-c30-b2b220` | `⛔️rejects-a-missing-b3adee` | `mutation.target-missing` (Error) |
| `create-section` | 🕸️mesh | `📐️appends-the-ipe300-profile` | `➕️adds-the-hea220-dfdf34` | `🚫️rejects-a-duplicate-e91bc7` | `mutation.duplicate-id` (Fatal) |
| `delete-section` | 🕸️mesh | `🚫️removes-the-spare-1c235a` | `✂️drops-the-spare-dcf609` | `⛔️rejects-a-missing-dbd0a4` | `mutation.target-missing` (Error) |
| `replace-section` | 🕸️mesh | `💪️stiffens-ipe200-with-5e9c08` | `🛠️thickens-the-chs-e235a5` | `⛔️rejects-a-missing-b468f4` | `mutation.target-missing` (Error) |
| `create-support` | 🛡️boundary | `🛞️adds-a-vertical-6161a1` | `🔻️props-the-canopy-b9d719` | `🚫️rejects-a-dangling-b0d60b` | `mutation.target-missing` (Error) |
| `delete-support` | 🛡️boundary | `🔓️releases-the-82b34f` | `🕊️frees-the-roof-tie-44562b` | `⛔️rejects-a-missing-23f3c3` | `mutation.target-missing` (Error) |
| `replace-support` | 🛡️boundary | `🔒️upgrades-the-834e4a` | `🔩️pins-the-left-base-7891ec` | `⛔️rejects-a-missing-afbf6d` | `mutation.target-missing` (Error) |
| `create-region` | 🕸️mesh | `🧱️appends-a-solid-d78275` | `🏢️infills-the-upper-6cc520` | `🚫️rejects-a-duplicate-11ca0d` | `mutation.duplicate-id` (Fatal) |
| `delete-region` | 🕸️mesh | `🚫️removes-the-slab-and-5b301a` | `🧹️drops-the-spare-460714` | `⛔️rejects-a-missing-a83a6d` | `mutation.target-missing` (Error) |
| `replace-region` | 🕸️mesh | `🪜️punches-a-stair-f7b3b1` | `🪟️widens-the-window-09a8ec` | `⛔️rejects-a-missing-6e0d70` | `mutation.target-missing` (Error) |
| `create-load-case` | 🏋️load | `📍️appends-a-live-case-59118a` | `❄️appends-the-snow-4c007c` | `🚫️rejects-a-dangling-4904f4` | `mutation.target-missing` (Error) |
| `delete-load-case` | 🏋️load | `🚫️removes-the-live-06415d` | `🗑️drops-the-spare-49435f` | `⛔️rejects-a-missing-79ed15` | `mutation.target-missing` (Error) |
| `add-load` | 🏋️load | `📏️appends-a-member-udl-to-the-dead-case` | `💨️pushes-a-wind-load-5c3f1e` | `🚫️rejects-a-missing-4271bc` | `mutation.target-missing` (Error) |
| `remove-load` | 🏋️load | `➖️strips-the-trailing-member-133914` | `✂️strips-the-roof-udl-0c1b3c` | `⛔️rejects-a-missing-1a8a80` | `mutation.target-missing` (Error) |
| `change-load-case-self-weight` | 🏋️load | `⚖️switches-self-abbff2` | `🏋️switches-self-5977a5` | `🔁️keeps-self-weight-ff696b` | `mutation.no-op` (Warning, document untouched) |
| `create-combination` | 🏋️load | `🔗️appends-an-uls-0c18bb` | `➕️appends-the-6-10a-eefe01` | `🚫️rejects-a-dangling-2aa3ea` | `mutation.target-missing` (Error) |
| `delete-combination` | 🏋️load | `✂️removes-the-uls-438c0c` | `🗑️drops-the-spare-60fda7` | `⛔️rejects-a-missing-d4bc03` | `mutation.target-missing` (Error) |
| `update-analysis-settings` | 📈️analysis | `🔢️doubles-the-modal-3fbb1a` | `🎚️raises-the-mode-908c2b` | `🔁️keeps-the-analysis-196e4a` | `mutation.no-op` (Warning, document untouched) |

23 of the 25 edge cases are genuine refusals. The remaining two are the *only* thing their kind
does when handed an already-satisfied payload — see finding **F1**.

### Case shape

* happy path — `🔺️diff/🔣️.json` (all 17 `Fem2dDiff` slots, the written one populated), a moving
  `➡️after`, outcome `{"status": "applied"}`; 7 `#[test]`s.
* refusal — `🔺️diff/🚫️.absent` (empty marker, contract D6), `➡️after` byte-identical to
  `⬅️before`, outcome `{"status": "rejected", "code": …, "path": […]}`; 5 `#[test]`s.
* no-op — `🔺️diff/🔣️.json` holding `Fem2dDiff::default()`, `➡️after` byte-identical to
  `⬅️before`, outcome `{"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}`;
  8 `#[test]`s.

The refusal shape is **not** the pre-existing fem template. That template asserts
`apply_fem2d_mutation(..).is_ok() == false` for a declared rejection, which can never hold here:
`vcs::apply_mutation` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:1192-1197`) is
policy-agnostic and a rejected outcome's diff is `D::default()` by construction (§C2 LAW 1), so
`apply` returns `Ok` with the snapshot untouched. The new refusal cases therefore assert on
`MutationOutcome::messages()` — code, `protocol::Severity`, and `target` — exactly as the
`🕸️dag`/`🔱️trinity` rejection fixtures do.

---

## 3. Refusal semantics, read out of each kind's own diff builder

Paths below are relative to
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/<subset>/🧬️schema/🧬️mutations/<kind>/🔺️diff/🦀️.rs`.

| kind | guards, in the order the code runs them |
|---|---|
| `create-node` | `:9` Fatal `duplicate-id`. Nothing else. |
| `delete-node` | `:9` Error `target-missing`. No cascade, no referential guard. |
| `create-element` | `:10` Fatal `duplicate-id`; `:16` `start` node; `:19` `end` node; `:22` `materialId`; `:25` `sectionId` — all Error `target-missing`. The only 4-reference create in the vocabulary. |
| `delete-element` | `:9` Error `target-missing`. |
| `replace-element` | `:9` Error `target-missing`; `:12` Warning `no-op` when the new value equals the old. |
| `create-material` | `:9` Fatal `duplicate-id`. |
| `delete-material` | `:9` Error `target-missing`. |
| `replace-material` | `:9` Error `target-missing`; `:12` Warning `no-op`. |
| `create-section` | `:9` Fatal `duplicate-id`. |
| `delete-section` | `:9` Error `target-missing`. |
| `replace-section` | `:9` Error `target-missing`; `:12` Warning `no-op`. |
| `create-support` | `:9` Fatal `duplicate-id`; `:12` Error `target-missing` on `support.nodeId`. |
| `delete-support` | `:9` Error `target-missing`. |
| `replace-support` | `:9` Error `target-missing`; `:12` Warning `no-op`. |
| `create-region` | `:9` Fatal `duplicate-id`; `:12` Error `target-missing` on `region.materialId`. |
| `delete-region` | `:9` Error `target-missing`. |
| `replace-region` | `:9` Error `target-missing`; `:12` Warning `no-op`. |
| `create-load-case` | `:9` Fatal `duplicate-id`; then per load variant `:15` `nodal.nodeId`, `:20` `memberUdl.elementId`, `:25` `area.regionId`, all Error `target-missing`. |
| `delete-load-case` | `:9` Error `target-missing`. |
| `add-load` | `:9` Error `target-missing` on `caseId`; `:13` Warning `no-op` when the load id is already in the case. |
| `remove-load` | `:9` Error `target-missing` on `caseId`; `:12` Error `target-missing` on `loadId`. |
| `change-load-case-self-weight` | `:9` Error `target-missing` on `caseId`; `:12` Warning `no-op` when the flag already has the requested value. |
| `create-combination` | `:9` Fatal `duplicate-id`; `:14` Error `target-missing` for a term naming neither a load case nor a combination. |
| `delete-combination` | `:9` Error `target-missing`. |
| `update-analysis-settings` | `:9` Warning `no-op` when the settings are unchanged. **No rejection branch at all.** |

Inverse behaviour on a refusal, read from `↩️inverse/🦀️.rs` beside each builder: the eight
`create-` verbs invert **payload-derived** (`create-node` `:8`, `create-element` `:9`, … always one
`delete-<noun>` of the id they were asked to create, even when the create was refused), while every
`delete-`/`replace-` verb and `add-load`/`remove-load`/`change-load-case-self-weight` invert
**base-derived** and collapse to `Vec::new()` when the target is absent. `update-analysis-settings`
always emits exactly one step carrying `base.analysis`. Each new refusal case pins its own arm.

---

## 4. Findings — where the implementation accepts what it arguably should not

Recorded rather than encoded as fixtures, per the brief: the code accepts these, so no committed
vector may claim otherwise.

**F1 — `update-analysis-settings` has no rejection branch whatsoever.** Its single guard is the
equality no-op. `modalCount`/`bucklingCount` are `usize`, so a negative literal fails to decode, but
any decodable value (0 modes, 4 294 967 295 modes, a negative or zero `deformationScale`) is
accepted unconditionally. `change-load-case-self-weight` is the only other kind whose edge case had
to be a no-op, but that one *does* have a rejection branch (`:9`).

**F2 — no verb in this vocabulary has a referential-integrity guard on `delete-`.** All eight
`delete-` verbs check only the target's existence:

* `delete-node` accepts deleting a node that elements, supports and nodal loads still name (this is
  the *specified* behaviour — the pre-existing vector is literally called
  `🚫️removes-node-n3-without-6eab3f`);
* `delete-element` orphans every member UDL naming it;
* `delete-material` orphans every element and region naming it (deleting `steel_s355`, which seven
  elements name, is accepted);
* `delete-section` orphans every element naming it;
* `delete-region` orphans every area load naming it;
* `delete-load-case` orphans every combination term naming it;
* `delete-combination` orphans every nesting combination term.

**F3 — `replace-` verbs validate only the TARGET, never the replacement's own references.**
`replace-element` will accept a new element naming a node/material/section that does not exist;
`replace-support` a new `nodeId` that does not exist; `replace-region` a new `materialId` that does
not exist. Their `create-` twins all check exactly those references, so the vocabulary is
asymmetric.

**F4 — `add-load` performs no reference check on the load it attaches**, while its `create-load-case`
sibling checks the same references per load variant (`:15/:20/:25`). So a `nodal` load naming a
missing node is refused when it arrives inside a new case and accepted when it is added to an
existing one.

**F5 — `add-load` treats a colliding load id as a Warning-level no-op, not a Fatal `duplicate-id`,**
unlike every other identity collision in the vocabulary. The load is silently dropped.

**F6 — `create-region`/`replace-region` perform no geometric validation.** A self-intersecting
outline, a degenerate two-point outline, a hole outside the outline, a zero or negative `thickness`
or `meshSize` are all accepted. The brief's suggested "create-region with a self-intersecting
outline" edge case therefore has **no refusal to encode**; the region kinds' edge cases pin the
duplicate-id / target-missing branches that do exist.

**F7 — the per-kind `🧬️.schema.json` does not describe the committed wire shape.** Validated with
`jsonschema` 4.26.0 (third-party): **all 75** committed fem2d mutation payloads — the 25
pre-existing ones included — fail their own kind's schema with
`Additional properties are not allowed ('mutation' was unexpected)`. The schema describes the bare
payload struct; the wire is internally tagged (`#[value(tag = "mutation")]`). Stripping the tag
leaves **12** further failures, all in the four enum-carrying kinds (`create-element`,
`replace-element`, `add-load`, `create-load-case`): the schemas spell the variant discriminators as
the PascalCase Rust names (`"const": "Bar"`, `"Beam"`, `"Nodal"`, `"MemberUdl"`, `"Area"`) while the
wire and `#[value(rename_all = "camelCase")]` use `bar`/`beam`/`nodal`/`memberUdl`/`area`. Both
defects pre-date this wave and affect fem3d identically. Not fixed here — schema authorship is
outside W10's slice.

**F8 — the committed independent Python reference is weaker than the Rust implementation on
referential integrity.** Replaying all 75 vectors through
`🌐️any/🧪️tests/*/🐍️.py` (§5) leaves exactly **4 divergences**, all of the same family: the
reference's generic `create-` path checks only for a duplicate id, so it ACCEPTS the four
dangling-reference payloads that Rust refuses — `create-element/🚫️rejects-a-dangling-b9e64c`,
`create-support/🚫️rejects-a-dangling-b0d60b`, `create-load-case/🚫️rejects-a-dangling-4904f4` and
`create-combination/🚫️rejects-a-dangling-2aa3ea`. (The reference agrees on all 19 other refusals,
including every `duplicate-id` and every `target-missing` on a delete/replace.)
The reference is therefore not yet a valid
oracle for the refusal half of the vocabulary. Closing this means adding the same guards to
`apply_mutation` in the five `🐍️.py` references — one `if` per reference — which is outside W10's
file ownership and is left to the coordinator.

**F9 — the Python leg cannot see the new cases yet, because case discovery is EXPLICIT in all three
languages, not directory-driven.**

* Rust: each case is mounted by name in the crate entry (`#[cfg(test)] #[path = …] mod tests_…;`) —
  done here, 50 mounts.
* oracle catalog: each case is a `{id, directoryName}` row under its vector's `scenarios` —
  done here, 50 rows; the coordinator's `mutationVectorRegistryBreaches`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:1470-1530`)
  reports any physical directory that is not registered, and any registered one whose bundle is not
  the closed shape.
* Python: the `spec-vector-<kind>` handler takes its fixture from the ONE `Examples` row the
  subset's `🥒️.feature` names (`…/🏋️load/🧪️tests/🏋️mutate-fem2d-1-load/🥒️.feature`, the
  `dir`/`fixture` columns). Scenario ids must be unique within a feature
  (`…/🧪️test/📦️packages/🟦️typescript/🟦️.ts:427`), so a second row for the same kind cannot reuse
  `spec-vector-<kind>`; and the reference registers handlers **by full scenario id**
  (`built.oracle("spec-vector-%s" % kind, …)`), so a new id would land unregistered.

  Routing the new cases through the coordinator therefore needs a paired change in
  `🥒️.feature` + `🐍️.py` (either a `spec-vector-<kind>-<case>` id scheme registered by iterating
  the catalog, or a directory-driven handler). Both files are outside W10's slice, so this wave
  instead proves the new triples with `🔨️w10-replay-fem2d.py`, which drives the very same
  reference functions over every case directory. **The evidence exists; the coordinator wiring is
  one follow-up edit.**

---

## 5. Verification (static + replay; no compiler run)

### (i) mount and fixture-read resolution — `python3 🔨️resolve-mounts.py --tree`

```
── ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs
   checked:    443
   resolved:   443
   unresolved: 0
── include_str! across ✏️s/🔌️plugins/🏗️fem
   checked:    1314
   resolved:   1314
   unresolved: 0
TOTAL unresolved: 0
```

(443 = the 343 W1 measured plus this wave's 50 fem2d mounts and W11's 50 fem3d ones; the counts are
whole-plugin, not W10-only.)

### (ii) Rust parse — `rustfmt --edition 2024 < <file> > /dev/null`

```
parsed 50 files, 0 failures
```

Run over exactly the 50 new `🦀️.rs`. This proves they parse; it does **not** prove they type-check.

### (iii) replay through the committed independent Python model — `uv run python 🔨️w10-replay-fem2d.py`

```
cases replayed: 75  (applied 50, refused 19, unchanged 2, diverged 4)
bundle-shape failures: 0
jsonschema (third-party) snapshot failures: 0
reference divergences (declared vs. what the independent Python model did): 4
```

* **applied 50** — the 25 pre-existing and the 25 new happy paths. For each, the reference
  independently reproduced `➡️after` from `⬅️before` + `🦠️mutation` (`equals_committed`),
  confirmed the document actually moved (`observable`), confirmed **exactly one of the nine members
  moved** (`touches_one`), and confirmed the reference's own computed inverse restored `⬅️before`
  (`restores`).
* **refused 19** — the reference refuses the same payload the Rust builder refuses.
* **unchanged 2** — the two no-op cases: the reference applies them and lands on a document equal
  to `⬅️before`, which is what the committed `➡️after` says.
* **diverged 4** — finding **F8**; the reference has no referential guards on `create-`.
* **bundle-shape 0** — every one of the 75 case directories is the exact closed bundle the
  coordinator's `expectedVectorBundle` demands (6 directories, 5 files, exactly one of
  `🔺️diff/🔣️.json` / `🔺️diff/🚫️.absent`, nothing else, no symlinks).
* **jsonschema 0** — all 150 committed snapshots validate against
  `🌐️any/🧬️schema/📸️snapshot/🔣️.json` under `jsonschema` 4.26.0 (PyPI, MIT), Draft 2020-12.
  This is the third-party leg; the mutation-payload leg is finding **F7**.

### (iv) catalog ↔ disk ↔ taxonomy

* All five fem2d `🔮️oracle/🔣️.json` re-parse; 25 vectors × 3 scenarios = **75 declared**, set-equal
  in both directions with the 75 physical case directories; `stem(directoryName) == id` for all 75;
  every id matches `^[a-z0-9]+(-[a-z0-9]+)*$`.
* `🔣️taxonomy.json` re-parses; `members-of-tests.memberNames` now holds 1187 entries, **0
  duplicates**, all NFC, and contains all 50 new names.
* Every new directory name is ≤ 28 code points (Windows path budget) and carries a handpicked
  leading emoji that is unique among its own `🧪️tests` siblings — the `duplicate` finding
  `pathEmojiStatuteFindings` raises for two siblings sharing an emoji
  (`…/📚️library/🔍️discovery/🟦️.ts:723-725`).
* Canonical-JSON discipline: `DslValue` distinguishes `Number::UInt`/`Int`/`Float`
  (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:68-77`), so every `f64` field is emitted with an explicit
  decimal point and no exponent, and `modalCount`/`bucklingCount` as bare integers. This is what the
  generated `committed_json_is_canonical` tests assert.

### What is NOT proved

No `cargo` ran. Nothing here shows the 50 new `🦀️.rs` type-check, that `Fem2dDiff::default()`
serialises to the committed 17-null diff, or that `protocol::Severity` and `message.code.0` are the
exact accessor spellings in the current tree (they are the spellings the `🕸️dag`/`🔱️trinity`
rejection fixtures use). The first `cargo test -p semio-s-plugin-fem` after this wave is the real
gate.

---

## 6. Files

**Added** — 300 files under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/*/🧬️schema/🧬️mutations/*/🧪️tests/<new case>/`
(50 case directories × `🦀️.rs`, `🦠️mutation/🔣️.json`, `🎯️outcome/🔣️.json`,
`📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`, and either `🔺️diff/🔣️.json` or
`🔺️diff/🚫️.absent`).

**Modified**

* `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` — 50 `#[cfg(test)]` mounts appended, two per
  kind, beside each kind's existing one.
* `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/{🏋️load,🕸️mesh,🧱️material,🛡️boundary,📈️analysis}/🔮️oracle/🔣️.json`
  — 50 `scenarios[]` rows appended (25 vectors go from 1 to 3 scenarios).
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — 50 `members-of-tests`
  entries appended.

**Ticket**

* `🔨️w10-fem2d-cases.py` — the model, the 50 case specifications, the transcribed diff builders and
  `apply_delta`, and the Rust templates. Idempotent; `--check` diffs against the tree.
* `🔨️w10-replay-fem2d.py` — the replay driver of §5 (iii).
* this report.
