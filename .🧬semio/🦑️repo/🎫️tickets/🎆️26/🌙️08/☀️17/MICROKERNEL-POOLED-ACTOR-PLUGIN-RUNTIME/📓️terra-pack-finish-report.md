# 📓 terra · pack-finish report

Packet: `pack-finish` (U-program, critical path — last crate before `semio-framework-plugin`, the
guest SDK, can be scoped). Crate: `semio-framework-pack`
(`🧰️framework/🔨️modules/🎒️pack/`). Owned/edited paths, confirmed by `git status --porcelain --
🧰️framework/🔨️modules/🎒️pack/` at the end of this packet (nothing outside this tree touched):

- `🎒️pack/📐️format/🦀️component.rs`
- `🎒️pack/🔌️io/🦀️component.rs`
- `🎒️pack/⏳️async/🦀️component.rs`
- `🎒️pack/🌐️http/🦀️component.rs`
- `🎒️pack/📦️packages/🦀️rust/Cargo.toml`
- `🎒️pack/📦️packages/🦀️rust/📦️glue.rs`

## 0. Starting measurement, re-confirmed fresh

`cargo check -p semio-framework-pack --lib` at the start of this packet: **exit 101**, 67 real
error blocks (E0277 ×35, E0308 ×15, E0369 ×9, E0728 ×3, plus 5 "method should be `async`" —
the brief's tally covered the `E`-coded ones; the 5 async-trait-shape errors were the uncounted
remainder). All 67 were located inside two files: `📐️format` (62) and `🔌️io` (5). Verified via
`--message-format` span parsing, not eyeballing.

## 1. The measurement oddity — resolved: warnings, not errors, and not feature-driven

The brief flagged that `cargo check -p semio-framework-pack --lib` cites files under
`📡️replication` (`🎮️mutation`, `📡️wire`, `🚰️source`, `⚙️codec`, `🔐️crypto`, `🔗️causal`) despite sol
having verified that crate green in isolation. I parsed the check output into individual
diagnostic blocks (first `-->` span per block) rather than grepping raw file mentions, and found:

- **68 `error` lines** (67 real blocks + 1 summary line) — **all 67 located inside `🎒️pack`**
  (`📐️format` or `🔌️io`). **Zero error blocks reference any `📡️replication` file.**
- **60 `warning` lines**, all attributable to `semio-framework-replication`: 39×
  `async_fn_in_trait` (replication has no crate-root `#[allow(async_fn_in_trait)]` yet — not my
  file to add one to) and 20× `unexpected cfg condition value: typegen` (a `#[cfg_attr(feature =
  "typegen", ...)]` gate in `📡️wire`/`⚙️codec` referencing a feature replication's own `Cargo.toml`
  doesn't declare — present regardless of which feature set is active, so not feature-unification
  fallout either) plus one `generated N warnings` rollup line.

**Conclusion: not a bug, not feature-driven blocking.** `semio-framework-replication` compiles
with **zero errors** even under `pack`'s feature-unified build (`deflate` enabled, since `pack`'s
`default = ["deflate"]` forwards to `semio-framework-replication/deflate`) — confirmed by
literally counting error blocks by file, a differently-implemented check from the brief's raw
grep. The files named in the brief only ever appear in **warning** diagnostics (pre-existing lint
noise in replication, not owned by this packet), never in an `error[...]` block. **No
lease-request needed** — replication was never actually broken by `pack`'s build; the earlier
citation the brief was working from was measuring warnings as if they were errors, or was taken
before sol's `semio-framework-replication` fix landed. `semio-framework-replication`'s own
`--lib`/`--all-targets` status was not re-measured by me (out of scope, not touched) — I can only
confirm it produces zero errors when built as `pack`'s dependency with `deflate` on, which is the
consumer-feature-flags check rule 22 asks for.

## 2. Cause 1 — pure-computation-made-async: the recipe

### The actual root: not a local CRC bug, a *propagated* one

