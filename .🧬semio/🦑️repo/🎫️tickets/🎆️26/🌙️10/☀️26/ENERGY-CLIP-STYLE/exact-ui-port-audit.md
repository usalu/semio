# Exact UI Port Audit

Read-only audit of presentation/🌐️browser, browser-ui-implementation.md and exact-ui-tests.md during the final source freeze. No implementation edits or browser re-runs were performed by this audit. Production chrome uses actual React components/CSS; this does not claim pixel parity for native authored lesson bodies.

## Findings

1. P2: source-closure memoization misses newly added Tailwind candidates. 🌐️browser/🐍️.py95-109 returns the cached closure when existing file contents match; it does not hash directory membership. 🟨️render.mjs38-40 scans all UI TS/TSX files, so adding a new file with new utility candidates can affect compiled CSS without altering any previously recorded file. A later batch recompiles and includes the new CSS, while publication state can retain the old source closure. Include scanner-directory membership in closure identity, or rediscover closure on each certification. Existing-file changes are covered correctly.

2. P2: generated adapter cleanup starts too late. 🟨️render.mjs31 writes browser-tools-PID.cjs beneath the shared owned work directory, but finally cleanup is only entered at79. Failures in adapter loading, stylesheet compilation, application bundling, contract/icon/cursor asset reads or sources-only output before79 retain this tool output outside the short b-attempt owned by Python. Put adapter creation and all subsequent compilation inside the cleanup finally, or place it under the short attempt path. Successful rendering and source queries clean it correctly.

## Verified Architecture

- Python browser port22-32 resolves provisioned repo from explicit environment, PATH or workspace marker; no hardcoded Windows binary.
- _call48-52 invokes repo tools exec --light node behind the existing runtime.run_command process interface.
- JS26-34 compiles and loads the existing repository loadPlaywrightV1 adapter. That adapter uses repository toolEnvironmentV1 and registry browser cache; launch80 uses its Chromium without system browser fallback.
- Chromium installation is not implicitly performed by capture. Existing registered Playwright interface supports provisioning through repo tools exec playwright -- install chromium. No new package/dependency declaration appears in this port; existing React, esbuild, Tailwind, Playwright remain behind the browser protocol.
- The versioned JSON protocol has a retained schema and Python-facing dictionary boundary. CLI validates version, explicit locale, positive integer viewport, frame ownership and required arrays. It does not execute a full JSON Schema validator.
- JS sources-only collection includes application and adapter esbuild dependency closures, Tailwind dependencies and scanned files, contract files, actual icons/cursors, font URLs, package.json and bun.lock. Python adds itself and checks input existence before certification.
- Runtime presentation_state obtains presentation.source_files before publication and rechecks source identities after render; known input content changes are caught. The directory-membership issue above is the remaining closure gap.
- Output work paths are workspace-owned and bounded; Python b-XXXXXXXX attempts keep request/response/log paths short. Output paths are checked against256. Success removes Python attempts and JS adapter. Failed attempt diagnostics are retained/bounded for caller cleanup.
- ToolSession owns cancellation and process descendants. Unix uses new session plus process-group termination; Windows taskkill tree termination is an existing process implementation. JS SIGINT/SIGTERM additionally closes Chromium and stops the frame loop.
- Progress emits bounded DEBUG frame milestones. Browser sources-only compile progress is through the enclosing tool process.
- Actual WindowChrome/Navbar/Footer/Tree/SemioLogo and interaction classes are imported directly in 🟦️.tsx5-11. The frame UI keeps canvas as window ancestor; mouse movement and press use real Playwright events.
- Captures wait for fonts, images, two animation frames and silhouette geometry; DOM metadata reports actual rectangles, paths and computed styles.
- Browser always lays out1600x900 CSS pixels and exports via DPR; Python only normalizes one-pixel dimension mismatch at boundary. Unsupported aspect ratios fail output dimension validation through current high-level settings.
- Authored body stills and domain thumbnails remain supplied content inside actual UI chrome. No assertion here extends browser parity to native lesson content.

## Validation Limits

This audit inspected source and the independent test report. The final five encode checks were still owned by the test agent; their eventual outcome must be read from that report. The prior report correctly recorded strict movie first/last-frame failures and did not relax their assertion. Browser/OS execution beyond the current host was not performed in this read-only audit.


## Resolution Review

Both original P2 findings are resolved in the inspected revision.

- Candidate closure: Python now retains actual compiler scan_sources, expands their brace globs, discovers current candidate membership, and requires matching candidate membership plus matching known-file hashes before cache reuse (🐍️.py99-154). The covered subset check141 also prevents a candidate arriving during compilation from being treated as cached/hash-covered on a later publication check. A source query may initially observe the race, but the later certification refresh rejects that stale cache and rediscovers it. The root reported a RED-to-GREEN regression for this race; this review verified the source guard without rerunning that test.
- Adapter cleanup: Node now starts an outer try before writing the generated adapter31-32 and unconditionally removes it in finally158. Stylesheet, bundling, assets, source queries and browser failures all pass through this finally. The root reported a failure probe with zero residual adapters; source inspection confirms that coverage.

The optional zero-touch Chromium provisioning branch was not yet present in this inspected revision (launch82 still reports provisioning instructions on failure). Its follow-up review remains separate from the two resolved findings.


## Final Zero-Touch Provisioning Review

The inspected final source resolves the prior missing-browser limitation. 🧰️tools.mjs2-13 checks cancellation before launch and provisioning, catches only the standard Playwright missing-executable message, provisions once, then retries once. Healthy launches and unrelated launch errors preserve their original behavior; cancellation propagates before a retry.

🟨️render.mjs84-93 injects the existing registered playwrightV1 installer with install chromium, inherited parent capacity through heavy:false, explicit cancellation signal, stderr output and DEBUG progress. The helper itself is explicitly included in the source closure22. Browser close, signal-listener removal and adapter-file cleanup remain in nested finally blocks164-169.

Root supplied passing evidence for eight portable cases, real Playwright missing-cache/healthy-cache checks, and the actual registered installer smoke (4.10s/3.98s). This review independently verified their source routing and bounded single-retry behavior; it did not rerun those tests or download a browser.

No unresolved finding remains from this audit. This audit agent owns no running browser, server, process session or open file handle: its earlier exploratory browser/server were closed in finally blocks, and all final review commands completed synchronously. Shared fleet process ownership remains with the root and test agents.
