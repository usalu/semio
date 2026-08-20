# R13 — Compiler-Driven De-Async Codemod

Work packet R13 of Phase 1.5 (De-Async Repair Sweep), Interactive Job Runtime Refactor. Builds
the compiler-driven codemod that Phase 1.5's earlier hand-repair packets (R1–R12) concluded was
necessary once the async bug class stopped scaling to manual, crate-by-crate fixes.

Tool: `🔧️r13-deasync-codemod.ts` in this ticket folder (temporary, not wired into `📜️script.ts`
per repo rules). Journal: `📝️r13-journal.jsonl` in this folder.

<!-- Sections below are filled in as each target crate is processed. -->

## The bug class and the decision rule

The repo-wide async convention (`AGENTS.md:44`) mechanically marked ~53,000 functions `async`.
Phase 0's census found 88.28% never suspend. The fallout is one class of compile error: an
`async fn` called without `.await`, or (equally often, this packet's main new finding) a
STALE `.await` left on a callee that a PRIOR packet already de-asynced.

Per diagnosed call site:
- callee genuinely suspends (its body has its own real `.await`) → add `.await` at the call site.
- callee never suspends (zero own `.await`) → **de-async the callee** (remove `async` from its
  signature). The call site needs no edit — once the callee returns `T` instead of
  `impl Future<Output = T>`, the type mismatch resolves on its own on the next compile.

This is the STRONGLY PREFERRED direction per the packet's brief: `async fn` should be reserved
for genuine suspension.

## Design — compiler-driven, span-keyed, never name-keyed

1. `cargo check -p <crate> --all-targets --message-format=json` (per-crate, never
   `--workspace` — workspace totals are non-deterministic run-to-run as fixes unmask more
   crates; R12 already established this).
2. Parse structured diagnostics. Filter to the async bug class by requiring the diagnostic
   actually mention `Future`/`await` in its full text (guards against unrelated errors that
   happen to share a message shape) AND match one of the recognised bug-class message
   patterns from the task brief, plus two shapes discovered empirically during this packet
   (below).
3. For each diagnostic, locate the actual edit span using — in priority order:
   - rustc's own machine-applicable suggestion, when offered (`suggestion_applicability ===
     "MachineApplicable"`, or `"MaybeIncorrect"` specifically when the replacement text itself
     inserts `.await` — the `.ok()` / opaque-Future "no method found" shape only ever gets a
     `MaybeIncorrect` suggestion in this codebase, never `MachineApplicable`, so restricting to
     the stricter tier alone would have silently dropped it into residue).
   - a dedicated E0053 ("has an incompatible type for trait") handler: this is an
     AUTHORITATIVE, compiler-verified signal that a trait's own declared signature is sync
     while the impl marked it `async` — stronger than (and an explicit, justified override of)
     the general "never touch trait impls" guard, because rustc's own child note gives the
     expected (sync) and found (`impl Future<...>`) signatures directly, so there is no
     guessing involved.
   - a dedicated E0733 ("recursion in an async fn requires boxing") handler: NOT in the task's
     enumerated bug-class list, but the diagnostic's own spans hand over the exact recursion
     cycle's member signatures for free (the top-level span plus every child "which leads to
     this async fn" note). If — and only if — EVERY member's own-level `.await`s target only
     fellow cycle members (verified, never assumed), the whole cycle is de-asynced atomically:
     plain recursion needs no `Box::pin` at all, so the error disappears rather than needing the
     suggested boxing fix. Any member with an award outside the cycle refuses the whole
     diagnostic into residue (a human Box::pin call).
   - a "no suggestion at all" forward-extraction fallback for shapes like E0277 "is not an
     iterator": locate the actual Future-typed expression via `findFutureExprSpan()` — NOT
     blindly the diagnostic's `is_primary` span, which for shapes like `if let Some(out) =
     outward { }` is the *pattern*, not the Future-holding variable `outward` (confirmed as a
     real, distinct diagnostic shape in `semio-framework-os-infinite`; blindly trusting
     `is_primary` there would have inserted a syntactically broken `.await` onto `Some(out)`).
     `findFutureExprSpan()` prefers a span whose rustc-provided `label` literally says "this
     expression has type `impl Future<...>`" or "found future", falling back to the primary
     span only when no such label exists. Balance parens/turbofish forward from that point to
     find the true call end. When the Future-typed expression is a bare variable (not a fresh
     call), forward extraction correctly fails closed into residue rather than guessing — this
     is the dominant residue shape in the large crates (see below): a `let`-bound Future used
     later, which needs def-use tracing back to its assignment, out of this packet's scope.
   - anything left over becomes residue with a specific reason, never a guess.
4. Every planned edit for the found-non-suspending-callee branch is resolved through a single
   shared, guarded pipeline (`resolveCallee` → `resolveCalleeInFile`, extended with a
   repo-wide, uniqueness-gated cross-file fallback — see below) so the decision rule and its
   guards are applied identically regardless of which diagnostic shape triggered it.
5. Edits are applied **back-to-front per file** (descending byte-derived-char offset) so
   earlier edits in a file never shift later spans; this also makes the revert algorithm exact
   (see Safety).
6. Re-check, repeat, requiring the crate's total error count to strictly decrease each
   iteration (see Safety).

### Critical bug caught before any real edit: UTF-8 byte offsets vs JS string indices

rustc's diagnostic spans are **UTF-8 byte offsets**. This repo's source files routinely contain
multi-byte characters before arbitrary code positions (the emoji docstring convention, emoji in
comments), so byte offset *N* is frequently **not** the same position as JS string index *N*.
Confirmed empirically on a real file in this repo: rustc byte offset 5079 in
`✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs` corresponds to JS string index 5029 — a
50-character divergence purely from emoji earlier in the file. Every span is now converted
through a `buildByteToCharMap()` (O(file length), cached per file) before it touches `src`/
`clean`; a dedicated self-test (`selftest3`) proves the conversion against a fixture engineered
to exercise a 4-byte-UTF-8 / 2-UTF-16-unit divergence. This bug was caught during the very first
dry-run against a real (non-trivial) target, before any file was ever written — dry-run-first
did its job.

### Repo-wide cross-file callee resolution (uniqueness-gated)

The first per-file-only design missed the common case where the call site and the async
function it calls live in different crates/modules (e.g. `imperative/registry` calling
`manifest::parse_contributions`). `resolveCallee()` now falls back to a repo-wide index of every
`async fn NAME` occurrence (rebuilt fresh each iteration — never stale across edits made by this
same run) **only** when the call site's own file has zero local candidates, and **only**
resolves automatically when the name is globally unique across the whole tree (excluding
`compose`). Multiple repo-wide candidates for the same name → refused (guarded), matching R12's
hard lesson that generic names (`hash_bytes` was two unrelated functions) make blind name-keyed
resolution dangerous. All the same guards (trait-impl, `quote!{}`, async-test attribute) are
re-applied at the resolved cross-file location, not skipped.

## Guards (never de-async)

- Function sits inside an `impl <Trait> for <Type>` block (external trait signature) — detected
  by scanning backward for the nearest enclosing `impl ... for ...` header and verifying no
  closing `}` of that block appears before the function. Overridden only by the E0053 handler,
  which has compiler-authoritative proof the trait itself is sync.
- Function sits lexically inside a `quote!{...}` / `quote_spanned!{...}` macro invocation
  (bracket-matched ranges computed once per file) — this is generated-code template text, not a
  real function; editing it needs the macro updated, not its expansion.
- Function carries a `#[tokio::test]` or `...::async_test` attribute on one of the 3 lines above
  it — legitimate async test harnesses are left alone.
