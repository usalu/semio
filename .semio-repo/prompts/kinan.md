Sketchpad Design app seletion Create selection tests within the existing test structure. All new tests must pass without breaking any current functionality.

Requirements

Cross-scene synchronization
Any element (Piece, Port, Connection, 3D Model, etc.) can be selected in either the Window scene or the Diagram scene, in any order. Selection state must remain perfectly synchronized between both scenes.

Default selection behavior (no mode active)
If no additive, subtractive, or intersect mode is enabled, selecting a new element replaces the current selection.

Canvas click deselection
If an element is selected and the user clicks on an empty canvas area (not on an element), the selection is cleared.

Additive mode
When additive mode is enabled, newly selected elements are added to the current selection without removing existing selections.

Subtractive mode
When subtractive mode is enabled, selecting an already-selected element removes it from the current selection.

Rectangular (box) selection

All elements within the selection box become selected.

If no additive/subtractive/intersect mode is active, a new box selection replaces the existing selection.


--------------------------------------------------------------------------------------------------------------------------------


Create a new ui element `Ring` for editing parameters t on a circle. Add stor



```yaml
Piece: # section,
  Type: "{{piece-type-select}}" # input tree item, only show types that can replaced the type (e.g. all used connectors must exist)
  Id: "{{piece-id-input}}" # input tree item
  Description: "{{piece-description-text-area}}" # input tree item
  Attributes:
    - name: "{{attribute-name-input}}" # input tree item
      value: "{{attribute-value-input}}" # input tree item
  Plane: # collection tree item, only show section when
    Origin: # collection tree item
      X: "{{origin-x-stepper}}" # input tree item
      Y: "{{origin-y-stepper}}" # input tree item
      Z: "{{origin-z-stepper}}" # input tree item
    X-Axis:
      X: "{{x-axis-x-stepper}}"
      Y: "{{x-axis-y-stepper}}"
      Z: "{{x-axis-z-stepper}}"
    Y-Axis:
      X: "{{y-axis-x-stepper}}"
      Y: "{{y-axis-y-stepper}}"
      Z: "{{y-axis-z-stepper}}"
Parent Connection:
  Translation:
    Gap: "{{gap-slider}}"
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"
```

-----------------------------------------------------------------------------------------------

Sketchpad Kit app
Representations is being rendered twice on the kit app. once as a file and a folder. anaylze this mistake and fix. always make sure to match the zip metabolism folder-file structure. add this to the excising test structure when finished 
-------------------------------------------------------------
Sketchpad
Correct in detail panel from slider to 

    Translation:
      Gap: "{{Gap-stepper}}" 
      Shift: "{{Gap-stepper}}"
      Rise: "{{Gap-stepper}}"


## Prompt: Duplicate Type Visibility Without App Switch

Fix and validate Sketchpad Design Workbench duplicate behavior.

Context:
- Scope only `semio/js/sketchpad/Design.tsx` and existing tests in `semio/js/sketchpad.test.ts`.
- In Workbench Types, duplicate action id is `semio.sketchpad.common.duplicateType`.
- Do not create new test files.
- Do not switch to Type app after duplication.
- Keep Workbench action renamed to `Duplicate Type` and keep it visually distinct from add-piece (different icon).

Task:
1. In Design Workbench, pressing `Duplicate Type` creates exactly one child type under the clicked parent type.
2. The duplicated child row becomes visible in Workbench immediately.
3. Current URL stays on the same Design route and never navigates to `/types/...`.
4. Existing behavior remains intact (drag/drop creation, plus-add-piece, double-click navigation).

Test requirements (existing `sketchpad.test.ts` only):
1. Capture current Design URL before duplicate click.
2. Click `semio.sketchpad.common.duplicateType` on a valid parent type row.
3. Assert child count for that parent increases by exactly `+1`.
4. Assert exactly one new child type guid appears.
5. Assert URL equals pre-click URL and does not contain `/types/`.
6. Assert new duplicated type name is visible in Workbench.

