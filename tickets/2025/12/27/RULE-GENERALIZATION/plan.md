# Previously

Three separate policies exist: `header-region`, `empty-region`, `inline-comment`. Each produces a single violation type.

# Plan

Generalize policies into scope-based groups that produce multiple related violations:

1. **header** policy (replaces header-region):
   - `header:missing-region` - No #region Header found
   - `header:missing-filename` - Missing filename line
   - `header:missing-contributors` - No contributor/author line (YEAR Name <email>)
   - `header:missing-license` - No license text
   - `header:wrong-license` - License doesn't match AGPL-3.0

2. **region** policy (replaces empty-region):
   - `region:empty` - Region contains no code
   - `region:missing-start-name` - #region without name
   - `region:missing-end-name` - #endregion without name
   - `region:name-mismatch` - Start and end names don't match

3. **comment** policy (replaces inline-comment):
   - `comment:inline` - Forbidden inline comment (//)
   - `comment:block` - Forbidden block comment (/\* \*/)
   - `comment:jsdoc` - Forbidden JSDoc comment (/\*\* \*/)

Violation kinds use `:` separator (e.g., `header:missing-license`).

# Changes

- Replaced `header-region` policy with generalized `header` policy
- Replaced `empty-region` policy with generalized `region` policy
- Replaced `inline-comment` policy with generalized `comment` policy
