# 📓️ terra — packet A3-kernel-types report

Ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet **A3-kernel-types**.

## Summary

Part 1 (additive `Effect`/`Event`/`UiPatch`/`Budget`/`TurnResult`/`Broker`/`Quota`/`PackageDescriptor` types) and Part 2 (the atomic `HostEffect` → `Effect` mechanical rename, repo-wide) are both done. `cargo check -p semio-framework --all-targets` and `cargo check -p semio-framework-os-kernel --all-targets` both exit 0 with only pre-existing, unrelated warnings. The workspace-wide check is blocked from being fully green only by three **registrar-only** files this packet is forbidden from editing (`📌️important.md` "Registrar-only files" / "`Shell/🧊️component.rs`") — `lease-request` blocks for all three are below.

## Part 1 — new contract types

### `🎠️kernel/🦀️component.rs` region map (new/changed regions only)

| region | content |
|---|---|
| `🔖️Effect` (replaces the old unlabeled `HostEffect` block) | `RequestId`, the full `Effect` enum (22 renamed variants + 23 new: `SendMessage`, `PublishEvent`, `BlobWrite`, `BlobLoad`, `HttpRequest`, `DocumentRead`, `DocumentWrite`, `LinkResolve`, `RegistryQuery`, `IoCompose`, `CacheDerive`, `CacheRead`, `SetTimer`, `SpawnJob`, `CancelJob`, `Respond`, `StorageRead`, `StorageWrite`, `StorageDelete`, `RequestCapability`, `ReleaseCapability`, `Subscribe`, `Unsubscribe`), `JobPlacement`, `IconRenderExportItem` |
| `🔖️Event` (new, appended end-of-file) | `MessageEndpoint`, `RequestOutcome`, `Event` (lifecycle/channel/surface/completion/messaging/timer/request variants) |
| `🔖️ActivationEvent` (new) | `ActivationEvent` |
| `🔖️UiPatch` (new) | `UiPatch`, `PatchOp` |
| `🔖️Budget` (new) | `Budget` |
| `🔖️TurnResult` (new) | `TurnStatus`, `Usage`, `TurnResult` |
| `🔖️Broker` (new) | `CapabilityId`, `CapabilityRequest`, `BrokerCapabilityGrant`, `CapabilityChange`, `QuotaSchema`, `QuotaTree`, `QuotaBreach`, `FailureAction`, `BrokerHooks` trait |

`InvocationResult.requested_effects: Vec<HostEffect>` → `Vec<Effect>` (the one other in-file reference, `🔖️Invocation` region).

### Naming-collision note: `CapabilityGrant` / `CapabilityRequirement`

`📓️design-abi.md` §5 names the broker's grant type `CapabilityGrant`, but `🎠️kernel/🦀️component.rs` already has a **different** `CapabilityGrant` (`{token: CapabilityToken, capability: Capability}`, `🔖️Capability` region, used by `ActionContext.granted_capabilities`) for the kernel-level action/window capability model. Same-module duplicate names aren't possible in Rust, and that existing type has live consumers outside this packet's owned paths:

- `📦️packages/🦀️rust/📦️glue.rs`'s `pub use manifest::kernel::{…, CapabilityGrant, CapabilityRequirement, …}` re-export list.
- `ActionContext` itself, used by 3 plugin editor components (`✏️s/🔌️plugins/🧩️puzzle/**` ×3, unchanged by this packet).

