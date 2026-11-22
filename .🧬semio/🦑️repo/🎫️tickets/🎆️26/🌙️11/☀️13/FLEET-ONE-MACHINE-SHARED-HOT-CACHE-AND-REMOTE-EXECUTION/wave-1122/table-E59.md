# E59 table: file -> before / after (pass/fail), via `repo test-file`

| file (relative to 🧰️framework) | before | after | change |
|---|---|---|---|
| process/🧪️testing/🧩️extracted-function/🧪️tests/🟦️.ts | 7/0 | 10/0 | ratchet law (2 tests) + multi-declarator harness test |
| …/🌐️platform/⏳️wait/🧪️tests/🟦️.ts | 5/0 | 5/0 | 2 evaluateExtractedFunctionV1 -> loadProductFunctionsV1 (real module) |
| …/🌐️platform/🥾️boot/🩹️recovery/🧪️tests/🔗️wiring/🟦️.ts | 3/0 | 3/0 | text extraction -> loadProductFunctionsSyncV1, 0 offenders left |
| …/🧹️cleanup/🧪️tests/🧩️model/fixture.ts | (via 🎛️command 6/0, 📥️operation 11/1) | 6/0, 12/0 | model fixture -> loadProductFunctionsV1; 📥️operation test updated to the product's observeNativeStorageVerificationAwaitV1 wrapper (product change) |
| build/📥️inputs/💾️storage/🧪️tests/🤝️shared-writer/🟦️.ts | 4/0 | 4/0 | text extraction -> loadProductFunctionsV1, 0 offenders left |
| build/📥️source/🔏️closure/🗓️scheduling/🧪️tests/🟦️.ts | 17/1 | 18/0 | 🔬️test-file classified as diagnostics prefix; policy.json + nx.json mirror re-sorted |
| build/…/🗓️scheduling/🧪️tests/🧭️boundary/🟦️.ts | 3/2 | see result | four members classified, resident policy port |
