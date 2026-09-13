# Audit — Regression Diff, `setContributions` Never Returns (2026-09-13)

Lane `audit-regression-diff`, Sonnet read-only audit. Repo/semio MCP both failed to connect
(`repo`: -32602 invalid initialize params; `semio`: CONNECTION_CLOSED) for the whole session;
bookkeeping is on disk only, ticket not opened/closed/reopened by this lane. No file was edited, no
build/test run, no server started.

Window: `git diff 8add1df147..HEAD` (4 commits: `78b716e661`@2026-09-13 00:05,
`a672f196c8`@00:30, `b2064cc237`@03:15, `5b6f77afcf`@11:27). `git diff --stat` touches ~1,690 files
across the whole monorepo (many unrelated tickets' commits landed in the same 4 commits); this report
scopes to the `setContributions` path named in the brief. Evidence for "it hung" comes from
`🗑️generated/s4-boot-check-1/console.txt` (browser, 2026-09-13 12:1x, after all 4 commits — log ends
right after `performInvocation command setContributions` / `command ingress lane seq=8`, no settle
line follows) and `🗑️generated/fix-forward-contributions/native-served-chain.txt` (native `cargo test
-p semio-s-artifact-procedural-generation3d --lib
host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`, same day, fails after
30.80 s).

## 0. Correcting the premise: the native test hangs AFTER install, not during it

The native trace shows all 31 contribution pages installing cleanly — `[MEMORY] after page 0` through
`after page 30` and `[MEMORY] after install: retained=25016366 peak=26794968` all print with no fault.
The panic —

```
panicked at …/✏️editor/🧪️tests/🔬️unit/🦀️.rs:151:185:
flowEvalTick: Fault { … message: "registered fixture typed operation did not retire within 30
seconds" … }
```

— fires from `drain_flow_eval_ticks_with_view` (`🦀️.rs:151`, the `dispatch_with_view(…
FlowEvalTick…)` call inside the `for _ in 0..1000` loop, `🦀️.rs:146`), which runs strictly AFTER
`context::dispatch(SetContributions…)` returns cleanly for every page (test body `🦀️.rs:1715–1725`).
So in the native reproducer the *install itself is not what hangs* — a subsequent `FlowEvalTick`'s
typed-operation retirement never reaches terminal. This matters because it re-centers the search away
from the contributions JSON/paging path (unchanged since Sep 12 02:57, before the regression window —
confirmed below) and onto whatever `FlowEvalTick`'s retirement touches once contributions have landed:
window-config commit, the flow retained artifact, or the plugin's `maintenance_step` dispatcher.

`"registered fixture typed operation did not retire within 30 seconds"` is ALSO on record as a
**pre-existing, non-regression** flake for one specific test
(`📓️viewer-eval-chain-2026-09-12.md:166`: `refresh_pending_effects_arms_flow_eval_tick_chain` fails
deterministically at ~32 s because the underlying BREP boolean cut alone costs 29.84 s in a debug
build, right at the 30 s ceiling). That is a different test. Treat any single hit of this message as
ambiguous between "genuinely stuck" and "debug-build geometry cost graduated a marginal budget" until
confirmed (§5).

## 1. Ranked candidates

### 1.1 TOP — `RetainedInflateHistory` demands its full 32 KiB window in ONE grant; the window-config retained loader only ever offers 4 KiB

**The mismatch, with exact sites:**

- `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs:276` — `pub const RETAINED_INFLATE_WINDOW_BYTES: usize = 32 * 1024;`
- `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs:341-345` (new this window) —
  ```rust
  fn next_allocation_bytes(&self) -> Option<usize> {
      (!self.ready() && self.allocation_fault.is_none() && self.target != 0).then_some(self.target)
  }
  fn reserve(&mut self, maximum_bytes: usize) -> Result<RetainedInflateAllocationStep, RetainedInflateAllocationError> {
      …
      let Some(exact) = self.next_allocation_bytes() else { return Ok(RetainedInflateAllocationStep::default()) };
      if maximum_bytes < exact { return Ok(RetainedInflateAllocationStep::default()); }
      if self.bytes.try_reserve_exact(exact).is_err() { … }
  ```
  `target = maximum_history_bytes.min(RETAINED_INFLATE_WINDOW_BYTES)` (line 468 of the same file, set
  once in `try_new_retained`). Unlike every OTHER retained structure touched this window (pack pages,
  the Flow frontier — see §1.3), this history is **one atomic `Vec::try_reserve_exact` of up to 32 768
  bytes**, not a paged/incremental allocation. A grant smaller than `exact` makes **zero** progress and
  returns `Ok(default)` — not an error, not a fault, just silent no-op — every single call.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:8198` —
  `pub const ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES: usize = 4_096;` — the codebase-wide per-tick byte
  budget convention (matches `RETAINED_PACK_PAGE_BYTES = 4_096` in
  `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1088`, so every OTHER retained pack structure this
  window sized its page to fit this budget in one call — only the inflate history didn't).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs:17-19` (new
  file this window) —
  ```rust
  impl WindowConfigPackLoadGrant {
      pub const fn one_page() -> Self {
          Self { maximum_items: 1, maximum_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }
      }
  }
  ```
  and `reserve_next` (same file, ~line 724-761) — for the `Replay` phase, when the active cursor is a
  `RetainedPackSegmentCursor` (compressed segment), it does:
  ```rust
  if let Some(requested) = segment.next_allocation_bytes() {
      if requested > maximum_bytes || requested > remaining { return Ok(false); }
      return segment.reserve_allocation(maximum_bytes.min(remaining))…
  }
  ```
  — correctly declines to over-request, but `maximum_bytes` here is **always** `one_page()`'s fixed
  4 096, so `requested` (up to 32 768 for any compressed segment) is **always** `> maximum_bytes`, and
  this returns `Ok(false)` — "no progress this call" — forever.
- The caller that fixes the grant for the whole retained load: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs:581` —
  `let grant = WindowConfigPackLoadGrant::one_page();` inside `pub async fn load(&mut self, pack:
  WindowConfigPack)`, then this SAME fixed `grant` drives up to `1_048_576` iterations of
  `advance_retained_load` (line ~586). It is never escalated across iterations. If the loaded config's
  underlying pack segment is DEFLATE-compressed and its decompressed size is ≥ 4 097 bytes (i.e.
  `target` computed as `min(raw_len/limit, 32768)` exceeds one page), `advance_retained_load` cannot
  progress in any of the 1,048,576 iterations, `load.terminal_is_empty()` stays false, the code falls
  into the cancel-and-close retry loop (also 1,048,576 iterations, same fixed grant) at line 604.

**Why this reads as "guest never returns from setContributions" and matches the native panic:** the
contributions-gated re-arm design (`📓️contributions-push-starvation-2026-09-12.md` §2.3) has
`setContributions` invalidate the retained session and re-arm one `FlowEvalTick` per attached preview
window. If that tick's settle/commit path touches a window-config retained load whose backing pack
happens to be compressed (persisted flow-window or preview-window config state), the load call above
spins doing no visible work — in the browser this reads as "hung after setContributions" (the
`command ingress lane seq=8` log line is the LAST thing printed because the next settle never comes);
natively, `settle_registered_typed_operation`'s outer 30 s wall-clock deadline
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6705-6716`) eventually fires with
`"registered fixture typed operation did not retire within 30 seconds"` — the exact panic text
observed.

