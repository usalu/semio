# PDF page editor crate split under the source-boundary ceiling

## Blocker

`semio-s-artifact-stdio-pdf-page-editor` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/📦️packages/🦀️rust`) measured by the ticket probe at OVER 28 files / 303,589 bytes against 32 files / 262,144 bytes. The lib mounts all sibling module folders of `🖼️page/` by `#[path]`.

## Concurrent work

Another agent split `✍️authoring` (crate `semio-s-artifact-stdio-pdf-page-editor-authoring`) and `🔭️view` (crate `semio-s-artifact-stdio-pdf-page-editor-view`) out of the same crate while this change was in progress. This change builds on top of those and mounts nothing of them. Their authoring crate had an unrelated compile error (unresolved `mut_base_4` io imports) during part of the session.

## Change (this agent)

- New crate `semio-s-artifact-stdio-pdf-page-editor-base` at `🖼️page/🧱️model/📦️packages/🦀️rust` (`Cargo.toml`, `📋️project.json`, `🦀️.rs`), registered in the `📖️pdf/Cargo.toml` workspace members next to its siblings. Its lib repeats the root import header and mounts by path the base layer modules: `🧱️model`, `🗺️scene`, `🧮️content-ops`, `🔤️metrics`, `🧭️geometry`, `🧰️arguments`, `🖌️graphics`, `🧮️function-colour`, `🌄️shading-preview`, `🔆️image-preview`, re-exporting each with `pub use`. It depends on the `-view` crate (scene needs `page_viewports`, `rect_segments`) and on the editor-document crate (`fault`, `append_advance`).
- Dependency direction: base never references windows, actions, inspector, edits, text layer or page text (verified by compiling it alone), so no cycle.
- In the moved module roots `pub(super)` and `pub(crate)` became `pub` so the glob re-export reaches the page crate.
- Page crate lib: the ten moved `mod` mounts removed, one `use semio_s_artifact_stdio_pdf_page_editor_base::*;` added, and the dependency added to its `Cargo.toml`. Remaining mounted modules (`🪟️windows`, `🎬️actions`, `🖱️object-edits`, `📄️document-edits`, `🔖️annotation-edits`, `🎨️resource-edits`, `🔎️inspector`, `♿️text-layer`, `📝️page-text`) keep their `use super::*` paths because the glob re-brings every moved item to the root. Public paths stay stable.

## Sizes (non-test Rust source, bytes, by hand sum)

- Base crate: about 111 KB (14 files).
- Page crate after the split, also without authoring and view: about 154 KB.
- Both well under 200 KiB and 32 files. The ticket probe was not rerun after the split because the repo tool chain was failing (see below).

## Checks

- `repo cargo --store=shared --configuration=prototype -- check -p semio-s-artifact-stdio-pdf-page-editor-base --lib` from the `📖️pdf` workspace: Finished, no errors.
- Check of `-p semio-s-artifact-stdio-pdf-page-editor --lib`: NOT confirmed. For more than 40 minutes the run was repeatedly stopped by unrelated concurrent breakage in dependencies (`semio-plugin-instance-app`, `-render`, `-jobs`, `-job-receiving`, `semio-os-dsl`, `graph-projection`, and the authoring crate), and by exit 75/capacity restarts. A retry loop (`/tmp/chk.sh`) was still running at the end of the session; no error in the page or base crate sources had been reported by it.
- The probe (`bun <ticket>/native-source-boundary-preflight.ts /Users/ueli/Documents/semio "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"`, takes about 2 minutes) must be rerun after the dependencies compile again.

## Files

- Added: `🖼️page/🧱️model/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,🦀️.rs}`
- Changed: `📖️pdf/Cargo.toml` (members), `🖼️page/🦀️.rs`, `🖼️page/📦️packages/🦀️rust/Cargo.toml`, and visibility (`pub(super)`/`pub(crate)` to `pub`) in `🖼️page/{🧱️model,🗺️scene,🧮️content-ops,🔤️metrics,🧭️geometry,🧰️arguments,🖌️graphics,🧮️function-colour,🌄️shading-preview,🔆️image-preview,🔭️view}/🦀️.rs`.
