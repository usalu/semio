# 🧊 W12 — fem3d mutation semantics, schemas and the three-language differential

Scope: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/**` (mutation `🔺️diff`/`↩️inverse` code, the shared guard
region, `🧬️.schema.json` × 4, the snapshot and mutation facet schemas, 21 new fixture bundles, the
five `🥒️.feature`/`🐍️.py`/`🦀️.rs` differential triples, the five `🔮️oracle/🔣️.json` catalogs) plus the
fem3d rows of `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Nothing under
`◻️2d` was touched.

**NOTHING WAS COMPILED.** The host's swap is exhausted and the coordinator compiles later. No
`cargo`, `bun`, `nx`, `npm` or state-modifying `git` command was run. Every claim below is a
`rustfmt --edition 2024` parse, a `json.load`, an `ast.parse`, a `jsonschema` validation, a static
`#[path]`/`include_str!` resolution, or a replay through the plugin's own Python reference — each
named with its command and its output in §7.

Before → after: **75 → 96 vectors**, **12 → 0 schema failures**, **5 → 0 DIVERGE**, **0 → 46**
vectors that pin a refusal or a declared no-op in all three languages.

---

## 1. The rule table — every kind, before → after

`D` = `mutation.duplicate-id` (Fatal) · `M` = `mutation.target-missing` (Error) ·
`N` = `mutation.no-op` (Warning) · `R` = `mutation.target-referenced` (Error, **new**) ·
`I` = `mutation.id-mismatch` (Fatal, **new**) · `V` = `mutation.invariant` (Fatal, **new here**).

| kind | before | after | what changed |
|---|---|---|---|
| `create-node` | `D` | `D` → `V` | node position must be finite |
| `delete-node` | `M` | `M` | **unchanged, deliberately** — see §3 |
| `create-element` | `D` → `M`(start,end,mat,sec) | same, via the shared `resolve_element` | no behaviour change; the four checks moved into a guard the replace now shares |
| `delete-element` | `M` | `M` → `R` | refuses while a member UDL names it |
| `replace-element` | `M` → `N` | `M` → `I` → `N` → `M`(start,end,mat,sec) | cannot rename; resolves the same four keys `create-element` does |
| `create-section` | `D` | `D` → `V` | `area`/`iy`/`iz`/`j` must be positive and finite |
| `delete-section` | `M` | `M` → `R` | refuses while an element names it |
| `replace-section` | `M` → `N` | `M` → `I` → `N` → `V` | cannot rename; same positivity bound |
| `create-solid` | `D` → `M`(material) | `D` → `M`(material) → `V` | geometry: ≥3 outline points, non-zero ring area, `height > 0`, `layers ≥ 1`, `meshSize > 0`, every hole non-degenerate and inside the outline |
| `delete-solid` | `M` | `M` → `R` | refuses while an area load names it |
| `replace-solid` | `M` → `N` | `M` → `I` → `N` → `M`(material) → `V` | cannot rename; resolves `materialId`; same geometry gate |
| `create-material` | `D` | `D` → `V` | `e`,`g`,`rho > 0` and `-1 < nu < 0.5` |
| `delete-material` | `M` | `M` → `R` | refuses while an element or solid names it |
| `replace-material` | `M` → `N` | `M` → `I` → `N` → `V` | cannot rename; same plausibility bound |
| `create-support` | `D` → `M`(node) | unchanged | already resolved its one key |
| `delete-support` | `M` | `M` | a support is a **leaf** of the reference graph — nothing points at it |
| `replace-support` | `M` → `N` | `M` → `I` → `N` → `M`(node) | cannot rename; resolves `nodeId` |
| `create-load-case` | `D` → `M`(each load's target) | same, via the shared `resolve_load` | no behaviour change; the check moved into a guard `add-load` now shares |
| `delete-load-case` | `M` | `M` → `R` | refuses while a combination weights it |
| `add-load` | `M`(case) → `N` | `M`(case) → `N` → `M`(load target) | **the asymmetry W11 found as finding 1 is gone** — `add-load` resolves a load's node/element/solid exactly as `create-load-case` does |
| `remove-load` | `M` → `M` | unchanged | |
| `change-load-case-self-weight` | `M` → `N` | unchanged | |
| `create-combination` | `D` → `M`(term case) | unchanged | |
| `delete-combination` | `M` | `M` | a combination is a **leaf** of the reference graph |
| `update-analysis-settings` | `N` | `N` → `V` | `modalCount`/`bucklingCount ≥ 1`, `deformationScale` finite and `> 0` |

### Guard order, and why

Identity first (`duplicate-id` / `target-missing` / `id-mismatch`), then the no-op short-circuit,
then foreign keys, then values. The no-op sits **before** the key and value checks on every
`replace-`: a payload equal to the record already there asks for nothing, so there is nothing to
validate. Every pre-existing `⏸️same-…` vector therefore keeps its committed verdict unchanged.

`create-` keeps its original layout — identity, then keys, then values — so W11's four
`🚨️dangling-…` vectors still pin exactly the branch they were authored for.

### Code-name choices

* `mutation.invariant` is **not** a new code: it is the repo-wide name for a value or geometry
  breach, used at `Fatal` by `🔱️trinity`'s `create-edge`/`move-node` and by `📸️remodel`'s
  `update-geo-params`/`update-camera-calibration` (969 occurrences repo-wide). W12 mirrors it rather
  than inventing a fem-local `invalid-geometry`/`implausible-value` pair.
* `mutation.target-referenced` is new to the repo; its nearest sibling is `🔋️energy`'s
  `mutation.target-in-use` `Fault` ("… is still referenced by at least one …"). It is raised at
  `Error`, the level and empty-diff shape `mutation.target-missing` already uses, and its `target`
  is **the target followed by every referrer**, so a caller can offer to release them.
* `mutation.id-mismatch` is new; `Fatal`, the level `mutation.duplicate-id` uses for the other half
  of the same identity contract.

### Where the guards live

One `//#region 🛡️Guards` in `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/🦀️.rs`
(the `crate::artifacts::fem3d::mutations` module itself), carrying `target_referenced`, `id_mismatch`,
`invariant`, the six `*_referrers` scans, `resolve_element`, `resolve_load`, and the five `*_breach`
predicates. Each per-kind `🔺️diff/🦀️.rs` calls them by name. CLAUDE.md's "if code is repeated, it
MUST be close to each other": the twelve verbs that share a rule now share one implementation of it,
one module up from the leaves that use it, and each leaf still reads as its own complete decision.

`solid_breach` deliberately restates the gate
`🌐️any/✏️editor/🧵️session/🦀️.rs:806` already applies before meshing
(`layers == 0 || height <= 0.0 || mesh_size <= 0.0 || outline.len() < 3`) — the point of moving it
into the mutation is that a document never reaches the mesher already unmeshable.

---

## 2. The one rule that has no fixture

`create-node`'s finite-coordinate guard is implemented and unreachable from a committed vector: JSON
has no literal for `NaN` or `±Infinity`, so a `📸️snapshot`/`🦠️mutation` file cannot carry one, and a
`1e400` spelling would break the `committed_json_is_canonical` fixed-point test on re-encoding. The
guard is stated because the rule is about the model, not about the carrier; the same reasoning is
written into the reference's own `finite()` docstring. Every OTHER new rule has at least one vector.

---

## 3. `delete-node` — the one deliberate asymmetry

The brief permits keeping `delete-node`'s permissive behaviour **only if a test or doc explicitly
specifies it**. One does, in so many words:

`…/🕸️mesh/🧬️schema/🧬️mutations/🕳️delete-node/🧪️tests/🚫️removes-the-column-head-056295/🦀️.rs:8`

> `delete-node` is cascade-free: frame `f1` keeps naming `n3` as its end node after the node is gone.

and its committed `➡️after` encodes exactly that: node `n3` removed, frame `f1` still naming it. That
is a specified behaviour, not an omission, so `delete-node` keeps refusing on `target-missing` only.
The asymmetry is recorded in the file itself (a new module docstring on
`🕳️delete-node/🔺️diff/🦀️.rs`) rather than left to be rediscovered, and `node_referrers` — the scan a
future cascade or refusal would use — is written and exported beside the other five.

`delete-support` and `delete-combination` are also exempt, but for a different and permanent reason:
both are **leaves** of the reference graph. Nothing in `Fem3dSnapshot` points at a support or a
combination, so there is no referrer to protect. Both files say so.

---

## 4. The dead `"rejected"` branch

W11 §3 identified it: the boilerplate `declared_outcome_holds` on all 25 pre-existing cases read
refusal off the `Result` (`assert!(!applied, …)`), and **a refused fem3d mutation returns `Ok`** —
`🌐️any/🧬️schema/🧬️mutations/🦀️.rs` delegates to `vcs::apply_mutation`, which is documented
policy-agnostic, and `MutationOutcome::fatal`/`::error` force `diff = D::default()`
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1146,1152`), so the empty diff applies cleanly.
The branch could never fire.

All 25 fem3d cases now read refusal off the OUTCOME:

```rust
let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
let refused = produced.messages().iter().any(|message| message.level >= protocol::Severity::Error);
…
"rejected" => {
    assert!(refused, "…: declared rejected but the diff builder raised no Error or Fatal, only {:?}", produced.messages());
    assert_eq!(produced.diff(), &crate::artifacts::fem3d::diff::Fem3dDiff::default(), "…: a refused mutation must carry the empty diff");
    assert_eq!(snapshot, before(), "…: a refused mutation must leave the snapshot untouched");
}
```

and the docstring above the test now says why the `Result` is not the place to look. The 21 new
rejection cases and W11's 19 assert on `messages()` (code, level, target) the same way.

---

## 5. Schemas

### 5.1 The four broken per-kind payload schemas — fixed

`create-element`, `replace-element`, `add-load` and `create-load-case` described their
`FemElement`/`FemLoad` argument as a `oneOf` of objects whose `kind` was the **PascalCase Rust
variant name** (`"Bar"`, `"Frame"`, `"Nodal"`, `"MemberUdl"`, `"Area"`) and which declared
`additionalProperties: false` with `kind` as their **only** property. The wire form is camelCase and
carries the variant's real fields, so **every committed vector of those four kinds failed its own
schema** — 12 failures, including three pre-existing ones. Each now carries the real variants:

| variant | `kind` const | fields |
|---|---|---|
| bar | `"bar"` | `id`, `start`, `end`, `materialId`, `sectionId` |
| frame | `"frame"` | … plus `roll` |
| nodal | `"nodal"` | `id`, `nodeId`, `dof` (enum `Tx Ty Tz Rx Ry Rz`), `value` |
| memberUdl | `"memberUdl"` | `id`, `elementId`, `wx`, `wy`, `wz` |
| area | `"area"` | `id`, `solidId`, `pressure` |

`create-load-case` embeds the load `oneOf` inside `loadCase.loads.items`.

### 5.2 The snapshot schema's empty `$defs` — filled

`🌐️any/🧬️schema/📸️snapshot/🔣️.json` declared all eight record types as bare
`{"title": …, "type": "object"}`, so snapshot validation passed **vacuously**: it proved the nine
members exist and nothing about their contents. `FemNode`, `FemElement`, `FemMaterial`, `FemSection`,
`FemSolid`, `FemSupport`, `FemLoadCase` and `FemCombination` now carry their real shapes, read off
the Rust structs in `🗿️artifacts/🧊️3d/🦀️.rs:37-175`, in the same style the already-correct
`FemAnalysisSettings`/`FemCamera` defs use (`additionalProperties: false`, alphabetical `required`,
declaration-order `properties`).

### 5.3 The mutation facet schema — rewritten

`🌐️any/🧬️schema/🧬️mutations/🔣️.json` was a **verbatim copy of the snapshot schema** with `title`
changed to `Fem3dMutation` — a defect the plugin's own Python references name in their docstrings.
It is now the real internally tagged union: 25 `oneOf` variants in `Fem3dMutation`'s declaration
order, each `{mutation: {const: <camelCase tag>}, …that kind's own payload properties}`, generated by
merging each `🧬️.schema.json` so the two can never drift.

### 5.4 What the schemas deliberately do NOT carry

The physical bounds (`e > 0`, `-1 < nu < 0.5`, `area > 0`, `layers ≥ 1`, `minItems: 3` on an outline)
are **not** baked into the JSON Schemas. A JSON Schema describes what is *representable on the wire*;
the diff builders describe what is *admissible into the document*. A rejection vector has to be
representable to be testable at all — `🧨️nu-at-a-half-8253d2` carries `nu: 0.5` on purpose — so
tightening the schema would make the refusal fixtures unschemable and delete the evidence. This is
stated here because it is a decision, not an omission.

### 5.5 Result

All 96 vectors × 3 validations (before snapshot, after snapshot, payload) + 96 tagged envelopes:
**0 failures**, and the snapshot validation is no longer vacuous.

---

## 6. Fixtures — 21 new refusal vectors

Every `⬅️before` is the SAME glulam workshop hall W11 introduced, **read back off disk** from
`🧩️create-element/🧪️tests/🏗️hall-new-tie-074a69/📸️snapshot/⬅️before/🔣️.json` rather than re-derived,
so there is exactly one copy of that model's truth in the repository. Every `➡️after` is that model
again — a refusal moves nothing. Each bundle is the closed 12-node rejection shape (`🦀️.rs`,
`🦠️mutation/🔣️.json`, both snapshots, `🎯️outcome/🔣️.json`, `🔺️diff/🚫️.absent`).

| kind | new vector | code | what it pins |
|---|---|---|---|
| `delete-element` | `⛓️rafter-under-udl-e0342d` | `target-referenced` | `raf_l_1` carries `ld_roof` + `ld_snow_l` |
| `replace-element` | `🪪️renames-brace-219be2` | `id-mismatch` | `brc_0` → `brc_9` |
| `replace-element` | `🚨️dangling-sec-70b168` | `target-missing` | section `sec_x`; start/end/material resolve first |
| `create-section` | `🧨️zero-area-475a19` | `invariant` | `area = 0` ⇒ singular `EA/L` |
| `delete-section` | `⛓️purlin-in-use-99eb01` | `target-referenced` | `sec_pur` shared by six purlins |
| `replace-section` | `🪪️renames-purlin-dfe160` | `id-mismatch` | `sec_pur` → `sec_pur_v2` |
| `replace-section` | `🧨️negative-iy-d4e0a8` | `invariant` | negative second moment |
| `create-solid` | `📐️sliver-outline-316a7c` | `invariant` | three collinear points, zero ring area |
| `delete-solid` | `⛓️raft-under-load-e4ea39` | `target-referenced` | `slab_raft` under `ld_floor` |
| `replace-solid` | `🪪️renames-apron-7bfadd` | `id-mismatch` | unreferenced and refused anyway |
| `replace-solid` | `📐️zero-height-2b131a` | `invariant` | a solid extruded by nothing |
| `replace-solid` | `🚨️dangling-mat-9c89da` | `target-missing` | `c40_50` never added |
| `create-material` | `🧨️nu-at-a-half-8253d2` | `invariant` | `nu = 0.5` is incompressible |
| `delete-material` | `⛓️glulam-in-use-1208e1` | `target-referenced` | `gl24h` under twelve members |
| `replace-material` | `🪪️renames-c24-b60696` | `id-mismatch` | W11 finding 3, made a refusal |
| `replace-material` | `🧨️negative-e-84dad7` | `invariant` | negative Young's modulus |
| `replace-support` | `🪪️renames-pin-29f41a` | `id-mismatch` | leaf record, refused anyway |
| `replace-support` | `🚨️dangling-node-d44469` | `target-missing` | `n_ext_z` |
| `delete-load-case` | `⛓️dead-in-combos-e73167` | `target-referenced` | all three combinations weight `dead` |
| `add-load` | `🚨️no-such-member-3fe6e9` | `target-missing` | W11 finding 1, made a refusal |
| `update-analysis-settings` | `🧨️zero-modes-a27c74` | `invariant` | `modalCount = 0`, empty path |

Names are ≤ 25 characters (budget 28), NFC, and collision-free in each kind's sibling leading-emoji
namespace (`⛓️` referential, `🪪️` identity, `🧨️` plausibility, `📐️` geometry, `🚨️` dangling key).
Every one is mounted in `📦️packages/🦀️rust/🦀️.rs`, registered in its subset's `🔮️oracle/🔣️.json`
`scenarios[]` and in taxonomy `members-of-tests`.

**No pre-existing vector needed re-authoring.** `🔨️w12-audit-fem3d.py` re-derived the outcome of all
75 committed vectors under the hardened rules before a line of Rust was written and reported
0 conflicts: every committed `🎯️outcome`, `🔺️diff` and `➡️after` already described what the hardened
code does. Per kind the count is now `3 + n`, minimum 3, maximum 6 (`replace-solid`).

### Three in-crate Rust tests that DID need re-authoring

`🌐️any/🧬️schema/🧬️mutations/🦀️.rs`'s own `mod tests` reached past the new rules:

1. `material_create_replace_and_delete_round_trip` deleted `steel` while frame `e1` named it — now
   creates an unreferenced `alu` spare and deletes that instead (and finally exercises the `create-`
   its own name claims).
2. `section_create_replace_and_delete_round_trip` deleted `hea200` while `e1` named it — same fix
   with an `shs120` spare.
3. `mutation_law_add_load_inverse_and_diff_absorb` hung an `Area` load on `sol1` in the cantilever
   fixture, which has no solids — now runs on `solid_slab_doc()`, which has both `sol1` and the
   `self` case, so the law is proved instead of passing vacuously.

Each would have kept PASSING after the hardening (a refused forward plus a refused inverse still
lands back on the base), which is exactly why they had to be fixed: they would have gone quietly
vacuous. `round_trip`'s docstring now states that every delete below it must strike an unreferenced
record.

Also checked and clean: all 50 real-model `@id-mutate`/`@id-inverse` feature rows still apply to the
steel frame under the hardened rules (§7 v); the `🗂️remove-selection`, `🧊️add-solid`,
`🧱️add-material` and `🧮️set-analysis-settings` command handlers and their tests emit only admissible
records (`mesh_size` defaults to `0.5`, materials to `nu = 0.3`/`rho = 7850`, settings to `3/3/50`).

---

## 7. The Python reference, the feature tables and the five DIVERGE

### 7.1 What W11 left open, and what it cost

W11 §5 could not wire its 50 new vectors into the `.feature` `Examples` tables: the reference
registers exactly `mutate-`/`inverse-`/`spec-vector-<kind>`, and its `observable()` law fails any
vector whose `before == after`, so no rejection or no-op could ever be replayed. 50 of 75 vectors
were Rust-only. W12 closes it, and the closure needed **three** files per subset, not two — the test
platform's `validateRegistration`
(`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:451`) requires **both**
a subject and an oracle registration for every planned scenario, so a feature row with only a Python
handler is a hard plan-time failure, not a skipped row.

### 7.2 The reference (`🌐️any/🧪️tests/*/🐍️.py`, all five)

New `# region 🔖️Integrity`, written from the record shapes the schema and the vectors state, in the
reference's own idiom (`AssertionError`, not ported code): `every_load`, `referrers`, `resolves`,
`resolve_element`, `resolve_load`, `finite`, `ring_area`, `encloses`, `admissible`, `bounded`, plus
`CASCADE_EXEMPT = ("delete-node",)` with the same justification as §3. `apply_mutation` gained the
identity, referential, key and value guards and a `carried()` helper demanding of a `replace-`
exactly what it demands of the matching `create-`.

