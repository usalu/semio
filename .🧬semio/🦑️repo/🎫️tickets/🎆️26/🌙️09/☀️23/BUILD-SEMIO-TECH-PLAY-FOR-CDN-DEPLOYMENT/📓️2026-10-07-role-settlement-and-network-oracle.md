# Role Settlement and Network Oracle

The acceptance oracle had a real ordering gap, now repaired: it checked the successor chip and painted content before waiting for the role transaction to settle. It now requires the role group to be visible and no longer busy, the pane's own shell outcome to be ready, no shell error or pane error boundary, and the expected app identity to remain active before inspecting paint. No viewer, blank-document, HTTP, or network exceptions were added.

Source evidence:

- `runSessionAppSwitchV1` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts` publishes the successor, seeds its layout, then awaits `ports.refresh(next)`. The chip derives directly from `session.app`, so its identity can update before refreshed bodies arrive.
- `ShellHost/🟦️.tsx`'s `runUiRefreshPass` preserves an existing window body temporarily when the new instance has the same window ID, invalidates the former instance's UI hash cache, forces a full fetch, and applies the fetched bodies. A paint witness read between publication and refresh could therefore observe predecessor content.
- `switchToPluginApp` clears `surfaceSwitchBusy` in `finally` after awaiting the transaction, including its guest refresh. The role group's `aria-busy` exposes that flag. A refresh failure is reported by the role control's error handler and remains caught by the strict console gate.
- `createUiRefreshCoalescerV1` resolves covered request waiters only after its awaited refresh pass finishes; it rejects them if that pass fails. This makes the role group's settled state an actual guest refresh barrier, rather than a click-completion indicator.
- The primary window provider's React key includes plugin, app, and instance identity. The barrier plus full fetch and identity recheck is required before paint; a generic initial boot beacon or changed chip alone does not establish that a replacement window is ready.
- Predecessor retirement starts asynchronously and deliberately does not hold up the successor. The gate waits for the successor's refresh, not for all background teardown to finish.

The language-neutral role fixture and its strict draft-07 schema now include `settlement: { controlId: "playground.navbar.roles", busy: false, beforePaint: true }`. Ajv independently validates the complete fixture. The focused test first failed on the missing settlement contract, then passed after the fixture, schema, and acceptance implementation were changed: **red 1 failed / 1 passed / 18 skipped; green 2 passed / 18 skipped**. Generated logs are `play-role-settlement-red.log` and `play-role-settlement-green.log`. Updated Playwright discovery also passed: **149 Chromium cases in one file**, log `play-role-settlement-discovery.log`.

HTTP response errors do not cover connections that fail without a response. `collectErrors` now captures Playwright `requestfailed` events with the error text, HTTP method, and URL. It records every such failure during the exercised page lifecycle. Collection explicitly stops after the final user interaction and on page close, and ignores events from an already closed page; this bounds the audit before context teardown without ignoring failed live requests or broad error categories.

An independent actual Chromium probe passed with exit code 0 through scoped Nx exec. The retained input `acceptance-oracle/📜️script.ts` extracts the actual collector and settlement functions from the acceptance source using the existing TypeScript compiler, then drives them with third-party Playwright against an explicitly synthetic local page. It proves:

1. A real `route.abort("failed")` produces `net::ERR_FAILED`, which the collector records with the actual failed request URL.
2. The same actual failed request event after `stop()` does not extend the evidence array; page close also does not extend it.
3. Settlement remains pending while a visible role group is `aria-busy=true`, even with a retained old surface already present, and resolves after the synthetic new surface is published and busy clears.

The first synthetic probe failed because its empty role group had zero size and Playwright correctly rejected it as hidden. The corrected probe renders visible buttons and passed; this fixture error is not classified as an app defect. The successful console proof is in `🗑️generated/play-acceptance-collector-oracle-visible.log`:

```text
[DEBUG] Actual Playwright requestfailed captured net::ERR_FAILED before stop; ignored after stop and page close.
[DEBUG] Actual Playwright settlement stayed pending while role group aria-busy=true and resolved after updated retained surface + cleared busy.
```

These checks validate the strengthened oracle. Actual role switches, document restoration, paints, and resource closure for all 148 panes still require the fresh production release acceptance run.

A further focused opening-path inspection found no additional systematic source defect to repair. `prepareDocumentSurfaceV1` captures the predecessor archive before creating the successor, checks predecessor identity between each await, restores and attaches before publication, and releases the unpublished successor on failure. `switchToSessionRole` passes the actual current session as the document source; the publish path marks it `documentInitialized=true`, preventing `useInitialExampleReadiness` from replacing the restored archive with a boot example. The browser `PluginRuntime` handle implements both archive ports through the live instance channel. These are source-contract findings, not claims that all family-specific archive payloads work at runtime.

The retained browser probe extracts only `collectErrors`, `waitForPaneShellOutcome`, `waitForSurfaceSettlement`, and `readPaintWitnesses` as top-level TypeScript function declarations. It excludes imports and test registration statements, supplies existing Playwright `expect` and the neutral role fixture explicitly, and operates only on its synthetic page. It introduced no application runtime dependencies or package manifest changes.

A separate systematic DOM paint-oracle defect was then demonstrated in actual Chromium: `Window/🟦️.tsx` mounts measures, search, and engagement overlays alongside its content host inside `window-body`, whereas the former witness counted all body text and elements. A synthetic empty content body plus a measures overlay returned `painted=true` with 8 characters and 4 elements; the neutral expected outcome was false. This red observation is in `🗑️generated/play-dom-paint-red.log`.

The witness now inspects the window's own visible `pane-host-root` content, excludes its separate `pane-host` portal overlay, busy status placeholders, hidden/aria-hidden nodes, and therefore does not count window chrome as document content. World3d retained geometry and Paint2d image extent checks remain unchanged. Four strict schema-backed neutral HTML cases cover empty content with chrome, pending content with chrome, portal-only controls, and actual document content. The registered actual Playwright probe executes all four cases against the extracted current witness, in addition to its settlement and network checks. Final verification is recorded after the latest visible-host fixture run.

Final verification passed on the visible-host cases: `🗑️generated/play-dom-paint-final.log`, exit 0, reports empty chrome false (0 content characters / 1 element), pending chrome false (0 / 0), portal-only false (0 / 1), and document content true (16 / 1), followed by the successful actual network and settlement proofs. The focused Ajv gate also passed 2 tests / 18 skipped (`play-dom-paint-schema.log`), and Playwright discovery still found 149 tests / one file (`play-dom-paint-discovery.log`). No real fleet runtime passing claim is made by these synthetic probes.
