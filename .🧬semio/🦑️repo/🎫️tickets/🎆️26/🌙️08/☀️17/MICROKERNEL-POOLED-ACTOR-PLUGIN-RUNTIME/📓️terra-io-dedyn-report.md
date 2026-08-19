# 📓️ terra-io-dedyn-report — packet `io-dedyn`

Scope: `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` only. Applied R11 (generics + associated types,
never an enum, never a box) to the four open host-extension-point traits `PayloadSource`,
`PayloadSink`, `RandomAccessPayload`, `ResourceResolver`, exactly as ruled. Along the way I found
and fixed a large amount of pre-existing, untagged missing-`.await` residue in the same file that
was blocking compilation regardless of the dyn work (see "Residue fixed" below) — this file was
staged in a mid-asyncify, non-compiling state before this packet started.

## What changed, mapped to R11's own line references

**(a) Trivially generic parameters/refs** — `:387, :435, :479, :504, :545, :624, :2156` (R11's line
numbers, pre-shift) — converted to generic type parameters:
- `DecodeContext::source<'source, S: PayloadSource>(&mut self, source: &mut S)`
- `EncodeContext::sink<'sink, S: PayloadSink>(&mut self, sink: &mut S)`
- `BoundedPayloadSource<'a, S: PayloadSource>`, `ResolvedPayloadSource<'a, S: PayloadSource>`
- `BoundedPayloadSink<'a, S: PayloadSink>`, `ResolvedPayloadSink<'a, S: PayloadSink>`
- `BoundedRandomAccessPayload<'a, A: RandomAccessPayload>`

**(b) The real one — `resolve_decode`/`resolve_encode` returning a runtime-chosen implementation**
(`:498, :499, :2172, :2176`) and their holders (`:370, :418`):
```rust
pub trait ResourceResolver: Send + Sync {
    type Source: PayloadSource;
    type Sink: PayloadSink;
    async fn resolve_decode(&self, request: &ResourceRequest) -> CodecResult<Self::Source>;
    async fn resolve_encode(&self, request: &ResourceRequest) -> CodecResult<Self::Sink>;
}
```
`DecodeContext<R: ResourceResolver>` / `EncodeContext<R: ResourceResolver>` replace
`Arc<dyn ResourceResolver>` with `Option<Arc<R>>`. `PayloadSource::random_access`'s return position
(`:479, :2156`) needed the associated type at the *owning* trait, exactly as R11 anticipated:
```rust
pub trait PayloadSource: Send {
    type RandomAccess: RandomAccessPayload;
    async fn random_access(&self) -> Option<&Self::RandomAccess> { None }
}
```

**Design choice that kept threading minimal**: `BoundedPayloadSource`/`BoundedRandomAccessPayload`/
`BoundedPayloadSink` used to hold `context: &'a mut DecodeContext` (the whole context, purely to
reach `budget`/`policy`). Since they never touch `context.resolver`, I split that back-reference
into `budget: &'a mut CodecBudget, policy: &'a DecodePolicy` (or just `budget` for the sink side,
which never reads `policy`). This means these wrapper structs stay **non-generic over `R`** even
though `DecodeContext<R>`/`EncodeContext<R>` are — only `ResolvedPayloadSource<'a, S>` and
`ResolvedPayloadSink<'a, S>` need a type parameter for the resolver's own `Self::Source`/`Self::Sink`
associated type, and that parameter is `S: PayloadSource`/`S: PayloadSink`, not `R` itself.

`PayloadCodec` gained `type Source: PayloadSource; type Sink: PayloadSink;` (needed once
`BoundedPayloadSource`/`BoundedPayloadSink` carry a type parameter). `ArtifactCodec::decode_artifact`/
`encode_artifact` became generic **methods** (`<R: ResourceResolver>`) rather than making the whole
trait generic — a codec body doesn't care which resolver its caller supplies.

## STOP-condition arithmetic (R11: stop and report if `R` threads through >~10 public types)

