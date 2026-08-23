# P4d R7/R8 Retained Fill-worker Envelope Acceptance Audit — 2026-08-24

## Verdict

**RED — reject P4d R7/R8 acceptance.** The current source resolves the two stated R7/R8
defects for the normal same-generation route, but it leaves a reachable cross-generation restore
clobber that strands an admitted authority permanently. It also does not enforce exact producer
identity at the worker ingress and still uses wrapping semantic operation generations. These are
P4d envelope lifecycle/identity failures, not P4e construction or direct spatial-operation
residuals.

## Evidence Read

- root `AGENTS.md`;
- the P4d implementation packet and the coordinator, Sol, and Terra rejection reports in this
  ticket, including the R6 and R7/R8 sections;
- current `precompute`, FillBuilder, fixed-owner geometry, two-caller action, plugin registration,
  and permanent verifier sources;
- current P4d source/verifier diff (production P4d paths are modified in the shared worktree).

No production or verifier source was changed by this audit. No Cargo, Nx, Wasm, browser, runtime,
or network command was run.

## Confirmed Repairs

1. **R7 normal admitted ownership: PASS.** Admission takes `engine.fill` by value before registry
   measurement (`precompute/component.rs:1459-1483`), and successful admission does not put it
   back. Registry reads borrow through `authority.fill.as_ref()?.try_lock()` without cloning
   (`:1291-1300`); restore explicitly leaves `engine.fill` empty (`:1425-1439`). The only
   production `authority.fill.clone()` is the worker-local opportunity (`:466-470`), which is
   dropped before `drive_fill_envelope` returns. Supersession asks the old generation to close,
   then builds/mutates only a distinct engine candidate (`:1302-1315`, `:1663-1666`). The direct
   R7 fixture captures old pointer/credit/page witnesses through weight and mesh supersession
   (`:2311-2345`).

2. **Actual fixed collection backing: PASS within the R6/P4d retained collection scope.** The ten
   FillBuilder and three spatial authorities are `FixedOwnerMap`/`FixedOwnerSet` fields. Their
   actual `Box<[Option<(K, V)>; N]>` page is capacity-bounded, non-Clone, exactly layout-credited,
   returns inserted-over-capacity and occupied input owners, and retires separately
   (`geometry/component.rs:16-164`, `fill/component.rs:1956-1967`,
   `geometry/component.rs:741-743`). The census consumes the live backing credit; it is not the
   former decorative-page or pair-size census.

3. **R8 normal partial-close reclamation: PASS.** Terminal-handle Drop records Closed intent then
   clears checkout (`precompute/component.rs:632-640`). `take_closed` can claim `Closing`, and
   session retrieval accepts it (`:385-393`, `:1557-1568`); `resume` rejects a partial close
   (`:542-560`). The R8 fixture holds the registry lock during session/handle drop, proves the
   same retirement cursor address after remount, drains to empty, and rejects rediscovery
   (`:2383-2415`).

4. **Bounded normal execution/retirement: PASS by static inspection.** The token cursor decodes
   one field per grant; the fault guard exists before decode; the worker has one `drive_step`,
   cancellation and operation/base/generation checks precede it, and close drains retained owners
   before the slot/aggregate credit (`:67-100`, `:428-513`, `:562-621`). The ordinary cap/+1,
   rejected-owner, cancellation/fault, terminal-empty, and stale-slot fixtures are present.

## Acceptance Blockers

### B1 — restore can replace a live mounted generation without terminalizing it

`restore_persisted_fill` accepts any token that names any live process-global registry authority;
it does not compare it with `self.fill_job`, request close for that prior generation, or bind the
token to this session/document/producer. It unconditionally assigns the new request at
`precompute/component.rs:1425-1438`:

```rust
self.engine.fill = None;
...
self.fill_job = Some(request);
```

Therefore, with admitted generation A mounted, a valid token for live generation B replaces the
only session reference to A. A is neither cancelled nor marked Closed. If A's worker completes,
it becomes `Terminal(Complete)`, but orphan reclamation selects only `Terminal(Closed)` or
`Closing` (`:385-393`); the session now polls B, and its Drop closes B only (`:1674-1681`). A is
then a permanently reserved registry slot with credited builder/pages. This directly disproves
full terminal/result registry recovery and no-livelock, and makes restoration a generation-handle
loss path despite R7's no-Arc-alias repair.

Required repair: restore must be transactional and identity-bound. Before replacing a different
mounted request it must durably close/retain-close the old request; it must only mount a token
authorized for the same document/session producer. Add a fixture with two simultaneous admitted
generations: restore B into A's session, drop/mount, and prove both generations each close once,
all slots empty, and aggregate credit zero.

### B2 — worker fault identity is not bound to the decoded registry request

The actual job producer sends the registry request's `job` as `Effect::SpawnJob`
(`fill-build-tick/component.rs:25-30`, `:45-49`), but `fill_job` does not verify that
`context.id().await` equals decoded `admitted_request.job` (`precompute/component.rs:1684-1711`).
The pre-decode guard is keyed only by the context job ID. A mismatched producer/context plus a
malformed token causes Drop to call `request_fill_envelope_terminal_by_job` for the unrelated
context ID (`:420-435`) rather than terminalize the token's exact registered generation. The
existing fixture supplies `request.job` as the guard ID (`:2440-2452`), so it does not
discriminate this mismatch.

Required repair: bind the producer ID to the registry identity before/while decoding, reject a
mismatch with a generation-keyed terminalization of the exact admitted authority, and add a
wrong-context/malformed-token fixture proving the original owner is observable and closes once.

### B3 — semantic operation generations can wrap to zero

Registry-slot generations are correctly checked, nonzero, and non-wrapping
(`precompute/component.rs:283-311`), and stale slot tokens are compared exactly. However the
operation generation carried in the token is advanced with `wrapping_add` in both rebuild and
refresh paths (`:731-732`, `:786`). It can become zero and later repeat. That fails the requested
checked/nonzero generation property for the token's operation generation; registry ABA protection
does not make this field checked. The permanent verifier has no mutation for either wrapping write.

Required repair: use checked increment with explicit exhaustion/rejection (never zero) for both
revision/generation fields as applicable, and add max/+1 and stale-token fixtures for this
semantic generation.

## Verifier Fidelity

`bun 📜️script.ts verify interactivity --self-test` passed its current P4d mutation corpus and
reported `DENY mode — clean` (one existing structurally invisible test-only allowlist record).
The corpus faithfully rejects the prior R7 alias, replacement-drop, R8 Closing-reclamation, fixed
backing, bulk-retirement, and listed fixture regressions. It does **not** mutate or observe:

- a restore of B while A is already mounted;
- session/document/worker producer identity mismatch at ingress; or
- wrapping-to-zero semantic operation generation.

Consequently the green verifier result is not acceptance evidence for B1–B3.

## Permitted Gates Run

- `rustfmt --edition 2021 --check` on the three P4d Rust sources: **PASS** (no output).
- `bun 📜️script.ts verify interactivity --self-test`: **PASS**, verifier baseline and mutation
  self-test clean as stated above.

## Requested Disposition

Keep P4d/Phase 4 **RED**. Do not begin P4e/P5b. Re-audit after B1–B3 are repaired with permanent
verifier mutations and direct source fixtures.
