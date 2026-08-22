# Owned Coordinator Validation

## Scope

The coordinator's five request-route schemas now use the repository-owned `OwnedSchema` boundary instead of importing Zod. The boundary implements only the coordinator's required schema algebra: objects, strings, minimum lengths, mailbox validation, booleans, literals, string enums, unknown values, arrays, defaults, nullability, and `safeParse` results.

## Changed Surfaces

- Added `🟦️validation.ts` and its focused `🧪️validation.test.ts` contract.
- Routed auth, diff, event, repo, and ticket request schemas through the owned factory.
- Removed `zod` from the coordinator manifest.
- Declared the coordinator's existing Vitest test-runner dependency in `devDependencies`; this fixes the parity gate without adding a new workspace identity.

## Verification

- Focused TypeScript compilation of `🟦️validation.ts`: passed.
- Bun route build of all five migrated API entry points: passed; 6 modules bundled into 5 entries in 7 ms.
- Differential corpus against Zod before removal: `[DEBUG] owned coordinator validator parity: 10/10`.
- `bun ./📜️script.ts test quick` from the coordinator package: 1 file, 7 tests passed in 109 ms.
- `bun install`: passed; 2,012 installs checked across 2,064 packages.
- `bun ./📜️script.ts verify dependencies`: passed at 164/238 identities, 74 removed.
- `bun ./📜️script.ts verify dependencies parity js --format json`: passed with zero undeclared imports. The optional historical unowned-row inventory remains outside this packet's gate unless `--no-unowned-rows` is requested.

## Result

Zod is absent from coordinator source and manifests. Successful parses preserve route defaults and strip unknown object keys; invalid inputs remain rejected before any route effect runs.
