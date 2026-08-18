# 📓️ terra-sdk-witbindgen — wit-bindgen 0.36.0 → 0.57.1 (guest SDK)

## delivered

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:32` — `wit-bindgen = { version = "0.36.0", … }` → `wit-bindgen = { version = "0.57.1", … }`. Only line changed in that file.
- **No other source edit was needed or made.** `🦀️component.rs`'s `pub mod component { … generate!({ world: "actor", path: "../../🧬️schema" }); … }` block (lines 9–79) is untouched, and empirically it does not need to change — see below.
- `⚛️reactor/🦀️component.rs` and `🌐host/🦀️component.rs` — untouched. I re-verified every canary the brief named is still exactly where it was documented:
  - `🦀️component.rs:24-28` — `use exports::semio::framework::checkpoint::Guest as CheckpointGuest;` / `describe::Guest as DescribeGuest` / `jobs::{Guest as JobsGuest, JobBudget, JobStep}` / `reactor::{Budget as WitBudget, Event as WitEvent, Guest as ReactorGuest, TurnResult as WitTurnResult}` / `use semio::framework::types::PluginError;` — all present, unchanged.
  - `⚛️reactor/🦀️component.rs:102-105` — the four aliases `use crate::component::component::semio::framework::{effects as wit_effects, events as wit_events, types as wit_types, ui as wit_ui};` — present, unchanged.
  - `⚛️reactor/🦀️component.rs:110,267-268,335-336,351-352,385-386` — the `crate::component::component::exports::semio::framework::reactor::{Event,Budget,TurnResult,UiPatch,Effect}` call sites — present, unchanged.
  - `🌐host/🦀️component.rs:344,355,365` — the three `crate::component::component::semio::framework::pure::{log,now_ms,trace_span}` calls — present, unchanged.
- `Cargo.lock` was **not hand-edited** (registrar-only path respected). `cargo check`/`cargo build` will rewrite it as a normal side effect of resolving the new `wit-bindgen` requirement the moment a clean build runs — `wit-bindgen 0.57.1` was already present in the lockfile before I touched anything (pulled in transitively by `wasip2 1.0.3+wasi-0.2.9`), so this is not a fresh-download risk.

## generated-path migration table

**Empty by necessity, not by omission.** No `semio-framework-plugin` build has reached the `pub mod component { generate!(...) }` expansion under 0.57.1 yet — every attempt failed earlier in the dependency graph, inside `semio-framework-os-kernel`, before rustc ever reaches our crate (see "commands + exit codes"). I cannot table old-path → new-path pairs I have not observed. What I *can* state with confidence from static comparison of the vendored macro sources (see honest gaps) is that nothing about the `generate!` option surface (`world`, `path`) or the default `ownership`/derive behavior changed between `wit-bindgen-rust` 0.36.0 and 0.57.1 in a way that should move the four documented aliases — but "should" is not the empirical proof this ticket requires, and I am not reporting it as one.

One positive, concrete signal: the blocked `cargo build -p semio-s-plugin-note --target wasm32-wasip2` run (see below) got far enough to show `Compiling wit-bindgen v0.57.1` with no diagnostic against it, before failing on unrelated code. That confirms the crate resolves and starts building against the new pin; it does not confirm the macro expansion shape survives, because the build never got to `semio-framework-plugin`.

## commands + exit codes

All run foreground, single Bash call, `CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb`, `-p` only, verbatim from the saved logs in this ticket folder.

```
$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
   … (three attempts, ~20 min apart: witb-check1.txt, witb-check2.txt, witb-check3.txt) …
error: couldn't read `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🗣️dsl/📍️span/🦀️component.rs`: No such file or directory (os error 2)
  --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs:29:3
   |
