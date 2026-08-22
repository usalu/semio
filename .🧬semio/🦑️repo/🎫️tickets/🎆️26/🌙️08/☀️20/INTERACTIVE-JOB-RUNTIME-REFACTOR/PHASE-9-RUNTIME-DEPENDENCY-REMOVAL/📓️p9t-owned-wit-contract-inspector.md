# P9t Owned WIT Contract Inspector

## Outcome

The plugin-host schema-parity suite no longer declares a direct `wit-parser` test dependency. A narrow owned source inspector now checks the exact WIT constructs this contract needs: named interface/record/variant/world blocks, nested generic field types, sync versus async functions, results, and explicit world imports/exports.

## Preserved Invariants

- Every request-bearing effect has a matching async host import with identical payload fields.
- `respond` remains the documented emit-only exception and `http-request` maps to `http-fetch`.
- `spawn-job` reuses its effect payload despite carrying no request ID.
- `emit` carries the whole effect variant and the two emit doors stay synchronous and result-free.
- The package has exactly one world, `actor`, with the exact explicit import/export boundary.
- All seven actor exports are async while the three pure imports remain synchronous.
- All 24 fallible host imports return `result<_, _>`.

## Verified Gates

- Focused Rustfmt check: passed.
- Standalone owned-inspector test binary: 7/7 passed.
- Direct source/manifest dependency census: no `wit-parser` declaration remains in plugin-host.

## Boundary

The inspector deliberately validates the repository's contract subset rather than implementing unrelated WIT grammar. While the Wasmtime binding remains, its component macro continues to provide a separate full-schema parse during normal crate builds; the owned interpreter packet will replace that runtime boundary independently.
