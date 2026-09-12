# Wave B52 — the reconcile retirement ladder never reaches its terminal

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B52, 2026-09-12. Written incrementally.

Predecessors: `📓️2026-09-13-wave-B48-nakagin-selection-lane.md`, `📓️2026-09-13-wave-B50-camera-lane-regression.md`,
`📓️2026-09-11-wave-B2-continuation-ceiling.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, shared live tree. No state-modifying git command, no
  worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, every command foreground. The ticket is NOT closed or
  reopened by this wave. Nothing under `🗑️generated` was deleted.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- `RUST_MIN_STACK=134217728` on every law run.

## 1 Where the module actually is

The brief's locator (`find 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin -type d -name '♻️reconcile'`)
returns NOTHING. The reactor has no `♻️reconcile` module; the peer's file is in the UI runtime:

| what | path | state |
| --- | --- | --- |
| the peer's `♻️reconcile` | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs` | mtime **2026-09-12 17:07:05**, git `RM` |
| its HEAD original | `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` | renamed away |

So the 17:07 edit is a **taxonomy relocation**, and rename-aware diffing shows it is almost pure: of the
19 changed lines, every one but a single call is a `#[path]` fix (`../../…` → `../…`). The one substantive
line is `record.children.release_empty_page()` → `release_empty_page(SURFACE_RECONCILE_PAGE_BYTES)`, an
adaptation to a byte grant another peer had just added to the list primitive
(`🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`, git `A`, mtime 17:55).

**The peer did not break the ladder.** Neither did B48: its two diffs (`🩹️patches/🦀️.rs` — the `release()`
refactor and `reserve_refusal`; `🔄️turn/🦀️.rs` — `redirty_acknowledged_deferred_surfaces`, the refused-commit
and deferred-capacity faults) are orthogonal to the failure below, which is why the laws were red with
B48's change on and off. No peer hunk was reverted.

## 2 Root cause — a retirement grant narrower than one indivisible allocation

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document/🦀️.rs:41` `UiDocumentArena::retire_exact`

The retirement ladder of every published surface ends in the document arena. Read from the top:

| # | hop | file:line |
| --- | --- | --- |
| 1 | the reactor steps the closing instance's terminal | `⚛️reactor/🩹️patches/🦀️.rs:887` `PatchTracker::close_step` → the `terminals` branch at `:995` |
| 2 | the terminal retires against a page grant | `🧠️runtime/♻️reconcile/🦀️.rs:3355` `SurfaceReconcileTerminal::close_step_with_grant` → `:2564` `close_admitted_run` → `:2548` `close_run` → `:2485` `close_unit` |
| 3 | `close_unit` retires the retained reconciler | `♻️reconcile/🦀️.rs:2508` `current.retire_one()` |
| 4 | which retires the canonical document read, at a FIXED grant | `♻️reconcile/🦀️.rs:407` `document.close_read_step_with_grant(1, SURFACE_COMPONENT_COPY_WORK_BYTES)` — **4 096 bytes** (`:2206`) |
| 5 | → the document owner ladder | `♻️retirement/🌳️typed/📃️document/🦀️.rs:147` → `:131` `arena.retire_exact(exact, maximum_bytes)` |
| 6 | → the typed cursor over the node table | `:47` `slot.retirement.advance(&mut slot.nodes.entries, 1, maximum_bytes)` |
| 7 | → an empty page is freed WHOLE or not at all | `🌱️value/📋️list/🦀️.rs:357` `if bytes > maximum_bytes { return Ok(PagedListProgress::default()) }` |

Measured on `patches::tests::mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes` with a
temporary `[DEBUG]` tap at hop 7 (removed):

```
67667 [DEBUG] release_empty_page leaf-starved bytes=6416 grant=4096 reserved=1 item=6416
```

**One `UiNodeRecord` is 6 416 bytes; the grant is 4 096.** The page holds ONE reserved slot, so its backing
`Vec` is a single indivisible 6 416-byte allocation. `release_empty_page` is contract-bound to refuse a
grant it does not fit (its own laws pin that: `📋️list/🧪️tests/📋️list/🦀️.rs:53` requires
`87 * size_of::<u64>()` NOT to progress and `88 *` to progress), so it answers
`PagedListProgress::default()` — and that is a ceiling **no amount of further retirement can ever meet**.
The page stays reserved, `terminal_is_empty()` (`= root.capacity() == 0`) stays false, and every owner
above it answers no progress with no completion, forever:

```
67667 [DEBUG] retire_exact scalar=0 step=UiValueRetirementStep { complete: false, progressed: false, … } entries_empty=false
67667 [DEBUG] close_document_owner retire_exact step=UiValueRetirementStep { complete: false, progressed: false, … }
68860 [DEBUG] current.retire_one document=true assembly_empty=true ordinals=false key_index=53 retire_scalar=0
65532 [DEBUG] close_admitted_run outcome=false current=true source=false … handback=true
```

and at the top of the ladder the tracker is frozen in exactly B50's shape — one terminal that will never
empty, so the closing instance never completes and the actor answers `more-work` for ever (the drain helper
now prints this state, §4):

```
instance 1 did not reach terminal empty: slots=[] ready=[] terminals=[g1:c--] producer_terminals=[]
  deferred=[] rejected=0 unadmitted=0 closing=1 output_fault=none reserve_refusal=none
  generation_exhausted=false close_cursor=0
