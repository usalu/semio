# R2 rv2: span mode in the React viewer range control

Base: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, React target `S/🎯️targets/⚛️react`.

## Change
- New module `🔨️modules/🎛️viewer/📏️range/🟦️.tsx`: `RangeControl`, `rangeOfMode`, `DEFAULT_SPAN_K = 25`. The old inline fixed-range control in `🎛️viewer/🟦️.tsx` (formerly lines ~203-209) is replaced by `<RangeControl range legend onChange />`.
  - Native radio group (existing `Segmented`: roving arrow keys, `role=radiogroup`, labelled) with Automatic / Fixed / Span; the Span radio has a `title` help text.
  - Span shows one labelled `NumberField` (unit "K", min 0.1, max 200, decimal comma accepted, `aria-invalid` on a bad entry). Non-positive or non-numeric entries change nothing. Choosing span starts at 25 K and keeps an edited span; fixed keeps its edited range (customizable via the field).
  - Modelled exactly like the existing modes: the state stays a `DisplayRange` (`{ mode: "span", spanK }`, contract type from `🔗️contracts/🟦️.ts:237`); the pipeline `depictionOptions` passes it through unchanged, the same value the cast exporter builds (`range: { mode: "span", spanK }`).
- Address (`🗺️routing/🟦️.ts`): query parameter `span` (K). Written only in span mode, read when no valid `range=min:max` is present, non-positive or unreadable values fall back to automatic. Round trip verified.
- i18n, no default language: `viewer.range.span`, `viewer.range.spanHelp`, `viewer.range.spanK` in `🌐️i18n/🔠️english/🎛️viewer.ts` ("Span", ...) and `🔡️german/🎛️viewer.ts` ("Spanne", "Spanne um die Lufttemperatur (± K)"). The German bundle is type-checked against the English key set.
- dn's depiction option changes: `depictionOptions` still takes `language` in the working tree; I touched nothing there. If dn makes language optional the new test passes `language: "en"` which stays valid.

## Tests
- New `🧪️tests/📏️range-span/🟦️.tsx` + `🥒️.feature` (jsdom + testing-library): 9 cases (three radios, labelled group, default 25 K, edit and rejection, aria-invalid, arrow keys across modes, mode transitions, German labels, address round trip, precedence of fixed, depiction option equality).
- React vitest (`T/q2-vitest-react-alias.config.ts`): 18 files; 17 passed, 116 tests passed, 1 skipped, 1 failed. The failure is `catalog-loading` ("ready 132, failed 15" of 147), caused by assets other agents were editing at that moment (same transient as in rx2 item 5); none of my files is involved. Previously 107 passed, so +9 new tests passed.
- Library vitest (`T/q2-vitest-alias.config.ts`): 1117 passed, 11 skipped, 2 failed (53 of 55 files). The failures are the `panel-radiator` height expectation (0.71 vs 0.83) and a radiator-transient digest, i.e. asset edits by other agents (an earlier run showed 14 radiator-related failures that settled between runs). The library code was not changed by me.

## Open
- Visual check of the control in a dev preview not done (tests only).
- The span applies to the thermal surface temperature only; the force view and thermal flux quantities fall back to shared (ts report), the control does not say so beyond the help text.
