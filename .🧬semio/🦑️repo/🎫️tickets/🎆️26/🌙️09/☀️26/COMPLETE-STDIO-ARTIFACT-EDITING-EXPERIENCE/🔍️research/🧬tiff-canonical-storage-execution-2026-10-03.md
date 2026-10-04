# TIFF Canonical Storage Execution

## Accepted design

Each TIFF image file directory owns one canonical raster storage form: no image payload, ordered strips, or ordered tiles. A chunk remains separate so its compression identity and the directory's authored strip or tile partition survive serialization. Offset and byte-count tag values are layout results and are rebuilt from the owned chunks; they are not competing persisted pointers.

The snapshot will not retain a second RGBA8 image authority. Display RGBA is an ephemeral projection of the selected directory. Natural region editing maps coordinates into lossless raw samples only for explicitly supported uncompressed chunky profiles and refuses compressed, planar, packed, or otherwise unsupported profiles until their own profile-specific edit path exists.

TIFF FLOAT and DOUBLE values own their exact IEEE binary32/binary64 words. This preserves signed zero, infinities, and every NaN payload across schema, native serialization, SQLite, and the source twin.

Page controls address an IFD ordinal together with the canonical document revision. Preview uses a browser-displayable first-party PNG projection with real width and height. Unsupported display profiles produce a visible diagnostic rather than an empty image or an assumed `image/tiff` browser decode.

## Validation plan

Neutral fixtures cover ordered multi-strip and tiled storage, exact IEEE words, and profile refusal. Rust and TypeScript consume the same fixture. Native byte evidence compares the writer with an independent TIFF decoder and verifies unchanged chunk partitions, sample projection, exact floating words, undo, stale revision refusal, cancellation, and terminal retirement. UI laws inspect the mounted page controls and PNG preview payload before browser acceptance.

Validation results are appended as each registered gate runs.

## 2026-10-03 execution receipt

The Rust and TypeScript snapshot twins now own per-IFD `TiffStorage`: the storage kind, authored offset/count word widths, and ordered chunks. ASCII values are exact octets and FLOAT/DOUBLE values retain exact IEEE words. Native decode captures authored strip/tile boundaries and native encode derives only physical offsets and counts. The SQLite projection has explicit directory storage and chunk relations and reconstructs every scalar domain without a root raster table.

The four document/baseline editor/viewer Main windows now project IFD 0 from canonical strip bytes to RGBA8 and encode that ephemeral image as first-party PNG. They publish the real width, height, and `image/png` MIME. Supported display profiles are 8-bit chunky grayscale/RGB/RGBA strips with no compression or PackBits. Tile, planar, packed, and other compressed profiles retain their canonical bytes and refuse only the ephemeral display projection. `ImageWindowKit::render_unavailable` keeps their image body mounted and renders the active shell locale's explicit `Preview unavailable` / `Vorschau nicht verfügbar` state, leaving sibling source and Details surfaces usable. The successful PNG path and its paged payload carrier are unchanged. The route still defaults to IFD 0; local page-selection controls and a cancellable retained preview encoder remain open.

Registered evidence:

- `tiff-canonical-rust-check-4.log`: canonical snapshot/IO/mutation Rust production mount passed `check`.
- `tiff-preview-rust-check-5.log`: PNG preview consumer mount passed `check`; Cargo 43.96 s, Nx 1 m 01 s.
- `tiff-canonical-typescript-check-3.log`: TypeScript production and contract sources passed `check`.
- `tiff-canonical-typescript-test-5.log`: 18/18 tests green across independent SQLite (5), ownership/cancellation frontiers (8), exact operation grants (2), and neutral Ajv/source canonical-storage laws (3).
- `tiff-preview-native-quick-9.log`: 78/78 native TIFF codec/schema laws green after the canonical storage and ephemeral PNG consumer migration. This run predates the registered target's component-feature repair and therefore does not prove editor/viewer component assembly.
- `image-window-unavailable-ts-typecheck-3.log`: the final shared TypeScript ImageWindowKit twin and neutral-fixture reader typecheck.
- `image-window-unavailable-ts-test-3.log`: 13/13 shared window-kit tests green, including the EN/DE mounted-unavailable state against the language-neutral JSON fixture.
- `image-window-unavailable-rust-test-1.log`: inconclusive first Rust ImageWindowKit attempt; compilation reached one test fixture include-path defect and no implementation diagnostic.
- `image-window-unavailable-rust-test-2.log`: corrected registered Rust ImageWindowKit witness green, 1/1; 976 unrelated laws skipped by the exact filter.
- `tiff-unavailable-mount-native-1.log`: inconclusive component-coverage witness; source compiled, but zero tests were selected because the TIFF router omitted `component-app-assembly`.
- `tiff-unavailable-mount-native-2.log`: component compilation exposed seven stale test-only references to the removed root pixel authority; production fallback sources compiled.
- `tiff-unavailable-mount-native-3.log`: the exact mounted-unavailable TIFF law is green, 1/1 with 101 unrelated component and codec laws skipped. The law proves a tiled IFD projection refuses explicitly while the German unavailable state mounts under the normal ImageWindowKit body.
- `tiff-component-native-full-1.log`: first full component-enabled run exposed two authored contract defects hidden by codec-only coverage: the baseline history fixture edited immutable `/schema`, and the set-snapshot input schema lacked localized field and option labels.
- `tiff-component-history-native-2.log`: red refinement run proved the field labels were read and narrowed the remaining refusal to enum option labels; it also proved the old patch witness did not supply an editable leaf.
- `tiff-component-history-native-3.log`: 4/4 exact history/input laws green after localized English/German option labels and a committed set-compression history fixture.
- `tiff-component-native-full-2.log`: 102/102 registered TIFF native laws green with `component-app-assembly`, zero skipped. This supersedes the earlier 78/78 codec-only qualification.
- `tiff-canonical-patch-pilot-ts-1.log`: 98/98 shared snapshot-editing and independent `fast-json-patch`/Ajv laws green after the TIFF pilot moved from the removed root pixels path to canonical IFD storage and byte-order vocabulary.

The normal TIFF native command is registered in both `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`; its router enables `component-app-assembly` without invocation overrides. Local page selection and retained cancellable preview work remain open.