**What is NOT yet confirmed (do this first in the Opus lane):**
1. Whether the specific window-config pack read by `drain_flow_eval_ticks`'s `FlowEvalTick`/render
   path is actually written with `codec: CodecId(1)` (deflate) rather than identity — compression is
   caller-selected in `encode_segment` (`🎒️pack/📐️format/🦀️.rs:226-234`, `flags: 1 |
   (codec.0 << 1)` only `if compressed`), not automatic/size-gated, so this needs a direct check of the
   window-config pack-write call site (`encode_document_pack_bytes`,
   `🏪️store/🦀️.rs:10152`, and whatever sets `codec` above it).
2. Whether `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features
   component-app-assembly --lib -- component::unit_tests::host_pushed_contribution_pages… --nocapture`
   re-run with `eprintln!` added at `reserve_next`'s `Ok(false)` branches (or reading
   `LAST_PENDING_TYPED_MASK`/`LAST_MAINTENANCE_STAGE`,
   `🔌️plugin/🦀️.rs:26296-27389`, already wired into the panic message format string at
   `🦀️.rs:6712-6714`) shows the mask/stage pinned on a window-config retained-load step for the whole
   30 s — that is the smoking gun. (Neither `native-served-chain.txt` nor `s4-native-served-chain.txt`
   in `🗑️generated/` currently show `[DEBUG] maintenance stage=…` lines — that eprintln,
   `🔌️plugin/🦀️.rs:32319`, sits in a different call path than the loop `settle_registered_typed_operation`
   drives, so it never fired in the traces already captured; a fresh run needs its own targeted trace.)
