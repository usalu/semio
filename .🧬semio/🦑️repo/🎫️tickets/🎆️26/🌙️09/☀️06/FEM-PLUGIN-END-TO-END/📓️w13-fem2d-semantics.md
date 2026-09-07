# 🛡️ W13 — fem2d mutation semantics, schemas and evidence

Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/**`, plus the fem2d mounts of the plugin's crate entry
and the fem2d `members-of-tests` rows of `🔣️taxonomy.json`. Nothing under `🧊️3d` was touched (W12
owns it); W5's `📈️analysis/🧪️tests/🧮️solves-fem2d-1-benchmarks/` and the `🌐️any/🔮️oracle/🔣️.json`
solver-oracle entries were left exactly as W5 wrote them.

**No compiler was run** — the host's swap is exhausted and the coordinator compiles. Every number in
§5 is the output of a `rustfmt` parse, a `jsonschema` validation, a static mount resolution or a
replay through the plugin's own committed Python reference, each quoted with its command. §6 states
precisely what that does and does not prove.

Three things changed, in this order, because each one is the reason the next is possible:

1. **Semantics.** 17 of the 25 diff builders gained guards; the 25 now share one `mutations::guards`
   module, so a noun's `create-`/`replace-` twins run literally the same function instead of two
   transcriptions that had drifted apart.
2. **Schemas.** All 25 per-kind payload schemas, the snapshot schema's nine empty record `$defs`,
   the aggregate mutation schema (which was a copy of the snapshot schema) and the 25 leaf
   descriptors' `outcomeClasses` now describe what the code does.
3. **Evidence.** 23 new committed vectors (75 → **98**), and — for the first time — every one of the
   98 is replayed by all three languages instead of one row per kind.

---

## 1. The rule table, before → after

Guards are listed in the order the builder runs them; the first one that matches is the answer.
Bold marks what this wave added.

| kind | before | after | new codes |
|---|---|---|---|
| `create-node` | duplicate-id | duplicate-id → **finite coordinates** | `invariant` |
| `delete-node` | target-missing | target-missing (**deliberately unguarded**, see §2) | — |
| `create-element` | duplicate-id → start/end/material/section | duplicate-id → `guards::element_references` (same four, now shared) | — |
| `delete-element` | target-missing | target-missing → **refuses while a member UDL names it** | `target-referenced` |
| `replace-element` | target-missing → no-op | target-missing → **id-mismatch** → **the four foreign keys** → no-op | `id-mismatch`, `target-missing` |
| `create-material` | duplicate-id | duplicate-id → **e > 0, ρ > 0, −1 < ν < 0.5, all finite** | `invariant` |
| `delete-material` | target-missing | target-missing → **refuses while an element or region names it** | `target-referenced` |
| `replace-material` | target-missing → no-op | target-missing → **id-mismatch** → **the same plausibility bounds** → no-op | `id-mismatch`, `invariant` |
| `create-section` | duplicate-id | duplicate-id → **area > 0, iy > 0, finite** | `invariant` |
| `delete-section` | target-missing | target-missing → **refuses while an element names it** | `target-referenced` |
| `replace-section` | target-missing → no-op | target-missing → **id-mismatch** → **the same positivity bounds** → no-op | `id-mismatch`, `invariant` |
| `create-support` | duplicate-id → node | duplicate-id → `guards::node_reference` (same, now shared) | — |
| `delete-support` | target-missing | target-missing (**structurally unguardable**, see §2) | — |
| `replace-support` | target-missing → no-op | target-missing → **id-mismatch** → **`node_id` resolution** → no-op | `id-mismatch`, `target-missing` |
| `create-region` | duplicate-id → material | duplicate-id → material → **geometry** (≥3 outline points, non-zero area, thickness > 0, meshSize > 0, holes inside the outline) | `invariant` |
| `delete-region` | target-missing | target-missing → **refuses while an area load presses on it** | `target-referenced` |
| `replace-region` | target-missing → no-op | target-missing → **id-mismatch** → **`material_id` resolution** → **the same geometry bounds** → no-op | `id-mismatch`, `target-missing`, `invariant` |
| `create-load-case` | duplicate-id → per-load target | duplicate-id → `guards::load_reference` per load (same, now shared) | — |
| `delete-load-case` | target-missing | target-missing → **refuses while a combination weights it** | `target-referenced` |
| `add-load` | case → duplicate-load no-op | case → **the load's own node/element/region target** → duplicate-load no-op | `target-missing` |
| `remove-load` | case → load | unchanged | — |
| `change-load-case-self-weight` | case → no-op | unchanged | — |
| `create-combination` | duplicate-id → per-term target | unchanged | — |
| `delete-combination` | target-missing | target-missing → **refuses while another combination nests it** | `target-referenced` |
| `update-analysis-settings` | **no rejection branch at all** | **modalCount ≥ 1, bucklingCount ≥ 1, deformationScale finite > 0** → no-op | `invariant` |

### The outcome vocabulary (fem2d and fem3d; W12 matches)

| code | level | meaning |
|---|---|---|
| `mutation.duplicate-id` | Fatal | a `create-` id already exists |
| `mutation.id-mismatch` | **Fatal** | a `replace-`'s new record carries a different `id` than the one it selects |
| `mutation.invariant` | **Fatal** | the payload violates a document invariant — geometry or physical plausibility |
| `mutation.target-missing` | Error | a named record or foreign key does not exist on this base |
| `mutation.target-referenced` | **Error** | a `delete-` target is still named by other records; the address lists them |
| `mutation.no-op` | Warning | the payload asks for what the document already says |

**Why those two levels and not others.** `Fatal` says the payload is wrong against EVERY base — a
negative modulus, a two-point polygon, a replacement that renames its target — so no merge policy
may absorb it and no rebase can rescue it. `Error` says THIS base cannot host an otherwise
well-formed payload — the referrer set and the id set are properties of the document, and a
different document may well accept the same op. `mutation.invariant` is the repository's existing
generic invariant code (969 call sites, always Fatal); the two new codes are the ones the brief
names. `mutation.target-referenced` is a deliberate near-synonym of the energy plugin's
`mutation.target-in-use` — the fem vocabulary spells it as the brief specifies, and the divergence
is recorded here rather than silently resolved either way.

### Where the shared guards live

`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs`,
region `🛡️Guards` — a `pub mod guards` beside the dispatch enum, re-exported as
`crate::artifacts::fem2d::mutations::guards` through the existing `pub use component::*`. Sixteen
functions: `identity_matches`, `referenced`, `node_geometry`, `material_plausibility`,
`section_plausibility`, `region_geometry` (with `signed_area` and `point_in_ring`),
`analysis_bounds`, `node_reference`, `material_reference`, `element_references`, `load_reference`,
and the six `*_referrers` collectors.

It is a module and not sixteen copies for one reason: the twins are only meaningful if they agree,
and a transcription per leaf is EXACTLY how they came apart — `create-element` resolved four
references beside a `replace-element` that resolved none, and `create-load-case` resolved every
carried load beside an `add-load` that resolved none. Each leaf now spends one line per rule and the
rule has one definition. **No new directory** was created: adding a sibling under `🧬️mutations`
would have been read as an unregistered mutation-leaf directory by
`mutationVectorRegistryBreaches`.

---

## 2. The two `delete-` verbs that stay unguarded, and why

* **`delete-node` — specified, not overlooked.** Its committed vector is literally called
  `🚫️removes-node-n3-without-6eab3f` and its own docstring says "deliberately cascade-free: support
  `s3` survives still pointing at the removed node". A plan node is a drafting coordinate; dropping
  one an element still names is the behaviour the fixture pins. This is the documented exception the
  brief allows, and it is now also stated in the builder's own module doc so the next reader does
  not have to find the fixture to learn it.
* **`delete-support` — structurally unguardable.** No record in this vocabulary names a support id,
  so removing one can never orphan a reference. Its docstring now says so, and says that the reason
  differs from `delete-node`'s.

---

## 3. Schemas

### 3.1 Per-kind payload schemas — 25 files, all rewritten

Two defects, both pre-dating this ticket and both reported by W10 (F7) and W11 (finding 8):

* **the internally tagged discriminator was missing** and every schema declared
  `additionalProperties: false`, so **all 75** committed payloads failed their own kind's schema
  with `Additional properties are not allowed ('mutation' was unexpected)`;
* **the four enum-carrying kinds** (`create-element`, `replace-element`, `add-load`,
  `create-load-case`) spelled `FemElement`/`FemLoad` variants as the **PascalCase Rust names**
  (`"Bar"`, `"Beam"`, `"Nodal"`, `"MemberUdl"`, `"Area"`) with `kind` as their **only** property,
  while `#[value(rename_all = "camelCase")]` puts `"bar"`/`"beam"`/`"nodal"`/`"memberUdl"`/`"area"`
  and the variant's real fields on the wire — 12 further failures.

Both are fixed. Each schema now leads with

```json
"mutation": { "const": "createElement", "description": "The internally tagged discriminator …" }
```

as a required property, and each enum argument is a `oneOf` of closed objects carrying the camelCase
tag plus the variant's real fields.

### 3.2 Snapshot schema — `🌐️any/🧬️schema/📸️snapshot/🔣️.json`

Its nine record `$defs` were bare `{"title": …, "type": "object"}` stubs, so snapshot validation
passed **vacuously**: it proved the nine members existed and nothing about their contents (W11
finding 9, and the Python references' own docstrings called it "a defect in the specification").
All nine are now filled from the Rust structs in `🗿️artifacts/◻️2d/🦀️.rs`, and they carry the
admissibility bounds as well as the field shapes:

| `$def` | bound added |
|---|---|
| `FemMaterial` | `e`, `rho` `exclusiveMinimum: 0`; `nu` `exclusiveMinimum: -1`, `exclusiveMaximum: 0.5` |
| `FemSection` | `area`, `iy` `exclusiveMinimum: 0` |
| `FemRegion` | `outline` `minItems: 3`; each hole `minItems: 3`; `thickness`, `meshSize` `exclusiveMinimum: 0` |
| `FemAnalysisSettings` | `modalCount`, `bucklingCount` `minimum: 1`; `deformationScale` `exclusiveMinimum: 0` |
| `FemNode` | coordinates are plain `number`s, so a NaN or an infinity is already excluded |
| `FemElement`, `FemLoad` | camelCase-tagged `oneOf` with the variants' real fields |
| `FemSupport` | `fixed` is an array of the six-value `FemDof` enum |
| `FemLoadCase`, `FemCombination` | full record shapes, `FemLoad` / `FemCombinationTerm` inlined |

**The division of labour between the two schema families is deliberate.** A payload carrying a
negative Young's modulus is a perfectly WELL-FORMED payload; what refuses it is the DOCUMENT's
invariant. So the per-kind payload schemas stay structural, and the bounds live in the snapshot
schema — where no valid fem2d document may violate them — each annotated with
`x-semio-invariant: "mutations::guards::material_plausibility — a positive Young's modulus"` so a
reader gets from the invariant to the code that keeps it in one hop. This is also what keeps
"every committed payload validates" true: the 23 new vectors carry deliberately inadmissible
payloads, and they are still well-formed wire shapes.

### 3.3 Aggregate mutation schema — `🌐️any/🧬️schema/🧬️mutations/🔣️.json`

Was a **verbatim copy of the snapshot schema** with `title` changed to `Fem2dMutation`, so it
required a mutation to carry `nodes`/`elements`/… and every real payload failed it. Both fem2d and
fem3d shipped that copy and both oracle catalogs' `rationale` fields report it. It is now the closed
25-variant tagged union: a `oneOf` over 25 `$defs`, each the same shape its kind's own
`🧬️.schema.json` declares. It is `include_str!`-ed into the schema declaration
(`🌐️any/🧬️schema/🦀️.rs:149`), so this is the shape the plugin actually publishes.

### 3.4 Leaf descriptors — `<kind>/🔣️.json`, `outcomeClasses`

Wrong in both directions, and for a traceable reason: they were produced by the v2-manifest
scaffolder in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:3479`,
which scans a `🔺️diff/🦀️.rs` for `MutationOutcome::error`, `::empty` and `::new` and has **no case
for `::fatal` or for the chained `.warn(…)`**. So `create-node`, `create-section` and
`create-material` were recorded as `["applied"]` although each raises a Fatal
`mutation.duplicate-id`, and every no-op was recorded as `info` although it is raised at `Warning`.

All 25 now declare the classes their builder can actually reach. Both spellings are in the derive's
own enum (`✨️derive/🦀️.rs:625` accepts `applied|info|warning|error|fatal`) and in the TS
`outcomeClassesOf`, which maps `warning`→applied and `fatal`→rejected, so nothing downstream has to
change. **The scaffolder's two missing cases are not fixed here** — that file is outside this
slice — and are the reason the same drift will reappear on any leaf it regenerates.

---

## 4. Evidence

### 4.1 The 23 new vectors

One per NEW refusal branch, all on W10's two-storey braced steel frame, all with
`➡️after` byte-identical to `⬅️before` and `🔺️diff/🚫️.absent`.

| kind | case | code | what it pins |
|---|---|---|---|
| `delete-element` | `🔗️blocks-udl-a1df8e` | `target-referenced` | the floor beam two member UDLs still load |
| `replace-element` | `🪪️denies-rename-0d46d8` | `id-mismatch` | renaming the roof beam through a replace |
| `replace-element` | `🚫️dangling-start-cda887` | `target-missing` | a replacement naming a `start` node that is gone |
| `create-material` | `⚗️denies-poisson-329e35` | `invariant` | an elastomeric bearing at ν = 0.5 (incompressible) |
| `delete-material` | `🔗️blocks-in-use-e99619` | `target-referenced` | the S355 grade all seven members are made of |
| `replace-material` | `🪪️denies-rename-a0d7aa` | `id-mismatch` | regrading the concrete by renaming its row |
| `replace-material` | `⚗️denies-zero-modulus-71e69a` | `invariant` | the half-filled form: modulus left at zero |
| `create-section` | `⚗️denies-zero-area-58b5ca` | `invariant` | an IPE 100 whose area was never entered |
| `delete-section` | `🔗️blocks-in-use-0a6a3c` | `target-referenced` | the HEB 200 profile four columns carry |
| `replace-section` | `🪪️denies-rename-1d02dd` | `id-mismatch` | renaming the roof-beam profile |
| `replace-section` | `⚗️denies-zero-iy-404e31` | `invariant` | a beam profile with no bending stiffness |
| `replace-support` | `🪪️denies-rename-63ec90` | `id-mismatch` | a rename that orphans nothing and is still refused |
| `replace-support` | `👻️dangling-node-98d979` | `target-missing` | moving the roof tie onto a node that is gone |
| `create-region` | `📐️denies-two-point-99954a` | `invariant` | a slab outlined by two points |
| `create-region` | `🕳️denies-loose-hole-d9efa1` | `invariant` | a door opening drawn beside the panel, not in it |
| `delete-region` | `🔗️blocks-in-use-7c7862` | `target-referenced` | the infill panel the wind case presses on |
| `replace-region` | `🪪️denies-rename-574c91` | `id-mismatch` | renaming the panel while presenting a geometry edit |
| `replace-region` | `👻️dangling-mat-7ef81b` | `target-missing` | repouring it in a grade the model lacks |
| `replace-region` | `📐️denies-zero-thick-7d805e` | `invariant` | a panel present in the mesh, absent from the structure |
| `delete-load-case` | `🔗️blocks-in-use-7cdc5f` | `target-referenced` | the dead case all three combinations weight |
| `add-load` | `👻️dangling-node-8d113b` | `target-missing` | the asymmetry with `create-load-case`, closed |
| `delete-combination` | `🔗️blocks-in-use-0b898b` | `target-referenced` | an ULS a design envelope nests |
| `update-analysis-settings` | `🚫️denies-zero-modes-babc1d` | `invariant` | the first vector this kind ever had for a rejection |

(Directory names above are the committed ones, verbatim.)

**One new base model, and only one.** `delete-combination`'s branch needs a combination that NESTS
another and the steel frame's three are flat, so that case's `⬅️before` is the frame plus one
design-envelope combination over `uls1` and `sls1`. Everything else replays the frame verbatim.

Each case carries 5 `#[test]`s: the document is untouched, the diagnostic is the declared code /
level / address, the inverse has the shape the verb's own `↩️inverse` gives it, the committed
outcome matches what the builder emits, and both snapshots are canonical.

**No existing vector changed.** All 75 pre-existing ones were replayed against the hardened rules
before any of them was written: 0 behaved differently. That is not luck — every committed `delete-`
happy path already targets an unreferenced trailing spare, every `replace-` already carries a
matching id and resolvable keys, and every committed region and material is already admissible.

### 4.2 Rust unit tests beside the dispatch enum

`🌐️any/🧬️schema/🧬️mutations/🦀️.rs` gained a `🛡️GuardLaws` region: ten tests, one per new rule
family, each reading `(code, level, target)` off the outcome. The most load-bearing is
`add_load_dangling_node_is_error`, which asserts that the SAME load on the same missing node is
refused **identically** through `add-load` and through `create-load-case` — the asymmetry itself,
turned into an equality.

Three pre-existing tests in that file had to change, and the diff says why in place:

* `material_create_replace_and_delete_round_trip` and `section_create_replace_and_delete_round_trip`
  deleted `steel` / `ipe300` while beam `e1` still named them. The shared fixture gained a trailing
  unreferenced spare (`timber_spare`, `shs_spare`) and the round trips delete that instead — which
  is also the only position whose `create-`-shaped inverse restores a byte-identical vec order.
* `mutation_law_add_load_inverse_and_diff_absorb` added an area pressure over a region `r1` that
  fixture never had; it is now a member UDL on the beam it does have.

### 4.3 All three languages now see all 98 vectors

Case discovery is explicit in every language, which is why 73 of the 98 were Rust-only evidence
before this wave (W10 F9, W11 §5). Per subset:

* **`🥒️.feature`** — two new `Scenario Outline`s. `@id-frame-vector` replays the steel-frame happy
  path (scenario id `frame-vector-<kind>`); `@id-reject` replays every refusal and no-op vector
  (`reject-<kind>-<n>`, numbered in the catalog's own order). Scenario ids are unique within a
  feature (`🟦️.ts:427`), which is why the numbering exists.
* **`🐍️.py`** — the independent Python reference, extended with the hardened rules and a
  `reject_handler`. Written to both the subset-owned case and the `🌐️any` duplicate, which are
  byte-identical and location-independent.
* **`🦀️.rs`** — the Rust SUBJECT half. `spec_vector` became `committed_vector`, keyed by SCENARIO
  id instead of by kind (a kind now has up to six vectors), `Vector.diff` became
  `Option<&'static str>` because a refusal bundle carries `🔺️diff/🚫️.absent`, and a `reject`
  handler was added. Two generated tables, `COMMITTED` and `REFUSED`, drive the registration.

**What makes the refusal rows a real differential.** Two implementations that both merely decline to
move a document agree vacuously. Both sides therefore project `{model, refusal}` where `refusal` is
`{code, level, target}`, so the comparison forces them to refuse for the SAME reason, at the same
severity, naming the same records. The Python `observable()` law — which fails any vector whose
`before == after`, and is why these rows could never have been routed through `spec_vector_handler`
— is replaced for these rows by a `refused()` law that checks exactly that triple against the
committed `🎯️outcome`.

The `@id-reject` Outline declares a fourth fixture step, `🎯️outcome/🔣️.json`, because the reference
needs the declared code to check itself against; the Rust half reads the same file through
`include_str!`.

---

## 5. Verification — commands and output

All run from the ticket folder. **No `cargo`, `bun`, `nx` or git-mutating command was run.**

### (i) Replay + third-party schema + structure — `uv run --with jsonschema python 🔨️w13-replay-fem2d.py`

```
vectors replayed: 98  (applied 50, refused 48, diverged 0)
jsonschema (third-party) failures: 0
structure problems: 0
reference divergences: 0
```

* **98 vectors**, every one driven through the committed independent Python reference:
  50 forward (the reference reproduces `➡️after`, the document moves, exactly one of the nine
  members moves, and the reference's own computed inverse restores `⬅️before`) and 48 refused (the
  document is untouched AND the code, level and address match the committed `🎯️outcome`).
* **0 divergences** — this replaces W10's four. Those four were `create-element`,
  `create-support`, `create-load-case` and `create-combination` dangling-reference refusals that the
  reference ACCEPTED because its generic `create-` path checked only id uniqueness (W10 finding F8).
  The reference now runs the same guards, so the refusal half of the vocabulary finally has a
  cross-language oracle.
* **jsonschema 4.26.0** (PyPI, MIT — a real external implementation) validated, per vector, the
  `⬅️before` and `➡️after` against the snapshot schema, the payload against its kind's own
  `🧬️.schema.json` (Draft 7) and the same payload against the aggregate `🧬️mutations/🔣️.json`
  (Draft 2020-12): **392 validations, 0 failures**. Before this wave the first leg was vacuous
  (empty `$defs`) and the other two failed on all 75.
* **structure**: catalog `scenarios[]` ↔ on-disk `🧪️tests` children set-equal in BOTH directions;
  `id` is the stem of `directoryName`; every directory registered in taxonomy `members-of-tests`;
  every bundle the closed 13-node shape with exactly one diff alternative; sibling leading emojis
  collision-free inside every `🧪️tests`; and every feature `Examples` row expands to a scenario id
  the reference actually registers and names a case that exists on disk.

### (ii) The 50 real-model feature rows, through the hardened reference

```
real-model feature rows through the committed reference: 50, failures 0
```

Every `mutate-<kind>` and `inverse-<kind>` row of all five subset features, applied to the derived
timber portal frame by the regenerated `🐍️.py`, with `observable`, `touches_one` and `restores`.
None of the 25 kinds' real-model parameters is refused by the new guards.

### (iii) Mounts and fixture reads — `python3 🔨️resolve-mounts.py --tree`

```
── ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs
   checked:    487      resolved: 487      unresolved: 0
── include_str! across ✏️s/🔌️plugins/🏗️fem
   checked:    1806 → 2123                 unresolved: 0
TOTAL unresolved: 0
```

(487 and 2123 are whole-plugin counts, not W13's alone — W12 mounts concurrently on the 3d side.
W13's own share is 23 `#[path]` leaves and 317 new `include_str!` reads: 4 per new refusal bundle
plus the regenerated subject adapters' vector tables.)

### (iv) Parse checks

```
rustfmt --edition 2024: 248 files, 0 failures      (every fem2d 🦀️.rs + the plugin crate entry)
python parse:            12 files, 0 failures      (the 10 references + 2 subject-side siblings)
json parse:             638 files, 0 failures      (every 🔣️.json under ◻️2d)
taxonomy.json parses; members-of-tests 1234 names (23 added here), 0 duplicates, all NFC
```

### (v) Naming

```
case directories: 110, over 28 code points: 0 of the 23 new ones, non-NFC: 0
```

Eleven pre-existing names exceed the 28-code-point Windows budget this ticket adopted; none of them
was authored here. Every new directory carries a handpicked leading emoji unique among its own
`🧪️tests` siblings — `pathEmojiStatuteFindings` raises a `duplicate` for two siblings sharing one.

### (vi) Idempotence

```
🔨️w13-fem2d-cases.py    --check → cases: 23, files: same 138
🔨️w13-fem2d-schemas.py  --check → schemas: same 27, descriptors: same 25
🔨️w13-fem2d-oracles.py  --check → features + references + subjects: same 20
🔨️w13-fem2d-register.py --check → mounts 0, catalog scenarios 0, taxonomy names 0
```

Every authoring script reproduces the committed tree exactly, so the evidence on disk is what the
declared rules generate and not a hand-edited approximation of it.

---

## 6. What is NOT proved

**No `cargo` ran.** Nothing here shows that the guards module type-checks, that
`protocol::MutationOutcome::fatal`'s `impl IntoIterator<Item = impl Into<String>>` accepts every
call site's argument, that the regenerated subject adapters compile against
`semio_repo_test_host`'s current `Adapter`, or that the 23 new case files type-check. `rustfmt`
proves they PARSE.

Three specific spellings are transcription risks the first build will settle: `messages[0].code.0`
and `protocol::Severity::{Fatal,Error}` (the spellings the `🕸️dag`/`🔱️trinity` rejection fixtures
use), `undo.new_element.as_ref()` on a `Box<FemElement>` (the spelling the production
`replace-element` diff builder already uses), and the `&String == &str` comparisons inside the
`matches!` guards of `element_referrers` / `region_referrers`.

**The first `cargo test -p semio-s-plugin-fem` after this wave is the real gate**, and the second is
the repo test coordinator over the five widened features — which is where the 96 new
oracle/subject scenario pairs (23 reject rows + 25 frame rows, each in both roles) actually execute.

**One live coupling to watch.** The new `delete-` guards change behaviour for any caller that
deletes a referenced record. Every caller inside `◻️2d` was checked: `🗂️remove-selection`'s three
tests delete only unreferenced records; `🧱️add-material` supplies ν = 0.3, ρ = 7850 and `🗺️add-region`
thickness 0.02, meshSize 0.25, so both stay inside the new bounds; `🧮️set-analysis-settings` merges
onto the current settings, which start at `{3, 3, 50.0}`. The DSL command parser
(`✏️editor/🦀️.rs:848`) uses `number("area").unwrap_or_default()`, so an `addSection` command that
omits `area` now emits a mutation the document refuses instead of silently storing a zero-area
profile — that is the intended hardening, and no committed test exercises that path.

---

## 7. Files

**Added** — 23 case bundles × 6 files = **138** files under
`◻️2d/🏅️standards/🔖️1/🪆️subsets/{🕸️mesh,🧱️material,🛡️boundary,🏋️load,📈️analysis}/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`
(`🦀️.rs`, `🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`,
`🎯️outcome/🔣️.json`, `🔺️diff/🚫️.absent`).

**Modified**

* `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` — the `🛡️Guards` module, the `🛡️GuardLaws` test region, two
  trailing spares in the shared Rust fixture and three repaired round-trip/law tests.
* 17 `<kind>/🔺️diff/🦀️.rs` rewritten (the guards wired in) and 2 more (`🕳️delete-node`,
  `🗑️delete-support`) given the module doc that states why they carry no referential guard.
* 25 `<kind>/🧬️.schema.json` — the `mutation` tag and the real enum variants.
* 25 `<kind>/🔣️.json` — truthful `outcomeClasses`.
* `🌐️any/🧬️schema/📸️snapshot/🔣️.json` — nine filled record `$defs` with their bounds.
* `🌐️any/🧬️schema/🧬️mutations/🔣️.json` — the 25-variant tagged union, replacing the snapshot copy.
* 5 `<subset>/🧪️tests/<case>/🥒️.feature` — the `@id-frame-vector` and `@id-reject` Outlines.
* 10 `🐍️.py` (5 subset-owned + 5 `🌐️any` duplicates) — the hardened reference and `reject_handler`.
* 5 `<subset>/🧪️tests/<case>/🦀️.rs` — `committed_vector`, `reject`, the two scenario tables.
* 5 `<subset>/🔮️oracle/🔣️.json` — 23 `scenarios[]` rows.
* `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` — 23 `#[cfg(test)]` mounts, appended beside each
  kind's existing ones.
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — 23 `members-of-tests` rows,
  inserted as one contiguous block after a unique anchor line.

**Ticket**

* `🔨️w13-fem2d-rules.py` — the hardened guard order, transcribed once, used to author and audit.
* `🔨️w13-fem2d-schemas.py` — the JSON-Schema and descriptor generator (`--check`).
* `🔨️w13-fem2d-cases.py` — the 23 case specifications and the Rust template (`--check`).
* `🔨️w13-fem2d-register.py` — the three anchored registration patches (`--check`).
* `🔨️w13-fem2d-oracles.py` — the feature / reference / subject generator (`--check`).
* `🔨️w13-replay-fem2d.py` — the verifier of §5 (i), replacing `🔨️w10-replay-fem2d.py`.
* this report.
