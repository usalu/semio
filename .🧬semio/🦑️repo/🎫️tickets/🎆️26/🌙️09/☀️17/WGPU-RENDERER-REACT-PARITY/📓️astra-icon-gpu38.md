# Request-Sized GPU PNG Export

Added a neutral 65×3 transparent raster fixture, whose width requires aligned GPU readback rows. The Chromium Canvas PNG and Sharp oracle passed 2/2 through the React Nx target (exit 0, 11.3 seconds).

The new WGPU export job takes an admitted prepared packet into a separate target on the same device. It advances raster ownership, uploads, presentation, asynchronous readback and PNG encoding, then retires cursor, raster, mesh and packet owners before allowing one publication. Missing world meshes fail the export instead of silently delivering an incomplete image. Cancellation clears the candidate and follows the same retirement path. A window-free device constructor supports standalone requests and actual native GPU laws.

Two native laws cover exact output probes, one-shot publication and cancellation at all six exposed stages. They are written but not yet run. The Shell effect and user-visible progress/cancellation are not connected yet, and SVG serialization remains open. The material agent owns request-sized scene/asset assembly; the root owns effect and delivery orchestration.
