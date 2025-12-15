---
slug: CREATE-INTERFACE-TAG-CLICK
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix create interface/tag buttons doing nothing
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---
# Previously
Create actions in the Kit app are triggered via filter-strip `Toggle` action buttons.

# Plan
Trace UI `actionId` → `onActionClick` → kit command wiring, then make create actions immediately visible by switching the kind filter and selection to the created artifact.

# Changes
Updated Kit app create actions for interfaces/tags (and aligned concepts/folders) to set the current kind filter and selection to the newly created entity, and moved default names to i18n-backed labels.
