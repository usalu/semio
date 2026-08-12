# stdio lane brief — 53 facets in one crate

Supplement to `📓️fanout-brief.md` (which is normative; read it first). This file covers only what
is special about `✏️s/🔌️plugins/🗄️stdio`.

## Why stdio is different

All 53 mutation facets live in ONE crate (`semio-s-plugin-stdio`) whose single
`📦️packages/🦀️rust/📦️glue.rs` mounts every facet. That makes glue a contended file across the
whole lane, and `cargo check -p semio-s-plugin-stdio` a slow, shared gate. Hence the funnel
topology below.

## Topology

- **Sub-lane agents** own a set of artifact families and edit ONLY files under their families'
  `🗿️artifacts/<family>/**` subtrees. They must NEVER edit:
  `📦️packages/🦀️rust/📦️glue.rs`, the plugin root, `🎛️apps/**`, `🛂️manifest.json`, or another
  family's subtree.
- Whenever a sub-lane agent needs a glue mount (every new triad dir needs one) or any other
  shared-file edit, it appends a `sharedFileRequests` entry to its report AND to the shared queue
  file `📓️waveM-reports/stdio-shared-file-requests.md` in the ticket folder, in this exact form:

  ```
  ## <family>/<standard>/<subset>  (agent: <your lane name>, <timestamp>)
  MOUNT pub mod <snake_slug> {
    mutation = 🗿️artifacts/<family>/🏅️standards/<std>/🪆️subsets/<sub>/🧬️schema/🧬️mutations/<emoji><slug>/🦠️mutation/🦀️component.rs
    diff     = …/🔺️diff/🦀️component.rs
    inverse  = …/↩️inverse/🦀️component.rs
  }
  UNMOUNT pub mod <snake_slug_of_deleted_dir>
  ```

- The **stdio funnel agent** is the sole writer of the shared files. It drains the queue in
  batches, applies mounts/unmounts to `📦️glue.rs`, updates `🎛️apps/**` and `🛂️manifest.json`
  call sites, and runs the crate gate. Sub-lane agents never run `cargo check -p
  semio-s-plugin-stdio` more than once at the very end of their lane (the crate is huge and the
  cargo target lock serializes everyone).

## Family sub-lanes

| sub-lane | families |
|---|---|
| media-raster | `📷️png` `📷️jpg` `🖼️bmp` `🖼️tiff` `🎞️gif`(87a, 89a) `🎨️svg` |
| av | `🎵️mp3` `🔊️wav` `🎥️mp4` `📼️avi` |
| text-data | `📄txt` `📝️md` `📊️csv` `📑️tsv` `🔣️json` `📰xml` `🌐️html` `🌦️epw` |
| office | `📜️docx` `📕️xlsx` `🎞️pptx` `📄️pdf`(1.4, 1.7) |
| geometry-a | `🟪️stl` `🧊️obj` `☁️ply` `☁️las` `🧊️gltf` |
| geometry-b | `📐️step` `🏗️ifc`(2x3, 4) `💬️bcf` `🖊️dxf` `🖊️dwg`(ac1018, ac1024) |
| containers | `🎒️zip` `🗜️deflate` `💾️binary` |
| semio-1 | `🧿️semio` subsets `✳️mesh` `✳️brep` `✳️model` `✳️object` `✳️cad` `✳️drawing` |
| semio-2 | `🧿️semio` subsets `✳️image` `✳️video` `✳️audio` `✳️animation` `✳️presentation` `✳️document` `✳️workflow` |
| semio-any | `🧿️semio` subset `✳️any` — **LAST**, only after semio-1 and semio-2 are both done |

## Facet-specific warnings

- **Every** stdio facet's dispatch enum currently leads with `NoMutation` + `SetSnapshot`. Both
  are deleted with no replacement (`NoMutation` → inverses return `Vec::new()`; `SetSnapshot` →
  `ArtifactStore::reset`). The remaining `Set*` variants are per-field setters over an opaque or
  semi-structured document — most become `change-<field>`; large structured sub-payloads become
  `replace-<payload>`; authored content bodies become `edit-<body>`; ordered anonymous element
  lists become `insert`/`remove`/`reorder` with the BASE/FINAL index law.