The brief's hypothesis was that a CRC helper had been wrongly made `async` and should be
de-asyncified like `🌱️value`/`⚠️diagnostic`. Investigating the call graph (not just the three cited
lines) found something structurally different:

`crc32c` is **not defined in `pack` at all** — it, and every varint primitive `format.rs` builds
on (`read_varint_u64`, `write_varint_u64`, `read_varint_i64`, `write_varint_i64`,
`is_minimal_varint`), live in `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️component.rs`, re-exported
into `pack`'s namespace via `📦️glue.rs`'s `pub use protocol::codec::*;`. **All of them are already
`pub async fn`** in replication — confirmed by direct read, not inference — and `PackSource`/
`PackSink`/`CompressionCodec` (also replication-owned) are **already plain-AFIT `async` traits**.
`replication` is explicitly outside this packet's writable paths.

I checked each for I/O markers per R9's decision procedure (`std::fs`, `tokio`, `reqwest`, `ureq`,
`File::`, `TcpStream`, `spawn`, `sleep`, `SystemTime`): **zero** in `crc32c` (a 256-entry
lookup-table CRC-32C) or in the varint helpers (pure byte-slice arithmetic). So by R9's letter
these functions *could* be sync — but I don't own the file they live in, and more importantly:
**every one of `format.rs`'s consumers of these helpers can become `async`.** R9 rule 3 governs
this case, not rule 2: *"If it is pure and every consumer can become async → make the consumer
async instead. That is the direction the decree wants; R9 is a fallback, not a shortcut."*

### Per-function decision, applied mechanically once the direction was fixed

I walked every function in `📐️format/🦀️component.rs` and classified it by whether its body calls
an already-async fn (transitively, `crc32c`/`read_varint_u64`/`write_varint_u64`/`PackSource`/
`PackSink`/`CompressionCodec` methods):

- **Became `async fn`** (calls an async fn somewhere in its body): `Header::write_bytes`,
  `Header::parse`, `Footer::write_bytes`, `Footer::parse`, `codec_compress`, `codec_decompress`,
  `encode_segment`, `read_u8_at`, `read_varint_u64_at`, `decode_segment_at`, `encode_symbols`,
  `decode_symbols`, `encode_chunk_table`, `decode_chunk_table`, `write_span`, `read_span`,
  `encode_manifest_bytes`, `parse_raw_manifest`, `PackWriter::{begin,position,write_segment,
  write_chunk,finish}`, `PackFile::{open_superblock,open_manifest,read_chunk,body_bytes}`,
  `read_footer_only`, `recover`. 26 functions total in this file.
