# R3 — De-Async Repair: Macro, Draw-FSM, Vulkan Backend

Packet R3 of Phase 1.5. Ownership boundary: `semio-framework-machine-derive`,
`semio-s-plugin-draw-fsm`, `semio-framework-ui-backend-vulkan` only.

## 1. `semio-framework-machine-derive` — proc-macro entry signatures

`📦️glue.rs` declared all four proc-macro entry points `async`:

```rust
#[proc_macro]
pub async fn statechart(input: TokenStream) -> TokenStream { … }
#[proc_macro_derive(StatechartEvent)]
pub async fn derive_statechart_event(input: TokenStream) -> TokenStream { … }
#[proc_macro_derive(StatechartSchema)]
pub async fn derive_statechart_schema(input: TokenStream) -> TokenStream { … }
#[proc_macro]
pub async fn export_wasm_machine(input: TokenStream) -> TokenStream { … }
```

These can never compile: a proc macro entry point runs inside `rustc` itself, which has
no async executor, so the compiler rejects any signature other than
`fn(TokenStream) -> TokenStream` (function-like/attribute macros) or
`fn(TokenStream) -> TokenStream` (derive macros) — hence "function-like proc macro has
incorrect signature" / "derive proc macro has incorrect signature" (4 errors, matching
the ticket's count exactly). Fix: dropped `async` from all four. Unambiguous, no
alternative reading.

### Macro-internal helper functions also generated with blanket `async`

The owner file `✨️derive/🦀️component.rs` (1520 lines: `mod analyze`, `mod codegen`,
`mod parse`, the four `expand_*` wrappers, and `#[test]` functions) had **59** `async fn`
in total and **zero** `.await` calls anywhere in the file. Every internal call site
(`self.add_node(...)?`, `compute_fingerprint(...)`, `codegen::emit(&ir)`, etc.) was
already written assuming a synchronous return value — confirming these were blindly
stamped `async` by the repo-wide convention and never actually used as such. Stripped
`async` from all 47 of these (parse-time AST builders, IR analysis, codegen-time
`TokenStream` builders, the four `expand_*` entry-adjacent functions, and the 10
`#[test]` functions, which cannot be `async` under plain `#[test]` at all — that alone
was 10 of the 14 "test" errors seen under `--all-targets`).

### Generated code that must stay `async` (trait-impl-constrained, cross-boundary)

Five `async fn` are literal tokens inside `quote!{}` blocks that become the macro's
*output* — code compiled later, in a consumer crate, as an `impl machine::StatechartEvent`
/ `impl machine::Machine` block:

- `emit()`: `async fn event_id`, `async fn event_name` (StatechartEvent), `async fn definition` (Machine)
- `expand_statechart_event()`: `async fn event_id`, `async fn event_name`

`semio-framework-machine` (the **runtime** crate, *outside this packet's ownership
boundary* — path `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs`, not the `✨️derive/`
one) currently declares these trait methods `async` too (`pub trait Machine { async fn
definition() -> …; }`, similarly for `StatechartEvent`). An `impl` must match its
trait's `async`-ness, so these five were **left async** — flipping them to `fn` would
just trade a missing-`.await` error for a trait-signature-mismatch error, and either way
the call is not mine to make: it belongs to whoever owns `semio-framework-machine`.
Their bodies contain no calls into other async APIs, so they need no further changes.

The `export_wasm_machine!` output block (`new`/`send_json`/`tick`/`snapshot_json`/
`restore_json`/`manifest_json`/`on_effect`, wasm-bindgen inherent methods, not trait
impls) was left as-is: also async, also calling into the (out-of-scope,
currently-async) `machine::init`/`macrostep`/`route_command`/`timer_elapsed`/
`persist`/`restore`/`Machine::definition()`. Nothing in my three owned crates exercises
`export_wasm_machine!`, so I could not compiler-verify a change here and left it
untouched rather than guess.

### Verification

- `cargo check -p semio-framework-machine-derive --all-targets` — clean (was 4 lib +
  14 lib-test errors).
- `cargo clippy -p semio-framework-machine-derive --all-targets` — clean.
- `cargo test -p semio-framework-machine-derive` (debug) — 10/10 pass.
- `cargo test -p semio-framework-machine-derive --release` — 10/10 pass.
- `bun ./📜️script.ts verify dependencies` — clean, 238/238, no new deps.
- Proc-macro crates always build for the host, never for `wasm32`/other targets — no
  wasm build applicable here.

## 2. Workspace cascade from the macro fix

`cargo check --workspace --all-targets` is **not deterministic** in this repo's current
state: with dozens of simultaneously-broken, mutually-independent crates, which crates
even get *reached* (vs. abandoned because an upstream dependency failed first) varies
between runs depending on job scheduling and incremental-cache fingerprints. I captured
multiple full-workspace runs before and after the fix; totals moved 1058 → 569 → 957 →
1145 across successive invocations with **no further edits in between** the last three.
So a single "before/after total" is not a meaningful number here — instead, the
reliable signal is per-crate, from `-p`-scoped standalone checks.