```

`terminals=[g1:c--]` reads: close requested, no fault, **`terminal_is_empty() == false`**. That is B50's
`more-work streak` 2 323 → 7 843 with `sources=["reconcile"]` and every retained surface stuck at its boot
revision: the guest is not idle and not faulted, it is retiring a page it is forbidden to free.

The reconciler's grant constant carries a compile-time guard for the value it was WRITTEN for and none for
the value it actually retires (`♻️reconcile/🦀️.rs:2207`):

```rust
const _: () = assert!(size_of::<ui_contract::Component>() <= SURFACE_COMPONENT_COPY_WORK_BYTES);
```

`Component` fits in 4 096; `UiNodeRecord` (6 416) does not, and nothing checked it.

## 3 Fix

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document/🦀️.rs`

1. **`retire_exact` prices its grant from the value it retires, not from the caller's constant** —
   `let grant = maximum_bytes.max(slot.nodes.entries.allocated_bytes());`. The caller's grant bounds the
   ITEMS retired per call, which stays one either way (`advance(root, 1, grant)`), so raising the byte
   ceiling to the table's own footprint costs no extra work per call and makes an indivisible page always
   admissible. The list primitive and its byte-refusal laws are untouched.
2. **A no-progress streak is named, not silent** — `UiDocumentSlot::stalled` counts consecutive steps that
   report neither progress nor completion and faults at `UI_DOCUMENT_RETIREMENT_STALL_LIMIT = 4_096`
   (`"document retirement made no progress for its whole stall budget"`). A single no-progress step is
   ordinary — measured: `UiPatchOps` answers one while a descendant waits on an owner a later step
   releases — so the cap is a page of steps, three orders of magnitude above any legitimate transient and
   four below the 67 667 that were measured.

## 4 Laws

`⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs:138` — the shared drain helper `close_instance_to_empty` panicked
with no state at all; it now names the tracker it is stuck in (`panic!("instance {instance} did not reach
terminal empty: {}", tracker.debug_state())`). That one line is what turned a silent 65 536-turn hang into
the census quoted in §2.

Two laws added, `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`:

- `an_alias_window_surface_the_host_never_acknowledges_does_not_block_the_retirement_ladder` — the
  `1:window` alias publishes, the host never acknowledges its revision, and the instance's close ladder
  must still reach terminal empty. (B50's census is quoted in its docstring, including the reading that
  `outSome(0)` is output SLOT 0, not a zero-byte output.)
- `a_node_record_wider_than_the_copy_grant_still_retires_its_document_to_terminal_empty` — a
  document-scaled surface publishes, retires, and its whole ladder must reach terminal empty; the failure
  message prices the stall in node records.

### 4.1 Red → green, and the neutralised baseline

`RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- patches:: reconcile_budget --test-threads=1`

| run | result |
| --- | --- |
| baseline, HEAD + B48 (`🗑️generated/wave-B52-laws-baseline.txt`) | 4 × `patches::tests` FAILED (`issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner`, `mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes`, `mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots`, `mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner`), then `a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns` **spinning past the 600 s budget** |
| with the fix (`🗑️generated/wave-B52-laws-final.txt`) | `test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 649 filtered out; finished in 0.59s` |

The three `did not reach terminal empty` failures and the fourth (`UiResidentSnapshot … used_slots: 4`
against `1`) are ONE defect: the first three leave their document slot's resident permit held, and the
fourth then reads the leaked permits.

