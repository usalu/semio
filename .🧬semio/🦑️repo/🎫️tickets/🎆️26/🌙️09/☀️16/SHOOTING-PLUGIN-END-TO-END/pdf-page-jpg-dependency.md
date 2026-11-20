# PDF Page Editor JPG Dependency

The native Shooting run reported E0433 at the PDF page editor’s JPG import. The shared worktree already carried the correction when inspected. The page editor’s manifest declares the first-party `semio-s-artifact-stdio-jpg-inferences` path dependency and the path resolves to the expected Cargo package. The crate publicly mounts `JpgComponentDecoder` under the imported snapshot module and reexports `ycbcr_to_rgb` through its document I/O module.

All six first-party locks containing the page editor record the same JPG dependency edge. These are Animate presentation, Shooting, Layout, Draw, stdio PDF and Hub. The root lock has no PDF page editor record. The current import, dependency declaration and lock edges agree, so no source or lock edits were needed.

This audit checked the live source and locks. The managed native retry must establish compilation after admitting the current worktree state.
