# Selected Async Typecheck Classification

## Commands and Scope

The strict production graph/admission check completed with exit 0:

`bunx tsc --noEmit --strict --skipLibCheck --module ESNext --moduleResolution Bundler --target ES2022 --allowImportingTsExtensions --resolveJsonModule --types bun` with the build graph and closed admission TypeScript files as roots.

The same options with `🏗️build/🧪️tests/🕸️graph/🟦️.ts` as root completed with exit 2 and the diagnostics listed below. This command does not enable allowJs. It consequently requires authored declaration files for direct .mjs imports. It is a separate raw TypeScript probe; it does not prove the status of the registered Source gate. There is no captured full-test-file prechange strict baseline, so this report does not label the whole diagnostic set as a proved preexisting baseline or claim a full strict pass.

## Canonical Async Boundary

All actual command input consumers were searched across the repository, including aliased method calls. The changed command closure, target closure, native/generic command inputs, project inputs, project normalization, Cargo declaration, print declaration and inferProjectInputs callers await the canonical asynchronous result. The inferProjectInputs defining .d.mts now returns a Promise. No old synchronous alias or array compatibility branch remains.

One actual JS consumer outside Library was found by the second search: Repo test inference spread `native.relativeScriptInputs` synchronously. It now awaits it, and its writer wrapper forwards the same received original invocation into the inference context and script input cache. The selected Rust unit descriptor consumer likewise supplies its existing original invocation. These were production repairs, not waived diagnostics.

The raw test-file check reports no Promise misuse, unawaited-result member access, new dependencyInputs map mismatch, or diagnostic on the new resource/cancellation/compiler-companion laws. Missing .mjs declarations prevent this particular command from fully typing the implementation internals; repository-wide call search and executed production semantic tests are separate evidence, not a substitute for claiming that command passed.

## Exact Remaining Diagnostics

Paths in this table are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Columns identify the exact statement in the current frozen file, not a line range.

| File | Line:Column | Code | Finding and Relation |
| --- | --- | --- | --- |
| `🏃️execution/🧪️tests/📥️preparation/🟦️.ts` | 6:9 | TS2305 | Observation .d.mts omits captureSourcePreparationV1; existing preparation interface, not async closure result. |
| same | 6:36 | TS2305 | Observation .d.mts omits receiveSourcePreparationV1; same boundary. |
| same | 6:63 | TS2305 | Observation .d.mts omits observedFs; same boundary. |
| same | 25:39 | TS7016 | Defining inference .mjs has no declaration file. |
| same | 41:38 | TS7016 | Same declaration boundary at another existing import. |
| `🧪️tests/🕸️graph/🟦️.ts` | 13:140 | TS7016 | Repo Library .mjs declaration absent. |
| same | 209:69 | TS2339 | Negative fixture adds an alias property outside inferred literal project keys. |
| same | 215:9 | TS2339 | Nx argument .d.mts omits executeLiteralBuildArgumentsV1. |
| same | 278:233 | TS7016 | Repo Library .mjs declaration absent at another import. |
| same | 288:176 | TS2769 | JSON graph fixture widens version/selection literals before typed equality. |
| same | 293:106 | TS7016 | Repo test inference .mjs declaration absent. |
| same | 465:28 | TS7016 | Build inference .mjs declaration absent. |
| same | 503:483 | TS7016 | Same declaration boundary in the configured Source test; this batch awaits its command helper results. |
| same | 511:111 | TS2488 | Existing optional expectedSources fixture union needs explicit narrowing. |
| same | 521:200 | TS7053 | Existing string index into inferred literal configured fixture file keys. |
| same | 560:288 | TS2345 | Invalidation inventory fixture widens entry kind to string. |
| same | 561:174 | TS2322 | Negative invalidation mutations widen the same kind union. |
| same | 646:139 | TS2769 | Cargo invocation JSON fixture widens mode and selection literals. |
| same | 654:904 | TS2345 | Dataset policy fixture widens version to number. |
| same | 660:219 | TS2769 | Dataset fixture widens version to number. |
| same | 682:223 | TS7016 | Build inference declaration absent in resource law import; the actual helper call is awaited. |
| same | 692:145 | TS7016 | Same declaration boundary in selected owner contract test. |
| same | 707:1358 | TS2345 | Nested handoff fixture's empty calls array infers never. |
| same | 718:189 | TS7016 | Build inference declaration absent in catalog manifest test. |
| same | 733:121 | TS7016 | Same declaration boundary in registry authority test. |
| same | 746:141 | TS7016 | Same declaration boundary in catalog discovery control test. |

These remaining categories concern declaration completeness and other literal fixture typing. They were not suppressed with any, skip directives, unsafe casts around the new facts contract, or a permissive asynchronous compatibility declaration. This bounded batch does not claim the whole raw test-file strict command passes.

## Executed Semantic Evidence and Limits

The selected semantic suite completed 7 passed tests with 73 assertions. The added production law validates the per-owner facts map with independent Ajv and refuses the old array form. Positive command closure agrees with esbuild, fast-glob and TypeScript; first inner continuation stops cold imports, directory entries and cache-hit replay under the same original signal. The compiler companion law matches an independent TypeScript program's defining .d.mts/type closure while retaining actual runtime .mjs Source.

The new independent Nx resource-prerequisite oracle was attempted and refused `Original bounded Nx namespace required` before assertions. The existing independent Nx selected-prerequisite test has the same environment requirement. No synthetic namespace was installed to convert either into a pass; both await execution in the original owned Nx environment.

One synchronous parser/fact validation and the existing Cargo declaration traversal remain non-preemptible within a single unit. No unrelated selected-native snapshot quota was reused, no guessed graph parser grant was authored, and no whole-discovery bounded-cancellation claim follows from the inner-loop law.
