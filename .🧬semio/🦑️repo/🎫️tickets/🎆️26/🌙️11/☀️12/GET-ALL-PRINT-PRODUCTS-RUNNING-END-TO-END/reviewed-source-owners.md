# print:test-viz reviewed source owners

Final result: `repo run @semio-tech/print:test-viz --lang=en --output=static --skip-remote-cache` exits 0 (run r13).

## Root cause
The reviewed source owners are hand-authored in `command-boundaries.json`; `verifyPrintCommandBoundaries` compares them with the esbuild graph of each command entry. New modules were not listed: `pdf/representation`, `tectonic-template-compilation/resources` (plus `refusal`, already added by someone else), and `resources` imports `sharp` (declared in the typescript package.json).

## Further failures fixed
1. PDF operator capture rejected `undefined` args of `save` operators.
2. Chart inference test: Ajv2020 lacked the draft-07 meta-schema.
3. Real-browser check needed the pinned Chromium headless shell: print pipeline verification now calls `prepareRepositoryBrowserV1` (existing repo toolchain mechanism) before `verifyVizChartInferences`, covering `test`, `test-viz`, `test-viz-full`. No cache-policy edit (no chromium toolchain/target pattern exists).

## Files changed
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📜️chart-inference-result/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🟦️.ts`

## Notes
- `sharp` is imported directly by `resources/🟦️.ts`; AGENTS.md asks for external libs behind an interface (follow-up).
- Runs r11/r12 were cancelled by SIGINT while waiting for host capacity.
