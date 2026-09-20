# 📓️ R2 — the reactor's retained command-ingress owner outlives its command

Slice R2 of `OS-HUB-COLLABORATION-AI-END-TO-END` (relaunch session 4, item (c)). Owns
`📓️a2-mcp-plugin-host-instance-open.md` §6.1 — `plugin.command-cursor-mismatch: … owner is 1, the
retained owner's is 554` — and, behind it, A2 §7 gaps 1–3: the semio MCP's mutation chain
(`action_prepare`/`action_invoke` → `artifact_snapshot` → `history_undo`/`history_redo` →
`transaction_begin`/`rollback` → `artifact_export`) against the real `.mcp.json` `semio` server.

All commands run from `/Users/ueli/Documents/semio`. Everything marked **measured** was executed and
captured under `🗑️generated/r2-*.txt`; everything else is labelled **derived** (read from both sides
of the wire) or **unverified**.

> ⚠️ **Build starvation, read §5 first.** From ~23:10 to the end of this slice the machine carried a
> peer fleet of 25–29 concurrent `cargo` processes at load average **100–173** on 10 cores. The
> slice's single queued `cargo test -p semio-framework-plugin --lib an_abandoned_ingress_owner`
> (pid 23886, started 23:33) sat on `Blocking waiting for file lock on artifact directory` for over
> an hour. The lock holder was a peer's `semio-s-artifact-block-2d` test with two live `rustc`
> children at 7–10 % CPU, i.e. genuine contention, not the known fine-grain-locking deadlock — so no
> process was killed (preamble rules 14/15). **The code fixes below are derived from both sides of
> the wire and are not yet runtime-verified.**

---

## 1. Inherited state

`🗑️generated` held no `r2-*` capture and `git status --short` no R2 hunk — nothing to inherit. A2's
captures (`a2-mcp-channel-test.txt`, `a2-client-e2e.txt`, `a2-host-laws.txt`) are the baseline. The
staged components under `.🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev/` are the ones
A2 left.

## 2. What the fault actually was — two defects, one symptom (derived)

The brief's framing (the reactor's `if let Some(owner) = retained.take()` block at
`⚛️reactor/🔄️turn/🦀️.rs:943-958` running before the current page is inspected) is the *second* half.
Reading both sides of the wire against A2's capture shows the numbers themselves are the tell.

`🌉️mcp/🏠️workspace/🦀️.rs` starts at `next_seq: 1` and mints ONE `seq` per `AppCommand`
(`exchange_one_slice`, `:1061`), and `CommandBatchDriver::new(seq, batch)` makes that `seq` the
cursor `owner` every page carries. A2's failing run is the **first** `exchange` on a freshly opened
channel — `ensure_instance` sends lifecycle events only, never a command page — so the host's driver
should have been owner **1**, not 554. 553 owners were burned inside one `PureCommand`.

**Defect 1 (host, `🏠️workspace/🦀️.rs` `exchange_one_slice`).** The guest turn is wrapped in
`prepare_suspend` → `execute_actor_turn` → `resume`. `prepare_suspend`
(`📡️spr/🧵️channel/🦀️.rs:820`) marks the driver `Suspended` **and links it into the registry's close
list**. Only the `Ok` arm called `resume`; the `Err(TurnFault::DeadlineExceeded | FuelExhausted)` arm
returned `budget.exceeded` immediately. So every 8 ms slice yield — the normal case for a cold guest,
hundreds per command — left the driver `Suspended`. The next slice read
`!pending_command_closes.is_active(…)` (`:1041`), took the "cancelled AppCommand" cleanup branch,
dropped the pending exchange, and the slice after that **re-sent the same command under a brand-new
`seq`**. That is the owner walk 1 → 554, and it is why a guest owner from the first attempt is stale
at all. It also means no cold-guest command could ever finish: the command restarted faster than the
guest could decode it.

**Defect 2 (guest, `⚛️reactor/🔄️turn/🦀️.rs`).** The guest keeps a fixed two-slot ingress authority
(`COMMAND_INGRESS: [Option<RetainedCommandIngress>; 2]`). The turn prologue took slot 0 if occupied
and otherwise slot 1, and wrote back to whichever it took — nothing else ever wrote slot 0, so in
practice **only one slot was ever used** and the second was dead. With one lane, an owner the host
has stopped driving can never be put down:

* a page naming a different command answered `Backpressure` (`:992-1000`) — forever, because the
  host never re-drives the abandoned command;
* a turn that carried **no** page answered `command_ingress_while_owned(Idle, retained)` →
  `CommandPending(stale cursor)` (`:1126`), and this gateway sends no page whenever the previous turn
  yielded mid-flight (`:1084`, `turn_in_flight`).

The host's `validate_cursor` then correctly refuses that status:
`plugin.command-cursor-mismatch: … owner is 1, the retained owner's is 554`.

Nothing on this wire tells a guest that a command was abandoned — there is no cancel event, and the
host's driver simply stops existing. The only evidence available to the guest is **a page that names
a different command on the same instance**, because both hosts that mint these cursors drive exactly
one command per instance at a time (`pending_exchanges` is keyed by instance;
`CommandDriverRegistry` slots by instance). That is the rule the fix derives from — not a special
case for the MCP host.

## 3. Reproduction — a native law

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
beside the existing ingress laws, driving the real `poll_kernel` with real `CommandBatchDriver`
cursors (no hand-written wire literals):

`an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving`

1. the host hands over page 0 of a **two**-page command (owner 4 096) and then abandons it — its
   driver is dropped and page 1 never arrives, which is what defect 1 did 553 times;
