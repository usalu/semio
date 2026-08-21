# R15 — Applying the De-Async Codemod to the Two Deferred Big Crates

Work packet R15 of Phase 1.5 (De-Async Repair Sweep), Interactive Job Runtime Refactor. Completes
the application R14 deliberately deferred for `semio-framework-os-infinite` and
`semio-s-plugin-stdio`, once the concurrent churn on their shared `🎲️board`/`🌍️world` files had
settled.

## Pre-flight

- `bun 🔧️r13-deasync-codemod.ts selftest` — all 14 pre-existing self-tests passed before any
  change was made.
- `bun ./📜️script.ts verify dependencies` → 238, clean, re-checked repeatedly throughout.
- Corruption-signature sweep (`\.await\.await`, `(Some|None|Ok|Err)\([^()]*\)\.await`), repo-wide
  excluding `compose/`/`target/`: 0/0 at every checkpoint.

## `semio-framework-os-infinite`

Baseline reconfirmed with the real compiler, matching the packet brief exactly:

| Target | Errors |
| --- | ---: |
| `cargo check -p semio-framework-os-infinite --lib` | 820 |
| `cargo check -p semio-framework-os-infinite --all-targets` (test) | 1103 |

Dry-run (`--dry-run --verbose`) reproduced R14's saved taxonomy byte-for-byte: 1871 total
diagnostics (workspace-feature-unification count, not the per-crate authoritative number above),
1022 async-class, **zero automatable edits planned**, 1022 residue, category counts identical to
`📝️r14-dryrun-osinfinite.txt`. Two of the target files (`🌍️world/component.rs`, `🖼️canvas/
component.rs`, `🎲️board/…/dag/component.rs`, root `component.rs`) showed unstaged modifications in
`git status` from a session prior to this one; hashed immediately before and again after the
~14s dry-run and the file content was byte-identical both times — no active churn during this
packet's work on this crate.

Real (non-dry-run) run: **0 edits applied, 1022 residue, all logged with a specific reason**
(`--verbose` category breakdown: `no let-binding found` 512, `bare identifier, no def-use
attempted` 214, `pattern binding` 90, `function parameter` 80, `RHS is a method chain` 28,
`shadowed binding` 24, `crosses closure boundary` 12, `reassigned` 8, `not lexically inside async
context` 4, `use precedes binding` 4, `RHS is block/conditional` 2, `Future-expr span starts at
reserved word` 6, `post-resolution guard` 31, `other/unclassified` 7 — sums to exactly 1022).
Post-run compiler re-check: **820 / 1103, unchanged** — no regression, because nothing was
touched. Corruption sweep: 0/0.

**Conclusion: this crate is already fully saturated by the span-local + def-use codemod.** Its
entire remaining residue is the class R14 already named as the top work-list item — Future-typed
expressions reachable only through reasoning the tool deliberately refuses to attempt (sibling
expressions in the same statement, multi-hop def-use, loop-carried bindings). Nothing here needs
"defer for quieter files" any more; it needs a strictly harder tool. Documented, not attempted —
per the packet's own stop condition ("residue that needs data-flow beyond a single function body →
leave it, document it").

## `semio-s-plugin-stdio`

Baseline reconfirmed:

| Target | Errors |
| --- | ---: |
| `cargo check -p semio-s-plugin-stdio --lib` | 4827 |
| `cargo check -p semio-s-plugin-stdio --all-targets` (test) | 9537 |

Exactly matches the packet brief and R14's wind-down number.

### A SIXTH and SEVENTH real bug, found and fixed before any real edit landed

The dry-run plan (109 edits: 100 `call-add-await`, 5 `def-remove-async`, 4 `defuse-call-add-await`)
was inspected BY HAND per the packet's mandate, not applied blindly. Two more real corruption
mechanisms were found — both pre-existing gaps in R13/R14's guard machinery, neither hit by the
sample sizes those packets tested at.