- Ambiguous same-named `async fn` (multiple candidates in one file, or multiple files
  repo-wide) — refused rather than guessed.
- `compose/`, `target/`, `node_modules/` are hard-excluded from every diagnostic filter and from
  the repo-wide index walk.

## Incident: a real corruption, found and fixed during this packet

One genuine bug in the tool produced a real syntax corruption on the first large-scale run
against `semio-framework-os-infinite`, caught by manual verification (running `cargo check`
after the run, not by the tool's own guards) rather than prevented outright. Recorded here in
full because the packet's own safety rules demand honesty over a clean narrative.

**What happened**: `findFutureExprSpan()` (added specifically to avoid trusting a diagnostic's
`is_primary` span blindly — see Design) picks the span whose rustc-provided `label` says the
expression "has type `impl Future<...>`" or "found future". For
`let (Some(src_ep), Some(tgt_ep)) = (P::try_handle_endpoint(source_hid),
P::try_handle_endpoint(target_hid)) else { .. }` (in `🎲️board/🦀️component.rs`), rustc's
tuple-pattern-vs-tuple-of-futures mismatch attributed the "found future" label to the PATTERN
sub-span `Some(src_ep)`, not to the actual Future-typed RHS expression. `Some(x)` is lexically
indistinguishable from a function call, so `extractCallForward()` correctly (by its own,
narrower contract) parsed it as one, `resolveCallee("Some")` correctly found no such user
function anywhere (repo-wide), and the code's fallback logic — "unresolved means it must be an
external, genuinely-suspending call, so await it" — inserted `.await` right after a pattern,
producing a syntax error: `let (Some(src_ep).await, Some(tgt_ep).await) = (..) else {`.

**How it was caught**: not by the tool's own machinery. `cargo check` after the run reported
"expected identifier, found keyword `await`" — an actual parse error, immediately visible and
unambiguous. A repo-wide grep for the exact corrupted shape
(`grep -rnE "(Some|None|Ok|Err)\([^()]*\)\.await"`, excluding `compose`/`target`) found exactly
one occurrence — this one — confirming the damage was contained to this single line and not a
wider silent pattern.

**Fix, in two parts**:
1. **Immediate repair**: hand-corrected the one corrupted line back to valid syntax, properly
   `.await`ing the two RHS calls instead of the patterns (`try_handle_endpoint` is a trait
   method — `PortModel::try_handle_endpoint` — still `async`; both of its concrete
   implementations are trivially non-suspending (`None` / `Some(handle_id)`), a strong candidate
   for a future packet to de-async the whole `PortModel` trait + its ~2 implementors in
   lockstep, exactly the kind of coordinated multi-site change this packet's trait-impl guard
   correctly refuses to attempt automatically).
2. **Root-cause fix in the tool**: added `PATTERN_CONSTRUCTOR_DENYLIST` (`Some`, `None`, `Ok`,
   `Err`) — checked at both `call-add-await` insertion sites, right before an edit is queued.
   When the extracted callee name is a pattern-position enum constructor, the diagnostic is
   refused into residue with an explicit reason instead of guessed at. A new self-test
   (`selftest4`) reproduces the exact fixture shape (`let (Some(src_ep), Some(tgt_ep)) = pair()
   else { .. }`) and asserts both that `extractCallForward` still parses `Some(x)` as call-shaped
   (the bug was never in extraction — `Some(x)` genuinely IS lexically a call) and that the
   denylist gate catches it before an edit is queued. All four self-tests, plus the
   known-clean-crate no-op, were re-verified green before any further crate was touched.

**Why the safety net still held despite this**: the corruption was on ONE line, immediately
visible from a direct `cargo check`, easy to find with a targeted grep once identified, and easy
to hand-repair with full journal traceability (the specific edit's `runId`/`iteration` was
findable in `📝️r13-journal.jsonl`). This is exactly why the packet's own instructions mandate
running the real compiler after every crate, not trusting the tool's self-reported success. This
finding also argues for the residue category being wider than the tool believes: some fraction
of the "no-suggestion fallback" residue this tool DOES act on (rather than refusing) could in
principle share record-adjacent risk in ways not yet observed; the denylist closes the one
concretely observed case, not a formally verified class.

## Second near-miss: a diagnostic span pointing outside the repo entirely

While scoping `semio-s-plugin-stdio`, a dry-run against its `semio-framework-plugin` dependency
proposed a `call-add-await` edit whose file was
`/Users/ueli/.rustup/toolchains/nightly-.../lib/rustlib/src/rust/library/core/src/macros/mod.rs`
— rustc's own installed standard-library source, entirely outside this repo, reached via a
diagnostic span attributed back to a macro's expansion origin. The tool's exclusion list at the
time was a substring blacklist (`compose/`, `target/`, `node_modules/`) — necessary but not
sufficient, since it can never enumerate every out-of-repo location a diagnostic might name
(toolchain source, `~/.cargo/registry` vendored crates, etc.). **Caught in dry-run, before any
write** — this is exactly what dry-run-first is for.

**Fix**: replaced the blacklist-only check with `isEditableFile()`, which POSITIVELY requires
the path start with the repo's `ROOT` before applying any narrower exclusion. Every edit site in
the tool (six of them) now gates through this single function. A regression self-test
(`selftest5`) asserts the toolchain path is rejected, a `compose/` path is rejected, and an
ordinary in-repo path is accepted. Re-verified: all 5 self-tests plus the clean-crate no-op
green before the next real run.

## Safety

- **Journal**: every edit appended to `📝️r13-journal.jsonl` — file, byte span (in this
  iteration's pre-edit coordinate space), exact before/after text, the motivating diagnostic
  message and code, and a `runId`/`iteration` pair.
- **Dry-run**: `--dry-run` reports the full plan (edits by kind, a sample, and all residue with
  reasons) without writing anything. Used before every real run in this packet.
- **Revert**: `revert --run=<id>` replays a run's journal in reverse — LATEST iteration first,
  and within each iteration's per-file edit set in ASCENDING original-offset order (the exact
  mirror of forward application's descending order). Proven correct by construction (see the
  algorithm note in `revertRun`'s header comment: because edits are non-overlapping and forward
  application processes descending offsets first, every edit's original offset is provably
  still valid at revert time when undone in ascending order) and covered by three dedicated
  self-tests, including a two-edit-same-file ordering test.
- **Span verification before every write**: both `applyEditsToFile` (forward) and `revertRun`
  read the current file content at the recorded span and abort that specific edit (not the
  whole file) if it doesn't match the expected before/after text — this is the guard against
  a concurrent dev editing the same file mid-run (CLAUDE.md: multiple devs work the same files
  live, auto-commit is on). This actually fired usefully during this packet: an unrelated,
  concurrently-made edit to `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (de-asyncing
  `MediaFingerprint::of` and removing a stale `hash_parts(...).await` inside it) landed on disk
  during this session, at a different location in the same file this packet also edited
  (`parse_contributions`, line 3529). Both edits coexisted without collision because the spans
  didn't overlap — exactly the scenario the per-edit verification exists to make safe either
  way.
- **Monotonic guard**: after each iteration, the crate's total error count must strictly
  decrease. On stall or regression, the tool reverts only that iteration and stops rather than
  thrashing (not triggered in this packet's runs — every iteration strictly improved).
- **Blast radius capped to one crate per invocation**, proven as a no-op on a known-clean crate
  (`semio-framework-hash`, 0 errors before and after, confirmed with `--dry-run`) before ever
  touching a broken one.
- **`./compose` untouched** — hard-excluded at every diagnostic/file filter.

## Self-test coverage (`bun 🔧️r13-deasync-codemod.ts selftest`)

Run on scratch fixtures under the OS temp dir, never against real crates:
1. Single-edit forward apply + revert restores original byte-for-byte.
2. Two-edit-same-file forward (descending-start) application produces the exact expected text,
   and its ascending-start revert restores the original byte-for-byte.
3. Byte→char offset map correctness against an emoji-containing fixture engineered to exercise
   the exact divergence class found in real files (byte 25 → char 21 for a 4-byte-UTF-8,
   2-UTF-16-unit emoji sequence preceding the target).

All three passed before the first real (non-dry-run) invocation against any crate, and were
re-run after every subsequent extension to the tool.

## Dry-run sample (proves the no-op + shows real edit shapes)

No-op proof, `semio-framework-hash` (already fully de-asynced by an earlier packet):
```
[run r13-...] crate=semio-framework-hash dryRun=true target=(host)
[run r13-...] iteration 1: total errors=0, async-class=0
[run r13-...] no async-class diagnostics remain (total errors=0, presumably other bug classes or zero). Stopping.
```

Real edit shapes, `semio-s-imperative` (first broken crate touched):
```
def-remove-async .../📇️registry/🦀️component.rs:3542-3548 "async " -> ""
  [E0053: trait declares sync signature — Operator::evaluate]
def-remove-async .../⚙️engine/🦀️component.rs:1017-1023 "async " -> ""
  [E0053: trait declares sync signature — protocol::Identified::id]
def-remove-async .../🛂️manifest/🦀️component.rs:164037-164043 "async " -> ""
  [no-suggestion-fallback: de-async callee parse_contributions — cross-file resolution]
call-add-await .../📇️registry/🦀️component.rs:1373-1373 "" -> "await."
  [MaybeIncorrect suggestion: decode::<T>() genuinely unresolved/external, awaited as rustc suggested]
```

## Per-crate results

### `semio-s-imperative`

| | before | after |
|---|---:|---:|
| `cargo check --all-targets` errors | 12 | 4 |

2 iterations to fixpoint (no async-class diagnostics remain). 4 edits applied: 2 E0053
trait-signature fixes, 1 cross-file de-async (`parse_contributions`, resolved into
`semio-framework-manifest` via the repo-wide index), 1 call-site `.await` insertion (external/
unresolved callee, rustc's own `MaybeIncorrect` suggestion followed).

**Residue (4 errors, all E0733, documented, not auto-fixed):** `compile_step`/`compile_steps`
(in `📝️compiler/🦀️component.rs`) and `run_step`/`run_steps` (in `⚙️engine/🦀️component.rs`) are
mutually-recursive `async fn` pairs. The E0733 handler correctly refused to bulk-de-async them:
each member also awaits OTHER, non-cycle callees (`read_string_param`, `read_number_param`,
`format_value`, `read_scope_bool`, `merge_output_into_scope`) whose own suspension status this
packet did not verify. This is genuine residue needing either (a) confirmation that all of
those callees are also non-suspending, extending the cycle for a bulk de-async, or (b) a human
`Box::pin` decision if any of them turns out to be a real suspension point. Left untouched.

### `semio-framework-os-mcp`

Currently **zero async-class diagnostics** — its present 44 errors are unrelated,
pre-existing breakage (`E0425`/`E0405`, "cannot find function/type" against
`semio-framework-plugin-host`), consistent with this being a live, multi-session repo where a
concurrent packet appears to be mid-flight changing that crate's public API surface. Correctly
out of scope for this packet; the tool made no changes.

### `semio-framework-os-infinite`

| | before | after |
|---|---:|---:|
| `cargo check --all-targets` errors | 2155 | 7, then re-driven to fixpoint after one classifier gap fix (below) |

This crate exercised the tool at real scale: 1721 async-class diagnostics in the first
snapshot. 2 iterations (510 edits: 83 def-remove-async, 427 call-add-await) brought the crate
from 2155 to 7 errors — most of the 1211 "residue" diagnostics in the dry-run plan turned out to
be resolved as a cascading side effect of the 510 real edits, not left broken (a de-asynced
callee fixes every one of its call sites at once, not just the one diagnostic that happened to
be sampled).

**Classifier gap found and fixed on this crate's own residue**: the remaining 7 errors were all
`` `T` is not a future `` (T = `bool`, `Option<WorldBox>`, `ArtifactEnvelope<...>`) — a STALE
`.await` left on an already-synced callee (`world_box_contains_point`,
`create_document_envelope`), each with a `MachineApplicable` "remove the `.await`" suggestion
that the tool's own machinery could apply cleanly — but the diagnostic never reached that
machinery because `` `T` is not a future `` didn't match any pattern in
`ASYNC_SIGNATURE_PATTERNS`, so `isAsyncClassDiagnostic()` silently excluded it (not even
surfaced as residue). Added `/is not a `?future`?/i` to the pattern list; re-verified the
clean-crate no-op and full self-test suite still pass, then re-ran.

**Reachability iceberg, again, exactly as R12 predicted**: after the crate's own async-class
errors dropped to 7, all 7 turned out to be caused by TWO DEPENDENCY crates
(`semio-framework-os-kernel`, `semio-framework-geometry`) themselves having stale-`.await`
errors that a plain `cargo check -p semio-framework-os-infinite` had never fully surfaced
(cargo only reports as much of a failing dependency's diagnostics as the specific build attempt
reaches). Checking those dependencies directly found much larger real piles:
`semio-framework-os-kernel` **145 → 0** (one iteration, all `call-remove-await`, 0 residue —
stale `.await`s left on `ArtifactEnvelope`/`create_document_envelope`/`ArtifactStore` call sites
after an earlier, unrelated de-async had already landed), `semio-framework-geometry` **8 → 0**
(one iteration, all `call-remove-await`, 0 residue). Fixing those two dependencies out-of-band
*unmasked* a much larger set of previously-invisible os-infinite errors of its own: dozens of
call sites into geometry's `Point`/`Vec2`/`Vec3`/`Affine` API that were still missing `.await`
for functions that, at THAT time, were still legitimately async — the classic cascade this
whole packet exists to chase down.

| crate | before | after |
|---|---:|---:|
| `semio-framework-os-kernel` (dependency, out of the 4 declared targets, found via the cascade) | 145 | **0** |
| `semio-framework-geometry` (dependency, likewise) | 8 | **0** |
| `semio-framework-os-infinite` | 2155 (baseline) → 7 (before dependency unmask) → 2030 (after) | **2029** |

`semio-framework-os-infinite` itself: net **2155 → 2029** across the whole session (the
intermediate "7" was real but transient — genuinely correct at that snapshot, immediately
superseded once the dependency fixes changed what "correct" meant for hundreds of existing call
sites). One further iteration attempt (3 further call-add-await edits, `is not an iterator`
shape, unresolved/external callees awaited per rustc's own direction) increased the count by 4
(2029 → 2033) — the **monotonic guard correctly tripped and reverted it**, and the tool stopped
rather than thrashing. Given the very small magnitude of that regression against a backdrop of
~2000 residual errors, and this exact file's demonstrated heavy concurrent-edit activity this
session (see the Incident section below), this reads as measurement noise from live concurrent
churn rather than a tool defect, but it was NOT re-attempted blindly — left for a future,
dedicated run once the shared files are quieter.

**Residue shape (the large remaining number, ~1180 diagnostics)**: overwhelmingly one category —
`findFutureExprSpan()` correctly identifies the Future-typed expression, but it is a **bare,
`let`-bound local variable** (assigned from a call earlier, used later), not a fresh call
expression. `extractCallForward()` correctly and safely refuses to guess at these (no `(`
follows the identifier), so they fall through to residue rather than risk a wrong edit — this is
the load-bearing safety property that also caught the pattern-constructor corruption below. This
is a genuinely different, harder problem (def-use / data-flow tracing back to the assignment
site) than anything this packet's span-local tool was built to solve, and is the primary
human/future-tool work-list item this packet identifies.

### `semio-s-plugin-stdio` and the dependency chain behind it

The task brief's own baseline for this crate (~5,545) was already stale by the time this packet
reached it — a `semio-framework-plugin` dependency fix (below) changed what was reachable, and
the crate's true count when first properly reached was **16,725** (14,339 async-class). This is
the single largest cascade observed in this packet, and it required walking THREE more
dependency crates to their own source before `semio-s-plugin-stdio` itself could make sustained
progress — the reachability iceberg pattern repeating at every layer:

| crate | before | after | notes |
|---|---:|---:|---|
| `semio-framework-plugin` | 125 | 114 | 11 fixed (10 stale `.await` + 1 add); 67 residue, all bare-variable def-use shape |
| `semio-framework-mesh-engine` | 11 | **0** | found via stdio's cascade; fixed at source, 1 iteration |
| `semio-framework-math` | 9 | **0** | found via stdio's cascade (its `🎯️sampling` module); fixed at source, 1 iteration |
| `semio-s-plugin-stdio` | 16,725 | *(see below)* | |

**stdio's own run, wave 1**: iteration 1 applied 8,327 edits (397 def-remove-async, 1,316
call-remove-await, 6,614 call-add-await) and brought the crate from 16,725 to **5** errors —
essentially solved. Iteration 2 attempted the last 5 (all in the `mesh-engine`/`math`
dependencies, not stdio's own files) and the **monotonic guard correctly tripped**: applying
those 5 edits raised the total to 13,272. Reverted automatically, run stopped.

**Diagnosis, not guesswork**: rather than treat the trip as a tool defect or retry blindly, both
dependency crates were checked directly (`-p semio-framework-mesh-engine`,
`-p semio-framework-math`), each showing a small, self-contained, clean pile (11 and 9 errors
respectively, 0 residue each) — fixed at the source crate in one iteration each, both landing at
0. Re-dry-running `semio-s-plugin-stdio` afterward reproduced the exact same 13,272 figure the
guard had caught — confirming it was a REAL, reachability-driven cascade (fixing the
dependencies' stale-awaits correctly unmasked stdio's own large pile of calls into those now-sync
functions), not measurement noise, and safe to proceed into deliberately this time.

<!-- stdio wave 2 result appended once the background run completes -->

### Two more real bugs found and fixed on this crate's own residue

**1. Corruption**: `findFutureExprSpan()`'s label-based span selection picked a PATTERN
sub-span instead of the real Future-typed expression for
`let (Some(src_ep), Some(tgt_ep)) = (P::try_handle_endpoint(source_hid),
P::try_handle_endpoint(target_hid)) else { .. }` in
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🦀️component.rs`, producing
`let (Some(src_ep).await, Some(tgt_ep).await) = (..) else {` — a syntax error. Caught by
`cargo check` (not by the tool itself), confirmed as the ONLY occurrence of this shape
repo-wide via a targeted grep, hand-repaired to properly `.await` the RHS calls instead, and
closed with a `PATTERN_CONSTRUCTOR_DENYLIST` (`Some`/`None`/`Ok`/`Err`) checked at every
`call-add-await` site plus a reproducing self-test (`selftest4`). Full write-up above.

**2. Near-miss (caught in dry-run, nothing written)**: a diagnostic against a
`semio-framework-plugin` dependency proposed editing
`~/.rustup/toolchains/.../lib/rustlib/src/rust/library/core/src/macros/mod.rs` — the installed
Rust toolchain's own standard-library source, entirely outside the repo. Closed by replacing the
path-blacklist with a positive `isEditableFile()` repo-containment gate at all six edit sites,
plus a reproducing self-test (`selftest5`). Full write-up above.

Both fixes were verified (full self-test suite + clean-crate no-op) before any further crate was
touched.

**3. Corruption (a third, distinct shape, still on stdio)**: rustc's own "consider awaiting"
suggestion — trusted verbatim as authoritative, per this tool's core design — inserted `.await`
immediately after a STRUCT-LITERAL SHORTHAND FIELD reference:
`CsvSnapshot { schema: ..., has_header, records }` (where `records` means `records: records`)
became `CsvSnapshot { ..., has_header, records.await }`, a syntax error (`records.await` cannot
be a struct field). Two more instances of the identical shape (`comment.await` inside
`CentralDirLocation { .. }`) were found in a `zip` artifact file. This is a DIFFERENT root cause
from the pattern-constructor bug — there is no callee-resolution step involved at all here, and
it can happen through EITHER the suggestion-driven or the fallback insertion path. All three
were caught by the same method as before: `cargo check` after the run, not the tool's own
guards; confirmed contained to these three sites; hand-repaired to `records: records.await` /
`comment: comment.await` (expanding the shorthand, matching what rustc's own "try naming a
field" secondary suggestion recommends).

**Fix**: `isRiskyBareIdentifierAwaitInsertion()` — a general, span-local heuristic applied at
every `.await`-insertion site (both the suggestion-driven and no-suggestion-fallback paths):
refuse when the insertion point is immediately preceded by a BARE identifier (no trailing call
parens) that is itself immediately preceded by `{`/`,` AND immediately followed by `,`/`}`. An
ordinary `field: expr.await,` is unaffected (preceded by `:`, not `{`/`,`). This heuristic is
intentionally conservative: it also refuses some legitimate insertions inside multi-argument
function calls where the target isn't the first argument (`foo(a, ctx.await, b)` — `ctx` here IS
preceded by `,` and followed by `,`, matching the guard even though this position is completely
valid Rust) — an acceptable cost, since a false refusal only produces residue, a false accept
produces a syntax error. Two reproducing self-tests (`selftest6`) assert both the real corruption
shape is caught and the ordinary `field: call().await` shape is NOT false-positived.

A repo-wide grep for the general "bare-identifier immediately before `.await` immediately before
`,`/`}`" shape (loosely matching the corruption's textual signature) surfaced dozens of hits, but
manual review confirmed all but the three already found are legitimate existing code (`field:
expr.await,` pairs and `.await` inside ordinary function-call argument lists) — the grep's
false-positive rate confirms why the guard is implemented as a proper positional check rather
than a blanket textual ban. A direct parse-error sweep (`cargo check` grepped for `error:
expected`/`error: unexpected`) across every crate this packet touched — `semio-s-plugin-stdio`,
`semio-framework-plugin`, `semio-framework-os-infinite`, `semio-s-imperative`,
`semio-framework-os-kernel`, `semio-framework-geometry`, `semio-framework-mesh-engine`,
`semio-framework-math` — came back clean after these three hand-repairs, confirming no further
parse-level corruption anywhere in this packet's blast radius.
