# Summary

Fixed issues causing incorrect affected definitions and section line attribution in ticket file metrics:

1. **Local variables**: Updated Go `definitionRegexp` to only match at column 0 (top-level declarations)
2. **False positives from removed lines**: Changed `computeAffectedSections` to only use added lines for determining affected definitions, since removed lines reference old file positions that don't map to new file definition ranges
3. **Removed line attribution**: Mapped removed line totals using base-commit section ranges while keeping added lines and affected definitions tied to current sections
