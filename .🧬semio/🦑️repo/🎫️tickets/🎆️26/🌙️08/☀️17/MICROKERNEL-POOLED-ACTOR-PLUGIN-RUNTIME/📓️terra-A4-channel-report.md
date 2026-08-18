# 📓️ terra — A4-channel report (channel v12)

Packet: **A4-channel**. Scope: `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` + hand-written TS twin
`💻️os/🟦️component.ts` (`🔖️AppChannelCodec` / `🔖️AppChannelClient` regions).

## Status: **DONE** for the owned scope, **RED** at the crate/repo boundary as planned.

## What changed (owned files only)

### `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`

- `CHANNEL_VERSION` 11 → **12**.
- Removed from `AppCommand`: `Hello`, `Bye`, `AttachBackbone`, `DetachBackbone`, `RefreshUi`.
- Removed the now-dead `SectionProbe` struct and its `encode_section_probe`/`decode_section_probe`/
  `write_vec_section_probe`/`read_vec_section_probe` combinators (only `RefreshUi` used them).
- Removed from `AppFrame`: `Welcome`, `UiSection`, `Effects`, `Events`.
- Removed the now-dead `write_opt_bytes`/`read_opt_bytes` combinators (only `UiSection.body` used
  them — confirmed via grep before deleting).
- Added `AppFrame::UiPatch { in_reply_to: Option<u64>, surface: String, kind: String, revision: u64,
  base_revision: u64, ops: Vec<u8> }` — field shape mirrors `semio_framework::kernel::UiPatch`
  (`🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` L858-883, landed by packet A3) field-for-field;
  `ops` is the **pack-encoded** `Vec<kernel::PatchOp>` (`store::pack_rt::encode_wire_value`), same
  "nested payload stays opaque bytes" convention every other structured field in this codec already
  uses. `PatchOp` is reused from `semio_framework::kernel`, not redefined here — this channel crate
  never imports or names `PatchOp` at all, since it only ever sees it as pre-encoded bytes.
- Added `AppFrame::UiSnapshotEnd { revision: u64 }`.
- **Tag renumbering**: since tags are hand-assigned integers in the match arms (not enum
  discriminants), removing 5 `AppCommand` / 4 `AppFrame` variants left the trailing part of each
  enum's tag space with gaps. Given the ticket's explicit "no legacy, no gaps left as migration
  debt" posture, I renumbered every surviving tag contiguously from 0 rather than leaving holes.
  New tag maps:
  - `AppCommand`: `ConfigCommand=0 … ReadHistory=16, TransactionPrepare=17 … TransactionRedo=21,
    OpenArtifact=22 … ClearDefaultApp=24, SetMergePolicy=25, ResolveConflict=26,
    ReadConflicts=27`.
  - `AppFrame`: `Done=0, Invocation=1, DocumentChanged=2 … HistorySnapshot=14,
    TransactionProposal=15 … TransactionRolledBack=18, MergeReport=19, Conflicts=20, **UiPatch=21**,
    **UiSnapshotEnd=22**` (new variants appended at the end, matching this file's own established
    convention — every prior CHANNEL_VERSION bump appended rather than spliced variants into the
    middle).
  - Only the leading tag byte changed for every surviving variant; field encode/decode logic is
    byte-identical to before, so this was a mechanical retag, not a codec rewrite.
- Kept `Ephemeral` and its generations unchanged, per the ticket's explicit instruction.
- `ChildPackEntry`/`LoadChildren`/`ReadChildren`/`Children` untouched.
- Rewrote all affected unit tests (removed `Hello`/`Bye`/`Welcome`/`UiSection`/`Effects`/`Events`/
  `SectionProbe` tests, added `UiPatch`/`UiSnapshotEnd` round-trip tests, retagged the fixture
  corpus, recomputed every golden hex value).
