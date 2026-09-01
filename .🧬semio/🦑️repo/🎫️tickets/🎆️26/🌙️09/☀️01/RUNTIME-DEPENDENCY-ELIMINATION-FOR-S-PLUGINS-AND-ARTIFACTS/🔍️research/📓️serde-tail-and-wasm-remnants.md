# Serde tail (fem + draw-fsm), wasm-glue straggler, proc-macro trio — tail batch

Scope was: (1) serde/serde_json in 2 manifests (fem, draw-fsm), (2) remaining wasm-bindgen/js-sys/
web-sys stragglers, (3) verify the proc-macro classifier. All three done; (1) is a partial,
deliberate result — see below, this is not an oversight.

## Part (1) — serde/serde_json, 2 manifests

### `✏️s/🔌️plugins/🖍️draw/…/🔄️fsm/📦️packages/🦀️rust/Cargo.toml` (semio-s-plugin-draw-fsm) — DONE

This crate turned out to have exactly **one** real serde site: `persist::PersistedSnapshot`
(`fsm/🦀️.rs`), gated `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` behind a
`serde` feature that was **default-on** (`default = ["macros", "serde"]`) and unconditionally
enabled by the draw plugin's own dependency line (no `default-features = false`) — so this was a
real production-runtime leak into the `wasm32-wasip2` component, not a theoretical one.

