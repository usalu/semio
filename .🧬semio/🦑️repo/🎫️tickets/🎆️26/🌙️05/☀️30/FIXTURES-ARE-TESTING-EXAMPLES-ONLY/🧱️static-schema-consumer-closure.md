# Static Schema Consumer Closure

Inspected direct TypeScript import/new URL and Rust include schema paths across 7501 framework files.

- `🧰️framework/🛍️products/💻️os/🧪️tests/🏷️schema-vocabulary/🟦️.ts:37` → `../🧬️schema-oracle/🔣️.json`

The listed schema-vocabulary URL is an exclusion comparison sentinel and is never read or imported. All actual directly consumed relative schema paths resolve. Rust escaped Unicode paths were excluded from the literal filesystem comparison because the Rust compiler decodes them before include resolution.
