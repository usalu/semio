# Exact UI Audit

Dimensions below are source verified CSS pixels, not browser measured.

| Measure | Regular UI | Current Clip |
|---|---:|---:|
| Navbar/footer | 28.8 each | 60/34 |
| Cap/control height | 22.4 | 44 |
| Canvas inset/gap | 3.2 | 40px poster margins |
| Small icon | 16 | custom |
| Tab font | Anta11.2 medium | glyph-height22 |
| Tiny labels | 9.6 | 20 |
| Normal stroke | 1 | navbar2 |
| Silhouette path inset | 1 | 0 |

All relative paths below start at 🧰️framework/🔨️modules/🖱️ui.
Palette: 🎨️styling/🎨️palette/🎨️.css218-241. Stroke/tiny: 🎨️styling/🖌️ui/🎨️.css821-886.

## Canvas
🧱️elements/🎨️Canvas/🟦️.tsx82 uses p-[var(--panel-inset)];99 fills h-full flex1. HorizontalWindows54/VerticalWindows61 use gap-[var(--padding-standard)]. Window source stories NestedLayout/HorizontalLayout/VerticalLayout demonstrate tiling. Five-window3+2 is authored choice; canonical geometry fills canvas with3.2 inset/gaps. At1600x900 inset canvas is x3.2,y32,w1593.6,h836.

## Fullscreen Tab
🧱️elements/🎨️Canvas/⚓️dock/🟦️.tsx57 defines22.4square focus/close hitboxes. Lines114-189 pack label/icon, focus(Maximize2/Minimize2 icon16), close16 and DragHandle in one tab chip. WindowChrome receives topLeft titleChips and topRight capRightChips; no topRight tabs leaves open U baseline after left chip. Generic WindowChrome right enlarge control is another supported pattern but differs from regular ModeDock.

🔨️modules/🎛️chrome-control-presentation/🟦️.ts20-35 gives font11.2 medium,padding3.2,gap3.2,min-height22.4,max-width192px with truncate. chrome-class-names modeDockTabLabelClassName icon-label gap3.2.

## Border/Brand/Typography
WindowChrome sole border stroke1 inset1; borderless cap cells. Glass only chips/body, never gap/whole cap row. Navbar/footer CSS after/before stroke1 at ui.css6559-6568, base level normal flow,120ms hover color.
Navbar exports SemioLogo inlineSVG viewBox350 with dark circular emblem and orange/red/teal paths. Current orange text semio is wrong.
Clip _text uses font_size24 then scales string to glyph-height22. Browser font-size11.2 differs: glyph metrics vary with text. Use actual projected font-size and preserve width/line metrics;192px includes packed controls.

## Browser Reference And Limitation
Current Window story path 🧱️elements/🪟️Window/📖️stories/🧪️.story.tsx title 🖱️ui⚛️react/Window. Existing .storybook/🧪️tests/🧪️browser-runner/🟦️.ts documents repo run workspace:test-storybook builds/serves6010 then existing Playwright Chrome/SwiftShader.
Installed Playwright pinned headless executable missing; systemChrome exists and launches channel chrome. Existing storybook-static/index.json has stale elements/react IDs and lacks iframe.html, so browser attempt404. Cannot use stale static artifact as current visual oracle. No screenshot fabricated.
