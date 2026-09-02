# Mathematical CSV Compiler Confirmation

The repository crate check is independently blocked by the known `semio-s-plugin-stdio` trait-implementation failures, so the exact source shape was isolated without changing its semantics in `🔬️mathematical-csv-future-repro/🦀️.rs`.

Command:

```text
rustc --crate-name mathematical_csv_future_repro --edition=2021 🔬️mathematical-csv-future-repro/🦀️.rs
```

Observed compiler result:

```text
error[E0609]: no field `nodes` on type `impl Future<Output = Graph>`
note: this implements `Future` and its output type has the field, but the future cannot be awaited in a synchronous function
```

This confirms the reported `MathematicalIntoCsv` defect before editing it. The fix should use the already-synchronous owner accessor because both the CSV and JSON serializers need an immediate, fallible materialized scene lookup; adding an await would leave the missing-owner contract unresolved.
