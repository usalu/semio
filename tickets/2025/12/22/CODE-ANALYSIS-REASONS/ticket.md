---
slug: CODE-ANALYSIS-REASONS
prompt: >-
  Extend the code analysis hook to require reason/solution text for each issue
  and document the policies.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-22T22:28:39.916Z"
iterations: []
---

# Previously

- New ticket created for extending code analysis issue metadata.

# Plan

- Audit existing code analysis issue generation.
- Add reason/solution fields to code issue output and populate for all issue kinds.
- Update dev docs for code analysis issue metadata expectations.

# Changes

- Added reason/solution fields to code analysis issues and populated defaults plus import/terminology specifics.
- Documented reason/solution metadata in README and AGENTS.
- Made reason/solution text specific to semio documentation structure and import boundaries.
