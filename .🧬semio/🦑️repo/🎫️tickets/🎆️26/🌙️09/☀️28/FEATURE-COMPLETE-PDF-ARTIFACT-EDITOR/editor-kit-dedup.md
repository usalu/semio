# Editor Kit Dedup

Goal: take the duplicated PDF subset editor wiring out of crate `semio-s-artifact-stdio-pdf-schema-5` (source-boundary gate 262,144 bytes).

## Structure

- New crate `semio-s-artifact-stdio-pdf-editor-kit` at `🧱️base/✏️editor/🧰️kit/📦️packages/🦀️rust` (no dependencies, registered in the PDF workspace members; schema-5 and `pdf-snapshot-value` depend on it).
- `🧰️kit/🧾️pdf17/🦀️.rs` defines `pdf17_editor!`: command enum, OpText/OpBinary, roster, view and presence lane stores, `ArtifactEditor` impl (render, render_with_peers, render_with_request_context, handle, native work), bounded native tools, manifest.
- `🧰️kit/🧾️pdf14/🦀️.rs` defines `pdf14_editor!` for the 1.4 canvas editors.
- Subset roots 1.7 `a`, `e`, `h`, `ua`, `x`, `vt` and 1.4 `a`, `x`, `base` now only import `edit`/`main` (and `page` for 1.4) and invoke the macro with their names, ids and literals (macro inputs that feed `literal` matchers cannot be `concat!`).
- 1.7 `base` is NOT folded in: it adds image import, native save, picker job and paged natural decode, so it stays its own root.

## Where to change things

All editor wiring (view lane, presence lane, rosters, render paths) now goes into the kit macro files. Expanded code is byte-identical to the previous roots except doc-comment text and the ordering of two items in `vt`.

## Verification

Equivalence of every root against its macro expansion was checked by a normalised line comparison before rewriting. 
- Roots rewritten: 1.7 a/e/h/ua/x/vt and 1.4 a/x/base shrank from about 23.5 KB / 20.8 KB each to 0.6-0.9 KB each. Source removed from schema-5: about 180 KB, added about 5.5 KB, so schema-5 is estimated near 87 KB (previous reading 263,220 bytes). Not measured by the gate.
- Compile: NOT yet verified. An earlier run by another agent failed with `cannot find crate semio_s_artifact_stdio_pdf_editor_kit` (it started before the dependency entries in the schema-5 and pdf-snapshot-value manifests and Cargo.lock existed). A fresh run (`🗑️generated/kit-schema5.txt`) was queued for 35+ minutes behind compiler image headroom with no result.
- Generators: `editor-kit-gen.py` and `editor-kit-apply.py` (apply re-checks equivalence before rewriting).
