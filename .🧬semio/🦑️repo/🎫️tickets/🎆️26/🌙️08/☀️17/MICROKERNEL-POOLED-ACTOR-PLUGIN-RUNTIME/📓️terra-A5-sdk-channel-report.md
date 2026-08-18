# 📓️ terra — A5-sdk-channel report (guest SDK consumes channel v12)

Packet: **A5-sdk-channel**. Scope: `🔌️plugin/🦀️component.rs` (`plugin_exchange` dispatcher) +
`🔌️plugin/⚛️reactor/🦀️component.rs` (`route_app_frame`/`poll`) — absorbing packet A4's channel v12
change (`📓️terra-A4-channel-report.md`).

## Status: **DONE**. Both acceptance commands reach `Finished`, zero errors.

## What changed

### `🔌️plugin/⚛️reactor/🦀️component.rs`

- `poll`'s `Event::InstanceOpen` arm now captures `actor` and calls
  `crate::plugin_runtime::set_instance_actor(numeric_instance, actor)` — channel v12 retired the
  `AppCommand::Hello` handshake that used to be the only caller of this, so the actor-id recording
  moves to instance-open time, matching design-abi.md §4 ("lifecycle is now `Event::InstanceOpen`/
  `InstanceClose` at the reactor level").
- `poll`'s `plugin_exchange` call site now matches `Ok(output)` (a `PluginExchangeOutput`, see
  below) instead of `Ok(frames)`: frames still route through `route_app_frame`; `output.effects`/
  `output.events` (wire-encoded `kernel::Effect`/`kernel::AppEvent` bytes) are decoded directly and
  pushed into the turn's `effects` list — this is the "effects/events travel as `kernel::Effect`/
  `kernel::Event` directly in `TurnResult`, not wrapped in an `AppFrame`" the packet brief named,
  since `AppFrame::Effects`/`Events` no longer exist as wire variants for `plugin_exchange` to
  construct.
- `route_app_frame`: deleted the `AppFrame::Effects`/`AppFrame::Events` arms (dead — those variants
  don't exist in channel v12; the decode logic that used to live there moved verbatim into `poll`,
  reusing the same `decode_wire_effect`/`decode_wire_app_event` helpers). Replaced the old
  `AppFrame::UiSection` stub arm with a real `AppFrame::UiPatch` → `kernel::UiPatch` passthrough:
  `surface`/`kind`/`revision`/`base_revision` are a direct field-for-field move (channel v12's
  `UiPatch` mirrors `kernel::UiPatch` exactly, per A4), `ops: Vec<u8>` is decoded via
  `store::pack_rt::decode_wire_value` → `dsl::from_dsl_value::<Vec<PatchOp>>` (same two-step
  wire-decode idiom `decode_wire_effect` already used) and pushed into `PENDING_PATCHES` — the same
  thread-local `poll`'s `dirty_render` path already drains into `TurnResult.ui_patches`. Added a
  no-op arm for the new `AppFrame::UiSnapshotEnd` (no snapshot-boundary consumer yet this wave).
  `plugin_exchange` itself never emits `AppFrame::UiPatch` today (nothing produces UI frames through
  the command-dispatch path in this wave — UI still renders via the separate `SurfaceVisible` →
  `plugin_render` path in `poll`), so this arm is forward-looking/dead-in-practice but wired
  correctly per the packet's explicit instruction, not silently dropped.

### `🔌️plugin/🦀️component.rs`

- **Deleted** (channel v12 removed the wire variants these matched):
  - `AppCommand::Hello` arm (handshake: channel-version check, actor recording, optional inline
    config load, `AppFrame::Welcome` reply). Actor recording moved to the reactor's
    `Event::InstanceOpen` handling (above); the optional config-load-on-hello convenience has no
    replacement in this wave — `AppCommand::LoadConfig` (a separate, still-live command) remains the
    only way to load config, same as always.
  - `AppCommand::RefreshUi` arm and its `SECTION_KIND_*` consts + `channel_refresh_section` helper
    (section-probe UI refresh — retired per design-abi.md §4: "surface rendering is driven by
    `surface-visible`/`hidden` instead of `RefreshUi` probes", which `poll`'s `dirty_render` path
    already implements independently). `fnv1a64` is kept — still used by `ui_refresh_fnv1a_hash`,
    which belongs to the unrelated JSON-based `plugin_refresh_ui` WIT export, not this channel.
  - `AppCommand::AttachBackbone`/`DetachBackbone` arms (both variants gone from the wire; no
    replacement command exists in channel v12 — `plugin_attach_backbone`/`plugin_detach_backbone`
    functions themselves are left in place since they're still `pub fn`, part of the crate's public
    surface, just currently uncalled from the channel).
  - `AppCommand::Bye => {}` arm (variant gone; was already a no-op).
- **Fixed** (real command-dispatch arms whose only problem was constructing the now-gone
  `AppFrame::Effects`/`Events` frames):
  - `push_invocation_side_frames` signature changed from
    `(frames: &mut Vec<protocol::AppFrame>, seq: u64, result: &InvocationResult)` to
    `(effects: &mut Vec<Vec<u8>>, events: &mut Vec<Vec<u8>>, result: &InvocationResult)` — same
    `encode_wire_serialized` per-item encoding as before, just collected as plain byte vecs instead
    of wrapped in a frame. Its 4 call sites (`ConfigCommand`, `Command`, `ArtifactCommand`,
    `PureCommand` arms) updated to pass the new accumulators.
  - `OpenArtifact`/`SetDefaultApp`/`ClearDefaultApp` arms: `frames.push(AppFrame::Effects{...})`
    → `effect_bytes.push(encode_wire_serialized(&effect))`.
  - End-of-batch `mutated` drain: same swap, `frames.push(AppFrame::Effects{...})` →
    `effect_bytes.extend(...)`.