`observable()` became two-sided:

```python
def observable(scenario, before, after, moves=True):
    if moves and before == after:  raise AssertionError("… left the model untouched, so nothing was proved")
    if not moves and before != after:  raise AssertionError("… claims nothing happens, but the model moved")
```

New `reject_handler()` replays a vector whose whole claim is that the model does not move and
projects `{"refused": <bool>, "model": <document>}`. The refusal **reason** is deliberately not
projected: each implementation words it in its own language, and what a differential can hold them
to is the verdict and the document. `adapter()` now registers `hall-vector-<kind>` (25 new) and
`reject-<vector-id>` (46 new) alongside the existing three families.

### 7.3 The features and the Rust subjects

Each `🥒️.feature` gained two `Scenario Outline`s — `@id-hall-vector` (rows keyed by kind) and
`@id-reject` (rows keyed by the vector's own 6-hex scenario id, because one kind can refuse in
several different ways). Each subject `🦀️.rs` gained `hall_vector_of(kind)` / `reject_vector_of(id)`
`include_str!` tables, a `REJECT_VECTORS` list, and the mirror `subject::hall_vector` /
`subject::reject` handlers that assert the committed `🎯️outcome` and the model's immobility and
project the same two facts. **96 scenarios now run in both languages**, up from 25.