What is solid and reproducible:

| Crate | Before | After | In R3 scope? |
|---|---|---|---|
| `semio-framework-machine-derive` | 4 (lib) + 14 (lib test, `--all-targets`) | **0** | yes — fixed |
| `semio-s-plugin-draw-fsm` | 5 | **0** | yes — fixed |
| `semio-framework-ui-backend-vulkan` | 1 | 1 (unchanged, expected — see §4) | yes — confirmed clean |
| `semio-framework-ui` | 557 | **0** | no — fixed as a side effect of unblocking the macro |
| `semio-framework-2d` | 28 | **0** | no — fixed as a side effect |
| `semio-framework-graph` | 8 | **0** | no — fixed as a side effect |
| `semio-framework-hub` | 70 (baseline) | 18–33 across runs | no — Phase 1 touched this, out of scope |
| `semio-framework-machine` (runtime crate, not `-derive`) | not reached (masked) | **351** (`cargo check -p semio-framework-machine --all-targets`, default features) | **no — same non-suspending-`async fn` bug class, needs its own packet** |
| `semio-framework-ui-backend-webgpu` | not reached (masked) | 476 in some runs, 0/absent in others (reachability-dependent) | no |
| `semio-framework-ui-backend-d3d12` | 332 (masked count, unreliable) | genuinely Windows-only (`compile_error!` gate) plus a real bug: `🦀️types.rs` does `use windows::Win32::…` **without** a `target_os = "windows"` cfg gate around the module, so on macOS/Linux it fails with "cannot find crate `windows`" — the module-gating discipline vulkan's own file documents (see §4) was not applied here | no |

The macro fix is confirmed to be the "highest-leverage fix" as briefed: unblocking it
let `rustc` proceed far enough to fully clear `semio-framework-ui`, `semio-framework-2d`,
and `semio-framework-graph` (593 errors gone), and also **unmasked** `semio-framework-machine`
itself, which turns out to be extensively broken by the identical bug class (`async fn`
declared with zero genuine suspension anywhere — same pattern as the derive crate before
this fix: traits/free functions like `Machine::definition`, `StatechartEvent::event_id`,
`kernel::macrostep`, `kernel::route_command`, `persist::restore`, `BitSet::contains`,
etc. are all `async fn` with call sites that assume sync results). This crate is
**out of R3's ownership boundary** and needs its own repair packet before the derive
crate's generated-code `async` (§1) can be safely reconsidered.

## 3. `semio-s-plugin-draw-fsm` — 5 errors, all missing `.await`

The plugin owns its own hand-copied kernel at `🔄️fsm/🦀️component.rs` (a sibling, not a
reuse, of the shared `semio-framework-machine` kernel — consistent with this repo's
"if code is repeated, keep it close" rule; the file even carries its own R10/R11
residue-shape comments documenting a prior, evidently-careful de-async pass). That pass
was almost complete: of 31 `async fn` remaining, all but three are `#[test]`-adjacent
functions using the crate's own `#[semio_framework_async_macros::async_test]` harness
(which does support async test bodies — unlike plain `#[test]`, so these were correctly
left alone), plus the `Migration` trait (`source_fingerprint`/`migrate` — genuinely
user-supplied, plausibly-suspending hooks) and `persist::restore()`, which calls them
with real `.await`s already in its body. `persist()` itself was already de-asynced.

The 5 real errors were exactly the leftover call sites that never got their `.await`
added when `restore()` was correctly kept `async`, plus one unrelated bug uncovered
alongside it:

- `🔄️fsm/🦀️component.rs:1398,1408,1431` (persist module tests) and `:2410`
  (`checkout_integration` test) — `restore::<…>(...)` called without `.await`
  (`E0599`/`E0308`/`E0369`-shaped: "method not found for `impl Future<...>`", etc.).
  Added `.await` at each of the 4 sites.
- `:1431` additionally had `E0107`: `restore::<UnitToggleMachine>(...)` supplied only
  one of `restore`'s two generic parameters (`M`, `Mg`); the migration type
  (`BumpFingerprint`, defined two lines above) was missing. Added it:
  `restore::<UnitToggleMachine, BumpFingerprint>(...)`.

### Verification

- `cargo check -p semio-s-plugin-draw-fsm --all-targets` — clean (was 5 errors).
- `cargo clippy -p semio-s-plugin-draw-fsm --all-targets` — clean.
- `cargo test -p semio-s-plugin-draw-fsm` (debug) — 26/26 pass, including all
  `#[async_test]`-attributed tests (confirms the custom harness genuinely executes them,
  not just compiles).
