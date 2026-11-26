# Draw Viewer Browser Acceptance

Run from the repository root after the new component is published and the server is serving that publication.

```sh
T='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END'
repo tools exec --light bun -- "$T/attest-draw-journeys-26.ts" viewer-final26 '<NEW_COMPONENT_SHA256>' 'http://127.0.0.1:6064/' viewer
```

The Viewer mode uses the same receipt and served core reconciliation as the editor journey. It starts a fresh Chromium context for each locale and captures the Draw core requests from that page. It verifies the response status, URL, length and hash of each served core, including actual Vite query strings. The before and after model, served component receipt, semantic descriptor and all three published cores must agree for each locale.

The browser journey clicks the localized Viewer button, waits for the Viewer canvas and pressed state, and checks that the Editor canvas has retired. It wheels at the center of the actual visible canvas. The screenshot oracle measures the Draw demo's orange artwork in the canvas bitmap. It requires artwork area to grow by at least 8% and both measured dimensions by at least 5%, with the original bitmap size. The final Editor click must retire the Viewer canvas, activate the original document and space in Editor, show the localized Catalogue/Katalog control and repaint the same demo.

The attestation report is written to `🗑️generated/attestation-viewer-final26.json`. Each locale records its own proof and the full rendered `viewer.png`, `zoom.png` and `editor.png` under `🗑️generated/viewer-viewer-final26-<locale>`. Both sets must be visually checked for the painted drawing, larger geometry and restored editor chrome.

The syntax check and independent Python fraction oracle passed. The JavaScript paint oracle agreed for all four neutral cases and both historical screenshot sets after restricting those screenshots to the canvas area. The historical orange area increased from 81,493 to 99,174 pixels in each locale. Viewer mode rejected a deliberate zero pin before any browser started. Fresh two-locale results are pending publication.
