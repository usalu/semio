

semio:
Define a copyPiecesAndConnectionsInDesign(design:Design, pieces:Guid[], connections:Guid[], anchor: ["byMiddle", "byCentroid", "byBottomLeftCorner", "byBottomRightCorner", "byTopLeftCorner", "byTopRightCorner"]):Design function that:

copy should copy all internal connected (=non-fixed) pieces,
copy all internal [both pieces are within the given pieces] connections (not partial [one piece is within the given pieces, the other is not] or external [none of the pieces are within the given pieces]),
copy but flatten all pieces that have the parent outside of the given pieces

cetroid should be calculated based on the bounding box of the pieces and connections being copied, and the offset should be applied to the new pieces in this logic 

look for conenctions that are outside of the selected pices. the selected piece which has the conenction should become fixed and the conenction should be deleted. the piece should get its new plane based on the anchor point . 

if the piece is fixed, the new plane should be normalized first and then the offset should be applied to the new plane so that the anchor point is placed at the same position as the original piece. 








if the piece is not fixed, 


and offset should be applied to the new plane 
so that their centroid is placed at the anchor point.

ddldlddddldld dddldlddddd


----------------------------------------------------------------------------------

analyze all different ui elements in the toolbar. i want all of them to be visually extrem consistent, font, sizing margins, Icon size,etc....) currently switching from create to filter feels like the elements have a complete different styling. i assume it has to do wit hthe fact that one tool group is command and the other is not, but i want to make sure that all of the elements in the toolbar have the same visual styling and consistency regardless of their function or grouping. please analyze the current toolbar elements and identify any inconsistencies in their visual design, such as differences in font, sizing, margins, icon size, or any other styling aspects. then, refactor the toolbar code to ensure that all elements follow a unified design system, making them visually cohesive and consistent across the entire toolbar. make sure that these changes doesnt change any other ui elements outside the toolbar and that it only affects the visual styling of the toolbar elements, without altering their functionality or behavior. also make sure to test the changes thoroughly to ensure that all toolbar elements are visually consistent and that there are no unintended side effects on the user experience. NO hardcoding 


sketchpad type app: gumball
---------------------------------------------------------------------

Fix the semio/sketchpad Details panel layout regression globally, including the Type app. The current implementation works perfectly in design app but in type app it is breaking the shared Details panel structure in nested sections such as connectors/point/direction, causing rows, controls, and tree guides to drift, overlap, or stop following the standard property row/value-column layout. the Details panel implementation must be consistent across all apps and all section types.

Refactor the shared Details panel tree/layout primitives so TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, and IndentationLines all resolve through the same global structure: tree header rows stay tree headers, property rows stay property rows, nested groups keep hierarchy, and all value-side controls stay within the same shared value-column bounds. Preserve PanelSection architecture, hierarchy, spacing rhythm, actions, and tree lines. Do not hardcode section names or field names. Apply the fix generically so nested connector content in the Type app follows the same stable Details panel structure as every other panel subtree. importaant is dont break the deign app structure only fix the type app structure to follow the same structure as design app by applaying global layout rules and not one off fixes.

------------------------------------------------------------

Extend the tree-path hover highlight so it also includes the terminal branch segment of the hovered lowest-level row. Right now the ancestor path highlights, but the final local connector for the hovered TreeRow / leaf-level row is missing. When a row is hovered, highlight the complete path: the vertical ancestor chain, the final vertical segment at the hovered depth, and the small horizontal branch/elbow segment that connects into the hovered row label. Keep the existing TreeContext, TreeSection, TreeItem, TreeRow, TreeContent, and IndentationLines behavior unchanged; this is only a completion fix for the active path rendering. Apply it generically to all leaf and non-leaf rows without hardcoding section names or field names.


Implement a generic tree-path hover highlight for the semio/sketchpad tree system. When any TreeSection, TreeItem, or TreeRow is hovered, highlight the full ancestor path in the tree gutter from that row up to the highest visible parent/root. Use the existing tree infrastructure — TreeContext, TreeContent, IndentationLines, and current guide rendering — without changing hierarchy, spacing, or layout behavior.

