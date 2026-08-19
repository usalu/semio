# 📓️ terra-math-dedyn report

Packet: `math-dedyn`. Scope: `🧰️framework/🔨️modules/🧮️math/**` (owned). Goal: zero first-party
`dyn Trait` in `🧮️math` (O1), using `#[dyn_enum]`/`dyn_enum_close!` where it fits and a sanctioned
alternative (concrete type / generics) where it doesn't.

## 1. Result

- **Zero live first-party `dyn` remains** in `🧮️math/**`. Proof (python3 over the real file,
  filtering out comment lines):
  ```
  $ python3 -c "... regex \bdyn\b over non-comment lines ..."
  7607             let _: &dyn std::error::Error = error;
  TOTAL non-comment dyn occurrences: 1
  ```
  That one hit is `dyn std::error::Error` — R1-permitted (std/lang `dyn Error`).
- **9 trait families measured myself** (python3 over the file, not shell grep, per the ticket's own
  "a negative result from a too-narrow query" warning): `RandomSource` 18 dyn-uses / 8 methods / 2
  impls, `TokenTextAdapter` 9/3/2 (1 after cleanup, see §3), `LogitsProcessor` 38/8/34,
  `TokenSampler` 7/3/4, `Constraint` 10/10/6, `StopCondition` 5/6/2, `SamplingObserver` 4/5/2,
  `Collective` 3/4/1, `Denoiser` 2/2/0-in-crate. **96 total dyn-uses converted.** The brief's
  `sol-dyn-families-postrevert.json` had `Constraint: uses 20` — stale; the real, hand-verified
  count is 10 (see §3.1 for why the census was wrong).
- **7 families → `#[dyn_enum]` + `dyn_enum_close!`**: `RandomSources`, `TokenTextAdapters`,
  `LogitsProcessors`, `TokenSamplers`, `Constraints`, `StopConditions`, `SamplingObservers`.
- **1 family → concrete type, no abstraction**: `Collective` (single impl `LocalCollective`), per
  the brief's explicit "exactly one impl" rule.
- **1 family → static generics (`impl Denoiser` / `AsyncFn`)**: `Denoiser` (zero in-crate
  production impls by design — "caller-supplied model evaluation"; see §3.3).
- `#![allow(async_fn_in_trait)]` added at the crate root (`📦️glue.rs`) with an R3/R7 comment.
- `Cargo.toml` (mine — not registrar-only) gained `semio-framework-dispatch-macros` as a real
  dependency and `semio-framework-async-macros` as a dev-dependency (for `#[test] async fn`, via
  the ticket's `async-test-attr.py`, 191 sites).

## 2. Acceptance — real command, real output, real exit code

**The real `cargo check -p semio-framework-math` cannot run** — `semio-framework-geometry` (a
dependency, **not** in my owned path) has 146 pre-existing errors from a bulk async-ification pass
that never got `.await` insertion (staged, not `HEAD`; unrelated to this packet). Re-verified at
the very end of this session, unchanged:
```
$ CARGO_TARGET_DIR=<scratchpad>/target-math-real cargo check -p semio-framework-math --lib --message-format=short
🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/../../⚙️engine/🦀️component.rs:747:51: error[E0277]: ...
... (146 lines, all in 📐️geometry, none in 🧮️math)
error: could not compile `semio-framework-geometry` (lib) due to 146 previous errors
```
This is the same shape as `pack-waker`'s "blocked upstream, verified not its own": I did not touch
`📐️geometry`, and I re-measured immediately before writing this report so the claim reflects the
current tree, not an earlier read.

**Real verification instead**: a scratch mirror (`<scratchpad>/mathdedyn-verify/`) that mounts the
REAL, currently-edited `🧮️math/🎯️sampling/🦀️component.rs` via `#[path]` and stands in a hand-written,
`.await`-complete replica of `geometry::random`'s public surface (the only part of `📐️geometry` this
crate touches) for the broken dependency. Full method: §4.

