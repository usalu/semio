# 🧩️ The WASI component-model toolchain cluster — what actually links into the `.wasm`

## Headline

**Of the 18 named crates, exactly ONE genuinely links into the guest component: `wit-bindgen`
itself.** The other 17 are host-only build machinery, reachable exclusively through the
`wit-bindgen-rust-macro (proc-macro)` subtree — the WIT parser/component-encoder/pretty-printer
that runs *at compile time* inside the `wit_bindgen::generate!` proc macro to turn `.wit` source
into Rust source. Proc-macro crates and everything reachable only through them compile for the
**host**, never for `wasm32-wasip2`, and never appear in the `.wasm`.

A 19th, unlisted crate — `bitflags` — was also found genuinely linked (via `wit-bindgen`'s own
default feature, not through the proc-macro), but proven dead code (no WIT `flags` type exists in
this world) and has been trimmed. See "Fix applied" below.

`wit-bindgen` is not a candidate for removal: it is the mechanism by which a `wasm32-wasip2` binary
**is** a WASI component (canonical-ABI lifting/lowering, resource handles, and — concretely
exercised here — the component-model async subtask/waitable machinery, because `world actor`'s
`poll` export is declared `async func`). Calling it a third-party runtime dependency to be
eliminated is the category error the ticket owner suspected. It stays.

## Method

1. `cargo tree -p semio-s-plugin-draw-fsm --target wasm32-wasip2 --edges normal --prefix none` —
   the crate the ticket's own headline table names. **Zero of the 18 crates appear.** Reason:
   `semio-s-plugin-draw-fsm` (`✏️s/…/🔄️fsm/📦️packages/🦀️rust`) depends only on
   `semio-framework-dispatch-macros`, `semio-framework-os-kernel`, `semio-framework-value-derive`
   — it never depends on `semio-framework-plugin`, so `wit-bindgen` never enters its graph. This
   crate is a statechart helper library, not a WASI component: it exports no `world actor` and its
   built `.wasm` (`target/wasm32-wasip2/debug/semio_s_plugin_draw_fsm.wasm`, already on disk,
   14,363 bytes) contains **zero** `wit_bindgen` symbols and **zero** `serde` symbols by string
   search. The ticket's "draw-fsm 11 (6 linked)" figure is real and about serde, but it is
   evidence-free on the wit-bindgen question — this crate cannot answer it. **Correction for the
   scoreboard: the actual shippable draw component is `semio-s-plugin-draw`
   (`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust`), which depends on `semio-framework-plugin` with
   `features = ["component-guest"]` and is the one that pulls the wit-bindgen cluster.**

2. `cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 --edges normal --prefix none` (239
   lines, exit 0, no stderr) — unique non-`semio-` crates: `anyhow arrayvec bitflags bytemuck
   bytemuck_derive equivalent foldhash hashbrown heck id-arena indexmap itoa kurbo leb128fmt log
   macro-string memchr polycool prettyplease proc-macro2 quote semver serde serde_core
   serde_derive serde_json smallvec syn unicode-ident unicode-xid wasm-encoder wasm-metadata
   wasmparser wit-bindgen wit-bindgen-core wit-bindgen-rust wit-bindgen-rust-macro wit-component
   wit-parser zmij` — all 18 named crates present, confirming this is the right crate to answer the
   question on.

3. `cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 -i <crate>` for each of the 18,
   **full un-truncated output read for every one** (the ticket's own caveat: a truncated `-i` once
   flattered a number by hiding a second tree). Classification below.

4. Ground truth: `target/wasm32-wasip2/debug/semio_s_plugin_space.wasm` (64,238,898 bytes, unstripped
   debug build, already on disk from a peer/earlier run) — `semio-s-plugin-space` wires
   `semio-framework-plugin` with `features = ["component-guest"]` identically to draw. `strings`
   search for demangled-prefix patterns:
   - `wit_bindgen` (specifically `wit_bindgen::rt::async_support::*`): **266 distinct mangled
     symbols present**, including `waitable_register`, `context_get`/`context_set`,
     `FutureState::callback`, `CallbackCode::encode`, `TaskCancelOnDrop` — genuine component-model
     async runtime code, not stubs.
   - `wit_parser`, `wit_component`, `wasmparser`, `wasm_encoder`, `wasm_metadata`, `prettyplease`,
     `indexmap` (`8indexmap`), `leb128fmt`, `macro_string`, `heck`/`prettyplease`: **zero**
     occurrences each.
   - Positive controls (known genuinely linked): `serde` 3,013 hits, `serde_json` 13,011 hits,
     `memchr` 53 hits — the search method finds real linked code when it's there.
   - This is byte-level confirmation of the `cargo tree -i` classification below, not just a
     structural inference.

## Classification of the 18

