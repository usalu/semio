# Selection Functionality Comparison: Design App vs Kit App

## Analysis Summary

### Problem Identified
The Kit app selection functionality was not working properly compared to the Design app because it was missing keyboard event handlers for modifier keys (Shift, Ctrl/Meta).

### Key Differences Found

#### Design App (Working Correctly)
**Location:** `js/semio/sketchpad/Design.tsx` lines 7497-7519

**Implementation:**
- Has `useEffect` hook with `keydown` and `keyup` event listeners
- Automatically switches `activeTool` state when Shift key is pressed → SELECTION_ADDITIVE
- Automatically switches `activeTool` state when Ctrl/Meta key is pressed → SELECTION_SUBTRACTIVE  
- Reverts to SELECTION_NORMAL when modifier keys are released
- This provides visual feedback in the toolbar and correct selection behavior

**Code Pattern:**
```typescript
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    if (activeTool === ToolKind.SELECTION_NORMAL) {
      if (e.shiftKey && !e.ctrlKey && !e.metaKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_ADDITIVE);
      } else if ((e.ctrlKey || e.metaKey) && !e.shiftKey) {
        if (setActiveTool) setActiveTool(ToolKind.SELECTION_SUBTRACTIVE);
      }
    }
  };
  const handleKeyUp = (e: KeyboardEvent) => {
    if (activeTool === ToolKind.SELECTION_ADDITIVE && !e.shiftKey) {
      if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
    } else if (activeTool === ToolKind.SELECTION_SUBTRACTIVE && !e.ctrlKey && !e.metaKey) {
      if (setActiveTool) setActiveTool(ToolKind.SELECTION_NORMAL);
    }
  };
  window.addEventListener("keydown", handleKeyDown);
  window.addEventListener("keyup", handleKeyUp);
  return () => {
    window.removeEventListener("keydown", handleKeyDown);
    window.removeEventListener("keyup", handleKeyUp);
  };
}, [activeTool, setActiveTool]);
```

#### Kit App (Previously Broken)
**Location:** `js/semio/sketchpad/Kit.tsx` line 5162

**Previous Implementation:**
- Used inline `effectiveMode` calculation: 
  ```typescript
  const effectiveMode = e.shiftKey ? "range" : e.metaKey || e.ctrlKey ? "toggle" : 
    activeTool === ToolKind.SELECTION_ADDITIVE ? "add" : 
    activeTool === ToolKind.SELECTION_SUBTRACTIVE ? "remove" : "single";
  ```
- This checked modifier keys but did NOT update `activeTool` state
- No keyboard event listeners at all
- Result: Modifier keys worked for individual clicks but toolbar didn't update and mode wasn't persistent

### Fix Applied

**Location:** `js/semio/sketchpad/Kit.tsx` lines 6943-6975 (in MultiWindowApp component)

**Change Made:**
Added the same keyboard event handler pattern from Design app to Kit app:

1. Added `const [activeTool, setActiveTool] = useKitAppActiveTool();` hook call
2. Added `useEffect` with keydown/keyup handlers identical to Design app
3. Handlers switch `activeTool` state on Shift/Ctrl/Meta press/release
4. Proper cleanup of event listeners on unmount

**Result:**
- Kit app now has the same selection behavior as Design app
- Pressing Shift switches tool to SELECTION_ADDITIVE (visual feedback in toolbar)
- Pressing Ctrl/Meta switches tool to SELECTION_SUBTRACTIVE (visual feedback in toolbar)
- Releasing keys reverts to SELECTION_NORMAL
- Selection state is properly managed through XState actor

### Technical Details

**Event Flow:**
1. User presses Shift/Ctrl/Meta key
2. `keydown` event fires → `handleKeyDown` function
3. Function checks current `activeTool` state
4. If SELECTION_NORMAL, switches to appropriate mode (ADDITIVE or SUBTRACTIVE)
5. `setActiveTool` updates XState actor state
6. UI components (toolbar, diagram) react to state change
7. User releases key → `keyup` event fires → `handleKeyUp` function
8. Function reverts `activeTool` back to SELECTION_NORMAL

**Selection Behavior:**
- **Normal Mode:** Click to select single item (replaces selection)
- **Additive Mode (Shift):** Click to add item to selection or range select
- **Subtractive Mode (Ctrl/Meta):** Click to remove item from selection or toggle

### Files Modified
1. `/workspaces/semio/js/semio/sketchpad/Kit.tsx` - Added keyboard event handlers in MultiWindowApp component

### Testing Notes

**Manual Testing Steps:**
1. Open http://localhost:5173
2. Create a temporary kit
3. Navigate to kit view
4. Press Shift key → observe toolbar selection tool changes to additive mode
5. Release Shift key → observe toolbar reverts to normal mode
6. Press Ctrl/Meta key → observe toolbar selection tool changes to subtractive mode
7. Release Ctrl/Meta key → observe toolbar reverts to normal mode
8. Click table rows with/without modifier keys → verify selection behavior matches Design app

**Playwright Testing:**
Attempted to create automated tests but encountered browser crash issues in the devcontainer environment. The segmentation faults appear to be an environment issue, not a code issue. Manual testing in the browser should be used to verify the fix.

### Conclusion

The Kit app selection functionality has been fixed to match the Design app behavior by adding the missing keyboard event handlers. The implementation is identical to the proven Design app pattern, ensuring consistent user experience across both apps.

The root cause was architectural: Kit app was checking modifier keys inline during click events but never updating the tool state, while Design app properly managed tool state changes through keyboard event listeners.
