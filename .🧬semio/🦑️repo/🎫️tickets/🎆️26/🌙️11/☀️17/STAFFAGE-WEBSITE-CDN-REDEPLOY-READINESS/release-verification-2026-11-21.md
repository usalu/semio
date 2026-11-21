# Staffage release verification on 21 November 2026

The Staffage archive builds as a static site. The release folder is `🎓️teaching/🏛️architecture/🧍️staffage/🌐️site/📦️packages/🟦️typescript/dist/release`. The live host is GitHub Pages at [Staffage](https://staffage.semio-tech.com/). The current work prepares that folder for the site owner's publication.

## Release defects addressed

- GitHub Pages caches the root document for up to 600 seconds and serves the generated `_headers` as an ordinary file. A query-string retry hit the same cached document. The release now uses a unique pathname that GitHub Pages resolves through the app's `404.html`. Both initial entry failure and a failed lazy import use this recovery. It preserves the query, fragment and history slot, and restores the original pathname after mount. See [the cache and document checks](stale-release-recovery.md).
- Desktop Settings focuses its visible first control. Keyboard users can focus and activate a playground placement independently, get a visible focus indicator, and cancel a drag without recording an edit. See [the interaction regression tests](frontend-and-humans.md).
- The first source scan found 31 assets failing semantic validation and 13 manifest discrepancies. The catalog fixes repair authored print bands, thermal and structural geometry, and duplicate thermal bindings. The manifest now agrees with asset dimensions and installed power. Furniture and appliance checks exercised every integer denominator across the affected bands. See [furniture print bands](furniture-print-bands.md), [appliance print bands](appliance-print-boundaries.md) and [animal thermal markers](animal-thermal-markers.md).
- A build in release mode now stops on catalog validation findings and reports the file, rule and bilingual diagnostics. Development retains those findings for inspection. The regression test reproduced a release that accepted a zero-sized asset before the gate was added.

## Verified release

The 10:25:16 CET production build finished in 75 seconds. Its entry is `assets/🌐️-bEPlCs9G.js`, its stylesheet is `assets/🌐️-DuDC_GxK.css`, and the static folder occupies about 76 MiB. All 23 Chromium checks passed on that release, covering English desktop and a 320 × 640 German phone viewport. The browser loaded all 152 cards, exercised visible keyboard focus, compare, playground, 3D and both fault-based recovery paths, and verified genuine SVG, PDF, DXF, DWG, GLB, animated FBX and WebM downloads. Decoded WebM frames showed motion. See [the browser run](release-browser-smoke.md).

The site suite passed all 16 tests, including the release asset gate and chunk splitting. React passed 299 tests, including full catalog readiness. Site and React TypeScript checks passed. A stale repeat-instancing test now compares exact indices and all world coordinates with Three.js at the production kernel's reference quality. The targeted repeat and physical breath-aperture tests passed. The separate whole-catalog silhouette sweep started against earlier source, remained active after 18 minutes and was stopped without a complete result. See [the repeat regression](repeat-instance-test.md) and [the aperture check](breath-aperture-check.md).

The repository build ingress waited on the shared graph and host swap pressure. The ticket's existing `native-vite.ts` ran the same release Vite configuration with the repository-pinned Bun. `check-release-artifact.mjs` verified the shell, CSP hashes, local HTML and CSS references, DWG WebAssembly and all 152 emitted catalog modules. It also reported six later worktree revisions for Lombardy Poplar, Multi-stem Birch, Date Palm, Miscanthus Grass, Small-leaved Lime and Wheel Loader. Those six source documents pass validation independently. The tested release remains the 10:25 snapshot. Use `--require-current` when verifying an artifact is identical to the catalog at the time of verification.

Build and browser verification ran on macOS arm64, using the portable Vite and Node entry points. The browser suite did not run on native Windows or Linux hosts.

## Publication

Upload the contents of the verified release folder at the host root. Preserve `.nojekyll`, `CNAME`, `404.html` and the original hashed asset paths. A missing asset must return HTTP 404, and the custom 404 must serve the bundled app shell. The GitHub Pages response policies are controlled by the host. The HTML carries the bootstrap CSP for that host.

The live site still serves the previous release. The configured checkout remote is the monorepo. The site's DNS target identifies the Pages account, and the earlier owner-upload procedure remains the documented publication route.
