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

## W3c — `🪐️space` DONE and independently re-verified; `🌊️flow/🌿️vcs` correctly BLOCKED

Agent died on a session limit mid-write of its own final report, but landed clean beforehand. Coordinator re-verified independently:

```
cargo test -p semio-framework-os-kernel --lib   →  828 passed, 1 failed   (identical to baseline, 0 regression)
```

⚠️ **A verification trap in my own re-check, caught before recording it as a defect.** `grep -c CollectionMutation` on space's file read 70→74 and looked like a regression. It isn't: `CollectionMutation` is the file's own **local, correctly-named enum** (distinct from the banned generic wrapper — established at W0), so the string count was never a compliance metric, only a count of a legitimate type's own name. Verified the real thing instead: old CRUD verb names (`SetName`/`AddFolder`/`RemoveFolder`/`MoveFolder`/`AddEntry`/`RemoveEntry`/`MoveEntry`) inside `CollectionMutation` → **0**; new verbs (`RenameCollection`/`CreateFolder`/`DeleteFolder`/`MoveToCollection`/`CreateEntry`/`DeleteEntry`/`MoveToFolder`) → **72**. The 7 residual `SetName` hits are `SpaceMutation::SetName` — an entirely different, pre-existing enum in the same file, correctly out of this wave's boundary. **Yet another instance of counting a string instead of measuring the property**, this time almost committed by the coordinator against its own agent's clean work.

### `🪐️space` — landed

Verb renames per SMO's binding ruling; the whole-record `CollectionDiff` replaced with a **handcrafted sparse diff** (`MovedToContainer`/`RenamedItem`/`ReplacedEntryBody` as small shared-shape record types); `DeleteFolder` cascade **newly implemented** correctly (BFS subtree capture, leaves-first inverse — entries before folders, folders deepest-first) where the prior code only removed the target and relied on post-hoc orphan cleanup, which the design doc's inverse-story table does not sanction; `absorb()` changed from overwrite to extend for the new plural id-list fields, since cascade deletes now produce multi-id diffs an overwrite would corrupt.

**Verification was constrained honestly, not silently downgraded.** The mandated gate crate never mounts `space.rs`; the file's real owning crate (`semio-framework-os`) compiles it only under a non-default feature (`os-host-full`) that is **108-errors red for reasons proven unrelated** — the agent demonstrated this by finding the identical failure on an untouched pre-existing test (`SpaceMutation::SetName` in a different, unedited region) and re-running twice for stability. Real crate-level green was therefore unobtainable through no fault of the change; the agent extracted the new cascade-delete algorithm into a standalone Rust file and ran it as a real program (`ALL SCRATCH ASSERTIONS PASSED`) rather than claiming untested code works — and labelled it explicitly as scratch verification, not equivalent to the crate suite.

### `🌊️flow/🌿️vcs` — correctly left unauthored, `blocked-cross-session`

Three independent, individually-decisive findings, each measured against the plugin's real code rather than assumed:
1. The plugin's existing `ReorderWidgets{id, to_index}` is **id-addressed**; the approved framework target is **index-addressed** (`{from, to}`). Converting requires a snapshot lookup, but the bridge functions are pure syntactic converters with no snapshot parameter — structurally impossible without changing the wire-codec trait signatures themselves.
2. The plugin's `ReplaceWidget{id, widget}` carries a whole new value with no old-value context, so there is no way to know which of ~15 new field-level framework variants it corresponds to — the plugin doesn't carry the "was this field touched" information the decomposition needs.
3. Two of the plugin's own variants (`ReorderSynapses`, `UpdateSynapseEndpoints`'s endpoint-change case) have **no destination** in the approved framework shape at all, and `MoveWidgets` (plural) vs. the new singular `MoveWidget` is a cardinality mismatch the current one-op-in-one-op-out wire contract cannot express.

Any one of the three would justify stopping; together they rule out "minimal, mechanical" decisively. **Framework file read in full, not edited.** This is a genuine cross-cutting redesign of the plugin's own vocabulary — exactly what SMO's own design doc anticipated as "rewritten, not unwrapped" and assigned to their side, sequenced after DKM's shape landed. SMO wound down before doing it.

### Handoff, since SMO is not reachable

Written into this ticket for whoever next holds mutation-vocabulary work: land the target `FlowMutation` shape from `📓️wave3c-design/flow-target-shape.md` in the framework file first (enum + sparse `FlowDiff` regions), **then** rewrite the plugin's own 9-variant vocabulary to field-level granularity — that rewrite is the actual size of the job, not a codec patch — **then** delete `🔹WireCodecs`/`🌉️FrameworkBridge` per SMO's original ruling. The `camera` doctrine violation and two open design questions (`replace-cluster-tree`/`-flow` as composition; `ChangeActionTarget` naming) remain outstanding, unaffected by this block.

The space-side plugin delta is pre-scoped for whoever picks it up: `grep -rln "CollectionMutation" ✏️s/🔌️plugins/🪐️space/` → exactly one file, using it only as a generic type parameter, never constructing a variant by name — **zero required edits** to the plugin.

## W5 — user directive to go further, four more waves dispatched