Behavior:
- on row hover, detect the hovered node’s ancestor chain
- highlight only the connector/indentation path that belongs to that chain
- keep the normal tree lines, but render the active path in a theme-aware brighter/darker tone within the existing color system depending on light/dark mode
- make the active path line 1.5x thicker than the default guide stroke
- apply this generically across the whole tree, including nested groups and sortable items
- clear the highlight on hover end
- do not hardcode section names or field names
- do not introduce new UI elements; extend the current tree line rendering only

---------------------------------------------------------------


very last refractor happened to details panel have introduced a misalignment and different input fields.
goal is 


very last refactor happened to Details panel have introduced a misalignment and different input fields. goal is to restore one consistent property row / value-column layout across the entire right-side property inspector. The same value-side field widget must not change width, x-position, or box model depending on subtree depth, local TreeItem header structure, or nearby actions. Input, Textarea, Combobox, disabled/readOnly Input, Stepper, Slider value/control area, Toggle, and Button when used as a field control must all resolve through the same shared value-column sizing rules.

Fix the regression narrowly in the layout plumbing introduced by the last refactor: nested collection items, nested field rows, and flat sibling rows must use the same field-column start and end lines, and the same field sizing logic. Tree hierarchy, TreeSection, TreeItem, TreeRow, TreeContent, IndentationLines, SortableTreeItems, actions, spacing rhythm, and tree lines should remain intact. Do not hardcode section names or field names. The adaptation should only remove the unintended subtree-dependent field sizing and restore a single consistent Details panel field layout.

/////////////////////////////////////////////////////////////


pervious prompt > move chat and setting from the right panel as tabs and place them as seperate buttons in the navbar following the exact same ui icon as right and left panel toggels. place them to the right of right and left panel toggles. they should open the setting and the chat exactly in the same panel as the right panel but should be only accesable through the navbar. one of the three togggles could be enabled at the same time (chat,setting,or right toggle panel) all render at the exact same place and size on the screen when active. remove them as tabs from the right panel toggle so that they are only accesable from navbar 
currently > toggles buttons are able to be activated at the same time, and they are also accessible from the right panel toggle as tabs. the chat and setting should be only accessible from the navbar and not from the right panel toggle, and only one of the three toggles (chat, setting, or right panel) should be able to be active at the same time. when one of them is active, it should render in the same place and size on the screen as the current right panel. this will help to declutter the right panel and make it more focused on its content, while still providing easy access to chat and settings through the navbar.


////////////////////////////////////////////////////////////////////////////////
move chat and setting from the right panel as tabs and place them as seperate buttons in the navbar following the exact same ui icon as right and left panel toggels. place them to the right of right and left panel toggles. they should open the setting and the chat exactly in the same panel as the right panel but should be only accesable through the navbar. one of the three togggles could be enabled at the same time (chat,setting,or right toggle panel) all render at the exact same place and size on the screen when active. remove them as tabs from the right panel toggle so that they are only accesable from navbar 

Refactor the semio/sketchpad Details panel tree spacing and line layout in index2.tsx using the Ant Design Tree `showLine` + `switcherIcon` example as the visual reference for gutter rhythm, label-start spacing, and connector-line behavior — but keep my existing tree system and UI primitives.

Do not replace anything with Ant components. Reuse the current TreeContext, TreeSection, TreeItem, TreeContent, IndentationLines, SortableTreeItems, PanelSection architecture, and existing tree lines. This is a spacing/layout adaptation only.

Goal:
Make the tree read more like the Ant-style tree from the reference: cleaner switcher/label spacing, more stable label start positions, and clearer connector-line rhythm — while preserving my current Details panel structure, property layout, actions, and widgets.

Implementation guidance:
- keep hierarchy depth in the tree gutter
- keep connector lines in IndentationLines
- keep expand/collapse icons in a fixed switcher slot
- make the tree item label start position predictable and clean at each depth
- avoid double-applying depth offset through both the row container and TreeContent
- adapt spacing around the switcher/label/line area so expandable rows and non-expandable sibling rows of the same depth read as part of the same vertical rhythm

