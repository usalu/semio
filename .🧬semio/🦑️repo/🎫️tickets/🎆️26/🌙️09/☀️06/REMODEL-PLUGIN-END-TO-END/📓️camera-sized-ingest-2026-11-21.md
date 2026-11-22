# Camera-sized reconstruction input

The reconstruction run decodes a stored still, measures sharpness on the decoded pixels, and attaches the capture's original intrinsics. It currently does not consume `downscale_long_edge_px`. The engine's 262,144-pixel limit is lower than the default 1,600-pixel edge setting for typical camera aspect ratios. The still decoder separately imposes 262,144 pixels, a 4,096-pixel PNG row limit, and 131,072 compressed bytes for JPEG. Durable raster storage and the reconstruction source enforce 1,114,112 bytes. AppendContent enforces the durable limit before reconstruction can see the input.

| Stage | Source and required behavior |
| --- | --- |
| Durable admission | `schema/📸️snapshot/🧩️asset-handles-and-durable-content` and the `AppendContent` validation in `🧬️schema/🔺️diff` define the byte and chunk limits. Input changes must maintain matching import, read-back, and retirement bounds. |
| PNG | `⚙️engine/🖼️images` uses the framework scanline decoder, whose constructor eagerly decodes Adam7 and verifies chunk CRCs. The newer `framework/🔲️pixels/📷️png/📥️decode` path has cooperative Adam7 passes. Feed bounded decoded samples into a resampling sink there, with cooperative header and row work. |
| JPEG | `stdio/📸️jpg/🚪️io/🪜️step-decoder` checks parsed dimensions, then initializes full component planes and finally a full RGBA raster. A bounded MCU-row mode needs resumable header and initialization work and a downsampling sink. Keep the original full-plane contract for the separate `🧮️decoded-components` consumer. |
| Reconstruction | `🧵️reconstruction-session/🏃️run-job` should choose working dimensions before sharpness and downstream feature stages, carrying the source dimensions for calibration. Keep durable source bytes and user-authored calibration intact. |

Use actual rounded output dimensions. With `sx = outputWidth / sourceWidth` and `sy = outputHeight / sourceHeight`, pixel-center sampling maps `x′ = (x + 0.5) sx − 0.5` and similarly for y. Scale fx and skew by sx and fy by sy, and apply the pixel-center transform to cx and cy. Keep distortion coefficients unchanged. Apply this transform only to the per-frame working capture.

Portable fixtures should cover an off-center, skewed calibration with unequal axis scales caused by integer rounding, a no-upscale case, and camera-sized PNG and baseline JPEG inputs. Existing framework triangle-resize fixtures compare against the independent Rust image crate. The full test should observe the imported bytes, output dimensions and pixels, transformed projection, and bounded progress or cancellation. PNG pixel equality and a declared JPEG tolerance allow a third-party codec oracle.

This is a read-only audit. It does not establish camera-sized media acceptance. No codec envelope or resize setting was changed while the component and native artifact builds were in progress.

## Bounded working resize

The reconstruction session now fits an admitted decoded frame to the requested long edge without enlargement, using bounded framework pixel-edit turns before sharpness measurement. Per-frame calibration is transformed on the actual rounded working grid. The shared resize fixture covers rounding and no enlargement; a native test compares its pixel result with the `image` crate, and the ingestion test covers calibrated streams.

The original compressed byte and decoded pixel caps remain in force. Camera-sized files that exceed those caps still cannot reach this resize; the durable admission and streaming codec work above remains necessary. The resize retains both source and working raster while sampling, within the existing 262,144-pixel source envelope. Static verification: `git diff --check` passed. No compilation or tests were run while the concurrent production builds were active.

The completed resize currently drops its source raster directly on handoff. The session's existing controlled retirement cursor is driven only during job close; retaining every source until then can accumulate memory for rejected or accepted frames. Effect-accounted prompt retirement requires a live cursor driven by the job's original effect between ingestion slices and included in cancellation and close ownership. Offline locked Cargo metadata resolved for the remodeling and hub workspaces. Fixture-sweep resolution could not finish offline because `rand` 0.8.6 was absent from the shared registry cache; the repo gate's incidental lockfile refresh was reversed with a guarded restoration of the preexisting targeted changes.
