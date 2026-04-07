# Plan: Get Kit App Test Complying

## Problem Analysis

The Kit test in `js/semio/sketchpad.test.ts` is failing because:

1. **Loading State Not Handled**: The test waits for text "Loading" to disappear, but the loading state uses a `<Spinner>` component, not text. So the wait never properly waits for kit loading to finish.

2. **pointer-events-none**: Loading rows have `pointer-events-none` CSS class which prevents double-click events from registering.

3. **stopPropagation**: The inner div in table rows has `onClick={(e) => e.stopPropagation()}` which can block event bubbling when clicking on child elements.

4. **No data-testid**: The table rows don't have `data-testid` attributes for reliable test targeting.

## Solution

### 1. Add data-testid to Home.tsx row elements
Add `data-testid={`home-kit-row-${row.id}`}` to the row div for reliable test targeting.

### 2. Fix the test to properly wait for kit loading
- Wait for the loading spinner to disappear instead of "Loading" text
- Wait for the row to not have `pointer-events-none` class
- Or wait for the row to have `data-row-id` attribute

### 3. Fix the double-click targeting
- Use the row element directly via data-testid instead of relying on text content
- Ensure the click is on the row element itself, not a child with stopPropagation

## Tasks

- [ ] Add data-testid to Home.tsx table rows
- [ ] Update initHome() to wait for kit to finish loading properly  
- [ ] Update initHome() to use data-testid for double-click
- [ ] Test that Kit test passes
- [ ] Test that Type and Design tests pass (they depend on initKit/initHome)
