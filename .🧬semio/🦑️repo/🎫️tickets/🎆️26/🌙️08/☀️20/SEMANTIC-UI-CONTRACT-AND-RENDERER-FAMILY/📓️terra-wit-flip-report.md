# 📓️ terra-wit-flip-report

## done

Flipped `component.wit`'s `interface ui` from path-addressed to node-id-addressed patches, and made
`🎠️kernel/🦀️component.rs` re-export the contract crate's `UiPatch`/`UiPatchOp` instead of declaring
its own copy.

### `component.wit` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`)

- `interface ui`: replaced `patch-replace`/`patch-insert-child`/`patch-remove-child`/`patch-set-props`
  + the 4-variant `patch-op` with `type node-id = u64` and 9 records (`patch-upsert`,
  `patch-set-component`, `patch-set-layout`, `patch-set-activity`, `patch-set-children`,
  `patch-set-style`, `patch-set-accessibility`, `patch-set-bindings`, `patch-set-menu`) + an
  11-variant `patch-op` (adds `remove(node-id)`, `set-root(node-id)`), exactly per the packet's
  literal spec. `ui-patch` loses `kind: string`.
- `interface events`: added `record ui-intent-event { instance: instance-id, intent: pack }` and a
  `ui-intent(ui-intent-event)` arm in `variant event`, placed right after `app-command`.
- `interface reactor`: added `record presence-update { peer: pack }` and a
  `presence: list<presence-update>` field on `turn-result`; `use types.{plugin-error}` widened to
  `use types.{plugin-error, pack}`.
- `patch-ack-event`/`patch-rejected-event` were **already** unchanged-shape with a `reason: string`
  field on `patch-rejected-event` — no edit needed, confirmed by reading them fresh.

### `🎠️kernel/🦀️component.rs`

- `//#region 🔖️UiPatch` (was ~L859–884): deleted the local `UiPatch`/`PatchOp` structs, replaced with
  `pub use semio_framework_ui_contract::{UiPatch, UiPatchOp};`. `TurnResult.ui_patches: Vec<UiPatch>`
  needed no edit — same name, now resolves to the re-exported type.
- Added `Event::UiIntent { instance: PluginInstanceId, intent: Vec<u8> }` right after
  `AppCommandEvent`, matching its shape/derive/doc conventions.
- No `TurnResult.presence` field was added here — the packet's kernel-file GOAL section named exactly
  two edits (delete/re-export UiPatch+PatchOp, add `Event::UiIntent`) and nothing about `TurnResult`,
  so I stayed surgical rather than scope-creeping. This means `component.wit`'s new
  `turn-result.presence` field and the Rust `TurnResult` SSOT are now out of sync — flagged below and
  in the consumer inventory as a follow-up.
- No new functions were added (only type re-export + enum variant), so R2's async-literal rule never
  came into play here — nothing to tag.

## acceptance: UNRUN (+commands)

Per U4, I do not run cargo. Cheap checks only:
- WIT read back in full after each edit (`sed -n` over the changed regions) — kebab-case identifiers,
  brace/comma balance, and `use` clauses checked by eye against the file's own existing style.
- Rust regions read back in full after each edit — braces balanced, docstring emoji present, no
  comments inside function bodies (none were added — no function bodies changed).
- No `rustc`/`cargo`/`wasm-tools` invoked.

## peer-coexistence

```
$ git log --oneline -3 -- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit
6cf8d6c858 🐙️ueli🎆️26🌙️06☀️04🚩️545
3966c824fa 🐙️ueli🎆️26🌙️06☀️04🚩️538
f69271685f 🐙️ueli🎆️26🌙️06☀️04🚩️537

$ git log --oneline -3 -- 🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs
cb9bcce7a4 🐙️ueli🎆️26🌙️06☀️04🚩️544
d16fc1017c 🐙️ueli🎆️26🌙️06☀️04🚩️542
153db71c51 🐙️ueli🎆️26🌙️06☀️04🚩️531

$ git status --porcelain -- <both files>
(empty before I started — no uncommitted peer work in either file)
```

Both files were clean in the working tree when I started (no uncommitted peer diffs to absorb). I
still re-read each region fresh from disk immediately before every `Edit` call (5 separate re-reads
across the two files), per U2/the collision rule — no peer edits landed mid-session; every re-read
matched what I'd already seen. Nothing needed to be absorbed or preserved beyond what's already in
this report.

