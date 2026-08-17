# UI Action Group Umbrella and Cycle Audit

## Classification

`ActionGroup/🟦️component.tsx` currently combines four responsibilities: a standalone action control, group/context composition, a dropdown adapter, and dead generic transaction callbacks. These responsibilities have different consumer closures and dispositions.

## Facet Evidence

| Facet | Symbols | Independent production consumers | Disposition |
| --- | --- | ---: | --- |
| standalone action control | `Action`, `ActionProps` | Tree and Toggle | split to a specific shared Action element |
| group/context | `ActionGroup`, `ActionGroupItem` and private context | Window | retain as the specific group boundary; no module extraction |
| dropdown adapter | `ActionDropdown`, option and props contracts | Scene | inline/localize in Scene |
| transaction callbacks | `startTransaction`, `finalizeTransaction` | zero callers | delete |
| presentation helper | `actionGroupItemVariants` | zero external consumers | make private/de-export |

The story is excluded consumer evidence and currently omits required `icon` fields, so it also needs contract-aligned fixture repair when the group surface is touched.

## Runtime SCC

The residual cycle is `React barrel -> ActionGroup -> React barrel`. ActionGroup imports `chromeControlGroupShellClass`, `loadingBorderElementClass`, and `waitingBorderElementClass` through the barrel even though their direct specific modules exist. Because CVA and the context initialize at module evaluation, this is a real initialization-order concern.

The exact cycle break is direct imports from:

- `🎛️chrome-control-presentation` for `chromeControlGroupShellClass`;
- `🌀️status-border-presentation` for loading/waiting element classes.

This minimum fix touches ActionGroup only and requires no registrar/product change.

## Boundary Findings

- Action and dropdown public props expose React-derived types.
- `ControlIcon` carries a React element and must remain behind repository-owned UI contracts during later splits.
- Public `actionGroupItemVariants` exposes a CVA-derived callable; privatize it.
- `UiLabel` is already repository-owned.

## Graph-Colored Queues

1. Immediate green: break the ActionGroup/barrel SCC with direct imports.
2. Green shared-control split: ActionGroup, new `Action` element, Tree, Toggle, and serialized barrel registration.
3. Green Scene-local dropdown cleanup: Scene, ActionGroup, and serialized barrel registration.
4. Window is required only if the group boundary itself is later inlined; avoid that larger closure now.

## Baseline SHA-256

- ActionGroup: `1978fe34c166fceba6be70cd478d65b66a5e85e3301347cde77e077b7287e9b4`
- chrome-control presentation: `b903d9d981f033d97ec16f0f28c765b61e53b80923579f183407fe9f8c1d6f17`
- status-border presentation: `f917be9da1cb4eda4f81dbb1857863380d60efab6f919e8378dadb9da67f6548`
- Scene: `b1c1ff9ee2aaf2c01a87ebb3096df53c2b4f118a89cdc5874f8b25ad402f7d56`
- Tree: `837c514ed223178c9327ad097185f70537c0dda0e6d33f727fbe83b3f84ab40e`
- Toggle: `5bd32b0c107de82c8a663b50bd860d7f87c2e24c7013ce94364ad93f924c3fdb`
- Window: `c0f072ad43d88336d883a797580630687d35d749a67a2afed14013aaa1c7afb0`
