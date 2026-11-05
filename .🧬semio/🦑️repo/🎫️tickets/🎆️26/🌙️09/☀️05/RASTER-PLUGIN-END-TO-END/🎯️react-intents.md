# React Retained Selection Intents

The React pixel overlay now submits `selectPixels` with a layer, target, source key, intrinsic extent, method, merge mode, and bounded JSON point path. Rectangle and ellipse submit two corners, lasso submits its sampled polygon, wand submits one seed, and all/invert/none submit empty paths. Selection shape generation, flood traversal, merge computation, and outbound RLE serialization were removed from the overlay. The remaining restoration and coverage rendering consume authoritative scene data; crop-to-selection still scans coverage for its small bounds result.

`maskFromSelection` now submits only the layer and expected image key, allowing its retained owner to capture the authoritative coverage. The controlled filter and image grid options added by the parent agent remain intact. Browser source preparation remains for the color picker; wand dispatches its semantic seed without reading or decoding source bytes.

Mounted tests now hold selection receipts to assert pending Tasks guidance, duplicate submission prevention, preserved prior scene coverage, applied/refused/superseded messages, and scene-controlled clearing. Neutral shape fixtures exercise all merge modes and pre-submission pointer cancellation, Escape, tool switch, and utility switch. Keyboard Select All/Deselect tests explicitly distinguish an admitted operation from local gesture cancellation. Focus-to-filter/mask tests inspect the semantic shape intent and consume a separately published mock owner scene, with Sharp supplying rectangle alpha coverage in that test adapter. These mounted tests do not establish native owner execution or cancellation correctness; the native retained owner tests must provide that evidence.

Validation: the repository inspection script's Bun transpiler scan accepted the modified overlay and mounted test source. This is syntax-only evidence. No mounted test, type check, native compilation, or runtime execution was completed in this subtask because the parent reported source admission blocked by the repository ingress pipe EPERM condition. No alternate test ingress was used.

Changed source paths:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🖱️selection-focus/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/✍️editing/🧪️tests/🧫️fixtures/🖱️selection-focus/🔣️.json`
