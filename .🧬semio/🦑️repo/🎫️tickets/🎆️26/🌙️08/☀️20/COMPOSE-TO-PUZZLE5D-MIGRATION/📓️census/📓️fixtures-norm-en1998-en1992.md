# 🧪️ Handcrafted mutation fixtures — `📕️norm` / `📘️en1992` (35) + `📘️en1998` (49)

Ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`, fixture-recipe contract D1/D6.
84 mutation leaves, 84 test cases, 504 committed files, plus 2 edited mutations-root
`🦀️component.rs` files. **`✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` was NOT touched.**

## 📐️ Scope

| tree | leaves | cases | files |
| --- | --- | --- | --- |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 35 | 35 | 210 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations` | 49 | 49 | 294 |

Every leaf carries exactly one case; every case carries
`📸️snapshot/⬅️before`, `📸️snapshot/➡️after`, `🦠️mutation`, `🔺️diff`, `🎯️outcome` and `🦀️component.rs`.
No case is `rejected`, so no `🔺️diff/🚫️component.absent` file exists in either tree — every case
declares `{"status": "applied"}` and therefore carries the mandatory `🔺️diff/🔣️component.json`.

## 🔎️ What the oracle actually said

Both artifacts are flat, id-less, document-root parameter forms. Reading all 84
`🔺️diff/🦀️component.rs` files confirmed they are structurally uniform, with **no** exceptions:

* each leaf writes **exactly one** `<Artifact>Diff` field via `Diff { <field>: Some(..), ..Default::default() }`;
* numeric leaves guard with `!payload.new_<field>.is_finite()` → `MutationOutcome::fatal("mutation.invariant", ..)`;
* every leaf guards `base.<field> == payload.new_<field>` → `MutationOutcome::empty().warn("mutation.no-op", ..)`;
* no leaf has a cascade, a target-missing rejection, a range clamp, or a second written field;
* `↩️inverse/🦀️component.rs` always returns exactly one counter-mutation reading the pre-change base value.

Non-`f64` leaves (which run no `is_finite` guard, and whose fixtures say so in prose):
en1992 `change-annex` (`AnnexChoice`), `change-fire-rating` (`FireRating`),
`change-tightness-class` (`TightnessClass`), `change-use-fem` / `change-anchor-cracked` (`bool`);
en1998 `change-seismic-zone` (`u8`), `change-multiple-resisting-systems` / `change-tower-is-chimney`
(`bool`), and the eight `String` leaves (`ground-type`, `importance-class`, `structural-system`,
`annex`, `en-ground-type`, `en-spectrum-type`, `retrofit-knowledge-level`, `retrofit-limit-state`).

## 🔺️ The diff JSON — the load-bearing file

`En1992Diff` / `En1998Diff` carry `#[serde(rename_all = "camelCase", default)]` and **no
`skip_serializing_if`** on any field, so serde emits **every** field on every diff — `null` for the
untouched ones. Each committed `🔺️diff/🔣️component.json` therefore has all 37 (en1992) / 51 (en1998)
keys in struct-declaration order — `artifact` first, the snapshot fields next, `selectedCheckIndex`
last — with exactly one non-null entry: the field that leaf's `diff()` writes.

Key spelling and field order were cross-checked against the repo's own derived
`🧬️schema/🔺️diff/🔣️component.json` and `🧬️schema/📸️snapshot/🔣️component.json` JSON Schemas
(produced by the `ArtifactSchema` derive, an independent oracle): exact match on all 37 + 51 + 35 + 49
property names, in order. That confirms the awkward cases — `aSMm2`, `aCMm2`, `liquidESMpa`,
`liquidSRMaxMm`, `anchorC1Mm`, `t1S`, `retrofitEDKn`, `retrofitRKKn`, `wallSoilGammaKnM3`,
`foundationAreaM2`.

### `Option<Option<u32>>` limitation — pinned, not asserted away

Both diffs end with `#[state(presence)] pub selected_check_index: Option<Option<u32>>`. `None` and
`Some(None)` **both** encode as JSON `null`, so a JSON fixture cannot distinguish them. No mutation in
either artifact writes this field (it is the presence lane, not the artifact lane), so every fixture
leaves it `null` and each `committed_diff_is_canonical` test asserts
`decoded.selected_check_index.is_none()` with a doc comment stating the limitation explicitly rather
than pretending the round trip is faithful.

