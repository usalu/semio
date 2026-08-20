# Verbindliche Baseline und Datenvertrag

Stand: 2026-08-20. Diese Datei friert die vom Nutzer bestätigte Git-Taxonomie für den Projektanhang ein. Ältere Forschungs- und Zwischenstände sind keine Integrationsquelle.

## Korpus

- 127 Projekte: P01–P127
- 426 Ereignisse
- P01–P67: 230 Ereignisse
- P68–P109: 128 Ereignisse
- P110–P127: 68 Ereignisse
- 251 im Projektanhang verwendete Zitationsschlüssel
- 314 Bibliografieeinträge
- 67 vorhandene Projektbilder und 60 absichtliche Platzhalter

Die 426 Ereignisse bleiben erhalten. Eine Löschung oder Zusammenführung ist nur mit direktem Gegenbeleg und dokumentierter Lead-Entscheidung zulässig. `Recycling` bleibt als Abgrenzungsfall sichtbar, zählt aber nicht als Reuse-Ereignis.

## Projektkopf

Exakt acht Werte:

`Stadt | Land | Jahr | Objekttyp | Projektcharakter | Projektphase | ReUse-Realisierung | Quellen`

- Objekttyp: `Gebäude · Infrastruktur · Außenraum · Bauteilsystem`
- Projektcharakter: `Dauerhaft · Temporär · Prototyp`
- Projektphase: `Geplant · In Ausführung · Fertiggestellt · Rückgebaut`
- ReUse-Realisierung: `Vorgesehen · Teilweise umgesetzt · Umgesetzt`

Vorrang: `Prototyp` vor `Temporär` vor `Dauerhaft`. Brücken und vergleichbare Bauwerke sind `Infrastruktur`; Plätze, Gärten und Promenaden `Außenraum`; Demonstratoren ohne eigenständiges Gebäude oder Infrastrukturbauwerk `Bauteilsystem`. `Unbestätigt` ist nur ein QA-Hinweis. Zieljahre nicht abgeschlossener Projekte erhalten `*`.

## Ereignistabelle

Exakt acht Werte:

`Bauteil | Neue Funktion | Menge | Material | Spender | Herkunftsweg | Systemebene vorher → nachher | Prozess`

Die vier Spaltenumbenennungen (`Neue Nutzung`, `Quelle`, `Herkunft → Ziel`, `Verfahren`) ändern für sich keine Daten.

### Material

`Stahl · Aluminium · Sonstiges Metall · Beton · Naturstein · Ziegel/Keramik · Sonstiges mineralisches Material · Glas · Holz/Holzwerkstoff · Kunststoff · Textil · Mehrstoff · —`

Vollständige Fenster, Leuchten, technische Systeme und Asphaltplatten sind `Mehrstoff`; einzelne Scheiben sind `Glas`. Keine kombinierten oder aus Namen/Fotos geratenen Werte.

### Herkunftsweg

`Vor Ort · Rückbau · Lager · —`

### Systemebenen

Ziel- und unveränderte Ebenen:

`Struktur · Hülle · Ausbau · Technik · Außenraum · Gesamtbauwerk`

Zulässige Herkunftstypen vor dem Pfeil:

`Infrastruktur · Industrieanlage · Energieanlage · Produktionsrest · Restposten · Verpackung · Konsumabfall · Abbruchabfall`

Ein Einzelbegriff bedeutet gleiche Systemebene. Ein Pfeil bezeichnet ausschließlich einen Systemebenenwechsel und erzwingt keinen Funktionswechsel.

### Prozess

`Direkter Wiedereinsatz · Angepasster Wiedereinsatz · Umnutzung · Recycling · —`

- Direkter Wiedereinsatz: gleiche Funktion ohne wesentlichen technischen Eingriff.
- Angepasster Wiedereinsatz: gleiche Funktion nach wesentlichem technischen Eingriff.
- Umnutzung: andere primäre Funktion; hat Vorrang vor der Eingriffstiefe.
- Recycling: Bauteilgestalt geht verloren; dokumentierter Abgrenzungsfall.

Prüfung, Reinigung, Transport, Demontage, Wiederaufbau und reversible Montage begründen allein keinen angepassten Wiedereinsatz.

Jede `Umnutzung` benötigt eine belegte `Neue Funktion`; außerhalb von `Umnutzung` muss `Neue Funktion` `—` sein. Funktionswechsel, Systemebenenwechsel und technische Bearbeitung werden unabhängig klassifiziert.

## Fehlwerte und Belege

- `—`: geprüft, aber nicht dokumentiert.
- `n. p.`: technisch nicht prüfbar; nur im QA-Nachweis, nicht in sichtbaren Tabellen.
- Keine leeren Zellen und keine geratenen Werte.
- Händler, Planer und Vermittler sind keine Spender.
- Tote Links werden nur auf eine kanonische Verschiebung oder ein Archiv derselben Veröffentlichung aktualisiert.

## Arbeitsgrenzen

Nur der Lead verändert kanonische Dateien. Der Zwischenbericht bleibt bei 67 Karten und seiner alten Taxonomie. Neo4j, Akteursnetz, bestehende Bildassets und fremde Arbeitsänderungen bleiben unverändert. Es werden keine Projekte oder Quellen außerhalb von P01–P127 ergänzt.