```
$ CARGO_TARGET_DIR=<scratchpad>/target-mathmirror cargo check -p mathmirror --lib
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```
Exit `0`.
```
$ CARGO_TARGET_DIR=<scratchpad>/target-mathmirror cargo check -p mathmirror --all-targets
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
```
Exit `0`. Zero warnings from the mirrored math source (the 2 warnings present are pre-existing,
from `semio-framework-dispatch-macros` itself — `🔀️dispatch` is not my path).
```
$ CARGO_TARGET_DIR=<scratchpad>/target-mathmirror cargo test -p mathmirror --lib
test result: ok. 191 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```
Exit `0`. **Named set**: all 191 tests in `sampling::tests` pass, including
`xoshiro_source_matches_underlying_rng_sequence` (numerically compares `XoshiroSource` against
`geometry::random::Rng` — meaningful because the stand-in's xoshiro256** body is copied verbatim
from the real `📐️geometry` file, only `.await` added, so it's bit-identical once `📐️geometry` itself
compiles).

**What this does and doesn't prove**: it proves every line I touched in `🧮️math` is real,
rustc-checked, dyn-free, and passes its own test suite. It does **not** prove the real workspace
graph builds, because that also needs `📐️geometry` fixed, which is out of my scope. Re-run
`cargo check -p semio-framework-math --all-targets` and `cargo test -p semio-framework-math` for
real once `📐️geometry` is green — I expect both to pass unchanged, since the mirror is the same
source file, but I have not been able to observe that directly.

## 3. The judgement calls

### 3.1 `Constraint` — census said "impls maybe outside the crate"; verified false
The brief flagged `sol-dyn-families-postrevert.json`'s `Constraint: uses 20` as possibly meaning
impls live in `✏️s/🔌️plugins/🌀️procedural`. I grepped the whole repo for `impl Constraint for` and
found 4 hits there — but read them: they're `crate::wfc_engine::constraint::Constraint`, a
WFC-grid-solver trait with the same short name, completely unrelated to
`semio_framework_math::sampling::Constraint`. Zero real cross-crate impls. Also re-confirmed by
grepping for `sampling::Constraint` / `math::Constraint` anywhere outside this file: nothing. All
6 real impls (`RegexConstraint`, `TrieConstraint`, `MustIncludeConstraint`, `JsonModeConstraint`,
`EbnfConstraint`, `JsonSchemaConstraint`) are in this file. Same check run for all 9 families —
**zero cross-crate impls or dyn-uses for any of them**, so every enum could close right here. The
census's `uses: 20` was simply stale (my own regex count is 10, matching the 10 real call sites).

### 3.2 `TokenTextAdapter` — macro can't take a lifetime, so `SliceTextAdapter` lost its borrow
`SliceTextAdapter<'a> { tokens: &'a [&'a [u8]] }` carried a lifetime. `dyn_enum_close!`'s DSL parser
(`DynEnumInput::parse` in `🔀️dispatch/🦀️component.rs`) reads a bare `Ident` for the enum name then
immediately expects `:` — **no generic/lifetime parameter slot at all**. An enum
`TokenTextAdapters<'a>` cannot be written through this macro today. Fix: made `SliceTextAdapter`
own its data (`Vec<Vec<u8>>` instead of `&'a [&'a [u8]]`) — it's test-only in this crate (never
constructed outside `#[cfg(test)]`), so the one-time copy costs nothing real. That made the
lifetime problem vanish entirely, rather than routing around the macro.

The crate also had a `MockAdapter` (test-only, `table: Vec<Vec<u8>>`) implementing the same trait —
identical shape to the now-owned `SliceTextAdapter`, existing only because the old borrowing
`SliceTextAdapter` couldn't serve one particular test. `dyn_enum_close!`'s variant list has **no
per-variant `#[cfg(...)]` support either** (`DynEnumVariant::parse` never calls
`Attribute::parse_outer`), so a test-only second implementor can't cleanly join a production enum
even after the lifetime is gone. Since `MockAdapter` was now redundant, I deleted it and pointed its
one call site at `SliceTextAdapter` instead — not a workaround, a genuine simplification once the
representations matched. `TokenTextAdapters` closes with one real variant: `Slice(SliceTextAdapter)`.

### 3.3 `Denoiser` — zero in-crate production impls, some test impls are function-local
`Denoiser`'s own doc comment: "Caller-supplied model evaluation... this trait is the seam" — by
design, no real implementation exists in this math crate; only test mocks do, and two of those
three (`BranchTrackingDenoiser`, `CountingDenoiser`) are defined **inside individual `#[test] fn`
bodies**, not at module scope — genuinely unnameable from a module-level `dyn_enum_close!` call, not
just inconvenient. Even if I hoisted them, forcing an enum here is the wrong model: the trait exists
so external callers (a real model backend, outside this crate) can supply their own type, and a
closed enum would force every future caller to register their type in an enum only this crate can
edit — backwards for a "caller supplies the implementation" seam. Fixed by making the two consumers
(`eval_denoised`, `run_diffusion`) generic (`denoiser: &mut impl Denoiser`) instead — 2 call sites,
fully contained, and it's honestly the more correct model for this trait's role.

