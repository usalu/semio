# TIFF Current Fidelity And Preview Frontier

Source inspection only; no TIFF native or browser receipt was run in this inspection.

The actual document subset snapshot now has both root `pixels` and per-IFD `pixels`. The top module documentation claiming no later-directory raster owner is stale: current decode retains later raw strips and the writer consumes them. Those two fields have different meanings: root RGBA8 versus later raw strip bytes. They cannot serve as a uniform editable image plane.

The native writer calls `rgba_to_rgb` for the first image and regenerates nine geometry/strip tags. It writes RGB8, one strip, and compression chosen by which export function is called. Thus editing stored SamplesPerPixel, BitsPerSample, photometric, rows-per-strip or compression does not directly control saved first-image storage. Alpha is absent from the RGB output path. Later images retain raw strip bytes but their original strip boundaries are combined into one strip and their RowsPerStrip is rewritten. A preservation claim needs per-directory sample storage and explicit encode-profile authority rather than a metadata tree whose declared storage fields are overwritten.

The decoder accepts only first-image eight-bit, one/three/four sample channels and uncompressed/PackBits raster decoding. This is verified from its branch conditions, not an assertion about external TIFF readers. Separate raster parsing and preservation would allow a document to retain a supported TIFF container and show an explicit unsupported display profile without destroying its payload.

Both current document editor/viewer paths encode the entire TIFF for each preview, put that payload into an image/tiff URI, and provide zero width/height in ImageView. The document editor Main surface is explicitly read-only; actual edits live in Details. A display projection should provide real dimensions, page selection and a browser-displayable preview format via first-party codec/domain-neutral image surfaces. Browser display support itself has not been tested here.

Next execution cut: schema-first per-directory canonical tag/sample/strip/tile ownership (including literal unknown bytes and exact IEEE words); complete native profile-preserving serialization and explicit conversion commands; named page/address editing with exact inverses and stale revisions; bounded progress/cancellation; independent TIFF decoder and byte-span preservation fixtures; real multipage preview/edit/save/reopen browser acceptance. BMP now has a dedicated execution lane; TIFF should follow the media live-route checkpoint rather than silently remain a read-only image plus raw fields.

Source anchors: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🦀️.rs` (TiffIfd/TiffSnapshot); adjacent `🚪️io/🦀️.rs` (read_raw_strips, decode_pixels_from_ifd, rgba_to_rgb, encode_tiff_with); adjacent `✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs` (image_view).
