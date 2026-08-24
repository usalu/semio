# P2a1 Universal Retained-job Ownership Partial Implementation and Blockers — 2026-08-24

## Status

**RED partial implementation; not source-audit-ready and not an acceptance claim.** The universal
job component now contains the retained payload, fixed child registry, and single-opportunity worker
session foundations described below. The required mounted caller migration and universal producer /
consumer conversion are not complete. No P2a or Phase 2 acceptance is claimed.

## Continued Implementation Checkpoint

- The public `InteractiveJob` taxonomy now requires `begin_close`, bounded `close_step`, and
  `terminal_is_empty`; there is no default or blanket no-op close proof.
- `WorkerJobSessionAdmissionRejected<J>` retains its exact job and parameters behind
  `ManuallyDrop` and exposes its own one-owner close cursor. Session-capacity rejection can no
  longer accidentally deep-drop or reconstruct the rejected job.
- Every admitted session reserves and writes its exact terminal-fault page before the first
  worker/caller opportunity. Panic and checked sequence exhaustion transfer that same page; an
  unused page is returned one page per close grant. The hostile panic fixture compares its backing
  pointer before submission and after terminal handback.
- `BatchJobSession` is now a thin owner over the same `WorkerJobSession` state machine. Its caller
  step executes one caught opportunity, then requires explicit take/resume or close; it has no
  independent batch drain.
- Checked-out Batch, worker-authority, and structured-child begin-close transitions report zero
  released owners: they transfer control into the close phase but do not claim disposal. The
  hostile fixture distinguishes both transition grants from the later exact job/child release.
- The renderer native-I/O handle and native clipboard registry now retain both admitted worker
  sessions and exact admission rejections. Each mounted turn submits, takes/resumes, adopts a
  terminal owner, or releases one close unit; neither path uses an async receiver or detached
  terminal drain.
- The renderer native-I/O registry is fixed and generation-qualified. Future polling only takes a
  result or registers a wake; `AppPresenter::present_step` advances one session/control/close
  opportunity. Handle Drop publishes durable cancellation. The max/+1 fixture captures the actual
  rejected request path backing, proves zero-pump and one-pump behavior, drains dropped handles,
  rejects the stale generation, and reuses the slot only under a greater generation.
- Native path and modified-path sets use fixed `ManuallyDrop` owner arrays. Ordinary populated Drop
  cannot deep-drop their paths. Max/+1 returns the same rejected path allocation and pending-job
  close consumes no zero grant and at most one path owner per positive grant.
- `JobScope` slots now retain an actual type-erased `InteractiveJob` child node, not a scalar debug
  count. Generation-qualified checkout, exact max/+1 rejected-child handback, durable close intent,
  one child owner per pump grant, stale/duplicate detection, and permanent slot exhaustion are
  exercised against the real child backing pointer.
- `RetainedJobPayloadWriter::write_slice_page` admits the actual page before copying and advances at
  most one 16 KiB page per job opportunity while retaining the exact rejected page source.
- Native-I/O and clipboard jobs, prepared/frame jobs, action-bus erasure, and router-effect fixture
  jobs now implement explicit close. Production router effects fail closed because no retained
  host pump is mounted; the former production `ComputePool::run_job` drain is test-only.
- Animate's `PresentEnvelopeMaterializeJob` now owns a pre-admitted retained fault writer, retains
  unexpected nested preview/checkpoint/terminal payloads for page-wise close, and implements the
  mandatory decode/field/completed/snapshot/envelope/fault close cursor. Its mounted handle uses
  fallible session admission, exact rejection retention, ticketed poll/take/resume, typed pool
  rejection, and separate terminal/session handback; the oneshot receiver and infallible
  constructor are gone. Fault and cancellation fixtures now prove a zero close grant changes no
  owner before the bounded registry close pump reaches terminal-empty.

Current changed P2a1 source includes:

