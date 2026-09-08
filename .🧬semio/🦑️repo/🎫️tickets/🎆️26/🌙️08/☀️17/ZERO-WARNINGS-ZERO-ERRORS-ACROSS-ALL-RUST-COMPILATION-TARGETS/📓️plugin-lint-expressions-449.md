# Plugin Expression Lint Cleanup

Pass 449 inspected strict WASI 428 diagnostics and prepared 2205 non-overlapping expression fixes in 739 files. Compiler byte spans, full source lines, and Unicode columns are checked before accepting each proposal. Only compiler suggestions marked machine-applicable in reviewed expression categories are included. No lint suppressions or edition-2024 let chains are included. Changes are not yet applied.

- redundant_clone: 98
- explicit_auto_deref: 48
- map_unwrap_or: 38
- clone_on_copy: 1974
- manual_saturating_arithmetic: 1
- useless_conversion: 1
- let_and_return: 1
- default_constructed_unit_structs: 1
- semicolon_if_nothing_returned: 1
- mem_replace_option_with_none: 2
- needless_return: 10
- missing_const_for_thread_local: 1
- manual_contains: 5
- manual_is_multiple_of: 11
- redundant_closure: 2
- chunks_exact_to_as_chunks: 4
- unnecessary_map_or: 3
- map_or_identity: 3
- cloned_instead_of_copied: 1

Skipped proposals: {"stale":261}

Pass 450 applied 2,205 guarded expression fixes across 739 files, including 1,974 copies of Copy values. Other changes simplify membership checks, Option extraction, saturation, scalar defaults and const thread-local initialization. No lint allowances were added. Compilation and existing plugin behavior tests remain pending.