### 3.4 `Collective` — exactly one impl
`LocalCollective` is the only implementor, used only as short-lived `&mut dyn Collective` borrows in
3 free functions, never stored. Per the brief's explicit rule for this case: deleted the trait
object, changed the 3 call sites to the concrete `&mut LocalCollective` type. The trait declaration
itself is untouched (still there as a real interface contract for a future distributed backend);
only the dyn-dispatch is gone.

## 4. The `📐️geometry` blocker and how I verified around it

`semio-framework-math` depends on `semio-framework-geometry` (`geometry::random::{Rng, SplitMix64}`
only — verified by grep). `📐️geometry` has two files (`⚙️engine`, `🎲️random`) staged with `async fn`
added but zero `.await` insertion — same defect class described in `📌️important.md`'s INCIDENT
section, but this is a **different, still-open** instance of it (not the one that was resolved) and
it is **not** in `🧰️framework/🔨️modules/🧮️math/**`, so I did not touch it.

I built `<scratchpad>/mathdedyn-verify/`:
- `geomstub/` — a from-scratch crate reproducing exactly `geometry::random`'s public surface that
  `🧮️math` calls (`SplitMix64::{new,next_u64}`, `Rng::{from_seed,next_u64,state,from_state}`), body
  copied verbatim from the real `📐️geometry` source with `.await` inserted at each call — i.e. the
  same mechanical fix `insert-await.py` would apply to the real file, just not applied there since
  `📐️geometry` isn't mine.
- `mathmirror/` — `Cargo.toml` depends on `geomstub` (path) and the **real**
  `semio-framework-dispatch-macros` (path to the actual repo crate, unmodified) and (dev)
  `semio-framework-async-macros`; `src/lib.rs` does `extern crate geomstub as geometry;` +
  `#[path = "<real repo path>/🧮️math/🎯️sampling/🦀️component.rs"] pub mod sampling;`. A
  `rust-toolchain.toml` pins the same `nightly-2026-07-07` the repo uses (needed — the workspace
  root's `[lints] workspace = true` / `trim-paths` profile setting requires nightly, and a cwd
  outside the repo tree loses the toolchain override otherwise).

This is the same technique `dyn-enum-macro` used for its own acceptance (a scratch build proving the
real macro against real code) — I'm noting it explicitly here because it generalizes: **any packet
blocked by a live sibling's broken dependency can mount its own real source via `#[path]` against a
hand-written stand-in for just the broken dependency's call surface**, without ever touching the
sibling's files or fabricating results.

## 5. Applying `dyn_enum_close!` — what the recipe gets wrong

Six real findings from actually applying it at scale (96 dyn-uses, 9 families), for the ~90
applications still to come.

**Finding A — the DSL has no generic/lifetime parameter slot for the enum.**
`DynEnumInput::parse` (`🔀️dispatch/🦀️component.rs`) reads `enum <bare Ident>` then `:` — it cannot
parse `enum Foos<'a>: Foo { ... }`. Any family where an impl type carries a lifetime (or any
generic parameter) cannot use `dyn_enum_close!` as written. There is no workaround from the calling
side; either make the carrying type own its data instead of borrowing (what I did — check whether
the type is genuinely reference-only for a good reason first), or fall back to the generic-consumer
route.

**Finding B — no per-variant `#[cfg(...)]` in the closing DSL.**
`DynEnumVariant::parse` never calls `Attribute::parse_outer`, so `Foos: Foo { #[cfg(test)]
Mock(MockAdapter) }` isn't accepted. A test-only implementor cannot join a production enum
conditionally. In my case the test type was genuinely redundant with a production type once its
own blocker (Finding A) was fixed, so deletion was the right call — but if a family has a real
test-only variant with no production equivalent, this needs a different pattern (a
`#[cfg(test)]`-gated second, test-only enum, or a generic test helper), not fighting the macro.

