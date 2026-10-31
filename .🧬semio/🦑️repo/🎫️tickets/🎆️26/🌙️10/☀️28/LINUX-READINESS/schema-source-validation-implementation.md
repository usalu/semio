# Schema Source Validation

The existing normative domain is framework/schema/source/closure: SchemaSourceInput carries id, path, baseUri and raw source; SchemaReferenceRequest declares a finite root/resources authority. The new public compileJsonSchemaSourceSubset(request, control) compiles Draft-07 subset validation through that domain's authenticated reference edges. It does not inject IDs, copy definitions, discover implicit resources or introduce another URI resolver.

The Source closure supplies each origin /$ref position and exact validated target pointer. The existing subset compiler now has one shared immutable plan engine; source-aware callers bind those pointers directly to captured JSON nodes. Body-only/local-definition callers retain their existing URI/ID engine and snapshots. Captured dialect is retained across cooperative yields. The public validator package explicitly exports the new function and its existing owned request/control types.

Production engine: 🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts; accepted frozen SHA256 64dfd4d58940ea498bc93a549e982b638f3bb70b393cd3a94631528a9ac6636a, 22,820 bytes. No engine change followed that checkpoint.

## Actual Gates

| Owner | Actual Result | Scope |
|---|---|---|
| 586575 | 1/null, 07:26:27Z | 14 existing URI laws passed; 38 new Source laws exposed the missing API. The initial physical Project case also had a wrong test root, corrected before acceptance. |
| 600569 | 0/null, 07:28:14Z | 53 tests, 138 assertions: existing URI laws, 38 Draft-07 Source cases, actual Project/Producer source documents and Ajv observations. |
| 604467 | 0/null, 07:32:45Z | Final 54 tests, 142 assertions; captured-input mutation during yielding and explicit non-Draft07 subset refusal added. |
| 606225 | 0/null, 07:35:10Z | Existing full body-only proof executed unchanged in Bun and bundled Node with Ajv: 112 instance cases, five value comparisons, 14 compilation refusals. Bun wrapper reports one test/one assertion; internal assertions remain node:assert. |
| Root 604793 | Parent reports 0/null | Actual physical-URI Ajv, new first-party Source compiler and canonical 188-cohort aggregate accepted. |

The Source laws preserve missing/ambiguous resources, duplicate members/identities, escaped/percent pointers, nested IDs, changed inputs, cancellation and finite source-byte refusal. Instance assertions retain the existing subset's scope; this is not a claim of complete Draft-07 or 2020-12 validation.

## Whole-Project Consumers

A read-only scan examined 18,367 framework TS/MJS/CJS/Rust files. Production Project consumers in graph/admission, build/invocation and task-model validate local definitions and require no retrieval change. The public JSON facade still exports its owner's unchanged document; source-aware compilation must carry its true physical retrieval coordinates.

Existing compileSchemaScopeExport now registers unchanged dependency/main documents at their actual file URLs before compiling the chosen whole document/export. All six bare whole-Project Ajv consumer files now use this existing oracle with the actual Project and binding schema sources. Their factories use literal new URL references so the command closure retains both JSON resources. No copied Producer definition or test-corpus schema was created.

| Consumer Owner | Actual Acceptance |
|---|---|
| build-surfaces 604476 | 15 tests, 21,789 assertions, 0/null |
| selected admission 604485 | 20 tests, 526 assertions, 0/null |
| portable model 606230 | Seven tests, 79 assertions, 0/null |
| portable CLI 606242 | 15 tests, 273 assertions, 0/null |
| invocation 607856 | Nine tests, 318 assertions, 0/null |
| Course shared-vectors | Narrow whole-Project constructor migrated; Course retains its own genuine broader acceptance. |

These five semantic consumer receipts preceded the final literal-URL resource annotation; that change preserves their compiled schemas/assertions and makes resource observation explicit. Current whole-goal canonical acceptance remains its owner's work.

Readiness also validates a complete Project through the first-party body-only API. Pure metadata reproduction 620297 ended 1/null at the exact unresolved Producer reference. Its law now awaits source-aware compilation of both physical documents, retaining every original readiness assertion. The genuine full-file owner 618586 remains normally queued; its historical label contains red but its body had not executed when the caller was corrected. No duplicate or capacity cancellation was issued.

## Native Scope

The existing Source-closure domain currently has no Rust implementation. Native OwnedJsonSchemaValidator still accepts body-only documents keyed by declared IDs. Neither the new TS acceptance nor physical Ajv registration certifies native retrieval semantics. A full Rust implementation of the same existing Source contract/corpus and native integration remain explicitly unaccepted; no smaller alternate resolver or filename alias was added.


## Final Readiness Caller

The earlier queued readiness618586 ended1/null08:01:14Z before SDK/test execution at shared frontend TDZ Cannot access exact before initialization. Root identified and repaired that separate current owner; no guard was changed here. A fresh justified normal file637436 ended0/null08:15:50.762Z:19tests234assertions, including the complete Project's source-aware validation and all original status, redirect, foreign-host, cancellation, continuous-target marker, variable/default-port and declaration laws.

All local loader URLs resolve to their actual current Project and binding files. The six Ajv consumer factories and this first-party readiness consumer retain both schema resources through literal new URL references. Course retains its own full shared-vectors acceptance. The production validator engine remains frozen at the accepted64df hash; native Source-closure implementation remains unaccepted.

