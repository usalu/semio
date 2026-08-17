# 📓️ terra — packet A2b-bridge-green report

Ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet **A2b-bridge-green**. Goal: make
the guest SDK's `component-guest` wasm32-wasip2 build actually compile. **Done — green.**

## The defining command, final run

```
$ cd /Users/ueli/Documents/semio
$ export CARGO_TARGET_DIR=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target"
$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
    Checking semio-framework v0.1.0 (.../🧰️framework/📦️packages/🦀️rust)
    Checking semio-framework-plugin v0.1.0 (.../🔌️plugin/📦️packages/🦀️rust)
warning: fields `child_slots` and `link_slots` are never read  [pre-existing, ArtifactDeclaration, not touched this session]
warning: fields `schemas`, `inferences`, `languages`, and `app_schemas` are never read  [pre-existing, PluginRuntimeRegistry, not touched this session]
warning: `semio-framework-plugin` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized] target(s) in 3.02s
[exited with code 0]
```

**EXIT: 0.** Also confirmed still green, same session, same target dir:

```
$ cargo check -p semio-framework-plugin --lib
    Finished `dev` profile [unoptimized] target(s) in 6.19s
[exited with code 0]
```

(5 warnings on `--lib`: the same 2 pre-existing ones above, plus 3 new-but-harmless ones from this
packet's own cleanup — `unused import std::collections::HashMap` and `outcome_to_result never used`
in `⚛️reactor`/`🌐host`, both inert under `--lib` because that build skips the `#[cfg(...wasm32...)]`
`wit_bridge` module that uses them; real under the wasm target, which is why the wasm run above has
none of these three.)

## 1. The module-path answer — how I verified it, not guessed it

**Empirically, per the packet's own instruction.** I ran the real build with the pre-existing wrong
`crate::component::component::exports::semio::framework::reactor::OpenWindowEffect` (and siblings)
still in place and read rustc's own `help: consider importing …` suggestions off a real
`cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest` (saved as
`📓️terra-A2b-build1.txt` in this folder, 1307 lines, 101 errors). rustc consistently suggested e.g.:

```
275 -     use crate::component::component::exports::semio::framework::reactor::ActivationEvent as W;
275 +     use crate::component::component::semio::framework::events::ActivationEvent as W;
```

**The rule, confirmed across every error in that log**: `world actor` directly `export`s only four
interfaces — `reactor`, `jobs`, `checkpoint`, `describe`. wit-bindgen aliases a type into
`exports::semio::framework::<that interface>::…` **only** when the type is named directly in one of
that interface's own function signatures (e.g. `reactor::poll`'s `list<event>`/`budget` params and
`turn-result` return give `reactor::{Event, Budget, TurnResult, Effect, UiPatch, TurnStatus}` — all
confirmed to compile unqualified). Everything nested one level deeper — a payload record referenced
only from inside one of those aliased types' own variant fields (`ActivationEvent` inside
`Event::Activate`, `OpenWindowEffect` inside `Effect::OpenWindow`, `PatchOp`/`SurfaceRef` inside
`UiPatch`) — stays at the **plain, non-`exports::`** path of the interface that actually declares
it, alongside the `pure` import: `crate::component::component::semio::framework::<origin
interface>::TypeName`. I cross-checked every origin interface against `🧬️schema/📜️component.wit`
itself (not just rustc's first suggestion, since some types like `message-endpoint` are declared in
`types` but re-exported through multiple `use` chains — I picked the canonical declaring interface
each time):

| type(s) | declared in (`component.wit`) | alias I added |
|---|---|---|
| `OpenWindowEffect` … `SubscribeEffect` (all 42 effect payload records), `JobPlacement`, `RespondResult` | `interface effects` | `use crate::component::component::semio::framework::effects as wit_effects;` |
| `ActivationEvent`, `CompletionResult` | `interface events` | `use …::events as wit_events;` |
| `SurfaceRef`, `PatchOp`, `PatchReplace`, `PatchInsertChild`, `PatchRemoveChild`, `PatchSetProps` | `interface ui` | `use …::ui as wit_ui;` |
| `MessageEndpoint` | `interface types` (re-`use`d by both `effects` and `events`, but this is the same aliased Rust type wherever you name it — verified by using `types::MessageEndpoint` uniformly and it compiling at every call site including the `effects`-interface `SendMessageEffect.target` field) | `use …::types as wit_types;` |
| `Event`, `Budget`, `TurnResult`, `Effect` (enum), `UiPatch`, `TurnStatus` | used directly in `reactor::poll`'s own signature | stayed on `crate::component::component::exports::semio::framework::reactor` (aliased `wit`) — unchanged |