### 7.4 The five DIVERGE — resolved, all five against the reference

| vector | verdict |
|---|---|
| `🚨️dangling-start-ab4132` (create-element) | **the reference was wrong** |
| `🚨️dangling-node-af37e2` (create-support) | **the reference was wrong** |
| `🚨️dangling-mat-1ebd78` (create-solid) | **the reference was wrong** |
| `🚨️dangling-solid-5e04d9` (create-load-case) | **the reference was wrong** |
| `🚨️dangling-term-b9d144` (create-combination) | **the reference was wrong** |

All five are the same defect (W11 finding 10): the reference's `apply_mutation` checked only
id-uniqueness on a `create-`, never a foreign key, so it applied where Rust refuses. The committed
vectors follow the Rust, which is the subject under test, and the Rust's behaviour is the correct
one — a record naming a nonexistent node/material/section/solid/case is not a document anyone can
solve. `resolves`/`resolve_element`/`resolve_load` fix the reference; the Rust was not changed for
any of the five.

### 7.5 A sixth divergence, found by the new replay

`⏸️dup-load-id-4f4a0a` (add-load, a declared **no-op**): the reference RAISED where Rust warns. Here
**the reference was wrong again, in the opposite direction** — the committed outcome declares
`status: "applied"` with a `mutation.no-op` warning, and W11 §3 records the design decision
explicitly ("`add-load` | load id already in that case | **Warning `no-op`**, not a duplicate-id
Fatal"). Adding a load a case already carries asks for nothing; the case already has it. The
reference now leaves the model untouched, and `apply_mutation`'s docstring names the two idempotent
families (`add-load` with a known load id, and every `replace-`/`change-`/`update-` whose new value
is the value already there). W11's replay had scored this vector `agree-noop` only because its
verdict rule accepted a raise for either a rejection **or** a no-op; W12's rule separates them and
the disagreement surfaced.

---

## 8. Files changed

**Created — 21 case bundles × 6 files = 126 files** under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/{🕸️mesh,🧱️material,🛡️boundary,🏋️load,📈️analysis}/🧬️schema/🧬️mutations/<kind>/🧪️tests/<case>/`
(`🦀️.rs`, `🦠️mutation/🔣️.json`, `📸️snapshot/⬅️before/🔣️.json`, `📸️snapshot/➡️after/🔣️.json`,
`🎯️outcome/🔣️.json`, `🔺️diff/🚫️.absent`).

**Edited** (all paths relative to
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/` unless noted)

