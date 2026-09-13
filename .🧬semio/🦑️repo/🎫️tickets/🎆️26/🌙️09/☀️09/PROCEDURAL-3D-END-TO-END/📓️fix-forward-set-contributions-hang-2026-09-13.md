# Fix Forward — `setContributions` Never Returns (2026-09-13)

Lane `fix-forward-set-contributions-hang` (Opus). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Repo/semio MCP both failed to connect all session (`repo`: -32602 invalid initialize params;
`semio`: CONNECTION_CLOSED) — no ticket was opened/closed/reopened by this lane, no `📓️status.md`
or `🎫️ticket.json` edit, no modifying git command, no dev server started or stopped.

## TL;DR

The playground boot on `http://127.0.0.1:6018/?plugin=generation3d` is **green again**.
`setContributions` now settles in **3.0 s** (was: never — 75 s of silence, previously a host
watchdog kill/recreate loop every 18–24 s), and the hex column reaches the preview as **3 meshes**.
All **8 bundled examples converge in edit mode** (3–8 s each, meshes ≥ 1).

Root cause was **not** in the contributions install path at all. The 248 635-char pack installs
fine; what never returned was the *next* `flowEvalTick`, whose typed operation sat in the **Worker**
stage forever because `FlowHostRetirement::close_page`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2366`) only ever *closed* a
reserve-then-close frontier: `crate::retained::FlowRetirement::close_step` answers `Blocked` — never
an error — while `next_allocation_bytes` still names a page the current owner's decomposition needs.
Zero progress, no fault, forever.

**I did not author that fix.** A concurrent peer lane landed it (uncommitted, in the working tree)
between my 12:26 and 12:31 measurement runs, while I was bisecting the same stall. My contribution
is the diagnosis chain that localised it, two unblocking fix-forwards, the restage, and the runtime
proof on 6018. I am reporting it so the coordinator does not assign the same hunt twice.

A **separate, crate-wide regression remains** and is *not* fixed: 34 of 74 `unit_tests` laws fail in
teardown on `returned snapshot-read disposer is waiting on external ownership`. Details in §5.

## 1. Method — how the stall was localised

The coordinator's fresh reproducer run (`🗑️generated/s4-native-served-chain.txt`) matched 0 tests
(152 filtered out) because the whole `editor` module is behind a feature gate. The reproducer only
compiles with:

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib <name>
```

(389 lib tests with the feature, 152 without — `📦️packages/🦀️rust/Cargo.toml`, `[features]
component-app-assembly`). With the feature the failure reproduced immediately
(`🗑️generated/fix-forward-set-contributions-hang/repro-1.txt`).

Three narrowing steps, each with temporary `[DEBUG]` instrumentation in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (all reverted, §6):

1. **Which pending predicate never clears.** Widened the timeout message in
   `settle_registered_typed_operation` (`🔌️plugin/🦀️.rs:6705`) with a bitmask of the seven
   `has_pending_typed_operations` disjuncts → `pending-mask=0b1` = `tool_operations` non-empty. The
   operation itself never completes; nothing else is stuck.
2. **Which operation stage.** Recorded `MountedTypedCommandFullOperationStage` at the publication
   driver's selection point → `operation-stage=0` = **Worker**. So the stall is upstream of
   publication, ACK and retirement — it is the job.
3. **Whether the job is running.** `sample` on the live 30 s hang
   (`🗑️generated/fix-forward-set-contributions-hang/sample.txt`): the test thread burns
   12 561/15 018 samples in `cthread_yield`/`swtch_pri` directly under
   `settle_registered_typed_operation`, and **all five `semio-pool-worker-*` threads sit in
   `__psynch_cvwait`**. No work was in flight. A worker-stage step was being asked for progress and
   truthfully answering "blocked", round after round.

This also corrected two premises in the brief:

- The hang is **not** in `setContributions`. In the native reproducer all 31 pages install cleanly
  (`[MEMORY] after page 0` … `after page 30`, `after install`) before the panic fires from
  `drain_flow_eval_ticks_with_view`. The parallel audit
  (`📓️audit-regression-diff-2026-09-13.md` §0) reached the same conclusion independently.
- Both the named reproducer and its sibling `hex_column_evaluates_end_to_end_through_the_extension_round_trip`
  fail **identically**, at the same `flowEvalTick`. The stall is not contributions-specific, which
  is what pointed at the shared flow-host retirement ladder rather than anything in the pack path.