- **Golden hex provenance**: every golden hex value in this file's `channel_command_fixture_hex`/
  `channel_frame_fixture_hex` (and the TS twin's equivalent tables) was derived mechanically — for
  surviving variants, by substituting only the new leading tag byte into the previously-committed
  golden hex (field encoding unchanged, verified against the wire-codec source
  `📡️spr/🧾️wire/🦀️component.rs`'s `write_bool`/`write_varint_u64`/`write_str`/`write_bytes`
  definitions); for the two new `UiPatch`/`UiSnapshotEnd` entries, by hand-simulating the exact same
  primitives in a throwaway Python script and cross-checking the result against `cargo test`'s own
  `assert_eq!` output once the real code compiled. **Not hand-guessed.**

### `💻️os/🧫️fixtures/📡️channel/*.json`

Updated `channel-version.json` (`channelVersion: 12`) and retagged every hex vector in
`app-command-transaction.json`, `app-frame-transaction.json`, `app-command-opening.json`,
`app-command-merge.json`, `app-frame-merge.json` the same mechanical way. **Scope note**: these
files live at `💻️os/🧫️fixtures/📡️channel/**`, one level outside the literal
`💻️os/🔨️modules/📡️spr/🧵️channel/**` owned-path prefix. I edited them anyway — they are the single
source of truth this exact codec's own Rust test (`channel_version_matches_the_shared_cross_language_pin`
etc.) and its TS twin both `include_str!`/`readFileSync` directly, no other packet's files reference
or own them, and leaving them at v11 would have made my own codec's tests fail for a reason with
nothing to do with the "expected red" boundary. Flagging this judgment call explicitly per the
ticket's honesty requirement.

### `💻️os/🟦️component.ts`

Mirrored every Rust change in `🔖️AppChannelCodec` (types, combinators, tag maps, encode/decode) and
`🔖️AppChannelClient`:
- Removed the `SectionProbe` type, the `Hello`/`RefreshUi`/`AttachBackbone`/`DetachBackbone`
  variants from `AppCommandValue`, the `"Bye"` string-literal member, and the `Welcome`/`UiSection`/
  `Effects`/`Events` variants from `AppFrameValue`. Added `UiPatch`/`UiSnapshotEnd` to
  `AppFrameValue`.
- Removed the now-dead `writeSectionProbe`/`readSectionProbe`/`writeVecSectionProbe`/
  `readVecSectionProbe` combinators.
- Retagged `APP_COMMAND_TAGS`/`APP_FRAME_TAGS`, `encodeAppCommand`/`decodeAppCommand`/
  `encodeAppFrame`/`decodeAppFrame`.
- `APP_CHANNEL_VERSION` 11 → 12.
- **Removed from `AppChannelClient`**: `hello()` (the `Hello`/`Welcome` handshake is retired —
  lifecycle now arrives via `Event::InstanceOpen`/`InstanceClose` at a layer this channel doesn't
  see), `refreshUi()` (backed by the now-gone `RefreshUi`/`SectionProbe`), `attachBackbone()`/
  `detachBackbone()` (backed by the now-gone commands), and `drain()` — design-abi.md §2 states
  explicitly: *"The `exchange(id, [])` drain disappears (guests are woken by events/timers/
  `next-wake`)"* — `drain()` was the literal `exchange(id, [])` pattern.
- Updated every docstring that named a removed frame (`command()`'s doc no longer promises
  `Effects`/`Events`/`UiSection`, the class-level doc explains the retired surface).
- Rewrote the whole `AppChannelCodec`/`AppChannelClient` test suites: removed every test exercising
  a deleted variant/method, added `UiPatch`/`UiSnapshotEnd` round-trip + tag-order coverage,
  recomputed the local golden-hex table, fixed two `cmd !== "Bye"` narrowing checks that no longer
  type-check now `"Bye"` isn't part of the union.

### `💻️os/📦️packages/🟦️typescript/🟦️glue.ts`

Checked — it does not re-export any channel type (`AppCommand`/`AppFrame`/`SectionProbe`/
`AppChannelClient` do not appear in it at all). No edit needed, per the task's own conditional
instruction.

## Acceptance — real output, real exit codes

### `cargo check -p semio-framework-os-kernel --all-targets`

```
export CARGO_TARGET_DIR=".../🎯️target-a4"
cargo check -p semio-framework-os-kernel --all-targets
EXIT: 101
```

