# Semio shared design alignment

The surface rules are applied through the shared frame and the entry points that own rendering. Window content has its own level and one paint layer. A docked chip uses glass one level above that content. Adjacent cells and frames share one neutral hairline. In print, the fallback glass and foreground retain readable contrast.

| Area | Changes and evidence |
| --- | --- |
| Shared React and OS | Corrected where content and controls inherit their level. Unified settled and loading dock paint. Anchored the crop guide dimensions inside the authored mask. Stacked World3d attribution and scalar information in one inset rail. The targeted React run passed 646 cases with one conditional skip. A separate 22-file run passed 328 cases including Chromium layout. The World3d modelling suite passed 46 cases. |
| Native UI | Kept WebGPU listbox dividers visible over selection and hover fills through a single row painter. The extracted production painter and an independent Canvas raster agreed. The existing terminal contour, emphasis and pointer-routing fixtures passed 22 cases. |
| Websites | Aligned Hub admin, course previews, staffage, Pets Stories, and demonstrator credits. Verified Play's shared card grid. Pets was checked in Chrome at desktop and phone widths, and focused component suites passed for the edited websites. |
| Animation and clips | Applied the palette and window frame to the standalone Animate HTML export. The native viewer and editor inherit the OS frame and Dock. In the Python renderer, chip edge and fill extents agree, overlapping lines collapse to one, stat cards tile, and the Grasshopper ribbon uses one frame. Verified the production HTML emitter in Chrome and 26 Python tests, including a Manim render. |
| Print | Used the correct glass level and contrast for heading and table chips, including number prefixes. Compiled and inspected the light and dark reports, thesis, flyer, and heading specimen. All five compiled without overfull boxes. |

The maintained surface inventory and test boundaries are in [audit.md](audit.md), [world-overlay.md](world-overlay.md), [web-audit.md](web-audit.md), [animation-and-clips.md](animation-and-clips.md), and [print.md](print.md). Published imagery, archived third-party captures, and artifact test fixtures keep their authored appearance.

## Validation notes

The graph and compiler ingress were tried through the repo command. Installed Chrome, pinned Tectonic, repository Vitest configuration, focused Bun tests, and Manim were used while shared-fleet work was queued. The full WebGPU paint and Animate package tests stalled in compiler scheduling and were cancelled before their assertions ran. Their isolated production painter, raster, HTML serializer, and browser checks passed. The retained probe sources are in this ticket.

The repo MCP build catalogue is available again when started in the workspace. Adding the missing policy entry for the native remote-execution cancellation command restored all 53 build tools.
