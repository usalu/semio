# Final Check Cohort Timing Audit — 2026-10-06

Read-only evidence explaining differences between a long-running schema-check receipt and a later physical census.

The final-schema-check-after-ack.log filesystem birth is03:49:59.552860, modification04:00:13.936881. Its extracted diagnostic receipt was created04:00:58. The DSL authored-static fixture schema birth/mtime is03:56:41.316503/03:56:41.316574: it was created six minutes forty-two seconds after the check log started. This establishes creation during that invocation, but does not independently timestamp the exact walk snapshot. The generated schema catalog’s modification03:44:43 predates the new file and contains no borrowed-object path.

walkRepositoryTree eagerly collects directory/file arrays before schema per-file analysis. It skips dot directories, symlinks, discovery skip names, taxonomy opaque exclusions and Git submodules. There are no DSL/Stdio-specific taxonomy opaque exclusions, and nested parent schema modules do not exclude tests/fixtures from the tree walk. The later collection loop evaluates every sorted file independently with schemaScopeCollectionPath and schemaCollectionContractPath. Therefore no specific enforcement hole for nested fixture facets is established by source inspection.

Neutral expected enforcement shape: `modules/domain/schema/tests/check/fixtures/sample/schema/JSON`. The existing guard classifies the file as inside a test collection and contract facet irrespective of the enclosing parent schema module. Moving parser inputs to ordinary names plus explicit inertSchemaData is valid; keeping schema facets under examples is not.

Graph hex-float schema was absent by this follow-up, so its filesystem birth/mtime is unavailable. Its absence from the earlier receipt must not be conclusively attributed to concurrent creation without preserved timestamp evidence. Plugin executor was asked for that evidence.

This audit qualifies temporal snapshot differences and does not weaken guards or claim a rerun passed. Final independent census follows executor removal acknowledgment.

Plugin executor confirms full hex-float schema and exact consumer contents were captured before deletion, but no birth/mtime receipt was captured. Its omission cannot be attributed to a race from available evidence. The additional whole-example authority was removed; executor reports four Semio suites passed18tests/622assertions, not independently rerun by this auditor.