**Bug 6 — `findFutureExprSpan`'s trusted-fallback still misattributes when a diagnostic carries
only ONE span.** The "byte-adjacent sibling span" fix (R13, closing bug 2) only fires when rustc
actually emits a second, non-primary span abutting the primary one. Confirmed on real stdio
diagnostics that it often does not: 168 single-span `"T is not an iterator"`/`"no field X on
type impl Future"`-shaped diagnostics in the lib target alone (of which the primary span is a bare
identifier — a trailing dot-method/field name, e.g. `into_iter` in
`<SemioBrepMutation as Mutation<..>>::inverse(m, b).into_iter().map(..)`). With no sibling to
prefer, `findFutureExprSpan` fell back to that primary span, and `extractCallForward` then
(correctly, mechanically, but WRONGLY) parsed the trailing method's own `()` as "the call",
producing `X.into_iter().await` instead of `X.await.into_iter()` — the exact bug-2 SYMPTOM, via a
different, previously-unexercised mechanism (a genuinely single-span diagnostic, not a
mis-selected sibling). Root-caused, not guessed at: verified byte-for-byte against the raw
`cargo check --message-format=json` diagnostic (`byte_start=9127,byte_end=9136`, label exactly
`` `impl Future<Output = Vec<SemioBrepMutation>>` is not an iterator ``, single span, no sibling).

Fix: `findFutureExprSpan` now returns `{ span, trusted }` — `trusted: true` only for the two
label-matched branches and the abutting-sibling branch; `trusted: false` for the raw-primary
fallback. The no-suggestion-fallback caller refuses (new residue category, machine-parseable) when
`!trusted && extractCallForward succeeds && the span is immediately preceded by "."` — a dot
immediately before an untrusted span structurally proves it is a trailing method/field name, never
the receiver. A trusted span legitimately CAN be dot-preceded (`self.fetch_data()` where
`fetch_data` really is the callee) — the guard only fires on the untrusted case.
**Impact at scale on this crate: 73 of the original 100 planned `call-add-await` edits were this
exact shape and are now correctly refused** (100 → 27 after the fix; residue category `R15:
untrusted primary-fallback dot-method/field misattribution` = 187 across lib+test).

