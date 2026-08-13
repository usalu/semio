# Status

## 📡️ AUTHORITATIVE CROSS-SESSION STATE (read this instead of messaging me)

Peer session *names* rotate after session-limit restarts and I have misrouted messages three times today. **This section is the single source of truth**; any session may read it.

## 🛠️ glue.rs RECOVERY — a concurrent session reverted this ticket's mounts

**What happened**: commit **497** (`382ace1b27`) overwrote `📦️glue.rs` with a version derived from an older copy plus that session's own inference additions — **silently reverting every mount this ticket added** for `✳️table`/`✳️graph`/`✳️object`/`✳️kit` (36/45/39/57 mount lines). The auto-committer then made that the new HEAD, so `git show HEAD:` returned the *regressed* file. Detected only because stdio failed with `cannot find 'table' in 'subsets'` despite all five directories existing on disk.

**Recovery performed** (merge, not overwrite — their work was preserved):
1. Restored the good version from commit **496** (`20252aa16d`, 8187 lines, our mounts intact).
2. Merged back the 4 `✳️drawing` inference mounts that only 497 had — verified by diffing inference-mount counts per subset between the two commits.
3. Replaced the 2 stale `📄set-snapshot` blocks (dirs deleted by DKM) with **17 `✳️drawing` + 13 `✳️brep`** generated triad mounts.
4. Mounted **31 previously-unmounted `💡️inferences` facets** found by sweeping the artifact tree rather than waiting for the compiler (many referenced only from test code, so invisible to a plain `cargo check`).

**Peak result achieved: stdio `--all-targets` clean, suite 2248 run / 2246 passed / 2 failed / 5 skipped** — the only failures the two unowned `dwg`/`ifc` `fixture_honesty_law`. IIF's four inference failures had by then been fixed by them, so the baseline improved from 6 failures to 2.

**Lessons recorded:**
- **`git show HEAD:` is not a safe restore point here.** Auto-commit can promote a regressed working tree to HEAD within minutes. Restore from a *known-good commit hash*, verified by content (line count + a marker grep), never from `HEAD`.
- **A merge, not a revert.** Blindly restoring 496 would have destroyed the peer's inference work; diffing per-subset mount counts made the merge safe.
- Unicode normalisation defeats literal emoji filenames in scripts — detect files by listing a directory, never by comparing a hard-coded `"🦀️component.rs"`.

## ▶️ W3 IN PROGRESS — stdio green, exemplars found real bugs under test, both being fixed

**stdio is confirmed green** as of this check: `cargo check --all-targets` clean; `cargo nextest --profile long`: **2423 run, 2418 passed, 5 failed, 5 skipped** (the same 5 pre-existing non-UCAS failures as before — csv/binary/dxf/dwg/ifc, all owned by IIF or unowned; unchanged). #2553's `⚙️engine` fan-out churn from the previous entry settled.

**`draw` independently confirmed fully done**: `cargo nextest -p semio-s-plugin-draw` → 93/93 passed. No further work needed on draw.

**`lowpoly` compiles clean but its own test suite found a genuine architectural bug** — not a false alarm, a real framework-law violation: `LowpolyObject.mesh_workspace: String` (ephemeral halfedge-mesh JSON, explicitly documented as "never the persisted representation") sits ON the persisted `LowpolySnapshot` struct, so `store::os_store::test_support::assert_document_text_round_trip`'s generic law — `parse_dsl(print_dsl(snapshot)) == snapshot` by exact `PartialEq` — fails by construction, because the hand-rolled codec correctly drops it while the live in-memory value keeps real content. Confirmed via full diff (`mesh_workspace: ""` parsed vs `mesh_workspace: "{halfedge JSON}"` live). **General lesson**: any field on a persisted snapshot struct that a codec deliberately excludes violates this law — the field must live in a genuinely separate session-side cache (the `draw::DrawSession` / DKM `EngineRep` pattern), never embedded in the persisted type. Dispatched a focused fix: move `mesh_workspace` off `LowpolyObject` into the app's session state, rewire the 18 touching call sites (mutation triads, engine, session, commands).

**`cad` needs a second round.** The schema-level migration (composed `model`/`drawing` children replacing the old inline B-Rep topology) is done and correct — `CadSnapshot` is coherent. But the app layer (`🎛️apps/📐️cad/🦀️component.rs`, 2505 lines) never got threaded through: 84 compile errors, almost all `objects`/`building_objects`/`energy_objects`/`structure_classic_objects`/`*_geometry` fields that no longer exist on `CadSnapshot`. Dispatched a focused completion: introduce an ephemeral `CadWorkingScene` (the `EngineRep` pattern again) built from resolved `SemioModelSnapshot` content, rewire the app's panes/commands onto it.

**lowpoly: DONE, independently re-verified.** `cargo check --all-targets` clean; `cargo nextest`: **124 run, 123 passed, 1 failed** (matches the fix agent's own numbers exactly). The one remaining failure (`inference_determinism_law`) is a pre-existing, unrelated fixture-grammar gap the agent correctly diagnosed and declined to touch — its `example.dsl.semio` fixture uses a structured half-edge grammar the hand-rolled hex/bracket codec doesn't implement, already flagged in that file's own doc comments before this ticket started.

**Fix quality worth recording**: this wasn't a mechanical field move. `mesh_workspace` now lives in `LowpolyScratch.mesh_workspace: HashMap<String, String>` (session-side, mirroring `draw::DrawSession`), and the agent added a genuine fail-safe — `LowpolyDocument::reload_meshes` now verifies the cached JSON's content-hash still matches the persisted `mesh` handle before trusting it (`LowpolyCoreError::StaleMeshWorkspace` on mismatch), because it discovered store-level undo/redo bypasses `ArtifactApp::handle` entirely, so a live session's cache can go stale across an undo of `create-mesh`/`delete-mesh`. It also caught and fixed a determinism bug along the way: two independent `HalfedgeMesh::box_prim().unwrap_uv().to_json()` calls weren't reliably byte-identical, spuriously tripping the new staleness check on ~30 tests — fixed by memoizing the combined build behind one `OnceLock`. 21 files touched (18 original + 3 discovered as direct consequences), self-caught 2 missed call sites in `🎮️commands/🧵️uv` before final verification.

**cad: still running (round 2, app-layer completion).**

## ⏸️ (superseded) Earlier: W3 exemplars CODE-COMPLETE, final verification blocked by fresh concurrent churn (#2553)

**lowpoly and cad migrations both landed on disk and both compiled clean at last check** (see their reports). Both W3 agents were then terminated mid-final-verification by a session limit.

**Orchestrator repaired 2 genuine mine-to-fix defects found while re-verifying stdio:**
1. Two stray `//!` inner-doc-comment lines in `☁️ply`'s `🚪️io/🦀️component.rs` (E0753, invalid mid-file inner doc) — converted to plain `//`.
2. `🖊️dxf`'s schema facet had `DxfSnapshot` imported twice (two separate `use` statements) — consolidated to one.
3. `📜️docx/🦀️component.rs`'s top-level `io_registry` still imported the pre-migration path `standards::v_ecma_376::engine::io_registry`; repointed to `standards::v_ecma_376::subsets::any::io::io_registry`, matching the pattern already working for `json`/`mp4`/`wav`/`mp3`/`avi`. Also removed 3 identical stale `pub mod engine { pub use …engine::*; }` external-caller shims in `📦️glue.rs` for docx's `any`/`strict`/`transitional` subsets — the underlying `⚙️engine` dir was already deleted per an in-file comment citing ticket #2553's mandate; the shim just hadn't been removed in the same change.

**Then hit a genuinely live, moving target — did not chase it, per this ticket's own established rule.** Immediately after the docx fix, stdio broke again: `📷️png` lost its `⚙️engine` dir (confirmed via `git log`, commit 501 — landed *during* this verification pass, after this session's earlier clean check). Widening further to `pptx`/`xlsx`/`docx` again as more commits landed while checking. This is ticket **#2553's own active fan-out** deleting `⚙️engine` directories across many stdio artifacts in real time — not a defect, not attributable to W1/W2/W3, and chasing a target that a peer session is actively rewriting is the exact trap this ticket already documented once today (the brep/drawing mount race). **Stopped here rather than continue fighting it.**

**Resume condition**: wait for #2553 to signal their stdio `⚙️engine` fan-out complete (or re-check `cargo check -p semio-s-plugin-stdio --all-targets` periodically), then: (1) re-verify lowpoly + cad end-to-end, (2) distill `📓️migration-recipe.md` from their two reports, (3) launch W4 fan-out across the remaining ~29 plugins, batched by SMO/APA/DKM/#2553 clearance exactly as W2's batching was. W4→W7 not started — realistic remaining scope at this pace is multiple further agent-hours, not completable in one sitting given the frequency of legitimate concurrent-session interruptions today.

## ▶️ W3 RUNNING — stdio unblocked, plugin migration started

DKM completed `✳️brep`/`✳️drawing`/`✳️mesh`. **stdio is green**: `cargo check --all-targets` clean; suite **2409 run / 2404 passed / 5 failed / 5 skipped**.

Notable: 3 of those 5 failures are `💡️inferences` tests that had **never executed before** — their facets were on disk but unmounted, so mounting them turned dead tests live (`binary::extent`, `dxf::bounds`, `zip::entries` `inference_default_law`). The other 2 are the long-standing unowned `dwg`/`ifc` `fixture_honesty_law`. None are UCAS's; all belong to the inference-family ticket or nobody.

**W3 exemplars dispatched** (parallel, disjoint plugin trees):
- **lowpoly** — the worst offender: `mesh_json: String` (an opaque JSON string with a 15-line comment explaining the DSL gap that forced it) → `ArtifactChild<SemioMeshSnapshot>`; paint layers → image children if they carry raster bytes; plus removal of its duplicate `3d.mesh` kind declaration.
- **cad** — deletes one of the repo's four independent B-Rep topologies (`CadEdge`/`CadWire`/`CadFace`/`CadShell`/`CadSolid`/`CadGeometry`) in favour of composed `model` + `drawing` children; retains genuinely cad-specific view/projection state. Its engine already consumes `SemioMeshSnapshot`/`SemioBrepSnapshot`, so the conversion path is reused rather than rewritten.