Also confirmed via the kernel file's own docstring (kept intact, just above the region I edited) that
`🎠️kernel/🦀️component.rs` is `#[path]`-mounted into **three** crates — `semio-framework` (root),
`semio-framework-graph`, and `semio-s-plugin-stdio` — verified fresh with
`grep -rn '#\[path.*🛂️manifest/🦀️component'` (the file is mounted into `🛂️manifest/🦀️component.rs`,
which is itself mounted into those three). This is why the registrar-request below is three lines,
not one.

## the consumer inventory

Grepped for `PatchOp::`, bare `PatchOp`, `UiPatch`, `wit_ui::Patch*`, `insert-child`/`set-props`/
`remove-child`, `PatchTracker`, `emit-patch`/`emit_patch`, `RenderedUi` (no hits) across `*.rs`/`*.ts`/
`*.tsx`/`*.wit`. Grouped by crate/package; `file:line` is the anchor as of this session (paths are
repo-relative).

### Hard compile breaks (the `use` statement itself fails — `PatchOp` no longer exists under that name)

- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️component.rs`**
  - `:30` — `use semio_framework::kernel::{Effect, Event, MessageEndpoint, PatchOp, RequestOutcome, TurnStatus, UiPatch};` — `PatchOp` unresolved.
  - `:711–723` — `kernel_ui_patch_to_wit`: reads `patch.kind` (field gone) and `patch.surface` as `String` (still a String on the kernel side pre-flip; contract's `UiPatch.surface` is now `SurfaceId`, so this whole function's input type shape changed too).
  - `:727–735` — `kernel_patch_op_to_wit`: matches `PatchOp::{Replace,InsertChild,RemoveChild,SetProps}` (all four gone) against `wit_ui::PatchOp::{Replace,InsertChild,RemoveChild,SetProps}` (also gone, replaced by the 11-variant node-id set) — needs a full rewrite to the new variant set, and needs a `node-id` source (this crate currently has no node identity, only `UiNode` tree diffing — see `PatchTracker` below).
  - `:738` (doc comment) references the now-dead `path: String`/`path: list<u32>` split.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`** (`PatchTracker`)
  - `:14` — `use semio_framework::kernel::{PatchOp, UiPatch};` — `PatchOp` unresolved.
  - `:48` — `PatchTracker::diff` constructs `UiPatch { surface, kind: "root".to_string(), revision, base_revision, ops: vec![PatchOp::Replace { path: String::new(), node: body }] }` — `kind` field and `PatchOp::Replace` both gone. This tracker currently diffs a whole `UiNode` tree (not a node-id-keyed flat table), so a real fix means either (a) rewriting it to emit `UiPatchOp::Upsert` of a pack-encoded `UiNodeRecord` root, keeping the "full-body only" scope this file's own header doc already admits to, or (b) waiting for the flat `UiSnapshotState` reconciler (`contract-doc`/`runtime-reconcile`) to replace `UiNode`-tree diffing here entirely. Explicitly named by the packet brief as expected breakage.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs`**
  - `:108` — `use semio_framework::kernel::{..., PatchOp, ..., UiPatch as KernelUiPatch};` — `PatchOp` unresolved.
  - `:494` — `apply_ui_patch`: `match patch.ops.as_slice() { [PatchOp::Replace { path, node }] if path.is_empty() => Some(node.clone()), _ => None }` — dead pattern, needs rewriting against `UiPatchOp::Upsert(record)` (now a whole `UiNodeRecord`, pack-encoded, not a raw `UiNode`).
  - `:301–304` (doc comment) documents the dead `PatchOp::Replace{path:"",node}`-only assumption.
- **`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🦀️component.rs`**
  - `:98–103` — synthesizes `UiPatch { surface: SurfaceRef{..}, kind: "replace".to_string(), ..., ops: vec![PatchOp::Replace(PatchReplace { path: Vec::new(), node: patch.bytes })] }` using the `wit_ui`-generated bindgen types directly (tuple-variant style) — `kind` field and `PatchOp::Replace`/`PatchReplace` all gone from the regenerated bindings; this load-test fixture needs to emit `patch-upsert` instead.

### Type-level only (compiles once bindings regenerate; no field access on the removed shapes)

- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`** `:858,940,1193` and
  **`⏳️imports.rs`** `:90,136,578` — `emit_patch(&mut self, patch: wit_ui::UiPatch)` /
  `Vec<wit_ui::UiPatch>` sinks that only push/drain, never destructure a field. Already documented in
  this file (`:1189–1200`) as deliberately unimplemented pending exactly the node-id/pack encoding
  this packet now settles — `ui_patches: Vec::new()` unconditionally. Low risk, but the follow-up that
  wires real marshaling needs the new `UiPatchOp` variant set, not the old path/`UiNode` one.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`** `:115` —
  `fn emit_patch(&mut self, _patch: actor_bindings::semio::framework::ui::UiPatch)` — parameter
  ignored (`_patch`), just an `eprintln!`. Recompiles clean once bindings regenerate.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️schema-parity/🦀️component.rs`** — validates
  `emit`/`emit-patch` are sync fire-and-forget (`:257,349,398,406`); does not inspect `ui-patch`'s
  record shape at all (confirmed: zero hits for `ui-patch`/`patch-op`/`surface-ref` in this file) — not
  a consumer of the changed shape, unaffected.

### `📡️spr/🧵️channel` — the shell-facing wire frame (own struct, not a hard break, but now stale)

- **`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`**
  - `:327–345` — `AppFrame::UiPatch { in_reply_to, surface: String, kind: String, revision, base_revision, ops: Vec<u8> }`. Its own docstring says `surface`/`kind`/`revision`/`base_revision` mirror `kernel::UiPatch` "field-for-field" and `ops` is pack-encoded `Vec<PatchOp>` reused from the kernel — both claims are now false (`kind` is gone kernel-side, `PatchOp` is `UiPatchOp` with the new variant set). Does not hard-fail to compile (independent struct), but is now a silently-stale wire frame.
  - `:870,957` — construct/pattern-match this variant (byte-level `pack`/`unpack` helpers), carries `kind` through unchanged.
  - `:1311,1319,1333,1508,1579` — tests, including a literal golden hex string at `:1579` that encodes a `kind` byte sequence and will need re-baselining once `kind` is dropped from the wire frame.

### TypeScript kernel mirror and reconciliation (`🎭️actor`, `os`, wgpu bridge)

- **`🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🖼️wire-turn.ts`**
  - `:47–52` — `WireUiPatch` type carries `kind?: string` (stale field) and no `presence` sibling on `WireTurnResult` (`:55–58`, missing the new `turn-result.presence`).
  - `:85–91` — `export type PatchOp = Replace|InsertChild|RemoveChild|SetProps` — full old-shape TS mirror, needs replacing with the 11-op node-id set (mirroring `UiPatchOp.ts`, the ts-rs-generated bindings that already exist in the contract crate's `bindings/` folder — this file should probably just import/re-export those instead of hand-mirroring, but that's a design call for the follow-up packet).
  - `:94–120` — `decodeWirePatchOps` switches on kebab-case `"insert-child"`/`"remove-child"`/`"set-props"` tags (all renamed/restructured in the WIT).
  - `:128–141` — `applyUiPatchToRetained` checks `op.kind === "Replace" && op.path.length === 0` — dead condition once `Replace`/`path` are gone.
- **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🐚️plugin-bridge.ts`** `:66,71,114,116` — consumes `decodeWirePatchOps`/`WireUiPatch` from the file above; breaks transitively, not independently.
- **`🧰️framework/🛍️products/💻️os/🟦️component.ts`** (the `📡️spr/🧵️channel` TS mirror)
  - `:1193` — `AppFrame.UiPatch` wire type literal includes `readonly kind: string`.
  - `:1659` — `writeStr(out, frame.UiPatch.kind);` (encoder).
  - `:1775` — `return { UiPatch: { in_reply_to, surface, kind, revision, base_revision, ops } };` (decoder).
  - Mirrors the `📡️spr/🧵️channel` Rust struct above field-for-field; same staleness, not a hard TS compile error (structurally typed), but wrong once the Rust side drops `kind`.
- **`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`** `:160–161,241–242` — re-exports the **already-current** ts-rs-generated `UiPatchOp`/`UiPatch` bindings (`bindings/UiPatchOp.ts`, `bindings/UiPatch.ts`, generated from the contract crate, which was already node-id-shaped before this packet). **Not stale** — flagging only so the follow-up packet doesn't waste time re-checking it.

### Kernel Rust ↔ WIT SSOT gap this packet leaves open

- `component.wit`'s `reactor::turn-result` now has `presence: list<presence-update>`; the Rust
  `TurnResult`/`Budget` SSOT in `🎠️kernel/🦀️component.rs` (the same region I touched for `Event`)
  does **not** yet have a matching `presence: Vec<PresenceUpdate>` field or `PresenceUpdate` type,
  because the packet's kernel-file GOAL section didn't list it. Whoever wires `reactor::poll`'s real
  marshaling (the `kernel_turn_result_to_wit`/host-native paths above) needs this field added first.

## decisions

