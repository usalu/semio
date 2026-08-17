# Mid-flight quality audit (2026-08-12, during Waves C/M)

Purpose: verify that migrated facets contain REAL handcrafted triads rather than shims, and that
no systemic anti-pattern is spreading across lanes. Run by the coordinator against the working
tree, independent of agent self-reports (an earlier lane was caught claiming progress it had not
made, so lane summaries are not trusted without disk evidence).

## Headline counts

| metric | at ticket restart | now |
|---|---|---|
| facets on `#[derive(dsl::Mutations)]` | 32 / 107 | **48 / 107** |
| `.rs` files under `✏️s/🔌️plugins` hitting banned tokens | 423 | **251** |

## Anti-pattern sweep across every triad leaf

| anti-pattern | hits | assessment |
|---|---|---|
| `base.clone()` / `snapshot.clone()` inside a `🔺️diff` leaf | **0** | clean — no apply-then-capture diffs anywhere |
| `todo!()` / `unimplemented!()` / `//! stub` in a triad leaf | 10 | all in playbook (9) + imperative (1), both in active lanes |
| apply-and-capture (`*snapshot = …` / `apply_*_mutation(…)`) in a `🦠️mutation` leaf | 73 | fully accounted for, see below |

### The 73 apply-and-capture leaves

| plugin | count | status |
|---|---|---|
| `🗄️stdio` | 61 | **deferred by cross-ticket agreement** — peer session owns `🗄️stdio/**` until their roster restructure lands |
| `📖️playbook` | 9 | active lane; these are exactly the shims the framework→plugin vocabulary move replaces |
| `🔱️trinity` | 1 | active lane (resumed) |
| `📜️imperative` | 1 | active lane |
| `➗️mathematical` | 1 | active lane (layout/gis/mathematical) |

**Conclusion: no systemic quality problem.** Every remaining instance is either in a lane running
right now or inside the deferred stdio territory. Nothing has leaked into finished facets.

## Spot-check of finished leaves (read in full, not grepped)

`🪐️space` / `🏠️home` — `🔢️change-catalog-generation/↩️inverse`:
```rust
pub fn inverse(_payload: &…::ChangeCatalogGeneration, base: &SHomeSnapshot) -> Vec<SHomeMutation> {
    vec![super::mutation::change_catalog_generation(base.catalog_generation)]
}
```
Correct: reconstructs the old value from `base` (the pre-state) rather than structurally inverting
the diff — exactly the addressing-convention rule in `📓️taxonomy.md`.

`🔋️energy` / `🔋️model` — `♻️replace-model/🔺️diff`:
```rust
pub fn diff(payload: &ReplaceModel, _base: &EnergyModelSnapshot) -> EnergyModelDiff {
    EnergyModelDiff { model_json: Some(payload.new_model_json.clone()), ..Default::default() }
}
```
Correct: sparse construction straight from the payload, leaving `schema` and `results_json`
untouched. No snapshot clone, no apply-then-capture.

Both facets carry a real `impl MutationKind<…>` in the `🦠️mutation` leaf and real
`pub fn diff` / `pub fn inverse` in the sibling leaves.

## Caveat on verification status

`semio-s-plugin-stdio` is currently red from the peer session's in-flight subset renames
(`E0433`), and every plugin depends on stdio, so **cargo gates cannot pass for any lane right
now**. Work landing during this window is structurally audited (above) but not compile-verified.
Wave V re-runs every gate once stdio is green; nothing is called done before that.
