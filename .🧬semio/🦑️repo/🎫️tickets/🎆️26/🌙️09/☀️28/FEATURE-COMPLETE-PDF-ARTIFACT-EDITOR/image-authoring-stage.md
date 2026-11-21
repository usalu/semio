# PDF local PNG authoring overlay

The overlay replaces the mounted PDF 1.7 base editor's gray placeholder action with a local PNG picker. The Page action “Insert PNG Image” stages page index and page-space rectangle. An image selected in the Object inspector offers “Replace Image with PNG.” The corresponding controls remain specific to the base editor. Page geometry is distinct from the encoded pixel dimensions.

The standard shell picker applies a 24,546-byte file limit and retains import progress and cancellation. The callback accepts the single action chunk used by this bound. The bounded browser and native file adapters label these bytes as `application/octet-stream`. The callback normalizes only the wrapper; the retained PNG codec checks the bytes. The 4,096-pixel import cap keeps an RGB u32 component array below the guest's 65,536-byte contiguous allocation ceiling. The scope supports PNG thumbnails and small diagrams. Typical photos and JPEG are outside this stage.

The callback is typed, owns its cooperative PNG decoder using `ControlledRetirement`, and carries the full canonical revision captured before the picker opened. The selected replacement button also carries the exact 64-bit published inspector revision, which the picker action checks when it opens. A moved or replaced target therefore refuses before applying bytes. The callback uses a dedicated progress/cancellation tool factory and publishes only after decoding finishes.

The typed producer installs a fresh image resource and optional grayscale soft mask to preserve alpha. Insert appends one isolated graphic scope. Replace swaps exactly the targeted paint operator. This also works for inline images, which become independent XObjects at the same operator position. The previous resource and its other uses stay intact. Existing draw transforms and surrounding operators remain in place. The new image's sample-dependent Decode, mask, matte, intent and related attributes start from the normal sampled-image defaults. The publication reserves three point-invertible mutations for one undo/redo transaction.

## Staged files and integration order

The paths below are relative to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base`.

| Path | Change |
| --- | --- |
| `✏️editor/📄️document/🦀️.rs` | Wire the document owner's image module |
| `✏️editor/📄️document/🖼️image-authoring/🦀️.rs` | Typed placement validation and sample mutation producer |
| `✏️editor/📄️document/🖼️image-authoring/🎛️replace-button.rs` | Localized selected-placement control |
| `✏️editor/📄️document/🖼️image-authoring/🧵️decode.rs` | Retained PNG decode, guarded work and incremental retirement |
| `✏️editor/📄️document/🖼️image-authoring/📥️admission.rs` | Picker argument contract, host effect and publication factory |
| `✏️editor/📄️document/📦️packages/🦀️rust/Cargo.toml` | Direct edges to existing `semio-framework-pixels` and `semio-s-artifact-stdio-contract` |
| `../🧾️vt/🧬️schema/📦️packages/🦀️rust/Cargo.toml` | Schema-5 controller's direct edge to `semio-s-artifact-stdio-pdf-editor-document` |
| Tracked PDF consumer lockfiles | Three dependency lines in the two existing PDF package records in each matching lock |
| `✏️editor/🦀️.rs` | Base-only action roster, typed commands and factory |
| `✏️editor/📥️local-image/🦀️.rs` | Bridge base-controller commands to the document owner |
| `✏️editor/🖼️page/🪟️windows/🦀️.rs` | Four lines joining the replacement control to the selected image |

The boundary draft stage touches the page `🔎️inspector`, the native page edit test and framework controls. The image overlay reads the publication revision in `🪟️windows` and uses no boundary editor field code. The font stage touches `🖱️object-edits`, `🧮️content-ops`, `🔤️metrics`, codec files, and existing editor tests. The image producer uses the granular page-content mutation APIs directly so those files have no overlap. The image stage composes from the current source at execution time. Run it after the preceding overlays are on disk so all its anchors reflect the combined source. The page owner gains only the four lines that attach the replacement control.

The initial controller-owned implementation exceeded the repository's 4,000 active-line boundary. The staged controller now retains its typed command mapping and connects to the document owner for picker admission and decode. The document owner already uses `semio-plugin` and the PDF mutation/snapshot crates. The staged decoder imports retirement and retained work through `semio-plugin::__value` and `semio-plugin::__job`. Its direct PNG dependency is the existing repository pixels crate. The existing stdio contract supplies the typed argument reader. Schema-5 already declares `semio-plugin`, `semio-framework-value` and its derive, `semio-framework-dsl-record-derive`, `semio-os-kernel`, `semio-ui-locale` and `semio-s-artifact-stdio-contract`. Its existing path for rendering is schema-5 → PDF tests-2 mirror → Page owner. The new document-owner dependency is internal. The three new manifest edges and corresponding records in all seven matching lockfiles were validated in the overlay.

The canonical repository Rust closure reader mounted all new modules. With test configuration, it measured 6 files and 782 active lines for the document owner, 24 files and 3,981 active lines for schema-5, and 22 files and 3,969 active lines for the page owner. With the existing boundary and font overlays loaded into the same source reader, the page owner measured 22 files and 3,994 test-active lines. The document and controller measurements stayed at 782 and 3,981 lines.

The stage writes a reviewable diff under the ticket’s generated folder. The `--apply` switch writes production sources and is reserved for the release owner. No production file was written while developing or testing the stage.

### Lockfiles

The stage discovers tracked Cargo lockfiles that already resolve the PDF editor-document package. It inserts the declared pixels and Stdio contract dependencies in that package's existing row and the declared editor-document dependency in the existing schema-5 row. It preserves the original line order and newline sequences and can run the insertion twice without changing the result.

The reader verified both rows in all seven matching locks. It compared the resulting entries to the staged normal Cargo manifest edges and checked the other package metadata and dependency order against the original lockfiles.

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.lock`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.lock`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock`
- `🌎️hub/Cargo.lock`

