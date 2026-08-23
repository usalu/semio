# Sol Independent P2d Live Preview and Progress Overlay Audit — 2026-08-23

## Verdict

**REJECT — source-only.**

The fixed actor-side owner store is a useful bounded foundation, and the shard outcome is no longer
discarded by a wildcard. The mounted WGPU route is not, however, a live preview/progress overlay:

1. the host never submits a production `Payload::JobStep`, so it never establishes the authority
   required to accept the shard's autonomous `ShardOutcome::Job` publication;
2. no production frame, presenter, or UI consumer checks out the retained preview; and
3. fault and realm shutdown do not enter an exact terminal overlay close.

The permanent verifier currently accepts all three gaps. No build or runtime acceptance is implied.
Phase 2 remains RED.

## Scope and evidence read

- `📓️p2-current-status-gap-audit-2026-08-23.md`
- `📓️p2d-live-preview-progress-overlay-2026-08-23.md`
- actor schema, overlay authority, and its five source fixtures in
  `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- generated actor TypeScript projection
- the single live shard producer in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- the WGPU kernel consumer/maintenance path in `📦️glue.rs`
- `toolJobActorProgressOverlayExact`, all ten P2d verifier mutations, and the current tool-job
  ledger output in `📜️script.ts`
- the complete scoped `HEAD` diff for actor, WGPU, shard, generated schema, and verifier files
- both stored deterministic ledgers

No production source was edited by this audit.

## Findings

### 1. The independent live authority hook is unreachable from the mounted WGPU route

**Blocking.**

`KernelPoolState::run_turn` builds only `Payload::Event` envelopes at WGPU glue lines 4230–4255.
The sole live call to `begin_job_progress` is guarded by `Payload::JobStep` at lines 4256–4264, but
there is no production `Payload::JobStep` construction in this host. The implementation report
itself acknowledges that exposing a product job-submit API is outside the packet.

The shard does not need another `JobStep` envelope to continue work. Its scheduler selects retained
`running_jobs` at shard lines 720–721, advances one job at lines 731–764, and emits the one live
`ShardOutcome::Job`. WGPU then calls `publish_job_progress`, whose first check at glue lines
3945–3949 rejects the publication `Stale` when no live authority exists.

Consequently, the single live producer and explicit WGPU `ShardOutcome::Job` arm are connected only
to a deterministic stale-owner retirement route. No live preview, checkpoint, or completion can be
accepted by this WGPU store.

The verifier does not prove reachability. It checks that a `Payload::JobStep` match text appears
before `runtime.submit`, and its mutation uses a synthetic string containing a JobStep envelope. It
does not require a production constructor/caller that can enter that arm.

### 2. Accepted previews have no production presenter or UI consumer

**Blocking.**

`publish_job_progress` publishes and immediately calls `job_progress.acknowledge(receipt)` at WGPU
glue lines 3945–3978. The exhaustive source census for `JobProgressOverlayStore::take`,
`job_progress.take`, and `JobProgressCheckout` finds:

- the public `take` definition; and
- actor unit-test calls only.

There is no production checkout in frame construction, retained surface composition,
`AppPresenter`, native presentation, browser presentation, or status/progress UI. Thus even a
synthetically admitted preview remains private storage and can never be displayed. The preview is
structurally distinct from committed scene state, but it is not a live overlay.

The verifier requires the `take` method's declaration and immediate ACK text, not a production
exact-consumer handoff. Its `WGPU-preview-without-ACK` mutation would still accept an implementation
that acknowledges before any presenter observes the owner, exactly as the current source does.

### 3. Fault and realm close do not reach a terminal overlay witness

**Blocking.**

Per-app and extension removal do call `begin_close_actor`, and one overlay close opportunity is
pumped before the next kernel request. Those scoped paths are present.

The complete source census for `begin_close_all()` finds only the method definition and two actor
unit-test calls. The process-global `run_kernel_pool` loop has no realm-close request, no
`begin_close_all`, and no `terminal_is_empty` witness before `KernelPoolState` release. The store has
no terminal-enforcing `Drop` shell. Therefore actor/app/extension closure is partial; realm closure
is absent.

In addition, WGPU's `ShardOutcome::Fault` arm at lines 4301–4314 completes the actor fault but does
not abort or begin-close its active job-progress slot. A runtime step fault can strand the admitted
operation until a later unrelated cancel/app close.

The verifier's purported realm-close mutation is synthetic: it changes a fabricated
`destroy_app` body and the live predicate merely searches `destroy_app` for `begin_close_actor`.
It never requires `begin_close_all`, a realm-close call site, a fault transition, or a terminal-empty
witness.

### 4. The permanent evidence matrix omits the live blockers and aggregate boundaries

**Blocking evidence gap.**

All 309 verifier self-tests pass, including the ten P2d mutations. Those mutations cover fixed
active storage, generation validation, rejected-owner retention, checkout Drop, last-valid abort,
the textual pre-submit hook, the explicit Job arm, ACK text, per-app close text, and cancel close.
They do not cover:

- a reachable production JobStep/authority source;
- a production preview checkout/presenter handoff;
- presentation-before-ACK ordering;
- shard fault retirement;
- realm `begin_close_all` plus terminal witness;
- the 128-retirement boundary;
- the 512-item or 4 MiB aggregate boundary; or
- operation/base/step/preview sequence mutations as distinct rules.

The actor fixtures exercise 64/+1 active slots and 16 KiB/+1 one-page admission, but no fixture
drives 128/+1 retirement, 512/+1 aggregate items, or 4 MiB/+1 aggregate bytes. The deterministic
fixture compares two in-memory stores; it does not traverse the mounted shard→WGPU→presenter route.

## Source properties that do pass

- The schema identity contains actor, job, operation, base revision, generation, step sequence, and
  preview sequence.
- Active and retirement storage are fixed `[64]` and `[128]` arrays. The overlay region contains no
  dynamic `HashMap` or `VecDeque`; its registry scans are over those fixed arrays.
- Source performs checked one-page and aggregate item/byte admission before moving a publication
  into the overlay store.
- Strict operation/base/generation and step/preview sequence checks are present.
- `JobProgressRejected` preserves the exact rejected publication; checkout Drop restores the exact
  preview owner and epoch.
- ACK, abort, displacement retirement, and per-actor close have explicit identity/token checks.
- Preview ownership is separate from committed scene storage.
- There is exactly one live shard `ShardOutcome::Job` producer and exactly one explicit live WGPU
  owned Job arm. The later wildcard covers other control outcomes, not Job.
- The shard continues to return after one selected job opportunity. The P2d scoped `HEAD` diff does
  not edit shard production scheduling.
- App cancel, app destruction, and extension cascade removal begin per-actor overlay close.

These properties do not overcome the missing live authority, presenter, fault close, and realm
close.

## Gates rerun

| Gate | Result |
|---|---|
| Rust-2021 `rustfmt --check` on actor, shard, and WGPU glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, 309 |
| `bun ./📜️script.ts verify interactivity --plain --deny` | PASS, DENY clean; one recorded allowlisted blocking bridge |
| plain tool-job coverage | expected DENY: 50 hosts, 50 invocations, 775 rows, 0 admitted, 884 residual, 18 failure classes |
| stored `p2d-tool-jobs-a.json` vs `p2d-tool-jobs-b.json` | PASS, byte-identical |
| ledger SHA-256 | both `8ab08296364d5faf82aa4d238dd589e445af5966b88bcc00e20fa74e34f2306a` |
| scoped working and staged diff checks | PASS |
| scoped `HEAD` diff inspection | complete; actor/WGPU/verifier are modified, live shard scheduling has no P2d diff |

The Rust fixtures were inspected but not executed because builds were explicitly prohibited. No
Cargo, Nx, Wasm, browser, runtime, network, or root-lint gate was run.

## Exact repair packet

1. Mount a real retained job-operation authority in the WGPU kernel realm before the shard can run
   the job. The live producer of that authority must be independently revision/generation witnessed,
   reachable in production, and atomically paired with the shard job identity; do not synthesize it
   from the arriving publication.
2. Add a fixed, exact-consumer presenter/frame handoff. A preview must remain last-valid and checked
   out until the matching frame/presenter ACK; stale/duplicate ACK must fail closed and return the
   exact owner. Immediate internal ACK without presentation is not sufficient.
3. Route `ShardOutcome::Fault`, cancel, actor/app/extension close, and realm close through the same
   retained close authority. Add a real realm close request/call site, `begin_close_all`, one-root per
   maintenance grant, and a terminal-empty witness before the store/realm can release.
4. Strengthen the permanent predicate and mutations to require the live authority constructor,
   production checkout/presenter consumer, ACK-after-consume ordering, fault close, true realm close,
   and terminal witness. Add distinct mutations for each.
5. Add mounted-route deterministic replay plus 128/+1 retirement, 512/+1 items, 4 MiB/+1 bytes,
   stale/cancel/fault during checkout, missing/duplicate/stale presenter ACK, and realm-close fixtures
   with exact pointer handback.

After source repair, rerun the non-build gates above. Cargo/native/Wasm/browser/runtime proof remains
mandatory for later acceptance when authorized.