All four new aliases live at the top of `⚛️reactor/🦀️component.rs`'s `mod wit_bridge` (the same
`#[cfg(...component-guest.../wasm32.../p2)]`-gated module as before), documented inline with the
verification method so the next person doesn't have to re-derive it.

## 2. Every WIT field I changed, and why (kernel is SSOT — design-abi.md's own directive)

Two genuine field-shape mismatches between the WIT (this packet's own prior draft) and the real
landed `semio_framework::kernel::Effect` (packet A3's SSOT), both found by rustc, both fixed on the
WIT + bridge side, **not** the kernel side, per the packet brief ("where they disagree, change the
WIT and the bridge, not the kernel"):

1. **`request-file-open-effect` was missing `import-action: string`.** `kernel::Effect::RequestFileOpen` has `req, accept, read_as: Option<String>, import_action: String, multiple: bool` — the WIT record had everything but `import-action`. Added the field to `🧬️schema/📜️component.wit`'s `effects` interface (right after `read-as`, before `multiple`, matching kernel field order). The bridge already passed `import_action` in its struct literal (a leftover from an earlier, correct draft) — it was the WIT record lagging, not the Rust.
2. **`request-media-frames-effect.payload` was `option<pack>` (`Option<Vec<u8>>`), kernel is `Option<String>`.** Changed the WIT field to `option<string>` — this is genuinely a string in the kernel type (not opaque bytes needing `pack_rt` encoding, unlike `args: Option<DslValue>` on the same record, which stays `option<pack>` and IS wire-encoded through `pack()`). The bridge's `payload,` field-shorthand now passes through unchanged (no cast needed, since both sides are `Option<String>`).

**Both changes are additive/type-narrowing on records nobody else has implemented yet** (`RequestFileOpen`/`RequestMediaFrames` have zero guest callers today per design-abi.md §0's own measured baseline) — low blast radius, but flagging per the packet brief since B1 (`🖥️host/🦀️component.rs`) parses the same WIT file. B1's `bindgen!` will pick these up automatically on next build; no action needed from B1 unless their own host-side code destructures `RequestFileOpenEffect`/`RequestMediaFramesEffect` by field list (I did not check B1's file — not my path).

No type or interface **names** were renamed — only these two record **field** changes (one add, one
type-narrow). Nothing on the `important.md` reserved-keyword list was reintroduced.

## 3. Other real bugs fixed along the way (not module-path, not WIT-field — plain Rust)

- **`Event::Activate` was unhandled** in `⚛️reactor/🦀️component.rs`'s `poll()` turn-loop match (`match wit_event_to_kernel(event) { … }`) — a non-exhaustive-match compile error once `Event` actually had all its real variants (it does, since A3 landed). Added it as a no-op arm alongside the other not-yet-acted-on lifecycle events (`SuspendRequest`, `CapabilityChanged`, `QuotaChanged`) — `wit_activation_to_kernel` already existed and decodes the `ActivationEvent` reason correctly; **acting** on activation (dispatching the app's `on-command`/`on-view-visible`/etc. handler) is genuine follow-up work this fix does not attempt, flagged below.
- **`&arg.control` used as a field, not a call** at two sites in `🔌️plugin/🦀️component.rs` (crate root, my owned path) — lines ~368 and ~12344. This is fallout from an **unrelated, concurrent** landing in `🛂️manifest/🦀️component.rs` (not mine — see §4) that turned `ActionArgDef.control` from a stored field into a derived method (`ActionArgDef::control()`, the manifest module's own D6 "stored vs. derived" split, its comment block explains the rationale). Fixed both call sites to `&arg.control()`. Mechanical, two-line, no semantic change — `ActionArgControl::Select { options }` destructuring is identical either way.
- Removed one now-dead `use crate::component::component::exports::…::reactor as wit;` inside `poll()` itself (unused after the module-path fixes above moved every payload reference to `wit_effects`/`wit_events`/`wit_ui`/`wit_types` — `poll()`'s own body never constructed a `wit::` value directly, only called helper functions).

## 4. A live, out-of-scope blocker I hit, waited out, and flagged — not mine, not touched

Between my WIT edit and the next `cargo check`, `🛂️manifest/🦀️component.rs` (explicitly **not** my
path — registrar/other-packet-owned) briefly had a hard compile error: `pub enum ArgFormat`'s
`#[serde(tag = "kind", …)]` collided with its own `EntityId { kind: String }` variant field name
(`error: variant field name 'kind' conflicts with internal tag`), which broke **not just my target**
but the previously-green `cargo check -p semio-framework-plugin --lib` too (confirmed by running it
mid-blocker and seeing the identical error cluster) — i.e. this briefly broke the whole fleet, not
only this packet. I did **not** touch that file. I re-ran the acceptance command ~9 times over about
9 minutes (`📓️terra-A2b-build-retries.txt` in this folder has the raw attempt log) confirming via
`git status`/`stat` that the file was live-modified (uncommitted, mtime moving forward each check) —
a peer actively fixing it, not a stable regression — and flagged it with `spawn_task` (task
`task_41bf2225`, since dismissed as resolved) with the exact fix. The peer landed the field rename
(`kind` → `entity_kind`) plus, one retry later, their own remaining call-site fix, and the blocker
cleared on its own. **Net effect on this report: zero**, since it resolved before I finished — noted
here only because "no fabricated command output" (A2's own report's standard) extends to not hiding
that the intervening logs briefly showed unrelated red.

## 5. Done / partial — honest list

**Done, compiler-confirmed, both required commands green (pasted above, this session, this run):**
- The module-path fix — all ~90 call sites across `⚛️reactor/🦀️component.rs`'s `wit_event_to_kernel`, `wit_activation_to_kernel`, `wit_completion_to_kernel`, `wit_endpoint_to_kernel`, `kernel_turn_result_to_wit`, `kernel_ui_patch_to_wit`, `kernel_patch_op_to_wit`, `kernel_effect_to_wit`, `kernel_endpoint_to_wit`, `kernel_placement_to_wit`, `kernel_outcome_to_wit_respond`.
- The two WIT record field fixes (§2).
- The `Event::Activate` non-exhaustive-match fix.
- The two `arg.control()` call-site fixes in the crate root (§3), fallout from someone else's landed change, fixed because the file is mine to write.

**NOT done — genuine remaining gaps, not attempted this session, none block the required command:**
- **Acting on `Event::Activate`** — currently a no-op arm (§3). Dispatching the app's declared activation handler for `on-command`/`on-view-visible`/`on-file-type`/`on-artifact-kind`/`on-extension-request`/`on-startup-finished` is real follow-up work; `wit_activation_to_kernel` decodes the reason correctly but nothing consumes it yet.
- Everything A2's own report (`📓️terra-A2-abi-sdk-report.md` §4/"anything deferred") already listed as not-done is still not-done — I did not expand scope beyond making the build compile: `Emit.tasks: Vec<AsyncTask>`, node-identity-path UI patch diffing (still full-body-replace only), `PluginBuilder` descriptor-populating methods, the VCS backbone channel bridging decision, real end-to-end `poll()` proof against a compiled `.wasm` (needs B1's `WasmtimeRuntime`/`ShardLoop`), `describe()`'s empty `activation_events`/`capability_requests`/`extension_points`/`io_entries`.
- I did not run `cargo test -p semio-framework-plugin --lib` (not in this packet's defining command) or `cargo check -p semio-s-plugin-note --target wasm32-wasip2` (the downstream plugin crate A2's report flagged as the real end-to-end proof) — out of this packet's stated scope, calling it out rather than silently skipping.

## Files touched (all within owned paths)

- `🔌️plugin/🧬️schema/📜️component.wit` — 2 field-level fixes (§2), no renames.
- `🔌️plugin/⚛️reactor/🦀️component.rs` — module-path fix (§1) + `Event::Activate` arm (§3) + one dead-import removal.
- `🔌️plugin/🦀️component.rs` — 2 `arg.control()` call-site fixes (§3), unrelated fallout, fixed because it's mine.

Not touched, not needed: `🔌️plugin/🌐host/🦀️component.rs` (already correct per A2's own prior fix —
verified its 3 `pure` calls still compile clean, no changes required), `🔌️plugin/📦️packages/🦀️rust/{Cargo.toml,📦️glue.rs}`.

## lease-request

None outstanding. The one out-of-scope blocker (§4, `🛂️manifest/🦀️component.rs`) resolved on its
own via a live peer session before I needed to request a lease on it.

## anything deferred

See §5's "NOT done" list — all pre-existing gaps from A2's own report, none newly discovered by this
packet, none blocking the required green build.

No `[DEBUG]` logs were added by this packet — nothing to strip. Scratch logs left in this folder:
`📓️terra-A2b-build1.txt` (the 101-error empirical-discovery run), `📓️terra-A2b-build-retries.txt`
(the manifest-blocker retry sequence, §4).