`R: ResourceResolver` (the resolver's own type parameter, the thing R11's cost warning is about)
appears on exactly **3** public items:
1. `DecodeContext<R>`
2. `EncodeContext<R>`
3. `ArtifactCodec::decode_artifact<R>` / `encode_artifact<R>` (one trait, two generic methods)

Well under the ~10 threshold — **R11 generalizes cleanly here, no revisit needed.** (The separate,
trivially-generic `S: PayloadSource`/`S: PayloadSink` parameter from part (a) touches more items —
`BoundedPayloadSource`, `BoundedRandomAccessPayload`, `ResolvedPayloadSource`, `BoundedPayloadSink`,
`ResolvedPayloadSink`, `PayloadCodec` — but that's the "no design question, just do it" half of the
ruling, not the part the STOP condition is about.)

## Verification — zero `dyn <first-party trait>`, two independently-implemented methods

```
$ grep -n "dyn " 🧰️framework/🔨️modules/🚪️io/🦀️component.rs | grep -v '^\s*[0-9]*:\s*//'
814:pub type ComposeFuture<'a> = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ComposedArtifact, ComposeError>> + Send + 'a>>;
1127:pub type IoFallback = dyn for<'a> Fn(&'a IoKey, &'a [ErasedComposeSource]) -> Option<ComposeFuture<'a>> + Send + Sync;
```
Both are `dyn Future`/`dyn Fn` — explicitly R1-legal (fn-pointer/erasure-table plumbing predating
this packet, `io-thunks`' work; not touched).

Second method — Python, independently extracting every first-party `trait` name declared in the
file itself and regexing for `dyn <name>\b` with a comment flag:
```
first-party traits declared here: ['ArtifactCodec', 'Deserializer', 'PayloadCodec', 'PayloadSink',
  'PayloadSource', 'RandomAccessPayload', 'ResourceResolver', 'Serializer', 'SubsetValidator', ...]
non-comment hits: []
```
Zero. The only two matches are inside my own new doc comments quoting the *old* `Arc<dyn
ResourceResolver>` shape for context.

## Verification — zero E0038

`cargo check -p semio-framework-os-kernel --lib` (this crate independently `#[path]`-mounts the
same file as `os_io`, so it's a second, real compilation of this exact source):
```
$ CARGO_TARGET_DIR=.../scratchpad/target-iodedyn cargo check -p semio-framework-os-kernel --lib
warning: `semio-framework-os-kernel` (lib) generated 16 warnings
error: could not compile `semio-framework-os-kernel` (lib) due to 785 previous errors; 16 warnings emitted
EXIT:101
```
```
$ grep -c '^error\[E0038\]' check2-oskernel.txt
0
$ grep -oE '🔨️modules/🚪️io/🦀️component\.rs:[0-9]+:[0-9]+' check2-oskernel.txt | sort -u
(no output — zero error locations anywhere in io/component.rs)
```
Baseline (before this packet, same command): **37/37 E0038**, all with primary span inside
`io/component.rs` (confirmed by pasting the diagnostic context, not just counting). After: **0**,
and io/component.rs contributes **zero** of the remaining 785 os-kernel errors under `--lib`.

`--all-targets` (compiles this file's own `#[cfg(test)] mod tests` too):
```
$ CARGO_TARGET_DIR=.../scratchpad/target-iodedyn cargo check -p semio-framework-os-kernel --all-targets
error: could not compile `semio-framework-os-kernel` (lib test) due to 5652 previous errors; 23 warnings emitted
EXIT:101
$ grep -c '^error\[E0038\]' check5-oskernel-alltargets.txt
0
```
io/component.rs still contributes error locations under `--all-targets`, but **only two categories,
both pre-existing and out of scope** (see below) — zero E0038, zero dyn, zero missing-`.await`,
zero anything else attributable to this packet's file.

## Residue fixed (not dyn, but blocking compilation of my own owned file)

The file was staged mid-asyncify with pervasive missing `.await` and a few sync-closure/`.await`
mismatches. All within `🚪️io/**`, so all fair game and all fixed:
- ~20 missing-`.await` call sites across `AnchoredSyntax::validate`, `ArtifactCodecResult::*`,
  `CodecBudget::*`, the rewritten `DecodeContext`/`EncodeContext`, `BoundedPayloadSource`/
  `BoundedRandomAccessPayload`/`Resolved*` impls, and ~15 test-module call sites (`TestPayload`,
  `TestResolver`, `codec_budget_enforces_limits_and_shared_cancellation`,
  `codec_context_bounds_streaming_random_access_recursion_and_resolved_resources`,
  `resolved_resources_cannot_outlive_their_cancellation_budget`,
  `codec_result_requires_valid_owned_spans_and_deterministic_opaque_order`,
  `wire_rejects_oversized_and_unbounded_dialect_inputs_before_interning`,
  `artifact_ref_uri_round_trips`, `artifact_ref_to_uri_matches_expected_shape`,
  `artifact_ref_parse_uri_rejects_malformed_input`).
- **R9 application**: `CancellationToken::new()` de-asyncified and tagged
  `// 🚫️async: E1 pure accessor consumed by external-trait impls (Default::default on
  DecodePolicy/EncodePolicy) — see R9`. Pure (no I/O), and both its problem consumers are
  `impl Default for {Decode,Encode}Policy::default()` — E1, signature fixed by the `Default`
  trait, cannot be async. No orphaned call sites needed fixing since nothing had awaited it yet.
- **7 sync-closure `.await` bugs** (R10 residue shape 1) around `CodecFailure::error(...)` used
  inside `.map_err(|e| ...)`/`.ok_or_else(|| ...)` closures — rewritten as `match`/`if let Err`
  blocks in the enclosing (already-async) function bodies rather than de-asyncifying
  `CodecFailure::error` itself, per R9 step 3 ("if every consumer *can* become async, make the
  consumer async instead" — none of these closures were actually language-barred, they were just
  written as closures for brevity).
- **3 sync-closure Ord/iterator bugs** in `io_mechanism::route_rank`/`resolve_route`/
  `resolve_identify` (`.map(|e| e.fidelity.rank())` / `.sort_by(...)` calling now-async `.rank()`
  inside a comparator) — fixed by wrapping with `super::resolve_ready(...)`, the file's own
  established E5-bridge idiom for exactly this shape (same pattern already used one token over for
  `to_coordinate()` in the very same lines by the prior `io-thunks` packet).
- One `E0507` move-out-of-shared-reference (`dialect: *dialect` on a non-`Copy` `ArtifactDialect`
  while iterating `&proposed`) → `.clone()`.
- Two sync fn-pointer-slot (`E4`, already tagged) bodies calling now-async `encode_pack()`/
  `print_dsl()` without a bridge → wrapped in `super::resolve_ready(...)`.
- Test-module: `TestPayload`/`TestResolver` updated to the new trait shapes
  (`type RandomAccess = TestPayload;`, `type Source = TestPayload; type Sink = TestSink;`,
  `resolve_decode`/`resolve_encode` return owned values, not boxes). Two `DecodeContext`/
  `EncodeContext::new()` call sites with no resolver argument needed `::<TestResolver>` turbofish
  since `R` can no longer be inferred from an absent argument.
- One new-code slip caught by my own second check-and-fix cycle: `super::resolve_ready` used
  inside a doubly-nested test module (`io_mechanism::tests::laws`) needed `super::super::` to reach
  `io_mechanism`'s parent — fixed and re-verified.

## Known pre-existing, out-of-scope blockers found in this file (NOT fixed, NOT mine)

Both confirmed pre-existing (untouched by any of my edits) and confirmed **not** io-specific by
their occurrence counts, so left alone per this packet's scope and per rule "do not chase [a
sibling's] errors, do not edit that crate":

1. **`error: async functions cannot be used for tests`** on all 24 `#[test] async fn` in this file
   under `--all-targets`. Repo-wide count in the same run: **769** — this is a whole-codebase
   `#[test]` → `#[semio_framework_async_macros::async_test]` migration gap, not an io-specific one.
   The ticket folder's own `async-test-attr.py` explicitly scopes its `--apply` to `⏳️async/**`
   only and states in its own docstring *"a repo-wide `--apply` is explicitly a LATER packet's job,
   not this script's to perform unattended"* — so I did not run it here either, even scoped to just
   this file. **This means `cargo test` cannot currently run this file's test module at all** —
   flagging as a cross-packet finding (W4 rule 8) since it blocks proving these tests pass, not
   just that they compile.
2. **`error[E0425]: cannot find type 'ErasedComposeSource'/'ComposeFuture' in module '$crate'`** (4
   locations, at the `compose_thunk!` invocations in this file's own test fixtures, lines ~2066-2072
   — code I never touched). Root cause: `semio-framework-os-kernel` mounts this file a *second*
   time as `pub mod os_io` (not re-exported at its crate root), while `compose_thunk!`'s body uses
   `$crate::ErasedComposeSource`/`$crate::ComposeFuture`, which only resolves when the invoking
   crate re-exports those types at its root (as `semio-framework` itself does, at
   `📦️glue.rs:91-103`). `os-kernel`'s own `📦️glue.rs:198-206` **explicitly documents this exact gap**
   as "recorded debt D2 ... cleaned up wholesale at W6 alongside the old registry itself" — already
   known, already scheduled, and requires editing `🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`,
   which is explicitly listed as NOT MINE in this packet's brief.

## Primary acceptance target — structurally unreachable right now, not because of this packet

`cargo check -p semio-framework --lib` / `--all-targets`:
```
error: could not compile `semio-framework-os-kernel` (lib) due to 647 previous errors; 16 warnings emitted
EXIT:101
```
`grep -c '^error\[E0038\]' → 0` in both runs, but **cargo never reaches checking `semio-framework`
itself** — no `Checking semio-framework` / `Compiling semio-framework` line appears in either log.
Cause: `🧰️framework/📦️packages/🦀️rust/Cargo.toml` has a hard `[dependencies]` edge on
`semio-framework-os-kernel` (comment: *"OS kernel owns spr/dsl types — aliases avoid dual trees /
cycle with old implementations"*), and Cargo must build a dependency before its dependent, so a
broken `os-kernel` makes `-p semio-framework` fail **before semio-framework's own source is even
parsed**, regardless of anything in `🚪️io/**`. `os-kernel` currently carries **647–961 errors owned
by a sibling packet** across these runs (count moves as other sessions in this live tree edit it
concurrently) — confirmed via `cargo check -p semio-framework-os-kernel --lib` directly, which
independently compiles the exact same `io/component.rs` source with **zero** errors attributable to
it, as shown above. This is a real, structural, pre-existing finding, not a gap in this packet's
work — surfacing it here per "cross-packet findings must be lifted the moment they are read."

## Owned paths touched

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (only file edited)
- Ticket folder: this report (`📓️terra-io-dedyn-report.md`) plus full command-output transcripts —
  `terra-iodedyn-baseline-37-e0038.txt` (pre-fix baseline, 37 E0038), `terra-iodedyn-oskernel-lib-
  zero-e0038.txt` (post-fix `--lib`, 0 E0038), `terra-iodedyn-oskernel-alltargets-final.txt`
  (post-fix `--all-targets`, 0 E0038, only the two documented out-of-scope categories remain),
  `terra-iodedyn-framework-lib-blocked.txt` / `terra-iodedyn-framework-alltargets-blocked.txt`
  (the `-p semio-framework` runs, showing they never reach compiling `semio-framework` itself).

## Bottom line

- Zero `E0038`, zero `dyn <first-party trait>` in `🚪️io/**` — verified two ways, pasted above.
- R11's cost warning (type parameter threading) settled at **3** public types for the resolver's
  own `R` — nowhere near the ~10 stop threshold. **R11 generalizes safely to the remaining open
  families; no revisit needed on this evidence.**
- `semio-framework`'s own `--lib`/`--all-targets` remains red, but structurally unreachable for
  reasons entirely outside `🚪️io/**` (a hard dependency on a sibling-owned, mid-repair
  `semio-framework-os-kernel`) — not a residual defect in this packet's file.
- Two pre-existing, documented, out-of-scope gaps found and left alone (see above), flagged as
  cross-packet findings.
