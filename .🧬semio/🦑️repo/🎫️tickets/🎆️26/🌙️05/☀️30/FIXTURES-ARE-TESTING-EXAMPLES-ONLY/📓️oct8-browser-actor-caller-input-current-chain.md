# Current Browser Actor Executable Caller Input Chain

This bounded read-only trace follows the actual current Store and Plugin child sources; it replaces older coordination terminology that is absent from current sources. It does not itself discharge the computed-import graph or prove any current published artifact.

| Stage | Current Owner And Actual Behavior |
| --- | --- |
| Selection | Store DocumentExecutionTargetLease owns private live fields, browser-actor grant and session/open authority; actor kind must be closed-browser-actor. |
| Acquisition | Store lease activateBrowserActor requests the Hub-selected browser-actor asset for the exact document/intent through current timeout/retry/cancellation ownership. |
| Verification | readExecutionTargetBody bounds the exact actor byte length; executionTargetSha256Hex must equal the lease actor.sha256 before transfer. Authority is rechecked after asynchronous reads/hashing. |
| Reservation | reserveBrowserActorChild validates exact admission fields, SHA syntax, generation, byte bound and cancellation; its capacity owns the child. |
| Transfer | BrowserActorChild.load copies/validates the exact admitted length and hash, then transfers the ArrayBuffer and admitted expected SHA over the bound private message port. |
| Worker Import | The child checks message shape/binding/phase, bounds ArrayBuffer and rejects resizable storage, hashes bytes again, checks phase, creates a JS Blob URL and imports that exact URL. Its finally block revokes the URL and wipes bytes. |
| Activation | The loaded module must expose activate; its returned actor must expose invoke/close; active phase and ownership remain checked. Store then invokes describe and verifies the actual descriptor against its retained target. |

Sources are Store 👷️worker/🟦️.ts around1129–1175 and2594, Plugin browser-bundle/🧵️child/🟦️.ts around16 and86, and child/👷️worker/🟦️.ts around71–96. No current authored source occurrence was found for the earlier names VerifiedExecutableValue, shaVerified or fixedHeader in the bounded Framework search. Those terms must not be credited as current proof.

The imported value is a genuine server-selected executable caller input with content-address verification. It is not a hardcoded registry module path. Its boundary needs exact caller/callsite/dataflow and actual selected byte ownership; a file-wide dynamic-import roster or arbitrary path waiver is insufficient. Existing exact dynamic callsite admission remains source/hash and supported lexical grammar proof only. Actual component/provenance and server byte evidence are still pending the registered production owners.

No production source or API was changed by this trace. Fixture inputs remain test examples; this trace does not admit fixture modules to production.
