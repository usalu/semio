# Puzzle 3D Context Menu Suggest Stays Off Brush

## Root cause
`openVortexSuggestions` forced `active_utility = "brush"` and emitted `SetActiveUtility`, and `acceptSuggestion` cleared the utility back to default. Context-menu / Alt+right-click suggestions are a one-shot placement picker, not an entry into brush mode.

## Fix
- Open/accept/close suggestion popup without changing host-owned utility or tool.
- Emit brush preview while `suggestion_menu` is open even when utility is not brush.
- Regression: open+accept preserves transform utility and still renders a placement preview.

## Verification
- `open_vortex_suggestions_opens_the_suggestion_popup`
- `open_and_accept_vortex_suggestions_preserve_active_utility`
- `close_vortex_suggestions_clears_the_menu`
- `hover_suggestion_updates_the_brush_candidate_index_and_live_preview`
- `accept_suggestion_appends_an_object_and_closes_the_menu`
