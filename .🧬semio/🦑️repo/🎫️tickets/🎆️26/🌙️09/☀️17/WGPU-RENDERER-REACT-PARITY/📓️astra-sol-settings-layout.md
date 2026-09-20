# Sol Settings Layout Repair

## Scope

This packet implements P1 from `📓️astra-terra-settings-visual-audit.md`:

- project General Settings through the shared `UiTree` grammar used by React;
- keep each setting's stable id, value, options, action, boot-lock omission, and authored section-open default;
- place bottom-anchor retained content above its tab path and use the same flow-owned coordinates for paint, hit registration, preview, and root-row drop targeting;
- repair the footer overlap through generic anchor geometry rather than a Settings-specific pointer exception.

Generic retained `Section` collapse, Select overlay origin, and accessibility visibility remain the separately bounded P2–P4 packets.

## Failure mechanism

`build_settings_general_ui` previously translated React property rows into `Stack → Section → Field → Select`. That changed both the contract and layout: controls were label-above-field, and the Driver editor's authored closed state did not remove its children from the generic Section layout.

The panel shell then ignored `anchor.flow().block`. It always put tabs at `panel.y`, moved content below them, and registered the retained body's pick clip there. For a bottom anchor, the resulting tall General document reached the footer; a footer close press could therefore remain under retained Driver control authority.

## Product repair

`build_settings_general_ui` now publishes one `UiNode::Tree`:

- the optional app section is open;
- General is open and uses inline `UiControlNode` cells;
- Driver is a closed `UiTreeSectionNode` with all seven axes, save name, save action, and conditional delete action;
- existing framework actions and boot-lock gates are unchanged.

The anchor shell now derives content, tab-row origins, the content/tab divider, and root-row drop targeting from `anchor.flow().block`. Upward bottom anchors reserve the tab path at the bottom, put the root row lowest and child rows above it, and clip the retained document to the content rectangle above those rows. `render_panel_step` already uses that rectangle for the document scissor, `pick_clip`, and retained-body hit registration, so the geometry correction also repairs input ownership without a product-specific branch.

## Schema and laws

Added the language-neutral `⚙️settings-general-layout` schema and fixture. The fixture records the React section/item/control identities and an independent two-row bottom-panel geometry example.

Added:

- a TypeScript Ajv schema law;
- an independent upward-flow geometry oracle;
- a React source oracle for `buildSettingsGeneralTree` and the bottom Panel's reversed block flow;
- a WGPU source-wiring law;
- Rust laws against the actual General producer and the actual flow geometry helpers, including the retained-content/footer boundary.

The Rust laws are mounted in the Shell native suite. The TypeScript law is mounted in the existing React Vitest configuration.

## Validation

The first focused TypeScript run produced the intended WGPU red result because General was not a Tree and the flow helpers did not exist. It also exposed an oracle mistake: React authors the seven Driver rows through `driverAxisSelectRow(key, …)`, so their complete ids are dynamic source output rather than seven literals. The oracle was corrected to inspect those seven actual calls.

After the product repair:

- focused Bun+Nx Vitest: **1 file passed, 4 tests passed**;
- both JSON files parse through `python3 -m json.tool`;
- the Shell production source and new Rust law parse through `rustfmt --emit stdout`;
- scoped `git diff --check`: clean.

Root's canonical WGPU11 build subsequently completed successfully. Native18 compiled the P1 source
and discovered two integration-test collector failures before a complete suite verdict:

- `settings_general_publishes_all_react_driver_controls_and_localized_dirty_state` serialized the
  new Tree and then mistook the row's `items` for a Select's option list;
- `the_general_leaf_offers_reacts_merge_policy_selector` traversed only the former
  Stack/Section/Field shape, so it could not see a Tree item's inline Select.

Both collectors now traverse `UiTreeItemNode.control` and nested Tree items through the typed Rust
model. Their original assertions remain intact: all seven Driver axes still compare complete option
sets with the neutral fixture, and merge policy still compares React's three exact values. Both
updated native test sources parse through `rustfmt --emit stdout`. Root's Native19 rerun passed both
repaired collectors. It executed 1,173 tests and
reported 1,172 passes; its only remaining failure belonged to the independent Display-drop packet.
Native19 therefore closes P1's typed collector integration gap without weakening either assertion.

