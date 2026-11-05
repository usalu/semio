# Compiled Input Export Closure

A pruned Rust source census covered all current framework and s source trees, including package facade files and test-host modules, for explicit public reexports of MutationLeaf, MutationLeafDescriptor and MutationSourceProvenance. A second census covered Mutation-only forwarding groups.

New compile facts are now explicitly forwarded through the replication facade (root and nested), OS SPR command facade, OS SPR facade, mutation-messages facade, store test prelude and VCS companion protocol.

The DSL derive reexport named MutationLeaf is a procedural macro export and intentionally does not receive data types. Domain fixture enum exports named Mutation (window state, scalar state and component fixtures) are not generic protocol forwarding groups and remain their actual domain types.

This source census closes the observed curated forwarding groups. Actual standalone protocol aliases still require a proper native retry to prove the selected current compilation closure; no compile success is inferred from text census.
