# Rust Reference Baseline

Before this session's taxonomy moves, a read-only scan of `rg --files -g '*.rs'` across `🧰️framework`, `✏️s`, and `🌎️hub` found 20,082 Rust files and 52,020 literal references from `#[path]`, `include!`, `include_str!`, and `include_bytes!` text. Forty-six literal targets were absent when resolved relative to their containing file.

Ten absent targets are documentation placeholders (`#[path = "..."]`) in existing UI element files. The remaining 36 are snapshot and mutation fixture references in four draw artifact test adapters: structure, metadata, transform, and style under `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests`.

This is a textual diagnostic, not a compiler test: inline Rust module path semantics, conditional compilation, and comments require inspection. The baseline is retained as generated JSON during execution to compare new missing references after moves. No absence in this baseline is attributed to the current changes.
