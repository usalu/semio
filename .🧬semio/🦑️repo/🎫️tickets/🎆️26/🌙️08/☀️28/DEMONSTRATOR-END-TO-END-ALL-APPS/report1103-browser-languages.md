# Browser Language Acceptance

Official browser configuration now declares separate en/en-US and de/de-DE Chromium projects. All 25 acceptance cases run per project (50 total), preserving the 32-case per-project cap, 24-character titles, path policy, explicit unsupported-browser picker, and both language-switch cases. Each ready pane asserts document and shell language against the explicit project. Both switch tests now switch away from and back to the initial project language, including German starts.

Action wording is strict per project for sourcing placeholders/removal, energy status/steps/results, FEM caption/playback, and process enabled control. Real input/click/keyboard operations and scene/document/history assertions remain intact. Overview/funding retain their explicit two-language subcases.

The language-neutral artifact fixture contains the two projects and localized expectations; its JSON schema requires their structure. Existing focused artifact law validates the fixture with Ajv, extracts the actual projects expression with the TypeScript AST, evaluates it with Playwright devices/defineConfig, and verifies both project locales and Chromium browser type. Both project names are included in worst-case artifact path validation. No policy limits were raised.

Validation: the single-chromium/unset-locale project baseline was temporarily restored under a try/finally restoration and rejected by the new Playwright configuration law. This baseline red check occurred after the implementation edit; no claim is made of a pre-implementation red execution. Restored implementation passed the focused artifact/Ajv/Playwright law. TypeScript transpile diagnostics for config, acceptance spec, and law were empty. These are focused syntax/oracle checks, not semantic typechecking or a native/browser execution. Real native and served acceptance remain for the parent orchestration. No browser or service was launched.

Changed files:

- ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/🎚️config/🟦️.ts
- ♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts
- ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json
- ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧬️schema/🔣️.json
- ♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧪️tests/🟦️.ts

No manifests, locks, AGENTS files, or generated outputs changed.
