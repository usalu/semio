# Next Direct Dependency Scout — Stale Browser Language Detector

Date: 2026-08-22
Recommendation: **retire `i18next-browser-languagedetector` only.**
Expected ratchet: **`139 = 76 JavaScript + 63 Rust` → `138 = 75 JavaScript + 63 Rust`**.

## Why This Is The Smallest Clean Packet

`i18next-browser-languagedetector` is one direct runtime identity owned by exactly one manifest:

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:42`.

It has exactly two executable source sites, both in the UI React barrel:

1. `📦️index.tsx:106` — the sole default import as `LanguageDetector`.
2. `📦️index.tsx:4027` — `i18next.use(LanguageDetector).use(initReactI18next)`.

There are no other production, test, type, config, script, dynamic `import()`, or `require()` uses
outside the manifest, `bun.lock`, and historical `🔒️dependencies.json` baseline. The scan included
hidden configuration and excluded `compose`, ticket history, build output, and dependency artifacts.
The removal does not touch P3 renderer/action/browser-worker files or P8 store/plugin/presence files.

This is materially narrower than removing `i18next` or `react-i18next`: both remain live public
i18n boundaries. It is also cleaner than the apparently empty MDX entries: hidden
`.storybook/main.ts:14,17-20,151-170` actively imports/configures the MDX/remark/rehype toolchain.
`dagre` remains excluded until its real Rust/Wasm/OffscreenCanvas gate is accepted.

## Source-Proven No-Op Detector Registration

The shared UI singleton already owns the entire locale choice before i18next initializes:

```ts
const requestedLocale = resolveRequestedUiLocale();
// ...
void i18next.init({
 supportedLngs: ["en", "de"],
 nonExplicitSupportedLngs: true,
 lng: requestedLocale,
 initImmediate: false,
});
```

`resolveRequestedUiLocale` at `📦️index.tsx:3988-3994` selects a valid persisted
`UI_CHROME_LOCALE_STORAGE_KEY` value first, otherwise normalizes the already-initialized i18next
language or `navigator.language`. Its closed locale mapping (`de* → de`, everything else → en) is
already owned by `normalizeUiLocale`/public `detectShellLocale` at `:3979-3986`.

The direct i18next public-contract probe below used a fresh instance and an instrumented language
detector, with the same explicit `lng`, supported locales, and synchronous initialization policy:

```text
{ "detects": 0, "language": "en", "resolvedLanguage": "en", "text": "EN" }
```

The detector's `detect()` was never called. Therefore the configured `detection` object
(`localStorage`, query string, navigator, html tag) is dead configuration: explicit `lng` has already
selected the language. The detector has no cache behavior either (`caches: []`). This corrects the
older broad i18n classification: the three-package i18n runtime remains active, but this one plugin
registration is now a stale leaf.

## Exact Implementation Packet

| File                                                                                | Required change                                                                                                                                                                                                                                                                                     |
| ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`  | Delete the detector import at `:106`; change `i18next.use(LanguageDetector).use(initReactI18next)` to `i18next.use(initReactI18next)`; delete the now-ignored `detection` option at `:4045-4049`. Do not change the existing owned resolver, language policy, shell instances, or public i18n port. |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | Remove only `i18next-browser-languagedetector`.                                                                                                                                                                                                                                                     |
| `bun.lock`                                                                          | Reconcile with Bun; expected removal is `i18next-browser-languagedetector@8.2.1`. `@babel/runtime` must survive through i18next, react-i18next, React Three, Nx, and test tooling.                                                                                                                  |
| Existing in-source UI React tests                                                   | Add a compact owned-locale/retired-identity regression near the current i18n tests. It should prove `de-AT → de`, non-German/undefined → en, explicit shell locale initialization, and no import/registration of the retired identity.                                                              |

No replacement API is needed. The owned replacement seam already exists: `resolveRequestedUiLocale`
produces the finite `UiTranslationLocaleCode` consumed as i18next's explicit `lng`. No external type,
value, or detector configuration may be exported or reintroduced.

## Required Gates

1. Run the new focused locale-retirement test and the complete UI quick suite.
2. Run UI typecheck, lint, and `check-ui-primitives`.
3. Reconcile `bun.lock` through Bun, then run the frozen-lock validation.
4. Run `bun ./📜️script.ts verify dependencies`, both JSON identity lists, and JS parity. The expected
   current list has 75 JS identities and no detector row; Rust remains 63.
5. Run absence scans for the package identity, `LanguageDetector`, dynamic import/require forms, and
   downstream public bindings; run formatting and `git diff --check` over the three product files.
6. Use an actual Vite-served UI-barrel browser fixture as the first-paint gate: seed
   `ui.chrome.locale=de` before dynamically importing the production barrel, then assert the actual
   `uiI18n` resolves German and a chrome label is German. Repeat with no stored locale and a German
   navigator-language fixture where the browser harness permits it. This is a bootstrap behavior gate,
   not a generic component screenshot.

## Lock, Conflict, And Risk Notes

`bun.lock:522` is the sole workspace edge and `:2950` the sole detector resolution. Its only declared
child is `@babel/runtime`, which is independently reachable from `i18next`, `react-i18next`,
`@react-three/*`, Nx, and test tooling; do not expect a runtime-row decrement beyond the direct
identity itself.

The implementation is confined to the UI manifest, its i18n region (`:106`, `:3988-4057`), and the
lockfile. Serialize the manifest/lock reconciliation with any other dependency packet and serialize
the monolithic barrel edit with other UI-barrel work. No P3/P8 implementation file is in scope.

Risk is low but non-zero only at locale bootstrap: a change that removes the owned explicit `lng` or
restores external detector precedence would change first paint. The listed resolver/unit/browser gates
make that observable. Query-string/html-tag precedence is deliberately not preserved because it has
been unreachable under the current explicit-`lng` configuration; do not add a compatibility layer for
it in this greenfield repository.
