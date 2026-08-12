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

## ⚠️ Session-limit event — four agents lost, ZERO edits landed (verified)

All four dispatched agents (W1, W2, W3c-design, W3a-recon) died simultaneously on an account session usage limit, two of them at their baseline step. **Verified rather than assumed** that nothing partial landed: `⚙️engine` mtime Aug 11 00:50 and `🖥️platform` mtime Aug 10 20:25 (both predating this session), `grep -c "EngineRep\|DraftEngineSession"` → 0, `grep -c "fn set_"` on platform → 4 (unchanged), ticket folder containing only the coordinator's own files. The bounded-rounds structure worked exactly as designed — agents died before any write. **The coordinator then executed W1 and the W3c design directly.**

## Environment events (all five sessions affected)

1. **Whole-workspace manifest outage (transient, resolved).** `✏️s/🔌️plugins/🖍️draw/🔄️fsm/` briefly vanished while root `Cargo.toml:66-67` and `🖍️draw/📦️packages/🦀️rust/Cargo.toml:27` still referenced it. Cargo refused to load the workspace — **no command worked repo-wide, not even `check -p`**. DKM diagnosed and reported it; APA confirmed their draw agent was relocating `🔄️fsm`, discovered it is a separate crate with two workspace-member entries, and reverted. `🔄️fsm` stays. **Rule now adopted repo-wide: a directory containing a `Cargo.toml` is inventory-only, never moved — a dangling `#[path]` mount is a local compile error, a dangling workspace member is a global one that stops cargo before it builds anything.**
2. **Disk full, machine-wide.** `/System/Volumes/Data` hit 100% (1.2 GiB free of 926 GiB). Root `target/` = **428 G**, mtime 17.5 h old, **zero files modified in the preceding 2 hours** — i.e. genuinely stale, since repo policy is a per-ticket `CARGO_TARGET_DIR`. Escalated to the user rather than deleted; being handled externally. APA's user separately chose to delete the ~17 G of per-ticket `🎯️target*` dirs (DKM's 333 M included, agreed — no build in flight).
3. ⚠️ **Consequence that invalidates evidence: all cargo results repo-wide in this window are untrustworthy.** SMO supplied the causal chain — disk at 100% → `rustc` fails to write `rmeta` → surfaces as plausible-but-bogus "cannot find crate" errors. This retroactively explains several phantom failures across sessions, including SMO's own retracted 144-error `tempfile` report (`tempfile = "3.20.0"` is present at `💻️os/📦️packages/🦀️rust/Cargo.toml:61`). **Standing rule: a cargo result is evidence only if you know the run reached the target.**
4. **`semio-framework-plugin` is red repo-wide** (E0499 `self.children`; E0560/E0609 from the `document`→`artifact` rename reaching definitions but not two `#[cfg(test)]` call sites). Two peers initially misattributed it as orphaned debt from a closed ticket; DKM's mtime measurement (`🛂️manifest` Aug 12 03:50 vs `🔌️plugin` Aug 12 17:33) showed it is one session's rename mid-propagation. Both retracted. **Rule adopted: check mtime before declaring anything unowned — "nobody owns this" is a much stronger claim than "I can't tell who owns it", and usually only the second is true.** Blocks the brep/mesh/2d lanes, did not block W1.

## W1 — Mechanism: DONE (green when measured, pending re-confirmation)

