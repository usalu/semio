# P4d Retained Fill-worker Envelope Implementation — 2026-08-23

## Pre-edit Reachability and Cap Census

The live fill action has exactly two production entry routes:

1. editor action dispatch at `✏️editor/🦀️component.rs:2326` calls `fill_build_tick`;
2. cached `ArtifactApp::handle` at `✏️editor/🦀️component.rs:2419` calls `fill_build_tick_cached`.

Both action functions call `Puzzle3dPrecomputeSession::enqueue_fill_job`. Before P4d that method called `fill_worker_checkpoint_bytes`, which cloned the request, scene, mesh sources, fill checkpoint, observation, and last checkpoint; sorted a collected mesh `Vec`; serialized the complete state with `serde_json::to_vec`; and only then rejected output above 4 MiB. Thus both UI routes reached whole clone/materialization/serialization before `Effect::SpawnJob`.

Pre-edit explicit limits were 1,000 accepted placements, a post-encode 4 MiB worker envelope, 64 meshes, 196,608 values per positions/indices vector, 393,216 aggregate mesh values, 4 KiB URL, and 4,096 cells per spatial entry. There was no pre-serialization operation/item/process-byte reservation, no <=16 KiB page owner, and no public rejected/terminal take/resume/one-owner close path.

## Implementation Status

Rejection remediation source-audit-ready; not independently accepted.

## 2026-08-23 Independent Rejection Remediation

The first independent source audit rejected four ownership seams. The remediation replaces the
literal maximum reservation with a retained `FillBuilderOwnerCensusCursor`. A session action tick
measures one explicit builder field class, accumulates checked item/backing/page credit, and leaves
the exact `Arc<Mutex<FillBuilder>>` in the admission cursor until the census completes. Only the
measured `credit.items` and `credit.bytes` are passed to the four-slot registry; cap failure restores
the identical source authority before registry mutation.

The existing two UI callers now mount terminal handling through `poll_fill_job`. The session keeps a
strong `FillEnvelopeTerminalHandle`, releases its shallow live builder ticket, and advances one
`close_step` per later tick. A dropped session marks its admitted authority Closed; another mounted
session can take that fixed-slot orphan and continue the same close cursor. Worker exits after token
admission are guarded by `FillEnvelopeWorkerFaultGuard`, so restore mismatch, stale ownership, or
token retrieval faults transition the matching generation to observable terminal fault ownership.

Close no longer discards `authority.fill.take()` as an ordinary final `Arc` drop. It waits for the
session ticket to return, unwraps the last builder authority, and installs a
`FillBuilderRetirementCursor`. Each close grant removes at most one retained builder field/root; the
fixed token page, cancel owner, item scalar, slot shell, and aggregate byte release remain later
separate grants. The terminal witness becomes true only after the slot has been removed and its byte
credit returned.

New discriminating fixtures cover an actual populated owner census cap/+1 with exact pointer
handback, production-mounted terminal close and capacity re-arm, early fault terminalization, and an
interrupted deep retirement that requires multiple grants before terminal-empty. The permanent
verifier now rejects literal maximum credit, an unmounted terminal pump, a missing worker fault
guard, a bulk builder drop, and removal of each new fixture.

### Remediation gates

- Rust 2021 formatting and scoped `rustfmt --check` over the precompute envelope, FillBuilder owner
  cursor, collision-index retirement helper, and the two-caller action file: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean, including the expanded P4d
  mutations.
- `bun 📜️script.ts verify interactivity`: PASS; DENY clean with the existing one recorded test-only
  allowlist finding and zero unlisted findings.
- Production scans: exactly two `.enqueue_fill_job()` action calls; mounted production terminal
  take/close is present; 56-byte token remains; no old `FillWorkerState`, restore bridge, envelope
  serde, blocking wait, runtime, or pool construction was introduced. The remaining FillBuilder
  serde matches are dormant checkpoint/test facilities outside the live envelope route.
- Scoped and whole working, staged, and `HEAD` `git diff --check`: PASS with no output.

Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run. This packet is returned for
independent source audit only; Phase 4 remains RED.

