# 📓️ terra-contract-layout-report

Packet `contract-layout`, wave W1. Anchor commit `5e7b8046be`.

## Done

Replaced all four scaffold placeholders wholesale, keeping the `//! @emoji` headers and
`//#region`/`//#endregion` structure:

- `🧬️contract/📦️packages/🦀️rust/🦀️layout.rs` — region `🔖️Layout`: `LayoutSpec` (Stack/Grid/Overlay/
  Scroll/Absolute/Leaf) + `StackLayout` (exact shape from the brief) + `GridLayout`/`OverlayLayout`/
  `ScrollLayout`/`AbsoluteLayout`/`LeafLayout` + `Axis`/`Align`/`Justify`/`GridTrack`/`ScrollAxes`/
  `Anchor`/`Sizing`/`SpaceToken`/`EdgeSpace`. Region `🔖️WindowLayout`: `WindowStackCorner` (ported
  verbatim) + the new internally-tagged recursive `WindowLayoutNode { Window, Stack, Split }` +
  `WindowLayout { root }`. 8 tests: one serde round-trip per `LayoutSpec` variant, one tagged-enum
  round-trip for `WindowLayoutNode` (asserting the `"kind":"split"` tag literally), one for
  `WindowLayout` itself.
- `…/🦀️style.rs` — region `🔖️Style`: `StyleSpec { variant, size, density, tone, emphasis }`, all five
  closed enums (`Variant`, `SizeToken`, `Density`, `Tone`, `Emphasis`), all fields
  `#[serde(default, skip_serializing_if = "is_default")]`. 7 tests: default-spec serializes to `{}`,
  one "field omitted at default" test per field, one full-value round-trip.
- `…/🦀️accessibility.rs` — region `🔖️Accessibility`: `AccessibilitySpec { label, description, live,
  shortcut, hidden }` (no `role` field — role is implied by `Component` per the brief) + `Liveness`.
  4 tests: default omission, `live` omission, `hidden` omission, a `shortcut` round-trip.
- `…/🦀️surface.rs` — region `🔖️Surface`: `SurfaceId(String)`, `SurfaceKind` (15 variants ported
  VERBATIM with identical `#[serde(rename = "...")]` strings, including the `virtualFileSystem`
  camelCase outlier), `SurfaceDoc { bytes: Vec<u8> }`, `SurfaceProps { surface_id, controller_id,
  kind, pane_id, binding_id, doc_schema, doc, domain_id, domain_granularity_id }`. 2 tests: verbatim
  rename spot-check (`World3d` → `"world-3d"`, `VirtualFileSystem` → `"virtualFileSystem"`), a
  `SurfaceProps` round-trip with a non-empty `doc.bytes`.

No other files touched. `📦️glue.rs`, `Cargo.toml`, and the sibling region files were read only.

## Acceptance

**UNRUN**, per ruling U1 in this ticket's `📌️important.md` §U4: "Executors write code and reasoning,
run only cheap non-cargo checks, and mark acceptance UNRUN … `sol` runs every gate." I did not invoke
`cargo check -p semio-framework-ui-contract --lib`.

Cheap non-cargo checks performed instead, on the four owned files only:
- Brace/paren balance (python string count): 0/0 on all four files.
- `grep -n "async fn"` — zero real hits (only doc-comment mentions of the U1 rule itself).
- `grep -nw "dyn"` — zero hits.
- Manual re-read of every written file against the brief's exact struct/enum shapes.

The coordinator should run `cargo check -p semio-framework-ui-contract --lib` (and `--all-targets`)
with `CARGO_TARGET_DIR` in the scratchpad and an explicit 600000 ms timeout. Expected outcome per the
packet's own acceptance criteria: errors for unresolved `crate::UiNodeId`, `crate::UiValue`,
`crate::Label` (all referenced by path, none defined here) — that is success, not a defect in this
packet's code.

## Decisions

**Token names — the real ui/styling tokens do not have the scale the brief assumed.** Read
`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️tokens.json` and its `🤖️generated.rs` mount in full. That file
is a canvas/map/DAG rendering token set (colors, `strokes`, `radii`, `opacities`, per-feature
`metrics.board`/`.dag`/`.map`/`.cad`…) — not a semantic-UI-kit token set. Specifically:
- `spacing` has exactly two keys: `compact` (`0.2rem`) and `touch` (`0.275rem`). No `xs/sm/md/lg/xl`
  ramp exists anywhere in the file.
- `colors` has a real semantic subset usable as `Tone`: `primary`, `secondary`, `tertiary`, `danger`,
  `warning`, `info`, `success` (the rest — `dark`, `gray-*`, `light`, `black`, `white`,
  `indirect-handle` — are raw palette/theme-scaffolding entries, not semantic roles, so excluded).
- Nothing token-shaped backs `Variant`, `SizeToken`'s ramp, or `Emphasis`.

Given that, I made these calls, each documented in the file's own doc comments:
- `SpaceToken { None, Xs, Sm, Md, Lg, Xl, Xxl }` — the scale is literally what the packet brief
  specified (`"None,Xs,Sm,Md,Lg,Xl,…"`), not invented independently; it just has no tokens.json
  backing today. **Registrar request**: add an `xs/sm/md/lg/xl` spacing ramp to
  `🔣️tokens.json`'s `spacing` table so a later packet can wire `SpaceToken` to real values instead of
  a name-only placeholder.
