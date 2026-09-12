# Publication Isolation Test Budget Repair

The independent Terra publication audit reproduced a red isolation test: its expensive case took 76.246 seconds against a hard 30-second Vitest timeout even with the long test level selected. The schema contract case passed. The timed-out run did not establish post-action live-output equality.

The coordinator read the full current test and confirmed five bounded child invocations: generation, check, preview and one Bun bundle for each of two variants. Each permits 20 seconds, before the native imports, esbuild comparison and before/after filesystem snapshots. A 30-second case budget is inconsistent with those admitted operations.

The repair gates only the expensive case with existing `atTestLevel(test, "long")` and gives it 120 seconds: up to 100 seconds of bounded children plus 20 seconds for the remaining oracle and snapshot work. The schema-only case remains fundamental. All original assertions, production functions, child deadlines, private-root isolation and final live snapshot equality remain intact. The existing long outer budget is 300 seconds.

## Verification

The coordinator executed the existing Vitest route with `SEMIO_TEST_LEVEL=fundamental` and an isolated ticket output root: one passed, one intentionally skipped, 5.05 seconds overall. Terra independently reran the repaired case with `SEMIO_TEST_LEVEL=long` and a fresh private ticket root: two passed in 47.67 seconds (37.56-second expensive case). That case reached its post-action live snapshot equality assertion, proving the tested production operations preserved the live staged files. The first red remains retained separately; it is not represented as a passing proof. No test was added merely to mirror a timeout constant.

## Exact Owned Changes

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🎮️playground-session/🟦️.ts`
- This retained Markdown report.
