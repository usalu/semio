# Prompt history

As the panels are above the windows and they need to have a single unit spacing to them, they need to have double spacing to the sides, navbar and footer. Currently they only have single spacing to everything.

The element border color should always be equal to the hover color. This changes according level. E.g. the element border color for panels are not equal the panel hover color. This should generally not be possible.

The panels toggeling mechanism and components should be extended and refactored. Introduce thre new component called SidePanel, HudPanel which are special kind of panels. The sidepanel is either left or right. It is scrollable and has at the top tabs for different content of the sidepanel. Each SidePanelTab is registerable.
Replace the dropdown toggle for the panel groups in the navbars with a toggle for the sidepanels and center hud panel. All current dropdown toggle options become tabs within the sidepanel. The HudPanel has HudPanelTabs.
Every app can register tabs to different sidepanels and hud panel.

All hooks for app state are massively overfetching. E.g. useDesignApp is fetching the entire design app state. But it only needs the selection, hover, camera, active tool, etc. Make sure to only fetch the necessary state. All app states should come directly from the state machine and use e.g. useSelector from xstate.
Refactor all apps (home, kit, design, type, docs, feedback).

Make sure yMap are used only inside kit store and that the app stores are only using the machine. When done with the migration, make sure all the app tests pass (as they did before). Only finish once all sketchpad tests are passing again.

All app stores are still entangled with yjs. Only the kit store should use yjs. All other stores should use the state machine for state management. E.g. the AppStore should not use yMap such as in the constructor. Make sure yjs (yMap, yArray) dont appear anywhere outside of kit store.

The testing system is currently not clean. Right now there are spread tests for indiviudal features. The testing strategy should not be feature-based but rather component-based. Consolidate all sketchoad tests into one test per app. Make sure that all tests are covering the same functionality. Make sure that all tests are passing.
This means only test("Home", ...), test("Kit", ...), test("Design", ...), test("Type", ...), test("Docs", ...), test("Feedback", ...) should remain. All other tests need to be integrated into the app tests.

The actions of the windows of are below the window ribbon and not in it.

Every window should have a full border arount it. Currently the bottom and right border are missing.

The create actions of all the rows in tables are not right aligned to the coloumn. In between the name and the action should be the strip of tags.

Every app has windows and there is always one active window. Make sure that the background of the table is set to active background color.

Make a refactor plan to turn every app into a multi-window system. Every app can registern window kinds. Generalize layout and remove duplicated code, etc. 
@Design.tsx@elements.tsx@Kit.tsx@Quality.tsx@Docs.tsx@Feedback.tsx@Home.tsx@shared.ts@Sketchpad.tsx@Type.tsx 

The window background color of all windows is still according base. Make sure to useLevel hook correctly and that all windows have the correct window background color.

The diagram component should be generalized to be used for all diagrams (kit app, design app, quality app, etc).
The layout is controlled over a diagram coordinate system (1 unit is equal to the diameter of the a circular nodes.)
Everything is rexported in semio coordinate system (onNodeDrag, onNavigate, etc)
A node can either be circular with an icon or square with a text label.
Handles are dots on the edges of the node controlled by a parameter from 0 to 1. 9 and 1 is 12'clock position and it increases clockwise.
elements.tsx should be the only file to import "@xyflow/react";
Make a refactor plan for Design.tsx, Kit.tsx and Quality.tsx to move to the new diagram component.
@Design.tsx@elements.tsx@Quality.tsx@Kit.tsx 

Analyze the js/js codebase for state managment inconsistencies (hooks, context providers, state machine, commands, etc).
Remember that every component should have a triadic hook: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
Every component should only use the state write state and never use the commands directly. Only the machine is allowed to use the commands.
Use fine grained subscriptions for all kit states.
Refactor all apps to be clean and consistent.
Make sure all sketchpad tests pass.

All ui elements must work for all ui levels (base, window, panel, overlay, temporary). They use a context, provider and useLevel hook for all elements to fetch the level. Base is lighter than window, window is lighter than panel, panel is lighter than temporary. Overlay is transparent and only affects z-index.
Extend all ui elements to work for all levels. Make sure borders, hovers and background dont collide with the background of the level.
Extend all storybook stories for all ui elements to have after the default story a story for each level (Base, Window, Panel, Overlay, Temporary).

- The kit app should have the same toolbar as the home app. The table should not have the band anymore and the filter toggles should move to the toolbar.
- The home app should be expanded to a window-based app like the kit app.
- The toolbar in type app is not floating ontop of the canvas but it is blocking the line between the canvas and the footer.

- All toolbars are in the footer area and not on top of it. The toolbar should be central above the footer with a single spacing unit between.
- Kit app still has no toolbar. The artifact kind toggles should also move to footer.
- Home should also be a window-based. The toggle and search strip should dissappear
- The design app toolbar is still broken and no tool is selectable.
Extend every app test to test tool. Make sure all sketchpad tests comply.

All panels touch the current border and navbar and footer. They should have a single unit margin towards the border and navbar and footer.

Another ui level is added: window
The hierarchy is base, window, panel, overlay, temporary (every one is on top of the previous one and has a darker background color [in light mode] or a lighter background color [in dark mode]. Overlay is an exeption because it it is transparent and only affects z-index). All ui elements need to work in all 5 levels. Work with a level context, provider and useLevel hook for all elements to fetch the level.

Extend ticket api to be able to reopen a ticket. This should remove the total files and lines from the ticket (not from the individual iterations) and set the status to open.

The windows dont have a border on the bottom and on the right. Make the window border dashed.

The tool mechanism should be generally improved and extended. Currently only the type app has working tools. The toolbar should be a floating panel in the canvas area above the footer.
The home app should have the filter toggles for different kit kinds in the toolbar.
The kit app should have the filter toggles for different artifact kinds in the toolbar.
The feedback app should have send in the toolbar.
The design app selection should be identical to the type app selection.
Extend each app test to test each tool.
Use playwright mcp.
Make sure all sketchpad app tests comply. Dont remove functionality from the tests.@Design.tsx@Feedback.tsx@Home.tsx@Kit.tsx@shared.ts@Sketchpad.tsx@Type.tsx @sketchpad.test.ts 

The panel toggles in the navbar still have no vertical borders. This shouldnt be possible. Dont override on the specific group but make sure that all toggle groups always have this.

Every window should have a single unit margin between the window and the border of the canvas or between windows. Every window has a continuous border around it.

A toggle group should like a button group always have vertical element borders between the items. This should be consistent for all groups. E.g. currently the navbar toggles have no vertical elelement borders.

The border mechnanism of all ui elements should be more flexible. Different semantic border kinds should have different kind of styles (stroke, color, pattern, etc).
Currently there is only one border color. All ui elements have border kind called element border (in tailwind we want to use border-element). The ui element border color should be the hover color. The second border kind is for distinguishing windows (border-window). The window border is as current normal border.

The testing system is currently not clean. Right now there are spread tests for indiviudal features. The testing strategy should not be feature-based but rather component-based. For sketchpad there should be only test per app that covers all the features. Consolidate all sketchpad tests. Dont remove any functionality from the tests.

The diagram of the kit app should only show the rows of the table. This means that e.g. if a design in the able is collapsed then all child design node in the diagram are hidden. Same for types. If a folder is collapsed then all the items of the folder are not present in the diagram. In the end every visible row equals one node.

- Not only top level rows should be displayed as node but all of the rows.
- Many nodes are missing (folders, authors,  tags, etc)
- When dragging nothing happens. Not even Machine logs.
- Edges are still wrong and not around the node
Extend kit app test to test all features (all nodes are visible, etc)
Use playwright mcp.

log.ts and all logs should change:
Every ticket should have
{slug, summary, status, author, date{created,finished}, commit, model,iterations{prompt,date,model,commit,files{updated[PATH{lines{added,removed}}],created[PATH],removed[PATH]},lines{added,removed}}}
, files{updated[PATH{lines{added,removed}}],created[PATH],removed[PATH]}, lines{added,removed}
E.g.
---
slug: TICKET-FILES-ONLY
summary: Restrict ticket files and aggregate stats
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T16:09:53.578Z'
  finished: '2025-12-16T16:25:36.733Z'
commit: c44e5e38193be007ca56cc649aa2f58238c1ec40
model: claude-opus-4-5
iterations:
  - prompt: >-
      Only allow files to be created, updated and deleted files. Create ticket
      shouldnt create an iteration. Iteration need files. Add author and date to
      ticket from git. Once finished, combine all the files from all iterations
      and add it as extra field to the ticket. Use git one last time to compute
      the lines.
    date:
      started: '2025-12-16T16:09:53.578Z'
      ended: '2025-12-16T16:25:23.282Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: c44e5e38193be007ca56cc649aa2f58238c1ec40
    files:
      updated:
        - scripts/log.ts
          lines:
            added: 701
            removed: 253
        - README.md:
          lines:
            added: 18
            removed: 9
        - AGENTS.md:
          lines:
            added: 113
            removed: 59
      created: []
      removed: []
    lines:
      added: 888
      removed: 321
files:
  updated:
    - AGENTS.md:
      lines:
        added: 72
        removed: 5
    - README.md:
      lines:
        added: 95
        removed: 10
    - scripts/log.ts:
      lines:
        added: 701
        removed: 253
  created: []
  removed: []
lines:
  added: 888
  removed: 321
---
Author, commit, lines and date should be taken from git and is forbidden to set manually.
Files and model must be set manually.
When the ticket is finished, the files and lines should be computed from git.
Write a migration script all existing logs to new schema. The log script should be clean and only work for the new schema. No legacy api, etc.

A new app should be created: Feedback
The goal of the feedback app is to make contributions (mostly bug reports) as easy as possible.
It is a single page form.
Login is not required. On submission a request is sent a server and a thank you message is shown with options to send another feedback or go back to the home page.
The form has a kind [bug or idea] field and depending on the kind other fields are shown.
The bug report has a title, a description how it happened and a dropdown in which app the bug happened [home, kit, design, type, quality, docs, feedback].
The feature idea has a title and a description what it is about.
All forms have an optional name, an optional field for email and a submit button.

The kit app is not finished.
- The icons should be the same avatars as the ones in the table window. The edges of the node have somehow a bigger circle than the circle of the nodes. 
- Nodes are currently not draggable.
- The states of the table and the diagram should be completly shared. When something is filtered in the table it should also be filtered in the diagram. When something is not expanded and hence no row exists in the table it should also not exist in the diagram. Only rows that are in the table should be nodes in the diagram. When hovering over something in the table it should also show in the diagram. The selection already works.
- Selection on the node fires the events but nothing happens.
Make sure to extend the kit app tests to check for all features.
You can use playwright mcp.

- Only allow files to be created, updated and deleted files.
- Create ticket shouldnt create an iteration. Iteration need files. Add author and date to ticket from git.
- Once finished, combine all the files from all iterations and add it as extra field to the ticket. Use git one last time to compute the lines.

Improve log script semantics.
Rename logs to tickets.
create log becomes ticket create.
Then a new command is ticket iteration start
Then there should be ticket iteration finish
finish becomes ticket finish
Throw an error if an iteration is unifinished for a ticket (e.g. when another iteration start or ticket finish is called)
Force files to be a necessary parameter to call for iteration start and iteration finish. Update the file list on finish for the iteration and compute stats (lines).

The diagram in kit app should be a d3-force layout. The nodes should be a circle with the icons. Add the paramters for the simulation to the settings of the kit app.

The model, commit, author, files in logs should be for every input {prompt,date,model,commit,files}. Make sure model is a required paramter for creating and updating. The rest is only taken from git. lines should be moved to every file. Make sure there is a command to finish an iteration (an iteration is when the agent stops working). Rename input to iterations. When the iteration is finished by the agent then use git to compute the lines for the files that were edited in this iteration.
Migrate all existing logs to new schema.

Make sure that comments in config files and comments between header region are ignored in comment analysis and removal. TODOs should also be ignored. <reference types... in typescript files should also be ignored. In python regions are classified as comments but they shouldnt.
Extend the fix script to automatically add license headers when they are missing. They all follow the same structure. Use Ueli Saluz as default.
Ignore all package READMEs such as net/Semio/README.md

- Table window is empty.
- All nodes should just be circle as all other nodes of the other diagrams with the icon of the artifact.
- The layout is not a draggable forced layout. Add force slider to diagram settings of kit app.
Migrate the existing kit app tests and make sure they pass.

The kit app should be extended to a multi-window app like the design app. It should have two window kinds: table and diagram. The table window is the current canvas. The diagram window should show a forced layout graph of all the artifacts of the kits and their relationships. There are two different kind of relationships: part of (children of parents, artifacts inside folders) and references (such as between a type and a design if there is a piece inside of the design with that type). Hover and selection of artefacts are again shared among the windows.