- **`presence-update` is `{ peer: pack }`**, pack-encoding `📡️replication/📡️wire::PresencePeer` (which
  already has `encode_presence_peer`/`decode_presence_peer`), rather than re-declaring the peer/
  cursor/selection shape a second time in WIT. Consistent with how every other structured payload in
  `interface ui`'s `patch-op` is `pack`, not a nested WIT record — the schema's established convention
  for "complex payload = pack, don't duplicate the type."
- **Declared `presence-update` inside `interface reactor`**, not `interface ui` or `types`, because
  `turn-result` is its only consumer — same locality convention `budget`/`turn-status` already follow
  in that interface.
- **`patch-set-activity` is `{ node: node-id, activity: pack }` only, no `disabled: bool`** — this is
  the packet's literal given spec, but it does **not** match the contract crate's own
  `UiPatchOp::SetActivity { id, activity, disabled }` (`🦀️document.rs:148`), which carries `disabled`
  as a sibling field. I followed the packet's literal WIT text rather than "fixing" the mismatch
  unasked, per U2's "surgical, region-scoped... never a redesign" framing — but this is a real
  discrepancy between the WIT schema and its own Rust contract crate that the next packet touching
  either side should resolve (either fold `disabled` into `pack`-encoded `activity`, or add a
  `disabled: bool` field to `patch-set-activity`).
- **`ui-intent` was added as a new event variant alongside `app-command`, not a replacement.**
  `app-command-event` still exists for genuine `protocol_channel::AppCommand` channel commands; only
  UI node interactions move to the new variant, matching the packet goal's own wording ("UI intents
  stop masquerading as generic `app-command`").
- Kept `patch-ack-event`/`patch-rejected-event` untouched — the goal said "unchanged in shape... should
  be able to carry a reason," and `reason: string` was already present on `patch-rejected-event`
  before I started. No edit needed; verified rather than assumed.

## registrar-requests

The kernel file's `pub use semio_framework_ui_contract::{UiPatch, UiPatchOp};` needs
`semio-framework-ui-contract` as a dependency on **all three** crates that `#[path]`-mount
`🎠️kernel/🦀️component.rs` (via `🛂️manifest/🦀️component.rs`) — verified fresh this session (see
peer-coexistence above). Exact lines, matching each manifest's existing dependency style:

`🧰️framework/📦️packages/🦀️rust/Cargo.toml` (`semio-framework`, `[dependencies]`):
```toml
semio-framework-ui-contract = { path = "../../🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
```

`🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/Cargo.toml` (`semio-framework-graph`, `[dependencies]`):
```toml
semio-framework-ui-contract = { path = "../../../🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
```

`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (`semio-s-plugin-stdio`, `[dependencies]`):
```toml
semio-framework-ui-contract = { path = "../../../../../🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust", package = "semio-framework-ui-contract" }
```

All three relative paths were verified to resolve (`ls` onto each target `Cargo.toml`) before writing
this request.

**Why this is safe to add**: `semio-framework-ui-contract`'s own manifest
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml`) depends on only
`ui_styling` (path dep, `semio-framework-ui-styling`), `serde`, and an optional `ts-rs` — no tokio, no
wasm-incompatible crate, and it already compiles for both `wasm32-wasip2` and `wasm32-unknown-unknown`
per its own doc comment. Adding it to these three crates (two of which — `semio-framework` and
`semio-s-plugin-stdio` — already pull in `ui_wgpu`/`semio-framework-ui` transitively, a much heavier
dependency) does not drag anything new into the wasm plugin build graph. This is exactly the kind of
new kernel dependency a reviewer should independently confirm rather than take my word for, given it
now fans out to three crates instead of the one the packet brief implied.

## deviations

- The packet brief said "put the exact line" (singular) for the registrar-request; I'm reporting three
  lines because the kernel file is mounted into three crates, not one — confirmed by re-reading the
  file's own docstring and re-running the `#[path]` greps fresh rather than trusting the brief's
  phrasing. Flagging this explicitly since a reviewer approving only one Cargo.toml edit would leave
  two crates red.
- Did not touch `TurnResult`/`Budget` in the kernel file to add `presence` — see "the consumer
  inventory"'s closing note. Read as staying inside the packet's literal GOAL list rather than as an
  omission; flagging in case the coordinator intended it implicitly.
- Did not touch `📡️spr/🧵️channel/🦀️component.rs`, any TypeScript file, `⚛️reactor/🦀️component.rs`,
  `⚛️reactor/🩹️patches/🦀️component.rs`, the wgpu `📦️glue.rs`, or the scale fixture — all FORBIDDEN
  (guest SDK, reactor, plugin fleet, host, UI crates, root manifest) or simply out of this packet's
  OWNS list. Every one of them is in the consumer inventory above instead.

## files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`
- `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs`
