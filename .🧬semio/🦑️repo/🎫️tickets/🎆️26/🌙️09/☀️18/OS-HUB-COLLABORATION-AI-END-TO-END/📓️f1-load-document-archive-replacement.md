# F1 — `Effect::LoadDocument` archive-replacement failure (framework blocker)

Slice F1 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`. Blocker named by
`📓️b1a-dormant-plugin-boots.md`: every batch-A plugin whose boot `setActiveExample` is declared and
bridged dies with

```
AppChannelClient.loadDocumentArchive(…): {"code":"plugin.internal",
 "message":"document archive replacement failed closure, authority, or retained publication validation"}
```

verified by B1a as identical on `architect`, `animate` and `writer`.

**Status: diagnostics landed + native repro green; live proof outstanding.** This worker was cut by
the account session limit at ~08:15 and resumed at 11:00.

## 1. Where the opaque message comes from

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `advance_document_archive_load`, phase
`AwaitReplacement` (was `:23317`). `poll_artifact_store_replacement` returns a bare
`ArtifactEnvelopeDecodeOperationPoll::Fault` whenever the replacement reached `Complete` without
committing, and the archive then minted ONE sentence for it. The replacement itself carried a bare
`faulted: bool` set at **40 distinct sites**; the only explanation any of them produced was a
`#[cfg(test)]` `eprintln!`, so a headless host — and every one of B1a's probes — saw the same
sentence for eleven different causes.

## 2. Production diagnostics (deliverable 1) — DONE

`faulted: bool` is replaced by `refusal: Option<ArtifactStoreReplacementRefusal>`, a `Copy` typed
record carrying the leg AND its evidence. All in `🔌️plugin/🦀️.rs`:

