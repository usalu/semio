# terra-test-attr-restore — packet report

Executor packet `test-attr-restore`. Goal: restore `#[test]`-above-`async fn` sites left illegal by the
universal-async codemod, across `semio-framework-os-kernel-db`, `semio-framework-plugin-host`, then
`semio-framework-plugin`, verifying genuinely-running test suites (not just converted attributes) after
each crate.

## Bottom line

| Crate | `--all-targets` errors (before → after) | `cargo test --lib` | Forced-rebuild dropped-future census |
|---|---|---|---|
| `semio-framework-os-kernel-db` | 2062 → 0 | **424 passed, 0 failed** | 0 (44 found and fixed) |
| `semio-framework-plugin-host` | 919 → 0 | **122 passed, 0 failed, 1 ignored** | 0 (30 found and fixed) |
| `semio-framework-plugin` | ~1384 → **536** | not runnable yet (test code doesn't compile) | not applicable — crate never reached green this packet |

`semio-framework-plugin`'s plain `cargo check -p semio-framework-plugin --lib` (production code only, no
test compilation) is **green** — this was the coordinator's explicit top priority mid-packet and is
confirmed holding. `cargo test -p semio-framework-plugin --lib` (which does compile `#[cfg(test)]` code)
still has 536 compile errors remaining in test code. This crate's test suite is **not finished** — see
"What's left" below. Per the packet brief's own instruction ("if a whole crate turns out to be a much
bigger job than the attribute swap, finish the three named crates properly and REPORT the rest rather
than half-converting a long tail"), this crate turned out to be roughly 3-4x the size of db/plugin-host
combined (its own test module lives in one ~21,200-line file) and is being reported rather than declared
done.

## Regression guards (re-verified this session, all green)

- `cargo test -p semio-framework-plugin-host --lib` → 122 passed, 0 failed, 1 ignored — EXIT 0
- `cargo test -p semio-framework-plugin-host --lib --all-features` → 122 passed, 0 failed, 1 ignored — EXIT 0
- `cargo test -p semio-framework-async --lib` → 17 passed, 0 failed — EXIT 0
- `cargo test -p semio-framework-os-kernel-db --lib` → 424 passed, 0 failed — EXIT 0
- `cargo test -p semio-framework-os-kernel --lib` → **exactly 779 passed, 0 failed** — baseline unmoved
- `cargo check -p semio-framework --all-targets` (shared framework crate `manifest`/`io`/`kernel` modules) → EXIT 0
- `cargo check -p semio-framework-plugin --lib` (production code only) → EXIT 0

`cargo test -p semio-framework-plugin --lib` and `--lib --all-features` are the two guards that do
**not** currently pass (536 test-code errors) — flagged explicitly rather than glossed over.

## `semio-framework-os-kernel-db` — fully done

Reduced `--all-targets` from 2062 errors to 0. `cargo test --lib` went from non-compiling to 424 passed /
0 failed. Forced-rebuild dropped-future census (`cargo clean -p` then check, grep `unused implementer
of`) found 44 dropped futures — all in test fixtures/assertions that were compiling clean as warnings,
never as errors — fixed all 44, re-census confirmed 0.

Representative fixes: sync-closure `.await` hoisting in `testkit`, `poll_once`/`block_on_ready`
architectural-tension accessor fixes, R9 revert of `decode_wal_bytes` (tagged `// 🚫️async: E1-adjacent`),
struct-literal shorthand corruption (mode-2) repair in `wal.rs`/`compact.rs`, a comment-interleaved gap in
`async-test-attr.py`'s regex (found via custom scan, fixed by hand — noted as a tool-limitation class
below).

## `semio-framework-plugin-host` — fully done