Per this packet's own instruction for `CapabilityRequirement` ("if it does [have consumers], report them instead of breaking them"), I applied the same rule to `CapabilityGrant`: **the new broker type is named `BrokerCapabilityGrant`**, not `CapabilityGrant`. `CapabilityRequirement`/`Rights`/`Scope` themselves were **not** deleted — they too have live consumers outside this packet (`🔌️plugin/🏗️builder/🦀️component.rs`, `🔌️plugin/🖥️host/🦀️component.rs`, `🔌️plugin/🦀️component.rs`'s own `SurfaceDeclaration`/`ArtifactRuntimeCapabilityRequirement` machinery) — `CapabilityRequest` (the new broker-facing type, exact name per design) sits alongside them, additive. **This naming collision needs a registrar decision** (rename the old `CapabilityGrant`/`CapabilityRequirement` once the legacy action-capability model is deprecated, or keep the `Broker`-prefixed names permanently) — flagging rather than deciding unilaterally since it touches files outside this packet's scope.

### `🎭️actor` dependency avoided

`Event::InstanceOpen.actor` is typed `String` (placeholder), not `RuntimeActorId` — per `📌️important.md`'s naming hazard note, this packet must not read/depend on the concurrently-created `🎭️actor` crate. Whichever packet lands `RuntimeActorId` should retype this field.

### `🛂️manifest/🦀️component.rs`: new `🔖️PackageDescriptor` region

`PackageRole`, `ExecutionMode`, `ExtensionPointDeclaration`, `AssetDeclaration`, `PackageHashes`, `DescriptorEntry`, `ContributionSet`, `PackageDescriptor` — inserted between the existing `🔖️Kernel` and `🔖️MediaVocabulary` regions. `ContributionSet`'s `commands`/`topic_contributions`/`artifact_contributions` reuse this crate's existing typed models (`CommandDefinition`, `TopicContribution`, `ArtifactContributionDescriptor`); `menus`/`file_types`/`panels`/`themes`/`inference_services`/`mutation_services`/`io_entries`/`composer_entries` don't have a typed manifest model anywhere in the codebase yet (verified: no `Menu`/`FileType`/`Theme`/`IoEntry`/`ComposerEntry` declarative type exists in `manifest.rs`, and giving each a bespoke first-cut shape without a design doc field-list was out of this packet's additive-only charter) — they're `Vec<DescriptorEntry>` (`{id, payload: Option<json>}`), explicitly documented as a placeholder for whichever packet (E1-describe) gives each category its typed shape.

### ts-rs (`typegen` feature) coverage

