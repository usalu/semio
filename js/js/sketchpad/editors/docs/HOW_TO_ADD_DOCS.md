# Adding New Documentation

The docs system follows the **Open/Closed Principle** - it's closed for modification but open for extension.

## Quick Start

### 1. Add a New Section

Edit `registry.ts`:

```typescript
docsRegistry.registerSection({
  id: "my-section",
  label: "My Section",
  emoji: "💡",
  description: "Description of my section",
  order: 7,
});
```

### 2. Add Pages to the Section

Edit `registry.ts`:

```typescript
docsRegistry.registerPages([
  {
    title: "My Page Title",
    description: "Short description for search",
    path: "docs/my-section/my-page",
    section: "my-section",
    order: 1,
  },
]);
```

### 3. Create the MDX File

Create `js/js/sketchpad/docs/my-section/my-page.mdx`:

```mdx
---
title: My Page Title
description: Short description
---

# My Page Title

Content goes here...
```

## That's It!

Your page will automatically:
- ✅ Appear in the navigation tree
- ✅ Be searchable via ⌘K
- ✅ Have working routing
- ✅ Support all doc features

## No Code Changes Needed!

The system automatically handles:
- Routing (via wildcard `docs/*`)
- Navigation tree building
- Search indexing
- Section state management
