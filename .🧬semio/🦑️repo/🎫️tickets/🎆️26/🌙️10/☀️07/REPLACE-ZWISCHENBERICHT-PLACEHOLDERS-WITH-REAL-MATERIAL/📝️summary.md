# Replace Zwischenbericht Placeholders With Real Material

Source: `temp/material` (four PNG screenshots, left untouched).

| Material | Placement | Staged asset |
| --- | --- | --- |
| `Repräsentationen des Bauteilobjekts .png` | Abbildung 1.2.2.1.a (Generatoren) | `🖼️asset/🧺️demonstrator/🪞️repraesentationen.png` |
| `Stützengenerator.png` | Abbildung 1.2.2.1.b (Generatoren) | `🖼️asset/🧺️demonstrator/🏛️stuetzengenerator.png` |
| `Ansichten der Entwurfsumgebung .png` | Abbildung 1.2.3.2.a (Human-Interface-Design) | `🖼️asset/🧺️demonstrator/🪟️entwurfsumgebung.png` |
| `Arbeitsprobe.png` | Cover window "Arbeitsprobe" | `🖼️asset/🧺️demonstrator/🧺️arbeitsprobe.png` |

## Changes

- `📋️zwischenbericht.tex`: the three `[Platzhalter: …]` figure bodies are `\includegraphics[width=\linewidth]`; the figures carry `break=false` because the first build split title bar and image across pages.
- Cover image renamed from the misspelled `🧺️aggegator.png` to `🧺️arbeitsprobe.png` in Zwischenbericht (new content) and Forschungsbericht (old content, rename only); `zukunftbau.cls` looks up `asset/demonstrator/arbeitsprobe.png`.

## Verification

`bun nx run @semio-tech/mit-bestand-bericht:build-zwischenbericht` exit 0, 194 pages. Rendered pages 1, 17, 18, 20 inspected: all four images present, figures unbroken.

## Open

- `Entwurfsgrammatik und Stand der Validierung` (text placeholder, page 21) remains: `temp/material` holds no content for it.
- Forschungsbericht was not rebuilt after the cover-asset rename.