Paired checkpoint 11 still reproduced a runtime close failure for Settings. Its control census
proved a different generic ownership bug: WGPU painted the open panel's root `PanelTab` at
`[1300, 974.4, 67.69668, 22.4]` over the footer-owned Settings `Toggle` at
`[1285.0319, 974.4, 67.696655, 22.4]`. P2 moves the root row back to the bonded footer/navbar
owner; that follow-up is documented in `📓️astra-sol-settings-section-collapse.md`.

No Cargo, native renderer, wasm build, or browser run was launched by this agent. Root owns those gates. P1's producer and native collector laws are green; runtime layout/close parity remains pending the P2 browser checkpoint.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧬️schema/⚙️settings-general-layout/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/⚙️settings-general-layout/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚙️settings-general-layout/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`

## Compact General extent follow-up

The paired checkpoint 14 runtime isolated a second lifecycle issue after the Tree and upward-flow
repairs. Opening General initially painted a full-height panel glass from y=29 through y=953 even
though its labels and physical controls occupied the correct lower band. Opening Appearance caused
the same panel to shrink to y=610 through y=953. That later interaction did not change General's
intrinsic content; it merely caused another retained layout/paint cycle. The evidence therefore
identified a missing accepted-layout handoff on the first Shell panel walk, rather than a wrong Tree
measurement.

The Shell panel walk now preflights its retained document through ordinary ingress, reconciliation,
and layout while no accepted intrinsic height exists. Once that document owns an accepted layout,
the Shell recomputes the bottom-anchored panel rectangle from the accepted content height, restarts
the viewport-dependent document phases, and only then commits panel glass, tab chrome, content
clip, paint, and hit ownership. A panel with no retained document or no accepted measurement keeps
the existing full-band fallback. The repair adds no Settings-specific coordinate and no new
scheduler lane.

The neutral fixture now carries the 1440×1000 bottom-right General vector, a material unused band,
and React's bottom-bonded/max-height/justify-end rules. Its native law runs the actual
`build_settings_general_ui` producer through publication and the complete stepped Shell panel walk.
It requires the first committed glass to be compact, a second Shell cursor to reproduce the same
bonded rectangle, every retained owner to use the compact content rect, the real Language target to
remain live, and the former unused band to carry no General hit. The React/Ajv oracle checks the same
fixture and production Panel/anchor source. Focused Bun+Nx Vitest passed **1 file / 4 tests** for
this checkpoint. Both changed Rust sources parse through `rustfmt --edition 2021 --emit stdout`.
Root owns the native and fresh browser receipts; checkpoint 14 predates this source repair and must
not be reported as its runtime validation.

## Appearance Select commit follow-up

The corrected checkpoint 14b journey opened the real Appearance listbox and physically published
`setAppearance({ value: "dark", windowId: "framework.settings.general" })`, but the Dark option
remained painted, the trigger stayed System, and Language could not open behind the lingering
popup. The option hit is a retained synthesized Stack published as `HitKind::Button`. Shell's
retained pointer ownership gate excluded Button, so release used the direct host action branch. That
branch could publish the setting action but never entered `EventRouter`, the sole owner that closes
the Select overlay.

All retained Buttons now route their complete pointer gesture through `EventRouter`. This is scoped
by the retained hit-owner map before the kind gate runs, so navbar, footer, dock, and other host
chrome Buttons remain on their existing Shell route. Select option activation now produces both one
bounded `UiCommand::App` action and `OverlayClosed`; ordinary retained `UiButton` controls use the
same one-action authority.

The language-neutral fixture adds the Appearance commit vector, including initial/next value,
option id, canonical action, exact queued-action count, and zero open options after commit. A mounted
React oracle activates the real Select and requires one Dark value plus immediate listbox
retirement. Focused Bun+Nx Vitest passed **1 file / 5 tests**. The native Shell law mounts the actual
General producer, opens Appearance through `handle_pointer_button`, paints its popup, activates the
Dark option through the same host ingress, and requires one addressed bounded action, Shell root
appearance mutation, a dark next-paint theme, a selected Dark General projection, zero option hits,
and one action from the ordinary Reset Dock retained Button. The law and production file parse via
`rustfmt`; root owns its native execution and the next browser journey. No native/browser pass is
claimed here.