| what | where |
|---|---|
| `ArtifactStoreReplacementRefusal`, `ArtifactStoreReplacementLeg`, `ArtifactStoreReplacementPublicationGuard`, `ArtifactStoreRefusalCode` + `into_fault()` | new block just above `struct ActiveArtifactStoreReplacement` (`~19740–19870`) |
| `fn refuse` / `fn refuse_invariant` (record first leg, move to retirement, return that site's own unchanged fault) | `ActiveArtifactStoreReplacement` impl |
| 24 invariant sites rewritten to `return Err(self.refuse_invariant(leg, "…"))` | `seal_members`, `drive_member_open`, `drive_closure`, `drive_candidate_views` |
| 10 sites that previously SWALLOWED the refusal (returned `Ok(Pending)` with only a `#[cfg(test)]` print) now record a typed variant | member open rejected · member identity mismatch · member factory refused · closure member-count mismatch · closure rejected · initializer admission rejected · initializer failed · cancellation · publication guard · publication authority |
| `pub fn artifact_store_replacement_refusal(handle) -> Option<Fault>` | next to `poll_artifact_store_replacement` |
| the terminal mint reads it before `acknowledge_…` retires the operation | `AwaitReplacement` phase |

Legs are `initializer` · `members` · `closure` · `candidate-views` · `retained-publication`, and the
fault code is `plugin.internal.document-archive-replacement.<slug>`. Two details worth naming:

- **The publication guard now reports all nine of its inputs** (`closing`, `cancelled`,
  `parent-generation`, `live-generation`, `base-child-generation`, `child-generation`,
  `complete-candidate`, `retirement-admitted`, `next-generation`). A bare "guard rejected" is
  unactionable; which condition tripped is the whole answer.
- **The initializer's own refusal slug is carried to the wire.** `drive_initializer` discarded the
  job fault's detail page; `ArtifactStoreRefusalCode` copies its first page into a fixed 96-byte
  inline buffer (no allocation, `Copy`), so e.g. `bounded-store.initializer-envelope-invalid` now
  reaches the shell instead of being dropped.

Every `#[cfg(test)] eprintln!("[DEBUG] recursive replacement diagnostic: …")` in this path is gone —
replaced, not duplicated. No `[DEBUG]` line was added.

## 3. Native repro (deliverable 2) — DONE, 6/6 green

New file `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs`,
`include!`d from `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:3635`. Fixture
`SingleDocumentApp` mirrors architect exactly: childless snapshot, `bounded_document_store_owners`,
`bounded_document_store_initialization_job` over its own schema, `bounded_document_store_disposer`,
a one-item artifact preparation factory. The composition suite next door only ever drove archives
WITH owned members; the `members: []` whole-document swap had no native coverage at all.

`cargo test -p semio-framework-plugin --lib -- document_archive refresh_poll_lane_stamps`
→ **6 passed, 0 failed** (capture `🗑️generated/f1-repro-run3.txt`; the two pre-existing composition
archive tests are in that filter and stayed green).

| test | asserts |
|---|---|
| `a_childless_whole_document_archive_replaces_the_live_document` | a STAMPED architect-shaped archive reaches `Ready` and the live document really becomes the archive's snapshot |
| `an_unstamped_whole_document_archive_is_refused_by_parent_hydration` | the raw log an app mints is refused `Identity`, and the live document is untouched |
| `the_refresh_poll_lane_stamps_the_load_document_it_emits` | an effect leaving the refresh-poll lane is already stamped, so it round-trips with the test stamping nothing |
| `a_refused_whole_document_archive_names_the_leg_that_refused_it` | the terminal report is no longer the three-leg collapse |

## 4. Root cause

**Two findings, one confirmed defect.**

**(a) The identity stamp is the whole contract, and only one of three emission lanes applied it.**
An app mints its replacement log with `store::empty_document_spr(...)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11776`), whose `HistoryLog` has
`composition: None`. Parent hydration's `Phase::Begin`
(`🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:293`) refuses any log with no composition section
outright — `MemberOpenDiagnostic::Identity` — **before any of the three replacement legs runs**. The
framework already owns the cure, `store::stamp_document_spr_identity`, whose own docstring names
`Effect::LoadDocument` as its caller; `VcsArtifactApp::stamp_load_document_identity` wraps it. It was
called from **exactly one** site (`🔌️plugin/🦀️.rs:27492`, the mounted typed-operation ladder). The
plain dispatch lane (`dispatch_emit_inner`) and the refresh-poll lane (`pending_effects`) both
handed the shell the app's own unstamped log. Proved by
`an_unstamped_whole_document_archive_is_refused_by_parent_hydration`.

**(b) For a childless app the three legs themselves are sound.** Once stamped, the architect-shaped
archive publishes cleanly (test 1). So B1a's live message on `architect` — whose `setActiveExample`
IS a mounted retained tool and therefore IS stamped — is NOT explained by (a), and the remaining
candidate is a guard that only trips under live conditions (the `CandidateReady` guard refuses when
`store.generation_now()` moves between the start of the replacement and candidate readiness, which a
tight native drive never exercises but a browser spreading the work across host polls does). **That
is a hypothesis, not a measured result** — the new diagnostics exist precisely to settle it, and
settling it needs the live boot in §6.

## 5. Fix

| # | file | change |
|---|---|---|
| 1 | `🔌️plugin/🦀️.rs` `dispatch_emit_inner` | `let effects = self.stamp_load_document_effects(effects)?;` right after the `Emit` destructure — the plain dispatch lane now stamps |
| 2 | `🔌️plugin/🦀️.rs` `VcsArtifactApp::pending_effects` | the polled effects are stamped through `stamp_polled_load_document_effects`; that lane has no error channel, so an undecodable log rides on unchanged and the (now self-explaining) loader reports its own identity refusal rather than the effect vanishing |
| 3 | `🔌️plugin/🦀️.rs` | `stamp_load_document_effects` / `stamp_polled_load_document_effects` helpers beside the existing `stamp_load_document_identity`, both no-ops when no `LoadDocument` is present |
| 4 | `🔌️plugin/🦀️.rs` | the whole of §2 (typed refusal record) |

