# Complete Aggregator Introduction Tour Steps

## Goal
Finish the Entwerfen mit Bestand Aggregator introduction: Verbindungspunkte (vortex show window option), suggestion via context menu, and supporting chrome (force-unfold measures when a measure id is introduced).

## Changes
- `ui/js/react`: controlled `measuresFolded` / `onMeasuresFoldedChange` on `Window` (same pattern as utility bar).
- `framework/renderer/react`: `windowMeasureTreeContainsId`; when an introduce/show id hits a window measure (e.g. `puzzle3d-play-vortex-show`), force `measuresFolded: false` on matching windows.
- `mit-bestand/aggregator/brand.ts`: steps `verbindungspunkte` (`setVortexShow`) and `suggest-objects` (`openVortexSuggestions`) after transform.
- `puzzle/plugin`: context menu labels via `Puzzle3dLabels`; reuse DE/EN vortex_show / suggest_objects wording (Verbindungspunkte / Baukomponenten vorschlagen).

## Verify
- Vitest (`-t "ENTWERFEN_MIT_BESTAND_BRAND introduction|windowMeasureTreeContainsId|introductionTargetsWindow"`): 4 passed — see `vitest-brand-measures.txt`.
- Cargo (`context_menu_at_selects_vortex_and_prepends_suggest_objects`, `vortex_show_window_option_defaults_to_selected`): both ok — see `cargo-context-menu.txt`.
