---
slug: CREATE-INTERFACE-TAG-CLICK
summary: Fix create interface/tag buttons doing nothing
prompt: Fix create interface/tag buttons doing nothing
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.905Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

Create actions in the Kit app are triggered via filter-strip `Toggle` action buttons.

# Plan

Trace UI `actionId` → `onActionClick` → kit command wiring, then make create actions immediately visible by switching the kind filter and selection to the created artifact.

# Changes

Updated Kit app create actions for interfaces/tags (and aligned concepts/folders) to set the current kind filter and selection to the newly created entity, and moved default names to i18n-backed labels.