2. the host drives a **different** command (owner 4 097) on the same instance exactly as
   `exchange_one_slice` does — `next_page()` when the driver has one, **no page when it does not**
   (the page-less turn is where the stale owner used to speak), and every status through the real
   `CommandBatchDriver::observe`;
3. the law fails the moment `observe` refuses a status (pre-fix: `plugin.command-cursor-mismatch`),
   again if the new command never completes (pre-fix: endless `Backpressure`), and again if the
   ingress authority does not drain back to empty.

```
cargo test -p semio-framework-plugin --lib an_abandoned_ingress_owner -- --nocapture
```

**Not yet executed** — see the starvation note above; capture reserved as
`🗑️generated/r2-reactor-law-green.txt`.

## 4. Root fix

| # | file | change |
| --- | --- | --- |
| 1 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` `exchange_one_slice` | the `DeadlineExceeded`/`FuelExhausted` arm now `resume`s the driver before answering `budget.exceeded`. A slice yield ends the turn, not the command; the suspension window is "while this guest turn runs", and it is over either way. This alone stops the owner churn that made a stale guest owner reachable — and stops a cold guest's command being restarted out from under it. |
| 2 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `COMMAND_INGRESS_LIVE_SLOT`/`COMMAND_INGRESS_RETIRING_SLOT`: the dead second slot becomes the **retiring lane**. The live lane holds the owner the host is driving; the retiring lane holds a superseded owner, takes one bounded close step per turn (`step_retiring_command_ingress`) and contributes **no** ingress status — the host has no driver that could acknowledge it. The prologue now always takes the live lane, and the epilogue always writes it back. |
| 3 | same file, page admission | a page that names a different command **on the same instance** supersedes the retained owner into the retiring lane (`park_retiring_command_ingress`) and answers `Backpressure(new cursor)`; the host re-offers the page and the now-free live lane admits it on the next turn. A page for another instance is genuine backpressure — that owner's host driver is still live — and is unchanged. |
| 4 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | the acknowledgement fault now carries the status the guest actually answered and the instance/seq this gateway drives. A2's message named the mismatching field but not the shape behind it, which is what made this a two-session diagnosis. |

Why supersede rather than refuse: a `Fault` naming the stale owner cannot help, because the host has
no driver left that could accept a cursor for it — any status about an abandoned command is
unacknowledgeable by construction. The only status the host can act on is one about the command it
is driving, so the stale owner must become silent, and the only way to make it silent without
leaking its pages and decoder is to close it down on a lane of its own.

## 5. Proof — what is and is not measured

| claim | status |
| --- | --- |
| A2 §6.1's fault reproduces as `owner is 1, the retained owner's is 554` on the FIRST exchange of a fresh channel | **measured** (A2's `🗑️generated/a2-mcp-channel-test.txt:576`, re-read this slice) |
| `next_seq` starts at 1 and `CommandBatchDriver`'s `owner` is that `seq` | **derived** (`🏠️workspace/🦀️.rs:835,1061,1077`; `📡️spr/🧵️channel/🦀️.rs:957,973`) |
| the `DeadlineExceeded` arm leaves the driver `Suspended` and the next slice tears the command down | **derived** (`🏠️workspace/🦀️.rs:1041,1098-1107`; `📡️spr/🧵️channel/🦀️.rs:820-838,903`) |
| only one of the guest's two ingress slots was ever used | **derived** (the old prologue/epilogue were the only writers) |
| the two fixes make the native law pass | **unverified** — the run never got the build-dir lock |
| the MCP mutation chain (A2 §7 gaps 1–3) | **unverified** — see §6 |
| which staged components carry a 1 MiB stack | **measured** — §7 |

## 6. A2 §7 gaps 1–3 — the MCP mutation chain: NOT reached

`bun nx run @semio-tech/framework-os-mcp:client-e2e` was not run. Fix 2/3 are **guest** code: they
only take effect in a rebuilt `.wasm`, so the chain needs, in order:

1. `cargo test -p semio-framework-plugin --lib an_abandoned_ingress_owner` (the law above);
2. `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo rustc -p semio-s-plugin-note --target wasm32-wasip2 --profile wasm-dev`
   and the same for `semio-s-plugin-draw` (A2 measured 1 m 46 s / 2 m 14 s on an idle machine);
3. `cargo test -p semio-framework-os-mcp --lib plugin_artifact_channel_mutation_verbs -- --nocapture`
   — the cheapest end-to-end check, and the one whose message now names the guest's status;
4. `bun nx run @semio-tech/framework-os-mcp:client-e2e` (fall back to the underlying
   `bun ./📜️script.ts` verb if the nx daemon stalls, preamble rule 16), where A2's three red rows
   (`artifact_create` on `draw`, `action_prepare`, `artifact_export`) are the bar.

None of these could be started: a single queued cargo never reached the lock in over an hour.

## 7. A2 §7.5 — the 1 MiB stack across the other staged components (measured)

The link arg is **already systemic**, and this is the right shape: A2 moved
`-C link-arg=-zstack-size=8388608` into `.cargo/config.toml` `[target.wasm32-wasip2] rustflags`
(mtime 2026-09-19 03:23:15), beside the `--max-memory` it is carved out of, and deleted the
per-invocation copies from `🔌️plugin/🏗️build/📋️plan/🟦️.ts` and the repo cache's
`📋️native-orchestration/🟦️.ts`. Nothing further is needed in the build; what is left is stale
**artifacts**.