- `Density { Compact, Standard, Touch }` — `Compact`/`Touch` are the two real `spacing` keys;
  `Standard` is the default absence-state between them.
- `Tone { Neutral, Primary, Secondary, Tertiary, Info, Success, Warning, Danger }` — the 7 real
  semantic color keys plus a `Neutral` default (no dedicated token — absence of an explicit accent).
- `Variant { Solid, Outline, Ghost, Plain }` and `Emphasis { Subtle, Regular, Strong }` and
  `SizeToken { Xs, Sm, Md, Lg, Xl }` — no tokens.json backing exists for chrome-treatment, prominence,
  or a component size ramp at all, so these are closed enums I chose directly (`SizeToken`'s one
  precedent is the wgpu target's old `StyleSpec.size: Some("md".into())` hit) rather than tokens I
  read from the file. Flagging per CLAUDE.md's "validate your assumptions" rule rather than silently
  presenting these as if they had the same tokens.json grounding as `Density`/`Tone`.
- `ui_styling` (the crate dependency already in `Cargo.toml`) is not imported by any of the four
  files — nothing in its generated `Board/Canvas/Chrome/MapPalette` structs is a spacing/size/tone
  enum reusable here; those are literally different token domains (canvas rendering, not a semantic
  UI kit). Confirmed the workspace has no `unused_crate_dependencies` deny lint, so this doesn't fail
  a build.

**`SurfaceId` ownership.** Re-read `document.rs` immediately before writing `surface.rs`: still the
empty scaffold (owned by concurrent packet `contract-doc`), no `SurfaceId` defined there at read time.
Defined `SurfaceId(pub String)` in `surface.rs` since `SurfaceProps.surface_id` needs the type and
nothing else currently provides it, with an explicit doc-comment flag: if `contract-doc` also lands a
`SurfaceId` (plausible — `UiSnapshot { surface, … }` in the master doc's field list may want the same
identity), that definition should win and mine becomes a `pub use` re-export. **Registrar/coordinator
request**: reconcile this when `contract-doc` lands, before both packets' outputs merge.

**`WindowLayoutNode` recursion and ts-rs.** Checked the vendored `ts-rs 10.1.0` source directly
(`~/.cargo/registry/src/…/ts-rs-10.1.0/tests/integration/self_referential.rs`) rather than assuming.
It has an explicit passing integration test for an internally-tagged (`#[serde(tag = "tag")]`)
recursive enum with `Vec<Self>`/`Box<Self>` fields — exactly `WindowLayoutNode`'s shape. **ts-rs does
derive it.** I could not get an end-to-end confirmation via `cargo check --features typegen` on this
crate specifically (forbidden from running cargo per U4, and it would fail anyway right now on the
unresolved sibling types), but the library-level guarantee is solid enough to build on without
flagging it as a risk.

**`SurfaceKind`'s `virtualFileSystem` inconsistency.** Left it exactly as in the wgpu source (camelCase
against 14 kebab-case siblings) because the brief demands a VERBATIM port with identical serde
renames. This is exactly the kind of inconsistency CLAUDE.md otherwise tells me to refactor on sight —
not refactored here only because the packet's own instruction is more specific and overrides the
general rule for this one field.

**Not ported from the wgpu source:** `SurfaceKind::as_str`/`is_viewport` impl methods (both `async fn`
in the original) — the brief asked only for the 15 kinds + renames, not the helper methods, and they'd
need a U1 rewrite to sync `fn` with no clear home yet; left for whichever later packet needs them.

**`EdgeSpace` representation.** Used an externally-tagged enum (`All(SpaceToken)` /
`Symmetric{vertical,horizontal}` / `Each{top,right,bottom,left}`) rather than internally-tagged
(`#[serde(tag = "form")]`), because serde's internally-tagged representation requires the variant
payload to serialize as a map, and `All`'s payload is a bare `SpaceToken` (serializes as a string) —
that combination is a serde derive error. Externally tagged sidesteps it and still gives the "uniform
case costs one token" property the brief asked for.

**Window split `size: Option<f64>`.** Kept as a fraction (`0.0..1.0` of the parent split), not wrapped
in `SpaceToken`, because it's a proportion/weight, not a pixel or spacing measurement — the
"no pixel geometry" rule targets `LayoutSpec`'s metrics, and this is a different vocabulary
(`WindowLayoutNode`) describing window-manager split ratios, matching the old code's own `Option<f64>`
size field.

## Registrar-requests

1. Add an `xs`/`sm`/`md`/`lg`/`xl` spacing ramp to `🔣️tokens.json`'s `spacing` table (currently only
   `compact`/`touch`) so `SpaceToken`/`SizeToken` can eventually resolve to real values.
2. Reconcile `SurfaceId` between this packet's `surface.rs` and `contract-doc`'s `document.rs` once
   both land — see the ownership decision above.

## Deviations

None from the OWNS/FORBIDDEN boundaries. The one deliberate content deviation (not silently applying
CLAUDE.md's "refactor inconsistencies on sight" to `SurfaceKind`'s `virtualFileSystem` rename) is
explained above and is required by the packet's own VERBATIM instruction, not a violation of it.