**Finding C — `fork()`-shaped factories need the SAME conversion at every construction site, and
missing one produces a misleading error.** Every `fn fork(&self) -> Box<dyn Self>` needs its
signature changed to `-> Selfs` and its body from `Box::new(EXPR)` to `EXPR.await.into()` — but if
`EXPR` is itself an `async fn` call (very common: `RepetitionPenalty::new(..)` etc.), a
**missing `.await` before `.into()`** produces `error[E0277]: the trait bound
'LogitsProcessors: From<impl Future<Output = RepetitionPenalty>>' is not satisfied` — which reads
like a missing enum variant, not a missing await, and sent me looking in the wrong place the first
time. Rule of thumb: any `X::new(..).into()` where `X::new` is async needs `.await` **before**
`.into()`, always — the message will never say "await" if you get this wrong.

**Finding D — every sync combinator downstream of a converted method breaks, and this is the
biggest source of post-macro work, not the macro itself.** Once a trait method is enum-dispatched
`async fn`, every `bool::then(|| ..)`, `Option::or_else(|| ..)`, `Vec::sort_by(|a,b| ..)`,
`.iter().map(|x| x.method())`, `Vec::retain(|x| ..)` that calls it from inside a **sync** closure
now fails with `E0728` ("await only allowed inside async functions"). None of this is
`dyn_enum`-specific — it is the mechanical consequence of O1 anywhere a value that used to be
looked up/scored/filtered synchronously is now behind an async call — but it dominates the actual
edit count (in this crate, roughly 40 sites, more than the dyn removal itself). The fix is always
one of: (a) precompute a plain sync lookup (`HashMap`/`HashSet` snapshot) before the closure, (b) an
explicit `for` loop building a `Vec` instead of `.map(..).collect()`, (c) decorate-sort-undecorate
for `sort_by`, or (d) a real `async |x| ..` closure (stable, works fine with `AsyncFn`/`AsyncFnMut`
bounds — used this for `BestOfN::run`'s `make_initial` parameter and several test helpers). Never
try to make the combinator itself async — none of the std ones support it.

**Finding E — self/mutually-recursive `async fn`s need `Box::pin(..)` at (in general) every direct
recursive call site, not just one edge of the cycle.** This crate's hand-rolled JSON parser, regex
compiler, EBNF grammar compiler, and NFA builder are all recursive-descent and all hit `E0733`
("recursion in an async fn requires boxing") the moment their bodies became async. `Box::pin(f())`
around the recursive call is the fix (`Pin<Box<impl Future>>`, never `dyn Future` — R1-legal, this
is opaque-type boxing for a finite-size requirement, not trait-object erasure). For a mutual cycle
(A calls B calls A), boxing one edge is enough; for true self-recursion with several call sites in
one function, every one of them needs it independently, since the compiler's requirement is about
the function's own future type being finite regardless of which branch runs.

**Finding F — the biggest hazard is fully avoidable and not the macro's fault at all: do not
bulk-automate `.await` insertion by matching bare names against a "this name is declared `async
fn` somewhere" list.** I built exactly this heuristic mid-session (extract every `async fn NAME`,
regex-match every `NAME(` / `.NAME(` call site, insert `.await` after that call's own closing
paren) to speed through the ~1,500 missing-await call sites this crate's file had accumulated. It
found and "fixed" 1,479 sites in one pass — and broke roughly 250 of them, silently, for two
independent reasons:
  1. **Short, common names collide with unrelated std sync methods on the receiver's real type.**
     `len`, `new`, `get`, `fill`, `count`, `count_ones`, `is_empty`, `clear`, `push`, `contains`,
     `split`, `as_str`, `add` are all genuinely `async fn` on SOME first-party type in this crate
     — and also inherent sync methods on `Vec`/`HashMap`/`HashSet`/`Option`/`&[T]`/`u64`/`String`.
     A name-only match cannot tell `self.words[..n].fill(u64::MAX)` (std slice, sync) from
     `self.mask.fill()` (our `TokenBitset`, async) apart. My script's own "exclude names that are
     ALSO declared as a plain `fn` somewhere in this file" guard caught exactly 2 of these 13
     (`get`, `new`) — it cannot catch a collision with a type it never declared, i.e. any std type.
     Recommend a hard ban on this shape of tool for the remaining ~49 packets.
  2. **Appending `.await` right after a call's own closing paren is wrong whenever that call's
     future is deliberately passed on un-awaited** — every `Box::pin(recursive_call(..))` (Finding
     E) and every async-closure factory got its inner call awaited and the outer `Box::pin`/closure
     left broken, because the tool has no notion of "this call's result flows somewhere other than
     immediate resolution."
  3. A latent bug in my own balanced-paren matcher (didn't skip over string-literal contents) meant
     a handful of edits landed at the wrong byte offset entirely when a nearby string literal
     contained bracket-shaped punctuation (`"expected ',' or ']'"`), producing genuine text
     duplication in a few spots, not just a misplaced token.

  **Recovery, for the record, since the next packet may hit the same temptation**: the sound
  half of automation is entirely rustc-diagnostic-driven and already exists — `insert-await.py`
  applies only a diagnostic's own unambiguous `suggested_replacement`. I built the missing
  mirror-image tool (`remove_bad_await.py`, left in the ticket folder as `.txt`-adjacent scratch —
  see Files, not a proposed permanent addition) that does the same thing in reverse: for every
  `E0277`/`E0599`/`E0308` diagnostic whose **primary span is exactly the token `await`**, delete
  that byte range (plus the preceding `.`) from the real file. Both tools only ever act on a span
  rustc itself identified as the exact problem; neither ever guesses a call site from a name.
  Iterating `remove_bad_await.py` → `insert-await.py` → manual fixes for the shapes neither tool
  can see (Findings C/D/E) is what actually recovered this file, over roughly a dozen passes, from
  1,479 blind edits down to a clean `cargo check --all-targets` + `cargo test`. **Recommendation for
  the ~49 remaining packets: if you build any bulk `.await` tool, make it diagnostic-span-driven
  like these two, never name-matched — and if you ever do run a name-matched pass by mistake, the
  fastest recovery path is exactly this: strip with `remove_bad_await.py`'s technique, then
  re-insert with `insert-await.py`, then hand-fix what's left, verified by `cargo check
  --all-targets` (not `--lib` alone — it hid the `#[test] async fn` class entirely in this crate,
  191 sites, until `async-test-attr.py` was run).**

## 6. Files touched

Owned path only:
- `🧰️framework/🔨️modules/🧮️math/🎯️sampling/🦀️component.rs` — the whole de-dyn conversion (9 trait
  families), plus the missing-`.await` repair across the file (pre-existing backlog from an earlier
  bulk asyncify pass that had never been processed), plus 3 `// 🚫️async: E1`/`E4`-tagged sync fns.
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📦️glue.rs` — `#![allow(async_fn_in_trait)]` (R3/R7).
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/Cargo.toml` — added `semio-framework-dispatch-macros`
  (real dependency) and `semio-framework-async-macros` (dev-dependency, via the ticket's
  `async-test-attr.py --apply` over this path only, 191 sites rewritten to
  `#[semio_framework_async_macros::async_test]`).

Ticket folder (scratch, not the repo):
- `📓️terra-math-dedyn-report.md` (this file).
- Scratch tooling and intermediate JSON reports live under the session scratchpad
  (`<scratchpad>/mathdedyn-verify/`, `<scratchpad>/*.py`, `<scratchpad>/*.json`), not the ticket
  folder or the repo — nothing there needs cleanup inside the repo itself, per rule 24.

Nothing outside `🧰️framework/🔨️modules/🧮️math/**` and the ticket folder was modified.

## 7. What is NOT done

- `semio-framework-geometry` remains broken (146 errors) — out of scope, flagged, re-verified
  unchanged at the end of this session. Whoever owns `📐️geometry` next should run this ticket's
  `insert-await.py --scope '📐️geometry'` (or equivalent) — the fix shape is identical to what I
  built into `geomstub` as a stand-in, just needs to land in the real file.
- The real `cargo check -p semio-framework-math --all-targets` / `cargo test -p
  semio-framework-math` have not been run successfully against the real workspace, because of the
  above. Re-run once `📐️geometry` is fixed; I expect both green based on the scratch-mirror result
  (same source file), but this is an expectation, not an observation.
- `semio-framework-math` also declares a `wasm32` target dependency (`wasm-bindgen`) and
  `crate-type = ["rlib", "cdylib"]`; I did not attempt a wasm build (not in my acceptance list, and
  `geomstub` doesn't stand in for a wasm target). Worth a quick wasm check once `📐️geometry` is
  fixed, since the whole crate went through a large mechanical edit.
- `cargo fmt --check` reports a handful of pure line-wrapping diffs (function calls/struct literals
  now slightly over rustfmt's width preference after `.await` insertion) — cosmetic only, no
  behavior change; left as-is rather than reformatting the whole 10k-line file and risking touching
  code outside this session's actual edits.