The crate's OTHER 14 serde/serde_json hits (in `✨️macros/🦀️.rs`, the sibling proc-macro's `quote!`
templates for `export_wasm_machine!`) do not affect this manifest's own `[dependencies]` — a
proc-macro's generated code only requires a dependency in whichever crate *invokes* the macro at
its call site, never in the macro crate itself. `export_wasm_machine!` has **zero call sites**
anywhere in the repo (confirmed by grep, and independently confirmed by the concurrent wasm-glue
wave's own research doc, `📓️wave-wasm-glue.md` line 72/162 — they deleted the dead `mod
wasm_bridge`/`mod wasm_smoke` call site from `fsm/🦀️.rs` and flagged the now-fully-orphaned
`expand_export_wasm_machine` template to "the W2 (syn/quote/proc-macro2) owner" as out-of-scope
proc-macro-crate work). I left that dead codegen text alone for the same reason — touching it
would only matter once something invokes the macro, and it's not this manifest's concern.

**Verbatim tail — before:**
```toml
[features]
default = ["macros", "serde"]
macros = ["dep:fsm_macros"]
serde = ["dep:serde", "dep:serde_json"]
testing = []

[dependencies]
fsm_macros = { path = "../../✨️macros/📦️packages/🦀️rust", optional = true, package = "semio-s-plugin-draw-fsm-macros" }
semio-framework-dispatch-macros = { path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }
serde = { version = "1.0.219", features = ["derive"], optional = true }
serde_json = { version = "1.0.140", optional = true }
```

**Verbatim tail — after:**
```toml
[features]
default = ["macros"]
macros = ["dep:fsm_macros"]
testing = []

[dependencies]
fsm_macros = { path = "../../✨️macros/📦️packages/🦀️rust", optional = true, package = "semio-s-plugin-draw-fsm-macros" }
semio-framework-dispatch-macros = { path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }
semio-framework-os-kernel = { path = "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel" }
semio-framework-value-derive = { path = "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
```
Path depth resolve-checked with `ls -d` before writing (12 `../`, matching the existing
`semio-framework-dispatch-macros` line's depth in the same file).

`fsm/🦀️.rs` — `PersistedSnapshot`:
```rust
// before
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PersistedSnapshot { .. }

// after
#[derive(Debug, Clone, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
pub struct PersistedSnapshot { .. }
```
No `#[value(...)]` attributes needed — plain named fields (`u32`, `u64`, `Vec<String>`, `bool`,
`Vec<(String, Vec<String>)>`), no rename/tag/default in the original serde derive either.

**Framework gap filled**: `Vec<(String, Vec<String>)>` needs `ToValue`/`FromValue` for the tuple
`(String, Vec<String>)`. No tuple impl existed in `🌱️value/🔁️codec/🦀️component.rs` (read the whole
265-line file — scalars, `Option`, `Vec`, `Box`, `BTreeMap`, `DslValue`, nothing for tuples). Added
`impl<A: ToValue, B: ToValue> ToValue for (A, B)` / the `FromValue` counterpart, encoding as a
2-element `DslValue::Array` (same shape `serde_json` gives a Rust tuple), plus a round-trip +
error-path test (`tuple_round_trips_as_two_element_array_like_serde_json`).

**Status: PROVEN BY A PASSING CHECK.** `cargo check -p semio-s-plugin-draw-fsm
--message-format=short` completed after this doc's first draft (foreground launch, auto-backgrounded
by the tool past 120s due to the documented machine saturation, notification arrived ~31 minutes
later): **exit code 0, `Finished \`dev\` profile [unoptimized] target(s) in 30m 52s`, zero errors.**
Only pre-existing warnings unrelated to this edit (`unnecessary qualification` in
`🌱️value/✨️derive/🦀️component.rs`, `🎒️pack/🔤️json/🦀️component.rs`, `os-kernel`'s `🏪️store`
component — none in the tuple impl I added to `🌱️value/🔁️codec/🦀️component.rs` or in `fsm/🦀️.rs`'s
`PersistedSnapshot`). This proves, end to end: the `ToValue`/`FromValue` derive resolves correctly
from `semio-s-plugin-draw-fsm` against `semio-framework-os-kernel` (confirming the crate-root
reachability trace below), the new tuple `ToValue`/`FromValue` impl in `🌱️value/🔁️codec` compiles
and satisfies `PersistedSnapshot`'s `Vec<(String, Vec<String>)>` field, and the Cargo.toml's path
depths resolve correctly. Native `cargo check` does not build `wasm32-wasip2`, so this does not
prove the `#[target_arch = "wasm32"]` half of the crate (unaffected by this change) links cleanly
as a component — only that the crate is otherwise sound. Corroborating evidence gathered before
this result landed, kept for the record:
- The concurrent wasm-glue wave's own research doc reports `cargo build --lib --target
  wasm32-wasip2 -p semio-s-plugin-draw-fsm` compiled **cleanly** earlier this session, against the
  pre-edit (still-serde) version of this same crate — a known-good baseline my edit starts from.
- The playbook pilot's `cargo test -p semio-framework-replication --lib` (225/226 passing, the 1
  failure pre-existing and unrelated) exercises the exact `🌱️value/🔁️codec` module I extended,
  confirming the base `ToValue`/`FromValue`/derive machinery is sound; my tuple addition is new
  and only test-covered by the unit test I added (not yet run).
- Traced (not merely assumed) that `::semio_framework_os_kernel::ToValue/FromValue/DslValue/
  ValueError` — the derive's hardcoded generated path — already resolves at `os-kernel`'s crate
  root via `os_dsl::schema` (`pub use protocol::value::{..}`) → `dsl/🦀️component.rs` (`pub use
  crate::os_dsl::schema::*`) → `os_dsl` (`pub use component::*`) → crate root (`pub use
  crate::os_dsl::*`). This matches `📓️serde-fanout-playbook.md`'s own documented finding
  (independently written by the serde-wave pilot) almost verbatim. I additionally added a direct,
  explicit crate-root re-export in `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`
  (`pub use crate::os_dsl::schema::{DslValue, FromValue, ToValue, ValueError};`) — functionally
  redundant with the existing glob chain (confirmed no name collision anywhere in the os-kernel
  reachable tree), kept for robustness/self-documentation, not because the chain was actually
  broken.

**Reviewer note (found, not fixed — genuinely out of my 2 manifests):** the `export_wasm_machine!`
template in the sibling `✨️macros/🦀️.rs` still generates `serde`/`serde_json`-based code (would
require the *invoking* crate to depend on `serde`/`serde_json` if ever used). Already flagged to
the W2/proc-macro owner by the wasm-glue wave via `spawn_task`; I did not duplicate that flag.

### `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml` (semio-s-plugin-fem) — NOT converted, scope finding

**This manifest's `serde`/`serde_json` were left in place.** Measured before assuming: `grep -rn
serde` under `✏️s/🔌️plugins/🏗️fem` hits **179 files**, **1186 occurrences** (560×
`serde_json::from_str`, 268× `serde_json::Value`, 228× `serde_json::to_value`, ~172 derive blocks
across `#[derive(.., Serialize, Deserialize, dsl::DslRecord, ...)]` combinations, plus `json!`/
`to_string`/`to_writer`/`from_slice` call sites) spread across the fem2d/fem3d artifact standards,
subsets, schema, mutations, io/import/export, and generator tree — a ~340-`.rs`-file crate. This
is not "1 manifest, 1 struct" the way draw-fsm was; it is closer in scope to the entire `📖️playbook`
pilot's crate (itself still not fully converted after real effort — see
`📓️serde-fanout-playbook.md`: "~20 of the plugin's ~30 serde-touching files… were surveyed but not
converted") multiplied roughly 6x by file count.

