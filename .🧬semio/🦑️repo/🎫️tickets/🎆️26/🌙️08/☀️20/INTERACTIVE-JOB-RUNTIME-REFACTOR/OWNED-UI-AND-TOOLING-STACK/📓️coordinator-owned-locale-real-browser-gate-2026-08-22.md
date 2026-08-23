# Coordinator Owned-Locale Real-Browser Gate — 2026-08-22

## Scope

This gate independently exercised the production React UI barrel after removal of `i18next-browser-languagedetector`. The fixture imports `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`; it does not duplicate locale-selection logic.

## Stored-Locale Case

- URL case: `stored`
- `localStorage["ui.chrome.locale"]`: `de` before the production barrel is imported
- navigator override: `unused`
- resolved locale: `de`
- resolved label: `Zurück`
- first-paint-ready: `true`
- rendered heading: `Fehlende Seite`
- rendered button: `Zurück`
- gate error: empty
- browser console warnings/errors: zero

## Navigator-Locale Case

- URL case: `navigator`
- stored locale removed before the production barrel is imported
- navigator override: `de-AT`
- resolved locale: `de`
- resolved label: `Zurück`
- first-paint-ready: `true`
- rendered heading: `Fehlende Seite`
- rendered button: `Zurück`
- gate error: empty
- browser console warnings/errors: zero

## Evidence Note

The first scripted read queried the nonexistent id `navigator-language` and consequently returned `null`. Direct DOM inspection and the fixture source establish that the actual owned field is `navigator-override`; its visible values were `unused` and `de-AT` for the two cases. The locale, label, readiness, rendered text, and error fields were read from their actual ids.

## Result

PASS. Explicit owned locale selection is effective before first paint for both persisted and navigator-derived German locale inputs without the detector plugin. Dependency retirement remains provisional until a separate Terra audit reproduces the source, manifest, lockfile, test, tooling, and dependency-boundary gates.