Only types reachable from `PackageDescriptor`'s tree got `#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]` + an `.export()` call in `exports_typescript_bindings`: kernel's `CapabilityId`, `CapabilityRequest`, `QuotaSchema`, `ActivationEvent`, plus every new manifest type. Everything else (`Effect`, `Event`, `RequestId`, `UiPatch`/`PatchOp`, `Budget`, `TurnResult`/`TurnStatus`/`Usage`, `MessageEndpoint`, `RequestOutcome`, `JobPlacement`, `BrokerCapabilityGrant`, `CapabilityChange`, `QuotaTree`, `QuotaBreach`, `FailureAction`) follows the file's **existing** convention for this exact kind of type: `HostEffect` itself never had `ts_rs::TS` either — `🎠️kernel/🟦️component.ts` is hand-maintained for the whole ABI-boundary surface (identifiers, `Capability`/`CapabilityGrant`/`CapabilityRequirement`, `HostEffect`, `UiDirtyScope`, …), and `PatchOp`'s `node: UiNode` field would fail to derive `TS` anyway (`ui_wgpu::wgpu::UiNode` is explicitly not-yet-typegen-derived per the comment beside `exports_typescript_bindings`'s `UiNode` variant exports).

### `🎠️kernel/🟦️component.ts`

`HostEffect` → `Effect`, expanded to mirror the Rust enum exactly (`req` added to `openWindow`/`requestFileOpen`/`requestMediaFrames`/`spawnPluginInstance`/`openDialog`/`dispatchAction`; `invokeExtension.responseAction` removed; 23 new variants added, typed `unknown`/`readonly number[]` for payload/pack-shaped fields per the file's existing convention). Two **pre-existing drifts** from the Rust side fixed while updating the hand-written twin to match exactly (as the packet's instructions require): `loadArtifact` (optional `pack?`/`spr?`/`artifactJson?`) → `loadDocument` (required `pack`/`spr`, matching `Effect::LoadDocument`'s actual shape), and `spawnPluginInstance.artifactJson?` → `documentJson?` (matching `document_json`). `InvocationResponse.requestedEffects?: readonly HostEffect[]` → `readonly Effect[]`.

### `🛂️manifest/🟦️component.ts`

One reference fixed: `PluginUiRefreshResponse.requestedEffects?: readonly HostEffect[]` → `readonly Effect[]`. **Note**: this reference has no visible import of `Effect`/`HostEffect` anywhere in this file (confirmed: no `import … from … kernel` line exists), so it was very likely already a dangling/unresolved-name error before this packet touched it — pre-existing, not introduced here; flagged for whoever owns this file's TS build config to confirm.

## peer-coexistence

Per the coordinator's amendment mid-task: a live peer session (ticket `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`, slice W1-D) has uncommitted work in three of this packet's owned files. All edits to these three were surgical `Edit` calls (re-read from disk immediately before each edit), never a wholesale rewrite:

- **`🎠️kernel/🟦️component.ts`** — peer's `🔖️IoRouter` region (`io_entries`/`resolve_io_route`/`run_io`/`identify` + its `describe("IoEntryGraph", …)` vitest block) verified **byte-identical** before and after this packet's work:
  - First line: `//#region 🔖️IoRouter`
  - Last line: `//#endregion 🔖️IoRouter`
  - Line count: **240** lines (559–798), unchanged
  - SHA-256 of lines 559–798: `ddb2ce7f1f8fb21ca2ebf6cb7934261e34e50fcce605455823c69ea19e8136a7` — identical before and after (checked immediately after the amendment landed, and again after all this packet's TS edits).
  - My edits to this file were two `Edit` calls, both well past line 1074 (the `HostEffect`→`Effect` type definition and the one `InvocationResponse.requestedEffects` field), nowhere near the peer's region.
- **`🎠️kernel/🦀️component.rs`** — peer's 1-line uncommitted change (`🔖️Presence` region: `PresencePoint, PresenceViewport` → `PresenceUi, PresenceViewKind, PresenceWindowView`) was already present on disk when I started (matched what I first `Read`); I never touched that line. All my edits are in the `🔖️Effect` region (replacing the old block) and a clean append at end-of-file.
- **`🛂️manifest/🟦️component.ts`** — peer's 3-line uncommitted addition (`WindowStackCorner` type + `WindowLayoutWindowNode.corner?`) was already present when I started; my one edit (`HostEffect`→`Effect` at the `PluginUiRefreshResponse.requestedEffects` field, line ~1011) is unrelated and doesn't touch those lines.

No other packet-owned file had peer state to preserve.

## Part 2 — mechanical rename

`HostEffect` → `Effect` at every call site found via `grep -rlE "\bHostEffect\b"` across `*.rs`/`*.ts`/`*.tsx` (excluding `🎯️target`/ticket dirs): **135 files**, done via a word-boundary `perl -pi -e 's/\bHostEffect\b/Effect/g'` pass (safe — no bare `Effect` identifier collision found anywhere except this packet's own new `kernel::Effect`, and one plugin's unrelated `fsm::Command::Effect` variant, always path-qualified so no ambiguity), followed by one manual fix the mechanical pass couldn't reach (`InvocationResult.requested_effects: Vec<HostEffect>` in `🎠️kernel/🦀️component.rs`, missed because it wasn't inside the enum block I replaced by hand). **132 files were mine to edit**; **3 are registrar-only** and are covered by `lease-request` blocks below instead.

`📦️packages/🦀️rust/📦️glue.rs` (the `semio_framework` crate root) additionally got `RequestId` added to its `pub use manifest::kernel::{…}` re-export list, so plugin call sites can name it as `semio_framework::RequestId` / `semio_framework_plugin::RequestId` (the latter via `🔌️plugin/🦀️component.rs`'s existing `pub use semio_framework::*;`) without a separate per-file import.

### `RequestId` threading — every site

The six variants that gained `req: RequestId` (`OpenWindow`, `RequestFileOpen`, `RequestMediaFrames`, `SpawnPluginInstance`, `OpenDialog`, `DispatchAction`) plus `InvokeExtension` (gained `req`, lost `response_action`) needed call-site handling beyond the bare rename. Classified every construction/pattern site (63 total, `OpenWindow`/`SpawnPluginInstance` have **zero live constructions** repo-wide — confirmed by `grep -rn "::OpenWindow\b"`/`"::SpawnPluginInstance\b"`, matching `📓️design-abi.md` §0's "zero guest callers today" note):

- **31 constructions** (building a new `Effect` value) → minted a literal `RequestId(N)` at the call site, `N` sequential starting at 100 (continuing from 1–8 used in `🛂️manifest/🦀️component.rs`'s own round-trip tests). These are **not** globally-unique real request ids — each is an independent hard-coded placeholder at its own call site (no shared state between sites), exactly as many real minting authority won't exist until the SDK's request registry (packet A2-abi-sdk, `📓️design-abi.md` §4) lands; noted here so no one mistakes the literal numbers for meaningful IDs.
- **6 patterns** destructured every field without `..` (`Effect::OpenDialog { dialog_id, args }` etc., 5× in `🪐️space/🗿️artifacts/🏠️home/**`/`🪐️space/🗿️artifacts/🪐️space/**`, 1× `🧩️puzzle`) → added `, ..` so the new `req` field doesn't need binding.
- **22 patterns** already used `..` → no change (verified, not just assumed — each was individually re-read).
- **4 `InvokeExtension` constructions** (`🌀️procedural2d`/`🧊️procedural3d` flow-eval-tick ×2, `🧊️procedural3d` editor's brep-tessellate, `🌊️flow` flow-eval-tick) → added `req: RequestId(101|104|105|106)`, removed the `response_action: "…".into()` field (its old redispatch string is now dead — no call site read it downstream in these 4 files, verified).

Full per-site table (path, line, variant, minted `RequestId` where applicable) is in `📓️req-id-mapping.txt` in this ticket folder.

### Acceptance

```
$ cd /Users/ueli/Documents/semio
$ export CARGO_TARGET_DIR=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target"

$ cargo check -p semio-framework --all-targets
    [... dependency build output omitted, full log: 📓️check-semio-framework.txt ...]
warning: value assigned to `pos` is never read
   --> 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/././../../🔨️modules/📡️spr/📡️wire/🦀️component.rs:448:13
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default
warning: `semio-framework-os-kernel` (lib) generated 1 warning
    Checking semio-framework-ui v0.1.0
    Checking semio-framework-mesh-engine v0.1.0
    Checking semio-framework v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 3m 52s
EXIT: 0
```

The one warning is pre-existing, in `📡️spr/📡️wire/🦀️component.rs` (not a file this packet touched, part of the `semio-framework-os-kernel` crate) — `git log --oneline -3 -- "🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs"` confirms it predates this session. **`semio-framework` itself (the crate containing every file this packet edited) produced zero warnings and zero errors.**

```
$ cargo check -p semio-framework-os-kernel --all-targets
    [... dependency build output omitted, full log: 📓️check-os-kernel.txt ...]
warning: `semio-framework-os-kernel` (lib) generated 1 warning   [same pre-existing `pos` warning]
warning: `semio-framework-os-kernel` (lib test) generated 10 warnings (1 duplicate)   [pre-existing dead-code warnings in 🗣️dsl/🧪️fixture-sweep/🦀️component.rs, not touched by this packet]
    Finished `dev` profile [unoptimized] target(s) in 3m 15s
EXIT: 0
```

Zero errors, zero warnings attributable to this packet.

```
$ cargo check --workspace --all-targets 2>&1 | tail -40
[PENDING — see below]
```

### TypeScript check of the two `🟦️component.ts` files

The repo's real typegen/TS pipeline (`bun ./📜️script.ts generate` in `🧰️framework/📦️packages/🦀️rust`, which runs `cargo test --features typegen exports_typescript_bindings` then diffs `🛂️manifest/🤖️generated/🟦️manifest.ts`) shares the same `CARGO_TARGET_DIR` lock as the checks above, so it's queued behind the workspace check — result below once it lands. As an independent, faster signal I also ran `node_modules/.bin/tsc --noEmit` directly against both files with the root `tsconfig.json`'s compiler options (no project-wide `paths`/bundler config, since that's Bun-specific, not raw-`tsc`-resolvable — so this run has real limitations, listed below). Exit 2, 140 errors, but **all but one are pre-existing/environmental**, verified by category:

- **113 errors** are resolution noise from running bare `tsc` without Bun's `.ts`-extension import support (`TS5097`) or without this repo's real module graph (`Cannot find name 'UiMenuRef'`/`'Label'`/`'UiPresence'`/etc. in files this packet never touched: `🤖️generated/🟦️manifest.ts`, `📦️typescript/🟦️glue.ts`, `🖥️platform/🟦️component.ts`, `🎨️styling/📦️index.ts`) — none reference `Effect`/`Event`/`RequestId`/any type this packet added.
- **1 error is exactly the pre-existing dangling reference already documented above**: `🛂️manifest/🟦️component.ts(1011,40): error TS2304: Cannot find name 'Effect'.` — this is `PluginUiRefreshResponse.requestedEffects`, which had the identical "no import, bare `HostEffect`" problem before this packet's rename (verified: no `import … kernel` line existed in this file beforehand either). The rename is faithful — it didn't fix or worsen this pre-existing issue.
- The remaining `🛂️manifest/🟦️component.ts` errors (`UiExternalSlotNode`, `UiComponentSceneNode`, `ContextMenuItemSpec`, a missing-return-statement at line 924) are unrelated names/functions this packet never touched.
- **Zero** errors mention `Event`, `RequestId`, `UiPatch`, `PatchOp`, `Budget`, `TurnResult`, `MessageEndpoint`, `RequestOutcome`, `JobPlacement`, or any `Broker`/`Quota`/`PackageDescriptor`-family name — the new types this packet introduced are internally well-formed.

Full output: `📓️tsc-check.txt` in this ticket folder.

## lease-request

Three files reference `HostEffect` (now dangling — the type no longer exists) and are registrar-only per `📌️important.md`, so I did not edit them. This is the entire reason `cargo check --workspace` cannot be fully green from this packet alone; every other crate in the workspace checks clean.

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs
reason: registrar-only ("Shell/🧊️component.rs (shared with live hover/selection tickets)"), 22 `HostEffect` references, mechanical rename only.
change:
  1. Rename `semio_framework::kernel::HostEffect` → `semio_framework::kernel::Effect` at all occurrences (lines 1680, 1683, 1686, 1689, 1698, 1701, 2949, 2952, 2958, 2973, 2982, 2995, 3032, 3033, 3037, 3042, 3049, 3552) and in 3 doc comments (412, 2549, 7252) — pure text substitution, no other change needed for those.
  2. Two `RequestMediaFrames` match patterns destructure every field without `..` and will fail to compile once `req: RequestId` is mandatory:
     - line ~1701: `semio_framework::kernel::HostEffect::RequestMediaFrames { accept, frame_action, done_action, fallback_action, sample_stride, max_frames, max_long_edge_px, fps_hint, payload, args } => {`
     - line ~2982: same shape, inside the second `queue_host_effects`-style loop
     Fix: append `, ..` before the closing `}` of each pattern (matching how this file's own `DispatchAction` arms already do it, e.g. line 1698's `{ action: dispatch_action_id, args, .. }`).
  3. All `DispatchAction`/`ReplayShellCommand`/`SetActiveUtility`/`Navigate`/`LoadDocument` patterns already use `..` or destructure only unaffected fields — no other change needed.
```

```lease-request
file: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx
reason: registrar-only per `📌️important.md`'s registrar list, 2 real type references + 1 field the removed-field change genuinely breaks (not just a rename).
change:
  1. `type HostEffect,` (line ~61) → `type Effect,`.
  2. `effects: readonly HostEffect[]` (line ~2631, `applyHostEffects`'s own param type) → `readonly Effect[]`.
  3. The `invokeExtension` branch (~line 2855) destructures `const { extensionId, capability, requestJson, responseAction } = effect.invokeExtension;` and later calls `makeEffectDispatchOne(requestingPlugin, baseSession, applyHostEffects)(responseAction, {...})` (~line 2870) — `responseAction` no longer exists on `invokeExtension` (design-intentional: `📓️design-abi.md` §2 "`response_action` disappears; the SDK resumes the awaiting future"). This is **not** a mechanical fix: the whole redispatch-by-action-id flow this branch implements needs replacing with a `req`-correlated completion once the host-side request registry exists (packet A2-abi-sdk / B1-host-native territory, not this packet's). Until that lands, the minimal compiling stopgap is dropping the destructured `responseAction` binding and the `makeEffectDispatchOne(...)(responseAction, {...})` call (i.e., `invokeExtension` becomes a no-op branch here), but that is a real behavior change someone who owns this file should decide on, not silently apply.
  4. Every other `"xInEffect" in effect` narrowing branch (`spawnPluginInstance`, `requestFileOpen`, `openDialog`, `dispatchAction`, etc.) destructures via plain TS object destructuring, which silently ignores properties it doesn't list (`req` included) — **no changes needed** for any of those; verified by reading each branch, not assumed.
```

```lease-request
file: 📜️script.ts
reason: registrar-only (root script, `important.md` "Registrar-only files").
change:
  `POLICY_HOST_EFFECT_CONSTRUCT_RE = /\bHostEffect::(\w+)\b/g;` (line 6816) drives the `APA/M5` capability-parity lint (`Effect`/capability parity: for each plugin, collects every construction without a declared capability`, line ~6821 doc). After this packet's rename, this regex matches **nothing** anywhere in the repo (zero `HostEffect::` constructions remain) — the lint silently goes from "real check" to "always passes", not a compile error but a real coverage regression. Fix: change the regex to `/\bEffect::(\w+)\b/g` (and update the doc/error-message strings at lines 6455, 6781, 6882, 6887, 6888 that say `HostEffect` for consistency — cosmetic, the regex is the only functional piece).
```

## Files touched

- **Hand-edited (5)**: `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`, `🎠️kernel/🟦️component.ts`, `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, `🛂️manifest/🟦️component.ts`, `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`.
- **Mechanical rename + (for 36 of them) `req`/`..` call-site fixes (129)**: full list with SHA-256 in `📓️file-hashes.txt` in this ticket folder (includes the 4 files where I additionally hand-fixed `InvokeExtension`'s `response_action` removal: both `🌀️procedural2d`/`🧊️procedural3d` `flow-eval-tick` command files, `🧊️procedural3d`'s own `editor/🦀️component.rs`, `🌊️flow`'s `flow-eval-tick` command file).
- **Not touched, `lease-request` filed instead (3)**: `📜️script.ts`, `Shell/🧊️component.rs`, `ShellHost/🟦️component.tsx`.

## anything deferred

1. **`CapabilityGrant`/`CapabilityRequirement`/`Rights`/`Scope` naming collision** (see above) — needs a registrar call on whether the legacy kernel-level action-capability model gets renamed/deprecated, or `Broker`-prefixed names (`BrokerCapabilityGrant`) stay permanent. Not a lease-request (no file outside my scope needs an edit for my packet to be correct) — a design decision for sol.
2. **`Event::InstanceOpen.actor: String` placeholder** — retype to the real `RuntimeActorId` once the `🎭️actor` crate lands (packet A1-actor). Tracked, not urgent — `Event` has zero constructors yet.
3. **`ContributionSet`'s 8 `DescriptorEntry`-typed categories** (`menus`/`file_types`/`panels`/`themes`/`inference_services`/`mutation_services`/`io_entries`/`composer_entries`) — no typed manifest model exists anywhere in the codebase for these yet; packet E1-describe (the packet whose own design section this table lives under) is the natural owner of giving each its real shape.
4. **`PackageDescriptor`/builder wiring** — this packet added only the data types per its Part 1 charter ("nothing else consumes them yet"); the `.activation(..)`/`.extension_point(..)`/`.requests(..)`/`.quota(..)`/`.execution(..)` builder methods `📓️design-abi.md` §3 also mentions are not implemented here (no `PluginBuilder`/`ExtensionBundle` file is in this packet's owned paths).
5. **3 `lease-request`s above** — `📜️script.ts`'s capability-parity lint regex, `Shell/🧊️component.rs`'s rename + 2 `..` additions, `ShellHost/🟦️component.tsx`'s rename + the `invokeExtension`/`responseAction` design-level rework.
6. **Workspace-wide `cargo check`** and the real typegen pipeline (`bun ./📜️script.ts generate`) were still running against a heavily-contended shared `CARGO_TARGET_DIR` (multiple concurrent sessions) at report-writing time — results appended above/below once they land; the two crate-scoped checks the acceptance criteria call out by name (`semio-framework`, `semio-framework-os-kernel`) both already passed clean.

No `[DEBUG]` logs were added by this packet — nothing to strip.

## final

- All three `lease-request`s were applied by sol, not by this packet: `📜️script.ts`'s `POLICY_HOST_EFFECT_CONSTRUCT_RE` now reads `/\bEffect::(\w+)\b/g`; `Shell/🧊️component.rs` has all 22 `HostEffect` → `Effect` plus `, ..` appended to both `RequestMediaFrames` patterns (lines 1701, 2982); `ShellHost/🟦️component.tsx` has `type Effect`/`readonly Effect[]`/the doc comment renamed, and the `invokeExtension` branch now destructures `req` instead of `responseAction`, still performs the invoke, and emits an explicit `console.error` naming the missing completion path (loud gap, not a silent no-op) — wiring it to a real `req`-correlated completion is now required work for packet H1-react.
- **Registrar ruling on the naming collision**: `BrokerCapabilityGrant` stays as named. The pre-existing `kernel::CapabilityGrant`/`CapabilityRequirement`/`Rights`/`Scope` action/window capability model is not being collapsed into the broker model by this packet — that's real design work belonging to whichever packet lands the broker end-to-end, and sol has recorded it as an open decision.
- **Workspace-wide `cargo check --workspace --all-targets` was handed to sol** — this packet stopped waiting on it (a subagent's background children don't survive its turn ending, so the wait could never have resolved here regardless). The two crate-scoped checks the acceptance criteria name explicitly (`semio-framework`, `semio-framework-os-kernel`) both already passed clean, pasted with exit codes above.
- **Two follow-ups for later packets**, both already listed under "anything deferred" above and repeated here per sol's request: (a) retype `Event::InstanceOpen.actor` from the placeholder `String` to the real `RuntimeActorId` once the `🎭️actor` crate lands; (b) give `ContributionSet`'s placeholder `Vec<DescriptorEntry>` categories (`menus`/`file_types`/`panels`/`themes`/`inference_services`/`mutation_services`/`io_entries`/`composer_entries`) their typed shapes in packet E1-describe.

Packet A3-kernel-types stops here.