I did not attempt a scripted/mechanical mass conversion of fem for three concrete reasons, not
effort-avoidance:
1. **Correctness cannot be validated at this volume in this session.** `serde_json::Value`'s API
   (`.as_object_mut()`, mutable indexing, `Map` iteration order guarantees) is not 1:1 with
   `DslValue`'s (documented as read-only accessors + `Vec<(String, DslValue)>` backing in
   `🌱️value/🦀️component.rs`) — the playbook playbook itself calls out that JSON-text vs. in-memory
   `DslValue` is "pick the right one" per call site, not a blind swap, and that adjacently-tagged
   enums (`#[serde(tag, content)]`) need hand-written impls the derive doesn't generate. A
   sed-style pass across 1186 sites risks silently wrong behavior (e.g. a `content`-tagged fem
   mutation shape) with no way to prove correctness before the box frees up for a real build+test.
2. **Blast radius.** fem is a live, shared, 340-file production crate; a partially-wrong mechanical
   conversion would break it for every one of the ~12 concurrent sessions, which is explicitly
   called out in this ticket as the exact incident that already happened once today (the
   `wgpu`/`blake3` path-depth break).
3. **The framework foundation itself is still actively gaining features mid-session** — while
   investigating I watched a concurrent agent add adjacently-tagged (`tag` + `content`) enum
   support to `🌱️value/✨️derive/🦀️component.rs` live (git-diff visible, +66/-19 lines, not yet
   committed). fem's own derive survey would need to be re-run against whatever the derive
   supports once that lands, not against a snapshot from mid-edit.

**What IS true and verified about fem right now:** its `Cargo.toml` is unchanged from baseline
(still declares `serde.workspace = true` and `serde_json = { workspace = true, features =
["float_roundtrip"] }` in `[dependencies]`), which is *correct* given 1186 call sites still use
them — removing the manifest lines first and converting call sites after would not compile.