* `🌐️any/🧬️schema/🧬️mutations/🦀️.rs` — the `🛡️Guards` region (18 helpers), one import line, and the
  three in-crate test fixes of §6.
* **22** per-kind `🔺️diff/🦀️.rs` rewritten (every kind except `➖️remove-load`,
  `⚖️change-load-case-self-weight`, `🔗️create-combination`); 12 of them gained a rule, the rest
  changed only to route a shared check through the new guard or to document why they have none.
* **25** pre-existing case `🦀️.rs` — the `declared_outcome_holds` rewrite of §4.
* `🕸️mesh/🧬️schema/🧬️mutations/🧩️create-element/🧬️.schema.json`,
  `…/♻️replace-element/🧬️.schema.json`, `🏋️load/…/➕️add-load/🧬️.schema.json`,
  `…/📋️create-load-case/🧬️.schema.json` — §5.1.
* `🌐️any/🧬️schema/📸️snapshot/🔣️.json` — §5.2. `🌐️any/🧬️schema/🧬️mutations/🔣️.json` — §5.3.
* **5** `🌐️any/🧪️tests/*/🐍️.py` references — §7.2.
* **5** `<subset>/🧪️tests/*/🥒️.feature` and **5** `<subset>/🧪️tests/*/🦀️.rs` subjects — §7.3.
* **5** `<subset>/🔮️oracle/🔣️.json` — 21 new `scenarios[]` entries.
* `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs` — 21 `#[cfg(test)] #[path = …] mod tests_edge_…;`
  blocks (63 lines added, 0 removed by W12).