- **Stayed sync, tagged**: `resolve_manifest` (`// 🚫️async: E1 pure struct-field resolution — no
  I/O, no call into any async fn`, genuinely zero async dependency, unlike its siblings in the
  same region), `VerificationLevel::checks_crc`/`checks_content_hash` (pure enum-predicate, same
  tag), and the pure getters on `PackFile`/`PackWriter` that touch only `self` fields
  (`superblock`, `manifest`, `symbol`, `chunk_count`, `chunk_range`, `content_hash`, `position` on
  `PackFile` — n/a, that one's on `PackWriter` and *is* async since it calls `sink.position()`).

Same treatment in `🔌️io/🦀️component.rs`'s native module: `impl PackSource for FilePackSource` and
`impl PackSink for FilePackSink` were plain sync `fn` bodies implementing the already-async
`PackSource`/`PackSink` traits (a staleness bug, not an `#[async_trait]` one — see §3).  Converted
`len`/`read_at`/`write_all`/`position`/`flush` to `async fn`; the bodies stay genuinely
synchronous `std::fs` blocking calls (no `.await` inside), matching the crate's existing idiom of
doing blocking I/O inside an `async fn` body rather than pretending it suspends (the file already
does this for `ureq` in `🌐️http`). `StreamingPackWriter::{create,write_segment,write_chunk,finish}`
and `recover_file` became `async fn` purely to `.await` their calls into now-async
`PackWriter`/`format::recover`.

### One subtlety the propagation surfaced: async futures capturing a shared borrow

`insert-await.py --scope '🎒️pack'` reached fixpoint on the first pass with 0 edits/0 ambiguous,
but flagged one E0502 the tool correctly refused to auto-fix: in `Header::write_bytes` (and its
Footer/`encode_segment` siblings), the original shape

```rust
let crc = crc32c(&buf[0..20]);
buf[20..24].copy_from_slice(&crc.await.to_le_bytes());
```

holds `crc` (a future capturing `&buf[0..20]`, immutable) alive into the same statement that
mutably borrows `buf[20..24]` for `copy_from_slice`'s receiver — a genuine borrow conflict now
that the surrounding fn is actually `async` and the compiler enforces it. Fix: resolve the future
to its value on its own line first (`let crc = crc32c(&buf[0..20]).await;`), then use the plain
`u32`. Applied at all three sites; second `insert-await.py` pass then reached a genuine 0/0/0/0
fixpoint.

### Tests

Every `#[test] fn` that touched a now-async fn was converted to
`#[semio_framework_async_macros::async_test] async fn` with `.await` added at each call site (24
tests in `format.rs`, 8 in `io/component.rs`'s native tests — the two `write_atomic_*` tests stay
plain `#[test] fn` since `write_atomic` never touches `PackSource`/`PackSink`). Added
`semio-framework-async-macros` as a dev-dependency in `pack`'s `Cargo.toml` (same relative path
replication's own `Cargo.toml` already uses: `../../../⏳️async/✨️macros/📦️packages/🦀️rust`).

### The recipe, generalized (repo-wide pattern, second worked example after `🌱️value`/`⚠️diagnostic`)

1. Find where the "pure" helper is actually *defined* — not just where it's called. A blind
   `.await`-insertion or de-asyncify pass at the call site is wrong if the helper lives in a
   crate/file you don't own; check `use`/`pub use` chains back to the real definition first.
2. Check the definition site for I/O markers. If it has none, it's a *candidate* for R9 — but
   candidacy alone doesn't decide the direction.
3. Check every consumer, transitively, for a hard sync boundary: an external-trait impl (E1), a
   fn-pointer slot (E4), or another already-decided-sync fn. **If you find one, R9 applies and the
   helper (if you own it) becomes sync-with-tag.** If you find **none** — every consumer either
   already calls other async fns, or is free to become async — **propagate `async` upward
   instead**, even through functions that are individually "pure" arithmetic. Universal async does
   not stop at a function that merely lacks I/O; it stops at a function a *language rule* forbids
   from being async.
4. If the helper's true definition is outside your owned paths and rule 3 says it should be
   sync, that's a lease-request, not a local fix. If rule 3 says propagate instead, you don't need
   the lease at all — the fix is entirely on your side of the boundary. This packet was the
   latter case throughout.
5. Convert every test that calls a newly-async fn to `#[semio_framework_async_macros::async_test]`
   rather than hand-rolling `block_on` in each one (R4 sanctions `block_on` in `#[cfg(test)]`, but
   the macro is the preferred form per the ticket's own guidance).
6. Re-run `insert-await.py` after the structural pass — it will catch borrow-checker fallout
   (E0502-shaped: a future holding a borrow alive across a statement that also needs a conflicting
   borrow) that a blanket `.await`-insertion can't safely auto-apply. Fix those by hand: resolve
   the future to its value on its own `let` binding before using it in an expression that needs a
   second, conflicting borrow of the same data.

## 3. Cause 2 — corrected: two distinct defects, not one

The brief described `🔌️io`'s 5 errors as `#[async_trait]` (R8) fallout. **Verified this is wrong**:
grepped `🔌️io/🦀️component.rs` for `async_trait` — **zero occurrences**, confirmed by two
independent queries (`grep`, then a python file-content search). The actual cause: `impl
PackSource for FilePackSource` / `impl PackSink for FilePackSink` used plain sync `fn` bodies
implementing traits whose methods are already-`async fn` (see §2) — the exact "method should be
`async` or return a future, but it is synchronous" rustc error, which fires for *any* stale sync
impl of an async trait method, `#[async_trait]` or not. Fixed as part of §2's propagation pass.

**The real R8 sites** (`#[async_trait::async_trait]`) were exactly where R8's own table said:
`🎒️pack/⏳️async` (3: the trait `AsyncPackSource` plus its two test-double impls
`RecordingSource`/`HangingSource`) and `🎒️pack/🌐️http` (5: the trait `RangeTransport`, `impl
AsyncPackSource for InnerSource<T>`, `impl AsyncPackSource for HttpPackSource<T>`, `impl
RangeTransport for UreqRangeTransport`, `impl RangeTransport for FakeTransport`). 3 + 5 = 8,
matching R8's measured total for the `🎒️pack` half.

**Checked for `dyn AsyncPackSource`/`dyn RangeTransport` before removing the macro** — the O1
concern R8 exists to guard against. Two independent searches (a plain grep, then a python
os.walk over every `.rs` file under `🧰️framework`, `🛍️products`, `✏️s`) both found **zero** matches
anywhere in the repo. Both traits mix one already-sync method (`len`, tagged E1/structural — see
`pack-waker`'s prior audit) with one genuinely-`async fn` method; native AFIT already supports
that mix without any macro. So the fix was a pure deletion: removed all 8
`#[async_trait::async_trait]` attribute lines (no signature changes — the methods were already
written as plain `async fn`/sync `fn` inside the attributed blocks), then dropped the `async-trait
= "0.1"` dependency from `pack`'s `Cargo.toml` entirely (it has zero remaining uses in `🎒️pack`,
confirmed by re-grep after the edit).

## 4. R7 — `async_fn_in_trait` allow, added at crate root

`AsyncPackSource::read_at` and `RangeTransport::fetch_range` triggered the standard
`async_fn_in_trait` lint once the macro that was suppressing it (async_trait doesn't trigger this
lint, since it doesn't literally use `async fn` in the trait after macro expansion) was removed.
Added `#![allow(async_fn_in_trait)]` to `📦️glue.rs` (the crate root) with a comment citing R3/R7,
**did not** take rustc's suggested `-> impl Future<...> + Send` fix (R7 explicitly bans this — it
would re-impose `Send` R3 forbids on guest-side futures). Every other warning class in `pack`'s own
output is zero.

## 5. Acceptance — all green, exit codes pasted from unpiped runs

```
$ CARGO_TARGET_DIR=/private/tmp/.../scratchpad/target-pack2 cargo check -p semio-framework-pack --lib
Finished `dev` profile [unoptimized] target(s) in 0.17s
$ echo $?
0
```

```
$ CARGO_TARGET_DIR=/private/tmp/.../scratchpad/target-pack2 cargo check -p semio-framework-pack --all-targets
Finished `dev` profile [unoptimized] target(s) in 19.19s
$ echo $?
0
```

```
$ CARGO_TARGET_DIR=/private/tmp/.../scratchpad/target-pack2 cargo test -p semio-framework-pack
running 44 tests
test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ echo $?
0
```

Rerun three times across this packet (once mid-flight after the first structural pass, once after
the R7 allow, once as a final consolidated pass) — all three converged on the same 0/0/0/44-passed
result, so this isn't a flaky green.

**`pack-waker`'s two poll-counting regression tests are present and passing**, by name, in the
44-test run: `async_::tests::cancel_watch_resolves_from_another_thread_via_waker_without_spinning_or_sleeping`
and `http::tests::sleep_resolves_via_timer_thread_without_busy_polling`. Neither was touched by
this packet beyond the mechanical `#[async_trait]`-attribute removal on `⏳️async`/`🌐️http` (their
bodies, the `CancellationToken`/`Sleep` waker machinery, are untouched) — `pack-waker`'s work is
intact.

**Warnings**: `--lib`/`--all-targets` both show 60 warning lines, **all 60 attributable to
`semio-framework-replication`** (39 `async_fn_in_trait` + 20 `unexpected cfg: typegen` + 1 rollup
line) — zero from any `🎒️pack` file, confirmed by grepping the output for `🎒️pack` outside the
`Checking semio-framework-pack` build-order line.

## 6. The headline — `semio-framework-plugin --lib`

```
$ CARGO_TARGET_DIR=/private/tmp/.../scratchpad/target-pack2 cargo check -p semio-framework-plugin --lib
...
error: could not compile `semio-framework-os-kernel` (lib) due to 1072 previous errors; 9 warnings emitted
$ echo $?
101
```

**This is not a `semio-framework-plugin` number, and it is not a `pack` number.** The build never
reaches type-checking `semio-framework-plugin`'s own source at all — it fails compiling
`semio-framework-os-kernel`, a **direct dependency** of `semio-framework-plugin` (see
`🔌️plugin/📦️packages/🦀️rust/Cargo.toml` line 31:
`semio-framework-os-kernel = { path = "../../../../📦️packages/🦀️rust", ... }`), before cargo ever
starts on `semio-framework-plugin`'s crate. Confirmed by grepping the build-order log for
`Checking semio-framework-plugin` — **it never appears**.

Characterized the 1072 errors by location and code (parsed diagnostic blocks, not raw grep):
error codes are E0277×364, E0599×320, E0308×289, E0038×37, E0605×15, E0369×12, E0053×4, E0609×3,
E0733×2, E0600×2, E0507×1 — object-safety violations (E0038), missing-method (E0599, the same
"forgot to `.await`" shape as this packet's own §2/§3 fixes, at much larger scale), type
mismatches (E0308), non-primitive `as` casts of a `Future` (E0605), and recursive-async-fn-needs-
boxing (E0733). **This looks like the same universal-async-codemod fallout this whole ticket
exists to fix, landed on a different, much larger crate** (`os-kernel`), not caused by anything I
touched. `os-kernel` is not in this packet's `path_scope`, so I did not attempt a fix — flagging
per the "cross-packet findings must be lifted the moment they're read" rule (W4 item 8).

**One single genuine downstream break traced to my own change, and it is out of scope, not a
regression to undo**: `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs:49` (note: this
is a *different* crate/directory tree from the one I own —
`🛍️products/💻️os/🔨️modules/🎒️pack/`, not `🧰️framework/🔨️modules/🎒️pack/`) calls
`read_footer_only(&bytes).map(|footer| footer.content_hash)` without `.await`. `read_footer_only`
became `async fn` in this packet (§2, an unavoidable consequence of `PackSource` already being
async upstream — there was no sync-preserving alternative available inside my owned files). Since
`semio-framework-pack` itself was already red (67 errors, unbuildable) before this packet started,
this downstream call site was already broken by construction — it could not have compiled against
a red dependency either way. Not fixed (outside `🎒️pack/**`), but named here so whoever owns
`🛍️products/💻️os/🔨️modules/🎒️pack/` knows the one-line fix is adding `.await`.

**SDK reverted-state claim, re-checked and found stale**: the brief stated the SDK file sits at 19
`async fn` (pre-conversion/reverted state) vs 1,489 in the index — this packet's live-tree-hazard
instructions required re-verifying this rather than repeating it. Current measurement (python,
counting `"async fn"` occurrences per file, disk vs `git show :<path>`, across every `.rs` file
under `🔌️plugin/`): **disk and index are now consistent** — `🦀️component.rs` alone shows 1489/1489,
and every other file in the tree matches within a handful (two generated files differ, 4 vs 0 and
1 vs 0, both `🤖️generated/**`/`📦️glue.rs` additions present on disk but not yet indexed — additions,
not reversions). **No revert is present at the time of this measurement.** Either another session
restored it since the brief was written, or the brief's snapshot was itself stale — either way,
report what's on disk now, not what was claimed earlier, per this ticket's own rule.

**Net conclusion for the SDK packet**: `semio-framework-plugin`'s own error count is currently
**unmeasurable** — not zero, not some other number — because `semio-framework-os-kernel` must go
green first. That crate is a new, large (1072-error) blocker, structurally identical in kind to
what `pack-waker`/`pack-finish` just finished fixing in `pack`, but far bigger and entirely
outside this packet's ownership.

## 7. Tools used

- `insert-await.py --crate semio-framework-pack --scope '🎒️pack' --max-files 60`: run twice.
  First pass: 0 await-edits (all missing-awaits were structural, needing function-signature
  changes the tool correctly declines to make on its own), 1 "other" (the E0502 borrow issue,
  §2). Second pass, after the hand-fix: `errors=0 await-edits=0 ambiguous=0 other=0`, fixpoint.
- `deasyncify-external-impls.py`: not run — no E1-impl-of-external-trait fallout was found in the
  scoped files beyond what `pack-waker` had already tagged.
- `async-test-attr.py`: not used directly — the tests needing conversion here were plain `#[test]
  fn` (not yet `#[test] async fn`), a shape the tool's docstring says it doesn't handle (it
  rewrites an already-`async fn` test's attribute), so these were hand-converted (adding both
  `async` and the attribute together) rather than run through the tool.

## Summary of files changed

- `🎒️pack/📐️format/🦀️component.rs` — 26 fns converted `fn` → `async fn` (propagating from
  replication's already-async `crc32c`/varint/`PackSource`/`PackSink`/`CompressionCodec`
  primitives per R9 rule 3), 3 fns tagged and kept sync (`resolve_manifest`, `checks_crc`,
  `checks_content_hash`), 3 CRC-future-borrow-conflict sites fixed (assign-then-use instead of
  inline `.await` inside a call that also needs a second borrow), 24 tests converted to
  `#[semio_framework_async_macros::async_test]`.
- `🎒️pack/🔌️io/🦀️component.rs` — `PackSource`/`PackSink` impls for `FilePackSource`/`FilePackSink`
  converted to `async fn` (stale, not `#[async_trait]` — see §3), `StreamingPackWriter`'s four
  methods and `recover_file` converted to `async fn`, 8 tests converted to
  `#[semio_framework_async_macros::async_test]` (2 `write_atomic_*` tests left as plain `#[test]`,
  untouched by the async surface).
- `🎒️pack/⏳️async/🦀️component.rs` — removed 3 `#[async_trait::async_trait]` attributes (R8); no
  signature/body changes.
- `🎒️pack/🌐️http/🦀️component.rs` — removed 5 `#[async_trait::async_trait]` attributes (R8); no
  signature/body changes.
- `🎒️pack/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-async-macros` dev-dependency;
  removed the `async-trait` runtime dependency (R8, zero remaining uses).
- `🎒️pack/📦️packages/🦀️rust/📦️glue.rs` — added `#![allow(async_fn_in_trait)]` at crate root (R7).

**Acceptance, all pasted with exit codes above**: `cargo check -p semio-framework-pack --lib` →
0. `cargo check -p semio-framework-pack --all-targets` → 0. `cargo test -p semio-framework-pack` →
0, 44 passed / 0 failed, including both `pack-waker` regression tests by name. `cargo check -p
semio-framework-plugin --lib` → 101, blocked entirely by `semio-framework-os-kernel` (1072
errors, not `pack`, not `plugin`'s own source) — flagged, not fixed, out of scope.

No `[DEBUG]` temp logs were left in any source file. Scratch files (`.txt`/`.json`) are in the
ticket folder or the session scratchpad target dir, never `.log`.
