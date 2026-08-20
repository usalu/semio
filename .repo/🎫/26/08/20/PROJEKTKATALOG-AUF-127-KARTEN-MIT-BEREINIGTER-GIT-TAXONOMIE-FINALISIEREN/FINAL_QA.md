# Finales QA-Protokoll

Datum: 2026-08-20  
Ziel: Projektkatalog des Forschungsberichts auf 127 Karten und das verbindliche Acht-Spalten-Schema finalisieren.

## 1. Forschungs- und Reviewabschluss

- 127 Projekte lückenlos: P01–P127.
- 426 Ereignisse lückenlos und ohne Löschung oder Zusammenführung:
  - P01–P67: 230 Ereignisse
  - P68–P109: 128 Ereignisse
  - P110–P127: 68 Ereignisse
- Drei unabhängige Auditpakete und drei Ring-Reviews liegen unter `staging/` vor.
- Endurteile der Ring-Reviews:
  - P01–P67: 63 ACCEPT, 4 CORRECT, 0 REJECT
  - P68–P109: 36 ACCEPT, 6 CORRECT, 0 REJECT
  - P110–P127: 7 ACCEPT, 11 CORRECT, 0 REJECT
- Sämtliche CORRECT-Entscheidungen und die cohortübergreifenden Lead-Entscheidungen sind in `LEAD_DECISIONS.md` dokumentiert und integriert.
- Für alle 426 Ereignisse existiert genau ein ereignisscharfer Nachweisdatensatz in den drei Evidence-Registern; keine fehlende, zusätzliche oder doppelte Ereignis-ID.
- P68-E04 bleibt als ausdrücklich dokumentierter Quellenwiderspruch erhalten: SteelConstruction belegt die Wiederverwendung der Treppe, UKGBC berichtet eine regelbedingte Ersetzung der alten Treppe. Es wurde keine unbelegte Auflösung, Löschung oder Mengenzuordnung vorgenommen.

## 2. Kanonischer Datenstand

- 127 eindeutige Projekt-IDs und 127 eindeutige Projektlabels.
- 127 Projektköpfe mit exakt acht Metadatenwerten.
- 426 Bauteilereignisse mit exakt acht Tabellenzellen.
- Keine leeren Zellen.
- Keine sichtbaren `n. p.`-Werte.
- Keine alten Taxonomiewerte `Ex situ`, `Direkte Wiederverwendung`, `Umfunktionierung`, `Remanufacturing`, `Translokation` oder `Gesamtstruktur`.
- 95 Ereignisse sind `Umnutzung`; alle 95 besitzen eine neue Funktion.
- 331 Ereignisse sind keine `Umnutzung`; alle 331 führen bei `Neue Funktion` korrekt `—`.
- 12 Recycling-Abgrenzungsfälle bleiben sichtbar und werden nicht als ReUse-Ereignisse statistisch gezählt.
- Zulässige, geprüfte Fehlwerte `—`:
  - Menge: 261
  - Material: 42
  - Spender: 43
  - Herkunftsweg: 142
  - Bauteil, Systemebene und Prozess: 0
- Fehlwerte wurden nicht geschätzt.

## 3. Quellen und Bibliografie

- 251 im Projektkatalog verwendete Zitationsschlüssel.
- Alle 251 Schlüssel lösen in der 314 Einträge umfassenden Bibliografie auf.
- Keine doppelten Bibliografieschlüssel.
- Keine undefinierten Zitate oder Referenzen in heller oder dunkler Fassung.
- Die ereignisscharfe Quellenzuordnung, Direkt-URL, Belegtext und der technische Zugangsstatus bleiben in den Evidence-Registern erhalten.
- Ein nicht vom Berichtfont abgedeckter Unicode-Pfeil in einem Bibliografietitel wurde renderneutral als LaTeX-Pfeil gesetzt; der Quelleninhalt blieb unverändert.

## 4. Bilder

- 67 bestehende Projektbilder werden aus den vorhandenen, unveränderten Assets geladen.
- 60 Projekte P68–P127 verwenden den vorgesehenen Platzhalter `Kein freigegebenes Bild`.
- Keine Bilddatei wurde ersetzt, verändert oder neu übernommen.
- Keine fehlende Bilddatei im Build.

## 5. Build- und Layoutprüfung

Ausgeführte Befehle:

- `bun nx run @semio-tech/mit-bestand-bericht:build-forschungsbericht --skip-nx-cache`
- `bun nx run @semio-tech/mit-bestand-bericht:build-zwischenbericht --skip-nx-cache`

Ergebnis:

- Forschungsbericht hell: Exit-Code 0.
- Forschungsbericht dunkel: Exit-Code 0.
- Zwischenbericht hell und dunkel: Exit-Code 0.
- Projektanhang in beiden Forschungsbericht-Logs: 0 Overfull-HBox-Warnungen.
- 0 fatale LaTeX-Fehler.
- 0 undefinierte Zitate.
- 0 undefinierte Referenzen.
- 0 fehlende Zeichen.
- 0 fehlende Projektbilder.
- Verbleibende Warnungen in Eingangsmodell, Skalierung und Akteursnetz sind bestehende, außerhalb dieses Tickets liegende Warnungen; im Projektanhang wurde keine neue Überlaufwarnung erzeugt.

## 6. Visuelle PDF-Abnahme

- Projektanhang der hellen Fassung vollständig geprüft: PDF-Seiten 26–105, 80 Seiten einschließlich Leseschlüssel und Leerseite vor dem Folgeanhang.
- Projektanhang der dunklen Fassung vollständig geprüft: PDF-Seiten 26–105, 80 Seiten.
- 160 gerenderte Seitenbilder sowie 16 Kontaktbögen liegen unter `render/light/` und `render/dark/`.
- Geprüft wurden: achtteiliger Projektkopf, acht Tabellenspalten, Lesbarkeit langer kontrollierter Werte, Quellenblöcke, 67 Bilder, 60 Platzhalter, Tabellenabschlüsse und Seitenwechsel.
- Keine abgeschnittene Projektkarte, keine abgeschnittene Tabellenzeile und kein über den Satzspiegel hinausragender Projektinhalt gefunden.
- Der Leseschlüssel ist vollständig und in beiden Farbfassungen lesbar.

## 7. Schutz- und Scope-Prüfung

- Der Zwischenbericht-Projektanhang blieb unverändert; SHA-256:
  `3D7B7443E96FDDB3115600679D4D15FD098711B3995F61048EC6C5E88C2C4748`.
- Der Zwischenbericht rendert weiterhin mit seinem bisherigen Vertrag; die Regression wurde durch den erfolgreichen hellen und dunklen Build geprüft.
- Bestehende Bildassets blieben unverändert.
- Keine neuen Projekte und keine neue Recherchekampagne wurden hinzugefügt.
- Neo4j, Akteursnetz und fremde Arbeitsänderungen wurden nicht bearbeitet.
- Kanonisch geändert wurden ausschließlich:
  - `mit-bestand/bericht/forschungsbericht/anhang/projekte.tex`
  - `mit-bestand/bericht/forschungsbericht/references.bib` (ein renderneutraler Pfeil im vorhandenen Titel)

## 8. Freigabe

Alle im Plan für Daten, Quellen, Bilder, Layout, Build und visuelle Abnahme definierten Gates sind bestanden. Die helle und dunkle Forschungsbericht-PDF sind lokal uploadfähig. Ein externer Upload wurde nicht durchgeführt, da kein Ziel benannt und keine ausdrückliche Uploadfreigabe erteilt wurde.
