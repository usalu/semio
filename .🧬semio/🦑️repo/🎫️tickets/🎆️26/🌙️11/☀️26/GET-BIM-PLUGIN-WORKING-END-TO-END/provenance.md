# Provenance receipt: "path must be a string"

## Root cause
`native-build/🟦️.ts` (compiler resource capture loop) copied `observation.producer` / `observation.caller` raw from the captured JSON (`?? null`). A capture cut short by a killed rustc (SIGTERM) or a failed unit can hold a partial identity (e.g. `{manifest}` without `source`, or no `crate` for a caller). The schema (`🧾️receipt/🧬️schema/🔣️.json`) requires `producer = {manifest, source}` and `caller = {manifest, crate, source}` or null, so `CargoProvenanceStreamV1.compilerResource` hit `ref(undefined)` and refused with "path must be a string". The row was invalid at its source; the schema already models it correctly (null is the partial form).

## Fix
- New `cargoCompilerIdentityV1(value, caller)` in `🧾️receipt/🟦️.ts`: returns the exact schema shape (dropping extra keys) or null when any required field is missing/non-string/empty.
- The capture loop in `native-build/🟦️.ts` uses it for producer and caller; `bind()` already returns null for such identities, so the unit bindings stay consistent.
- The receipt failure no longer arises from partial captures, so it cannot mask the real rustc failures from the same build.

## Test
`🧾️receipt/🧪️tests/🟦️.ts`: new test "a partial compiler resource capture normalizes to null identities and still encodes to a schema-valid receipt" (normalizer cases, stream encode, decode, Ajv validation against the schema).
`repo test-file` on that file: 28 pass, 0 fail.
`repo typecheck` on touched files: my files clean; one unrelated pre-existing error in `🧩️component-pass/🔁️failure-churn/🧪️tests/reclaim.test.ts(63,39)` (ConcurrentEditReasonV1), not touched by me.

## Files (under `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/`)
- `🟦️.ts`
- `🧾️receipt/🟦️.ts`
- `🧾️receipt/🧪️tests/🟦️.ts`
