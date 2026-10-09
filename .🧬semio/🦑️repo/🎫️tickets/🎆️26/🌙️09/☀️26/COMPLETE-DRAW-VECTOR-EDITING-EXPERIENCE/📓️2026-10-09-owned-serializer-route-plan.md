# Owned Serializer Route Plan

## Actual Current Source

OS IO `io_mechanism::Serializer<S>` borrows snapshot and ArchiveChildren; IoEntry.run borrows IoPayload. Registration and native_carrier reconstruct archive/snapshot roots and resolve async work on first poll. The Draw reserved command exporter now has a genuine owned vector/read path; this does not repair the public serializer route. Native route custody must be adopted at the generic boundary rather than relabeling a reconstructed DTO as captured source.

## Required Route

A native app captures its actual SnapshotRead and original child archive authority at the chosen revision. The generic descriptor selects a declared owned serializer factory without converting or cloning those roots. Factory admission declares exact capacity and depth before constructing its cursor; refusal returns the original request, including source authority and route metadata. A bounded step borrows the original read under its actual per-turn grant. Streamed output pages become the existing sealed output/download owner only after successful completion and still-current revision validation. Cancellation, formatter errors, stale revisions, unsupported font/image data and factory refusal retain every original child through caller-funded close. Four independent demand/grant currencies propagate through generic IO, host registration, app wrapper, writer and output close.

## Schema and Oracle Corpus Before Integration

Cover PNG, SVG and PDF; empty and nonempty documents; transformed geometry; paint styles; raster leases; text source; read authority refusal; exact revision mismatch; zero/copy-only/capacity-only/release-only/depth-only grants; cancellation at source/formatter/output/seal/publication boundaries; repeated acknowledgement; no mounted consumer; and original allocation identity on every refused admission. Use the same language-neutral source/receipt expectations in Rust and TypeScript. Validate emitted bytes with existing independent decoder libraries, SVG renderer/XML parser and PDF renderer. Observe real native heap receipts, including no unreported release during normal work and zero terminal destructor release.

## Integration Constraints

Replace borrowed route dispatch where a registered owned native serializer is required. Do not add a PNG-only sibling host route, reconstruct a reduced snapshot, fabricate a read from ownership arguments, or pump an async future without a work/cancellation bound. Keep mechanism in generic IO and concrete typed Draw writer implementations in Draw. Final acceptance needs mounted export/download/cancel in the rebuilt editor. This is an implementation plan, not a completed capability.