Reduced `--all-targets` from 919 errors to 0. `cargo test --lib` → 122 passed / 0 failed / 1 ignored (the
one ignore is pre-existing and legitimate — needs a pre-built wasm32-wasip2 fixture, documented in the
test's own doc comment). Forced-rebuild dropped-future census: found 30 dropped futures across
`🧵️shard/🦀️component.rs` (`pack_encode`/`push_inbound`/`script_turn`/`script_job_step` call sites),
`🧵️shard/🏃️executor.rs`, `⚡️effects/🦀️component.rs`, and `🧵️shard/🚚️process-transport/🦀️component.rs` —
fixed all, re-census confirmed 0.

Representative fixes: a `macro_rules!`-templated test body invisible to `async-test-attr.py`'s regex
(fixed by hand, documented below as a tool-limitation class), a missing `use crate::GuestRuntime;` trait
import masquerading as ~10 separate "missing method" errors, `E0507` index-then-await corruption on
`Vec<ShardExecutor>`, and a genuinely-sync production `fn main()` in `🧵️shard/👶️child/🦀️main.rs` that
insert-await had illegally awaited (E0728) — fixed by wrapping each step in `block_on`, matching the
file's pre-existing pattern.

## `semio-framework-plugin` — in progress, 536 errors remaining

### What was fixed this packet (partial list, highest-impact first)

- **`LocalizedLabel::data(...)` stale-await sweep**: 162 call sites in this crate's own test module were
  calling `.await` on `LocalizedLabel::data`/`native`, which had already been correctly reverted to sync
  (tagged `// 🚫️async: E1 pure accessor`) by an earlier packet in the shared
  `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`. This single fix eliminated
  ~150 errors at once.
- **R9 reverts** (pure builder methods, blind-codemod-inflated, needed by sync `catch_unwind`/`try_fold`
  closures) applied to the shared `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`:
  `IntroductionStepDefinition` (whole impl: `new`/`introduce`/`show`/`placement`/`interact`/
  `interact_ordered`/`logos`/`demonstrate`), `DialogDefinition` (whole impl), `ActionRef::new`. Verified
  no other in-repo consumer needed these async (checked all call sites repo-wide before reverting;
  `semio-framework` and the `📺️renderer` crate's one sync-`#[test]` consumer both confirmed compatible).
- **R9 revert local to this crate**: `ExtensionBundle::new`/`extends`/`depends_on`/
  `assert_extends_matches_primary_dependency` (its `contributes` method stays genuinely async — real
  registry I/O, not touched); the `subset!` macro's two dialect-registration arms bridged through
  `semio_framework::io::resolve_ready` since `Once::call_once`'s closure is std-fixed sync.
- Dozens of `catch_unwind(|| {...})` → `catch_unwind(move || {...})` conversions hoisting the async
  builder-chain prefix (everything up to but excluding the final `.build_definition()`, which genuinely
  panics and is what the test is proving) out of the sync closure into a preceding `let __chain = ...`
  statement.
- `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs` and `⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs` test
  modules: comprehensive `.await` placement fixes across `start_job`/`step_job`/`checkpoint_jobs`/
  `cancel_job`/`restore_job`/`register_dialect_migration`/`ArtifactContribution::builder(...).mutation(...)
  .build()` chains, plus one genuine non-exhaustive-match gap (`JobStep::Running(None)` uncovered) fixed
  with an explicit panicking arm, not silently wildcarded.
- `⚛️reactor/📸️checkpoint/🦀️component.rs`: `task_restarts()` accessor missing `.await` at 5 call sites.
- `🌐host/📖️body/🦀️component.rs`: `BodyReader::poll_buffered(...)` missing `.await` at declaration, with
  the repeated-use-of-`.await` (mode-1) symptom at each of 5 call sites.
- Production (non-test) code: `crate::app::WireArtifactMutationPlanRequest`/`with_instances_mut(...)`
  dispatch-handling code around `🦀️component.rs:16500-17000` had several missing `.await` on its own
  `Result`-returning helper — not test-only residue, a genuine production-code gap this packet's audit
  surfaced.
- Systemic mode-1 (repeated-`.await`-instead-of-await-at-declaration) recovery across dozens of
  `definition`/`history`/`app`/`item`/`entry`/`outcome` locals, all following the same shape: `let X =
  ASYNC_CALL(...);` (missing `.await`) then `X.await.field`/`X.await` reused at 2+ downstream sites.

### An incident this session, disclosed in full

Partway through, a custom Python script I wrote to batch-hoist `with_instances_mut(|list| {...})` async
declarations used naive `{`/`}` character counting (no string/comment awareness) to find each closure's
matching close-brace. On one match it mis-parsed brace depth and the resulting text reassembly dropped
roughly 16,000 lines from the middle of `🦀️component.rs` (21,243 lines → 5,049 lines) while leaving a
structurally-valid head and tail, so the damage was not obvious from a quick glance. I caught it via the
sanity checks I now run after every batch edit — a plain `wc -l` line-count check flagged it immediately
before I made any further changes on top of the corrupted file.

Recovery: I had a scratchpad snapshot (`plugin_component_fixed.rs`, 21,228 lines) saved earlier this
session at the point I fixed an *earlier*, unrelated file-duplication incident (same file, different root
cause — see below). Restoring from it lost roughly the last third of this session's fixes on this one
file (the R9 reverts, LocalizedLabel sweep, and most of the `catch_unwind` hoisting all had to be
redone), which is reflected in the error count not monotonically decreasing in the session transcript.
Everything reported as "fixed" above reflects the *current, verified* state, re-applied by hand with
individual `Edit` calls (never another blind whole-file script) after the restore, with a `wc -l` +
`grep -c` mode-2/mode-1 audit after every single batch from that point forward.

A **second**, earlier incident in this same file (unrelated cause): a "hoist the entire builder chain out
of a `catch_unwind` closure" script's paren/brace matching went wrong on one match out of fifteen and
duplicated a ~13,500-line tail of the file. Found by `wc -l` immediately after that script ran (34,722
lines instead of ~21,200), root-caused by comparing the two halves line-for-line (they were byte-
identical), and fixed by truncating the duplicate — no data was actually lost that time, just doubled.

**Lesson applied going forward, and worth recording for any future packet touching this file**: no more
whole-file scripts that do their own brace/paren matching. Every fix from the recovery point onward used
either the ticket's existing audited tools (`insert-await.py`, `remove-bad-await.py`,
`fix-repeated-await.py`/`fix-repeated-await-wide.py`) or individual `Edit` calls with a unique,
pre-verified `old_string`, each followed by a `wc -l` line-count sanity check and a
`grep -c '[a-zA-Z_]*\.await,'` / `grep -c '[a-zA-Z_]*\.await }'` mode-2 corruption check before moving on.

### Tool-limitation classes discovered this packet (all already known-documented in this ticket's other
reports, confirmed recurring)

