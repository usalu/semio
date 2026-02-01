# Kit Selection System Migration Prompts

Use these 5 prompts in order with an LLM to migrate the selection system from Design.tsx to Kit.tsx.

---

## PROMPT A: Extract Design Selection Behavior

```
In js/semio/sketchpad/Design.tsx, identify and document all selection entry points and helpers.

Specifically, provide:

1. **Selection State Shape**: What does DesignAppSelection look like?
2. **Selection Events**: What events mutate selection (e.g., DESIGN.SET_SELECTION)?
3. **Selection Helper Hooks**: List all useDesignApp* hooks related to selection:
   - What each hook does (select, add, remove, toggle, clear, select-all, delete)
   - Function signature (inputs and outputs)
   - Any special cases (pieces vs connections vs connectors)
4. **Event Handlers**: How do UI components call these helpers?
   - Example: click handler, modifier key semantics (Ctrl/Cmd, Shift, etc.)
5. **Inverse Diff**: How does inverseDesignAppSelectionDiff work?

Format as a structured "Selection Contract" that summarizes Design's approach.
```

---

## PROMPT B: Extract Kit Selection Surface Area

```
In js/semio/sketchpad/Kit.tsx, identify the current selection infrastructure.

Specifically, provide:

1. **Selection State Shape**: What does KitAppSelection look like (types, designs, qualities, ports, tags, concepts, files, folders, authors)?
2. **Existing Selection Events**: What events currently mutate selection?
   - Examples: KIT.SET_SELECTION, KIT.SELECT_TYPE, KIT.DESELECT_TYPE, etc.
   - Which are implemented vs. stubs?
3. **Existing Selection Hooks**: List all useKitApp* hooks related to selection:
   - useKitAppSelection() and how it works
   - useKitAppSelectType() / useKitAppDeselectType() if they exist
   - Any dimension-specific helpers
4. **Gaps vs Design**: Based on PROMPT A's Design contract, what selection helpers does Kit lack?
   - Are there merge-style helpers (add/remove/toggle per dimension)?
   - How is "select all" handled?
5. **Selection Inverse Diff**: Is there an inverseKitAppSelectionDiff function?

Format as a gap analysis showing what Design has that Kit is missing.
```

---

## PROMPT C: Propose Kit Merge-Style Helper Layer

```
Based on Prompts A & B, design a Kit-native "merge-style" helper layer for selection.

Goals:
- Use existing useKitAppSelection() and KIT.SET_SELECTION under the hood
- Preserve unrelated selection dimensions (selecting a type shouldn't clear ports/tags)
- Provide helpers for add/remove/toggle/clear per selection dimension

Provide:

1. **Generic Utility Functions** (pseudocode OK):
   - addToSelection<K extends keyof KitAppSelection>(selection, key, value): KitAppSelection
   - removeFromSelection<K extends keyof KitAppSelection>(selection, key, value): KitAppSelection
   - toggleInSelection<K extends keyof KitAppSelection>(selection, key, value): KitAppSelection
   - clearSelection(): KitAppSelection
   - clearSelectionDimension<K extends keyof KitAppSelection>(selection, key): KitAppSelection

2. **Hook Wrappers** (for each selection dimension: types, designs, ports, tags, concepts, qualities, files, folders, authors):
   - useKitAppAddTypeToSelection(): ActionField<[typeGuid: string]>
   - useKitAppRemoveTypeFromSelection(): ActionField<[typeGuid: string]>
   - useKitAppToggleTypeInSelection(): ActionField<[typeGuid: string]>
   - useKitAppSelectSingleType(): ActionField<[typeGuid: string]>
   - (repeat for each dimension)

3. **Modifier Key Strategy**:
   - Click with no modifier: replace selection for that dimension
   - Click with Ctrl/Cmd: toggle in selection
   - Click with Shift: add to selection
   - Click with Alt: remove from selection

4. **Empty Selection Convention**:
   - Should empty arrays be kept as [] or deleted from the object?
   - Propose one approach and explain why.

Show TypeScript type signatures and pseudocode. Real implementation code optional at this stage.
```

---

## PROMPT D: Implement Helper Hooks and Wire into UI