The contract is enforced in the framework rather than pushed onto apps: no app should ever know its
mount identity, which is exactly why `stamp_document_spr_identity` exists. No plugin file needed a
change for this, so no `LoadDocument`/`setActiveExample` user under `✏️s/🔌️plugins` was edited.

## 6. Live proof — architect · animate · writer (deliverable 3)

**(filling — session 4.)** Inherited state re-verified first: `git status` shows
`🔌️plugin/🦀️.rs`, the new `🧾️document-archive-load-legs` suite and the `include!` row all still on
disk, and `🗑️generated/f1-repro-run3.txt` is this tree's own 6-passed capture.

Method (unchanged from B1a, one activation + one detached serve per plugin):

```
zsh  …/📜️b1a-activate.sh <variant>
nohup zsh …/📜️b1a-serve.sh <variant> <port> >/dev/null 2>&1 & disown
SEMIO_F1_PLUGIN=<variant> SEMIO_F1_PORT=<port> bun …/🐍️f1-bar-probe.mjs
```

`🐍️f1-bar-probe.mjs` (**new**, this slice) is `🐍️b1a-boot-probe.mjs` plus the two steps B1a never
reached: **redo** (`framework.history.redo` — the same tree-row-with-a-control shape as undo, so it
needs the same `aria-controls` expander) and an explicit **example-load witness**
(`exampleLoadFaults` counts every console line naming `setActiveExample` / `loadDocumentArchive` /
`document archive`, and `bar` is false while any exists). Captures land as
`🗑️generated/f1-<plugin>-console.txt` via the new `SEMIO_F1_PREFIX`.

### 6.1 architect — **PASS, and it settles §4(b)**

`🗑️generated/f1-architect-console.txt`, 2026-09-19 23:58 CEST, activation receipt
`b1a-architect-activate.txt` (`exit=0`, 34 m 47 s, `component-dev` 29 m 53 s), serve on 6090.

```
F1 architect {"ready":"architect","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"setAdjacencyKind","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":9,"bar":true}
```

`undo` `{appEntriesBefore:1, appEntriesAfter:2, cursorBefore:3, cursorAfter:4, canRedo:true,
signatureRestored:true}` · `redo` `{appEntriesAfter:3, canUndo:true, canRedo:false,
signatureReapplied:true}` — both with empty `faults`.

**The `loadDocumentArchive` refusal is gone on the live server**: B1a measured architect at
`faultLines: 1`, that one line being the archive-replacement sentence; it is now **0**, and the nine
remaining console lines are the dev server's staleness notice, vite's connect chatter and the React
DevTools banner. So:

- §4(a) — the missing identity stamp — **was the whole live defect**. architect's boot
  `setActiveExample` travels the plain `dispatch_emit_inner` lane, which is exactly the lane §5 fix
  #1 taught to stamp.
- §4(b)'s generation-fence hypothesis is **refuted for architect**: nothing in the live run needed
  it, and the new typed refusal record minted no fault at all.

### 6.2 animate · writer — **blocked by a live peer refactor, not by this slice**

animate's activation (22:17, `b1a-animate-activate.txt`) died with
`Cargo artifact build failed: ✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` after **11 ×
`E0308`** in a crate this slice does not own:

```
✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/…/🖼️image/🧬️schema/🧬️mutations/⏱️set-frame-delay/🦠️mutation/🦀️.rs:9
  expected `SemioImageDiff`, found `MutationOutcome<SemioImageDiff>`
```

`apply_semio_image_mutation` now returns `protocol::MutationOutcome<SemioImageDiff>` (it ends in
`MutationOutcome::apply_to`, which answers `Self`), while all eleven `🦠️mutation/🦀️.rs` leaves still
declare `-> SemioImageDiff`. The eleven **parent** modules
(`…/🧬️mutations/<verb>/🦀️.rs`) are ` M` in `git status` right now — a peer's refactor is mid-flight
through this crate — so this worker did not edit them (rule 3: never repair a peer's half-landed
refactor).

