# UI Button Group Retention and Cycle Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- ButtonGroup SHA-256 after ClassNames source split: `acef2a6a27df2373cf607a002751fa61a213f9854202f2da3fe57efe3a5cabd0`
- ToggleGroup SHA-256 after ClassNames source split: `7097f9ded7e6945b1be691640215a91836177da341f405962aa732c27951194f`
- Button SHA-256: `ed5087bf46db0f7e1a988c566e564a11e656b658a3bff4c01f538739aba52918`

## Responsibility

`🎛️ButtonGroup` and `ButtonGroupItem` form one coherent grouped-control component. They share level context, group sizing/presentation, accessible labels, icon rendering, and hotkey-badge placement. `buttonGroupItemVariants` is a private presentation facet used by the item and as a type source by Button.

## Independent Production Consumers

- framework `🔘️Button`
- framework `🎨️Canvas`
- protected renderer `UtilityTree`
- protected renderer `ShellHost`

The framework React barrel is glue and stories/tests do not count. Both the group and item therefore have at least two independent production terminals.

## Disposition

Retain ButtonGroup and ButtonGroupItem together. `ButtonGroupProps` has no external active consumer and is not re-exported by the barrel; make it private and avoid an external React-derived public contract. Keep the CVA factory private after replacing Button's `VariantProps<typeof buttonGroupItemVariants>` dependency with a repository-owned local variant contract.

## Remaining Cycle

ButtonGroup and ToggleGroup still import chrome presentation and `ControlHotkeyBadge` from the authored React barrel, producing an assembly-to-element SCC. Chrome presentation now has a direct specific module and must be imported from there.

`ControlHotkeyBadge` is used independently by ButtonGroup and ToggleGroup, so it qualifies as shared code, but its implementation currently remains inside the barrel with the UI keybinding context, formatting, driver visibility, and default keybinding behavior. Extracting only the badge would preserve the cycle through those helpers. A later atomic SCC lease must move the complete badge/keybinding responsibility to a cycle-free specific UI module, update both groups, and reduce the barrel to explicit assembly. No product terminal requires modification while the group APIs remain explicitly re-exported.
