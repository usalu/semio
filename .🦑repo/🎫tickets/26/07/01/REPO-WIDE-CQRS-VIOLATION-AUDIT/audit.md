# Repo-Wide CQRS Violation Audit

The Trinity Jack CQRS gate plan claimed _"every other CQRS-migrated technology already follows the rule correctly."_ That claim is **false**. Trinity was the worst offender (Jack + host direct graph writes with dead VCS stubs), but many technologies still violate the rule in different ways.

## Rule (restated)

1. **No CRUD** — mutations are named semantic commands.
2. **Central gate** — `store.dispatch(Apply { operations })` with `Operation::diff` / `backwards`.
3. **Play TS** may mirror projection into a ledger, but only **after** semantic ops are computed — not as a blind `setDocument` blob that _is_ the mutation.

---

## Tier A — Direct fixture/document mutation in play (no semantic op path)

Play controllers spread/mutate projection in TS, then `recordProjectionChange(..., [{ op: "setDocument", document: next }])`.

| Technology       | File                                                 | Pattern                                                                        |
| ---------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------ |
| Flow             | `flow/play/index.ts`                                 | `commitFixture({ ...fixture, widgets, synapses })` on rename/move              |
| DAG              | `mathematical/graph/port/directed/dag/play/index.ts` | `commitFixture({ ...fixture, nodes, edges })`                                  |
| Procedural 2d/3d | `procedural/*/play/index.ts`                         | widget id/rename via spread                                                    |
| GIS 2d           | `gis/2d/play/index.ts`                               | `setDocument` only ledger                                                      |
| Puzzle 3d/5d     | `puzzle/3d/play`, `puzzle/5d/play`                   | `setDocument` only ledger                                                      |
| Presentation     | `framework/product/presentation/play`                | `setDocument` only ledger                                                      |
| Shooting         | `shooting/play/index.ts`                             | `setDocument` only ledger (Rust has `ShootingOp` but play doesn't dispatch it) |

**Rust op catalogs exist** for several of these (`FlowOp`, `FlowDagOp`, `FormOp`, `ShootingOp`, …) but play never calls `dispatch` — only the TS mirror.

---

## Tier B — Coarse bulk ops (technically Operation, not semantic)

Rust `Operation` impls that are whole-subtree/blob setters, not fine-grained mutations:

| Crate                                  | Op enum     | Variants                                         |
| -------------------------------------- | ----------- | ------------------------------------------------ |
| `flow/core`                            | `FlowOp`    | `SetFlow`, `SetTree` only                        |
| `mathematical/graph/port/directed/dag` | `FlowDagOp` | `SetNodes`, `SetEdges` only                      |
| `draw/rs`                              | `DrawOp`    | 10+ fine ops **plus** `SetDocument` escape hatch |

These pass the letter of `Operation<P>` but not the spirit of the Trinity plan (manifest-validated semantic units).

---

## Tier C — Compliant play pattern (reference)

| Technology | Pattern                                                                |
| ---------- | ---------------------------------------------------------------------- |
| Draw       | `applyDrawEditOp` semantic ops → `recordProjectionChange(setDocument)` |
| Forms      | `applyFormEditOp` semantic ops → ledger                                |
| Raster     | `applyRasterEditOp` semantic ops → ledger                              |
| Writer     | `WriterOp::SetText` in Rust; play mirrors via setDocument              |

---

## Tier D — Trinity residuals (post CQRS gate)

| Location                                    | Issue                                                                                                                        |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `trinity/rewrite/engine/lib.rs`             | `set_camera` writes `self.graph.camera` directly (ephemeral UI — may be acceptable)                                          |
| `trinity/rewrite/engine/lib.rs`             | `sync_ephemeral_positions_from_engine` mutates graph cache during drag (ephemeral — OK if commit goes through gate)          |
| `trinity/jack/play`, `trinity/rewrite/play` | `patchTrinityNodes` / LHS sync use `runJackOnFixture` on **ephemeral session**, bypassing canvas WASM store undo stack       |
| `trinity/react/index.tsx`                   | `runJackOnFixture`, `applyRewriteOnFixture` still spin throwaway sessions (LSP/editor OK; play should prefer canvas session) |

---

## Tier E — TS mirror debt (flagged in Trinity plan, out of scope there)

~20 `*/play` files use `framework/core/vcs-sync.ts` `DocumentVcsStore` with `setDocument`-only `*EditOp` types. Ticket `TYPESAFE-RUST-VCS-ENGINE` marked completed but mirror not retired.

---

## Status (2026-07-01)

**Tier A remediated.** All listed play controllers now route structural edits through semantic `*EditOp` types and `recordProjectionChange(docStore, [op])` instead of blind projection spreads.

| Technology           | Semantic ops                          | Play gate                     |
| -------------------- | ------------------------------------- | ----------------------------- |
| Flow                 | `FlowFixtureEditOp`                   | `applyFixtureEdit`            |
| DAG                  | `DagFixtureEditOp`                    | `applyFixtureEdit`            |
| Procedural 2d/3d     | reuses `FlowFixtureEditOp`            | `applyFixtureEdit`            |
| GIS 2d               | `GisMapFixtureEditOp`                 | `applyFixtureEdit`            |
| Puzzle 3d            | `Puzzle3dFixtureEditOp` (24 ops)      | `applyFixtureEdit`            |
| Puzzle 5d            | `Puzzle5dModelEditOp`                 | `applyModelEdit` + store sync |
| Presentation         | `PresentationEditOp` (extended)       | `applyDeckEdit`               |
| Shooting             | `ShootingFixtureEditOp`               | `applyFixtureEdit`            |
| Forms                | `FormEditOp`                          | `applySpecEdit`               |
| Trinity jack/rewrite | `jackDispatch` / `beforeJackDispatch` | canvas WASM gate              |

**Tier D (Trinity):** `patchTrinityNodes` in jack + rewrite play now dispatches Jack queries through canvas session instead of `runJackOnFixture` mutation shim. Read-only `runJackOnFixture` for LHS highlight preview retained.

**Remaining (out of scope):** Tier B coarse Rust ops; Tier E TS mirror retirement to WASM-only dispatch.

### Phase 2 (2026-07-01) — All DocumentVcs technologies

| Technology        | Forwards gate                      | Semantic backwards undo                                       |
| ----------------- | ---------------------------------- | ------------------------------------------------------------- |
| Flow / Procedural | `FlowFixtureEditOp`                | `backwardsFlowFixtureEditOp`                                  |
| DAG               | `DagFixtureEditOp`                 | `backwardsDagFixtureEditOp`                                   |
| GIS 2d            | `GisMapFixtureEditOp`              | `backwardsGisMapFixtureEditOp`                                |
| Shooting          | `ShootingFixtureEditOp`            | `backwardsShootingFixtureEditOp`                              |
| Puzzle 3d         | `Puzzle3dFixtureEditOp`            | `backwardsPuzzle3dFixtureEditOp`                              |
| Puzzle 5d         | `Puzzle5dModelEditOp`              | `backwardsPuzzle5dModelEditOp`                                |
| Presentation      | `PresentationEditOp`               | `backwardsPresentationEditOp`                                 |
| Forms             | `FormEditOp`                       | `backwardsFormEditOp`                                         |
| Draw              | `DrawEditOp`                       | `backwardsDrawEditOp` (property inverses + snapshot fallback) |
| Raster            | `RasterEditOp` + `setBrushOpacity` | `backwardsRasterEditOp`                                       |
| Writer            | `WriterEditOp` + `setText`         | `backwardsWriterEditOp`                                       |

**Canvas-native (no play DocumentVcsStore):** Puzzle 2d (Rust `Puzzle2dOp` + renderer graph), Semios app store (`dispatch`), Trinity WASM graph store.

1. **Flow + DAG** — foundational graph techs; same shape as Trinity; play must call WASM `dispatchJson` / Rust store instead of `commitFixture` spread.
2. **Procedural 2d/3d** — reuses flow fixtures; fix after flow gate exists.
3. **Puzzle 2d/3d/5d** — wire existing `Puzzle2dOp` etc. through play/WASM hosts.
4. **Shooting, GIS, Presentation** — wire Rust semantic ops through play.
5. **Retire TS mirror** — once each tech is Rust-backed (separate migration).
6. **Trinity residuals** — route inspector patch + rewrite LHS through canvas session gate.

---

## Tests to add per tech

- Unknown kind / derived property rejection (where manifest applies)
- `dispatch(Apply)` → `undo` restores projection
- Play vitest: no `commitFixture({ ...spread })` for structural edits