* `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — 21 `members-of-tests` rows.
  **Formatting note**: an early pass reformatted the whole file with `json.dump(indent=2)`
  (11 183 insertions). It was restored by rebuilding from the `HEAD` text with only the
  `memberNames` array replaced, after proving the sole semantic difference between `HEAD` and the
  working tree was that one array; the diff is now confined to it. Peers' concurrent rows are
  intact (the array grew from 1 211 to 1 234 during the session, 0 duplicates).

**Ticket**

* `🔨️w12-fem3d-rules.py` — the hardened rule set as a standalone predictor.
* `🔨️w12-audit-fem3d.py` — re-derives every committed vector's outcome under it.
* `🔨️w12-author-fem3d-cases.py` — the 21 vectors, the emitter and the three anchored registration
  patches (idempotent; `cases` / `patch` / `manifest`).
* `🔨️w12-wire-fem3d-differential.py` — the reference/feature/subject wiring of §7.
* `🔨️w12-replay-fem3d.py` — the verifier of §9.
* `🔨️w12-fem3d-rewrites.py` — the six one-shot rewriters that produced the hardened `🔺️diff`
  builders, the outcome-branch replacement, the repaired schemas, the facet union, the three
  in-crate test fixes and the reference's `add-load` idempotence, in the order they ran.
* this report.

---

## 9. Verification — commands and output

All run from the repository root. **No cargo, bun, nx, npm or git-mutating command was run.**

### (i) The hardened rules against every committed vector — `uv run python 🔨️w12-audit-fem3d.py`

```
vectors audited: 96
  agree     96
  CONFLICT  0
