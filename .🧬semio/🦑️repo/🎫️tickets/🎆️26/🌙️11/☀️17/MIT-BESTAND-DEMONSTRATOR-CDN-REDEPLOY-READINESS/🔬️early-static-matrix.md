# Early Static Matrix (dry run)

Output served: `.cache/b/p/m/vite/release/5dc1b077e86de2aa/out` (rebuilt from current source, bundles `🌐️-C2odevWr.js`, `🟦️-B2v9lN1s.js`), via `static-origin.py` on `127.0.0.1:8765`, Playwright `DEMONSTRATOR_VERIFY_CDN=1`, base `http://cdn.localhost:8765/`, acceptance `" boot"` test, one Chromium at a time (`early-static-matrix.sh`, `MEM=2560`; the default 4096 MB admission waited 15 minutes behind other jobs, so the dry run asked for 2560 MB).

| Pane | en | de | requests (en / de) | agent-bridge | 404 | other 4xx/5xx |
|---|---|---|---|---|---|---|
| energie | pass (11.0 s) | pass (11.6 s) | 53 / 53 | 0 | 0 | 0 |
| statik | pass (1.8 min) | pass (1.8 min) | 53 / 53 | 0 | 0 | 0 |
| aggregator | pass (2.6 s) | pass (2.4 s) | 111 / 111 | 0 | 0 | 0 |

Totals: 6 of 6 boots passed, 328 origin requests, zero `/__semio/agent-bridge` requests, zero 404, zero other 4xx/5xx.

No static-path defect found; no bundle, site or matrix script changed. Only added `early-static-matrix.sh` (per-pane, per-locale runner with per-run access-log slicing). Servers and browsers were stopped afterwards (port 8765 free, no Chromium left).

Not covered: the other 5 panes (generator, koordinator, aussuchen, bearbeiten, verfolgen) and the 32-receipt gate; the full matrix still needs `verify-boot-matrix.py` with all 16 boots.
