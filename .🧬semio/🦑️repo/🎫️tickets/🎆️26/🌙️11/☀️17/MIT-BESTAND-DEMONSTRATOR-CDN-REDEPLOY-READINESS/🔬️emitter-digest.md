# Descriptor emitter: "Exact compiler input/artifact digest required"

Thrower: `add()` in `completedCargoClosureRosterV1`
(`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/📋️inventory/🟦️.ts`),
which rejects a non-absent row without a 64-hex sha.

## Candidates (from the producer in native-build/🟦️.ts)
- unit inputs: kind and sha come from one observation, so they are consistent (null kind maps to absent). Not the cause.
- artifacts: `observe.artifact()` returns null when an output is no longer a file; real failure, kept strict.
- compiler resource captures: `observe.sha256(file)` null when the capture vanished; real failure, kept strict.
- resource witnesses (`row.sha256`, `row.outputSha256`): null whenever a proc-macro/build script probed a path that did not exist at observation time (or a copy target that was absent). They were recorded as kind "file"/"directory" with a null sha, which can never be exact. This is a mis-classification: an observed-missing path is "absent". It is intermittent because it depends on whether a concurrently edited file exists at that moment.

## Fix
Resource witness rows now go through `witnessed()`: null digest -> kind `absent`, otherwise unchanged.
Failures that remain real now say what failed: `Exact compiler input/artifact digest required: <kind> <path> without a digest` (or malformed digest).

## Caveat
No reproduction of the throw occurred in my runs (attempt 1 passed the emitter; the emitter log of the current retry is clean), so the exact row was not captured. The witness classification is the code-derived cause; if the error recurs, the message now names the path and kind (artifact or compiler resource would then be the cause).

## Test
`.../📦️retention/🧪️tests/🟦️.ts`: "records a probed missing resource path as absent and still refuses an artifact without an exact digest" (passes; other tests in that file need the repo test harness env and fail identically without my change).
