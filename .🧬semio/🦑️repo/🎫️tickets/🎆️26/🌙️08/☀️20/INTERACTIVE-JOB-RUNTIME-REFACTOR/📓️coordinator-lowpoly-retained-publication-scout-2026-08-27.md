# Lowpoly Retained Publication Scout

## Scope

Read-only review of the actual Lowpoly editor, preparation factories, work-unit close cursor and the shared plugin publication preflight. No Lowpoly native test or browser workflow was run in this scout. The full command census still distinguishes source-admitted registrations from actual runtime proof.

## Exact Findings

The editor at `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` declares Transient publication for paintStrokeBegin, paintStrokeEnd, transformBegin and setActiveUtility (PUBLICATION_CONTRACTS near lines 887 and 901–903). The app installs Artifact and Config preparation factories near lines 1394–1400, but not the exact Transient preparation/root-retirement pair. The shared plugin preflight near line 19920 requires both for Transient. Existing Migrated action labels do not supply those owners; blanket admission would hide the defect.

The Artifact preparation permits a 16 MiB root and the Config preparation a 16 KiB root. Artifact advance near line 1102 takes its mutation, computes the complete inverse/post state and scans retained size before one prepare_one_item call. Config follows the same whole-result pattern. Reporting one item does not split those operations into bounded work. Error paths after taking the mutation also require explicit owner-retirement review.

Artifact close near lines 1139/1146 and Config close near lines 1280/1287 require a single grant at least as large as prepared_bytes or retained_bytes. The actual production grant is one item and 4,096 bytes. Any admitted retained owner above that grant can remain blocked indefinitely. This requires paged retained roots/preparation/retirement, not a larger minimum grant.

The Lowpoly work-unit close near lines 781–820 separately pops paint runs, then requires one grant covering the entire paint_runs vector capacity before releasing it. The legal 4,096-run envelope can exceed 4,096 bytes for that outer allocation alone. Clearing lengths first does not retire the remaining allocation within the fixed grant.

## Implementation Packet

After the shared exact-close and host-clock foundation, add schema-first laws for exact Transient ownership and maximum-envelope preparation/cancellation/retirement; validate against existing third-party semantic oracles. Implement retained per-page roots and cursors at the owning domain boundaries. Verify actual registered dispatch, stale publication, cancellation at every phase, final-owner close, and unchanged outer timing.

The four source-blocked rows are only this first cohort. Other Lowpoly BatchOnly mesh and paint commands, complete command ingress, both renderers, fresh Wasm and browser workflows remain part of the all-app exit gate. No all-Lowpoly completion is inferred from admitting these four.