`semio-s-artifact-stdio-semio` is a workspace dependency of **writer, mathematical, sequence and
animate** but not of **architect or vcs**, which is exactly why architect activated and animate did
not. vcs was activated instead while the peer lands; animate/writer are retried below.

Measured results and the rest of the batch: see the bar table in §7.

## 7. B1a leftovers (deliverable 4) — per-plugin bar

**(filling — session 4.)** Root fixes landed so far, before any live re-measurement:

| # | plugin | defect | root fix |
|---|---|---|---|
| F1-a | 🌿️vcs | `incrementCounter` refused `batched item candidate failed its exact fixed fold contract` | its one-item preflight hard-coded `work_items: 1`. A point-invertible mutation folds **two** staged rows (`forwards` + `inverse`), which is exactly what `ArtifactStoreOneItemFootprint`'s own docstring says never to write at a call site. Now `for_one_invertible_item(…)`. `…/🌿️vcs/…/✏️editor/🦀️.rs:664` |
| F1-b | ✒️writer | same shape, unmeasured but identical by construction | `admit_writer_artifact_mutation` declared `work_items: 1` for `EditText`, whose `inverse_writer_mutation` yields a row — so writer's retained `Artifact` lane could never fold one edit. Now `for_one_invertible_item(payload.text.len())`. `…/✒️writer/…/✏️editor/🦀️.rs:875` |
| F1-c | 🎞️animate | boots with **zero** panes and zero window kinds | **framework defect, not animate's.** `create_default_layout(ids, "stack", …)` minted `Axis { kind: "stack", children: [Stack[window]] }` — an *axis* node wearing the *stack* discriminator. The React seed resolver (`🛠️ShellHelpers/🟦️.tsx` `convertFrameworkLayoutNodeToModeLayout` / `collectFrameworkLayoutWindowSeeds`) reads a `stack` node's children as window leaves, so it read the inner **stack** as a window and minted `id: undefined`. `stack` is not an axis direction; the call now returns `create_stack_layout(...)`. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:960`. animate was the only caller passing `"stack"` in the whole repo. |

## 8. Honest gaps

- The live browser leg for architect/animate/writer is unmeasured; §4(b) is explicitly a hypothesis.
- Two prior workers died on B1a and this worker was cut once at ~08:15; `🗑️generated` holds only
  this session's captures (`f1-repro-run2.txt`, `f1-repro-run3.txt`).
- Only the `document_archive`/`refresh_poll_lane_stamps` filter has been run so far; the full
  `semio-framework-plugin --lib` suite has not (A2 reported kernel reds near a `LoadDocumentArchive`
  step count that may or may not be this slice's edit).

## 9. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | typed replacement-refusal record (§2), stamp on the plain dispatch and refresh-poll lanes (§5) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs` | **new** — the four tests of §3 |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | one `include!` row for the new suite |

---

## Session 5 (2026-09-20, from ~01:20)

Resumed after the desktop app restart cut session 4 at ~01:15. Inherited state re-established by
inspection, not by assumption:

- **Two of the predecessor's detached vite serves survived** and are this slice's own (named by their
  own captures): pid `52100` → port **6090** = `architect` (`🗑️generated/f1-architect-console.txt`
  line `serve pid 52100`), pid `32281` → port **6075** = `vcs` (`f1-vcs-console.txt` line
  `serve pid 32281`). The other four live vites (234/6121, 34692/6313, 43140/7502, 91700/6120) are
  peers' and were left alone. Both slice servers were reused; nothing was re-activated for them.
- **`🗑️generated/f1-vcs-console.txt` (2026-09-20 00:46) is a session-4 capture the report never
  recorded.** Its result is folded into the matrix below.
