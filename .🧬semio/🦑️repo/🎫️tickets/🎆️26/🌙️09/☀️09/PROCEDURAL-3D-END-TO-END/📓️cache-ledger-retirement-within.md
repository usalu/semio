# Cache Ledger Retirement `within`

A peer added `"within": "fleet-state"` to the `f/cache-server` retirement in `⚡️caching/💾️storage/🧾️ledger/🔣️policy.json` without schema or code support. Every repo command that read the ledger failed with `Closed cache ledger required: /retirements/0/within is not allowed`.

## Fix
- Schema `🧾️ledger/🧬️schema/🔣️.json`: optional `within` (area id) on a retirement.
- `🧾️ledger/🟦️.ts`: a retirement with `within` nests inside that control/tree area of the same root (strictly inside its directory) instead of claiming its own disjoint home; anything else is refused. The classifier checks retirements before areas so the nested retired directory is reported as retired.
- Tests `🧾️ledger/🧪️tests/🟦️.ts`: four refusal cases plus nested classification. 14/14 pass.
