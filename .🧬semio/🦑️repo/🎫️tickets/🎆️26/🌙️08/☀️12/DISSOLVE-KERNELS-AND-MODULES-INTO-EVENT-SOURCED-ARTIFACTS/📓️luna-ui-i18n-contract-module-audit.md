# UI I18n Contract Module Audit

## Scope

- Component: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️component.tsx`
- Audit mode: read-only
- Active source excludes `compose`, `♻️mit-bestand`, ticket/history, generated output, tests, examples, and glue as production-consumer evidence.

## Classification

The component is a coherent shared I18n contract capability. It contains no runtime state; `UI_RIBBON_PARENT_CATEGORIES` is its only runtime value. Its responsibility groups are:

- locale and label contracts: `UiLocale`, `UiLabelPair`, `UiLabelValue`, `UiChromeTerminologyId`;
- ribbon taxonomy: `UiRibbonParentCategory`, `UI_RIBBON_PARENT_CATEGORIES`, `UiRibbonParentKey`, `UiRibbonParentEntries`;
- schema and key derivation: private `DeepUiTranslationKeys`, `UiTranslationSchema`, `UiTranslationKey`, and compile-time coverage assertions;
- adapter contracts: `UiTranslateFn`, `UiI18nPort`, and branded `UiRegisteredTranslationKey`.

This is not an independently rendered UI component and is not an unrelated umbrella. Its semantic identity is I18n contracts, and the current owner is the lowest common UI owner of its independent consumers.

## Direct and Terminal Consumers

Direct edges:

- the React UI package barrel imports and re-exports the contract and owns translation bundles, locale wiring, the i18next-backed port, and registration;
- `Label/🟦️component.tsx` directly imports `UiTranslationKey`, `UiRegisteredTranslationKey`, and `UiTranslateFn`.

Independent active terminal consumers through the UI barrel include:

- `ShellScope` for `UiLocale`;
- OS `ChromePanels` for `UiLocale` and `UiTranslationKey`;
- OS `Interpreter` for `UiTranslationKey`;
- OS `Shell` for `UiLocale`;
- OS `ShellHelpers` for locale, ribbon taxonomy, terminology, and translation keys;
- OS `ShellHost` for `UiLocale`;
- OS `TextEditor` for `UiTranslationKey`;
- `Label` as the additional direct translation-contract consumer.

Facet evidence:

- `UiLocale`: five independent terminals;
- `UiTranslationKey`: five terminals including `Label`;
- `UiTranslateFn` and `UiRegisteredTranslationKey`: `Label` plus the UI-package implementation;
- ribbon category/constant and terminology: one terminal plus the UI-package owner implementation;
- `UiLabelPair`, `UiLabelValue`, `UiRibbonParentEntries`, `UiTranslationSchema`, and `UiI18nPort`: package-local resource/adapter contracts;
- `UiRibbonParentKey`: no active production consumer; an excluded inline test is its only use;
- the three compile-time assertion aliases have no external production consumers.

Unused OS package bridge imports are not counted as terminals. Mesh and manifest comments are not edges.

## Dependency and Boundary Findings

- No I18n SCC exists. I18n depends only on framework contracts; the UI barrel and `Label` depend on I18n.
- Existing barrel/Label cycles do not return through I18n.
- External-type leakage is confined to exported aliases over repository/framework contracts: `UiLocale = ShellLocale` and `UiChromeTerminologyId = ShellTerminology`.
- `UiI18nPort` does not expose i18next types.

## Disposition

Retain the I18n contract capability at the UI owner. Do not split or move it as part of keybinding or Label work. A bounded follow-up may make `UiRibbonParentKey` and compile-time assertion aliases private after verifying the UI barrel no longer exports them; no product consumer changes are required.