Measured, not inferred: `🐍️r2-wasm-stack-size.py` walks each component's embedded core modules,
parses the largest one's global section and reads global 0 — which under `wasm-ld --stack-first`
*is* the shadow-stack top, i.e. the `-zstack-size` the component was linked with. Capture:
`🗑️generated/r2-wasm-stack-sizes.txt`.

```
bun/python3 .🧬semio/…/🐍️r2-wasm-stack-size.py .🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2/wasm-dev
```

| 8 MiB — safe (7) | 1 MiB — will trap on `Event::InstanceOpen` (13) |
| --- | --- |
| `block`, `cad`, `draw`, `note`, `remodel`, `space`, `writer` | `architect`, `flow`, `gis`, `imperative`, `imperative_text`, `layout`, `lowpoly`, `procedural`, `process`, `raster`, `shooting`, `sourcing`, `wfc` |

Two of the 8 MiB seven (`space` 02:16, `writer` 02:17) predate the config change — they were built
through the TS build plan while it still passed the flag per invocation, which is exactly the
asymmetry A2 removed. mtime is therefore **not** a valid proxy for this; the global read is.

Each of the 13 needs one `cargo rustc … --target wasm32-wasip2 --profile wasm-dev` rebuild with
`CARGO_PROFILE_WASM_DEV_DEBUG=false` (~2–3 min each on an idle machine) and nothing else.

## 8. Honest gaps

1. **Nothing in §3–§4 has been run.** The two fixes are derived from reading both sides of the wire
   and are unverified at runtime — including whether they compile. The next worker's first act
   should be the §6 ladder, in that order.
2. **Defect 1 is stated from the code, not from a trace.** The 553 wasted owners are the arithmetic
   that follows from `next_seq: 1`, one `seq` per command and `owner == seq`; no run has yet printed
   the seq walk. Fix 4's richer acknowledgement fault is what will print it.
3. **Supersession is keyed on the instance, not on a per-instance lane.** The guest still has ONE
   live lane for all instances, so an owner retained for instance A still backpressures a page for
   instance B. That cannot livelock the MCP gateway (one instance per channel) but it can stall a
   multi-instance shard in the React/wgpu hosts. Not observed, not fixed, named here.
