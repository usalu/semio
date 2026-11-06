# Media Route Review — 2026-11-06

Reviewed in-progress collector, media/file declaration resolver and ShellHost scope. Initial resolver counted identical per-window action clones as ambiguous; owner subsequently reported allowing identical defaults and refusing conflicting metadata. Scope originally threw declaration failure before its catch; owner reports funnel now awaits after target/current-owner resolution. These evolving source snapshots require recheck after final implementation.

Collector concern: running applied_progress is compared monotonically with complete status, while reactor ⚛️reactor/🏃️runtime/🔄️exchange/🔀️plugin-exchange/🎞️media-commands/🦀️.rs:49–50 sets completion applied_progress to total_bytes. Producer checkpoint units may differ and exceed output bytes (especially GLB normal scan), causing valid completion to appear regressive. Poll/submit/take can also remain pending without a collector deadline/cancellation interrupt; drain errors need owned cleanup. Completion descriptor currently lacks an explicit size/nonempty bound before allocation and decode; existing agent route has a descriptor cap.

Producer descriptor is MediaArtifactDescriptor with camelCase outer fields portId/mediaType and Binary wire format; reactor constructs kind_id None, wire format_kind from status.schema. The helper field comparison agrees with source naming subject to actual encoded fixture verification.

Native framework shell-only classification: plugin/👷️job/🏛️reserved/🦀️.rs:876–885 names document transfer IDs; plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:187 refuses those IDs as shell-owned, sibling 🧵️instance-root/🦀️.rs:50 maps to Shell. New generic media action needs consistent classification and refusal where applicable to avoid passing accidental direct invocation to a product job. Existing paired-app history law tests document-transfer IDs.

Independent GLB cursor review sent owner: per-turn bounded scanning of absent normals is O(vertices×triangles) overall and depends on immutable snapshot-backed source. MeshData source floors position array length divided by three, so malformed trailing components can be silently omitted unless validated. No code edits or test results in this review.

## Native Shell Guard Changes

Added EXPORT_MEDIA_FILE_ACTION_ID to plugin 👷️job/🏛️reserved classifier, 🧵️instance/🧷️command-routes/📥️staging/🚦️actions shell-owned refusal and sibling 🧵️instance-root shell-kind classifier. Paired-app convergence law skips this framework shell action. Existing plugin builder contract law asserts reserved classification and updated refusal source. Framework Rust facade 📦️packages/🦀️rust/🦀️.rs explicitly reexports the action ID and factory in both manifest and top-level lists. The applicable existing guard law has no neutral fixture for this ID-only assertion; no runtime law was run.

Latest GLB cursor uses exact-length normal scratch and O(V+T) clear/accumulate/normalize. Duplicate triangle indices are accumulated in sequence and candidate normals validate before scratch writes. Declared copied-byte quote covers current transition and zero funding returns unchanged. Layout now rejects trailing position components. Stable source and truthful bounded read_work remain required; source review is not execution.

## Updated Collector and Browser Predicate Review

Current collector checks source handle, running-only monotonic progress, MIME, nonempty bounded descriptor and output bounds; it drains refusals to terminal and attempts cancel on drain exceptions. Deadline checks run before/after awaits, but cannot interrupt a permanently unresolved submit, poll, take or cancel promise; it is not a strict wall-clock bound. Actual fresh focused gate remains pending.

Browser abort and undo predicates compare five displayed product row texts, not a complete document/archive or history witness; they may miss unrelated mutation, so cannot alone establish document-byte identity or one complete undo. The identity predicate binds surface IDs to both shared document IDs, but its population query is not explicitly tied to the same matching surface/instance and only requires A population. Owner was notified. New examples/JPEG/corrupt/cancel remain unexecuted rather than falsely passed in the observed source.

## Extracted Delivery and Draft Mesh Source

Current downloadDeclaredMediaFileV1 checks document identity after drain and immediately before sink close, aborts opened sink on copy/authority failure and verifies descriptor against declared output. Its neutral delivery cases exercise changed revision, cancelled or retired owner, wrong format and complete delivery with assertions outside catches. An arbitrary async sink or port can hang or delay delivery past authority change; current buffered shell sink close delivers synchronously, narrowing that risk for actual route. No runtime pass inferred.

Unmounted Remodel 🎞️mesh-export/🗃️source/🦀️.rs incrementally indexes source leaves and validates ordering, lengths, envelope counts and per-leaf UTF-8. Its scalar vector reports fixed work 7, but each scalar performs a BTreeMap lookup by variable-length content ID; index advance likewise charges fixed work around map lookup and handle parsing. Those variable comparisons are not evidenced in its quote. Owner was notified to bound/meter them or use a bounded original lookup capability. Packed 12-bit offsets fit current 4096-byte leaf limit; built-in box face sequence matches mesh_box. Draft rejects empty meshes more strictly than the native envelope, requiring explicit export semantics. No tests or edits were performed.
