# Plugin Example Module Deduplication

Strict WASI diagnostics identify 30 example sources compiled more than once. Pass 446 records their current Rust module scopes before changing declarations or callers. Canonical domain modules should retain the definitions, and duplicated top-level example paths should be removed with caller updates. No source edits have been applied by this audit.

Pass 448 removed 31 duplicate declarations across 21 package roots and updated 7 qualified references in callers/documentation. Repository-wide Rust identifier search found three live references, all in Remodel example metadata; these now point to its canonical artifact example. Existing example test modules remain registered. CAD's redundant artifact-level example wrapper was removed in favor of the standard/subset module. No compatibility aliases were added. Compiler validation is pending.