```
Implement the helper functions and hooks proposed in Prompt C.

Tasks:

1. **Create kitSelectionHelpers.ts** (new file in js/semio/sketchpad/):
   - Generic utility functions: addToSelection, removeFromSelection, toggleInSelection, clearSelection, clearSelectionDimension
   - Export them clearly

2. **Add hooks to Kit.tsx** (in the "Hooks" region):
   For each selection dimension (types, designs, ports, tags, concepts, qualities, files, folders, authors), implement:
   - useKitAppAdd{Dimension}ToSelection(): ActionField<[id: string]> or ActionField<[id: Guid]>
   - useKitAppRemove{Dimension}FromSelection(): ActionField<[id: string]> or ActionField<[id: Guid]>
   - useKitAppToggle{Dimension}InSelection(): ActionField<[id: string]> or ActionField<[id: Guid]>
   - useKitAppSelectSingle{Dimension}(): ActionField<[id: string]> or ActionField<[id: Guid]>

   All hooks must:
   - Use existing useKitAppSelection() and setSelection under the hood
   - Respect canSetSelection gating
   - Return ActionField pattern (with execute() and canExecute)

3. **Wire UI Click Handlers**:
   Show how a table row click handler should work:
   - Normal click: replace selection (useKitAppSelectSingleType)
   - Ctrl/Cmd click: toggle selection (useKitAppToggleTypeInSelection)
   - Shift click: add to selection (useKitAppAddTypeToSelection)
   - Alt click: remove from selection (useKitAppRemoveTypeFromSelection)
   - Background click: clear selection (useKitAppClearSelection)

4. **Handle Scope Correctly**:
   - Ensure hooks use useKitScope() to get the correct kitGuid
   - Verify actor.send() targets the correct kit

Provide actual TypeScript code ready to paste into Kit.tsx and kitSelectionHelpers.ts.
```

---

## PROMPT E: Parity Checks and Test Plan

```
Create a parity checklist and test plan to verify Kit's selection system matches Design's behavior.

Provide:

1. **Unit Tests** (pseudocode or real test code):
   - Test addToSelection: duplicate detection, returns unchanged if duplicate
   - Test removeFromSelection: removes correctly, handles non-existent IDs
   - Test toggleInSelection: adds if missing, removes if present
   - Test clearSelection: returns empty object
   - Test with multiple dimensions: adding type doesn't affect ports/tags

2. **Integration Tests** (describe or implement):
   - Click table row with no modifier: replaces selection for that dimension
   - Click with Ctrl/Cmd: toggles item in selection
   - Click with Shift: adds item without removing others
   - Click with Alt: removes item from selection
   - Click background: clears selection
   - Escape key (optional): clears selection

3. **Edge Cases**:
   - Adding a type that's already selected (no-op)
   - Removing a type that's not selected (no-op)
   - Selecting when canSetSelection is false (hook returns no-op function)
   - Switching kits (selection should reset or transfer appropriately)
   - Empty arrays vs deleted keys (internal convention consistency)

4. **State Machine Gating**:
   - Verify snapshot.can() is checked before allowing selection mutations
   - Verify selection scope matches kitGuid
   - Verify undo/redo works with selection changes

Format as:
- [ ] Checklist items for manual verification
- Test code snippets (vitest/jest format preferred)
- Description of how to run tests
```

---

## How to Use These Prompts

1. **Copy Prompt A** and paste into your LLM (provide Kit.tsx + Design.tsx context if needed)
2. **Review output** for completeness. If gaps, ask clarifying questions.
3. **Copy Prompt B** and ask the LLM to run it (now with Prompts A's context)
4. **Copy Prompt C** and ask the LLM to design the helper layer
5. **Copy Prompt D** and ask the LLM to implement (get real code)
6. **Copy Prompt E** and ask the LLM to create tests

Each prompt depends on prior outputs, so maintain context throughout the session.

---

## Tips

- **If stuck at any stage**: Ask the LLM "Can you explain why..." or "Show me an example..."
- **If output is too verbose**: Say "Summarize this as a bulleted list"
- **If output is wrong**: Show the LLM the exact code from Kit.tsx/Design.tsx and ask "Does this match?"
- **To iterate**: "The helpers are too generic, make them simpler by..."
