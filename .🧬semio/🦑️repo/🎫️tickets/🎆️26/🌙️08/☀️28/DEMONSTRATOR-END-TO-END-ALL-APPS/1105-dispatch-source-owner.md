# Pure Invocation Dispatch Source Ownership

Actual combined editor r3 ended before Rust/tests at the fixed Session source boundary. Runtime's census found Session 262953 bytes, 809 above its unchanged boundary, while the existing Source owner had ample room. This is a source ownership correction, not a cap change or test verdict.

The complete FlowInvocationDispatchWork now mounts from its existing physical invocation/dispatch-work file under the existing Source crate as invocation_dispatch. Session explicitly reexports that module and its original public names. The four-string FlowInvocationDispatch wire DTO moves with its sole pure producer; the genuine FlowInvocationOrigin, its lease, source metadata and authority remain Session-owned. No snapshot, origin adapter, record authority, clone or dependency was introduced.

The existing FlowInvocationRequestSource interface gains only extension_id(), borrowing the actual original field through the existing direct FlowInvocationOrigin implementation. Dispatch source authentication uses the existing actual invocation_source_identity. Request wire order, bytes, fixed capability/response strings, output transfer, output restoration, cancellation and raw child retirement remain unchanged. Existing native denial/source-substitution/output-restore/cancellation assertions were preserved.

The public context pump forwards actual recorded raw CPU independently of retained progress and error status, removing the earlier error-implied minimum of one. It also retains the genuine OriginalSourceBinding after funded progress and validates the actual source parent; equal operation/generation labels do not authorize another ledger.

Four touched Rust sources pass the registered Node/tree-sitter syntax parser. Native typing and behavioral laws remain pending; Runtime owns final source census and the next unchanged native target. No lower Value source, Cargo manifest, dependency, stock, cap or identifier changed.