With ONLY the grant escalation neutralised (`let grant = maximum_bytes;`, restored immediately —
`🗑️generated/wave-B52-laws-neutralised.txt`), the new ladder law goes red **by name** instead of spinning:

```
panicked at …/♻️reconcile/🦀️.rs:406:90:
canonical read retirement preserves exact fault authority:
  "document retirement made no progress for its whole stall budget"
```

so both halves of the fix are demonstrated separately: the counter converts a silent 67 667-step spin into
a named fault inside 4 096 steps, and the grant removes the stall altogether.

## 5 Live census — `:6013`, wasm #60, 2026-09-12 19:59

`🔍️b52-ladder-census.ts` (kept; runtime diagnostics armed before the first module evaluates, one Alt+right
orbit, then a 30 s watch). Log: `🗑️generated/b52-2026-09-12T19-59-34-wasm60.md`.

**Boot is healthy.** All 13 surfaces of instance 1 reach a clean, acknowledged, reservable slot:

```
boot maxStreak=34 effectlessTurns=44
boot census x10 slots=[1:puzzle3d-main#g1:--R:ack1/rev1:outNone,1:puzzle3d-main-top#g2:--R:ack1/rev1:outNone,
  1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone,1:window#g4:--R:ack1/rev1:outNone,…13 surfaces…]
  ready=[] terminals=[] deferred=[] output_fault=none reserve_refusal=none
boot namedFaults=0
```

**One orbit freezes it, and the census names the hop.** 6 035 consecutive turns in ONE state:

```
orbit rigMoved=false publishedMoved=false waitedMs=42211
orbit maxStreak=6169 effectlessTurns=6123
orbit sources ["reconcile"] x5947
orbit terminals=[] x6170
orbit reserve_refusal=1:framework.section.catalogue:registry-reservation-unavailable x6035
orbit reserve_refusal=1:window:slot-output-held x5
orbit census x6035 slots=[…,1:window#g14:-J-:ack1/rev0:outSome(0),…] ready=[g14:--r-] terminals=[]
  producer_terminals=[] deferred=[1:puzzle3d-main,1:puzzle3d-main-top,1:puzzle3d-main-perspective,1:window,
  1:framework.panel.artifact,1:framework.panel.catalogue,1:framework.panel.inspection,
  1:puzzle3d.panel.settings,1:framework.panel.history,1:framework.section.engagements,
  1:framework.section.measures,1:framework.section.tools,1:framework.section.catalogue]
  rejected=0 unadmitted=0 closing=0 output_fault=none
  reserve_refusal=1:framework.section.catalogue:registry-reservation-unavailable
orbit namedFaults=0
```

Read off it, in order:

| question (brief §1) | live answer on #60 |
| --- | --- |
| what state does the ladder sit in? | ALL THIRTEEN surfaces `deferred`, `effects=0`, `sources=["reconcile"]`, 6 169 consecutive more-work turns |
| which surface? | the refusal names **`1:framework.section.catalogue`**; the surface HOLDING the ladder is the alias **`1:window`** — `#g14:-J-` (job live, reconciler checked out) with `ready=[g14:--r-]`, an output that is neither published nor closing and still holds its reservation, unchanged for 6 035 turns |
| what does `reserve_refusal` name? | **`registry-reservation-unavailable`** — `SurfaceReconcileReservation::try_new` has nothing left to hand out (`🩹️patches/🦀️.rs:478`). Also `1:window:slot-output-held` ×5 |
| do B48's named faults fire? | **no** — `namedFaults=0`, `output_fault=none`, no `ui.surface-render-uncommitted`, no `ui.dirty-surface-deferred-capacity`, no `did not publish` |
| is `outSome(0)` a zero-byte output? | **no** — `out{:?}` prints `slot.output_index`, so it is output SLOT 0. `rev0` likewise is the debug format reading `0` from a CHECKED-OUT reconciler, not a real revision |

### 5.1 Why this is the same defect, one hop downstream

`terminals=[]` live, yet the natively measured stall is a terminal that never empties — because the stuck
retained owner is not held in a tracker terminal at all, it is held in the handback REGISTRY. That is
exactly what the native tap showed at hop 2 above:

```
65532 [DEBUG] close_admitted_run outcome=false current=true source=false … handback=true
```

