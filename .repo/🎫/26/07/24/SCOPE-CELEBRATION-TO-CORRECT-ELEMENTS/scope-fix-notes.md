# Scope celebration to correct elements

## Bug
`celebrateCompletedInteraction` stamped `data-celebrated` on every discrete control click (panel tabs, toggles, buttons, actions, pane chrome). Unrelated UI elements therefore showed the spinning celebration ring even when they were not part of the current introduction step.

## Fix
- Removed blanket `celebrateCompletedInteraction` from UI control click handlers.
- Deleted the helper; celebration is only applied via:
  - `celebrateElements(elementIdSelector(step.introduce))` from `advanceIntroductionByDoing` when an introduction step completes by doing
  - `celebrateWorldInstances` for newly placed catalogue-drop meshes
- Added regression tests that panel tabs and toggles do **not** celebrate on ordinary presses.
