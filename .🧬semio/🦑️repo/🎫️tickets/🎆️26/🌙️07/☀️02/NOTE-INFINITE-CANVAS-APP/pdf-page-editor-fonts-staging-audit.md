# PDF page-editor font dependency staging audit

Attempt 22 (driver `wjNmucY`) reported unresolved `semio_s_artifact_stdio_pdf_fonts` imports in the PDF page editor's metrics, Type 3 scene text, and TrueType scene text modules.

The shared page-editor manifest now declares `semio-s-artifact-stdio-pdf-fonts` as a direct dependency. Its relative path resolves to the 1.7 base font package with that exact Cargo crate name. The font crate publishes `FontCodec`, `base_encoding_table`, and `truetype::TrueTypeOutlineSegment` through the paths used by the three page-editor imports.

Every located repository `Cargo.lock` block for `semio-s-artifact-stdio-pdf-page-editor` already lists the font crate. This covers the Hub, PDF, presentation, drawing, layout, and shooting workspaces. The audit made no source or lockfile edits and started no competing Rust build. The next Note guest staging run will verify the fresh dependency graph.