- `cargo test -p semio-s-plugin-draw-fsm --release` — 26/26 pass.
- `cargo check -p semio-s-plugin-draw-fsm --target wasm32-unknown-unknown --all-targets`
  (debug and `--release`) — clean. The crate is wasm-gated (`export_wasm_machine!` via
  its own sibling `semio-s-plugin-draw-fsm-macros` crate, `wasm-bindgen`/`wasm-bindgen-futures`
  dependencies under `target_arch = "wasm32"`); this needed checking separately from the
  native target and does compile. One pre-existing, unrelated `wasm_bindgen` deprecation
  warning surfaced ("async constructors produce invalid TS code and support will be
  removed") from that macro's generated `ToggleMachine::new` — not part of the
  never-suspends bug class (wasm-bindgen's own tooling advice, unrelated to whether the
  function type-checks), left as-is.
- `cargo clippy -p semio-s-plugin-draw-fsm --target wasm32-unknown-unknown --all-targets`
  — clean.
- Remaining warnings in this crate (one `unused_qualifications` at `:2380`) are
  pre-existing and outside this bug class; left untouched.

## 4. `semio-framework-ui-backend-vulkan` — the "1 error" is not a bug

`cargo check -p semio-framework-ui-backend-vulkan --all-targets` on this (macOS) host
reports exactly one error:

```
error: semio-framework-ui-backend-vulkan builds only on Linux.
  --> …/🌋️vulkan/📦️packages/🦀️rust/📦️glue.rs:22:1
   |
22 | compile_error!("semio-framework-ui-backend-vulkan builds only on Linux.");
```

This is an intentional, documented platform gate (`📦️glue.rs`'s header explains it
exists specifically so a non-Linux `cargo check` sees *one* clean, deliberate error
instead of cascading "can't find crate `ash`" noise, since `ash`/`ash-window` are
`[target.'cfg(target_os = "linux")'.dependencies]`). It is **not** part of the
never-suspends `async fn` bug class and needs no code change.

To actually validate the crate, I cross-checked it against its real target:
`cargo check -p semio-framework-ui-backend-vulkan --all-targets --target
x86_64-unknown-linux-gnu` — **clean**, zero errors, only pre-existing `dead_code`/clippy
warnings. `cargo clippy` on the same target — clean. `cargo test --target
x86_64-unknown-linux-gnu --no-run` builds the test binary successfully but fails at the
final **link** step (`-fuse-ld=mold`: "invalid linker name") — a macOS-host cross-linker
toolchain gap (no `mold`/Linux cross-linker installed here), not a code defect; type
checking (the meaningful signal for this bug class) already succeeded.

No changes were made to this crate. The reported "1 error" before and after this packet
is the same, expected, intentional platform gate.

## Summary of files touched

- `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/📦️glue.rs` — removed
  `async` from all 4 proc-macro entry points.
- `🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs` — removed `async` from 47
  macro-internal functions (parser, analyzer, codegen, `expand_*`, tests); left 5
  `async fn` inside `quote!{}`-generated trait-impl output unchanged (must match the
  out-of-scope `semio-framework-machine` trait declarations); reformatted via `rustfmt`.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️component.rs`
  — added 4 missing `.await`s on `restore(...)` calls, fixed 1 missing generic argument
  (`Mg = BumpFingerprint`); reformatted via `rustfmt`.
- `semio-framework-ui-backend-vulkan` — **no changes**; confirmed clean on its real
  (Linux) target.

## Cross-boundary findings for other packets / the coordinator

- **`semio-framework-machine`** (runtime crate, `🧰️framework/🔨️modules/🔄️machine/🦀️component.rs`,
  *not* `✨️derive/`) is extensively broken by the identical bug class this whole sweep
  targets — **351 errors** via `cargo check -p semio-framework-machine --all-targets`
  (default features) once the derive crate no longer masks it. It was invisible in the
  original ticket's inventory only because the macro's signature errors aborted the
  build before `rustc` ever reached this crate's own body. It needs its own repair
  packet, and that packet's fix should be coordinated with whether the 5 generated
  `async fn` left alone in `machine-derive` (§1) get flipped to sync once the trait
  declarations they implement do.
- **`semio-framework-ui-backend-d3d12`** has an incomplete platform gate: unlike
  vulkan's careful per-`mod` `#[cfg(target_os = "linux")]` gating, d3d12's `🦀️types.rs`
  imports `windows::…` without a `target_os = "windows"` guard, so it hard-fails on
  macOS/Linux with "can't find crate `windows`" well beyond the intentional
  `compile_error!` banner. Out of this packet's scope; flagged for whoever owns that
  crate.
- Full-workspace `cargo check --workspace --all-targets` totals are unstable
  run-to-run in the current tree (observed 1058 → 569 → 957 → 1145 with no edits between
  the last three) because so many crates are simultaneously and independently broken
  that dependency-graph reachability — and therefore which crates even get diagnosed —
  varies with build scheduling/incremental-cache state. Per-crate `-p`-scoped checks are
  the reliable unit of measurement until more of the workspace is repaired.
