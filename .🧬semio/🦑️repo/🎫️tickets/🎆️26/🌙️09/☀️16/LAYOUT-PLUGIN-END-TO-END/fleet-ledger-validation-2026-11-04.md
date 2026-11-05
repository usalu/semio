# Ledger Policy and Compact Root Validation

The initial normal main/model run failed with 12 passing tests, three failures and 831 assertions. Two model failures were existing drift: the machine total/weights did not derive from current capacities, and the English dependency table still said 8 GiB instead of 36 GiB. The third failure was a transient root-owned enforcement edit (`envelope` undefined), subsequently corrected by the root lane.

## Narrow Reconciliation

Only derived `defaultTotalBytes` and store weights changed in the machine policy. The unchanged declared capacities plus loose roots total **212400603136 bytes (197.8 GiB)**. Image capacities contribute 173.6 GiB; loose roots contribute 24.25 GiB. The third compact Rust root has maximumBytes=0 and is not counted twice. Exact integer largest-remainder allocation yields rust 4853, products 1213, tools 405, dependencies 1820, graph 303, diagnostics 101, nx 76, leases 3, loose 1226 basis points. The existing independent Python oracle remains unchanged.

English and German README/build-guide capacities, total and weight percentages now reflect the declared Rust 96 GiB and dependencies 36 GiB. No partition capacities, envelopes, caps, store floors, headroom or host floor changed.

## Actual Verification

- Strict six-root controller 23042 ended 2 with exactly two TS2769 errors in the new compact Rust fixture matchers, caused by JSON string widening. Production ledger, cache-gc runner, doctor caches and enforcement emitted no diagnostics. The shared root lane corrected these using symmetric exact equality operands, without casts or assertion weakening.
- Normal strict six-root successor **18803 ended 0**, with strict/noUncheckedIndexedAccess and normal repository-owned Bun/TypeScript. All six requested roots and their imported closure typecheck; log `🗑️generated/fleet-host-2026-11-04/ledger-scoped-strict-green.log`.
- Normal main/model successor 75366 ended 1: all 12 main tests passed, then policy loader rejected weights summing to 5804. My initial textual edit accidentally matched the earlier dedupe tools identifier and overwrote the rust store weight. The correction scopes edits to the stores array and independently checks a 10000 basis-point sum.
- Corrected normal full main/model successor **87704 ended 0: 15 tests passed, 863 assertions, two files, 285 ms**. This includes the unchanged independent Python allocation oracle, English/German documentation law, and compact Rust root law. Log `🗑️generated/fleet-host-2026-11-04/ledger-main-model-green2.log`.

## Changed Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🔣️policy.json`
- `🧰️framework/🛍️products/🦑️repo/README.md`
- `🧰️framework/🛍️products/🦑️repo/📓️build-guide.md`
- Shared root-authored matcher typing correction: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🧪️tests/🟦️.ts`

Fresh native20 and canonical activation remain held for the parent source-stability checkpoint. Earlier native88926 refused changed native authority before semantic compilation; it is not a native test result.
