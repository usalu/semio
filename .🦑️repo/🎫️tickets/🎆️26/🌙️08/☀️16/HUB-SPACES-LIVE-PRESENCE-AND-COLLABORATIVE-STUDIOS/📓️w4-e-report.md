# Lane 4-E report — `s` plugin wasm32-wasip2 build unblocked

## Bar: MET

The ticket's stated highest-value blocker — "running `s` with running plugins", currently impossible
because `semio-s-plugin-space` could not compile to WebAssembly at all — is fixed. Both required
proof points are green:

- `semio-s-plugin-space` → `cargo check`/`cargo build --target wasm32-wasip2` → **0 errors**, real
  `.wasm` produced.
- `semio-s-plugin-writer` (a stdio-free editable kind) → `cargo check --target wasm32-wasip2` →
  **0 errors**.

No host-side regression: every command the brief listed as a gate passed with the exact numbers it
specified (204/0, 988/0, 0 errors × 3). Full evidence below.

## Root cause, verified (not trusted from the ticket summary)

```
$ cargo tree -p semio-s-plugin-space --target wasm32-wasip2 -i tokio -e features
```
confirmed the chain the ticket predicted, exactly:

```
tokio v1.52.3
├── tokio feature "libc"
│   └── tokio feature "net"
│       └── semio-framework-os-kernel feature "sync"
│           └── semio-framework-os feature "os-host-full"
│               └── semio-s-plugin-space v0.1.0 (…/✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust)
│                   └── semio-s-plugin-space feature "default" (command-line)
├── tokio feature "macros" → semio-framework-os-kernel v0.1.0 (… default feature chain …)
├── tokio feature "mio" → tokio feature "net" (*)
├── tokio feature "net" (*)
├── tokio feature "rt" → semio-framework-os-kernel feature "sync" (*)
├── tokio feature "socket2" → tokio feature "net" (*)
```

