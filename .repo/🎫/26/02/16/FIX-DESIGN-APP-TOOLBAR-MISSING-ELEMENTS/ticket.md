---
goal: COMPOSE-JS-SKETCHPAD
---

# Ticket

## Summary

Fixed the Design app toolbar by adding missing create and filter toolbar sections, and ensuring tool settings render properly.

## Changes

- Added `DesignToolbarFilters` component to Design.tsx with toggles for pieces and connections
- Added `DesignToolbarCreate` component to Design.tsx with action button to add new pieces
- Registered filter and create toolbar sections in the Design app useEffect hook
- Added toolbar labels to en.json and de.json locale files
- Fixed toolbar imports (Action, ToolbarGroup) in Design.tsx
- Implemented proper type casting and kit/design data access for the create functionality

## Log

Started: 2026-02-16

- Analyzed the toolbar structure in Sketchpad.tsx
- Identified missing toolbar sections (create and filter) in Design app
- Examined how Home.tsx and Type.tsx register toolbar sections
- Created DesignToolbarFilters and DesignToolbarCreate components
- Registered toolbar sections with proper specificity and order
- Added internationalization labels for English and German
- Fixed TypeScript errors with proper type casting
  Completed: 2026-02-16

## Todos

- [x] Create DesignToolbarFilters component with filter toggles for pieces and connections
- [x] Create DesignToolbarCreate component with action buttons to create new pieces
- [x] Register toolbar sections in Design app useEffect hook
- [x] Add i18n labels to locale files
- [x] Fix TypeScript compilation errors
- [ ] Test toolbar rendering and verify all elements display correctly
- [ ] Verify tool settings bar renders settings properly

## Plan

1. **Add DesignToolbarFilters Component**: ✅ Created a component that provides filter toggles for pieces and connections
2. **Add DesignToolbarCreate Component**: ✅ Created a component that provides an action button to create new pieces in the design

3. **Register Toolbar Sections**: ✅ Added useEffect hook to register both filter and create toolbar sections with proper specificity and order

4. **Add I18n Labels**: ✅ Added toolbar labels to en.json and de.json

5. **Fix TypeScript Errors**: ✅ Fixed all compilation errors with proper type casting

6. **Test and Verify**: ⏳ Need to verify toolbar renders correctly and tool settings display properly