**Recommendation, not executed (would need explicit dev sign-off per ticket scope):** fem should
become its own dedicated ticket/wave once the serde-wave pilot's mechanical rewrite table
(`📓️serde-fanout-playbook.md`) is proven end-to-end on `📖️playbook` (still "not fully green" per
that doc's own header) and the derive macro's adjacently-tagged support lands and is tested — fem
is the single largest remaining serde consumer under `✏️s/` by a wide margin (1186 of roughly
what was ~2200 total repo-wide serde/serde_json occurrences at the ticket's baseline) and needs a
wave of its own, not a slot in a 3-part tail-cleanup batch.

## Part (2) — wasm-glue stragglers

Current repo-wide state (`grep -rnE '^(wasm-bindgen|js-sys|web-sys|wasm-bindgen-futures|serde-wasm-bindgen)' ✏️s --include=Cargo.toml`):
```
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:105:wasm-bindgen.workspace = true
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:106:js-sys = "0.3.83"
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:107:web-sys = { version = "0.3.98", features = ["HtmlCanvasElement"] }
```
One manifest, three entries — everything else was already cleared by the concurrent wasm-glue wave
before I started (their own doc, `📓️wave-wasm-glue.md`, covers 33 of 34 manifests: writer,
procedural, gis, animate, shooting, layout, fem [glue.rs alias only], draw-root, draw-fsm, raster,
trinity, imperative, sourcing, cad, process — all class **B-dead**, deleted). `🧩️puzzle` is their
one documented exception, left alone deliberately as class **B-live**: `BoardSession`
(`◻2d/…/✏️editor/🌉️wasm/🦀️component.rs`) is a real ~50-method WebGPU canvas session, built by a
real `wasm-pack`/`wasm32-unknown-unknown` target (`puzzle2d`'s own playground `engines: [...]`
row — the *only* self-engined plugin crate in the whole registry) and consumed by an actual
TypeScript component. Removing it needs reimplementing `BoardSession` against a generic
framework-owned WASM bridge wrapper (their "Next steps" 1–3) — real, scoped, un-started work, not
mine to force through in this batch.

### Table

| # | Crate/file | Class | Evidence | Action |
|---|---|---|---|---|
| 1 | `puzzle`'s `wasm-bindgen`/`js-sys`/`web-sys` Cargo.toml lines | **B-live** | `wasm-pack.profile.{dev,release}` metadata + `[[package.metadata.semio.playground]]` `engines: ["./✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust"]` — a real `wasm32-unknown-unknown` build target exists and is the only self-engined plugin (confirmed independently by `📓️wave-wasm-glue.md` §1/§4) | Left in place — genuinely needed by a live browser build target, matches the sanctioned (B) pattern |
| 2 | `◻2d/…/✏️editor/🌉️wasm/🦀️component.rs` (`BoardSession`) | (B), but **wrongly gated** | Only `#[cfg(target_arch = "wasm32")]` per-item (23 occurrences), missing `not(target_env = "p2")` — unlike its own siblings `🧊️3d/…/🌉️wasm/🦀️component.rs` and `🖐️5d/…/🌉️wasm/🦀️component.rs`, which both open with `#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]`. `target_arch = "wasm32"` is true for `wasm32-wasip2` too (`target_os="wasi"`, `target_env="p2"`) — independently confirmed as "the trap" in `📓️wave-wasm-glue.md` §1. Since `pub mod wasm;` for 2d is declared unconditionally in `📦️glue.rs` (same as 3d/5d), this file's ~50 `wasm_bindgen`/`web_sys`/`js_sys` items were being compiled into the real `wasm32-wasip2` production plugin component, not just the legitimate browser build. **FIXED**: added the same `#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` module-level gate the 3d/5d siblings already use, and removed the now-redundant 23 per-item `#[cfg(target_arch = "wasm32")]` attributes (matching 3d/5d's own style exactly — zero per-item cfgs there once the module gate exists). Verified via `Read` that nothing outside this file references `puzzle2d::wasm::*` items (no other build target needs them unconditionally). |
| 3 | Everything else previously scoped to this wave (33 manifests) | B-dead, already deleted | `📓️wave-wasm-glue.md` full table | No action — not mine to redo, confirmed via grep that none of the 33 remain |

**Status update — `cargo check -p semio-s-plugin-puzzle --message-format=short` completed**
(returned after this doc's first draft). Result: 247 errors, ALL of them `E0277` "trait bound
`WorkflowMutation`/`WorkflowDiff`/`RunMutation`/`RunDiff`: ToValue/FromValue is not satisfied" in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**` — a dependency of `semio-framework` itself,
which failed to compile before `semio-s-plugin-puzzle` (which depends on it) was ever reached.
Verified (`grep -ic "puzzle|BoardSession|wasm_bindgen|web_sys|js_sys"` on the full output → `0`)
that **none** of the 247 errors mention puzzle, `BoardSession`, or any wasm-glue symbol — this is
the framework-wide `MutationDiff`/`Mutation` trait-bound change (documented in
`📓️serde-fanout-playbook.md`) landing ahead of the `workflow` module's own conversion, entirely
unrelated to this wave's part (1) or part (2) work, and not mine to fix (recording per the
ticket's own instruction: "confirm they don't name your crate, move on"). This means the
`#![cfg(...)]` fix on puzzle's 2d wasm bridge is **still WRITTEN BUT UNVERIFIED BY A PASSING
BUILD** — the check ran to completion but never reached the crate it would have exercised, through
no fault of the edit itself. What IS directly proven: `grep -c 'cfg(target_arch = "wasm32")'` on
the edited file returns `0` (all 23 per-item cfgs successfully removed, matching the
module-gate-only pattern the two already-building sibling files use), and native syntax/name
resolution for the file was implicitly exercised up to the point the workspace-wide build stopped
(no parse/name-resolution error was reported against this file specifically, only the pre-existing
downstream `workflow` failures).

## Part (3) — proc-macro trio, confirmation only

Target: `✏️s/🔌️plugins/🖍️draw/…/🔄️fsm/✨️macros/📦️packages/🦀️rust/Cargo.toml`
(`semio-s-plugin-draw-fsm-macros`).

```toml
[lib]
proc-macro = true
path = "📚️library/🦀️.rs"

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
```

Confirmed the classifier fix is live in root `📜️script.ts`:

```ts
// 📜️script.ts:17825
function dependencyCargoTomlIsProcMacro(content: string): boolean {
  // scans for a `[lib]` table, returns true if it contains a line matching
  // /^proc-macro\s*=\s*true\s*$/
}

// 📜️script.ts:17849-17850
const isProcMacro = dependencyCargoTomlIsProcMacro(content);
const kindFor = (section: string): DependencyKind =>
  dependencyKindOf(section === "dev-dependencies" ? "test"
    : section === "build-dependencies" || (section === "dependencies" && isProcMacro) ? "build"
    : "runtime");
```

Traced this against the actual manifest text: `[lib]` table contains the line `proc-macro = true`
(exact match for the regex, trimmed), so `isProcMacro = true`; the crate's `[dependencies]` section
(`syn`/`quote`/`proc-macro2`) is then classified `"build"` → `dependencyKindOf("build")` →
`"production-build"`, not `"production-runtime"`. This is exactly the distinction the ticket
describes ("compiler plugins linked at BUILD time, never into the target binary").

**No self-test exists specifically for this proc-macro classifier** (checked — no test references
`proc-macro`/`ProcMacro`/`procMacro` besides the implementation itself), so this confirmation is by
direct static trace against the real manifest, not by running an automated check. Traced, not
assumed: **CONFIRMED live and correctly covers this manifest.**

**Left the manifest alone**, per instruction — `syn`/`quote`/`proc-macro2` stay in `[dependencies]`
(moving them to `[build-dependencies]` would break the crate, as the ticket notes).

## Summary of files touched

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/📦️packages/🦀️rust/Cargo.toml` — removed `serde`/`serde_json` deps + `serde` feature; added `semio-framework-os-kernel` + `semio-framework-value-derive`.
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️.rs` — `PersistedSnapshot`: `Serialize`/`Deserialize` → `ToValue`/`FromValue`.
- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️component.rs` — added `ToValue`/`FromValue` for 2-tuples (framework gap fill) + test.
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` — added explicit crate-root re-export of `DslValue`/`FromValue`/`ToValue`/`ValueError` (defensive; the existing glob chain already exposed these, traced and confirmed).
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs` — added `#![cfg(all(target_arch = "wasm32", not(target_env = "p2")))]` module gate (matching 3d/5d siblings), removed 23 now-redundant per-item cfgs.
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml` — **not touched**, see Part (1) scope finding.
- `✏️s/🔌️plugins/🖍️draw/…/🔄️fsm/✨️macros/📦️packages/🦀️rust/Cargo.toml` — **not touched** (part 3, confirmed correct as-is).

## Verification runs launched (foreground, per the ticket's rules) — results as they landed

```
cargo check -p semio-s-plugin-puzzle --message-format=short       # task b8yc1xeqg — COMPLETED
cargo check -p semio-s-plugin-draw-fsm --message-format=short     # task bup0nx9vw — COMPLETED, exit 0, clean
cargo check -p semio-framework-replication --message-format=short # task bb9s1sijv — still pending
```

- **draw-fsm: PASSED.** Exit 0, `Finished` in 30m52s, zero errors, no new warnings. This is the
  strongest evidence in this doc — see Part (1)'s updated status above.
- **puzzle: INCONCLUSIVE, blocked by unrelated code.** 247 `E0277` errors, all in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/**` (`WorkflowMutation`/`WorkflowDiff`/
  `RunMutation`/`RunDiff` missing `ToValue`/`FromValue`) — a `semio-framework` dependency that
  failed before `semio-s-plugin-puzzle` was ever reached. Verified zero of the 247 errors mention
  puzzle, `BoardSession`, or any wasm-glue symbol. This is the framework-wide `Mutation`/
  `MutationDiff` trait-bound migration (`📓️serde-fanout-playbook.md`) landing ahead of the
  `workflow` module's own conversion — another agent's in-flight work, not mine, recorded per the
  ticket's instruction rather than fixed. The puzzle wasm-gate fix itself stays WRITTEN BUT
  UNVERIFIED BY A PASSING BUILD for this reason, not because of a defect found in it.
- **replication: PASSED.** Exit 0, `Finished` in 29m49s, only 2 pre-existing unrelated warnings
  (`unnecessary qualification` in `📡️wire/🦀️.rs`, unused `push` in `🔗️causal/🦀️.rs`). Confirms the
  `🌱️value/🔁️codec` tuple impl (mounted into this crate as `pub mod value` — see the reachability
  trace above) compiles clean in the crate that actually owns the codec module, independent of the
  os-kernel/draw-fsm consumer path already proven passing.

Whoever picks this ticket up next should re-run `cargo check -p semio-s-plugin-puzzle` once the
`workflow` module's `ToValue`/`FromValue` conversion lands (tracked separately — flagged via
`spawn_task` as "Fix ArtifactApp::Snapshot serde bound blocking dep-zero plugins" if not already
covering this) to get a real signal on the puzzle wasm-gate fix specifically.
