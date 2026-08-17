# UI Flow Direction Context Module Audit

## Responsibility

`🧭️Flow` owns one coherent logical layout-direction context:

- inline direction (`ltr` or `rtl`);
- block direction (`down` or `up`);
- the combined Flow contract;
- nested partial-override provider;
- ambient reader hook.

## Independent Production Consumers

- Popover
- PanelTabBar
- ContextMenu
- Select
- Panel
- Dialog
- Tree

Panel, PanelTabBar, and Tree independently mount providers; all seven components read the ambient direction. The React barrel is glue and does not count. No public symbol is zero-consumer.

## Boundary and Cycle Evidence

The implementation imports only React and direct `reactHostPort`; Ports does not import it. It has no direct barrel, Label, UiDriver, or keybinding cycle. Context creation occurs at module evaluation and therefore must retain its direct host-port dependency. `FlowInline`, `FlowBlock`, and `Flow` are repository-owned types. `FlowProvider` currently exposes `React.FC`/`React.ReactNode`; a repository-owned named props contract should replace the exported external-derived signature.

## Disposition

The behavior is qualified shared code, not a visual element. Relocate it intact to a specific UI-owner `flow-direction-context` module under `🔨️modules`, update all seven direct consumers and the explicit barrel registration, and delete the old Flow element identity/directory. Do not split the types/provider/hook or add a forwarding export. Preserve nested merge behavior and default directions exactly.

This relocation waits until the active keybinding lease releases ContextMenu and the shared barrel.
