# Array prefix vectors

The Array cursor staged fixture and schema define eight language-neutral bitwise prefix cases, including zero body, exact and short work, page ceiling, zero-width elements and a terminal suffix. The ticket's `🧪array-prefix-oracle.mjs` validates schema with Ajv and independently computes the count and receipts with SQLite. The normal managed Bun command completed with exit 0 and validated all eight cases. This validates the fixture model only; the Rust cursor has not been executed by this audit.
