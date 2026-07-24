# Aggregator label pass (2026-07-24)

## Problem

Locale-locked Entwerfen mit Bestand Aggregator (`de` + `reuse`) still leaked English / calque chrome because puzzle3d `app_labels` left `mode_labels`, `dialog_labels`, `action_arg_labels`, `introduction_labels`, and `group_labels` empty — `resolveAppLabel` fell back to the English manifest. Action overlays were locale-only (no reuse) and used bad calques (`Context Menu At` / `Kontextmenü bei`, `Welt-Auswahl (Pick)`, `Welt-Vortex-Hover`, `Vorschlag hovern`). Brand intro also said `Kontextmenü`.

## Done

- `puzzle/plugin/rs/lib.rs` (d3): full terminology×locale overlay via `puzzle3d_app_labels_overlay` — mode Bearbeiten, dialog Baukomponente hinzufügen, action/utility/arg/intro/group maps; `contextMenuAt` → Aktionsmenü öffnen / Open Actions Menu; Object→Baukomponente under reuse.
- `mit-bestand/aggregator/brand.ts`: intro uses Aktionsmenü + überfahren wording.
- Tests: `app_labels_overlay_is_german_reuse_branded_for_aggregator`, `app_labels_overlay_stays_english_native_without_brand_locks` — ok.

## Still open (ticket scope)

- Full Part I Label builder migration across plugin crates.
- Remaining TS chrome literal leaks + `check-chrome-i18n` gate.
- Static layout instance titles (`Top` / `Perspective`) still English (no overlay slot).
- puzzle2d/5d overlays still incomplete (aggregator is puzzle3d only).
