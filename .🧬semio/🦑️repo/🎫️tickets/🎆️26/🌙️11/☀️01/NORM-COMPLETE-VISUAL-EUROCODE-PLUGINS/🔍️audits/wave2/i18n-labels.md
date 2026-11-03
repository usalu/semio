# 🌐️ i18n labels (wave 2)

Result: 201 of 459 `x-semio-ui.label` objects with DE == EN got German text in the schema JSON source (mutation parameter labels; every untouched twin form is generated). Rescan: 7,642 labels, 0 empty DE, 258 DE == EN, all accepted.

- Touched (labels only, twins regenerated, `schema twins check --strict` ok): en1990 (2), en1991 (3), en1992 (11), en1993 (15), en1995 (135), en1997 (20), en1999 (17). en1994/1996/1998 had only accepted cognates.
- en1992 was frozen (graph-plan 07:52); edited and regenerated after the freeze cleared. `- ready for rebuild: i18n labels` appended to build-en1990/1991/1992/1993/1995/1997/1999 `latest.md` where present.
- TS/proto/graphql/rust twins carry no labels (identical); snapshot fixtures hold mutation payloads, not labels, so none touched. Field-meta/choices are Rust-side, not fed by these JSON labels.
- Tests: i18n-sinks repo test ok; i18n-5 scan (`json-ident.py`) as above.
- Pre-existing, not mine: en1990 `check --strict` reports drift=1 from "snapshot record BridgeSls derives Default" (NO-DEFAULTS), left to the en1990 owner.
- Also seen: some mutation Rust labels still mix languages, e.g. en1997 `change-wall-height` DE "Stützwand: height ändern" (Rust `LocalizedLabel::native`), not touched here.

## Translations (EN -> DE)
shape Form; activity Tätigkeit; face Fläche; member id Bauteilkennung; kind Art; label Bezeichnung; name Name; variable/permanent destabilizing Veränderlich/Ständig, destabilisierend; permanent stabilizing Ständig, stabilisierend; horizontal/vertical permanent Horizontale/Vertikale ständige Einwirkung; length Länge; height Höhe; backfill gamma Wichte der Hinterfüllung γ; concrete gamma Wichte des Betons γ; total stress Gesamtspannung; ocr Überkonsolidierungsgrad OCR; pore pressure Porenwasserdruck; wall movement Wandbewegung; wall friction deg Wandreibungswinkel δ in Grad; backfill phi deg Reibungswinkel φ der Hinterfüllung in Grad; surcharge Auflast; governing layer id Kennung der maßgebenden Schicht; stem thickness Schaftdicke; embedment Einbindetiefe; position Lage; source Quelle; eps uk Charakteristische Dehnung ε_uk; bend Biegeform; structure kind Tragwerksart; bolts material Schraubenwerkstoff; weld direction Schweißrichtung; fabrication class Ausführungsklasse; weld process Schweißverfahren; Bracing Aussteifung; Cutout Ausschnitt; Taper Verjüngung; Truss Fachwerk; Diaphragm Scheibe; Panel Tafel; PanelLayer Tafelschicht; Floor Geschossdecke; Deck Fahrbahnplatte; Fastener Verbindungsmittel; FastenerPosition Verbindungsmittellage.

## Accepted (cognates/symbols)
Position (233), Position x/y, Tank (11), Silo (11), Explosion.