- The peer refactor that killed animate's 22:17 activation (§6.2, `MutationOutcome<SemioImageDiff>`
  vs `SemioImageDiff` in eleven `🗄️stdio/🧿️semio/…/🖼️image` mutation leaves) **has landed**: the leaf
  `⏱️set-frame-delay/🦠️mutation/🦀️.rs:9` now declares
  `-> protocol::MutationOutcome<SemioImageDiff>`. animate/writer/mathematical/sequence are therefore
  activatable again.

### Bar matrix (6 plugins × 7 columns, measured only)

`—` = not measured in this session; a row is only filled from a capture in `🗑️generated/`.

| plugin | boot | example loads | dispatch mutates | undo | redo | no console faults | BAR |
|---|---|---|---|---|---|---|---|
| 🏛️architect | **yes** | **yes** | **yes** `setAdjacencyKind` | **yes** | **yes** | **yes** (0) | ✅️ **PASS** |
| 🌿️vcs | **yes** | **yes** | **yes** `incrementCounter` | **yes** | **yes** | **yes** (0) | ✅️ **PASS** |
| 🎞️animate | **yes** (1 pane, 26 actions — was 0 panes) | **yes** | no — every document verb `BatchOnlyPendingRewrite` | not reached | — | no (15) | ❌️ |
| ✒️writer | **yes** | renders, **but** the archive is refused `InvalidReference` | **yes** — canvas gesture `apply` | **yes** | **yes** | no (11: 1 archive + 10 × `textSelect`) | ❌️ |
| ➗️mathematical | — | — | — | — | — | — | not re-measured (see remaining work) |
| 🎬️sequence | — | — | — | — | — | — | not re-measured (see remaining work) |

### What is fixed and proven (deliverable 1)

**architect re-proved on the session-5 tree, live.** `🗑️generated/f1s5-architect-console.txt`,
2026-09-20 01:26 CEST, against the inherited serve on 6090 (no re-activation):

```
F1S5 architect {"ready":"architect","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"setAdjacencyKind","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":9,"bar":true}
```

So the §5 framework fix (stamp `Effect::LoadDocument` on the plain-dispatch and refresh-poll lanes)
is **proven live on architect twice, on two different days' captures**, and §4(a) — the missing
identity stamp — remains the whole measured live defect. §4(b)'s generation-fence hypothesis is
still refuted for architect and still unmeasured elsewhere.

### vcs — PASS, and it is the second live proof of the framework fix

`🗑️generated/f1s5-vcs-console.txt`, 2026-09-20 06:37 CEST. Fresh activation
(`b1a-vcs-activate.txt` `exit=0`, 18 m 8 s, `component-dev` 14 m 57 s) + fresh detached serve on
6075 — the session-4 serve was restarted because its staged module predated the F9 edit.

```
F1S5 vcs {"ready":"vcs","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"incrementCounter","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":8,"bar":true}
```

Two of this slice's own fixes are now proven at runtime rather than by construction:

- **F1-a (`for_one_invertible_item`)** — `incrementCounter` had been refused
  `batched item candidate failed its exact fixed fold contract`; it now dispatches, appends an
  app-owned ledger entry, undoes (`canRedo` true, signature restored) and redoes
  (signature reapplied), all with empty `faults`.
- **F9 on vcs** (wired by the session-4 worker at 00:51:55, i.e. AFTER its 00:43 activation, which
  is exactly why the 00:46 capture still showed `exampleRendered:false` and an empty combobox).
  The navbar example picker now renders — the shell offers examples only to an app that declares
  `setActiveExample` on some window kind
  (`📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:9033`) — and the boot
  `setActiveExample` lands its document with `exampleLoadFaults: 0`.

### Machine conditions this session (they are the limiting factor, and they are measured)