```

(the same script reported `75 / 75 agree / 0 CONFLICT` before any Rust was edited, which is what
made it safe to harden without re-authoring a single pre-existing fixture.)

### (ii) Replay through the plugin's own reference — `uv run python 🔨️w12-replay-fem3d.py`

```
vectors: 96
  agree          50
  agree-noop      7
  agree-refused  39
schema problems: 0
structure problems: 0
wiring problems: 0
structure advisories (pre-existing names, not authored here): 8
```

* **50 `agree`** — every applied vector: forward = committed `➡️after`, `observable`, `touches_one`,
  `equals_committed` and the inverse round-trip all hold;
* **7 `agree-noop`** — the reference leaves the model exactly where it was, and the two-sided
  `observable(…, moves=False)` proves it did not move;
* **39 `agree-refused`** — the reference refuses, as the committed rejection claims;
* **0 DIVERGE** (W11: 5), **0 schema problems** (W11: 12), **0 wiring problems** — every one of the
  96 `🥒️.feature` scenario ids has both a Python oracle and a Rust subject registration.
* the 8 advisories are all pre-existing case names over the 28-character budget; none authored by
  W11 or W12.

### (iii) `#[path]` and `include_str!` resolution — `python3 🔨️resolve-mounts.py --tree`

```
── ✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs
   checked:    487      resolved: 487      unresolved: 0
── include_str! across ✏️s/🔌️plugins/🏗️fem
   checked:    2123     resolved: 2123     unresolved: 0
TOTAL unresolved: 0
```

