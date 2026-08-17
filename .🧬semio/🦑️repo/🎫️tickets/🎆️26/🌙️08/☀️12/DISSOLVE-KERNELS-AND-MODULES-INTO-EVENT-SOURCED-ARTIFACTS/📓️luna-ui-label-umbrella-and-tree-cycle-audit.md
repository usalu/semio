# UI Label Umbrella and Tree Cycle Audit

## Baseline

- Label SHA-256 after ClassNames split: `754e706044acfa16efccd7f4c9330c3c3064550cbe60087654105a87544fa301`
- The source imports class-name composition and UiDriver directly and is outside the active keybinding write set.

## Responsibilities and Consumer Evidence

The current Label element contains three independent shared capabilities plus the visual Label component:

- localized-label resolution: `useUiTranslation`, `useLabel`, `useIdLabel`, and `resolveTranslationLabel`;
- control-label policy: `useControlAccessibleLabel` and `useControlInlineText`;
- visual tree/property Label presentation;
- imported Tree layout geometry/context used only to render that presentation.

Direct framework consumer counts are strong:

- `useLabel`: 18 UI elements;
- `useIdLabel`: Input, Textarea, and Tree;
- `resolveTranslationLabel`: Canvas and Tree, plus protected ShellHelpers;
- `useUiTranslation`: Canvas and Tree;
- `useControlAccessibleLabel`: Slider, Tree, ToggleGroup, ChromeControlHint, ActionGroup, and ButtonGroup;
- `useControlInlineText`: PanelTabBar, ActionGroup, ButtonGroup, Tree, and ToggleGroup;
- visual `Label`: ten UI elements.

Eighteen independent protected OS renderer terminals additionally use the public label surface. The product package barrel is glue and is excluded. No public symbol has zero consumers; `LabelProps` is private.

## Identity Ownership

UiDriver remains the single owner of control-ID mapping through `resolveControlLabelId`, `activeUiDriver`, and `useUiDriver`. Keybinding resolution and label presentation both consume that mapping for different outcomes. It must not be duplicated in Label or keybinding modules.

## Cycles and Leakage

- Direct SCC: `Label → Tree → Label` because Label imports Tree contexts/layout primitives and Tree imports Label helpers and checks Label identity.
- Larger SCC: React barrel → Label/Tree → React barrel through other authored barrel imports.
- `useUiTranslation` currently exposes `typeof i18next`, an external-library type.
- Visual Label's private props contain React types; they should remain private and the public component should avoid exporting an external-derived props alias.

## Disposition

The shared helpers cannot remain inside a visual element under the repository-wide rule. Move localized-label resolution and control-label policy into separate specific UI-owner modules; both have many independent component consumers. Retain the visual Label component only for tree/property label presentation. Extract the shared Tree layout contract and presentation primitives to a specific `tree-layout-presentation` UI-owner module consumed directly by Tree and Label, removing their direct source cycle.

Update all direct framework importers to their actual specific owners. The React barrel explicitly imports/re-exports retained public symbols from those owners; do not add forwarding imports to Label or Tree. Protected product consumers may continue using the package API because the barrel maps the same symbols directly to their new owners.

Use a repository-owned translation-hook result rather than exposing i18next's type. Keep LabelProps private. This lease waits until the active keybinding SCC work releases ButtonGroup, ToggleGroup, UIDialog, and ContextMenu.
