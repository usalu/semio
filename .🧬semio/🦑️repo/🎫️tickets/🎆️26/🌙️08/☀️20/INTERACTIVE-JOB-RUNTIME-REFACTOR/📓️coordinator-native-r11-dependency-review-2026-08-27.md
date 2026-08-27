# Native R11 Dependency Review

The registered latest-wins native r11 attempt exited 1 before any tests executed. The coordinator read the retained compiler output directly. Its only reported Rust error is:

```text
error[E0432]: unresolved import dsl_derive::MutationLeaf
  --> framework OS DSL facade, component.rs:20:83
pub use dsl_derive::{DslArtifact, DslDiff, DslEnum, DslOps, DslRecord, DslScalar, MutationLeaf, Mutations};
no MutationLeaf in the root
```

Full original output: `🧪️member-latestwins-registered-r11-native-2026-08-27.txt`. The build restarted from proc-macro2 and rebuilt cold dependencies; this run did not lose its artifact directory. No registered-dispatch, Presence, reader race, peer overlap, or CAD test pass is inferred from the compiler reaching the kernel.

## Exact Source Join

The coordinator read the DSL facade and the derive crate's primary/glue sources. The derive source contains mutation-leaf descriptor parsing and emission helpers but no `proc_macro_derive(MutationLeaf)` export. The current framework/app Rust census finds MutationLeaf trait use and descriptor helpers, not a concrete use of the missing derive macro. The affected source was included in peer commit `a8d1caf41f`; no peer changes are reverted.

The Flow executor owns a narrow export/consumer reconciliation. It must validate the actual consumer census and source/glue ownership, preserve the existing MutationLeaf trait and descriptor implementation, and correct only the premature facade export if there is no corresponding macro. This native prerequisite does not authorize implementing a separate unfinished derive feature or removing working mutation semantics.

The publication executor is completing the already identified rejected-actor ownership bundle while the dependency join is repaired. Compiler use remains serialized within this fleet; unrelated peer builds are untouched.
