# Portable Cohort Window Review

The proposed boundary is appropriate: strict zero-window checkpoint before each cohort; default500ms window around the managed source-read batch; strict zero-window final checkpoint in settlement, including body failure. Keep both arbitrary failure values by explicit failed booleans, not truthiness. Preserve a single failure by identity; when both body and final checkpoint fail, AggregateError.errors must be [bodyFailure, finalFailure] in that order, including thrown undefined. The strict precheck fails before opening the cohort and needs no invented body completion.

## Current Export Surface

The function is exported by physical process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🟦️.ts:57. Its only verified clients are the adjacent owned tests; cleanup imports related roster functions internally. There is currently no public package roster subpath. @semio-tech/process/artifacts/native-build/storage/image resolves to process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🟦️.ts, which exports the physical image module; that module currently does not re-export the window API. Explicitly re-exporting only withNativeRecheckWindowV1 from the existing image module would expose it through the existing facade without a new package route. Do not assume the public export already exists.

## Existing Test Placement

The graph suite already owns graph/🔄️settlement/🟦️.ts, ./🔣️.json and ./🧬️schema/🔣️.json. Its test uses TypeScript createSourceFile to extract the actual authored function, transpileModule, controlled ports, Ajv corpus admission and an independent SQLite outcome oracle. This is the closest existing pattern for testing the proposed small actual cohort-settlement function, rather than copying its behavior into a mirror. Existing roster tests separately cover physical gate expiry, single-flight, cheap-check failure and context restoration. Extend graph settlement fixtures/schema and execute the actual new function via extraction; retain existing settlement cases.

## Neutral Expected Sequences

S denotes checkpoint with observed window0; W+ enters default500; B executes the actual batch callback; W- restores prior context; F is final checkpoint with observed window0. Every row must also assert exact callback counts, no post-failure acceptance and context restored to an outer sentinel window.

| Case | Ordered Events | Result |
| --- | --- | --- |
| success | S,W+,B,W-,F | exact body value |
| body error | S,W+,B(error A),W-,F | exact A |
| final error | S,W+,B,W-,F(error Z) | exact Z |
| both errors | S,W+,B(error A),W-,F(error Z) | AggregateError.errors=[A,Z] |
| cancellation in body | S,W+,B(cancel A),W-,F | exact A if final succeeds; aggregate if final refuses cancelled control |
| precheck refusal | S(error A) | exact A; zero B/F |
| expiry within body | S,W+,B(check at0,100,501),W-,F | real gate full at0 and501; cheap check all3; F strict regardless of age |

Use undefined and frozen object variants for errors, plus an ambient positive window to ensure both S/F explicitly override it. Cancellation must not invent fresh authority or suppress the final attempted check. Expiry requires the actual existing gate with a controlled clock, not sleeping. A final check outside the inner window but inside an outer positive window is insufficient unless explicitly zero.

No tests or commands were run and no Source changed. This cannot alter window context of already-running82722. Parent will author after architecture66599 terminal as specified.
