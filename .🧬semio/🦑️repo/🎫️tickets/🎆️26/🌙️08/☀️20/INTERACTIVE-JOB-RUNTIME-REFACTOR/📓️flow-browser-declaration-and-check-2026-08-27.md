# Flow Browser Declaration And Native Check

## Browser Contract

The existing browser bridge exposed JavaScript without its own declaration, while the renderer maintained a manually asserted subset. The owned ABI schema now drives `FlowSession` declarations in the bridge source package and the emitted Flow-core package. The canonical `semio-framework-os-flow-core:declarations` command generates and verifies the declaration; the Wasm build awaits its build helper and runs the same declaration publisher. Package exports include the explicit browser `types` branch. The authoritative launch seed registers this command.

The schema has 111 operations. `open` belongs to session construction; `attachSurface` and `renderFrame` are exposed through canvas methods rather than direct prototype methods. The other 108 operations receive schema-derived positional and record signatures. Semantic results remain `FlowTask<unknown>` because the current bridge decoder accepts JSON or text, not a per-operation output schema. No renderer-only ambient declaration or fabricated `void` result was added.

The strict fixture, real runtime prototype reflection, and existing third-party TypeScript parser agree on all 108 methods; three hostile fixtures are rejected. The canonical source runs passed in `🧪️flow-browser-declarations-r3-2026-08-27.txt` and `🧪️flow-browser-declarations-r4-2026-08-27.txt`; the final parser pass uses only TypeScript's public Program API. This verifies the declaration surface, not a browser/Wasm behavioral run.

## Metadata Check

The exclusive compiler lease ran canonical `semio-framework-os-flow-core:check --args=--tests` with the warm ticket target and one Cargo worker. It exited 1. Its redirected log disappeared externally during the run; no agent-owned deletion was performed. Exact compiler diagnostics were recovered from the current Cargo fingerprint into `🧪️flow-core-detailed-check-recovered-r1-2026-08-27.txt`.

The Flow library fingerprint has zero errors. The test build reports 18 errors: two nonexistent DSL fixture paths, two missing owned async-test macro references, three obsolete awaits on synchronous reads, three references to the removed free mutation helper, one old HashMap test input, one missing await on the genuinely asynchronous Store constructor, and six playbook fixture values using JSON values instead of schema-owned DSL values.

Owned host/VCS source repairs use real asynchronous Store calls and synchronous immutable reads, native mutation diff application, and the correct BTreeMap channel input. A handcrafted default Flow DSL example supplies the missing fixture rather than disabling those tests. The six playbook value fixes were completed by their existing owner with exact DslValue constructors. All 18 observed diagnostics therefore have source corrections, but these changes have not yet been rerun through the compiler; no native tests executed in this metadata-only pass.

The compiler lease has been returned to the registered-publication executor. Remaining queued evidence includes the raw-wire native law, four scalar-wire native laws, six-route actual Flow command-wire law, shared Flow lifecycle laws, and registered production-grant publication/cancellation coverage.

The next local-interaction producer packet remains staged in `📓️local-interaction-producer-contract-2026-08-27.md`. Its authority requires both current interaction identity and captured document/topology revision, and its tutorial ACK/retry must use fresh authority without reordering replay. No broad interaction-map migration or fake renderer query field was introduced in this compiler/declaration packet.
