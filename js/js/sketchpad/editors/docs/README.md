# Docs Integration into Sketchpad

This document describes the integration of documentation into Sketchpad, transitioning from Astro Starlight to an embedded docs system.

## Structure

### Editor (`js/js/sketchpad/editors/docs/`)

- **registration.tsx**: Registers the docs editor with route prefix `docs/`
- **store.tsx**: Manages docs state including:
  - Current selection (section, page)
  - Section states (expanded, progress, completed pages)
  - Scroll position
- **commands.ts**: Commands for:
  - Selecting pages
  - Toggling sections
  - Updating section progress (for tutorials)
  - Marking pages as complete
- **Editor.tsx**: Main editor component
- **panels/**: Panel components
  - **Workbench.tsx**: Navigation tree with 6 sections
  - **Details.tsx**: Heading structure tree for current page
- **canvas/**: Rendering components
  - **Page.tsx**: MDX page renderer

### UI Elements (`js/js/elements/docs/`)

- **Page.tsx**: Page container with frontmatter support
- **Section.tsx**: Section wrapper for content organization
- **Card.tsx**: Card and CardGrid components (replaces Astro Starlight)
- **Steps.tsx**: Steps component for tutorials (replaces Astro Starlight)
- **index.ts**: Exports all docs elements

### Docs Content (`js/js/sketchpad/docs/`)

Documentation files organized by section:
- `getting-started/` - 🚀 Getting Started
- `tutorials/` - 📝 Tutorials
- `integrations/` - 🔀 Integrations
- `manuals/` - 📖 Manuals
- `theory/` - 📚 Theory
- `showcases/` - 🌟 Showcases

Each file is an MDX file with:
- Frontmatter (title, description, sidebar config)
- MDX content using Semio components

## Features

### Navigation System

The docs use the same navigation system as kits but with `docs/` path prefix:
- `docs/` - Docs home
- `docs/getting-started/intro` - Specific page
- `docs/tutorials/hello-semio/model-brick-set` - Nested page

### Section State Management

Sections can track:
- **Expansion state**: Whether the section is expanded in the tree
- **Progress**: Percentage completion (useful for tutorials)
- **Completed pages**: List of completed pages

### Fragment Navigation

The details panel shows the heading structure. Clicking a heading scrolls to that section using fragment identifiers (e.g., `#installation`).

### Component Mapping

Astro Starlight components have been replaced:

| Astro Starlight | Semio Element |
|----------------|---------------|
| `<Card>` | `<Card>` |
| `<CardGrid>` | `<CardGrid>` |
| `<Steps>` | `<Steps>` |
| `<Tabs>`, `<TabItem>` | Use existing `<Tabs>` from `elements/aggregation` |

## TODO

1. **MDX Processing**: Set up MDX compilation pipeline
   - Parse frontmatter
   - Convert MDX to React components
   - Handle component imports

2. **File System Integration**: 
   - Load MDX files from `js/js/sketchpad/docs/`
   - Parse folder structure for navigation
   - Extract headings for details panel

3. **Panel Integration**:
   - Wire up Workbench panel in Editor
   - Wire up Details panel in Editor
   - Connect panels to editor state

4. **Search & Navigation**:
   - Implement search across docs
   - Implement breadcrumb navigation
   - Add prev/next page navigation

5. **Additional Components**:
   - Note/Tip/Warning callouts
   - Code blocks with syntax highlighting
   - Image handling

6. **Migration**:
   - Move existing docs from `js/docs/` to new structure
   - Update all component imports
   - Test all pages

## Usage Example

```tsx
// In a docs MDX file:
---
title: Getting Started
description: Learn how to use Semio
sidebar:
  order: 1
---

import { Card, CardGrid } from "@semio/elements/docs";

# Getting Started

<CardGrid>
  <Card title="Quick Start" icon="rocket">
    Get up and running in 5 minutes
  </Card>
  <Card title="Tutorials" icon="book">
    Step-by-step guides
  </Card>
</CardGrid>
```

## Notes

- All translations are added to `locales/en.json` and `locales/de.json`
- The docs editor follows the same patterns as other editors (design, type, kit, quality)
- Fragment-based scrolling works automatically with heading IDs
- Section state is persisted in Y.js for collaborative features
