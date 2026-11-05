# Nested Edit Source Review

Source inspection only. No build or test was launched by this review.

## Fixture And Native Oracle

The native oracle at `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/✏️edit-rules/🧪️tests/🦀️.rs` includes `../../../../../🧪️tests/🪆️nested-edit/🧫️cases/🔣️.json`; this resolves to the existing norm-owned fixture. The test module is registered in its parent. SetFieldRule is Copy, so const array indexing used by rule fixtures is supported.

The eleven cases cover parent replacement, sibling preservation, id and index selectors, nested selectors, nearest parent, exact-rule precedence, null child versus exact clear child, missing field, unknown selector and textual-prefix refusal. The serde_json pointer oracle independently constructs expected replacement payloads and compares successful-case source immutability.

Coverage gaps in this snapshot: the feature describes an atomic bundle but these eleven fixtures do not execute a two-write bundle; direct null parent clear is not a fixture; refusal branches continue before the explicit source-equality assertion. The synthetic ISO rule kind is Script, while actual native ISO uses script: this fixture tests abstract boundary resolution, not actual ISO mutation admission. No Rust compile error was established by the bounded source review.

## Existing Owned Typed Scratch Executor

Owned guard: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs:589`, public `semio_framework_os_store_ledger::ColdReplayProjection<P, M>`, requiring M: ColdMutation<P>.

- new takes ownership of a projection.
- Deref reads the current projection; projection_mut lends it exclusively.
- advance installs the next projection and retires the displaced projection through M::ReferenceDiff::retire_projection.
- Drop retires the final scratch projection, including early returns.
- into_inner transfers ownership and therefore retirement responsibility to the caller.

Admission-aware execution: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs:267` defines public `fold_operation_into<P,M>(&mut P,&M,u32) -> (bool,Vec<MutationMessage>)`, requiring M: ColdMutation<P>. The replay root explicitly reexports it at line 44. This executor calls outcome.admit, applies via central apply_diff_cold_into, retires each diff, and converts central apply failures into stamped Fatal messages.

A bundle can own one document.clone() in this guard, resolve each write against its current typed projection, and execute each generated typed mutation in order through the free executor. On false, reject the entire emission and let the guard retire scratch; retain returned messages rather than silently discarding them. Resolving the second write against that evolved typed projection preserves the first sibling write within the same replacement parent. Apply operation indices consistently across every emitted mutation, not merely every requested field.

The ColdFoldProjection trait convenience method ignores returned messages. Lower protocol::apply_operation does not call outcome.admit. Neither is an equivalent replacement for admission-aware bundle validation.

The in-place executor may leave a failed private scratch partially changed by a failing central applier; this is harmless only if failure immediately abandons and retires the entire scratch, never commits or continues it. Original source remains untouched because it was cloned before scratch execution. No arbitrary JSON projection is needed.

This review does not establish dependency direction feasibility: callers must use existing declared framework exports or introduce the correct owned crate dependency without a cycle. It also does not claim runtime passage of the authored red test.

## Existing Kernel Facade And Integration Outline

No new direct store dependency is necessary. Existing public paths are:

```rust
use semio_framework_os_kernel::os_store::{
    ledger::ColdReplayProjection,
    replay::fold_operation_into,
};
```

The store root explicitly publishes the ledger crate as ledger and publishes replay with its public crate exports. The kernel os_store mounts component::* publicly. These imports therefore use the already-declared os-kernel dependency.

Required generic additions are D: ToValue + Clone and M: ColdMutation<D>, with existing resolver FnMut(&D,&NormEdit)->Result<Vec<M>,String>. The current Mutation trait alone does not expose diff; ColdMutation does.

