# Celebrating on completed UI interactions

`celebrateElements` was only fired from introduction step advances. Discrete control activations (panel tabs including Katalog, toggles, buttons, actions, window pane chrome toggles) now stamp `data-celebrated` via `celebrateCompletedInteraction` → `celebrateElement` for `CELEBRATE_STAMP_DURATION_MS` (2.4s).

Shared path lives next to `celebrateElements` in `ui/js/react/index.tsx` 🧭ElementState; controls call it from their click handlers after applying the interaction.