## Implemented Source Boundary

- `⏳️precompute/🦀️component.rs` replaces `FillWorkerState` and its scene/mesh/checkpoint clones, sort, full JSON encode, full JSON decode, and isolated-session reconstruction with a process-wide fixed registry.
- The registry has four generation-keyed slots. Each operation reserves at most 65,536 semantic items and 256 pages of 16 KiB (4 MiB); process aggregate credit is 16 MiB. Item, operation, operation-byte, and process-byte rejection occurs before the source `Arc<Mutex<FillBuilder>>` is transferred.
- `enqueue_fill_job` takes the exact source authority by value from the session. On contention or admission rejection it restores that exact `Arc` synchronously; after successful admission the registry owns the transferred authority and the session receives only a shallow ticket to the same pointer.
- The UI-visible job input/checkpoint is a fixed 56-byte identity token carrying slot, registry generation, job, operation, generation, and base revision. It contains no scene, mesh, preview, fill plan, or checkpoint payload.
- `FillEnvelopeTokenCursor` decodes one fixed token field after each `JobCtx::tick` grant. Once decoded, each later grant calls exactly one `drive_step`; cancellation and operation/base-revision/generation checks precede FillBuilder mutation.
- The checkpoint-facing FillBuilder publication returns empty state/output bytes. The retained shared FillBuilder is the source of preview, progress, and final state, so no whole preview/checkpoint/result is encoded by a worker turn.
- Terminal completion, cancellation, and fault remain in the generation slot. `take_terminal_fill_job` returns a public handle with an observable reason, `resume`, atomic checked-out Drop handback, `close_step`, and `terminal_is_empty`. Close releases one shallow FillBuilder owner, cancel owner, fixed page owner, item-credit scalar, and final slot/byte credit on separate grants.
- Slot generations never wrap: `u64::MAX` permanently exhausts that slot. A closed slot is re-armed only by a later reservation; stale tokens cannot observe or mutate the replacement authority.

No compatibility synchronous bridge was retained. The two action files were not edited; their existing `Option` result receives synchronous exact owner handback on `None` and submits the fixed token on `Some`.

## Direct Fixtures

The precompute component now contains discriminating fixtures for:

- exact source pointer identity across admission and token restore;
- one retained worker opportunity;
- the four-operation boundary, fifth-operation rejection, close-driven capacity re-arm, and stale-generation ABA rejection;
- item cap + 1 and byte cap + 1 with exact pointer handback;
- checked-out terminal Drop handback, observable cancellation reason, and one-owner close;
- one-field-per-grant token decode; and
- proof that job checkpoint bytes are the fixed token rather than whole-scene JSON.

The FillBuilder fixture was updated to assert shared preview publication without serialization.

## Permanent Verifier

The existing root `📜️script.ts` interactivity verifier now reads the precompute component, checkpoint-facing FillBuilder region, and the one action file containing both call sites. Its permanent predicate denies changed/dynamic caps, clone-before-admission, missing item/byte preflight, whole serde, whole-token decode, missing base freshness, more than one drive opportunity, missing terminal retrieval/Drop handback, missing fixtures, whole preview publication, or a caller census other than exactly two. Thirteen faithful mutations are reconstructed from live source and all are rejected before the baseline is evaluated.

## Permitted Gates

- `rustfmt --edition 2021 --check` on both Rust files: PASS.
- Bun TypeScript parser over root `📜️script.ts`: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS, DENY clean. The permanent P4d mutations and baseline ran.
- `bun 📜️script.ts verify interactivity`: PASS, DENY clean.
- Production source scans: zero `FillWorkerState`, `restore_fill_worker_state`, or `fill_worker_checkpoint_bytes`; no production whole serde/block_on/pool construction in the envelope path; exactly two `enqueue_fill_job()` action call sites; fixed empty FillBuilder preview/complete publications present.
- Scoped and whole working-tree, staged, and `HEAD` `git diff --check`: PASS with no output.

Cargo, Nx, Wasm, browser, runtime, network, and root lint were not run, as required.

## 2026-08-23 Final Re-audit Remediation