- `📜️script.ts`
- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs`
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️host.rs`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{🦀️frame_job.rs,📦️glue.rs}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/{🦀️component.rs,🦀️native_io.rs}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️component.rs`

No P1q kernel/channel/storage region was edited. No P5b-owned UI-contract `UiValue`/map/reconcile,
renderer `UiValue` constructor, or plugin DSL/JSON conversion region was edited.

## Implemented Foundation

- Non-`Clone` 16 KiB page sources, retained payload writers, exact rejected-source handback,
  operation item/byte counters, per-stream counters, a process byte counter, and one-page close.
- `Checkpoint`, `CommitCandidate`, `JobFault`, preview payloads, and payload-bearing progress events
  now transfer `RetainedJobPayload` rather than cloneable `Vec<u8>`.
- Preview and step sequence advances use checked exhaustion.
- `JobScope` now uses 64 fixed generation-qualified child slots. Admission is fallible, `u64::MAX`
  permanently exhausts a slot, stale/duplicate completion is typed, and release builds reject parent
  completion while a child remains live.
- The public production definitions of `run_to_completion`, `run_on_worker`, and
  `run_on_worker_async` were removed. `BatchJobSession` advances exactly one externally requested
  opportunity.
- `WorkerJobSession` now transfers one exact job authority into one pool closure; pool rejection
  recovers the returned closure and publishes an exact rejected owner. Its public vocabulary covers
  typed contention, ticketed take, rejected take, terminal take, Drop handback, resume, incremental
  close, terminal-empty, and quiet wake registration/recheck.
- Focused hostile fixtures were added for page max/+1 pointer identity, zero close grant, separate
  state/output close, child max/+1/stale/duplicate/exhaustion, overlapping submission, exact terminal
  Drop handback, pool-shutdown rejection, panic, quiet wake, and batch one-opportunity behavior.

## Exact Blocking Boundaries

### 1. Other mounted session owners still use the removed vocabulary

Renderer native-I/O and clipboard now have mounted registries. Plugin cold-relay, plugin operation
controllers, Animate presentation, Layout export, Puzzle 2D brush, and Wasm entry points still
contain `WorkerJobSession::new`, receiver, `session.step(...).await`, or legacy `run_*` vocabulary.
They need the same fixed take/resume/terminal/close state machine before production source can be
accepted.

The current production-only source census is 9 `WorkerJobSession::new` sites: plugin component 6,
plugin host 1, infer 1, and Layout Wasm 1. Four fully-qualified legacy drains remain:
assembly/WFC 2, Layout 1, and Puzzle 2D brush 1. These counts increased after the
`e7bd5ecdf7` preservation boundary because overlapping packets added new old-vocabulary callers;
none was hidden by a compatibility API.

Eight additional production callers bypass the retained session through the public one-step
`drive_step` function: plugin cleanup 2, actor 1, Puzzle 2D 3, and Puzzle 3D 2. The
frame deadline and prepared-render callers have moved to `BatchJobSession` with exact
admission-rejection close, checked-out outcome inspection, and retirement transfer. The verifier
now rejects fully-qualified,
imported, and module-aliased forms; these
must be mounted on the same session machine rather than renamed.

The outer native `FrameBuildHandle` no longer uses `Receiver<RuntimeFrameResult>`. It owns one
generation-qualified `WorkerJobSession<ActiveFrameBuild>` or exact admission rejection. Native
submits one opportunity to the shared pool and registers a durable callback-backed wake; Wasm calls
the same session's one caller opportunity. Both use ticketed checkout/resume, terminal frame take,
incremental close, and terminal-empty before reuse.

The OS-services `ComputePool::run_job`/recursive scheduler is `#[cfg(test)]`; the protocol-specific
production-source stripper now removes attributed test items as well as test modules so that test
oracles cannot create a false production residual or a false acceptance.

### 2. Universal payload conversion exposes production `Vec<u8>` producers and consumers

The current nonempty old-codec groups are Board fill, plugin/store/guest relay, WFC/assembly, Layout
export, FEM graph/sparse/assembly, Puzzle 3D/5D, Deflate, and Energy. Store-initializer
fault producers also remain in GIS, Draw, Writer, Raster, and Jack. These sources still build
`Vec<u8>` candidates/faults or expect a whole byte vector from a terminal candidate.

Adding `From<Vec<u8>>`, a whole-buffer getter, or a compatibility `WorkerJobSession::step().await`
would compile those callers but directly violate the repair contract: allocation would precede
admission, whole output would again be public, and mounted code could recreate a terminal drain.
Each producer needs a retained page cursor in its own job state; each consumer needs page-wise
adoption/close. Multi-page outputs require multiple external opportunities.

### 3. Mandatory close is declared but not implemented by every job

