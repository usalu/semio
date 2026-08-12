# Status

Coordinator: Opus 5 session (sole writer of this file). Executors: Sonnet 5 agents. Scouts/verifiers: Haiku 4.5, read-only.
Plan authored by a Fable session: `/Users/ueli/.claude/plans/dissolve-and-unify-all-splendid-fountain.md`.
**Only the coordinator edits this file.** Agents append to their own report files.

## W0 — Recon, ticket, handshakes: DONE

Ticket opened (#2550, goal `🎯aioptimizedrepo`). `📌️important.md` written: doctrine table, SMO verb rulings, hard rules, full-path discipline, hot-file ownership, five-session protocol.

### Baselines (measured 2026-08-12, pre-edit)

| Metric | Value | Path |
|---|---|---|
| `fn set_` in claimed kernel dirs | **89** total | `🗺️surface/🗺️tiled-map` 29, `🗺️surface/🎨️paint` 27, `🗺️surface/🕸️node-graph` 12, `◻2d/🗄️store` 6, `🗺️surface/🏔️terrain` 5, `🧊️3d/🥽️mesh` 4, `🖥️platform` 4, `◻2d/⚙️engine` 2 |
| banned collection type | **40** | `💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs` |
| banned collection type | **70** | `💻️os/🔨️modules/🪐️space/🦀️component.rs` |
| `set-snapshot` mutation dirs | **70** repo-wide | 14 of 15 stdio `🧿️semio` subsets (all but `✳️text`) |
| existing `💡️inference*` dirs | **72** under `✏️s/` | corroborated independently by the IIF session |
| brep kernel | ~17,439 LOC / ~30 files | `🧰️framework/🔨️modules/🧊️3d/📐️brep/**` |

### Census corrections to the approved plan

1. **The banned collection type spans 9 framework files, not 2** — `💻️os/🦀️component.rs`, `🌊️flow/🌿️vcs`, `📡️spr/🦀️component.rs`, `🏪️store`, `📡️spr/🎮️command`, `🪐️space`, `🌿️vcs`, `♾️infinite/…/🕸️dag`, `💻️os/🖥️host`.
2. ⚠️ **CORRECTION (SMO, independently verified by DKM): TWO UNRELATED TYPES SHARE THE NAME.** My first map was wrong; this is the corrected one. `grep -rn "pub enum CollectionMutation"` returns exactly two definitions:
   - **`💻️os/🔨️modules/🌿️vcs/🦀️component.rs:280`** — the generic wrapper `CollectionMutation<TId, TItem, TPatch> { Add, Remove, Move, Patch }`. This is the type SMO's ticket targets and what most of the 9-file surface is about. **UCAS-claimed file.**
   - **`💻️os/🔨️modules/🪐️space/🦀️component.rs:736`** — a **non-generic domain enum** (`SetName`, `AddFolder`, `RemoveFolder`, `MoveFolder`, `RenameFolder`, …). Unrelated type, same name.
   - **`📡️spr` is only a re-export**, not a definition (`📡️spr/🦀️component.rs:27` pulls the name through `crate::os_spr::command`; `📡️spr/🎮️command` is mounted at `💻️os/📦️packages/🦀️rust/📦️glue.rs:160`, `🌿️vcs` at :217). **Consequence: the three-session sequencing problem I described does not exist.** Deleting the re-export is trivial once consumers are gone; the real removal happens in `🌿️vcs`.
3. **`🪐️space` (framework module) added to scope, but as its OWN work packet.** Its 70 hits are largely the *second* type. It is still squarely DKM's mandate (`Set*`-shaped, CRUD-flavoured), but it is **not the same job as flow's 40 and must not share a work packet or a success criterion**. Design constraint to solve or preserve deliberately: its own doc comment says `Move*`/`Rename*`/`ReplaceEntryBody` diff as the WHOLE post-mutation folder/entry record rather than a field delta, to sidestep the derive engine's lack of nested-`Option` support.
4. **SMO's ticket exit criterion is plugin-scope only** (zero banned tokens under `✏️s/`, all facets on the derive, allowlists empty). Framework-side elimination is explicitly not in their close — their final ratchet only demotes the generic type to `#[doc(hidden)]`/`pub(crate)` if reachable. So DKM clearing flow + space + infinite-dag is a clean continuation, not something SMO is blocked on. The one thing they ARE blocked on is `✏️s/🔌️plugins/🌊️flow/…/🧬️mutations/🦀️component.rs` still constructing the generic type in `from_framework_mutation`/`to_framework_mutation` — our pre-authoring shape handoff resolves it.
5. **`🪐️space` the PLUGIN is already released in SMO's predicate file.** If DKM's space-module work needs matching plugin-side changes, SMO makes them — we do not reach into `✏️s/🔌️plugins/🪐️space/**`.
6. **`set-snapshot` is far broader than brep/drawing** (70 dirs). SMO confirms all stdio ones are inside their ticket and die with no replacement. DKM's contribution is deriving real vocabulary for `✳️brep`/`✳️drawing`, which have none.
7. **`MeshExporter`/`MeshImporter` live inside our boundary** (`🔺️mesh/🦀️component.rs` `//#region MeshCodec` ~:811-882, re-exported at `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:50-55`). APA had been ceded the deletion by UCAS; DKM has taken it back since our mesh lane dissolves that file anyway.
8. **`🔣️taxonomy.json`'s `pluginChildDirs` is already flipped** to `["🎛️apps"]`. W6 must not be authored against the stale multi-entry value. `🗿️artifacts` must never be added to that key (no leaf `🦀️component.rs`; governed by `artifactsDirName`) — adding it panics the gate on all 33 plugins.

### Handshake outcomes (all five sessions; verbatim replies in the peer messages)

- **SMO** (`semio-9f`) — ruled on the full verb roster BEFORE authoring. Corrections: `replace-curve`/`replace-surface` (not `change-*`), `change-active-app{new_id}` (not `set-*`), `update-widget`/`update-synapse` **rejected** (option-bag `Patch` payloads are forbidden outright). `set-panel-visibility{panel,visible}` approved. `replace-primitive-geometry` reasoning approved but **the rename is SMO's, not ours**. SMO **handed DKM** the `🌊️flow/🌿️vcs` kernel bridge (it is the floor under their plugin-side elimination) and flagged `🪐️space`. Binding coupling: DKM sends the target enum shape for both files **before authoring** so their `from_framework_mutation`/`to_framework_mutation` bridge can be updated or deleted in step. Final `✳️brep`/`✳️drawing` slug lists go to SMO before authoring.
- **APA** (`semio-52`) — zero claim on all nine DKM paths, confirmed. Approved `DraftEngineSession` with a binding invariant (below). Ceded `MeshExporter`/`MeshImporter` deletion to DKM. Disclosed a **live correctness bug**: `🎪️demonstrator` registers IO for four kinds it does not own; for `3d.process`/`3d.procedural` both owner and demonstrator write the same key into a process-global `HashMap`, so **plugin load order silently decides the winner**. It fails non-deterministically rather than erroring. DKM lanes must NOT preserve current resolution behaviour — there is no well-defined behaviour to preserve.
- **UCAS** (`semio-b2`) — handshake sent; awaiting reply. Open asks: confirm no hidden claim; **write-handoff of `✳️brep`/`✳️drawing`/`✳️mesh` subset dirs** (stdio is theirs; DKM will not enter unilaterally); a corrigendum reversing the "engine survivor" carve-out in their `📓️design-full-plan.md`, which currently asserts the opposite of DKM's mandate; and their "composition primitives frozen" signal.
- **IIF** (`uds:…/64627.sock`) — the fifth session, resuming the inference-family ticket. Split agreed: **IIF excludes `✳️brep`/`✳️drawing`/`✳️mesh`; DKM authors those three inference facets**, because their derived fields are exactly what we extract while deleting `BrepEngineHost` and `DrawingEngine::compute`. IIF keeps the other ~31 semio subsets plus an 11-subset geometry/BIM file-format batch (no overlap — those are codec artifacts). IIF flips `💡️inference` into `schemaChildDirs` only **after** DKM authors the three, rather than punching an allowlist hole in `policySchemaRepresentationBreaches`.
- **`📜️script.ts` / `🔣️taxonomy.json` queue is now five deep**: APA → UCAS-W6 → SMO → IIF → **DKM (5)**. Verify with `git log --oneline -5 -- 📜️script.ts` that all four prior edits landed before starting. Report-mode first.

### Binding invariant for `DraftEngineSession` (negotiated with APA)

> Drop the session at any instant and rebuild from the draft base; if a user would notice anything other than a pause, the session is holding state it shouldn't.

Nothing in the session may be unrecoverable by rebuilding from the draft base. A field that survives that rebuild (a user-set tolerance, a selection, a pending value) is authored state living outside the store and belongs in the Draft snapshot with a real mutation. This is a **design** requirement, not a review checklist item: the type must make violation structurally hard (only `Rep` values keyed by a content hash of the base; no user-supplied fields). Precedent: APA's ruling that lowpoly's texture cache is an inference, not draft state.

## W1 — Mechanism: DISPATCHED

Sonnet agent, single writer of `💻️os/🔨️modules/⚙️engine/🦀️component.rs`. Deliverables: `//#region 🔖️EngineRep` (doctrine tier-(d) marker trait), `//#region 🔖️DraftEngineSession` (shaped to the invariant above, matching `💡️inference`'s `InferenceSession`/`InferenceCache` idiom rather than inventing a parallel one), `EngineCache` scope-narrowing note + an exhaustive construction-site census classified `wasm-boundary` vs `kernel-cache` (that table seeds W6's policy allowlist). Report: `📓️wave1-mechanism-report.md`.

## W2 — Exemplar (platform): DISPATCHED

Sonnet agent, single writer of `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` (245 lines, 4 setters). Chosen smallest-first deliberately: its report becomes `📓️migration-recipe.md` for six-to-eight later lanes. Three-way split required: authoritative UI state (`active_app_id`, `uri`, `panel_visibility`) → artifact snapshot + triads; runtime wiring (`ActionBus`, the `apps` registry — APA's declarative-registration territory) → left alone; **dirty-flag counters (`generation`, `chrome_generation`) → deleted, not migrated**, since the edit log already provides change notification (agent must grep every reader first and report consumers). Verbs fixed by SMO. Open question the agent must answer for everyone who follows: **where does a framework-module-owned artifact schema live**, given framework modules have no `🗿️artifacts/` tree — precedents to study are `🪐️space` and `♾️infinite`; the shape to imitate is stdio's `✳️text` facet (read-only, another session's). Instructed to report the placement question rather than create directories in a contested area.

## Remaining

W3a brep/mesh/2d fan-out (gated on W1 + W2 recipe + UCAS stdio handoff + SMO slug sign-off) · W3b surface (4 lanes, gated on W2) · W3c flow/space/db/infinite (gated on W1; flow+space gated additionally on sending SMO the target enum shape; db gated on the pre-existing `semio-framework-os-kernel-db` breakage, ~53 errors, `task_9a4155cc`, which predates DKM) · W5 serializer · W6 ratchet at queue position 5 · W7 adversarial verify + close.