The later coordinator re-audit found five remaining P4d source defects and two adjacent accounting
defects. This remediation supersedes the earlier heuristic-census and shallow-retirement claims in
this report.

### Exact retained census

- The multiplier-based FillOwnerMeter, pair-size BTreeMap estimate, Hash capacity estimate, and all
  whole-field measure helpers are removed.
- FillBuilderOwnerCensusCursor now retains field, section, phase, entry, inner-entry, and leaf
  positions. A fixed-depth FillDslOwnerCensusCursor retains a 16-level traversal path and advances
  one node backing, object key, child transition, collection entry, or small fixed schema string
  group per admission call. CollisionIndexOwnerCensusCursor separately retains the spatial-index
  section, entry, and nested-id positions.
- Every variable vector/string backing is measured from its actual capacity with checked arithmetic
  and rejected above one 16 KiB page. Dynamic collection cardinality is capped at 32 before its
  entries are visited. A max-cardinality fixture proves the cursor needs more than 32 grants and
  rejects cardinality 33.
- The ten live FillBuilder standard collections and three spatial-index standard collections use
  explicit separately owned 16 KiB backing-contract pages. Those pages are actual fixed owners
  included individually in admission and retirement. Semantic entries and their nested strings are
  credited separately; the implementation no longer pretends that pair size describes a
  standard-library node or bucket allocation.

### Exact retained close and terminal intent

- The nested FillBuilderRetirementCursor retains popped values and retires at most one dynamic
  backing/root per close grant. It also retires all ten fixed collection pages, every spatial bucket
  vector/string and all three spatial backing pages before the terminal-empty builder shell can be
  released.
- A four-entry atomic generation/job terminal-intent authority makes worker fault and session close
  transitions durable across registry-lock contention. The intent is applied to admitted,
  measuring, and already-terminal authorities; completed-before-session-drop work therefore becomes
  a mounted Closed orphan rather than an unreachable completed slot.
- Admission begins by moving the exact builder into the fixed registry before census. Session Drop
  requests close for measurement, admitted, rejected, and already-terminal phases. Mounted terminal
  take/resume contention returns the checked-out owner and re-arms the same generation.
- FillEnvelopeJobEntryCursor installs FillEnvelopeWorkerFaultGuard from the job authority before the
  first fallible token decode step. Malformed input cannot escape without recording terminal fault
  ownership.

### Discriminating evidence

Direct source fixtures now cover low-fuel/max-cardinality nested census, collection cap + 1, ten
fixed backing pages rather than pair-size heuristics, spatial bucket/backing close, session Drop
during admission while the registry is contended, completion before session Drop, checked-out
terminal resume contention, and malformed production token entry before the fault guard is
disarmed.

The permanent verifier uses only the live census slice and rejects restoration of a whole-field
iter-all census, a 33-item nested cap, pair-size backing estimation, missing fixed-page census,
missing fill or spatial backing retirement, lossy terminal locking, incomplete session close,
malformed guard reordering, bulk preview/builder/spatial drops, and removal of the discriminating
fixtures.

### Final source gates

- Rust 2021 formatting and scoped rustfmt check over the precompute envelope, retained FillBuilder
  census/retirement, collision-index ownership, and two-caller action file: PASS.
- bun 📜️script.ts verify interactivity --self-test: PASS; DENY clean. The audit reports one
  structurally invisible test-only allowlist record and zero unlisted findings.
- bun 📜️script.ts verify interactivity: PASS; DENY clean with the same recorded test-only allowlist
  entry and zero unlisted findings.
- Exact scans: two production enqueue_fill_job action calls; zero old FillWorkerState,
  restore/checkpoint-envelope bridge symbols; zero multiplier/pair-size/whole meter helpers; zero
  whole-field iter-all, loop, clone, or serde occurrence in the live retained census slice.
- Scoped working, staged, and HEAD diff checks over the P4d source, verifier, and this report: PASS
  with no output. Whole working diff check: PASS with only an unrelated CRLF warning. Whole staged
  and HEAD checks remain RED only for six pre-existing trailing-whitespace findings in the
  concurrently owned P3m/P3n and Phase 10 coordinator reports; no P4d path is named.