## Native25 reconciliation

Root's Native25 gate executed 1,196 tests and returned three failures in the new General integration
laws. The failures exposed one production lifecycle defect and two test-authority mistakes; they did
not provide a green native receipt for the packet.

The production defect was in `render_panel_step`'s first-layout preflight. The walk re-evaluated
`retained_content_height(window).is_none()` on every phase-1 opportunity. The retained layout engine
can publish intrinsic height while `UiDocumentFrameCursor` is still in `Layout`, before the cursor
advances to its accepted `Paint` phase. The next opportunity therefore skipped the acceptance and
re-anchor block and committed the original full-band rectangle. The walk now latches whether a
preflight is required when phase 0 selects the active document. Once required, it keeps driving that
same document cursor until layout is accepted or terminal, recomputes the anchor rectangle, and
restarts viewport-dependent work before any glass or tab chrome is committed. New cursors with an
already accepted height still take the direct path.

The footer law previously required a General `PanelTab` while the panel cursor was deliberately
parked before accepted layout. Panel tabs are committed at panel phase 7, so that expectation
contradicted the atomic frame contract. The repaired law first proves that the complete footer
Toggle is interactive while no premature General tab exists, then drains the same panel cursor,
renders a fresh footer, and requires both real hit rectangles to be disjoint before physically
closing Settings. It retains the no-selected-tab and no-drag postconditions.

The appearance law queried authored ids after publication. `panel_ui_records` intentionally qualifies
each retained control key as `<surface>/<authored-id>`, and reconciliation uses that key as the
Select/Button id. The repaired law requires the exact qualified Select and Reset Dock ids and also
requires their retained owner to resolve to the General surface. Synthesized Select option rows keep
their exact item value (`dark`) and are likewise checked against the General owner. This strengthens
the address/ownership coverage while preserving exact option identity.

Both changed Rust sources parse with `rustfmt --edition 2021 --emit stdout`, and the scoped diff has
no whitespace errors. No Cargo/native/browser run was launched here. Root owns the Native26 receipt
for the three focused laws and the integrated gate.

## Retained Select authority and refresh ownership

Native28 reached the focused General laws after the unrelated pool/S3 compile failures were
repaired. Five of six selected laws passed. The remaining Appearance law produced a concrete red:
the Dark option center overlapped the Layout Select, the host hit registry resolved that underlying
Select on both pointer edges, and zero actions were queued where exactly one was required. The same
sealed checkpoint15 runtime showed the corresponding product symptoms. The short journey sometimes
closed the popup but never reflected the selected value; the full journey left the Appearance popup
open and blocked Language. Language's published descriptor named the live Puzzle3d session as its
controller even though the Settings producer authored the `framework` action scope.

Three generic retained boundaries caused that result:

1. `record_action` replaced every binding scope with the live document controller. Published nodes
   normally dispatch through their whole `ActionId`, masking the substitution. Synthesized Select
   option rows have no document record and use the copied descriptor, so their only dispatch path
   used the wrong authority.
2. The retained paint walk emitted popup and ordinary control hits in tree order. The host resolves
   the last matching hit, so a later ordinary row could take a point visibly occupied by an overlay.
   The internal router's recursive overlay flag had the same limitation when the Select was nested
   below a Tree row: it prioritized direct overlay siblings, not the router's topmost overlay
   subtree.
3. Every document reconcile retired all synthesized rows before relinking the published tree. A
   host refresh between an option's pointer down and up therefore invalidated the router's captured
   row and made the release terminal without action or overlay close.

The reconcile now projects a descriptor from the binding's own scope, matching React's
`uiIntentToActionDescriptor(intent).controllerId = intent.action.scope`. The shared neutral
document-tree fixture keeps the live document controller deliberately different from the binding
scope and models a Settings locale option with authored arguments plus the chosen value. Its Rust
law projects the actual Select and synthesized option row; its TypeScript twin checks the same
fixture against React's production mapper. Focused Bun+Nx Vitest passed **1 file / 11 tests**.

