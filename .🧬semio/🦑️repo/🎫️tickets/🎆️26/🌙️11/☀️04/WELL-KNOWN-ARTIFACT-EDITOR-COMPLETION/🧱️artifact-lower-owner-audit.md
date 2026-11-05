# Artifact Lower Owner Dependency Audit

Read-only source search; no compilation launched. Active noncomment crate::engine references across stdio artifact sources are confined to OBJ, STL, PLY and LAS. Other matches are historical documentation. Medium already owns OBJ/STL/PLY canonical lower-codec/inference repairs and is correcting remaining editor unit calls to direct lower owners.

## Confirmed LAS Production Leak

LAS header editor source at native encode/decode lines190 and194 calls crate::engine::encode_las/decode_las. The actual semio-s-artifact-stdio-las-mutations package root (header/schema/mutations/packages/rust source) mounts that editor directly, but defines no engine module or reexport. The upper artifact root engine reexport cannot provide a crate module inside this dependency. This is a concrete unresolved production reference, not merely a test façade concern.

LAS edit-main24 and viewer-main23 also call crate::engine::scene_geometry, while their source bodies are mounted into lower snapshot/schema component crates. The mutation wrapper reexports those lower main bodies. The correct dependency direction is to expose codec/geometry from their canonical lower owned module and call that owner; adding an upper tests2 dependency would recreate the cycle. Exact paths and source evidence sent to medium.

Within remaining14 requested artifact families (OBJ/PLY/STL/JPG/DXF/EPW/ZIP/DWG/BCF/TXT/TSV/SVG/CSV/IDML), no additional active engine references beyond the three already-owned mesh repairs were found. cfg(test)-only public demo/helper functions exist in ZIP/OBJ/SVG/DWG, but this audit did not establish a currently mounted cross-crate consumer failure for them; do not claim unresolved symbols solely from their existence. In particular, old source unit files may be unmounted after carving. Real queued compilation remains required to prove these dependencies.

## LAS Repair Review

The relocated natural module path resolves to the existing IO/natural file; schema wrapper edit-main anchor resolves correctly. Authored Rust search across s/framework finds no remaining semio_s_artifact_stdio_las_snapshot::editor reference. Local manifest graph remains acyclic: snapshot has no schema edge; schema depends snapshot; mutations depends schema/snapshot/lower-tests; lower-tests depends schema/snapshot; tests2 and root sit above them. The added independent oracle is dev-only and does not depend on LAS production crates. Natural codec imports resolve from schema root and IO reexports preserve exact aliases.

One remaining actual test leak was found after repair: editor source mounts editor/unit under cfg(test), so mutation --lib tests compile unit line110, still calling crate::engine::scene_geometry without an engine module in mutation root. Reported to root for a direct lower-owner call. Remaining mutation-unit engine codec calls belong to the tests2 wrapper, which explicitly has an engine mirror; do not conflate those with the directly mounted editor test. No Rust compilation was run by this audit.

## Final Natural Codec Boundary

All moved private codec helpers and header/VLR offset modules are consumed inside the new leaf. External search found no dependency on those formerly lexical IO helpers; oracle read_fixed_str is independently defined, and binary diff reader methods are unrelated. External adapters need only public encode_las/decode_las, both reexported from IO, and geometry has its explicit lower alias. Root corrected the remaining directly mounted editor unit engine reference.

The neutral layering/Ajv test checks ownership structure, not native Source admission or build integrity receipts. The new leaf is explicitly mounted by Rust path. Its bytes and changed owner closure must pass authoritative native compilation and source-bound/receipt recalculation; no manual receipt rewrite is supported by this audit. Source composition app IDs, schemas and codec signatures remain unchanged.