## 2. Root cause

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2366` `FlowHostRetirement::close_page`,
before the peer's fix:

```rust
} else if !state.domain.is_empty() {
    if state.domain.close_step(1, maximum_bytes).is_err() {
        state.faulted = true;
    }
}
```

`crate::retained::FlowRetirement` is a **reserve-then-close** frontier. Its `close_step` returns
`Blocked` — not an error — for as long as `next_allocation_bytes` still names a page the current
owner's decomposition requires. A driver that only ever calls `close_step` therefore spins forever
on any fixture whose widgets claim continuation slots, silently: no error, no fault, no progress.
`FlowRetirement::retire_cold` already paid the reservation first; this ladder did not.

The fix (peer-authored, uncommitted, lines 2388–2394) pays the reservation before closing:

```rust
match state.domain.next_allocation_bytes() {
    Ok(Some(demand)) => { if state.domain.reserve_allocation(demand).is_err() { state.faulted = true; } }
    Ok(None) => { if state.domain.close_step(1, maximum_bytes).is_err() { state.faulted = true; } }
    Err(_) => state.faulted = true,
}
```

This is the same defect *family* the parallel audit ranked first (a 4 KiB grant that can never
satisfy a larger exact demand, silently returning "no progress" rather than erroring), but at a
different site: the audit's top candidate was `RetainedInflateHistory` driven by
`WindowConfigPackLoadGrant::one_page()`. I did **not** confirm that candidate — the window-config
`load` loop is bounded (1 048 576 iterations, then cancel, then a `window-config.load-retirement`
fault), so it would surface as an error rather than the observed silent spin, and my stage
instrumentation put the stall in the Worker stage rather than in a window-config commit. It may
still be a real latent bug; it is simply not this one.

## 3. What I changed

Two fix-forwards for peer in-flight breakage that blocked the crate from compiling at all. Neither
is part of the root-cause fix.

1. `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:838`
   — `pub(super) fn node_move_operations_json` → `pub(crate)`. A peer's new
   `🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs:15` calls it from a sibling module; `E0603`
   broke every build of the crate.
2. Waited out (did not author) a peer's half-created
   `🎮️commands/📤️export-document/🧪️tests/🔬️unit/🦀️.rs` — `📤️export-document/🦀️.rs:56` declared
   `mod tests;` against a file that did not exist yet for ~10 min. Writing their test file would
   have collided with their work, so I polled until it landed.

Everything else I touched was temporary `[DEBUG]` instrumentation, fully reverted (§6).

## 4. Laws run in the foreground, with counts

All runs are my own, in the foreground of my own Bash calls, logs under
`🗑️generated/fix-forward-set-contributions-hang/`.

| Run | Command | Result |
|---|---|---|
| `repro-1.txt` | `cargo test … --features component-app-assembly --lib host_pushed_contribution_pages_install_the_registry_the_served_chain_needs` | **FAILED** 0 passed / 1 failed, 31.83 s — `flowEvalTick: … did not retire within 30 seconds` (the stall, reproduced) |
| `repro-hex.txt` | same, `hex_column_evaluates_end_to_end_through_the_extension_round_trip` | **FAILED** 0/1, 31.06 s — identical stall |
| `repro-hex-2/3.txt` | same, instrumented | **FAILED** 0/1 — `pending-mask=0b1`, `operation-stage=0` (Worker) |
| `repro-final.txt` | `cargo test … --lib -- --test-threads=1 host_pushed_contribution_pages_install_the_registry_the_served_chain_needs` | `[STATS] host-pushed chain finished: meshes=3` in **2.87 s** — the served chain converges; **1 failed** on the teardown assertion only (§5) |
| whole `unit_tests::` module | `<test-bin> --test-threads=1 unit_tests::` | **40 passed, 34 failed**, 244 s — every failure is the §5 teardown assertion |

Note on `RUST_MIN_STACK`: libtest's default thread stack (2 MiB here) is **not enough** for this
chain in a debug build — it aborts with `has overflowed its stack` before reaching any assertion. At
≥ 4 MiB the chain runs to completion. I used `RUST_MIN_STACK=16777216` for the final runs. This is a
debug-build native harness artifact and I am **not** claiming it is a guest/wasm defect, but it is
worth a look: the served guest's shadow stack is fixed and smaller, and nothing in the harness
declares this requirement today.

## 5. NOT fixed — crate-wide teardown regression

`Generation3dAppFixture::drop`
(`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:53`) drives `close_step` up to 1 000 000 times and then asserts
`close_terminal_is_empty()`. **34 of 74** `unit_tests` laws now fail there, with the framework's own
named cause:

```
registered fixture did not reach its exact terminal-empty witness,
last pending close authority: returned snapshot-read disposer is waiting on external ownership
```

That reason string is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8576` —
`store::SnapshotRetirementStep::Blocked` from the returned-snapshot-read disposer. It is the same
"a close ladder is blocked and nobody unblocks it" shape as the root cause above, at a different
owner, and it is almost certainly from the concurrent close-ladder/retained-allocation refactors
landing today (`💾️binary` snapshot + mutations, `📥️retained` window config, `🗜️deflate`). It is
teardown, not the served chain — the browser is unaffected — but it makes the crate's unit suite
unusable as a gate. **This needs its own lane.**

## 6. Runtime proof

Restage: `CARGO_PROFILE_WASM_DEV_DEBUG=false NX_DAEMON=false bun nx run
@semio-tech/framework-os-dev:activate-generation3d-react-dev`, foreground, 32 m 35 s, exit 0,
"Activated generation3d react dev: 11 completed components (changed)"
(`🗑️generated/fix-forward-set-contributions-hang/restage-1.txt`). This built the guest wasm from the
working tree **including** the peer's `flow/host` fix.

### Boot probe — `🗑️generated/fix-contrib/boot-1/`