3. A minimal, cheap confirmation that needs no rebuild of the whole tree: a unit test in
   `🧰️framework/🔨️modules/🗜️deflate/🧪️tests/🔬️unit/🦀️.rs` constructing `Inflater::try_new_retained(40_000,
   1_000_000)` and calling `reserve_retained_history(4_096)` in a loop — it should assert
   `progressed=false` forever, proving the one-shot-grant behavior in isolation without touching the
   plugin/window-config machinery at all.

### 1.2 SECOND — the whole `RetainedPackSourceCursor`/`RetainedPackSegmentCursor` admission contract changed to require reservation-before-admission

`🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`:
- `RetainedPackSourceCursor::try_new` gained a third parameter (`maximum_allocation_bytes`,
  line ~83) and `preflight_page` now hard-fails with `"retained-pack.page-allocation-required"`
  (line ~187-189) unless `has_reserved_page()` is true — i.e. a caller MUST poll
  `next_allocation_bytes()`/`reserve_page()` before every `admit`, not just size-check.
- `RetainedPackSegmentCursor::try_new` gained a `maximum_inflater_allocation_bytes` parameter
  (line ~331) and `preflight()` now also gates on `inflater.can_admit()` only while
  `RetainedPackSegmentPhase::Payload` (line ~358-363, narrowed from the old unconditional check).

Grepping the whole tree for non-test callers of `RetainedPackSourceCursor::try_new` /
`RetainedPackSegmentCursor::try_new` turns up exactly one: the new
`🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`, and it DOES drive `has_reserved_page()` /
`reserve_page()` / `next_allocation_bytes()` correctly for the SOURCE cursor (lines ~726-736) — so this
half of the contract looks honored. The risk is narrower than it first appears (contained to the one
new caller), but it is the SAME file that has the §1.1 bug, so the whole retained window-config load
path is new, large (1,536-line new file), and this audit did not exhaustively re-derive every phase
transition — worth a second pair of eyes on `advance_decode`/`advance_pack_retirement`/`advance_history`
(lines 1082, 1116, 1147) specifically for the same "grant narrower than the phase's minimum unit"
shape as §1.1, not just the segment/inflater case already found.

### 1.3 THIRD — same failure CLASS, already fixed once this window, evidence it is a live risk pattern

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌳️typed/📃️document/🦀️.rs:24-30` (diff vs
`8add1df147`) — `UiDocumentArena::retire_exact` was raising `grant = maximum_bytes.max(slot.nodes
.entries.allocated_bytes())` specifically because `PagedList::release_empty_page` frees a page WHOLE or
not at all, and one `UiNodeRecord` (6,416 bytes) versus the reconciler's 4,096-byte
`SURFACE_COMPONENT_COPY_WORK_BYTES` grant froze retirement solid — measured 67,667 consecutive
`progressed:false` steps on one surface (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B52, cited in the
docstring). A `UI_DOCUMENT_RETIREMENT_STALL_LIMIT: u16 = 4_096` (line 72) was added as a backstop that
turns an unresolvable stall into a hard error after 4,096 no-progress steps, rather than spinning
forever.

This is a **different file, already patched**, but it is the identical "page/window must be released or
allocated in one piece, and the caller's fixed grant is narrower than that piece" shape as §1.1 — strong
corroborating evidence that this exact class of bug is actively shipping in this rework, and that §1.1
has no analogous stall-limit backstop (`RetainedInflateHistory::reserve` never errors on a persistently
undersized grant — it just returns `default()` indefinitely; compare to `UiDocumentArena`'s explicit
`slot.stalled` counter). Recommend porting the same stall-counter-then-hard-error pattern into
`RetainedInflateHistory`/`WindowConfigPackLoad` regardless of the §1.1 root-cause fix, so a future
instance of this shape fails loudly within seconds instead of hanging.

`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` (695 lines changed,
`RetainedValueCursor`/`RetainedRecordBodyCursor`) was NOT fully re-derived line-by-line in this pass for
the same page-vs-grant shape — flagged for the Opus lane to check, since it sits directly on the
window-config `DslField::from_value` path (`📥️retained/🦀️.rs:359`) and is large enough that a similar
mismatch could hide in it undetected by this audit.

### 1.4 FOURTH — actor/shard heartbeat timeout, browser-only, does not explain the native repro

`🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts:109-118` adds a pass-through
`heartbeatTimeoutMs` option specifically because a `MainThreadShardWorker` blocks its own progress
ticker for the whole of a compute-bound guest turn, so the ordinary 5 s liveness ladder prices a long
real turn as death (docstring cites this exact ticket). The wgpu plugin-bridge call site
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:198`)
sets `heartbeatTimeoutMs: 180_000`. `🗑️generated/s4-boot-check-1/console.txt:4` shows a DIFFERENT pool
logging `heartbeatTimeoutMs=120000` — a second, uninvestigated call site. Neither value is infinite, so
this cannot by itself produce a true indefinite hang — at most it would explain a shard-lost/restart
storm at the 120s–180s mark, which is NOT what the browser log shows (log simply stops after
`setContributions` ingress with no later `[DEBUG] wgpu plugin-bridge: shard … lost` line, but the
capture may just have been cut before that fired). Ruled down in priority because it cannot explain the
native, non-worker, single-threaded test hang at all — that repro has no shard/heartbeat machinery in
play. Worth a quick check of which pool logs 120000 vs 180000 and whether the react (non-wgpu) target
generation3d dev serve uses the 120000 one, but this is secondary to §1.1.

