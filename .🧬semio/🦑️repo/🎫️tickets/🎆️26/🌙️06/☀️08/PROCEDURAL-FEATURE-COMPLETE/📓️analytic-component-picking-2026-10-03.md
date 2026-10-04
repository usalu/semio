# Analytic Component Picking

Current source tracing reaches the kernel `MeshTransfer` face and edge groups, which carry lossless persistent labels, then `mesh_data_from_mesh_transfer`, which discards both groups. The procedural preview reads the delivered `MeshData`; the React and WGPU component pick paths consume numeric `faceIds` and `edgeIds`. Successful polygon mesh component tests do not establish analytic BRep component selection.

The schema-first fixture at the existing BRep engine's `component-picking` fixture directory specifies separate numeric renderer IDs and `componentReferences` tables containing full persistent label strings. It deliberately includes labels above both u32 and JavaScript's exact integer domain. Mapping groups to consecutive renderer IDs permits existing hit testing; the table preserves the analytic identity without hashing, truncating, or storing another registry.

The new native law first asserts face and edge picking coverage, then the reference table and independently computed Parry triangle area. Production has not changed for this feature. Actual RED execution is pending the root's current native import gate. Later work must carry the table through the same retained preview encoding and use existing mutation and inference paths to create selective editing and analysis operations. Analytic vertex selection and general curved-surface trim editing remain open.

The current registered BRep roster already contains planar profiles, NURBS surfaces, Coons patches, lofts, sweeps, solids, selective fillet/chamfer, sections, split operations, and topology analysis. No explicit generic trimmed-surface operator was found; this report does not claim arbitrary curved trim editing is implemented.

## Portable RED and Pending Replay

The registered `@semio-tech/s-3d-js:test-quick` run executed two tests: the new analytic picking law failed with `faceIds` undefined versus `[0,1]`; the existing renderability law passed. The first-party TypeScript converter now retains both face and edge group identities and lossless string reference tables. A current-source replay is active. Native BRep production remains unchanged until its own actual RED is reached. The neutral fixture also drives Three's geometry groups and triangle area independently.

## Current Portable Range Acceptance

The registered s-3d-js target reached the intended malformed-range assertion RED (true versus false), then passed all 3 tests after the same converter validated ordered complete coverage, triangle alignment, segment alignment, finite safe counts, nonempty labels and bounds. The fixture retains labels above u32 and JavaScript integer precision; independent Three.js area remains 1. Native conversion and application picking remain unverified. Logs: 🗑️generated/sol-2026-10-03/brep-component-ranges-portable-red.log and brep-component-ranges-portable-green.log.

## Native Red and Shared Record Acceptance

After correcting test-fixture relative paths, the focused native test reached the intended RED: face_ids was empty instead of [0,1], 0 passed/1 failed/591 skipped. The converter now returns an owned invalid-input refusal for bad buffers/ranges, numeric renderer IDs, and lossless string reference tables. Every reached session and CAD caller has the new result handled by its existing refusal policy. Native replay is pending.

The independent OS TypeScript decoder reached intended RED on the metadata whitelist, then GREEN7/7 (470 skipped). componentReferences uses existing metadata field13, existing bounded metadata cursor, existing world digest and existing MeshData retirement. The original analytic fixture now also owns the cross-language record bytes. A separate malformed-reference test is running before stronger validation. Cached metadata retirement is authored as a native private law; the existing session ownership schema and roster now explicitly include that lib target (11 native laws/four portable contracts). The registered source-check passed. No application picking, native record-packing, wire-reference or vertex-reference acceptance is claimed yet.

## Verified Native Conversion and Reference Refusals

The native replay passed 1/1 (591 skipped), Nx7m31s including shared queue/build; runtime [DEBUG] records two pick domains, four full labels and independent Parry area1. The stronger malformed-reference portable law reached intended RED (missing exception), then the full mesh-pack decoder subset passed 8/8 (470 skipped). Native MeshData value decoding and pack decoding now enforce known domains, bounded labels and matching numeric pick-buffer cardinality. New native range/reference refusal laws are authored but not yet run. The exact cached metadata byte-retirement RED is still waiting on the shared Cargo lock; its production bridge remains unchanged.

## Cached Metadata Retirement — Actual Native Replay

The focused registered Session law first failed at runtime: 4096 labels × 128 bytes disappeared while the retired mesh frontier reported only 4 released bytes. The same native PayloadRetirement now accepts the existing first-party owned_retirement cursor and RetiredMeshes transfers the complete MeshData, including attributes, materials, textures and component references. No additional registry or evaluator was added.

Actual native GREEN: 1 passed, 0 failed, 0 filtered; exact grant 1 item/3 bytes; 524296 bytes released over 184370 turns. Receipt: `🗑️generated/sol-2026-10-03/session-component-retirement/exact-cargo-laws-ah0s8v/00/law-0.stdout` and `.json`; Nx exit 0, 26.4 seconds. This proves this cleanup path only. Broader Session, analytic wire picking, application selection and fresh browser acceptance remain open.

## Analytic Component Native Coherence

Fresh actual native replay completed: 4 passed, 0 failed, 592 skipped, Nextest 0.025 seconds, Nx exit 0 after 6 minutes 42 seconds including the shared build queue. DEBUG confirmed four wire references resolve through the same kernel family and independent Parry perimeter 7; face area 1 and all malformed range/reference tables passed. Receipt: `🗑️generated/sol-2026-10-03/brep-component-wire-native-coherent-green.log`. Fresh all-11 Session ownership replay is now dispatched, not yet green. Actual application BRep component editing remains open.
