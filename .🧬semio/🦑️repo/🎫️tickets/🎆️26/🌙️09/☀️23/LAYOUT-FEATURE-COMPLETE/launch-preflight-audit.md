# Layout Launch Preflight Audit

Source inspection on 2026-11-20; no listener started or live success claimed.

- The WGPU configuration and shared serveVite listener do not supply COOP or COEP. The route probe requires same-origin and require-corp or credentialless. The existing playgroundIframeEmbedHeadersPlugin sets same-origin and credentialless but is not consumed by the WGPU configuration. The nearest WGPU route test is 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/🟦️.ts. Verify real headers after launch.
- The Specific generated catalog exists and its plugin inventory contains Layout; its playground row selects the Specific WGPU composition. The source staging script invokes the session owner as session layout, activation as activate layout wgpu prototype, and the WGPU owner as serve layout prototype --port 6179, with SEMIO_PLUGIN_CATALOG and SEMIO_WGPU_COMPOSITION_PATH set. The session output directory exists under .cache/b/p/m/browser/prototype/sessions/layout. This does not establish completed activation or usable guest bytes.
- WGPU Vite requires renderer JS and WASM, browser boot, renderer boot, frame worker and the font before configuration succeeds; it reads the activation completion marker when the listener configures. Its mounts serve guest and renderer artifacts with a missing-artifact 404 boundary. The route probe checks those artifacts and rejects HTML fallback, but only a live run establishes their availability.

## Recheck on 2026-11-21

The launch, linked-asset and gumball witness scripts passed through the repository Bun tool. The linked-asset check includes both registered examples, two UTF-8 transfer boundaries, exact frame selection and the embedded PNG. The gumball check uses Sharp to validate the physical journey’s visible-handle and changed-paint detectors. These checks run before browser host activation.

The physical hover report records time from each pointer dispatch to the first observed painted or cleared stroke. These timings include browser observation overhead, and the detector still checks painted pixels with Sharp and verifies that the pointer moves leave selection empty. Syntax and the existing Sharp detector check passed.
