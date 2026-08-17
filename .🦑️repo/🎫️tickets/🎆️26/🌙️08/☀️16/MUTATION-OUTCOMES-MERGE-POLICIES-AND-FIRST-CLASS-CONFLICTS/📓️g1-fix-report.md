# Lane G1 — Fix Report

## Fixes applied (the 4)

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:193`** — `widget.id ==
   entry.id` → `widget.id() == &entry.id` (`Identified<String>::id` is a method on `Widget`, not a
   field; every other access in the guard block already used `.id()` correctly — this was the only
   bad one, confirmed by grepping the whole file for `.id ==`/`.id !=` and diffing against the
   commit that introduced the guard). This was the sole compile error in
   `semio-framework-os-flow`/`semio-framework-os-kernel`.

2. **`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:63`**
   (`apply_raster_mutation`) — was feeding the whole `MutationOutcome<RasterDiff>` into
   `MutationDiff::apply`; changed to `protocol::MutationDiff::apply(protocol::Mutation::diff(mutation,
   snapshot).diff(), snapshot)`, matching the reference pattern in
   `✏️s/🔌️plugins/🕸️dag/…/🧬️mutations/🦀️component.rs:65` (`.diff().apply(snapshot)?`).

3. **`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{📄️set-snapshot,📄️commit-document,📄️set-active-example,📄️set-fixture-json}/🦀️component.rs`**
   — all 4 `load_document_effect`'s `store::ArtifactEnvelope{..}` struct literals were missing the
   two fields this ticket added to `ArtifactEnvelope`. Added `edit_messages: Vec::new(), conflicts:
   Vec::new()` to each, matching `🏪️store/🦀️component.rs:2074-2075`'s own constructor.

4. **Space (`WorkflowMutation`) / Puzzle (`Puzzle2dMutation`/`Puzzle3dMutation`/`Puzzle5dMutation`)
   — triaged, NOT touched, NOT ours.** Both are case B ("registration site changed"), not case A
   ("derive stopped emitting"):
   - **Space**: `WorkflowMutation` (`🧰️framework/…/🔁️workflow/🦀️component.rs:1138`) is fully
     hand-written — no `#[derive(dsl::Mutations)]` at all, only a hand `impl
     protocol::Mutation<WorkflowSnapshot>` (line 1435, already return-type-converted). It never had
     `SemanticMutation`. The failure comes from `document_app<A: ArtifactApp>()`'s bound `A::Mutation:
     protocol::SemanticMutation<A::Snapshot>`
     (`🧰️framework/…/🔌️plugin/🏗️builder/🦀️component.rs:202-204`), which powers the "owner mutation
     roster" / `contributor.list-artifact-mutations` feature — unrelated to message/outcome/policy
     work and outside C4 (`SemanticMutation … unchanged`). `git log --date=iso -S
     "protocol::SemanticMutation<A::Snapshot>"` on the builder file shows this bound was first
     introduced 2026-08-16 03:32:28 (commit `dbcc4fa462`), well before this ticket's own lane work
     started, in a squashed multi-ticket auto-commit whose message references opening the
     `ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` ticket. `type Mutation = WorkflowMutation` on
     `SpaceApp` (`✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs:270`) itself has existed
     since 2026-08-09, unrelated to any of this.
   - **Puzzle**: the mutation enums DO derive `dsl::Mutations` and the derive correctly emits
     `impl SemanticMutation<Puzzle2dSnapshot> for Puzzle2dMutation` (per the `#[mutations(snapshot =
     Puzzle2dSnapshot, …)]` attribute at
     `…/◻2d/…/🧬️schema/🧬️mutations/🦀️component.rs:33-36`) — confirmed the derive itself is not
     broken (the plain-doc `Snapshot` type is right there). The error is a genuine snapshot-type
     mismatch: `✏️editor/🦀️component.rs:888-889` sets `Puzzle2dPlayApp`'s `ArtifactApp::Snapshot =
     Puzzle2dPlaySnapshot` (a `Value` newtype wrapper, distinct from `Puzzle2dSnapshot`) while keeping
     `Mutation = Puzzle2dMutation`, so the `editor_mutation_roster`/`document_app` bound now asks for
     `SemanticMutation<Puzzle2dPlaySnapshot>`, which nothing implements — the viewer's sibling
     `type Snapshot = Puzzle2dSnapshot` (matching) proves this is editor-only. `git log --date=iso -S
     "Puzzle2dPlaySnapshot"` on the editor file dates this `type Snapshot =` line to commit
     `5a1367dfcc` (2026-08-16 14:18:35), the editor/viewer split rework — a registration-site change,
     not a derive regression. Same shape for Puzzle3d/Puzzle5d. Not touched, per the brief's explicit
     instruction for this exact scenario.

## Verify — real cargo check counts

| Crate | Result | Remaining errors (all pre-existing/foreign) |
|---|---|---|
| `semio-s-plugin-flow` | **2 errors** (was blocked earlier by the framework file; now blocked by its own code) | `✏️editor/🎚️config/🦀️component.rs:286` and `✏️editor/👥️presence/🦀️component.rs:122` — both `E0053`, hand-written `impl Mutation<FlowConfig> for FlowConfigMutation` / `impl Mutation<FlowPresence> for FlowPresenceMutation` still return the bare snapshot instead of `MutationOutcome<..>`. **Newly exposed by fix #1** (previously masked because `semio-s-plugin-stdio` blocked the whole build transitively — see `📓️w3-flow-config-compose-report.md`'s "BLOCKED" note — stdio has since gone green). This is squarely this ticket's own fan-out recipe territory (hand-written config/presence dispatch enums) but was **not in my assigned four** and I did not touch it per the brief's strict scope — flagging for the coordinator to assign as a 5th item or confirm it's someone else's lease. |
| `semio-s-plugin-forms` | **0 errors** | — |
| `semio-s-plugin-playbook` | **0 errors** | — |
| `semio-s-plugin-procedural` | **0 errors** | — |
| `semio-s-plugin-demonstrator` | blocked transitively (no errors of its own) | via dependency `semio-s-plugin-gis` (`✏️s/🔌️plugins/🌍️gis/🦀️component.rs:15-16`, `ArtifactDeclaration`-returns-`Result` change, foreign) and `semio-s-plugin-puzzle` (see below) |
| `semio-s-plugin-raster` | **4 errors**, none ours | `👁️viewer/…/🧭️navigator/🦀️component.rs:47` "cannot find `composite` in `super`"; `🚪️io/…/dwg/🔖️ac1018/🦀️component.rs:6` `DwgSnapshot.bytes` field (FULL-STDIO); `👁️viewer/…/🖼️composite/🦀️component.rs:53,55` base64 `GeneralPurpose::decode`/`encode` (external crate API drift) — all as named in the brief, our `MutationDiff<RasterSnapshot>` error is gone |
| `semio-s-plugin-draw` | **1 error**, not ours | `👁️viewer/…/🖼️canvas/🦀️component.rs:11` `cannot find crate ui_wgpu` — as named in the brief, our `ArtifactEnvelope` initializer error is gone |
| `semio-s-plugin-space` | **1 error** (down from 2 originally attributed to us) | `🦀️component.rs:394` `WorkflowMutation: SemanticMutation<WorkflowSnapshot>` — triaged above, not ours |
| `semio-s-plugin-puzzle` | **3 errors** | `🦀️component.rs:44,48,52` `Puzzle{2,3,5}dMutation: SemanticMutation<Puzzle{2,3,5}dPlaySnapshot>` — triaged above, not ours |

`bun ./📜️script.ts verify mutation-outcome-law` — **passed, 0 breaches** (unchanged from before this
lane's edits).

## Files touched
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️commit-document/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-active-example/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-fixture-json/🦀️component.rs`

Logs: `🧪️g1-flow-check-before.txt`, `🧪️g1-flow-check-after.txt`, `🧪️g1-forms-check.txt`,
`🧪️g1-playbook-check.txt`, `🧪️g1-procedural-check.txt`, `🧪️g1-demonstrator-check.txt`,
`🧪️g1-raster-check-after.txt`, `🧪️g1-draw-check-after.txt`, `🧪️g1-space-check-before.txt`,
`🧪️g1-puzzle-check-before.txt`, `🧪️g1-gate-check.txt`.