`SEMIO_PROBE_OUT=fix-contrib/boot-1 SEMIO_PROBE_SECONDS=90 bun 🐍️console-dump-probe.mjs`

```
2331 ms  performInvocation {"invocationKind":"command","actionId":"setContributions"}
5359 ms  performInvocation settled  {"actionId":"setContributions","frames":2,"effects":2}
5359 ms  contributions publish {"outcome":{"status":"installed","chars":248635,"kinds":[…16 kinds…]}}
```

**`setContributions` settles in 3.0 s.** No watchdog line, no shard re-creation, no fault. The
console goes quiet at 13.6 s with the chain converged. `hosts.json`:

- `window:procedural-main` — all 7 widgets `"status":"ok"` (`height`, `radius`, `sides`, `profile`,
  `extrusion-axis`, `extrude`, `column-preview`)
- `window:procedural-preview` — **`meshes: 3`**, `phase:"idle"`, `ratio: 1.0`, 8/8 faces

### Journey probe — `🗑️generated/fix-contrib/journey/`

`SEMIO_PROBE_OUT=fix-contrib/journey bun 🐍️journey-probe.mjs` — 23 steps, 21 mesh steps, exit 0.

| Step | Converged | Time | Meshes |
|---|---|---|---|
| boot (Hexagonal Mushroom Column) | ✅ | 11 s | 3 |
| edit: No example | — | 61 s | 0 (expected: empty document) |
| edit: Hexagonal Mushroom Column | ✅ | 3 s | 3 |
| edit: Rectangle Extrude Volume | ✅ | 8 s | 1 |
| edit: Sphere Cut With Torus | ✅ | 7 s | 1 |
| edit: Box Fillet Preview | ✅ | 6 s | 1 |
| edit: Sphere Box Fuse | ✅ | 6 s | 1 |
| edit: Face Sweep Extrude | ✅ | 8 s | 1 |
| edit: Rectangle Wire Preview | ✅ | 3 s | 1 |
| edit: Box Shell Preview | ✅ | 5 s | 1 |
| back-to-edit | ✅ | 3 s | 1 |

**All 8 bundled examples converge in edit mode with geometry.**

## 7. What is NOT claimed

- **I did not author the root-cause fix.** `FlowHostRetirement::close_page` was fixed by a
  concurrent peer lane, uncommitted in the working tree. I localised the same defect independently
  and am reporting it; credit is theirs.
- **The audit's top candidate is unconfirmed.** `RetainedInflateHistory` / `WindowConfigPackLoadGrant::one_page()`
  is neither confirmed nor refuted by my evidence. My stage instrumentation points elsewhere and the
  window-config `load` loop is bounded, so it is not *this* hang — it may still be a latent bug.
- **The teardown regression (§5) is open**, root cause not established beyond the framework's own
  `returned snapshot-read disposer is waiting on external ownership` reason at `🔌️plugin/🦀️.rs:8576`.
- **The journey probe's non-edit legs report `converged=false`**: generate-mode, generate-added,
  viewer-role and all 9 `view:*` steps. They *do* carry meshes ≥ 1 (viewer shows 3 meshes for the
  hex column, 1 for the others), and the probe's convergence predicate needs a per-node status map
  that viewer/generate surfaces do not publish — so this is most likely a probe-predicate artifact
  rather than a regression. **I did not verify that**, and make no claim either way about
  generate mode or the viewer role.
- **No wasm/guest stack claim.** The 2 MiB libtest stack overflow (§4) is a native debug-harness
  observation only. I did not measure the guest's shadow stack.
- **No `📓️status.md` / `🎫️ticket.json` edit, no ticket lifecycle call, no git mutation**, and
  nothing in `🗑️generated` that another lane created was touched or deleted.

## 8. Files touched

Source (2 lines total, both peer-unblocking fix-forwards):

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  — `node_move_operations_json` visibility `pub(super)` → `pub(crate)` (line 838).

Temporary `[DEBUG]` instrumentation, **all reverted** (verified: a repo-wide grep for
`LAST_PENDING_TYPED_MASK`, `LAST_TYPED_OPERATION_STAGE`, `LAST_WORKER_STEP_REASON`,
`LAST_CLOSE_BLOCKERS`, `debug_close_blockers` returns zero hits outside `⚡️cache`; `git diff HEAD`
on `🔌️plugin/🦀️.rs` shows only a peer's `settle_history_verb` addition):

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — pending-mask, operation-stage,
  worker-step-reason and close-blocker witnesses.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — one `eprintln!` in the fixture `Drop`.

Report + logs (this lane's own, under `🗑️generated/fix-forward-set-contributions-hang/`):
`repro-1.txt`, `repro-hex.txt`, `repro-hex-2.txt`, `repro-hex-3.txt`, `repro-hex-4.txt`,
`repro-6.txt`, `repro-final.txt`, `build-5.txt`, `sample.txt`, `sample-run.txt`, `bin-path.txt`,
`restage-1.txt`; probe output under `🗑️generated/fix-contrib/boot-1/` and
`🗑️generated/fix-contrib/journey/`.