Retained hit publication now maintains ordinary targets before overlay targets in the one bounded
registry, without another buffer or per-target metadata. The existing overlay traversal index is
reused as the suffix count during the hit phase. Reverse hit resolution therefore observes overlay
controls first. `EventRouter` also probes the topmost overlay subtree before ordinary content for
uncaptured pointer events. The General law explicitly requires a real underlying Select at the Dark
option center and requires the option itself to be the host authority.

A document reconcile now preserves and relinks only the synthesized rows owned by a still-published
Select whose exact option row has pointer capture. The normal retained sync refreshes those rows'
specs from the new Select record. A removed or retyped owner is not preserved, so its gesture
retires without stale dispatch. The General law republishes the actual General document and runs a
complete panel walk between option press and release; it then requires the same option geometry,
one `framework.setAppearance` action, Shell appearance mutation, next-theme resolution, selected
projection, and popup retirement.

All changed Rust sources parse through `rustfmt --edition 2021 --emit stdout`, and the scoped diff
has no whitespace errors. Root owns the focused native receipt and the fresh browser activation;
the checkpoint15 artifact predates these repairs and remains failure evidence, not validation.

## Host overlay classification correction

Native29 kept five of the six focused General laws green and moved the remaining failure to the
strong host-registry assertion before the option press. The engine-local suffix ordering was
present, but the published Dark option still lost to the overlapping Layout Select. The missing
fact was Select popup representation: portal overlays publish an overlay walk origin, while a
Select intentionally keeps its synthesized option rows as children of the trigger and paints its
own popup geometry. Those rows therefore arrived with `overlay_root = None` and were still inserted
as ordinary content.

`UiTree::is_open_select_popup_row` now recognizes a ledgered synthesized row only when its owner is
a live Select with both the open bit and resolved popup geometry. The retained hit publisher folds
that fact into its overlay classification, so the bounded registry keeps option rows after every
ordinary row and `InputState`'s reverse scan gives them host authority. This does not classify
closed, stale, or arbitrary child rows as overlays. The exact overlap assertion from Native28/29 is
unchanged. The correction parses through `rustfmt` and has a clean scoped diff; its native verdict
belongs to the next root-owned gate.

## Token-backed inline control geometry

The neutral Settings fixture now records the existing shared styling contract rather than WGPU's
private 120px literal: `controlValueColumnUiSpacing = 50` at compact spacing `3.2`, yielding a
160px Tree value column. It also distinguishes authored small Select/Input controls (`5 × 3.2 =
16px`) from default Stepper/Toggle/Button controls (`7 × 3.2 = 22.4px`). Its schema requires all
five control kinds, fill/fit ownership, height mode, and expected dimensions.

The independent React oracle mounts the actual `Tree` with real Select, Input, Stepper, Toggle, and
Button controls. It verifies the live property grid style, each control's
`data-detail-panel-control` ownership, and its actual `h-small`/`h-medium` class. Together with the
neutral arithmetic law, the focused Bun+Nx React run passed **1 file / 7 tests**. Two earlier command
attempts selected configurations that excluded this suite and exited with “No test files found”;
they are command-selection errors, not product verdicts. The first correctly selected run exposed
only two oracle mistakes (floating-point exact comparison and selecting the outer Tree grid); both
were repaired before the green receipt.

WGPU's one `TreeRowMetrics` authority now derives the value-column width directly from the same
styling tokens. Mounted layout carries the actual control kind's height into the flex placement:
Tree Input/Select use `control_height_small`, while other controls keep `control_height`. The static
Tree painter calls the same width/height rect helper, removing its second 120px/generic-height
formula. Retained hit rectangles already come from mounted layout, so paint, layout, and host input
now share the result. The actual Shell General law requires the published Appearance target to be
160×16 before exercising its full Select gesture. All edited Rust files parse via `rustfmt`; the
scoped diff is clean. Native and browser validation remain root-owned and are not claimed here.

## Root UI 39 Validation

Uncached WGPU UI suite: 632 of 632 passed. This covers the 160 × 16 control geometry helpers and corrected real Tree row-band glyph law. General full Shell interaction still awaits a compile-complete reference-producer boundary; Native 31 did not execute tests.
