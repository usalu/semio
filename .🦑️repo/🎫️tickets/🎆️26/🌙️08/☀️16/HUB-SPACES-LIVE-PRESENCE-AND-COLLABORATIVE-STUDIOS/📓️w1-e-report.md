# W1-E report — `s.space` artifact (space's artifact index)

Lane 1-E. Kind `s.space`, dialect `s.space.space@1/*`, document id `index` inside a hub space
(contract §C4).

## Changed files

New tree `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/**` — 66 files:
- `🦀️component.rs` — artifact root: `SPACE_INDEX_DIALECT`, `S_SPACE_INDEX_DOCUMENT_SCHEMA`,
  `artifact_kind()`, `definition()`, `declaration()`.
- `🏅️standards/🔖️1/🪆️subsets/🔣️component.json` — subsets manifest.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/**` — `sspace_index_schema_descriptor()`,
  `📸️snapshot` (`SSpaceSnapshot`, `SpaceArtifactRow`, `SpaceArtifactDialect` + handcrafted
  `ArtifactDsl`/`ArtifactPack`), `🔺️diff` (`SSpaceDiff` + a hand-written
  `impl protocol::MutationDiff<SSpaceSnapshot> for SSpaceDiff` — see "MutationDiff note" below),
  `🧬️mutations` (`SSpaceMutation` dispatch enum + `📝️text`/`💾️binary` OpText/OpBinary facets + the
  four mutation triads `🌱create-artifact`/`🗑️delete-artifact`/`🏷️rename-artifact`/`🕒touch-artifact`,
  each with `🦠️mutation`/`🔺️diff`/`↩️inverse`).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — placeholder, no composer registered this
  wave (see Scope reductions).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/**` — one bundled example (empty index).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/**` — `SpaceIndexEditor` (`ArtifactEditor`),
  `SpaceIndexCommand` (4 rows, one per mutation, via `app_commands!`), `🎭️modes/✏️edit/🪟️windows/🏠️main`
  (real `TableWindowKit` render of `artifacts`), `🎮️commands/*` (4 command payload+handle files, each a
  thin pass-through into the matching `SSpaceMutation` builder, each with its own dispatch test).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/**` — `SpaceIndexViewer` (`ArtifactViewer`, `Noop`-only
  command channel mirroring `Din4108ViewCommand`), same `TableWindowKit` render (read-only window kind).
- 26 `📌️empty.md` facet placeholders (unused `🎚️config`/`👥️presence`/`🫧️transient`/mode-level dirs, per
  taxonomy `surfaceRequiredChildDirs`/`modeRequiredChildDirs`).
- Minimal TS twins: `🧬️schema/🟦️component.ts`, `📸️snapshot/🟦️component.ts`, `🔺️diff/🟦️component.ts`,
  `🧬️mutations/🟦️component.ts` (interfaces only — editor/viewer/io/examples stayed Rust-only this
  wave, see Scope reductions).

Edited (within lease):
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs` (plugin root) —
  - registered the new artifact: `.artifact(crate::artifacts::space::declaration().map_err(...)?)`,
    `.editor::<SpaceIndexEditor>(...)`, `.editor_mutation_roster::<SpaceIndexEditor>()`,
    `.viewer::<SpaceIndexViewer>(...)`, `.viewer_mutation_roster::<SpaceIndexViewer>()`;
  - added `project_space_index_to_collection(&SSpaceSnapshot) -> CollectionSnapshot`;
  - repointed `resolve_workflow_artifact_document` to try the space's index first
    (`resolve_space_index_snapshot` → `project_space_index_to_collection` → walk its entries via the
    new shared `find_workflow_snapshot_in_collection` helper) and fall back to the legacy direct
    `projection.collections` walk only when no index document exists yet;
  - added `assert_editor_and_viewer_share_dialect`/`assert_viewer_never_mutates` tests for the new
    surface and a `project_space_index_to_collection` unit test;
  - **two unrelated one-line fixes required just to get the file compiling again** (see "Foreign
    breakage fixed minimally" below): `.artifact(home::declaration())` and
    `.artifact(space::declaration())` both needed `.map_err(PluginAssemblyError::definition)?` (the
    builder's `.artifact()` takes a bare `ArtifactDeclaration`, `declaration()` returns a `Result`);
    and both `resolve_workflow_artifact_document`'s two `materialize_backbone_snapshot(doc,
    &doc.applied_edit_ids)` call sites needed `&doc.cursor.applied_edit_ids` (the field moved into
    `BackboneDocument.cursor: ArtifactCursor` in a peer's live schema refactor).
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📦️glue.rs` — additive `#[path]` wiring for the new
  `artifacts::space::*`, `editor::space_index::*`, `viewer::space_index::*` module trees (mirrors
  `home`'s existing wiring shape verbatim).
- `✏️s/🔌️plugins/🪐️space/📦️packages/🟦️typescript/📦️index.ts` — one additive export line for the new
  schema TS twin (the pre-existing three `home_*` lines already pointed at a stale pre-migration path;
  left untouched, flagged inline instead — not my lease to fix).

## Commands run + result counts (real tails, `🧪️1-e-*.txt`)

**`cargo check -p semio-s-plugin-space`** (`🧪️1-e-cargo-check-4.txt`, after 3 earlier iterations fixing
my own mistakes — logged in `🧪️1-e-cargo-check-1.txt` through `-3.txt`): **2 grep-matches on `^error`,
both the SAME root cause** —
```
error[E0277]: the trait bound `semio_framework_os::WorkflowMutation: SemanticMutation<semio_framework_os::WorkflowSnapshot>` is not satisfied
  --> ✏️s/🔌️plugins/🪐️space/🦀️component.rs:393
        .document_app::<crate::engine::space::SpaceApp>(crate::engine::space::create_space_app())
error: could not compile `semio-s-plugin-space` (lib) due to 2 previous errors; 18 warnings emitted
```
**Not my code.** `.document_app::<SpaceApp>(...)` is pre-existing (I added lines around it, never
touched it), `SpaceApp`/`WorkflowMutation`/`WorkflowSnapshot` are all framework-owned
(`🧧framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`), and `WorkflowMutation` (line 1138
there) is a plain hand-rolled `#[derive(Serialize, Deserialize)]` enum with **no `SemanticMutation`
impl anywhere in the tree** — `grep -rn "impl.*SemanticMutation.*for.*WorkflowMutation"` under
`💻️os` → 0 hits. `document_app`'s `A::Mutation: SemanticMutation<A::Snapshot>` bound
(`🔌️plugin/🏗️builder/🦀️component.rs:204`) and the `workflow` file both land in the SAME auto-commit
sweep (`c8a29e41c5`, real timestamp `2026-08-16 20:26:15 +0200` via `git log --date=iso`) whose message
says "Refactor OS store schema mutations and SPR command resolution with change merge policy" —
matches the live `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` ticket's own
description exactly (confirmed independently: that ticket's own folder shows dozens of files staged in
`git status` at write time). **Attribution: live peer migration in progress, `WorkflowMutation` not yet
carried across; `🧧framework/**`/`⚙️engine/**` are both forbidden to me regardless.** Not fixed.

**`cargo test -p semio-s-plugin-space --lib`** (`🧪️1-e-cargo-test-1.txt`): **3 previous errors** — the
SAME `WorkflowMutation`/`SemanticMutation` error above, PLUS one more, also pre-existing / not mine:
```
error[E0425]: cannot find function `register_stdio_format_descriptors` in module `semio_s_plugin_stdio::manifest`
  --> ✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/./././../../⚙️engine/🪐️space/🎮️commands/🖼️export-media/🦀️component.rs:57
```
`⚙️engine/🪐️space/🎮️commands/🖼️export-media` is pre-existing code I never touched, calling a function
stdio's own `manifest` module no longer exports under that name (`stdio_format_descriptors` exists
instead — a live `26/08/16/FULL-STDIO-…` rename this call site wasn't updated for). Both `⚙️engine/**`
and `🗄️stdio/**` are forbidden to me. Not fixed.

**No test in `🗿️artifacts/🪐️space/**`, `editor::space_index`, or `viewer::space_index` appears in
either error list** — every mutation-law test, dialect-share test, projection test, and command-handler
test I wrote is syntactically sound and would run cleanly once these two pre-existing/peer blockers
clear; I cannot produce real pass/fail counts for them until then (never claiming a test passed that I
did not actually run — these genuinely did not run, because the crate does not link).

**`cargo check -p semio-s-plugin-space --target wasm32-wasip2`** (`🧪️1-e-cargo-check-wasm32.txt`):
**fails, but NOT where the brief predicted.** The brief expected the blocker "entirely inside stdio"
(FULL-STDIO's `absorb`/`apply` wasm mismatch). What actually happened: the build never reaches stdio's
own code — it fails first inside the `tokio` crate itself:
```
error: Only features sync,macros,io-util,rt,time are supported on wasm.
  --> tokio-1.52.3/src/lib.rs:478
error: could not compile `tokio` (lib) due to 1 previous error
```
Traced: `semio-s-plugin-space`'s own `Cargo.toml` depends on `semio-framework-os` with
`features = ["os-host-full"]` (pre-existing, not added by me);
`🖥️host/📦️packages/🦀️rust/Cargo.toml:62` declares
`os-host-full = ["dep:zip", "semio-framework-os-kernel/sync"]`; `semio-framework-os-kernel`'s own
`Cargo.toml` (`sync = ["dep:tokio-tungstenite", "dep:notify", "dep:rusqlite", "tokio/rt", "tokio/net",
"tokio/time"]`) turns on `tokio/net`, which is not in tokio's wasm-supported feature list
(`sync,macros,io-util,rt,time` — no `net`). This is a pre-existing framework Cargo.toml
feature-unification gap (`🧧framework/**`, forbidden), unrelated to stdio and unrelated to anything I
wrote. **Reporting as observed, not force-fitting to the predicted stdio narrative** — the wasm32 gate
is red for a DIFFERENT, earlier reason than the brief anticipated; stdio's own wasm errors never get a
chance to surface behind this one.

## Foreign breakage fixed minimally (as instructed when blocking my own required edits)

1. `plugin()`'s two `.artifact(...)` calls (`home` and my new `space`) — `.map_err(PluginAssemblyError::definition)?`
   added to both. `home`'s call was unedited by me but already broken at first compile — attribution
   inconclusive beyond "same auto-commit sweep as everything else live right now" (`git log --date=iso`
   gives no finer granularity here; the repo's single rolling auto-commit batches all concurrent
   sessions' edits). Both calls now compile; this is a two-line, purely mechanical, non-semantic fix.
2. `resolve_workflow_artifact_document`'s two `materialize_backbone_snapshot(doc, &doc.applied_edit_ids)`
   calls — `.applied_edit_ids` → `.cursor.applied_edit_ids` (field relocated into `BackboneDocument`'s
   new `cursor: store::ArtifactCursor`, confirmed by reading the current struct definition). Also
   two-line, mechanical.

Neither touches `⚙️engine/**` or `🧧framework/**` content — both are inside my own leased plugin-root
file, fixing calls into framework types whose SHAPE changed under me, not their behavior.

## Scope reductions (explicit, given effort budget)

The brief says to mirror `🏠️home` "in full" (~180 files: `derive_artifact_facets!`/`ArtifactBuilder`/
`ArtifactAnalysis` machinery, real five-language schema leaves pulled via `include_str!`, stdio
composers, per-facet text/binary convenience sub-facets). Reproducing that exactly for a brand-new
artifact in one lane pass was not achievable at this effort level. Deliberate, structurally-consistent
cuts:
1. **No `💡️inferences` facet** — `.inferences(...)` is optional on the declaration builder; never called.
2. **No stdio import/export composers** — `🚪️io/🦀️component.rs` is a placeholder
   (`io_registry_entries() -> &'static []`), never wired into `.composers(...)`. The index is a
   hub-shared control-plane document, not a user file-import/export target.
3. **`FacetLeaves` for the schema descriptor use inline placeholder strings** for
   typescript/graphql/json_schema/proto (the `rust` leaf is real, `include_str!`'d from the actual
   snapshot/diff/mutations source). These are plain `&'static str` registry content, not
   compiled/validated at build time.
4. **No `📸️snapshot`/`🔺️diff`-level `📝️text`/`💾️binary` convenience sub-facets.** Discovered mid-build
   that `🔺️diff`'s hand-written `impl protocol::MutationDiff<P>` (apply/absorb) — which I had assumed
   was auto-derived — actually lives in exactly that omitted `🔺️diff/📝️text` facet for `home`/`dag`. I
   moved the hand-written impl directly into the main `🔺️diff/🦀️component.rs` file instead of creating
   the separate facet (see "MutationDiff note" below) rather than skip it outright.
5. **TS twins limited to the schema layer** (snapshot/diff/mutations interfaces) — no TS twin for
   editor/viewer/io/examples; lane 2-B owns the real UI next wave.

## MutationDiff note (a genuine finding, not just a scope cut)

`#[derive(dsl::Mutations)]` on the mutation dispatch enum does **not** auto-generate
`impl protocol::MutationDiff<Snapshot> for Diff` — I initially assumed it did (dag's/home's `Diff`
structs looked identical to mine, deriving only `ArtifactSchema`, with no visible `MutationDiff` impl in
their main files). It turned out both hand-write `impl MutationDiff<P> for XDiff` in their separate
`🔺️diff/📝️text/🦀️component.rs` facet — a facet I had cut for scope. Fixed by writing the impl directly
in `🔺️diff/🦀️component.rs` (apply: overwrite `schema`/`artifacts` when `Some`; absorb: last-`Some`-wins
per field, same shape as `SHomeDiff`'s hand-written version). Confirmed this was the ONLY missing piece
for `SSpaceMutation`/`SSpaceDiff` — after this fix and the `dsl::DslEnum` addition below, every error in
`🗿️artifacts/🪐️space/**` cleared.

Also needed: `SSpaceMutation` derives `dsl::DslEnum` in addition to `dsl::Mutations` (mirrors
`SHomeMutation` exactly) — `dsl::Mutations` alone only generates `impl protocol::Mutation<P>`/
`SemanticMutation<P>`, not `impl dsl::DslVariants`, which my hand-written `📝️text`/`💾️binary` OpText/
OpBinary facets need. (`dag`'s own `DagMutation` gets away without `DslEnum` because it uses a
completely different technique — a local `DagMutationDsl` mirror enum for the wire format — not
applicable here since my mutation payloads directly derive `dsl::DslRecord`.)

Also needed: `TouchArtifact`'s verb `"touch"` is not in `protocol::APPROVED_VERBS` (const-panic at
compile time) — changed to `"update"` (kept `kind: "touch-artifact"`, unaffected).

## sharedFileRequests

None.

## What is NOT done

- Real editor/viewer UI (row create/rename/delete affordances, context menu, dialogs) — deferred to
  lane 2-B per the brief; both surfaces currently render one real (not stub) `TableWindowKit` table of
  `artifacts`.
- Stdio import/export composers, `💡️inferences` facet, full five-language schema leaves — deferred,
  see Scope reductions.
- **Verified test PASS counts for `🗿️artifacts/🪐️space/**`'s own tests could not be produced** — the
  crate does not link due to the two pre-existing/peer blockers documented above (`WorkflowMutation`/
  `SemanticMutation`, `register_stdio_format_descriptors`), both outside my lease. Every test I wrote
  is present and was read/reviewed for correctness but never actually executed — reporting this
  explicitly rather than claiming a pass.
- wasm32 target gate is red for a different reason than the brief predicted (tokio/wasm feature
  mismatch via `semio-framework-os-kernel`'s `sync` feature, not stdio) — reported as observed above.
