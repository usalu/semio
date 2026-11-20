# Descriptor Emitter Reuse Audit

Play's build metadata declares `job:describe-tool` unconditionally and requires it before every `describe` stage. Its script invokes `buildDescriptorEmitterV1`, which runs the host Cargo artifact build and publishes the executable together with the captured compilation inventory. The job is not conditional on committed descriptor existence.

A committed descriptor pair proves only the stored pair. Release `describeComponentStageV1` hashes the actual linked Wasm and actual emitter, keys its stage cache by those two digests, and on a miss runs the emitter against the component. It verifies the emitted pair and expected identity; `settleComponentDescriptorV1` then compares both emitted bytes against the committed pair. Model publication retains the actual emission and binds the component digest, descriptor digest and component provenance in its receipt. Skipping describe just because the files exist would remove this byte-to-descriptor witness. The particular host executable is the current implementation, not proof that an independently validated equivalent witness is impossible.

A warm stage cache already avoids repeated emission when component and emitter digests match, but even a cache hit first enters `withDescriptorEmitterTaskV1`. That reader checks the published executable digest, exact retained compilation inventory, completed compiler receipt, Cargo input reader, toolchain/source observations and rechecks. The release-mode Darwin emitter product currently exists in the products cache with executable and compilation inventory; mere presence does not establish its validity for a new run. The unconditional driver job rebuilds/publishes before any stage can consume it. Cargo warm artifacts and the optional native unit cache can reduce compilation, but do not remove job admission and source validation.

A possible future optimization is build-on-miss reuse of an already published mode/platform emitter, but only if the existing reader and compiler-input validation prove that its complete retained witness matches current inputs and remains valid through use. The pipeline must still run semantic comparison or a stage cache keyed by the actual component and validated emitter. No such conditional job path was demonstrated here, and no change, Cargo or build was run.

## Published Darwin witness on November 20

The release product at `.cache/b/p/m/browser/release/tools/descriptor-emitter/darwin` is complete and stale for the observed checkout. Its receipt records a successful, uncancelled compilation of 572 units at 00:03:32 local time. The repository’s pure retained-index decoder validated all 10,503 closure claims and 3,958 source-to-blob mappings. An independent bounded file audit matched the publication manifest’s 3,934 named entries against the physical tree, verified all 3,931 distinct blob digests and sizes, and found one exact compiler output for the published executable, 4,547,744 bytes with SHA-256 `f6b29bd7bc36291b259d459e41768461f78e80db46df5e0158d9415597d8ad95`.

Checking just the input paths marked current in the retained index found these results.

| Current observation | Matching | Changed |
| --- | ---: | ---: |
| Required absence | 6,545 | 0 |
| File digest | 3,358 | 13 |

The changed inputs include the root `Cargo.lock`, value authoring control, actor retained-turn context, action-bus admission, DSL notation/shape parsing/decoding, job context and construction, three store hydration sources, and the plugin reactor call. For example, the observed lock digest was `22f98e7c7bc60db11650426ae3825e774d721880714bdd5c8c7664d3b98a9fb1` and the retained claim was `f72ae976da4f3a0f844a529bf772102f803e4c46da2afca57321d066b26afad2`. The inspection checked file and descriptor identity around each physical digest. The product directory identity held throughout the artifact audit.

The retained workspace toolchain pin still matches `nightly-2026-07-20`. The repository’s installed `aarch64-apple-darwin` Rustup generation receipt matched its directory identity, sealed manifest digest, normalized logical seal, count of 4,873 entries and declared total bytes. The two actual binaries `bin/rustc` and `bin/cargo` also matched their seal byte counts and digests. This targeted toolchain check did not rehash the full toolchain tree or establish a compiler-generation binding to the older emission. The descriptor receipt records compiler-resource observations but carries no full Rustup seal field.

The emitter reader retains the published artifact and separately opens and rechecks the live Cargo input view. Its product validation compares the retained blobs with the historical record and confirms the selected Cargo home. The observed current-source changes show why that integrity check alone cannot authorize skipping the fresh owner job. The audit used only exact retained artifact and input paths, without starting Cargo, another describe job, or a release build.
