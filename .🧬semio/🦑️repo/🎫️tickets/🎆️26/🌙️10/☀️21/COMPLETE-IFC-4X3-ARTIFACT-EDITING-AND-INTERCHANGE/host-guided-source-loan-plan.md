# Guided Original Source Loan

The native guided type-input candidate must reuse the actual source selected by the render host. `PendingReserved.pending_render_source` already retains one original-funded `Arc<A::Snapshot>` under `OriginalRenderTreeIdentity`; the host checks it against the actual time-travel/tool-overlay Arc and retires query work before this source and before tree donation. `ArtifactView.snapshot` alone cannot manufacture its owning authority.

## Contract First

Add an immutable `source: &Arc<Self::Snapshot>` argument to the existing `ArtifactApp::render_with_work_context` and `ArtifactEditor::render_with_work_context`, including `EditorApp` forwarding. The host passes only its identity-checked `pending_render_source` owner. `OriginalRenderWork<Arc<S>>::snapshot_loan(identity, expected, cx)` returns that borrowed Arc after original identity, active-owner, cancellation, and exact pointee identity checks. A same-content substitute is refused before artifact work or allocation. The borrow lifetime is tied to the existing retained owner; no owned snapshot or source authority is returned.

The IFC wrapper additionally checks `Arc::as_ptr(source) == doc.snapshot` before binding or advancing either details or guided child work. The existing full canonical revision, body, donated request/window/locale and initiating caller pins remain. Guided type work may admit one required `SnapshotSource` projection from a real paid Arc alias. The alias is placed in a retained slot before posting its copy receipt; constructor success or refusal is likewise stored before a fallible receipt. This projection is a lease over the same immutable model, not a model copy. Its quote includes the actual Arc descriptor copy, retained metadata, shared headers and lease payload. Child candidate cursors close before this projection, which closes before the host Arc.

## Laws Before Production

Language-neutral cases live alongside the render ownership laws: original pointee admitted; equal-content different pointee refused; original caller replacement refused; cancelled caller refused; closing owner refused. Native laws call the real host loan and retain/close every owner. Source-dependent body tests pass the real source Arc, and a mismatched Arc must fail before child birth. Additional alias admission laws deny independent axes, preserve original pointer/refcount on denial, retain a born alias across posting failure, and close under cancellation. The seam changes no IFC model output: existing schema/IfcOpenShell expected candidate output and exact inverse laws remain the semantic oracle; no third-party ownership/runtime claim is inferred from that output oracle.

## Execution Scope

First implement and audit the host/IFC loan seam and retained source projection. Then connect progressive guided candidate preparation to the existing roster, private `RetainedTypeAdmission`, strict `DomainWork` and compiler-issued local rules. REAL/NUMBER/BINARY field controls remain a separate unfinished producer frontier until that connection is coherent. No free SELF, new candidate certificate, public sender, native capability, full conformance or publication flag is introduced.

Authentic Cargo currently refuses enforcing compiler ingress before rustc. Native syntax, custody laws and source audit will be reported separately from unavailable type checking, execution, actual allocation and physical receipt proof.