The trait now has the required close vocabulary and the migrated jobs retain their partial writers.
A fresh production-only brace census finds 40 impl/macro sites and 29 without all three methods.
The remaining source families include plugin/runtime/host 8, board/store 2, FEM 6, WFC 2, Layout 3,
Puzzle 5, Deflate 2, and Energy 1. The Store-owned `ArtifactEnvelopeDecodeAuthority` nested by the
Animate caller is among the board/store residuals: its P1q-owned implementation still lacks the
mandatory close methods and still constructs a `Vec` terminal fault. The Animate cursor is ready
to call that retained close vocabulary once the coordinated storage region supplies it. The source
therefore intentionally does not compile as a complete workspace yet;
weakening the mandatory methods with defaults would be false proof.

### 4. Permanent verifier remains RED by design

`📜️script.ts` now has a P2a1 predicate that rejects optional job close, legacy constructors/drains,
post-work terminal-fault allocation, resizable/deep-dropping Native I/O outputs, Future-driven
session progress, missing generation recheck/host pump, and missing exact hostile identity proofs.
Faithful mutations remove the pre-admitted fault identity assertion, substitute the mounted +1
request identity assertion, remove zero-grant proof, restore deep-drop path storage, reintroduce
legacy APIs, and move work back into Future polling. Animate-specific mutations restore its
infallible constructor/receiver, remove its retained nested-outcome or fault writer, and remove
each fault/cancel zero-grant fixture. The live predicate is correctly RED while the callers above
remain.

## Scoped Checks Run

- `rustfmt --edition 2021` on the job, Native I/O, renderer glue, ProgramBridge, and frame-job files.
- `git diff --check` on the scoped P2a1 files: clean.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: `self-tests=356 clean`.
- `bun ./📜️script.ts verify interactivity tool-jobs`: expected RED at the universal retained-job
  predicate and the broader pre-existing migration predicates; it reports 884 remaining live
  command registrations.
- Static production-definition census: no public production definition of
  `run_to_completion`, `run_on_worker`, or `run_on_worker_async` remains in the universal job
component. Mounted external callers remain exactly as enumerated above.

No Cargo, Nx, Wasm, browser, runtime, network, or broad build/test command was run. Type or runtime
success is not claimed.

## Required Continuation

1. Add the mandatory job-owned retained close cursor and migrate every non-Puzzle `InteractiveJob`
   producer before relying on non-Clone universal outcomes.
2. Add mounted renderer/plugin/native/Wasm registries that own one session generation through
   take/resume/terminal/close/terminal-empty and advance only one opportunity per host turn.
3. Migrate all payload consumers to page-wise adoption; remove indirect whole-vector expectations.
4. After P4e and P1q quiesce, migrate their deferred callers without overwriting accepted peer work.
5. Add the complete hostile cap/+1/zero-fuel/cancel/panic/stale/ABA/drop/lost-wake suite and faithful
   verifier mutations, then run only the explicitly allowed scoped gates.

## 2026-08-24 File-Disjoint Layout And Cold-Relay Continuation

The bounded Layout-export and plugin cold-relay assist packets are now implemented without editing
the coordinated job core, Native I/O, renderer, Animate, Puzzle, P1q, or P5b regions.

### Layout Export

- `LayoutExportJob` now owns a fixed 634-byte publication buffer plus
  `RetainedJobPayloadWriter`. Preview, checkpoint, commit-state, and fault publication each advance
  through the retained page grant; checkpoint encoding writes directly into a fixed array instead
  of constructing a terminal `Vec<u8>`.
- `LayoutExportJob`, `LayoutExportToolJob`, and `LayoutMediaExportJob` implement the mandatory
  begin-close/one-owner-close/terminal-empty vocabulary. A pre-admitted empty snapshot placeholder
  removes allocation from snapshot close, and either the framework lease or a live exact external
  `Arc` owner witnesses snapshot handback.
- Layout Wasm uses `WorkerJobSession::try_new`, caller-side one-opportunity stepping, ticketed
  take, exact outcome-page close, resume/terminal handback, and incremental session close. Its fixed
  eight-slot mounted registry retains both rejected admissions and live dropped sessions together
  with the exact snapshot owner, so generation replacement or JS handle Drop cannot strand the
  snapshot backing.
- The production synchronous headless batch exports and `run_to_completion` reachability were
  removed. The old convenience exports exist only under `#[cfg(test)]` as an explicitly named test
  oracle; the scene re-export is likewise test-only.

### Plugin Cold Relay

