# Actual OS Dev CPU Profile

Core's single uninstrumented normalizer child exited successfully under its original 60-second envelope. It normalized the actual `@semio-tech/framework-os-dev` declaration in 5,346.501 ms, producing 52 targets and 3,534 captured facts. Manifest SHA is `297a325c73cd28ea4a3127d3de1c45ad689a0fca5d6110d6c530ccce80fcc58c`; fact digest is `ad75de99c95a18e673a1bbe5f19a8c791c2ac87199cc6d09be77b8c5f84c7b0e`. No Map wrappers or counters instrumented this run. No FactStore, graph producer, additional profile or product task was launched by this aggregation.

The baseline profile was preserved intact as `dev-before.cpuprofile` and `dev-before-result.json` before Core's separately authorized comparison. Its SHA is `a73f0de94013117a53ba02057cd33214aff6d282daa843c61d3f38c22a32a520`. Its 8,882 nodes and 2,297 samples cover 5,756.432 ms including startup. The normalizer's epoch interval contains 2,149 samples. A retained Bun/SQLite recursive query independently reproduces every self and inclusive frame sample count in both whole-process and normalizer windows: four exact comparisons pass. The independent query uses raw sample deltas, SQL window accumulation and recursive parent traversal rather than the aggregation's JavaScript stack walker.

## Self Samples

These rows count only the sampled leaf. Percentages use the 2,149 samples inside the normalizer's epoch interval.

| Leaf | Samples | Share |
| --- | ---: | ---: |
| Native/unattributed `readFileSync` | 303 | 14.1% |
| Native/unattributed `lstatSync` | 185 | 8.6% |
| Native/unattributed `stringify` | 179 | 8.3% |
| Unattributed `anonymous` | 123 | 5.7% |
| `jsonSchemaSubsetSnapshot`, validator line 107 | 115 | 5.4% |
| `prepare`, validator line 127 | 74 | 3.4% |
| Native/unattributed `URL` | 60 | 2.8% |
| `ready`, validator line 152 | 56 | 2.6% |

The largest two exact repeated full stacks tie native leaves to production source: 159 samples follow `relativeScriptInputs`' compute callback at inference line 529 → observed file read at observation line 283 → native `readFileSync`; 146 follow the same callback → `readPhysicalSource` line 56 → `requirePhysicalSourceV1` line 47 → observed `probe` line 261 → native `lstatSync`. These ties identify actual observed filesystem work; they do not distinguish kernel I/O wait from native CPU execution.

## Inclusive Stacks

Inclusive rows count a frame whenever it occurs in a sampled stack; recursion of the same exact frame is counted once per sample. Different rows overlap and must not be summed.

| Frame | Samples | Share |
| --- | ---: | ---: |
| Command-source compute callback, inference line 529, column 44 | 1,259 | 58.6% |
| Async observed memo frame, observation line 113, column 88 | 926 | 43.1% |
| Per-file command import compute callback, inference line 539, column 122 | 745 | 34.7% |
| `observedSourceAnalysisV1`, observation line 194 | 583 | 27.1% |
| `sourceImports`, caching inference line 16 | 527 | 24.5% |
| TypeScript `createSourceFile`, installed implementation line 33019 | 443 | 20.6% |
| `validateJsonSchemaSubset`, validator line 206 | 277 | 12.9% |
| `compileJsonSchemaSubset`, validator line 123 | 273 | 12.7% |
| Observed filesystem `probe`, observation line 261 | 272 | 12.7% |
| Observed `readFileSync`, observation line 283 | 253 | 11.8% |
| `retainObservationsV1`, observation line 90 | 167 | 7.8% |
| `projectObservationHoldsV1`, observation line 55 | 144 | 6.7% |

The actual parser creates a TypeScript source file and visits its syntax tree in `sourceImports`. The schema validator's `compileJsonSchemaSubset` snapshots the supplied schema/root/documents and prepares check plans; its caller is `validateJsonSchemaSubset`. The profile establishes both paths occur materially in this owner run. It does not establish their call frequency, cross-owner reuse eligibility or safe memo lifetimes, and therefore does not justify a production optimization by itself.

All 277 normalizer samples containing `validateJsonSchemaSubset` have the same direct caller: `readScriptPolicy` in process routing invocation at line 23, column 33. Their retained caller chain is `advanceScriptInvocation` line 42, column 184 → `checkScriptInvocation` line 29, column 67 → `readScriptPolicy` → `validateJsonSchemaSubset` → schema compilation. No other schema-validation caller appears in that sampled subset. This is the specific invocation-policy contract to inspect; it is not evidence for caching arbitrary mutable schemas or weakening original checks. The exact grouped stacks are retained in `dev-schema-callers.json` and reproducibly in the aggregation's `schemaCallers` property.

