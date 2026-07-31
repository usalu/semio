# Notes — Fix German Translations for Context

Professional German AEC / UI terminology — not literal English calques.

## CAD (`cad/plugin/rs/lib.rs`)

| Before | After | Why |
|--------|-------|-----|
| Struktur Klassisch | Tragwerk Klassisch | Structure = Tragwerk in German AEC |
| Balken | Träger | Structural beam; Balken is timber/carpentry |
| Box | Quader | German CAD solid term |
| Steckplatz | Platz | Steckplatz is hardware (SIM/RAM) |
| Primitiv | Grundkörper | Primitiv sounds pejorative; CAD solids |
| Rotation | Drehung | Natural German UI |
| Breite (Welt) | Breite (Weltkoordinaten) | Clearer |
| Sechseckig Geschnittener Betonwald Links | Sechseckig geschnittener Betonwald links | German sentence case |
| Primitivauswahl festlegen | Grundkörperauswahl festlegen | Match Grundkörper |
| Rohanfrage laden | Rohdaten laden | Rohanfrage is nonsense calque |
| Welt-Auswahl (Pick) | Punkt in der Welt wählen | Avoid raw Pick anglicism |
| Welt-Hover / Hover festlegen / Referenz-Hover | Überfahren (Welt/Referenz) / Überfahren festlegen | Avoid raw Hover anglicism |

## Puzzle (`puzzle/plugin/rs/lib.rs`)

| Before | After | Why |
|--------|-------|-----|
| Concrete Forest (native DE, 2d/3d/5d) | Betonwald | Was left untranslated |
| Welt-Hover / Hover / Art-Hover | Überfahren (…) | Consistency with CAD |

## Other plugins

Aligned `Welt-Hover` / `Hover festlegen` → `Überfahren (Welt)` / `Überfahren festlegen` in lowpoly, shooting, procedural, note, gis, raster, process, draw, layout; writer AST-Hover similarly.

## S / UI / Compose

| Before | After |
|--------|-------|
| schliessen / Schliessen / ausschliessen | schließen / Schließen / ausschließen |
| zulaessige | zulässige |

## Tests

- `cad_labels_translate_document_tree_panes_in_german` — ok (Tragwerk Klassisch)
- `cad_labels_translate_catalogue_typologies_in_german` — ok (Träger, Quader)
- `document_tree_shows_name_with_kind_as_secondary_label` — ok (Träger)