- **New return shape**: `plugin_exchange` now returns `Result<PluginExchangeOutput, Fault>` instead
  of `Result<Vec<Vec<u8>>, Fault>`, where `PluginExchangeOutput { frames: Vec<Vec<u8>>, effects:
  Vec<Vec<u8>>, events: Vec<Vec<u8>> }`. `plugin_exchange` has exactly one caller in the whole tree
  (`⚛️reactor`'s `poll`, confirmed via `rg`), so this is a contained signature change, not a public
  API break.
- `set_instance_actor` changed from private `fn` to `pub(crate) fn` (needed cross-module now that its
  only caller is `⚛️reactor::wit_bridge::poll`) with an updated doc comment; the `INSTANCE_ACTORS`
  thread-local's doc comment updated to match.
- Stale doc-comment references to the deleted `AppCommand::Hello`/`RefreshUi`/`Bye`/
  `AppFrame::UiSection`/`SectionProbe` cleaned up (`plugin_exchange`'s own doc, `fnv1a64`'s doc,
  `INSTANCE_ACTORS`'s doc) — all now describe the current (post-v12) mechanism.

## Acceptance — real output, real exit codes

### `cargo check -p semio-framework-plugin --lib`

```
export CARGO_TARGET_DIR=".../🎯️target-a5"
cargo check -p semio-framework-plugin --lib
```

```
    Finished `dev` profile [unoptimized] target(s) in 5.83s
```

Zero errors. 6 warnings, none new/blocking:
- 1 pre-existing warning in `📡️wire/🦀️component.rs:448` (`pos` never read) — not mine, present
  before this packet's edits too (confirmed against the pre-edit build output).
- `unused imports: Effect, Event, MessageEndpoint, PatchOp, RequestOutcome, TurnStatus, UiPatch` and
  `unused import: std::collections::HashMap` in `⚛️reactor/🦀️component.rs` — pre-existing on `--lib`
  builds specifically: the entire `wit_bridge` module (everything using these types) is
  `#[cfg(... target_arch = "wasm32" ...)]`-gated and simply doesn't compile on a native `--lib`
  check, same as before this packet touched the file (confirmed identical in the pre-edit failing
  build's warning list).
- `function outcome_to_result is never used` (🌐host) — same native-build cfg-gating story,
  pre-existing.
- `function set_instance_actor is never used` — new, and expected: on native `--lib` builds its only
  caller (`⚛️reactor::wit_bridge::poll`, wasm-gated) isn't compiled, exactly mirroring
  `outcome_to_result`'s existing pattern above. Confirmed NOT dead on the real target below.
- 2 unrelated pre-existing dead-field warnings (`ArtifactDeclaration`, `PluginRuntimeRegistry`) —
  untouched by this packet.

### `cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest`

```
export CARGO_TARGET_DIR=".../🎯️target-a5"
cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest
```

```
    Finished `dev` profile [unoptimized] target(s) in 3m 34s
```

Zero errors. Only 2 warnings, both the same pre-existing dead-field ones as above — no unused-import
or dead-`set_instance_actor` warnings here, confirming `wit_bridge` (and therefore `set_instance_actor`'s
real caller) compiled and type-checked cleanly on the actual wasm32-wasip2/component-guest target.

## Design judgment calls made explicit

- `plugin_exchange`'s return type change (`Vec<Vec<u8>>` → `PluginExchangeOutput`) was not literally
  spelled out in the packet brief, but is the direct mechanical consequence of "effects and events
  now travel as `kernel::Effect`/`kernel::Event` directly in `TurnResult`, not wrapped in an
  `AppFrame`" (packet brief, echoing design-abi.md §2) applied to a function that previously had no
  other way to surface them. Verified single-caller before changing the signature.
- Did NOT port `AppCommand::Hello`'s optional inline config-load convenience forward into
  `Event::InstanceOpen` handling — the packet brief says "Remove the dead paths rather than adapting
  them" and `📌️important.md`/root `CLAUDE.md` forbid compatibility-layer thinking on a greenfield
  repo; `AppCommand::LoadConfig` already exists as the real, separate way to load config, so no
  capability is actually lost, only the one-shot bundling with the handshake.
- Did port actor-id recording (`set_instance_actor`) from the deleted `Hello` arm into
  `Event::InstanceOpen` handling, since `kernel::Event::InstanceOpen.actor` already carries the exact
  same data and design-abi.md §4 explicitly names `Event::InstanceOpen`/`InstanceClose` as where
  lifecycle now lives — leaving `instance_actor()` permanently falling back to `"local"` would have
  been a silent regression the packet brief didn't ask for.

## Lease-requests

None. Both files edited are fully inside this packet's owned `path_scope`
(`🔌️plugin/🦀️component.rs`, `🔌️plugin/⚛️reactor/**`); no shape mismatch forced a WIT change.

## Debug logs

None added — this packet's changes are direct code edits, no `[DEBUG]` instrumentation needed or
left behind.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`
- `📓️terra-A5-sdk-channel-report.md` (this file)