4. **`command_ingress_while_owned` is untouched.** An owner mid-assembly that gets a page-less turn
   still answers `CommandPending`, which sets `waiting` on the host driver and stops it paging — a
   latent deadlock for a multi-page command whose host ever skips a page. Supersession masks the
   abandoned case; the mid-assembly case is a separate defect with a cross-language contract
   (`🧫️fixtures/command-ingress-drain.json`'s `ownedTurnNeverIdle` and the TS twin) that must move
   with it. Deliberately out of this slice.
5. **`🔌️plugin/🦀️.rs:37506`** carries a live `[DEBUG]` line (`debug_runtime_line("[DEBUG]
   plugin_exchange entry …")`). It is a peer's, in a file several slices are editing; left alone,
   reported.
6. **A2 §6.3's `semio-framework-os-kernel` peer reds** were not re-checked.

## 9. Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | `resume` on the slice-yield arm of `exchange_one_slice`; acknowledgement fault now carries the guest's status and this gateway's instance/seq |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | live/retiring ingress lanes (`COMMAND_INGRESS_LIVE_SLOT`, `COMMAND_INGRESS_RETIRING_SLOT`, `step_retiring_command_ingress`, `retiring_command_ingress_is_free`, `park_retiring_command_ingress`); supersession in the page-admission chain; prologue/epilogue use the live lane |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` | new law `an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving` |
| `…/🎫️tickets/…/🐍️r2-wasm-stack-size.py` | new: reads each staged component's linked shadow-stack size from its core module's global 0 |
| `…/🎫️tickets/…/📓️r2-reactor-retained-command-owner.md` | this report |

Captures (delete with `🗑️generated`): `r2-wasm-stack-sizes.txt`,
`r2-reactor-law-green.txt` (queued, never got the lock), `r2-mcp-channel-test-1.txt` (queued,
cancelled before it got the lock so that the run would include all of this slice's edits).

---

## 10. Session 5 (2026-09-20, measured)

Session 4's §8.1 ("nothing in §3–§4 has been run") is **withdrawn**: the queued run did get the lock
after this slice's turn ended and wrote its capture. Everything below was executed here.

### 10.1 The law is green — twice, on two different runs

`🗑️generated/r2-reactor-law-green.txt` (the session-4 run, finished 00:48 after 75 m 17 s, 73 of them
lock wait) ends:

```
test …::an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 810 filtered out; finished in 0.29s
```

Re-run here against the current tree, together with every other ingress law in the crate
(`🗑️generated/r2-s5-ingress-family.txt`, 23.37 s):

```
test result: FAILED. 20 passed; 1 failed; 0 ignored; 0 measured; 790 filtered out
…::an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving ... ok
…::a_long_command_stream_never_pins_the_retained_ingress_authority ... ok   (peak ingress occupancy 0)
…::command_ingress_terminal_tests::* (6) ... ok
…::cold_pair::tests::cold_pair_ingress_* (6) ... ok
failures: component::app::child_member_registry_tests::owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally
```

The one red is **document** ingress in `child_member_registry_tests`, not command ingress: a
different authority, a peer's area, untouched here.

**Method note worth keeping.** `cargo test -p semio-framework-plugin` costs 75 min under this fleet
almost entirely in lock wait, and the built test binary can be run directly with no cargo and no lock:

```
RUST_MIN_STACK=67108864 \
  .🧬semio/🦑️repo/⚡️cache/cargo/build/debug/build/semio-framework-plugin/<hash>/out/semio_framework_plugin-<hash> \
  <filter> --nocapture --test-threads=1
```

`RUST_MIN_STACK` is **required**: it comes from `.cargo/config.toml`'s `[env]`, so a direct run
without it dies with `has overflowed its stack` before the first assertion (measured).

### 10.2 The reactor test file cannot complete — and the abort is not this slice's

`cargo test -p semio-framework-plugin --lib plugin_builder_contract_tests`
(`🗑️generated/r2-s5-reactor-file.txt`): **223 tests**, run aborted with `SIGABRT` at
`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` — an assertion failure whose
unwind then hits `⚛️reactor/🚪️lifetime/🦀️.rs:489 runtime lifetimes require terminal exact ACK before
teardown` in a destructor, i.e. `panic in a destructor during cleanup` → non-unwinding abort. Every
test alphabetically after `a_settled…` — including this slice's law — is never reached in a file run.

| reading | value |
| --- | --- |
| F1's baseline, same law, 2026-09-19 11:33 (`🗑️generated/f1-retention-alone.txt`) | `per_turn=-128 B` — green |
| this slice, in the 223-test file run | `per_turn=211 B` |
| this slice, law alone (`🗑️generated/r2-s5-retention-alone.txt`) | `per_turn=83 B` — ceiling is 64 B |

**Not this slice's regression**, on three measurements: (a) the law polls a settled reactor with an
empty event vector, so no command exists and **both** ingress lanes are `None`; the only new per-turn
call, `step_retiring_command_ingress()` (`🔄️turn/🦀️.rs:869`), is a `take()` on `None` — a no-op that
allocates nothing; (b) `a_long_command_stream_never_pins_the_retained_ingress_authority` passes and
reports `peak ingress occupancy 0` over 320 commands, so the lanes drain; (c) the reading is not
stable between runs of the same binary (`211` in-file vs `83` alone, and the same law's sibling
reports `-68 B/command` in one run and `+88 B/command` in the other), which a structural leak in a
fixed two-slot array would not be. Twelve hours of peer framework edits sit between F1's green and
this red. Reported to F1's owner, not touched here.

Other reds in that file, all pre-existing and all the same peer defect — `Fault{ code:
"interactive-job.missing-factory", message: "typed command '<verb>' has no exact
controller/owner/factory/tool/schema proof" }` on `compositeEdit`, `setLabel`, `increment`,
`a_pick_is_never_undoable…`, `a_child_survives_a_full_persist_and_reload_cycle…` — the classification
gate, not the command wire.

### 10.3 The guest fix is live, and the retained-owner blocker is GONE

The guest half of §4 (fixes 2/3) only exists in a rebuilt `.wasm`, so the chain needed a real
component carrying it:

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo rustc -p semio-s-plugin-draw --target wasm32-wasip2 --profile wasm-dev
    Finished `wasm-dev` profile [unoptimized] target(s) in 21m 19s
```

(`🗑️generated/r2-s5-build-draw.txt`; the component uplifts straight into the directory the gateway
reads, `🏠️workspace/🦀️.rs:88 PLUGIN_WASM_TARGET_DIR`.) `🐍️r2-wasm-stack-size.py` re-run on the staged
set confirms the new `draw` carries **8 MiB**, so §7's link-arg finding still holds for it.

Then the MCP binary — which nx's `build` target caches on `🌉️mcp/**/*.rs` inputs that do **not**
include the framework crates it links, so an nx-cached binary can silently predate a reactor fix.
Built through the underlying verb instead of the nx target (`bun ./📜️script.ts build`, preamble rule
16), which stages with the repo's own `rm`+copy+sign path: 71 736 976 B at 02:06, `codesign -v`
clean.

**The e2e gate, `🗑️generated/r2-s5-client-e2e.txt` — `12/16 steps green`:**

```
PASS  os: initialize / tools/list (27, all 9 required) / tools/list pagination walk
PASS  os: tools/list rejects a foreign cursor / resources/list pagination + unique URIs
PASS  os: capabilities_search — hits=20 first=block.s.block.block2d@1/*#editor.edit
FAIL  os: capability catalog health — 30 diagnostic(s)   (A1 §3, descriptor regeneration)
PASS  os: artifact_create — artifactId=mcp-client-e2e-mu923sgo revision=1
PASS  os: artifact_open — kind=os.agent.probe/v1 sizeBytes=394
PASS  os: capabilities_describe — draw.s.draw.drawing@1/*#editor.setSnapshot
FAIL  os: artifact_create (a real plugin artifact kind) — INTERNAL: `draw` refused ReadArtifact
      (budget.exceeded): InstanceOpen yielded after the 8 ms owned-interpreter slice; retry to resume
FAIL  os: artifact_export — PLUGIN_UNAVAILABLE (the create above never landed)
FAIL  os: action_prepare — BUDGET_EXCEEDED: InstanceOpen yielded after the 8 ms owned-interpreter slice
PASS  repo: initialize / resources/list (8) / resources/read repo://goals (9222 bytes)
```

**The measured movement is the whole point of this slice.** The same three plugin rows failed for A2
with `command acknowledgement: plugin.command-cursor-mismatch: … owner is 1, the retained owner's is
554`. That string **does not occur anywhere in this run**. The retained command-ingress owner no
longer answers for a command the host has stopped driving: §2's two defects are closed at runtime,
not just in a unit law. The rows now fail *further along*, on a different mechanism — one open that
never settles.

### 10.4 The new blocker, root-caused: a UI-frame budget on a headless open

`ensure_instance` (`🏠️workspace/🦀️.rs:948`) pumps the cold open in slices of
`owned_interactive_budget()` — `deadline_ms: 8` — under a 120 s `INSTANCE_OPEN_WALL_BUDGET`. Both
failing rows consumed that entire wall (the gate's own two journeys took ~9 min, ≈ 4 × 120 s), i.e.
**~15 000 slices without settling**.

The 8 ms figure buys exactly one thing: a UI frame that stays responsive while a plugin comes up. A
headless MCP gateway has no frame — nothing can observe the instance until the open settles anyway —
and the slice is not free: every one re-serialises `OwnedPollInput` through the guest's `Allocate`
export and boxes a fresh turn future, so an open that is ~25 s of interpretation (A2's measurement
for `🗒️note`) pays that round-trip ~3 000 times and spends the wall on overhead rather than on the
guest. `🗒️note`, the smaller app, fits inside 120 s; `🖍️draw` does not.

This is the same argument, on the same wire, that A2 already accepted for the inference lane:
`headless_inference_budget()` exists precisely because "`owned_interactive_budget`'s 8 ms slice
exists to keep a UI frame responsive; the inference lane has no frame to keep".

**Fix (landed, `🏠️workspace/🦀️.rs`):** new `headless_open_budget()` — the inference lane's ceilings on
a headless job's time scale (`deadline_ms: 30_000`, `fuel: 2_000_000_000`, `max_effects: 4096`,
`max_frames: 4096`) — and `ensure_instance` now runs its slices under it. The 120 s wall is
unchanged, so a wedged guest is still stopped and still reported as a bounded `budget.exceeded`;
only the number of round trips inside that wall collapses. `budget_fault`'s message no longer
hard-codes "the 8 ms owned-interpreter slice", which was now false for this caller, and
`ensure_instance`'s doc-comment claim that it is bounded by `COMMAND_RESUME_WALL_BUDGET` is
corrected to `INSTANCE_OPEN_WALL_BUDGET`, which is what the code actually reads.

### 10.5 A peer's mid-edit break, unblocked

`bun ./📜️script.ts build` first failed with two errors in `🌉️mcp/💡️inference/🦀️.rs`
(`🗑️generated/r2-s5-build-mcp.txt`): `GisMapInferenceSubmitRequestV1::new` had gained a leading
`service_id` parameter and the call site at `:1715` (`gis_map_hub_inference_read`) still passed two
arguments. Fixed by mirroring the peer's own pattern at `:1386` — resolve the route from the
artifact's descriptor (`resolve_hub_inference_route`) and pass `route.service_id` — rather than
hard-coding the constant, which is what that function's own docstring forbids ("the artifact's own
descriptor kind decides which hub-backed service this intent names, never the call site").

### 10.6 Session 5b — the open's remaining cost is the INTERPRETER, measured

The coordinator killed a 4 h cargo deadlock at 06:12 (preamble rule 23) and this slice resumed. Three
more gate runs, each after a real rebuild + restage of the binary:

| run | slice budgets in force | `artifact_create` (plugin kind) / `action_prepare` |
| --- | --- | --- |
| `r2-s5-client-e2e.txt` | 8 ms everywhere | `InstanceOpen` … budget.exceeded / same |
| `r2-s5b-client-e2e.txt` | `headless_open_budget` (30 s) on `ensure_instance` | **`AppCommand` … budget.exceeded** / `InstanceOpen` |
| `r2-s5c-client-e2e.txt` | + `headless_command_budget` (5 s), command wall 15 s → 60 s | `InstanceOpen` / `InstanceOpen` |

The s5b row is the informative one: with the UI-frame quantum removed from the open, `draw`'s
`InstanceOpen` **settled** and the failure moved one layer out, into the `AppCommand` lane — which
still carried the same 8 ms quantum, so `headless_command_budget` was added for the same reason
(§10.4). s5c then fell back to `InstanceOpen` at a higher machine load: the open sits right at the
120 s wall and which side of it lands depends on contention.

**The decisive measurement.** `INSTANCE_OPEN_WALL_BUDGET` was raised to 420 s purely to ask "slow or
livelocked?" (`🗑️generated/r2-s5d-client-e2e.txt`). The answer came from the *client*: the gate's own
per-call budget is **240 000 ms** (`🌉️mcp/🟦️.ts:258`), so the journey did not produce a red row at all
— it aborted with `tools/call did not answer within 240000ms` while the server was still pumping.
`draw`'s cold open therefore needs **more than 240 s of interpretation**, which is more than the MCP
client will ever wait. The wall was reverted to 120 s, which is the correct value precisely because
it is under the client's budget: it yields an honest typed `budget.exceeded` instead of a dead tool
call.

**Root cause, and it is A1 §5.1's, now measured rather than suspected.** The MCP gateway is the only
caller that opens plugin instances through `OwnedRuntime`, the repo-owned **interpreter**; the os dev
host boots ~20 plugins through `WasmtimeRuntime` in seconds, and A2's own law
`wasmtime_runtime_instance_open_settles_against_a_real_plugin_component` proves that runtime opens a
real component. `🗒️note` (smaller app) settles under the interpreter; `🖍️draw` does not, and no budget
tuning can close a gap of that size. The three headless budgets in §10.4 remove the round-trip
overhead that was *multiplying* the cost — they are right, and they moved the failure one layer out
— but they cannot make an interpreter fast enough.

**Why this slice did not do the port.** `PluginArtifactChannel` holds `Arc<OwnedRuntime>` and drives
its **sync, inherent, resumable** API (`execute_actor_turn`, `turn_in_flight`, plus
`retryable_lifecycle_turn` and the `undelivered` queue). `GuestRuntime` — the trait `WasmtimeRuntime`
implements — is **async and has no `turn_in_flight`**: under wasmtime a turn is atomic, so the whole
mid-flight-admission pump A2 built for `ensure_instance` has no meaning there and would have to be
rewritten, not re-pointed. That is a redesign of the channel's runtime seam, not a field swap, and it
is the single highest-value next slice for outcome 4.

### 10.7 The channel test on a rebuilt `note` — the cursor mismatch is gone there too

`🗒️note` was still the pre-fix component (built 2026-09-19 03:57, before the reactor edit), so the
cheapest end-to-end check was meaningless until it was rebuilt:

```
CARGO_PROFILE_WASM_DEV_DEBUG=false cargo rustc -p semio-s-plugin-note --target wasm32-wasip2 --profile wasm-dev
    Finished `wasm-dev` profile [unoptimized] target(s) in 10m 11s          (🗑️generated/r2-s5-build-note.txt)

cargo test -p semio-framework-os-mcp --lib plugin_artifact_channel_mutation_verbs -- --nocapture
[W3] real PureCommand round trip: Err(Fault { code: "budget.exceeded", message:
     "InstanceOpen yielded on its owned-interpreter wall budget before it settled; retry to resume" })
[W3] real TransactionPrepare round trip: Err(Fault { code: "channel.not-wired", message:
     "command ingress: faulted for seq 1 after bounded exact-owner cleanup" })
test result: FAILED. 0 passed; 1 failed; … finished in 259.25s              (🗑️generated/r2-s5e-channel-test.txt)
```

Two readings, both measured:

1. **`plugin.command-cursor-mismatch` does not appear.** A2 §6.1's blocker — the whole subject of
   this slice — is gone on this wire as well as on the gate's.
2. **`🗒️note`'s open now also exceeds the 120 s wall.** A2 measured that same open at ~25 s on a calm
   machine; this run was under 26 live `rustc` at load ~27. The interpreter's open cost is
   load-sensitive by roughly an order of magnitude, so under a fleet even the *small* plugin no
   longer comes up inside a budget the MCP client will wait for. §10.6's conclusion is not a `draw`
   size problem; it is the interpreter.

The second `[W3]` line is a consequence of the first: with the open never settled, the command's
ingress owner is cleaned up and the next verb is refused by that cleanup, not by the cursor rule.

## 11. Session 5 — gate numbers, files, honest gaps

**Final MCP client e2e gate: `12/16` steps green** (`🗑️generated/r2-s5c-client-e2e.txt`, the run whose
binary matches the tree as it now stands). A1 recorded `13/16` but its own transcript shows the same
`12 PASS / 4 FAIL`; the four red rows are unchanged by name — `capability catalog health`,
`artifact_create (a real plugin artifact kind)`, `artifact_export`, `action_prepare` — and three of
the four now fail for a *different, later* reason than they did for A1 and A2.

| red row | A1 (09-19) | A2 (09-19) | R2 now |
| --- | --- | --- | --- |
| catalog health | 31 skips | 30 skips | 30 skips — A1 §3, descriptor regeneration, untouched |
| `artifact_create` (plugin kind) | `guest trapped: memory write out of bounds` | `plugin.command-cursor-mismatch` | `budget.exceeded` on the cold open |
| `artifact_export` | no binding | no binding | no binding (cascades from the create) |
| `action_prepare` | `guest trapped` | `plugin.command-cursor-mismatch` | `budget.exceeded` on the cold open |

The mutation chain the brief asks for (`artifact_create/open` → `action_prepare/invoke` →
`artifact_snapshot` → `history_undo/redo` → `transaction begin/rollback/commit` → `artifact_export`)
is **not green**, and the reason is now a single measured one: no plugin instance can be opened
through the interpreter inside the client's own 240 s tool-call budget (§10.6, §10.7). Everything
this slice owned — the retained command-ingress owner — is closed at runtime.

### Files changed this session

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs` | `headless_open_budget()` + `headless_command_budget()`; `ensure_instance`, `activate_plugin_instance` and `exchange_one_slice` moved off the 8 ms UI-frame quantum; dead `owned_interactive_budget()` removed; `COMMAND_RESUME_WALL_BUDGET` 15 s → 60 s; `budget_fault`'s message no longer hard-codes "8 ms"; `ensure_instance`'s doc corrected to name `INSTANCE_OPEN_WALL_BUDGET` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs` | `:1715` call site updated for a peer's new `service_id` parameter, routed via `resolve_hub_inference_route` (§10.5) |

Captures added (delete with `🗑️generated`): `r2-s5-reactor-file.txt`, `r2-s5-ingress-family.txt`,
`r2-s5-retention-alone.txt`, `r2-s5-retention-phases.txt`, `r2-s5-build-draw.txt`,
`r2-s5-build-note.txt`, `r2-s5-build-mcp.txt`, `r2-s5-client-e2e.txt`, `r2-s5b-build-mcp.txt`,
`r2-s5b-client-e2e.txt`, `r2-s5c-build-mcp.txt`, `r2-s5c-client-e2e.txt`, `r2-s5d-build-mcp.txt`,
`r2-s5d-client-e2e.txt`, `r2-s5e-build-mcp.txt`, `r2-s5e-channel-test.txt`, `r2-s5f-build-mcp.txt`.

### Honest gaps

1. **`headless_command_budget` and the 60 s command wall are not runtime-verified.** They were
   derived from the s5b run, where the command lane was the failing layer; s5c and the channel test
   both fail *before* reaching it, so no run has yet exercised a command under them. The reasoning is
   the inference lane's own accepted precedent, but it is reasoning, not a measurement.
2. **The `WasmtimeRuntime` port is not done** (§10.6) and is the single next step for outcome 4.
   `GuestRuntime` is async and has no `turn_in_flight`, so `ensure_instance`'s mid-flight pump must
   be rewritten rather than re-pointed.
3. **`a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford` is red and aborts the whole
   `plugin_builder_contract_tests` file** (§10.2). Shown not to be this slice's, but it means no one
   can run that file to completion until F1's owner fixes it — every test alphabetically after
   `a_settled…` is unreachable in a file run.
4. **`owned_document_ingress_is_heap_backed_…`** (§10.1) is red in `child_member_registry_tests`;
   document ingress, not command ingress, not investigated.
5. **13 staged components still carry a 1 MiB stack** (§7 table minus `draw`/`note`, both now 8 MiB).
   Unchanged this session.
6. **§8's gaps 3 and 4 stand**: supersession is keyed on the instance rather than a per-instance
   lane, and `command_ingress_while_owned`'s mid-assembly `CommandPending` is still untouched.

---

## 12. Session 5c — the mutation chain on M7's LIVE gate

Coordinator direction: the live bridge, not the headless runtime, is the acceptance path for
outcome 4. M7's gate (`@semio-tech/framework-os-mcp-rs:live-agent-loop-check`,
`🌉️mcp/🧪️tests/🤖️live-agent-loop/🟦️.ts`) was extended with the chain, additively — one inserted block
between (c) and (e), nothing else in the file touched, so M5a's concurrent approval-step work does
not conflict.

### 12.1 The staged components carry the fix

```
semio_s_plugin_note.wasm  09-20 07:03  grep -ac 'the superseded owner was retained' → 1
semio_s_plugin_draw.wasm  09-20 01:59  grep -ac 'the superseded owner was retained' → 1
```

(`the superseded owner was retained` is the `expect` string from §4 fix 3's supersession arm, so its
presence in the component IS the guest fix.)

### 12.2 The eight new steps

`(f1) artifact_create/open` → `(f2) action_prepare` → `(f3) action_invoke changes the head` →
`(f4) artifact_snapshot shows the change` → `(f5) the live shell shows the same artifact` →
`(f6) history_undo/redo` → `(f7) transaction begin/rollback/commit` → `(f8) artifact_export`.

Each leg asserts on returned wire data and states its own cascade reason rather than inheriting a
green. The wire shapes are the handlers' own, read off `🌉️mcp/🦀️.rs` and `🗿️artifact/🦀️.rs` rather
than guessed — `artifact_create{artifactId,kind}`, `action_prepare{capabilityId,input}` →
`preparedHandle`, `action_invoke{preparedActionHandle}` → `status`/`revisionBefore`/`revisionAfter`/
`undoToken`, `history_undo{undoToken}` (keyed by the invocation, not the artifact — an agent may only
walk back a mutation it can name), `transaction_begin{preparedHandles[]}` → `transactionHandle`, and
a fresh `action_prepare` per saga leg because a prepared handle is consumed by its invoke.

### 12.3 Measured — `🗑️generated/r2-s5i-live-agent-gate.txt`

**`6 passed, 9 failed, 4 skipped of 19`** against the real `s` host on `:6070`, driving the
`.mcp.json`-verbatim stdio gateway.

| M7's original 11 | this run |
| --- | --- |
| 0 rendezvous, (a) dial, (a) presence, (d) ui_reveal, (b) tool-call rows, (c) Cancel | **PASS** (6) |
| boot, (d) ui_focus | **FAIL** — `ready=null error=s windows=` : `:6070`'s signed-out Home does not boot a shell session (S2's in-flight fix), so there is no window to focus. Not this slice's, and not the mutation chain's. |
| (e1)/(e2)/(e3) approval | **SKIP** — unchanged, M7 §5.4's authoring gap |
| **the 8 new (f) steps** | **1 SKIP + 7 FAIL, all one cause** |

```
FAIL (f1) artifact_create/open :: kind=s.note.note: INTERNAL: `note` refused ReadArtifact
     (budget.exceeded): InstanceOpen yielded on its owned-interpreter wall budget before it settled
FAIL (f2)…(f8) :: cascaded
SKIP (f5) the live shell shows the same artifact :: the gateway mutates its own --folder workspace
     and the shell owns a different document: no [data-semio-artifact-id="live-agent-loop-…"] in the
     live DOM. The gateway has no shell-bound document route — only ui_reveal/ui_focus cross the
     bridge (🌉️mcp/🖥️ui). Wire one and this step runs.
```

**Two findings, both measured, both bigger than a budget number.**

1. **The live lane does not bypass the headless runtime.** `plugin.command-cursor-mismatch` is gone
   here too, but `action_prepare`/`action_invoke` resolve through `ActionAdapter` → `ArtifactChannel`
   → `PluginArtifactChannel` (`OwnedRuntime`) exactly as the headless `client-e2e` does. Only
   `🌉️mcp/🖥️ui` is shell-bound (`ui_reveal`/`ui_focus`) — grepped across the crate: no other module
   references the shell connection. So M7's "mutation verbs remain red" was **two** faults stacked,
   not one: the retained-owner fault this slice removed, and underneath it the interpreter's open.
2. **(f5) is a missing route, not a failing one.** "The agent's change is the change a human sees"
   cannot be true today for any plugin: the gateway's `--folder` workspace and the shell's own
   document are different owners with no bridge between them. The step is recorded as a
   runtime-checked SKIP in M7's own precondition style (the DOM really is queried for the artifact's
   id), so it turns green by itself the day that route exists and can never silently pass.

### 12.4 Shell-host caveat

`:6080` (M5a's note serve) **boots** but failed `0 rendezvous` when I drove it
(`🗑️generated/r2-s5h-live-agent-gate.txt`, `2 passed, 13 failed, 4 skipped`) — M5a is driving the
same gate against the same serve, and the gateway offer is a single per-shell rendezvous. `:6070`
rendezvouses cleanly but does not boot signed-out. Neither is this slice's defect; the numbers above
are from `:6070` because the (f) chain needs the gateway, not the DOM. Once S2's signed-out Home
lands, the same command re-run on `:6070` should carry `boot` and `(d) ui_focus` as well.

### 12.5 Headless `client-e2e` with the new budgets, on a calm machine — NOT obtained

The coordinator asked for a calm-machine number. The machine did not become calm inside this
session: load averages were **92 → 77** with 10–48 live `rustc` throughout (`uptime` sampled at
08:26 and 08:49). The honest statement of what is known: the budgets of §10.4 are proven to move the
failure one layer out (§10.6's s5b row) and are proven NOT to be sufficient, because §10.6's 420 s
experiment showed the open needs more than the client's own 240 s budget. A calm-machine run would
change how far past the open it gets, not whether the open is viable. The last full headless number
stands at **12/16** (`r2-s5c-client-e2e.txt`).

---

## 13. The `WasmtimeRuntime` seam — design for its owner (§11 gap 2)

1. **Subject.** `PluginArtifactChannel` (`🌉️mcp/🏠️workspace/🦀️.rs`) holds `runtime: Arc<OwnedRuntime>`
   and is the ONLY caller in the repo that opens plugin instances through the interpreter. The os dev
   host opens ~20 through `WasmtimeRuntime` in seconds. Measured gap: `note` ~25 s calm / >120 s under
   load, `draw` >240 s — against a client budget of 240 s (§10.6, §10.7).
2. **Two APIs, not one.** `OwnedRuntime` exposes **sync inherent** methods — `compile_component`,
   `instantiate_actor(compiled, actor)`, `turn_in_flight(inst)`, `execute_actor_turn(inst, events,
   budget)`. `WasmtimeRuntime` implements the **async trait** `GuestRuntime` (`🖥️host/🦀️.rs:679`) —
   `compile`, `instantiate(compiled, actor, caps, budget)`, `execute_turn`, `start_job`/`step_job`/
   `cancel_job`, `checkpoint`/`restore`, `drop_instance`. The port is therefore a rewrite of three
   call sites plus one loop, not a field swap.
3. **Async.** The crate already blocks at this boundary (`semio_framework_async::block_on` at
   `🏠️workspace/🦀️.rs:921` on the inference lane); reuse it rather than making `ArtifactChannel` async,
   whose trait is sync and is implemented by the mock too. `WasmtimeRuntime::new(SharedEngineConfig)`
   is async as well and should be built once per gateway, beside `ensure_inference_route`'s runtime.
4. **Capabilities move earlier.** `instantiate` takes `caps` and `budget`; today `ensure_instance`
   builds the `BrokerCapabilityGrant` vector *after* `instantiate_actor` and passes it inside
   `Event::InstanceOpen`. Build it before, pass it to `instantiate`, and keep sending it in the event
   as well — the guest reads it there.
5. **The pump collapses, and this is the load-bearing simplification.** `ensure_instance`'s
   `undelivered` queue, `turn_in_flight` guard and `retryable_lifecycle_turn` re-queue all exist
   because an owned turn can yield MID-FLIGHT and refuse new events until it settles. A wasmtime turn
   is atomic: it returns a `TurnResult` or faults. The loop becomes — send `InstanceOpen`; if the turn
   published an `ActorInstanceLifecycleReceipt`, send the `InstanceLifecycleAck` on the next turn;
   stop when `TurnStatus::Idle` and nothing is owed. Keep the ack: it is A2's real fix (an open the
   host never acknowledges is re-offered every turn) and is runtime-independent.
6. **Keep unchanged.** A2's trap poisoning and half-open drop (a trapped guest's memory is undefined);
   the `pending_exchanges`/`pending_command_closes`/`rejected_command_builds` registries, which are
   host-side command bookkeeping and never touch the runtime; and §10.4's `headless_open_budget`/
   `headless_command_budget` — wasmtime enforces `deadline_ms` through `EpochTicker`, and A2 already
   measured that an 8 ms epoch deadline kills a real guest call (`epoch deadline exceeded`), so those
   budgets are the right shape for either runtime.
7. **Expected wins beyond speed.** `WasmtimeRuntime::compile` caches the compiled component on disk
   (`compiled_cache_path`, keyed by engine-config hash + package hash), so only the first open in a
   machine's life pays compilation; and `INSTANCE_OPEN_WALL_BUDGET` can then drop well under the
   client's 240 s instead of being pinned just below it.
8. **Acceptance, already written.** A2's law
   `wasmtime_runtime_instance_open_settles_against_a_real_plugin_component`
   (`🔌️plugin/🖥️host/🧪️tests/🔬️owned-instance-open/🦀️.rs`) proves that runtime opens a real component;
   extend it to `draw`, then the bar is this slice's own two gates going green on their plugin rows:
   `client-e2e` (§11) and `live-agent-loop-check`'s (f1)–(f8) (§12).
9. **Risks to check first.** Whether `GuestInstance` under wasmtime is `Send` where
   `PluginArtifactChannel` is held (it carries a `Store`); and whether `drop_instance` must be driven
   on close, since `OwnedRuntime` has no such call and the channel currently just drops the value.