This is a source-only author packet returned for independent audit. It is not P4d or Phase 4
acceptance.

## Honest Residuals

- P4d owns only the fill-worker envelope. It does not claim that existing FillBuilder collision/narrow-phase work has been accepted under P4b; a `FillBuilder::step` remains the indivisible semantic opportunity behind this envelope.
- FillBuilder::new, fixture fingerprinting, restore/checkpoint helpers, spatial query/upsert/remove,
  point-inside/world-bounds work, and preview/checkpoint/complete materialization remain the
  separately scoped P4e census. In particular, the existing constructor still contains whole
  loops/clones and the dormant checkpoint helpers still use serde. P4d does not describe those
  residuals as bounded.
- Collision/index/preview renderer math and files were not changed.
- Compile/runtime validation was intentionally not run, so this is a source-only packet awaiting independent audit.
- The broader Phase 4 vertical slice remains RED for the collision/index/preview packets listed in the preceding Sol status-gap audit. No ticket or phase acceptance is claimed here.

## 2026-08-23 P4d-R6 Actual Fixed-owner Storage Remediation

The second independent remediation re-audit correctly rejected the ten FillBuilder and three
spatial-index byte arrays as decorative: standard-library tree/hash allocations remained outside
those boxes. This R6 repair removes both backing-token types and replaces all thirteen retained
standard collections with `FixedOwnerMap`/`FixedOwnerSet`. Each authority has one
`Box<[Option<(K, V)>; 32]>`; that exact box is the sole entry/control storage, its requested layout
is at most 16 KiB, its exact layout size is the byte credit, and the map cannot allocate another
entry backing. FillBuilder and the collision-index census now read credit from those same live boxes.

Insertion at the 32-entry boundary returns the exact input key/value owner without page or state
mutation. An equal occupied key also does not replace or drop either side: the typed `Occupied`
outcome returns the distinct input key and value while the original key/value remains in its slot.
`remove_entry` returns the stored key and value together. No Clone implementation exists on the
fixed map/set or spatial-index authority. The retained terminal cursors pop semantic entries first,
then release exactly one of the ten FillBuilder or three spatial boxes per later close grant; the
terminal witness requires every slot and its box to be absent.

Direct structural fixtures populate every FillBuilder fixed-map/set monomorphization and all three
spatial collections to 32, reject 33 with pointer-identical dynamic owners, verify that the backing
pointer never changes, retire semantic owners, and return the same actual box only afterward. A
separate unequal-capacity/equal-key fixture proves input key/value handback and preservation of the
stored nested value. The census fixture now compares each credit delta with the actual live slot
array layout rather than a constant token page or pair-size heuristic.

The permanent P4d predicate now rejects decorative byte arrays, a standard collection restored to
any of the thirteen fields, 33 slots, missing live backing credit, occupied-owner erasure,
value-only removal, Clone restoration, bulk fill/spatial backing release, and removal of the new
boundary/occupied-owner fixtures. The current verifier reconstructs every mutation from the live
source before evaluating the baseline.

### R6 permitted source gates

- `rustfmt --edition 2021 --check` over the fill, geometry, and precompute components: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean with the existing one
  structurally invisible test-only allowlist record and zero unlisted findings.
- `bun 📜️script.ts verify interactivity`: PASS with the same DENY-clean result.
- R6 scans: zero decorative `FillBuilderCollectionBackings`/`CollisionIndexCollectionBackings` or
  byte-token boxes; zero standard map/set type in the live thirteen retained fields; the checkpoint
  DTO still has five BTree fields and the process session still owns its source mesh HashMap, both
  outside the live retained collection authority.
- Scoped working, staged, and HEAD diff checks: PASS with no output. Whole working: PASS with an
  unrelated CRLF warning. Whole staged and HEAD: RED only for the same six concurrently owned
  P3m/P3n/Phase 10 report whitespace findings; no P4d file is named.

