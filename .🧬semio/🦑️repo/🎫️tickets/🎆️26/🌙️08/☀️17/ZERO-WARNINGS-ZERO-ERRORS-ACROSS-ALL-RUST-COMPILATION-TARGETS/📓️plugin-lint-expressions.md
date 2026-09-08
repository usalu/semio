# Plugin Expression Lint Cleanup

Pass 444 inspected strict WASI 428 diagnostics and prepared 261 non-overlapping expression fixes in 122 files. Compiler byte spans, full source lines, and Unicode columns are checked before accepting each proposal. Only compiler suggestions marked machine-applicable in reviewed expression categories are included. No lint suppressions or edition-2024 let chains are included. Changes are not yet applied.

- map_unwrap_or: 62
- redundant_clone: 38
- needless_borrow: 16
- explicit_auto_deref: 31
- manual_is_multiple_of: 13
- unnecessary_map_or: 7
- redundant_closure: 4
- chunks_exact_to_as_chunks: 7
- let_unit_value: 2
- clone_on_copy: 11
- unnecessary_to_owned: 3
- while_let_on_iterator: 1
- useless_conversion: 2
- needless_return: 2
- unnecessary_lazy_evaluations: 3
- iter_overeager_cloned: 1
- manual_div_ceil: 3
- semicolon_if_nothing_returned: 7
- unnecessary_first_then_check: 1
- needless_lifetimes: 8
- needless_bool: 1
- useless_vec: 1
- needless_option_as_deref: 3
- double_parens: 33
- unnecessary_mut_passed: 1

Skipped proposals: []

Pass 445 applied all 261 guarded expression fixes across 122 files. Inspection confirmed the eagerly evaluated bounds arithmetic uses f64, so empty paths still return None without integer overflow. Compilation and existing plugin behavior tests remain pending.

Pass 447 replaced 35 no-destructor Option take/drop expressions with direct slot clearing and removed one redundant local drop after its emptiness check. Draw helpers accept slices/str. CAD checked_add alone enforces the i32 generation ceiling; Puzzle terminal/resume branches keep short-circuit ordering. Three snapshot docstrings now attach to their actual exports; other doc gaps and two ineffective lint attributes were removed. Compiler and runtime follow-up remains pending.
