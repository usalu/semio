# Fix Toolbar Dropdown Positioning and Content

## User Request
- Fix dropdown toggle rendering (currently renders "somewhere else").
- Dropdown should show list of subtools directly on top of the tool as a vertical table list.
- Selection and Hand tools should be in this subtool list.
- Additive/Subtractive selection should show in the tool setting bar instead of Hand/Selection.

## Plan
1.  **Analyze `Sketchpad.tsx`**: Understand toolbar rendering, dropdown positioning, and how subtools are handled.
2.  **Analyze `Design.tsx`**: Check how Selection, Hand (Pan), and selection modes are configured.
3.  **Fix Dropdown Positioning**: Ensure the dropdown renders correctly above the tool button.
4.  **Refactor Subtool Content**:
    - Move Hand (Pan) and Selection into the subtool list for the Selection tool.
    - Move Additive/Subtractive modes to the tool setting bar when Selection is active.
5.  **Verify**: Ensure changes match the requirements.

## Current State
- Reading `js/compose/sketchpad/Sketchpad.tsx` to understand current implementation.
