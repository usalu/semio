# 📓️ terra — sdk-final report

Packet: **sdk-final**. Crate `semio-framework-plugin` (guest SDK), owned path
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` excluding `🖥️host/**`.

**Bottom line: `--lib` went 26 → 7. All 19 fixable errors are fixed. The remaining 7 are ONE
pre-existing, already-documented, store-side blocker (`dispatch_group`/`MemberFactory`) that is
provably impossible to close from the plugin crate (Rust orphan rule — see below), confirmed still
blocked by a fresh re-read of `🏪️store` (not mine). `--all-targets` surfaces a SEPARATE, much
larger residue (1,381 errors, almost entirely in `#[cfg(test)]` code) that this packet's brief never
scoped for — flagged prominently below, not attempted. Both downstream fleet crates
(`semio-s-plugin-note`, `semio-s-plugin-stdio`) are blocked upstream by an unrelated crate
(`semio-framework-number`, 620 errors) before the dependency graph even reaches `semio-framework-plugin`
— also not mine, also flagged. `os-kernel` and `framework` are confirmed EXIT 0 at time of writing.**

---

## 1. Baseline re-measured fresh (not trusted from the brief)

`CARGO_TARGET_DIR=.../scratchpad/target-sdkfinal cargo check -p semio-framework-plugin --lib`,
foreground, single turn:
```
error: could not compile `semio-framework-plugin` (lib) due to 26 previous errors; 9 warnings emitted
EXIT:101
```
(Brief said 27; disk showed 26 — one fewer than expected, noted and moved on per rule 11 — named
sets, not counts, and I verified every one of the 26 individually below.)

Profile, independently re-derived by grepping the full (non-`--message-format=short`) output:
- **E0728 × 2** — `.await` inside the sync `marks_for` closure (~:11940/:11943 pre-shift)
- **2 plain "lifetime may not live long enough" + 1 `E0515`** (3 total, all same root cause) —
  `validate_element_id` closure at ~:5124
- **E0277 × 3** — `ExampleSource::new(...)` (an `async fn`) passed un-awaited into
  `example_source(impl Into<ExampleSource>)` at ~:7900
- **E0277 × 3 + E0308 × 4** (7 total) — `dispatch_group`/`MemberFactory` mismatch at ~:11512/:11520/
  :11688/:11696
- **11 plain "future cannot be sent between threads safely"** (no E-code — this is most of the
  brief's "+ a tail") — the three `erased_compose` fn-pointer thunks at ~:537/:576/:614

2+3+3+7+11 = 26. Exact accounting, no unexplained residue.

## 2. Fixes applied (19 of 26 errors)

### 2a. E0728 — `.await` inside sync closure (`marks_for`, ~11940)

Brief's own pattern-1 diagnosis was right, but the deeper cause was that `InteractionView::
peers_selecting`/`peers_hovering` (`🦀️component.rs:8140`/`:8153`) were themselves wrongly `async fn`
— pure in-memory `BTreeMap`/`Vec` filters, **zero I/O**, no suspension point anywhere in their
bodies. Consumer (`marks_for`) is a plain `&dyn Fn(&str) -> Vec<UiPeerMark>` that
`ui_tree_stamp_presence` takes — language-barred from being async. Textbook **R9** (E1-transitive):
made both methods sync and tagged them:
```rust
// 🚫️async: E1 pure in-memory filter, no suspension point — consumed by the sync `marks_for`
// closure `ui_tree_stamp_presence` takes (a plain `&dyn Fn`, language-barred from being async) — see R9.
pub fn peers_selecting(&self, domain: &str, id: &str) -> Vec<PeerMark<'a>> { ... }
```
**Both halves shown, per R9's own requirement**: (a) no I/O in the callee — verified by reading both
bodies, pure `.iter().filter_map().collect()` over an in-memory map; (b) the only real consumer
(`marks_for`) is a plain sync `Fn` closure, language-barred from async. Updated the two call sites
(dropped `.await`). **Bonus finding**: an existing test at `:18325`
(`interaction_view_peers_selecting_returns_actor_and_color`) already called both methods
*without* `.await` and chained `.len()`/indexing directly on the result — i.e. the test file already
encoded the CORRECT (sync) shape and would not have compiled once `--all-targets` reached it. This
is strong independent confirmation the R9 read is right, not just permitted.

### 2b. Lifetime / E0515 — `validate_element_id` closure (~5124)

Two separate defects were fused into one closure:
1. **E0515** (temporary): `&format!("introduction step {}", step.id)` built a `String` inline as a
   closure argument on every call — the future returned by `validate_referenced_element_id`
   borrowed it, but the temporary died at the end of the closure-call statement. Fixed by hoisting:
   `let step_context = format!(...);` once per `step` iteration, closure/call sites borrow
   `&step_context` instead.
2. Two **plain lifetime errors** (`'1` must outlive `'2`) persisted even after the hoist — the
   closure returns `impl Future` that borrows BOTH its per-call arguments (`id`, `role`) AND
   outer-captured state (`&self.id`, `&step_context`, three more refs), and a plain (non-HRTB)
   closure can't express the right lifetime relationship between "per-call argument lifetime" and
   "closure-capture lifetime" for a future-returning body. Per the brief's own stated preference
   ("prefer the loop if the closure has few call sites" — here exactly 4), **deleted the closure
   entirely** and inlined the 4 calls to `validate_referenced_element_id(...).await` directly. Zero
   lifetime ambiguity left — each call site now has its own concrete borrow.

### 2c. E0277 × 3 — `ExampleSource::new(...)` not awaited (~7900)

`ExampleSource::new` is `pub async fn`; `App::example` (itself `async fn`) passed the un-awaited
future straight into `example_source(impl Into<ExampleSource>)`. One-line fix:
```rust
self.example_source(ExampleSource::new(id, label, document_json, icon_id).await).await
```

### 2d. The 11 "future cannot be sent" errors — R1's own sanctioned Send erasure vs. plain-AFIT traits

This was the real investigation. `ComposeFuture<'a> = Pin<Box<dyn Future<...> + Send + 'a>>` is
defined in `🚪️io/🦀️component.rs` (NOT mine) and is **explicitly R1-legal** — confirmed by reading
`📓️terra-io-dedyn-report.md`'s own verification section, which calls it out by name as
"fn-pointer/erasure-table plumbing... not touched." The three `erased_compose` fn-pointer thunks
(E4, already tagged) in `composer_entry_of`/`deserializer_entry_of`/`serializer_entry_of` build a
`Box::pin(async move { ... })` that must coerce into that `Send`-bounded type, but its body
`.await`s `ArtifactSerializer::serialize`/`ArtifactDeserializer::deserialize`/`ArtifactComposer::
compose` (mine, plain AFIT, deliberately NOT Send per O1/R3) and `store::ArtifactPack::decode_pack`/
`encode_pack` (NOT mine, also plain AFIT). **R7 forbids rewriting a guest trait's async fn into
`-> impl Future + Send`** to silence this — so that route was out regardless of whose file it's in.

The fix already lives in this exact file, used ~30 times: `resolve_ready<F: Future>(fut: F) ->
F::Output` (`🚪️io/🦀️component.rs:886`, tagged E5) polls a future that is *guaranteed Ready on first
poll* and returns its `Output` directly — no `Send` bound at all, since it never actually suspends.
The doc comments on `ArtifactSerializer::serialize`/`ArtifactDeserializer::deserialize`/
`ArtifactComposer::compose` already say outright: "every existing leaf's serialize never truly
awaits, so its future resolves the moment it is first polled (see `resolve_ready`...)" — i.e. the
bridge was already the intended design, just not yet applied at these three call sites. Replaced all
four `.await`s per thunk with `resolve_ready(...)`:
```rust
let composed = resolve_ready(C::compose(&typed_sources))?;
let bytes = resolve_ready(ArtifactPack::encode_pack(&composed.snapshot));
```
Once there is no real `.await` left inside the `Box::pin(async move { ... })` bodies, the block is
trivially `Send` (nothing but plain data crosses a suspension point, because there is no suspension
point) — no `+ Send` added anywhere, R3 untouched, R1's erasure point satisfied exactly as designed.

## 3. The remaining 7 — confirmed BLOCKED, not fixed, not mine

`dispatch_group<M: SpaceMember + MemberFactory>(parent: &mut M, ..., children: &mut [(&mut M, ...)],
...)` (`🏪️store/🦀️component.rs:8294`) needs ONE `M` for both the parent (`&mut self.store:
ArtifactStore<A::Snapshot, A::Mutation>`) and every child. Even in the degenerate same-type case,
production `ArtifactStore<P, Mutation>` implements `SpaceMember` but **not** `MemberFactory` — I
re-read `🏪️store/🦀️component.rs` fresh (line 9087) and confirmed the only `impl MemberFactory for
ArtifactStore<P, Mutation>` that exists is `#[cfg(test)] mod tests`'s **local shadowing newtype**
(`struct ArtifactStore<P, Mutation>(super::ArtifactStore<P, Mutation>)` at line 8962) — invisible
from `semio-framework-plugin`, exactly as `📓️terra-dedyn-fw-os-spacemember-report.md` already
documented and left as a `🚧️ BLOCKED` inline comment (still present, still accurate, at
`🦀️component.rs:11513` in the current file).

**Why this cannot be closed from the plugin side at all, not just "not yet"**: `MemberFactory` is
declared in `🏪️store` and `ArtifactStore` is also declared in `🏪️store` — both foreign to
`semio-framework-plugin`. Rust's orphan rule requires the trait OR the type to be local to the
implementing crate; neither is. No plugin-side newtype, wrapper, or generic trick can add this impl.
It structurally requires one of the two fixes the predecessor already named, on the `🏪️store` side:
1. Split `CompositionCoordinator::dispatch_group`/`undo_group`/`redo_group`/`compensate` into
   `<Mp: SpaceMember, Mc: SpaceMember + MemberFactory>` (parent vs. children), or
2. Give production `ArtifactStore<P, Mutation>` a real (non-`#[cfg(test)]`) `MemberFactory` impl.

**`lease-request`**: `🏪️store/🦀️component.rs` — need (1) or (2) above to close the last 7 errors in
`semio-framework-plugin --lib`. Not attempted (not owned; store is explicitly the live sibling's
path). Exact 7, current line numbers: `dispatch_emit_group`'s `dispatch_group` call (3×E0277 +
1×E0308 at ~11512–11520) and `dispatch_group_history_action`'s two `members.push((&parent_ref,
&mut self.store))` sites (2×E0308 at ~11688/11696) plus one more E0308 downstream of the first
(`absorb_created_children` type mismatch).

## 4. Acceptance — run in order, pasted verbatim, foreground, `CARGO_TARGET_DIR=.../scratchpad/target-sdkfinal`

### 4a. `cargo check -p semio-framework-plugin --lib`
```
error[E0277]: the trait bound `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>: MemberFactory` is not satisfied
error[E0308]: mismatched types
error[E0277]: the trait bound `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>: MemberFactory` is not satisfied
error[E0277]: the trait bound `ArtifactStore<<A as ArtifactApp>::Snapshot, ...>: MemberFactory` is not satisfied
error[E0308]: mismatched types
error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `semio-framework-plugin` (lib) due to 7 previous errors; 9 warnings emitted
EXIT:101
```
**NOT EXIT 0.** 26 → 7, all 7 accounted for and blocked per §3. This is the honest result — I am not
claiming green on a crate that isn't.

### 4b. `cargo check -p semio-framework-plugin --all-targets`
```
error: could not compile `semio-framework-plugin` (lib test) due to 1381 previous errors; 10 warnings emitted
EXIT:101
```
**Flagging prominently, per the brief's own instruction for corruption/surprises.** This is NOT the
same 7 errors plus test-harness noise — it is a categorically separate, much larger residue, almost
entirely inside `#[cfg(test)] mod tests`, breaking down as:
```
579  E0599  (method not found)
344  E0308  (mismatched types)
235  E0277  (trait bound)
 92  E0609  (field not found)
 60  E0728  (await outside async)
 44  E0369  (binop not supported)
 13  E0600
  7  E0608
  2  E0432  (unresolved import — incl. crate::app::__semio_dispatch_PluginApp)
  1  E0659  (ambiguous __semio_dispatch_PluginApp)
  1  E0425, E0391, E0283 each
```
This packet's brief was framed entirely around the `--lib` 27(26) errors and gave zero indication of
a residue two orders of magnitude larger sitting in the test module. I did **not** attempt to fix
this — it needs its own scoped packet (rule 25: atomic packets get redirected before start or finish
clean, not partially absorbed into an unrelated one under time pressure). Sampling suggests it's
NOT simple await-insertion residue like the lib fixes above — the `__semio_dispatch_PluginApp`
ambiguous-import errors in particular look like a macro-expansion issue possibly orthogonal to
async conversion entirely. Needs a coordinator triage pass before any packet is dispatched at it.

### 4c. `cargo test -p semio-framework-plugin --lib`
**Not run.** `--lib` doesn't compile (§4a), and the test *harness* needs `--all-targets`-equivalent
compilation (§4b) to even build, which fails at 1,381 errors. Running `cargo test` would just
reproduce §4b's failure with no new information — skipped rather than pasting a duplicate.

### 4d. Fleet payoff — `semio-s-plugin-note` / `semio-s-plugin-stdio`
```
$ cargo check -p semio-s-plugin-note --lib
...
error: could not compile `semio-framework-number` (lib) due to 620 previous errors
EXIT:101

$ cargo check -p semio-s-plugin-stdio --lib
...
error: could not compile `semio-framework-number` (lib) due to 620 previous errors
EXIT:101
```
**Neither reaches `semio-framework-plugin` in the dependency graph.** Both abort earlier, inside
`🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust` (`semio-framework-number`, 620 errors) — a
crate with no relationship to plugin/io/store, evidently mid-refactor by an unrelated concurrent
session (matches the documented "Concurrent Cargo Workspace Churn" pattern: transient, another
session's in-progress work, not mine to fix). This means the two fleet crates' OWN readiness against
the new SDK is still unmeasured — the dependency graph never got that far. Re-run once
`semio-framework-number` is green elsewhere.

### 4e. `os-kernel` / `framework` — re-verified fresh, immediately before writing this report
```
$ cargo check -p semio-framework-os-kernel --lib
warning: `semio-framework-os-kernel` (lib) generated 57 warnings
    Finished `dev` profile [unoptimized] target(s) in 1.77s
EXIT:0

$ cargo check -p semio-framework --lib
warning: `semio-framework` (lib) generated 27 warnings
    Finished `dev` profile [unoptimized] target(s) in 3.56s
EXIT:0
```
Both **EXIT 0**. (Note: mid-session, `os-kernel` transiently broke twice more from a concurrent
peer's in-flight edits to `🗣️dsl/🧬️schema/🦀️component.rs`, `📡️spr/🧪️testkit/🦀️component.rs`, and
`📇️directory/🔌️client/🦀️component.rs` — all outside my path, all self-resolved as that session's
own repair sweep progressed; I polled rather than touched them, per rule 3. By the time of the runs
above they were clean.)

## 5. Corruption check (brief's explicit ask)

Grepped my own edited regions and their surroundings for the `)await` / dangling `await.` / in-string
`await.` corruption pattern described in the brief. **None found in anything I touched.** The
delimiter-mismatch corruption I *did* observe (§4e note) was a different shape (missing `);` between
two statements, not a misplaced `.await`) and was in files outside my path — not investigated further
since it self-resolved and isn't mine.

## 6. Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — all
  fixes in §2 (composer/deserializer/serializer `erased_compose` thunks; `InteractionView::
  peers_selecting`/`peers_hovering` de-asyncified with R9 tags + 2 call sites;
  `validate_element_id` closure removed, 4 call sites inlined; `App::example`'s
  `ExampleSource::new(...).await`).
- Ticket folder scratch (this report only; no other new scratch files needed — verification was done
  via direct `cargo check` runs, not custom tooling).

## 7. What's next (for the coordinator, not attempted by me)

1. **Store-side** (`🏪️store/🦀️component.rs`): give production `ArtifactStore<P, Mutation>` a real
   `MemberFactory` impl, or split `dispatch_group`/`undo_group`/`redo_group`/`compensate` into two
   type params. Either closes all 7 remaining `semio-framework-plugin --lib` errors — I'd expect
   EXIT 0 immediately after, since I've verified there's nothing else in `--lib`.
2. **New packet needed**: `semio-framework-plugin`'s `#[cfg(test)]` module, 1,381 errors, scoped
   separately from `sdk-final` (§4b's breakdown is a reasonable starting triage).
3. **Unrelated, needs its own owner**: `semio-framework-number`, 620 errors, blocking both
   `semio-s-plugin-note` and `semio-s-plugin-stdio` before they even reach the new SDK.
