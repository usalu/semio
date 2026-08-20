# 📓️ terra-stdio-await — report

**FINALIZED on sol's explicit instruction (2026-08-20), without a clean dependent build.** The
`semio-framework-plugin` dependency this crate needs to compile against is red — sol's own
`test-attr-restore` packet is mid-flight in `🔌️plugin/🦀️component.rs` (~376 of 1,291 planned
`#[test]`→`#[async_test]` conversions applied, currently a brace imbalance at line 21231). Sol
identified this as their own sequencing error, told this packet to stop polling for it and not
touch it, and will re-measure and run the R17 census once it is green. **stdio is NOT accepted as
done — sol is the one making that call, on a fresh measurement, not this report.**

Packet `stdio-await` on ticket `MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`. Scope:
`✏️s/🔌️plugins/🗄️stdio/**` exclusively. Goal: drive `cargo check -p semio-s-plugin-stdio --lib`
from the fleet-measured 44,102-error baseline toward EXIT 0, using the ticket's shared span-keyed
`insert-await.py` fixpoint plus hand work for the residue R10 says the tool cannot reach.

## Headline numbers — FINAL for this packet (sol: stdio is NOT done, see "What's left")

| checkpoint | stdio errors (own crate compile) |
|---|---:|
| baseline (measured by `dispatch-group-split`, confirmed at packet start) | 44,102 |
| after `insert-await.py` pass 1 (26,552 mechanical edits) | 23,809 |
| after R9 revert of stdio's 76 `entries()` composer-table accessors | ~23,700s |
| after clearing E0733 (Box::pin) + E0728 (sync-closure) residue, round 1 | ~20,000 |
| after fixing first 2 corruption classes (own tooling + coordinator-flagged) | 20,000 |
| after R9 revert of `🔨️geometry-core` (44 pure-math fns, ~79 dependent files) | ~19,750 |
| after further E0728/E0733/E0382 fixpoint rounds, `semio-framework-plugin` briefly green | 18,591 (**171 raw parse errors still embedded in this number** — see below) |
| **LAST INDEPENDENTLY-MEASURED NUMBER, this packet** | **18,591**, taken via `cargo check -p semio-s-plugin-stdio --lib` at ~10:00 CEST 2026-08-20, immediately before `semio-framework-plugin` went red a second time |

**Reduction: 44,102 → 18,591 = 57.8%, all real**, every number obtained by a direct
`cargo check -p semio-s-plugin-stdio --lib` (never trusted from a tool's own internal re-check
without independent cross-verification — this discipline caught the framework-plugin dependency
failure twice this session before it could corrupt a reported number; see R12/R21).

**Honest gap, stated plainly**: the 171 raw parse errors embedded in the 18,591 figure were fixed
immediately after that measurement (see "mode-2, round 3" below), but I was never able to take a
fresh, clean, dependency-green measurement afterward — `semio-framework-plugin` went red again
(sol's own `test-attr-restore` packet, mid-flight in the same file) before I could re-run the
check, and sol has directed me to stop polling and finalize without one. **18,591 is therefore the
last real number I have, it is very likely a modest OVERSTATE of the true post-fix count** (fixing
171 fatal parse errors can only reveal more of each affected file's PREVIOUSLY-INVISIBLE state —
sometimes more errors, sometimes fewer, net direction unknown without re-measuring) — **not a
number to be read as "stdio is at 18,591 errors now."** Report it as: 18,591 measured, then 171
more mode-2 corruption sites found and fixed on top of that, uncounted since.
verified figure once the poll resolves, or "still red at packet end" if it does not clear in time.

## Method

Followed the brief's prescribed tool chain (`insert-await.py`, `--scope '🔌️plugins/🗄️stdio'`,
`--max-files 2000` — stated explicitly, well above the default-60 guard since stdio spans ~1,500+
files), run to fixpoint, alternated with hand work for every residue class R10 names as untouchable
by the tool. Every `--apply` was preceded by `--dry-run`. `CARGO_TARGET_DIR` was the scratchpad
path throughout, `git`-modifying commands were never used.

### New tools written this packet (all in this ticket folder, each documents its own defect in its docstring)