Specific refactor intent:
- review how detailPanelIndentPx(level) is used in TreeSection, TreeItem, and TreeContent
- stop the current layout from over-shifting labels because padding/indent is being applied in multiple places
- preserve indentationLinePx(i) and the existing guide rendering model, but tune the gutter/label spacing so the lines and labels feel closer to the Ant tree reference
- reserve a consistent switcher slot even when a row is not expanded/collapsible, so sibling labels at the same depth align correctly
- make property-level expandable headers in the Details panel align more naturally with nearby sibling rows at the same hierarchy level
- keep child rows clearly nested, but do not let chevron spacing create awkward label drift
- keep tree connector lines continuous and visually clean through nested groups

Constraints:
- do not flatten the hierarchy
- do not redesign the inspector
- do not introduce new UI elements
- do not hardcode section names or field names
- do not break the existing property row/value-column layout, actions, sortable behavior, or nested content rendering

This should be a narrow spatial refactor: adapt the tree gutter, switcher slot, label-start spacing, and connector-line rhythm to feel closer to the Ant `showLine` tree, while preserving everything else in the current build.




//////////////////////////////////////////////////////////////

Fix  Details panel alignment for expandable property-level `TreeItem`s. In the right-side property inspector, a property-group header like `Location` is at the same hierarchy level as sibling property rows, so its label must start on the same vertical label line as nearby non-expandable rows such as the surrounding field rows. Do not let the chevron/icon slot shift the whole header inward. Instead, keep the chevron inside a reserved control slot within the existing tree gutter / left label area, while preserving the same label-start alignment for sibling rows at that level. fix for all property-level expandable headers across all sections in the Details panel, without affecting the alignment of nested child rows, tree lines, or spacing elsewhere in the tree.

--------------------------------------------------
Keep the current PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, TreeContext, hierarchy, and property row/value-column layout. Do not flatten the tree and do not rework unrelated alignment behavior. This should be a narrow adaptation: fix property-level expandable headers so they align with sibling rows of the same depth, while preserving all existing nested child alignment, actions, tree lines, and spacing everywhere else.


Keep the current PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, TreeContext, hierarchy, and property row/value-column layout. Do not flatten the tree and do not rework unrelated alignment behavior. This should be a narrow adaptation: fix property-level expandable headers so they align with sibling rows of the same depth, while preserving all existing nested child alignment, actions, tree lines, and spacing everywhere else.
---------------------------------------------------------------- 
 Fix the semio/sketchpad Details panel tree rendering regression . remove the duplicate/background tree lines that still show the old broken/gapped path and keep only one clean continuous set of tree guides. `IndentationLines` and all connector rendering should produce a single continuous vertical guide system with no overlapping secondary lines, no broken background line remnants, and no double-rendered gutter strokes.

Also remove the small rectangular element currently overlapping the chevron in nested `TreeItem` / `SortableTreeItems` headers. Eliminate that box visually and structurally from the shared tree/header layout while preserving expand/collapse, drag-reorder behavior, hit targets, alignment, and spacing. If it is tied to a sortable drag handle or wrapper, refactor that handle so it no longer renders as a separate rectangle in the tree gutter and instead integrates cleanly with the existing header row.

Apply this generically across `PanelSection`, `TreeSection`, `TreeItem`, `TreeRow`, `TreeContent`, `SortableTreeItems`, `IndentationLines`, and `TreeContext` without hardcoding section names or one-off fixes. Keep the existing hierarchy, chevrons, property row/value-column layout, and continuous tree lines, but remove the overlapping background guides and the rectangular gutter artifact everywhere in the Details panel.
 
 .........................................................................................
 
 The previous refactor applied the shared right-edge alignment rule too broadly and mixed tree header rows with property rows. Restore the distinction between tree header layout and property row/value-column layout.

Apply one shared vertical end line only to value-side field widgets: Input, Textarea, Combobox, Stepper, Slider value/control area, Toggle, and Button when used as a field control. Do NOT force TreeSection / TreeItem / SortableTreeItems header rows or `actions[]` into that same property field column. Collection/container headers and nested item headers must remain tree headers: chevron, drag handle, label, and add/remove actions should read as one header row and should not be stretched or positioned like value-field rows.

Keep PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, TreeContext, the existing hierarchy, and tree lines unchanged. Preserve the current property row/value-column layout for actual field rows only. Refactor the layout rules generically so tree headers use header alignment, field rows use value-column alignment, and only real value widgets terminate on the shared vertical right edge.