**Bug 7 — a second, independent manifestation of failure mode 5 ("misattributed to a fully
synchronous expression") that `isInsideAsyncContext` does not close.** That guard only asks "is
this position lexically inside an async fn/block?" — true whenever the ENCLOSING FUNCTION is
itself `async`, which is the overwhelming common case in this codebase (88% of `async fn`, per the
Phase 0 census). Found on real code: `semio_isobmff/mp4` diff test helper `build_moov`
(`async fn`) contains
`write_box(b"moov", &[build_mvhd(&snapshot.movie), traks, build_udta(&snapshot.movie)].concat())`
— an array literal mixing two un-awaited async calls with one ordinary, ALREADY-CORRECT local
(`traks: Vec<u8>`, built via `Vec::new()` + `.extend(..)`, nothing async about it at all). rustc's
element-type unification infers the array's type from the FIRST element (the un-awaited Future),
then reports the mismatch at the first element that does not match it — `traks`, an innocent
bystander — with the self-describing label `` "expected future, found `Vec<u8>`" `` (confirmed via
the diagnostic's own child note: `` expected opaque type `impl Future<Output = Vec<u8>>`, found
struct `Vec<u8>` ``). Before the fix, def-use would resolve `traks`'s own (entirely synchronous)
binding, fail to resolve `Vec::new`'s trailing segment `new` as an async callee (correctly, since
it isn't one), and default to inserting `.await` — producing `Vec::new().await`, which awaits a
non-Future and would not have compiled. Caught by hand-verifying the def-use edit at
`mp4/…/component.rs:41476` against the raw diagnostic JSON before applying anything for real.

Fix: any diagnostic whose (chosen) span carries a label matching
`` /^expected\s+(`?impl\s+)?[Ff]uture.*,\s*found\s+/i `` is refused outright, before def-use or
`extractCallForward` is attempted — that label PROVES the span is the CONCRETE ("found") side of
an element-unification mismatch, never the Future-producing expression; the real fix is on an
unidentified sibling earlier in the same array/tuple/call, which needs reasoning this span-local
tool does not attempt. New residue category `R15: span is the concrete side of an
element-type-unification mismatch`. Removed the `mdat_data_offset`-adjacent false match entirely
(2 of the original 4 `defuse-call-add-await` edits were this shape).

Both fixes are additive-only (new refusal branches; no existing accepted edit shape was narrowed),
type-checked clean (`bunx tsc --noEmit`, isolated flags, zero errors), and each has a dedicated,
from-scratch reproducing self-test built against the exact real-world shape:

- **selftest15**: single-span `"T is not an iterator"` diagnostic, dot-preceded untrusted primary
  fallback → `planEditsForDiagnostic` end-to-end asserted to produce zero edits and exactly one
  residue entry citing the new guard.
- **selftest16**: `"expected future, found <concrete type>"` array-literal-unification shape (the
  `traks`/`build_moov` reproduction) → same end-to-end assertion.

`bun 🔧️r13-deasync-codemod.ts selftest` → **16/16 pass**, including both new tests, run before any
real edit was applied to `semio-s-plugin-stdio`.

### Corrected dry-run, after both fixes

| | before fix | after fix |
| --- | ---: | ---: |
| edits planned (iteration 1) | 109 (100 call-add-await, 5 def-remove-async, 4 defuse-call-add-await) | 34 (27 call-add-await, 5 def-remove-async, 2 defuse-call-add-await) |
| residue | 6827 | 6902 |

The corrected 34-edit plan was hand-inspected site by site (every `def-remove-async` target
confirmed to be a real `async fn` declaration outside any trait-impl/test-harness/quote! guard;
every `call-add-await`/`defuse-call-add-await` insertion point confirmed, via a JS-UTF16-aware
byte-offset script cross-checked against the real `cargo check --message-format=json` diagnostic,
to land immediately after the actual Future-producing call — never mid-identifier, never on the
wrong side of a chain) before it was applied for real.

### Real application, run 1 (`r13-g1aiw3r2`)

Applied all 34 planned edits across 22 files. Corruption sweep: 0/0. `git diff` confirmed, via the
journal's own `runId` filter (not by eyeballing the diff), that exactly one edit landed in
`🧰️framework/…/🔌️plugin/🦀️component.rs` (`from_function_pointer` de-asynced) — six OTHER changes
visible in that file's diff (`.await` removed after `viewer_surface`/`editor_surface` calls) belong
to the concurrent peer session and were pre-existing, unstaged, uncommitted content already on
disk before this packet touched anything; left untouched, not attributed to this run.

Post-apply compiler re-check: `--lib` unchanged at 4827 (all 34 edits are in `--all-targets`-only
/ test-adjacent files); `--all-targets` (test) **rose 9537 → 9552 (+15)**. Diagnosed rather than
retried, per the packet's explicit instruction: reverted this run's 34 edits via
`revert --run=r13-g1aiw3r2` (byte-for-byte, confirmed by the journal), re-measured stdio with ONLY
the concurrent session's unrelated edits present — **9537, exactly baseline** — isolating the +15
to this run's own 34 edits and ruling out the concurrent session as the cause. A
before/after diagnostic diff (matched by file+message, byte offsets deliberately ignored since
`.await` insertions shift every later byte position in the same file) showed the +15 is not
corruption: it is the documented reachability-cascade pattern (fixing one un-awaited use of a
Future variable in a function frequently unmasks the compiler's SUBSEQUENT complaint about a
DIFFERENT, still-un-awaited use of the SAME variable later in the same function — e.g. `"no method
named contains found for … Future<Output=String>"` resolving into `"impl Future<Output=String>
doesn't implement Debug"` a few lines later) — net +46/-31 across 56 (file, message) buckets, every
one of them still squarely in the async-fn-without-`.await` bug class, none a parse error, none an
unrelated new bug class. Re-applied the same 34 edits (`r13-g1aiw3r2` → re-run as `r13-g1aiw3r2`
redo, identical 34 edits reproduced deterministically).

### Real application, run 2 (`r13-bpl56l2o`, continuing from run 1's state)

Confirms the cascade self-corrects on iteration, exactly as the codemod's iterate-to-fixpoint
design intends: measured from the post-run-1 state (total 13110, async-class 6932), planned 21
further edits, applied 20 (one deduped). A Bash-tool timeout killed the driving process mid-run —
verified via the journal (20 entries for this `runId`, all sharing one write timestamp, i.e. the
apply loop completed before the kill landed, consistent with the process being killed during the
NEXT iteration's read-only `cargo check`, not mid-write) and via the corruption sweep (0/0,
unchanged) and a full re-check for parse errors (`grep -iE "expected|unexpected"` against a fresh
`cargo check` — none, only pre-existing unrelated `unexpected cfg` warnings). Post-run compiler
re-check: `--lib` still 4827 (unchanged); `--all-targets` (test) **9541** — down from run 1's 9552
spike, converging back toward the 9537 baseline (+4 net), confirming the cascade is resolving
itself with more iterations rather than being a real regression.

### Real application, run 3 (`r13-o9bdzimk`, backgrounded to avoid the timeout that hit run 2)

Continued from run 2's state (total 13093, async-class 6915). Iteration 1: 4 edits (1
`def-remove-async` on `build_stco` in the mp4/isobmff io module — unrelated to what follows;
2 `defuse-call-add-await` on `mycielskian`/`apply_semio_mutation`). Iteration 2: 13091 (improved
from 13093), planned and applied 2 `call-remove-await` edits — rustc's own machine-applicable
suggestion, removing the SAME two `.await`s iteration 1 had just added. Iteration 3 re-measured at
13091 (unchanged from iteration 2's result) and the **monotonic guard correctly tripped**
(`13091 >= previous 13091` — a tie, not a strict improvement) and auto-reverted iteration 2,
restoring the two `.await`s iteration 1 had inserted, then stopped.

**Diagnosed, not retried — and it uncovered an EIGHTH real bug**, the mirror image of the sixth.
Root-caused against the pre-edit diagnostic JSON: `mycielskian` is confirmed (`grep`) to be a
plain, never-`async fn` returning `Storage<Normal, D>` directly — genuinely synchronous, already
correct. The diagnostic def-use actually responded to was
`` `impl Future<Output = impl Iterator<Item = u64>>` is not an iterator `` on the FULL expression
`myc.nodes()` inside `for n in myc.nodes() { .. }` — `.nodes()` (not `myc`) is the real,
un-awaited async call. The def-use fallback's `identMatch` regex captures only the LEADING
identifier of a Future-expr span; when that span is actually `RECEIVER.method(..)` in full (a
method-chain, exactly Bug 6's shape but on the RECEIVER side instead of the trailing-method side),
treating the leading identifier as a bare variable and tracing it to ITS OWN (already-correct)
binding is simply wrong — the fix inserted `.await` onto `mycielskian(&g)`, and only a LATER
compiler run (once that bad edit was already on disk) proved it wrong: `` `graph_core::Storage<
Normal, Undirected>` is not a future ``. Iteration 2's `call-remove-await` (from rustc's own
suggestion) had already found and was correctly undoing this exact mistake — the guard's
conservative "any non-strict-improvement reverts" tripped on the tied count and undid the
CORRECT fix along with whatever caused the tie, restoring the ORIGINAL wrong edits. Manually
reverted those two `.await` insertions by hand (`mycielskian(&g).await;` → `mycielskian(&g);` in
`graph/operators-internals/component.rs`; `&wrapped).await;` → `&wrapped);` in
`any/mutations/component.rs`), confirmed against the journal's own recorded `before`/`after` text.

Fix: in the def-use fallback, before `resolveDefUse` is even called, refuse when the character
immediately after the matched leading identifier is `.` — that structurally proves the Future-expr
span covers a `receiver.method(..)` chain, not a bare variable use, so tracing the leading
identifier to its own binding cannot be trusted. New residue category `R15: def-use identifier is
a method-chain receiver, not a bare variable`. **selftest17** reproduces the exact `mycielskian`/
`myc.nodes()` shape end to end (zero edits, one residue entry citing the new guard).

`bun 🔧️r13-deasync-codemod.ts selftest` → **17/17 pass** (all three R15 guards — 15, 16, 17 —
each with its own from-scratch reproducing test against the real-world shape that found it).
`bunx tsc --noEmit` clean throughout.

### Halted: a large concurrent session began rewriting nearly the entire crate

While finishing verification, `git status` on `✏️s/🔌️plugins/🗄️stdio` jumped from the ~15 files
this packet had touched to **747 modified files** within about a minute, confirmed via `find
-newer` (740 of them newer than this write-up's own first draft) — an unmistakable, large-scale,
in-progress rewrite by the peer session (plausibly the `COMPOSE-TO-PUZZLE5D-MIGRATION` ticket
visible in the same day's ticket tree), not this packet's doing (cross-checked against every
`runId` this session created — none touch files outside the ~20 this packet's own edits and
reverts named). Per the explicit mandate ("if a file is being actively churned, skip it and say so
rather than racing"), further real application to `semio-s-plugin-stdio` was **halted here**,
mid-crate, rather than raced. Corruption sweep and dependency ratchet re-verified clean
immediately before stopping (0/0 repo-wide; 238 dependencies, unchanged).

**Last stable, attributable measurement** (taken before the concurrent burst, after manually
restoring the two Bug-8 edits): `--lib` 4827 (unchanged from baseline — every edit landed in
`--all-targets`-only code); `--all-targets` (test) 9541 (net +4 over the 9537 baseline, the tail
of the same benign reachability-cascade pattern documented under run 1, actively still resolving
itself when the concurrent session's rewrite began). This number should not be read as this
packet's final word on the crate — it is a snapshot mid-iteration, honestly reported as such.

## Refusal taxonomy — full category list surfaced across both crates this session

Every category below is machine-parseable (`residueCategory()`); the three marked **R15 (new)**
did not exist before this packet and were exercised on real code, not merely unit-tested:

| Category | `os-infinite` | `stdio` (pre-fix dry-run) |
| --- | ---: | ---: |
| `def-use: no let-binding found` | 512 | 1906 |
| `R13: bare identifier, no def-use attempted` | 214 | 2256 |
| `def-use: function parameter` | 80 | 584 |
| `def-use: pattern binding (match/if-let/for/closure-arg)` | 90 | 144 |
| `R13: Future-expr span starts at reserved word` | 6 | 296 |
| `R13/R14: not lexically inside async context` | 4 | 114 |
| `def-use: post-resolution guard` | 31 | 43 |
| `def-use: RHS not a resolvable call (other)` | — | 104 |
| `def-use: RHS is a method chain` | 28 | 57 |
| `def-use: mutably borrowed` | 8 | 24 |
| `def-use: crosses closure boundary` | 12 | 95 |
| `other/unclassified` | 7 | 1188 |
| `R13: guard refusal (denylist/risky-insertion/stacking)` | — | 12 |
| `def-use: use precedes binding` | 4 | 1 |
| `def-use: reassigned` | 8 | 1 |
| `def-use: shadowed binding` | 24 | 2 (post-fix) |
| **`R15: untrusted primary-fallback dot-method/field misattribution`** | 0 | **187** |
| **`R15: concrete side of element-type-unification mismatch`** | 0 | *(subset of the 187+other buckets; not separately re-run post-fix — see note)* |
| **`R15: def-use identifier is a method-chain receiver`** | 0 | *(found live during real application, not yet re-measured via a fresh dry-run before the concurrent halt)* |

Note: the corrected 34-edit dry-run (after guards 15/16) was captured and hand-verified in full;
the halt arrived during real, iterative application (guard 17's incident), before a fresh
`--dry-run --verbose` could be re-run to get guard 17's exact residue count on this crate. That is
the natural first step of any follow-up session.

## Final workspace gate

`cargo check --workspace --all-targets --keep-going 2>&1 | grep "could not compile"`, run
immediately after the concurrent-churn halt (literal output, not curated):

```
error: could not compile `semio-framework-mesh-engine` (lib test) due to 4 previous errors
error: could not compile `semio-framework-math` (lib test) due to 6 previous errors
error: could not compile `semio-framework-os-kernel` (lib test) due to 9 previous errors; 20 warnings emitted
error: could not compile `semio-framework-os-kernel-db` (lib test) due to 10 previous errors; 62 warnings emitted
error: could not compile `semio-framework-graph` (lib test) due to 6 previous errors
error: could not compile `semio-s-imperative` (lib) due to 2 previous errors
error: could not compile `semio-s-imperative` (lib test) due to 2 previous errors; 1 warning emitted
error: could not compile `semio-framework-os-mcp` (lib) due to 22 previous errors; 4 warnings emitted
error: could not compile `semio-framework-os-mcp` (lib test) due to 22 previous errors; 8 warnings emitted
error: could not compile `semio-framework-plugin` (lib) due to 5 previous errors; 29 warnings emitted
error: could not compile `semio-framework-os-infinite` (lib) due to 820 previous errors
error: could not compile `semio-framework-plugin` (lib test) due to 143 previous errors; 42 warnings emitted
error: could not compile `semio-framework-os-infinite` (lib test) due to 1103 previous errors; 19 warnings emitted
error: could not compile `semio-compose-rs` (lib) due to 17 previous errors; 89 warnings emitted
error: could not compile `semio-compose-rs` (lib test) due to 34 previous errors; 160 warnings emitted
```

Two things worth flagging about this specific snapshot, both **attribution-checked against this
session's own `runId`s in the journal, not assumed**:

1. **`semio-s-plugin-stdio` does not appear at all** — the exact "workspace-scan reachability
   artifact" R13's own methodology warns about (a `--keep-going` scan can still fail to reach a
   crate if something upstream of it aborts first); it is mid-rewrite by the concurrent session
   right now regardless, so no number for it would be meaningful this instant.
2. **`semio-framework-mesh-engine`, `semio-framework-math`, `semio-framework-graph`,
   `semio-framework-os-kernel`, `semio-framework-os-kernel-db` now show test-target errors**,
   contradicting R14's "cleared and holding" status for all five. None of this session's `runId`s
   touch any of these five crates (verified by grepping the journal's `crate` field per `runId`
   this session created) — not this packet's regression. Given the same-minute 747-file burst
   observed in `stdio`, the far more likely explanation is the concurrent session's in-progress,
   repo-wide edit passing through a transiently-broken intermediate state (consistent with prior
   observed behavior of large concurrent refactors in this repo). Left for the next measurement
   once that session's work settles, per the packet's own "poll rather than chase" precedent.

`bun ./📜️script.ts verify dependencies` → 238, clean, re-checked at every checkpoint through the
halt. Corruption-signature sweep (`\.await\.await`, `(Some|None|Ok|Err)\([^()]*\)\.await`),
repo-wide excluding `compose/`/`target/`: **0/0**, re-checked at every checkpoint including
immediately before the halt.

## What remains for humans / future packets

1. **`semio-s-plugin-stdio`**: real application interrupted mid-crate by a large concurrent
   rewrite (747 files). The tool is left in a materially safer state than it started this session
   (17/17 self-tests including three new, real-world-shaped regression tests; three new,
   independently-verified corruption mechanisms closed), but the crate itself needs a fresh
   dry-run and a full iterate-to-fixpoint pass once the concurrent session's work has landed and
   settled — do not resume against files still visibly in flux.
2. **`semio-framework-os-infinite`**: confirmed fully saturated by this tool (0 edits, 1022
   residue, all logged). Its residue needs a genuinely harder tool — multi-hop/sibling-expression
   reasoning beyond a single `let`-binding's own RHS — not another run of this one.
3. The guard-17 incident is a reminder that `identMatch`'s "leading identifier of the Future-expr
   span" heuristic has (at least) two structurally distinct failure shapes depending on which side
   of a `.` the span sits — guard 15 (span IS the trailing method) and guard 17 (span IS the
   receiver, method comes after) are siblings, not duplicates; a future packet tightening
   `ASYNC_SIGNATURE_PATTERNS`/`findFutureExprSpan` further should keep both in mind rather than
   assuming one implies the other.
4. Five previously-clean crates (`mesh-engine`, `math`, `graph`, `os-kernel`, `os-kernel-db`) show
   fresh test-target errors in the final workspace gate above, not attributable to this packet —
   needs an independent re-check once the concurrent session's work settles, before assuming
   regression.

## Files touched

- `🔧️r13-deasync-codemod.ts` — extended in place: `findFutureExprSpan` now returns
  `{ span, trusted }`; three new refusal guards (untrusted dot-method-as-callee, "expected future
  found concrete" element-unification, def-use method-chain-receiver); `residueCategory()` gained
  three new buckets; self-tests 15–17 added (17/17 total, up from R14's 14/14); selftest7 updated
  for the new `findFutureExprSpan` return shape.
- `📝️r13-journal.jsonl` — appended: dry-run/real runs against `semio-framework-os-infinite`
  (`r13-w76rvs0i`, `r13-laj8idqn`, `r13-kd2k3mhh` — all no-op, 0 edits) and `semio-s-plugin-stdio`
  (multiple dry-runs; real runs `r13-37nq6yr3`→reverted, `r13-g1aiw3r2`→applied 34→reverted→
  re-applied 34, `r13-bpl56l2o`→applied 20 (partial, safe), `r13-o9bdzimk`→applied 4, reverted 2,
  2 manually corrected).
- Real edits currently on disk from this packet, in `semio-s-plugin-stdio` and one file in
  `semio-framework-plugin` — see the journal for the exact byte-span list; `git diff --stat`
  against the relevant paths is the fastest way to enumerate them.
- This write-up: `📓️r15-bigcrate-application.md`.