## 🦠️ Mutation payload shape

`En1992Mutation` / `En1998Mutation` derive `Serialize`/`Deserialize` with **no** `#[serde(...)]`
attribute, so they are externally tagged with the variant name verbatim; the payload structs carry
`#[serde(rename_all = "camelCase")]`. The committed form is therefore, e.g.:

```json
{ "ChangeMEdKnm": { "newMEdKnm": 187.5 } }
{ "ChangeSeismicZone": { "newSeismicZone": 4 } }
{ "ChangeAnnex": { "newAnnex": "En" } }
```

(en1992 `annex` is the `AnnexChoice` enum → `"De"`/`"En"`; en1998 `annex` is a plain `String` →
`"de"`/`"en"`. Same mutation name, different wire type — the two fixtures assert accordingly.)

## 📸️ Snapshot values

`⬅️before` is each artifact's own `Default` snapshot, transcribed field by field from
`📸️snapshot/🦀️component.rs`; `➡️after` is that snapshot with the single targeted field moved to the
new value. All new numeric values are dyadic (`.0` / `.25` / `.5` / `.75`, plus `0.015625` and
`0.0078125` for the two dimensionless ratios) and small enough that serde renders them in plain
decimal, so the canonical-JSON fixed-point assertion holds. Every `f64` key is written with an
explicit decimal point (`45.0`, not `45`) so the `serde_json::Value` comparison sees an `f64`, not a
`u64`; `seismicZone` is the only integer key and is written as `4`.

## 🧪️ The seven assertions, per case

Each `🦀️component.rs` names its own mutation, its own field and its own values in every message:

1. `<kind>_applies_to_committed_after` — asserts the field's new value, then whole-snapshot equality, then that no message was raised.
2. `<kind>_inverse_restores_before` — asserts the inverse is exactly one step, that the field is back to its pre-change value, then whole-snapshot equality.
3. `<kind>_committed_json_is_canonical` — both snapshots and the mutation payload are decode→encode fixed points.
4. `<kind>_declared_outcome_holds` — the declared `applied` status, an empty `messages()`, and a successful diff-apply.
5. `<kind>_produces_committed_diff` — the diff's own field equals the new value, `artifact` is `None` (never the whole-artifact replacement path), a **named neighbouring field** is `None`, then equality with the committed diff JSON.
6. `<kind>_committed_diff_is_canonical` — the committed diff decodes into the artifact's diff type carrying that field, leaves `selected_check_index` unset, and re-encodes identically.
7. `<kind>_committed_diff_applies_to_after` — applying the committed diff to `⬅️before` yields the field's new value and the whole committed `➡️after`.

Swapping any case's files for another leaf's fails at assertion 1, 5, 6 and 7: the test file names its
own diff field, its own value and its own neighbour. Entry points are the artifact's own
`vcs::apply_mutation`, `<Mutation as protocol::Mutation<Snapshot>>::{diff, inverse}` and
`<Diff as protocol::MutationDiff<Snapshot>>::apply`. Style follows the committed
`📚️examples/*/🧪️tests/🦀️test.rs`: `#[semio_framework_async_macros::async_test] async fn`, de-async
call sites, no `.await`.

No `.dsl.semio` / `.pack.semio` / `.op.semio` / `.spr.semio` / `.patch.semio` file was authored — those
are `fixtures generate` output (contract D12).

## 🔌️ Wiring — self-wired, not via `📦️glue.rs`

`📦️glue.rs` is shared by all fifteen norm artifacts and is under concurrent edit by peer agents, so
each artifact mounts its own cases from a new `//#region 🧪️FixtureTests` at the bottom of its
mutations-root `🦀️component.rs`:

```rust
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "🐮change-m-ed-knm/🧪️tests/raises-m-ed-knm-to-187-5/🦀️component.rs"]
    mod tests_change_m_ed_knm_raises;
    // …35 / 49 lines
}
```

