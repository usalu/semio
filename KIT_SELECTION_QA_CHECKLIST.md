# Kit Selection QA Checklist

**Purpose:** Quick checklist for QA verification of the Kit selection system  
**Estimated Time:** 45-60 minutes  
**Status:** ⏳ Pending Execution

---

## Pre-QA Setup

- [ ] Start dev server: `npm run dev:sketchpad`
- [ ] Open browser to `http://localhost:5173`
- [ ] Create or open a kit with multiple types, designs, files, etc.
- [ ] Open browser console (for debugging if needed)
- [ ] Have second browser tab ready (for sync testing)

---

## Part 1: Unit Tests (10 minutes)

### Execute Tests

```bash
npm run test -- kitSelection.test.ts
```

### Verification

- [ ] All tests pass (35+ tests)
- [ ] No TypeScript compilation errors
- [ ] No runtime errors
- [ ] Performance tests under thresholds (<10ms)

### If Tests Fail

1. Note which test failed: _________________
2. Review error message: _________________
3. Check implementation vs test expectation
4. Fix and rerun
5. Document fix applied: _________________

---

## Part 2: Basic Selection (5 minutes)

### Types Table

- [ ] Click a type row → Only that type selected
- [ ] Click another type → First deselected, new selected
- [ ] Selection highlights row correctly
- [ ] Selection state visible in UI

### Other Dimensions

- [ ] Click a design → Selects correctly
- [ ] Click a file → Selects correctly
- [ ] Click a port → Selects correctly
- [ ] Type selection remains when selecting design (independence)

---

## Part 3: Modifier Keys (10 minutes)

### Ctrl/Cmd (Toggle)

- [ ] Select type-1
- [ ] Ctrl+Click type-2 → Both selected
- [ ] Ctrl+Click type-1 → type-1 deselected, type-2 remains
- [ ] Ctrl+Click type-1 again → Both selected again

### Shift (Add)

- [ ] Clear selection
- [ ] Select type-1
- [ ] Shift+Click type-2 → Both selected
- [ ] Shift+Click type-2 again → No duplicate (still both)
- [ ] Shift+Click type-3 → All three selected

### Alt (Remove)

- [ ] Select types 1, 2, 3 (using Ctrl+Click)
- [ ] Alt+Click type-2 → Removed, 1 and 3 remain
- [ ] Alt+Click type-4 (not selected) → No change
- [ ] Alt+Click type-1 → Only type-3 remains

### No Modifier (Replace)

- [ ] Select types 1, 2, 3
- [ ] Click type-4 (no modifier) → Only type-4 selected
- [ ] Previous selection cleared

---

## Part 4: Global Actions (5 minutes)

### Select All

- [ ] Trigger select all (Ctrl+A or button)
- [ ] All types selected
- [ ] All designs selected
- [ ] All files selected
- [ ] All other dimensions populated
- [ ] Empty dimensions not included

### Clear Selection

- [ ] Select multiple items across dimensions
- [ ] Click background area → All cleared
- [ ] Press Escape key → All cleared
- [ ] Verify selection state is empty `{}`

---

## Part 5: Multi-Dimensional Independence (5 minutes)

- [ ] Select type-1
- [ ] Select design-1
- [ ] Both remain selected independently
- [ ] Click type-2 (no modifier) → design-1 still selected
- [ ] Click design-2 (no modifier) → type-2 still selected
- [ ] Clear types → designs unaffected
- [ ] Clear designs → types unaffected

---

## Part 6: Table ↔ Diagram Sync (5 minutes)

### Table to Diagram

- [ ] Select type in table view
- [ ] Switch to diagram view
- [ ] Type node highlighted in diagram
- [ ] Deselect in diagram
- [ ] Switch to table view
- [ ] Type row no longer highlighted

### Diagram to Table

- [ ] Select type node in diagram
- [ ] Switch to table view
- [ ] Type row highlighted
- [ ] Select another type in table
- [ ] Switch to diagram
- [ ] Both nodes highlighted

---

## Part 7: Real-Time Sync (Y.js) (10 minutes)

### Multi-Tab Sync

- [ ] Open kit in Tab 1
- [ ] Open same kit in Tab 2
- [ ] Select type-1 in Tab 1
- [ ] **Wait 1-2 seconds**
- [ ] Selection appears in Tab 2
- [ ] Deselect in Tab 2
- [ ] **Wait 1-2 seconds**
- [ ] Deselection reflects in Tab 1