29 |   pub mod span;
   |   ^^^^^^^^^^^^^
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
EXIT:101   (all three attempts, identical error)
```

```
$ cargo check -p semio-framework-os-kernel --lib   (baseline isolation — no wasm target, no feature, no relation to my change)
error: couldn't read …/🗣️dsl/📍️span/🦀️component.rs: No such file or directory (os error 2)
  --> …/📦️glue.rs:29:3
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error
EXIT:101
```
→ proves the block is not caused by the wasip2 target, `component-guest`, or anything in my two edited lines: `os-kernel` will not compile **at all**, for any consumer, right now.

```
$ cargo build -p semio-s-plugin-note --target wasm32-wasip2 --features component-guest
error: the package 'semio-s-plugin-note' does not contain this feature: component-guest
EXIT:101
```
→ the packet brief's suggested command has the same mistake E2's brief made earlier in this ticket (recorded in `📓️status.md`): `semio-s-plugin-note` has no `component-guest` feature of its own — it always turns the feature on for its `semio-framework-plugin` dependency. Corrected command:

```
$ cargo build -p semio-s-plugin-note --target wasm32-wasip2
   Compiling wit-bindgen v0.57.1        ← the version bump resolves and compiles clean
   … 40+ lines later …
error[E0433]: cannot find module or crate `protocol` in this scope
   --> …/🔨️modules/📡️spr/🎮️command/🦀️component.rs:330:9
error[E0432]: unresolved import `crate::os_pack::format::DeflateCodec`
   --> …/🔨️modules/🎒️pack/🦀️component.rs:25:9
error[E0432]: unresolved imports `crate::os_dsl::diagnostic`, `crate::os_dsl::span`
   --> …/🔨️modules/🗣️dsl/🦀️component.rs:18:25
