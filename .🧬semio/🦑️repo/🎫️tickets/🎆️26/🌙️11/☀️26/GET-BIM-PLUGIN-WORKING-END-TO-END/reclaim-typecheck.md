# Reclaim Test Typecheck Fix

## Problem

`repo typecheck` reported `TS2769` at `reclaim.test.ts` line 63, column 39: `expect(edit?.reason ?? null).toBe(row.reason)`. `row.reason` came from the JSON fixture `reclaim.json`, so TypeScript saw it as `string | null`, while `ConcurrentEditReasonV1` is a closed union.

## Source of truth

- `ConcurrentEditReasonV1` is declared in `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts`.
- Its schema is `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json` (`definitions.Reason.enum`, 12 values).
- The runtime table `churnProtocolV1().reasons.en` lists every reason, and the production code validates reasons the same way (`Object.hasOwn`).

## Change

`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🔁️failure-churn/🧪️tests/reclaim.test.ts`:

- Imported `churnProtocolV1` and `ConcurrentEditReasonV1` from the churn module.
- Added `checkedReasonV1`, which maps `null` to `null`, throws on an unknown reason, and otherwise narrows the string to `ConcurrentEditReasonV1`.
- The test loop now iterates `fixture.cases.map(row => ({ ...row, reason: checkedReasonV1(row.reason) }))`.

The type was not loosened and the schema was not changed.

## Verification

- `repo typecheck <reclaim.test.ts>`: passed, 0 errors.
- `repo test-file <reclaim.test.ts>`: 8 pass, 0 fail, 52 expect() calls.
