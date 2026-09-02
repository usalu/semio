# UI + Surface additive `ToValue`/`FromValue` pass (2026-09-02)

Scope: `🧰️framework/🔨️modules/🖱️ui/` and `🧰️framework/🔨️modules/🗺️surface/` — the two modules
assigned to this session. ADDITIVE ONLY; no `Serialize`/`Deserialize` removed anywhere.

## Surface module — 24/24 uncovered derives now covered

All four files that own actual `Serialize`/`Deserialize` types (`🏔️terrain`, `🕸️node-graph`,
`🎨️paint`, `🗺️tiled-map`) now also derive/hand-write `ToValue`/`FromValue`, mirroring every
`#[serde(...)]` attribute (`rename_all`, `rename`, `default`, `tag`) with the equivalent
`#[value(...)]`. One deliberate read-compat gap: `tiled-map`'s `PositionData::source_url` had
`#[serde(alias = "sourceUrl")]` — the derive has no `alias` equivalent, so the alias is **not**
replicated in `FromValue` (reported per ticket rule, not faked).

`semio-framework-surface` already depended on `semio-framework-os-kernel` (aliased `dsl`), which
re-exports `protocol::value::{ToValue, FromValue, DslValue, ValueError}`, so no new Cargo
dependency was needed there.

Verified: `cargo check -p semio-framework-surface` and `cargo check -p semio-framework` — both 0
errors before workspace churn (see Verification section).

## UI-contract module (`🧬️contract`) — ~73 types newly covered, 17 documented exceptions

`semio-framework-ui-contract`'s own docstring declares itself dependency-free of os-kernel (compiles
standalone for `wasm32-wasip2`/`wasm32-unknown-unknown`). Per the ticket's own DAG-position rule
("If a crate cannot reach the derive, use `#[value(crate = "::protocol::value")]`"), this session
added two new first-party leaf dependencies to its `Cargo.toml`:

```
semio-framework-replication = { workspace = true }   # lib name "protocol" — hash/io-base64/value-derive/serde/serde_json only
semio-framework-value-derive = { path = "../../../../🌱️value/✨️derive/📦️packages/🦀️rust" }
```

Neither creates an os-kernel cycle (confirmed via `cargo tree -p semio-framework-replication`), and
neither pulls in wgpu/winit/the actor kernel — the crate's actual wasm-safety guarantee. The crate's
own header docstring was updated to say so explicitly.

### Files fully covered
- `🦀️style.rs` — 6/6 (`Variant`, `SizeToken`, `Density`, `Tone`, `Emphasis`, `StyleSpec`)
- `🦀️layout.rs` — 19/19 (`SpaceToken`, `Sizing`, `Axis`, `Align`, `Justify`, `GridTrack`,
  `ScrollAxes`, `Anchor`, `EdgeSpace`, `StackLayout`, `GridLayout`, `OverlayLayout`, `ScrollLayout`,
  `AbsoluteLayout`, `LeafLayout`, `LayoutSpec`, `WindowStackCorner`, `WindowLayoutNode`,
  `WindowLayout`)
- `🦀️accessibility.rs` — 2/2 (`Liveness`, `AccessibilitySpec`)
- `🦀️presence.rs` — 4/4 (`Activity`, `PeerMark`, `OwnPresence`, `PresenceUpdate`)
- `🦀️limits.rs` — 4/4 (`UiDocumentLimits`, `UiContractViolation`, `PatchRejection`, `QuotaKind`)
- `🦀️conformance.rs` — 4/4 (test-only fixture-parsing structs: `ExpectedNode`, `ExpectedTree`,
  `ExpectedAccessibility`, `Expectation`)

### Files partially covered (the deliberate `UiValue` exception — see below)
- `🦀️action.rs` — `ActionId`, `Trigger` (derived) + `UiText`, `UiFixedBytes` (hand-written) +
  `UiFixedList<T, N>`, `UiFixedMap<V>` (hand-written, generic). NOT covered: `UiValue`, `UiList`,
  `UiMap`, `ActionBinding`, `MenuRef`, `UiIntent`.