Acceptance criteria:
- Duplicate creates one visible child type in Workbench.
- No navigation to Type app occurs.
- Assertions are implemented in existing Design e2e flow.
- Report exact commands and real test outcomes.

------------------------------------------------------------------------------
when adding piece in the diagram window, it should appear in the the scene window with the correct model attached to the type
------------------------------------------------------------------------------

Validate that Workbench type piece creation is fully implemented and working in Sketchpad, using the existing test structure only.

Context:
- Repo: semio monorepo
- Relevant files:
  - semio/js/sketchpad/Design.tsx
  - semio/js/sketchpad.test.ts
- Existing expected behavior:
  - A piece can be created from Workbench Types by drag-and-drop.
  - A piece can also be created by clicking the `+` action on each type row.
- Do not create new test files.
- Extend/refactor only existing test files and existing implementation files as needed.

Goals:
1. Confirm implementation is correct for both creation paths:
   - Type row `+` adds a piece to active design.
   - Dragging a type avatar from Workbench to diagram adds a piece.
2. Ensure behavior is covered in existing test suite structure.
3. Run tests and verify runtime behavior before claiming success.

Required workflow:
1. Inspect current Workbench implementation in `Design.tsx`:
   - Find `TypeTreeItem` actions and creation handlers.
   - Verify `+` action calls add-piece flow (not only create-child flow).
   - Verify drag-drop flow still adds piece via drop handling.
2. Inspect `semio/js/sketchpad.test.ts`:
   - Locate existing Design app left panel/workbench tests.
   - Add/adjust tests in-place to cover both creation paths in one coherent unit flow.
3. Implement missing behavior if needed:
   - Keep existing functionality (create child, double-click navigation, drag/drop).
   - Do not remove features to make tests pass.
4. Add robust assertions:
   - Capture piece/node count before action.
   - Perform action.
   - Assert count increases by exactly 1.
   - Do this for both `+` and drag-drop.
5. Run relevant tests and report exact results:
   - Include command used.
   - Include pass/fail summary and failing test names if any.
   - If flaky, document exact failure and rerun evidence.
6. Provide concise final report:
   - What was broken/missing.
   - What changed (files + key functions/sections).
   - Test evidence that both creation paths work.

Test constraints:
- Use existing test framework and conventions in `sketchpad.test.ts`.
- No new test files.
- Keep selectors resilient and aligned with current UI ids/data attributes.
- If temporary logs are needed, prefix with `[DEBUG]` and remove them before finalizing.

Acceptance criteria:
- Clicking Workbench type row `+` creates one new piece in active design.
- Dragging a Workbench type into diagram creates one new piece.
- Both are validated in existing test file structure.
- Test run executed and reported with real results (no assumptions).

Deliverables:
- Updated implementation (if required).
- Updated `semio/js/sketchpad.test.ts` with coverage for both flows.
- Final summary with:
  - changed files
  - what each change does
  - exact test command(s)
  - exact test outcome.


Multiple connections 



```yaml

Multiple Connections: # section
  Plane: # collection tree item
    Translation:
      Gap: "{{gap-slider}}" # applied to all selected connections (supports mixed values)
      Shift: "{{shift-slider}}"
      Rise: "{{rise-slider}}"
    Orientation:
      Rotation: "{{rotation-slider}}"
      Turn: "{{turn-slider}}"
      Tilt: "{{tilt-slider}}"

  Diagram:
    X Offset: "{{diagram-x-offset-stepper}}" # applied to all selected connections
    Y Offset: "{{diagram-y-offset-stepper}}"


```





Multiple Pieces: # section