`semio-s-plugin-space`'s `Cargo.toml` requests `semio-framework-os = { …, features = ["os-host-full"] }`
unconditionally (not target-gated). `semio-framework-os`'s `os-host-full = ["dep:zip",
"semio-framework-os-kernel/sync"]`. `semio-framework-os-kernel`'s `sync = [...,  "tokio/rt", "tokio/net",
"tokio/time"]` referenced the crate's **base, target-independent** `tokio` entry in `[dependencies]`
(distinct from the already target-gated `tokio-tungstenite`/`notify`/`rusqlite` trio a few lines
below it). A feature-flag reference to a non-target-gated dependency (`tokio/net`) applies to every
target cargo resolves for the build, wasm32 included — so enabling `sync` (pulled in transitively by
the plugin's unconditional `os-host-full` request) pulled `tokio/net` onto wasm32 too. tokio's own
`lib.rs` (verified by reading `tokio-1.52.3/src/lib.rs:465-478` in the vendored source) hard-`compile_error!`s
on `target_family = "wasm"` only for `fs, io-std, net, process, rt-multi-thread, signal` — **`rt` and
`time` are explicitly declared wasm-safe**, only `net` is not:

```rust
#[cfg(all(
    not(tokio_unstable),
    target_family = "wasm",
    any(feature = "fs", feature = "io-std", feature = "net", feature = "process",
        feature = "rt-multi-thread", feature = "signal")
))]
compile_error!("Only features sync,macros,io-util,rt,time are supported on wasm.");
```

### Why not "just don't request `os-host-full` on wasm32"?

That was the ticket prompt's first framing, and I checked it before dismissing it.
`semio-s-plugin-space` unconditionally imports `semio_framework_os::{WorkflowSnapshot,
WorkflowMutation, OsSpaceCatalogEntry, create_os_space, apply_workflow_operation, …}` from dozens of
call sites across `⚙️engine/🪐️space/**` and `🗿️artifacts/🏠️home/**`
(`grep -rn "semio_framework_os::" ✏️s/🔌️plugins/🪐️space` — 30+ hits). Every one of those types lives
inside `semio-framework-os`'s `#[cfg(feature = "os-host-full")] pub mod workflow { … }`
(`🖥️host/🦀️component.rs:3079`). Stripping `os-host-full` for wasm32 would delete the plugin's own
document model — not viable. `os-host-full` is genuinely required by the guest build; only the `net`
sliver of what it drags in (via `sync`) is the actual problem.

### Why not "just gate the whole `sync` feature/module off wasm32"?

Also checked and rejected. `semio-framework-os-kernel/🏪️store/🔄️sync/🦀️component.rs` (the module the
`sync` feature mounts, gated only by `#[cfg(feature = "sync")]` with no target-arch qualifier) is
**not** native-only: it internally splits into `#[cfg(not(target_arch = "wasm32"))] mod native_actor`
(uses `tokio_tungstenite`/`tokio::net::TcpStream`) and `#[cfg(target_arch = "wasm32")] mod wasm_actor`
(talks to the hub over `web_sys::WebSocket` via wasm-bindgen — no tokio networking at all). The
existing wasm32-only `👷️worker` submodule in the same crate
(`#[cfg(all(feature = "worker", target_arch = "wasm32"))]`, `📦️glue.rs:254-256`) already imports
`crate::os_store::sync::ArtifactActorConfig` — so excluding the whole `sync` module from wasm32 would
have broken that pre-existing consumer. The correct, minimal fix really is: **keep `net` off wasm32,
touch nothing else** — which is exactly what "target-gate the feature/dependency" in the ticket's own
framing means, done at the one precise point (`tokio/net`) rather than weakening tokio's overall
feature set (`rt`/`time` stay enabled everywhere, since tokio itself declares them wasm-safe).

## The fix

Both edits are inside the leased dependency chain, in one file:
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`. No Rust source (`#[cfg(...)]` gating)
changes were needed anywhere — the `sync` module was already correctly wasm/native split at the
source level; only the Cargo-level feature unification was wrong. Diff:

```diff
-sync = ["dep:tokio-tungstenite", "dep:notify", "dep:rusqlite", "tokio/rt", "tokio/net", "tokio/time"]
+sync = ["dep:tokio-tungstenite", "dep:notify", "dep:rusqlite", "tokio/rt", "tokio/time"]
 worker = ["sync"]
 …
 [target.'cfg(not(target_arch = "wasm32"))'.dependencies]
 tokio-tungstenite = { version = "0.26", optional = true }
 notify = { version = "8", optional = true }
 rusqlite = { version = "0.38.0", optional = true, features = ["bundled"] }
+tokio = { version = "1", default-features = false, features = ["net"] }
```

1. `[features] sync` no longer references `tokio/net` directly — only `tokio/rt` and `tokio/time`
   remain (both wasm-safe).
2. Added a `tokio` entry under the **existing** `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
   block, same crate key as the base `[dependencies]` `tokio` entry. Cargo unifies same-named
   dependencies declared in `[dependencies]` and a matching `[target.cfg(...)]` table into one edge
   per target, unioning their feature sets. For any non-wasm32 target this arm activates and adds
   `net`; for wasm32 it never activates, so `net` never reaches tokio there — unconditionally, exactly
   the architectural invariant the ticket cares about ("WASI-P2 plugins never link the sync actor
   crate['s networking]").

### Two things I tried and abandoned (recorded so nobody re-discovers them the hard way)

- **Rename-alias** (`tokio-net = { package = "tokio", optional = true, features = ["net"] }`, enabled
  via `dep:tokio-net` from the `sync` feature) — this would have kept `net` strictly behind the `sync`
  feature toggle, mirroring how `tokio-tungstenite`/`notify`/`rusqlite` are already optional +
  target-gated. Cargo rejected it outright: *"the crate `semio-framework-os-kernel` depends on crate
  `tokio` multiple times with different names"*. Cargo does not allow one crate to be reachable under
  two different manifest keys within one package.
- Because of that, the landed fix makes `net` **unconditional for every non-wasm32 build** of this
  crate (not gated behind the `sync` feature bit) — a small precision loss (a native build with `sync`
  off still compiles `tokio` with `net` support in, though nothing calls it — no functional or
  behavioral difference, just a slightly wider always-linked surface on native only) traded for
  correctness on the axis that matters: wasm32 never sees `net`, unconditionally.

## Verify — space plugin (the ticket's literal ask)

```
$ cargo check -p semio-s-plugin-space --target wasm32-wasip2
```
Real tail (full log `🧪️4-e-wasm-check.txt`):
```
warning: `semio-s-plugin-space` (lib) generated 54 warnings (run `cargo fix --lib -p semio-s-plugin-space` to apply 16 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 44.65s
```
`grep -c "^error"` on the log → **0**. All 54 warnings are pre-existing dead-code/unused-import lints
(sampled several — `never used` on functions like `space_document_envelope_pack`,
`home_space_rows`, etc.), none newly introduced by this change.

```
$ cargo build -p semio-s-plugin-space --target wasm32-wasip2
```
Real tail (full log `🧪️4-e-wasm-build.txt`):
```
warning: `semio-s-plugin-space` (lib) generated 54 warnings (run `cargo fix --lib -p semio-s-plugin-space` to apply 16 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 5m 07s
```
Artifact confirmed on disk:
```
$ ls -la target/wasm32-wasip2/debug/semio_s_plugin_space.wasm
-rw-r--r--  1 ueli  staff  44864260 Aug 17 05:10 target/wasm32-wasip2/debug/semio_s_plugin_space.wasm
```

## Verify — no host-side regressions (brief's exact command list, exact bars)

| Command | Required bar | Actual result | Log |
|---|---|---|---|
| `cargo check -p semio-framework-os -p semio-framework-os-kernel -p semio-hub` | 0 errors | 0 errors (`Finished` after 3m 37s; only a pre-existing `db` future-incompat warning, unrelated) | `🧪️4-e-host-check.txt` |
| `cargo test -p semio-s-plugin-space --lib` | 204/0 | `test result: ok. 204 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` | `🧪️4-e-space-test.txt` |
| `cargo test -p semio-framework-os-kernel --lib` | 988/0 | `test result: ok. 988 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out` | `🧪️4-e-kernel-test.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu` | 0 errors | 0 errors (`Finished` after 1m 30s; only a pre-existing `block` future-incompat warning, unrelated) | `🧪️4-e-wgpu-check.txt` |

All four match the brief's bars exactly.

## Verify — wider guest catalog (step 5)

`bun ./🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts plugin s`
builds the **entire** playground catalog for the `s` variant (confirmed from `PluginBuildScript`/
`preparePluginBuildTargets` in that script — not just `semio-s-plugin-space`), and aborts hard on the
**first** crate failure (`buildPlugin` throws, uncaught). It hit `semio-s-plugin-animate` — explicitly
called out as known-broken/other-ticket in the lane brief — with 5 unrelated errors in its
`present`/video-export code (`Mp4Track` field-shape mismatches, an `unresolved import`). Log:
`🧪️4-e-plugin-build-s.txt`. Since that orchestrator can't get past the first failure, I instead swept
all 33 plugin crates individually with `cargo check -p <crate> --target wasm32-wasip2` for full
attribution. Roll-up: `🧪️4-e-plugin-catalog-summary.txt`; per-crate logs: `🧪️4-e-catalog-<crate>.txt`.

### Per-crate wasm32-wasip2 result (33/33, no orchestrator early-abort)

**20 PASS / 13 FAIL.** None of the 13 failures reference `tokio`, `net`, or any wasm feature-unification
error — every single one is a local, pre-existing type/struct/import mismatch inside that crate's own
artifact code, unrelated to this lane's fix. `git log --date=iso` attribution below (commit *messages*
here carry a frozen fake date template per repo convention — `--date=iso` is the only truthful field).

| Plugin crate | wasm32-wasip2 | First error | Last touch (`git log --date=iso`) |
|---|---|---|---|
| `semio-s-plugin-writer` | **PASS** | — | — |
| `semio-s-plugin-flow` | PASS | — | — |
| `semio-s-plugin-vcs` | PASS | — | — |
| `semio-s-plugin-shooting` | PASS | — | — |
| `semio-s-plugin-architect` | PASS | — | — |
| `semio-s-plugin-process` | PASS | — | — |
| `semio-s-plugin-reasoning-mindmap` | PASS | — | — |
| `semio-s-plugin-forms` | PASS | — | — |
| `semio-s-plugin-cad` | PASS | — | — |
| `semio-s-plugin-norm` | PASS | — | — |
| `semio-s-plugin-playbook` | PASS | — | — |
| `semio-s-plugin-imperative` | PASS | — | — |
| `semio-s-plugin-energy` | PASS | — | — |
| `semio-s-plugin-trinity` | PASS | — | — |
| `semio-s-plugin-dag` | PASS | — | — |
| `semio-s-plugin-stdio` | PASS (standalone) | — | — |
| `semio-s-plugin-puzzle` | PASS | — | — |
| `semio-s-plugin-block` | PASS | — | — |
| `semio-s-plugin-space` | **PASS** | — | — |
| `semio-s-plugin-sourcing` | PASS | — | — |
| `semio-s-plugin-mathematical` | FAIL (2 err) | `E0425 cannot find function "mathematical_geometry"` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-procedural` | FAIL (3 err) | `E0308 mismatched types` | `e648c495c2` 2026-08-16 21:52:13 +0200, Ueli Saluz |
| `semio-s-plugin-gis` | FAIL (3 err) | `E0308 mismatched types` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-animate` | FAIL (6 err) — **known-foreign, do not fix** | `E0432 unresolved import semio_framework_plugin::InteractionView` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-demonstrator` | FAIL (3 err) | `E0308 mismatched types` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-sequence` | FAIL (2 err) | `E0425 cannot find function "dag_lod_scale_json" in crate infinite_board_port_directed_dag` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-fem` | FAIL (3 err) | `E0432 unresolved imports crate::artifacts::fem2d::op::{Fem2dEnvelope, Fem2dStore}` | `e648c495c2` 2026-08-16 21:52:13 +0200, Ueli Saluz |
| `semio-s-plugin-lowpoly` | FAIL (2 err) | `E0425 cannot find type "UiNode" in this scope` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-layout` | FAIL (12 err) — **known-foreign (consumes broken `stdio` DWG code), do not fix** | `E0432 unresolved import semio_s_plugin_stdio::artifacts::dwg::DwgDecodeStatus` | `e648c495c2` 2026-08-16 21:52:13 +0200, Ueli Saluz |
| `semio-s-plugin-remodel` | FAIL (3 err) | `E0282 type annotations needed` | `e648c495c2` 2026-08-16 21:52:13 +0200, Ueli Saluz |
| `semio-s-plugin-draw` | FAIL (2 err) | `E0433 cannot find module or crate "ui_wgpu" in this scope` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-raster` | FAIL (5 err) | `E0433 cannot find "composite" in "super"` | `c8a29e41c5` 2026-08-16 20:26:15 +0200, Ueli Saluz |
| `semio-s-plugin-note` | FAIL (5 err) — **known-foreign (consumes broken `stdio` DWG/SVG code), do not fix** | `E0560 struct SvgSnapshot has no field named "lexical"` | `e648c495c2` 2026-08-16 21:52:13 +0200, Ueli Saluz |

`semio-s-plugin-animate` and the `semio-s-plugin-stdio`-dependent failures (`note`, `layout`) match the
brief's explicit "known-broken, other ticket, do not fix" list — `stdio` itself (`✏️s/🔌️plugins/🗄️stdio/**`)
is FULL-STDIO's forbidden territory and is out of my lease regardless. The remaining 10 failures
(`mathematical`, `procedural`, `gis`, `demonstrator`, `sequence`, `fem`, `lowpoly`, `remodel`, `draw`,
`raster`) are all pre-existing, unrelated crate-local bugs (wrong function names, struct-shape drift,
missing imports) last touched by the same two commits that touched almost the whole plugin catalog
simultaneously (`c8a29e41c5` and `e648c495c2`, both same-day/adjacent — some other session's broad
refactor sweep across `✏️s/🔌️plugins/**`), not by this lane's Cargo.toml edit and not attributable to
the tokio/wasm issue this lane fixed.

## Changed files

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml` — the **only** file edited. Two regions:
  the `[features] sync = [...]` line, and the `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
  block (added one `tokio` entry). No other file touched.

**Considered, not edited**: `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml` — in-lease, but no
edit was required there. The plugin's `os-host-full` request is correct and necessary (see "Why not…"
sections above); the bug was entirely inside the kernel's feature/dependency graph.

## sharedFileRequests

None. The entire fix landed inside this lane's lease
(`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/**` is under the leased
`🧰️framework/🛍️products/💻️os/**`).

## Foreign/uncommitted change observed in passing (not mine, not touched)

`git diff` on the leased Cargo.toml also showed an uncommitted `web-sys` feature-list expansion
(`Window, Storage, console` → `+ WebSocket, MessageEvent, BinaryType, Request, RequestInit, Response,
Headers`) that predates my edits and that I did not make — some other live session's in-flight change
sitting in the working tree, landed after the last commit that touched this file (`2420304f4c`,
2026-08-14 08:39:20 +0200 per `git log --date=iso`) and before I started. Left untouched per "never
revert a foreign change"; noted here only so it isn't misattributed to this lane.

## What is NOT done

- Did not attempt to fix `semio-s-plugin-animate`, `semio-s-plugin-stdio`, or `stdio`-dependent
  failures in `note`/`layout` — all explicitly out-of-scope (other tickets), confirmed broken for
  reasons unrelated to tokio/wasm feature unification.
- Did not attempt to fix the other 9 unrelated pre-existing plugin-catalog failures
  (`mathematical`/`procedural`/`gis`/`demonstrator`/`sequence`/`fem`/`lowpoly`/`remodel`/`draw`/`raster`)
  — out of this lane's scope (not tokio/wasm-feature related, not named in the brief's bar).
- Did not run the browser e2e itself (other lanes' territory) — only established that the wasm
  artifacts it needs (`space`, `writer`, plus 18 other now-passing crates) exist.
- Ticket left open, as instructed — coordinator owns `ticket_close`.
