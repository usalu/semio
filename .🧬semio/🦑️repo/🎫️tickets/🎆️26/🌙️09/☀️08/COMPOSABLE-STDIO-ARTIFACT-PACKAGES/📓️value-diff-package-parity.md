# Value Diff Package Parity

The package task did not edit the Value diff algebra or its associativity laws. Exact current-versus-HEAD byte comparisons are recorded below; they provide source evidence, not a baseline native test run.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/🦀️.rs`: tracked at HEAD=True, identical bytes=True, SHA-256 `207158fec5ea29e141b8e30db93eb7e3ace459760f38a4a77ce6a30ef39e35bb`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🧬️schema/🔺️diff/🧪️tests/🔬️unit/🦀️.rs`: tracked at HEAD=True, identical bytes=True, SHA-256 `b5a61e24ec90c63cc8244997326e6fb9000c75c62a367a02cdd0a83e721ccbec`.

The observed native failures remain real: positional additions retain stale indices after later key-only removals. A sound general repair requires the transport to carry ordering information or composition to receive its base. Clamping indices, subtracting every removal, or rewriting the laws would conceal the missing information. That algebra redesign is separate from package extraction and is not included in package acceptance. The raw aggregate remains failing; it must not be reported as passing.