Execution outline:
1. Own document.clone() in ColdReplayProjection::<D,M>::new.
2. For each requested write, materialize the CURRENT scratch's to_value(), resolve its current leaf value, and perform existing numeric coercion against that current leaf. Missing paths must remain errors rather than silently falling through coercion.
3. Invoke the existing domain resolver against &*scratch and the coerced edit.
4. For every returned real typed mutation, call fold_operation_into(scratch.projection_mut(), &mutation, op_index). Preserve the returned messages. Increment operation index per mutation, using checked conversion where usize crosses u32.
5. On false, reject the whole bundle and retire staged mutation owners through their existing mutation retirement interface; guard Drop retires scratch. Do not continue execution after a failed in-place apply.
6. Only after all writes succeed, return the accumulated typed mutations for the single existing event emission. Guard Drop retires the final validation projection. Do not use into_inner when no caller receives that projection.

MutationOutcome::admit (outcome-and-message source line 293) refuses Error and Fatal, because worst_level >= Severity::Error. Warning and lower levels remain admitted. fold_operation_into returns false for semantic refusal and central apply failure; a central apply failure appends a Fatal message. Rejecting all nonempty message vectors would incorrectly reject warnings. Existing Result<Vec<M>,String> cannot return successful warning metadata directly: preserve native warnings in the resulting emitted operations' real execution, and include returned refusal detail when forming the error; do not invent a second persisted message channel.

The scratch materialization must occur inside the write loop, after earlier mutations execute. This is required both for sibling payload preservation and for numeric coercion after a preceding mutation changes a field type/value. This is a source-verified API outline; compilation remains the parent-owned gate.

## Minimal Live-Core Typed Test Model

Prefer a small local fixture using the existing owned diff macro, rather than adding norm-artifact or store-testing dependencies.

```rust
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct Configuration { a: f64, b: f64 }

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
struct Document { configuration: Configuration }

semio_framework_os_kernel::sparse_record_diff! {
    record: Document,
    diff: DocumentDiff,
    fields: { configuration: Configuration }
}
```

ToValue and FromValue derives are from the already-available value-derive crate. Macro expansion generates the sparse typed configuration replacement, value codecs, algebra, and ColdMutationDiff::apply with sealed central ApplyCapability/PreparedField installation. Because the generated diff is public, make fixture record types public inside the private test module if Rust reports private-interface visibility concerns.

Implement a local enum with ReplaceConfiguration(Configuration), WarnConfiguration(Configuration), RefuseError and RefuseFatal variants. Only ColdMutation<Document> is required by the production staging interface: type ReferenceDiff = DocumentDiff, diff constructs MutationOutcome::new(DocumentDiff { configuration: Some(payload.clone()) }), warning adds .warning(code,text), error and fatal use MutationOutcome::error(code,text,[configuration]) and ::fatal. inverse returns one ReplaceConfiguration(base.configuration.clone()). No descriptor, text codec, binary codec, schema registry, artifact codec or Mutation implementation is needed unless another real caller adds such a bound.

A same-parent witness writes configuration.a then configuration.b, deriving each ReplaceConfiguration payload from the resolver's provided typed current Document. Starting (a=1,b=2), writes a=3 then b=4 must emit payloads (3,2) then (3,4); actual typed replay yields (3,4), while source remains (1,2). An independent serde_json pointer mutation yields expected (3,4) and provides the external comparison.

Add a second-call resolver witness that observes the first staged a=3; an admission failure on the second mutation must return Err without emitting either mutation and leave source untouched. Separate Warning admission should succeed; Error and Fatal should refuse. Resolver refusal and admission refusal are different cases. An operation list containing several mutations also verifies operation-index progression independently of requested write count.

Working manual implementation reference: framework OS store replay/🧪️tests/🔐️authority/🦀️.rs defines private SetCounter and CounterDiff, implements all central diff traits and ColdMutation, and exercises the explicit cold executor. The macro above removes most of that boilerplate using the same owned interfaces.

Existing DemoSnapshot, DemoMutation and SeverityMutation (with SetWarningN, SetErrorN and SetFatalN payloads) live in the separate os-store-testing companion crate. Kernel test_support points instead to os-store-test-support; no public reexport of those model fixtures was found through it. Adding a new dependency solely to borrow the one-field demo would be unnecessary, and its scalar n model would not itself witness two siblings in one typed replacement parent.