- 52 of 53 facets have exactly ONE triad dir (`📄set-snapshot`). `💾️binary` and `📝️text` sitting
  beside it are **codec** dirs, not triads — do not treat them as mutations, but DO update them
  where they reference the banned vocabulary.
- **`🧿️semio ✳️image` is a trap**: it has 12 extra triad dirs
  (`📄set-dimensions 📄set-colorspace 📄set-bit-depth 📄set-icc 📄insert-frame 📄remove-frame
  📄move-frame 📄set-frame-delay 📄set-frame-pixels 📄set-metadata-entry 📄remove-metadata-entry`)
  that are NOT mounted in glue (dead, uncompiled scaffolding) and whose leaves are pure
  apply-and-capture (`apply_semio_image_mutation(snapshot, &SemioImageMutation::SetDimensions{…})`)
  — the exact banned anti-pattern. Rewrite them as real diff/inverse triads with approved verb
  names and mount them; do not trust or copy their current content. They also all share the `📄`
  emoji, which violates the per-facet emoji-uniqueness rule — assign unique ones.
- **`🧿️semio ✳️any` is a union dispatch**: 13 pass-through variants (`Brep Mesh Model Object
  Document Cad Drawing Image Video Audio Animation Presentation Workflow`) delegating to the
  sub-subset enums. It is not a field-derivation problem. Migrate it only after all 13 sub-subsets
  are migrated, and keep it a delegating dispatch — each variant wraps the corresponding subset's
  mutation type, and its `MutationKind` semantics describe the delegation.
- `🖊️dxf`'s `🔺️diff/🦀️component.rs` already has a local `named_apply()` helper — reuse it rather
  than inventing a parallel engine, and consider whether the shared
  `protocol::named_apply`/`indexed_apply` from the framework's DiffKit region fits better.
- Several facets carry header comments claiming `#[derive(dsl::DslOps)]` fails on their enum
  (e.g. `☁️ply`). Investigate before believing it — the usual cause is a variant shape the derive
  rejects, which the migration fixes anyway by moving to single-tuple payload variants.

## Size buckets (variants excluding NoMutation/SetSnapshot) — for lane pacing

- trivial 0–3: `📄️pdf 1.4`(0) `🎵️mp3` `🔊️wav` `🏗️ifc 2x3` `🖊️dwg ac1018` `🗜️deflate` `💾️binary raw`,
  `🖊️dwg ac1024`(4)
- small 4–8: `📊️csv` `📝️md` `📄txt` `📑️tsv` `🟪️stl` `🖼️bmp` `📰xml` `🖼️tiff` `🎞️pptx`
  `semio ✳️video` `☁️ply` `🌐️html` `📕️xlsx` `🎨️svg` `semio ✳️model/✳️audio/✳️object`
- medium 9–15: `🎥️mp4` `🏗️ifc 4` `📐️step` `🎞️gif 87a` `☁️las` `📜️docx` `🌦️epw` `🎒️zip` `💬️bcf`
  `📄️pdf 1.7` `📷️jpg` `📼️avi` `semio ✳️animation/✳️image/✳️workflow/✳️presentation/✳️mesh/✳️cad/✳️document`
- large 16+: `📷️png`(15) `🖊️dxf`(18) `🎞️gif 89a`(19) `semio ✳️drawing`(16) `🧊️obj`(20)
  `semio ✳️brep`(21) `🧊️gltf`(22)

## Gate discipline

Because the crate is huge and shared: sub-lane agents run `cargo check -p semio-s-plugin-stdio`
ONCE at the end of their lane (not per facet), and expect to wait — other agents hold the cargo
target lock. If the check fails on a family that is not yours, that is another lane mid-flight:
report it, do not fix it. The funnel agent runs the authoritative crate gate after each drain.
