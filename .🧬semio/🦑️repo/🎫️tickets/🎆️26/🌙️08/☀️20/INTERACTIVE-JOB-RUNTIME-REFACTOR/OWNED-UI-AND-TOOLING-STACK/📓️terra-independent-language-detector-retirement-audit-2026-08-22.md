# Terra Independent Language Detector Retirement Audit — 2026-08-22

## Verdict

**ACCEPT.** The in-scope direct UI package retirement and owned locale behavior are proven, and the dependency ratchet is exactly **138 identities = 75 JavaScript + 63 Rust**. The in-scope UI source, public API, manifest, lock graph, runtime behavior, and tooling gates are green.

The broader no-ignore scan finds one excluded residual in `./compose`, which the governing plan explicitly declares out of scope. It is not a current edge, use, or public API of the in-scope UI React packet and does not change this acceptance.

No in-scope detector identity, manifest/lock edge, dynamic loading form, binding, export, or `detection` configuration remains.

## Scope And Evidence Reviewed

- Current implementation report: `📓️p10-stale-language-detector-retirement-2026-08-22.md`.
- Current coordinator browser report: `📓️coordinator-owned-locale-real-browser-gate-2026-08-22.md`.
- Current source, package manifest, `bun.lock`, UI test/target scripts, browser fixture, and Compose Vite configuration.
- No product source, manifest, lockfile, coordinator report/list, or ticket metadata was changed. This report is the only audit artifact written.

## Reproduced Results

| Gate                     | Exact command / method                                                                                                       | Result                                                                                                                                            |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focused regression       | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- -t 'owned locale detector retirement'`                        | PASS — 1 file, 3 passed; 720 skipped; 723 total.                                                                                                  |
| Full UI quick suite      | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                                 | PASS — 20 files, 723 passed.                                                                                                                      |
| UI typecheck             | `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                                  | PASS.                                                                                                                                             |
| UI lint                  | `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                                       | PASS.                                                                                                                                             |
| Primitive policy         | `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                                                        | PASS — 0 violations; 2 allowlisted files.                                                                                                         |
| Frozen lock              | `bun install --lockfile-only --frozen-lockfile --ignore-scripts --no-progress --no-summary`                                  | PASS.                                                                                                                                             |
| Dependency ratchet       | `bun ./📜️script.ts verify dependencies`                                                                                      | PASS — baseline 238, current 138, 100 removed, no new third-party identities.                                                                     |
| JavaScript identity list | `bun ./📜️script.ts verify dependencies list js --format json \| bun -e 'const rows=JSON.parse(await Bun.stdin.text()); …'`   | 75 identities; detector rows: 0.                                                                                                                  |
| Rust identity list       | `bun ./📜️script.ts verify dependencies list rust --format json \| bun -e 'const rows=JSON.parse(await Bun.stdin.text()); …'` | 63 identities.                                                                                                                                    |
| JS manifest/lock parity  | `bun ./📜️script.ts verify dependencies parity js`                                                                            | PASS — 83 manifests, 260 external rows, 110 evidenced, 150 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 lock fixtures. |
| UI-manifest keys         | `bun -e 'const paths=["package.json","…/react/package.json"]; …'`                                                            | No detector key in either inspected root/UI manifest dependency section.                                                                          |
| Lock graph literals      | `bun -e 'const lock=await Bun.file("bun.lock").text(); …'`                                                                   | `literalCount: 0`, `resolvedCount: 0`.                                                                                                            |
| Scoped diff hygiene      | `git diff --check -- bun.lock <ui-barrel> <ui-manifest>`                                                                     | PASS; exit 0.                                                                                                                                     |
| Scoped formatting        | `bunx prettier --check <ui-barrel> <ui-manifest> <browser-fixture>`                                                          | PASS — all matched files formatted.                                                                                                               |

## Source, API, And Configuration Audit

The UI barrel retains an owned locale path:

- `readStoredUiChromeLocale` only accepts persisted `en` or `de`.
- `normalizeUiLocale` maps `de*` to `de`, and all other/absent values to `en`.
- `resolveRequestedUiLocale` gives persisted locale precedence, then normalizes existing i18next/navigator language.
- `initializeUiI18n` registers only `initReactI18next` and initializes synchronously with explicit `lng: requestedLocale`, `supportedLngs: ["en", "de"]`, and `initImmediate: false`.

The focused regression directly verifies the closed locale mapping, explicit initialization, German label resolution, and source absence of the prior import/binding/configuration. The direct UI package manifest and `bun.lock` have no detector edge/resolution. The UI barrel has no detector import, dynamic import, `require`, `LanguageDetector` binding, detector export, or `detection:` configuration.

The broad default `rg` check reported no product match because the Compose area is ignored. A no-ignore scan was therefore required:

```sh
rg -n -i --no-ignore --hidden --glob '!**/node_modules/**' --glob '!**/dist/**' --glob '!**/target/**' --glob '!**/.🧬semio/**' --glob '!.cursor/**' 'i18next-browser-languagedetector' package.json bun.lock 🧰️framework ✏️s 🌎️hub ♻️mit-bestand compose/client/bin/engine/js/vite.mcp-app.config.ts
```

It returned one excluded Compose configuration hit at `compose/client/bin/engine/js/vite.mcp-app.config.ts:109`. The governing plan explicitly places `./compose` outside the packet boundary, so it is not an in-scope failure. A narrower executable-form scan found that excluded hit plus the UI regression test's deliberately assembled assertion string. The test string is not an import, edge, binding, export, or configuration.

## Browser Reproduction

I read both supplied browser reports but independently reran the actual Vite-served fixture in the in-app browser against the production `@semio-tech/ui-react` barrel.

1. `bunx vite --host 127.0.0.1 --port 4181 --strictPort` from the fixture directory failed to resolve the absolute production-barrel import. That attempt is not counted as proof.
2. `bunx vite --host 127.0.0.1 --port 4182 --strictPort` from the repository root served the same fixture and production barrel successfully. The temporary Vite listeners were stopped after verification.

The following browser DOM/console results were observed after two animation frames:

| Case      | Bootstrap input                                                           | Resolved locale | Resolved/rendered label | First-paint-ready | Gate error | Warnings/errors |
| --------- | ------------------------------------------------------------------------- | --------------- | ----------------------- | ----------------- | ---------- | --------------- |
| Stored    | `localStorage["ui.chrome.locale"] = "de"` before import                   | `de`            | `Zurück` / `Zurück`     | `true`            | empty      | 0               |
| Navigator | no stored locale; overridden `navigator.language = "de-AT"` before import | `de`            | `Zurück` / `Zurück`     | `true`            | empty      | 0               |

This independently confirms the coordinator's behavior claim.

## Excluded Residual

`compose/client/bin/engine/js/vite.mcp-app.config.ts:109` contains a detector-specific Vite stub. The governing plan states that `./compose` is out of scope, and the packet implementation report already records it as such. It was not modified or used as a reason to reject the in-scope **138 = 75 JS + 63 Rust** acceptance boundary.

Historical dependency baseline/ticket evidence and generated package caches are likewise excluded from current in-scope source/API/manifest/lock/runtime evaluation.
