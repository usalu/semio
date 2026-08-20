# 📓️ terra-contract-doc — report

Packet: `contract-doc`, wave W1. Anchor commit `5e7b8046be`. Read `📌️important.md` (U1–U9) and
`📋️master.md` first, per the packet brief.

## done

Replaced the scaffold placeholders in both owned files, wholesale, keeping the `//! @emoji` header
style and the `//#region`/`//#endregion` structure (with subregions inside each):

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs` — region `🔖️Document`,
  subregions `🆔️Ids`, `🌳️Snapshot`, `🩹️Patch`, `🗄️SnapshotState`, `🧪️Tests`.
  - `SurfaceId(String)`, `UiNodeId(u64)`, `UiRevision(u64)` + `UiRevision::next`.
  - `UiNodeIdAllocator` — per-surface monotonic, never-reused `UiNodeId` source (the "helper" the
    packet's TEST section asks for).
  - `TransitionHint { Introducing, Celebrating }`.
  - `UiNodeRecord`, `UiSnapshot`, `UiPatch`, `UiPatchOp` exactly as specified in the brief's pseudocode.
  - `UiSnapshotState` (read-only receiver-side state: `surface`/`revision`/`root`/
    `nodes: HashMap<UiNodeId, UiNodeRecord>`, all fields `pub` so `contract-action`'s `apply_patch`
    can mutate them directly) with `new`, `get`, `root`, `revision`, `children_of`, `iter_subtree`
    (stack-driven preorder DFS, no recursive call) — plus a `From<UiSnapshot> for UiSnapshotState`
    convenience conversion.
  - In-file tests: `UiNodeIdAllocator` monotonicity/uniqueness, `UiRevision::next`, a 3-level nested
    `UiSnapshot` byte-identical JSON round-trip (serialize → deserialize → re-serialize → compare
    strings, not just structural `==`), `iter_subtree`/`children_of` ordering, and a round-trip for
    every `UiPatchOp` variant.

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️component.rs` — region `🔖️Component`,
  subregions `🏷️Label`, `🎛️Enums`, `🧱️Nested`, `🎨️Props`, `🧩️Enum`, `🧪️Tests`.
  - `Label(String)` — this crate's own minimal label type (see **decisions**).
  - `ContainerRole { Plain, Section, Group, Field, Form, Toolbar }`, `InputKind { Text, LongText,
    Number, Date, Color, File }`, `RowActionPlacement { Row, Menu }`.
  - `DropOverlaySpec`, `SelectItem`, `KeyValueEntry`, `RowAction` (nested value types referenced by
    the props below).
  - One props struct per `Component` variant: `ContainerProps`, `TextProps`, `ButtonProps`,
    `SeparatorProps`, `InputProps`, `SelectProps`, `ToggleProps`, `KeyValueListProps`, `SliderProps`,
    `NumberStepperProps`, `RingProps`, `IconSelectProps`, `TreeProps`, `TreeSectionProps`,
    `TreeItemProps`, `ImageProps`, `ExtensionProps` (`Surface` uses `crate::SurfaceProps` directly,
    no wrapper).
  - `Component` enum tying them together, `#[serde(tag = "type", rename_all = "camelCase")]`.
  - In-file tests: byte-identical JSON round-trip for every `Component` variant, plus `Label`
    conversion/`Display` sanity.

Every non-derived, non-test `fn`/method is tagged
`// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md` per U1.
No `async fn` anywhere. No `dyn` on any first-party trait (U3) — none needed.

## acceptance

**UNRUN**, per binding ruling U4 ("the coordinator owns every build... Executors write code and
reasoning, run only cheap non-cargo checks, and mark acceptance UNRUN"), which supersedes this
packet's own ACCEPTANCE section (that section pre-dates/conflicts with U4's repo-wide ruling). I did
not run `cargo check -p semio-framework-ui-contract --lib` or any other cargo command.

