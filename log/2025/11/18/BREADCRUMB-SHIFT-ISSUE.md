---
slug: BREADCRUMB-SHIFT-ISSUE
summary: Migration from 2025-11-18_BREADCRUMB-SHIFT-ISSUE.md
---
# Diagnosis: Breadcrumb Shift Issue - RESOLVED

## Solution

**Removed:**

1. All diagnostic console logs
2. Empty breadcrumb items that caused double separators

**Result:**
The breadcrumb API now works cleanly:

- Items with `items` prop (dropdown options) automatically get a chevron dropdown trigger at the end
- Items without `items` are simple clickable breadcrumb items
- Separators are automatically added between items (except items with dropdowns already have their own chevron)
- No more double separators or empty breadcrumb items

**Implementation:**

- `BreadcrumbItem` with `items`: renders content + dropdown chevron button (no separator added after it)
- `BreadcrumbItem` without `items`: renders content (gets automatic separator after, unless it's the last item)
- `BreadcrumbList`: automatically inserts separators between items, but skips adding separator if the item has a dropdown

## Problem Description

The breadcrumb navigation is showing incorrect items and missing some elements:

**Expected:**

```
HOME > TEMPORARY > KITNAME > KITVERSION > DESIGN > DESIGNNAME > CHILDDESIGNNAME >
```

**Actual (Kit app):**

```
HOME > TEMPORARY KITNAME > KITVERSION > > | | >
```

**Actual (Design app):**

```
HOME > TEMPORARY KITNAME > KITVERSION > > DESIGN > DESIGNNAME > CHILDDESIGNNAME >
```

Issues:

1. Missing visual separator between TEMPORARY and KITNAME (they appear merged as "TEMPORARY KITNAME")
2. KITVERSION is visible but seems shifted
3. Empty breadcrumb items (> > and | |) in kit app
4. Extra empty breadcrumb item (> >) before DESIGN in design app

## Analysis & Root Causes

### 1. Visual Merging of TEMPORARY and KITNAME

From console logs:

- `kitGuid` exists, `kitKind` = 'temporary', `kitName` = 'New Kit'
- TEMPORARY item renders with `items={undefined}` (no dropdown when kitGuid exists)
- KITNAME item renders with `items={kitItemsWithCreate}` (has dropdown)

The separator logic works correctly - TEMPORARY gets a separator (> chevron), KITNAME gets NO separator (has dropdown).

**Visual appearance:** `[TEMPORARY icon] > [KITNAME text]>[chevron]`

The issue is perceptual - the separator chevron and the dropdown chevron appear close together, making it look merged. **This is actually correct behavior** - the separator after TEMPORARY and the dropdown trigger on KITNAME are both needed.

### 2. Empty Breadcrumb Items

**Root cause:** BreadcrumbItems with empty content were being rendered:

1. **Kit app** - Had an "artifacts" dropdown breadcrumb with empty content that showed even when no filter was active
2. **Design/Type apps** - Had "selectChild" dropdown breadcrumbs with empty content

**Fix applied:** Removed the empty artifacts breadcrumb from kit app. The artifacts dropdown should only show when a filteredKind is active, and it now does show the appropriate icon (designs/types/qualities/etc).

The child selection breadcrumbs already had proper conditionals (`{designChildItems.length > 1 &&`) to only show when there are actual children to select.

### 3. Solution Summary

**Fixed:**

- Removed empty artifacts breadcrumb in kit app that was showing when no kind filter was active
- Verified child selection breadcrumbs have proper conditionals

**Not an issue:**

- The visual appearance of "TEMPORARY KITNAME" is actually correct - there IS a separator between them (the chevron after TEMPORARY icon)
- The dropdown chevron on KITNAME is also correct

The breadcrumb structure is now:

```
HOME > [TEMPORARY icon] > [KITNAME text]▼ > [KITVERSION text]▼ > [filtered content if any]
```

Where `>` is a separator and `▼` is a dropdown trigger.