- `GuestRelayCompletion` no longer has direct or indirect `JobStep`/`Vec<u8>` byte variants and no
  longer travels through a oneshot. Its byte-bearing step/rejection/fault variants transfer exact
  `GuestRelayOwnedBytes` owners. A fixed completion slot owns one exact completion, records a
  durable wake bit, installs/rechecks one waker, and transfers the completion once.
- Guest progress/output/fault bytes move into `GuestRelayPublication`, which owns a retained writer,
  copies at most one admitted payload page per job opportunity, then retires the exact guest source
  one page per opportunity before publication. Oversized `max + 1` sources become a retained fault
  without losing the rejected source owner.
- `GuestColdRelayJob` implements semantic close for pending completion, partial retained writer,
  start kind/input, explicit guest cancellation, fault/panic cleanup, and terminal-empty. The old
  detached cleanup-drain helper was deleted; close waits for the exact cancel completion.
- `PluginInstanceHandle` owns a fixed 16-slot generation-qualified mounted relay registry. The
  mounted future's poll calls one registry pump opportunity; the registry performs exact
  caller-step/take/page-close/resume or terminal-close ownership and keeps dropped futures mounted
  for later host-turn retirement. The old `WorkerJobSession::new`, `session.step(...).await`,
  receiver, and `run_job_on_worker` loop are absent from this production route.
- Each mounted reservation pre-admits its fixed one-operation output backing before
  `WorkerJobSession::try_new`. The registry copies at most one retained terminal page per pump and
  has no `terminal_bytes: Vec<u8>` accumulator. Full-registry rejection preserves every output
  backing identity; generation `u64::MAX` is issued once and then changes to a permanent exhausted
  sentinel instead of wrapping or aliasing.
- Cold-relay tests now use a retained take/resume oracle. Added exact mounted max/+1 generation and
  zero-pump coverage plus publication max/+1 source-preservation coverage; existing cancel, guest
  fault, panic, drop, and quarantine tests remain in place.

### Updated Exact Census

- In the four touched Layout/scene/plugin-host sources: zero `WorkerJobSession::new`, zero
  production `run_to_completion`/`run_on_worker`, zero `session.step(...Lane...)`, and zero
  `oneshot::{Sender,Receiver}<GuestRelayCompletion>` occurrences remain.
- Whole-tree raw `WorkerJobSession::new` occurrences are now 8 across three files. Seven are the
  known production sites (framework plugin component 6 and plugin infer 1); the assembly occurrence
  is test-local. This packet removed the prior Layout Wasm and plugin-host production sites.
- Whole-tree protocol legacy drains are now the three known production families: Energy 1,
  Assembly 1, and Puzzle 2D brush 1. The Layout production drain is gone; same-named machine FSM
  helpers and explicit test oracles are not protocol callers.
- This packet closes four previously missing mandatory-close implementations: the three concrete
  Layout jobs and `GuestColdRelayJob`. Other P2a1 families in the earlier census remain for their
  coordinated packets.
- One explicit outer codec group remains outside this file-disjoint packet: plugin ABI/router
  public methods still return their established `Vec<u8>` API. The mounted registry now keeps the
  result in its pre-admitted fixed backing through terminal-empty and materializes that outward ABI
  value only after job/output/session close completes; there is no resizable terminal accumulator
  or session compatibility step/drain. Replacing the route-wide API and its JSON/WIT schemas is part
  of the still-open universal caller-codec migration.

### Faithful Verification And Scoped Checks

- Added `toolJobLayoutColdRelayRetainedExact` to the permanent P2a1 predicate. Its mutations restore
  a Layout `Vec` writer, infallible Wasm construction, live-session Drop, cold-relay oneshot/direct
  or indirect `Vec` completion, resizable mounted output, wrapping mounted generation, lost output
  backing identity, the terminal run loop, and removal of the exact max/+1 generation fixture;
  every mutation is killed by the self-test.
- Scoped `rustfmt --edition 2021` on the four touched Rust sources: clean.
- Scoped `git diff --check` on those sources: clean.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: `self-tests=365 clean`.
- `bun ./📜️script.ts verify interactivity tool-jobs`: expected repository-wide RED with
  884 live command registrations and the still-open universal families above; no A/B legacy seam
  was reintroduced.

No Cargo, Nx, Wasm, browser, runtime, network, or broad build/test command was run. Type or runtime
success is not claimed.
