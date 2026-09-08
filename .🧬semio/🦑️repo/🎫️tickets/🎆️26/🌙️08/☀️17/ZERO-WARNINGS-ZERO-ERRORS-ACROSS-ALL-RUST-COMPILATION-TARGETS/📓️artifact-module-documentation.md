# Artifact Module Documentation

Native pass 429 found E0753 in 36 Stdio artifact roots after crate imports were inserted before existing module doc comments. Pass 457 moves each affected module-doc block to the top while preserving all declarations and crate paths from the concurrent package extraction. 0 files were changed and 36 were already corrected when read.

Rustfmt parsed 36 of 36 files using edition 2021 with child modules skipped. This validates root-file syntax only; it does not resolve the extraction's pending crate imports, build targets or runtime behavior.


Pass 477 fixes missing source bases on artifact-local editor/viewer grouping modules after extraction. It inspects all 36 extracted Stdio artifact roots and adds path-dot annotations only where absent, after verifying the real component file relative to that artifact root. This preserves the new independent crate paths and avoids introducing nonexistent snake-case directories. Changed 0 artifact roots. Native compilation must be rerun.


Pass 482 applies 27 exact compiler suggestions across 6 extracted artifact roots: unnecessary crate qualification and unnecessary mutability. Full source lines, UTF-8 offsets and file contents were checked before mutation; 0 stale diagnostics were skipped. Unused-import diagnostics from failed macro expansion were deliberately excluded because the newly supplied dispatch macro dependency can make those imports live. Native and WASI checks remain required.
