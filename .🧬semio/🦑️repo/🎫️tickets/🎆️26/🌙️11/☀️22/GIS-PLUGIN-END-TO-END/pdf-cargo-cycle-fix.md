# PDF cargo cycle and surface oracle dev-dependency

## Findings (checked 17:0x)

1. Cyclic package dependency (`semio-s-artifact-stdio-pdf-tests-2` <-> `page-editor`): no longer present. Page-editor's manifest (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🖼️page/📦️packages/🦀️rust/Cargo.toml`) has no `tests-2` dependency anymore; the other lane removed it. No edit was needed from this lane.
   Lock-only resolution passes for both workspaces (no cycle):
   - gismap: `repo cargo ... -- update --offline --workspace --manifest-path <gismap Cargo.toml>` -> `Locking 0 packages` (log `pdfcycle-update-1.log`)
   - pdf: same for the pdf `Cargo.toml` -> `Locking 0 packages` (log `pdfcycle-update-pdf.log`)
2. `semio-framework-surface` already declares `[dev-dependencies] serde_json = "1.0.140"` (test `tiled_map_mercator_oracle`). No edit needed.

## Compile checks

Not run to completion: the shared compile lock was held by `gismap-lane` (since 16:25, an active gismap compile) for over an hour. Queued runner: `pdfcycle-all.sh` (lock, `check --tests -j 3` for gis2d + gismap + gismap-viewer in one invocation, then pdf page-editor, 120 s pause after each release); logs `🗑️generated/pdfcycle-gismap.log`, `pdfcycle-pdf.log`, marker `pdfcycle-all.done`.
