# Lane `products` handoff

Old names are gone from the lane except two intentional negative assertions (see Open 5). Aligned with `compositor-handoff.md` (slots `window-chrome-chip-cap`, `window-chrome-body-surface`, `data-window-frame*`, `data-frame-remeasure`).

## Per project

| Project | Files | Change |
| --- | --- | --- |
| os | `🛍️products/💻️os/.../🧪️tests/🏘️window-ids-and-layout-seed/🟦️.ts`, `.../🪟️window-pane-chrome/🔣️.json`, `.../🧬️shell-components/🔣️.json`, `.../📚️storybook-plugins/🟦️.ts`, `✏️s/🧑‍💻dev/📖️stories/🎭️wgpu/🧪️.story.tsx` | `data-window-frame-content`; gap slot removed from the scaffolding census; component fixture field `window_silhouettes` -> `window_frames` (must equal the Rust field of lane `wgpu-os`); story test id `wgpu-frame-visual-floor`. |
| course | `🎪️pets/stage/🟦️.tsx`, `🪟️chrome/🟦️.tsx` (comment), tests `pet-companions`, `pets-on-stage`, `shared-borders` | `COURSE_PET_SURFACES` is now the card frame (`[data-card] [data-window-frame]`) plus footer: one rectangle top edge, no tab part and no body-edge part. Tests assert one frame surface per card with the chip inside it (`data-window-frame-chip`, dock top). |
| pets | `📡️survey/🟦️.ts` (doc), `surface-survey/🔣️.json` (two case names) | Generic multi-part survey kept; wording no longer speaks of silhouettes. |
| demonstrator | `🎨️globals.css`, `⚛️demonstrator-card.tsx`, `🧪️tests/🎭️acceptance/🟦️.ts` | gap hover selector removed; acceptance asserts frame, frame border, glass top chip, glass body surface, no gap element. |
| play | `⚛️play-card.tsx` | comment only. |
| teaching adaptive-layout | `📐️adaptive-layout/🟦️.ts` | `svg[data-window-frame-border]`. |
| teaching clip presentation geometry | `📐️geometry/🐍️.py`, `🧬️schema.json`, `🧪️tests/🔣️.cases.json`, `🧪️tests/🐍️.py` | Schema first: `outline`/`border_outline` are exactly 4 corners, new `chip_regions` (dock top, inside the frame) and `safe_clearances`; `surface` is the whole frame. Cases: `silhouetteCases` -> `frameCases` (hand-derived chips and rectangles), window case gets `chipRegions`, `safeClearances`, `floorPixel`. New test `test_frame_is_one_rectangle_with_glass_chips_inside_it` (third-party check: Skia path union of frame + chips has the frame area). Browser pixel test: gap of the cap row shows content, not the floor. |
| teaching clip presentation browser | `🟦️.tsx`, `🟦️settled.ts`, `🟨️render.mjs`, tests `settled.mjs`, `settled.cases.json`, `seam.mjs`, `🪡️axis/🟦️.ts`, `🥞️layered/🟨️.mjs`, `🐍️.py`, `🎥️camera/🐍️.py` | `@presentation/frame` alias to `WindowFrame`; `--window-frame-top-clearance`; settled = border painted and content plane `data-frame-state=ready` (no clip-path compare); `data-frame-remeasure`; metadata adds chip regions, chip glass, `gap_hit_in_content`, `gap_element_count`, `content_clip_path`; geometry test asserts them. |
| teaching clip presentation interaction | `🖱️interaction/🐍️.py`, `🧬️schema.json`, `🔣️.json`, tests + cases | Polygon outline hit-testing removed (a frame hit-tests as its rectangle); schema `transparentGap: polygon-outline` -> `chipGap: content`; case `transparent-window-cap-gap` -> `window-cap-gap-hits-content` (path app/canvas/window). |
| teaching visuals (Manim) | `🎨️visuals/🐍️.py`, `🧪️tests/🎨️semio-style/🐍️.py`, `.cases.json` | `window_silhouette_outline` -> `window_frame_outline` (4 corners) + `window_frame_chip_regions`; `window_frame` is one rectangle polygon with a translucent lighter glass chip (and inner edges) added as submobjects; cases `silhouettes` -> `frames`; tests incl. Skia union cross-check. |

## Verified

- `node --experimental-strip-types .../🧪️tests/settled.mjs`: all PASS (new frame cases, roots, camera remeasure, transitions).
- Python: pure functions of `window_frame_outline`/`window_frame_chip_regions` extracted with `ast` and run against `frames` cases: ok. `window_geometry` run against `frameCases` and the window case: ok (outline, border, chips).
- Not run (no manim, pathops, numpy, Chromium, bun here; `repo test-file` runs the plain bun runner with 0 tests for the course suites, which are vitest suites): the Python test classes, the Chromium render tests, the course/demonstrator vitest and playwright tests.

## Open

1. Audit O1 (OS canvas footer `panel tab rows become chips inside the canvas frame's bottom edge`): not done. `useFooterItemsState` feeds `PanelChromeTabBar`s into the separate `Footer` band; moving them into the canvas frame's bottom chip row needs a `Canvas`/`Layout` footer-chip API that the UI lanes have not published (frozen API has none). Needs lane `elements`/`os` decision.
2. Audit C1/C2, staffage title chip CSS, play/demonstrator cards: they are `OverviewCard` consumers; migrate with the elements lane. The wrap-in-chip CSS rules stay valid (chip depth is measured, content clearance follows).
3. `data-shared-border-hover="silhouette"` in `OverviewCard` (elements lane) is now `"frame"` per compositor.
4. The `window_frames` rename in the shell component fixture is mirrored from design; the Rust field is owned by `wgpu-os`.
5. Two deliberate negative assertions keep the removed names in strings: `render.mjs` (`gap_element_count`) and the demonstrator acceptance test (`window-chrome-gap` count 0).
6. Pet stage: after the frame becomes one surface, the pets keep-out of the title chip relies on the generic text-line keepouts; not looked at in a browser.
