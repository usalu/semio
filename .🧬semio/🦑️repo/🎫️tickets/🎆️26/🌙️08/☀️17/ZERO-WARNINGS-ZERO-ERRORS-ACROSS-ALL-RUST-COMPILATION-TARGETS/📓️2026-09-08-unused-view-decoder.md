# Unused View Decoder

Native582 reported decode_view_state as dead code in the plugin framework. Pass596 removes this private, unused fallback decoder and its duplicated docstring. A Rust source search across the framework and S found no callers. Current view-state handling calls decode_wire_serialized<ViewModel> or the explicit JSON decoding paths; those call sites and their error behavior are unchanged. The ViewModel import remains needed by other functions.

Changed file:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

Fresh parse, compiler and runtime validation remain pending. Native582's earlier warning remains in its log because that process had already compiled the original helper.

Pass597: plugin-root edition-2021 parse exit 0, source unchanged true; the updated ticket runner passed Bun TypeScript parse validation. The existing Flow selected-copy language-neutral/source-contract checks passed 15 assertions against their current files. These are not Rust runtime test executions. Fresh Cargo verification remains pending.