The `#[path = "."]` re-base was verified empirically with a standalone `rustc --edition 2021` probe
(a non-mod-rs file loaded through a `#[path]` attribute, with an inline `#[path = "."]` module inside
it): rustc resolved the nested module against the *containing file's own directory*, i.e. the
`🧬️mutations/` tree — not a `component/` subdirectory. Module names are `tests_<glue-module>_<verb>`
(`raises` / `turns_on` / `turns_off` / `switches`), all 84 unique.

The stale header sentence in both roots ("no self-wiring `#[path = "."]` blocks are needed here") was
corrected to name this one exception.

## ✅️ Verification performed

* **Repo-wide lint**, `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures lint --by-tree`:
  * `🧬️ 115 artifact mutation trees · 1558 mutations · 806 covered · 752 uncovered`
  * `❌️ 774 error(s)` — **none of them in either of my trees**; `grep -c "en1992\|en1998"` over the
    whole `--by-tree` output returns `0`, so neither tree appears in the uncovered list nor in the
    (40-row-truncated) error list.
* **Scoped re-run of the lint's own rules** against only these two trees, reimplementing
  `declaredMutations` / `lintArtifact` / `lintCase` faithfully:
  * `📘️en1992: 35 variants, 35 leaves, 35 covered, 0 uncovered | ERRORS 0 | derived-encoding warnings 280`
  * `📘️en1998: 49 variants, 49 leaves, 49 covered, 0 uncovered | ERRORS 0 | derived-encoding warnings 392`
  * 672 warnings = 8 derived-encoding gaps × 84 cases — expected and correct pending `fixtures generate`.
* **`#[path]` resolution**: all 84 targets exist; all 84 module names unique; every leaf wired exactly once.
* **`include_str!`**: all 420 targets exist and all 420 parse as JSON.
* **`rustfmt --edition 2021 --emit stdout`**: parses all 84 test files (0 failures) and both edited
  mutations-root `🦀️component.rs` files.
* **Structural cross-check against the sources** (not against names): for all 84 cases the diff JSON's
  single non-null key equals the field that leaf's `🔺️diff/🦀️component.rs` writes; `➡️after` differs
  from `⬅️before` in exactly that one key; the after value, the diff value and the mutation payload
  value are the same value; the mutation's variant exists in the enum and its payload key equals
  `camelCase(pub new_… )` read from the leaf's own `🦠️mutation/🦀️component.rs`.
* **`cargo` was NOT run** (workspace broken by an in-flight de-async sweep, per the recipe). **No test
  is claimed to pass.** Validation above is structural and parse-level only.

## ⚠️ Surprises

* Both mutations-root files carried the stale claim that no self-wiring is needed; corrected in place.
* `🧬️mutations/🔣️component.json` in **both** trees is titled `"En1992Mutation"` / `"En1998Mutation"`
  but its `properties` are the **snapshot** fields (identical to `📸️snapshot/🔣️component.json` plus
  `selectedCheckIndex`). It is a mislabelled snapshot schema, not a mutation schema. Pre-existing,
  outside this slice, not touched — flagged for whoever owns the schema-emission pass.
* en1992's `change-annex` leaf still lives in the pre-migration directory `🐝set-snapshot/` (the
  struct inside is `ChangeAnnex`); en1998's equivalent is properly named `🗻change-annex/`. The
  en1992 fixture's header comment records the mismatch so the directory rename can be done later
  without confusion.
* The same semantic mutation `change-annex` is an enum (`AnnexChoice`) in en1992 and a bare `String`
  in en1998 — a real cross-artifact inconsistency, visible now that both fixtures sit side by side.

## 📄️ Authoring aid

`📓️census/🔧️gen-norm-en1998-en1992.py` (this ticket folder) emitted the 504 files. It reads each
leaf's own `🦠️mutation/🦀️component.rs` and `🔺️diff/🦀️component.rs` for the struct, payload field,
semantic kind, written diff field and guard set — nothing is inferred from a directory name. The
per-field before/after values are a hand-authored table transcribed from each artifact's `Default`
impl. `📓️census/🧪️wiring-en1992.snippet.rs` and `🧪️wiring-en1998.snippet.rs` are the emitted
wiring blocks as pasted into the two mutations roots.
