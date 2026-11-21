# Staffage release browser smoke

Run [smoke.mjs](./smoke.mjs) under Node with the workspace Playwright dependency and a Chrome installation. Supply the production release directory or the public site URL. The script exits nonzero if an assertion, an ordinary browser request, or a runtime check fails. Set `STAFFAGE_CHROME_CHANNEL=chromium` to use Playwright-managed Chromium, or set `STAFFAGE_CHROME_EXECUTABLE` to select an installed binary. The Chrome launch disables crash reporting because it prevented headless startup on the macOS verification host.

The local mode serves compiled files on a temporary loopback port, uses the generated `404.html` with HTTP 404 for misses, and checks the recovery paths. Pass `--recovery-only` after the directory to repeat just the recovery and static 404 checks. A live URL exercises the distributed site without simulating faults there.

| Scenario | Expected behavior |
| --- | --- |
| English desktop, German phone with an unsupported browser locale | Choose a language, retain it on reload, navigate to Humans and Walking Commuter, draw the asset in three representations and start playback. The current phone overview provides a 44 CSS pixel touch target and a compact list. |
| Catalog routes | All Assets shows authored links at both viewport widths. The desktop archive loads all 152 cards without unavailable assets. German Collections is reachable and localized. |
| Desktop exports | Walking Commuter downloads SVG, PDF, DXF, DWG, GLB, animated FBX and WebM files. The bytes identify each format. The SVG metadata identifies the asset, the FBX contains animation curves for the walking clip, and Chrome decodes different WebM frames at 0.15 and 0.65 seconds. |
| Desktop 3D, compare and playground | The 3D renderer reaches ready with a canvas. A shared comparison opens two drawn representations. One placement persists when switching from section to floor plan. |
| Desktop keyboard focus | Settings opens on the selected text size, moves to motion with Tab and restores the trigger on Escape. Focusing a playground figure leaves the current selection alone until activation. |
| Missing entry or lazy 3D module | A simulated module 404 causes one recovery navigation to the extensionless path. Its 404 shell loads the module successfully and restores the original query and fragment without adding a history entry. |

The initial local artifact dated 10:00 displayed 152 cards, 33 marked unavailable. This included Walking Commuter and prevented its drawing, export, 3D, comparison and playground checks. The artifact also failed the new recovery assertions. Its initial entry lacked the recovery ID, and its lazy failure revisited the original URL.

The public site passed the desktop and phone baseline, loading all 152 desktop cards and generating the five drawing and pose downloads.

The 10:16:00 local production artifact passed all 22 checks in headless Chrome with exit status 0. The desktop loaded 152 catalog cards with zero unavailable, generated all six files, and passed both keyboard focus checks. The German phone flow passed at 320 × 640. Both recovery checks used the generated Vite chunks and observed a single cache-busting navigation after an actual module 404. The subsequent boot restored the exact query and fragment and preserved the history length. The normal browser flow had zero runtime errors, failed HTTP requests or CSP violations.

The 10:18:45 build passed 22 of 23 checks, including the new WebM download and decode check. Chromium offered WebM, MP4 and PNG sequence for the clip. The exhaustive catalog check failed for two cards. Dining table had no 1:500 shape for its `slide-rails` thermal region. Rock Pigeon's 1:200 front-elevation height was 0.207 m against a declared 0.225 m.

The 10:25:16 release passed all 23 checks with exit status 0. Dining Table and Rock Pigeon were available alongside the other 150 cards. All seven format checks passed, including WebM motion playback. Both recovery scenarios and the browser runtime and network checks passed on the same release.