| crate | reachable via | linked into `.wasm`? |
|---|---|---|
| `wit-bindgen` | direct dep of `semio-framework-plugin` (not through any proc-macro) | ✅ **yes** — real runtime crate (`rt` module) |
| `wit-bindgen-rust-macro` | is itself `(proc-macro)` | ❌ host-only |
| `wit-bindgen-rust` | dep of `wit-bindgen-rust-macro` only | ❌ host-only |
| `wit-bindgen-core` | dep of `wit-bindgen-rust` / `wit-bindgen-rust-macro` only | ❌ host-only |
| `wit-component` | dep of `wit-bindgen-rust` only | ❌ host-only |
| `wit-parser` | dep of `wit-bindgen-core` / `wit-component` only | ❌ host-only |
| `wasm-encoder` | dep of `wasm-metadata` / `wit-component` only | ❌ host-only |
| `wasm-metadata` | dep of `wit-bindgen-rust` / `wit-component` only | ❌ host-only |
| `wasmparser` | dep of `wasm-encoder`/`wasm-metadata`/`wit-component`/`wit-parser` only | ❌ host-only |
| `leb128fmt` | dep of `wasm-encoder` only | ❌ host-only |
| `semver` | dep of `wasmparser`/`wit-parser` only | ❌ host-only |
| `anyhow` | dep of `wasm-metadata`/`wit-bindgen-core`/`wit-bindgen-rust`/`wit-bindgen-rust-macro`/`wit-component`/`wit-parser` — all host-only | ❌ host-only |
| `id-arena` | dep of `wit-parser` only | ❌ host-only |
| `indexmap` | dep of `wasm-metadata`/`wasmparser`/`wit-bindgen-rust`/`wit-component`/`wit-parser` — all host-only | ❌ host-only |
| `prettyplease` | dep of `wit-bindgen-rust`/`wit-bindgen-rust-macro` only | ❌ host-only |
| `heck` | dep of `wit-bindgen-core`/`wit-bindgen-rust` only | ❌ host-only |
| `macro-string` | dep of `wit-bindgen-rust-macro` only | ❌ host-only |
| `unicode-xid` | dep of `wit-parser` only | ❌ host-only |

17/18 have **no path into the graph that does not pass through a `(proc-macro)` node**. `anyhow`
and `indexmap` each have several parents, but every one of them is itself host-only, so the
transitive rule holds: a crate reachable only via host-only crates is host-only.

## Fix applied — the 19th crate

`wit-bindgen`'s own `Cargo.toml` (registry copy inspected directly,
`~/.cargo/registry/src/…/wit-bindgen-0.57.1/Cargo.toml`) declares `default = ["macros", "realloc",
"async", "std", "bitflags", "macro-string"]`. `semio-framework-plugin`'s manifest declared
`wit-bindgen = { version = "0.57.1", features = ["macros"] }` **without** `default-features =
false`, so all six defaults were active — including `bitflags`, a real (non-proc-macro) optional
dependency of `wit-bindgen` itself, gated by that default feature.

Checked whether it does anything: `world actor`'s WIT
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`, 1,314 lines) declares **no
`flags` type** — the only WIT construct wit-bindgen lowers to `bitflags::bitflags!`. Confirmed dead
in the actual `.wasm`: `strings` search for `bitflags` in `semio_s_plugin_space.wasm` finds exactly
**one** hit, a leftover `core::ptr::const_ptr::is_aligned_to` monomorphization tagged with the
`bitflags` crate hash — not a real function, not exercised.

Changed `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`:

```toml
wit-bindgen = { version = "0.57.1", default-features = false, features = ["macros", "realloc", "async", "std", "macro-string"] }
```

`async`/`std`/`realloc`/`macro-string` are kept — `async` is load-bearing (see below), `realloc`
provides the component's `cabi_realloc` allocator hook that every component needs regardless of
world shape, `macro-string` only forwards a feature to the proc-macro crate (host-only, harmless),
`std` matches the plugin SDK's existing `std` usage elsewhere. Only `bitflags` — proven unused —
was dropped.

**Verified**: `cargo tree -p semio-s-plugin-draw --target wasm32-wasip2 -i bitflags` now shows
`bitflags` reachable *only* through the `wasmparser`/`wit-component` host-only subtree — the direct
`wit-bindgen → bitflags` edge is gone. `cargo check --target wasm32-wasip2 -p semio-framework-plugin
--features component-guest` completed in 7m17s, 0 errors (187 pre-existing warnings, unrelated to
this change, same crate the ticket has been touching all day). This removes one real crate-name
from every plugin's `cargo tree --edges normal` scoreboard count (draw, animate, puzzle, flow,
trinity, space, stdio, and anything else on `semio-framework-plugin` with `component-guest`) at
zero functional risk — it was already dead-code-eliminated from the shipped bytes, so this is a
graph-hygiene fix, not a behavior change.

## Why the async runtime IS genuinely needed (so `wit-bindgen` itself cannot be trimmed further)

`🧬️schema/📜️.wit` line 1142: `poll: async func(events: list<event>, command-page:
option<command-ingress-page>, budget: budget) -> result<turn-result, plugin-error>;` — the
component's one real export is declared `async`, and `host-async` (line 1167) is an entire imported
interface of async functions (`storage-read`, etc.). This is deliberate, documented at
`🧬️schema/📜️.wit:1060-1066` as the "B1 world-collapse" — the guest genuinely runs on an
async-capable `wasmtime::Store` and needs the WASI-0.3-style async ABI (subtasks, waitables,
context get/set, callback codes). The 266 `wit_bindgen::rt::async_support::*` symbols found in the
built `.wasm` are the evidence this is exercised, not incidental.

## Recommendation for the ticket's scoreboard

The plan's own measurement script —
```bash
cargo tree -p <plugin> --target wasm32-wasip2 --edges normal --prefix none \
  | sed 's/ (\*)$//' | awk '{print $1}' | sort -u | grep -v '^semio-' | grep -v '^$' | wc -l
