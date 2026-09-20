# Sol Retained Select Origin

## Scope

This P3 packet gives a retained Select one targetable popup geometry from its solved trigger, then
translates that window-local geometry exactly once when a frame is published at a non-zero host
origin. It must preserve bottom placement, top collision flip, overlay clipping, option hit
ownership, keyboard routing, and accessibility ownership.

Production now keeps one local geometry authority and translates it at frame publication.

## Neutral contract

`retained-select-origin` defines a non-zero viewport origin, trigger-local rectangles, bottom and
top-flipped menu rectangles, and the corresponding global menu and first-option rectangles. The
schema uses exact fixed-length point/size/rect tuples and rejects undeclared fields.

## Independent React oracle

The existing Select component test mounts the real Select trigger and portal inside a scoped
Dialog isolation root. It supplies the fixture's trigger/content DOM rectangles, dispatches the
real pointer open gesture, and observes the mounted listbox. For both fixture cases it checks:

- schema validity through Ajv 2020;
- resolved bottom/top side;
- viewport-local content style coordinates;
- the once-translated global menu rectangle;
- the first option's global rectangle and complete option count.

The focused Bun+Nx Vitest receipt is **1 file passed, 12 tests passed**. The initial red run exposed
a strict-schema tuple omission; after adding exact tuple bounds, the mounted oracle exposed an
invalid Dialog-content nesting assumption. The final oracle uses the actual isolation-root
contract and passed without changing production React code.

## WGPU red law and source diagnosis

The Rust law opens the real retained Select through pointer down/up, runs the bounded
`frame_into_step` pipeline at the fixture's non-zero host origin, and compares popup glass, option
glyph bounds, option hit bounds, and retained Select ownership with the neutral fixture.

Current retained synchronization stores `SelectPopupGeometry` in window-local coordinates from
`UiTree::absolute_rect`. Ordinary node paint and hit publication add the frame origin, but the
Select's direct glass, row paint, and scroll-button rectangles consume the stored popup geometry
without that translation. The generic overlay-origin publisher also treats the Select popup like a
portal-root overlay even though the Select owns its own glass and rows. Those two authorities can
move the trigger and popup independently.

UI25 executed 619 tests: 617 passed and both P3 laws produced the intended red evidence. The
positioned frame reached popup paint but emitted glass x=20 instead of the fixture's global x=980.
The companion normal-layout flow settled its post-open layout and completed `frame_into_step`, but
published no first-option hit. Earlier UI23/UI24 runs had stopped on fixture-pump obligations before
geometry; those were corrected rather than treated as product evidence.

## Product repair

The production repair keeps the event router's popup geometry window-local, excludes the
Select-owned popup from generic portal-root relocation, and derives one translated popup from the
actual painted trigger bounds for direct paint and hit registration. `SelectPopupGeometry::translated`
copies menu and scroll-button rectangles without changing scroll/window state. Retained glass,
rows, scroll glyphs, and direct scroll hits use that same copy. Synthesized option rows continue to
inherit the normal frame walk origin, so they receive the host origin once. No Shell or
Settings-specific offset is involved.

The positioned law now runs the real layout session before opening, drains post-open layout, then
uses the double-buffered accepted-layout test seam to publish the neutral fixture geometry. It never
clears layout flags. Its companion uses only ordinary layout jobs from mount through option
activation on the actual General shape: an Up-flow Tree row with an inline Select. It requires the
real Select action plus `OverlayClosed`.

UI26 made the positioned glass/glyph/hit translation law green. Its initial normal-layout companion
still used a bare Stack, whose single child correctly stretched to 253.6 px and therefore left zero
popup height; UI27's geometry diagnostics identified that as an invalid fixture, not a production
failure. After replacing it with the real Up-flow Tree-control shape, UI28 passed all **619/619** UI
laws with zero skips, including mount → pointer open → post-open layout → non-zero-origin frame →
option activation. Root still owns the fresh wasm/browser receipt; no browser P3 claim is made here.

## Files

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🔽️retained-select-origin/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🔽️retained-select-origin/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🧪️tests/🧩️component/🟦️.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔽️retained-select-origin/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`

## Host option-retirement follow-up

Checkpoint 14b proved that correct popup geometry alone did not retire the popup after a physical
Appearance option click. WGPU published the Dark option as `HitKind::Button`; Shell excluded that
kind from retained router ownership and directly dispatched its action on release. The action was
visible in diagnostics, while `EventRouter` never received the option gesture and therefore never
emitted `OverlayClosed`.

The Shell ownership gate now includes retained Buttons. The hit must first resolve through the
retained owner map, so this does not claim host chrome Buttons. The `settings-general-layout`
fixture now carries the commit contract. Its mounted React oracle closes the real listbox after one
Dark value, and the native Shell law requires one addressed bounded action, applied dark root state
and next-paint theme, zero option hits on the next frame, plus a working ordinary retained Button.
Focused React/Ajv validation is **5/5**. The native law is source-coherent and syntax-checked; root
owns native execution and fresh browser validation.