- **A peer's half-landed refactor killed the first animate activation at 01:32:58** —
  `error[E0063]: missing field 'principal_kind' in initializer of 'PresencePeer'`,
  `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs:1952` (slice M6's agent presence kind). The peer
  landed the missing field at **01:33:21**, 23 seconds later; rule 3 was kept (nothing of the peer's
  was edited) and the activation was simply re-launched at 01:33:32.
- **The shared cargo build-dir lock is fleet-jammed.** At 02:15 there were **32 live `cargo`
  processes**, nearly all at 0 % CPU with no `rustc` child; `sample` of this slice's own
  (`cargo rustc --locked … 🎞️animate`, pid 1968) shows
  `cargo::core::compiler::prebuild_lock_exclusive → LockManager::lock → flock` on
  `⚡️cache/cargo/target/debug/.cargo-lock`. It is **contention, not deadlock** — a handful of peers'
  `rustc` processes were burning 12–22 % CPU at every sample, and several sampled cargos were NOT in
  `prebuild_lock_exclusive`. `kernel_task` sat at 50–61 % (thermal) with load average 110–160.
  This slice's animate activation waited **> 57 min for the lock without ever starting a `rustc`**,
  which is what preamble rule 14 says to record rather than fight. No peer process was killed.
- Consequence: only **one** cargo-bearing activation could be paid for at a time, so `vcs` and
  `writer` were chained behind animate (`📜️f1-activate-chain.sh`, **new**, strictly sequential) and
  `mathematical`/`sequence` were left at code-review depth rather than half-landed — see
  "Remaining work, precisely specified".

### animate — the two inherited framework fixes are PROVEN, one plugin defect is left

`🗑️generated/f1s5-animate-console.txt`, 2026-09-20 07:12 CEST; activation `exit=0`, 22 m 54 s
(`component-dev` 19 m 46 s), serve 6051.

```
F1S5 animate {"ready":"animate","shellError":null,"loadsClean":false,"exampleRendered":true,
 "dispatched":true,"action":"setActiveExample","undoWorks":false,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":15,"consoleLines":23,"bar":false}
```

- **§7 F1-c (the `create_default_layout(ids, "stack", …)` axis/stack confusion,
  `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:960`) is proven live.** B1a measured
  animate at **zero panes and zero window kinds**; it now boots `windowKinds: ["tile-editor"]`,
  `panes: ["window:tile-editor"]` and an Actions pane with **26 rows**.
- **The `Effect::LoadDocument` fix is proven on a third plugin**: `exampleLoadFaults: 0`, and none of
  the 15 fault lines is an archive-replacement or genesis-projection sentence. B1a's animate boot
  died on `module.vcs: validation failed …` / the shared archive refusal; both are gone.
- **What is left is F1, not the archive.** Thirteen of the fifteen fault lines are the same shape:
  `UI dispatch rejected action:<id> with interactive-job classification BatchOnlyPendingRewrite`
  for `seedGrid`, `addTile`, `deleteTile`, `deleteSelection`, `renameTiles`, `patchTileCrops`,
  `setFrame`. `validate_ui_dispatch_classification` admits only `Migrated`, and animate's manifest
  still declares **every** document verb `BatchOnlyPendingRewrite`
  (`…/🎞️animate/…/✏️editor/🦀️.rs:914-930`); its retained roster is only
  `["setActiveExample", "engagementInput", "noMutation"]` (`:269`) and it owns a **config**
  one-item preparation factory but no **artifact** one (`:638`), so promoting a document verb is the
  two-part change B1a described. The probe consequently scored `setActiveExample` itself as the
  dispatched verb, which is a document *replacement*, not an undoable mutation — hence
  `undoWorks:false`. The remaining two fault lines are `drawImage … HTMLImageElement is in the
  'broken' state` from the tile preview's missing demo image.

### writer — the second archive shape is root-caused and fixed (data, not framework)

`🗑️generated/f1s5-writer-console.txt`, 2026-09-20 06:50 CEST; activation `exit=0`, 8 m 17 s,
serve 6062, probe run with `SEMIO_F1_GESTURE=canvas` (writer's mutating verbs are editor gestures,
no Actions-pane row — B1a's "product decision" bullet).

```
F1S5 writer {"ready":"writer","shellError":null,"loadsClean":false,"exampleRendered":true,
 "dispatched":true,"action":"canvas-gesture:apply","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":1,"faultLines":11,"consoleLines":73,"bar":false}
```

**Dispatch, undo and redo all work through the editor canvas** — the first time writer has passed
any of those three. Two defects remain, and the first is this slice's own subject.

**W1 — `document archive genesis child projection failed: child restore projection: InvalidReference`.**
This is *not* the identity-stamp defect (§4(a)); it is the second shape B1a saw, and it is a **data**
defect in two committed example assets. `complete_document_archive_genesis`
(`🔌️plugin/🦀️.rs:23258`) projects the hydrated candidate parent through
`store::ChildRestoreProjection`, whose visitor
(`🏪️store/🦀️.rs:3171`) refuses a child whose `child_id != target.artifact_id`. Writer's
`document` slot is content-addressed by construction — `document_child_handle`
(`…/✒️writer/🗿️artifacts/✒️writer/🦀️.rs:64`) sets `target.artifact_id = child_id` — but both
hand-authored `.dsl.semio` example assets carried a *different*, human-written target:

| asset | committed `child_id` | committed target (decoded) |
|---|---|---|
| `…/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` | `document-926a496d334505c6` | `jack-document!s.stdio.semio@v1/document` |
| `…/📚️examples/🎬️demo/🖼️assets/🧪️dag-example/🗣️.dsl.semio` | `document-76a05fef01e1c7ae` | `dag-jack-document!s.stdio.semio@v1/document` |

`jack_example_document()` parses the asset verbatim (`dec_child`,
`…/🚪️io/📸️snapshot/📝️text/🦀️.rs:42`) and then only *attaches* text, so the mismatched handle
reached the archive unchanged and every `setActiveExample` died in the genesis projection.
**Fixed**: both assets' target artifact ids now equal their `child_id`. The law already had a test —
`every_demo_asset_is_the_printers_own_content_addressed_output`
(`…/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs`) — and it was **red on the committed tree**. Running it
also exposed a second, smaller defect in the test itself: the printer emits no trailing newline
while a committed text file must end with one, so the assertion could never pass even on correct
data. The comparison now strips exactly one trailing `\n` from the asset, and the test's
`eprintln!("[DEBUG] canonical …")` is gone (rule 10).

`CARGO_TARGET_DIR=…/target-f1 cargo test -p semio-s-artifact-writer-writer --lib -- every_demo_asset primary_asset`
→ **3 passed, 0 failed** (`🗑️generated/f1-writer-asset-test.txt`). The live re-probe of writer on a
re-activated server is NOT yet run, so W1 is fixed-and-unit-proven, not yet runtime-proven.

> **Overlap note (coordinator, 08:20):** B2c found the same class across **13** committed
> `*.dsl.semio` assets (writer ×2, animate ×2, sequence ×4 among them) and is fixing them at the
> producer. Writer's two were already corrected and gated here before that message arrived; the
> remaining eleven are B2c's and were not touched.

**W2 — `textSelect` is declared by no window kind (10 of writer's 11 fault lines).** The framework's
text-editor surface dispatches a fixed id set — `textEdit`, `textSelect`, `textHover`,
`requestCompletions`, `commitRename`, `formatDocument`
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:924`) — and writer declares five of the six; every caret
move therefore logs a console **error** plus a refusal warning. The worked precedent is
`🔱️trinity/🔌️jack`, which routes `textSelect` through a retained **`WindowTransient`** lane
(`…/🔌️jack/…/✏️editor/🦀️.rs:378,388,1103,1136`) — selection is per-window transient state, not a
document mutation. Not landed by this slice.

### Work landed this session