The analyze script should be extended to create a report for the codebase producing `code.json` (for typescript, python, c#). It should check for:
- Comments in the code. Code needs to undocumented/uncommented.
- Missing License headers.
- Regions that dont close (every `#region REGIONNAME` needs to have a corresponding `#endregion REGIONNAME`).

design app:
- Hover is not showing in piece nodes background and piece geometry material.
- Selection is not showing in piece geometry material.

Updated logs in log.ts should NEVER take the updated files from the git commit and only take lines from it. files can only be added to the list (added, updated or removed) over the cli explicitly. The reason is that multiple agents work in one commit hence the files list would be cluttered. But they usually work on different files hence the lines are ok.

The refactor is far from done.
- There are still 2 createMachine calls (ui should be consolidated into sketchpad)
- You are still not following the open/closed principle. Sketchpad should be independed of the apps and the apps should be self-contained. E.g. all design app events in Sketchpad.tsx dont belong there. Same for type app events, etc. Adding/removing an another app to sketchpad should just be to add or remove a file without having to modify the internals of Sketchpad.tsx

The way that code and documentation are written should be improved.
Every feature, decision should be undocumented/uncommented in the code and documented in the dev docs (AGENTS.md and README.md). The documentation ALWAYS happens four times:
1. Under products in README.md where it is described from user perspective [architects, designers, engineers, …] (framework-agnostic, no implementation references, etc)
2. Under components in README.md where it is described from junior-developer perspective (mechanism explanation and reasoning behind the decision, how theory links to implementation, etc).
3. Under Software Requirements Specification in AGENTS.md where it is described from human-interface-designer perspective (concise technical terms without explanation, framework-agnostic, no implementation references).
4. Under Codebase in AGENTS.md where it is described from senior-developer perspective (framework-mechanisms, consice technical terms without explanation, implementation details, etc).
The AGENTS.md `# Codebase` section has the same header structure as the files and folders. All files and folders are flat with `## PATH` e.g. `## js/js/sketchpad/` or `## net/Semio.cs`
The README.md structure is more human friendly according ecosystem and components.
Migrate all existing docs and code to the new structure. Update outdated docs.
Example
1. User
```markdown
# 🛍️ Products [↑](#-overview)
## ✏️ sketchpad [↑](#%EF%B8%8F-products-)
[sketchpad](#%EF%B8%8F-sketchpad-) is a simple-to-use, accessible and browser-based user interface for semio🖱️
It is the digital pencil for sketching plans and digital scalpel for building models in semio ✍️
![sketchpad demo](/assets/images/sketchpad-demo.gif)
```
2. Junior-Developer
```markdown
# 🛍️ Products [↑](#-overview)
## 🟨 [@semio/js](https://github.com/usalu/semio/tree/main/js/js) [↑](#-components-)
<details>
<summary><strong>📚 Resources:</strong></summary>
- [React](https://www.npmjs.com/package/react) - `npm`
  - [Docs](https://react.dev) - `official`
  - [Issues](https://github.com/facebook/react/issues) - `github`
- [Vite](https://www.npmjs.com/package/vite) - `npm`
  - [Docs](https://vitejs.dev/guide) - `official`
  - [Config](https://vitejs.dev/config) - `config`
  - [Issues](https://github.com/vitejs/vite/issues) - `github`
- [Tailwind CSS](https://tailwindcss.com) - `official`
  - [Docs](https://tailwindcss.com/docs) - `docs`
  - [Issues](https://github.com/tailwindlabs/tailwindcss/issues) - `github`
- [Shadcn](https://ui.shadcn.com) - `official`
  - [Docs](https://ui.shadcn.com/docs) - `docs`
  - [Issues](https://github.com/shadcn-ui/ui/issues) - `github`
- [Radix UI](https://www.radix-ui.com/) - `official`
  - [Docs](https://www.radix-ui.com/primitives/docs/overview/introduction) - `docs`
  - [Issues](https://github.com/radix-ui/primitives/issues) - `github`
- [Lucide](https://www.npmjs.com/package/lucide-react) - `npm`
  - [Docs](https://lucide.dev/docs/lucide-react) - `docs`
  - [Icons](https://lucide.dev/icons/) - `gallery`
- [Storybook](https://www.npmjs.com/package/@storybook/react) - `npm`
  - [Docs](https://storybook.js.org/docs) - `official`
  - [Issues](https://github.com/storybookjs/storybook/issues) - `github`
- [Three.js](https://www.npmjs.com/package/three) - `npm`
  - [Docs](https://threejs.org/docs/) - `official`
  - [Examples](https://threejs.org/examples/) - `gallery`
- [React Three Fiber](https://www.npmjs.com/package/@react-three/fiber) - `npm`
  - [Docs](https://docs.pmnd.rs/react-three-fiber) - `official`
  - [Issues](https://github.com/pmndrs/react-three-fiber/issues) - `github`
- [React Three Drei](https://www.npmjs.com/package/@react-three/drei) - `npm`
  - [Docs](https://github.com/pmndrs/drei) - `github`
  - [Examples](https://drei.pmnd.rs/) - `storybook`
- [React Flow](https://www.npmjs.com/package/@xyflow/react) - `npm`
  - [Docs](https://reactflow.dev/docs) - `official`
  - [Examples](https://reactflow.dev/examples) - `gallery`
- [Yjs](https://www.npmjs.com/package/yjs) - `npm`
  - [Docs](https://docs.yjs.dev) - `official`
  - [API](https://github.com/yjs/yjs) - `github`
  - [Issues](https://github.com/yjs/yjs/issues) - `github`
- [sql.js](https://www.npmjs.com/package/sql.js) - `npm`
  - [Docs](https://sql.js.org) - `official`
  - [API](https://sql.js.org/documentation) - `docs`
  - [Issues](https://github.com/sql-js/sql.js/issues) - `github`
  - [Playground](https://sql.js.org/examples/GUI) - `demo`
- [dnd kit](https://www.npmjs.com/package/@dnd-kit/core) - `npm`
  - [Docs](https://docs.dndkit.com/) - `official`
  - [Examples](https://master--5fc05e08a4a65d0021ae0bf2.chromatic.com/) - `storybook`
- [Cytoscape](https://www.npmjs.com/package/cytoscape) - `npm`
  - [Docs](https://js.cytoscape.org/) - `official`
  - [API](https://js.cytoscape.org/#core) - `reference`
- [Markdoc](https://www.npmjs.com/package/@markdoc/markdoc) - `npm`
  - [Docs](https://markdoc.dev/docs/getting-started) - `official`
  - [Issues](https://github.com/markdoc/markdoc/issues) - `github`
- [Motion](https://www.npmjs.com/package/motion) - `npm`
  - [Docs](https://motion.dev/docs) - `official`
  - [Examples](https://motion.dev/examples) - `gallery`
</details>
<details>
<summary><strong>📼 Videos:</strong></summary>
- [React State Managment](https://www.youtube.com/watch?v=-bEzt5ISACA)
</details>
The core which is shared in the [semio JavaScript ecosystem](#-javascript-) 🥜
```
3. Human-Interface-Design
```markdown
# Software Requirements Specification
## UI/UX
### sketchpad
- canvas-based (navbar, canvas, panels on top of the canvas, footer)
- multi-app (home, kit, design, type, quality, docs)
- multi-window (every app has its own window kinds)
- multi-user (users collaborate inside a studio)
- multi-device (desktop, tablet, mobile)
- multi-language (english, german)
- multi-theme (light, dark)
- multi-expertise (beginner, intermediate, advanced)
- consistent ui (tables, diagrams, scenes)
- local-first (by default all data is stored locally in the browser and only synced to the server when the user wants to share it)
#### Apps
##### Home
- canvas (filter band, concept strip, table)
```
4. Senior-Developer
```markdown
# Codebase
## js
## js/js
## js/js/sketchpad
## js/js/sketchpad/Sketchpad.tsx
### State managment

- ui components access and modify state only via triadic hooks `[STATE,SETSTATE,CANSETSTATE] = useSELECTOR()`
- one global sketchpad `createMachine` is used for app state
- apps register their state machine contributions to the global sketchpad machine
- kits have specialized stores that use Y.Doc and use `observe` in conjunction with `useSyncExternalStore` to sync the kit data.
The kit hooks use the kit store for STATE and the global state machine for SETSTATE and CANSETSTATE.
```

Consolidate all useHOOKXState into useHOOK. There should never be double hooks but just the useHOOK that internally uses xstate to write (setState) and check if the transition can be taken (canSetState).

Rename layout in useLayout to device and useDevice, etc. All types enums etc. Not the Layout component. 

When holding shift and selecting rows then the last selected row shouldnt update. E.g.
A
B
C (last selected)
D
E
Then clicking on A should select A, B, C.
If afterwards clicking on E should select C, D, E and not A, B, C, D, E as currently.

The scroll bar is just a line that is one spacing unit away from the edge of the scrollable element.

Refactor the state machines:
-Currently there are two machines being used (createMachine). There should be only one global sketchpad machine.
- All app specfic logic should be part of the APP.tsx files. There should be no design, type, etc logic part of Sketchpad.tsx file. All should follow open/closed principle. If the file is deleted then sketchpad should work, if a new file is added, the new app should work.
  Make sure all tests pass after the refactor.
- Add comment detector and fixer.

design app:
- The piece nodes dont show hover color when hovering over the piece in diagram.
- The piece geometry material is not showing hover or select color.

i18n script:
- has hardcoded german translations (should only use locales files)
- has mjs and ts file
- is falsely classifying a lot of keys as unused

The development section should be extended by a section port numbers (not semio ports but "regular" port). There should be an overview table of all ports used for dev commands (such as storybook, sketchpad, play) or final packages (such as engine that has a variable port number according release numer r25.02-1->2507). The new port for sketchpad should be 3000 and for play 4000.

Add a new rule that whenever a new file is created, deleted or moved, it should update the file and folder structure in the dev docs (AGENTS.md and README.md)

Concepts shouldnt use toggles but actions (active when part of filter). The concept band should be a concept strip. The concepts next to the name should be wrapped into a strip that when there is not enough space only the strip is scrollable.

Every name row (such as in home and kit app) should have between the name and the + action a strip with concepts. When a concept is added for the first time it should be added to the concept filter. If it is already active then it should be removed if pressed again.

There is a complex additional contraint for designs: A design can only have design pieces of different design families. This means that e.g. in workbench the design tree items must be disabled in design app. Or the drag and drop to reparent in kit app can only be designs where no piece of the design is a design piece of the same design family.
Terminology: A design family is tree of designs. The root of the tree is called primitve design. The branches on the same level are called siblings. There are child designs and parent designs.
Same for types. A type family is tree of types. The root of the tree is called primitive type. The branches on the same level are called siblings. There are child types and parent types.

When I add a child design in workbench in design app I get:

Update log system:
Logs should instead of having prompts and date {created,updated} have input [{prompt,date}]
affected files should be extended to nested: files {read,updated,removed,created}
Affected files in logs shouldnt be derived from git but added manually based on the files that were edited. The update command should have the same api as create and automatically add the new input with the current date and add the new affected files (in case they were not already added). But the lines should be derived from git with the affected files.
Migrate all existing logs to the new format.

The ci/cd system should be improved. The individual commands should work more closely together and be more integrated.
Currently preflights runs all analysis and formatters. There should be two new commands: analyze and fix. Preflight runs both of them. Test should run preflight and then test. build should run test. prepublish and publish should run build. All scripts should have a skip mechnaism to skip preceeding individual steps. Adding a command always means updating all hooks, nx configs, .vscode tasks, launch.json, etc.

The ui system needs to be more tightly integrated with itself and new components are added and existing ones refactored.
In general there should be as little props as possible and the system needs to take the decisions.
Bands should only be horizontal and never vertical. Bands should be optionally scrollable. Navbar should be a non-scrollable band.
A new ui element should be introduced called Strip. A strip is a smaller version of a band. It is also optionally scrollable.
Both band and strip receive an items prop which is an array of compatible items. This is determined by height. Compatible for bands are items with medium height. Compatible for strips are items with small height.
Actions should be extended by a text prop (tiny text height same as tiny icon size). Either action have icon or text or both.
Heights should not be variable but rather defined by the system. E.g.
tiny: icons within actions, tiny text size
small: actions, avatars, small text size
medium: tree items, buttons toggles, inputs, sliders, steppers, footer, table row, strip, …
large: band, navbar, table header
Update elements and all usages in sketchpad. No need to worry about breaking changes the ui elements are only used in this codebase. Just refactor everything cleanly.

The git section of the dev docs is outdated.
The git repo has a compressed main branch. If the release receives updates after main already has progressed, then a parallel release branch is created that works like main but for this release. The first symbol is a summary of the main task of the commit. The last symbol is encoded the amount of work (🪛🔨🛠️🏗️).
The ai part is outdated.
Due to token vs request based we use mainly copilot for most tickets, windsurf for the most token-heavy test-driven-development workflows with mcp (such as playwright), claude code for small bugs, cursor when docs are needed and as main editor with tab autocomplete, codex for simple tasks.
opus 4.5 is the current model.
gpt 5.2 alternative.
In general the dev docs are often written for the js codebase but it is a monorepo. Make sure to complete all repo information and move all js to the ecosystem and the packages.

Yjs should only be used to synchronize the kit data. All app state should be stored in the state machine. E.g. when updating sketchpad settings I get:
[Machine] Y_UPDATE → {"navigation":"design"}

Extend the log.ts functionality. Enhance the script and documentation.
Include in the frontmatter: prompts an array of all the prompts provided by the user. Whenever the user sends a new prompt append it to the array.
Expand log.ts to take model name (of the llm) as forced input. Use the enum values. When a model doesnt exist extend the enum.
Include stats in the frontmatter. When done with a task add stats to the ticket. affected files and then use git to compute stats for the task: total added lines of code and total removed lines of code. Add an additional command to update the stats of the ticket. Use the affected files to recompute with git the changes. This will happen when tasks take multiple prompts.

Remove stats nesting, add nesting to lines {added;removed}, Add nesting to date {created,updated}, rename base to commit.
Migrate all existing logs to new format.

Expand the app test to check each individual panel kind.
E.g. When opening the details from kit app and then changing to settings I get:

kit app details:
⦁ The concept section always appears twice although it shouldnt.
⦁ The name appears twice in design section in details in kit app. Only description appears. The other properties of design are missing.
⦁ When a type is selected then the type section has no items.

When pressing the panel drodown toggles in the navbar most of the time nothing happens, or it only toggles on or it changes the state of other panels. Make sure that every panel group (left or right) work independent and when toggeling on and off the panels appear. Extend all app tests to include checking to toggle every panel kind once on and off. You can use playwright mcp.

The preflight mechanism should be broken down into

Table rows should have the same height as the footer bar (same as height of e.g. action + 1 unit spacing top and bottom)

semio.sketchpad.app.design.properties details section is empty. Extend the design test to check for the name input to be a tree item.

transaction={{
              start: () => transaction?.start(),
              finalize: () => transaction?.finalize(),
              abort: () => transaction?.abort(),
            }}
should be
replaced by transaction context, providers and useTransaction.
Replace all elements to use useTransaction and add a transaction provider for every app.

The codebase should follow the open/closed principle. Everything related to an app should be inside that file. But every app contributes to the state machine.
Plan a refactor that moves all app specific logic into the app files.
Here a few rules:
⦁ Every ui component uses a triadic hook: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
⦁ Ui components never use the store (neither for read, write or canWrite)
⦁ Hooks never use the commands to write and only the state machine.
⦁ Hooks always use the store to read

Every command has an origin paramter but the hooks should never provide it directly. Instead they internally use useOrigin along with context and providers.
Check all hooks. When done you can use tsc to check for api errors.

Make sure that store and commands are never used directly by components.
Components should always use the triadic hooks for read and write.
The hooks should automatically use the state machine for write and can write and the store for read.
Only the sketchpad machine should use the commands.

The design tests should be extended to check that all the flat planes and center are equal to the flat plane of the asset (similar how it is handeled in the unit tests)

Every SETSTATE from the triadic hook used by the ui i uing

A state managment refactor was recently started.
Every CRUD from components should be handeled over triadic hooks: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
App state read/write/canWrite should be exlusively over the state machine.
Kit state read are an exception because the read/subscribe comes from granular, specialized and synchronized yjs store.
E.g. [theme,setTheme,canSetTheme] = useTheme()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
Here what is missing:
⦁ The hooks should never use command (such a use\*Commands) directly and instead every write MUST go over the state machine. The machine is the only client allowed to use the commands.
⦁ Get rid of the Safe versions of hooks. Instead use clean error/loading boundary mechanism together with the state machine.
⦁ Get rid of Triadic versions of hooks. All hooks for ui components should be triadic.
⦁ Get rid of Granular versions. Granular is default behaviour.
Then make sure all the sketchpad.tests.ts pass again.
Do/Extend/Refactor/Change whatever is neccessary to pass.
Files were recently consolidated.
Dev server is running.
You can use playwright mcp.

The finite state machine currently only has one state with many reflexive transitions. Start extraction logic and constraints from the code and migrate it into the machine (e.g. select or opening context menu can only happen before hover, deleting selected can only happen with selection, aborting a transaction can only happen after one was started, etc).
Once done, make sure that you still pass the sketchpad tests.
Here is a draft for a machine with more states:

---

---

useSyncDeep, useSyncField, useSyncNestedArrayItemMembership, useSyncSelectionItemMembership,

All imported geometry (such as imported models) are displayed with their original materials. All meshes should instead have plaster material and anything 2d like lines or points should have plaster-edge material.
@js/js/globals.css
@js/js/sketchpad/elements.tsx
@js/js/sketchpad/Design.tsx

Previously all SETSTATE used to have origin as first argument. The new sketchpad works with OriginProvider/Context and useOrigin(). The implementation of the triadic hook then fetches the origina and adds it to the command as first argument.
Make sure that every component that has an id is also providing it to all the children and children to their children, etc.
@Sketchpad.tsx @Design.tsx @Type.tsx @Docs.tsx@Home.tsx@Kit.tsx@Quality.tsx

A state managment refactor was recently started.
Every CRUD from components should be handeled over triadic hooks: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
App state read/write/canWrite should be exlusively over the state machine.
Kit state read are an exception because the read/subscribe comes from granular, specialized and synchronized yjs store.
E.g. [theme,setTheme,canSetTheme] = useTheme()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
Currently the app stores (and the machine) still use yjs which they shouldnt. Make sure they Design.tsx and Type.tsx gets rid of the yjs import. Only kits are synchronized over yjs. The sketchpad machine has currently a yDoc which it should not have. The hooks implementation currently work in dual mode with actor or store (if actor then it uses send and otherwise store.execute). store.execute should only be used by the state machine and never by hooks. Only use the machine.
The current sketchpad still mirrors the old structure where there is one state and only reflexive transitions for every command. This doesnt use the full potential of state machines. Turn it into a proper state machine (e.g. select or opening context menu can only happen before hover, deleting selected can only happen with selection). The state machine is an additional protection layer that not always all commands can be executed. Make sure that the CANSETSTATE is derived through can of the state machine (reachability comming from different states with a subset of transitions). A draft is added below.
Make sure all apps only use the triadic hook and never filter or call commands directly.
All ui components that use SETSTATE should also read CANSETSTATE and disable the element if not.
Finish when no ui component (such as trees, diagrams, scenes, geometry, etc) is using use\*Commands.
Then make sure all the sketchpad.tests.ts pass again.
Do/Extend/Refactor/Change whatever is neccessary to pass.
Files were recently consolidated.
Dev server is running.
You can use playwright mcp.
@Sketchpad.tsx @Type.tsx @Design.tsx @shared.ts @sketchpad.test.ts

We have a muti-app software with a navigation system, selection, hover, context menu, command system, hotkeys, etc.
Every CRUD from components is handeled over triadic hooks: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
There is a general diff and command system.
Kit state is stored in a special store.
App data is stored directly in the state machine.
To further protect the ui from state bugs, we use states with guards, etc to make sure the software is deterministic.
An idea for a machine was started. Finish it.

Refactor triadic hook: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
to
[STATE,SETSTATE]=useSELECTOR()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
but return null for the set callback if it cant be set.

The way apps consume and set state will fundamentally change.
All hooks will follow this scheme: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()
The hook will always have no parameter and work with scopes.
Refactor all apps to be ready for the new hook architecture.

Make a second refactor plan:
The write mechanism should also change. The ui should only consume exported hooks like this: [STATE,SETSTATE,CANSETSTATE]=useSELECTOR()
All hooks have no parameters and context is passed purely over scopes.
The implementation of the hooks calls the sketchpad state machine.
The state machine calls the commands of the store.
E.g. [flatPiecePlaneXAxisY,setFlatPiecePlaneXAxisY,canSetFlatPiecePlaneXAxisY] = useFlatPiecePlaneXAxisY()

We have a neeply nested yjs store and a highly interactive complex application.
To get granular updates in react we use useSyncExternalStore in conjunction with observe on the yjs primitives.
We plan to wrap the interaction with the store within a state machine.
What do you recommend?

A migration to xstate was recently started.
The hard requirements are:
⦁ sketchpad has no more yjs doc
⦁ Kits are synced over yjs
⦁ Every state read hook must use useSelector from xstate
⦁ All apps mut have a flexible command system (commands are side effect free and only the app is allowed to change state over the machine).
Make sure to use all the benefits of state machines (e.g. the transaction system has transaction.start, transaction.abort, transaction.finalize transitions which should be guarded correctly; a hover can only be cleared if something was previously hovered in design app, etc)
Make sure to pass all the sketchpad tests once done.

⦁ Currently there is one yjs document per sketchpad. All app state should be

⦁ actions
⦁ enqueue
⦁ params
⦁ emit
⦁ spawnChildren

Migrate statemanagment to happen exclusively over XState.

Currently yjs is used everywhere as store framework. Only kits are shared and should use yjs. All apps should use zustand.

Reorder task

The code for panels is partially broken/incomplete.
Extend the sketchpad tests for each app to use every kind of panel.
Fix the code.
Use playwright mcp.

When hovering over a piece in scene in design, I get a massive amount of hover piece events. Just one piece should be hovered and then after the hover is cleared, a new one can be set.

Disable the mechanism of dropping zip files into kits. Just treat it as a regular file.

AUTOMATE Clean
⦁ Find temporary console logs
⦁ Find comments between code

Take a very close look at how to overcome the hover issues. With larger kits it becomes unusable depsite it only being design app state.

semio and threejs have different coordinate systems.
ports in type app are not displayed correctly.
geometry with plane (such as as pieces is not correctly rendered).
pieces should be displayed at the flat planes (the flat

There is a big schema change:
ids and diffs are often just guid strings. From now on they are always <ENTITY>Id {guid}.
Make sure to adjust all the schemas (e.g. diffs), algorithms, commands, ui, etc to use the new api.
E.g. the api of diffs should change from
{designs:{updated:{id,diff}} to {designs:{updated:{design{guid,diff}}
Change all attatched code, assets, scripts and docs.
Finish when all existing tests run.

Write a temporary script to:
⦁ Migrate Cylindric Capital to be a child of Capital
⦁ Migrate Cyclindric Tambour to be a child of Tambour

⦁ Not all Model ENTITY components have the right inputs/outputs (e.g. diff and diffs components have nothing). Some entities are missing entirely (such as folders, concepts, tags, interfaces, etc). Every Entity (exception weak entities such as side) has as first three params: ENTITIY?, Vd?, Gd
Check the semio.ts schema throughly.
⦁ Model ENTITYId components are no longer required because every entity has a guid. Keep The ENITITYId Params with casts, etc.
⦁ Almost all Params are missing

By design the Input json should always be a subset of the output json. Hence loading it as input should always.
All equality functions for kit (and hence recursively all children such as designs, etc) should have a flag strict (default false) which when on should also check for all date equality such as created or updated. By default kits are equal even if they have different timestamps.
Keep on until the tests 100% comply. Dont skip or simplify tests. Everything should deep match.

The store is still massively overfetching and overrendering. Optimize the state managment.
Every single information fetched from the store should be directly subscribed to the yjs map/array. When only depending on some items of a collection, it should not update when another item of the collection changes.
Analyze every custom hook.
Fix/adjust/refactor whatever is necessary in the implementation (dont simplify or change the test) to pass the design app test.
The design app works very smooth for small kits.
@sketchpad.test.ts @Sketchpad.tsx @Type.tsx @shared.ts

The python tests are massively incomplete compared to the semio.tests.ts. They need to check the same functionality. Never shortcut. Use the same test structure. Additionally there are two more engine tests: rest and graphql. Both tests have the same scheme: the first assertion uses the metabolism kit json to create it and then reads it. They must be 100% identical. The second assertion creates the metabolism kit, then sends the kit diff to update it and then reads from the kit. The result must be 100% identical to diffed kit.
Finish when all tests are setup and the implementation complies to it. Refactor whatever is necessary to comply to the tests. No test simplification or shortcuts allowed.
@semio.ts @semio.test.ts @engine.py @engine.test.py @kit_metabolism.json @kit_metabolism_diffed.json @diff_kit_metabolism.json

The validation mechanism must work identical on all implementations (typescript, pyton and c#). The serialization must be identical. For this purpose there should be a new Validation test that is added everywhere. validation.json must be the output from all impementation. From there on different uis exist that use the validation mechanism (such as vscode extension). Make sure that Validation tests comply and refactor/extend whatever is necessary.
@validation.json @kit_invalid.json @semio.ts @semio.test.ts @extension.ts @Semio.cs @Semio.Grasshopper.cs @Tests.cs @engine.py @engine.test.py

The schema from C# and Grasshopper are out of date compared to semio.ts. The test suite from C# matches the typescript one. Make sure that the C# implementation makes the tests pass again.
"Nakagin Capsule Tower"
"Nakagin Capsule Tower", "Slanted"
"Nakagin Capsule Tower", "Twisted"
"Nakagin Capsule Tower", "Dancing"
"Capsule Dream"

The python codebase should be split up into two packages: semio and semio-engine
semio has all the domain logic and engine
Make sure to exclusively use uv and not poetry.
The test should be consolidated into the following test suites:
Diffs

 
Flattening Design
  Nakagin Capsule Tower
  Normal
  Slanted
  Twisted
  Dancing
  Capsule Dream

All the performance issues come guaranteed from overfetching. The app is really smooth for small kits. Think harder. Dont shortcut because you think the logic could be too hard. The test is designed to be easy passable (no complex mesh) etc.

In the ui system of sketchpad every ui element has an id. All dom elements receive this id. Ids are globally unique and must also be unique for the dom. If a component has multiple wrappers then only add the id to the dom element that is interacted with.
Plenty of components are missing ids.
Analyze the codebase, make a plan, fix all implementations and add missing documentation.
E.g. the workbench panel should have "semio.sketchpad.app.design.panel.workbench"
the pieces div should be "semio.sketchpad.app.design.panel.workbench.pieces"
the types div should be
"semio.sketchpad.app.design.panel.workbench.pieces.types"

Currently the Grasshopper components are tied with reflection to the Semio.cs schema. This means that it breaks on schema changes. In Grasshopper the input/output structure should never change. The new Grasshopper Implementation will support opening different versions of Semio.cs by renaming old Components and marking them as obsolete and always update the logic to work with the newest buissness logic.
For this purpose reflection should disappear and input/output should be hardcoded.
All Meta section with reflection should be deleted.
Refactor the complete Semio.Grasshopper.cs Plugin and finish once it compiles again.

The store is still massively overfetching and overrendering. Optimize the state managment.
Every single information fetched from the store should be directly subscribed to the yjs map/array. When only depending on some items of a collection, it should not update when another item of the collection changes.
Analyze every custom hook.
Fix/adjust/refactor the code to pass the type app test.
@sketchpad.test.ts @Sketchpad.tsx @Type.tsx @shared.ts

There is an infinite loop in type app.

The interfaces and tags are missing

Problem is still there. Make sure to not believe but actually check the logs in the design app test and the type app test.
Uncaught Error: Maximum update depth exceeded. This can happen when a component repeatedly calls setState inside componentWillUpdate or componentDidUpdate. React limits the number of nested updates to prevent infinite loops.
Further I get plenty of [TypeMesh] File URL not available errors despite the kit_metabolism.json having all files needed.
@sketchpad.test.ts @Sketchpad.tsx @Design.tsx @Type.tsx @export-metabolism.ts @regen-metabolism.ts @kit_metabolism.json

When opening the design app and the type app after importing in the sketchpad test. It hangs very long, I get infitinite loop warning and the navbar and footer are gone and only the canvas loads. Fix the code for it to not happen. You can use playwright mcp.

The store is still massively overfetching and overrendering. Optimize the state managment.
Every single information fetched from the stroe should be directly subscribed to the yjs map/array. When only depending on some items of a collection, it should not update when another item of the collection changes.
Analyze every custom hook.
Fix/adjust/refactor the code to pass the type app test. Dont change or simplify the test. The model should only be selected once. Currently there is an infinite rerender and the console message keep on appearing even with no ui event.

- Finish everything.
- Importing metabolism kit in initHome is broken now.
- Extend the design app test to open Nakagin Capsule Tower from kit app and pan. The pan shouldnt take longer than 1 second. If it takes longer then you know that the store is not yet fixed. Use logging to analyze where the bottleneck is. FIx everything until panning on the design works. Dont remove any functionality to simplify it. Only stop when the test complies. A hint: probably somewhere the full kit is used where it only needs portions of it (shallow). Make sure that type and design level granular access is possibe (e.g. flattenDesign needs some designs and types of the kit but not all of them. It shouldnt overfetch and only subscribe to the updates in the yjs store needed.)

The state managment of sketchpad needs to be completly refactored.
Add systematic logging to understand where data is overfetched. Currently often hooks are nested or use only selectors instead of subscribing and hence syncing with the yjs map/array.
Use playwright MCP to get access to the ui. Work with the imported metabolism kit. You will see huge performance issues when navigating or using the ui. Simple ui actions or navigation can take up to seconds.

Rules:
⦁ Components should never use general hooks (such as useKit) and then filter locally but instead only use targeted hooks that only update on changes. The hooks are in the sketchpad store region.
⦁ Every change in state works over commands. Commands have no side effects and only the store is applying the diffs.

⦁ Schema change: Add mime to files.
⦁ Write a migration script that migrates the kit_metablism.json to be semio.ts conformant. E.g. currently files have path name but they should have name + folder + mime

Extend/fix the tests:
⦁ You removed plenty of functionality which you should not do. E.g. drag and drop of pieces into diagram and scene. 5 times for diagram and 5 times for scene in the middle and near each corner of the winow. See old code.
⦁ Kit app should check for concept, interface and tag rows (see fixture).
Extend/fix the code:
⦁ Test is failing because Tambour still shows messages that the type has no model.
⦁ Details in type app are not showing two sections type and kit with items (name, description, etc). They are collapsible individually.
⦁ Kit app shows no concept, interface and tag rows.

Extend/fix the tests and the code.
type app:
⦁ Check that the type is corrently showing the model and not showing an warning/error that the type has no model.
⦁ Check that the details panel is showing two sections: type, kit with all tree items (name, description, etc)
⦁ Check that the settings panel is showing three sections: type editor, kit editor, sketchpad
design app:
⦁ Check that the piece is corrently showing the model of the type and not showing an warning/error that the type has no model.
⦁ Check that the details panel is showing two sections: design, kit with all tree items (name, description, etc)
⦁ Check that the settings panel is showing three sections: design editor, kit editor, sketchpad
Finish when all tests comply. You can use playwright MCP.

Change home, kit, design and type tests. They should not work on new kit/design/type but instead specifically on metabolism that should be import in initHome.

Add explicit mime field to files

Finish:
Write a migration script for kit_metabolism.json.old to kit_metabolism.json that extract the models for each type (formerly called representations - along with tags, etc). Analyze the new schema in semio.ts. After executing the migration script as long as it is wrong, use git checkout on the kit_metabolism.json file to restore it. Finish once all mising information from the old kit is migrated.

There are schema changes:
⦁ Tags should become kit entites (with guid, name, description, attributes, etc)
⦁ Concepts should become kit entities (with guid, name, description, attributes, etc)
⦁ Models should link to files with guid same as all other ids.
Adjust all attatched files.

The scenes in design app and type app still use geometry placeholders (boxes) instead of loading models.
Every type has multiple models. Each model with the highest jaccard index is displayed in the scene.

The scenes in design app and type app still use geometry placeholders (boxes) instead of loading models.
Every type has multiple models. Each model is a file with metadata (such as tags for filtering). Add a validation rule that gives a warning if the file extension is not a common 3d file (take the list from supported three.js importers). Types and pieces then use a model to display geometry in the scene. In the footer of design app and type app should be all names of tags. Then tags can be selected. Each model with the highest jaccard index is displayed in the scene.

Consolidate all tests. The checked features should be the same but the tests shouldnt be split. There should only remain one test per component.

sketchpad.tests.ts should in the end just have one test per app (currently only Home, Kit, Design, Type, Docs). Make sure that all child apps use inititalition of parent. E.g. Home should import kit before each child test (Kit, Design, Type).
Adjust all tests to check for the same functionality but with the new strucutre. Finish when all tests are complying.

There is an app hierarchy such as sketchpad -> home -> kit -> design | type and each app has certain settings, details, etc.
The panel system works like this that panels from the same kind have different section from most specific (top) to least specific (bottom).
E.g. for settings: Home - Sketchpad section; Kit - Kit section, Sketchpad section; Design - Design section, Kit section, Sketchpad section; etc
Update docs, code, extend tests for checking all panels that checks for all apps if the settings are available and if the order is correct. Make the code comply to the test. Use playwright mcp. You will need to iterate because currently it doesnt always work. Adjust/refactor/extend all code neccessary.
Same for details. The details of kit also show in design or type app.

After importing metabolism kit in sketchad (see test) the app is completly unresponsive and only expanding a type row takes multiple seconds. Analyze and fix.

The state managment needs to be refined. Currently hooks often use parent hooks that use selectors. To avoid this make sure that every selector has the proper subscribe in the store only the yjs datastructure that it needs. Come up with a clean solution. Implement it everywhere. Make sure all existing tests are running.

Before reading from yjs data strucutres (such as maps or arrays) they need to be first assigned to a y.doc.
When importing a zipped kit in home app I get thousends of
Sketchpad.tsx:1148 Invalid access: Add Yjs type to a document before reading data.
Fix the code and make sure that the warning in the console disappears.

⦁ The files and folders of the kit are missing (not visible in rows) after dropping.
⦁ Adding files is extremly slow. Investigate why and fix it.

There is an app hierarchy such as sketchpad -> home -> kit -> design | type and each app has certain settings, details, etc.
The panel system works like this that panels from the same kind have different section from most specific (top) to least specific (bottom).
E.g. for settings: Home - Sketchpad section; Kit - Kit section, Sketchpad section; Design - Design section, Kit section, Sketchpad section; etc
Update docs, code, create a test for settings panel that checks for all apps if the settings are available and if the order is correct. Make the code comply to the test. Use playwright mcp. You will need to iterate because currently it doesnt always work. Adjust/refactor/extend all code neccessary.

All UI elements in sketchpad should receive a custom right click context menu.
Use as a base and then integrate into elements: npx shadcn@latest add context-menu

Sketchpad should be expanded by error and loading mechanisms.
E.g. When a kit is dropped in home screen, then a new kit should be created and the row is disabled with a loading spinner on it until the import is finished.
Same mechanism if a new file is dopped in kit app. A new file row should be created and then disabled with a loading spinner until the file is imported.
Or whenever a kit/design/type/quality/etc or docs page is not found display display a message and offer link to nearest parent.

Refactor all the state managment and command execution of the apps (home app, kit app, design app, type app).

The python codebase is out of date and incomplete compared to js. Migrate all unit tests from semio.ts. Use pytransforms3d for spatial maths and networkx for graphs. Setup the tests and extend/change/refactor the codebase until it complies to it. Then also add CRUD tests for the rest and graphql endpoint. Everything inside test_engine.py

The C# Codebase is out of date compared to js. Get the unit tests from semio.ts working. Use the same fixtures from semio assets.

The drag and drop test

Extract and create a tree of ids used for ui components in sketchpad.
Create a section in README.md and AGENTS.md

Make sure to expand the design app test by:
⦁ Dropping 4 pieces near corner in scene currently have all wrong planes. A piece that is dropped on a scene receives the plane that intersects with the grid (easy first check: plane must have z=[0,0,0]; second check: the piece is immediately hovered over if the plane is correct because the geometry is right under the cursor). Make sure that the tests reflect this. Fix/extend/refactor/change the code until it passes.

Then refactor

Refactor the toolbar mechanism. Currently the toolbar is not visible in design app and type app ().
Tools is another toggle right next to the panels toggle group that affects all tools being visible. Tools should be on by default. The purpose to toggle tools off is to get a distraction free view (such as for a presentation) of the

Still failing:
⦁ Dropping 4 pieces near corner in scene have all wrong planes. No hover happens.
⦁ Dropping piece after panning (holding left mouse and moving) and zooming (mousewheel) diagram leads to wrong centers.
Dont forget that every piece must be immediately hovered afterwards. This only happens if the center or plane are correct.

Create a test for drag and drop that drops the kit assets/semio/metabolism.zip into canvas. After this check that every type and design are present and imported. Check for the tambour ports that they are all present and have correct values. Check for nakagin capsule tower design that all pieces are present. Make sure there is no .semio folder imported. Check that all folders/subfolders/files etc are present.

Extend/refactor and adjust until test is implemented and code fixed:
⦁ Extend the test to not only drop into the middle but also near all four corners. Every time the hover needs to happen to check if the plane or center is correct. Repeat the process after panning and zooming in the diagram and scene. This time only drop somewhere in the middle.

First Integrate the hover test into the drag and drop test.
Then:
The drag and drop test partially works. Extend it and fix implementation.
⦁ The center of the dropped piece in the diagram is correct but as soon as the diagram is zoomed or dragged the piece is no longer on the correct center.
⦁ The plane of the dropped piece should be the intersection of the grid and the cursor
You can use a trick: If the location is correct the cursor immeadiatly hovers over the piece because it is under it. If the the location is wrong it doesnt.

The drag and drop test partially works. Extend it and fix implementation.
⦁ The center of the dropped piece in the diagram is correct but as soon as the diagram is zoomed or dragged the piece is no longer on the correct center.
⦁ The plane of the dropped piece should be the intersection of the grid and the cursor
You can use a trick: If the location is correct the cursor immeadiatly hovers over the piece because it is under it. If the the location is wrong it doesnt.
Use playwright mcp.

Drag and dropping a piece from the workbench into diagram works. Then immediately after there should be a hover on the piece node. But somehow hovering and selecting pieces in the diagram doesnt work. only setDiagramCenter is called. Create a test and use it to fix the implementation. Adjust/refactor everything necesarry until the test is complete and the implementation complies to tthe test.
Use playwright mcp.

Panels indivudual

The home app should support drag and drop of zip files and create and import the kit.
Both home and kit app shouldnt import the .semio folder and only import from it. If the kit cant be loaded then everything should fail.

Add a script export-metabolism.ts and make it callable in vscode und run/debug. It should take the kit_metabolism.json semio asset and a subset of files from examples/metabolism (all files from the representations and all files from the icons folder) and export it to metabolism.zip in the semio asset folder. semio.tests.ts already uses this feature.

Add a test to docs and fix: The dropdowns > in the navbar in docs pages dont show any options. Show all child pages of the parent.

Create a test for the docs app. It should check that the content is loaded, images are visible, the workbench panel shows all pages, the details panel shows page section with all headings. Make sure to implement the test and adjust the code, until everythings runs.

A new command should be added to the monorepo: preflight
preflights run all formatters, linters, i18n, ... etc.
preflight should be called by husky for precommits.
Make sure to integrate it into to codebase (creatings scripts, documentation, configs, vs code tasks/launch).
Commands always work hierarchically. A command has a script and calls the same command for all children.

The cursors in sketchpad are not consistent and not documented.
⦁ Clickable pointer should only be used
The label of ui elements are showing clickable pointer.

Get the drag and drop from workbench to diagram windows working. There is a test that works and should not be modified. Finish until

Tolerance: 0.001

The CI/CD should be improved.
This includes a new folder with hooks/ that has scripts (.ts) that run before any commit.
Setup pre-commit for the monorepo. Use existing pre commit configuration if they exist. For the rest create custom hooks.
Add all formatters (prettier, ruff, …).
Currently some reports are saved under agents/\* folder. Rename the it to reports/.
Add all linters to produce reports.
Migrate i18n script to be a hook.
Analyze for existing linters, formatters, etc
Implement until everything runs.
Document everything.

Extend frontmatter to include tokens used. When updating
tokens:
  - cacheRead
  - cacheWrite
  - input
  - output

Our app has a composable approach:
There are temporary, local or cloud kits.
Further our ui allows to modify kits.
We have modify tests for all features.
We would like to run the tests for each kind of kit.
How can be run this parametrically in playwright without breaking the test ui etc?

Sketchpad full screen mechanism is still a prototype. Finally it should be:
⦁ F11 for Window Full screen. Ctrl + F11 for sketchpad full screen (footer and navbar position fade out to top and bottom and only fade in again when the mouse is near them). Ctrl + Shift + F11 for toggeling both fullscreens together

The test system still needs to be setup, some things are missing implementation and documention.
Analyze the new folder and file strucutre. It shouldnt change. The code needs to be adjusted. Look at the existing tests and explain the test design behind them.

unit test are directly next to the module with .test.ts extension.

sketchpad:
e2e:
Rules:
⦁ There is a neested seeding according app hierarchy. Seed include only the bear minimum to get the subtests working.
sketchpad -> kit -> design | type | quality)
sketchpad -> docs
⦁ Only use id locators e.g. `page.locator('\\\[id="semio.sketchpad.navbar.back"]')`
⦁ Never use browser API directly because sketchpad also runs in different context such as desktop through electron. Use only sketchpad ui elements.

vscode:
- add test for invalid kit. Complete the invalid kit for all other validation rules. The invalid kit should be max invalid.
- Remove VALIDATION.md and integrate into README.md and AGENTS.md
  Generalize

The current sketchpad ui system is not sufficiently consistent and documented.
Rules:
⦁ Every ui component has an id. Only the final dom element receives the id. The id is used for i18n, hotkey, command logs, recording, testing, …

The log system should be expanded. Every task is associated with a log.
Create a log.ts script for CRUD of logs. logs should be reorganized to be nested inside folders: YEAR/MONTH/DAY/SLUG.md
Implement, document and migrate everything.
All markdown logs should have a yaml frontmatter:
date: TIMESTAMP
slug: SLUG
author: NAMEANDEMAILFROMGITCONFIG
summary: SUMMARYFORCOMMITHEADER
model: CURRENTLLMMODELIDENTIFIER

---

Currently powershell is the main scripting language for ci/cd. Change this to be typescript. Migrate the whole codebase.

CI/CD: There should be only this five commands: dev, build, prepublish, publish, test
Depending on what level they are executed they always start their child packages to do the same.
dev is the only watching command which doesnt return.
All other commands must always return (e.g. no watching tests that need to be terminated manually) because they are used in ci/cd or agents etc.
Make sure that all projects in the monorepo follow this and document it.

Make sure the designs have the following parent child relationship in the end:

Nakagin Capsule Tower -> Flat
Nakagin Capsule Tower -> Slanted | Twisted | Dancing -> Flat
Capsule Dream -> Flat

@.claude/agents/playwright-test-generator.md Generate a test for drag and drop of pieces from workbench panel to diagram in design app. Seed the apps correctly. Use create temporary kit for kit app. Use create design for design app. Then start the test by toggleing the workbench panel and

- Refactor all SQL code to be centralized.

- ALWAYS run specific tests and NEVER use default interactive test mode that creates a never ending process.

A strip is currently a

should be expanded to strip group. It should work the same as toggle and toggle group.
There should be strip items. Strip items are scrollable areas.
The strip component is just a strip group with one item.
Every strip item is separated with a vertcial border. All

When a zip file is dropped onto kit app, check if the folder has a .semio file which means that it is a kit. Then import the kit. If it doesnt have the .semio folder then just import the files.

Fix i18n script:

- Some ids are not detected such as:
  semio.sketchpad.navbar.fullscreen

- Not all actions have description tooltips (such as dropdon toggles e.g. sort toggles of table headers) which should be detected.
- All tooltips and manuals are missing.
  Then I18N

PLAN and IMPLEMENT
A test for diffs.
First create a script that based on a seed takes the metabolism kit and generates a kit diff where it uses all the features from kit diff. Then saves the kit diff as diff_kit_metabolism.json, the inverted diff as diff_kit_metabolism_inverted.json and the modified kit as kit_metabolism_diffed.json.
The test should take metabolism, metabolism diff, inverted metabolism diff and diffed metabolism. It should compute the diff from metabolism to diffed metabolism and the inverted diff from diffed metabolism to metabolism. Check that they are equal. Then apply diff on metabolism and check that diffed metabolism is the outcome. Same for inverse.

Write a script that uses

PLAN and IMPLEMENT
Import and Export of kits. Move the import/export code from kit command to semio.ts.
Import should receive an url and fetch it from there (extract the .zip, etc)
Export should receive a kit and files and return an in-memory zipped file.
Create a test that exports the metabolism kit from the json and pure zip files (zip the folder examples/metabolism without the .semio folder), then exports it (as zip) and then import it again. Check that the original kit and files match to the one after the roundtrip.
Finish when the test succeeds. Make sure to check the new schema in semio.ts.

Currently there are no Implement a test that tests

PLAN and IMPLEMENT
For displaying purpose every piece needs the calculcated center in diagram and plane in scene. Both come from flatten design (depends deep on kit). Implement all the hooks in the necessary stores and use them in the diagram and scene components.

In my vs code test tab 3 tests are failing but when I run vitest from the command line all are passing. How is that possible?

PLAN and IMPLEMENT
The panel system usually works like this:
The sections are ordered by specificity (top: most specific, bottom: least specific)
Every child also renders the sections of the parent
E.g. settings panel: sketchpad settings are most general, then kit app settings, then design app settings
E.g. details panel: kit details, design details, selection details

- All manual and tutorials are missing now
- Further this misses:
  semio.sketchpad.app.design.windows
  semio.sketchpad.app.home.createTemporary

When hovering over the options from the dropdown panel toggle in navbar the description tooltip of the toggle shows and not of the options. The options description tooltip should be left.

PLAN and IMPLEMENT
There are several schema changes:

- Piece, Port and Model receive a name.
- Interface becomes a separate kit artifact (with guid, name, description, icon, compatibleInteraces [InterfaceId with guid]).

- Refactor all commands to not have side effects. (e.g. setLanguage)

- The guid of the types match but the guid of the designs are not aligned.
- The port guid for connections are missing. The port guid must match and exist on the type of the piece.

Fix the i18n script because it is missing e.g.
semio.sketchpad.navbar.breadcrumb.temporary.hotkey
then I18N

HOME > TEMPORARYKITKIND > KITNAME > KITVERSION > DESIGNKIND > DESIGNNAME > CHILDDESIGNNAME > CHILDCHILDDESIGNNAME >

The > of DESIGNNAME > CHILDDESIGNNAME is not showing anything and the > in CHILDCHILDDESIGNNAME > is showing the options of DESIGNNAME > CHILDDESIGNNAME instead of showing all children and Create child of CHILDCHILDDESIGNNAME

-Finish migration of assets. No need to be general. It only needs to work for the semio/assets folder two commits ago.

In kit app:
HOME > TEMPORARY > KITNAME > KITVERSION >

The > of TEMPORARY > KITNAME is not showing the other KITNAMES and Create Kit

- guids need to be consistent accross all assets. e.g. Tambour type needs to have the same guid in the json file, in the metabolism kit and in the piece type guid.
- view does not exist on designs
- normalize all json (use sorted keys recursively)

HOME > TEMPORARYKITKIND > KITNAME > KITVERSION > DESIGNKIND >

KITVERSION > DESIGNKIND currently shows Create Version but should show all KINDS

Make sure to implement flat design correctly for the
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN >
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN >
DESIGN > should show all DESIGNNAMES | Create Design

Analyze if the options are meant for "children" or "siblings". The options api on breadcrumb should be for child options.

HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME > CHILDCHILDDESIGNNAME >

The options in HOME > should be KITKIND (TEMPORARY | LOCAL | CLOUD)
The options in TEMPORARY > should be KITNAMES | Create Kit
etc

Currently:
HOME > TEMPORARY isnt showing any options
TEMPORARY > KITNAME is showing options for HOME > TEMPORARY
KITNAME > KITVERSION isnt showing any options
KITVERSION > DESIGN is showing options for TEMPORARY > KITNAME
DESIGN > DESIGNNAME isnt showing any options

Should be:
HOME > TEMPORARY to show alternatives to TEMPORARY (other kit kinds)
TEMPORARY > KITNAME to show alternative KITNAMES not KITVERSIONS and an additional Create Kit
KITNAME > KITVERSION to show all KITVERSIONS and an additional Create Version
KITVERSION > DESIGN to show all alternatives to DESIGN (other artifact kinds)
DESIGN > DESIGNNAME to show all siblings of DESIGNNAME and an additional Create Design
DESIGNNAME > CHILDDESIGNNAME to show all children of DESIGNNAME and an additional Create Child
CHILDDESIGNNAME > CHILDCHILDDESIGNNAME to show all children of CHILDDESIGNNAME and an additional Create Child
CHILDCHILDDESIGNNAME > to show all children of CHILDCHILDDESIGNNAME and an additional Create Child

Some options are shifted
e.g.
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME
is shiftet starting with TEMPORARY > KITNAME
All options from there on should be moved up one item.

- The old system used Type -> Variant or Design -> Variant -> View. The new system just uses parents. The variant or the view name is just the name of the child type or design. When a type or a design has no default view or variant but has children in the old schema then create a new abstract type or design in the new schema.
  The new capsule hierarchy is like this: Capsule [abstract] -> (Box [abstract] | Ellipsoid [abstract] | Trapezoid [abstract] | Balcony [abstract]) -> ( / | \ | p | q | s | z | L | J )
  The new tambour hierarchy is like this: Tambour [default] -> First Storey | Last Storey | Single Storey
- Restore only the semio assets from 3 commits ago. Run the migration until you succesfully migrated. Restore the semio assets as many times as necessary.

The breadcrumb api should be refactored. Only two components should remain: Breadcrumb and BreadcrumbItem
Breadcrumb has a prop called items.
Every item has an optional prop options.
All items have a > between. When options are provided then the on click it expands to v showing the options.
Then refactor the navbar with the new api

Breadcrumb should receive a prop called items. Every breadcrumb item should have a prop called options. When options are provided then > appears after the item.

Something like the current wrong navbar in kit app:
HOME > TEMPORARY KITNAME > KITVERSION > >
which should be
HOME > TEMPORARY > KITNAME > KITVERSION >
shouldnt be possible.

PLAN
There is a large schema refactor: Currently guids are used to reference entities. Now every entity should receive a class called ENTITYId (e.g. KitId, DesignId, ...) which only has the guid prop. The reason behin this is that graphql can later use the same json for more complex queries.
Make sure to adjust all code, introduce new types in typescript, adjust all store and command api, etc.
Dont worry about breaking compatibility.
E.g. a kit looks now like this:
{ "types": [ { "guid": "GUID1" } ], "designs: [ { "pieces": [ { "type": "GUID1" } } ] } ] }
and after the refactor like this:
{ "types": [ { "guid": ... } ], "designs: [ { "pieces": [ { "type": { "guid": "GUID1" } } ] } ] }

The i18n.ps1 script doesnt catch all errors. E.g. semio.sketchpad.navbar.breadcrumb.temporary is still displayed as key. Fix script. Then run it and solve all i18n. E.g. there are a lot of leftovers of unused keys that shouldnt existst.

AUTOMATE
UI elements have ids. Those ids are used to render label, tooltips, assign hotkeys, link to tutorials and manuals, etc. All locales must be complete. Write a script to automatically to create a summary of incomplete or wrong id/i18n setups. Understand how the existing i18n system is setup.
Assumptions: ui element ids always start with "semio.sketchpad."

kit app:
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME >
is
HOME > TEMPORARY KITNAME > KITVERSION > > | | >
design app:
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME >
is
HOME > TEMPORARY KITNAME > KITVERSION > > DESIGN > DESIGNNAME > CHILDDESIGNNAME >

Starting from > > The breadcrumbs are shifted. E.g. CHILDDESIGNNAME > is showing to create new design instead of new child.

Refactor the drag and drop system for pieces in design app. Originally there was only one fixed positioned digram. Now there is a flexible window mechanism.

- Add a draggable avatar to every tree item for types and designs
- Make sure that the type and design avatar are drag and droppable into any diagram. Create a piece with the correct center.

The navbar is not working properly e.g. in design app for
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME >
is
HOME > TEMPORARY KITNAME > > DESIGN > DESIGNNAME > | | >

New cursors have been added: nesw-resize and not-allowed
Integrate them properly

Refactor the navbar. All items of the navbar should have a single gap between them.
Navigation buttons and navigation should be left aligned. Panel toggles, focus toggle and search toggle right aligned. The navigation is taking the remaining space until search toggle.

Create a new aggregation ui element called Strip. A strip has a direction (horizontal [default] | vertical). The height (or width on vertical) is large and it is wrapped into a scrollable. Strip has a prop called items. Items can be anything that is medium heigh (or wide) e.g. toggle, button, input, stepper, etc. All items have a single gap between them. All items together are wrapped into a single padding.
Implement the strip for in home app and kit app.

Refactor the label mechanism. Instead of passing the label={useLabel...} just make sure that every ui element has an id and then when showLabel is passed then fetch useLabel internally.

Refactor the breadcrumb mechanism. Every Item should have a prop "items" which when provided shows a chevron at the end and when clicked on the chevron the list appears. Separators shouldn never have to be added manually. Then adjust all navbars to use the new breadcrumb mechanism.

Single: 1. Spacing between elements is always 1 unit. Spacing between icon and the element is always one unit. Etc
Tiny: 3. (e.g. height and width of icons within actions, small text size)
Small: 5. (e.g. height and width of actions, height and width of avatars, default text size)
Medium: 7. (e.g. height of tree items, height and width of buttons and non-actionable toggles, height of input)
Large: 9. (e.g. height of navbar, height of table row, height of table header).
Huge: 11. (e.g. height of navigation buttons at the bottom of a docs page)
Mega: 13 spacing (e.g. width of toggle with actions)
Giga: 15 spacing

Refactor the ui elements. Create a component called Element. Every Element has an id and a level.
There should be a general context and hook system which can set level for all children. Every ui element can override the level.

- Consolidate all ui element groups (e.g. button group and toggle group) into a single one and make sure that the specialized groups are just calling the group component. All ui elements with medium height (button, combobox, input, select, slider, stepper, toggle) are groupable. A group

- Dont leave the old startTransaction, etc singular props. The transaction prop can be set together or not at all.
- The toggle with action width is wrong. They are extremly squeezed. It should be SPACING | ICON [small] | SPACING | ACTION [small] | SPACING

- The toggle with action is broken. It shows no more icon and the action still has no unit spacing towards the right.
- All ui elements should take a prop called transaction with {start, finalize, abort} callbacks and implement it. E.g. pressing escape while interaction should always abort.
- The slider width should take all the remaining width.

- All default input stories should have showLabel
- The button cycle is not showing any icons. Cant say if it works.
- the toggle with action active state should show inside the complete rectangle (where the border is around). The action should be on top and have the level background (hence cover the active color on that spot).
- The toggle with action action has no unit spacing to the right border.
- E.g. dropdown toggle tooltip has plenty of space between the icon and the tick. It should just have a unit spacing between.
- The toggle group story should have a normal toggle item, then an action toggle item and then a dropdown action item
- Dropdown toggle action code should always appear after with action because it is a specialization from it.

- Dropdown toggle is just a special case from toggle with action. It should appear after in the source code and stories, and just call it.
- The dropdown toggle shouldnt have a vertical border between action and icon.
- The button cycle shows no icons at all.
- The tooltip is not always fitting to the content. E.g. Id tooltips too wide sometimes.
- The label and the ui elements currently have a gap in between which shouldnt exists.
- The label is not always consistent in the height. E.g. combobox is good but select, input and stepper are not heigh enough

- The tooltip is not always fitting to the content. E.g. Id tooltips too wide sometimes.

- A toggle group item should have an optional action prop. This shouldnt be a ActionGroupItem but Action. ToggleGroup should only have ToggleGroupItems as children.
- The dropdown toggle should show the active icon and next to it an action with a select item
- Cycle toggles should be cycle buttons (as they just switch and are never turned on and off)

All ui elements with groups (e.g. action, toggle, button) should be implemented the following way:
The group file export the items and the single ui element is just a wrapped group with one item. This ensures that groups are styled and behave the same single items. All logic and styling should be in the group files.

Remove primary variants from elements.
E.g. toggle should instead

No backwards compatibility necessary. Keep it clean. Refactor all actions to use the icon prop which is automatically size-tiny

Refactor all ui elements. Create a base Element component that has an id (not optional).
Create a base Input component which is an element and has a showLabel prop. Take a look at the existing input components and abstract the duplicated code.

The icon of the toggle with action is not placed properly it should be
SPACE
SPACE ICON SPACE ACTION SPACE
SPACE

Actions should have small size with and icon prop where the icon is automatically tiny.

Update README and AGENTS with console logging based problem solving.

Update README naming and AGENTS rules with the notice to never use `type` and instead always use `kind` to not be confused with the native type in semio. E.g. ArtifactType is ArtifactKind, WindowType is WindowKind, etc

There should be a general panel kind enum (workbench, details, chat, settings, hud, stats, params, etc). Then there should be a general config (e.g. workbench is left, details, chat, settings are right, and so on). Icons and all other things are derived from that.

Update README.md and AGENTS.md rules for @semio/js that the code runs in different environments (different browsers and even on electron, mobile/desktop/tablet). Hence everything that is platform specific needs to be generalized and provided as a prop to Sketchpad.

Most icons are not semantic yet. E.g. Box is used for Scene, Wrench for Workbench, etc. Those are just leftovers of the temporary lucide icons. All icons should exactly describe what they are.

The icon system should be generalized and futureproofed. Currently placeholder icons are used everywhere. From now on Icons are imported drom @semio/assets. @semio/assets internally uses placeholder from lucide but exports them semantically. Ever ui element that uses an Icon should use a semantic icon. E.g. home app uses Clock for TemporaryKit, etc. In the end of the refactor no import from lucide should remain in @semio/js.

label should also be derived from id

Make sure that all labels, hotkeys, tooltips are fetched over useLabel(id), useHotkey(id)

In the navbar all items should have 1 unit spacing

Refactor the id/i18n system to never assign a string directly to the id. But instead always use
{
"label": {
"normal": "...",
"beginer: "..."
}
This way the error returned an object instead of a string can be easily fixed.
Make sure to adjust all code.

Update code, README.md, AGENTS.md
The whole ui system (elements and sketchpad) should be more consistent.
The new ui system has standardized sizes:
Unit: 1. Spacing between elements is always 1 unit. Spacing between icon and the element is always one unit. Etc
Tiny: 3. (e.g. height and width of icons within actions, small text size)
Small: 5. (e.g. height and width of actions, height and width of avatars, default text size)
Medium: 7. (e.g. height of tree items, height and width of toggles/buttons, height of input)
Large: 9. (e.g. height of navbar, height of table row, height of table header).
Huge: 11. (e.g. height of navigation buttons at the bottom of a docs page)

1.5
(3)
4.5
(6)
7.5
(9)
10.5
(11.5)
14

1
(2)
(3)
4
(5)
6
()

Think about a clean way to decouple the Apps. The sketchpad app shouldnt use any implementation or logic from the apps.

The codebase was recently consolidated and refactored. Compare to the old code to find mistakes (the commit before the consolidaton).
I will give you a list of different errors.
Make sure to always go through the old code to understand the code and then apply it to the new code.
Then understand the current implementation by adding debug logging.
After I provide you the logs, come up with a plan to fix it and implement it.
When I tell you that it works again clean up the logging.
Here the first:

- Resizing Panel doesnt work
- Home app details, chat and settings panel are completly empty.
- The details panel in kit app only shows kit section but there are no tree items within it. Check if the old register mechanism is working correctly.

- Design app only shows scene (instead of diagram and scene)
- Navigation doesnt work properly. Back jumps too far, sometimes unavailable, forward is sometimes available and up should just always go one item in the navbar. Every navigate should automatically add itself to the history and sketchpad should cleanly handle it.

Sketchpad should receive an embedded prop which when passes uses the memory router. This is used e.g. for stories. If not, use the default router.

Refactor the Apps to not have the same navbar and footer but instead every app uses the navbar and footer base. Instead wrap every router in the appropriate provider. E.g. kit in kit provider, design in design provider, etc. Make sure the routers are nested e.g. design and type are below kit. Then refactor the navbar and footer to have direct access to useKit, etc without having to provide explicit guid or make a useStore call.

The codebase was recently consolidated and refactored. Compare to the old code to find mistakes (the commit before the consolidaton).
I will give you a list of different errors.
Here the first:

- Navigation doesnt work properly. Back jumps too far, sometimes unavailable and up should just always go one item in the navbar
- Pressing panel toggles doesnt work in any app

The codebase was recently consolidated and refactored. Compare to the old code to find mistakes (one commit ago). The behaviour and styling of the new should be equivalent to the old.

- The old docs app used to show in the left panel group overview with clickable headings tree (that would scroll to it)
- The new navbar has wrong styling, is too heigh, is missing the navigation button group, etc.
- The new panel system is not working and no panel is showing.
- When a new kit is created it shows failed to load kit

The docs app should have an overview left panel instead of a workbench

Think about a more general and cleaner solution for panels.
Panels float ontop of the canvas. Some panels like MIDDLE or BOTTOM are transparent. Some panels like left and right are groupable. For every group there is a dropdown toggle in the navbar.
The (normal: desktop or tablet) layout is

---

## | NAVBAR |

| | MIDDLE | |
| LEFT | ---------- | RIGHT |
| | BOTTOM | |

---

## | FOOTER |

LEFT and RIGHT are horizontally resizable.
Different editors assign different panel to the different category (e.g. Workbench or Explore for LEFT, Hud or Stats for MIDDLE, Toolbar for BOTTOM, Details, Chat or Settings for LEFT)
The mobile layout has just one panel group when the panel is toggeled on:

---

## | NAVBAR |

## | PANEL |

## | FOOTER |

## otherwise:

## | NAVBAR |

## | CANVAS |

## | BOTTOM |

## | FOOTER |

Consolidate all imports to be only once on top of the file.

Consolidate all files into App.new.tsx. Use the existing regions.

1. Use a script to put the content of all (other than App.new.tsx) the files into the regions
2. Remove all other files than App.new.tsx
3. Remove the existing headers
4. Fix imports and other module related code changes
5. Delete App.tsx, Rename App.new.tsx to App.tsx
6. Integrate into existing code.

All apps (design, type, quality, docs, home) in sketchpad (and sketchpad itself) should be refactored to be one file App.new.tsx
Use the existing regions.
The only exception is pages/\* in docs which should remain.

The app folder should look like this:
├── js
│ ├── js
│ │ ├── sketchpad
│ │ │ ├── apps
│ │ │ │ ├── design
│ │ │ │ │ └── App.new.tsx
│ │ │ │ ├── docs
│ │ │ │ │ ├── pages
│ │ │ │ │ └── App.new.tsx
│ │ │ │ ├── home
│ │ │ │ │ └── App.new.tsx
│ │ │ │ ├── kit
│ │ │ │ │ └── App.new.tsx
│ │ │ │ ├── quality
│ │ │ │ │ └── App.new.tsx
│ │ │ │ └── type
│ │ │ │ │ └── App.new.tsx
│ │ │ ├── App.new.stories.tsx
│ │ │ ├── App.new.tsx

The codebase file structure is inconsistent.
There are too many individual files that should be part of

Here a list of files that shouldnt exist:

- SharedTransformControls (Scene related)
- WindowLibrary (Workbench related)
- FreezButton (Footer related)
- HotkeySettings (Settings related)
- TimetravelButton (Footer related)
- ConceptFilter (Canvas related)
- hotkeys (Sketchpad related)
- designAppHooks
- designAppIntegration
- mdx-loader
- mdx-provider

Make sure that all tools_registry/\*\* are just inside the tools.tsx

Make sure that useTranslation(); is never used inside sketchpad. All ui elementes should have an id which is at the same time the i18n key. E.g. TreeSection still uses label.
Check that 100% of id have matching i18n keys

Introduce Params (scope: kit | design | type), formula (scope: kit | design | type) and variant (scope: kit | design | type).
Params are pure input ( number [slider for bounded or stepper for unbounded] | text [input] | toggle | choice [select for set of text]) which can be used to make anything parametric. Params must have a default value. There is a panel group with Params that exposed them in the ui (e.g. design app has two section design and kit)
Variant assigns to params predefinied values.
Formula is anything that needs to be computed.

- Create ActionGroup with Actions (exactly as toggles can have toggle groups or buttons can have button groups)
- Create a dropdown action which can have a value and when clicked opens a menu with all the remaining options. Display always the icon of the current value. There must be always a value present.
- Use an action group for the window controls (open in new window, maximize/minimize, close) in the canvas.
- Add a dropdown action to scene windows for projection (camera | orthographic)

- In the footer of design app

- Coord should use u and v instead of x and y to avoid confusion with points xyz

Refactor the canvas/window system

- Everything should by default use golden layout (and get rid of the explicit golden layout naming)
- Every canvas has an active window (relevant to commands and tools). Use active background on the name to show it.
- Every window kind can register toggles, dropdowns. E.g. A scene window has a toggle
- Implement a drag and drop system in Workbench with tree of preconfigured windows (e.g. scene>orthographic>top|bottom|right|left)

- The desktop mode and the mobile mode are not work

- Importing a zip file in kit app should create the proper files and folders

- Avatars should have the same height as buttons or toggles.
- Every row for every table should have an avatar before the name. Use an icon if available and otherwise use fallback from the name.

- Add to footer of

- Generalize Tools to not only work on app level but also on canvas level.
  E.g. Selection for scene works both for design app (on piece models) and for type app (on port models)
- Make selection tool gneral to work only on the base scene
- Introduce a general tool

new tool: walk tool

The folder feature is not complete.
Designs (only protodesigns), types (only prototypes), qualities and files can have a folder.
Dragging the rows in kit app should set the parent according the folder where they are dropped.
Make sure to properly hightlight everything. All drag and drop preview accross sketchpad should be consistent.
Make sure to use dnd-kit and refactor all existing drag and drop to also use it (and same styling)

- Sketchpad layout system with a canvas and window should turn fully customizable through goldenlayout.
  When hovering over the border of a window then action buttons in the middle should appear for each kind of window. E.g. In design app either diagram or scene windows can be added. If a border is between windows then the new window is created between them. If the border is not touching another window then add another row/column.
  The window state of the canvas should be part of the store.
  Every app has a default window setup.

- Add toggles for the following footer items:
  Grid Snap, Orth, Planar, Gumball

- When modifying the x and y in connection details, the child piece should move.
- The child piece shouldnt have any notice in the details. Currently it shows an editable plane and center which shouldnt be there. Only fixed pieces have planes and center.
- The piece node in diagram should preview fixed pieces with a second circle. Design pieces currently have a double circle. To avoid confusing replace the design pieces with a thick border. Fixed design pieces also receive the second border.

- When making the window smaller it breaks to mobile width and then sets the layout to touch but when the screen is made wide again it doesnt remove the touch.

Sketchpad

- isNavbarExpanded is only possible on mobile. isFooterExpanded is also possible on mobile.
  Refactor the store to one consistent value (string or object for mobile) - layout: "desktop" | "tablet" | {isNavbarExpanded: false, isFooterExpanded}
- remove access. It will be handeled differently.
- change appSettings to "settings": { "apps": { "design": { "diagram": { "proximityConnectDistance": 10 } } } }

- Whenever the initialState is provided (or timetravel is used) try to load all files for the kits into the store. The files for kits are by convention zip files with the GUID.zip under public.
- Extend the drag and drop into kit app to treat zip files different. When a zip file is dropped then unzip it and add all files.

- Whenever a connection is established one of the pieces loses its plane and center. Check the old implementations for details. The plane and center for rendering are calculated through the flatten process.

- A big schema change: type variant and design variant and view are replaced by more general approach: parent (&children)
  From now on types can have a parent type and design can have a parent design.
  All ui elements (such as rows in kit app or avatars in workbench need be changed to support flexible depth.
  Further a type or a design can be abstract. An abstract type or design can't be used by a piece or opened inside an editor.

- When inital state is provided

- The ui elements in settings dont show the label. The mode is missing under expertise.

- Every App receives its own Nabar and Footer file - same as with Panels.
  All

- For now center and plane of pieces were set on every piece. Now connections should be possible again. The algorithm that allows this is flatten design. In the old implementations you can see how it works.
  Introduce new hooks useFlatPiece, useFlatPieceCenter, useFlatPiecePlane, etc and use them in diagram and model.
  Adjust the details. E.g. if piece is connected then add an action to fix it (which uses the current center and plane and removes the parent connection).

- Rename mode to expertise
- Introduce mode: user (default), dev
  E.g. tutorials are not supposed to be recorded by users and only by devs. Temporary kits are only available to devs.
  Add optional user flag to commands. By default all commands are user commands.
- Introduce a new prop to Sketchpad: initialState
  The inital state is a json which enables to load sketchpad for a specific moment.
  Add dev commands to export state and set state

design app:

details:

- created at and last updated at is not showing the label. The date should show time including minutes.

- The details of the representations

- Tutorials should be avaible in search
- Recording button should be on the left of footer.
- The first tutorial is sketchpad tour (some introduction, create temporary kit, crete type, drag and drop file into type app, create two port, create design, drop two pieces of the type, connect both pieces)

Not all ui elements are conistently styled:

- Tabs should use the same active background

- The hover on active pages in workbench should be the same active hover color as e.g. a toggle. Not the normal hover.
- The designs filter toggle and + action toggle have no tooltips (hence wrong id/i18n config). Same for the other filter toggles
- All string placeholders should be placeholderId with a proper id and i18n match.

- The tag-based mechanism for selecting representations should be implemented. For this purpose a (modified) jaccard index is used. The representation with the closest matching tags wins (special: no tags means default and no tags and no tags has the highest match). All available tags should be in the footer and once one is clicked then all representations to be considered a filtered. Then the remaining tags show and once can be selected again. Until no more tags are avaible. This mechanism should be used in type app and in design app. Use the old implementations for reference.

- When selecting another representation in the footer of the type app it should switch the model of the representation to the other. Currently nothing happens when selecting something else in the dropdown. Not even the selected name changes in the dropdown.
- Replace the placeholder cube in design app for pieces with the first representation of the type of the piece. Ignore pieces with a design for now. You can check type app how the model is loaded.

- A new mechanism should be introduced to sketchpad: tutorial (& recordings)
  A tutorial is guided experience/explanation through sketchpad.
  A tutorial conists of milestones.
  A milestone is completed when a certain interaction (a command with a certain origin is reached) then the tutorial is automatically continued. The user can also click next and then the
  Tutorials can turn the focus to ui element (such as an animation where one element is highlighted and the rest is dimmed in order for the user to click it). Tutorials have an active cursor that moves along.
  Tutorials can further have audio and video.
  The length and controls (such as pause/play or clickable timeline with milestones) of tutorials and recordings happens over the footer.
  Think of a clean solution & implement it directly both to be able to record and play tutorials

 receives (e.g. highlight animation for) UI elements such as toggle should receive

- The workbench panel of docs shouldnt have folder and file icons. Currently every folder has a subpage with the same name where the folder is just foldable but not clickable. Make the upper tree item clickable and navigate to it and get rid of the extra tree item with the same name.

- All ui elements should only have an id from which label, tooltip, manual, tutorial, hotkey etc is derrived through i18n (keys are equal to id). Somehow the ui elements use t internally but when switchting the language the only thing that changes is the date format in tables. All the other stays the same.
- Check for outdated locales
- Some ui elements still have labels, tooltips that use direct text strings. This should never be the case. Every text/string that is displayed to the user must use the i18n setup.
  A lot of ui elements optionally can display the label. Turn label into a boolean flag instead of the string. Depending on the flag use the id for fetching i18n.
  -Make sure 100% of all commands executed ad the id to the call. Adding an origin should not be optional.

- Hotkey settings should just regular tree section and home, panels, views, tools, etc should just be nested tree items. The individual hotkeys have no ui element to change the hotkey.
- Hotkeys 1,2, …,9, 0 dont work

- Extend and refactor all ui elements in sketchpad.
  Every ui element has an id. e.g. <Sketchpad kind="semio">, Design: <App id="design">, <Details id="panel.details">, etc and then from this a complete id such as "semio.sketchpad.app.design.details

- Recordings of sketchpad should be introduced

- A new mechanism should be introduced to sketchpad: tour
  A tour is guided experience/explanation through sketchpad.
  A tour conists of checkpoints.
  A checkpoint is completed when a certain interaction (a command with a certain origin is reached) then the tour is automatically continued. The user can also click next and then the

E.g. The kit editor tour could be something like: 1. Create a type

- Refactor all commands to include an origin which is a string that describes the origin of the command (such as the id of the ui element). Make sure that every command triggered from the ui sets the id correctly. If the ui element doesnt have an id yet, assign it (the same id is used for i18n)

Refactor the hotkey system. The new hotkey system should be derived from the language and can be overwritten by the user.

- The default hotkeys are read from the locales files.
- Add a Hotkey section to settings below the general settings. Use the i18n key hierachy for the nested tree items. Add an action to the hotkey section to restore it default.
- Inside a tooltip where the hotkey is shown when the user clicks it, it should take the user to the hotkey settings (open settings and unfold everything but the selected hotkey and all the ancestors that lead to it.)
- In general every app should follow the guideline: Most important interactions should be triggerable by hotkey with 1,2, …, 9, 0
- All tools of the toolbar should be activatable by hotkey with 1,2, …, 9, 0 in the order they appear in the toolbar. This is the default for all apps with tools.
- All filter toggles (e.g. home app or kit app) should be activatable by hotkey with 1,2, …, 9, 0 in the order they appear

- A big schema change was made.
  Types previously

- Refactor the state managment of all sketchpad components.
  Components should never use the store directly and never have to do any computation, obervation, selecting themself but only use clean hooks which are implemented and exported by the respective stores.

- 8.10 Email von Lucie Leder an Kinan mit Aufforderung der Unterlagen 12.10. Antwort von Kinan

i18n issues:

- tooltip.manual and tooltip.tutorial are not showing
- home app: temporary kit, local kit and remote kit toggle have no tooltips. all sort toggles have no tooltips
- kit app: all filter toggles (designm, type, quality, file) have no tooltips. all sort toggles have no tooltips

tooltip formatting issue: Manual, Tutorial and Hotkey are all optional. It should always fill the line with equal spacing.

- Type editor should have a dropdown in the footer for selecting a representation. The scene then uses the representation of this file to show the model.
- Files from system should be droppable into type app. When this happens then a new file is created, a new representation that references this file and the representation is selected in the type app.

- Representations currently have a url that are either relative urls to files in the kit or remote (e.g. starting with http).
  The new representations always reference a file (same as a piece references a type or a design) in the kit.
- Make sure that after dragging the file into the kit app the files appear as rows. According the path they should nested.

- A lot of keys are not consistent. Make sure that all keys follow the explicit structure:
  e.g. "semio.sketchpad.app.design.panel.details.section.design.name" for the name of the design section of the details of the design app in sketchpad.
  Check all ui elements. Currently a lot mismatch.
- E.g. tooltips for all toggles dont work

- Sketchpad can be used in-memory only, locally persited or remotely synchronized.
  Currently it works with yjs.
  Now files should be added to kits. Files are too large to be part of the yjs doc. Files are consumed by other components over urls (URL.createObjectURL). Similar to the yProvider there should be an optional fileProvider prop that should be passable to Sketchpad. When fileProvider is passed, then it should automatically sync the files of the kit. Design the fileProvider api general so any backend provider can be used. Implement the example for s3.
- Files from system should be droppable to the kit app canvas.

- Assigning label, tooltips (label, manual, tutorial) currently happens in code directly. It should be refactored that every ui element receives an i18n string key id prop and everything is moved into the locales json files (even the paths)
  e.g.
  <Stepper i18n="semio.type.panel.details.port.direction.y" >

"semio.type.panel.details.port.direction.y.label": "Y",
"semio.type.panel.details.port.direction.y.description": "Y coord(inate)",
"semio.type.panel.details.port.direction.y.description.beginner": "Y diagram coord(inate) of center of the piece.",
"semio.type.panel.details.port.direction.y.manual": "semio/design/diagram/coord#y",
"semio.type.panel.details.port.direction.y.tutorial": "metabolism/thinking-about-the-diagram",

- All ui elements in details should show a hover effect and a tooltip with a short description of what the field is about, manual and tutorial path. const { t } = useTranslation(); shouldnt exist afterwards.

- Whenever an app starts a transaction if there is an ongoing transaction then the ongoing transaction is first finalized.

- Tooltips with neither manual, tutorial or

- sketchpad is receiving a custom context menus.
  Every ui element can have a custom menu.
  The styling of the current context menu doesnt match the other ui elements (border, temporary layer, 1 unit spacing, 9 units line height, …)
  A right click on ui elements with no context menu nothing happens

- New cursors were added: selectable and foldable
  They are specializations of the clickable pointer.
  Make sure to add them to all ui elements
  Make sure to replace the clickable which are either selection or foldings with the specific one.
- Sometimes forbidden cursor shows despite elements being functional (E.g. on actions such as in workbench in design editor)

- Every page in docs should have buttons with the previous page and the next page
- Every folder in docs should have in the end a tree with all subpages and subfolders

- The search currently shows plenty of titles for empty groups such as All designs, All types, etc.
  First shorten the names to just Designs, Types, etc. Second only show the group if there are items in it.
- The recent list in search should not just be the last item but all last items. The list should be scrollable and not end.
  Same for recently focused.

- The avatar colors in workbench in design editor should also use the transitive hover colors
- The piece nodes in diagram in design editor should show the same as the type or design avatar.

- A tooltip mode used to exist which was just duplicated for mode. There are still remainders of the old code:
  The requested module '/elements/display/Tooltip.tsx?t=1761601385498' does not provide an export named 'TooltipMode'

- The > of the navbar in the docs app are not showing all the options. Show all sibling pages in the same folder with the correct order.
- The panel toggles of docs app work but dont show the state. E.g. workbench displays always active and details never.

- The mdx styling in docs app is not yet fully functional. E.g. headers are currently displayed as normal text.

- Sketchpad should remain a list what was recently opened (ordered set). When opening the search the default is showing the recents.
- Same for recently focused for each app.

- Generalize the

- Refactor the js/js code base to be closed for modification and open for extension. This means that adding new features should just be adding files and folders and not having to edit existing ones.
  E.g. Adding a new editor should just be adding a new folder under editors; Adding a new tool should just be adding a new file under tools; Adding a new panel should just be adding a new file to the panels folder.

- The toggle group should receive a new variant: tree
  A tree is a line-based interactive selection for tree nodes.
  E.g. home editor has the tree
  root

  > KITKIND (Temporary | )

- All avatars should be rendered consistently. E.g. the

- The section panels on mobile should have the same content as on desktop

- The designs of the workbench in the details of the design editor are not updating when designs change.

- kit editor and home still have no panel toggle for details
- Plenty of i18n keys and translations are missing
- Multiple section of multiple pieces in design editor is showing type under a redundant multiple piece tree item.
- Multiple section of multiple ports is showing in type editor
- All tree items with no children should not never have > for folding/unfolding on an empty list (e.g. authors, representations, etc)

- kit editor and home still have no panel toggle for details
- the kit section has a tree item kit which shouldnt be there as intermediate (e.g. in design editor or type editor)
- The type editor has too many sections (ports, representations, etc.) which should all be tree items under type section
- When selecting multiple piece in design editor it shows Pieces > Multiple Pieces but should only show multiple pieces with the nesting.
- All plurals should always show in the section name Multiple to make it more explicit.
- E.g. Locatin is showing no > when not existant but Authors and Attributes are showing it even when the collection is empty. Only show the > for non empty children.

- The details are currently not consistent. They should always display sections from most specific to most general (top to bottom). The general sections dont disappear when going more into detail but just go to the bottom. Every section has a multiple equivalent which replaces the single section. E.g. When one kit is selected then a kit section is shown, if multiple kits are selected then multiple kits section is shown.
  Here some examples:
  In home there should be a kit section if a kit is selected.
  In kit editor there should be on the bottom always the kit section. If a design is selected then the additional design section is above the kit section. If only multiple types are selected then multiple types section is above kit. If different artifact kinds are selected (e.g. designs and types) then multiple artifacts section is above the multiple designs section which is above multiple types sections which is above the kit section.
  In design editor there should always be kit section on the bottom then design section above it. If a piece is selected then the piece section is above the design section.
  In type editor the same with kit and type then with added sections for selections (port)

design editor scene:

- Generalize transform to model (e.g. design editor scene piece should be model) of the general scene. Every model can have a plane (semio). Models can be transformable in which case they show gumball transform controls. There should only be one transform gumball for all selected models (in the average plane) and transforming should affect all of the models at the same time.

- Details should always change according to the selection.
  When nothing is selected then show general details inside a section called like the editor. All props are nested tree items. E.g. in type editor the section is called Type.
  For every kind of selected entity add another section. This section changes for singular and multiple. E.g. in type editor port and ports; in design editor piece and pieces, connection and connections.
- Ports detail should have parameter t slider

- Table rows should be scrolable
- Header of pages are rendered correctly (e.g. in docs editor)
- Headers in details in docs editor are not appearing. All headings should appear under headings section with tree items that focus on click.

- All editors should provide a way to scroll/zoom towards indivdual element called focus.
  In sketchpad you can press ctrl + f to open the focus. It works like the search. There is an icon in the navbar and then the dialog opens where the user can type something in. Then a list with the closest items appear. Once pressed the editor zooms/scrolls towards the element.
  All state is stored in the fragment portion of the url.
  The kind of interaction is editor specific. E.g. diagram has nodes and edges (e.g. design editor: pieces and connections) to zoom towards; page has headings to scroll towards (e.g. in docs editor); scene has models (e.g. type editor: ports; design editor: pieces) to zoom to; tables have rows to scroll to (e.g. home: kits; kit editor: artifacts).
  Make sure to implement the functionality on the general components (scene, diagram, table) and not on the specific (design editor, type editor, kit editor, home, docs editor)

- Tooltips and translations should be generalized. A tooltip should be a component. A tooltip can have a key (for i18n) for the label, a path to the manual, a path to a tutorial and a hotkey. All are optional. Navigation, rendering, etc is all done by the component.
  Beginner mode: tooltip should include a link to the manual and a link to a tutorial.
  Normal mode: tooltips should include a link to the manual.
  Experts mode: no tooltips as before.

- Make sure to remove all content specific code from docs editor, navbar, …
  No hardcoded section, pages, icons, … The content should 100% just based on the folders and files. Add all necessary information about names of pages etc into files with extending the frontmatter. For folders use index.mdx files.
  In the end there is no getting-started, tutorials, … inside any file in the folder of docs editor.

- The home screen should be extended to include docs.

- The docs are under a heavy rewrite process. They used to be a separate astro starlight package. Now the docs will be integrated into sketchpad. This will enable the docs can be directly accessed within sketchpad.
  The docs are written in mdx. Metadata should be part of the frontmatter on the top of the code. They use the same navigation system as kits but with docs/ path prefix. The paths are determined by the files and folder names.
  The workbench of the docs has currently 6 tree sections: getting-started, tutorials, integrations, manuals, theory, showcases; All folders are translated into sections and files are translated into pages. Sections can have state that can be modified by the pages (e.g. tutorials can store progress, etc)
  All used astro components in mdx need to be replaced by semio elements. If not a similar one exists, new ones need to be created.
  Install all necessary frameworks for mdx.

Still

- No details are showing in quality editor
- In workbench workbench should be a second section with qualities. Use the dot separated key to create groupings e.g. semio.area.floor should be in semio/area as avatar
- Latex formula is never rendering in quality editor
  Further
- Tree sections should never be nested. A tree just has tree sections at the top level. For further nesting use tree items e.g. quality editor should just have two sections: functions and qualities.

- Properly generalize the different windows (diagram, scene, table)
  All of the specific instances of the windows should never import base components. E.g. all diagrams should be <Diagram> and not <ReactFlow>.
  Introduce <Model> to Scene which is a <group> that automatically is selectable, hoverable, etc. Along with the consistent colors, etc. Generalize as much as possible and take as much of the configs into the general component. It will lead to a uniform experience.

- Introduce proper error mechanism to canvas and loading mechanism to windows.
  Show errors and loading animations. Every window should provided loading skeletons. E.g. Table has loading skeleton rows; Diagram has some skeleton node circles and edge lines; etc for all windows.
  E.g. home table needs to load kits; kit editor needs to load artifacts. Design editor currently initializes diagram and scene but then jumps quickly to correct center, camera, etc.

- Dragging a piece in scene in diagram editor doesnt update the plane. Maybe it also just doesnt show in details.

In the workbench there should be a second section with quality avatars to drop

- The quality editor is a forced vertical layout. Each node has + placeholders (a connection and a node) where other functions or quality avatars can be dropped into. Functions should have an icon. The order of rendering is equivalent to the order in the s-expression of the formula
- Generalize avatars as a ui element. All avatars have same size, same colors/cursors for selection/hover/drag, borders etc. Take a look at design and type avatar in design editor.

- E.g. holding the stepper for x of the origin of the plane of the selected piece in the details of the design editor doesnt live update the piece in the scene. Only when clicking into scene it shows the updated piece.

- E.g. holding the stepper for x of the origin of the plane of the selected piece in the details of the design editor doesnt live update the piece in the scene. Use the same diff piece approach as node in diagram where the mesh colors changes on the status.

- Moving a piece in scene in the design editor should be scoped within a transaction and finished on drag end and aborted with escape.

- E.g. when using the stepper (clicking and holding) for in design editor details for changing x of the center, then the piece node has a dark background and not the changed background. It should be within the transaction and due to the diff should be changed color (which derives from warning color).

1. Add all cursors (normal and dark) to style
2. Use the proper cursors in the ui elements

- The code (navigation in nabnar, stores, etc) should be generalized. Currently the navbar is resposible for knowing about the different type of editors etc. The code should all be part of the editors. Code should be closed for modification and open for extension. A new feature (such as a new editor, new tool, new windows, …) should just work by adding code.

- The code has been recently refactored. Some things might not be finished. Probably typescript helps to find some of those errors.

This task was started:

- A new editor should be introduced: quality editor
  The main purpose of the quality editor is to build a formula s-expression string and render it nicely.
  It uses a canvas with two vertical windows: formula and diagram
  Formula.tsx is the MathJax rendered ribbon window on the top of the screen.
  Diagram.tsx is a dagre forced vertical formula builder with drag and drop of function from the workbench panel (similar to how drag and drop works in design editor).
  The exisitng functions are numeric, branching, data structures, etc
  Every function defines a way to calculate it based on the operands and a way to render itself in latex.
  Formulas: Function names start with a capital, quality keys are lowercase with dots, variables start with $ and lowercase, units are first class citizens e.g.
  If ( StartsWith ( Name ( $semio.design "Nakagin" ) ) '20 m' '23 m' )
  If ( Smaller ( semio.floor-area.usable '100 m²' ) Divide ( usalu.area.first '3' ) $semio.design.connections )
  InList ( '100 cm' List ( '1 m' '200 mm' )
  HasKey ( '100 cm' Dictionary ( KeyValuePair ( Key ( '1 m' ) Value ( "One meter." ) )
  or another snippet:
  {
    "qualities": [
    {
    "key": "semio.area.floor.gross",
    "name": "Gross Floor Area",
    "locales": {
    "de": "Brutto-Grundfläche"
    },
    "description": "The gross floor area encompasses all floor areas within the external dimensions of a building.",
    "si": "m²",
    "imperial": "ft²",
    "symbol": "A*{gfa}",
    "formula": "Add ( semio.area.floor.gross.net semio.area.floor.gross.construction )",
    "format": "#,##0.##"
    },
    {
    "key": "semio.area.floor.gross.net",
    "name": "Net Floor Area",
    "locales": {
    "de": "Netto-Raumfläche"
    },
    "description": "The net floor area is the usable floor area excluding construction elements.",
    "si": "m²",
    "imperial": "ft²",
    "symbol": "A*{nfa}",
    "formula": "Add ( semio.area.floor.gross.net.usable semio.area.floor.gross.net.technical semio.area.floor.gross.net.circulation )",
    "format": "#,##0.##"
    },
    {
    "key": "semio.area.floor.gross.net.usable",
    "name": "Usable Floor Area",
    "locales": {
    "de": "Nutzungsfläche"
    },
    "description": "The usable floor area includes all areas directly used for the building's intended purpose.",
    "si": "m²",
    "imperial": "ft²",
    "symbol": "A\_{usbl}",
    "formula": "Add ( semio.area.floor.gross.net.usable.living-staying semio.area.floor.gross.net.usable.office-work semio.area.floor.gross.net.usable.production-experiments semio.area.floor.gross.net.usable.storage-distribution-sales semio.area.floor.gross.net.usable.education-culture semio.area.floor.gross.net.usable.healing-care semio.area.floor.gross.net.usable.other )",
    "format": "#,##0.##"
    },
  }

What doesnt work:

- Pressing on details panel
- No workbench panel with functions
- Navbar doesnt update when inside quality editor.

Make sure to add all functions

- Generalize the navbar, canvas, footer, windows, scenes, panel and panel group, tables and make them reusable. Decouple them entirely from sketchpad. All code inside elements is not domain specific.
  E.g. Left panel group, middle panel group, bottom panel group(below middle and between right and left) right panel group on desktop and a huge panel group with

- The tool mechanism should be generalized.
  Every active tool has a render function where the state of the editor and the kit is passed similar to command context. The tool can contribute children to the different kind of windows. E.g. a type editor tool can contribute r3f-compatible children to the canvas. A design editor tool can contribute nodes and edges to the diagram, and r3f-compatible children to the canvas.
  All tool related code should be completly within the tool. E.g. The port tool currently has logic spread around Canvas.

- design editor and type editor are both kit diff editors. The current kit diff should be displayed in every editor. For every hook like usePiece introduce a new hook useDiffedPiece. E.g. When a piece has center diff then the original node should be shown with muted border and the diffed piece should have a changed background color. In scene the original piece should only have edges and the diffed piece should have the changed mesh color.

- All ui input elements in details should all be transaction bound (e.g. input, textarea, stepper, …) and when starting the interaction start a transaction and when ending it with enter (or loosing focus) finalize the transaction and when pressing escape aborting the transaction

- Add descriptions only to tooltips and never to labels.
- Dropdown toggles should never show the current selected option.

- When selecting a port (or ports) the first element should be a slider for t [0 to 1]. It is interactive hence when moved all panels should turn transparent but the slider. Same as when drag and dropping avatars in design editor workbench

- When a new piece is created without any connections that connects it, then plane should always be set to the default plane (origin:0,0,0 xAxis:1,0,0 yAxis:0,1,0)

-Every toggle should have different tooltips according the state and options. E.g. the dropdown toggle for the panels in the navbar should be: "Show Details Panel" when the toggle is off and "Hide Workbench Panel" when on. The label for dropdown should be "Show Chat or Settings Panel".
Make sure to add tooltips for all toggles for normal, extensive, different languages, …

- The tool mechanism should be generalized.
  Every editor has tools and always one can be active.
  A new panel type is introduced called tools. Normally (not on mobile where there is only one panel group) it is part of left panel group.
  Every tool has an id e.g. "semio.typeEditor.port", a name e.g. "Port", a description e.g. "Create a port on the surface of geometry with the normal direction of the surface." and an icon.
  Tools from the

- When clicking in the empty threejs canvas then deselect everything
- When adding a port with the port tool, it shouldnt be added to the selection
- The details panel should just have one Type section and the other sections are just tree items

- When inside the design editor and new design, variants or views should create a new design and navigate to it. Note that when creating a new variant then the name of the design is from the parent. A new view inherits the name and variant from the parent. E.g. when inside design editor clicking + variant should create New Variant and New Variant 2 if a variant with this name already exists, etc

- The type editor should have a hover and selection for representations and ports.
- The type editor should receive a new tool: Port tool
  When the port tool is active then the cursor in the scene is mapping to the mesh previewing the port (a point on the mesh and the normal direction). When clicked then the port is created. Click& hold should still do usual orbit etc. Only the preview and the click are different on the port tool.
- The port tool should show in the toolbar (all tools should automatically show in the toolbar of the respective editor)

The state managment has recently changed. Previously all entities have been directly passed as props (kit, design, type, …). The store has now hooks (useKit, useDesign, useType, …). Further all referenceable enties have a guid. Refactor the code to make sure that the state is only accessed over hooks and references only use guids.

- The navigation in the navbar should always start next to the navigation buttons. The panel toggles should be left from the fullscreen toggle. The navigation should fill until the panel toggles.
- When the kit editor has a kind filter then for every unique name there should become a toggle. Use search params to store ?name=NAME. After a name has been selected show the toggle and then show all unique variant names. Same for views. Analogous for type that uses name and variant.

- The horizontal spacing between the navbar items is too big and should be the same as the vertical space between the items and the border.

- The toggles from kit editor for variant unique names and view unique names should include Default (displayed as in the navbar) if there is a default.

- The panel toggles of type editor are not working
- The dropdown in the navbar does not fit the content.
- All UI elements should have the same height. E.g. Breadcrumb is not the same as toggle groups.

- Showing additional design rows in kit editors still doesnt work.
- The design editor store should be expanded to include
- When a new kit is created it should create the default version and not 1.0.0
- When toggeling temporary kit in Home I get:

The code base is inconsistent and not as general as it could be. A lot of utility is spread and repeated.
All domain logic should be in semio.ts
All state should be in store.tsx - kits have their own (later cloud) synced yjs document; sketchpad has its own local state (optionally persisted when provided with an id)
Everything that is reusable should be exported by index.ts and imports should be from "@semio/js" unless the they are only internal then they should be pathbased imported.
All react hooks should start with use and be named as concise as possible.

- The protoype is finished. Now it is time to cleanup and refactor.

- Showing additional design rows based on the hierarchy name -> variant -> view in kit editors still doesnt work. There is just one row and it cant be expanded despite the chevron showing.
- In kit editor when clicking the the design kind, then clicking one of the names, an empty toggle (probably it should be default in italics same as in navbar) but then the name again repeats instead of showing new toggles for the unique variant name. Same problem once more for view.

- The previous kit name repeating is because the new variant are called New Design for both new variant and design. It should be New Variant and New Design.
- This still doesnt work: Each design editor should store its own camera of the model.
- Currently if a new design is created with a default view and variant then three rows appear. The first row (parent) should be the default variant and view. If a second variant exists the it is beneath it with the name Variant: VARIANT. If it is the default view then there is no additional row.
- The toggle logic of the unique names, variants and views is the same as with the kind. Once one is selected then all the other options dissappear and only the selected toggle is shown.

- Toggeling on Home works similar to toggeling on the kit editor. The toggles just affect what rows are shown. Currently as soon as a kind toggle is used there is a new message with No kit loaded
- When in kit editor with design kind toggled on the > on the artifact kind shows + Create which shouldnt exist because the artifact kinds are set.
- Default should be renamed to Default Version, Default Variant, Default View.
- When clicking on the navbar automaitcally the right search params should be set e.g. clicking on VIEW in the design editor HOME > KIND > KIT > Designs > NAME > VARIANT > VIEW should go to kit editor with the right path and ?name=NAME&variant=VARIANT&view=VIEW

- This still doesnt work: Each design editor should store its own camera of the model.
- Rename tooltips to be consistent and explicit. E.g. View all kinds should be Click to expand all kit kinds, Temporary to Click to see all temporary kits, View all artifacts to Clieck to see all artifacts that are port of the kit
- The type rows are not displayed same as the designs in kit editor. They work the same but without the view.

- This still doesnt work: Each design editor should store its own camera of the model. The diagram point of the center of the diagram is remembered but starts flickering and never stopping once navigated.
- Default (e.g. in toggles of kit editor) should be renamed to Default Version, Default Variant, Default View.
- New Types are called UNNAMED but should be New Type

- The design editor camera is not remembered.
- Also store the type editor camera.

- Make all i18n keys consistent and explicit. For all languages.

- The horizontal spacing between the items in the navbar should be smaller and equal to the vertical spacing to the horizontal borders.

- The heights of the elements should all be equal but they dont match: A toggle within a group has 57px, breadcrumb 53px, toggle 54px, input 54px, table header 61 px.

- The border of navigation should fill until the panel toggles
- The lazy input should abort when escape is pressed
- The textarea highlight should just be a primary border like input
- The search input in home and kit editor mostly wrap on a new line but they should just fill the space and be minimally a search icon.

There should be three level of tooltips: None, Consice, Extensive
Sketchpad store and settings should be able to set it like theme, layout, language
By default tooltips are extensive
All i18n keys should have a further .extensive with the extended version
In general consice just describes what it is but without telling the user what to do. E.g.
Go back vs Click to go back, hold to see history
Expand kit kinds vs Click to expand all kit kinds

- Tooltips are not changeable in the settings
- Whenever something is clickable and the tooltip is shown then the cursor should also indicate it

- The dropdown in breadcrumbs dont indicate clickability with the cursor
- Tooltips level can be changed but then also all tooltips should change. Make sure every key and every ui element is using this.

There are different type of kit stores: isLocallyPersisted (a getter property which is true when indexeddb persistence is set), isRemotelySynced (a getter property which is true when yDocProvider is set), isTemporary when not locally persisted and not remotely synced.
The home should be a table view as the kit editor is but instead of having a filter for different artifact types it has filter with create actions for Temporary, Local, Remote. Expand the createKit command with two Boolean flags local, and remote.

The details panel of the kit editor is just showing no selection section but it should show all the general editable elements.
Dont create a separate TransactionalInput but add a lazy flag that when given only fires onChange at when enter is pressed or another element is clicked and only aborts on escape.

The command mechanism is not yet (properly) implemented. There are 3 kinds of commands:

- sketchpad: only access to sktechpad state. no access to kits.
- kit: Only domain logic, nothing ui related. Only access to the entire kit and the fileUrls which are files loaded in memory by a path (e.g. representation/capsule.glb) and an url (result from URL.createObjectURL(blob)). Returns KitDiff and Files (Not semio files). When files are returned then check if the path already exists in memory and replace existing if so. Make sure no memory leaks.
- design editor: Access to kit and design editor. Returns KitDiff and DesignEditorDiff. Transaction mechanism. Stores two stacks of edits. One for the current transaction, one for the past transactions. Undo/redo acts during a transaction on the curent stack and outside of a transaction on the past. If a transaction is finalized then the edit is pushed on the past stack.

- The Tooltip levels should be generalized to Mode: Beginner, Normal, Expert. Same behaviour but in the future more than tooltip will be derived from it.
- Instead of using useTooltip for the the level, turn it into const tooltip = useTooltip(key)
- The i18n keys are not consistent. Every extensive key should include the action it takes in the beginning such as Click to

All UI elements must work in this three levels:

- Level: Background, Panel, Temporary
- Every level has a darker background color than the previous one (or lighter in dark mode)
- Background is default and is the lowest. It has the default background color.
- Panel is a permanent toggleable panel level.
- Temporary is for temporary menus that appear on click events.
- All effects (such as hover) must work and be distinguishable. E.g. currently the hover color of toggles is the same as the background color of the panels.
- Light/Dark works over css and is not handeled in the code but the right color must be chosen

- Beginners should whenever seeing a screen for the first time receive a tour.

- Storybook should receive a toggle for switching between system, light and dark mode. It should work like in Sketchpad and modify the stories. This way no story needs to be duplicated for light and dark.

- The hover color in dark mode is not dark enough

- The hover of the toggle action should be the same as on the toggle
- The hover of the navigation breadcrumb doesnt match the others.
- The background of temporary level is darker as base. E.g. dropdown of language in settings or navbar is temporary but it is lighter as panel.

- The navbar should show a toggle group for all panel toggles
- The toggles for the panels on the kit editor work but not in the design editor

- Scan all components for hardcoded English words and use i18n (provide en and de)
- Update the tooltips to always match what happens when you click (often it shows the current state which is not correct)

- E.g. when the design editor is active then there should be another breadcrumb item designs. This offers all designs as selection. Same for other kinds.
- When in the navbar e.g. designs are pressed from the dropdown then the kit editor should set the active filter kind to designs. Same for the other kinds.

- Input should have a border
- Replace icons for Mode in Settings
- The floating panels dont consider the footing and dont have spacing towards the bottom as to the side or up
- Not all tooltips for ui actions have extensive equivalents

- On mobile the touch option should disappear in the setting and always be evaluated to true.
- The touch spacing currently almost doubles everything. Make the effect less dramatic to be around 1.5

- The tree indentation lines are too light
- Inputs in the panel have currently light text and no (visible) border.
- Textarea has on hover a thick border which should not be case and just change the border color to primary

- Inputs should always have a border such as the other elements. E.g. design editor details has no visible borders on inputs.

- The navigation on the navbar shows KIT > KIND but it should show KIT > VERSION > KIND
- When toggeling default version, variant or view then the navigation in the navbar doesnt update
- Clicking on the version should open home with the right search params
- Clicking on the variant and view should open the kit editor with the right search params

- In design editor details panel it shows Design > Design > which is doubled.
- Only Tree sections should be capitalized and greyed out. Tree items shouldnt.

- Add border around actions of toggle

- ui elements in general have borders. All toggles, buttons etc. Make sure they are conistent in background color, hover color, etc. All elements can be either base, panel or temporary layer. All colors must be consistent across each layer.

store.tsx:6996 Encountered two children with the same key, `piece-undefined`. Keys should be unique so that components maintain their identity across updates. Non-unique keys may cause children to be duplicated and/or omitted — the behavior is unsupported and could change in a future version.

- The text sizes are inconsistent. The children of the tree items have larger fonts than the section and the items. The font size should only decrease.

- The 3d component compisition has changed. Scene (a canvas) has Models (design, type, file, …)

A new mechanism should be implemented. The panels should turn transparent when interacted with. E.g. When an avatar is dragged then only the visible avatar should remain visible. Or if a slider is moved then only the slider should be visible.

- When hovering over the toggle box and the cursor is not inside the action box then the toggle box minus the the action box should have a hover effect. Currently the complete toggle box is highlighted.
- Some the panel toggle of the navbar doesnt have the proper toggle with action styling (box with a small box inside). All dropdown toggles should look like the one from home and kit editor.

- A single toggle has currently not the same size then a group with a single group. There should be no difference. All single line elements (toggle, toggle group, input, breadcrumb, …) should have exactly the same height.

- There should never be nested Tree sections. Location Authors, Attributes should just be tree items with actions.
- Removing Location from design editor details doesnt work
- Remove the Metadata tree item and put created at and updated at directly in the section.
- Removing an author shouldnt be a separate icon but an action on the tree item

- The breadcrumb currently breaks always on the same spot when on mobile. Instead make it dynamically take as many lines as it needs.

Design editor details panel:

- Adding a location works but removing doesnt.
- Adding and removing author and attribute should be like adding and removing a location
- Use conistent icons (+ and - not trashbin or similar)

General:

- In general when something has the active state then it is primary. But when hovering over an active should also be visible (e.g. a toggle that is on). Add the hover to all elements that remain active and still can be hovered over.

On mobile screen width the workbench panel shows: No workbench sections available
Just resizing and then types and designs show. This should not happen.

- Adding authors should add futher nested tree items to authors. Authors tree item has a + and each author tree item has a - to remove it.
- When hitting + on attributes nothing happens. It should be same as authors.

- The navigation in the navbar is not updating according the name and version toggles as the kit editor is. Home should work equivalent. E.g when selecting a version from the dropdown then it doesnt even show in the navbar afterwards.

- The view toggles should be shown in the kit editor after design, name, variant is selected. Same as

- On mobile the active toggles should be on the first lines and the suggested toggles always on the second line. E.g. Once a design name is selected the toggle should appear next to the design kind toggle. Then in the second line all variants appear. If one is pressed then it gets added to the first line next to the name. Same for version. The home editor should be the same with kit name and version.

- Somehow the sort toggles in home and kit editor have no border. This should not be possible. All toggles should always have a border.

tree:

- When adding children to the tree item then they should have exactly the same space as items in the navbar. E.g. design editor details name and descriptions borders are touching but should have 1 space.

design editor details:

- removing Location doesnt work
- adding attributes doesnt work

- A double click on a type piece should open the type editor.
- A double click on a design piece should open the design editor.

- E.g. when inside design editor and I press on the name, variant or view then it should navigate to the kit editor with the right filter parameters.

- The current workbench is only for design editor. Make sure to generalize workbench such as settings and details where sections can be mounted. Add the design editor from within the design editor.

design editor details:

- After adding one attribute, the + action does nothing. - deletes the attribute and then a new one can be added again but not never more than one.
- Sorting of authors and attributes doesnt work.
- Concepts should be a tree item with a plus on it with

- The old connection used to have a composite id key but the new connections have guids. Refactor all the code still using the old complicated way.

- Selecting pieces (clicking on it) in design editor doesnt work

- Add select mm, cm, dm, m, ft,

- The action toggle of drag

- A tool mechanism will be introduced. Every editor can have an active tool. E.g. design editor can have the selection tool (there are three different selection tools: normal, additive [while holding shift], subtractive [while holding ctrl]. Another tool is the lasso tool (rectangular and freeform).
- A new panel is introduced: Toolbar. The toolbar has general purpose tools that every editor has such as undo and redo, specific tools such as design editor tools and custom tools.

- The toolbar panel toggle should be between workbench and details/etc

- Clicking on the VERSION of the navbar should navigate to the kit editor for this specific kit
- The navbar in the type editor is not updating as the design editor. It is analogous to name, variant but without view

- When selecting one piece, then the multiple piece section

A new mechanism should be introduced: Tiles
The main canvas should

- A new ui level is introduced: overlay (base < panel < overlay < temporary )
- A new panel group (similar to chat/details/settings) should be introduced: hud/stats
  This panel group is special because it is doesnt have a background and is just overlayed. It takes the remaining space in the middle (left and right bound by workbench and on the bottom bound to toolbar)
- workbench is upgraded to a panel group. The second member is tools.
- The toolbar toggle should be between the workbench and details/etc
- When selecting a piece, the details panel shouldnt toggle on

- Toggeling the tools panel doesnt open any panel
- The hud/stats panel group is missing a dropdown toggle
- The toolbar panel toggle should be generalized to toggle tools in general. E.g. diagram and scene in fullscreen show tools such a gizmo, minimaps, etc.
- The panel toggle group somehow has sometimes double left and upper border in design editor and a double left border tables

- The two line search / toggle layout in the tables should become more flexible and wrap automatically instead of hardcoded two lines. Make sure Search just fills at the end.
- The section mechanism doesnt register and shows nothing when the screen sizes drops to mobile.

- The toolbar panel toggle should be generalized to toggle tools in general. E.g. diagram and scene in fullscreen show tools such a gizmo, minimaps, etc.
- Remove the toolbar panel toggle from the panel toggle group and add it on the right as seperate toggle (both on normal and mobile)

- When clicking on the navbar then always navigate to the right table with the right parameters
  Currently often nothing happens

- Ever edior should store a value between 0 and 1

E.g. HOME > TEMPORARY > KITNAME > KITVERSION > DESINGS > DESIGNNAME > DESIGNVARIANT > DESIGNVIEW
When clicking on TEMPORARY then the temporary parameter should be set
When clicking on KITNAME then the temporary and name parameter should be set
When clicking on DESIGNNAME then the name parameter should be set
When clicking on DESIGNVARIANT then the name and variant should be set
When clicking on DESIGNVIEW then the name, variant and view should be set

Further when toggeling the variant in kit editor for design then all different views should appear as toggle. Same mechanism as with name and variant.

- In home and kit editor the panel toggle of the navbar has somehow a double left border. Something like that shouldnt be possible.
- The tool toggle in the navbar works but is not showing active state
- The fullscreen toggle cycle should be a normal toggle with the fullscreen icon and active state.

- Every clickable element should show it with the cursor. Currently this is spread on some individual instances but should be implemented in general to work for all clickable elements. E.g. breadcrumb, toggles, etc. Basically all clickable elements already have a tooltip. Now they should also show it on the cursor.
- All dragable/movable elemetns should also show it with the cursor.
- Same for resizable

- When selecting a sort mode for a column in table then the sort is active but the toggle state is not set to on.
- The sort toggles dont have a border. A toggle without a border shouldnt be possible.
- When clicking on a row in the table it should add it to the selection of the editor. Holding shift for selecting everything in between. Holding ctrl for toggeling individual rows. E.g. in kit editor: designs, types, qualities files and authors can be selected

- The > between KITNAME > KITVERSION should show a + item to create a new version
- Kit rows should have + at the end of the name column to create new version
- Design rows should have + at the end of the name column to create new variant
- Design variant rows should have + at the end of the name column to create new view
- Type rows should have + at the end of the name column to create new variant

- When clicking a row it doesnt select it (or doesnt show it with a primary background)
- The hover color over the rows over the table is too light and not the same as when hovering over e.g. navbar, toggles, etc
- When clicking on the sorting toggle then it doesnt toggle. Only from dropdown it works.
- The panel toggles group toggles always have a doubled border. Make sure that toggles and toggle groups have the same border and height but never double. For this purpose move all toggle code to toggle group item and then reexport toggle as toggle group with one toggle group item. Do the same for button and button group.

A new input ui element should be introduced: Action

- Action are small square icon-only bordered buttons that can be reused for building more complex ui elements. E.g. dropdown toggle should use an action, TreeItems should use actions. The + of the Rows for adding version, variant, view should use actions.

Further:

- The sort toggles dont have the same height as the other toggles. All toggles, buttons, input, etc should always have the same height. The sort toggles are missing a border somehow. A toggle is just a toggle group with one item and should never be used alone.

- Colors should never be used directly. E.g. primary, secondary, light, dark, etc. but instead only globals.css uses them to define semantic colors e.g. active, active-hover, hover, disabled, …
- Cursor pointer should never be set on elements on instance level but just on definition level.
- basic html elements such as <button … should never used directly outside elements. All elements export all memebers that are necessary to compose the ui.

- The actions are too big. They should be so small that they fit with the border and still have a border into toggles, tree items, rows, inputs, etc
- The sort toggles dont have the same height as the other toggles. All toggles, buttons, input, etc should always have the same height. The sort toggles are missing a border somehow. A toggle is just a toggle group with one item and should never be used alone.

- The + actions on the rows of the table should be proper actions and right aligned to the column.
- The + and - on the tree items inside of panels should all be proper actions.
- The hover and selection of the rows is not working

- Clicking the sort toggle doesnt toggle. Only over the dropdown it works.
- The home editor should also have same actions, toggles, etc. The children should work the same as type with name and variant but instead kit name and version. E.g. separate default rows should dissappear.
- When clicking on a row it should add it the selection. The selection should have primary background
- New Variant and Views shouldnt be numbers but work analogous to the navbar with New Variant, New Variant 2 if already taken, etc. This is language specific, Unify the code to be consistent.

- Row selection doesnt work in home editor and kit editor
- Rows should only navigate to the item when hovering over the text of the name. The rest should select it.
- Design rows have two action + for new variant and + for new view (which creates a new view for the default variant)
- The home editor style is not consistent with the kit editor. Make sure to apply all alignment and elements to kit editor. E.g. right align of +. Or search toggle style.

- Columns in table should be

- Navigating up should

- Design editor should not only have a selection but also hover effects. Either a piece or a connection or a port can be hovered over. The hover effect should show everywhere. E.g. workbench avatars, diagram nodes, 3d mesh material

- Use the same solid colors (and not just borders) for pieces hovers and selection (=active) as for toggles, breadcrumb, etc

- Dragging a piece doesnt work. You can check the old implementation for reference.

- Pressing Up in navigation should go up in the navigation of the navbar and not just to the kit. E.g. HOME > TEMPORARY > KITNAME > VERSION > DESIGNS > DESIGNNAME > DESIGNVARIANT > DESIGNVIEW would need 7 ups

- Implement the search in the navbar to look for kits, designs, types, qualities. Use the shallow type for it. Implement it with Fuse.js

- Dragging a piece doesnt work. It may be that the current transaction mechanism is not yet working properly. The store should work like this:
  Every editor has two internal stacks of Edits (do, undo) where EditorStep (diff, selection) is changing the editor for one undo/redoable step. After every command one edit is triggered. The first stack is for the edits within the current transaction, the second stack is for past completed transactions. When undo/redo are pressed during a transaction, they modify the current transaction stack. When a transaction is finalized then all the edits are merged to a big edit and pushed onto the past transactions. When a transaction is aborted, all current edits are merged and the merged edit is reverted. The on step saves the diff from the command and the new selection. The undo step saves the inverted diff along with the old selection. Both editor and kit commands contribute to the two stacks when there is an active design editor. Every editor store exposes one (computed) kit diff which is the the merged kit diff of the current transaction. The editor is using this diff to display it.

- Whenever something is part of the current kit diff then all the colors are mixed. E.g. selected is then selected-changed (50% selected, 50% warning), selected-removed (50% selected, 50% danger), selected-added (50% selected, 50% success). This way whatever happens within a transaction is visible.

- The details panel toggle has a double left border
- The home editor is displaying default kit as a child of kit but instead the parent should be the default kit it it exists. Same as kit editor with types.
- A new kit version should be called New Version or New Version 2 if already taken etc

- The hover shouldnt take border away of the nodes in the diagram. The hover color is not the same of the workbench avatar. Same goes for the mesh material which is not the hover color.

- The top and bottom spacing of the panels is too tight and not the same spacing as e.g. the items in the navbar have towards the

- The workbench doesnt remember which sections and tree items were open. E.g. after a drag and drop of an avatar the tree items are always collapsed again
- The workbench should have + actions for creating new designs, types, variants and versions on the sections and tree items
- Same goes for the other panels

- No, make new general hooks useDesignEditorIsPieceHovered [depends on if the piece is hovered], useDesignEditorIsPieceTransitiveHovered [depends on if the piece is hovered or the type of the piece is hovered or the design of the piece is hovered], usePieceStatus [depends on kit diff], useDesignEditorPieceColor [depends on status and transitive hover], useIsTypeHovered [depends on if the type is hovered], useIsTypeTransitiveHovered [depends on if the type or a piece of that is hovered], useTypeStatus [depends on kit diff], useTypeColor. etc

- Every clickable element which performs a callback (such as button, toggle, breadcrumb, …) should show a clickable cursor
- Every draggable element (such as sortable tree items, …) should show a draggable and a dragging cursor

- Introduce a new hook which returns

- The hover currently is either piece, connection, port, type, design. Everything should be pluralized. E.g. hovering over the tree item of a type name should set the hover to all types with that name (which are all the avatars in that list). At the same time all the pieces of those types are by transitivity also highlighted in the diagram and scene.

- The hover currently is either pieces, connections, ports, types, designs. Everything should be composable and not exclusive. Currently there is no mixed kind ui element but it will soon come. For now e.g. hovering over the tree item of types should set the hover to all types with that name (which are all the avatars in that list). At the same time all the pieces of those types are by transitivity also highlighted in the diagram and scene.

- All actions should have the same color as the context. E.g. the + of tree section label should be the same gray, tree item + on e.g. design editor details section should be same foreground, The dropdown toggle action the same as the icon in the toggle, etc
- The toolbar panel toggle should be renamed and generalized to tools. The tools toggle is responsible for toggeling all tools (e.g. toolbar but also when in full screen and all the tool elements such as e.g. in design editor: in diagram the minimap, the fit controlors, in scene gizmo)

- The design editor should have to following tools: selection (normal, additive (when holding shift), subtractive (when holding ctrl)), lasso (rectangular, freeform).
- Every editor can register tools and the tools should automatically appear in the toolbar. The width of the toolbar should fit the width of the tools. It should be in the middle and grow to the sides.
- The toolpanel toggle should be tools toggle. It currently works but is not showing the active state like the the other toggles.

- The panels should have the same spacing towards the top (navbar) and bottom (footer) as to the left and right. Currently top and bottom are touching.
- The avatar border for types and designs (in workbench in design editor) is not properly bordering but the background is larger than the border.
- By default all tree sections should be uncollapsed
- The nodes in diagram have a solid fill but only the border should be visible on default state (unselected/unhovered)

- The files have changed. Update the paths in the docs.

- The transitive hover for pieces with types works. But when hovering over a design piece e.g. node in diagram then it doesnt highlight the design avatar in the workbench

- The store should be split up into smaller files. The store

- Make sure that every editor has their own tools, their own active tool state, etc
- The toolbar should use a toggle group with toggles for single mode tools (such port creator) and dropdown toggles for multimode tools (such as selection)

- Sketchpad is currently only for editing kits. It should be expanded to include the docs.

- Tools should be generalized composable components. Every tool can have different modes. A tool with one mode is rendedered as a simple toggle. A tool with multiple modes as dropdown toggle. The registration, rendering etc should all happen automatically that no ui/core logic is duplicated and only tool specific code and information are props of the tool.
- The toolbar should have the same height as the navbar and be resizable but only to multiples of this height. The spacing top to bottom and between the lines should all be equal. The toolbar panel fits the width to around the tools.

- The breadcrumb navigation by default works in single line mode. When

- Hovering and selecting ports should be possible either over the tree item in the details or in the scene. (Similar to how pieces are hovered but without the transitive part in design editor)

- removing ports over the details in type editor doesnt work

- Design editor: Make sure that when selection tool is active and shift is holded it switches to additive mode and when shift is no longer holded it goes back to normal mode. Same for ctrl with subtractive mode.
- Leave cursor for normal selection and replace additive with + and subtractive with - icon
- Introduce the same selection mechanism from design editor to type editor (with modes, ctrl and shift mechanism, icons, etc)

- The hover and selection are on piece nodes in diagram is not the full circle but just a tiny part in the middle. The rest is unreactive.
- Hovering over Types tree section should transitive hover over all types. Same for Designs tree section.
- The border of type and design avatar in workbench in design editor is is smaller than the color. E.g. active bg goes beyong the border of the avatars.

- The tools in the toolbar are cut off by the footer. They should have the same 1 unit spacing.

- Toggle and button group should automatically have a vertical border between the items (e.g. navigation buttons have no border between.)
- Breadcrumb should automatically have a vertical border between things. E.g. navbar navigation has just items and chevron but they should all be separated by vertical borders.

- Cleanup all temporary console logs
- Cleanup all comments
- Search for missing i18n keys and translations

- A dropdown toggle should never show Select but instead always have one option selected (either explicitly provided or otherwise the first one)

- When additive select tool is active in design editor, then when new pieces are clicked they should be added to the selection, Analogous for subtractive. Currently it just acts as normal select.

- Select should be default tool in type editor

- New ports created with the port tool should

Details:
type editor:

- Adding and removing attributes doesnt work
- Updating port properties doesnt work

type editor:

- Adding more than one attribute doesnt work

- Clicking on the toggle (not the dropdown) on a toolbar should set the active tool to that value. Currently only setting over dropdown works.

- The panel logic should be generalized to a composable panel component. Make sure to extract all shared logic and refactor all individual panels.

- The canvas with with windows logic should be generalized to a composable canvas and window component where a canvas has window children (e.g. a window can be fullscreen). Make sure to extract all shared logic and refactor all individual windows (tables, scenes, diagram, …) and place them into the canvas.

- The details panel of type editor is not deep subscribed and hence doesnt update properly e.g. when representation, ports, etc are updating.

- Generalize Store, EditorStore (abstract) and KitDiffEditorStore (e.g. kit editor, design edito, type editor)
  Store holds data for any component.
  EditorStore holds data for any editor. Every editor has transaction support with undo/redo (two stacks: one for current transaction which is merged once it is finsished; one for previous finalized transactions). Every edit is always diff and inverted diff to enable both way undo/redo.
  A KitDiffEditorStore edit has a kit diff along with editor specific diff.

- Add transform controls to selected pieces (set the plane after a transform). Make sure to consider that the threejs coordinate system is not equal to the semio coordinate system.

- Double click on avatars in design editor workbench should navigate to the editors. Double click on Tree items should navigate to the kit editor with the correct parameters.

- Editing point and direction in type editor details doesnt work

Make sure to complete the store and adjust the y store and the hook implementations:
The design editor has two internal stacks of DesignEditorEdit (do:DesignEditorStep, undo:DesignEditorStep) where DesignEditorStep (diff:KitDiff, selection:DesignEditorSelection) is changing the design editor for one undo/redoable step. After every command one edit is triggered. The first stack is for the edits within the current transaction, the second stack is for past completed transactions. When undo/redo are pressed during a transaction, they modify the current transaction stack. When a transaction is finalized then all the edits are merged to a big edit and pushed onto the past transactions. When a transaction is aborted, all current edits are merged and the merged edit is reverted. The on step saves the diff from the command and the new selection. The undo step saves the inverted diff along with the old selection. Both design editor and kit commands contribute to the two stacks when there is an active design editor.

Somehow setIsMobile and syncNavigation are called too often.
setIsMobile should be called when the width changes.
syncNavigation should be called on navigation events.
E.g. When I click create kit action in home then I get:
store.tsx:1162 Executing (special) command: "semio.sketchpad.createKit"
store.tsx:1195 Executing command: "semio.sketchpad.setIsMobile"
store.tsx:1195 Executing command: "semio.sketchpad.setIsMobile"
store.tsx:1195 Executing command: "semio.sketchpad.setIsMobile"
store.tsx:1195 Executing command: "semio.sketchpad.syncNavigation"
store.tsx:1195 Executing command: "semio.sketchpad.setIsMobile"
store.tsx:1195 Executing command: "semio.sketchpad.setIsMobile"
store.tsx:1195 Executing command: "semio.sketchpad.syncNavigation"
setIsMobile shouldnt be there.
syncNavigation should only be called once.
