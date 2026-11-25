# Semio surface audit

The maintained design uses rectangular content planes, a single owner for each shared hairline, and chips within the plane. Frame chips use glass one step above the painted content, with an opaque fallback and saturation at menu level. The frame owns the perimeter. Each chip contributes only its inner edges. The rest of the chip row belongs to the content. Documents reserve chip clearance. Scenes remain full bleed.

## Sources

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json` owns shared styling tokens for base, window, pane, panel, dialog, and menu.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowFrame` owns the geometry contract and language-neutral vectors exercised by React and Rust. WindowChrome consumes its geometry and CardStack handles device-snapped adjoining frames.
- The November 22 rectangle and chip design ticket documents the common React, WebGPU and terminal implementations. The November 24 UI laws ticket adds the hover, navigation band and line ownership rules.
- The print README explains print adaptation, edge ownership, paper contrast, and heading chips. The reopened print ticket contains the companion verification.

## Maintained surfaces

| Surface | Existing implementation and audit boundary |
| --- | --- |
| Browser shell and plugin hosts | Shared WindowChrome, WindowFrame, SharedBorderRow and the existing React design and plugin harnesses. Checked the level context, dock, loading frames, select and shared border code. |
| Native WebGPU OS and terminal | Rust WindowFrame and Dock, WebGPU shared-edge resolution and native splitters. Terminal frames and inset chips have their own glyph and surface tests. |
| Standalone websites | Play, demonstrator, course, staffage, Hub admin, pets, and presentation viewer. The November 17 border ticket recorded viewport probes at 1440, 768 and 375 for the principal websites. The current web pass covers the remaining local surface exceptions. |
| HTML and WebGPU animation | The standalone Animate HTML compiler owns page chrome. The WebGPU presentation editor and viewer mount shared Canvas2d and Dock. Their crop lines are authored scene geometry. |
| Python animation clips | The shared Manim frame renderer owns border and chip geometry across the teaching clip families. |
| LaTeX print | Shared print packages and templates own page chrome, structured content, and print-safe chips. |

Historical HTML snapshots, artifact test inputs and generated exports reproduce data or earlier outputs. The audit evaluated maintained render entry points and their source. Domain graphics such as crop masks, drawings, staffage and physics obstacles have their own visual meaning.

## Shared UI findings

- WindowChrome previously reported the stack level to its descendants regardless of the level painted at the body or chip. The frame now opens the correct surface scope at each region. The body uses a typed fill and paints once. The settled dock and its loading skeleton use the same window body and pane chip levels.
- The icon shot guide now paints one theme hairline and anchors its dimension chip inside the crop. The chip uses the next glass level. The rectangular and elliptical masks still represent the shot geometry.
- A selected WebGPU listbox row could cover the hairline drawn between itself and the preceding row. Immediate, retained and reference paint now share the same row paint function. It emits the fill first and the one neutral divider last.

## Verification workflow

Repository commands were attempted first. The app's repo MCP instance addressed the filesystem root and returned empty resources there. The local repo command created this ticket in the workspace. Machine capacity and graph worker delays prevented prompt test execution, so focused browser and renderer verification also used the repository's pinned native tools and existing test configurations.

- The targeted React run passed 646 tests with one pre-existing conditional skip. The package export and window regressions also passed after removing the obsolete body-fill class.
- The expanded UI run passed all 328 tests across 22 files by using installed Chrome with the existing component test configuration. The browser binaries for the managed Playwright channel were missing from the tool cache on this machine.
- The separate Chrome probe checked actual bounds, clip radii, hairline width, chip levels and computed glass in both appearances.
- The terminal fixture suites passed 22 cases through `repo test-file`. They cover window and chip contours, shared divider ownership, emphasis, hit regions, and pointer routing.
- The styling geometry suite passed all five cases after correcting an old assertion that still required an inset around the canvas. The test now checks the flush placement exercised by the dock and browser layout suites.
- The WebGPU row painter compiled independently. The raster oracle reproduced one neutral pixel line above selected and hovered fills. An integration assertion also checks that order in the normal Rust renderer. The package test was stopped after compiler workers stayed parked inside the fleet scheduler, before that assertion ran.

## Repository tool repair

The workspace-scoped MCP server rejected its build surface when the new `remote-execution cancel` operation had no policy entry. The policy now classifies bulk fleet cancellation as a native administration operation. A fresh MCP server advertised all 53 build tools and answered the workspace goal resource.