### Multi-User Simulation

- [ ] Tab 1: Select types 1, 2
- [ ] Tab 2: Select types 2, 3
- [ ] **Wait for sync**
- [ ] Both tabs show types 1, 2, 3 selected (merge)

---

## Part 8: Edge Cases (5 minutes)

### No-Op Operations

- [ ] Select type-1
- [ ] Shift+Click type-1 again → No duplicate
- [ ] Alt+Click type-2 (not selected) → No change
- [ ] Selection state unchanged

### Empty Selection

- [ ] Clear all selections
- [ ] Verify selection object is `{}`
- [ ] No empty arrays stored (check console/DevTools)
- [ ] Keys absent when dimensions empty

### Rapid Operations

- [ ] Rapidly toggle same type 10 times
- [ ] No lag or errors
- [ ] Final state correct

---

## Part 9: State Machine Gating (5 minutes)

### Permission Check

- [ ] Open Kit app
- [ ] Selection works (canAct = true)
- [ ] Navigate away from kit
- [ ] Try to select (should be no-op if properly gated)

### Scope Isolation

- [ ] Open kit-1, select types
- [ ] Navigate to kit-2
- [ ] Selection cleared or scoped to kit-2
- [ ] Navigate back to kit-1
- [ ] Selection is fresh (not restored)

---

## Part 10: Undo/Redo (Optional, 5 minutes)

- [ ] Select type-1
- [ ] Press Ctrl+Z (undo)
- [ ] Selection reverted
- [ ] Press Ctrl+Shift+Z (redo)
- [ ] Selection restored
- [ ] Verify transaction integration

---

## Part 11: Cross-Browser (Optional, 10 minutes)

### Chrome/Edge

- [ ] All basic selection works
- [ ] Modifier keys work
- [ ] Y.js sync works

### Firefox

- [ ] All basic selection works
- [ ] Modifier keys work
- [ ] Y.js sync works

### Safari (if available)

- [ ] All basic selection works
- [ ] Modifier keys work
- [ ] Y.js sync works

---

## Issues Log

**Issue 1:**
- **Description:** _____________________
- **Severity:** [ ] Blocker [ ] Major [ ] Minor
- **Steps to Reproduce:** _____________________
- **Expected:** _____________________
- **Actual:** _____________________
- **Resolution:** _____________________

**Issue 2:**
- **Description:** _____________________
- **Severity:** [ ] Blocker [ ] Major [ ] Minor
- **Steps to Reproduce:** _____________________
- **Expected:** _____________________
- **Actual:** _____________________
- **Resolution:** _____________________

**Issue 3:**
- **Description:** _____________________
- **Severity:** [ ] Blocker [ ] Major [ ] Minor
- **Steps to Reproduce:** _____________________
- **Expected:** _____________________
- **Actual:** _____________________
- **Resolution:** _____________________

---

## Summary

**Date:** _____________________  
**Tester:** _____________________  
**Environment:** _____________________  
**Browser(s):** _____________________

**Results:**
- [ ] All tests passed
- [ ] Minor issues found (documented above)
- [ ] Major issues found (blocking)

**Total Time:** _______ minutes

**Overall Status:**
- [ ] ✅ Ready for production
- [ ] ⚠️ Minor fixes needed
- [ ] ❌ Major issues, needs rework

**Sign-off:** _____________________

---

## Next Steps After QA

### If All Pass ✅

1. Update `KIT_SELECTION_TEST_PLAN.md` appendix with results
2. Update `KIT_SELECTION_MIGRATION_COMPLETE.md` status to 100%
3. Create PR for review
4. Deploy to staging

### If Issues Found ⚠️

1. Document issues in this checklist
2. Create GitHub issues for tracking
3. Fix issues in order of severity
4. Re-run affected tests
5. Repeat QA when fixed

### If Major Blockers ❌

1. Document blocking issues
2. Rollback if necessary
3. Schedule fix session
4. Full QA re-run after fix

---

## Reference Documents

- **Test Plan:** `KIT_SELECTION_TEST_PLAN.md`
- **Quick Reference:** `KIT_SELECTION_QUICK_REFERENCE.md`
- **Implementation:** `KIT_SELECTION_COMPLETION_SUMMARY.md`
- **Migration Summary:** `KIT_SELECTION_MIGRATION_COMPLETE.md`

---

*Checklist version 1.0 - February 1, 2026*