The repository root `Cargo.lock` contains neither affected owner. Another 25 locks contain schema-5 without the existing editor-document package. Those older dependency graphs require broader lock resolution to add absent packages. This overlay scopes its lock edits to the seven locks with the editor-document package already present.

## Verification

`image-authoring.json` supplies a 2 × 2 lossless PNG, straight component samples, soft-mask channel, intended placements, and the pixel probes. The exact mutation source compiled and passed the focused Rust harness, including malformed extents, opaque alpha, both replacement kinds, one untouched page sharing the old XObject and an image/form ID collision. PyMuPDF 1.24.14 independently decodes the PNG and renders saved/reopened reference outputs with all four colors and both replacement modes.

The complete staged Rust controller, decoder, typed producer and replacement widget type-checked against the fresh real repository WASI libraries. The type probe uses the existing page library plus a harness around unrelated current editor entrypoints. It does not run the mounted job or verify its transaction settlement. An older WASI lane mixed incompatible cached metadata, so its probe was rejected. The fresh lane passed.

```text
repo tools exec --light python -- <ticket>/image-authoring-stage.py --picker
repo tools exec --light python -- <ticket>/image-authoring-stage.py --closure --neighbors
repo tools exec --light uv -- run --no-project --with pymupdf==1.24.14 -- python <ticket>/image-authoring-check.py
repo tools exec --light python -- <ticket>/image-authoring-types.py .cache/b/r/m/u/l/p32ef76
```

The actual-catalog browser scenario `image-authoring-browser.ts` transpiled. It opens the independently generated shared-XObject and inline-image seed through the real shell picker. It enters the Insert PNG geometry, selects a file, replaces only the addressed XObject and inline occurrences, checks one undo/redo, cancels a picker, and downloads and reopens the resulting PDF. A companion PyMuPDF oracle checks pixels from these actual downloads against the original fixture. This scenario needs the activated catalog with the new image overlay; it was prepared but not run against the gates' frozen source.

```text
repo tools exec --light uv -- run --no-project --with pymupdf==1.24.14 -- python <ticket>/image-authoring-check.py --prepare-browser <ticket>/🗑️generated/image-authoring
repo tools exec --light bun -- <ticket>/image-authoring-browser.ts http://127.0.0.1:6260/ <ticket>/🗑️generated/image-authoring
repo tools exec --light uv -- run --no-project --with pymupdf==1.24.14 -- python <ticket>/image-authoring-check.py --verify-browser <ticket>/🗑️generated/image-authoring
```

Remaining verification includes mounted progress, operation cancellation after decode begins, stale picker targets and starvation of the host work grants. Supporting normal photos requires revision-bound multi-chunk or binary admission and a paged/encoded representation that avoids the single component-array ceiling.
