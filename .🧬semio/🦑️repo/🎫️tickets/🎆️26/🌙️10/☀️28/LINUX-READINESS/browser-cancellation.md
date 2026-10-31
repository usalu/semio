# Browser Test Cancellation

Demonstrator and Play each created a local interruption controller, while readiness and browser work omitted the caller Original's cancellation. Actual startToolV1 forwards the explicitly supplied signal; the previous calls supplied none.

Both bodies now combine the same issued invocation.control.signal with their existing local controller, pass the combined signal to readiness and the registered browser tool, and await that owned tool's exit. The tool layer owns cancellation; duplicate manual kills were removed. Existing progress, SIGINT exit130, SIGTERM exit143, and ordinary nonzero-exit failures remain. No browser configuration, GPU lane, service lifetime, argument selection, Original issuer, or deadline changed.

Schema-first neutral data covers caller cancellation during readiness/browser execution, SIGINT readiness, SIGTERM browser, successful completion and failed browser exit. The six cases execute both actual AST-transpiled bodies through explicit ports. Each case uses the framework's real envelope/withSignal APIs with empty capabilities, never a fabricated Original. Ajv and the owned validator independently validate observed stop/exit/failure/kill/signal outputs against the neutral expected contract; caller reason identity and listener cleanup are checked.

Causal red:2pass10fail. Actual current proof:12pass0fail. These are genuine-control Source-port tests, not a replacement for real browser acceptance. Registered through the existing Demonstrator compileclosure suite, with both source bodies declared in test and canonical-architecture inputs/resources. The updated project validates against the actual project schema. Four-root imported semantic validation is running. Read-only review found no blocker; three new test paths are115/117/127UTF16 units.

Production paths:
- ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🟦️.ts
- 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🟦️.ts
- ♻️mit-bestand/🧺️demonstrator/📋️project.json
- ♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts
- ♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🎭️e2e/🛑️cancellation/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}

Actual four-root semantic15686 finished1 with11diagnostics, all from missing generated specific catalog modules and their resulting Play callback inference. No owned modified-file diagnostics occurred, but this does not establish full semantic success. The authoritative @semio-tech/s-dev-composition:generate target declares these real outputs; submitted canonical generation under normal admission with NX_DAEMON=false to avoid competing for the shared daemon slot. No generated placeholder or type cast was added.

The same12actual-body laws now also pass on registered Node24.15.0 after adding explicit JSON import attributes to the new test module. The first native Node import was refused at missing JSON import attributes before any test body; this was corrected, not ignored. Actual Bun and Node controls both exercise all12cases. Real catalog generation26181 completed0; missing public package entry files were independently repaired and tested4/43 (catalog-public-exports.md). Current imported semantic retry21603 remains in its original normal-governor queue; no 0-diagnostic claim yet.

Current full imported semantic21603 TERMINAL0: all4requested roots and2441imported files have0diagnostics after real producer generation and public-package repair. This includes both modified production orchestrators, the shared neutral law implementation and its real default test registration. No full browser or monorepo green claim.
