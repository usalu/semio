# Panels → catalogue toggle → expand Baukomponenten

Replaced the single `catalogue` step with:
1. `panels` — explain panel chrome (catalogue tab pulsed, advance next)
2. `catalogue-panel` — open catalogue via tab (`advance: panel`)
3. `catalogue-objects` — expand `puzzle3d-play-kinds.objects` (`advance: expand`); section defaults closed

Added `IntroductionAdvance::Panel` / `Expand` with shell effects that advance when the panel opens or the tree section expands.