(W11 measured 443 / 1314; the totals are higher because W6 and W10 are mounting concurrently. W12's
own share is 21 `#[path]` leaves and 84 `include_str!` reads — 4 per rejection bundle, which carries
no `🔺️diff/🔣️.json` — plus the subject adapters' new `include_str!` tables.)

### (iv) Rust parse — `rustfmt --edition 2024 < <file> > /dev/null`

```
rustfmt parse: 248 fem3d 🦀️.rs files, 0 failures
crate mount file: PARSE OK
```

**This proves the files parse, not that they compile.**

### (v) The real-model feature rows under the hardened rules

Every inline `@id-mutate`/`@id-inverse` payload in the five features, replayed against
`🧊️steel-frame.snapshot.json` through `🔨️w12-fem3d-rules.py`:

```
feature real-model rows checked: 50 broken: 0
```

### (vi) JSON, Python and NFC

```
json parse: 670 files under 🧊️3d, 0 failures
python parse: 14 files, 0 failures
non-NFC names under 🧊️3d: 0
taxonomy parses; members-of-tests 1234 names, 0 duplicates
```

### (vii) Catalog ↔ disk set-equality

Part of (ii)'s `structure problems: 0`: every subset catalog's `scenarios[]` equals its kind's
on-disk `🧪️tests` children **in both directions** (no unregistered vector, no phantom entry); every
`id` is the stem of its `directoryName`; every `directoryName` is in taxonomy `members-of-tests`;
every sibling leading-emoji namespace is collision-free; and every bundle is the closed source shape
with exactly one of `🔺️diff/🔣️.json` / `🔺️diff/🚫️.absent`.

---

## 10. What is still owed

* **`create-node`'s finite-coordinate branch has no vector** (§2) — unreachable through JSON.
* **`delete-node` still leaves dangling references** (§3) — a specified behaviour, kept under the
  brief's own exception. If it is ever revisited, `node_referrers` is already written and exported;
  the change is one `if` in `🕳️delete-node/🔺️diff/🦀️.rs` plus one new vector.
* **The third-party-oracle debt is untouched.** W12 raises the fem3d differential from 25 to 96
  scenarios and makes the JSON Schema validation non-vacuous, but the reference is still a
  cross-implementation oracle, not a third-party one; `📓️explore-tests-oracles.md` §1's accounting of
  `fem3d-1-mutate-uncarried` stands unchanged.
* **Nothing was compiled.** The 22 rewritten diff builders, the 18 new guards, the 21 new case
  files, the 25 rewritten outcome assertions, the three in-crate test fixes and the five rewritten
  subject adapters have been parsed, never type-checked.
