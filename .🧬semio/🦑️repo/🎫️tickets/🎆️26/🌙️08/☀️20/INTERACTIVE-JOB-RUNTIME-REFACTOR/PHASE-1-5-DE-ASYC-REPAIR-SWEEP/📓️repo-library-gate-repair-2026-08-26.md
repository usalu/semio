# Repo Library Focused Gate

Date: 2026-08-26

## Fresh command

```sh
bun test '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern 'the real repo raises no discovery problems|🏛️ layering'
```

Result: exit 1 in 10.02 seconds. Five focused tests ran: two passed and three failed.

## Passed

- The layering baseline contains no already-clean file.
- Area-local policy exemptions are discovered and merged by the router.

## Failed

1. `the real repo raises no discovery problems` returns a large live ledger rather than `[]`. Representative categories are `package-implementation`, `manifest-without-marker`, `package-role-unresolved`, `packaging-violation`, and `ambiguous-lang-shape`. This aligns with the concurrently staged taxonomy-v7 change that declares every area `clean`, removes transitional area states, and makes package-boundary uncertainty a problem.
2. `the ratchet never allows a file to grow past its baseline` reports 12 implementation-reference breaches. The largest are the CAD/Draw path-projection fixture (448, baseline 0), root `📜️script.ts` (178, baseline 138), root `Cargo.toml` (93, baseline 0), taxonomy (15, baseline 7), and root `package.json` (15, baseline 0).
3. `no repo-wide or framework file exceeds what the baseline records` fails first on the 448-reference CAD/Draw projection fixture.

## Scope decision

These are real repository-taxonomy/layering failures, not Interactivity gate regressions and not safe to erase by growing the shrink-only baseline or restoring broad generated-file exemptions. The all-app discovery/launch gate remains independently executable and currently reports all 101 app contexts covered. Final workspace-wide test closure must nevertheless record these failures until the concurrent taxonomy-v7 migration either normalizes the reported package boundaries/references or supplies an independently justified owner-local policy design.

No baseline, taxonomy area state, `AGENTS.md`, or discovery allowlist was weakened in this packet.
