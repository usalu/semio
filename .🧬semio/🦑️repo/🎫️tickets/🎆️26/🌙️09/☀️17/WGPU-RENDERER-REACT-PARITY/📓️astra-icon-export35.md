# Icon Export Effect Parity

Current source confirms that the Shooting app's `exportActiveShot` and `exportAllShots` commands emit `Effect::IconRenderExport`. React ShellHost processes every item by awaiting its `iconRenderPort.render(request)` and downloading the returned data URL.

WGPU Shell's `queue_host_effects` match handles `DownloadMediaExport`, file imports, extension requests and several other effects, but has no `IconRenderExport` arm. The icon export effect falls through to its `[DEBUG] wgpu-shell effect dropped` diagnostic. No alternate icon-export handler exists in the WGPU target or its tests. This is a confirmed functional gap independent of the Icon preview repairs.

The repair must route the existing neutral Icon request through an owned, cancellable export operation, render at the authored dimensions, preserve PNG/SVG format and masking, then deliver its encoded file through the existing platform download boundary. It cannot treat the visible preview's scaled framebuffer as the requested output or silently drop an export. A neutral request/result contract and actual React renderer output should validate the encoder path before the app export journey. Browser and native delivery both require verification. Production interfaces are held while build 27 runs.

## Byte Delivery and Shared Scene Construction

Preview construction now calls one `icon_render_world_scene` helper for camera, mesh URL, subject instance, environment, and presentation. The upcoming export job will call the same helper at scale 1. The material agent was notified to share this seam.

Browser and native media export now delegate accepted bytes to `present_media_export_bytes`, an async platform boundary returning success, user cancellation, or a write/transport error. Existing envelope decoding remains on the native IO lane and retains the shared kernel encoding contract. The native byte path chooses the file extension from the requested filename, avoiding `svg+xml` as an SVG file extension, and reports write failures instead of discarding them. The icon effect is still not wired and these changes have not been compiled or exercised as downloads yet.