Landed by the coordinator in `💻️os/🔨️modules/⚙️engine/🦀️component.rs`: `//#region 🔖️EngineRep` (tier-(d) marker trait; `build(&P)` is the **only** constructor — no seeded/incremental variant, so a representation cannot be grown from a previous one) and `//#region 🔖️DraftEngineSession` (holds one `Rep` keyed by a `DraftBaseHash` newtype; no `&mut R` accessor, no insert/replace, no `From<R>`, no user-supplied field anywhere on the struct — APA's invariant holds **by construction**, not by review). `EngineCache` given its narrowed-contract docstring, behaviour untouched.

Verification at ~17:35, before the disk filled: `cargo check -p semio-framework-os-kernel` → **0 errors / 49 warnings, unchanged from baseline**; `cargo test -p semio-framework-os-kernel --lib engine` → **11 passed, 0 failed** (818 filtered out — the other tests were NOT run and are not claimed). Re-run both once the disk is clear. Full detail: `📓️wave1-mechanism-report.md`.

## W3c design — DONE, both documents delivered to SMO

`📓️wave3c-design/flow-target-shape.md` and `📓️wave3c-design/space-target-shape.md`. Authored by the coordinator after the design agent was lost.

**Flow — a correction to SMO's own premise.** They rejected `update-widget` because "the `Patch` arm is an option-bag". Measured, `TPatch` is bound to the **full record type** (`CollectionMutation<String, Widget, Widget>`), so it is a whole-record swap, not an option-bag. SMO accepted the correction and asked that the replacement reasoning be the one recorded: *a whole-record swap is wrong not because it is an option-bag but because it is whole-document replace one level down — it records what the record became, never what the user did.* `Widget` is a **9-variant enum**, so the honest size is ~30 per-variant × per-field mutations, not a flat set. `SetFixture` and `SetLayout` die with no replacement.

**SMO rulings (binding):** `connect`/`disconnect` for synapses + `create`/`delete` for widgets ✅ · `change-neuron-preview` over `toggle` ⚠️ (`toggle` is value-blind — under concurrent merge two toggles converge to the original state rather than either user's intent; `change` records intent) · `edit-note-text` ✅ · **no bridge** — `from_framework_mutation`/`to_framework_mutation` disappears entirely, which removes the last floor under SMO's plugin-scope exit.

**`camera` doctrine violation found and resolved better than DKM proposed.** `FlowFixture.camera` is a persisted snapshot field while `FlowMutation`'s doc comment three lines above says the camera is ephemeral view state. DKM proposed deleting it; **SMO's resolution — route it to APA's draft lane** — keeps the capability and puts ephemeral local-only state where it belongs. Needs UCAS (snapshot shape) + APA (draft lane).

**Space — the 70-hit count overstates the job.** The vocabulary is already close to conformant. Three verb changes (`SetName`→`RenameCollection`, `Add*`/`Remove*`→`Create*`/`Delete*`, and `Move*`→`ChangeFolderParent`/`ChangeEntryFolder` **pending SMO's ruling** — tree re-parenting is neither `move` (spatial) nor `reorder` (ordered list), so by the field test it is `change`). The real defect is `CollectionDiff` storing **whole post-mutation records**; both claimed derive-engine gaps were verified real (no nested-`Option`, no "record + position" composite). It violates gate 3 — mergeability, not replayability, is the property the rule protects. **DKM position: handcraft the diff, do not extend the derive engine** (blast radius of every facet in the repo for the benefit of one; the file already handcrafts its `OpText`/`OpBinary` for the same reason).

## W1 — RE-CONFIRMED after the disk cleared

Disk resolved (441 GiB free; root `target/` 428 G → 407 M). W1 re-verified on a **cold rebuild**, so the earlier result was genuine and not an artefact of the degraded window:

- `cargo test -p semio-framework-os-kernel --lib engine` → **11 passed, 0 failed** (5m21s cold).
- Full lib suite baseline: **828 passed, 1 failed**. The single failure is `os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures` — an `os_dsl` fixture sweep over *plugin grammars*, unrelated to `os_engine`; UCAS recorded the same `fixture_sweep` family as concurrent churn from another session's in-flight plugin work. **Recorded as the baseline failure**, not attributed to DKM, and not fixed by us.

## ⚠️ W2 exemplar CHANGED — `🖥️platform` is dead scaffolding, not a dissolution target

Measured before authoring, and the result cancels the planned exemplar:

- **The Rust `Platform` struct has ZERO consumers.** `set_active_app_id`, `set_panel_visibility`, `Platform::new`, `PlatformSpec` have **no call sites anywhere** in `🧰️framework`, `✏️s`, or `🌎️hub`. The only reference is a re-export at `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:75`.
- **`chrome_generation`/`notify_chrome` have zero readers outside the module** — confirming the doctrine call that dirty-flag counters are dead weight, but also that nothing would notice their removal.
- **The TS file `🖥️platform/🟦️component.ts` is NOT a twin.** It contains UI layout/element-id helpers (`windowElementId`, `createWindowLayout`, `canvasPickTargetKey`, …) — unrelated content, one setter, no importers found. So there is no live TypeScript counterpart keeping the Rust shape honest.

**Consequence: dissolving `Platform` would validate nothing.** An exemplar exists to prove a recipe against real consumers; a dissolution of code nobody calls cannot fail, and would encode a facet shape no call site tests. Under the greenfield rule (delete rather than adapt) `Platform` should be **deleted as dead scaffolding** — deferred to W6's dead-code sweep, where it belongs, rather than done here as a side quest. Its re-export at `📦️glue.rs:75` goes with it.

### Replacement exemplar, chosen on measured evidence

| Candidate | LOC | External refs | Verdict |
|---|---|---|---|
| `🗺️surface/🏔️terrain` (`TerrainSessionCore`) | 569 | 9 (gis plugin, `♾️infinite` ×2) | ✅ **CHOSEN** |
| `🗺️surface/🕸️node-graph` | 1144 | 2 | too few consumers to prove much |
| `🗺️surface/🎨️paint` | 1837 | 4 | good but 3× the size |
| `◻2d/🗄️store` (`DrawingStore`/`DrawingEngine`) | 1177 | 8 | the architecturally important target, but **blocked** on UCAS's `✳️drawing` stdio handoff |
| `🧊️3d/🥽️mesh` | 2769 | **1117** | `Vec3`/`VertexId` are everywhere — far too large a blast radius for a first exemplar |

`🏔️terrain` is the smallest module that has *real* consumers, is unblocked (no stdio dependency), and is representative of the whole W3b surface family — `paint`, `node-graph` and `tiled-map` share the same "session object with mutable state + setters" shape, so a recipe proved here transfers directly to the other three.

## Environment recovered (2026-08-12, later)

- **Disk resolved by user decision** — root `target/` (428 G) deleted. 441 GiB free, 49% used. ⚠️ **Everything is a cold rebuild now**: a scoped check can run 5–10 minutes. **A slow build is not a hung build** — do not kill it, do not read slowness as breakage.
- **`semio-framework-plugin` reported GREEN** by the inference-family session (UCAS finished propagating the `document`→`artifact` rename; 0 errors, 37 warnings). **DKM has not independently confirmed this yet** — deliberately not run while the W2 agent holds the build lock, since concurrent cargo serialises on the flock and would starve it. To be verified before any brep/mesh/2d lane is dispatched, per the standing rule that a peer's build claim is not evidence until re-run.
- ⚠️ **Every cargo result recorded between roughly 17:40 and the disk fix is void, in BOTH directions.** Reds were upstream (`semio-framework-plugin`) or `No space left on device`, not local regressions; greens may have been builds that never reached their target. Anything gated, baselined or released in that window must be re-run. DKM's own W1 green was re-confirmed on a cold rebuild after the fix (11/11) and is therefore sound.

## W3a recon — DONE, and it INVERTS the fan-out plan

Full report: `📓️wave3a-brep-recon.md` (391 lines). Headline numbers: **42 component files, 23,840 LOC** — the approved plan's estimate of ~30 files / 17,439 LOC undercounted by ~37%. 263 tests. Benchmark at `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs` (313 LOC, 9 groups covering primitives, curves, sweeps, booleans, features, measures, intersections, tessellation, patterns).

### ⚠️ The finding that changes the plan: brep cannot be fanned out first

The approved plan assumed 6–8 parallel dependency-cluster lanes from the start. The recon shows that is **not possible in the current shape**:

- **`📐️brep/🧰️kernel` is a hub with 191 `&mut self` methods and 26 internal dependencies.** Every operation mutates shared `kernel.body`/`kernel.live`/`kernel.counter`.
- **`📐️brep/🏟️arena` is the storage backbone**, depended on by ~20 modules.
- **`BrepEngineHost` owns `cache: Mutex<EngineCache>` + `kernel: Mutex<Brep>`** — a process singleton.

So lanes 4 (boolean/features), 5 (queries/intersection) and 6 (construction/export) all write through the same arena. Parallelising them today would mean N agents mutating one shared topology store — the exact defect the ticket exists to remove, reproduced in the workforce.

**Consequence: the dissolution and the parallelisation are the same problem, in that order.** The kernel must be event-sourced FIRST (its 191 `&mut self` methods replaced by mutations whose diffs are computed against `base`), and only then do the dependent lanes become independent. Revised W3a shape:

| Phase | Lanes | LOC | Character |
|---|---|---|---|
| **W3a-0 (serial, single writer)** | `🧰️kernel` + `🏟️arena` | 1,146 | the dissolution proper — everything else gates on it |
| **W3a-1 (parallel, after W3a-0)** | boolean/features · queries/intersection · construction/export | 2,306 / 3,421 / 6,061 | become independent once the kernel is event-sourced |
| **frozen, no work** | math foundations (2,148) · nurbs/curves (1,725) | 3,873 | pure functions, zero `&mut`, zero deps — doctrine tier (e), already conformant |

The two frozen lanes are a genuine result, not a deferral: ~3,900 LOC of the kernel is *already* pure compute and needs no dissolution at all. That is 16% of the subsystem removed from scope by measurement.

### 🎁 `📐️brep/📜️history` is provenance, NOT an event log — and it is the right foundation

The recon proposed "use `OpRecorder` as the event log; replay to rebuild arena state". **That reading is wrong, and the reality is better.** Read directly (`📐️brep/📜️history/🦀️component.rs`, 157 LOC):

- **`PersistentLabel(u64)`** — a stable identity per topological entity, issued from a per-`Body` monotonic counter at birth, **never reused**, surviving arena compaction. Its own docstring says it is *"the identity the document layer's persistent naming keys off of."*
- **`OpDelta { generated, modified, deleted }`** — what one mutating Euler operation touched, expressed in labels rather than arena ids.
- **`OpRecorder`** — accumulates an `OpDelta` as a checked editor runs; every `📐️brep/🔺️euler` operator is passed one *"so no operation can forget to log what it touched."*

So it records **what changed**, not **what was asked for** — in CQRS terms it is shaped like a *diff*, not like a command log. Replay is not what it is for, and W3a-0 must not be designed around replaying it.

**Why this materially de-risks W3a-0:**

1. ~~**The addressing problem is already solved.**~~ ⚠️ **CORRECTED by the W3a-0 design — I overstated this.** The *type* exists and is right: arena ids are **generational and reused after removal**, so they would be a correctness bug as mutation addresses that only surfaces after a delete, while `PersistentLabel` is the non-reused stable id the taxonomy requires. But the design agent measured what I asserted: **`OpRecorder`/`OpDelta` never escape a function boundary.** Every top-level constructor (`make_box`, `boolean_solid`, `extrude_face` — ~14 functions across 8 files) creates a recorder locally and **discards it**. The provenance is correctly threaded *within* a call and thrown away at the end of it. So this is real, scoped Phase-1 work, not a finished mechanism I can build on. My "already threaded through every Euler operator" was true and irrelevant — threading is not the same as surfacing.
   ⚠️ **A second correction from the same pass**: the `EngineRep` target is **`Body` (`📐️brep/🕸️topology`)**, not `Store<T,Id>` (`📐️brep/🏟️arena`). The recon named the wrong type and I repeated it.
   ⚠️ **New gap, flagged to SMO**: `Loop`/`Coedge` carry **no `PersistentLabel`**, which conflicts with the `create-loop`/`delete-loop` verbs SMO already approved. Those two verbs have no stable address to key on as things stand.
2. **`OpDelta` maps almost 1:1 onto the diff shape** — `generated`/`modified`/`deleted` against the `create-*`/`change-*`/`delete-*` verb families SMO approved.
3. **The gap is exactly the command side**: the mutations themselves and their inverses. The kernel already knows what it touched; it does not yet record what the user asked for, nor how to undo it.

**The imperative anchor to dissolve is named in that same docstring**: *"**Host authority:** `LabelSource` lives only inside a `Body` owned by engine compute or cache."* That host-owned `Body` — reached via `BrepEngineHost { cache: Mutex<EngineCache>, kernel: Mutex<Brep> }` — is the state that must move into an `ArtifactStore` envelope, with `LabelSource` becoming part of the snapshot so label issuance is deterministic and replica-convergent rather than host-local.

### ⚠️ `Vec3` — the same trap as the collection type, caught again

The earlier "~1117 external references" figure is **not one type**. There are **three separate `Vec3` types**: a math `f32` one, a brep `f64` one, and an engine `[f64;3]` alias. Total ~1211 refs (974 `Vec3`, 161 `FaceId`, 110 `VertexId`). **Current blast radius is LOW** — the brep types are not yet re-exported as public API through `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`, so they can be touched. Risk rises to MEDIUM only once lanes exchange geometry and must agree on a representation.

This is the third instance today of one identifier naming several unrelated types (`CollectionMutation` ×2, `🌿️vcs`/`🪐️space`/`🧊️3d` path glyphs, now `Vec3` ×3). The grep-is-a-search-not-a-census rule in `📌️important.md` is earning its place.

## W1 — Mechanism: DISPATCHED (superseded by the section above)

Sonnet agent, single writer of `💻️os/🔨️modules/⚙️engine/🦀️component.rs`. Deliverables: `//#region 🔖️EngineRep` (doctrine tier-(d) marker trait), `//#region 🔖️DraftEngineSession` (shaped to the invariant above, matching `💡️inference`'s `InferenceSession`/`InferenceCache` idiom rather than inventing a parallel one), `EngineCache` scope-narrowing note + an exhaustive construction-site census classified `wasm-boundary` vs `kernel-cache` (that table seeds W6's policy allowlist). Report: `📓️wave1-mechanism-report.md`.

## W2 — Exemplar (platform): DISPATCHED

Sonnet agent, single writer of `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` (245 lines, 4 setters). Chosen smallest-first deliberately: its report becomes `📓️migration-recipe.md` for six-to-eight later lanes. Three-way split required: authoritative UI state (`active_app_id`, `uri`, `panel_visibility`) → artifact snapshot + triads; runtime wiring (`ActionBus`, the `apps` registry — APA's declarative-registration territory) → left alone; **dirty-flag counters (`generation`, `chrome_generation`) → deleted, not migrated**, since the edit log already provides change notification (agent must grep every reader first and report consumers). Verbs fixed by SMO. Open question the agent must answer for everyone who follows: **where does a framework-module-owned artifact schema live**, given framework modules have no `🗿️artifacts/` tree — precedents to study are `🪐️space` and `♾️infinite`; the shape to imitate is stdio's `✳️text` facet (read-only, another session's). Instructed to report the placement question rather than create directories in a contested area.

## Remaining

W3a brep/mesh/2d fan-out (gated on W1 + W2 recipe + UCAS stdio handoff + SMO slug sign-off) · W3b surface (4 lanes, gated on W2) · W3c flow/space/db/infinite (gated on W1; flow+space gated additionally on sending SMO the target enum shape; db gated on the pre-existing `semio-framework-os-kernel-db` breakage, ~53 errors, `task_9a4155cc`, which predates DKM) · W5 serializer · W6 ratchet at queue position 5 · W7 adversarial verify + close.

## Cross-session service: measured the `📚️examples` relocation fallout (not DKM's to fix)

Two peers disagreed on the fallout from the `📚️examples` dir relocation (`🗿️artifacts/<a>/📚️examples/` → `🗿️artifacts/<a>/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`): one reported 32 files / 24 plugins / one wrong-depth pattern with a mechanical 7→3 fix; the other reported 62 unresolved across several patterns and refused to substitute. DKM resolved it from the filesystem, no compiler, while the machine was too loaded for anyone to build.

**Method**: resolve every `include_str!` whose path contains `📚️examples` under `✏️s/` against its real on-disk target. 197 candidates → **11 unresolved, 5 plugins**. Two structurally different classes:

- **Class A — 4-up where 3 is correct** (depth rewrite genuinely fixes; verified): `🕸️dag`, `🪐️space/🏠️home`, `🪵️sourcing/🗂️curate`, `🔋️energy/🔋️model` — all in `📸️snapshot/📝️text/`.
- **Class B — cross-artifact / cross-plugin references, which NO depth rewrite fixes** (7 files): `🔱️trinity/🎛️apps/{🔌️jack:40,41 · ♻️rewrite:52 · ♻️rewrite/🌍️world:759,969}` reach into `🗿️artifacts/🔌️jack/📚️examples/…`; `🪐️space/🦀️component.rs:30,31` reach into **two other plugins'** example dirs (`../🖍️draw/…`, `../✒️writer/…`). Their targets moved *structurally*, not just shallower.

**Outcome**: confirmed one peer's finding from disk (saving them a cold rebuild started to answer it), and confirmed the other's retraction was correct — a blanket substitution would have silently corrupted all seven Class B files. **DKM holds none of the 11 and fixed none of them.**

⚠️ **The inventory is moving**: 32 → 11 between the two measurements, because a session is actively repairing them, and `🏗️fem` (the plugin the 7→3 recipe was generalised from) is **already fixed**. Anyone acting on a list more than a few minutes old is acting on stale data — including this one.

### Rules earned here, added to `📌️important.md`'s family

- **An auto-commit flag carries no attribution signal at all.** We knew `git status` is useless in this tree because it auto-commits; the corollary nobody had drawn is that a commit's *contents* are equally useless for attribution — a timer-based commit bundles every session's in-flight work, so co-location in a flag means only "before 18:08". Attribution needs **content evidence** (here, `R100` pure-rename records) not co-location.
- **A recipe generalised from two samples is a hypothesis.** The 7→3 fix was correct for both files it was derived from and wrong for the majority of the real population.

## W2 exemplar — DONE, PASS (214/214), with a plan-changing negative result

Full detail: `📓️wave2-reports/terrain-report.md`.

**The finding is negative and that is its value.** `TerrainSessionCore` owns **no tier-(a) authoritative state**, so no mutation vocabulary applies. Every field was traced to its real owner rather than inferred from the struct's shape: `origin_lon`/`origin_lat`/`exaggeration` are *mirrors* of gis-plugin state, and `GisTerrainSnapshot.exaggeration` **already has a shipped, law-tested `change-exaggeration` mutation on the correct owner**; `elevation` holds network-fetched DEM PNG bytes, not derivable from any snapshot, so it fails `EngineRep::build(&P)`'s contract by construction. Terrain is a rendering-support cache, not a hidden document. The agent authored **no vocabulary** rather than inventing some — the correct outcome.

⚠️ **Consequence for the plan: "session object + setters" is NOT a dissolution signal, and setter count — the heuristic this exemplar was chosen with — is the wrong instrument.** W3b is likely much smaller than scoped. Each surface lane must trace fields to owners before assuming there is anything to dissolve. This does NOT generalise to "no work in W3b": `📓️wave3b-surface-2d-recon.md` (as corrected) found real tier-(a) state in `MapHost.features` and real consumers for all three siblings.

**Verification**: `cargo test -p semio-framework-surface --lib` → **214 passed, 0 failed, 1 ignored**; terrain subset **20 passed, 0 failed** including 2 new determinism law tests.

### Two PRE-EXISTING defects fixed to reach the gate — separate from DKM's dissolution diff

Both in `🗺️tiled-map/🦀️component.rs`, inside DKM's claimed boundary, so ours to fix (unlike the peer-owned files we declined all day). Attribution measured: tiled-map mtime **Aug 10 20:13**, only `🏔️terrain` dirty, all 6 compile errors in tiled-map and zero in terrain.

1. **6 stale field accesses (E0609)** from an Aug-10 `MapHost` refactor that nested `positions` under `features` and `tile_images`/`vector_tiles` under a private `tiles`. **The crate's test target had not compiled for two days** — invisible to `cargo check`, so the refactor landed green.
2. **12 stale RUNTIME fixture paths.** Ticket date dirs gained emoji prefixes (`🎫️tickets/26/06/03/…` → `🎫️tickets/🎆️26/🌙️06/☀️03/…`); 12 `std::fs::read` literals kept the bare form. The fixtures existed all along. 10 tests now pass.

### 🔑 Generalisable: a gap in every path audit run today

Defect 2 is the **same class** as the `📚️examples` relocation fallout four sessions spent the afternoon on — but it lives in a **runtime `std::fs::read`, not a compile-time `include_str!`**. It was therefore invisible to *both* instruments in use: `cargo check` never compiles `#[cfg(test)]`, and the repo-wide include-target audits (4343 targets, 0 unresolved) only scanned compile-time macros. **Anyone auditing path staleness should scan `std::fs::read`/`File::open` literals too.** DKM found it only because the crate was in our boundary and we ran the tests rather than the checker.

### Coordinator errors the agent caught (recorded, not quietly fixed)

- **Wrong crate in the dispatch brief**: `🗺️surface` does not mount into `semio-framework`; it is `semio-framework-surface`. The agent proved the negative (`cargo test -p semio-framework --lib terrain` → 0 tests matched) rather than assuming.
- **Miscounted setters**: brief said 5, truth is 2 (+2 wasm wrappers); the fifth match was the *test function* `set_exaggeration_clamps_negative_to_zero`. That is the coordinator committing the "a grep is a search, not a census" error **in the brief that cites it**.

## W3a-0 — design dispatched; one phantom gate removed

**Baseline measured**: `cargo test -p semio-framework-3d --lib` → **407 passed, 0 failed**.

⚠️ **`semio-framework-3d` does NOT depend on `semio-framework-plugin`** (verified in its `Cargo.toml`). The plan recorded brep as gated on that crate going green; **that gate never existed**. Brep work is unblocked by the plugin SDK entirely.

A **design pass precedes implementation** for this wave, because one architectural question is load-bearing and currently unanswered: the framework cannot depend on a plugin, stdio owns `SemioBrepSnapshot`, stdio is another session's and unfrozen, and the W2 exemplar found **zero framework-module precedent** for schema registration (all 106 real `register_artifact_schema_descriptor` call sites are under `✏️s/🔌️plugins/**`). Guessing it wrong would put ~30 triads in the wrong crate. The previous wave burned 356k tokens partly by building before the architecture was settled.

## W3a-0 design — DONE. Blocking question resolved; scope shrank substantially.

Full design: `📓️wave3a-design/brep-dissolution-design.md`.

### The load-bearing question, answered

**The authoritative brep snapshot stays `SemioBrepSnapshot` in stdio's `✳️brep` subset** (already real, registered, codec'd — untouched). The mutation triads belong there too, plugin-owned, still gated on the UCAS handoff (2 of 3 gates open). **`semio-framework-3d` becomes pure compute**, with dependency direction **stdio → framework-3d**.

Crucially that direction is the **established** pattern, not a novel shape: `cad`, `process`, `procedural`, `demonstrator` and `lowpoly` already depend on `semio-framework-3d`. stdio doesn't yet, but adding the edge is one `Cargo.toml` line. Framework-3d gains exactly **one** new tier-(d) type — an ephemeral seed struct (never persisted, explicitly *not* a second snapshot) serving as the `P` in a new `impl EngineRep<Seed> for Body`.

### 🎁 The finding that shrinks the wave

**The 191 `&mut self` kernel methods are overwhelmingly two layers of thin indirection** — a `GeometryHandle`-registry wrapper plus an async-trait wrapper — around functions that **already take `&mut Body`/`&Body` and are already pure**. Every real geometry algorithm (boolean, sweep, blend, offset, measure, classify, tessellate, validate) is *already* in the shape the doctrine wants.

**So the bulk of this lane is deletion, not rewriting**, and it needs far fewer triads than the ~30 previously quoted to SMO. The imperative surface is a wrapper, not the substance.

### Corrections this design forced (recorded, including to my own claims)

1. ⚠️ **The `EngineRep` target is `Body` (`📐️brep/🕸️topology`), not `Store<T,Id>` (`📐️brep/🏟️arena`).** The recon named the wrong type; I repeated it.
2. ⚠️ **My "the addressing problem is already solved" was wrong** — see the corrected entry above. Provenance is threaded within a call and discarded at its boundary; ~14 top-level constructors create an `OpRecorder` locally and throw it away. Real Phase-1 work, not a finished mechanism.
3. ⚠️ **New verb gap, flagged to SMO, unresolved**: `Loop`/`Coedge` have **no `PersistentLabel`**, so the `create-loop`/`delete-loop` verbs SMO approved have no valid stable address (arena ids are generational/reused — a delete-triggered correctness bug). Three options put to them; DKM's weak prior is dropping those two verbs and treating loops as structure implied by face/edge mutations. **Not decided unilaterally.**
4. ⚠️ **New cross-session finding: `BrepEngineHost` has two live consumers outside `🧊️3d`** that the recon missed — a plugin-owned struct field in `process3d`, and a **process-global `OnceLock` in `cad`** (the purest instance of the anti-pattern this ticket exists to remove). Its deletion is therefore **not framework-3d's to do alone**; it needs cross-session migration and reaches into APA's `SolidExporter`/`SolidImporter` territory via `💻️os/🖥️host`. Flagged to APA; DKM touches none of it.

### Six-phase plan

**Phases 1–3 are fully executable inside `semio-framework-3d`'s own boundary** and depend on no peer: surface the provenance past function boundaries, add `impl EngineRep<Seed> for Body` with a snapshot→body→snapshot round-trip law, and strip the two indirection layers. **Phases 4–6 are explicitly gated** on the stdio handoff (triad authoring) or the cross-session `BrepEngineHost` migration. Each phase is independently verifiable against the **407 passed / 0 failed** baseline, with `benches/kernel.rs` (9 groups) checked on the phases touching boolean/sweep/tessellate.

## W3a-0 Phases 1–2 LANDED and independently verified; Phase 3 correctly self-blocked

Report: `📓️wave3a-reports/phases-1-3-report.md`. **Coordinator re-ran the claimed gate rather than accepting it** (standing rule: a scout re-runs whatever an executor claims):

```
cargo test -p semio-framework-3d --lib   →   413 passed; 0 failed
```
Baseline was **407/0**; the wave added 6 executed law tests and regressed nothing. **Boundary discipline verified**: 16 changed files, all under `🧰️framework/🔨️modules/🧊️3d/📐️brep/`. Other diffs in the tree are peers' plugin migrations, not ours.

**Phase 1 — provenance now escapes the call. DONE.** `rec: &mut OpRecorder` threaded through 12 primitives constructors, boolean (4 public + 4 private), sweep (6+3), blend (3+1), offset (6+4), sew (1), heal (3), plus ~30 call sites in the kernel and test sites across 6 files. Two functions — `heal_solid`, `convert_to_nurbs` — **previously bypassed euler's provenance entirely** and now genuinely record what they touch. A new law test proves `make_box`'s whole op-delta escapes the call.
The agent improved on the design's literal file list: functions *transitively* calling those also needed threading, or the delta would still be swallowed one frame up. That is the correct reading of the intent, not scope creep.

**Phase 2 — `impl EngineRep<BrepArenaSeed> for Body`. DONE.** Seed built natively on `PersistentLabel` rather than `String` (refining the design's own sketch per its §2 guidance) — which matters, because it makes the ephemeral representation key off the same never-reused identity the mutation layer will address by. `build()`/`to_seed()` in `🕸️topology`, with 4 executed round-trip/determinism laws against a real box constructed through the checked euler editors.

**Phase 3 — BLOCKED in its full form, and this is the right outcome.** The agent verified by inspection that all ~92 `BrepKernel` async trait methods delegate 1:1 to `SyncApi`, and that `BrepEngineHost` — explicitly out of scope, with live consumers in `process3d` and `cad` — depends on that surface remaining intact. **Deleting the Registry/SyncApi layer now would break code outside this crate's boundary.** It therefore deleted only the two `SyncApi` methods with zero callers anywhere (`retain_sync`, `tessellate_to_mesh_data_sync`) and stopped. Checking consumers before deleting, then stopping when the answer said stop, is exactly the instructed behaviour.

**Benchmarks**: no regression in the 5 of 9 groups that completed. 3 pre-existing bench-fixture bugs found, attributed, and **not fixed** — correctly reported rather than silently repaired or blamed on this diff.

### What this means for the wave

The imperative surface's *substance* is now dissolvable — provenance escapes, and the ephemeral representation exists with proven round-trip laws. What remains of Phase 3 is **gated on the cross-session `BrepEngineHost` migration APA owns**, not on anything DKM can do alone. That confirms the design's phasing was right: 1–3 inside the boundary, 4–6 gated.

## 🧊️ STDIO ROSTER FROZEN — the third gate opened (2026-08-12, late)

UCAS broadcast the explicit signal: **"roster frozen — `🗄️stdio` is released; start whenever you're ready."** They were deliberate that this means *the directory structure is final*, not merely that it compiles — the distinction this tree has repeatedly got wrong.

**Final `🧿️semio` v1 roster — 18 subsets + `✳️any`** (19 dirs, verified by UCAS on disk rather than from an agent report): `animation audio brep cad document drawing flow graph image kit mesh model object presentation table text value video` + `✳️any`.

**stdio baseline to diff against: `cargo nextest --profile long -p semio-s-plugin-stdio` → 2174 run, 2168 passed, 6 failed, 5 skipped.** All 6 pre-existing and attributed elsewhere (`dwg`/`ifc` `fixture_honesty_law` unowned; `html`/`json`/`pdf` `inference_default_law` + `md` outline are IIF's). **Diff against 6, never against zero, and attribute none of them to DKM.**

### ⚠️ The `object`/`value` rename bites DKM specifically

`workflow` → `flow` is a plain rename. But the old value-tree **`object` → `value`**, and **`object` now means a *spatial* thing** — transform plus owned **brep/mesh/value children**. Our design docs and SMO's approved verb roster predate that. Two consequences:
1. A mechanical search for `object` finds the **wrong directory and still compiles** — a name surviving without its meaning, the same class as the two-types-one-name traps this ticket has hit three times.
2. Spatial `object` **owns brep and mesh children**, so it sits directly downstream of the two subsets DKM is about to author. Our triads must be authored knowing they will be composed by it.

### 🔑 Binding for DKM's triad authoring: do NOT derive law tests from `din4108`

UCAS found the `din4108`-derived `round_trip()` helper diffs each inverse against the **stale pre-operation `base`** rather than the evolving state — silently discarding the forward mutation's effect. It caused 3 false failures and **reproduced independently in two more helpers copied from the same reference**.

**General form, now binding here: a bug in a reference pattern is one bug per copy, and every copy looks correct.** DKM's brep/drawing law tests are authored from scratch against `(payload, base)` semantics; any inherited helper is suspect until read. (Our existing Phase-2 laws are representation round-trips — seed → `Body` → seed — a different shape, so they don't inherit it. Checked, not assumed.)

**Grammar traps recorded for authors**: `✳️any`'s two hand-maintained `.semio` grammars need every new tag in their alternation and are **invisible to `cargo check`** (only grammar-conformance tests catch it); grouping is `{ }` never `( )`; `|` continuation unsupported.

### Gate count for `✳️brep`/`✳️drawing`/`✳️mesh` — now 3 of 3, with one caveat

| Gate | State |
|---|---|
| SMO verb approval | ✅ open — **except `create-loop`/`delete-loop`**, which have no stable address (`Loop`/`Coedge` carry no `PersistentLabel`). Ruling requested; SMO appears to have wound down |
| IIF deferral of the three inference facets | ✅ open |
| UCAS stdio write-handoff | ✅ **open** — read from "stdio is released", sent directly to DKM, with an explicit request to correct that reading immediately if wrong |

**Authoring proceeds without `create-loop`/`delete-loop`**, which stay unauthored and flagged rather than invented — the sanctioned outcome per `📌️important.md` ("leave the enum EMPTY and flag it, never invent vocabulary"), applied narrowly to two verbs rather than a whole facet.

**Peer sessions have wound down from five to one.** Remaining cross-session dependencies are therefore unlikely to resolve interactively: APA's `BrepEngineHost` migration (blocking W3a-0 Phase 3+), SMO's Loop ruling, and the `🧮️math` regeneration DKM claimed. All are recorded above with enough detail to resume without conversational context.

## The UCAS disagreement is CLOSED — and the answer belongs in the doctrine

UCAS asked the question that had been implicit since W0: *does DKM's mandate require the halfedge structure itself to be a snapshot with a diff, or only that no mutable CRUD store remains?*

**Answer: the halfedge `Body` is NOT a snapshot and must never become one.** It is doctrine tier (d) — an ephemeral working representation, like a BVH or a tessellation buffer. The authoritative geometry is `SemioBrepSnapshot`; `Body` is derived from it by `EngineRep::build(&seed)` and dropped when the building function returns. No diff, no mutations, no persistence, no identity. **Making it a snapshot would be a violation, not compliance** — two authoritative representations of the same geometry is exactly the duplicated-state failure this ticket exists to remove.

### Their carve-out was right about the types and wrong about the lifetime

UCAS's `📓️design-full-plan.md` recorded *"engine (behind traits, never serialized) = framework MeshData+HalfedgeMesh, brep topology kernel, DrawingScene"*. **Everything except the parenthetical survives.** The types live, the algorithms live, nothing is serialized.

The residue is narrow and real: **"behind traits, never serialized" permits a long-lived host-owned kernel session; `EngineRep` forbids it.** `EngineRep` is strictly stronger in three ways — `build(&P)` is the only constructor (no seeded variant, since a rep grown from a previous rep is no longer recoverable from the snapshot, which is how a cache becomes hidden state); it may not outlive the call; and it must be wholly derived, so dropping it at any instant costs only a pause.

`BrepEngineHost { cache: Mutex<EngineCache>, kernel: Mutex<Brep> }` satisfies "behind traits, never serialized" and **fails all three**. So does `📐️cad`'s `static OnceLock<BrepEngineHost>` — write-once, passing every mutability test, and still a plugin holding a handle to host-owned engine state for the process lifetime. **Not ambient mutability; ambient reach.**

So the two tickets never disagreed about what should exist, only about how long it may live. Recorded as a one-line correction rather than a live dispute.

## ✅ `✳️brep`/`✳️drawing`/`✳️mesh` handoff CONFIRMED explicitly

UCAS confirmed the grant was explicit and predated the freeze — DKM was not relying on inference. **The split, in one place:**

| Scope | Owner |
|---|---|
| `🧬️schema/🧬️mutations/**` in all three — dispatch enums, all triads | **DKM** |
| the three `💡️inference` facets | **DKM** |
| the engine dissolution behind them | **DKM** |
| composition slots (`ArtifactChild`/`ArtifactLink` *fields*) in `📸️snapshot/` for `mesh` and `drawing` | **UCAS, later** — ping before entering those files |
| `✳️brep` | a DAG leaf; **UCAS never touches it** |

### ⚠️ Correction to my own downstream-coupling warning

I recorded spatial `object` as downstream of brep/mesh and therefore constraining our triad authoring. **UCAS showed that is much weaker than I claimed.** `ArtifactChild<S>` is `{ child_id, target: ArtifactRef, PhantomData<S> }` — a two-string handle carrying **no snapshot content**, with `S` only keeping the slot compile-time typed. So `SemioBrepSnapshot`/`SemioMeshSnapshot` internals can be restructured freely and `object` never sees it.

That is the child-as-own-document decision paying off: **the parent holds two strings, not a subtree**, so a child's internal churn cannot propagate upward. The only things that would break `object` are renaming/deleting those types or changing the kind strings `s.stdio.semio.{brep,mesh}` — and either would be done in one joint change. **This supersedes the broader warning recorded above.**

## `🧮️math` generated-mirror defect — FIXED and proven (claimed late, announced, unobjected)

**The symptom was local; the defect was repo-wide.** Worth separating, because the framing changed once measured.

**Symptom**: `🤖️generated/🦀️nakagin.rs` and its TS twin still embedded `flatPosition` as `"kind":"derived"` though the JSON source no longer declared it. ⚠️ **But `🤖️generated/` is gitignored** (`.gitignore:86` — `**/🤖️generated/`), so those files are **never committed**. The stale mirror was a per-machine build artifact, not repo debt — a smaller impact than the original report implied.

**Defect**: `🧮️math/📦️packages/🦀️rust/build.rs` — tracked, committed, and shipped to every developer. Its own docstring claims it *"re-runs it whenever a manifest source changes"*. It did not. It registered `cargo:rerun-if-changed` correctly on every source (:22, :31, :32), so cargo re-ran the build script on every edit — and the script then **did nothing**, because the work was gated on `if !generated.is_file()`. Regeneration fired only when the output was *missing*, never when it was *stale*. So the intent was right and the implementation checked the wrong predicate, meaning **every generated artifact in that module silently went stale on every source change, on every machine** (~10 files: `🦀️nakagin.rs` plus TS twins for concrete-forest, draw-layers, flow-dag, note-blocks, puzzle2d/3d/5d, rewrite-lhs/rhs, wires, writer-languages…).

**Fix — both parts, because part 1 alone would silently re-break:**
1. Resynced via `bun ./📜️script.ts generate` → *"wrote 12 manifests"*; `flatPosition` count **1 → 0** in both mirrors.
2. **Replaced the existence gate with a real staleness comparison.** Added `note_newest()`, threaded a `newest: &mut Option<SystemTime>` through `watch_manifest_sources`, and made `main` compare the newest watched source against the generated file's mtime — regenerating when the output is missing **or older than any source**.

**Verification — the gate was proven to fire, not assumed:**
```
cargo check -p semio-framework-math                       →  Finished, 0 errors
generated mtime before touch:  Aug 12 21:07:45
touch 🧮️math/📦️packages/🦀️rust/📜️script.ts   (a watched source)
cargo check -p semio-framework-math                       →  Finished in 8.35s
generated mtime after:         Aug 12 21:08:44   ← regenerated
```
The old `!is_file()` gate could not have produced that transition — the file existed throughout.

**A cache whose invalidation condition is "the file is missing" has no invalidation condition.** That the module's own docstring already described the correct behaviour makes it a good example of a class worth watching for: **the doc comment was the specification, and it had been silently false for as long as the gate existed.**

## W4 — `✳️brep` DONE and independently verified; `✳️drawing` vocabulary done; `✳️mesh` dispatched

**Independently re-run by the coordinator with a forced recheck** (`touch`ing glue.rs first, because a cached `cargo check` re-emits no diagnostics and looks identical to clean — I nearly reported green off exactly that artefact):

```
cargo test -p semio-s-plugin-stdio --lib  →  2246 passed; 2 failed; 5 ignored
```
Baseline was **2168 passed / 6 failed**. So **+78 tests, and failures 6 → 2**. The 2 remaining are the pre-existing unowned `dwg`/`ifc` `fixture_honesty_law`; **zero failures attributable to DKM**. (The 4 inference failures in the old baseline were fixed by their owner in the interim.)

| subset | triads | `set-snapshot` | inference | banned vocabulary |
|---|---|---|---|---|
| `✳️brep` | **13** | gone | `💡️inferences` present | **0 files** |
| `✳️drawing` | **17** | gone | pending | **0 files** |
| `✳️mesh` | 1 | present | none | 5 files — **dispatched** |

### `✳️brep` — the finding that mattered

**The facet was not a blank slate.** It carried a fully working but non-conforming **22-variant** enum: banned vocabulary, wrong verbs (`SetEdgeEndpoints`), and **zero matching triad directories**. It was replaced wholesale with SMO's exact 13-verb table. This is worth recording because the ticket's premise — "`✳️brep` has only `set-snapshot`, i.e. no vocabulary" — was **half wrong**: there was vocabulary, it just wasn't conforming and wasn't addressable. A directory census missed it because the variants were inline.

**Cascade reasoning, which is the good part**: `delete-vertex` cascades to `delete-edge` for dependent edges (both addressable, so the cascade is invertible). `delete-edge`/`face`/`shell` deliberately do **not** cascade into `loop.edges`/`shell.faces`/`solid.shells` membership — because no modify-verb exists for those collections, so the cascade could not be inverted, and **an uninvertible cascade would violate the vocabulary rather than serve it**. Flagged in each triad's doc comment rather than silently omitted.

**Inference authored honestly**: only `validation-report` got a real `InferredField` with a genuine `DepHash` chain. `tessellation` and `mass-properties` were **omitted with stated reasoning** (they need real NURBS evaluation not honestly available at that layer) rather than faked. That is the instructed behaviour and the right call.

### 🔑 The laws caught three real bugs that the four gates would have passed

Running the round-trip/consistency/determinism laws — rather than merely authoring them — found and fixed: a Vec-order-vs-set-equality test defect; **all five `delete-*` diffs failing to return a truly empty diff for an absent target**; and a cross-subset `✳️any` test needing restructuring. Direct vindication of the rule adopted from UCAS's `✳️text` incident: **a structural audit is not a correctness audit.** All three would have shipped behind a clean gate pass.

### `✳️mesh` — provenance to preserve

SMO owned stdio mutation vocabulary and **explicitly claimed the `set-primitive-geometry` → `replace-primitive-geometry` rename as theirs**, approving the reasoning but reserving the edit. **They wound down without doing it.** DKM is completing it under the user's explicit instruction to finish end to end. The dispatched agent is required to record this provenance prominently: **SMO-approved in reasoning, SMO-reserved in execution, completed by DKM after SMO ended.** Not presented as ours.

## W4 final state — DKM's boundary is CLEAN; residual belongs to a concurrent session

Measured authoritatively (`RUSTC_WRAPPER="" … --all-targets`, forced recheck):

| subset | triads | glue mounts | banned-vocabulary files | errors |
|---|---|---|---|---|
| `✳️brep` | 13 | 13 | **0** | **0** |
| `✳️drawing` | 17 | 17 | **0** | **0** |
| `✳️mesh` | 1 | 1 | 5 | 0 — **not started** (agent died at dispatch) |

**Errors attributable to DKM: 0.** The 8 remaining stdio errors are all one kind — `cannot find 'inferences' in 'schema'` across `✳️animation`, `✳️any`, `✳️audio`, `✳️cad`, `✳️document`, `✳️flow`, `✳️graph`, `✳️image` — i.e. inference-facet mounts for **other** subsets, belonging to the concurrent inference fan-out. `📦️glue.rs` mtime was **21 seconds old** at measurement: that session is actively mid-edit. **Not touched.**

### What the coordinator repaired directly

All three agents died simultaneously on a session limit, two of them mid-edit. Repairs, all inside DKM's boundary:
- **Added brep's 13 missing triad mounts** to `📦️glue.rs` — the dispatch imported 13 leaf modules and the glue mounted only `component`/`binary`/`text`. Generated from a real `os.listdir()`, never hand-typed, with each triad's three leaves verified present before emitting its block. Errors 61 → 44.
- Drawing's 34 errors cleared without intervention (its mounts were already complete; the failures were transient mid-edit state).

### ⚠️ Two measurement lessons, both learned by getting them wrong first

1. **My verification flags were inadequate all evening.** `cargo test -p semio-s-plugin-stdio --lib` reported **2246 passed / 2 failed**. The same tree at the same moment under `RUSTC_WRAPPER="" cargo check --all-targets` reported **61 errors**. The repo sets `rustc-wrapper = "sccache"` in `.cargo/config.toml:2`, and the default target set excludes exactly where a vocabulary rename leaves casualties. **Both flags are now mandatory for every DKM verification and every dispatch brief.**
2. **A Unicode-normalization mismatch silently defeated my own tooling — twice.** A Python script matching the emoji path `✳️brep/…/📄set-snapshot` returned **no match** while `rustc` reported that exact mount as dangling at `glue.rs:5572`, and a shell `grep -c` for the same string returned **0**. The literal in my script did not byte-match the on-disk name. **Fix: discriminate on ASCII-only substrings** (`"brep" in body and "set-snapshot" in body`) and never on emoji literals. This is the same trap UCAS warned about producing silently-empty modules — it also produces silently-empty *searches*, which is worse, because an empty search reads as "already clean".

### ⚠️ "A verification is a timestamp, not a property"

Adopted verbatim from session #2553. Demonstrated three times in ten minutes on one file: the dangling `set_snapshot` block was **present, absent, present, then absent again** across successive reads — removed concurrently between my read and my write, twice. I stopped chasing it and re-measured instead. Corollary for a shared tree: **never act on a measurement you did not take in the same command as the action**, and prefer re-measuring to re-fixing.

## W6 — Policy ratchet LANDED in `📜️script.ts` (report mode)

Queue position 5 confirmed clear before writing: `📜️script.ts` was **not dirty**, mtime 4h old, four prior peer edits committed (flags 489/490/494/495/496). New region `//#region 🔧️PolicyRuleDissolvedKernels` inserted after `PolicyRuleSchemaOverhaulPC`, registered in the top-level aggregator, following the file's existing conventions (`BreachRecord`, shrink-only allowlist **with stale-entry detection**, `priority: "medium"` = reports, never gates).

### The three rules

1. **`policyDissolvedRepEscapeBreaches` — measures REACH, not mutability.** Flags a durable field *or static* holding `HalfedgeMesh`/`BrepEngineHost`/`DrawingStore`/`DrawingEngine`/`EngineCache`. The docstring states why this is a *separate* rule from any `&mut`/`static mut` check: a `static HOST: OnceLock<BrepEngineHost>` is write-once, so every mutability-based rule passes it, and it is still a plugin holding a handle to host-owned engine state for the process lifetime. (Adopted from APA, who found their own `PolicyRulePluginPurity` had exactly this hole.)
2. **`policyDissolvedEngineCacheScopeBreaches`** — `EngineCache::new`/`impl EngineHost for` outside the sanctioned wasm-boundary modules. **No seed allowlist**: the narrowed scope is the target state, so it fails on new violations rather than burning down a backlog.
3. **`policyDissolvedWholeDocumentReplaceBreaches`** — any `📄set-snapshot` triad directory. Deliberately a **directory-level** check, because the dispatch arm and the triad dir fail independently and an identifier grep sees only one of them. Its `solution` text carries the hard-won rule: delete the dir, its dispatch arm **and its `📦️glue.rs` mount in the same change**, since a mount pointing at a removed directory aborts the build for every crate in the workspace.

### Verified standalone against the real tree (8,818 Rust files)

The full `bun ./📜️script.ts policy` run **cannot complete** — but not because of this work (see below). The detection logic was therefore verified in isolation:

- **rep-escape: 8 hits**, and they are exactly the known census — `📐️cad`'s `BrepEngineHost` reach, `◻2d/🗄️store` (`DrawingStore` + `EngineCache`), `🧊️3d/📐️brep/⚙️engine/🖥️host` (`EngineCache`), `💻️os/🔨️modules/🌊️flow/🖍️drawing` (`DrawingStore`), and `🔌️plugin/🖥️host` (`EngineCache`, sanctioned). 4 are seeded in the shrink-only allowlist; 1 is in a sanctioned dir; the remainder report.
- **whole-document-replace: 51 triad dirs**, down from the **70** measured at W0 — the difference is largely the three subsets this ticket cleared.

That 51 is the honest, countable measure of how much of the mandate remains: those directories belong to other tickets, and the rule now makes them visible rather than assumed.

### ⚠️ Blocker, not ours: `bun ./📜️script.ts policy` throws before reaching any rule

```
ReferenceError: policyArtifactRootOfMutationsDir is not defined
  at policyMutationTriadCompletenessBreaches (📜️script.ts:6332)
```
**Attributed, not assumed**: that identifier appears **0 times in `HEAD`** — both its three call sites *and* its definition are another session's **uncommitted, in-flight** work. DKM's region sits ~5,000 lines below it and cannot cause a `ReferenceError` at 6332. The module parses and executes as far as 6332, which also proves DKM's region is syntactically valid. **Not fixed** — it is another session's live edit, and the standing rule is to work on something else and say so rather than repair a file mid-edit.

## Unblocking stdio — and a distinction the whole tree had been conflating

A peer asked DKM to remove a premature `💡️inferences` glue mount at `📦️glue.rs:7017` whose target files did not exist yet. **Their argument was right and I acted on it**: four lines against six blocked sessions, with a trivial re-add once the files land, and I had honestly said I could not give a reliable ETA. Removing it surfaced a second layer — several mesh triad mounts pointing at unwritten leaves — so I swept the whole mesh block and unmounted every triad whose targets did not all exist. (All 17 triads proved complete; the inference block was the only real offender.)

### 🔑 A dangling mount blocks everyone; a broken test blocks only its own crate

Measured after the removal (`RUSTC_WRAPPER=""`, forced recheck):

```
cargo check -p semio-s-plugin-stdio                → 0 errors    ← what every other crate links
cargo check -p semio-s-plugin-stdio --all-targets  → 1 error     ← stdio's OWN test target only
```

The residual is `no method named is_empty for SemioMeshDiff` at `✳️mesh/…/🧬️mutations/🦀️component.rs:417` — a **trait-in-scope problem inside a `#[cfg(test)]` assert** (`MutationDiff` is imported at :368-369; the failing assert sits outside that scope). Left alone: my agent is mid-authoring that exact file.

**Other plugin crates link stdio's lib, not its tests, so a lib-test failure does not propagate.** Spot-checked `semio-s-plugin-space`: 6 errors, **all its own** (`no field 'document' on OsAppRegistration`, `no field 'headers'/'rows' on CsvSnapshot`) — nothing stdio-derived.

So "six sessions behind one file" was **true while the mount dangled** — a dangling `#[path]` is a *lib* error and does propagate — and **stopped being true the moment it came out**. All night this tree treated those two as one category. They are not:

| shape | scope of damage | visible to |
|---|---|---|
| dangling `#[path]` / workspace member | **every crate on the machine**, aborts before compilation | plain `cargo check` |
| broken `#[cfg(test)]` code | **only the owning crate's test target** | only `--tests`/`--all-targets` |

### Taxonomy-flip bar, corrected by the peer

`policySchemaRepresentationBreaches` is allowlist-free and hard-gating, so the flip needs mesh's `💡️inferences/` **directory to exist with its leaves** — not a finished, wired, law-tested facet. A much lower bar than DKM had assumed when recommending "flip with mesh as a known breach". That directory currently does **not** exist (its mount was removed precisely because the files were missing). DKM will report when it lands, and will state plainly whether it is merely present or actually complete, since only the former gates the flip.

## Inference representation leaves — 4 gaps closed, taxonomy flip unblocked repo-wide

A peer's enumerator found the last 4 leaf gaps in the entire repo, all DKM's: `📝️text/` and `💾️binary/` missing from `✳️brep` and `✳️mesh`'s `💡️inferences` families (110 of 112 families complete). Those two representation dirs are what `🔣️taxonomy.json`'s `schemaChildDirs += 💡️inferences` flip gates on, via the **allowlist-free, hard-gating** `policySchemaRepresentationBreaches` — so four missing directories were holding a flip that six sessions were waiting on.

**Closed.** 8 + 6 leaves per family, mirrored from `✳️cad`'s complete family with facet identity substituted throughout (`s.stdio.semio.{brep,mesh}.inference`, `Stdio_semio_{brep,mesh}_inference`, `Semio{Brep,Mesh}Inference{Text,Binary}`, JSON `$id` path, ksy `meta.id`, proto package). Verified no `cad`/`Cad` token leaked into either tree.

**Verified with DKM's own independently-written enumerator, not the peer's**: `TOTAL LEAF GAPS: 0` across all inference families; `cargo check -p semio-s-plugin-stdio` → 0 errors.

### Two judgement calls, recorded rather than buried

1. **These leaves are minimal, and that is the designed convention — not a stub.** I checked `✳️cad`'s before copying, specifically because the inference-family ticket's own summary rejects placeholder grammar leaves as worse than honest incompleteness. Its `🦀️component.rs` explains itself: *"Inference values are never authored via DSL text (they are always computed from a snapshot, never a source of truth), so — unlike `📸️snapshot/📝️text`'s live `parse_dsl`/`print_dsl` pair — this leaf declares the wire grammar only."* Declaration-only is correct for this facet family. **Had cad's been a live parse/print pair, copying a scaffold would have been shipping stubs, and I would have said so instead.**
2. **Copied the framing magic `0x8953f83f7d340d0b` rather than minting new ones** — measured first: 16 of 17 existing inference binary protocols share that value, 1 uses `…0c`. A shared magic is the convention; a fresh one would be the anomaly. Flagged to the peer that if the outlier is deliberate and magics are meant to be per-facet, DKM will mint distinct ones — better before their flip than after.

### Honest state of the two facets

Structurally complete, **not finished work**. brep carries a real `✅validation-report` slug with a genuine `DepHash` chain; mesh carries `📦aabb`. Both deliberately omit fields whose dependency chains could not be built honestly (brep omitted `tessellation` and `mass-properties`, with reasoning recorded) rather than faking them. stdio's **test** target still carries one DKM error — a `MutationDiff` trait-in-scope problem in a mesh `#[cfg(test)]` assert — which does not propagate to any other crate.

## W4 CLOSED — all three stdio subsets complete, registered, and verified

```
cargo check -p semio-s-plugin-stdio --all-targets (RUSTC_WRAPPER="")  → 0 errors
cargo test  -p semio-s-plugin-stdio --lib                             → 2415 passed, 5 failed
failures in brep / drawing / mesh / semio::                            → 0
```
Baseline at W0 was **2168 passed / 6 failed**. Now **+247 tests and fewer failures**, with the 5 residuals owned elsewhere (`dwg`/`ifc` `fixture_honesty_law`, unowned; `dxf` bounds + `zip` entries, the inference-family session's).

| subset | triads | inference | leaves | banned vocab | registered |
|---|---|---|---|---|---|
| `✳️brep` | 13 | `✅validation-report`, real `DepHash` chain | 8/6 | 0 | ✅ |
| `✳️drawing` | 17 | `🎛flattened-scene`, honest per-entity chain | 8/6 | 0 | ✅ |
| `✳️mesh` | 17 | `📦aabb` | 8/6 | 0 | ✅ |

**Registration gap closed.** A peer's coverage scan (extract each family's descriptor fn name, then search for call sites *outside the defining file*) found exactly three unregistered families repo-wide — all DKM's. The facets compiled and passed their laws while being invisible to the registry the family exists to feed. Fixed by mirroring the 16 sibling subsets: a `register_artifact_inferences()` fn calling the fully-qualified descriptor, invoked from `register()` in each subset's `🚪️io/🦀️component.rs`. **All 112 families now register, not 109.**

**The repo-wide taxonomy flip landed clean on the back of this work** — `💡️inferences` entered `schemaChildDirs`, and the allowlist-free, hard-gating `policySchemaRepresentationBreaches` began demanding the full inference tree on all 112 owning subsets **with zero new representation breaches** (high breaches 24802 → 24801). The repo's hardest structural check now enforces the family and passes.

### 🔑 The instrument lesson — the most transferable thing learned tonight

DKM's leaf enumerator asked *"does `📝️text/` exist?"* and reported **0 gaps**. A peer's asked *"how many files are in it?"* and found **10 missing** in `✳️drawing`. Same tree, minutes apart. Mine was not unlucky — it was **structurally incapable** of seeing the defect, because it was a **presence check labelled as a completeness check**.

The sting: `📌️important.md` already carried *"grep to find, enumerate to count"*, written days-equivalent earlier after watching two peers inflate counts. **Knowing a rule and encoding it into the instrument you actually run are different acts.** That gap between written rule and shipped instrument bit DKM twice tonight — here, and in the dispatch briefs that omitted the mount rule.

The peer's counterpart, which completes it: within the same hour their own scan produced *false* gaps across a dozen families from unquoted-`$VAR` word-splitting in zsh, and **they caught it only because the number looked implausible**. So:

> **The instrument needs verifying as carefully as the tree** — and a *plausible-looking* wrong number gives you no signal at all, which is precisely why an under-reporting check is more dangerous than an over-reporting one.

Neither session was ignorant of the principle at the moment it violated it.

## `✳️mesh` DONE — final state of all three subsets, independently re-verified

| subset | triads | mounts | leaves | banned vocab | registered | inference mount |
|---|---|---|---|---|---|---|
| `✳️brep` | 13 | 13 | 8/6 | **0** | ✅ | ✅ |
| `✳️drawing` | 17 | 17 | 8/6 | **0** | ✅ | ✅ |
| `✳️mesh` | 17 | 17 | 8/6 | **0** | ✅ | ✅ |

`stdio glue.rs`: **3,209 `#[path]` mounts, 0 dangling.** `cargo check --all-targets` (`RUSTC_WRAPPER=""`) → **0 errors**. `cargo test --lib` → **2415 passed / 5 failed**, none in `brep`/`drawing`/`mesh`/`semio::`.

### Two defects the final sweep caught that the agent's own report called clean

The mesh report claimed completion, and the wave *was* substantially right — but a coordinator sweep found two things it had missed, both real:

1. **2 files still carried banned vocabulary — in prose.** `✳️mesh/🚪️io:410` and `✳️mesh/🧬️schema/🔺️diff:2` named the banned identifiers inside comments *explaining that those identifiers no longer exist*. `📌️important.md` warns explicitly that **the policy greps raw file content including comments**, so a comment describing the removal trips the rule as surely as the code would have. Rewritten to describe the concept without naming it ("the banned no-op sentinel", "a whole-document replace"). This is a genuinely counter-intuitive failure mode: **the documentation of a fix can violate the rule the fix satisfies.**
2. The agent's report listed the inference-registration `sharedFileRequests` item as still open; the coordinator had already closed it for all three subsets. Harmless, but a reminder that a report is a snapshot of its author's knowledge at write time, not of the tree.

### Concurrent-churn incident worth preserving

The mesh agent verified its inference mount complete at **76/76 paths resolving**, then found it had dropped to **74/76** on a later pass — **another session's commits had landed on the shared `📦️glue.rs` and silently dropped two of its mounts.** It re-applied and re-verified with a full recompile. This is the sharpest instance tonight of *"a verification is a timestamp, not a property"*: the agent's first measurement was correct **and** the tree later disagreed with it, through no error of its own.

It also caught its own **unicode-normalization typo twice before shipping** — a corrupted CJK sequence where `🏅️standards` belonged — using the same path-resolution script. Exactly the trap that produced silently-empty modules and silently-empty searches elsewhere tonight, caught here by an instrument built for it.

## W3b — surface family DONE, and the result reframes what "73 setters" meant

Coordinator-verified independently (forced recheck, both mandatory flags):
```
cargo test  -p semio-framework-surface --lib        → 214 passed, 0 failed   (baseline unchanged)
cargo check -p semio-framework-surface --all-targets → 0 errors
fn set_ across 🗺️surface                            → 73  (unchanged — zero public API change)
```

### The finding: all three modules own no authoritative state — for a *stronger* reason than terrain

Terrain's fields mirrored plugin state where only *one* field had a shipped mutation. Here, **every mirrored field's real owner is an already-shipped, event-sourced artifact with conforming triads**, confirmed on disk rather than asserted — and I re-verified the two headline claims myself:

| surface field | real owner | owner's vocabulary (verified) |
|---|---|---|
| `RasterHost.document` | `✏️s/🔌️plugins/🖨️raster` `RasterSnapshot` | **12 triads** (create/delete/rename/move/resize/reorder-layer, change-layer-*, add/remove-layer-asset) |
| `RasterHost` camera/brush/tool | raster's shipped `RasterConfig` | LOCAL_UI mutations |
| `MapHost.features` | `✏️s/🔌️plugins/🌍️gis` `GismapSnapshot` | **12 triads** (positions/routes/regions) |
| `MapHost` render/style/visibility | gis's shipped `Gis2dConfig` | config-lane mutations |
| `GraphHost.dag` | OS `flow`'s `Widget`/`SynapseSpec` | ⚠️ real but **not yet event-sourced** — that is W3c's boundary, in flight |

The gis case is the most convincing: the plugin's own `map_host_from()` **rebuilds `MapHost` fresh from the snapshot on every call**, which is the definition of a projection rather than a store.

### 🔑 Why this is the doctrine working, not the doctrine being dodged

Those 73 `fn set_` are **setters on a render-side projection**, not CRUD over authoritative state. Dissolving them into mutation triads would have **duplicated the vocabulary across two owners** — precisely the violation this ticket exists to remove. The right outcome was to author **nothing** and document the ownership, which is what happened: docstring-only changes (paint 1838→1880, node-graph 1145→1169, tiled-map 4236→4272), zero public API change.

**This is the second time the honest answer was "no vocabulary here".** The pattern worth carrying: the test is never *"does this look like a setter"* — it is *"who owns the authority"*. A setter over a projection whose owner is already event-sourced is correct code; converting it would be the regression.

⚠️ **It also means the W0 baseline metric was measuring the wrong thing.** "89 `fn set_` in claimed kernel dirs" was recorded as the size of the CRUD problem. Of those, **73 are surface** and are now confirmed legitimate. The count was never a measure of violations — the same instrument error as the presence-vs-completeness check, in the ticket's own headline number.

### Honest remainders from this wave

- `GraphHost.dag`'s owner (`💻️os/🔨️modules/🌊️flow/🌿️vcs`) is **not yet event-sourced**; node-graph's `move`/`connect`/`disconnect` verbs are designed and were sent to SMO, but cannot be authored until W3c lands the conforming dispatch enum.
- The framework-module schema placement question **remains genuinely unpiloted repo-wide** — not because it was dodged, but because after tracing, no surface module needed a schema to place.
- `semio-s-plugin-gis` has 3 pre-existing errors in `🏔️gisterrain` (not `🗺️gismap`), confirmed by mtime/git-log as concurrent churn predating this wave. Not fixed, not attributed here.
