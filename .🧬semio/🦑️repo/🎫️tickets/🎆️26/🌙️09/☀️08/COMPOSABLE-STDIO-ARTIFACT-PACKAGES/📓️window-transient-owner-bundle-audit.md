# Window Transient Owner-Bundle Audit

Source snapshot: 2026-09-09T13:25:52+02:00.

This was a read-only source audit of the shared window-transient owner bundle and current direct callers. It did not run Cargo, Nx, formatting, or a runtime test, so it does not establish native acceptance.

## Verified Source Properties

`WindowTransientOwner::build_owners` now returns one bundle with explicit preparation, state-retirement, and mutation-retirement owners. A partition derives its disposer from the exact bundled state-retirement factory through `transient_store_disposer`, so displaced roots and partition closure share the same owner.

The existing terminal protections remain in source. The bounded value retirement asserts emptiness before Drop; preparation asserts every retained request, base read, prepared root, and active retirement has drained; a partition is removed only after its disposer reports terminal-empty; and the registry removes an owner only after that owner reports terminal-empty. The registry’s maintenance and close paths advance one selected owner or partition per step, with no added interactive cold-drain loop.

A scoped source scan found no remaining callers or public reexports of the retired transient trio: `bounded_window_transient_preparation_factory`, `bounded_window_transient_root_retirement_factory`, and `bounded_window_transient_store_disposer`.

`BoundedViewerFixture` forwards every declared `ArtifactViewer` hook. The direct method-set check found 44 declared methods and 44 fixture methods, with no missing lifecycle, retirement, registration, rendering, context, or media hook.

## Findings

Two direct custom callers do not yet preserve the generic bundle contract.

- Flow’s window transient preparation and Wires Canvas’ preparation accept any nonzero byte grant through `permits_one`, despite declaring nonzero retained footprints. Their checkpoints report zero completed bytes. The generic bundle preparation blocks below the captured retained byte count.
- Those same Flow and Wires close paths retire prepared state and request mutation through direct `store::retirement::owned_retirement` calls, bypassing the state and mutation factories supplied in their `WindowTransientOwnerBundle`. The custom factories happen to use the same current retirement implementation, but the bundle is not the source of authority on those paths.

The generic fallback also has a boundedness-proof gap. `bounded_window_transient_store_owners` accepts state and mutation types without a `RetireOwned` or equivalent cursor bound, then directly drops each value after a fixed 4,096-byte grant. `ArtifactDsl` and `ArtifactPack` do not add such a retirement bound, while generic one-item admission permits up to 1,048,576 retained bytes. The source therefore cannot prove that direct destruction consumes no more work or bytes than the fixed grant for every permitted type.

The shared runtime cleanup callback presently records an interactive-ceiling overrun as a debug message and preserves its prior status, instead of returning `RuntimeMaintenanceStatus::Fault(InteractiveCeiling)`. Its rationale and authorship are outside this audit, but the current source cannot serve as proof of strict callback-ceiling failure.

## Evidence

The machine-readable source receipt is [window-transient-owner-bundle-audit.json](🗑️generated/window-transient-owner-bundle-audit.json). The Flow/Wires and callback findings were sent to the coordinator and Norm while the audit was running. Runtime acceptance remains pending.

## Settled Follow-up

Follow-up source snapshot: 2026-09-09T13:39:14+02:00.

The prior Flow and Wires findings are closed. Both owner builders now create state and mutation OwnedValueRetirementFactory instances, pass those exact owners into ArtifactEphemeralTransferPreparationFactory, and pass the same owners into WindowTransientOwnerBundle. The shared transfer source contract treats a moved owner as zero-byte transfer work after the declared retained-footprint admission; its unit source explicitly exercises a 12 KiB string with a zero-byte preparation grant and checks a zero completed-byte checkpoint. This is static-source evidence only; that test was not run in this audit.

The generic fixed-page direct-drop implementation and its public helper are gone. A current repository scan found zero source references to bounded_window_transient_store_owners, BoundedWindowTransientValueRetirement, or the earlier window-owner trio.

Trinity Jack Results now has a focused explicit owner. Its state and mutation retirement factory constructs strict terminal-checked child retirement: strings, columns, and rows use RetireOwned; an optional graph fixture delegates to the existing JackSnapshotRetirementFactory; and the same factory authorities serve transfer preparation and the owner bundle. The helper advances one child per supplied grant and rejects a child that reports Complete without its terminal witness. This closes the generic fixed-page retirement proof gap for the prior final generic caller.

A new static fixture recheck resolves 352/352 references in the 42 affected sources. That includes all 23 previously missing Norm router #[path] targets. It is a literal/module-resolution audit, not a Cargo or native acceptance result.

The independent interactive-ceiling maintenance policy remains outside the owner-bundle changes and is retained as the stated strict-ceiling proof limitation. No callback policy was modified or attributed by this audit.

The follow-up receipt is [window-transient-owner-bundle-followup-audit.json](🗑️generated/window-transient-owner-bundle-followup-audit.json).
