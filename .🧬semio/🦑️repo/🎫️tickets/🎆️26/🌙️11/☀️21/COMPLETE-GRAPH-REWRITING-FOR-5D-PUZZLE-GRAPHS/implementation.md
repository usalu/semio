# Graph rewriting for Puzzle 5D

Trinity publishes the authored rule and its parameter bindings on `rule:out`. A workflow connects this output to the Puzzle 5D editor's `rewrite:in`. Both ports use the `trinity.rewrite.invocation` media schema. The Puzzle editor evaluates the rule against its own document and publishes the resulting Puzzle mutations through its document history. The rewritten document is available on the existing `artifact:out` port. Puzzle archives carry the exact composed kit catalog. History replay reads the same declared tagged mutation records the editor publishes.

The Puzzle projection maps parts to nodes, grips to directed ports, and fasteners to directed edges. Kind names come from the Puzzle catalog and placed records. Unclassified parts, grips, and fasteners use `Part`, `Grip`, and `Fastener`. The adapter checks graph effects against staged Puzzle mutations before handing them to the editor for publication. Puzzle metadata, target volumes, catalogs, and unrelated document fields stay in the Puzzle artifact throughout.

For example, author this left-hand pattern in Trinity to match Puzzle parts without an assigned kind.

```json
{"pattern":{"leftVar":"part","leftKind":"Part"}}
```

Give the rule this right-hand side to change each matched part’s 3D label. Use the catalog’s part kind in the left-hand pattern to target a specific kind.

```json
{"create":[],"delete":[],"merge":[],"parameters":[],"set":[{"var":"part","prop":"label","value":"Selected part"}]}
```

Connect `rule:out` to `rewrite:in` on the target Puzzle instance and run the workflow. Read the rewritten document from that instance’s `artifact:out`. A rule whose left-hand side matches nothing produces no edit.

The rule model supports a single node or a directed edge pattern with an optional predicate. Planning runs through Jack's stepped executor. A Puzzle import budgets 64 KiB for the invocation text or intrinsic value, up to 16,384 source records across the Puzzle document and its catalogs, and up to 65 planned effects. A cancelled, refused, or over-budget rewrite leaves the document untouched.

Native verification exposed compile breaks in related code. Flow's hover route now imports its existing ABI size constant. Puzzle's localized example preview uses UTF-8 text as bytes. Trinity's layout retirement, nested cold graph diffs, and tagged rule mutation receipt use the current framework contracts.

## Independent verification

The shared build fleet stalled during project graph and resource admission. The focused verification runs used Cargo on macOS. Both hub component crates type-checked for `wasm32-wasip2` using the locked workspace. These runs did not launch a workflow with both packaged guests or exercise the desktop UI on macOS, Windows, or Linux.

- Six native Puzzle adapter tests passed. They cover the imported Puzzle fixture, graph identities, catalog-backed edits, ordered topology changes, preservation and refusals.
- All six retained DSL field tests passed, including value-type allocation and cancellation at multiple frontiers.
- The rewrite engine's language-neutral fixture matched an independent SQLite oracle for the graph effects.
- Ajv validated both the Trinity and Puzzle media fixtures against the authored invocation JSON Schema and rejected two malformed examples.
- All 31 native workflow runner tests passed, including Wasmtime host command admission, cancellation, direct intrinsic media negotiation, scoped port routing, and recovery after cache eviction. The native prototype runner binary also passed type-checking.
- The Trinity editor passed native type-checking. All 25 Trinity rewrite-engine tests passed.
- Seven native Trinity editor tests passed across the focused runs. They covered cold birth, blank and demo archive reload, typed archive replay after an edit, Undo and Redo, port declarations, and intrinsic Pack output carrying the authored rule and parameter bindings.
- All three mounted Puzzle rewrite tests passed. The import waited for publication, the archive reopened with the rewritten projection, and one Undo restored the original. Refusal and cancellation preserved the original projection.
- All six mounted Puzzle kit import tests passed, including archive reload after two catalog updates and preservation of a single exact catalog member. The tagged mutation wire also passed its text and binary round-trip guard.
- Both plugin bridge manifests resolved with their locked dependencies on macOS.
- The native encoding suite passed 27 laws, with one pre-existing ignored case.
- The registry validated the published Pack and JSON descriptor pairs for both plugins. The Puzzle 5D editor declares `rewrite:in` in its packaged manifest and at runtime.
