# Bilingual Acceptance Review — 2026-11-03

Read-only static review; no tests, builds, browser or Source edits.

## Actionable Issue

D=♻️mit-bestand/🧺️demonstrator. D/🧪️tests/🎭️acceptance/🧬️console/🧪️.ts:74–76 still inspects scenario property accesses in the energie run-cancel AST and requires finalized, finalStep and acceptedResult. Current browser spec reads projectLanguage().texts.finalized/finalStep/acceptedResult instead. Its collected scenario properties therefore omit those names, making this source-law deterministically stale. Update the AST law to assert the new locale-owned access while preserving automatic finalization and abort checks. Sent directly to root/browser_languages and root.

## Strict Texts Match Production

All current project texts checked match production labels:

- sourcing noSelection/remove: ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs:30–31.
- energy noRun/acceptedResult/noResult: corresponding energy editor 🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs:123–124,170. Final timestep text: energy editor 🧵️simulation-session/🔣️.json:156–157, consistent with artifact session implementation localized label.
- finalized/aborted: 🧰️framework/🔨️modules/⏯️tool-run/🏷️actions-labels/🟦️.ts:84,86 (also Rust label definitions184,186).
- FEM caption/play: FEM 🌐️any editor 🗣️terminology/🦀️.rs:98,112; result window constructs localized case caption at lines327,386. Exact settled caption deliberately excludes progress/playback suffix.
- process Enabled/Aktiviert: process3d editor 🗣️terminology/🦀️.rs:16.

No new wording mismatch found. Existing leading status regex prefixes and timestep regexes preserve the earlier checks with the chosen language narrowed.

## Placeholder Scope Is Correct

Keep exact toHaveText for sourcing preview. The id framework.window.sourcingPreview is attached to ChromeAwareWindowScrollSurface, which wraps body content only, in 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📋️panel-hosting/🟠️mode-windows/🟦️.tsx around116. Window title/chrome lives in separate window spec fields. Extra-instance aliases are likewise on the body scroll surface around159. WindowChrome itself renders titleChips outside its body. No evidence supports weakening exact placeholder text.

## Readiness and Overrides

Current Playwright config creates en/de projects with locale en-US/de-DE. projectLanguage() resolves test project identity. waitForPaneShellOutcome asserts html and shell lang equal that identity. All current call sites are in project-default pane boot/interaction cases or language switch live before it changes language. Overview en/de and funding en/de override locale inside describe blocks but do not call this readiness helper. The fr-FR language-picker override also does not call it. Thus current overrides do not conflict with project-name-based readiness. A future helper call from an overridden subcase would need its actual expected language passed explicitly; no current repair is needed.

## Proven Stale Scenario Fields

D/🧪️tests/🎭️acceptance/🧬️simulation/🧫️cases.json retains energy finalized, aborted, finalStep, noResult, acceptedResult and fem caption bilingual regexes, while the spec now uses project texts for each. Corresponding simulationCases TypeScript fields are unused by the browser body. The only other current reader of the scenario file in D is acceptance/🧬️console/🧪️.ts: it validates schema, reads restarts and completion, and contains the stale AST law above; it does not consume the values of those locale regex fields. Cleanup should remove these six duplicated fields together from scenario JSON, schema required/properties and spec type, then update the stale law. Keep completion, ids, numerical FEM settings/steps and restart cases; these remain consumed.
