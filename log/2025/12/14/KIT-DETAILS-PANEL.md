---
slug: KIT-DETAILS-PANEL
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix Kit app details panel rendering
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---
# Previously
Kit app details panel had inconsistent rendering:
- Concept section appeared twice.
- Design details rendered the name twice and omitted most design properties.
- Type details section rendered empty when selecting a type.

# Plan
- Inspect Kit app details section registration and section components.
- Ensure dynamic sections are removed consistently and re-added deterministically.
- Align Kit app read-only detail fields with the corresponding app detail field sets.
- Update dev docs with the details-panel section mechanics.

# Changes
- Fixed details section registration to remove all conditional section ids (including tags/concepts) and to remove them again in cleanup.
- Wrapped kit artifact detail sections in `KitScopeProvider` so `useKit()`-based sections render reliably.
- Updated kit app `SingleTypeSection` and `SingleDesignSection` to render the expected property fields and removed the duplicated name fields.