```yaml
  Type: "{{piece-type-select}}" # input tree item, shows shared type or mixed state; only valid replacement types allowed
  Variant: "{{piece-variant-select}}" # input tree item, may show mixed values
  Id: "{{piece-id-input}}" # input tree item, may represent multiple ids (mixed)
  Description: "{{piece-description-text-area}}" # input tree item, may show mixed values
  Attributes:
    - name: "{{attribute-name-input}}" # input tree item, applied to all selected pieces
      value: "{{attribute-value-input}}" # input tree item
  Plane: # collection tree item, only show section when applicable to all selected pieces
    Origin: # collection tree item
      X: "{{origin-x-stepper}}" # input tree item, supports multi-edit
      Y: "{{origin-y-stepper}}" # input tree item
      Z: "{{origin-z-stepper}}" # input tree item
    X-Axis:
      X: "{{x-axis-x-stepper}}"
      Y: "{{x-axis-y-stepper}}"
      Z: "{{x-axis-z-stepper}}"
    Y-Axis:
      X: "{{y-axis-x-stepper}}"
      Y: "{{y-axis-y-stepper}}"
      Z: "{{y-axis-z-stepper}}"

Parent Connections: # section (editing multiple connections simultaneously)
  Translation:
    Gap: "{{gap-slider}}" # applied to all selected parent connections
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"

Diagram:
  X Offset: "{{diagram-x-offset-stepper}}"
  Y Offset: "{{diagram-y-offset-stepper}}"

```
-----------------------------------------------------------------

Drag and drop from workbench to the diagram window 


Currently Type cant be dragged from workbench window toward the diagram window. 

By dragging and dropping from workbench to diagram a new type should be created


-----------------------------------------------------------------------

when connection is selected 

```yaml

connecting:
  Translation:
    piece id: "{{gap-slider}}"
    port id: "{{shift-slider}}"

connected:
  Translation:
    piece id: "{{gap-slider}}"
    port id: "{{shift-slider}}"

Plane:
  Translation:
    Gap: "{{gap-slider}}"
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"

Diagram:
  Gap: "{{gap-slider}}"
  Shift: "{{shift-slider}}"
  Rise: "{{rise-slider}}"
  X Offset: "{{origin-x-stepper}}"
  Y Offset: "{{origin-y-stepper}}"
```

Parent Connection:
  Translation:
    Gap: "{{gap-slider}}"
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"
```
----------------------------------------------------



Sketchpad Design app

When piece is selected tab crashes




```yaml
Piece: # section,
  Type: "{{piece-type-select}}" # input tree item, only show types that can replaced the type (e.g. all used connectors must exist)
  Id: "{{piece-id-input}}" # input tree item
  Description: "{{piece-description-text-area}}" # input tree item
  Attributes:
    - name: "{{attribute-name-input}}" # input tree item
      value: "{{attribute-value-input}}" # input tree item
  Plane: # collection tree item, only show section when
    Origin: # collection tree item
      X: "{{origin-x-stepper}}" # input tree item
      Y: "{{origin-y-stepper}}" # input tree item
      Z: "{{origin-z-stepper}}" # input tree item
    X-Axis:
      X: "{{x-axis-x-stepper}}"
      Y: "{{x-axis-y-stepper}}"
      Z: "{{x-axis-z-stepper}}"
    Y-Axis:
      X: "{{y-axis-x-stepper}}"
      Y: "{{y-axis-y-stepper}}"
      Z: "{{y-axis-z-stepper}}"
Parent Connection:
  Translation:
    Gap: "{{gap-slider}}"
    Shift: "{{shift-slider}}"
    Rise: "{{rise-slider}}"
  Orientation:
    Rotation: "{{rotation-slider}}"
    Inversion: "{{inversion-slider}}"
