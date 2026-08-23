# P10 React Router Owned Navigation Retirement

Date: 2026-08-22

<!-- #region Verdict -->

## Verdict

**VERIFIED.** The production replacement, dependency retirement, permanent tests, lock reconciliation, static gates, dependency ratchet, and coordinator-run actual-browser gate are green at **139 = 76 JavaScript + 63 Rust**. No blocker remains inside this serialized packet.

<!-- #endregion Verdict -->

<!-- #region Implementation -->

## Implementation

- Replaced the sole `useNavigate` operation with the private closed `OwnedRouteTarget`, `parseOwnedRouteTarget`, and `navigateOwnedRoute` seam beside `RouteLink`.
- The parser preserves accepted path/query/fragment text and rejects empty, whitespace/control/NUL, explicit-scheme, protocol-relative, cross-origin-normalized, and backslash targets.
- The command creates its event before mutation, catches rejected `pushState`, publishes exactly one synthetic `popstate` only after a successful mutation, and reports `{ navigated: false }` without throwing or mutation on failure/SSR.
- Migrated the actual `NotFound` and `RouteLink` consumers. Invalid `NotFound.parentPath` omits its button. Modified/download/non-`_self`/external/pre-prevented anchors remain native.
- Removed the import, public router facade, manifest row, resolved package, and Bun-proven unreachable children. The unrelated `cookie@0.7.2` identity remains reachable; Bun collapsed its former `express/cookie` alias into the canonical row.
- Added six permanent in-source behavior/API tests and a Vite browser harness that imports and mounts the actual public `NotFound` and `RouteLink` from the production barrel.

<!-- #endregion Implementation -->

<!-- #region Differential -->

## Legacy Differential

Before deleting the dependency, a temporary `BrowserRouter` fixture mounted legacy `NotFound`, clicked `parentPath="/spaces/a?tab=history#entry"`, and ran:

```text
bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'legacy route navigation differential'
```

Result: PASS, 1 passed / 714 skipped. Approved legacy result: `/spaces/a?tab=history#entry`. The temporary legacy import and fixture were removed before manifest retirement; the approved result is now a permanent owned assertion.

<!-- #endregion Differential -->

<!-- #region Validation -->

## Validation

| Command                                                                                     | Result                                                                                                                       |
| ------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'owned route navigation'` | PASS; 6 owned-navigation tests passed.                                                                                       |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                | PASS; 20 files, 720 tests.                                                                                                   |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                 | PASS.                                                                                                                        |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                      | PASS.                                                                                                                        |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                       | PASS; no violations, 2 allowlisted files.                                                                                    |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary`                   | PASS; lockfile saved by Bun.                                                                                                 |
| `bun install --lockfile-only --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS; frozen-lock validation saved no new resolution.                                                                        |
| `bun ./📜️script.ts verify dependencies`                                                     | PASS; baseline 238, current 139, no new dependencies.                                                                        |
| `bun ./📜️script.ts verify dependencies list js --format json`                               | PASS; 76 identities.                                                                                                         |
| `bun ./📜️script.ts verify dependencies list rust --format json`                             | PASS; 63 identities.                                                                                                         |
| `bun ./📜️script.ts verify dependencies parity js`                                           | PASS; manifests 83, external rows 261, evidenced 111, unowned 150, undeclared imports 0, lock mismatches 0, lock fixtures 5. |
| `bunx prettier --write <owned source> <manifest> <browser fixture>`                         | PASS.                                                                                                                        |
| `git diff --check -- <scoped files>`                                                        | PASS; no whitespace errors.                                                                                                  |

The focused tests cover static/ordinary rendering without a Router provider, exact URL preservation, one-event success, native-anchor fallthrough, malformed/external omission, rejected-`pushState` atomicity, and a direct source/API retirement scan.

<!-- #endregion Validation -->

<!-- #region Absence -->

## Absence And Consumer Scans

The repeated pre-edit consumer scan found no downstream binding of the former router facade through `@semio-tech/ui-react`, and the only runtime use remained `NotFound`'s one navigation call. Post-edit executable source/config/manifest/lock scans found no `react-router` occurrence and no former binding identifiers. The only repository-wide literal left outside ticket history is `🔒️dependencies.json`, the immutable baseline snapshot generated at commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`; it is neither a current manifest nor executable configuration.

Bun removed `react-router@7.18.2` and `set-cookie-parser@2.7.2`, replaced the no-longer-needed `cookie@1.1.1` row with the still-required `cookie@0.7.2` row, and retained zero lock mismatches.

<!-- #endregion Absence -->

<!-- #region Browser -->

## Real Browser Gate

The Browser skill required the in-app Browser control surface, a complete selected-browser documentation read before interaction, and prohibited substituting standalone Playwright. That requirement determined the retained actual-module harness at `🧪️p10-owned-route-browser-gate.html`: it imports the production UI barrel through Vite, mounts real `NotFound` and `RouteLink`, and records current URL, exact `popstate` count, and event locations for NotFound, RouteLink, Back, and Forward checks.

Harness readiness checks:

```text
bunx vite --host 127.0.0.1 --port 4179 --strictPort
curl -sS -o /dev/null -w '%{http_code}\n' '<ticket fixture URL>'
curl -sS -o /dev/null -w '%{http_code}\n' '<production UI barrel transform URL>'
```

Results: Vite ready; fixture HTTP 200; production TSX transform HTTP 200.

The root-level Vite server also printed a non-blocking, out-of-scope dependency-scan warning for `compose/client/lib/sketchpad/play/index.html` importing unresolved `@semio-tech/compose-sketchpad/boot`, plus HTML-proxy errors for this subtask's failed percent-encoded Browser attempts. The coordinator's successful visible actual-component DOM and interaction sequence proves that neither affected the correctly opened harness. Compose remained untouched.

This subtask's isolated Browser connection remained unavailable after initializing the mandated runtime and reading `bootstrap-troubleshooting`:

```text
agent.browsers.getForUrl(<fixture URL>) -> No browser is available
agent.browsers.get("iab") -> Browser is not available: iab
agent.browsers.list() -> []
```

No alternate browser controller or standalone Playwright was used. The coordinator then executed the same actual compiled-module harness from a connected in-app Browser and recorded the independent result in `📓️coordinator-owned-route-real-browser-gate-2026-08-22.md`.

Coordinator result: PASS. The initial DOM contained actual `NotFound` and `RouteLink` controls with count 0; `NotFound` reached `/spaces/a?tab=history#entry` with exactly one synthetic event; browser Back restored the harness URL at count 2; Forward restored the exact owned URL at count 3; a subsequent Back plus actual `RouteLink` reached `/spaces/b?tab=route#link` at count 5. The ordered location ledger matched, and browser console warnings/errors remained empty after every interaction.

<!-- #endregion Browser -->

<!-- #region Residuals -->

## Residuals

1. The intentionally removed public facade had no repository consumer; future route-tree/memory-history/data-router needs require a new owned domain, not compatibility exports.
2. No Cargo command, P3 source, P8 source, compose source, shared dependency script, or ticket lifecycle operation was touched.

<!-- #endregion Residuals -->