1. `async-test-attr.py`'s regex misses a `#[test]` separated from `async fn` by a plain `//` line
   comment (only handles `///`/`//!`) — found once in db, fixed by hand.
2. `async-test-attr.py`'s regex only matches literal `#[test]` above literal `async fn`; a
   `macro_rules!`-templated test body (`#[test] fn $name() { ...await... }` in the macro's own source,
   `async` only appearing after expansion) is invisible to it — found once in plugin-host, fixed by hand.
3. `fix-repeated-await-wide.py`'s original regex had a `(?!\.)` negative lookahead that silently
   contradicted its own docstring (which claims it handles bare `IDENT.await.FIELD` field-access) — the
   lookahead excluded every dot-followed shape, including the one the docstring specifically claims to
   cover. Found and fixed this session (removed the lookahead); the tool now matches its documented
   behavior. Left in the ticket folder for the next packet.
4. `remove-bad-await.py` only targets `E0277 ... is not a future`; it does not catch the same
   already-resolved-value bug when it surfaces as `E0728` (await outside async fn) because the receiver
   sits inside a still-sync closure — those need the closure itself restructured first (hoisting), which
   is inherently a per-site judgment call, not a further-automatable case.

## What's left for `semio-framework-plugin`

536 `--all-targets` errors remain, broken down by code: `E0277` 184, `E0599` 101, `E0369` 70, `E0308` 89,
`E0609` 51, `E0600` 13, `E0283` 12, `E0608` 11, `E0432` 2, `E0425` 2, `E0659` 1. Both `insert-await.py`
and `remove-bad-await.py` are at a fixpoint against the current state (0 more unambiguous mechanical
edits available) — everything remaining needs the same per-site judgment calls documented above: either
add `.await` at a specific call site after checking the callee's real signature, or identify a further
R9 revert candidate and check its blast radius before reverting. The file is a single ~21,200-line
`component.rs` plus several smaller reactor/job submodules; the test module alone is roughly the size of
the other two crates' entire test suites combined, which is why it was not finished in this packet.

No sweep of the remaining repo-wide `#[test]`+`async fn` sites (outside the three named crates) was
started — per the packet brief's own sequencing, the three named crates come first and this one is not
yet done.

## Files touched this session (plugin crate work)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (primary; ~21,245 lines)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🔀️migrate/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📸️checkpoint/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/📖️body/🦀️component.rs`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (shared; R9 reverts + stale-await cleanup)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/fix-repeated-await-wide.py`
  (bug fix: removed the `(?!\.)` lookahead that contradicted its own docstring)

## Files touched this session (db + plugin-host work — see earlier report content, summarized above)

`semio-framework-os-kernel-db`'s full file list and `semio-framework-plugin-host`'s full file list are
as previously reported; both crates are complete and verified green in this session's final regression
pass (see table above).