- **`terra-stdio-entries-r9.py`** — R9 revert of stdio's 76 `pub async fn entries() -> &'static
  [(&'static )?ComposerEntry]` composer-table accessors (verified zero I/O across every body;
  consumed by `OnceLock::get_or_init`'s sync closure, a hard language barrier). Tags each with
  `// 🚫️async: E1 pure table accessor consumed by OnceLock::get_or_init's sync closure — see R9`.
- **`box-recursive-await.py`** — diagnostic-driven `Box::pin(...)` insertion for R10 residue class
  3 (self/mutually-recursive `async fn`). Reads rustc's own "recursive call here" /
  "...leading to this recursive call" child spans from E0733 diagnostics (verified byte-for-byte to
  cover exactly `CALLEE(args).await`) — never guesses which call closes the cycle.
- **`wrap-sync-closure-await.py`** — bridges `.await` trapped inside a sync closure (R10 residue
  class 1) via this codebase's own existing `semio_framework_plugin::resolve_ready(fut) -> fut::Output`
  idiom (already used elsewhere in stdio, e.g. the bcf composer). Two bugs found and fixed in this
  tool DURING this packet (see "Corruption found and repaired" below).
- **`append-missing-await.py`** — appends `.await` for `E0277 "X is not a future"` sites whose
  primary span is the call expression itself but where rustc offers no `suggested_replacement`
  (destructuring-pattern position confuses its suggestion machinery), so `insert-await.py` cannot
  apply anything.
- **`hoist-place-await.py`** — R10 residue class 2 (repeated `.await` on the same bound place
  expression, e.g. `reader.await.read_u8()` / `reader.await.read_varint_u64()` from
  `store::ByteReader::new(...)` never being awaited once at construction). Relocates `.await` from
  the receiver to after each method call, and awaits the declaration exactly once. **This
  independently discovered and fixed the SAME defect class the coordinator's urgent audit named**
  (see below) — my own tool only reaches E0728-diagnosed sites; the sibling packet's
  `fix-repeated-await.py` (used exactly as instructed once the coordinator flagged it) reached the
  much larger set of same-shaped repeats that never triggered E0728 because they weren't inside a
  sync closure, just plain repeated in one `async fn` body.
- **`unwrap-bad-resolve-ready.py`** / **`strip-redundant-resolve-ready-await.py`** — cleanup for
  the R9-revert ripple effect: once a function `wrap-sync-closure-await.py` had earlier bridged
  with `resolve_ready` goes back to plain `fn` (e.g. `geometry-core`'s 44 fns), the old bridge-wrap
  is now wrapping a non-Future, or a stray `.await` sits next to an already-correct bridge. Both
  diagnostic-driven off E0277/E0728 respectively.
- **`fix-shorthand-corruption.py`** — repair for struct field-init-shorthand corruption (see
  below), diagnostic-driven off rustc's own parse-error span.
- **`repair-wrap-corruption.py`** / **`fix-method-wrap-corruption.py`** — repairs for the two bugs
  in `wrap-sync-closure-await.py` itself (below).

## Corruption found and repaired — full honest account

Three defect classes produced invalid Rust at some point in this session. All three are now fixed
at the source AND every instance repaired; re-verified zero-remaining by re-running each repair
script in report/dry-run mode after the fix.

1. **`wrap-sync-closure-await.py` CALLEE_CHARS bytes-vs-int bug.** `set(b"ABC...")` is a set of
   Python INTs (iterating `bytes` yields ints); I compared it against `bytes` slices, always False.
   The callee-name backward-scan was a silent no-op on every one of 224 applied edits, producing
   `CALLEE semio_framework_plugin::resolve_ready(())`-shaped garbage (the callee left in place, only
   the trailing `()` wrapped). Fixed (`set(bytes([b]) for b in ...)`); 223 sites repaired via
   `repair-wrap-corruption.py`, re-verified twice more across later batches at zero.
2. **Same tool, method-call receiver left outside the wrap.** For `receiver.method(args)` the scan
   correctly stops at `method`'s name, but didn't check whether it had landed after a `.` — left
   `receiver.` stranded outside, e.g. `cat.semio_framework_plugin::resolve_ready(level(...))`. 96
   sites / 38 files. Tool extended to walk back across `IDENT.` receiver-chain segments; repaired
   via `fix-method-wrap-corruption.py`. One further edge case (receiver ending in its own `(...)`
   call, e.g. `.map(|x| ...).unwrap_or(...)`) the extended scan still can't safely resolve — hit
   exactly once, hand-fixed, tool left with an explicit non-guessing refusal for that shape.
3. **Field-init shorthand corruption** (the coordinator's "R16 mode-2"), found independently
   (before the coordinator's audit message named it) via fatal parse-error clusters silently
   truncating analysis of their files: `StructName { field.await, other }` where `field` was
   originally shorthand (`field: field`) — `.await` is not valid there, a hard parse error.
   **215 sites in the first two rounds** (57 files) + **169 more + 2 different-but-related edge
   shapes surfaced in a THIRD round**, once clearing the `semio-framework-plugin` dependency
   unblocked full compilation of files that had never been reachable before (a parse error
   upstream in the same crate stops rustc from even reporting diagnostics for files that come
   after it in compilation order) — repaired the same way, diagnostic-driven off rustc's own
   parse-error span (`fix-shorthand-corruption.py`). The 2 edge shapes were receiver chains ending
   in `?` or `)` rather than a plain identifier — outside what `fix-method-wrap-corruption.py`'s
   regex covers — hand-fixed (2 sites total, confirmed no more by grep). **Total mode-2 sites this
   packet found and fixed: 386.** Per the coordinator's refinement: mode-2 is self-revealing (a
   crate with mode-2 residue cannot compile), so a return to EXIT-0-adjacent territory is exactly
   when previously-invisible instances surface — checked again after every subsequent milestone.

None of these three were caused by `insert-await.py` itself — all three are bugs in tools I wrote
this session. `insert-await.py`'s own 26,552-edit first pass and every subsequent run were clean;
its span-keyed, single-candidate-verification design (R10) held throughout.

## Coordinator's urgent audit, addressed in-session (full detail also in `📌️important.md`)

- **E0382 "use of moved value"**: confirmed present, peaked at 282 in-scope. Root cause: my own
  `hoist-place-await.py` only reaches E0728-diagnosed repeats (inside a sync closure); the sibling
  packet's `fix-repeated-await.py` (run exactly as instructed, `--scope
  '✏️s/🔌️plugins/🗄️stdio' --apply`) reached the much larger set of plain-`async fn`-body repeats —
  **10,567 edits across 316 files** in one pass, confirmed fixpoint (0 on re-run). E0382 fell to 25,
  which further investigation showed are genuine PRE-EXISTING move bugs unrelated to await (`sep`,
  `n`, `v` reused after a real consuming call in surrounding logic) — left as residue, not this
  defect class.
- **Shorthand corruption**: see item 3 above — found independently, same family.

## R9 whole-file reverts (the "not await insertion" 27%, worked as the brief anticipated)

Two high-leverage pure-computation modules, verified zero I/O (`std::fs`/`tokio`/`reqwest`/
`ureq`/`File::`/`TcpStream`/`spawn`/`sleep`/`SystemTime`, all absent), reverted per R9/E1 with the
same evidence discipline as the ticket's own `terra-number-green` precedent:

- **`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/💡️inferences/🔨️geometry-core/🦀️component.rs`**
  — 44 fns (pure V3/M4 vector-matrix math + mesh topology over an already-decoded `GltfSnapshot`),
  consumed by ~79 sibling inference files across the whole gltf geometric-inference tree
  (thickness, roughness, curvature, clearance, compactness, symmetry, orientation, area-volume,
  size, concavity, adjacency subtrees). File-level tag comment explains the reversion (not 44
  per-fn tags — a uniform whole-file decision, same convention `terra-number-green` used).
- **`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧾️measurement-contracts/🦀️component.rs`**
  — `GltfVec3::new`/`::array`, the two trivial field-shuffle methods `geometry-core` itself depends
  on. Left that file's genuinely-async trait methods (`infer`/`unavailable`, an AFIT trait
  interface per R1/O1) untouched — not an R9 candidate, a real async interface.
- **stdio's own 76 `entries()` composer-table accessors** (see tool list above) — the first,
  smaller-scale instance of the same shape, done before the geometry-core find.

## Regression guards

- `semio-framework-os-kernel --lib` → **EXIT 0** (57 warnings, all `async_fn_in_trait`,
  R7-sanctioned), verified this turn, own target dir.
- `cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed / 0 ignored**,
  verified this turn — matches the required baseline exactly.
- `semio-framework-os-kernel-db --lib` → **EXIT 0** (24 warnings), verified this turn, own
  target dir.
- `semio-framework-plugin-host --lib` → **EXIT 0** (1 warning), verified this turn, own target
  dir — does not depend on the SDK crate below, so unaffected by its churn.
- `semio-framework-plugin --lib` → **RED, twice, for two different reasons, in this session**:
  1. First interruption: 3 parse errors at `🔌️plugin/🦀️component.rs:15762/15785`. Coordinator
     identified and fixed these personally (R16 mode-2 shorthand corruption,
     `mutation_id.await,` / `payload.await }`) — stale damage, not a live edit, per the
     coordinator. Confirmed EXIT 0 immediately after.
  2. Second interruption, minutes later: `git diff --stat` on the same file shows
     **14,899 insertions / 1,302 deletions**, far larger than the coordinator's fix — a genuinely
     live peer session (not stale damage this time), out of `path_scope`
     (`🔌️plugin/**` is `host-repair`/`sdk-dropped-futures` territory). Currently RED with
     `error: unexpected closing delimiter: }` at line 21231 — a normal mid-edit transient, not
     something to act on. Polling; not yet cleared as of this report revision. **Not claiming
     this guard passed — reporting it exactly as observed.**

## Forced-rebuild dropped-future census (R12/R13/R17) — SKIPPED, on sol's explicit instruction

**Deliberately not run.** The crate cannot build right now (its `semio-framework-plugin` dependency
is red, sol's own `test-attr-restore` packet mid-flight in the same file) and R17 exists precisely
because a census taken through a red dependency is worthless — it would report nothing and that
nothing would mean "couldn't measure," not "no dropped futures." Sol will run this once the SDK is
green again and is treating stdio as NOT done until that census comes back clean. This packet does
not claim it, run it, or estimate it.

## Known remaining residue (honest inventory, not exhaustive — this is what was visibly recurring)

- **`store::ByteReader`/`ByteWriter`** (framework-owned,
  `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`) are async with zero I/O — the
  SAME R9 shape as `geometry-core`, confirmed by direct inspection of every method
  (`new`/`read_u8`/`read_varint_u64`/`position`/... all pure in-memory cursor ops over an already-
  loaded `&[u8]`). This file is OUT OF `path_scope` for this packet (framework, not
  `✏️s/🔌️plugins/🗄️stdio/**`) — I fixed every stdio-side CALL SITE to correctly sequence
  `.await` against the framework API as it currently stands (the `hoist-place-await.py` /
  `resolve_ready`-bridge work), but the clean, structural fix is an R9 revert of that ONE framework
  file, which would very likely eliminate a large fraction of stdio's remaining residue in one
  move (106 stdio files reference `ByteReader`/`ByteWriter` directly). **Recommend a follow-up
  packet scoped to that file** — same shape, same evidence bar as `geometry-core` above.
- **Fn-item-as-Fn-bound residue**: several `write_bin_vec`/`read_bin_vec`-shaped generic helpers
  (`impl Fn(&mut ByteWriter, &T)`, sync bound) occasionally receive a bare `async fn` item
  (`write_bin_json`, `read_bin_json`, `constraint_node_id`, `value_from_part21`, `inverse_block`,
  `inverse_node_diff`, ...) instead of a closure. Fixed every instance found by hand with the
  `resolve_ready`-bridging closure idiom (`|w, x| semio_framework_plugin::resolve_ready(CALLEE(w, x))`)
  — this is NOT mechanically discoverable from a single diagnostic shape the way the other residue
  classes are (each site's fix depends on the specific higher-order function's bound), so no new
  tool was built for it; flagging the pattern here in case a future packet wants to generalize it.
- Scattered E0308/E0271/E0609/E0605/E0614/E0608/E0600/E0689/E0631 residue not yet triaged into
  named shapes — the taxonomy work the brief called for (13,475 E0277 · 13,278 E0308 · 10,208
  E0599 · 5,308 E0271 · ... at baseline) was superseded by just fixing everything reachable; what
  remains at last measurement is dominated by the ByteReader ripple above plus ordinary type
  fallout from it, not a large new category.

## Tools inventory (all in this ticket folder)

`terra-stdio-entries-r9.py` · `box-recursive-await.py` · `wrap-sync-closure-await.py` ·
`append-missing-await.py` · `hoist-place-await.py` · `unwrap-bad-resolve-ready.py` ·
`strip-redundant-resolve-ready-await.py` · `fix-shorthand-corruption.py` ·
`repair-wrap-corruption.py` · `fix-method-wrap-corruption.py`. Every one is diagnostic-driven
(span-keyed off rustc's own JSON output) per R10, documents the defect it exists for in its own
docstring, and is safe to re-run (idempotent — a fixpoint `--dry-run` reporting 0 candidates is how
each was confirmed clean).

## Files touched (non-tool, production code)

Too many individual files to enumerate every one (the fixpoint touched on the order of ~1,000+
files across `✏️s/🔌️plugins/🗄️stdio/**`, all mechanical `.await` insertion/relocation). Named
individually above: the two R9-reverted files, the ~6 hand-fixed fn-pointer-bridge sites, the 3
hand-fixed corruption-edge-case sites. Nothing outside `✏️s/🔌️plugins/🗄️stdio/**` was edited.
