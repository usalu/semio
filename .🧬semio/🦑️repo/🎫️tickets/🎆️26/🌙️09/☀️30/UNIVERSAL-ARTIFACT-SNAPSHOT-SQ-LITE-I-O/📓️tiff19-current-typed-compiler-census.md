# TIFF19 Current Typed Compiler Census

Read-only current log/source inspection, 2026-10-03. `🗑️generated/root-authentic-tiff19-canonical-typed-paired-compiler-current.log` completed with uncached Nx failure45.2s, current kernel compile prerequisites. Zero artifact assertions executed; this is not TIFF TDD RED.

| Primary Owner | Code / Category | Count |
| --- | --- | ---: |
| OS kernel Semio module | E0432/E0433 `crate::os_dsl::ValueRefusalKind` unresolved |3|
| Store snapshot-capability native decoding | E0599 missing `protocol::PackError::into_value_error` |1|
| Store snapshot-capability native encoding | Same missing enum method |1|
| Store space-history snapshot SQL native | Same missing enum method |2|
| Store space-history snapshot SQL native line52 | E0308 census/construct expect `&DslValue`, receive `DslValue` |2|
| Framework IO native decode line2654 | E0599 missing `protocol::PackError::TextRefusal` |1|

The terminal compiler summary reports11 previous errors and389 warnings. The table groups10 located primary diagnostics extracted by the error-bracket/location parser; it is not the compiler's authoritative total. Compiler progress crate names are not used as attribution. Both borrow mismatches are at the same line52 Binary record field: diagnostic suggestions borrow `&value` for census and construct; re-evaluate after the producer method prerequisite because inference may improve.

## Actual Pack Producer Identity

Store line5876 reexports `crate::os_pack::PackError`; OS package `os_pack` resolves the protocol module. The earlier audited standalone `🧰️framework/🔨️modules/🎒️pack/⚠️error/🦀️.rs` enum has `TextRefusal` and `into_value_error`, but this is not proof that the active protocol PackError has either. The compile receipt confirms the mismatch. Shared port must target the real producer enum, not classify its rendered messages.

The exact current active producer definition is `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` line9. Its enum contains `Schema(String)` at line21 and lacks the inspected standalone enum's typed variant/method. The OS pack facade `pub use pack::*` resolves this identity. A controlled replication Deflate terminal at line413 additionally calls `PackError::Schema(error.into_message())`, an explicit actual ValueError kind-erasure site for its output refusal.

Store `text_error_to_pack_error` at current line10853 still maps `TextError` to `PackError::Schema(error.to_string())`. This demonstrably erases its typed kind and structured text detail at that producer conversion. The native IO change requiring TextRefusal cannot compile until the real protocol producer supports the typed variant or another explicit typed equivalent. No runtime behavior is inferred here.

## Inspected Shared Boundary Behavior

Store five-hook forwarding and kernel typed SQL IO preserve ValueError causes with `IoError::from_value_error`. `validate_owned_sqlite_snapshot_subset` retains actual subset diagnostics for both successful warning outcomes and hard-error refusal. IO hop decoration preserves `error.cause.kind` and diagnostic vector explicitly. Shared controlled snapshot record parser and binder retain `TextError.kind/message` at the ValueError boundary and settle actual native-owned bytes through allocation_stage. No string message classifier was found in these inspected boundary mappings.