```
counts crate **names in the resolved graph**, which includes proc-macro-only subtrees. That is why
`verified-outcomes.md` already flagged `cargo tree --edges normal` as an *overstatement* for
`syn`/`quote`/`proc-macro2`/`serde_derive` — this cluster is the same phenomenon at 17-crate scale.

Two honest ways to report it, either is fine, but pick one and say so at the top of the scoreboard:

1. **Report the linked figure**, computed by `cargo tree -i` on each named crate and excluding any
   crate whose *every* path passes through a `(proc-macro)` node. For the wit-bindgen cluster that
   number is **1** (`wit-bindgen`), not 18.
2. **Report the raw graph count but annotate the split**, e.g. "40 total, of which N are
   compile-time-only (proc-macro subtree) and M are genuinely linked." This is more honest about
   toolchain footprint (build times, supply-chain surface) without conflating it with the runtime
   goal.

Either way: **do not count 18 crates against the "dependency-free at runtime" goal.** Only 1 does,
and it is the component-model ABI itself — removing it means the plugin stops being a WASI
component, which is explicitly out of bounds ("a plugin that no longer loads is not progress").

## The "vendor pre-generated bindings" idea — scoped, and not recommended

The ticket asked whether `semio_owned_alloc_v1`/`semio_owned_dealloc_v1`/`semio_owned_poll_v1`
(`🔌️plugin/🦀️.rs:31646-31660`, the first-party raw ABI trio) mean the WIT `world actor` surface
could be generated once and vendored as first-party code instead of invoking the generator in every
plugin's build.

Checked what that trio actually is: three `extern "C" #[unsafe(no_mangle)]` exports emitted by the
`plugin_exports!` macro, cfg-gated on `wasm32`/`p2`, that expose a flat JSON-over-pointer calling
convention (`owned_abi::{allocate, deallocate, take_json, return_json}`) **alongside** — not instead
of — the `wit_bindgen::generate!`-produced `world actor` component export. They coexist in the same
binary; the raw trio doesn't touch the canonical-ABI/resource/async machinery the real host↔guest
component contract needs.

Given that, vendoring the macro's *output* would only remove `wit-bindgen-rust-macro
(proc-macro)` from the Cargo graph — and proc-macros never link into the `.wasm` regardless, so this
buys **zero runtime bytes**. It would cost real things: hand-maintained generated Rust
(hundreds–low-thousands of lines per WIT world) that must be regenerated by hand on every `.wit`
change, with no compiler check that it's still in sync (`generate!` is exactly that check, run every
build). That is worse than what CLAUDE.md calls a migration shim — it is a permanent hand-copied
artifact. **Not recommended.** Scoped and rejected with reasons, per the brief's instruction not to
start a plugin-ABI rewrite this wave.

## Bottom line

- 1 of 18 named crates links into the guest: `wit-bindgen`, and it is the component-model ABI
  itself — legitimately un-removable, not a violation of the ticket's goal.
- 17 of 18 are host-only proc-macro build machinery, confirmed two independent ways (`cargo tree
  -i` full-output structural analysis, and `strings` search against an actual built `.wasm`).
- Found and fixed a 19th, unlisted crate (`bitflags`) that *was* genuinely linked but unused —
  trimmed via `default-features = false`, verified by both `cargo tree -i` and a clean
  `cargo check --target wasm32-wasip2` (7m17s, 0 errors).
- The ticket's own "draw-fsm 11" figure is not evidence on this question — `semio-s-plugin-draw-fsm`
  doesn't depend on `semio-framework-plugin` at all. `semio-s-plugin-draw` is the crate that matters
  here and was used instead.
- No rewrite of the plugin ABI attempted or recommended.