### 1.5 RULED OUT — Flow retained artifacts (`🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`, 474 lines)

Read in full. `FlowOwner`'s new `push`/`reserve_allocation`/`next_allocation_bytes`
(lines ~154-219 of the diff) correctly uses the SAME incremental `PagedList`
(`frontier: ManuallyDrop<PagedList<…>>`) pattern as the pack-source pages — `reserve_capacity_one(target,
maximum_bytes)` genuinely allocates one page (bounded by the caller's grant) per call and accumulates
across many calls, unlike `RetainedInflateHistory`'s one-shot design. This file is NOT the odd one out;
its rewrite is internally consistent with the rest of the "budgeted incremental allocation" convention
introduced this window. Not a candidate.

### 1.6 RULED OUT — the `set-contributions` command handler itself

`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs`
— file mtime 2026-09-12 02:57, BEFORE the regression window (`8add1df147`@21:17 same day is already
after this mtime); `git diff 8add1df147..HEAD --name-status` shows no entry for this path at all. The
command's own decode/dispatch logic is unchanged across the regression window; whatever changed is
downstream of it, consistent with §0's finding that the native test's install phase itself succeeds.

## 2. Suggested confirmation order for the Opus lane

1. Cheapest, fastest: the isolated `deflate` unit test in §1.1 point 3 (`reserve_retained_history(4_096)`
   looping forever) — proves or disproves the core allocator mismatch in under a minute, no plugin/window
   machinery needed.
2. Grep the window-config pack WRITE path (`encode_document_pack_bytes` and callers,
   `🏪️store/🦀️.rs:10152` upward) for where `codec`/`CodecId` gets chosen, to confirm whether any
   window config actually gets written compressed (§1.1 point 1) — if none ever do, §1.1 cannot be the
   live cause today and the search moves to §1.2/§1.3's pack/value.rs.
3. Re-run the failing native test with `RUST_LOG`/extra `eprintln!` at
   `🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`'s `reserve_next` `Ok(false)` branches (§1.1) to catch
   it live, since the two existing traces in `🗑️generated/fix-forward-contributions/` predate that
   instrumentation and don't show it.
4. If §1.1 is cleared, re-derive `🎒️pack/🌱️value/🦀️.rs`'s `RetainedRecordBodyCursor` page-vs-grant sizing
   line-by-line (§1.3) the way this audit did for the deflate history.

## 3. Scope note

`git diff 8add1df147..HEAD --stat` touches ~1,690 files repo-wide; only files on/adjacent to the
`setContributions` → `FlowEvalTick` → window-config-commit path were read in full (pack/format,
deflate, pack/value diff headers, flow retained, UI document retirement, plugin.rs `settle_registered_
typed_operation`/`maintenance_step`, the new window-config retained loader, shard-runtime). Areas named
in the brief but NOT independently re-derived beyond what's cited above: the flow-extension catalogue/
registry proper (only touched via the `🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` file read in §1.5),
`🖱️ui/🧬️contract/♻️retirement/🩹️patch/🦀️.rs` (diff pulled but not read — grep-listed as
touched, same directory as §1.3's file), and the full `🎒️pack/🌱️value/🦀️.rs` 695-line diff (headers
scanned for the reserve/grant vocabulary, not read hunk-by-hunk — flagged as open in §1.3).
