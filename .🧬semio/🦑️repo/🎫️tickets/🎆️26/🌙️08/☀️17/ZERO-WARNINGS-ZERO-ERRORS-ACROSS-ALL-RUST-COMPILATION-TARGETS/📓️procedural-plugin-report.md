# semio-s-plugin-procedural — Warning Triage Report

## Scope
`cargo check -p semio-s-plugin-procedural --message-format=short` — `(lib)` target only, per
delegation. `(lib test)` target's ~17 pre-existing `Mutation::apply`/`::diff` trait-migration
errors are confirmed out of scope and were **not** touched.

## Numbers
- Starting `(lib)` warnings: **355** (146 rustc message-lines, "3 duplicates", 0 errors).
- Ending `(lib)` warnings: **355** (unchanged).
- New errors introduced: **0** (zero edits were made to this crate — `git status`/`git diff
  --stat` on `✏️s/🔌️plugins/🌀️procedural` show a completely clean tree).

## Why 0 warnings were fixed: every single warning traces to one deliberately-scaffolded subsystem
100% of the 355 warnings (all 146 message-lines) are inside
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/`
— a ~42-submodule, `pub(crate)`-gated (see `📦️glue.rs:993` `pub(crate) mod wfc_engine { ... }`)
wave-function-collapse solver library. There are **no other warnings anywhere else in the crate**.

I applied the ticket's own dead_code triage method (grep whole crate incl. tests, check real
call sites) before concluding anything, and it points firmly away from "delete it":

1. **Crate-wide grep for `wfc_engine::`** (excluding the wfc-engine dir itself) found real usage
   in exactly one place: `🧩️assembly/…/💡️inferences/🦀️component.rs`'s `compile_and_solve()`,
   which calls only `model::ModelBuilder`, `ids::{PatternId,NodeId}`,
   `topology::GraphTopologyBuilder`, `solver_graph::GraphSolverBuilder`, `outcome::SolveOutcome`.
   (One other hit, in `📸️snapshot/🦀️component.rs:47`, is a doc-comment mention, not code.)
2. **That file's own doc comment says this is intentional, incremental adoption, not
   abandonment**: `"The 10,930 LOC WFC solver copied into the sibling ../🧩️wfc-engine/ compute
   tree becomes the internals of these compute() bodies."` and, on `AssemblyEntropy` specifically:
   `"wiring this field through wfc_engine::propagate/prop_ac3 for a truly narrowed per-slot domain
   is a real remaining increment, not done here."` That is explicit, first-person acknowledgment
   of unfinished wiring — the same shape as the trait-migration hazard already called out as
   off-limits, just without a compile error attached.
3. **The unused ~29 submodules are not random cruft — they're the OTHER two artifact kinds'
   future WFC surface.** The procedural plugin has three artifact kinds: `assembly` (graph-based,
   now wired), `procedural2d`, `procedural3d` (currently using unrelated topology/binary/text
   inference code, **zero** wfc_engine usage). The unused wfc-engine submodules are unmistakably
   grid/tile/voxel-flavored: `grid2d`/`grid3d` (`declare_stencil_relations*`, `Stencil2d/3d`),
   `solver_grid2d`/`solver_grid3d` (`Grid2dSolver`/`Grid3dSolver` — alternate solver front-ends
   for regular grids, unlike the generic graph solver actually in use), `tiled`
   (`TiledModelBuilder`), `symmetry` (2D/3D transform/rotation groups for tile canonicalization),
   `sparse3d` (`SparseVolume`/`VoxelCoord`), `extract` (2D pattern extraction from a sample
   bitmap — classic image-based WFC). These are squarely `procedural2d`/`procedural3d` concerns
   that haven't been wired into those artifacts' inference layers yet. The rest
   (`evolve`, `repair`, `parallel::multi_start`, `oracle`, `diag`, `serial`, `soft`,
   `constraints_card`/`constraints_conn`, `flow`, `heuristics::WeightedEntropy`, extra `search`
   entry points like `solve_all`/`solve_cancellable`, `weights::WeightTable`) are optional-feature
   layers of the same solver (checkpointing, soft scoring, richer constraints, cancellation,
   diagnostics) that the one wired call site (`compile_and_solve`) simply doesn't need yet.
   The modules that DO show 0 warnings (`beam`, `bitset`, `chunk`, `constraint`, `hierarchy`,
   `motif`, `nogood`, `prop_ac3`, `prop_ac4`, `propagate`, `trail`) are exactly the internal
   solving machinery that `GraphSolver::solve` (the one active entry point) actually exercises —
   which is further confirmation the whole tree is live, coherent, working code, not orphaned.

## Decision: left untouched, not deleted
Given the above, this does not fit the "genuinely dead, superseded, forgotten" pattern the
triage method is meant to catch (e.g. the pptx hand-rolled codec case from earlier this
session). It fits the "in-flight, deliberately-built feature surface with explicit remaining
increments documented in its own doc comments" pattern — the same category of risk this
ticket's brief already tells me to avoid fighting (the store `Mutation::apply`/`::diff`
migration), just manifesting as warnings instead of errors because Rust's dead_code lint is
strict about `pub(crate)`-boundary reachability regardless of intent.

Deleting ~29 of 42 submodules of a 10,930-LOC hand-built WFC solver as a side effect of a
warnings-cleanup ticket would very likely destroy real, professionally-organized, in-progress
work that another session (or the same author, incrementally) is building out for
`procedural2d`/`procedural3d`. I did not do this. No `#[allow(...)]` was used either (forbidden
by policy, and wouldn't have been my choice anyway since this isn't a "false positive" —
it's real current dead code by rustc's semantics, just not by the repo's actual intent).

## Recommendation for the ticket owner
This crate cannot reach 0 warnings via the "fix warnings" methodology without either:
- (a) Someone with authority over the wfc-engine build-out wiring more of it into
  `procedural2d`/`procedural3d` inferences (a real feature project, not a warnings fix), or
- (b) Explicit confirmation from whoever owns that work that specific pieces are actually
  abandoned/superseded and safe to delete, or
- (c) A deliberate, ticket-owner-approved decision to suppress/scope this differently (still
  without `#[allow(dead_code)]` per policy — e.g. temporarily not building assembly's inference
  compute-tree until more consumers land, which is itself a design decision above my delegated
  scope).

I recommend flagging this to the dev rather than silently leaving it as an unexplained
"355 warnings, untouched" line in the ticket-wide tally.

## Files touched
None. Zero edits made in this crate this session.