`close_admitted_run` reserves a handback before retiring (`♻️reconcile/🦀️.rs:2565`), and `close_unit` can
only release it AFTER `current` drains (`:2525`) — which is the step that never progressed. So every stuck
retirement holds one registry reservation for ever, the fixed pool drains, and from then on
`reserve_mounted` refuses EVERY surface with `registry-reservation-unavailable`, defers all 13, and the
actor answers `more-work` with `effects=0`. The live refusal predicate is the downstream reading of the
native stall, and it is why nothing faults: refusal is not a fault, and B48's two faults are on the
commit and deferred-capacity paths, neither of which is taken here.

`rigMoved=false` is a second, separate reading (B50 §"gesture intercepted"): with 57 455 console lines in
60 s the diagnostics-armed page never got the drag to `OrbitControls`. It is not evidence about the guest.

## 6 Verification (every command foreground; tails quoted)

| command | result |
| --- | --- |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- patches:: reconcile_budget --test-threads=1` | `test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 649 filtered out; finished in 0.59s` (baseline: 4 failed + one law spinning past 600 s) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- camera selection --test-threads=1` | `test result: FAILED. 32 passed; 1 failed` — `gumball_active_only_for_transform_utilities_with_object_selection` (`an unattached gumball must never render`), **PRE-EXISTING**: identical failure with the fix neutralised (`🗑️generated/wave-B52-puzzle3d-gumball-baseline.txt`) |
| `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` | `EXIT=0`, **0 errors**, 117 warning lines emitted (`Finished dev profile … in 1m 48s`) |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | `EXIT=0`, **0 errors**, 127 warning lines emitted (`Finished dev profile … in 2m 20s`) |
| `RUST_MIN_STACK=134217728 cargo test -p semio-framework-ui-contract --lib` (the crate I changed) | fix: `125 passed; 45 failed`; neutralised baseline: `124 passed; 46 failed`. Failure-name diff: my change **fixes** `action::retirement::typed_tests::instance_lifetime_ui_document_terminal_includes_typed_value_descendants` and **breaks nothing** (`comm -13` empty) |

The wasm build was NOT run (the coordinator builds #61).

### 6.1 Pre-existing reds, recorded not worked around

- `semio-framework-ui-contract --lib`: 45 failures, all present with this wave's change neutralised.
  `action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`
  aborts the test process (`binding copy did not retire exact owners` → `UiValueRetirement requires exact
  terminal closure` → `panic in a destructor during cleanup`), so the suite must be run with
  `--skip retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases` to see the rest at all.
  A peer is live in that area.
- `semio-s-artifact-puzzle-3d`: `gumball_active_only_for_transform_utilities_with_object_selection`.

## 7 Verdicts

1. **Root cause** — `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document/🦀️.rs:41`
   `retire_exact` priced the document retirement ladder with the caller's fixed 4 096-byte grant
   (`♻️reconcile/🦀️.rs:407`, `SURFACE_COMPONENT_COPY_WORK_BYTES`) while the value it retires is a list of
   6 416-byte `UiNodeRecord`s, and `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:357` frees a page whole or
   not at all. **Ours, not the peer's**: the 17:07 `♻️reconcile` relocation changes only `#[path]`s and one
   call signature, and no peer hunk was reverted; B48's diffs are orthogonal, which is why the laws were
   red with them on and off.
2. **Fixed** — the grant is priced from the value (`maximum_bytes.max(entries.allocated_bytes())`), and a
   no-progress streak is capped at 4 096 steps by a named fault instead of spinning silently. The list
   primitive's byte-refusal contract and its laws are untouched.
3. **Laws** — the 4 reds and B2's nakagin ladder law are green (42 passed), plus the two new laws; the
   ladder law goes red by name under a neutralised fix.
4. **Live** — confirmed on #60 that the guest freezes after one gesture with all 13 surfaces deferred and
   `reserve_refusal=registry-reservation-unavailable`, which §5.1 ties to the same stall one hop
   downstream. **NOT yet proven live**: #61 does not exist, so
   `--only=camera-gestures,selection-surfaces --port=6013` was not run against the fix.
5. **What rides #61** — the two guest-side changes above
   (`♻️retirement/🌳️typed/📃️document/🦀️.rs` and the `UiDocumentSlot::stalled` field in
   `📃️document/🦀️.rs`). Both are in the guest's Rust, so nothing of this fix is host-reachable and #60
   cannot show it. Once #61 is deployed the census to re-read is the same one: `reserve_refusal` must
   return to `none` and the `deferred=[…13…]` state must stop recurring.