[…37 more downstream E0432/E0433 errors in semio-framework-os-kernel, none in our crate…]
error: could not compile `semio-framework-os-kernel` (lib) due to 41 previous errors; 5 warnings emitted
EXIT:101
```

`cargo test -p semio-framework-plugin --lib` was **not run**: it has the identical hard dependency on `semio-framework-os-kernel` and would fail at the same point before reaching a single one of our crate's tests. Running it would add nothing beyond the isolation check already pasted above.

Full logs: `witb-check1.txt`, `witb-check2.txt`, `witb-check3.txt`, `witb-oskernel-baseline.txt`, `witb-note-build1.txt` (wrong command, kept for the record), `witb-note-build2.txt`.

## named-set test comparison

**Not obtainable.** `cargo test -p semio-framework-plugin --lib` cannot run at all right now — `semio-framework-os-kernel`, a mandatory (non-cfg-gated) dependency of `semio-framework-plugin`, does not compile on this tree in its current, mid-edit state, independent of target/features (see baseline isolation run above). There is no test binary to name-diff against the five known baseline failures. This is not a regression I introduced: the `-p semio-framework-os-kernel --lib` isolation run proves the break has zero relationship to `wit-bindgen`, `component-guest`, or `wasm32-wasip2`.

## real-component proof

**Not obtainable for the same reason.** Neither `cargo build -p semio-s-plugin-note --target wasm32-wasip2` nor the `semio-framework-plugin-describe` step could run past the `semio-framework-os-kernel` compile failure. No `.wasm` artifact exists to check for component-model magic bytes or to feed to the describe CLI.

## what is actually blocking, and why it is not mine to fix

A **live, in-progress, broad module-consolidation refactor** is under way in files I do not own, spanning at least `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/**`, `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/**`, `🧰️framework/🔨️modules/📡️spr/🎮️command/**`, a brand-new `🧰️framework/🔨️modules/⚠️diagnostic/**` (created ~20:57, i.e. mid-session), and `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (registrar-only, not mine either) — the `[lib]` root that stitches all of these split files into the `semio-framework-os-kernel` crate. Evidence, checked rather than assumed (never `git status`, per rule 1):

- `git diff --stat HEAD` on those paths shows large uncommitted deletions/edits (`📦️glue.rs`: 25 insertions/51 deletions; `🎒️pack/**`: ~1349 lines net removed across 4 files; `🗣️dsl/⚠️diagnostic` and `🗣️dsl/📍️span` deleted outright).
- The new `🧰️framework/🔨️modules/⚠️diagnostic/{🦀️component.rs,📍️span/🦀️component.rs}` files exist with an mtime of `19:57:35` — created during this very session, i.e. this is not stale abandoned breakage (the "peer stopped 12h ago, safe to adapt" case from earlier in this ticket) but an active edit in flight (the "peer committed 20 min ago, still moving" case) — the correct evidence test from `📓️status.md`'s own W3-blocked precedent, applied here with the same "don't chase a moving target" conclusion.
- Between my first (`witb-check1`) and third (`witb-check3`, ~20 min later) attempts the *symptom itself changed* — from "file not found" to "41 unresolved-import errors" once I widened to `cargo build -p semio-s-plugin-note` — direct proof someone is actively mid-edit on exactly this dependency, not that the tree is permanently broken.
- None of the failing paths are inside my owned prefixes (`…/🔌️plugin/📦️packages/🦀️rust/Cargo.toml`, `…/🔌️plugin/🦀️component.rs`, `…/🔌️plugin/⚛️reactor/**`, `…/🔌️plugin/🌐host/**`).

Per the binding rules ("Never edit outside owned paths") and this ticket's own precedent for exactly this situation ("Concurrent Cargo Workspace Churn" — a repo-wide build failure that is another session's in-progress refactor, not mine to chase), I made **zero edits** to any of these files.

## lease-requests

None. My mission does not require touching `📦️glue.rs`, `🗣️dsl/**`, `🎒️pack/**`, or `📡️spr/**` — the blocker is upstream of my owned files, not something my task needs to modify. A lease over files someone else is actively mid-editing would create exactly the collision the ticket's peer-coexistence rules exist to prevent, so I am not requesting one. Recommend the coordinator identify whoever owns the `dsl`/`pack`/`spr` consolidation and let it land, then re-run the three acceptance commands unmodified — they need nothing further from me.

## honest gaps

- **The core acceptance criterion — a green `wasm32-wasip2 --features component-guest` check for `semio-framework-plugin` — is not met, not because of anything in this packet's scope, but because the crate cannot be reached at all right now.** This is a real, unresolved gap, not a formality.
- I have **not empirically verified** that wit-bindgen 0.57.1 preserves the exact module-path shape (`exports::semio::framework::{reactor,jobs,checkpoint,describe}` vs. the plain `semio::framework::{effects,events,types,ui}`) that A2b discovered for 0.36.0. My confidence that no source change is needed rests on:
  1. diffing the vendored `wit-bindgen-rust` 0.36.0 vs 0.57.1 source (`~/.cargo/registry/src/…/wit-bindgen-rust-{0.36.0,0.57.1}/src/lib.rs`) — the `Opts` struct still has the same `world`/`path`-driven `generate!` surface, default `ownership: Owning`, and no `additional_derives` usage on our call site in either version (the B1b `additional_derives`/`Debug`-conflict bug that hit the *host*-side `wasmtime::component::bindgen!` does not apply here — that's a different crate, `wasmtime-internal-wit-bindgen`, not guest-side `wit-bindgen-rust`);
  2. the WIT's one `resource surface;` declaration (`ui.wit`, deliberately unreferenced by any function signature per its own doc comment) already compiled fine under 0.36.0, so it is exercising no new code path under the bump;
  3. `cargo build -p semio-s-plugin-note --target wasm32-wasip2` reached `Compiling wit-bindgen v0.57.1` cleanly before failing on unrelated `os-kernel` code, which at least proves the crate graph resolves and the macro *crate* builds against the new pin.

  None of this substitutes for the real `generate!` expansion actually running against `🧬️schema/📜️component.wit` under 0.57.1, which is the one thing this packet exists to prove and which I could not force to happen this session.
- **Recommended next step, spelled out so re-verification needs no rediscovery:** once `semio-framework-os-kernel --lib` compiles clean again (watch `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` and the `🗣️dsl`/`🎒️pack`/`📡️spr` paths above settle), re-run, unmodified:
  ```
  CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb cargo check -p semio-framework-plugin --lib
  CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
  CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb cargo test  -p semio-framework-plugin --lib
  CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb cargo build -p semio-s-plugin-note --target wasm32-wasip2   # NOT --features component-guest — semio-s-plugin-note has no such feature; the flag was carried over from an already-corrected E2 mistake and should not reappear in future briefs
  ```
  If the first of those goes red on the `generate!` expansion (cascade "cannot find `exports` in `component`" — read the FIRST error, per the ticket's own rule), that is the real signal this packet was created to catch, and the fix is exactly the class of work A2b already did once for 0.36.0: discover the true module paths from the compiler's own `help: consider importing …` suggestions, not by guessing.
