# Raster User Flow Wiring Audit

Current source at ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖥️app/🦀️.rs:373 selects RasterImportJob for import-media, cloning port and the entire media value before passing the request. The live job at 🎞️reserved-import/🦀️.rs accepts only Structured input and synchronously calls raster_image_layer_and_asset; the helper in 🚪️io/🧩️composite-and-media/🦀️.rs:254 decodes all base64 and PNG at once. The job charges one fuel after the whole operation. Thus import lacks original input custody and bounded decode/progress/cancellation on this route. The framework retained preparation alone does not establish Raster import acceptance.

In 🎞️reserved-import/🦀️.rs:132 the job takes the mutation vector before checking completion authority and mounted consumer. Either absent-authority fault drops the candidate without funded close and cannot retry the handoff. Validate authority before taking the vector and retain the actual rejected completion on a failed handoff.

These are current source findings only; no native tests or live user workflow were run. Active runtime ownership work may change the route concurrently.