The command `sol` should run: `cargo check -p semio-framework-ui-contract --lib` (`timeout: 600000`).
**Expected outcome, not measured**: unresolved-name errors ONLY for the eight sibling names this
packet was told to leave unresolved (`crate::LayoutSpec`, `crate::StyleSpec`,
`crate::AccessibilitySpec`, `crate::ActionBinding`, `crate::MenuRef`, `crate::Activity`,
`crate::SurfaceProps`, and `crate::UiValue` — see **decisions** for why `UiValue` joins that list even
though the packet brief's explicit list omitted it) — that is success for this packet per the
brief's own ACCEPTANCE wording.

Cheap non-cargo checks actually performed:
- Brace/paren/bracket balance check (small Python scanner respecting `//`, `/* */`, string and char
  literals) on both files: `balanced OK` for both.
- `//#region` / `//#endregion` marker counts match in both files (6/6 in `document.rs`, 7/7 in
  `component.rs`).
- Grepped both files for any `//` line comment that is not a doc comment (`///`), a region marker, or
  the required U1 tag — none found, i.e. no comments inside function/method bodies (only the
  ticket-mandated U1 tags, which sit immediately above a signature, same placement precedent as the
  rest of the tree, e.g. `ui_styling`'s `📦️glue.rs`).

## decisions

1. **`Label` is NOT reused from the old wgpu-target UI package.** `crate::wgpu::Label`
   (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`) imports that
   package's own `Locale`/`Terminology` axes and exists to enforce compile-time-checked labels via
   `app_labels!`/`LabelText` (no `From<&str>`) — a policy that belongs at the authoring boundary, not
   the wire boundary, and which this crate cannot depend on anyway (no engine/wgpu deps, per
   `📦️glue.rs`'s own header). Defined a minimal independent `pub struct Label(pub String)` in
   `component.rs` instead, with `From<String>`/`From<&str>`/`Display` for ergonomics. Full reasoning
   and doc-comment cross-reference live at `component.rs`'s `🏷️Label` region.

2. **Icon fields are plain `String`, not `IconName`.** `IconName` is generated per-consuming-crate via
   a `#[path]` mount (`🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`), not a
   crate dependency — and `Cargo.toml` is registrar-only / forbidden for this packet. `ButtonProps`,
   `ToggleProps`, `TreeItemProps`, and `RowAction` all carry `icon`/`icon_id` as `String`. See
   **registrar-requests** below if a shared icon crate should exist instead.

3. **`InputKind` is `{ Text, LongText, Number, Date, Color, File }`** — not the packet brief's
   illustrative `{Text, Number, Search, File, Color, Date}`. I grepped the fleet rather than assume:
   Rust `UiInputNode.input_kind` literals in the tree are only `"text"`, `"textarea"`, `"number"`,
   `"file"`; the React `Interpreter` (`🧱️elements/Interpreter/🟦️component.tsx` ~L317) additionally
   branches on `"date"` and `"color"` (defaulting everything else to `"text"`). No `"search"` literal
   exists anywhere in the fleet (`.rs`/`.ts`/`.tsx`), so it is not in the closed set. I also closed a
   real pre-existing Rust/TS spelling mismatch: Rust emitted `"textarea"`, TS checked for
   `"longText"` — those never actually matched each other before this contract; `InputKind::LongText`
   is now the one canonical spelling both sides converge on.

4. **`crate::UiValue` is referenced but not defined here**, for `ExtensionProps.props`. The packet
   brief's explicit "sibling types to leave unresolved" list is `LayoutSpec`, `StyleSpec`,
   `AccessibilitySpec`, `ActionBinding`, `MenuRef`, `Activity`, `SurfaceProps` — it does not name
   `UiValue`. But `📋️master.md`'s "1. Contract crate" section places `UiValue` directly beside
   `ActionId`/`Trigger`/`UiIntent` (the action/value model), which reads as `contract-action`'s
   `🦀️action.rs` territory, not `contract-doc`'s. Since U2 explicitly calls a duplicate definition
   worse than a temporary unresolved name, I treated the brief's list as non-exhaustive and left
   `UiValue` unresolved too, rather than invent a ninth type here. **If `contract-action` does NOT
   intend to own `UiValue`, this needs a registrar/coordinator decision before W1 closes** — see
   registrar-requests.

5. **Only the two data-carrying enums (`Component`, `UiPatchOp`) use `#[serde(tag = "type", ...)]`.**
   The plain C-like enums (`ContainerRole`, `InputKind`, `RowActionPlacement`, `TransitionHint`) use
   bare `#[serde(rename_all = "camelCase")]` and serialize as plain strings, matching the precedent in
   the old `component.rs` (`UiState`/`UiStatus`/`UiTreeActionPlacement` are untagged; only
   `UiControlNode`/`UiNode`, which carry payloads, use `tag = "type"`). Applying `tag = "type"` to a
   unit-only enum would force `{"type":"plain"}` instead of `"plain"` on the wire, which is heavier
   and inconsistent with that precedent; I read the brief's blanket "enums use tag=type" line as
   scoped to payload-carrying enums, matching how `Component`/`UiPatchOp` are actually specified.

6. **`UiSnapshotState` does not derive `Serialize`/`Deserialize`/`ts-rs::TS`.** It is the retained
   receiver-side projection, not itself a wire type — `UiSnapshot`/`UiPatch` are the wire surface, and
   those are fully derived. Its `nodes: HashMap<UiNodeId, UiNodeRecord>` would also complicate ts-rs
   derivation (non-string map key) for no benefit, since nothing serializes this type directly.

7. **All per-variant action fields (`activate`, `on_change`, `drop_action`, `on_absolute`,
   `on_delta`, tree item `action`) moved off the props structs onto `UiNodeRecord.bindings:
   Vec<crate::ActionBinding>`**, per the brief's own schema (which gives `UiNodeRecord` a `bindings`
   field and does not repeat action fields on any props struct). `RowAction.action` also reuses
   `crate::ActionBinding` rather than a second parallel action-id type, since a row action is a
   binding fired unconditionally on click.

8. **Every `id: Option<String>` field on the old per-node structs (Stack/Button/Image/Input/Select/
   Toggle/Slider/NumberStepper/Ring/IconSelect/TreeItem/TreeSection/Field/Section/Group) was dropped**
   — `UiNodeRecord.key` is the one identity a renderer/reconciler needs now, and carrying a second,
   optional, differently-scoped id per component would reintroduce the ambiguity the flat keyed table
   exists to remove.

## registrar-requests

None required to land W1 — `Cargo.toml` was not touched and nothing here needs a registrar edit to
compile once the sibling packets land. Two open items for `sol`/the coordinator, not registrar edits:

1. Confirm `UiValue`'s owner (see decision 4) — if it is not `contract-action`, someone needs to add
   it to a file's scope explicitly before W1's `cargo check` can go green on that name.
2. If a shared icon-key type is ever wanted instead of plain `String` (decision 2), that is a new
   dependency decision for `sol`, not something this packet can request unilaterally since it touches
   `Cargo.toml`.

## deviations

- Packet ACCEPTANCE section asked the executor to run `cargo check` directly; I followed the more
  authoritative, later-stated U4 ruling instead and marked acceptance UNRUN. Noted explicitly so this
  isn't mistaken for skipped work.
- `InputKind`'s variant set differs from the brief's illustrative list (see decision 3) — grepped
  evidence, not a guess.
- `UiValue` referenced as unresolved despite not being in the brief's explicit sibling list (see
  decision 4) — flagged for coordinator confirmation.

## files touched

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️document.rs` (replaced scaffold)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️component.rs` (replaced scaffold)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📓️terra-contract-doc-report.md`
  (this file, new)

No other files were read-write touched. `Cargo.toml`, `📦️glue.rs`, and every sibling region file
(`🦀️layout.rs`/`🦀️style.rs`/`🦀️accessibility.rs`/`🦀️surface.rs`/`🦀️action.rs`/`🦀️presence.rs`/
`🦀️limits.rs`/`🦀️builder.rs`) were left untouched, per the packet's FORBIDDEN list.
