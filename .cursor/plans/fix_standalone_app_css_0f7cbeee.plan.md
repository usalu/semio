---
name: Fix standalone app CSS
overview: Standalone apps collapse because their Tailwind entry declares no `@source` globs, so the compiled stylesheet ships 96 class rules instead of ~400 and the shell's layout utilities (`flex-1`, `min-h-0`, `overflow-hidden`) never exist. The fix moves framework source declarations into the shared UI and renderer stylesheets so every app inherits them, and extends the existing verify check to all app entries.
todos:
  - id: ui-sources
    content: Declare the UI module's own class sources (react target + 🧱️elements) in the shared ui react globals.css
    status: completed
  - id: renderer-globals
    content: Add the OS renderer react stylesheet layer declaring engine elements and infinite renderer sources, importing the ui one
    status: completed
  - id: app-entries
    content: Point os dev, demonstrator and storybook entries at the renderer stylesheet, keeping only their own @source and plugin renderer globs; drop stale infinite globs
    status: completed
  - id: plugin-renderers
    content: Give the animate present renderer stylesheet its own @source and keep cad renderer globs in the hosting app entries
    status: completed
  - id: verify-guard
    content: Widen checkStorybookFreshness in script.ts to validate @source/@import literals and the shared-stylesheet chain for every app Tailwind entry
    status: completed
  - id: verify-runtime
    content: Verify compiled CSS contains the layout utilities and probe procedural 3d, block 3d, gis 2d and the demonstrator for correct box heights; run verify plus ui/renderer tests
    status: completed
  - id: cleanup
    content: Remove the temporary [DEBUG] contributionsJson log from ShellHost and update the ticket files
    status: completed
isProject: false
---

## Root cause (measured, not assumed)

The standalone dev server and the demonstrator were both running, so I fetched each compiled stylesheet and compared them:

- standalone os dev (port 6086): 174 KB, 96 class selectors
- demonstrator (port 6029): 212 KB, 396 class selectors
- 295 utilities exist in the demonstrator stylesheet and are absent from the standalone one, including `flex-1`, `flex-col`, `absolute`, `fixed`, `border`, `gap-*`, `bg-*`

Without `flex-1`, `min-h-0` and `overflow-hidden`, every nested `h-full` in the shell resolves against a content-sized parent instead of a bounded flex box, so heights compound. The probe artifact [layout-ancestors.json](.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️03/FEATURE-COMPLETE-PROCEDURAL-3D-ENGINE-AND-BREP-KERNEL/layout-ancestors.json) captured exactly that on procedural 3d: the node graph host measures 196374px tall and one ancestor 815355px wide, with computed `overflow: visible` and `flex: 0 1 auto` even though the class attribute says `overflow-hidden flex-1`. That is the vertical growth, the empty space, and the hang (a canvas sized to those boxes).

Why the demonstrator looks fine: [demonstrator globals.css](♻️mit-bestand/🧺️demonstrator/🎨️globals.css) declares `@source` for the UI react target, while [os dev globals.css](🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🎨️globals.css) declares nothing at all:

```
@import "../../../../🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css";
```

The demonstrator is also incomplete (91 of 94 element-only utilities are missing there too) because no app declares the `🧱️elements` directories that now hold `Layout`, `Panel`, `Window` and `ShellHost` after the co-location restructure. Only [.storybook/globals.css](.storybook/globals.css) lists them, and two of its globs are stale: the `♾️infinite` `⚡️implementations/🟦️typescript` paths no longer exist on disk.

## Fix: declare sources once per module, inherit through the import chain

Each stylesheet layer owns the globs for its own module, so an app entry only declares its own directory.

1. [ui react globals.css](🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css) gains the UI module's own sources:

```css
@source ".";
@source "../../../../🧱️elements";
```

2. New sibling stylesheet next to the OS renderer react entry (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🎨️globals.css`) that imports the UI one and adds the shell layer:

```css
@source ".";
@source "../../../../🧱️elements";
@source "../../../../../../♾️infinite/🖼️canvas/🎨️react-renderer";
@source "../../../../../../♾️infinite/🌍️world/🎨️r3f";
```

3. Every shell app entry imports that renderer stylesheet instead of the UI one and keeps only its own `@source "."`:
   - [os dev globals.css](🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🎨️globals.css) (this is the standalone runner that is broken)
   - [demonstrator globals.css](♻️mit-bestand/🧺️demonstrator/🎨️globals.css)
   - [.storybook/globals.css](.storybook/globals.css), dropping the two stale `♾️infinite` globs now covered centrally

4. Plugin-side renderers keep their own class sources. Only two exist: [cad renderer](✏️s/🔌️plugins/📐️cad/🔨️modules/📺️renderer/🟦️component.tsx) and [animate present renderer](✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🟦️component.tsx). The animate present [globals.css](✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/📺️renderer/⚛️react/🎨️globals.css) currently declares nothing and gains `@source "."`; the s plugin renderer globs stay in the app entries that host them (os dev, storybook), since `🧰️framework` must not reference `✏️s`.

5. Pure UI consumers ([compose desktop](compose/client/ui/desktop/globals.css), sketchpad entries, [präsentation](♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🎨️globals.css)) drop their now-redundant UI react `@source` line and keep their own directories.

## Guard so this cannot regress

`checkStorybookFreshness` in [script.ts](📜️script.ts) already validates that `@import`/`@source` literals in `.storybook/globals.css` exist on disk (lines 795-810) - it would have caught the stale `♾️infinite` globs, but it only looks at one file. Widen it in place to every app-level Tailwind entry, and additionally assert that each one reaches the shared UI stylesheet through its import chain, so a future app entry cannot silently ship a 96-rule stylesheet.

## Verification (must all be observed, not assumed)

- Restart `dev procedural 3d`, fetch the compiled stylesheet from the dev server and assert `.flex-1`, `.min-h-0`, `.overflow-hidden` are present and the selector count matches the storybook build.
- Playwright probe in the ticket folder re-running the ancestor dump: assert the node graph host height is within a few pixels of the 900px viewport (not 196374) and `document.body.scrollHeight === clientHeight`, plus a screenshot for the record.
- Repeat the probe for a second and third app family (block 3d, gis 2d) and for the demonstrator, to confirm no regression there.
- Run the verify script so the widened path check passes, plus the ui and renderer vitest suites.
- Remove the temporary `[DEBUG] contributionsJson flowExtension count` log added to [ShellHost](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx) in the last commit once the apps are confirmed working.

## Ticket

Repo MCP is not registered in this session, so ticket files are edited directly, as the other tickets from today already record. Work continues inside the open ticket `26/08/07/GET-ALL-APPS-WORKING-END-TO-END` (the last commit already references it) and all probes and logs land in its folder. The duplicate stub `26/08/07/FIX-APPS-VERTICAL-LAYOUT-GROWTH-AND-HANG` gets closed as superseded by it.