-----------------------------------------------------------------------------------------------------

sketchpad workbench fix the name of add piece to show as add piece when hover currenly semio.sketchpad...types..addpiece

Refactor the semio/sketchpad Details panel so the right edge of every value-side UI widget ends on the same exact x-coordinate, regardless of widget type or nesting level. In the right-side property inspector, all Input, Textarea, Combobox, Stepper, Slider, Toggle, Button, and tree add/remove action controls must share one common vertical end line. For composite controls, align the outermost rendered control boundary: the right border of Input/Textarea/Combobox, the right edge of the plus button in Stepper, the right edge of the value/action area for Slider, the right edge of Toggle/Button, and the right edge of TreeSection/TreeItem action buttons. Keep PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, the existing hierarchy, and the property row/value-column layout unchanged. Apply this generically by shared layout rules, not by hardcoded section names or one-off fixes, so every control in the Details panel terminates on one shared vertical line.


sketchpad kit app tags should be never visible in the ui. remove from the diagram and table views and make sure they are not rendered anywhere in the app. also make sure that they are not selectable or interactable in any way, and that they do not cause any visual glitches or layout issues when present in the data. if they are used for internal logic or data management, ensure that they are properly filtered out before rendering and that their presence does not affect the user experience in any negative way. overall, ensure that the kit app maintains a clean and professional appearance without any unintended tag elements visible to the user. remove the tag from the toolbar filter as well.


 sketchpad details panel, align `TreeSection` / `TreeItem` add/remove actions so the `AddIcon` / `RemoveIcon` buttons sit flush with the right edge of the property value field column. Treat these as tree actions, not Stepper controls. Keep the existing PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, TreeContext, and property row/value-column layout unchanged. Apply this generically to collection nodes in the right-side property inspector so action buttons share the same right boundary as the field column and do not drift or misalign across nested groups.


make the default right-side property inspector width 1.3x the current default, resize all Stepper to match Input/Combobox height and control scale, and always show the current numeric value. Make sure all input fields have the same size, currently some of them are longer or shorter than others. Keep the existing PanelSection, TreeSection, TreeItem, TreeRow, TreeContent, SortableTreeItems, IndentationLines, TreeContext, and property row/value-column layout. Fix nested alignment mismatches in Parent Connection and similar subtrees so labels, row starts, widths, and controls follow one consistent property-layout structure without flattening the hierarchy or breaking tree lines. Apply this generically, not through hardcoded section-specific fixes.

Make node sizing noticeably smaller, Do not scale the text; instead, size each circle from the text’s bounding box so the circle is roughly twice as large as the label footprint, with the label centered and enough inner padding to feel balanced, then optimize the remaining D3 force parameters—charge, link distance/strength, collision, centering, alpha decay, and velocity decay—for a compact, balanced, readable relational layout with minimal overlap and crossings, while maintaining clear visual distinction between nodes and labels. Test various parameter combinations to find the best fit for the new smaller node/label sizes, ensuring the graph remains legible and aesthetically pleasing without excessive whitespace or clutter. Document the final parameter values and their impact on the layout for future reference and adjustments.

//////////////////////////////////////////////////

Refine the semio/sketchpad Details panel spacing in the right-side property inspector without changing the existing PanelSection architecture, section registration logic, interaction model, or tree-based inspector layout.

Keep the current TreeSection, TreeItem, TreeRow, SortableTreeItems, and property row/value-column layout. Improve readability by introducing structure-driven spacing rules in the shared tree primitives: use minimal spacing between rows that belong to the same logical property group, use a clearly larger gap between sibling nested groups, and use the strongest separation plus a divider between top-level property sections. Apply this generically by hierarchy and structural role, not by section name or field name.

Make sure indentation guides / tree connector lines remain continuous across the full subtree height and never break because of row gaps, group spacing, section spacing, or sortable list spacing. Do not use spacing techniques that interrupt the vertical guide stroke. Also increase the horizontal offset between the tree guide / chevron column and the item label to about 2x the current spacing.



Detail Panel Edits:


- sketchpad design app , currently when selecting the base fixed base piece the plane sections and its tree items and rows are breaking. fix to follorw the structure elsewhere in the detail panel regarding alignment and sizing of the input fields and section titles. make sure that the plane section and its items are visually consistent with the rest of the detail panel, with proper spacing, alignment, and no overlaps between fields or section titles. also make sure that all input fields are fully visible and editable without being cut off or hidden behind other elements, and that the section titles are clearly distinguishable from the input fields with enough gap to avoid confusion. overall ensure that the detail panel maintains a clean and organized layout that allows users to easily view and edit all properties of the selected piece without any visual issues or usability problems.

- sketchpad detail panel, currently input field is two rows indead of a single row, fix the layout to have the input field as a single row. by the end of the field cut the last word and add ellipsis if the content is too long to fit in a single row. make sure that when the input field is clicked or in focus, it expands to show the full content, and when it is not in focus, it collapses back to a single row with ellipsis if necessary. also ensure that this behavior is consistent across all input fields in the detail panel, and that it does not cause any layout issues such as overlapping with other fields or section titles. overall make sure that the input fields are user-friendly, visually clear, and functionally robust in handling varying content lengths while maintaining a clean and organized layout in the detail panel.

sketchpad detail panel double the current gap between tree sections and tree items to seperate them more and make it more clear that they are different sections. also add more gap between the section titles and the first item in the section to make it more clear where the section starts. overall make sure that there is enough spacing between all elements in the detail panel to improve readability and visual clarity, without making it look too sparse or disconnected. also make sure that the spacing is consistent across all sections and fields in the detail panel, and that it does not cause any layout issues such as overlapping or misalignment of elements.

- Gaps between sections and fields
- fix overlap between input fields and section titles
- make sure all fields are editable and support multi-edit where applicable
- when piece 

- input fields are a single line , they get expanded to show the full content when clicked on them, as well as when they are in focus. when they are not in focus they get collapsed to a single line with ellipsis if the content is too long. make sure this is working for all input fields and that it is consistent across all of them, and that it doesnt cause any layout issues such as overlapping with other fields or section titles. also make sure that when multiple pieces are selected and they have different values for a field, the field shows a mixed state (e.g. empty value with a dash or "mixed" text) and that when the user edits the field, it applies the change to all selected pieces, and that the mixed state is cleared and shows the new value after editing. also make sure that when multiple pieces are selected and they have the same value for a field, the field shows that value and allows editing it for all selected pieces. overall make sure that the input fields are user-friendly, visually clear, and functionally robust in handling single and multi-selection scenarios, with proper spacing, no overlaps, and consistent behavior across all fields.


- and not overlapping with each other or section titles, and that they are all editable and support multi-edit where applicable


-"Fix Selected Pieces" button is visible and functional when multiple pieces with inconsistent values are selected.

-side labels should be connecting and connected 

- hover in diagram should also hover in scene window. currently when i hover over a piece in the diagram window it doesnt get hovered in the scene window. make sure to implement this functionality and that it works both ways (hovering in scene window should also hover in diagram window) and that it is consistent across all pieces, and that it doesnt cause any performance issues or visual glitches such as flickering or delayed hover states. also make sure that when multiple pieces are selected and hovered, all of them get hovered in both windows, and that the hover state is visually clear and distinguishable from the selection state, with proper spacing and no overlaps with other elements.
---------------------------------------------------------------------------

dragging is the action of offsetting a piece centers in the diagram 
moving is the action of translating piece planes in the scene

your task is to extend semmo with draging.
introduce a new function to every programming language :`dragPiecesInDesign(design,pieces,offset):DesignDiff `

Add a Design/Drag test to every programming language. Take the input and output from the assets.


The drag algorithm ignores child (or grandchild) pieces of a fixed parent. When a child is draged but the parent is not selected then the parent connection of the child is adjusted.

sketchpad:
Use the the drag function for dragging nodes.



------------------------------

extend semio with anotehr function
`movePieces` 
design
offset
pieces

-----------------------------
create a design diff that offsets the selected pieces:

add offset to piece center

Pieces:
b0
b1

add offset to connection  

Conns:
b2 -- t_fx_b2_co

ensure consistency with semio.ts

------------------

migrate design and pieces to the new format
all infos must be consistent with Metabolism.json


exchange
"guid": "{{piece-guid}}" 

map piece id to Name




-------------------------------------------------

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
