# Drawing Retirement Trait Resolution

Native3 original6942 is the genuine test-first compiler RED. Four E0404 diagnostics named the root Value RetireOwned derive macro where generic ownership bounds required the actual trait: Drawing root15/23 and birth-transfer5/7. No new runtime invocation was needed to demonstrate these exact failures.

## Defining Canonical Contract

Value's Rust facade exports semio_framework_value_derive::RetireOwned at its root as a derive macro. It separately reexports the retirement module from the defining retirement package. The canonical trait is semio_framework_value::retirement::RetireOwned, defined in framework/value/retirement/🦀️.rs:20. It requires Send+'static, consumes the original owner into a retirement cursor, declares controlled retirement support and optional priced original allocation retirement. Its cursor is Send; shared original operations add Sync only at their actual shared operation boundary. This patch adds no Clone/Cold/Sync requirement and creates no compatibility name or public API.

## Exact Two-File Change

- schema/🪆️drawing/🦀️.rs: both LayoutDrawingBirth<S> and LayoutImportBundle<S> bounds now fully qualify the canonical retirement trait. Existing fully qualified derive macro attributes remain unchanged. Field layout and all behavior remain unchanged.
- schema/🪆️drawing/🌱️birth/📤️transfer/🦀️.rs: the existing grouped import now imports retirement::RetireOwned. The two generic transfer bounds thereby resolve to the same defining trait. The demand, original source/target Option custody, deficient-grant refusal, cancellation retention, occupied/empty slot refusal, paid header move and returned progress bodies are untouched.

No source in the root model/snapshot/validation files was edited by this packet. Dependency owner separately removes the obsolete opaque Drawing carrier and updates the two stale consumers, following the canonical reference-only member design rather than restoring removed validators. Both changes require combined review before a genuine native retry.

## Source Identity And Limits

The exact before/after hashes and full paths are retained in drawing-retirement-trait-working-files.json. Current after hashes:

- Drawing root:9a51d5565aa4b32cfa68592fa049235f9093ebee0a2bf76d22b866a71aa425d6.
- Transfer:c30de168951f62947936f5346e0d6e83d5f3ec30c5b30fae7269317c8f34e1a2.

Fresh reads and compare-before-write preserved peer hunks. Bounded Drawing subtree inspection found no other stale RetireOwned trait imports/bounds; root-qualified derive uses are intentional and remain. The patch adds no files to the compiled source closure and only qualifies three existing names. No owner quota, feature, profile or allocator/lifetime guard was changed.

## Verification State

The actual Native3 six-error result remains preserved; this patch addresses only its four trait-name diagnostics. Existing original Drawing birth/transfer and language-neutral fixtures remain unchanged and must execute through their proper selected owning gates. This is a name-resolution repair to an existing contract, not a new feature or a claim those laws have run. No compiler, test, Cargo retry, setup, preview, cache operation or peer action was launched by this packet. Full Layout15 remains the mandatory integration/runtime selection after Root reviews the combined correction; no focused check is substituted as completion evidence.