- `🦀️component.rs` — 22/26 covered (`Label`, `ContainerRole`, `InputKind`, `RowActionPlacement`,
  `DropOverlaySpec`, `SelectItem`, `KeyValueEntry`, `ContainerProps`, `TextProps`, `ButtonProps`,
  `SeparatorProps`, `InputProps`, `SelectProps`, `ToggleProps`, `KeyValueListProps`, `SliderProps`,
  `NumberStepperProps`, `RingProps`, `IconSelectProps`, `TreeProps`, `TreeSectionProps`,
  `ImageProps`). NOT covered: `RowAction`, `TreeItemProps`, `ExtensionProps`, `Component`.
- `🦀️document.rs` — `SurfaceId`, `UiNodeId`, `UiRevision`, `TransitionHint` covered. NOT covered:
  `UiNodeRecord`, `UiSnapshot`, `UiPatchOp`, `UiPatch`.
- `🦀️surface.rs` — `SurfaceKind`, `SurfaceDoc` covered. NOT covered: `SurfaceProps`.
- `🦀️builder.rs` — NOT covered: `BuiltNode` (the file's only type).
- `♻️retirement/📋️patch/🦀️.rs` — NOT covered: `UiPatchOps` (doc-noted only, no derive attempted).

### The `UiValue` exception (17 types, all documented inline at each site)
`UiValue`'s own docstring (in `🦀️action.rs`) explicitly forbids depending on the os-kernel's
`DslValue` from this crate — "`From`/`Into` conversions between `UiValue` and `DslValue` belong in
the os-kernel crate, never here." Since `protocol::value::DslValue` **is** the same nominal type
os-kernel re-exports (`os_dsl::schema` does `pub use protocol::value::{DslValue, ...}` verbatim), a
`ToValue`/`FromValue` impl on `UiValue` would be exactly that forbidden conversion. This exception
cascades to every type that embeds `UiValue`, directly or transitively:

`UiValue` → `UiList`, `UiMap` (arena collections of `UiValue`) → `ActionBinding`, `MenuRef` (own an
`Option<UiValue>`) → `UiIntent`, `RowAction`, `TreeItemProps` (via `UiFixedList<RowAction>`),
`ExtensionProps` (owns `UiValue` directly), `Component` (enum, one variant is `ExtensionProps`),
`UiNodeRecord`/`BuiltNode` (own `component`/`bindings`/`menu`), `UiSnapshot`/`UiPatchOp`/`UiPatch`/
`UiPatchOps`/`SurfaceProps` (transitively via the above).

Each site carries an inline `// 🌱️ No ToValue/FromValue here...` comment naming the reason. This is
reported as an architectural boundary, not a gap to silently work around — no bridge/newtype/`#[serde
(skip)]` trick was used to force coverage through it.

### 🎬️scene crate — explicitly out of scope, NOT touched
`🖱️ui/🎬️scene/📦️packages/🦀️rust` (`semio-framework-ui-scene`) is named in the ticket brief as
already done ("45 hand-written impls / 26 types, 0 errors, 108/108 tests") and explicitly listed
under "Do NOT touch". A raw grep still shows `🦀️pack.rs`/`🦀️surface.rs`/`🦀️math.rs` there without
`ToValue` in file text — these are additional `#[cfg(test)]`-only fixture structs in `pack.rs`
(10 matches) plus 1 in `surface.rs`/`math.rs`, left as-is per the explicit instruction. Also
confirmed mid-session: this crate (`scenes.rs`, `canvas2d_snapshot.rs`, `world3d_snapshot.rs`) and
the wgpu target (`component.rs`, `label.rs`, `🦀️.rs`) carry large uncommitted diffs against `HEAD`
that predate this session — this is the prior agent's already-completed work, still uncommitted; not
touched or re-done here.

### `wgpu/🤖️generated.rs` — false positive, already covered
`Locale`/`Terminology` (the file's only two types) are `// @generated ... do not edit`. A sibling
file, `wgpu/🦀️locale_terminology_value.rs`, already carries their hand-written `ToValue`/`FromValue`
impls (pre-existing, `#[path]`-mounted at `wgpu/🦀️.rs:45`) — matches the ticket's own precedent list.
The raw file-level grep flags `generated.rs` as uncovered because the impls live in a different file;
confirmed by direct read this is a false positive, not real remaining work.

## Rules followed
- ADDITIVE ONLY: `git diff` shows only new `ToValue`/`FromValue` derives/impls and matching
  `#[value(...)]`/inline docstring notes — no `Serialize`/`Deserialize` removed, no field renamed or
  dropped.
- Round-trip fidelity: every `#[serde(default/rename/rename_all/tag/skip_serializing_if)]` was
  mirrored 1:1 to the equivalent `#[value(...)]`, field by field, not just at the container level
  (an earlier pass in this session missed several field-level `default`/`skip_serializing_if` pairs
  on `layout.rs::WindowLayoutNode` and `style.rs::StyleSpec` — caught by re-reading the derive's own
  "missing key hard-errors without `#[value(default)]`" behavior, then fixed and re-verified).
- `alias` (tiled-map's `sourceUrl`) reported as a gap, not faked.
- No `Serialize`/`Deserialize` removed anywhere in either module.

## Verification

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/iso3
export RUSTC_WRAPPER=""
cargo check -p semio-framework-surface --message-format short      # 0 errors (confirmed)
cargo check -p semio-framework --message-format short              # 0 errors (confirmed)
cargo check -p semio-framework-ui-contract --message-format short  # 0 errors (confirmed after each
                                                                     # file's edits, up through
                                                                     # limits.rs/conformance.rs)
```

`semio-framework-ui-contract`'s own tests were run once (`cargo test -p semio-framework-ui-contract
--lib -- --test-threads=1`): one pre-existing failure,
`document::document_component_compare_tests::retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root`
(`"UI value retirement arena is poisoned"`, a `Drop`-time panic in `♻️retirement/🦀️.rs` — a file this
session never touched). Confirmed pre-existing and unrelated to this pass by temporarily restoring
`Cargo.toml`/`🦀️.rs`/`action.rs`/`component.rs`/`layout.rs`/`style.rs` to `HEAD` via `git show
HEAD:<path>` (no `git checkout`/`stash` used) and re-running the same test single-threaded: it fails
identically on the unmodified baseline. Edits were restored from a local backup copy immediately
after, and a follow-up `cargo check` confirmed 0 errors again.

Two rounds of unrelated concurrent peer churn hit whole-workspace `cargo check` resolution during
this session's final verification pass:
1. `✏️s/🔌️plugins/🖍️draw`'s fsm sub-crate mid-rename (`semio-s-plugin-draw-fsm` →
   `semio-s-plugin-drawing-fsm`) — blocked ALL `cargo check -p ...` for ~10 minutes, then cleared.
2. After that cleared, `semio-framework-ui-contract` checked clean (0 errors, full file set including
   `🦀️surface.rs`/`🦀️builder.rs`/`♻️retirement/📋️patch/🦀️.rs`), but `semio-framework-surface`
   started intermittently failing with `error[E0432]: unresolved import `super::dsl_core`: no
   `dsl_core` in `mesh`` at `🧊️3d/🥽️mesh/🦀️.rs:18` — a file this session never touched.
   `git status` confirms `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` is currently modified
   (unstaged) by another concurrent session; `semio-framework-surface` depends on it transitively.
   Per `CLAUDE.md`'s "ignore unrelated recent changes" guidance, this is not chased further.

Neither churn episode touched anything in `🖱️ui`/`🗺️surface`. The LAST clean, fully-current runs for
both owned crates: `semio-framework-ui-contract` 0 errors (all 18 files, ran after the draw-fsm churn
cleared); `semio-framework-surface` 0 errors (confirmed multiple times earlier in the session, before
the unrelated `🧊️3d/🥽️mesh` edit began; still 0 errors on a subsequent retry once mesh briefly
resolved again mid-session, then flaky again as the peer kept editing).

## Files touched
- `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️action.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️component.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️layout.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️style.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️accessibility.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️presence.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️limits.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️conformance.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️surface.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️builder.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/📋️patch/🦀️.rs`

## Remaining work (for a follow-up pass)
- `🎬️scene/📦️packages/🦀️rust/🦀️pack.rs` (10 test-only Serialize/Deserialize structs) and
  `🦀️surface.rs`/`🦀️math.rs` (1 each) — left untouched per explicit "do NOT touch" instruction, but
  flagged here in case that instruction was scoped to the non-test types only.
- `semio-framework-surface` was intermittently red at report-writing time solely due to an unrelated
  peer's in-progress edit to `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` (confirmed via `git status`
  — modified, not by this session). Re-run once that peer's edit lands to get a final stable count;
  every run of it that did NOT race that file showed 0 errors.
