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

**NOT DONE.** Outstanding. Recipes `📜️b1a-activate.sh` / `📜️b1a-serve.sh`, probe
`🐍️b1a-boot-probe.mjs` (ports architect 6090 · animate 6051 · writer 6062).

## 7. B1a leftovers (deliverable 4)

**NOT STARTED** — writer's missing pane-reachable mutating verb, animate's empty default layout,
vcs's fold-contract violation, and the vcs/animate `DocumentStoreOwners` unit tests.

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
