# Viewport Runtime Audit

## Source observations

No remaining interaction defect was confirmed in the inspected paths. The edit preview reads graph hover and selection from the framework interaction state, projects graph and wire targets onto instances, and attaches window-scoped grouped hover references (editor edit preview Rust, lines 198–208 and 384–453). Flow semantic rows bind hover to that same graph domain (editor edit flow Rust, lines 120–135). Preview group publication and lookup are scoped by window and revision (preview-eval preview-geometry Rust, lines 398–475). Viewer publication is conditional on successful rendering and prunes closed windows (viewer interaction topology Rust, lines 344–348). Channel-geometry and flow unit laws exercise grouped aliases, wire endpoints and semantic bindings; these do not establish a live renderer result.

## Runtime status and next verification

No browser runtime panel was present in the available computer-use inventory during this audit; no runtime interaction was verified and no build or artifact staging was started. Once both already-published renderer panels are available, use the same loaded generation in each: wait for settled nonempty preview geometry, hover a graph node, port and wire and inspect the corresponding exact preview instances; then hover an instance and inspect the matching graph ports, including coincident geometry and pointer exit. Repeat with two mounted previews and verify each window resolves its own published group. The existing demonstrator acceptance suite exposes generator preview and waits for settled geometry (acceptance TypeScript, around lines 340 and 990–1018); it is a useful setup reference but does not itself prove this bidirectional hover path.
