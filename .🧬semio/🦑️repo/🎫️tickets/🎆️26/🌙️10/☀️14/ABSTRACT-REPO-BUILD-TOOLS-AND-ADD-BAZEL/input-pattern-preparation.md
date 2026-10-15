# Operation-Local Source Pattern Preparation

The actual 180-second whole-producer prefix profile supplied the measured candidate: `inferProjectInputs → compactSourceInputs → buildInputPathMatchesV1 → compile`, with 6,016 self samples (5.57%). This supports investigating repeated pattern compilation in that exact path, not an ALL runtime or speedup prediction.

## Bounded Change

The existing compiler was extracted into a private function without changing its grammar or the exported matcher's behavior. Each `compactSourceInputs` call now owns a local Map keyed by the exact normalized exclusion string. It compiles a pattern only when a still-unexcluded path needs it and reuses that non-global regex for subsequent paths in the same operation. The Map is discarded on return; no process/global cache or public API was added.

The original exclusion order, duplicate semantics, project/workspace coordinate normalization, `!^` filtering and lazy short circuit remain intact. A malformed later pattern still stays uncompiled when earlier exclusions removed every exact candidate; a reachable malformed pattern still throws. Candidate sorting, residual inputs, immutable closure publication and helper inputs are unchanged. No Source observation, authority, cancellation, progress, retention or quota check was changed.

## TDD and Independent Oracle

The new language-neutral fixture supplies a closed case schema, current inputs/expected outputs, compilation counts, matching vectors and malformed-pattern errors. Independent Ajv validates cases, and minimatch checks brace/extglob/class/dot/Unicode and normalized negative selection. A synchronous try/finally RegExp constructor proxy counts only the actual pure compaction call; it is restored before the test yields and is not production/profile instrumentation.

- RED: one failing test / 20 assertions / 600 ms. Output and independent matcher assertions passed before the count assertion: the first operation constructed **16** regexes instead of the three distinct used normalized patterns. Generated `input-pattern-preparation-red.txt` retains the actual failure.
- GREEN: one passing test / **38 assertions / 87 ms**, including repeated calls that each compile three patterns independently, lazy unreachable errors, original-array return with no exact candidates and reachable malformed errors. Generated `input-pattern-preparation-green.txt` retains the actual run.

The focused vectors deliberately exclude all exact candidates before closure publication, so this pure regression acquires no graph writer or fabricated storage capability. It does not claim a new immutable graph, complete ALL execution or published-closure runtime verification. The existing graph suite imports the law, making it part of its established registered test entry; no extra product command, script API or launch entry is required.

## Owned Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🟨️.mjs` — reviewed implementation SHA `5e4df28f13bb89d58eb66b5722d107b5ab64f43a36dd4f32dd3781d5dc93fe4c`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🟦️.ts`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🧪️tests/🧫️fixtures/🔣️.json`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🟦️.ts` — one registration import; concurrent membership-test registration preserved.

Missing paths were appended to the retained file inventory without replacing existing entries. No full producer, extra CPU profile or product compiler ran for this change. End-to-end benefit remains unmeasured.