Both briefed: consume stdio subsets, never edit `✳️brep`/`✳️drawing`/`✳️mesh` (DKM's), approved verbs only, `--all-targets` verification, foreground cargo.

`semio-s-plugin-stdio` currently fails (14 errors) because DKM (#2550) is mid-replacement of the banned whole-document vocabulary in `✳️brep`/`✳️drawing` — subsets **we handed them**. Their dispatch enums now reference triad modules whose `#[path]` mounts are still being added. This is expected churn from an authorized handoff, **not a defect in our work**, and every plugin depends on stdio, so W3 exemplars cannot be verified until it clears.

**Mount ownership transferred**: DKM now owns `📦️glue.rs`'s mount blocks for `✳️brep`/`✳️drawing`/`✳️mesh`. Rationale — the original boundary gave them the *directories* but not the *mounts that make those directories real*, so every deletion was guaranteed to strand a `#[path]` reference in a file they could not touch. DKM's phrasing is worth keeping: **a boundary that separates a definition from its registration is not a boundary, it's a race.** Apply the same test to composition slots in `📸️snapshot/`: if a slot ever needs a matching mount or dispatch entry, one owner takes both.

Orchestrator removed two stale `📄set-snapshot` mounts (`✳️drawing`, `✳️brep`) to unblock; the breakage moved between subsets mid-fix, which is what prompted the transfer.

**Resume condition**: DKM reports brep/drawing/mesh complete → re-run the full stdio suite as the gate → then W3.

# 🧊️ ROSTER FROZEN — 2026-08-12. stdio `🧿️semio` v1 is 18 subsets + `✳️any`. SMO / APA / IIF / DKM / #2553 may start.

**Verified independently by the orchestrator, not taken from an agent report:**
```
subsets on disk                                        → 19 entries = 18 + ✳️any ✓
cargo nextest --profile long -p semio-s-plugin-stdio   → 2174 run: 2168 passed, 6 failed, 5 skipped
cargo nextest -p semio-framework-plugin                → 150 run:  150 passed, 0 failed
```
The 6 stdio failures are the **pre-existing, non-UCAS baseline** and are NOT a reason to hold: `dwg` + `ifc` `fixture_honesty_law` (unowned), `html`/`json`/`pdf` `inference_default_law` and `md` outline (IIF's, being fixed — `csv` already passes).

**Final roster** — `animation audio brep cad document drawing flow graph image kit mesh model object presentation table text value video` + `✳️any` (18-arm union).

⚠️ **Two names changed meaning — do not author against the old ones:**
- `workflow` → **`flow`** (rename).
- the old value-tree `object` → **`value`**; **`object` now means a *spatial* thing** (transform + owned brep/mesh/value children). Anything written against the old `object` semantics belongs to `value`.

New since the start: `text`, `table`, `graph` (leaves) and `object`, `kit` (the first **composite** subsets, carrying real `ArtifactChild`/`ArtifactLink` slots — `kit` is the first user of the link-slot verbs `bind`/`unbind`/`change-link-pin`).

**Directory structure is final.** Composition child/link slots on the *pre-existing* subsets (`mesh` gaining image children for textures, `drawing` composing text/image, `model` composing objects, `cad` composing models) are a **later wave** and will not move or rename any directory.

### ⚠️ W1's "signed off" was PREMATURE — two real mechanism bugs found by running its tests

W1 was declared signed off on `cargo check --all-targets` being clean. **Compiling is not passing.** APA flagged failures in `semio-framework-plugin`'s test surface; running it showed **2 failed / 150**, and both were the *core* of this ticket — the composite-gesture and group-undo tests. Two genuine mechanism defects, now fixed:

1. **`register_child` did not seed the ownership edge.** Its sibling `open_child` does, and `dispatch_group`'s phase-1 check (`CompositionGraph::owner_of`) is **fail-closed** — so any child adopted via `register_child` was rejected as an `OwnershipViolation` the first time anything dispatched against it. Two public entry points, only one maintaining the invariant the validator depends on. Fixed: `register_child` now seeds the edge and returns `Result<(), Fault>` (3 in-crate call sites updated).
2. **One `InvocationResult` identified the parent document two different ways.** Its `KernelMutation.document` used `ArtifactHandle(meta.instance_id)` while its `member_edits` entry used `artifact_handle_of(parent_id)`. Any consumer correlating `mutations` with `member_edits` by handle would silently fail to match the parent — i.e. group undo could not find the parent's own edit. Fixed: parent keeps the instance handle, children keep the content-addressed one.

**Now: `cargo nextest run -p semio-framework-plugin` → 150 passed, 0 failed.** W1 is signed off *on evidence* this time.

**Lesson, third instance today**: `cargo check` — even `--all-targets` — proves compilation, never behaviour. Every W1/W2 sign-off must run the tests. Both of these defects were invisible to a clean `--all-targets`.

### ✅ ROOT CAUSE CONFIRMED — and it was NOT the diff shape

The 6 UCAS failures are **fixed**. Current: **2066 tests, 2060 passed, 6 failed, 5 skipped** — and **none of the 6 remaining are UCAS's** (4 are IIF's pre-existing inference tests, 2 are unowned `dwg`/`ifc` `fixture_honesty_law`).

Actual causes, found by the `✳️text` agent on resume:
1. **`✳️any`'s two hand-maintained grammar `.semio` files were missing `| "text"` in their tag alternation.** Invisible to `cargo check`; only the grammar-conformance tests catch it. That was both `any::…::diff_grammar_conformance_law` and `ops_grammar_conformance_law`.
2. **A `din4108`-derived test helper computed inverses against a stale base** — fixed in `text`'s copy, and confirmed as a **latent bug in the reference pattern itself** (flagged to SMO, not ours to fix). That was the three `text` round-trip laws.

**The earlier "whole-list diffs are apply-then-capture" diagnosis was wrong, and retracting it was correct.** Neither cause has anything to do with diff shape. Had that retraction not happened, the "fix" would have reworked `✳️text` + `✳️table` (120 files) + `✳️graph` (142 files) onto DiffKit — ~380 files of churn — and left both real bugs in place. **Prove the cause before changing the design.**

| stdio `🧿️semio` subset | state |
|---|---|
| `✳️text` | authored; **6 failing law tests**, root cause under investigation |
| `✳️table` | authored (120 files); placeholder fixtures, not mounted |
| `✳️graph` | authored (142 files); placeholder fixtures, not mounted |
| spatial `object`, `kit` | **not started** |
| renames `workflow`→`flow`, `object`→`value` | done, clean |

**Grants and queues (all live):**
- `🔌️plugin/🦀️component.rs` — **RELEASED to APA (#2549)**, acknowledged by them, in progress. **#2553 is next.** UCAS re-enters only to repair `ArtifactChildren`/`ChildEmit`/`dispatch_group`/`SpaceMember`.
- `🔣️taxonomy.json` (at `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/`, **NOT repo root** — my earlier "repo-root" wording was wrong and was propagated widely) — **#2553 next**, then UCAS W6. Their `⚙️engine` mandate repeal is verified sound: `trait ArtifactEngine` has **0 hits** repo-wide; both rules to delete are `policySubsetEnginePresenceBreaches` (`📜️script.ts:5626`, aggregated `:5807`) and `policyArtifactEnginePresenceBreaches` (`:6418`, aggregated `:7066`) — each has a second registration site, delete both halves.
- `📜️script.ts` (repo root) — APA done; queue UCAS-W6 → SMO → IIF → DKM.
- stdio `✳️brep` / `✳️drawing` / `✳️mesh` — **mutation vocabulary handed to DKM (#2550)**. UCAS retains *composition slots only* (`ArtifactChild`/`ArtifactLink` fields in `📸️snapshot/`, later). DKM pings before entering `📸️snapshot/` on mesh/drawing.
- `schemaChildDirs` missing `💡️inferences` — **IIF's (#2546)**, land with or after their fan-out, never before.

**Current stdio baseline**: 2066 tests, **2054 passed, 12 failed, 5 skipped**. Of the 12: 4 are IIF's pre-existing inference failures (csv now passes — they are fixing them), 6 are UCAS's (text ×4, any ×2), 2 are `dwg`/`ifc` `fixture_honesty_law` which are **unowned**.

**✅ RESOLVED — UCAS vs DKM engine question.** The halfedge `Body` is **not** a snapshot and must never become one: it is an ephemeral working representation derived from the authoritative `SemioBrepSnapshot` via `EngineRep::build(&seed)` and dropped when the call returns. Our carve-out was **right about the types, wrong about the lifetime** — "behind traits, never serialized" permits a long-lived host-owned kernel session, which is hidden authoritative state. `EngineRep` is strictly stronger: `build(&P)` is the only constructor, it may not outlive the call, and it must be wholly derived. `BrepEngineHost` and `📐️cad`'s `static OnceLock` pass every *mutability* test and still fail all three — ambient **reach**, not ambient mutability.

Corrected rule: *engine types survive as ephemeral `EngineRep`s rebuilt from the snapshot; host-owned kernel sessions do not.* Recorded in `📓️design-full-plan.md`. #2553 already verified compatible. APA's ownership check constrains registration, not representation, so it never added a third constraint.

**Coupling finding worth keeping**: DKM flagged spatial `object` as downstream of brep/mesh. It is far weaker than that — `ArtifactChild<S>` is `{child_id, target, PhantomData<S>}`, a two-string handle carrying no snapshot content. Internal churn in `SemioBrepSnapshot`/`SemioMeshSnapshot` cannot propagate to `object`; only renaming/deleting those types or changing the kind strings would. **This is the child-as-own-document decision paying off directly: the parent holds two strings, not a subtree.**

**Known gap DKM will leave flagged, correctly**: `create-loop`/`delete-loop` go unauthored — `Loop`/`Coedge` carry no `PersistentLabel`, and arena ids are generational and reused after deletion, so those verbs have no valid stable address. SMO approved them before that was known and has wound down. Leaving them empty and flagged is the sanctioned outcome; inventing an address would not be.

**Rule adopted after three failures today**: *a release is not released until acknowledged.* Facts true when sent were consumed as if still true, with no ack closing the loop — that was the actual defect behind the `🔄️fsm` misreport, the `📓️status.md` clearance confusion, and the unacknowledged W1 release.

## W0 — Recon: DONE

- Ticket opened, goal `🎯aioptimizedrepo`.
- `📌️important.md` (hard rules + hot-file ownership) and `📓️design-full-plan.md` (the approved plan) written.
- `📓️smo-clearance.md` snapshot computed: 18 plugins SMO-clear, 15 not (incl. stdio itself — expected).
- Baseline `CARGO_TARGET_DIR=🎯️target cargo check -p semio-framework-os-kernel`: clean, 49 pre-existing warnings, 0 errors (`scratch-baseline-kernel-check.txt`).
- SMO's own `📓️status.md` at recon time: wave 0 done, wave 1 exemplars (cad/draw/gis×2/fem-3d) done, wave 2 mass fan-out in flight (25 reports so far). No mention of wave 4 (ratchet) starting — `🔌️plugin/🦀️component.rs` is clear for W1 to claim.

## W1 — Kernel primitives + CompositionCoordinator: IN PROGRESS (restructured)

**Claim**: this ticket owns `🔌️plugin/🦀️component.rs`, `🚪️io`, `🧬️schema`, `🛂️manifest`, `📡️spr`, `🏪️store`, `🌿️vcs`, `🎠️kernel`, `🗣️dsl`. Re-verified at restart: SMO is still at wave-2 (25 reports, no wave-3/4 dirs, status.md unchanged) — no ratchet contention on `🔌️plugin/🦀️component.rs`.

**First attempt (single mega-agent) was killed** when the driving session exited, landing ZERO edits (`git status -- 🧰️framework` empty) and no report. Restructured into crate-scoped bounded rounds so each increment is independently verifiable and survivable:

| Round | Agent | Crate | Scope |
|---|---|---|---|
| 1 | A1 | `semio-framework` | `🚪️io` `🔖️ArtifactRef` region (ArtifactRef, ArtifactKindId, canonical-grammar validator, uri codec) + `🎠️kernel` InvocationId/EditRef/`UndoGroup.member_edits` |
| 1 | A2 | `semio-framework-schema` | `🔖️ArtifactCompositionSpec` (ChildSlotSpec/LinkSlotSpec/ArtifactCompositionFields, GraphQL preamble) + `#[derive(ArtifactSchema)]` slot emission |
| 2 | B1 | `semio-framework-os-kernel` | `📡️spr` `MutationMeta.group_id` + `🌿️vcs` `Checkpoint.composition_pins` & new VcsError variants |
| 3 | B2 | `semio-framework-os-kernel` | `🏪️store` `🔖️Composition` (ArtifactChild/ArtifactLink/OwnerRef/LinkPin/ArtifactRefs/LinkResolver/ChildStoreFactory, envelope.owner) + `🔖️CompositionCoordinator` (dispatch_group, SpaceMember wire methods, CompositionGraph) + handcrafted DSL/Pack encoding of the new value types |
| 4 | C1 | `semio-framework-plugin` | `Emit.child_emits`, VcsArtifactApp children map, dispatch_emit group routing, group undo/redo, ArtifactChildren + derive_artifact_facets! children arm, WIT `resolve-artifact-link` |
| 5 | D1 | `semio-framework-os-kernel` | testkit composition laws + kernel unit tests |

Crate map established at restart: `semio-framework` = 🚪️io, 🛂️manifest (mounts 🎠️kernel via #[path]:2824), 🔺️mesh, …; `semio-framework-schema` = 🧬️schema+derive; `semio-framework-os-kernel` = 🌿️vcs, 🎒️pack, 🏪️store, 📡️spr, 🗣️dsl, ⚙️engine, 💡️inference, 🧩️extension, 🧬️semio; `semio-framework-plugin` = 🔌️plugin.

Reports land in `📓️wave1-reports/`.

### Design deviation D1 — no dedicated `Shape::Child`/`Shape::LinkRef` DSL variants

The approved plan carried these from the earlier INLINE-child design, where a child slot embedded a nested snapshot and needed a block-with-header text form. The user's child-as-own-envelope decision reduced `ArtifactChild` to a flat two-string handle (`child_id` + `target: ArtifactRef`), so an ordinary record encoding is sufficient and loses nothing — composition semantics are carried by the schema-level slot tables (`ArtifactCompositionFields`, agent A2), not by the wire shape.

Measured cost of keeping them: `Shape::` is exhaustively matched across ~20 files in this crate (🎒️pack ×4, 📡️spr/📜️history, 🏪️store, 🪐️space, ♾️infinite ×3, 🔁️workflow, 🌊️flow/🌿️vcs, 🗣️dsl ×8). Adding enum variants would break every one of those matches for zero capability gain. Dropped; the DSL/Pack encoding of the new value types is handcrafted inside `🏪️store` by agent B2 instead (the pattern `BackboneMessage` already uses there).

### Orchestrator collateral fix (outside any agent's boundary)

A1's additive `UndoGroup.member_edits` broke two struct literals in `🔌️plugin/🦀️component.rs` (E0063 at :5415, :5476 — serde defaults do not help Rust struct construction). Fixed in place by the orchestrator (`member_edits: Vec::new()` on both) because Round 4's owner had not started and leaving `semio-framework-plugin` red would have poisoned every downstream agent's verification. `cargo check -p semio-framework-plugin` green afterwards.

### Progress

- **A1 (framework-core): DONE** — `🚪️io:85-185` ArtifactRef/ArtifactKindId + validator + uri codec (5 tests); `🎠️kernel` EditRef:451, `UndoGroup.member_edits`:465, TS mirror :242-253; `InvocationId` already existed at :46 and was reused. `cargo check -p semio-framework` clean; `cargo test -p semio-framework --lib` 125 passed.
- **A2 (schema): DONE** — `🧬️schema:93-140` `🔖️ArtifactCompositionSpec` (ChildSlotSpec/LinkSlotSpec/ArtifactCompositionFields with `&[]` defaults, GRAPHQL_COMPOSITION_PREAMBLE) + tests :695-732; `#[derive(ArtifactSchema)]` now emits `ArtifactCompositionFields` by syntactic field classification; TS mirror updated. `cargo check -p semio-framework-schema` clean; `--lib` 9/9 passed. Two findings promoted into `📌️important.md`: the derive-crate glue duplication rule, and `#[link(...)]`→`#[link_slot(...)]` (`link` is a built-in Rust attribute — hard error as a field attribute).
- **B1 (spr+vcs): DONE** — `MutationMeta.group_id: Option<String>` (`📡️spr/🎮️command:389-425`) threaded through `.spr` persistence via `HistoryOpMeta.group_id` (`📡️spr/📜️history:83-97`, presence-bit-4 in `write_op_meta`/`read_op_meta` :602/:651) and the store bridge fns (:1169-1194), round-trip proven narrowly and through the shared `sample_log()` fixture. `CompositionPin` + `Checkpoint.composition_pins` (`🌿️vcs:109-129`), `content_addressed_checkpoint_id` pin-extended (byte-identical for empty pins, proven by reimplementing the old formula in a test) with deterministic sort. `VcsError::{CompositionCycle, OwnershipViolation}`. Check clean, 49 warnings = baseline; `--lib` 802 passed / 2 failed, both `os_dsl::fixture_sweep` over fem/norm/dag plugin fixtures = live SMO churn (retried 3×, grep-proven unrelated).
- **B2 (store composition + coordinator): DONE** — `🔖️Composition` (`🏪️store:166-571`): `ArtifactChild<S>` with hand-written `Clone`/`Debug`/`PartialEq` + `#[serde(bound="")]` to dodge the `PhantomData<S>` bound trap, `OwnerRef`, `ArtifactLink`/`LinkPin`, `ArtifactRefs`, `LinkResolver`/`LinkState`, `ChildStoreFactory` + registry, handcrafted `DslField` records (no new `Shape` variants, per D1). `🔖️CompositionCoordinator` (`:4494-5003`): `CompositionGraph` (ownership forest + link DAG), two-phase `dispatch_group` with reverse-order compensation, `undo_group`/`redo_group`. `CompositionPin.child_ref` corrected to the real `crate::os_io::ArtifactRef`. 15 new tests pass; check clean at 49 warnings; `--lib` 817 passed / 2 failed (the same pre-existing fem/norm/dag fixture-sweep pair).
  - **Deviations accepted**: `SpaceMember` gained 8 object-safe methods, not 3 — the extra 5 (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`, …) are needed to stamp group ids and owners through a type-erased interface. `GroupReceipt` gained `created_children` because a genesis-created member has no caller-held reference and would otherwise be silently dropped, making `ChildGenesis` pointless. `CompositionCoordinator` is STATEFUL (owns the graph) because `SpaceMember` is fully type-erased, so a cross-document ownership graph cannot be derived from a single call's arguments. `GroupMeta.actor`/`coalesce_key` accepted but not wired (no object-safe seam yet) — filed as a `sharedFileRequests` item.
  - **Correctly deferred**: making `ArtifactEnvelope.dialect` required — measured blast radius 106 files / 168 call sites. Left `Option`, filed rather than half-done.

### W1 COMPLETE (kernel). Now running:
- **C1 (plugin composition runtime)** — `Emit.child_emits`/`ChildEmit`, `VcsArtifactApp` child-store map, `dispatch_emit` group routing, group undo/redo, `ArtifactChildren` + `derive_artifact_facets!` children arm, WIT `resolve-artifact-link`. Crate `semio-framework-plugin`.
- **W2a (`✳️text` subset exemplar)** — crate `semio-s-plugin-stdio`. Deliberately ONE subset end-to-end as the template for the remaining four (`table`, `graph`, spatial `object`, `kit`), mirroring the exemplar→fan-out pattern that worked for SMO and is planned for our W3/W4. Its report carries a `## Template for the remaining subsets` section. `🚪️io` hub-routing leaves are explicitly out of scope for that round.

Different crates, so the two run in parallel safely.

### Correction to B1 — `ArtifactRef` IS reachable from os-kernel

B1 concluded `ArtifactRef` could not be imported into `semio-framework-os-kernel` without inverting the dependency graph, and fell back to a raw `String` for `CompositionPin.child_ref`. **That premise was wrong.** `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` is **dual-mounted**: `semio-framework` mounts it as `io`, and os-kernel mounts the *same source file* as `os_io` (`💻️os/📦️packages/🦀️rust/📦️glue.rs:237-238`, `#[path = "../../../../🔨️modules/🚪️io/🦀️component.rs"]`). Store already consumes `crate::os_io::ArtifactDialect` at :88/:105/:662. B2 is fixing `CompositionPin.child_ref` to the typed `crate::os_io::ArtifactRef` and using the real type throughout, so the ticket keeps ONE identity type rather than degrading to stringly-typed refs — the specific outcome this ticket exists to prevent.

`InvocationId` is a genuine exception: `🎠️kernel` is mounted only by `🛂️manifest` inside `semio-framework`, so it really is unreachable here and `group_id: Option<String>` stays.

### Environment discovery — the repo AUTO-COMMITS

A background process periodically commits the whole tree (`🐙️ueli…🚩️<n>`; flag 492 landed mid-wave and swept B1's framework edits). Consequence: `git status` reports CLEAN for work that landed minutes ago, so it is **not** a churn detector, and a vanished edit is committed, not lost. `📌️important.md` rule 6 now directs all agents to `git log --oneline -- <path>` + mtime instead. This also explains why an early progress check showed zero framework modifications despite confirmed edits.
- **W2-prep (stdio subset renames): running** — see stdio claim below.

## W2 — stdio: CLAIMED (prep started early)

**Claim**: this ticket owns `✏️s/🔌️plugins/🗄️stdio/**` from now. Justification for starting before full SMO clearance: stdio is 0/37 covered by SMO and would otherwise never clear; SMO has been dormant since our recon (wave-2 report count static at 25, no wave-3/4 dirs, `git status -- 🗄️stdio` empty). If the SMO session resumes and needs stdio, it must coordinate via this file.

### Cross-session agreement with SEMANTIC-MUTATIONS-OVERHAUL (SMO), 2026-08-12

The SMO session is **live, not dormant** (my earlier inference from a static report count was wrong — that count was a *previous* session's stopped wave). It currently has ~9 migration lanes running. Agreement reached over the session channel:

1. **stdio ordering: we go first, SMO follows.** Ours changes which subsets *exist*; theirs rewrites the mutation *vocabulary inside* each. SMO is holding all ~53 stdio mutation facets — its stdio lane is written but unlaunched, and nothing of theirs is in `🗄️stdio/**`. I must signal **"roster frozen"** (not merely "compiles") before they start.
2. **New subsets must be born conforming.** SMO asked that the 5 new subsets not be scaffolded with banned vocabulary. We take the strict path, not their empty-enum fallback: every new facet follows `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` + `📓️fanout-brief.md`. Three hard rules — no `SetSnapshot` (whole-doc replace goes through `ArtifactStore::reset`, outside history; a locked user decision), no `NoMutation` (return `Vec::new()` from `MutationKind::inverse`), no public `CollectionMutation`. If a subset cannot be done conformingly, leave its enum EMPTY and flag it — never invent vocabulary.
3. **The `✳️workflow`/`✳️object` mutation facets carry across unchanged** — we invest nothing in their existing vocabulary; that debt is SMO's to clear.
4. **`✳️any` becomes an 18-way union**; SMO has the final variant list and migrates that facet last. Trap flagged to them: the name `object` survives the roster change but its *meaning* does not (old value-tree → `✳️value`; `✳️object` is now spatial).

### Future overlap: `🏪️store/🦀️component.rs` (agreed protocol)

SMO's **final ratchet** also lands in this file — it adds `SemanticMutation` trait bounds to `ArtifactStore` and starts populating `MutationMeta.semantic_kind`/`label` (both currently written `None` at every construction site, including the six we touched for `group_id`). Our work there is the `🔖️Composition` + `🔖️CompositionCoordinator` regions and the handcrafted DSL/Pack encoding of the new value types — different regions, so no collision expected. Their ratchet is gated behind every facet migrating, so it is realistically the last thing they do. Agreed protocol: **they ping before entering the file and will wait rather than race if we are mid-edit.** Reciprocate.

### Third session: ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE (APA, #2549) — scope split

A third session (`semio-52`) owns APA: make artifacts the ONLY IO/state/registration mechanism in plugins (a plugin = apps + artifacts). It deletes per-plugin `🛂️manifest`/`🎟️capabilities`/`🔧️setup` facet dirs, replaces imperative `.setup(fn)` with declarative `ArtifactDeclaration`, seals the global `register_*` family behind a `Registrar` token, deletes the `semio_framework_os::register_mesh_*/solid_*/dwg_*/app_io` escape hatches, and capability-gates WIT imports/HostEffects. Agreed split:

1. **Registration CEDED to APA.** Our `declare_artifact!`/`plugin!` macros are deleted from this plan — APA's declarative shape subsumes them, with a fuller design behind it. We keep only the composition *runtime* in `🔌️plugin/🦀️component.rs`. Signal APA when C1 unfreezes the file.
2. **W2 scope reduced**: stdio subset roster only, NO registration migration — avoids half-converting 37 artifacts to a shape APA would replace.
3. **W6 scope reduced**: `MeshExporter`/`MeshImporter` deletion is APA's.
4. **APA takes repo-root `📜️script.ts` + `🔣️taxonomy.json` BEFORE our W6**, sequentially — our W6 sits behind W2/W3/W4/W5 and blocking their seal that long is unreasonable. They add their five policy regions (report-mode only, non-gating), notify, then W6 adds ours without touching theirs.
   - ~~Hazard: flipping `pluginChildDirs` before per-plugin cleanup would fail W2 verification.~~ **RETRACTED — the hazard does not exist, and the ordering is REVERSED.** APA read the enforcement code and pushed back; independently verified all three of their findings:
     1. `🔌️plugin/🦀️component.rs:2226-2235` holds a **runtime `assert!`** (not a lint) that reads `pluginChildDirs` dynamically and asserts `<plugin>/<child>/🦀️component.rs` is a file, per plugin. Deleting `🔧️setup/` from even one plugin while the list names it panics the gate — so the flip is a **precondition** for incremental facet cleanup, not a consequence of it.
     2. `policyPluginRootShapeBreaches` (`📜️script.ts:4646-4692`) and the registry codegen (`📇️registry/📜️script.ts:1232-1236`) are **presence-only** — they flag *missing* leaves and have no absence/forbidden check. Flipping the list is purely permissive; stdio's leftover facet dirs go unflagged. Our objection was raised from plan text without reading the enforcement.
     3. The flip target in APA's own plan was wrong and would have broken all 33 plugins: `🗿️artifacts/🦀️component.rs` exists in **0 of 33** plugins (all 33 have `🎛️apps/🦀️component.rs`), and `🗿️artifacts` is governed by the separate `artifactsDirName` key (`🔣️taxonomy.json:173`). Correct target is **`["🎛️apps"]`**.
   - Consumer APA missed, reported back: `🔍️discovery/🟦️component.ts:174` (type) and `:582-586` (`validateTaxonomy` **requires a non-empty array**). `["🎛️apps"]` satisfies it, but emptying the list later — tempting under APA's "plugin = apps + artifacts" thesis — is a schema change, not a value change, and `🧪️index.test.ts:1290-1293` asserts the rejection.
5. **W4 ordering: UCAS before APA, per plugin**, using our `📓️wave4-reports/<plugin>-report.md` as their clearance oracle (mirroring how we gate on SMO). Rationale: our pass deletes plugin-local types and repoints to stdio; theirs moves files between dirs — moving first would invalidate our agents' paths mid-flight. Offered them **demonstrator, vcs, space, note** immediately (our change there is reference-only/trivial).

**Boundary corrections issued to APA:**
- The `register_mesh_exporter`/`register_app_io` family also lives in `💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (OUR file), not just their two — they file `sharedFileRequests`, we remove during C1.
- `🛂️manifest` is two different things sharing one emoji: `🧰️framework/🔨️modules/🛂️manifest/` (ours) vs `✏️s/🔌️plugins/*/🛂️manifest/` (theirs). A `**/🛂️manifest/**` glob would cross the line.
- Their `💻️os/🖥️host/🦀️component.rs` is NOT our `💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (IoRouter, needed by C1's WIT work).

**Three sessions are now in the plugin tree** (SMO mutations, UCAS content, APA structure). Each publishes per-plugin reports; each gates on the others'.

### Design corrections C1/C2 — verb set, after peer review

**C1 `adopt` → `inline`.**

SMO's `📓️taxonomy.md` revealed that the closed `APPROVED_VERBS` table already defines `extract`/`inline` as inverse partners meaning "hoist a fragment into a reusable entity / dissolve back" — exactly our child↔standalone promotion semantics. Our planned `adopt` verb is NOT approved. Dropped in favour of `inline`.

**C2 `update-link-pin` → `change-link-pin`** (caught by SMO's review of our verb set). `update` is reserved for an inseparable ≥2-field facet rewritten atomically. Re-pinning sets the single `pin` field while `target`/`role` stay put, and a link with a stale pin is meaningful (that is the point of `Head` vs a frozen pin) — so it is `change`, record `ChangedLinkPin`. Root cause worth remembering: `update` was reached for as a generic "modify", the exact habit the mutations overhaul exists to kill; `pin`'s value being an enum-with-payload made it *look* multi-field, but an enum-with-payload is still **one field taking one cohesive value**. That distinction recurs for `LinkPin`'s neighbours.

Also ruled and recorded in `📌️important.md`: **`bind`/`unbind`, not `connect`/`disconnect`** — a link fills a *named slot* as a handle, not an edge row in an edge collection ("a parameterization gets bind/unbind instead").

Net: the complete composition verb set is **entirely within the existing approved core** — `create`/`delete`, `extract`/`inline`, `bind`/`unbind`, `change` — so this ticket needs no verb-spine change in `📡️spr`. Plan file and `📓️design-full-plan.md` updated; stale `Shape::Child`/`Shape::LinkRef` references in the design doc also corrected to match deviation D1.

### W2-prep (subset renames): DONE

`✳️object`→`✳️value` (76 files) and `✳️workflow`→`✳️flow` (72 files), exhaustive: dirs (plain `mv`), Rust type/field/fn/const families, all 5 schema-twin languages plus grammar/protocol/spicy/ksy leaves, the `✳️any` union arms, `glue.rs` `#[path]` tree, engine registrations, `🪆️subsets/🔣️component.json`, and ~52 cross-subset doc-comment citations elsewhere in stdio. Fixtures **regenerated** from real `print_dsl`/`encode_pack` output (greenfield rule — not hand-migrated) for `🕸️graph`, `🌊️pipeline`, `🌐️envelope`.

Two judgement calls worth keeping:
- **Collision avoided**: the subset's top-level snapshot-diff struct became `SemioValueTreeDiff`, NOT `SemioValueDiff` — the latter already exists as the recursive leaf-value diff. Renaming blindly would have collided two distinct concepts.
- **Bulk-substitution bug caught**: `JsonValue::Object` (an unrelated external enum from the `json` subset, cited inside the value subset's JSON bridge) was mis-rewritten to `JsonValue::Value`; found via compiler errors and reverted. Exactly the "read each hit in context, do not blind-sed" hazard the brief warned about.

**Verification**: `cargo check -p semio-s-plugin-stdio --tests` 0 errors. `cargo nextest --profile long -p semio-s-plugin-stdio`: **2021 passed, 5 failed, 3 skipped** (2026 total), reproduced identically twice. The 5 failures are `inference_default_law`/outline tests in the csv/html/json/md/pdf `inferences` facets — the agent reports them git-clean and pre-existing (i.e. untouched by this or any concurrent session).

✅ **Independently verified — 2021 / 5 / 3 IS the stdio baseline.** Orchestrator re-ran `cargo nextest --profile long -p semio-s-plugin-stdio --no-fail-fast` and reproduced the numbers exactly. The five are:
```
csv|html|json|pdf  …/💡️inferences/…::tests::inference_default_law
md                 …/💡️inferences/outline/…::collects_headings_and_counts_words_and_blocks
```
**Decisive evidence they are pre-existing**: `git log` on those `💡️inferences` facet dirs shows last commit `a46ac1f883` (flag **491**) — the HEAD at this session's start. Every commit from this ticket is 492+. They therefore cannot have been touched by the renames. Corroborated by scope: they are `💡️inferences` facets of the csv/html/json/md/pdf *format* artifacts, untouched by the `🧿️semio` subset work; SMO attributes them to ticket `26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`. **Any run showing more than these five is a new regression.**

(Transient blocker cleared: `semio-framework-os-kernel` briefly went red with 29× `E0753 expected outer doc comment` from a stray `//!` at `🏪️store:179` written by our own in-flight B2 agent. SMO pinpointed it to the exact line; B2 corrected it itself before intervention. Note for future one-character "helpful" fixes in a live region: the correct repair was `//`, and guessing `///` would have silently attached a doc comment to whatever item landed next — compiling cleanly while being wrong.)

Prep agent runs the two mechanical subset renames only (`✳️workflow`→`✳️flow`, `✳️object`→`✳️value`), which are independent of the kernel primitives and unblock the rest of W2. New subsets (`text`, `table`, the spatial `object`, `graph`, `kit`) and child/link slots come in the main W2 agent after Round 3 lands.

## Session-limit interruption + orchestrator repairs (2026-08-12 ~17:30-18:15)

Both in-flight agents (C1, W2a) were killed mid-edit by an API session limit. With agents unavailable, the orchestrator stabilised the tree directly:

1. **Repo-wide test-build blocker CLEARED.** C1's child-store map made `VcsArtifactApp` non-`Send` (`PluginApp` requires it), producing 57× `E0277 dyn SpaceMember cannot be sent between threads safely` and blocking test builds for **all five concurrent sessions**. C1 had chosen and documented the right fix before dying — `pub trait SpaceMember: Send` (`🏪️store:3902`) over widening `Box<dyn SpaceMember + Send>`. Four residual errors fixed by the orchestrator:
   - `🔌️plugin:3152` `TutorialBase { document_dsl }` → `artifact_dsl`; `:3439` `definition.document_json` → `artifact_json`. Stale references to fields renamed by the **closed** ticket `26/08/10/RENAME-DOCUMENT-TO-ARTIFACT-THROUGHOUT-CODEBASE` — same orphaned debt that left the panel mounts. (Trap avoided: `source.document_json()` one line above is a *method* that still exists.)
   - `🔌️plugin:10326`/`:10368` `IoPayload::Text("")` → `String::new()` inside the `subset!` macro.
   - Result: `cargo check -p semio-framework-plugin --all-targets` **clean**.
2. **`tempfile` dev-dependency added** to `💻️os/📦️packages/🦀️rust/Cargo.toml`, target-gated `cfg(not(target_arch = "wasm32"))` to match the test gating. The crate had **no `[dev-dependencies]` section at all** while `🏪️store/🔄️sync` used `tempfile::tempdir()` at 3 sites. This unblocked `cargo check -p semio-framework-os-kernel --tests` and with it the triad law harness that four sessions were gated on.
3. **Nine stale panel mounts repointed** `📌️panels/📄️document` → `📄️artifact` in `📦️glue.rs` for writer, flow, imperative, dag, forms, reasoning, sequence, vcs, lowpoly. Traced to commit `c31024cc6c` (flag 480, 1801 files) from the same closed rename ticket. Fix direction was settled by measurement, not guesswork: **0 of 33** plugins retained a `📄️document` panel while 9 glue files still mounted it.

### ⚠️ ENVIRONMENT BLOCKER — disk at 98%

`df`: 18 GiB free of 926 GiB. Repo is 458 G, of which the default `./target` is **428 G**. Symptoms already seen: corrupt dep-info (`could not parse/generate dep info`), spurious third-party failures (`futures-executor`, `icu_*`, `schemars`, `libc` build script). These are environment artifacts, **not code** — do not chase them, do not `cargo clean`, do not delete any target dir.

**Near-miss worth recording**: `./target` read as stale (mtime 17h old, zero recent activity) and deletion was approved on that basis — but a re-check immediately before acting showed `.rustc_info.json` and `wasm32-wasip2/` touched minutes earlier, and `ps` showed **18 live rustc processes** (os_kernel, stdio, compose_rs, framework_math, framework_ui) from other sessions. Deletion was aborted. Third instance today of an observation of this tree ageing out between reading and acting — and the first where acting on it would have been destructive. **Re-verify immediately before any irreversible action, not merely before deciding on one.**

## W1 SIGNED OFF (2026-08-12 ~19:30) — `🔌️plugin/🦀️component.rs` released to APA

`cargo check -p semio-framework-plugin --all-targets` → **Finished, 0 errors**, 47 warnings (pre-existing). Disk recovered to 51% used / 429 GiB free, so builds complete again.

**Near-miss worth recording.** A reply was drafted telling APA the file was released *and green*. It was not — C1's own final output reported a failed check, and a re-run showed **10 errors**. Had that draft gone out, APA would have started work on a red file and reasonably attributed the breakage to itself. **Always re-verify before asserting a state to another session, especially when the assertion unblocks them.**

All 10 were in C1's new **test** code while the library compiled — the same hiding place as both earlier repo-wide blockers. Fixed by the orchestrator:
- `VcsArtifactApp.{store, children, composition}` (`:5307`, `:5333`, `:5338`) and `absorb_created_children` (`:5860`), `dispatch_action` (`:6149`) → `pub(crate)`. Deliberately **not** `pub`: the public API is unchanged, and the widening exists solely so the in-crate test module can reach them. Flagged to APA in case their purity/SDK-surface lint counts crate-visible items.
- `ChildrenTestConstruction` (`:10510`) gained `#[derive(Clone, Debug)]`.
- A test called `super::artifact_handle_of(…)`; the helper lives in `crate::app` and `super` from `plugin_builder_contract_tests` does not reach it — path corrected.

**Standing lesson, now three-for-three today**: a green `cargo check` proves nothing about `--all-targets` in this repo. Every blocker found today (`📌️panels` mounts, `SpaceMember: Send`, `tempfile` dev-dep, these 10) lived in `#[cfg(test)]` code.

## 🚨 W2 VERIFICATION FOUND A REAL DEFECT (D2) — roster further from frozen than reported

First full stdio run since the subsets landed: **2066 tests, 2054 passed, 12 failed, 5 skipped.**

| failures | owner |
|---|---|
| `text::…::insert_remove_run_round_trips`, `add_remove_mark_round_trips`, `reorder_runs_round_trips`, `text::io::…::fixture_honesty_law` | **OURS** |
| `any::io::derived_composition::…::diff_grammar_conformance_law`, `ops_grammar_conformance_law` | **OURS** |
| html/json/md/pdf `inference_default_law` + outline | pre-existing (IIF) — note csv now passes, so IIF is fixing them |
| `dwg`/`ifc` `fixture_honesty_law` | neither ours nor SMO's — unowned, likely the DWG schema-id ticket |

**Root cause: NOT YET ESTABLISHED. An earlier entry here blamed whole-list diffs; that was retracted as unproven.**

The retracted claim was that `✳️text`'s whole-list collection diff is apply-then-capture and therefore the cause. Evidence against it, found on closer inspection: `SemioTextDiff::apply` is correct (`next.runs = list.values.clone()`), and the failing assertion (`restored == ["hello"]` against a 2-run fixture) is only reachable if **the forward mutation had no effect at all** — `InsertRun.diff(base).apply(base) == base`. Whole-list-vs-sparse cannot produce that; an **empty diff** can. Prime suspect is therefore the dispatch enum failing to route a variant to its triad leaves. The `🔺️diff` file also argues in place that whole-list is honest for `text` because its snapshot has exactly one mutable field — a reasonable argument that was dismissed too quickly.

A debug agent is on it under explicit instruction to **prove the cause before changing any design**, because `✳️table` (120 files, 8 triads) and `✳️graph` came from the same template and a wrong fix multiplies by three. First diagnostic: assert `forward != base` inside `round_trip`.

SMO was told the wrong diagnosis and has been sent a retraction, since they might have grepped their fan-out lanes on the strength of it.

**Whether sparse-vs-whole-list should be enforced at all is now a separate, non-blocking question** for the vocabulary owner (SMO), decided on its merits — not because a test failure was mis-attributed to it. DiffKit primitives, if wanted: `IndexedTripleDiff`/`indexed_apply` (`📡️spr/🎮️command:510,531`), `NamedTripleDiff`/`named_apply` (`:468,489`).

**Correction to the record**: `✳️text` was reported here and to SMO as "complete and audited clean" on the strength of SMO's four-gate structural audit. That audit was accurate about structure and says nothing about correctness — **the gates verify a `pub fn diff` exists, not that it is sparse.** A structural audit is not a correctness audit; only the law tests distinguish them. SMO has been told, since the same shape may be passing their fan-out lanes' sign-off.

### Sixth session + `🔌️plugin` queue

`26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` (#2553) is a sixth ticket in this tree. It reclassifies `⚙️engine` across 95 dirs, promotes draw's FSM into a core `🧰️framework/🔨️modules/🔄️machine/`, and revives the dead presence lane by adding a 4th defaulted generic + `presence_mutations` to `Emit` — the same struct C1 extended. Verified their census: `presence_mutations` is **0 repo-wide**, `Emit` (`:4060`) has 3 generic params with a `Default` at `:4085`, so a 4th defaulted param absorbs cleanly.

**File queue agreed: APA (now) → #2553 → others.** UCAS is out of `🔌️plugin/🦀️component.rs` except to repair composition items.

**Design ruling issued to them**: presence must **NOT** route through `CompositionCoordinator::dispatch_group`. That path stamps `group_id`, records `UndoGroup.member_edits`, and makes edits jointly undoable *and durable* — correct for multi-document gestures, wrong for presence, which is ephemeral and non-undoable. Wire it like the **draft** lane. Routing it through the coordinator would push cursor/selection churn into undo stacks and the durable `.spr` log.

**Two open cross-ticket conflicts flagged, not ours to settle alone:**
- `🖍️draw/🔄️fsm/` is wanted by both APA (→ artifact engine tree) and #2553 (→ core framework module). APA's earlier attempt briefly broke the **workspace manifest** (root `Cargo.toml:66-67` + draw's manifest pointing at a vanished path — a manifest load failure aborts cargo before compilation, so it is *not* hidden behind `--tests`).
- **Three tickets now disagree on what "engine" means**: our design keeps framework kernel types (brep topology, halfedge mesh, `DrawingScene`) as engine types behind traits with stdio subsets as persisted interchange; DKM (#2550) says it reverses that carve-out; #2553 reclassifies `⚙️engine` wholesale. Needs one agreed answer.

## State at hand-off (2026-08-12 ~18:40)

**W1 (kernel): CODE COMPLETE.** A1, A2, B1, B2, C1 all landed. C1's remaining tasks 5–6 (`ArtifactChildren` + `DerivedArtifactSpec::Children` + `derive_artifact_facets!` children arm; WIT `resolve-artifact-link` in `📜️world.wit`) are confirmed present on disk. Report at `📓️wave1-reports/c1-plugin-composition-report.md`.

**W2 (stdio roster): 1 of 5 new subsets done.** `✳️text` complete, integrated into `✳️any` (14 arms), and independently audited clean by SMO on all four mechanical gates. Template for the remaining four (`table`, `graph`, spatial `object`, `kit`) at `📓️wave2-reports/w2a-text-subset-report.md`. Renames (`workflow`→`flow`, `object`→`value`) done earlier.

**⚠️ VERIFICATION OUTSTANDING — environment, not code.** Both agents were killed by a session limit, then on resume stalled in idle-wait loops (~600k tokens each) on builds that never returned. Cause: ~20 concurrent `rustc` processes from five sessions on a volume at **98%** (18 GiB free; default `./target` = 428 G). Builds exceed 10 minutes and fail with disk-pressure artifacts — corrupt dep-info, spurious third-party failures (`futures-executor`, `icu_*`, `schemars`, `libc`).

**Before signing off W1/W2, on a quiet machine, run:**
```
cargo check -p semio-framework-plugin --all-targets      # expect clean
cargo check -p semio-framework-os-kernel --tests         # expect clean, 49 warnings
cargo nextest run --profile long -p semio-s-plugin-stdio --no-fail-fast
   # expect >2021 passed, exactly 5 failed (csv/html/json/pdf inference_default_law + md outline), 3 skipped
```

**Do NOT spawn further agents until the machine is quiet** — they cannot verify, and they burn budget idling on the build lock.

**Roster is NOT frozen**; four subsets remain. SMO (52 stdio mutation facets), APA (stdio registration conversion) and IIF (34-subset inference fan-out) are all explicitly waiting on the "roster frozen" signal, which has not been sent.

## Remaining

W2 stdio roster (gated on W1 + stdio SMO-clearance or explicit claim), W3 exemplars, W4 mass fan-out (~29 plugins, width 7), W5 serializer, W6 policy ratchet, W7 verify+close. Per the "no pause between waves" precedent SMO itself set, each wave launches automatically as the prior one's report lands — this will span many turns/notifications.

## Update (2026-08-13) — W2 frozen, W3 exemplars: lowpoly + cad done, writer/draw next

Since the hand-off above: **W2 roster is frozen** (all 18 subsets + any landed, stdio SMO-cleared, broadcast to SMO/APA/IIF). W1's `register_child` ownership-graph bug and the dual-identity `InvocationResult` bug (both flagged in the hand-off's outstanding-verification section) were found and fixed; `semio-framework-plugin` is green.

**W3 lowpoly: complete.** 123/124 tests (1 pre-existing unrelated: `inference_determinism_law`, a DSL grammar gap). Real architectural fix landed, not just a schema migration: `mesh_workspace` moved off the persisted `LowpolyObject`/`LowpolyObjectPatch` onto `LowpolyScratch.mesh_workspace` (session-only cache), with a new `LowpolyCoreError::StaleMeshWorkspace` fail-safe in `reload_meshes` since store-level undo/redo can bypass `ArtifactApp::handle`. Full report: `📓️wave3-reports/lowpoly-report.md`.

**W3 cad: complete.** 137/139 tests, 2 confirmed pre-existing/unrelated failures (both traced via `git log -p` to commits dated 2026-06-04, two months before this ticket — an interaction-spec-asset path test off-by-one `..` and a STEP-repair test with a self-evidently wrong hardcoded expectation). Round 3 (this session, done directly rather than via agent) fixed a real codec-completeness bug round 1's own schema work had introduced: `references_by_model_definition_id`/`nodes` were added to `CadSnapshot` but never wired into the hand-rolled text/pack codecs, silently dropping data on every save/reload — fixed, plus the demo DSL fixture was regenerated from real `print_dsl()` output (it predated the current codec format). Full report: `📓️wave3-reports/cad-report.md`.

**Standing lesson reconfirmed twice more this round**: neither the "stdio churn is the blocker" caveat nor the "7 pre-existing failures" classification survived independent verification untouched — both were partially right (some failures genuinely were unrelated/pre-existing) and partially wrong (a real, ticket-introduced bug was hiding inside the "pre-existing" bucket). Never accept an agent's out-of-scope classification without tracing it to a commit.

**Next**: writer or draw as the third W3 exemplar (text/2d family) — draw was already confirmed clean with zero additional work needed earlier in this ticket, so writer is the remaining pick. Then distill `📓️migration-recipe.md` from the three exemplars before starting W4's ~29-plugin fan-out.

## Update (2026-08-13, continued) — W3 complete: writer exemplar done, all three exemplars verified

**W3 writer: complete, independently re-verified.** `WriterSnapshot.text: String` → `document: store::ArtifactChild<SemioDocumentSnapshot>` (`#[child(kind = "s.stdio.semio.document")]`), hand-rolled `ArtifactDsl`/`ArtifactPack` codecs (same wall lowpoly/cad hit — `dsl::DslRecord` derive doesn't reach through a composed child), real bidirectional converter (`document_snapshot_from_text`/`text_from_document_snapshot`, one `DocBlock::Code` leaf, lossless), `WriterWorkingScene` ephemeral `thread_local!` cache mirroring lowpoly's `LowpolyScratch` (no `LinkResolver` seam exists yet at the framework layer, confirmed by direct inspection — same documented gap as cad). Banned `WriterMutation::SetSnapshot` replaced with `reset_document_effect`/`HostEffect::LoadDocument`.

The agent found the writer crate was already red before touching it (16 pre-existing compile errors — stale mutation-variant references, a dead import, unrelated stdio pdf/docx schema drift from APA) and fixed all of them as part of reaching green, rather than deferring. Independently re-verified by the orchestrator (not trusted at face value, per this ticket's standing rule):
```
cargo check -p semio-s-plugin-writer --all-targets   → clean, reproduced
cargo nextest run -p semio-s-plugin-writer --no-fail-fast → 100/100 passed, reproduced
grep for SetSnapshot/NoMutation/CollectionMutation in WriterMutation → none; only command-layer names remain, correctly routed outside history
```
Full report: `📓️wave3-reports/writer-report.md`.

**W3 exemplars are now ALL DONE**: lowpoly (123/124, 1 pre-existing), cad (137/139, 2 pre-existing traced to 2026-06-04 commits), writer (100/100). Every exemplar independently verified, not just trusted from an agent's final claim. `📓️migration-recipe.md` distilled from all three. W4 mass fan-out (~29 plugins) underway.

## Update (2026-08-13, continued) — W4 batch 1 dispatched (process, fem, gis, flow); fem lands honest partial

`remodel` was skipped for batch 1 — found with a live uncommitted edit matching ticket #2553's in-flight `⚙️engine`-dissolution pattern (an `engine as X` → `subsets::any::io as X` import fix in its video component), substituted `flow` instead. See `📌️important.md`'s new "W4 fan-out tracking" section.

**`fem`: partial, correctly so — not a shortfall.** The design doc's "ONE fem-core... kills 11-type dup + 4 mesh types" turned out to only be 2/11 types genuinely byte-identical (`FemDof`, `FemAnalysisSettings` — now consolidated, `fem3d` re-exports `fem2d`'s copies, zero behavior change, verified compile-clean and 335/356 tests passing both before and after). The other 9 types differ in real shape (2D vs 3D DOF/section/geometry are not the same data) and forcing them into one type would recreate the "whole-object-replace with half the fields always None" anti-pattern this ticket's own D2/Concern B already flags. More importantly: the "4 mesh types" and the mesh-consuming engine code live in `✏️s/🔨️modules/🏗️fem/⚙️engine/**` — a **shared module tree outside the plugin's own artifact directory**, moved there by ticket #2553 specifically because "an artifact is a schema + io system, never an engine." Independently confirmed (grepped the glue.rs mount myself): true, not an excuse. The remaining 21 test failures were traced via `git log` to commits from 2026-08-10 and earlier-2026-08-13, both before this task started; spot-checked one myself and it matches exactly. Filed as `sharedFileRequests`: mesh-type dedup needs a future wave scoped to the `⚙️engine` module directly (different ownership, not in this ticket's plugin-fan-out boundary), and the brep/drawing-link half needs the same missing `LinkResolver` seam every exemplar has already flagged. Full report: `📓️wave4-reports/fem-report.md`.

**`flow`: complete, independently re-verified.** `FlowSnapshot.{widgets,synapses,layout}` → `content: ArtifactChild<SemioFlowSnapshot>` (stdio's canonical flow subset — `camera` stays inline, no stdio counterpart). No codec wall hit (flow already hand-rolled its codec, never derived `DslRecord`). Real converter (every `Widget` variant round-trips through `FlowParam` key/value pairs), `FlowWorkingScene` thread_local cache mirroring writer's pattern, 9 pre-existing mutation triads rewired onto the scene. Re-verified: `cargo check` clean, `cargo nextest run` → **93/95**, exact same 2 failures reproduced (`host_from_snapshot_deletes_edge_selected_by_synapse_domain`, `delete_selection_action_removes_selected_synapses`) — confirmed via `git status` that the framework file they trace to (`♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`) genuinely has a live staged edit from another session, outside this plugin's boundary. Report: `📓️wave4-reports/flow-report.md`.

**`gis`: complete, independently re-verified.** `gisterrain` composes `Option<ArtifactChild<SemioMeshSnapshot>>` (content-addressed off exaggeration+imported-features), `gismap` composes `drawing`/`image`/`value` children derived from its existing positions/routes/regions (image stays honestly `None` — no raster-basemap capability exists). Duplicate `3d.mesh` `ArtifactKindSpec` registration deleted — re-verified myself via grep: only legitimate media-port string tags remain, plus a test that explicitly asserts the old duplicate declaration is gone. Both snapshots got hand-rolled codecs, both demo fixtures regenerated. Re-verified: `cargo check` clean, `cargo nextest run` → **171/171**. Report: `📓️wave4-reports/gis-report.md`.

**Batch 1 status: process complete.** 0 compile errors, 157/157 tests (see correction below). **Batch 1 (process, fem, gis, flow) is now fully done.**

## 🚨 Correction (2026-08-13) — the auto-commit message's embedded date is FAKE; two reports drew a false conclusion from it, both now fixed

While independently re-verifying process's report, cross-checked its cited commit's date with `git log --date=iso` and found it landed **2026-08-13**, not "2026-06-04" as its message text (`🐙️ueli🎆️26🌙️06☀️04🚩️503`) claimed. Checked 20 consecutive auto-commits spanning 2026-08-11 through 2026-08-13: **every single one carries the identical, fixed `🎆️26🌙️06☀️04` string regardless of its real date** — it's a stale template baked into the auto-committer, not a timestamp. Full warning now in `📌️important.md`.

This invalidated two "confirmed pre-existing, two months old" claims that had used the fake date as evidence:
- **cad's final 2 failures** (`every_interaction_asset_on_disk_parses_as_interaction_spec`, `repair_step_trailing_comma_before_close_paren_is_quote_aware`) — re-investigated on the merits (not by date), both were trivial self-contained test bugs (an off-by-one path, a wrong hardcoded expectation) unrelated to composition. Fixed directly. cad is now genuinely **139/139**.
- **process's 3 failures** (`rename_machine_round_trips`/`change_machine_icon_round_trips`/`replace_machine_capabilities_round_trips`) — the report's own root-cause diagnosis (an index-`[0]` test-fixture assumption colliding with `Workshop::default()`'s seeded generic-machine roster) was independently re-derived and confirmed correct on the merits, just wrongly dated. Fixed directly. process is now genuinely **157/157**.

**fem's classification is unaffected** — its agent used `git log -1 --date=iso` (real dates) throughout, not the message text; independently spot-checked one of its citations and it matches exactly.

**Open risk for W7 final verification**: any "pre-existing/unrelated" classification made *before* this correction landed (anything in `📓️wave3-reports/lowpoly-report.md`, or earlier stdio/W1/W2 work) that cited a commit's dating from the message text rather than `git log --date=iso` should be treated as unverified until re-checked. lowpoly's one remaining failure (`inference_determinism_law`) was diagnosed by content (a structured half-edge grammar the hand-rolled parser can't read yet) rather than by date, so it's likely unaffected, but has not been re-confirmed against this correction.

**Batch 1 (process, fem, gis, flow): fully done.** fem landed an honest partial (2/11 type dedup, rest architecturally out of plugin-only scope); process/gis/flow all fully complete and independently verified, cad now also fully green. Proceeding to batch 2.

## Update (2026-08-13, continued) — batch 2a dispatched; DKM cross-session coordination; sequence blocked by DKM's live churn

Dispatched batch 2a: sequence, imperative, mathematical, dag (all confirmed clean of concurrent edits before dispatch). DKM (DISSOLVE-KERNELS-AND-MODULES ticket) messaged asking about cad/spatial-kernel file ownership and a taxonomy amendment — replied: cad is fully clear (our composition work there is committed, plus one trivial STEP-test-assertion fix just made), spatial-kernel has zero UCAS edits and unknown provenance, and the taxonomy amendment belongs to #2553 (not us) since that ticket owns the "engineless" policy — also passed along the auto-commit-date-is-fake warning since they'd have hit the same trap tracing that spatial-kernel commit.

**`sequence`: migration itself complete (agent report: 126/128, 0 compile errors at authoring time) but still blocked on independent re-verification, twice now.** `cargo check -p semio-s-plugin-sequence` fails with 6 errors, all still in `🧰️framework/🔨️modules/🖱️ui/**` (zero in sequence's boundary). DKM reported fixing this exact class of error (`cargo check -p semio-framework-ui --features wgpu → 0 errors`), but re-checking just now shows the SAME 6-error shape with the broken import renamed from `semio_framework_math` to `semio_framework_geometry` — DKM's own math-dissolution rename is still actively moving through that file. Not sequence's bug, not worth chasing a live target — will re-verify again once it settles rather than re-checking on every tick.

**`imperative`: complete, independently re-verified.** `ImperativeSnapshot` composes stdio's `flow` subset (control-flow tree, `control.*` bodies JSON-encoded into a reserved param) and `text` subset (JSON-encoded seed map — an honest documented non-prose use, no separate prose field exists). Hand-rolled codec, real converters, `thread_local!` working-scene cache mirroring writer/flow. Re-verified: `cargo check` clean, `cargo nextest run` → **93/94** (exact same failure, `delete_step_inverse_law`, traced by code comparison — not dates — to a pre-existing append-only mutation design, out of this agent's SMO-governed scope to redesign). Report: `📓️wave4-reports/imperative-report.md`.

DKM confirmed (separate message) they independently hit the same fake-auto-commit-date trap tonight and are correcting their own ticket's records — cross-session gotcha, not unique to us.

**`mathematical`: complete, independently re-verified.** Composes `text`/`table`/`value` (node labels→text runs, node id/x/y→table rows, everything else→one structured value map), `thread_local!` working-scene cache, all 14 mutation triads rewired, dead `SetSnapshot`-shaped diff slot removed outright (found unused, not just banned). The design brief's claimed degenerate `"a"` kind id was searched for and genuinely not found — reported honestly rather than invented. One trivial pre-existing test gap was fixed outright; the other failure matches this ticket's own documented D2/Concern B class (whole-collection diff against stale `base`), deferred to the same future DiffKit rework already tracked. Re-verified: `cargo check` clean, `cargo nextest run` → **72/73**, exact match. Report: `📓️wave4-reports/mathematical-report.md`.

**`dag`: complete, independently re-verified.** `DagSnapshot.{nodes,edges}` → composed `SemioGraphSnapshot` child, hand-rolled codec, real converter for an 11-variant node-kind enum via an "honest string boundary," working-scene pattern, 14 mutation triads + ~137 app-layer call sites rewired. Caught and fixed a real self-introduced bug mid-migration (first codec draft persisted only the opaque handle, silently vacuous-passing tests on fresh-process reload) before shipping. Re-verified: `cargo check` clean, `cargo nextest run` → **93/95**, exact match on both failure names (traced to a pre-existing append-only mutation defect, commit dated correctly via `--date=iso` this time, predates this migration). One `sharedFileRequests` item: the framework's own separate `DagFixture::default()` parses this plugin's fixture file independently via `include_str!` — regenerating the fixture may need a framework-side follow-up if any out-of-plugin caller exists. Report: `📓️wave4-reports/dag-report.md`.

**Batch 2a: all 4 done** (sequence, imperative, mathematical, dag), all independently re-verified.

**cad integrity check after DKM's promised brep rewrite landed**: confirmed my one-line STEP-comma test fix survived untouched exactly as DKM said it would, and `cargo check -p semio-s-plugin-cad --all-targets` is still clean after their `BrepEngineHost`→`InferredField` rewrite of `🚪️io`/`💡️inferences`/`🚪️io/🗺️geometry-import`. No action needed.

## Update — batch 2b: trinity skipped (live DKM edit), norm's first pass pushed back on

`trinity` skipped for batch 2b — live edit from DKM's own math→geometry/graph crate-extraction rename found mid-flight in `🗿️artifacts/🔌️jack/**`. Dispatched `reasoning` (→C:graph, dag's sibling target) instead, still in flight.

**`norm` (~60k lines, 15 artifacts, verified against real code not just the design doc's count): first pass landed real, verified LocalizedText dedup (1104/1107 tests, independently re-verified — exact match) but did ZERO subset composition**, on the argument that composing e.g. `en1990.q_k: Vec<En1990QkEntry>` into a `table` child would regress its 5 granular mutation triads into a whole-blob-replace, citing this ticket's own D2/Concern B warning. **That reasoning doesn't hold up** — D2/Concern B is about stdio's OWN internal collection-diff sparsity, not about whether a composing plugin loses mutation granularity; `mathematical`'s own wave-4 report is direct counter-evidence (14 granular triads over graph/geometry content, composed 3 subsets, zero granularity lost, via the working-scene + re-mint-handle pattern). Dispatched a corrective follow-up citing `mathematical`/`dag`/`process` as the counter-precedent, scoped to 2-4 of norm's smallest artifacts (starting with `en1990`) rather than all 15, appending to the same report rather than overwriting the good LocalizedText work. In flight.

**`reasoning`: complete, independently re-verified** (crate is actually named `semio-s-plugin-reasoning-mindmap`, artifact `wires` — noted for anyone else's future reference). `WiresSnapshot.board_fixture: DslValue` → composed `SemioGraphSnapshot` child, `camera`/`meta` stayed as separate small persisted fields (view state, not graph data). Caught a real bug mid-migration: routing `DslValue` through `serde_json::Value` silently reorders object keys and broke order-sensitive equality — fixed by using `DslValue`'s own order-preserving codec directly. Baseline was genuinely red (22+24 errors — an incomplete prior attempt, missing `semio-framework` dep, stale mutation-variant references from an SMO fan-out) and was fixed as part of reaching green. Re-verified: `cargo check` clean, `cargo nextest run` → **78/79**, exact match; cited commit's real date independently confirmed (`880c37b4be`, 2026-08-13 01:03:02, predates this migration). Report: `📓️wave4-reports/reasoning-report.md`.

**`norm` round 2: complete, independently re-verified — the pushback was correct.** Composed `table` into `en1990.q_k` (all 5 existing granular mutation triads kept their exact public payload shapes, only diff/inverse internals rewired through a working-scene cache, mirroring `mathematical`) and `din18599.climate` (12×2 monthly table). Found and fixed a real bug along the way: this workspace's `serde_json` doesn't round-trip every `f64` losslessly, discovered while wiring `set-snapshot` through it — fixed by routing through the artifact's own hand-rolled `ArtifactDsl` text codec instead (which does round-trip correctly), with a regression-guard test added. Re-verified: `cargo check` clean, `cargo nextest run` → **1105/1108**, same 3 pre-existing failures as round 1, zero regressions. Report appended (not overwritten) to `📓️wave4-reports/norm-report.md` (257→494 lines).

**`norm` final state: honest partial.** LocalizedText dedup done across all consumers; composition proven real and working on 2 of 15 artifacts; 13 remain undone, explicitly not extrapolated to "done." `ucas-status: partial` is the correct and honest final call — this is expected and sanctioned for an ~80k-line, 15-artifact plugin, same class of outcome as `fem`.

**Batch 2b: fully done** (reasoning complete, norm honest partial with real composition proven). Proceeding to batch C.

**`shooting`: complete, independently re-verified — and the table-composition decline is sound, unlike norm's earlier flawed version.** Investigation found zero video/audio content (it's a 3D icon/product-render studio, not a recorder) — only `ShootingSceneLighting.emblem_base64` genuinely duplicated `image`. Composed that (`Option<Option<ArtifactChild<SemioImageSnapshot>>>`, real converter, opaque-bytes boundary matching prior behavior). Declined `table` for `assets`/`shots`/`savedCameras` — spot-checked myself: unlike norm's `q_k: Vec<(String, f64)>` (a clean 2-column fit that WAS composable), `ShootingSavedCamera` nests a whole `#[dsl(block)] camera: ShootingCamera` sub-struct and `ShootingAsset`/`ShootingShot` carry heterogeneous array/optional fields (`origin: [f64;3]`, `orientation: Option<[f64;4]>`, foreign-key refs) — a genuinely poor flat-table fit, not the same mistake. Re-verified (transient stdio churn from concurrent W1 work settled on retry): `cargo check` clean, `cargo nextest run` → **104/104**. Report: `📓️wave4-reports/shooting-report.md`.

**`layout`: agent drifted into the exact background-wait-loop anti-pattern this ticket forbids** (427k tokens spent, ended the turn "waiting for a background nextest run to finish" instead of running it in the foreground). Intervened directly via `SendMessage` telling it to abandon the background job and verify synchronously — same failure mode that burned 500-600k tokens per incident earlier in this ticket, caught before it repeated.

**`raster`: migration itself looks sound (composed `image` per-asset children, correctly found no `drawing` field to link, working-scene + real converter, matches the recipe), but independent re-verification is currently blocked by live external churn — confirmed, not assumed.** `cargo check -p semio-s-plugin-raster` fails on `ArtifactView::new(&projection, &HistoryView::empty())` — a temporary-lifetime error at a call site raster's own migration didn't touch. Checked `git status` on that exact file: it shows `M`, currently mid-edit by another session (the same `HistoryView`/`ArtifactView` signature churn the shooting/raster agents both independently flagged as W1-adjacent framework work in progress tonight). Retried after a pause, still churning. Not treating this as raster's bug; will re-verify once it settles rather than chase a live target further right now.

**`trinity` (batch Ca): blocked-mechanism, correctly.** Re-checked clear before dispatch but DKM's math→geometry/graph rename turned out still mid-sweep there (some call sites converted, some not, in the same file — a stopped-not-finished rename, not just uncommitted work; mtimes ~1h newer than HEAD, no auto-commit sweep since). Agent stopped cleanly without touching anything, exactly per `📌️important.md`'s instruction for this scenario. Left a provisional note on the likely 2-graph-child split (manifest/schema graph vs. instance graph) for whoever resumes. Deferring again — will re-check before any future batch.

## Roster-freeze ruling (2026-08-13) — DKM's math-dissolution candidates, all kept plugin-owned

DKM asked whether their `🧮️math` module dissolution (WFC/Assembly, CAS/Equation-Function, statistics, LLM sampler, fuzzy, number theory — ~72k LOC, their user mandated zero deletion) needed new `🧿️semio` v1 subsets or should stay plugin-owned artifacts, since our roster is frozen at 18+any.

**Ruling: all stay plugin-owned (Option B).** The freeze is evidence-gated (clears the same "≥2 independent plugin consumers" bar the original 18 cleared), not an absolute lock — none of DKM's candidates currently have a second consumer in our own plugin migration map. Equation/Function → `➗️mathematical` (obvious single owner). Statistics-flavored content → pushed further than DKM's own framing: it overlaps `✳️table`/`✳️value` directly and reads as **inference logic over existing generic content**, not a new persisted shape — whoever owns it should compose `table`+`value`, not mint a subset. Sampler (LLM token-sampling, "misfiled under math") → not generic content at all, plugin-owned. Fuzzy/number-theory → too small, fold as inference helpers. Assembly (WFC/Slots/Modules/Rules) was the one real judgment call — no second consumer today either, but told DKM to build it **composing `kit` (existing generic types/designs subset) + `value`** for its actual content rather than a closed local type, so it gets real genericity without a 19th subset. Also clarified for them: `ArtifactLink` (reference) has no stdio-membership restriction — only `ArtifactChild` (owned composition) requires a stdio snapshot type — so none of their plugin-owned artifacts lose cross-plugin referenceability by staying out of the roster. Left the door open: if a genuine second Assembly consumer surfaces, they'll come back and I'll reconsider.
