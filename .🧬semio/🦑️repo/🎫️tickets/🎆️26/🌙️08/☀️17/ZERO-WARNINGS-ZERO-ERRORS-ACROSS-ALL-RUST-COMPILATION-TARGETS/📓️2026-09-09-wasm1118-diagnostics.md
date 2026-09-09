# WASI 1118 Diagnostic Pass Diagnostics

Completed receipt: {"diagnosticPass":true,"strict":false,"status":101,"signal":null,"reason":"exit","counts":{"error":1,"warning":0},"cargoWarnings":["warning: build failed, waiting for other jobs to finish..."],"startedAt":"2026-09-09T13:37:56.465Z","finishedAt":"2026-09-09T13:41:00.046Z","packages":160}

## E0277 — `?` couldn't convert the error to `dsl::Fault`

error[E0277]: `?` couldn't convert the error to `dsl::Fault`
     --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:18204:83
      |
18204 |             if !store::OwnedDocumentClosureSource::child_projection(&source, None)?.admits_member(slot, expected) {
      |                 ------------------------------------------------------------------^ the trait `std::convert::From<dsl::ChildRestoreProjectionError>` is not implemented for `dsl::Fault`
      |                 |
      |                 this can't be annotated with `?` because it has type `Result<_, dsl::ChildRestoreProjectionError>`
      |
      = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
help: `dsl::Fault` implements trait `std::convert::From<T>`
     --> 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/../../../⚠️diagnostic/🦀️.rs:538:1
      |
  538 | impl From<&str> for Fault {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^ `std::convert::From<&str>`
...
  544 | impl From<String> for Fault {
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::convert::From<std::string::String>`