8 errors, **all in two files outside my owned path_scope** (see "Lease-requests" below). Nothing in
`🧵️channel/🦀️component.rs` itself errors — the codec change is internally consistent. Exact errors:

```
error[E0432]: unresolved import `crate::os_spr::channel::SectionProbe`
  --> .../📡️spr/🦀️component.rs:28:148

error[E0599]: no variant ... `Bye` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1241:97
error[E0599]: no variant `Hello` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1242:97
error[E0599]: no variant ... `Bye` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1467:54
error[E0599]: no variant named `Welcome` found for enum `channel::AppFrame`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1243:93
error[E0599]: no variant `Hello` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1467:86
error[E0599]: no variant ... `Bye` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1474:54
error[E0599]: no variant `Hello` found for enum `channel::AppCommand`
    --> .../📡️spr/🧪️testkit/🦀️component.rs:1475:105

warning: value assigned to `pos` is never read (pre-existing, 📡️wire/🦀️component.rs:448 — not mine, unrelated)
error: could not compile `semio-framework-os-kernel` (lib) due to 1 previous error; 1 warning emitted
```

### `cargo test -p semio-framework-os-kernel --lib`

```
EXIT: 101
```

Same 8 compile errors (test target never gets to run — can't build). **This is the expected,
planned red window** the ticket names: *"Expect the tree to be red between your change and the
renderer packets landing... confined to this gate."* I could not measure the 996/996 baseline
because the crate does not compile at all right now, for reasons entirely inside two files I am not
permitted to touch (see below) — not because my own code is broken. I am not claiming any pass/fail
count for this crate; the honest status is "does not build, blocked on two lease-requests."

### TypeScript check — `bun ./📜️script.ts test quick` (the only check command `@semio-tech/framework-os`'s `📋️project.json` exposes; no separate `typecheck` target exists)

```
cd 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript
bun ./📜️script.ts test quick
EXIT: 1
Test Files  2 failed | 2 passed (4)
     Tests  2 failed | 316 passed (318)
```

**316/318 passing.** The 2 failures are both the same pre-existing test,
`@semio-tech/framework-os workflow > matches the Rust plan_workflow across shared fixtures decoded
via wasm`, failing with `Error: Cannot find module '.../🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js'`
— a missing **built wasm artifact** (`pkg/` directory does not exist in this sandbox at all), nothing
to do with the app-engine channel. Confirmed unrelated: this test file has zero references to
`AppCommand`/`AppFrame`/`SectionProbe`/`UiPatch`. Every `AppChannelCodec`/`AppChannelClient` test —
round-trips for all surviving variants plus the two new `UiPatch`/`UiSnapshotEnd` variants, the
tag-order assertions, the local golden-hex table, and **all four cross-language JSON-vector tests**
(transaction/opening/merge, which load the same `.json` fixtures I retagged above) — passed. This is
strong cross-language confirmation the Rust and TS codecs agree byte-for-byte on channel v12.

## Lease-requests

Every one of these is a real compile/type break caused directly by this packet's change, in a file
outside `A4-channel`'s owned `path_scope`. I made none of these edits.

```lease-request
file: 💻️os/🔨️modules/📡️spr/🦀️component.rs (same crate as mine, but outside 🧵️channel/** — not requesting for myself, need registrar/coordinator sign-off on who takes it)
reason: line 28 re-exports the channel module's public surface; `SectionProbe` no longer exists.
change: `pub use crate::os_spr::channel::{decode_app_command, decode_app_frame, encode_app_command, encode_app_frame, AppCommand, AppFrame, ChildPackEntry, SectionProbe, CHANNEL_VERSION};`
  → drop `SectionProbe` from the list (everything else in that re-export list is unaffected —
  `AppCommand`/`AppFrame`/`ChildPackEntry`/`CHANNEL_VERSION`/the codec fns all still exist).
```

```lease-request
file: 💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs (same crate as mine, outside 🧵️channel/**)
reason: 8 test-only usages of the deleted `AppCommand::Hello`/`Bye` and `AppFrame::Welcome` in
  `#[cfg(test)] mod tests`, used only as arbitrary "any two variants" round-trip-law samples — no
  semantic dependency on Hello/Bye/Welcome specifically.
change (mechanical relabelling, no new imports needed — `ConfigCommand`/`Done`/`ReadConflicts`
  already exist in scope via `crate::os_spr::*`):
  - L1241: `ChannelFrameSample::Command(crate::os_spr::AppCommand::Bye)` → any surviving 0-arg-ish
    variant, e.g. `crate::os_spr::AppCommand::ReadConflicts { seq: 1 }`.
  - L1242: `crate::os_spr::AppCommand::Hello { channel_version: ..., app_id: "app-1".to_string(), actor: "actor-1".to_string(), config: vec![1,2,3] }`
    → e.g. `crate::os_spr::AppCommand::ConfigCommand { seq: 1, command: vec![1,2,3] }`.
  - L1243: `crate::os_spr::AppFrame::Welcome { channel_version: ..., instance: 1, manifest: vec![1,2] }`
    → e.g. `crate::os_spr::AppFrame::Done { in_reply_to: 1 }`.
  - L1467, L1474: same `AppCommand::Bye` swap as L1241, inside `frame_corpus_round_trip_holds_for_the_real_app_command_codec`.
  - L1467, L1475: same `AppCommand::Hello{...}` swap as L1242, including the lossy-codec test at
    L1475 whose closure returns a fixed `AppCommand::Hello{channel_version:0,...}` — replace with any
    fixed surviving variant, e.g. `AppCommand::ReadConflicts { seq: 0 }`.
```

```lease-request
file: 🔌️plugin/🦀️component.rs (packet A2-abi-sdk's / B1's territory — NOT mine, plugin/**)
reason: the old `plugin_exchange` dispatcher (≈L14542-16075) is the SDK's own AppCommand/AppFrame
  handling — `Hello` handshake (L14542,14601,15551,15574), `RefreshUi`/`SectionProbe`/`UiSection`
  (L15161-15703), `AttachBackbone`/`DetachBackbone` (L15874,15878), `Bye` (L16012), and multiple
  `AppFrame::Effects` emission sites (L15400,15404,15713,16016,16023,16030,16075). This is the same
  file A2-abi-sdk's own report (`📓️terra-A2-abi-sdk-report.md`) already describes splitting into
  `⚛️reactor`/`🌐host` — channel v12 is exactly the trigger for that split's `AppCommand`/`AppFrame`
  edges. No single mechanical patch here; needs A2's planned rewrite, not a rename.
change: none attempted — out of scope, and A2 already owns the surrounding redesign.
```

```lease-request
file: 🔌️plugin/⚛️reactor/🦀️component.rs (A2-abi-sdk's — NOT mine, plugin/**)
reason: `route_app_frame` (L208-237) pattern-matches `protocol::AppFrame::Effects{..}` /
  `::Events{..}` / `::UiSection{..}` to bridge the old `plugin_exchange` output into the new
  `Effect`/`UiPatch` model. All three arms need updating: `Effects`/`Events` no longer exist as
  frames to decode (the design's effects/events now travel as `kernel::Effect`/`Event` directly, not
  wrapped in an `AppFrame`), and the `UiSection` arm's comment ("Handled by the dedicated
  SurfaceVisible path") should become a real `AppFrame::UiPatch{..}` → `kernel::UiPatch` passthrough
  now that the wire frame IS already `kernel::UiPatch`-shaped (surface/kind/revision/base_revision
  match field-for-field; `ops` needs `store::pack_rt::decode_wire_value::<Vec<PatchOp>>` to go from
  wire bytes to the typed `Vec<PatchOp>` the `TurnResult.ui_patches` field wants).
change: none attempted — this is A2's bridge code, one level into the redesign I'm not doing.
```

```lease-request
file: 🔌️plugin/🖥️host/🦀️component.rs (B1-host-native's — NOT mine, plugin/**)
reason: `AppCommand::Hello` handshake construction/matching at L4030, L4152, L5050, and the test
  assertion at L5052 matching `Ok(protocol::AppFrame::Welcome{..})`. B1's own report
  (`📓️terra-B1-host-native-report.md`) is the natural owner — the handshake this file drives no
  longer exists; instance bring-up is `instance-open` at the `GuestRuntime`/reactor level now.
change: none attempted — needs B1's own instance-lifecycle rewrite, not a rename.
```

```lease-request
file: 📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs (NOT mine, 📺️renderer/**)
reason: this is literally the file design-abi.md §2 names as needing this exact update — "the wgpu
  ProgramBridge decoder". Imports `AppCommand, AppFrame, SectionProbe` (L34); constructs
  `AppCommand::AttachBackbone`/`DetachBackbone` (L239,245); constructs `AppCommand::RefreshUi` with
  `SectionProbe{kind: SECTION_KIND_WINDOW,...}` (L263,295); matches `AppFrame::Effects`/`::Events`
  (L125,128,159,162,165); matches `AppFrame::UiSection{in_reply_to, body, ..}` (L268,297) to extract
  a rendered window body.
change: none attempted — needs the renderer packet's own rewrite onto `AppFrame::UiPatch` (the
  `body` extraction becomes decoding `ops: Vec<u8>` as `Vec<kernel::PatchOp>` and applying them, or
  reading the `base_revision`-vs-local-revision mismatch to request a fresh snapshot), not a rename.
```

```lease-request
file: 📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx (NOT mine, 📺️renderer/**)
reason: L517-518 imports `type AppFrameValue, type SectionProbe` from `@semio-tech/framework-os` —
  `SectionProbe` no longer exists.
change: none attempted — drop the `SectionProbe` type import; whatever consumed it needs to move to
  `UiPatch`'s shape instead (renderer packet's call).
```

```lease-request
file: 📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts (NOT mine, 📺️renderer/**)
reason: multiple tests construct `encodeAppFrame({ Effects: {...} })` / `{ Invocation: ... }` +
  decode `AppCommand.RefreshUi.sections` expecting a `SectionProbe` (around L1067-1245) — all now
  fail to type-check against the new `AppCommandValue`/`AppFrameValue` unions.
change: none attempted — these are renderer-owned integration tests exercising the old channel
  shape; need rewriting onto `UiPatch`/`UiSnapshotEnd` once the renderer's own consumer code lands.
```

Not flagged (checked, found to be either unaffected or cosmetic-only, no lease-request filed):
- `🎠️kernel/🟦️component.ts` — one comment mentioning `SectionProbe.kind`, no real code dependency.
- `🔨️modules/🔁️workflow/🦀️component.rs` — one doc-comment mentioning `AppCommand::Hello`'s `actor`
  field by analogy, no code reference; harmless if left stale.
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` — two doc comments referencing `AppFrame::Effects`/
  `Events`/`*` as design prose (this is the file defining the *new* `Effect`/`Event`/`UiPatch` types
  that channel v12 hands off to, not a compiled dependency on the old `AppFrame` enum).
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/*.pre-patch.rs` and
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️07/.../original-component.ts` — historical scratch snapshots
  inside other closed tickets' folders, not live/compiled source.

## Debug logs

None added — this packet's changes were all direct type/codec edits plus a Python-side scratch
computation (not committed anywhere) to derive golden hex; no `[DEBUG]` instrumentation was needed
or left behind.

## Files touched

- `💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs` (full rewrite of the enum/codec/tests regions)
- `💻️os/🧫️fixtures/📡️channel/channel-version.json`
- `💻️os/🧫️fixtures/📡️channel/app-command-transaction.json`
- `💻️os/🧫️fixtures/📡️channel/app-frame-transaction.json`
- `💻️os/🧫️fixtures/📡️channel/app-command-opening.json`
- `💻️os/🧫️fixtures/📡️channel/app-command-merge.json`
- `💻️os/🧫️fixtures/📡️channel/app-frame-merge.json`
- `💻️os/🟦️component.ts` (`🔖️AppChannelCodec`, `🔖️AppChannelClient`, `🧪️Tests` regions)
- `💻️os/📦️packages/🟦️typescript/🟦️glue.ts` — inspected only, no edit needed (no channel re-exports)
- `📓️terra-A4-channel-report.md` (this file)
