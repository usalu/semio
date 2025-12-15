---
date:
  created: '2025-11-27T00:34:29.374Z'
  updated: '2025-11-27T00:34:29.374Z'
slug: DOCS-APP-TEST
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Create E2E test for docs app covering content loading, images, workbench panel
  pages, and details panel headings
model: claude-sonnet-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

No prior work on docs app E2E tests.

# Plan

1. Create E2E tests for the docs app in `js/js/sketchpad.test.ts`
2. Test content loading (page title, description, cards)
3. Test workbench panel shows all documentation sections
4. Test workbench panel shows pages within sections
5. Test details panel shows page section
6. Test navigation between pages works

# Changes

## `js/js/sketchpad.test.ts`

Added `test.describe("Docs")` block with 5 new tests:

1. **Content Loads** - Verifies docs index page loads with:
   - "Welcome to Semio" h1 heading
   - Description text "Design Information Modeling for Architecture"
   - Card headings ("Just want to toy around", "More into research")

2. **Workbench Panel Shows All Sections** - Verifies workbench panel displays:
   - Getting Started, Tutorials, Integrations, Manuals, Theory, Showcases sections

3. **Workbench Panel Shows Pages In Sections** - Verifies workbench panel displays:
   - Installation, Starter, Rhino, sketchpad pages

4. **Details Panel Shows Page Section** - Navigates to sketchpad manual page and verifies:
   - Page headings are visible (Apps h1, Home/Kit/Design h2)
   - Details panel opens and shows Page section button (id: `semio.sketchpad.app.docs.page`)

5. **Navigation Works Between Pages** - Tests page navigation:
   - Clicks next button (Intro)
   - Verifies URL changes to docs/getting-started/intro
   - Verifies intro page title is visible

All 7 tests (2 existing + 5 new) pass with `npx playwright test --project=chromium`.