Source locations use one-based call-frame declaration lines and columns, preserving distinct same-line frames. Relevant files are:

- [Command-source traversal](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs:524).
- [Observed memo and fact replay](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:113).
- [Observed physical reads](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:261).
- [Source parser](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📥️inference/🟨️.mjs:16).
- [Schema snapshot and preparation](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts:107).

## Timing And Attribution Limits

Raw deltas sum to 5,755.574 ms, leaving 0.858 ms between the final sample and the profile endpoint. Deltas range from 0.902 ms to 590.487 ms despite the documented default 1 ms sampling interval. Assigning a delta to its subsequent leaf is only an elapsed-interval approximation; the largest gap can substantially distort weighted rankings. Sample counts are reported separately from these weights in the generated aggregation.

The normalizer's process-clock elapsed duration is 5,346.501 ms; its `Date.now` epoch boundaries span 5,347 ms and are used to align samples, with millisecond precision. Profile startup time is excluded from the normalizer window. Whole-profile `positionTicks` are retained separately because they contain no per-tick timestamps; they are not presented as normalizer-only line hits.

Self-leaf categories in the normalizer window are 1,171 empty-URL/unattributed-or-native samples (54.5%), 493 installed dependency/cache-source samples (22.9%), 476 repository-source samples (22.2%) and nine runtime-module samples (0.4%). No explicit idle or GC frame labels are present. This does not prove idle/GC absence or allow unattributed samples to be relabeled as GC/native CPU. Async scheduling can also truncate inclusive ancestry.

This is one actual owner's pure normalization, not a profile of the interrupted ALL producer or all 1,929 projects. It identifies measured hot paths for a bounded next investigation. It neither proves the full producer's timeout cause nor accepts an ALL graph, exhaustive plan or Stdio result.

## Authorized Preparation Comparison

After focused policy contract tests passed and independent source review was clean, Core ran one separately authorized comparison through the same uninstrumented child and original 60-second control. It exited 0. The profile is now `dev.cpuprofile` with SHA `a1396ddb8afbbd1ac2e428943a506a4644d839758aa8742811f6509d1f843233`; result is `dev-result.json`. The actual owner, manifest SHA, 52 targets and 3,534 fact count are unchanged. Fact digest changes with the legitimate implementation Source change; identical authority bytes are not claimed.

| Normalizer metric | Before | After |
| --- | ---: | ---: |
| Sample count | 2,149 | 2,173 |
| `validateJsonSchemaSubset` inclusive samples | 277 | 0 |
| `compileJsonSchemaSubset` inclusive samples | 273 | 0 |
| Schema snapshot inclusive samples | 124 | 0 |
| Schema `prepare` inclusive samples | 119 | 0 |
| Actual value-validator `ready` inclusive samples | 58 | 47 |
| Exact-key guard inclusive samples | 35 | 54 |
| Observed process-clock elapsed ms | 5,346.501 | 4,819.102 |
| Largest sampling gap ms | 590.487 | 140.506 |

No samples of repeated generic schema preparation remain in the after normalizer interval, while actual value validation and exact-key guards remain sampled. Combined with the exact architectural witness and mutation/control tests, this supports the intended narrow change. A zero sample count alone would not prove absence of calls. The private schema preparation executes during module initialization, outside the normalizer interval; no policy read is omitted.

The observed elapsed difference is 527.399 ms in this single pair. It is descriptive, not a measured CPU saving, stable speed estimate or full ALL improvement. Startup and large sampling gaps differ, native/wait attribution remains incomplete, and all owner/time-budget conclusions require the actual next producer. The after process spans 5,121.720 ms including startup and has 2,323 total samples. Independent SQLite again matches every self/inclusive frame count in both windows: four more exact comparisons pass.

Reproduction input is `cpu/📜️script.ts`. Generated evidence is under `🗑️generated/cpu`: baseline `dev-before.cpuprofile`, `dev-before-result.json`, `dev-before-aggregation.json` and `dev-before-aggregation.txt`; after `dev.cpuprofile`, `dev-result.json`, `dev-after-aggregation.json` and `dev-after-aggregation.txt`. The first baseline aggregation and caller map remain retained as `dev-aggregation.json`, `dev-aggregation.txt` and `dev-schema-callers.json`. Controller logs are `🗑️generated/cpu-normalizer-dev.log` and `cpu-normalizer-dev-after.log`. The aggregation performs no normalization or storage mutation.
