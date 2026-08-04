# Notes

- Glossary terms were defined but never wired with `\Gls{}`, so Seiten stayed empty.
- `\Gls` now accepts optional display text for German inflection: `\Gls[Baukomponenten]{Baukomponente}`.
- Compound glossary keys that never appear verbatim (`Leistungsbewertung / Vorbewertung`, `Plattform / Gesamtsystem`, `Human-Interface-Design / User Experience`) were split into body-usable terms.
- Added `Einspeiseplattform` / `Entwurfswerkzeug` for the former platform compound; introduced `Bestandsquelle` in the opening Ergebnisse paragraph.
- Abbreviations (AP, API, KI, LLM, HID, REST, NGS, KET, BBSR) were already wired; left bare in section titles and front matter.
- Build: `bun run build:mit-bestand:zwischenbericht` → 108 pages; Glossar and Abkürzungsverzeichnis both show page refs.