The pre-existing `try_from_btree`, `cloned_btree`, checkpoint restoration, and explicitly named
`clear_for_rebuild_residual` whole-owner paths remain P4e constructor/restore/rebuild residuals. This
R6 packet does not call them bounded and does not change their semantics. No P4e source work was
started. Cargo, Nx, Wasm, browser, runtime, network, and root lint remain closed. This is source-only
and audit-ready, not accepted; Phase 4 remains RED.

## 2026-08-24 P4d-R7/R8 Exclusive Admission and Interrupted-close Remediation

The sixth independent audit accepted R6's actual fixed owner storage but rejected two admitted
lifecycle paths. This narrow remediation changes only admitted-owner reachability and resumable
Closing handback; it does not start P4e.

### R7 exclusive admitted mutation ownership

- `enqueue_fill_job` moves `engine.fill` into the registry before census and never assigns that Arc
  back to the session. `restore_persisted_fill` mounts the generation request, cancellation ticket,
  scalar observation, and fixed token only; it explicitly leaves `engine.fill` empty.
- Mounted reads use a scoped immutable registry guard. The Arc is not cloned or returned to the
  session, so the only mutable admitted path remains `drive_fill_envelope`. Unadmitted candidates
  continue to use the engine-local owner.
- Scene, weight, brush, and mesh inputs first durably cancel and request retained close for the
  admitted generation. If no unadmitted candidate exists, the engine builds a distinct candidate;
  that candidate is later measured and admitted independently. Existing pre-admission
  `soft_replan_fill_tail`, `refresh_fill_job`, `restart_search`, and `configure` calls therefore
  cannot reach the registry-owned builder.
- The terminal pump no longer takes or drops `engine.fill`. Closing an old admitted generation
  consequently cannot destroy a newer unadmitted replacement.
- Test-only fixed backing witnesses cover all ten FillBuilder pages and three spatial pages. The
  discriminating source fixture captures the admitted Arc pointer, exact item/byte credit, every
  page pointer/layout, and semantic length, applies both weight and mesh supersession, and requires
  all old witnesses to remain identical until retained close. It then proves the separate candidate
  survives old-generation close and is independently re-censused.

### R8 durable partial-Closing handback

- `FillEnvelopeTerminalHandle::Drop` records the same generation's Closed intent before clearing
  `checked_out`. It does not reset or move the partial retirement cursor.
- Registry orphan retrieval first applies pending intents and can atomically claim either
  `Terminal(Closed)` or `Closing`. Session terminal retrieval likewise admits `Closing`, while
  `resume` refuses to turn a partial close back into admitted work.
- The interrupted-close fixture advances exactly one close grant to create
  `FillBuilderRetirementCursor`, holds the registry lock while the session and checked-out terminal
  handle are dropped, mounts a new session, and compares the retained cursor address before and
  after reclamation. Subsequent grants reach terminal empty, and readiness cannot rediscover the
  generation a second time.

### Permanent verifier and permitted evidence

The P4d predicate now rejects a restore-time or post-admission mutable Arc alias, dropping a newer
candidate from the terminal pump, missing immutable registry reads, missing durable supersession,
removal of Closing from orphan/session retrieval, and a terminal-handle Drop that only clears
`checked_out`. Faithful mutations reinsert both former alias paths, reinsert the terminal-pump drop,
remove Closing reclamation or the Drop wake, and remove either discriminating fixture.

- Rust 2021 formatting and `rustfmt --check` over the precompute, FillBuilder, and geometry
  components: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean. It reports the existing one
  structurally invisible test-only allowlist record and zero unlisted findings.
- `bun 📜️script.ts verify interactivity`: PASS with the same DENY-clean result.
- Exact scans: the only `authority.fill.clone()` is the one-turn worker-local guard; the only
  `engine.fill.take()` is pre-admission ownership transfer; there is no restore/session assignment
  from registry authority to `engine.fill`; the UI caller census remains exactly two.
- Scoped working, staged, and HEAD `git diff --check` over the three P4d Rust files and root verifier:
  PASS with no output. The same checks including this report are rerun below before handoff.