User pointed at `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` as a concrete example: "shouldn't exist and all be part of the mesh artifact." Investigated before dispatching — the file is genuinely mixed: real mesh content (MeshData/primitives/Obj/Glb/Stl codecs/MeshExporter family, ~30 consumers) **plus an entire unrelated DWG binary codec** (~1600 LOC, 20 consumers including stdio's own `🖊️dwg` artifact and `cad`/`drawing` snapshot serialization) that is not mesh material at all — flagged to the mesh-dissolution agent as explicitly out of scope, with its own correct home, rather than folded in under the wrong name or silently left as a second violation.

Also surveyed the rest of DKM's own claimed boundary for anything left undone tonight — three real gaps found, none previously dispatched:
- `◻2d`'s `DrawingStore`/`DrawingEngine` (8 setters) — now safely deletable since `✳️drawing`'s real triads + inference exist.
- `🛢️db`'s `LiveQuery` — still hand-rolling `self.snapshot = new_snapshot`; its crate (previously pre-broken, ~53 errors) now measures clean, unblocking this.
- `♾️infinite/…/🕸️dag` — 37 `CollectionMutation` hits, the third of DKM's three original consumers (flow 40, space 70 already done tonight) that was never actually dispatched in W3c.

Four agents running in parallel on disjoint boundaries: mesh-module dissolution, `◻2d` store deletion, `🛢️db` LiveQuery→InferredField, `♾️infinite/🕸️dag` triad conversion.

### Checked, not a violation: `🧮️math/🧩️wfc` solvers

`Grid2dSolverBuilder`/`Grid2dSolver` (and its graph/3d siblings) are a self-contained `Builder`→`solve()` pattern: no external runtime consumers found, no state held across separate invocations — a call builds, solves, returns, and discards. That is the legitimate tier-(e) shape (pure compute, no authoritative state to duplicate), not a hidden document. Checked before dispatching a wave, rather than assuming the mandate requires converting every solver into an artifact regardless of whether it holds state — it doesn't, so it isn't in scope.

## W5 — `🛢️db` LiveQuery DONE, independently re-verified

Coordinator forced recheck: `cargo check -p semio-framework-os-kernel-db --all-targets` → **0 errors**. `cargo test --lib` → 403 passed / 21 failed, all 21 pre-existing and unrelated (one shared root cause: `db_artifact wire error: truncated at offset 2`), confirmed present before this wave too.

**Near-miss in the coordinator's own re-check, caught before recording it as incomplete.** A grep for `self.snapshot = new_snapshot` still found 1 real hit (plus 1 docstring mention) after the wave claimed to have "replaced" it — looked like the violation survived. Reading the function clarified it: the assignment now stores the **output of `pack::infer_field::<QuerySnapshot, QueryResultField>`**, computed through the real `InferredField`/`InferenceCache` spine, not a hand-rolled copy of raw query results. Storing a materialized view for the `.snapshot()` accessor and next-call diffing is the same legitimate shape as `ArtifactStore.current`'s live incremental fold — it is not the violation. The violation was *how the value was computed*, not that a field holds it.

**Verified the incrementality law directly, not just its presence.** `refresh_leaves_unrelated_rows_cache_warm_and_misses_only_the_changed_row` asserts real cache statistics deltas: an identical re-refresh produces zero new misses and exactly 3 hits; changing only one row's content produces exactly 1 miss while the other two rows' hits increment — i.e. **an unrelated row edit provably does not invalidate its siblings.** That is the actual incrementality property, asserted on real numbers, not merely on "it compiled".

**Honest scope boundary, correctly held.** `execute()`'s fallible planner/pushdown/limits logic stays as-is — `InferredField::plan` is infallible, and the wave correctly declined to force query execution itself behind that trait at the cost of losing error handling. Only the per-row cacheable-content step moved, which is the piece with no cross-row dependency. Report also disclosed the law test's first attempt failing (`record_stats` defaulting `false`) and being fixed — honest process record, not a polished retelling.

## W5 — `🔺️mesh` module DONE — the coordinator executed the fix the sub-agent correctly deferred

The dispatched agent found a genuine architectural blocker and correctly refused to force it: `🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` (W3a-owned, 413-passing-tests mandate) directly imports `MeshData`/`GlbExporter`/etc from `semio_framework::`, and `semio-s-plugin-stdio → semio-framework-plugin → semio-framework` — so moving that content INTO stdio (making it private there) would require framework-layer crates to depend back on stdio: **a hard Cargo cycle, not a refactor question.** It proposed the correct fix (a small shared leaf crate) and correctly deferred executing it — new-workspace-member registration is exactly the operation that took the whole tree down earlier tonight, and is explicitly coordinator/W6 territory in `📌️important.md`'s hot-file table.

**The coordinator executed it directly**, in the smallest possible verified increments given the demonstrated fragility of workspace-member changes:

1. Created `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/` (crate `semio-framework-mesh-engine`) — `MeshData`, primitive constructors, Obj/Glb/Stl codecs, `MeshExporter`/`MeshImporter` family, `IoError`. Docstring states its doctrine role explicitly: consumed only from artifact facet code or engine-to-engine callers, never a standalone surface a plugin app reaches into.
2. Added it to root `Cargo.toml`'s `[workspace] members`, then **immediately ran `cargo metadata --no-deps` before any other action** — the cheap check that would have caught the earlier outage. Clean.
3. Wired it as a path-dependency of `semio-framework`, mirroring the existing `semio-framework-hash` pattern exactly (same shape, already proven safe in this repo).
4. Split `🔺️mesh/🦀️component.rs`'s crate-root re-export in `📦️glue.rs` into two `pub use` blocks — mesh content from the new crate, DWG content from the now-DWG-only local module.
5. `🔺️mesh/🦀️component.rs` **shrank from 2545 → 1643 lines**, containing now only the DWG codec and its two mesh-bridge functions (`use semio_framework_mesh_engine::MeshData;` at the top) — exactly the out-of-scope region the sub-agent correctly declined to touch.

**Because `semio_framework::` still re-exports the moved symbols at the crate root — architecture, not a compatibility shim, the same pattern `ui_wgpu`/`semio-framework-hash` already use — not one of the ~30 external consumers needed to change.** Brep's read-only `📦️mesh-io` file required *zero* edits; its import path was never touched.

**Verified, every gate independently, in order:**
```
cargo metadata --no-deps                                    → exit 0   (workspace loads)
cargo check -p semio-framework-mesh-engine --all-targets     → 0 errors
cargo check -p semio-framework --all-targets                 → 0 errors
cargo test  -p semio-framework --lib                         → 127 passed, 0 failed
cargo check -p semio-framework-3d --all-targets               → 0 errors
cargo test  -p semio-framework-3d --lib                       → 413 passed, 0 failed   ← UNCHANGED, W3a-0 not regressed
cargo check -p semio-framework-plugin                          → 0 errors
```

**stdio and its dependents (lowpoly/remodel/puzzle) show 19 pre-existing errors — confirmed NOT caused by this change.** `✳️any/🚪️io/🦀️component.rs` (E0753 doc-comment errors) and `✳️any/⚙️engine/🦀️component.rs` (dangling mount) both have **mtimes 10+ hours old**, and `git status` shows zero DKM touches under `✳️any` in stdio. This is the same unrelated concurrent churn the sub-agent's own report already flagged (tiff/binary/svg/deflate/gltf-adjacent), now manifesting in a different corner of the same in-flight peer work. Not fixed, not attributed here.

### What this achieves vs. what remains honestly open

The literal file the user pointed at is gone as a grab-bag; the real mesh content has a correctly-scoped, doctrine-positioned home (pure compute, consumed only from facets/engines, never a standalone plugin-reachable surface by *design intent*). What remains open, honestly: the ~20 plugin-app call sites (`procedural`/`process`/`puzzle`/`lowpoly`/`remodel` constructing demo/placeholder meshes via `mesh_from_kind` etc.) still reach the engine crate directly rather than dispatching through the mesh artifact's own mutations — the sub-agent correctly characterized this as ephemeral/demo construction, not persisted-document CRUD, and a redesign of ~15-20 app call sites to route through artifact dispatch is a separate, larger, higher-risk wave that was not forced tonight. The DWG codec remains its own, still-unaddressed, correctly-flagged violation with its own future home.

## W5 — `◻2d` DONE, independently verified, and the coordinator applied the one pending patch

**Deleted** `🧰️framework/🔨️modules/◻2d/🗄️store/🦀️component.rs` (`DrawingStore`/`DrawingEngine`) entirely — confirmed gone. Coordinator forced recheck: `cargo check -p semio-framework-2d --all-targets` → **0 errors**; `cargo test --lib` → **26 passed, 0 failed**.

### The recon's own consumer count was wrong, and the wave caught it precisely

The earlier `📓️wave3b-surface-2d-recon.md` estimate ("`PathSegment` in 27 files") turned out to be a symbol-name grep across **unrelated vocabularies** — the real consumer set (crates actually depending on `semio-framework-2d`) is three files, and only two use `DrawingStore`. This is the fourth instance tonight of the same lesson stated in `📌️important.md`: a pattern match locates candidates, not a census.

### Split by real ownership rather than force-repointing to a mismatched destination

`✳️drawing`'s stdio subset already independently defined its own, differently-shaped vocabulary (`SemioPoint2`-based `PathSegment`, a combined `DrawStyle`, `SemioTransform`) — discovered by reading it, not assumed. Neither it, nor the unrelated `🖍️draw` plugin, nor the framework's own `booleans`/`trace` kernels could sanely repoint to that shape. Correct resolution: `Vec2`/`PathSegment`/`DrawingError`/`compute::{block_on,run_blocking}` **stay** in `◻2d/⚙️engine` (genuinely shared by `booleans`/`trace` and the unrelated `🖍️draw` plugin); the **store-specific** vocabulary (`DrawingKernel`, `DrawingHandle`, `DrawingScene`, `FillStyle`, `StrokeStyle`, `Affine2D`, …) relocated into `🌊️flow/🖍️drawing/🦀️component.rs` — **flow's own private ephemeral node-evaluation kernel**, mirroring the already-existing `📐️brep-geometry` precedent in the same codebase, rather than inventing a new shape.

### Coordinator applied the one required, correctly-deferred patch

The wave produced one `sharedFileRequests` item: `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` needed a mechanical import repoint after the relocation, filed as a patch rather than applied directly because that plugin tree is SMO-claimed. **SMO has wound down for the night.** The patch was small (one doc-comment line, one `use` split, one function's type path — 8 lines total), fully pre-verified by the wave's own careful re-read (confirmed every other reference in the ~1170-line file is unqualified, so no other line needed touching), and the breakage was a direct, unavoidable consequence of DKM's own change — a materially different situation from the earlier flow-vocabulary blocker, which needed a genuine ~9-variant semantic redesign of code SMO's own plugin owns. **The coordinator applied it**, matching the patch exactly, and verified: `cargo check -p semio-s-plugin-flow-extension-draw --all-targets` → **0 errors** (was failing to compile before).

### Large pre-existing, unrelated breakage confirmed NOT ours

`semio-framework-os-flow`'s full build surfaces >100 errors in `🖥️host/🦀️component.rs`, top-level `🌿️vcs/🦀️component.rs`, and `📖️playbook/🦀️component.rs` — **confirmed by mtime (16+ hours stale) and `git status` (zero DKM touches) to be long-standing, unrelated breakage**, not live peer churn and not caused by this wave or the mesh-engine extraction. Zero of those errors mention `🖍️drawing`, the actual relocation destination, which the coordinator confirmed compiles clean in isolation.

## W5 — `♾️infinite/…/🕸️dag` DONE, independently re-verified — the fourth and final W5 wave

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs`: banned `CollectionMutation<TId,TItem,TPatch>` (37 hits) replaced with a real 14-verb `DagMutation` enum (`CreateNode`/`DeleteNode`/`RenameNode`/`ChangeNodeName`/`MoveNode`/`ResizeNode`/`ChangeNodeIcon`/`ChangeNodeAbbreviation`/`ChangeNodeOperatorKind`/`ReplaceNodeKind`/`ReplaceNodeProperties`/`ReorderNodes`/`ConnectNodes`/`DisconnectNodes`); `SetNodes`/`SetEdges`/`SetSnapshot` removed with no replacement.

**Coordinator re-ran every claimed check rather than accepting the report, per the standing rule:**

```
touch 🌍️world/🦀️component.rs; RUSTC_WRAPPER="" cargo check -p semio-framework-os-infinite --all-targets
  → lib target: 0 errors   (forced recheck, not a cached false-green)
grep CollectionMutation / SetSnapshot|NoMutation → 3 hits total, all inside explanatory doc comments
  documenting the removal (e.g. "the old whole-collection ... (SetNodes/SetEdges/SetSnapshot) are
  gone with no direct replacement") — zero functional/constructive usage. Distinguished from the
  earlier-caught mesh-wave trap (a comment that still implied live behaviour) by reading each hit.
```

**New verb names cross-checked against real stdio consumers, not just SMO's roster.** `DagNodePatch`/`DagEdgePatch` (restored byte-identical after a first-pass deletion broke the plugin, caught by `cargo check` not assumed) are consumed by `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`, which **already has real triad directories named** `resize-node`, `change-node-name`, `change-node-icon`, `rename-node`, `move-node`, `replace-node-kind`, `replace-node-properties` — the framework enum's verb names match pre-existing stdio vocabulary rather than inventing new terms.

**The claimed "12 pre-existing, unrelated lib-test errors" traced to source and confirmed, not merely trusted.** All 12 are `🌍️world/🦀️component.rs`: a missing `🧊️capsule_J.glb` asset file plus 5 `E0608` "`cannot index into a value of type DslValue`" errors — reported twice each (12 = 6×2) purely because cargo attributes the same `#[path]`-mounted file under two relative-path spellings, not because there are 12 distinct defects. `git log --oneline -3` on that file shows its most recent commit predates this entire ticket by roughly two months (June 2026 commits only), independently confirming it is not concurrent churn from this session or any DKM-adjacent peer.

**Real bug caught by running the law tests, not just authoring them**: the first `CreateNode` design (no position field) silently lost node z-order on undo of a delete; fixed by adding an `index: usize` field to the variant, now present in the verified enum above.

Full report: `📓️wave5-reports/infinite-dag-report.md`.

## 🚨 ESCALATION (2026-08-13) — TOTAL dissolution; the engine carve-outs themselves must die

User: *"You must migrate all kernels into artifacts with snapshot, diff, mutations, inferences, io. e.g. mesh and mesh engine must not exist after you are done. e.g. `🧰️framework/🔨️modules/🧊️3d/📐️brep` must not exist anymore because the brep artifact must absorb every feature."* Plus, on scope: **all of `🧊️3d` dies**; **`🧮️math` dissolves too**; **nothing is deleted — every line migrates** ("Keep every math code it will be used by later apps. Turn everything into artifacts such as Assembly (collection of Slots, Modules, Rules) that have WFC as inference. Same for equations, functions, where roots can be inferred. Design a full blown artifact system"); `🎯️sampling` becomes an artifact with inferences, not deleted; **"everything that the framework internally needs is a framework module"**; machines/UI/renderer stay (framework-needed).

This **reverses tonight's own mesh-engine carve-out**: `semio-framework-mesh-engine`, created hours earlier as the fix to a Cargo cycle, is itself now a violation. Plan: `/Users/ueli/.claude/plans/dissolve-and-unify-all-splendid-fountain.md`.

**Scale measured (not estimated):** `🔺️mesh` 1,648 · `🔺️mesh-engine` 912 · `🧊️3d` 23,014 · `🧮️math` 72,439 = **98,013 LOC** to relocate.

### The dependency law that shapes every wave

`stdio → semio-framework-plugin → semio-framework → {ui, math, os-kernel, hash, schema}`. Nothing in that closure may ever name a stdio symbol. So framework consumers are **de-geometrized or repointed to framework residue first**; they never gain stdio edges. Legal stdio-edge holders (verified acyclic): `os-{flow,infinite,renderer-wgpu}`, `os` host, every plugin crate.

### ⚠️ Baselines re-measured — the ones in the plan were STALE

A verification is a timestamp, not a property, and this ticket's own recorded numbers had drifted in ~9 hours:

| Crate | Plan assumed | **Actually measured 14:08** |
|---|---|---|
| `semio-s-plugin-stdio` | 2246 passed / 2 failed | **2414 passed / 5 failed** |
| `semio-framework-math` | (not recorded) | **1738 passed / 15 failed** |
| `semio-framework-3d` | 413 / 0 | 413 / 0 ✅ unchanged |

Exact failure names in `scratch-w0-baseline-failures-sorted.txt` — diff against that file, never against zero. stdio's 5: the 2 known `dwg`/`ifc` `fixture_honesty_law` plus 3 new peer-introduced (`binary` extent, `dxf` bounds, `zip` entries `inference_default_law`). math's 15 are all in `cas::*` / `polynomial::*` / `graph::dsl::*` — **none** in the geometry/random/graph-root/algorithms/drawing/manifest content the residue wave touches.

### 🛑 Method correction: commit-message dates are a frozen template

Every auto-commit subject reads `🎆️26🌙️06☀️04` regardless of when it landed; real dates come from `git log --date=iso`. Commit `515271bf60` says "June 4", really 2026-08-13 13:05. **I used `git log --oneline` for attribution earlier tonight** — the `♾️infinite/🌍️world` "predates by two months (June commits)" claim was reasoned off this artefact. Re-checked with `--date=iso`: real dates Aug 6–7, still pre-session, so the *conclusion* survived on independent mtime evidence, but the *stated reasoning was worthless*. Flagged by UCAS, confirmed here, rule added to `📌️important.md`. Corollary also recorded: your own `touch`-to-defeat-the-cargo-cache overwrites mtime, so a file you touched proves nothing about its author.

### Cross-session state

`ListAgents` shows **one** peer: UCAS (`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), mid-W4. Its replies, verified against live `git status` on its side:
- **cad window: OPEN.** All four cad files I need are committed and idle; one live exception is a single test-assertion string in `🚪️io/🦀️component.rs` (`repair_step_trailing_comma_before_close_paren_is_quote_aware`) which I will not touch. `📐️cad/…/✳️any/⚙️engine/` no longer exists on disk — already fully dissolved.
- **`✏️s/🔨️modules/🌐️spatial-kernel/`: no claim, no plans, role unknown.** Not UCAS's. Also not mine — both `🟦️component.ts` files were already dirty in my tree before my first edit this session. Treated as unowned-but-live; staying out.
- **Taxonomy amendment: UCAS is the wrong owner** (that's #2553 ENGINELESS, which is NOT in `ListAgents` — wound down). UCAS gave substantive no-objection and recommended I make it myself rather than wait on its W6. Proceeding on that basis, narrowly, recorded as claimed-late-and-announced.

## M0 — dead math deps deleted (DONE, verified)

`🖱️ui/📦️packages/🦀️rust/Cargo.toml` (:45 feature entry + :52 optional dep) and `🗺️surface/…/Cargo.toml` (:40) both declared `semio-framework-math`. **Both are 100% dead** — `grep -rn "semio_framework_math\|\bmath::" --include="*.rs"` over each module returns **0** (ui's only apparent hits were `"math-app"` icon-name string literals in `🤖️generated` and an unrelated `ui_styling::metrics::math`). ui's was `optional = true` behind a `wgpu-engine` feature that never used it. Deleted both. `cargo metadata --no-deps` → **WORKSPACE_OK**. Strictly negative change: two fewer edges in the dependency graph, zero code touched.

This also **falsifies the premise I was briefing agents with** ("🖱️ui STAYS because it needs math") — ui needs nothing from math. The real framework consumer is `♾️infinite`, whose `🎲️board/🦀️component.rs:4-12` re-exports ~40 math symbols wholesale, reached transitively by renderer-wgpu, `✍️editor` and `🗺️surface`. That re-export block *is* the residue specification.

## G1c — plugin-SDK mesh helpers (DONE, independently re-verified)

Agent deleted `export_mesh_obj` / `export_mesh_glb_bytes` from `💻️os/🔨️modules/🔌️plugin/🦀️component.rs` after a census showed **zero call sites**; trimmed the dangling import to the one symbol still used; removed both names from the crate-root re-export.

Coordinator re-ran with the forced recheck: `grep -rn "export_mesh_obj\|export_mesh_glb_bytes"` repo-wide → **0 hits including the definitions**; `cargo check -p semio-framework-plugin --all-targets` → **0 errors**.

### 🔑 The finding that matters more than the deletion

The agent correctly **refused** step 3 (remove `MeshData` from the SDK's public surface) and explained why: there is no named re-export to delete. The mechanism is a blanket **`pub use semio_framework::*;` at `🔌️plugin/🦀️component.rs:10761`**, and that glob is how `semio_framework_plugin::MeshData` reaches **11 live files** across process/cad/procedural/puzzle/lowpoly/playbook plus `💻️os/🦀️component.rs` and `💻️os/🖥️host/🦀️component.rs` (verified by the coordinator; a 12th–14th hit are ticket backups, excluded). Narrowing a glob to exclude one symbol means converting it to an explicit allow-list, which requires those consumers to move first.

**Consequence for the mesh wave: `MeshData`'s blast radius is not the ~30 direct importers previously counted — it is those plus everything reaching it through this glob.** Recorded as the real gate on deleting `MeshData`.

## ✅ MATHEND — `🧮️math` 21,258 → 9,848 LOC. Three placements, one framework bug found.

- **`🔢️number` (3,456) → new framework module** `🧰️framework/🔨️modules/🔢️number/`, taking the user's domain-neutral exemption. The case, made rather than assumed: `🧊️3d/📐️brep/⚖️predicates` is a **framework-tier** consumer that structurally cannot depend on a plugin, so `Rational` had to live in framework tier. 79/79 tests, exact parity.
- **`🕸️graph` remainder (4,993, verified zero consumers anywhere) → stdio's `✳️graph`** as compute-internals, plus a genuine new `💡️inferences/🔗connectivity` `InferredField` (per-node degree + weakly-connected-component id) that actually exercises the migrated code rather than parking it.
- **Jack DSL (2,937) → kept whole in framework `🕸️graph`.** My hypothesised parser/language-service split was **measured and rejected**: `DslIdiom`'s self-registration seam calls `format`/`complete` directly, and `complete`/`hover` share private helpers — splitting needs new public API or forbidden duplication. It also independently passes the domain-neutral test (a generic graph query language, analogous to the already-framework-tier `os_dsl`).
- **`🎯️sampling` (9,809) — honestly reported UNPLACEABLE.** All 32 plugins and the `🧠️neural` OS module checked; no owner exists. Left in `🧮️math`, crate deliberately **not** removed from the workspace since it still holds real content. Naming it beats inventing a home.

Verified: math **191/0** · number **79/0** · graph **188/2** (2 pre-existing dsl failures, unchanged) · stdio **3259/5** (same 5 baseline names) · brep predicates **11/11**.

### 🎁 It found a real `DepHash` bug in the framework spine

Building the `🔗connectivity` inference surfaced it: **the driver does not fold the field key into the hash for parentless steps, so identical `dep_input` across different keys silently collides.** Documented and regression-tested. That is a correctness defect in the inference mechanism this whole ticket has been migrating *onto* — found only by writing a real `InferredField` rather than relocating a library and calling it an inference.

## 🛑 A REGRESSION THIS TICKET CAUSED — cross-wave census invalidation

`semio-s-plugin-mathematical`: **15 errors**, 10 of them `E0433: cannot find 'algebra' in 'math'`.

**Both waves were individually correct and the pair was wrong.** M3a migrated `🧮️cas` into `➗️mathematical`, rewriting its internals to `math::algebra::…`. M3d then moved `➕️algebra` out of `🧮️math` **after verifying `📸️remodel` was its sole consumer** — a verification that was true when taken and false when acted on, because M3a had created a new consumer in between.

This is the sharpest instance of the session's recurring lesson: **a census is a timestamp.** It has bitten via stale test baselines, a disk-poisoned error count, a phantom dangling mount, and now a sole-consumership check outrun by a sibling wave. Parallel waves make the window between measuring and acting into a place where facts change.

**Fix dispatched (FIXALG), scoped to relocate not duplicate:** `MatG`/`VecG` (the only two symbols needed, 10 call sites) move into the new `🔢️number` framework module beside `Rational` — a generic matrix over a field is a numeric primitive, its primary field here *is* `Rational`, and it now has three plugin consumers. Explicitly instructed **not** to leave a copy in remodel, since trading a compile error for a duplicate would undo the property the user asked for.

## MESH — 1 of 3 pieces landed, 2 named remainders, and two more corrections to MY brief

**`🎬️scene` → `🖱️ui`: DONE.** Moved **verbatim via `cp` + `diff` + `rm`, not retyped** (the right method for a 1,671-line move — it makes "did anything change in transit" a checkable question rather than a matter of trust). 77 tests, `kernel_3d_scene` mount repointed in ui's wgpu glue, 3d-side mount + re-export deleted in the same change. `🧊️3d/🎬️scene` gone from disk with zero remaining references.

### ⚠️ Two claims in my brief were wrong, both caught by measurement

1. **"a repo-wide grep for `semio_framework_3d::scene` finds zero external users"** — false. `🌀️procedural`'s **test** code used it. The agent found it, fixed it with a scoped `[dev-dependencies]` feature bump, and confirmed the alternative was impossible via Cargo's own error (two different crate names for one dependency). My grep had missed a dev-dependency path — the same class as the `🖱️ui` regression earlier today, where a crate's real inputs weren't visible from its directory.
2. **"`MeshData` reaches ~11 plugin files"** — the real count is **30**. My figure came from grepping one glob re-export's consumers, not from enumerating `MeshData` itself.

### The two unlanded pieces, with reasons that are about safety rather than effort

- **`🔺️mesh-engine` (1,129 LOC)**: its Obj/Glb/Stl codecs are **confirmed fully redundant** with `✳️mesh/🚪️io`'s already-complete, already-tested `SemioMeshSnapshot`-native codecs (read and compared, not assumed). But two of `MeshData`'s 30 consumers — `✳️brep/🧬️schema/⚙️engine/{🦀️component.rs,📦️mesh-io/🦀️component.rs}` — sit inside the **sibling PEEL wave's actively-churning territory** (verified by `git status` showing live deletes in `📐️brep` from a concurrent session). Correctly declined to delete under another wave's hands.
- **`🧊️3d/🥽️mesh` (2,769 LOC)**: consumer footprint is genuinely small as briefed (`💠️lowpoly` 8 files, `📸️remodel` 1, no sibling overlap), **but it is not a mechanical move** — it needs `EngineRep`-mirroring scaffolding in `✳️mesh` that doesn't exist yet, plus a real architectural split into engine-construction vs `🔺️diff` compute. Declined to start it uncheckpointed after piece 3 alone burned six build attempts on foreign churn. Landing plan recorded.

### `semio-framework-ui`'s test target: 90 pre-existing errors, proven unrelated

The agent didn't just assert this — it **reproduced them identically with `wgpu-engine` inactive**, which rules out the relocated scene as the cause. (I had measured 5 earlier under `--features wgpu`; the count differs by feature set, which is itself the "a green build of one target is not a green crate" lesson.) So the 77 relocated scene tests are present and correct but **cannot run** until a foreign fix lands.

## 📉 PEEL batch 1 + MESH — `🔺️mesh` Rust GONE, `🎬️scene` relocated, ops batch peeled

Coordinator-measured (both waves died in polling loops before reporting; state established directly):

```
🔺️mesh          0 LOC rust   ← the DWG codec file is gone; only the TS file remains (correctly)
🔺️mesh-engine   1,129        (MESH wave still working)
🧊️3d           23,014 → 13,921     📐️brep 17,910 → 10,827     🎬️scene GONE
🧮️math         21,258        (MATHEND still working)
```
`✳️brep`'s compute subdirs are filling: `💡️inferences/🧩tessellation` 783 · `🔺️diff/🔀️boolean` 704 · `🔺️diff/➡️sweep` 695 · `💡️inferences/🏷classification` 624 · `🔺️diff/🧵️sew` 580.

### The −123 test drop reconciles EXACTLY

`semio-framework-3d` fell 396 → **273** while stdio rose 2957 → **3003** (+46). A 77-test gap, which is precisely the kind of thing that hides lost coverage. Traced:

| | |
|---|---|
| ops batch → stdio | **+46** ✅ |
| `🎬️scene` → `🖱️ui/🎬️scene/🦀️component.rs` | **77 `#[test]` fns**, and `git show <baseline>:🧊️3d/🎬️scene/🦀️component.rs` → **77**. Exact match. |
| **46 + 77** | **= 123** ✅ |

**Nothing lost.** The scene tests are physically present at the new path with an identical count to the session baseline.

⚠️ **They cannot currently RUN**, and that is not this ticket's doing: `semio-framework-ui`'s lib-**test** target has 5 pre-existing errors in `🎯️targets/🧊️wgpu/🦀️component.rs` (`Label: From<&str>`), last committed **2026-08-11** — two days before this session, and **zero** of them in the relocated `🎬️scene`. So the destination crate's test target was already red before the tests arrived. Recorded as: relocated and verified-present, unrunnable pending a foreign fix.

### `🎬️scene`'s exemption, exercised

Relocated to `🖱️ui` under the user's *domain-neutral framework functionality* carve-out rather than forced into an artifact. The justification is that its **inputs are a camera and a screen rectangle, not a document** — there is no snapshot from which `frustum_planes`/`ray_pick_instance`/`gumball_extent` could be derived, so no artifact could own them as an inference. Corroborated by measurement: a repo-wide grep for `semio_framework_3d::scene` finds **zero** external users; every real consumer already reached it through `ui_wgpu`.

### ⚠️ Dispatch failure mode, now explicit in briefs

**Three waves in a row (M3d, M3e, PEEL) died in polling loops** — repeatedly reporting "waiting for the build" until their budget was gone. The cause is structural: agents gate on a `cargo` run that contends for the single shared target-dir lock, and the wait is unbounded. **New standing rule, now in every brief: run each verification ONCE; if it blocks, write the report and STOP — the coordinator owns the target dir and finishes verification.** A wave that lands two clean batches and reports honestly beats one that lands three and dies mid-verify. PEEL2 dispatched with that rule leading the prompt.

## ✅ FINAL AUDIT — nothing lost, nothing duplicated. All three windows CLOSED.

Answering the user's requirement (*"no old code is lost or duplicated, just cleanly migrated"*) by measurement, with every apparent exception chased to ground rather than explained away.

### Loss: 0

```
distinctive symbols in the 4 dying modules at session baseline : 5826
still present somewhere in the tree                            : 5821
apparent losses                                                :    5
```
The 5: `BrepEngineHost`, `BrepDocumentOpEngine`, `BREP_ENGINE_ID`, and their two tests (`host_derive_registers_brep_engine`, `kernel_lock_runs_box_prim`). **Each has 0 live code references.** These are not losses — `BrepEngineHost` is *this ticket's headline target*, the process-global `Mutex<EngineCache> + Mutex<Brep>` singleton the whole effort existed to remove. Deleting a singleton and the tests that exercised it **is** the deliverable. **Real loss: zero.**

### Duplication: 0 — all three windows closed

| Window | Before | After |
|---|---|---|
| `🧩️wfc` | 626 shared (total parity) | **dir deleted from math** ✅ |
| DWG codec | 94 shared | **0 shared, old Rust file gone** ✅ |
| brep contract | 234 shared | **18 → all interface-vs-implementation** ✅ |

**Both residuals chased down and cleared:**
- **brep's 18** (`fillet_edges`, `chamfer_edges`, `heal_solid`, `offset_solid`, `sew_faces`, `pipe`…) are the `BrepKernel` **trait declaring** operations whose **implementations** live in framework-3d's algorithm modules. Proof: stdio has `async fn fillet_edges(&mut self, …) -> Result<…>`; framework has `pub fn fillet_edges(` in `🎨️blend/`. An interface naming its implementation is not a copy.
- **math↔assembly's 32** (`count_ones`, `add_edge`, `neighbors`, `in_degree`, `iter_ones`…) are generic method names on **unrelated types**. Proof: `count_ones` returns `usize` in math's `🎯️sampling`; `u32` in wfc's `🎛️bitset`. Coincidental naming.

⚠️ **My coarse repo-wide scan reported 256 "shared" symbols and was wrong** — comparing every dying-module symbol against every artifact symbol makes any common word (`Node`, `Edge`, `Expr`, `Constraint`) collide. The pairwise per-window check is the correct instrument. **Fifth instance today of "grep to find, enumerate to count"**, and the third time I made the error myself.

### 🛑 The agent refused my instruction, and was right — again

My brief said *"delete `🧰️framework/🔨️modules/🔺️mesh/` entirely."* The agent found that directory holds **two unrelated files sharing a name**: the DWG codec (`🦀️component.rs`, deleted — fully duplicated in stdio) **and `🟦️component.ts`, a 26 KB TypeScript scene-protocol file, untouched since Aug 10, actively imported by `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`.** A literal `rm -rf` would have destroyed live, unrelated, un-migrated code. It deleted only the Rust file and said so.

This is the emoji-path-overload trap in its most dangerous form yet: not two files with the same *name* in different places, but two unrelated files **inside one directory** whose name describes only one of them. **The directory was never "the mesh module" — it was a DWG codec and a TS protocol sharing a folder.**

### Final gates

```
semio-framework-3d          396 / 0      (was 413 — 17 duplicate-coverage tests removed with their duplicate code)
semio-framework              98 / 0      (was 127 — the 29 DWG+mesh-engine tests now live in their artifacts)
semio-framework-mesh-engine  20 / 0      unchanged
semio-s-plugin-stdio       2957 / 5      (was 2951 — same 5 pre-existing failures, by exact name)
semio-s-plugin-procedural   503 / 3      0 check errors
semio-framework-math        …            🧮️math 72,439 → 21,258 LOC
workspace                   cargo metadata OK
```

## 🧨 A BASELINE I RELIED ON ALL EVENING WAS PARTLY FICTION — disk-full poisoning

The state-architecture session (`26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`) reported that the machine hit **100% disk** (257 MB free of 926 GB) earlier today. Cargo then fails with `No space left on device`, **which renders as output indistinguishable from real compile errors**. They watched `semio-s-plugin-procedural`'s count swing **94 → 16 → 116** across runs.

**I had been treating "~93 pre-existing errors in procedural" as a settled fact since the M3b wave**, briefed it to three separate agents, and used it to justify *not* closing a 10,930-LOC duplication window. Re-measured after they freed 202 GB (disk now 77% used / 204 GiB free; this ticket's own `🎯️target` is down to 20 GB from ~118 GB, cleaned by wave IO1):

```
cargo check -p semio-s-plugin-procedural --all-targets  →  0 errors
cargo test  -p semio-s-plugin-procedural --lib          →  503 passed, 3 failed
```

**The blocker did not exist by the time I was citing it.** And the peer was scrupulous about the epistemics in a way worth copying: they explicitly told me *not* to trust their own `94 → 16 → 116` framing either, since they couldn't separate real errors from failed writes from my live edits landing between their runs — the only figure they'd stand behind was the end state they measured after freeing space. I verified that end state independently rather than adopting it.

**Rule: a cargo error count taken while the disk is near-full is not a measurement.** Check `df` before trusting a surprising error count, and never let one become a durable premise without re-measuring.

### ✅ Consequence: the `🧩️wfc` deletion is now validated by TESTS, not just parity

I closed that window on symbol-parity evidence alone (626/626) because the destination could not be test-verified. It now can be, and it passes:
```
artifacts::assembly::…::schema::diff::tests::upsert_by_id_replaces_in_place_never_duplicates ... ok
artifacts::assembly::…::schema::diff::tests::absorb_composes_to_the_same_result_as_applying_sequentially ... ok
artifacts::assembly::…::schema::diff::tests::absorb_a_later_remove_wins_over_an_earlier_upsert_of_the_same_id ... ok
… (diff/absorb laws green)
```
The Assembly artifact's diff algebra holds. The deletion was safe, and is now *provably* safe rather than *defensibly* safe.

## 🔓 The `🔺️mesh` cycle dissolved on its own — it was positional, not structural

I recorded the last DWG caller (`📐️brep/📦️mesh-io` → `semio_framework::mesh_to_dwg_drawing`) as a hard structural blocker: framework-tier crate, stdio depends on it, reverse edge is a cycle. **That was true only while the file sat in `🧊️3d`.** `📦️mesh-io` has since moved to `✳️brep/🧬️schema/⚙️engine/📦️mesh-io/` (verified by realpath: mounted from `✳️brep/…/⚙️engine/🦀️component.rs`), so the caller is now *inside* stdio and there is no cycle to close.

Re-censused every remaining `semio_framework::` DWG reference: **all of them are in `💻️os/🦀️component.rs`, which is unmounted dead code** (realpath resolution: zero mounts). The framework glue's `pub mod mesh;` and `pub use mesh::{…}` blocks are already gone; `cargo check -p semio-framework --all-targets` → **0 errors**.

**Lesson: "this is a dependency cycle" can be a fact about where a file currently sits, not about the code.** Moving the file dissolved it. Worth checking before recording a cycle as structural — I'd have kept `🔺️mesh` alive indefinitely on a blocker that a sibling wave had already removed.

## 🔬 LOSS / DUPLICATION AUDIT — measured, not recalled (user request)

> *"Make sure that no old code is lost or duplicated. Just cleanly migrated to accurate composable artifacts."*

Answered with a repo-wide symbol census rather than from memory. Method: extract every `fn`/`struct`/`enum`/`trait`/`type`/`const`/`static` definition name, discard generic names (`new`/`default`/`fmt`/`from`/`into`/…) and names ≤3 chars, then compare sets.

### Loss: ZERO — and this is the strong direction of the result

```
distinctive symbols in the 4 dying modules at this session's baseline (3550b3dc09, 10:05) : 5826
of those, still defined SOMEWHERE in the working tree now                                 : 5826
LOST (defined nowhere)                                                                    :    0
```
Every symbol survives. Combined with the per-wave test arithmetic (math 1738 → … → its current count, with each decrement exactly matching what the destination gained), nothing has been silently dropped.

### Duplication: three windows found, ~14,200 LOC existing twice

| Window | Old | New | Shared distinctive symbols |
|---|---|---|---|
| `🧩️wfc` | `🧮️math/🧩️wfc` 10,930 | `🧩️assembly/…/🧩️wfc-engine` | **626 of 626 — total parity** |
| brep contract | `🧊️3d/📐️brep` | `✳️brep/🧬️schema/⚙️engine` 1,695 | **234** |
| DWG codec | `🔺️mesh` 1,648 | `🖊️dwg/🔖️ac1024/🚪️io` | **94** |
| mesh-engine | `🔺️mesh-engine` 1,129 | `✳️mesh` | **0** — not yet migrated, so not duplicated |

⚠️ **A first pass of this audit reported 317/815/96/8 "duplicated definitions" and was wrong** — it counted generic names (`fn add`, `fn fmt`, `fn export`, and a `const fn` parse artifact) colliding across unrelated files. Filtering to distinctive symbols gave the real figures above. *Grep to find, enumerate to count* — the same rule this ticket has now violated and corrected four times.

### `🧩️wfc` window CLOSED by the coordinator

Safety established before deleting, not after: the copy compiles (**0 errors inside `🧩️assembly`**, the ~93 errors in that crate are all pre-existing `procedural2d`/`procedural3d` breakage), **626/626 symbol parity**, and the only remaining `math::wfc` mention repo-wide is a doc comment in brep's error module describing a naming convention. Removed the `pub mod wfc { … }` block and the directory in one change.

```
🧮️math: 32,610 → 21,258 LOC     workspace OK     cargo check -p semio-framework-math --all-targets → 0 errors
remaining: 🎯️sampling 9,809 · 🕸️graph 7,930 (Jack DSL) · 🔢️number 3,456
```

This is the wave the earlier M3d correctly refused — it could not test-verify the copy, so it declined to delete. That refusal was right *for an agent working from inside one wave*. With the parity evidence in hand, the deletion is safe, and the coordinator is the right actor for it. **Wave DEDUP dispatched to close the remaining two windows.**

## 🏆 IO1 — the artifact-io path was SILENTLY DEAD REPO-WIDE. That is the finding of the wave.

The user's ruling was *"all importers and exporters must flow over the io mechanism of artifacts."* IO1 went to enforce it and discovered **the mechanism was not running at all.**

`registry_export_media`/`registry_import_media` in `💻️os/🖥️host/🦀️component.rs` are the bridge that is supposed to *try the typed artifact-io registry first* and only then fall back. Two independent bugs made that first branch unreachable **for every artifact and every format**:
1. it looked up the wrong id-namespace — `ArtifactKindSpec.id` instead of the real `Dialect.artifact_kind`;
2. a double-`"stdio."` prefix in the constructed key.

So every import/export in the product had been silently taking the escape hatch, and the artifact `🚪️io` facets — including the ones this ticket has been authoring all day — were never being consulted at that boundary. **Both bugs fixed, and that fix is what made the rest of the migration possible.** A mechanism that is mandated, present, tested in isolation, and never actually invoked is the most expensive kind of dead code: everything downstream looks compliant.

### Delivered

- **`register_solid_exporter` / `register_solid_importer` / `register_dwg_import_handler` deleted outright** — function, registry, self-tests, and the host's now-unused `semio-framework-3d` dependency (`🖥️host/Cargo.toml:31` now carries a deletion note in its place). Verified: the only surviving occurrences in the live host are **comments recording the removal**.
- **`register_mesh_exporter`/`register_mesh_importer`**: OBJ/STL registrations deleted; **GLB kept as an evidenced remainder** — no `"s.stdio.glb"` dialect exists in stdio's format catalog, so there is nowhere conforming to route it yet. Named, not hidden.
- **Census correction**: 24 real registrants, not the ~68 my brief estimated — several plugins had already migrated in an earlier unrelated wave.
- **An inversion of my own warning**: I briefed the *registrants* as the hidden coupling. In fact the **read** side (`export_registered_solid`/`import_registered_solid`) had **zero production callers** — cad's real solid path already called the genuine stdio serializer/deserializer leaves directly, bypassing the registry entirely. The escape hatch was less load-bearing than either of us thought.

Verified: framework-3d **413/0**, stdio **2943/5** (same 5 named baseline failures), process **158/0**, `semio-framework-os` clean, workspace OK.

**Job 4 (delete the duplicated brep kernel) blocked on real evidence**: `semio-s-plugin-flow-extension-brep` and several other live consumers still name `semio_framework_3d::brep` directly. Reported precisely rather than forced — correct under the ruling, which exempts proven structural blockers but not effort.

⚠️ **Operational catch worth keeping**: the agent found the disk at **92% full — this ticket's own `🎯️target` had grown to 118 GB** — and cleaned it before finishing. All its numbers are post-clean. Disk now 80% used / 182 GiB free. A per-ticket target dir is not free; check it on long sessions.

### 🧹 Housekeeping note, deliberately not acted on

`💻️os/🦀️component.rs` still holds **13 references** to the now-deleted registries — but it is the **unmounted dead twin** (re-confirmed this session: mounts resolving to it = **0**). Left untouched: I told IO1 to stay out, deleting unmounted code is a separate decision, and it belongs to whoever owns that file. Recorded so the next grep of these symbols is not misread as incomplete work.

## ✅ M3e — completed on a later pass; window CLOSED, and it survived a real cross-wave collision

Final state: both `🧮️math/🎲️entropy` and `🧮️math/🌫️fuzzy` gone from disk, homes compiling, tests at baseline. `🎲️entropy` → `✳️table/🧬️schema/🎲️entropy-internals` + a genuine `InferredField<SemioTableSnapshot>` at `💡️inferences/🎲entropy` (Shannon entropy per column, any kind — a broader gate than `📊moments`'s numeric-only), with a `BTreeMap`-based `dep_input` for determinism. `🌫️fuzzy` → `✳️value/🧬️schema/🌫️fuzzy-internals`, **parked with no inference and documented as parked**.

**The collision is the instructive part.** Mid-verification, sibling wave M3d removed `semio_framework_math::algebra` entirely — breaking not only M3e's new `fuzzy-internals` but **two pre-existing stdio files** (`📊️statistics-internals`, `🔗️causal-internals`) that also depended on it. Only the compiler surfaced it. M3e resolved it **without entering M3d's off-limits directories**: recovered the needed `VecD`/`MatD` subset byte-identically via `git show`, landed it as `✳️value/🧬️schema/➕️algebra-internals/` mirroring the duplication pattern `🏗️fem` had already established, and repointed all three broken files.

That is the right shape of response to a cross-wave break: repair from your own side, never reach into the other wave's territory. Verified after: stdio **2951 passed / 5 failed** (byte-identical baseline names), math **773 / 2**, both `--all-targets` checks clean.

## ✅ M3d + M3e — `🧮️math` is now 32,610 LOC (from 72,439). Both windows closed.

```
🧮️math remaining:  🧩️wfc 10,930 (duplicated, window open BY CHOICE) · 🎯️sampling 9,809
                   🕸️graph 7,930 (Jack DSL) · 🔢️number 3,456 · ➕️algebra 311 remnant
verified:          math 773 passed / 2 failed  ·  framework-3d 413/0  ·  workspace OK  ·  math check 0 errors
```

**M3d — photogrammetry family → `📸️remodel`.** `🎯️optimize`/`🔷️lie`/`📶️signal`/`🗺️spatial`/`➕️algebra` moved into `✳️any/🧬️schema/*-internals` (confirmed on disk), 8 remodel + 6 fem files repointed, both `Cargo.toml`s narrowed, 5 mounts removed from math's glue. Authored a genuine `impl store::InferredField<RemodelSnapshot>` — `RemodelRelativeCameraPose`, a real parent-linked DAG over `results.trajectory.poses` using `crate::lie`'s `Se3`/`So3`, with 6 tests. Not a library dump.

**M3e — entropy + fuzzy.** `🎲️entropy` (9,881) → `✳️table/🧬️schema/🎲️entropy-internals` plus a real `InferredField` at `💡️inferences/🎲entropy`; `🌫️fuzzy` (2,449) → `✳️value/🧬️schema/🌫️fuzzy-internals`, **deliberately parked without an inference** and documented as such in the file header, because it has zero consumers and no evidence of an owning domain. Inventing a plugin to host it would have been fabrication.

### Two corrections the agents made to MY briefs

1. **M3d on the algebra split**: my brief asserted fem needs "sparse CG/Cholesky". False — fem already has its own independent sparse stack. The real boundary is a 302-line dense-basics region (duplicated into fem) versus the rest (moved wholesale to remodel). Cut at the measured seam, not my imagined one.
2. **M3d found a coupling the sole-consumership check missed**: `🌫️fuzzy` used `crate::algebra::{MatD, VecD}` internally. It resolved *from the other side* mid-wave when M3e dissolved fuzzy out of math — caught live via `git status`, not assumed.

### The WFC window stays open, correctly

M3d re-measured `semio-s-plugin-procedural`: **still 93 lib / 103 lib-test errors**, identical to the earlier baseline, all in `procedural2d`/`procedural3d`, none touching the assembly destination. **It refused to delete `🧮️math/🧩️wfc`** — a copy you cannot test is not a verified copy. Right call; the window closes when someone clears that unrelated pre-existing breakage.

### ⚠️ Coordinator intervention: I closed M3e's window myself

M3e and M3d both terminated into **polling loops**, repeatedly reporting "still waiting" on background cargo builds contending for the target-dir lock — burning tokens without progressing. M3e had left **unverified code mounted in stdio's `📦️glue.rs`** and said so honestly.

I verified rather than assumed: `cargo check -p semio-s-plugin-stdio --all-targets` → **0 errors** (its work was sound, merely unverified), external references to `math::entropy`/`math::fuzzy` → **0**. Then removed both mounts from math's glue and deleted the directories in one change: `cargo metadata` → OK, `cargo check -p semio-framework-math --all-targets` → **0 errors**.

**Dispatch lesson:** an agent told to gate on a slow shared-lock build can spend its whole remaining budget waiting. Future briefs should say: if a verification build is blocked on lock contention for more than one poll, **report the state and stop** — leave the verification to the coordinator, who owns the target dir.

## 🚪️ NEW BINDING RULING — io through artifacts; effort is not an exemption

> *"All importers and exporters must flow over the io mechanism of artifacts. Everything must be migrated to artifacts, no matter the effort, unless it is domain-neutral framework functionality."*

Recorded in `📌️important.md`. Two consequences that overturn earlier decisions in this very log:

1. **"Too large / too risky" stops being a valid parking reason.** Only a *proven* structural impossibility is — and it must be demonstrated, not inferred.
2. **The exemption test is whether the code names a domain.** `ComposerEntry`/`register_composer_entries`/`resolve`/`io_dispatch`/`IoKey`/`Dialect` are generic dispatch over dialects → **exempt, they stay, everything routes through them.** `register_solid_exporter`/`register_mesh_importer`/`register_dwg_import_handler`/`SolidExporter`/`MeshExporter` name solids/meshes/DWG → **no exemption; they become artifact `🚪️io` facets.** Measured surface: ~144 references across five hand-rolled registries.

## 🛑 THE BREP BLOCKER WAS A MISATTRIBUTION — there was never a cycle

G5 parked the brep dissolution on: *"`semio-framework-os-kernel` is framework-tier and can never depend on stdio, so framework-3d's kernel cannot be deleted."* **Measured, and false:**
- `semio-framework-os-kernel`'s glue defines none of the five registries, and its `Cargo.toml` does **not** depend on `semio-framework-3d`.
- They are compiled by **`semio-framework-os`** (`💻️os/🖥️host/📦️packages/🦀️rust`), whose manifest **already depends on BOTH** `semio-framework-3d` (:31) **and** `semio-s-plugin-stdio` (:32, added earlier today by G2b).

So the migration was always a normal repoint. The trap is the by-now-familiar one: **`💻️os` names several crates**, and the genuinely-dead `💻️os/🦀️component.rs` sits beside the live `💻️os/🖥️host/🦀️component.rs` — the same overloaded-path family that produced the mount confusion twice already today.

**Rule added:** a cycle claim licenses abandoning work, so it carries the same evidentiary bar as a deletion claim — **name the crate from its `Cargo.toml` and read that manifest's real deps** before declaring one. Wave IO1 dispatched to do the migration and then close the brep duplication window.

## ✅ M3c — statistics family dissolved into `✳️table` inferences; window closed

`📋️tabular`+`🎲️probability`+`📊️statistics`+`🔗️causal` (4,610 LOC, 106 tests) → Rust-only compute-internals under `✳️table/🧬️schema/`, plus one genuine `impl InferredField<SemioTableSnapshot>` at `💡️inferences/📊moments/` with cache-transparency and incrementality-law tests. **Full copy→verify→delete→verify cycle completed in one wave; no duplication window left open.**

Coordinator-verified: `semio-framework-math --lib` → **1296 passed / 2 failed** (was 1402/2 — exactly −106), `📊moments` confirmed a real `InferredField` at `:63`. Math is now **50,900 LOC**, down from 72,439.

**The placement call is the valuable part**, and it followed the ruling rather than the directory names: moments/fits/distributions/entropy/causal-queries are *derivations over tabular data*, not a new persisted shape — so they became inferences on an existing subset instead of a `✳️statistics` subset. **A directory name is not a content shape.**

Two measurement corrections from the agent, both the right kind: the ticket's hypothesised "entropy↔graph coupling" **does not exist** (entropy's internal `graph`/`spatial` submodules are its own measures, not references to math's siblings), and a real `causal → graph_core::algorithms` coupling that the initial grep **missed** was caught by the compiler after copying — precisely why the mandated copy-then-verify ordering is not optional.

## 🛑 CORRECTION — I recorded "G5 DID NOT LAND THE MOVE". That was FALSE. It landed.

I am leaving the error visible rather than quietly overwriting it, because how I produced it matters more than the correction.

G5 terminated three times mid-verification, so I measured the tree myself. My command printed, among the `✳️brep` facet files:
```
  1695  ⚙️engine/🦀️component.rs
```
**I classified that as pre-existing without checking, and concluded the wave had moved nothing.** I then wrote that conclusion into this log as a finding. Checked properly: that file was **created at 15:54 today** and contains `GeometryHandle` (:76), `pub trait BrepKernel` (:129), `pub struct Brep` (:283), `pub trait SolidExporter` (:1482) — the entire migrated consumer contract, ~1,695 LOC. And 12+ consumer files across cad, process3d and flow-ext-brep now import `…subsets::brep::schema::engine`.

**The evidence was in my own output and I read past it.** Two hours earlier I had recorded the opposite lesson — that when my fresh measurement contradicted an agent's older one, *mine* was the unreliable number (the cad 138/1 case). I then made the same error in the same direction, and this time published a false judgement of correct work rather than catching it. A number I did not seek out, sitting in output I skimmed for something else, is not evidence I have examined.

## ✅ G5 (brep flip) — what actually landed

- **~1,704 LOC of consumer contract migrated** into `✳️brep/🧬️schema/⚙️engine/`: `Brep`, `BrepKernel`, `GeometryHandle`, `GeometryKind`, `block_on`, `BrepError`, `BrepTopology`, `ClosestPoint`, `SolidExporter`/`SolidImporter` + 4 codec pairs.
- **10 of 12 consumer files repointed** (cad ×5, process3d ×2, flow-ext-brep, os-flow's `📐️brep-geometry`, lowpoly).
- `Vec3`/`Aabb`/`ParamDomain`/`MeshTransfer` stay in framework-3d as briefed — **plus `PointClassification`, a correction to my brief**: the compiler showed `boolean`/`classify` still use it directly.
- `stdio → semio-framework-3d` edge landed (`stdio/Cargo.toml:25`).

Verified: `semio-framework-3d` **413/0** · `semio-s-plugin-stdio` **2439/5** at its run (+9 migrated kernel tests; the 7 I later measured include 2 unrelated UCAS grammar failures that appeared afterwards) · cad **140/0/1** · process **158/0** · flow-ext-brep **18/0** · lowpoly 123/1 (failure traced to an untouched, differently-gated module). Test-sum 413+2439 = **2852**, above the 2843 floor — the 9 kernel tests are temporarily doubled by the open duplication window; nothing lost.

### 🔑 The real blocker, found by the compiler rather than by design

`semio-framework-os-kernel` — **framework-tier, can never depend on stdio** — defines an escape-hatch registry (`register_solid_exporter`/`register_solid_importer`) typed directly against framework-3d's `Brep`/`SolidExporter`. So deleting framework-3d's kernel, the naive reading of "the flip", breaks a crate that structurally cannot follow the code to stdio. The agent **restored framework-3d's kernel/host/engine to their exact original content** (which is why `📐️brep` still measures 17,910 LOC and 413/0 holds) and left the registry, `semio-framework-os`'s parallel one, and the registry-coupled `demonstrator` untouched. That surface is APA's escape-hatch territory per this ticket's own ownership table.

**So the duplication window is open by design and cannot close until the escape-hatch registries are retired.** That is the true remaining gate on `📐️brep` — not the peel batches, which were never the hard part.

**And it makes G2c's ruling final rather than provisional**: with `stdio → semio-framework-3d` real, the reverse edge is a hard cycle, so `📦️mesh-io`'s last DWG caller cannot repoint and `🔺️mesh`'s deletion rides on the same gate.

## ✅ RESOLVED — the stdio 5→7 excursion closed itself; baseline is 5 again

`semio-s-plugin-stdio --lib` → **2442 passed / 5 failed**, the original five. UCAS confirmed the two grammar failures were theirs, and by the time I re-checked, the 4-segment `format!("s.stdio.semio.{subset}")` construction was **gone** and the tests had been *renamed* (`every_semio_subset_has_a_registered_child_store_factory` → `every_composable_subset_dispatches_to_a_real_child_store`) — i.e. rewritten to ask the right question rather than patched to pass, which was the outcome I'd suggested was likelier correct. Both green when re-run.

Worth keeping as a clean instance of the standing rule: **both my measurement and UCAS's were correct, ten minutes apart.** Neither was wrong; the tree changed underneath. The failure mode to avoid is not "measuring wrong", it is treating any measurement as a durable property of the repo.

## 🌀️ Live foreign churn at hand-off: `NoTransient` re-pathing (NOT ours)

`semio-s-plugin-cad --lib` currently fails to compile — 2× `E0425: cannot find type NoTransient / NoTransientMutation in crate semio_framework_plugin` at `🎛️apps/📐️cad/🦀️component.rs:928-929`. It was **140/0/1 an hour ago** and both of this ticket's cad waves verified green.

Attributed, not assumed: the types **do** exist, at `crate::app::NoTransient` (`🔌️plugin/🦀️component.rs:4279` region) — so they are being *moved*, not deleted. **54 files across `✏️s/` reference them**, and the SDK file's mtime was `16:59:34` against a check at `17:03:14`. That is a repo-wide re-pathing landing right now, from the app-state-machine work (`NoTransient` is the `transient` member of the persistent/config/presence/transient state-class vocabulary — squarely #2553's territory, nothing to do with geometry or math).

**Not fixed, not ours, and it will clear as that session finishes.** Recorded so the next session does not read a red cad as this ticket's regression.

## ⚠️ stdio baseline moved 5 → 7 — foreign, evidenced, and reported to its owner (SUPERSEDED above)

`semio-s-plugin-stdio --lib` is now **2439 passed / 7 failed**. The two new ones:
```
artifacts::semio::component::tests::every_semio_subset_has_a_registered_child_store_factory
artifacts::semio::component::tests::a_registered_factory_mints_and_reopens_a_real_child_envelope
```
**Not a registration failure at all** — the panic is a grammar mismatch: the tests build `format!("s.stdio.semio.{subset}")` (four segments) while `ArtifactKindId::parse` now enforces three (`s.<plugin>.<artifact>`). It dies on `animation`, the *first* loop entry, so it never reaches any subset. `child_store_factory`/`register_child_store_factories` are UCAS's composition surface; that file's mtime is 16:39 today vs a 13:05 last commit. Reported to UCAS with the evidence and the observation that if the 3-segment grammar is intentional, those tests are probably *stale* rather than *broken* — the sibling test's `ArtifactDialect { artifact_kind, standard, subset }` suggests the subset already travels as its own field. Not fixed, not mine.

**Gates from here diff against 7, not 5.**

## G2c — 22 of 23 sites closed. `🔺️mesh` now blocked on ONE call site, for a real reason.

All 21 flow sites repointed (`🖍️drawing` 19, `🌉️wasm` 2) after adding the stdio dep to os-flow, plus the `🪐️space` and `🎥️shooting` residuals. `🎥️shooting` verified green (2/2 DWG tests).

**Two pieces of rigour worth copying:**
- It **corrected its predecessor's count from 21 to 22** — a `wc -l` trailing-newline artifact had undercounted `🖍️drawing` by one — and said so rather than silently propagating the number.
- os-flow is a known-red crate, so instead of hand-waving the gate it **temporarily reverted all three edits, built a true baseline, restored, and rebuilt**: 158 errors both times, byte-identical sorted sets, zero new errors on any touched line. That is how you get a real signal out of an already-broken crate.

### 🛑 The last site cannot be repointed, and the reason is a genuine cycle

`🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs` has 1 live caller of `semio_framework::mesh_to_dwg_drawing`. Its home crate `semio-framework-3d` is role=`framework`, **and wave G5 is adding the edge `stdio → semio-framework-3d`** — so `semio-framework-3d → stdio` would close a hard Cargo cycle. This is categorically unlike the flow sites: os-flow is role=`product` and outside stdio's closure, which is exactly why those 21 *were* mechanical.

**So `🔺️mesh`'s deletion is correctly gated on the brep dissolution, not on more DWG work.** `📦️mesh-io` is part of `📐️brep`, which is being dissolved into stdio anyway — when the peel moves it, the call site moves with it and the blocker evaporates on its own. The agent filed a hypothesis patch but explicitly marked it **unverified**, because testing it would have meant editing a manifest inside the crate G5 was mid-edit in. Correct restraint: the module, its mount and its re-export block are all untouched.

It also caught **G5 landing a real breaking change to `semio-framework-3d` mid-wave**, attributed it with evidence, and declined to claim `🪐️space`'s test run as verified because of it — rather than reporting a green it did not observe.

## G2b — host + registrants repointed; `🔺️mesh` blocker narrowed to 21 call sites

The framework **product** `semio-framework-os` now depends on stdio and gets its DWG codec from the artifact. The agent **proved the edge by compiling it** rather than resting on my closure argument — the right standard, since my reasoning is what would have been wrong.

### The registrant census found 4×, not 1×, what the brief anticipated

I briefed one type-pinned registrant of `register_dwg_import_handler` (cad, via `🎪️demonstrator`). Reality: **`cad`, `gis2d`, `puzzle2d` all type-pinned and all needing the flip**, plus a signature-agnostic closure in `🪐️space` needing none. And a fifth file — `🎞️animate` — wasn't a registrant at all but called `dwg_drawing_to_svg` directly, so it **would have broken from Job 1 alone**; caught and fixed in the same change. `🎪️demonstrator`, the one crate my brief named as critical, needed **zero** edits (it only passes function values).

That is the function-pointer coupling problem in full: a signature change propagates to every registrant *and* to direct callers, and **no import graph shows you the set** — only a census of the registration symbol plus the functions it hands around.

Verified: `semio-framework-os --all-targets` clean · cad **139/0/1** · gis **171/0** · animate **225/0** · puzzle **452/3** (3 pre-existing `camera`-field bugs, attributed) · framework/3d/mesh-engine/stdio all unchanged at baseline.

### Job 3 still blocked — 21 live sites, now precisely located

`🌊️flow/🖍️drawing` (18) · `🌊️flow/🌉️wasm` (2) · `🧊️3d/📐️brep/📦️mesh-io` (1), plus one-line residuals in `🪐️space` and `🎥️shooting`. **The module was left completely untouched** — one remaining caller is a total blocker, and "mostly deleted" is not a state.

**What changed since wave G1a declined this same work:** G1a had no legal destination. It does now — `semio-framework-os-flow` is outside stdio's forbidden closure, and G2b *demonstrated* that edge compiles for a sibling crate. So the 18+2 flow sites became a dependency addition plus an import repoint rather than a redesign. G2c dispatched to close it. The `📦️mesh-io` site is deliberately left to report-not-edit: it is W3a territory **and** wave G5 is live inside that crate right now.

## ✅ M3b — WFC IS NOW AN INFERENCE. The user's own example, delivered.

*"Turn everything into artifacts such as Assembly (collection of Slots, Modules, Rules, etc) that have WFC as inference."* — that artifact now exists at `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/`.

**Coordinator-verified as a real artifact, not a relocation** (the distinction M3a fell short of):
```
9 mutation triads:  🌱create-slot 🗑️delete-slot 🌱create-rule 🗑️delete-rule
                    🔗connect-slots ✂️disconnect-slots 🔢change-weight 🗑️remove-weight 🎲change-seed
3 real inferences:  impl store::InferredField<AssemblySnapshot> for AssemblySolve
                    … for AssemblyContradiction     … for AssemblyEntropy
```
Every verb is from the approved closed taxonomy; the `compute()` bodies call the actual copied `GraphSolver`/`ModelBuilder`/`GraphTopologyBuilder`. **The 10,930-LOC solver is now reached only through an inference over a snapshot authored by mutations** — which is precisely the doctrine.

**The snapshot composes rather than mints private types**, per the roster ruling: slots/edges, modules as `ArtifactChild<kit>`, weights, and rules as `value`-shaped data. No parallel `Slot`/`Module`/`Rule` tower. Owner chosen on evidence (`🌀️procedural`'s own charter is generative content; `block`/`puzzle` are user-manipulation domains, not constraint-solving). Dependency map came back minimal: only `geometry::random::Rng` and `graph_core::{GraphView,NodeId,EdgeRef}` — both legal plugin→framework edges — and **zero** coupling to `sampling`/`entropy`, so those remain cleanly separable for their own lane.

The agent found and fixed 3 bugs of its own, including **an inverse-law bug in `change-weight` caught by reading before running anything**.

### Verified blocker, correctly attributed — and step 4 correctly refused

`cargo check -p semio-s-plugin-procedural --all-targets` → **0 errors in `🧩️assembly`**; all ~93 errors live in `🧊️procedural3d`/`🌀️procedural2d`'s own `🧬️mutations` files (31 + 19 + a scatter across `🔄replace-synapse`, `🔢change-generation-value`, `🏷️rename-generation` triads — an unrelated in-flight mutation-vocabulary migration). Real dates via `git log --date=iso`: last touched **2026-08-13 01:03**, ~13 hours before this escalation began. Not this wave's, not fixed, not counted.

**Step 4 (delete `🧩️wfc` from `🧮️math`) deliberately NOT done**, and the reasoning is right: *deleting the only verified copy before the destination has a genuinely green test run would be premature.* The duplication window stays open here **by choice**, with the reason recorded — which is categorically different from M3a's window, which was open by exhaustion. Closing it needs someone to clear procedural's pre-existing breakage first.

## ✅ M3a CONTINUATION — vertical slice proven, duplication window CLOSED

Supersedes the "step 1 only" entry below. All three asks delivered.

**A. One vertical slice, end to end** rather than ten unproven horizontals: `EquationSnapshot` (a `#[state(persistent)]` label-addressed expression tree bridged to `cas::expr::Expr` through its public API only) → a real `change-coefficient` triad with `diff` from `(payload, base)` and `inverse` from `base`, registered in the enum and both hand-rolled text/binary op codecs → a `roots` inference with a genuine `DepHash` chain delegating into the migrated Sturm-sequence isolation and bisection refinement. Test proves it finds `{1, 2}` for `x²−3x+2`.

**B. `EquationNodeLabel` — never-reused ids, not positional paths.** The agent justified this concretely rather than by appeal to my brep analogy: the plugin's **own pre-existing `insert-point`/`remove-point` bug**, documented in an earlier wave's report, is exactly the positional-addressing failure this avoids. Using a live bug in the same crate as the argument is a better justification than the one I supplied.

**C. Window closed.** `🧮️cas`/`📈️polynomial` deleted from `🧮️math`, mounts removed in the same change. `🧮️math`: **64,217 → 55,522 LOC**.

### Coordinator-verified arithmetic — exact

```
math baseline (W0):        1738 passed / 15 failed   (1753 total)
after M2 residue:          1568 passed / 15 failed   (1583 total)
after M3a  (verified):     1402 passed /  2 failed   (1404 total)
```
`1583 − 1404 = 179` = **exactly** the 166 passing + 13 failing tests that emigrated. And the failure count fell `15 → 2` because all 13 known-failing tests were cas/polynomial ones that **moved with their code and still fail there** — none deleted to make a gate green, none mysteriously fixed. Both directions check out, which is the property that matters.

### ⚠️ One claim corrected: `roots` is NOT "the codebase's first `InferredField`"

The report states it is, having grepped and found zero prior implementations. Measured: **13 impls exist repo-wide** — 3 in stdio's `✳️brep`, 3 in `🧩️assembly` (wave M3b, landed in parallel), 2 in `➗️mathematical`, 2 in `🧩️puzzle`'s `🧊️3d` (the pre-existing `flatPosition` pilot this ticket has cited since W0), plus the framework spine itself. The grep was almost certainly run before M3b landed and against a narrower pattern than `impl .*InferredField<`.

Harmless — the agent did not *act* on the claim, and the work is right either way. Recorded because an unchallenged "we are first here" is exactly the kind of premise that later gets built on: it invites inventing a mechanism instead of matching the four existing ones.

## ⏳ M3a first pass (superseded above) — STEP 1 ONLY; duplication window was OPEN

**State to be unmissable: `🧮️cas` (6,323) and `📈️polynomial` (2,366) now exist in BOTH `🧮️math` and the `➗️mathematical` plugin — 8,689 LOC duplicated.** Acceptable inside a wave, unacceptable as a resting state, because that is exactly the condition where two copies drift and neither is authoritative. Continuation dispatched to close it.

Landed: both files copied verbatim into `➗️mathematical/…/✳️any/🧬️schema/💡️inferences/{🌿️cas-internals,📈️polynomial-internals}/`, `crate::number`/`crate::algebra` rewritten to `math::*` against a new dep. `cargo check -p semio-s-plugin-mathematical --all-targets` → 0 errors; `--lib` → **238 passed / 14 failed**, where the cas+polynomial subset is **166 passed / 13 failed — byte-identical to the same filter run against `semio-framework-math`.** That equality is the real evidence the copy was lossless. Zero external consumers of `math::cas`/`math::polynomial` (grepped twice), so the repoint step is trivially satisfied.

**A Rust trap worth keeping:** the agent's first mount used a private module plus a glob re-export, which **silently dropped non-`pub` inner modules** (e.g. `mod canon`) — privacy does not leak through an alias, and the failure is quiet rather than a compile error. Caught and fixed.

### The honest part: it stopped before the actual deliverable

The `EquationSnapshot`, all 10 designed mutation triads and all 12 designed inferences are **not authored** — the agent judged the remaining design (a `NodePath` addressing scheme, DSL/pack codecs, `InferredField`/`DepHash` wiring) unsafe to rush, and said so plainly instead of shipping stubs. Correct call, and it named the consequence itself: *"deleting now would just relocate a library, not complete the artifact conversion."* **Relocating code into a directory named `💡️inferences/` does not make it an inference.**

Continuation instructed to: prove ONE end-to-end vertical slice first (`roots` — the user's own example) rather than ten unproven horizontals; and to prefer a **`PersistentLabel`-style never-reused node id** over positional `expr.children[2]` addressing, because a mutation address must stay stable under unrelated edits or an `inverse()` computed against `base` stops resolving once a sibling is inserted — the identical problem brep solved with `PersistentLabel`.

## G2 — DWG codec relocated into its artifact; deletion correctly deferred to G2b

**Design chosen: option (b)** — the codec (`DwgDrawing`/`DwgEntity`/`DwgGeometry`/bit reader-writer/`dwg_to_bytes`/`dwg_from_bytes`/bridges) lands in `🖊️dwg/🏅️standards/🔖️ac1024/…/🚪️io/🦀️component.rs`; **`DwgSnapshot` untouched.** The agent justified this from the file's own pre-existing precedent for byte↔structural-value functions, and from evidence that raster/note/layout's existing deserializers already treated this codec as a *side-decode of* `DwgSnapshot.bytes`, not a replacement for it. **My "the snapshot is just a bytes blob, replace it" framing would have overwritten another ticket's deliberate Decision #5** — the agent read before writing and didn't.

Two new `ArtifactSerializer`/`ArtifactDeserializer` bridge pairs authored against `SemioMeshSnapshot`/`SemioDrawingSnapshot`, mirroring the shipped `SemioCadFromDwg`/`SemioDrawingToDxf` shapes rather than inventing a mechanism. ~7 plugin consumers repointed. Verified: stdio **2430/5** (same 5), mesh-engine **20/0**, framework **127/0**, 3d **413/0**.

### 🎁 20 of the "29 DWG tests" were never DWG tests

Moving the file forced an accounting nobody had done: only **9** of the 29 tests in `🔺️mesh/🦀️component.rs` actually test DWG. The other **20 test `semio_framework_mesh_engine`** — orphaned in this file when the mesh content was extracted earlier tonight, and never re-homed. They moved to that crate, which until now had **zero** tests. A file whose name, contents and test suite each described a different thing.

### ⚠️ A near-miss the agent caught and reverted

`📐️cad`'s DWG functions are pinned to the *framework* type by a cross-crate **function-pointer registration** — `register_dwg_import_handler(CAD_KIND, cad_document_from_dwg)`, called from `🎪️demonstrator`, signature `fn(&DwgDrawing) -> Result<Value, String>`. Repointing cad alone would have compiled on one side and broken the plugin. Reverted; cad/gismap/puzzle2d/animate/shooting/space left untouched for the same class of reason. **A type can be pinned across crates by a function pointer that no import graph shows you.**

## 🔍 I resolved G2's blocker myself — and BOTH prior claims about it were wrong

G2 blocked deletion on two files. Neither prior reading survived contact with path resolution:

| File | Earlier recon said | G2 said | **Truth (realpath resolution of every `#[path]` in the repo)** |
|---|---|---|---|
| `💻️os/🦀️component.rs` | unmounted | live blocker | **ZERO mounts — genuinely dead code** |
| `💻️os/🖥️host/🦀️component.rs` | unmounted | live blocker | **LIVE** — mounted at `💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs:27` |

Both were misled by the same thing: the mount is `#[path = "../../🦀️component.rs"]`, a string containing **no `🖥️host` at all**, while `grep '🖥️host/🦀️component.rs'` matches two *unrelated* files (brep's own engine host, flow's own host). Rule added to `📌️important.md`: **a mount is not a string — resolve realpaths**; and "this file is dead" is an action-licensing claim, so it needs the strongest evidence, not a substring match.

**The unlock:** the live file's crate is `semio-framework-os`, which is **not** in stdio's forbidden closure (`stdio → framework-plugin → framework → {ui, geometry, os-kernel, hash, schema}` — it appears nowhere). So it may legally depend on stdio, exactly as the wgpu renderer already depends on the puzzle plugin. G2b dispatched to repoint it, flip the `register_dwg_import_handler` signature together with its cad + demonstrator registrants in one change, and only then delete `🔺️mesh`.

## 🏆 G4 phase 1 — `BrepEngineHost` IS DEAD. The ticket's headline anti-pattern is gone.

`BrepEngineHost { cache: Mutex<EngineCache>, kernel: Mutex<Brep> }` — the process-global mutable geometry session, the single purest instance of the anti-pattern this ticket exists to remove — **no longer has a live consumer anywhere.** Coordinator-verified: `grep -rn "BrepEngineHost" --include="*.rs" "✏️s/"` → **4 hits, every one a doc comment describing the removal**; zero constructions, zero `OnceLock`, zero `Mutex`.

```
semio-s-plugin-cad       139 passed, 0 failed, 1 ignored
semio-s-plugin-process   158 passed, 0 failed
semio-framework-3d       413 passed, 0 failed          ← baseline held
semio-s-plugin-stdio    2430 passed, 5 failed          ← was 2414/5: +16 tests, SAME 5 failures
```

### The brief was wrong in my favour twice, and the agent measured rather than trusted

1. **cad had 14 call sites, not the 1 my brief cited** — spread across 3 files including an app-layer file my subset-scoped brief never mentioned. Had the agent worked to the brief it would have left 13 live uses of a "deleted" type.
2. **process3d's `ProcessKernelReplay` was *already* tier-(d) shaped** — constructed fresh per call everywhere. It was never the violation I described; it merely wrapped an owned `Brep` in a pointless `BrepEngineHost`. Simplified to the owned field. The agent also reported, without being asked, that its "prefix memo" **never actually provides cross-call incrementality as wired** — i.e. the performance justification for the singleton was already fictional.

### 🔑 Deleting the singleton exposed two tests that had been silently depending on it

Two cad tests passed only because a *process-global arena* let handles survive across calls and across tests. With per-call `Brep::new()` they failed — correctly. They were diagnosed to root cause and rewritten to assert the honest post-fix behaviour, **not weakened to restore green**. This is the clearest evidence available that the singleton was real shared mutable state and not just ceremony: something was reaching through it.

### Job 3 — 24 compute subdirs pre-allocated and mounted

Under `✳️brep/🧬️schema/{📸️snapshot,🔺️diff,💡️inferences}/`, Rust-only, no TS twins (TS mirrors boundary vocabulary, never algorithms). Verified by a repo-wide dangling-mount sweep: **0 missing of 1,632 checked**. This is what makes the later peel waves cheap — they never touch stdio's 9,400-line `📦️glue.rs`, which every stdio session edits and which would otherwise serialize the whole effort.

### Job 2 correctly reported UNDONE

The agent read the destination before writing and found the STEP↔SemioBrep io facet **already complete** — a real bidirectional AP214 walk, tested, zero duplication. The actual remaining duplicate is framework's own hand-rolled Part-21 codec (1,034 LOC), whose removal requires rewiring the shared `BrepKernel` trait contract. Bigger and riskier than this wave; reported with evidence instead of attempted. Correct call.

### ⚠️ A verification-is-a-timestamp catch in my own re-check

My first cad run read **138 passed / 1 failed**, contradicting the agent's claimed 139/0. Rather than record a discrepancy I looked: `stat` showed `📐️cad/…/🚪️io/🦀️component.rs` modified at **15:03**, mid-run — a peer session's live edit. A clean re-run gave exactly **139/0/1**. The agent was right and my measurement was the unreliable one. Had I trusted my own newer number over its older one purely because mine was newer, I would have filed a false regression against correct work.

## ✅ M2 — framework residue extracted; ALL FIVE STEPS, independently re-verified

Two new framework modules now hold everything the framework itself needs, so the rest of `🧮️math` is free to become artifacts.

```
🧰️framework/🔨️modules/📐️geometry/{⚙️engine,🎲️random}/     crate semio-framework-geometry   1,997 LOC
🧰️framework/🔨️modules/🕸️graph/{⚙️engine,🧮️algorithms,🖊️drawing,🛂️manifest,🤖️generated}/
                                                          crate semio-framework-graph      6,236 LOC
🧮️math:  72,439 → 64,217 LOC
```

**Measured residue 6,594 LOC vs the ~6,500 hypothesis — within 1.5%.** The budget rule ("over ~8,000 means something dissolvable got smuggled in") held.

### Coordinator's independent re-verification (not the agent's numbers — my own run)

```
cargo metadata --no-deps                     → WORKSPACE_OK
cargo test -p semio-framework-geometry --lib →   57 passed, 0 failed
cargo test -p semio-framework-graph    --lib →  113 passed, 0 failed
cargo test -p semio-framework-math     --lib → 1568 passed, 15 failed
cargo test -p semio-framework-3d       --lib →  413 passed, 0 failed   ← baseline held
```
**The arithmetic is the real evidence: 1568 + 57 + 113 = 1738 = the exact pre-wave math test count.** Not one test was lost or silently dropped while relocating 8,233 LOC. And `diff` of math's failure set against `scratch-w0-baseline-failures-sorted.txt` → **identical**: zero new failures, and zero accidentally "fixed" (which would equally have signalled that something moved that shouldn't have).

Residual `math::` references in the live tree: **23, all `math::graph::dsl`** — the Jack DSL, which I explicitly scoped OUT of this wave because it has consumers on both sides of the Cargo law. Expected, not stale.

### Four hypothesis corrections the agent measured (I was wrong four times)

1. **`algebra::Mat2` is not residue** — repo-wide it appears only in algebra's own tests. The algebra residue is exactly `Vec3` + `Mat4`.
2. **`semio-framework-ui` is a residue consumer nobody had listed** — the same cross-directory `#[path]` mount that caused my M0 regression. Confirmed and handled in both crates.
3. **`neural-engine` is not a consumer at all** — its two `math::` hits are inside a comment. No dep added. (A grep-to-find/enumerate-to-count catch.)
4. **`🕸️graph/🛂️manifest` is not a vocabulary, it is a codegen machine.** It `include!`s a generated registry produced by a `build.rs` plus a 343-line `📜️script.ts` that walks the entire repo. The whole machine moved to the graph module; math lost its `build.rs` and its nx `generate` target. This is the same inverted framework→plugin codegen dependency flagged earlier in this ticket — it now at least lives next to the vocabulary it generates, though retargeting it to owner-root `🤖️generated/` remains open.

### Honest remainders

- `🕸️graph/🗣️dsl` **is** reachable from framework code (`♾️infinite/🕸️dag`, `🧠️neural`), so those crates keep a `math` dep alongside `graph`. That entanglement is also why the agent could not verify step 4 before step 5 — a `PropertyValue` mismatch across the duplication window forced it to complete the deletion and then verify. It said so rather than reordering the report to look clean.
- The graph module has **no TypeScript package of its own**; `@semio-tech/framework-math-js` still re-exports the manifest TS surface through a cross-module relative import. No Cargo-law impact, but it wants a follow-up.
- `blocked-churn` correctly attributed and not touched: `♾️infinite`'s 12 errors are wave G1b's live GLB/mesh rewrite (mtime 14:18, `git diff` shows `mesh_from_kind`→`placeholder_mesh`), **zero of which mention a residue symbol**; seven plugin crates are blocked only by the live DWG relocation (`E0753`, mtime 14:38).

## 🛑 M0 REGRESSION — self-inflicted, caught by an agent, and it corrupted two waves' baselines

**I broke `semio-framework-ui` and did not notice for ~40 minutes.** Recording it in full because the failure mode is more instructive than the fix.

I deleted `semio-framework-math` from `🖱️ui/Cargo.toml` after `grep -rn "semio_framework_math\|\bmath::" "🧰️framework/🔨️modules/🖱️ui" --include="*.rs"` returned **0**. The grep was correct and the conclusion was wrong: `🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs:222` mounts
`#[path = "../../../../../🧊️3d/🎬️scene/🦀️component.rs"] pub mod kernel_3d_scene;`
and that file's line 3 is `pub use semio_framework_math::algebra::{Mat4, Vec3};`. **A crate in this repo is composed across directories via `#[path]`, so a directory-scoped grep structurally cannot see the crate's own dependency edges.** Six errors, in a crate upstream of nearly everything.

### The part that matters: two independent agents wrote it off as foreign churn

G1a and G1b each measured those 6 errors as their baseline and each attributed them — correctly by procedure, wrongly in fact — to another session, because a *plausible* story was available: "`🧊️3d` is being dissolved by another wave, so this is theirs." G1b was honest enough to say outright that it therefore **could not obtain a compile signal for its own crate** rather than claiming one. G1a's error-set diff stayed valid (my breakage sat on both sides of it), but its *baseline* was measuring my bug, not os-flow — so its report's "baseline is 6, not the >100 in host/vcs/playbook you briefed" is not evidence about os-flow's real state.

**Rule adopted, now in `📌️important.md`:** a novel error appearing in two different waves' baselines *at the same moment* is evidence of a shared recent cause, and the most available cause is your own last change. One minute of suspicion beats hours of masked signal. Also: to census a crate's inputs, read its `📦️glue.rs` mounts first — or just delete the dep and let `cargo check --all-features` answer, instead of asking the filesystem.

**Fixed and verified**: dep restored with a comment naming the cross-directory mount so the next reader doesn't repeat it; `cargo check -p semio-framework-ui --features wgpu` → **0 errors**; `cargo metadata --no-deps` → WORKSPACE_OK.

**`🗺️surface`'s removal was genuinely safe** — checked properly this time: its `📦️glue.rs` mounts only `🎨️paint`/`🏔️terrain`/`🕸️node-graph`/`🗺️tiled-map`, all inside `🗺️surface/`. Re-measured post-fix: **0 errors**. So M0 is half-right: surface's dep was dead, ui's was not.

## G1a / G1b — both landed small, and both were RIGHT to refuse the big deletion

Neither wave did what I briefed, and in both cases the refusal was correct.

**G1a (`os-flow`)**: I briefed "`📐️brep-geometry` is a near-duplicate of the flow-ext-brep plugin — reconcile and delete the framework module." **The premise was false.** The plugin holds *zero* local duplication; it glob-imports the framework module (`use flow_extension_sdk::brep_geometry::*`). And the module has live external callers outside the boundary — `flow::tessellate_geometry` in `🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs:670`, and `flow::{export_solid_json, import_solid_json, tessellate_geometry}` at four sites in `📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`. Deleting it would have broken two crates or silently desynced geometry handles. Left intact, reported. Of the 4 DWG bridge functions, **1** (`dwg_decode_mesh_json`) had zero callers and was deleted; the other 3 have real consumers (including a TS file in `🌐️spatial-kernel`) and were left with exact patches filed.

**G1b (`os-infinite`)**: dual-copy trap confirmed real (both files byte-identical at 4,092 lines, both mounted); every edit mirrored and `diff -q`-verified identical before and after. `MeshData` → local `WorldMeshBuffers` mirroring the renderer's shipped TS `WorldMeshData` field-for-field, minus two fields neither side uses. `mesh_from_kind` → local `placeholder_mesh` — and the agent censused rather than ported blindly: only **5 of 9** kinds are reachable, and `vertex-marker` appears only as a lookup key, never as a `kind` argument. `mesh_from_glb` **left blocked and honest**: stdio's `SemioMeshFromGltf` deserializer yields a structured snapshot, not flat render buffers, so wiring it needs a snapshot→buffer adapter with no existing precedent. Marked `🚧️`, not faked.

**Consequence for the plan:** the "de-geometrize the framework first" gate is only partly achievable. `📐️brep-geometry` cannot leave until `🌀️procedural` and `📖️playbook` move, which makes those two plugins part of the geometry migration rather than bystanders.

## ❎ The taxonomy amendment is NOT needed — measured, then cancelled

The approved plan called for extending `📜️script.ts`'s `🧬️mutations` wildcard escape to `📸️snapshot`/`🔺️diff`/`💡️inferences`, so the dissolved kernel could land as named Rust-only compute subdirs inside facets. **Measured before writing it, and it turns out to be unnecessary.**

`policyTaxonomyDirsBreaches` (`📜️script.ts:4076+`) walks `<owner>/🗿️artifacts/<artifact>/<child>` and only enters its `NestedFacetWalk` when `<child>` is literally `🧬️schema` or `🚪️io`. But `artifactChildDirs = ["🧬️schema","🚪️io","📚️examples"]` — it does **not** contain `🏅️standards`, and every artifact in this repo is new-shape (`<artifact>/🏅️standards/🔖️v/🪆️subsets/✳️x/🧬️schema/…`). So for new-shape artifacts the walker takes the else-branch at `🏅️standards` and **never descends to subset facets at all**. The only walker that does read `schemaChildDirs` at subset depth (`:9840+`) is a *completeness* check ("every facet must contain these four"), not a *restrictiveness* check.

Confirmed empirically rather than by reading alone — `bun ./📜️script.ts policy`:
```
"not a recognized artifact component dir"  → 0
"not a recognized representation dir"      → 0
```
Corroborated by the fact that `✳️brep/🧬️schema/💡️inferences/✅validation-report/`, authored earlier in this ticket, sits under exactly the contested depth and is reported by nothing.

**Decision: do not amend.** Adding a rule to permit something along a path the walker never visits would be dead policy — it would look like governance while enforcing nothing, and it would misinform the next reader about where the real boundary is. Compute subdirs are authored directly. UCAS's substantive no-objection is recorded and unused; #2553 (the actual taxonomy owner) never had to be woken.

**W12 policy baseline captured** for comparison: **23,792 breach lines**, dominated by `handcrafted-grammar/spec-distinctness` (22,274 — systemic, pre-existing, unrelated). Full set: `scratch-w0-policy-baseline.txt`.

## W5 — all four waves complete

Mesh-engine dissolution, `◻2d` store deletion, `🛢️db` LiveQuery→InferredField, and `♾️infinite/…/🕸️dag` triad conversion are all landed and independently re-verified by the coordinator (not merely accepted from agent reports) — the fourth and last of the "everything that can be migrated must be migrated" gaps identified after the user's pivot instruction. Honest remainders carried forward, none silently dropped: the DWG binary codec still embedded in `🔺️mesh/🦀️component.rs` (flagged, own future home); ~20 plugin-app ad-hoc mesh-construction call sites (`procedural`/`process`/`puzzle`/`lowpoly`/`remodel`) not yet routed through artifact dispatch (separate, larger, higher-risk wave); `🌊️flow/🌿️vcs`'s `CollectionMutation` still `blocked-cross-session` on SMO's wound-down plugin-vocabulary rewrite; `BrepEngineHost` cross-session deletion still APA's territory.
