# PDF page editor TrueType dependency

Native Shooting compilation reached the PDF page editor and reported Rust errors E0432 and E0599. The editor imported `FontCodec` and the TrueType outline segment from the PDF content mutation test mirror. The mirror's font codec predates the outline API. The PDF font library already owns `FontCodec::outline_program`, `TrueTypeOutlineSegment`, and the encoding table used by Type 3 text.

The page editor now declares the PDF font library and imports its APIs directly in font measurements, TrueType painting, and Type 3 text. The six lockfiles with a page editor package entry record the new dependency edge. Its font decoding, outline conversion, and rendering logic are unchanged.

The existing `canvas_paints_the_embedded_true_type_program_as_curved_outlines` test in `semio-s-artifact-stdio-pdf-page-editor-tests` reaches the production editor through its reexport. It uses the shared TrueType outline JSON fixture to check path geometry, text decoding, and glyph advances. The manifest path, import paths, public methods, and lockfile edges were checked in source. Compilation and test execution are pending native build capacity.