No Rust build or runtime test was run; the new Rust fixtures are source evidence guarded by the
permanent mutation corpus, not claimed runtime results. Cargo, Nx, Wasm, browser, runtime, network,
and root lint remained closed. P4e constructor/restore/spatial/preview residuals are unchanged. This
is a source-only remediation packet returned for independent audit; P4d and Phase 4 remain open.

## 2026-08-24 P4d-R9/R11 Restore, Producer Identity, and Exhaustion Remediation

The R7/R8 acceptance audit rejected three remaining P4d envelope seams. This remediation closes
only those seams and preserves the accepted exclusive admitted owner, one-owner turns, and
rediscoverable partial-Closing cursor.

### Transactional cross-generation restore

- `restore_persisted_fill` first decodes the complete nonzero token identity and resolves the exact
  registry authority. If the session already mounts a different live request, or owns a different
  live terminal handle, restore returns `false` before changing any engine/session field.
- Same-request restoration remains available when its authority is not checked out. A stale local
  request may be replaced only after its exact registry authority is absent.
- Direct fixtures exercise a mounted A in Measuring, Admitted, Complete-but-unclaimed, Cancelled,
  Fault, Closed, and partial Closing phases while restoring B. They preserve A's request,
  observation, phase, retirement cursor, and checked-out handback, then retire A and B once and
  require every registry slot plus aggregate byte credit to reach exact zero.

### Exact worker producer binding

- The early worker fault guard now retains the full raw envelope request identity. Its Drop records
  Fault only through the token's exact slot/job/registry generation; the job-only global lookup is
  removed.
- After the bounded one-field decode, `fill_job` binds `JobCtx::id()` to
  `admitted_request.job`, the raw envelope identity, and the current exact registry authority
  before restored-token decode or any drive opportunity.
- Malformed-token/wrong-context, well-formed wrong-context, and stale-envelope fixtures prove that
  the decoded producer faults once while an unrelated live owner remains Admitted, and that a stale
  no-owner identity cannot transition its replacement.

### Checked nonzero semantic identity

- Rebuild and refresh share one atomic `allocate_fill_identity` path. Revision and generation are
  computed with `checked_add`, required nonzero, and assigned only after both checks succeed.
  `u64::MAX` is the final identity and every later allocation is permanently refused; no counter
  resets or saturating/wrapping aliases.
- Token decoding rejects zero registry generation, job, operation, semantic generation, and base
  revision one field at a time. Registry aggregate credit release now uses checked subtraction
  instead of masking an accounting defect with saturation.
- Max/+1, repeated exhaustion, atomic revision refusal, zero token counters, and exhausted-token ABA
  fixtures cover both semantic counters. The adjacent P4d scan found no remaining wrapping
  fill-revision/fill-generation write; FillBuilder's pre-configure zero constructor/checkpoint
  residual remains the already-declared P4e construction scope and never reaches a live P4d token.

### Permanent verifier and permitted source gates

- The P4d predicate now requires transactional live-request restore rejection, exact raw-owner
  guarding, context/request/live-authority binding before drive, checked semantic allocation,
  nonzero token decoding, checked aggregate credit release, and all new discriminating fixtures.
- New faithful mutations reintroduce cross-generation restore clobber, omit context/stale binding,
  restore wrapping revision or generation, admit zero semantic identity, saturate credit release,
  and remove each new fixture.
- `rustfmt --edition 2021 --check` over precompute, FillBuilder, and geometry: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean with the existing one
  structurally invisible test-only allowlist record.
- `bun 📜️script.ts verify interactivity`: PASS with the same DENY-clean result.
- Scoped working, staged, and HEAD diff checks over the three P4d Rust sources and permanent
  verifier: PASS with no output. Production scans found no job-only fault lookup, wrapping semantic
  fill counter, saturating aggregate-credit release, or old job-keyed guard construction.

Cargo, Nx, Wasm, browser, runtime, network, broad builds, and root lint were not run. The Rust
fixtures are source evidence only and are not reported as runtime-passing. P4e/P5b was not started.
This packet is source-audit-ready; P4d and Phase 4 remain open pending independent acceptance.
