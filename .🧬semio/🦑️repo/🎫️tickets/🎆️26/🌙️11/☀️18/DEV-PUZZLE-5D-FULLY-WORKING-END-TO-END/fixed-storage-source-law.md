# Fixed Storage Source Law

The fixed storage measurement port still calls `meter.admit(request, actual.signal)` inside the awaited `observeNativeStorageVerificationAwaitV1("allocatedUsage", ...)` wrapper, after checking the exact backing and before the second physical recheck. The source law expected a direct awaited call and failed after the observation wrapper was introduced. The assertion now checks the awaited wrapper and its meter call.

Verification: `repo tools exec --light --lang=en bun -- test ./🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/🟦️.ts`, with `SEMIO_TEST_ARTIFACT_DIR` set to this ticket’s generated directory: 7 passed, 0 failed, 59 assertions.