```




 sketchpad design app : Add End-to-End Coverage for Type Piece Creation (Plus + Drag/Drop) 
Extend existing sketchpad tests (do not create new test files) to cover both Workbench type-piece creation paths.
Scope:


Prompt A: Enable Plus-Button Piece Creation in Workbench Types
Implement Workbench Types so each type row + creates a new piece in the active design (same outcome as drag-and-drop).
Scope:

Update the TypeTreeItem row action in Design.tsx so primary + adds a piece using that type.
Keep drag-and-drop behavior unchanged.
Keep “create child type” available as a separate action (not removed).
Place created piece at a deterministic default location if no cursor/drop point is available.
Ensure transaction handling and selection/focus behavior are consistent with existing add-piece flows.
Acceptance:
Clicking + on a type increases piece count by 1 in current design.
Drag-and-drop still creates pieces.
No regression in Workbench navigation/double-click behavior.
Prompt B: Add End-to-End Coverage for Type Piece Creation (Plus + Drag/Drop)
 


Fix the Kit table so both Folders and Files are rendered only when they physically exist in the imported metabolism zip payload.

Bug:
- The table shows extra folder/file elements that come from metadata but do not exist as actual zip entries.

Required behavior:
- Folder rows: show only folder paths that exist in the zip entry tree.
- File rows: show only files that exist in the zip entry tree.
- No metadata-only, inferred, or remote-only folder/file rows.
- If a folder has no real children from zip entries, it must not appear.
- Keep normal table behavior (expand/collapse, selection, sorting, filtering) intact.

Scope:
- Investigate `importKit` result and where zip entry paths are stored.
- Update `Kit.tsx` row generation (`buildFileTree` / `flattenFileTree` usage) to use existence from imported zip file map.
- Implement in existing files only.
- Do not alter metabolism sample data to hide the bug.

Tests (extend existing test files only):
1. Existing zip folders/files are shown.
2. Metadata-only folders/files are hidden.
3. Parent folder visibility depends on real zip descendants.
4. Expand/collapse/sort/filter/selection still works with filtered folder/file data.

Acceptance criteria:
- In metabolism, no extra folder or file element appears unless it exists in the zip content tree.
- All relevant tests pass.
- Provide concise root-cause and fix summary.

Extend/Change/Refactor whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out. Create as many tickets as needed.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to track everything (plan, todos, changes, summary, etc) 

-------------------------------------------------------

 Sketchpad Kit app
 Currently the table in kit app isnt pulling the correct kit data. Files with 
 
 
 
 currently when a piece is selected in design app piece section appears in the details panel. task : piece and parent connection should be showing in this tree structure

├─ Piece
│ ├─ ID: <piece_id>
│ ├─ Type: <piece_type>
│ └─ Variant: <variant>
│
└─ Parent Connection
├─ Connecting
│ ├─ Piece ID: <connecting_piece_id>
│ └─ Port ID: <connecting_port_id>
│
├─ Connected
│ ├─ Piece ID: <connected_piece_id>
│ └─ Port ID: <connected_port_id>
│
├─ Plane
│ ├─ Translation
│ │ ├─ Gap: <number>
│ │ ├─ Shift: <number>
│ │ └─ Rise: <number>
│ │
│ └─ Orientation
│ ├─ Rotation: <degrees>
│ ├─ Turn: <degrees>
│ └─ Tilt: <degrees>
│
└─ Diagram
├─ X Offset: <number>
└─ Y Offset: <number>

analyze old build and get the tree structure in detail panel to work as in the old build











├─ Piece
│  ├─ ID: <piece_id>
│  ├─ Type: <piece_type>
│  └─ Variant: <variant>
│
└─ Parent Connection
   ├─ Connecting
   │  ├─ Piece ID: <connecting_piece_id>
   │  └─ Port ID: <connecting_port_id>
   │
   ├─ Connected
   │  ├─ Piece ID: <connected_piece_id>
   │  └─ Port ID: <connected_port_id>
   │
   ├─ Plane
   │  ├─ Translation
   │  │  ├─ Gap: <number>
   │  │  ├─ Shift: <number>
   │  │  └─ Rise: <number>
   │  │
   │  └─ Orientation
   │     ├─ Rotation: <degrees>
   │     ├─ Turn: <degrees>
   │     └─ Tilt: <degrees>
   │
   └─ Diagram
      ├─ X Offset: <number>
      └─ Y Offset: <number>

Implement filter in the type app similar to the design app but with different elements according to the Type app. e.g. Filter connector  


Scan the repo first.

Review the selected ../sketchpad files and identify all exported UI components. Compare them against existing Storybook stories in ../sketchpad/stories/** and ../sketchpad/panels/**.

For any component without Storybook coverage:

Add it to an existing relevant story file

If no direct match exists, extend the closest existing story file

Do not create new files

Follow the current Storybook patterns and typings

Keep changes minimal and consistent with the existing structure

Verify .storybook/main.ts includes all sketchpad story locations. Update globs only if something is clearly missing.

Output:

Components added

Files modified

Any intentionally skipped components (with reason)

No new files. Only update the existing structure.


sketchpad toolbar:
tool option buttons/toggles have inconsistent spacing between icon and name. e.g. kit app tool option create buttons are too narrow.

sketchpad Toolbar :
toolbar has currently diffeent elements.. toggles, drop down, commans and they are render slightly in different sizes. i want to create a mechanism to unify the spacing without hardcoding. e.g in filter -- settings toolbar has designs and it renderes in a different size to create Designs. i want to unify systematically. create a plan first 




Sketchpad toolbar

extend the funtionality mechanism of the toolbar to have subtools for eachtool. for the settings toolbar i want to be able to have filter, command and tools. create a prompt to 

sketchpad toolbar kit app:
same select tool option as design app


sketchpad toolbar kit app
the setting toolbar for select should be implemented the same way as in the design app with these categories: 

65fb3ff445f7fcfe870cebdcd2c40bcbf30bdc34

sketchpad design app
freezing ocurs while selecting elements in the scene window 

sketchpad toolbar:
each tool setting category should have a divider between. Currently design app has it and kit app not. This shouldnt be possible.


---------------------------------
sketchpad toolbar kit app:
same select tool option as design app


sketchpad toolbar kit app
the setting toolbar for select should be implemented the same way as in the design app with these categories: 


Category 1: Selection Mode
– Additive
– Subtractive
– Intersect

Category 2: Selection Shape
– Rectangular
– Lasso

Category 3: Navigation
– Hand

---




sketchpad toolbar:
Tool setting bar should have groups.
e.g. 

design 


t appears there is a duplicate panel. A panel similar to the Toggle Right panel is showing at a narrower width on the right side of the screen while the Toggle Right panel itself is inactive. Please verify that there are only three toggle panels in total: Left, Right, and HUD.


The Hub panel’s left and right panels as well as HUD PANEL are missing the tree elements, and their icons are not displaying. Please double-check for any extra or duplicate panels and ensure everything is properly integrated. During the migration from the old website new panels were mistakenly created instead of integrating with the existing one

Fix the bottom toolbar so it stops jumping when the settings panel width changes. The center point must stay locked to the middle of the screen. The tool bar should only grow to the left, the settings bar should only grow to the right, and a small constant gap between them must always remain. Update the layout so the center seam is anchored independently of either bar’s width, ensuring size changes never recenter or shift both toolbars from two sides, only a single side

In the Select tool’s settings toolbar, organize into distinct categories separated by visual dividers

Category 1: Selection Mode
– Additive
– Subtractive
– Intersect

Category 2: Selection Shape
– Rectangular
– Lasso

Category 3: Navigation
– Hand



sketchpad toolbar:
hand is currently a subtool from select. it should be seperate to the left of  select tool 

sketchpad toolbar:
the select tools should have 3 subtools: normal, lasso.

sketchpad toolbar:
aditive, substractive, intersect are not subtools. but they should be instead setting for the selection tools

 rendered in the tools setting bars while select is activated 


the select drop down menu has following selection sub-tools ( )

sketchpad kit app toolbar:
Currently 1 select, 


add five selection sub-tools for the select. it should be a single column with 5 rows for each subtool


sketchpad toolbar:



drop down toggle (select) in the toolbar, currently when clicked on its arrow it should open an single column (list) upward and align to it, and exactly the same width as the button (and visually matching its height/shape style). 

The dropdown column contains a compact list/table of Selection sub-tools (one row per sub-tool). Each row has a clear hover state (background highlight + optional icon emphasis) and is clickable to select that sub-tool. 

The dropdown must feel anchored to the button (same left/right edges), open instantly with no animation, and close when a sub-tool is selected or when clicking outside. The selected sub-tool should be indicated (e.g., checkmark or active row highlight)




When running `npx playwright test sketchpad/toolbar.spec.ts` over the cli it works but when trying to debug the test over vscode I get:
Error: browserType.connect: Target page, context or browser has been closed
Browser logs:


╔════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Looks like you launched a headed browser without having a XServer running.                     ║
║ Set either 'headless: true' or use 'xvfb-run <your-playwright-app>' before running Playwright. ║
║                                                                                                ║
║ <3 Playwright Team

You’re working in repo on a new Design editor build.
The old build (.old files) has working logic that’s missing or incomplete in the new one

Please migrate behavior, not UI, from the old build into the new architecture.

Focus on:

Details panel edits: make type, variant, and design fields editable again (single + multi-select), and re-implement “fix selected pieces” using the new store/command patterns.

Diagram actions: implement cluster and expand/explode as real commands (undo/redo must work).

Diagram correctness: ensure port/handle mapping and connection rules match the old behavior, especially for design pieces and external connections.

Follow the new build’s patterns (commands, transactions, hooks).
Don’t copy old layout code — only logic and semantics.
Work in small, clean diffs and keep behavior consistent with the old editor

fix these in the toolbar 

fix these : 
- Seperate both Tool Bar and setting toolbar visually using two sperate boarders 
- Make a tiny visual gap between the toolbar and the setting tool bar from the middle to the beginning of each boarder.
- Make sure all buttons are rendered and visible in exactly the space they need to be visible. currently under Create tool setting bar the Button are not clearly visible, where in filter all buttons are visible but they dont all fit whithin the Toolbar setting boarders 
- make sure the boarders of both toolbar and tool setting bar are created based on the grown space out of all buttons or drop down menus 


- all button and drop down toggle should visible be a single word with an icon in both the toolbar and the tool setting bar.  Currently seelction tool is semio,sketpad.toolbar.subtool. It should be selection+ the cion
-Any tool that has sub selection tool should be implemented as a drop down button and rendered vertically from the exact same position as the dropdown button and in the same size.



-extend the boarder of the setting tool bar to contain all the toggles. 
- make sure to not include an extra field e.g. in kit app there is a filter field where user can type. i dont need any typing fields in the toolbar

Level 2 :


- Both toolbar and the tool setting bar should grow from the middle of the page. Toolbar middle to left and Tool settings bar middle to right. Not full width only as much as it needs.
- The tool settings bar items are inconsistent. Use named toggles/buttons for all of them. Leave the title such as "Tool Settings Bar" and group names e.g. "Create" out and just have toggles or buttons.



Kit Editor:
- ~~Slection in diagram windows doesnt work. i am only able to select through the table window ~~
-   





Design App:

~~Implement colorization of port family ~~

Smarter logic for the diagram 

~~Fix selections~~ 



~~create a a color strategy for ports to inhance the user experience.. copatable ports, diferent port types,...  etc. Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to add the plan.md, to track everything (todos, changes, summary, etc) in ticket.md~~


Type App 

Preset Port modis 





Windows spesific toolbar that follows the same logic but is not floating 



I want to redesign the existing global toolbar used across all applications while keeping it fixed at the bottom center of the screen, exactly where it is today. The new system consists of two distinct but synchronized toolbars aligned on the same horizontal axis.

The left toolbar is the primary tool strip. It is anchored at the bottom center and expands leftward as tools are added, while tools within the strip are ordered from left to right. Only one tool can be active at a time. This tool system is hierarchical and built as a tree structure. When a tool is selected, its sub-tools expand vertically upward from that tool. When a sub-tool is selected and contains further categories, sub-sub-tools expand horizontally to the right of the selected sub-tool.

The right toolbar is a contextual tool settings bar that displays tool settings for the currently selected tool, sub-tool, or sub-sub-tool. This bar expands from left to right, starting near the center of the screen and extending to the far right edge. The content of the tool settings bar must always stay fully synchronized with the current selection state, updating immediately whenever the selected tool level changes.

The interaction model must ensure that only one node in the tool hierarchy is active at any time and that all tool settings shown are driven solely by that active selection. The overall design should clearly express hierarchy through spatial layout (vertical for depth, horizontal for categories) without relying on animation.

Use the provided sketch as a structural reference and deconstruct it alongside this description to fully understand the intended layout, hierarchy, and interaction model. Generate a professional, extremely detailed prompt that precisely describes this toolbar system, its behavior, and its design intent.  





create a plan to change the current visual look of the nodes and edges in the diagram window in kit editor. I want to change how nodes look and how they connect to each other through the current proximity connect 
each shape should have N points which functions as snapping point where edges connect to the nearest of these N points of each shape. A circle would have 4 points. A rectangle 4 as well one on the middle of each edge. try two other shapes. i want to implement it as a strategie where later on new shapes will e defined in the future. For now Designs should be a circle, Types are rectangular, files are triangles with three points and all the rest would be long rectangle with four points as well 






Currently in the kit app when pressing the action                                                    
00000000000000000000000000000000000000000000000

# Observations

### Elements 
- Button action parent bug 
- Add action works for Design and Type only
- Design ToolTree

#### Table

#### Diagram

### Sketchpad

### Apps

#### Home  

#### Kit

- export Kit
- Import drag and Drop ? 

#### Design

- selection 
- tree toolbar
- Ports coloring and selection of connectors


# Prompts

## A

Seperating filter and selction tool + fixing 

# Finalized

Separate filter tools and selection tools into two distinct toolbar sections. Render them in separate horizontally-adjacent toolbar containers. Filter section shows all artifact kind toggles (currently all active by default—make them inactive by default, activate on click). Selection section should show tool kind buttons with icons. Both sections should use the same Toolbar component but be registered as separate sections with different specificity values to maintain left-to-right ordering

## B

## D


 Elements 
- Button action parent bug 
- Add action works for Design and Type only


## C

# Drafts

Currently as i open the kit app, all filters are active. Make filters all inactive by default and as i click on any of them it gets activated. Also the selection tools are missing the icons . 
Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to add the plan.md, to track everything (todos, changes, summary, etc) in ticket.md

render the selection tools in a seperate container (Add the toolbar by starting the element sharing. Output should be two seperate Toolbar frames next to each other 


Currently in the D3 Force Diagramm the Nodes and edges doesnt align visaully as they should be. The nodes circle seems to be much smaller than the circle where the edges which are connected to creating a gap between The nodes and the edges. Change/refactor/extend whatever is necessary to get it working. Even if it seems unrelated to you. The goal is clear.
Dont ask in between, no confirmations, no matter the issue. Figure it out.
Be sure that it works everywhere before stopping.
Make sure to open and close a ticket. Dont forget to add the plan.md, to track everything (todos, changes, summary, etc) in ticket.md


I want the diagram window to visualize the nodes of the elements that are only shwoing on the table.

The unfolded elements wouldnt show yet until they are folded. 
All different other categories like files, folders, types, design, etc .... 
It should be at the end a perfect syncronized tabale and diagram layout  






# Prompt history

allow multiple filters at once

Change D3 Force parameters 

introduce multiple select of multiple nodes through pressing shift or rectangular selection




Bugs:

Diagram automatically selects multiple nodes as if i was clicked on shift

The tables left and right arent aligned (Hierachie table doesnt match with Diagram window)


Nodes and edges arent aligned 






There is a mismatch between the avatar circle and the edges. The edges are offset on a larger circle making it appear as the node is larger.

Implement the D3 simulation to match the example.
Currently the rest of the nodes dont move while dragging a single node.
There is no simulation. There seems to be a fundamental state issue.
Analyze in depth what could be the core problem and fix everything.

The red selection should only heighligh the node outline like in the design app 

i want to fix the diagram 

previously : adapt the atlas by explaining all general concepts with concrete examples, workflow, programming langaues, package manager, programming styles, repo structure, use cases etc exclusively from semio. do it one section at a time
Now: I like the resault but combine.old with the new where you explain general concepts first with examples from semios programing languages, workflows, architecture, package manager etc.. Simplify the lanague for non dev to start learning while keeping all techical terms and comlexity. always comment code and its systemetic thinking by relating it to the main concept. this should be an intro to programming speciffically on semio as an example guide

Extract prompt strategies out to plans/prompt-strategy.md 
