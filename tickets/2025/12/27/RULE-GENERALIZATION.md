---
slug: RULE-GENERALIZATION
prompt: >-
  Generalize and extend existing rules to produce sets of related violations.
  E.g. header-region to header (missing region, missing contributors, missing
  license, wrong license), empty-region to region (empty region, missing start
  name, missing end name, unmatching names), inline-comment to comment
  (forbidden inline, forbidden block).
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-26T23:23:12.673Z'
commit: fb102e545bdc3ee63de0e3d93cf631da7fa2fe7c
---
# Previously

Three separate rules exist: `header-region`, `empty-region`, `inline-comment`. Each produces a single violation type.

# Plan

Generalize rules into scope-based groups that produce multiple related violations:

1. **header** rule (replaces header-region):
   - `header:missing-region` - No #region Header found
   - `header:missing-filename` - Missing filename line
   - `header:missing-contributors` - No contributor/author line (YEAR Name <email>)
   - `header:missing-license` - No license text
   - `header:wrong-license` - License doesn't match AGPL-3.0

2. **region** rule (replaces empty-region):
   - `region:empty` - Region contains no code
   - `region:missing-start-name` - #region without name
   - `region:missing-end-name` - #endregion without name
   - `region:name-mismatch` - Start and end names don't match

3. **comment** rule (replaces inline-comment):
   - `comment:inline` - Forbidden inline comment (//)
   - `comment:block` - Forbidden block comment (/* */)
   - `comment:jsdoc` - Forbidden JSDoc comment (/** */)

Violation kinds use `:` separator (e.g., `header:missing-license`).

# Changes

- Replaced `header-region` rule with generalized `header` rule
- Replaced `empty-region` rule with generalized `region` rule
- Replaced `inline-comment` rule with generalized `comment` rule
