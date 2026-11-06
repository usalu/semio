# Raster Import Fixture Paths

The Raster model schema publicly reexports SemioImageSnapshot and the layer model exports RasterLayerNode. The editor can use `semio_s_artifact_raster_raster_model::{SemioImageSnapshot, RasterLayerNode}`.

A small valid fixture is `semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1, 1, vec![0, 0, 0, 255])`. It creates the existing schema, RGBA colorspace, 8-bit depth, one frame, no ICC and no metadata, without cloning. Save `image.frames[0].rgba8.as_ptr()` before moving the image into `RasterImagePayload::from_owned`; on refused admission the error tuple returns the original image, enabling comparison of the same pixel pointer. Fixture creation itself allocates and should occur outside the measured granted construction turn.

`RasterImagePayload::owned_birth_demands()` quotes the shared-root admission; `from_owned` returns the original image with the error if refused. After successful conversion the payload requires controlled retirement. A retained two-mutation vector and boxed layer likewise need their actual admitted construction and retirement, rather than ordinary dropping on denial.

At inspection the reserved import source still showed the pre-change job and did not yet contain `from_owned`; the concurrently edited candidate was therefore not ready for source review. No native build was run.
