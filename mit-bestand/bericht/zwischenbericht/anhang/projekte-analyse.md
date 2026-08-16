% Regel- und Quellenabgleich Anhang K (Stand: 16.08.2026). Arbeitsdatei – nicht Teil des Berichtstexts.

# Projektkatalog (Anhang K) – Analyse und Korrekturliste (Review)

**Stand:** 16.08.2026  ·  **Abdeckung:** 67/67 Projektkarten, 230/230 Bauteilzeilen, alle 8 Spalten; anschließend 67/67 Projektkarten extern vertieft  ·  **Ergebnis:** 42 interne Regelbefunde plus 22 konkrete Quellenkorrekturen oder Modellierungsentscheidungen

## Was geprüft wurde – und was nicht

Der erste Prüfschritt gleicht den **Datensatz gegen seinen eigenen Leseschlüssel** (Anhang L, `projekte.tex` Z. 496–536) sowie gegen sich selbst ab: gleiche Konstellation ⇒ gleiche Kodierung. Der zweite Prüfschritt, dokumentiert unter *Projektweise Vertiefung – Recherchelauf 16.08.2026*, verifiziert Identität, Beteiligte, Mengen, Herkunftsobjekte, Verfahren, Status und – soweit publiziert – Wirkung und Kosten gegen zusätzliche Primär-, Fach- und Forschungsquellen. Nicht öffentlich belegte Felder und Quellenkonflikte bleiben ausdrücklich markiert; sie werden nicht durch Schätzwerte geschlossen.

Formal ist der Katalog sauber: 67 Karten mit lückenloser Nummerierung `proj:01`–`proj:67`, keine Doppel-IDs, alle 67 Bilddateien vorhanden, alle 74 Zitierschlüssel in `references.bib` aufgelöst.

## Der strukturelle Befund

Die Spalte 7 heißt laut Makro-Definition **„Herkunft → Ziel"** (`print/tex/semio-table.sty:1309`), der Leseschlüssel definiert den Pfeil als „Wechsel der **Funktion oder Systemebene**". Die Verfahrensdefinitionen in Spalte 8 knüpfen dagegen ausschließlich an die **Funktion** an („in gleicher Funktion", „bei Wechsel der Funktion"). Zwischen beiden Spalten besteht damit **keine ableitbare Regel**: Ein Pfeil kann einen Funktionswechsel bedeuten – muss aber nicht.

Genau in dieser Lücke liegen fast alle Inkonsistenzen. Die Zahlen zeigen, dass faktisch doch nach dem Pfeil kodiert wurde, nur nicht durchgängig:

| Verfahren | Zeilen | davon mit Pfeil | davon mit „Neue Nutzung" |
|---|---|---|---|
| Direkte Wiederverwendung | 109 | 5 (5 %) | 0 |
| Angepasste Wiederverwendung | 50 | 8 (16 %) | 1 |
| **Umnutzung** | **41** | **37 (90 %)** | **29 (71 %)** |
| Reststoff-/Restpostennutzung | 18 | 15 (83 %) | 0 |
| Recycling | 12 | 8 (67 %) | 2 |

„Umnutzung" ist damit de facto das Pfeil-Verfahren. Die 5 Zeilen „Direkte Wiederverwendung **mit** Pfeil" und die 8 Zeilen „Angepasste Wiederverwendung **mit** Pfeil" sind die Abweichungen von dieser ungeschriebenen Regel — und sie erzeugen den Effekt, dass **derselbe Pfad in verschiedenen Karten verschieden kodiert ist**:

- `Gebäudehülle → Innenausbau`: 5× Umnutzung (P01, P04, P15, P25, P41), 2× Angepasste Wiederverwendung (P14, P40)
- `Gebäudehülle → Außenraum`: 3× Umnutzung (P40, P54, P57), 1× Direkte (P14), 1× Angepasste (P41)
- `Dach → Gebäudehülle`: 2× Angepasste (P24, P52), 1× Direkte (P59)
- `Dach → Tragwerk`: 1× Direkte (P06), 1× Umnutzung (P13)
- `Außenraum → Innenausbau`: 2× Direkte (P08, P49) — obwohl der Leseschlüssel für „direkt" *gleiche Funktion* verlangt
- `Produktionsrest → Innenausbau`: 3× Recycling (P07, P52×2), 2× Reststoffnutzung (P50, P52)

Dasselbe auf Bauteilebene, bei identischer Bauteilbezeichnung *und* identischem Pfad:

| Bauteil / Pfad | Direkte Wiederverwendung | Angepasste Wiederverwendung |
|---|---|---|
| Wandplatte (WBS70) / Tragwerk | P10 | P11 |
| Deckenplatte (WBS70) / Tragwerk | P10 | P11 |
| Hohldielendecke / Tragwerk | P32 | P34, P35, P67 |
| Stahlträger / Tragwerk | P17 | P02, P03, P08, P16, P20, P54 |
| Fenster / Gebäudehülle | P01, P04, P09, P61, P62 | P13, P51 |
| Fassadenziegel / Gebäudehülle | P42 | P40, P43, P45 |
| Ziegelstein / Gebäudehülle | P03, P06 | P04 |
| Fassadenblech / Gebäudehülle | P09 | P13 |
| Stahlkonstruktion / Tragwerk | P48 | P04 |
| Bodenplatte / Tragwerk | P61 | P31 |

Diese zehn Paare sind der belastbarste Befund der Prüfung: Sie lassen sich nicht mit unterschiedlicher Quellenlage erklären, weil in beiden Fällen dieselbe Information vorliegt. Sie entstehen daran, dass zwischen „direkt" und „angepasst" **kein Kriterium definiert ist** — der Leseschlüssel sagt „unverändert" vs. „nach Bearbeitung oder Ertüchtigung", aber ob Zuschnitt, Reinigung, Entnagelung oder Prüfung schon „Bearbeitung" ist, bleibt offen. Bei P10/P11 (identische WBS70-Platten, identische Quelle `epfl-atlas-of-reused-concrete`, benachbarte Karten) ist die Divergenz mit Sicherheit ein Kodierfehler, keine Sachaussage.

## Vorgeschlagene verbindliche Kodierregeln

Analog zu den für die Bauteilbörsen festgelegten Regeln – vor der Korrektur zu bestätigen, danach tabellenweit anzuwenden:

1. **Pfeil = Funktionswechsel = Umnutzung.** Ein Pfeil in Spalte 7 wird nur gesetzt, wenn die Baukomponente eine *andere Funktion* übernimmt; dann ist Spalte 8 zwingend „Umnutzung" (bzw. bei Gestaltverlust „Recycling"). Bleibt die Funktion gleich und ändert sich nur die Systemebene, entfällt der Pfeil und Spalte 7 nennt die Zielebene allein.
2. **„Neue Nutzung" (Sp. 2) nur für Nutzungen, nicht für Einbauorte.** „Dachaufstockung" (P08) ist ein Einbauort, keine neue Nutzung — im Gegensatz zu „Sitzbank", „Pergola", „Rankhilfe", „Möbel".
3. **„Direkt" vs. „angepasst" wird am Zuschnitt entschieden.** Vorschlag: geometrisch verändernde Bearbeitung (Kürzen, Zuschneiden, Neubohren, Ertüchtigen) ⇒ angepasst; Reinigen, Prüfen, Entnageln, Nachbehandeln ⇒ direkt. Ohne Quellenangabe zur Bearbeitung ⇒ direkt (konservativ).
4. **Reststoff-/Restpostennutzung und Recycling verlangen eine Nicht-Bauteil-Quelle in Sp. 7.** Zulässige Ausgangsebenen: `Produktionsrest`, `Restposten`, `Verpackung`. Für rückgebautes Material, das stofflich aufbereitet wird (P14, P51, P53, P64), fehlt bislang eine Ausgangsebene — **`Abbruchabfall` ist zu ergänzen**, sonst bleibt der Widerspruch bestehen, dass P64 dieselbe Sache als `Produktionsrest → Tragwerk` und P51/P53 als `Tragwerk` kodieren.
5. **Eine Karte = ein Projekt.** P46 fasst zwei Gebäude zusammen und ist aufzutrennen oder als Cluster kenntlich zu machen (betrifft die Fallzahl 67).

## Korrekturen je Projekt

Regel-Spalte: `R1`–`R5` = obige Regel · `FIX` = eindeutiger Fehler unabhängig von der Regelfrage · `FLAG` = Einzelfallentscheidung nötig.

| ID | Projekt | Zeile / Feld | Alt | Neu | Sev | Regel/Notiz | Status |
|---|---|---|---|---|---|---|---|
| PK-01 | P56 Hastings Pier | Pier-Deckholz (Z. 386), Sp. 3 *Menge* | `Sitzbank` | `—`, „Sitzbank" nach Sp. 2 *Neue Nutzung* | high | FIX · Spaltenversatz: „Sitzbank" steht in der Mengenspalte. Einziger Fall im Katalog. | offen |
| PK-02 | P08 Holbein Gardens | York-Stone-Pflaster, Sp. 8 | Direkte Wiederverwendung | Umnutzung | high | R1 · Pflasterstein wird Innenbodenbelag – Funktionswechsel; „direkt" verlangt gleiche Funktion. | offen |
| PK-03 | P49 The Green House | Pflasterklinker, Sp. 8 | Direkte Wiederverwendung | Umnutzung | high | R1 · identisch zu PK-02 (`Außenraum → Innenausbau`). | offen |
| PK-04 | P14 Recyclinghaus | Eternit-Platte, Sp. 8 | Direkte Wiederverwendung | Umnutzung | high | R1 · `Gebäudehülle → Außenraum`; P40/P54/P57 kodieren denselben Pfad als Umnutzung. | offen |
| PK-05 | P06 The Swan Kindergarten | Holz-Dachbinder, Sp. 7 | `Dach → Tragwerk` | `Tragwerk` | high | R1 · Ein Dachbinder *ist* Tragwerk; der Pfeil bildet keinen Funktionswechsel ab. Verfahren bleibt „direkt". | offen |
| PK-06 | P13 CRCLR House | Holzverschnitt, Sp. 7 | `Innenausbau` | `Produktionsrest → Innenausbau` | high | R4 · Quelle „Verschnitt aus Tischlereien". P52 kodiert dasselbe Bauteil bereits korrekt. | offen |
| PK-07 | P55 Brighton Waste House | Sperrholz, Sp. 7 | `Innenausbau` | `Produktionsrest → Innenausbau` | high | R4 · Quelle „Verschnitt"; identisch zu PK-06. | offen |
| PK-08 | P45 Chiro d’Itterbeek | Tragstruktur, Sp. 7 | `Tragwerk` | `Produktionsrest → Tragwerk` | high | R4 · Quelle „Baustellen-/Produktionsüberschuss"; P59 kodiert dieselbe Konstellation korrekt. | offen |
| PK-09 | P46 Verbiest / Karreveld | Karte gesamt | eine Karte für zwei Gebäude | auftrennen oder als Cluster auszeichnen | high | R5 · Betrifft die Fallzahl (67 Karten ≠ 67 Projekte) und die Bild-/Ortszuordnung. | offen |
| PK-10 | P48 Alliander HQ | Sp. 6 bei Quelle „Bestand vor Ort" | 2× `Gleiches Gebäude`, 1× `Gleiches Areal` | vereinheitlichen | high | FIX · Identische Herkunftsformel, widersprüchlicher ReUse-Ort *innerhalb einer Karte*. | offen |
| PK-11 | P61 Plattenvereinigung | Sp. 6 bei Quelle „Plattenbau" | 1× `Lokal ex situ`, 1× `Extern ex situ` | vereinheitlichen | high | FIX · wie PK-10, innerhalb einer Karte. | offen |
| PK-12 | P59 Pavillon Circulaire | Steinwolldämmung, Sp. 8 | Direkte Wiederverwendung | Umnutzung *oder* Pfeil streichen | high | FLAG · `Dach → Gebäudehülle`; P24/P52 kodieren denselben Pfad als „angepasst". Drei Karten, drei Verfahren – muss zusammen mit PK-19/20 entschieden werden. | offen |
| PK-13 | P14 Recyclinghaus | Innenwand, Sp. 8 | Angepasste Wiederverwendung | Umnutzung | medium | R1 · `Gebäudehülle → Innenausbau`, Mehrheitskodierung ist Umnutzung (P01, P04, P15, P25). | offen |
| PK-14 | P40 Maison Vignette | Wandfliese, Sp. 8 | Angepasste Wiederverwendung | Umnutzung | medium | R1 · identisch zu PK-13. | offen |
| PK-15 | P41 MULTI Brussels | Blausteinblock, Sp. 8 | Angepasste Wiederverwendung | Umnutzung | medium | R1 · `Gebäudehülle → Außenraum`, Mehrheitskodierung ist Umnutzung. | offen |
| PK-16 | P56 Hastings Pier | Pier-Deckholz Z. 1, Sp. 8 | Angepasste Wiederverwendung | Umnutzung | medium | R1 · `Außenraum → Gebäudehülle`; P33/P52 kodieren denselben Pfad als Umnutzung. | offen |
| PK-17 | P08 Holbein Gardens | Stahlträger, Sp. 2 + 7 | `Dachaufstockung` / `Tragwerk → Dach` | `—` / `Tragwerk` | medium | R2 · „Dachaufstockung" ist Einbauort; der Träger bleibt Träger. Verfahren bleibt „angepasst". | offen |
| PK-18 | P36 Lokomotion | Hohldielendecke, Sp. 7 | `Tragwerk → Dach` | `Tragwerk` | medium | R2 · wie PK-17; Hohldiele bleibt spannendes Deckenelement. | offen |
| PK-19 | P24 ELYS | Trapezblech, Sp. 7/8 | `Dach → Gebäudehülle` / angepasst | einheitlich mit PK-12/PK-20 | medium | FLAG · Wetterschale bleibt Wetterschale ⇒ Pfeil streichen wäre konsistent mit R1. | offen |
| PK-20 | P52 TRÆ High-Rise | Trapezblech, Sp. 7/8 | `Dach → Gebäudehülle` / angepasst | einheitlich mit PK-12/PK-19 | medium | FLAG · identisch zu PK-19. | offen |
| PK-21 | P51 Upcycle Studios | Recyclingbeton, Sp. 7 | `Tragwerk` | `Abbruchabfall → Tragwerk` | medium | R4 · Recycling verlangt Nicht-Bauteil-Quelle; vgl. P64. | offen |
| PK-22 | P53 Woongroep Boschgaard | Recyclingbeton, Sp. 7 | `Tragwerk` | `Abbruchabfall → Tragwerk` | medium | R4 · identisch zu PK-21. | offen |
| PK-23 | P14 Recyclinghaus | Fundament, Sp. 7 | `Tragwerk` | `Abbruchabfall → Tragwerk` | medium | R4 · Quelle „Bau-/Abbruchabfall". | offen |
| PK-24 | P48 Alliander HQ | Dachasphalt, Sp. 7 | `Dach` | `Abbruchabfall → Dach` | medium | R4 · Verfahren „Recycling" bei erhaltener Bauteilebene. | offen |
| PK-25 | P15 Thoravej 29 | Holzabfall, Sp. 7 | `Innenausbau → Mobiliar` | `Abbruchabfall → Mobiliar` | medium | R4 · Spanplatte aus Abfallholz – die Bauteilgestalt geht verloren. | offen |
| PK-26 | P58 People’s Pavilion | Fassadenfliese (Leihgabe Govaerts), Sp. 4 | `—` | prüfen: `Kunststoff` | medium | FLAG · Zweite Fassadenfliesen-Zeile derselben Karte ist als Kunststoff kodiert. | offen |
| PK-27 | P32 Circular Centre Nederland | Sp. *Jahr* | `—` | geplantes Jahr mit `*` ergänzen | medium | FIX · Stand „Geplant" ohne Jahr; der Leseschlüssel sieht `*` + Jahr vor. | offen |
| PK-28 | P43 Lo-Reninge | Sp. *Jahr* | `—` | Fertigstellungsjahr ermitteln | medium | Stand „Ausgeführt" ohne Jahr. | offen |
| PK-29 | P44 Institut de Botanique ULg | Sp. *Jahr* | `—` | Fertigstellungsjahr ermitteln | medium | wie PK-28. | offen |
| PK-30 | P45 Chiro d’Itterbeek | Sp. *Jahr* | `—` | Fertigstellungsjahr ermitteln | medium | wie PK-28. | offen |
| PK-31 | P47 Zinneke | Sp. *Jahr* | `—` | Fertigstellungsjahr ermitteln | medium | wie PK-28. Die Karte ist sonst die mengenvollständigste des Katalogs (9/9). | offen |
| PK-32 | P10 / P11 | Wand-/Deckenplatte (WBS70), Sp. 8 | P10 direkt, P11 angepasst | vereinheitlichen | low | R3 · Identisches Bauteil, identische Quelle, benachbarte Karten. | offen |
| PK-33 | P32 / P34 / P35 / P67 | Hohldielendecke, Sp. 8 | P32 direkt, übrige angepasst | vereinheitlichen | low | R3 | offen |
| PK-34 | P17 vs. P02/03/08/16/20/54 | Stahlträger, Sp. 8 | P17 direkt, übrige angepasst | vereinheitlichen | low | R3 · Wiederverwendeter Baustahl wird in der Praxis stets geprüft und zugeschnitten. | offen |
| PK-35 | P42 vs. P40/43/45 | Fassadenziegel, Sp. 8 | P42 direkt, übrige angepasst | vereinheitlichen | low | R3 · Rückbauziegel werden regelmäßig entmörtelt ⇒ „angepasst" wäre einheitlich. | offen |
| PK-36 | P03/P06 vs. P04 | Ziegelstein, Sp. 8 | uneinheitlich | vereinheitlichen | low | R3 | offen |
| PK-37 | P09 vs. P13 | Fassadenblech, Sp. 8 | uneinheitlich | vereinheitlichen | low | R3 | offen |
| PK-38 | P01/04/09/61/62 vs. P13/P51 | Fenster, Sp. 8 | uneinheitlich | vereinheitlichen | low | R3 | offen |
| PK-39 | P48 vs. P04 | Stahlkonstruktion, Sp. 8 | uneinheitlich | vereinheitlichen | low | R3 | offen |
| PK-40 | P61 vs. P31 | Bodenplatte, Sp. 8 | uneinheitlich | vereinheitlichen | low | R3 | offen |
| PK-41 | P07 Villa Welpeloo | Aufzug, Sp. 8 | Angepasste Wiederverwendung | prüfen | low | R3 · Einziger TGA-Eintrag, der nicht „direkt" ist (8 andere TGA-Zeilen: direkt). | offen |
| PK-42 | P25 vs. P59 | Mobiliar-Ebene, Sp. 8 | Regal direkt, Holzstuhl angepasst | vereinheitlichen | low | R3 | offen |

## Datenlage je Spalte

| Spalte | belegt | `—` | Anmerkung |
|---|---|---|---|
| 1 Bauteil | 230/230 | 0 | vollständig |
| 2 Neue Nutzung | 32/230 | 198 | strukturell dünn besetzt, das ist konventionsgemäß |
| 3 Menge | 82/230 | 148 | **36 %** – 29 der 67 Karten führen *überhaupt keine* Menge |
| 4 Material | 208/230 | 22 | 10 %, unkritisch |
| 5 Quelle | 230/230 | 0 | aber 34 Zeilen nur als „Rückbaubestand" ohne Ortsangabe |
| 6 ReUse-Ort | 147/230 | 83 | **36 %** – die schwächste Klassifikationsspalte |
| 7 Herkunft → Ziel | 230/230 | 0 | 73 Zeilen mit Pfeil, davon 40 mit fraglicher Verfahrenszuordnung |
| 8 Verfahren | 230/230 | 0 | vollständig, aber s. o. |

**Doppelt unbestimmt:** In 34 Zeilen (15 %) ist *sowohl* das Herkunftsobjekt („Rückbaubestand") *als auch* der ReUse-Ort (`—`) unbestimmt. Diese Zeilen tragen zur Herkunftsauswertung nichts bei. Konzentriert in P47 (7 von 9), P09 (5 von 5), P08 und P14 (je 3).

**Konsequenz für den Haupttext:** Der Katalog belegt die im Haupttext beschriebene Prozessabfolge und die These „form follows availability" gut. Er trägt dagegen **keine belastbare Aussage über Transportdistanzen oder lokale Beschaffung** — dafür ist Spalte 6 zu 36 % leer und in zwei Karten intern widersprüchlich. Falls im Bericht mit ReUse-Ort-Anteilen argumentiert werden soll, muss die Bezugsgröße die 147 belegten Zeilen sein, nicht 230.

## Quellenlage

- **54 von 67 Karten stützen sich auf eine einzige Quelle**; 13 Karten auf zwei. Keine Karte hat drei.
- **Eine Quelle trägt 7 Karten:** `epfl-atlas-of-reused-concrete` (P10, P11, P12, P29, P30, P31, P61). Das ist der gesamte deutsche Plattenbau-Cluster. Alle sieben Karten sind damit von einer einzigen Sekundärquelle abhängig — und genau in diesem Cluster liegt mit PK-32 auch ein Kodierwiderspruch.
- Die übrigen 73 Quellen tragen je eine Karte.
- **21 Karten stützen sich auf eine Betreiber-, Büro- oder Bauherrenquelle** (P01, P05, P06, P07, P08, P09, P16, P18, P24, P41, P45, P47, P49, P50, P51, P52, P53, P56, P58, P59, P60 — lendager-\*, superuse-\*, rotor-\*, in-situ-\*, grosvenor-\*, landsec-\*, cepezed-\*, drmm-\*, gmp-\*, hawkinsbrown-\*, zirkular-\*, encore-heureux-\*, bureau-sla-\*). Bei Mengen- und Einsparungsangaben sind das Eigenangaben ohne unabhängige Bestätigung. 17 dieser 21 Karten haben zugleich nur diese eine Quelle.

## Priorität für die Live-Prüfung

Falls ein Verifikationsdurchgang folgt, in dieser Reihenfolge:

1. **P16 Timber Square** — Jahr 2026 bei Stand „Ausgeführt" und 33.450 m² Bodenfliesen; die größte Einzelmenge des Katalogs, an zwei Quellen zu bestätigen.
2. **P18 55 Great Suffolk Street** — als einziges Projekt „Unbestätigt", 139 t Baustahl, Jahr 2026*. Entweder bestätigen oder aus dem Katalog nehmen.
3. **P02 BedZED** — 54.000 m Holzständerwerk und ca. 98 t Stahl aus einer Betreiberquelle von 2002.
4. **P55 Brighton Waste House** — 20.000 Zahnbürsten, 4.000 VHS-Kassetten, 2 t Denim: die auffälligsten Mengen, Einzelquelle.
5. **P42 Musée de Folklore** — 30.000 Fassadenziegel aus 8 Abbruchobjekten, Einzelquelle.
6. **P23 Europa Building** — 3.750 Eichenfensterrahmen, plausibel, aber katalogweit die höchste Stückzahl bei Fenstern.
7. **P10/P11/P12/P29/P30/P31/P61** — der EPFL-Cluster, gegen eine zweite Quelle je Projekt.
8. **P32, P43, P44, P45, P47** — fehlende Jahre (PK-27 bis PK-31).

## Register aller 67 Karten

`Q` = Quellen · `BT` = Bauteilzeilen · `Menge` = Zeilen mit Mengenangabe · `Ort⁻` / `Mat⁻` = Zeilen ohne ReUse-Ort / ohne Material · `Pfeil` = Zeilen mit Ebenenwechsel.

| ID | Projekt | Ort | Jahr | Typ | Stand | Q | BT | Menge | Ort⁻ | Mat⁻ | Pfeil | Befund |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| P01 | Kopfbau Halle 118 | Winterthur, Schweiz | 2021 | Gebäude | Ausgeführt | 1 | 4 | 0/4 | 0 | 0 | 1 | keine Mengen |
| P02 | BedZED | London, Vereinigtes Königreich | 2002 | Gebäude | Ausgeführt | 1 | 3 | 2/3 | 2 | 1 | 0 | Mengen prüfen (Prio 3) |
| P03 | BioPartner 5 | Leiden–Oegstgeest, Niederlande | 2021 | Gebäude | Ausgeführt | 1 | 3 | 1/3 | 0 | 0 | 0 | PK-36 |
| P04 | Kristian Augusts gate 13 | Oslo, Norwegen | 2021 | Gebäude | Ausgeführt | 1 | 11 | 1/11 | 5 | 0 | 4 | umfangreichste Karte; PK-36, PK-38, PK-39 |
| P05 | Recypark Demets | Anderlecht, Belgien | 2024 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | sauber |
| P06 | The Swan Kindergarten | Gladsaxe, Dänemark | 2022 | Gebäude | Ausgeführt | 1 | 4 | 0/4 | 0 | 0 | 1 | **PK-05** |
| P07 | Villa Welpeloo | Enschede, Niederlande | 2009 | Gebäude | Ausgeführt | 1 | 8 | 0/8 | 2 | 0 | 5 | PK-41; keine Mengen |
| P08 | Holbein Gardens | London, Vereinigtes Königreich | 2023 | Gebäude | Ausgeführt | 1 | 6 | 2/6 | 3 | 1 | 2 | **PK-02**, PK-17 |
| P09 | Werkhof 29 | Zürich, Schweiz | 2025 | Gebäude | Ausgeführt | 1 | 5 | 0/5 | 5 | 2 | 0 | schwächste Karte: 57 % belegt, alle 5 Zeilen doppelt unbestimmt; PK-37, PK-38 |
| P10 | Haus HOS | Mühlhausen, Deutschland | 2008 | Gebäude | Ausgeführt | 1 | 3 | 3/3 | 0 | 0 | 0 | PK-32; EPFL-Cluster |
| P11 | Mehrow Pilot House | Mehrow, Deutschland | 2005 | Gebäude | Ausgeführt | 1 | 2 | 2/2 | 0 | 0 | 0 | PK-32; EPFL-Cluster |
| P12 | Bröthen Twin-House | Hoyerswerda, Deutschland | 2001 | Gebäude | Ausgeführt | 1 | 2 | 2/2 | 0 | 0 | 0 | EPFL-Cluster |
| P13 | CRCLR House | Berlin, Deutschland | 2023 | Gebäude | Ausgeführt | 1 | 6 | 2/6 | 0 | 2 | 1 | **PK-06**, PK-37, PK-38 |
| P14 | Recyclinghaus Hannover | Hannover, Deutschland | 2019 | Gebäude | Ausgeführt | 1 | 7 | 0/7 | 6 | 1 | 4 | **PK-04**, PK-13, PK-23; 6/7 Zeilen ohne ReUse-Ort |
| P15 | Thoravej 29 | Kopenhagen, Dänemark | 2025 | Gebäude | Ausgeführt | 1 | 4 | 0/4 | 0 | 0 | 4 | PK-25; einzige Karte mit Pfeil in *allen* Zeilen |
| P16 | Timber Square | London, Vereinigtes Königreich | 2026 | Gebäude | Ausgeführt | 2 | 3 | 2/3 | 0 | 0 | 1 | Prio 1 – größte Einzelmenge (33.450 m²) |
| P17 | TBC.London | London, Vereinigtes Königreich | 2025 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | PK-34 |
| P18 | 55 Great Suffolk Street | London, Vereinigtes Königreich | 2026* | Gebäude | Unbestätigt | 1 | 1 | 1/1 | 0 | 0 | 0 | **Prio 2** – einziges „Unbestätigt" |
| P19 | Brent Cross Town Substation | London, Vereinigtes Königreich | 2023 | Gebäude | Ausgeführt | 1 | 1 | 0/1 | 1 | 0 | 1 | Öl-Pipeline → Baustahl, keine Menge |
| P20 | Boulder Fire Station 3 | Boulder, USA | 2024 | Gebäude | Ausgeführt | 2 | 1 | 1/1 | 0 | 0 | 0 | PK-34 |
| P21 | Big Dig House | Lexington, USA | 2006 | Gebäude | Ausgeführt | 1 | 3 | 0/3 | 0 | 0 | 3 | Infrastruktur → Tragwerk, keine Mengen |
| P22 | Saxum Vineyard Barn | Paso Robles, USA | 2018 | Gebäude | Ausgeführt | 1 | 1 | 0/1 | 0 | 0 | 1 | sauber |
| P23 | Europa Building | Brüssel, Belgien | 2016 | Gebäude | Ausgeführt | 2 | 1 | 1/1 | 0 | 0 | 0 | Prio 6 – 3.750 Stk. |
| P24 | ELYS Kultur- & Gewerbehaus | Basel, Schweiz | 2021 | Gebäude | Ausgeführt | 2 | 4 | 3/4 | 1 | 0 | 3 | **PK-19** (FLAG mit P52, P59) |
| P25 | Lycée Michel Lucius | Luxemburg, Luxemburg | 2021 | Gebäude | Ausgeführt | 1 | 9 | 8/9 | 2 | 1 | 5 | mengenstärkste Karte; PK-42 |
| P26 | Jeugdkliniek Ithaka | Kloetinge, Niederlande | 2019 | Gebäude | Ausgeführt | 1 | 6 | 0/6 | 0 | 4 | 0 | alle 6 Zeilen aus einer Quelle (RWS-Büro); 4× Material `—` |
| P27 | gjG House | Gent, Belgien | 2015 | Gebäude | Ausgeführt | 1 | 1 | 0/1 | 1 | 0 | 0 | dünnste Belegung |
| P28 | Maison DnA | Asse, Belgien | 2013 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | sauber |
| P29 | Association House Gröditz | Gröditz, Deutschland | 2007 | Gebäude | Ausgeführt | 1 | 2 | 2/2 | 2 | 0 | 0 | EPFL-Cluster |
| P30 | Association House Plauen | Plauen, Deutschland | 2007 | Gebäude | Ausgeführt | 1 | 3 | 3/3 | 3 | 0 | 0 | EPFL-Cluster |
| P31 | Berlin-Schildow Pilot House | Schildow, Deutschland | 2005 | Gebäude | Ausgeführt | 1 | 2 | 1/2 | 2 | 0 | 0 | PK-40; EPFL-Cluster |
| P32 | Circular Centre Nederland | Heerde, Niederlande | — | Gebäude | Geplant | 1 | 2 | 0/2 | 0 | 0 | 0 | **PK-27**, PK-33 |
| P33 | Juch-Areal Recyclingzentrum | Zürich, Schweiz | 2027* | Gebäude | Geplant | 1 | 4 | 0/4 | 2 | 0 | 1 | Stern-Konvention korrekt |
| P34 | Melkinlaituri School | Helsinki, Finnland | 2027* | Gebäude | Teilweise eingebaut | 1 | 1 | 1/1 | 1 | 0 | 0 | PK-33 |
| P35 | Härmälänranta | Tampere, Finnland | 2024 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | PK-33 |
| P36 | Lokomotion Technology Centre | Tampere, Finnland | 2025 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 1 | **PK-18** |
| P37 | Grande Halle de Colombelles | Colombelles, Frankreich | 2019 | Gebäude | Ausgeführt | 2 | 8 | 6/8 | 2 | 0 | 3 | gut belegt (81 %) |
| P38 | La Ferme du Rail | Paris, Frankreich | 2019 | Gebäude | Ausgeführt | 2 | 3 | 0/3 | 3 | 0 | 0 | alle Zeilen ohne ReUse-Ort |
| P39 | Résilience | Stains, Frankreich | 2020 | Gebäude | Ausgeführt | 2 | 1 | 0/1 | 0 | 0 | 0 | sauber |
| P40 | Maison Vignette | Auderghem, Belgien | 2020 | Gebäude | Ausgeführt | 1 | 5 | 4/5 | 0 | 0 | 2 | **PK-14**, PK-35 |
| P41 | MULTI Brussels | Brüssel, Belgien | 2024 | Gebäude | Ausgeführt | 2 | 3 | 3/3 | 1 | 0 | 2 | **PK-15**; mengenvollständig |
| P42 | Musée de Folklore | Mouscron, Belgien | 2018 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | Prio 5; PK-35 |
| P43 | Lo-Reninge Town Hall | Lo-Reninge, Belgien | — | Gebäude | Ausgeführt | 1 | 1 | 0/1 | 1 | 0 | 0 | **PK-28**; Quelle „nicht dokumentiert" – schwächste Karte des Katalogs |
| P44 | Institut de Botanique ULg | Lüttich, Belgien | — | Gebäude | Ausgeführt | 1 | 2 | 1/2 | 1 | 1 | 0 | **PK-29** |
| P45 | Chiro d’Itterbeek | Dilbeek, Belgien | — | Gebäude | Ausgeführt | 1 | 4 | 0/4 | 0 | 0 | 0 | **PK-08**, **PK-30**, PK-35 |
| P46 | Vergleichscluster Verbiest / Karreveld | Brüssel, Belgien | 2020 | Gebäude | Ausgeführt | 2 | 6 | 0/6 | 0 | 0 | 0 | **PK-09** – zwei Gebäude in einer Karte |
| P47 | Zinneke | Brüssel, Belgien | — | Gebäude | Ausgeführt | 1 | 9 | 9/9 | 7 | 3 | 1 | **PK-31**; mengenvollständig, aber 7/9 ohne ReUse-Ort |
| P48 | Alliander HQ | Duiven, Niederlande | 2015 | Gebäude | Ausgeführt | 2 | 6 | 0/6 | 2 | 1 | 2 | **PK-10**, PK-24, PK-39 |
| P49 | The Green House | Utrecht, Niederlande | 2017 | Temporär | Ausgeführt | 1 | 3 | 0/3 | 1 | 0 | 1 | **PK-03** |
| P50 | Resource Rows | Kopenhagen, Dänemark | 2019 | Gebäude | Ausgeführt | 1 | 3 | 0/3 | 2 | 1 | 2 | keine Mengen trotz prominenter Ziegelfassaden-Module |
| P51 | Upcycle Studios | Kopenhagen, Dänemark | 2018 | Gebäude | Ausgeführt | 1 | 3 | 0/3 | 3 | 0 | 0 | **PK-21**, PK-38 |
| P52 | TRÆ High-Rise | Aarhus, Dänemark | 2025 | Gebäude | Ausgeführt | 1 | 8 | 0/8 | 8 | 0 | 7 | **PK-20**; alle 8 Zeilen ohne ReUse-Ort, keine Mengen |
| P53 | Woongroep Boschgaard | ’s-Hertogenbosch, Niederlande | 2022 | Gebäude | Ausgeführt | 2 | 2 | 1/2 | 2 | 0 | 0 | **PK-22** |
| P54 | Kindergarten Mööslistrasse | Zürich, Schweiz | 2023 | Gebäude | Ausgeführt | 1 | 4 | 0/4 | 1 | 0 | 1 | PK-34 |
| P55 | Brighton Waste House | Brighton, Vereinigtes Königreich | 2014 | Prototyp | Ausgeführt | 1 | 8 | 5/8 | 7 | 0 | 5 | **PK-07**; Prio 4 |
| P56 | Hastings Pier Visitor Centre | Hastings, Vereinigtes Königreich | 2017 | Gebäude | Ausgeführt | 1 | 2 | 1/2 | 0 | 0 | 2 | **PK-01** (Spaltenversatz), PK-16 |
| P57 | Kamikatsu Zero Waste Center | Kamikatsu, Japan | 2020 | Gebäude | Ausgeführt | 1 | 2 | 1/2 | 0 | 0 | 1 | einziges außereuropäisches Projekt außer USA |
| P58 | People’s Pavilion | Eindhoven, Niederlande | 2017 | Temporär | Ausgeführt | 1 | 7 | 0/7 | 0 | 2 | 1 | **PK-26**; einzige Karte mit Leihgabe-Modell durchgängig |
| P59 | Pavillon Circulaire | Paris, Frankreich | 2015 | Temporär | Ausgeführt | 1 | 5 | 2/5 | 0 | 0 | 3 | **PK-12** (FLAG), PK-42 |
| P60 | Christ Pavilion | Volkenroda, Deutschland | 2001 | Gebäude | Ausgeführt | 1 | 1 | 1/1 | 0 | 0 | 0 | Gesamtstruktur-Verpflanzung |
| P61 | Plattenvereinigung | Berlin, Deutschland | 2010 | Temporär | Ausgeführt | 1 | 6 | 0/6 | 0 | 1 | 0 | **PK-11**, PK-38, PK-40; EPFL-Cluster |
| P62 | Plattenpalast | Berlin, Deutschland | 2009 | Prototyp | Ausgeführt | 2 | 2 | 2/2 | 0 | 0 | 0 | PK-38; mengenvollständig |
| P63 | SUPERLOCAL Expogebouw | Kerkrade, Niederlande | 2019 | Prototyp | Ausgeführt | 1 | 1 | 0/1 | 0 | 0 | 0 | Gesamtstruktur |
| P64 | CascadeUp Glulam Demonstrator | London, Vereinigtes Königreich | 2024 | Prototyp | Ausgeführt | 1 | 2 | 0/2 | 2 | 0 | 2 | Referenzkodierung für R4 (`Produktionsrest → Tragwerk`) |
| P65 | Re:Crete Footbridge | Lausanne, Schweiz | 2021 | Prototyp | Ausgeführt | 1 | 1 | 1/1 | 1 | 0 | 0 | Umnutzung ohne Pfeil, aber mit „Neue Nutzung" – regelkonform |
| P66 | Bestandsverpflanzung Pavillon | München, Deutschland | 2008 | Prototyp | Ausgeführt | 2 | 1 | 1/1 | 0 | 0 | 0 | Gesamtstruktur |
| P67 | Montessori Maassluis | Maassluis, Niederlande | 2027* | Gebäude | Im Bau | 1 | 2 | 0/2 | 1 | 1 | 0 | PK-33; Stern-Konvention korrekt |

## Projektweise Vertiefung – Recherchelauf 16.08.2026

### Leseschlüssel und Abgrenzung

Dieser Abschnitt prüft alle **67/67 Projektkarten** gegen zusätzliche Primär-, Fach- und Forschungsquellen. Er unterscheidet konsequent zwischen (a) Erhalt des vorhandenen Gebäudes, (b) direkter Wiederverwendung eines Bauteils, (c) Aufbereitung mit Funktions- oder Geometrieänderung, (d) Recycling zu einem neuen Werkstoff und (e) bloßer Planung für spätere Demontage. Prozentwerte werden nur mit ihrer jeweiligen Bezugsgröße geführt. `B` = extern belegt, `E` = Eigenangabe eines Projektbeteiligten, `K` = Quellenkonflikt, `O` = öffentlich nicht publiziert. `n. p.` bedeutet, dass das Feld in den geprüften Quellen nicht publiziert wurde; `—` bedeutet, dass es für das Projekt nicht zutrifft. Die Befunde ändern die Projektkarten noch nicht automatisch, sondern bilden ein belastbares Korrektur- und Importregister.

### P01 – Kopfbau Halle 118, Winterthur

**Identität und Beteiligte.** Aufstockung und Umbau einer Werkhalle, fertiggestellt 2021; Baubüro in situ, Zirkular und ZHAW begleiteten Entwurf, Beschaffung und Auswertung. Zwölf Werk- und Atelierräume von jeweils rund 60 m² entstanden. **Reuse und Herkunft.** Rund 430 t Re-Use-Elemente kamen aus einem Radius bis 90 km; belegt sind das Stahlskelett einer Coop-Verteilzentrale in Basel, eine mehr als 30 Jahre alte Außentreppe, Fenster und Granitplatten des Bürogebäudes Orion in Zürich sowie regionale Fassadenbleche, Heizkörper, Holzböden, Sanitärteile und eine PV-Anlage. Die Fensterzahl variiert je nach Abgrenzung zwischen 44 und 80 (`K`). **Prozess, Wirkung und Kosten.** Umgekehrte Planung nach verfügbaren Bauteilmaßen, Zustands- und Funktionsprüfung von mehr als 2.750 Sonnenschutz-Einzelteilen und Reparatur nur bei Bedarf. Die Quellen nennen 60 % weniger Erstellungsemissionen beziehungsweise rund 500 t CO₂; für untersuchte Einzelbauteile 85–99 % weniger THG. Vor Baustart entfielen rund 11 % der Erstellungskosten auf gebrauchte Bauteile. **Datenentscheidung.** 430 t, Radius 90 km und 60 % sind die besser abgegrenzten Vergleichswerte; 500 t CO₂ bleibt als projektspezifische Eigen-/Sekundärangabe separat. Quellen: [Bundesamt für Energie](https://pubdb.bfe.admin.ch/de/publication/download/12590), [Schenker/In-situ-Projektdaten](https://www.storen.ch/de/inspiration/projekt-k118/), [LUBW](https://www.lubw.baden-wuerttemberg.de/abfall-und-kreislaufwirtschaft/inzibau-good-practice/-/asset_publisher/wcwKprBkZG7i/content/kopfbau-halle-118), [LCA-Fachartikel](https://www.sciencedirect.com/science/article/pii/S2210670720305436).

### P02 – BedZED, London

**Identität.** 2002 fertiggestelltes Wohn- und Mischnutzungsquartier in Sutton/London, entwickelt von Peabody und Bioregional, Entwurf ZEDfactory/Bill Dunster. **Reuse und Herkunft.** 98 t rückgewonnener Baustahl deckten rund 95 % des konstruktiven Stahlbedarfs; die Abbruchquellen lagen innerhalb eines Radius von 35 Meilen. Das in der Karte geführte Holzvolumen von 54.000 laufenden Metern bleibt als Betreiberangabe bestehen, ist aber in der vertieften unabhängigen Quelle nicht reproduziert (`O`). **Prozess und Qualität.** Die Tragwerksplanung ließ Bandbreiten für Profilgrößen zu. Sichtprüfung und Materialbewertung umfassten Zustand, Alter, vorhandene Verbindungen und Eignung für erneute Fertigung. **Datenentscheidung.** 98 t und 95 % sind belastbar; Stahl und Holz nicht zu einer gemeinsamen Re-Use-Quote addieren. Quellen: [Bioregional](https://www.bioregional.com/projects-and-services/case-studies/bedzed-the-uks-first-large-scale-eco-village), [Buildings 2024](https://www.mdpi.com/2075-5309/14/4/979).

### P03 – BioPartner 5, Leiden–Oegstgeest

**Identität und Beteiligte.** Labor- und Bürogebäude im Leiden Bio Science Park, 6.827 m², fertiggestellt 2021. **Reuse und Herkunft.** 165.000 kg Stahl aus dem rund 750 m entfernten Gorlaeus-Hochhaus wurden auf dem Campus ausgebaut und verarbeitet; weitere Re-Use-Posten sind Pflaster, Innenwände, Bodenfliesen, Teppiche und Möbel. Die Konstruktion ist demontierbar. **Wirkung und Qualität.** Die Dekonstruktion des Spendergebäudes wurde BREEAM-Excellent bewertet; die Projektkommunikation beschreibt den Vorgang als CO₂-neutral. **Datenentscheidung.** Die bisherige Katalogmenge „ca. 150 t“ auf **165 t** präzisieren; 150 t ist eine gerundete Frühangabe. Quellen: [Universität Leiden](https://www.universiteitleiden.nl/en/news/2020/11/gorlaeus-highrise-lives-on-in-two-new-buildings-in-leiden), [Nationale Staalprijs](https://www.nationalestaalprijs.nl/project/biopartner-5), [Leiden Bio Science Park](https://leidenbioscienceparkprojects.nl/en/sustainability/circulariteit).

### P04 – Kristian Augusts gate 13, Oslo

**Identität und Beteiligte.** Bürotransformation und Erweiterung, 4.297 m², fertiggestellt 2021; Bauherr Entra, MAD arkitekter, Asplan Viak, Scenario. Bestand 2.734 m², Untergeschoss 708 m², Erweiterung 855 m². **Reuse und Herkunft.** Materialien aus mindestens 25 lokalen Spenderprojekten: 20.000 Ziegel, 21 Hohlplatten, rund 70 % des Stahls, etwa 100 m² Parkett, 2.200 m² Teppichfliesen, 1.500 m² Mineralwolle-Deckenplatten, 340 m² Keramikfliesen, 85 m² Granit, 100 m² Terrassenholz, 12 Schlauchschränke und 58 m Kabeltrassen. **Prozess und Wirkung.** Kalkmörtel, Schraubverbindungen, lösbare Trennwände und Bauteilrückverfolgung; laut FutureBuilt knapp 80 % wiederverwendete Materialien und rund 70 % weniger THG gegenüber Referenz. **Datenentscheidung.** Projektquote nur mit der FutureBuilt-Systemgrenze speichern; Bauteilmengen einzeln importieren und die ursprünglichen Nutzungsebenen erhalten. Quellen: [FutureBuilt-Projektseite](https://www.futurebuilt.no/forbildeprosjekter/kristian-augusts-gate-13-oslo), [FutureBuilt-Erfahrungsbericht](https://www.futurebuilt.no/assets/originals/download/cfc84d1c2685849e18a5cacc24276c6a.pdf), [Nordischer LCA-Bericht](https://pub.norden.org/temanord2022-551/).

### P05 – Recypark Demets, Anderlecht

**Identität und Beteiligte.** 2024 fertiggestellter Recypark, rund 5.000 m², Budget 8,3 Mio. € ohne MwSt. und Honorare; Bauherr Bruxelles-Propreté, 51N4E und Les Marneurs, Greisch/Witteveen+Bos, Rotor DC, Eiffage Art Valens. **Reuse.** Zwanzig Brettschichtholz-Dachbögen aus einer ehemaligen Reithalle nahe Lüttich wurden als Haupttragwerk eingesetzt. **Prozess.** Bauteilprüfung und erneute konstruktive Verwendung wurden von Tragwerks- und Reuse-Fachplanung begleitet; eine publizierte Gesamtmasse, Transportdistanz und LCA fehlen (`n. p.`). **Datenentscheidung.** 20 Stück beibehalten, Spenderobjekt „ehemalige Reithalle nahe Lüttich“ ergänzen; keine Masse ableiten. Quellen: [Brussels Architecture Prize](https://brusselsarchitectureprize.be/en/project/recypark-demets/), [Rotor](https://rotordb.org/en/projects/recypark-anderlecht), [Reuse-Bericht Brüssel](https://bma.brussels/app/uploads/2024/10/The-architecture-of-reuse-in-Brussels.pdf).

### P06 – The Swan Kindergarten, Gladsaxe

**Identität und Beteiligte.** Umbau einer Schule zum Kindergarten, 1.436 m², 2019–2022; Gladsaxe Kommune, Lendager, Sweco und NIRAS. **Reuse.** Auf dem Gelände selektiv demontiert, gelagert und wieder eingesetzt wurden 61.500 Ziegel, 12.000 Dachziegel, Holz-Dachbinder und rund 2.600 Stahlblech-Fassadenelemente. **Wirkung und Qualität.** Projektquellen nennen 6.200 t vermiedenen Materialverbrauch und 178 t CO₂; Nordic-Swan-Zertifizierung. Die 6.200 t sind eine vermiedene/erhaltene Materialgröße, nicht die Masse der vier Katalogposten. **Datenentscheidung.** Die vier Bauteilzeilen um Stückzahlen ergänzen; 6.200 t separat als projektbezogene Wirkung mit Eigenangabenstatus führen. Quellen: [Lendager](https://lendager.com/project/the-swan/), [Metropolis](https://metropolismag.com/projects/lendager-completes-the-worlds-first-ecolabeled-kindergarten/), [Springer-Fallstudie](https://link.springer.com/article/10.1007/s44150-025-00161-3).

### P07 – Villa Welpeloo, Enschede

**Identität.** Wohnhaus von Superuse Studios, fertiggestellt 2009; publizierte Flächen schwanken zwischen 250 und 312 m² (`K`), Baukosten rund 0,9 Mio. €. **Reuse und Herkunft.** Etwa 60 % des Materials wurden innerhalb von maximal 30 km, mit Schwerpunkt 18 km, beschafft: Stahl aus einer Textilmaschine, Holzfassade aus Kabeltrommeln, Gewächshausglas, Dämmung und weitere Bauteile. Zur Zahl der Kabeltrommeln finden sich widersprüchliche Angaben von etwa 200 bis „über tausend“ (`K`). **Prozess und Wirkung.** Entwurf nach lokalem Harvest Map, niedrigste plausible Materialgüte angenommen, überwiegend mechanische Verbindungen; für Tragwerk und Fassade werden rund 90 % CO₂-Einsparung angegeben. **Datenentscheidung.** Prozent- und Distanzwerte mit Systemgrenze speichern; keine Kabeltrommelzahl ohne Primärbeleg importieren. Quellen: [Superuse](https://www.superuse-studios.com/projectplus/villa-welpeloo/), [Circular Material Systems](https://circularmaterialsystems.com/en/case/villa-welpeloo/), [SE2050-Fallstudie](https://se2050.org/wp-content/uploads/2024/07/SEI-CE-WG-Circular-Economy-Case-Studies_5-Villa-Welpeloo.pdf).

### P08 – Holbein Gardens, London

**Identität.** Büro-Retrofit von Grosvenor, fertiggestellt 2023. **Reuse und Herkunft.** Insgesamt **24 t** wiederverwendeter Stahl: 9 t aus Grosvenors Biscuit-Factory-/Eigenbestand und 15 t über Cleveland Steel; zusätzlich Hohlraumboden, Fliesen, York-Stone-Pflaster und Ziegel. **Qualität, Wirkung und Kosten.** Prüfung, Sortierung und mechanische Verbindungen; 67,5 t CO₂e für den Stahl und 3,2 t CO₂e für York Stone vermieden. Wiederverwendeter Stahl war wegen Ausbau und Prüfung teurer als Neuware. **Datenentscheidung.** Die Karte enthält beide Stahlzeilen bereits, aber ein Projekt-Summenfeld muss 24 t lauten; 9 t darf nicht als Gesamtmenge erscheinen. Quelle: [ASBP-Fallstudie](https://asbp.org.uk/case-studies/holbein-gardens), [Grosvenor](https://www.grosvenor.com/news-insights/some-of-uk%E2%80%99s-first-salvaged-steelwork-reused-in-holbein-gardens-retrofit).

### P09 – Werkhof 29, Zürich

**Identität.** Aufstockung Grubenstrasse, fertiggestellt 2025; Bauherr Modissa, Entwurf in situ. **Reuse.** Belegt sind 650 m² Fassadenblech, 145 m² Aluminiumdach, 19 Fenster, 25 Türen, 6 Küchen, eine Außentreppe, 40 Heizkörper, 700 m² PIR-Dachdämmung, 549 m² Faserzementplatten und 20 Briefkästen; ergänzend rund 125 m³ Stroh (ca. 600 Ballen) und 14 m³ Aushublehm. **Kosten und Prozess.** Re-Use-Materialkosten rund 200.000 CHF beziehungsweise 2,7 % des Gesamtbudgets; Spenderposten wurden aus verschiedenen Regionen beschafft und an den verfügbaren Bestand angepasst. Eine publizierte Gesamt-Reuse-Masse und LCA fehlen. **Datenentscheidung.** Die bisher mengenleere Karte kann bei fünf vorhandenen Zeilen direkt ergänzt und um weitere belegte Bauteile erweitert werden. Quellen: [TEC21-Dokumentation](https://www.b-3.ch/userdata/publikationen/tec21-2025-20-grubenstrasse.pdf), [Espazium](https://www.espazium.ch/de/aktuelles/in-situ-werkhof-29), [in situ](https://insitu.ch/projekte/351-werkhof-29-aufstockung-grubenstrasse).

### P10 – Haus HOS, Mühlhausen

**Identität.** Dreigeschossiges Wohnhaus, 250 m² Nutzfläche, fertiggestellt 2008; Seidl & Seidl. **Reuse und Herkunft.** 58 WBS70-Elemente – 28 Wand-, 23 Deckenplatten und 7 Treppen – mit rund 190 t Gesamtmasse aus Leinefelde, Transport etwa 30 km. Rund 75 % des Rohbaus bestanden aus wiederverwendeten Elementen. **Kosten.** Produktionskosten rund 300.000 €; etwa 25 % Kostenvorteil gegenüber konventioneller Ausführung wird angegeben. **Datenentscheidung.** Die Kartenmengen sind korrekt; Masse, Distanz und Bezugsgröße ergänzen. Quellen: [Seidl & Seidl Projektdokumentation](https://www.seidlarchitekten.de/wp-content/uploads/2022/10/SeidlSeidl-Architekten_Haus-Hos_Muehlhausen.pdf), [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid).

### P11 – Mehrow Pilot House

**Identität.** Privates Flachdach-Wohnhaus, fertiggestellt 2005. **Reuse und Herkunft.** 22 Wand- und 27 Deckenplatten des Systems WBS70, zusammen 118 m³; Spenderentfernung je nach Spenderblock 8 beziehungsweise 17 km, Bauteilalter etwa 21 Jahre. **Prozess.** Einige Elemente wurden zugeschnitten; Bauteile wurden geprüft und logistisch just-in-time remontiert. **Datenentscheidung.** 49 Elemente und 118 m³ als getrennte Mengen führen; Entfernung als Bandbreite, nicht als Mittelwert. Quellen: [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid), [Journal of Cleaner Production](https://infoscience.epfl.ch/server/api/core/bitstreams/00948e56-68fd-4f05-9280-5e835b8d2570/content).

### P12 – Bröthen Twin-House, Hoyerswerda

**Identität.** Doppelhaus, fertiggestellt 2001. **Reuse.** 26 Wand- und 50 Deckenplatten des Systems P2, insgesamt 76 wiederverwendete Komponenten; Atlas-Angabe zur Entfernung etwa 6 km und Bauteilalter rund 32 Jahre. **Konfliktbereinigung.** Die Angabe „200 Stück/245 m³ aus 60 Decken- und 50 Innenwandplatten“ gehört **nicht** zu Bröthen, sondern zum Schildow-Projekt P31. **Datenentscheidung.** Kartenwerte 26 + 50 bestätigen; den Schildow-Datensatz nicht auf P12 übertragen. Quellen: [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid), [JCP-Vergleichsstudie](https://infoscience.epfl.ch/server/api/core/bitstreams/00948e56-68fd-4f05-9280-5e835b8d2570/content).

### P13 – CRCLR House, Berlin

**Identität und Beteiligte.** Transformation und Holzaufstockung in Berlin-Neukölln, fertiggestellt 2023; ZRS Architekten/Ingenieure, Größenangaben 4.871 m² Nutz-/Projektfläche beziehungsweise 6.100 m² BGF (`K` durch unterschiedliche Flächenbegriffe). **Reuse.** Rund 120 Stahlträger bis 18 m Länge, 106 Fenster, Brandschutztüren, Fassadenbleche, Schiebetüren und Holzverschnitt. **Qualität und Wirkung.** Jeder vierte Stahlträger wurde beprobt; Zug- und chemische Prüfungen, Entrosten und Korrosionsbewertung. Rund 70 % der Innenausbaumaterialien stammen aus Rückbau, Messen, Museen oder Lagerbeständen. Die Quellen nennen 615 t CO₂e vermieden durch Erhalt statt Abriss; weitere 150 t gespeicherter Kohlenstoff und über 600 t gegenüber Neubau sind anders abgegrenzte Angaben. **Kosten.** BGF 6.100 m², KG 300/400 rund 7,4 Mio. € netto, Gesamtangabe 17,11 Mio. € in anderer Kostengrenze. **Datenentscheidung.** Flächen, Kosten und CO₂-Werte nie ohne jeweilige Systemgrenze zusammenführen. Quellen: [DBZ](https://www.dbz.de/artikel/crclr-house-berlin-3945221.html), [nbau](https://www.nbau.org/2022/12/08/transformation-bauen-das-crclr-haus-in-berlin/), [ZRS](https://www.zrs.berlin/en/project/crclr-house-2/).

### P14 – Recyclinghaus Hannover

**Identität.** Wohnhaus-Prototyp, rund 285 m² BGF, fertiggestellt 2019; Bauherr Gundlach, Cityförster. **Materialstrategie.** Fundament aus Recyclingbeton mit bis zu 60 % Rezyklat und Schaumglasschotter, Holztragwerk lösbar gefügt; Re-Use-Posten umfassen Glasfassadenelemente aus Bauherrenbestand, historische Türen, Platten, Fliesen und Holz. Rezyklat, nachwachsende Neuware und direktes Re-Use müssen getrennt bleiben. **Prozess.** Suche nach gebrauchten Bauteilen begann etwa zwei Jahre vor Baubeginn; Konstruktion und Raster wurden an Funde angepasst und verschraubt. **Datenentscheidung.** Keine Gesamt-Reuse-Quote publiziert; die bisherige Karte darf Recyclingbeton nicht als direkte Bauteilwiederverwendung zählen. Quellen: [Cityförster-Projektdaten](https://www.cityfoerster.net/fileadmin/pdf/projects/CF-PR_1532-CYC_Recyclinghaus_Kronsberg_A4_20180425_EN.pdf), [RWTH-Fallstudie](https://publications.rwth-aachen.de/record/952715/files/952715.pdf), [Deutsches Architektenblatt](https://www.dabonline.de/architektur/recyclinghaus-in-hannover-von-cityfoerster-baustoffe-baumaterialien/).

### P15 – Thoravej 29, Kopenhagen

**Identität und Beteiligte.** Transformation eines 1967 errichteten Industrie-/Bürobaus, 6.336 m², fertiggestellt Anfang 2025; Bikubenfonden, Pihlmann Architects, Hoffmann A/S, ABC Rådgivende Ingeniører. Umbaukosten 120 Mio. DKK beziehungsweise rund 19.200 DKK/m². **Reuse.** 95 % der vorhandenen Materialien blieben erhalten oder wurden im Haus umgenutzt: TT-Decken wurden zu Treppen und Möbeln, Fassadenmauerwerk zu Böden, Türen/Holz zu Platten und Tischoberflächen; funktionsfähige Kunststofffenster bleiben für ihre Restlebensdauer. **Wirkung.** DTU-Auswertung: bis 88 % weniger CO₂ als Neubau und bis 90 % weniger Abfall; DGNB-Gold-Vorzertifizierung. **Datenentscheidung.** 95 % ist Erhalt/Umnutzung des **vorhandenen** Materials, nicht Anteil aller eingebauten Massen. Quellen: [Thoravej 29](https://www.thoravej29.dk/en/sustainability), [Architectural Review](https://www.architecture-now.co.uk/article/115050/material-remix-thoravej-29-in-copenhagen-by-pihlmann-architects), [Renoverprisen](https://renover.dk/projekt/thoravej-29/).

### P16 – Timber Square, London

**Identität.** Großes Büroprojekt von Landsec; die Katalogkarte führt 2026 als fertiggestellt. Der Live-Status ist vor Import weiterhin projektseitig zu bestätigen (`O`), da Quellen teils Ziel- statt Übergabedaten verwenden. **Erhalt und Reuse.** Rund 80 % des ursprünglichen Tragwerksvolumens bleiben erhalten und bilden etwa 25 % des neuen Gebäudes; dadurch werden rund 7.300 t CO₂e vermieden. 33.450 m² wiederverwendete Doppelbodenplatten sparen laut UKGBC weitere 1.362 t CO₂e. Ein Teil der Stahlmenge wird mit etwa 115 t angegeben; Abgrenzung zu erhaltenem Bestandsstahl ist offen. **Zielwert.** 552 kg CO₂e/m² GIA, beziehungsweise 433 kg bei Anrechnung von Kohlenstoffspeicherung. **Datenentscheidung.** Bodenplattenmenge bestätigen; Stahl und Fertigstellungsstatus bis zu einem aktuellen Abschlussbeleg als vorläufig markieren. Quellen: [UKGBC](https://ukgbc.org/resources/timber-square/), [Landsec](https://www.landsec.com/places/timber-square).

### P17 – TBC.London, London

**Identität.** Bürogebäude nahe Tower Bridge, fertiggestellt 2025. **Reuse.** Abschlussunterlagen nennen 40 t Stahl aus dem Tragwerk der 1930er Jahre, rund 20 % des Projektstahls und mehr als 100 t CO₂e Einsparung. Eine frühere Projektmeldung nannte 16 t; dies ist ein Planungs-/Zwischenstand und wird durch die Abschlussmenge ersetzt (`K`). **Weitere Wirkung.** 98,96 % Baustellenabfall von Deponie ferngehalten, BREEAM Outstanding und WELL Platinum; Betriebsenergie laut Southwark 71 % besser als Referenz. **Datenentscheidung.** Kartenwert von **16 t auf 40 t** aktualisieren und den früheren Wert als historische Quelle kennzeichnen. Quellen: [Southwark Council](https://www.southwark.gov.uk/news/2025/southwark-sets-standard-boroughs-first-living-wage-building), [Completion Brochure](https://s3-eu-west-1.amazonaws.com/agents-society-assets-files/f029cdd8858c1339f9b8015b10536218-tbc-completion-brochure-2025.pdf), [Willmott Dixon – Frühstand](https://www.willmottdixon.co.uk/now-or-never/case-studies/tower-bridge-court-sets-standard-for-reuse-of-building-materials).

### P18 – 55 Great Suffolk Street, London

**Identität und Status.** Geplantes/entwickeltes Büroprojekt von Fabrix/HawkinsBrown; die tatsächliche Fertigstellung und Ausführung der Re-Use-Beschaffung bleibt öffentlich nicht abschließend bestätigt (`O`). **Stahl.** Ein Fachbericht beschreibt mehr als 20 t wiederverwendeten Stahl aus 1 Broadgate, entsprechend 97 % des für den betrachteten Projektteil vorgesehenen Stahls. 139 t wurden von Fabrix **für zwei Projekte zusammen** – 55 Great Suffolk Street und Roots in the Sky – beschafft; die Katalogzuordnung von 139 t allein an P18 ist falsch. Eine frühere UKGBC-Fallstudie nennt 9,5 t und 25 t CO₂e Einsparung als damaligen Planungsstand. **Datenentscheidung.** 139 t entfernen; „>20 t vorgesehen, Ausführung unbestätigt“ führen, bis ein Abschlussbeleg vorliegt. Quellen: [ADVANCE-Stahlbericht](https://www.steelconstruct.com/wp-content/uploads/ADVANCE-D2.1-Circular-Economy-of-Steel-Based-Components.pdf), [UKGBC](https://ukgbc.org/wp-content/uploads/2022/08/Whole-Life-Carbon-Circular-Economy-Report.pdf), [HawkinsBrown](https://www.hawkinsbrown.com/projects/55-great-suffolk-street).

### P19 – Brent Cross Town Primary Substation, London

**Identität.** Primärumspannwerk, fertiggestellt 2023. **Reuse und Herkunft.** 33,46 t wiederverwendeter Stahl aus ungenutztem Öl-/Gaspipelinebestand über Cleveland Steel; 45 % des geplanten Stahls. **Qualität, Wirkung und Kosten.** Unabhängige Werkstoff- und Schweißprüfung nach SCI P427, CE-/UKCA-Dokumentation; rund 66 t CO₂e beziehungsweise etwa 40 % der stahlbezogenen Emissionen vermieden. Rohmaterial etwa 50 % und vollständig eingebaute Lösung etwa 25 % günstiger als Neuware. **Datenentscheidung.** Kartenmenge ergänzen; Herkunft als unbenutzter Lagerbestand kennzeichnen, nicht als ausgebautes Bauteil. Quelle: [Alliance for Sustainable Building Products](https://asbp.org.uk/case-studies/brent-cross-town-primary-substation).

### P20 – Boulder Fire Station 3, Colorado

**Identität.** Feuerwehrstation, 28.300 ft², fertiggestellt 2024; Stahlspender war das ehemalige Boulder Community Hospital. **Reuse.** Von 584 ausgebauten Stahlprofilen – rund 98 % des Spender-Rohbaus – wurden 89 Profile in der Feuerwache eingesetzt. Die Katalogangabe 22 t ist als frühe/gerundete Masse zu kennzeichnen. **Prozess und Qualität.** Rund 10 % der Spenderprofile geprüft; weniger als 5 % zeigten Schäden. Brandschutzbeschichtung wurde entfernt, Profile inventarisiert und neu bemessen. **Wirkung und Kosten.** Projektbezogene Einsparung je nach Berechnung 25.000 beziehungsweise 36.344 kg CO₂e (`K` Methodenversion); Spenderlager insgesamt 167.338 kg CO₂e. 93,5 % Abbruchmasse umgeleitet; Dekonstruktion 9,2 Mio. US-$ versus 7,7 Mio. US-$ konventioneller Abriss. **Datenentscheidung.** Stückzahl 89 als Primärmenge, CO₂ mit Berechnungsversion speichern. Quellen: [AISC Owner Toolkit](https://www.aisc.org/globalassets/aisc/sustainability/sustainabilitytoolkitforowners.pdf), [Modern Steel](https://www.aisc.org/modern-steel/news/inside-davis-partnerships-reuse-of-steel-in-a-new-fire-station/).

### P21 – Big Dig House, Lexington

**Identität.** Wohnhaus, fertiggestellt 2006; Entwurf Single Speed Design/Project Architecture. Publizierte Flächen reichen von etwa 3.400 bis 4.300 ft² (`K` nach Bezugsgröße). **Reuse.** 17 „Inverset“-Stahl-Beton-Fahrbahnsegmente einer Big-Dig-Auffahrtsrampe wurden als Decken verwendet; Gesamtgewicht aus Stahl und Beton über 600.000 lb beziehungsweise rund 272 t. Einzelne Segmente waren bis 40 ft lang und mehr als 20 US-ton schwer. **Prozess und Kosten.** Tragwerk in rund drei Tagen montiert; Baukostenangabe etwa 645.000 US-$. **Datenentscheidung.** 17 Segmente und Gesamtmasse ergänzen; Stahl und Beton nicht als zwei unabhängig gezählte Bauteilmengen duplizieren. Quellen: [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid), [Project Architecture](https://projectarchitecture.com/big-dig-house), [ASCE-Fallstudie](https://www.markfitz.work/s/bigdig-ITF.pdf).

### P22 – Saxum Vineyard Equipment Barn, Paso Robles

**Identität und Beteiligte.** Landwirtschaftliche Gerätehalle, 2.340 ft²/217 m², fertiggestellt 2018; Clayton Korte, SSG/Buehler, Rarig Construction. **Reuse.** Hauptstützen und Dachtragwerk aus wiedergewonnenen Ölfeld-Bohrrohren Schedule 40 mit 2, 3 und 3,5 Zoll Durchmesser; weitere verwitterte Stahlrahmen als Bekleidung. Eine Rohrstückzahl oder Masse ist nicht publiziert (`n. p.`). **Betrieb.** Glas-Photovoltaikmodule bilden das Dach und kompensieren mehr als 100 % des Strombedarfs der Weinkellerei; geringe Ortbetonfundamente und versickerungsfähige Flächen. **Datenentscheidung.** Keine Menge schätzen; Maße, frühere Funktion und Off-grid-/PV-Wirkung ergänzen. Quellen: [SE2050](https://se2050.org/wp-content/uploads/2025/11/SEI-CE-WG-Circular-Economy-Case-Studies_11-Saxum-Vineyard_2025.pdf), [ArchDaily](https://www.archdaily.com.br/pt/935640/equipamento-para-vinicola-saxum-clayton-and-little).

### P23 – Europa Building, Brüssel

**Identität.** Sitz des Europäischen Rates im umgebauten Résidence Palace, fertiggestellt 2016, Gesamtfläche 70.646 m²; rund 40 % des Projekts sind renovierter Bestand. **Reuse.** Etwa 3.750 Eichenfensterrahmen aus Gebäuden aller EU-Mitgliedstaaten wurden geschliffen, repariert, neu lackiert und in Edelstahlrahmen zur charakteristischen Außenhülle montiert. **Materialeffizienz.** Die Fassadenkonstruktion benötigte laut ausführendem Unternehmen rund 30 % weniger Stahl als eine konventionelle Lösung. **Datenentscheidung.** 3.750 Stück bestätigen; Herkunft „EU-weit, mehrere Spendergebäude“ und Aufbereitungsprozess ergänzen. Quellen: [EU-Factsheet](https://www.consilium.europa.eu/media/24207/2016-12-05-press-pack-europa-buildng-factsheet-final-en.pdf), [Jan De Nul](https://www.jandenul.com/projects/europa-building-brussels-belgium).

### P24 – ELYS Kultur- und Gewerbehaus, Basel

**Identität.** Transformation des Lysbüchel-Silos zum Kultur- und Gewerbehaus, fertiggestellt 2021; Zirkular begleitete die Wiederverwendung. **Reuse.** Für die neue, rund 1.000 m² große Fassadenfläche wurden Re-Use-Elemente eingesetzt; rund 150 m³ Konstruktionsholz wurden gereinigt, neu zugeschnitten und verarbeitet, etwa 200 Fenster geprüft und wieder eingebaut. Mineralwolle-Verschnitt wurde als Dämmstoff eingesetzt. **Wirkung.** Direkte Re-Use-Maßnahmen sparen rund 91 t CO₂e; Erhalt und Umbau der Tragstruktur vermeiden im Vergleich zum Ersatzneubau rund 7.000 t CO₂e. **Datenentscheidung.** Die Katalogangabe „ca. 1.000 m² Trapezblech“ nicht ungeprüft als Materialmenge führen: Die Quelle bezeichnet primär die Fassadenfläche. 91 t und 7.000 t wegen verschiedener Systemgrenzen getrennt speichern. Quellen: [Zukunft Bau](https://www.zukunft-bau.at/en/project/sport-culture/adaptive-reuse-elys-cultural-and-commercial-building-lysbuchelareal), [Reuse-LCA-Bericht](https://zirkular.net/wp-content/uploads/2025/07/8169-20250331-reuse-lca-heig-vd-final-report-e-ec-vf2.pdf).

### P25 – Lycée Michel Lucius, Luxemburg

**Identität.** Umnutzung von Flügel 6000 zur 660-m²-Bibliothek und selektiver Rückbau von Flügel 3000; Projektabschluss/Publikation 2021–2023. **Reuse.** 88 Leuchten; fast 52 lfm Regale sowie Tische und Stühle aus ca. 20 km Entfernung; 135 m² Pflaster; 38 Betonfertigteile; 419 m² Gips-Akustikplatten; 12 Metallplatten/4,3 m²; Stahlfassadenpaneele; 61 m² Bodenblech; 11,8 t Stahlprofile. **Prozess und Qualität.** Selektiver Rückbau, getrennte Stoffströme, vereinbarte Lager-/Verantwortungskette; Laborversuche ermöglichten 60 % Recyclingzuschlag in neuen Betonteilen. **Wirkung.** Umbau erforderte 72 t neue Materialien statt 2.200–2.400 t bei Neubauvarianten, reduzierte Abbruchabfall um 79 % und sparte laut LCA 458–792 t CO₂e. **Datenentscheidung.** Alle neun Kartenmengen bestätigen; Recyclingzuschlag und Bestandserhalt separat vom direkten Re-Use ausweisen. Quellen: [Opalis](https://opalis.eu/en/projects/conversion-two-wings-lycee-michel-lucius), [Luxemburgische Bauverwaltung](https://abp.gouvernement.lu/fr/actualites.gouvernement2024%2Bfr%2Bactualites%2Btoutes_actualites%2Bcommuniques%2B2023%2B09-septembre%2B12-bausch-lucius.html), [Projekthandbuch](https://gouvernement.lu/dam-assets/documents/actualites/2023/09-septembre/12-bausch-lucius/11357-abp-broch-lml183x280-pp-web.pdf).

### P26 – Jeugdkliniek Ithaka, Kloetinge

**Identität und Beteiligte.** Kinder- und Jugendpsychiatrie, 3.334 m², fertiggestellt 2019; Emergis, Rothuizen, Lüning und ein regionales Sozialunternehmen. **Reuse und Herkunft.** Rund 20 km vom erst 17 Jahre alten Rijkswaterstaat-Büro Terneuzen: Außenfenster, Innentüren, Hartholzschindeln aus bereits zuvor wiederverwendeten Dalben, Holzböden, Pflaster sowie elektrische und technische Komponenten. Nahezu 40 % des Spendergebäudes wurden wiederverwendet. **Prozess.** Bestandspläne ermöglichten Materialinventar und digitale Entwurfsvarianten; Holzteile wurden in Middelburg gelagert, entnagelt, gereinigt und angepasst. Die FCRBE-Massenbilanz nennt 49.462 kg direkt wiederverwendete Materialien bei 675.369 kg Gesamtmasse, also 7,32 %. **Datenentscheidung.** Das 40-%-Spenderbergungsmaß und die 7,32-%-Empfängerquote nicht vermischen; Materialspalten der sechs Kartenzeilen konkretisieren. Quellen: [Rijkswaterstaat](https://magazines.rijksoverheid.nl/ienw/duurzaamheidsverslag/2018/01/circulaire-economie-rws), [Circonnect](https://www.circonnect.org/en/kennisbijdrage/circulaire-jeugdkliniek-rothuizen-architecten/), [FCRBE-Auswertung](https://www.cstb.fr/getmedia/602d356c-10ba-4b8f-b22b-50d7131b41e5/Projet-FCRBE-reemploi-materiaux-construction.pdf).

### P27 – gjG House, Gentbrugge

**Identität.** Einfamilienhaus von BLAF architecten, 190 m² BGF, fertiggestellt 2015. **Reuse und Konstruktion.** Selbsttragende, gekrümmte Außenhülle aus wiederverwendeten „Boomse machinesteen“-Ziegeln; die Form stabilisiert ohne Querwände, Stützen oder Balken und trägt zusammen mit dem Dach. Leichte Stahl- und Holzrahmen bilden den unabhängigen Innenausbau. **Menge und Wirkung.** Eine belastbare Stück-, Volumen- oder CO₂-Menge ist öffentlich nicht publiziert (`n. p.`). **Datenentscheidung.** Karte fachlich präzisieren, aber keine Ziegelzahl aus Fassadenfläche ableiten. Quellen: [BLAF](https://www.blaf.be/project/11-0452-gjg/), [ArchDaily](https://www.archdaily.com/951845/gjg-house-blaf-architecten), [Fachbeitrag Brick Wall City](https://cdn.blaf.be/wp-content/uploads/Eerlijk-Baksteen.pdf).

### P28 – Maison DnA, Asse

**Identität.** Privates Einfamilienhaus von BLAF, fertiggestellt 2013; Ausführung Nieuw-Ingels Kristof. **Reuse.** 50 m³ gebrauchte Ziegel von De Roover P. bilden die selbsttragende Außenhülle. Die unabhängige innere Holzrahmenschale trägt im Wesentlichen nur sich selbst und bleibt räumlich anpassbar; Schichten sind trennbar. **Qualität.** Ziegel wurden vorab geprüft und erreichten trotz ihres Alters bessere Eigenschaften als manche Neuware. **Datenentscheidung.** Kartenmenge bestätigen, Teststatus und tragende Funktion ergänzen. Quelle: [Opalis](https://opalis.eu/fr/projets/maison-dna-blaf-architecten).

### P29 – Association House Gröditz

**Identität.** Sportvereinsgebäude, fertiggestellt 2007. **Reuse.** 279 Komponenten – Außen- und Innenwände, Innenwandrahmen, Decken, Sockel und Treppen – aus einer Schule des Dresden-Typs sowie 159 WBS70-Paneele aus einem zweiten Spenderbau: zusammen **438 Komponenten**. Mauerwerkslagen glichen Höhen aus; Fassadenkomponenten wurden überlappend eingesetzt. **Datenentscheidung.** Die beiden Kartenzeilen sind korrekt, dürfen aber für Projektsummen addiert werden; Spendergebäude getrennt halten. Eine verlässliche Masse, Distanz und LCA sind `n. p.`. Quelle: [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid).

### P30 – Association House Plauen

**Identitätskonflikt.** Die Katalogkarte ist falsch gekoppelt: 17 Wand-, 14 Deckenplatten und eine Treppe beschreiben im Atlas das **Plauen house** von 2006, nicht das **Plauen association house**. **Richtiger Vereinsbau.** Für das Vereinsgebäude nennt der Atlas 145 Decken-, 19 Außenwand-, 14 Innenwand- und 11 Kellerwandplatten des Systems IW73/6, zusammen 189 Komponenten. **Datenentscheidung.** Projekt entweder in „Plauen house“ (32 Komponenten) umbenennen oder die vier Mengen des Vereinsbaus übernehmen; die aktuelle Mischkarte ist nicht importfähig. Quelle: [Atlas of Reused Concrete](https://concrete-reuse.epfl.ch/list?view=grid).

### P31 – Berlin-Schildow 2nd Pilot House

**Identität.** Zwei verbundene Wohnhäuser mit 186 und 101 m², zusammen rund 280 m², Baubeginn/Remontage 2005. **Reuse und Herkunft.** 200 zugeschnittene Teile mit 245 m³ aus 60 Decken-/Dach- und 50 Innenwandplatten des Systems WBS70, Transport 33 km, Bauteilalter etwa 18 Jahre. Auch das Satteldach besteht aus diagonal gesägten Re-Use-Platten; nur die Treppe ist neuer Beton. **Prozess und Wirkung.** neue demontierbare Schwerlastdübel und Stahlverbindungen; angegebene Energieäquivalenz 12.250 l Heizöl. **Datenentscheidung.** Die Kartenzeile „200 Bodenplatten“ ist falsch: 200 ist die Zahl der **zugeschnittenen Teile**, nicht der ursprünglichen Bodenplatten. Quellen: [Atlas](https://concrete-reuse.epfl.ch/list?view=grid), [Fachartikel 2007](https://d-nb.info/1003144454/34), [JCP](https://infoscience.epfl.ch/server/api/core/bitstreams/00948e56-68fd-4f05-9280-5e835b8d2570/content).

### P32 – Circular Centre Nederland, Heerde

**Identität und aktueller Stand.** Geplantes 8-geschossiges Hauptgebäude mit 6.000 m² Büro und 5.400 m² Halle; Baubeginn war im August 2026 noch für September 2026 angekündigt, daher nicht „ausgeführt“. Cepezed, IMd und Lagemaat. **Reuse und Herkunft.** Prinsenhof A in Arnhem lieferte 7.400 m² Hohldielen sowie Fassadenplatten, Wände, Treppen, Fenster und Oberflächen. Beim Spenderprojekt wurden 17.000 t Material, davon 12.000 t Beton, gewonnen; 92 % der Materialien seien wiederverwendet, angegebene Einsparung 3.500 t CO₂. Nicht alles davon geht zwingend in CCN. **Qualität und Prozess.** Laserscan/BIM, QR-Kennzeichnung, Druck- und Belastungstests; Hohldielen erwiesen sich als doppelt so tragfähig wie ursprünglich dokumentiert. **Datenentscheidung.** 7.400 m² als empfängerbezogene Menge führen; 17.000/12.000 t nur auf Spenderlauf verknüpfen. Quellen: [CCN](https://circulaircentrumnederland.nl/actueel/van-prinsenhof-a-naar-circulair-centrum-nederland/), [Lagemaat](https://lagemaat-heerde.nl/projecten/circulaire-ontmanteling-prinsenhof-a/), [IMd](https://imdbv.nl/project/698), [IVVD-Status 2026](https://www.ivvd.nl/kennisdossiers/vastgoed-duurzaamheid/circulair-centrum-nederland-in-heerde-bouwen-met-wat-er-al-staat).

### P33 – Recyclingzentrum Juch-Areal, Zürich

**Identität und Stand.** Öffentliches Pilotprojekt; Baubeginn 2026, Fertigstellung Ende 2027 geplant. Ausführungskredit 33,1 Mio. CHF; Graber Pulver, Perita, Weber + Brönnimann, Zirkular. **Reuse.** Stahltragwerk und Trapezbleche der ehemaligen Recyclinghalle Hagenholz werden 1:1 remontiert; Pilzstützen/Deckenteile aus der Schellinghalle Rümlang, Faserbetonplatten aus dem Kerenzerbergtunnel und Leitplanken. **Prozess und Wirkung.** Kennzeichnung, Eignungsprüfung, rechtliche Klärung, kontrollierte Demontage, Lagerung, BIM und Design for Disassembly. Prognose knapp 600 t CO₂ beziehungsweise gut 40 % weniger als konventioneller Neubau. **Datenentscheidung.** Status auf „im Bau“ setzen; Prognose als ex ante markieren, bis Abschlussmessung vorliegt. Quellen: [Stadt Zürich Projektseite](https://www.stadt-zuerich.ch/de/planen-und-bauen/projekte-und-ausschreibungen/hochbauvorhaben/planung-ausfuehrung/recyclingzentrum-juch-areal.html), [Stadt Zürich Studie](https://www.stadt-zuerich.ch/de/aktuell/publikationen/2026/reuse-tragstrukturen-studie.html).

### P34 – Melkinlaituri School and Day-care, Helsinki

**Identität und Stand.** Lebenszyklusprojekt der Stadt Helsinki, YIT; Baubeginn Anfang 2025, Fertigstellung Frühjahr 2027 geplant. **Reuse.** Rund 350 m² mehr als 40 Jahre alte Hohldielen aus einem multifunktionalen Gebäude der 1980er Jahre bilden Teile des Erdgeschossbodens. **Qualität und Prozess.** Inspektion, Prüfung und Werksaufbereitung bei Consolis Parma Nummela; Genehmigungs- und Materialeffizienzbegleitung durch Sustera. Ziel RTS vier Sterne, PV für mindestens 30 % des Beleuchtungs-/Gebäudestroms. **Datenentscheidung.** Status „im Bau/Elemente eingebaut“ und 350 m² bestätigen; kein abgeschlossenes Gesamt-LCA-Ergebnis publiziert. Quellen: [Sustera](https://sustera.com/melkinlaituri-primary-school-and-day-care-centre-pilot-project-for-the-reuse-of-hollow-core-slabs/), [Helsinki-Webinar](https://suite.icareus.com/web/helsinkikanava/player/vod?assetId=391448801).

### P35 – Härmälänranta/Ernst, Tampere

**Identität.** Wohnungsbau-Mini-Pilot von A-Kruunu/Skanska im ReCreate-Projekt; Einbau Herbst 2024. **Reuse.** 25 Hohldielen aus einem 1980er-Jahre-Bürogebäude in Tampere wurden über dem Schutzraum eingebaut. **Qualität und Prozess.** Prüfung und Werksaufbereitung bei Consolis Parma in Kangasala; Ramboll dokumentierte die bauplatzspezifische Eignung. Montage unterschied sich laut Baustelle nicht wesentlich von Neuplatten. **Wirkung und Kosten.** Quelle nennt rund 95 % geringeren CO₂-Fußabdruck je Element, aber damals noch höhere Kosten als Neuware. **Datenentscheidung.** 25 Stück bestätigen; 95 % als produktbezogene, nicht Gebäudequote führen. Quellen: [Yle](https://yle.fi/a/74-20127570), [Betoni](https://betoni.com/lehti/2024/12/23/betonielementtien-irrottaminen-ehjana-uudelleenkayttoa-varten/).

### P36 – Lokomotion Technology Centre, Tampere

**Identität und Stand.** Erster Bauabschnitt eines Metso-Produktionskomplexes, Skanska; Mini-Pilot im Sommer 2025, Gesamtzentrum wird in weiteren Phasen bis in die frühen 2030er erweitert. **Reuse.** 27 werksaufbereitete Hohldielen aus dem finnischen ReCreate-Spendergebäude wurden als Dächer eines freistehenden Technikbaus und von Personalräumen innerhalb der Halle eingesetzt. **Prozess.** Gemeinsam mit weiteren Piloten ermöglichte das Los eine temporäre Aufbereitungslinie bei Parma Nummela; kontrollierte Demontage, Transport, Aufarbeitung und projektspezifischer Nachweis. **Datenentscheidung.** „Ausgeführt 2025“ gilt für den Re-Use-Mini-Pilot, nicht für das gesamte Lokomotion-Vorhaben. Quelle: [ReCreate](https://recreate-project.eu/2026/02/24/second-reuse-mini-pilot-successful-in-finland/).

### P37 – Grande Halle de Colombelles

**Identität und Beteiligte.** 3.000 m² Industriehallen-Transformation zum Innovations- und Kulturzentrum, Übergabe Oktober 2019; Normandie Aménagement/EPFN, Encore Heureux + Construire, WIP. **Reuse.** Abschlussbilanz: 430 m² Steinwolldämmung, 29 Guss- und 30 Stahlheizkörper, 21 Holzpfosten, 45 Holzstücke für eine Treppe, 20 Sanitärobjekte und 50 Massivholztüren, darunter zwei Brandschutztüren. Dies korrigiert ältere Kartenwerte von 27/25 Heizkörpern, 18 Türen, 31 Sanitärobjekten und 200 m² Dämmung. **Prozess und Qualität.** Ein separates Reuse-Los suchte Quellen im 30-km-Radius, tatsächlich meist unter 5 km; Materialblätter, technischer Prüfer und Versicherungsdossier. Dämmung wurde auf Schallabsorption, Wärmeleitfähigkeit, Feuchte, Dichte und Maße getestet; Heizkörper wasserstrahl-entlackt, entrostet, lackiert und eingebrannt. **Datenentscheidung.** Abschlussmengen übernehmen und Frühwerte als ersetzt markieren. Quelle: [FCRBE-Versicherungsfallstudie](https://vb.nweurope.eu/media/21155/fcrbe_cashallecolombelle_final18oct2023_en.pdf).

### P38 – La Ferme du Rail, Paris

**Identität.** Urbane Farm und soziale Infrastruktur, 830 m² SDP, eröffnet 2019; Réhabail, Grand Huit, Bellastock und Sozialunternehmen. **Reuse.** Granitbordsteine als trocken gesetzte Stützwand, Asphalt-/Betonblöcke für Wege, Steinplatten, Restfliesen, Holz für Möbel, Fensterrahmen als Pflanztröge/Attika/Geländer und als Hirnholzparkett. **Prozess und Qualität.** Gelegenheiten wurden kurzfristig visuell bewertet; klassische Einbauweisen und frühe Abstimmung mit Prüfer/Versicherung. Ein Steinplattenlos erlitt Frostschäden im Lager und konnte nur noch als Füll-/Bodenplatte genutzt werden – wichtiger dokumentierter Verlustpfad. **Wirkung.** 90 % der Materialien werden als biobasiert und/oder wiederverwendet beschrieben; keine getrennte Re-Use-Masse publiziert. **Datenentscheidung.** 90 % nicht als direkte Re-Use-Quote speichern. Quelle: [Ekopolis](https://www.ekopolis.fr/operations-batiment/la-ferme-du-rail), [Circular Material Systems](https://circularmaterialsystems.com/en/case/05_ferme-du-rail/).

### P39 – Résilience/Ferme des Possibles, Stains

**Identität.** 1.882 m² großes bioklimatisches Mischgebäude für Novaedia, eröffnet September 2020; Archipel zéro/Frédéric Denise, Bellastock, SOCOTEC. **Reuse und Herkunft.** 300 identische Holzfenster aus einem rund 4 km entfernten Wohnungsumbau bilden eine große Glasfassade; weitere Lehmsteine, Sanitärausstattung, Heizkörper, Glastüren, Doppelglasfenster, Akustikpaneele, Leuchten und Pflaster. **Prozess.** Gebäudegeometrie wurde an den Fund angepasst, Fenster im Werk vorbereitet und in den Holzrahmen montiert; Risikoprävention mit technischer Kontrolle. **Datenentscheidung.** Katalog von einer mengenlosen Fensterzeile auf mindestens 300 Stück plus belegte Nebenposten erweitern. Quellen: [SOCOTEC](https://www.socotec.com/media/client-projects/reused-materials-novaedia), [Circular Material Systems](https://circularmaterialsystems.com/en/case/resilience/).

### P40 – Maison Vignette, Auderghem

**Identität.** Privates Reihen-Einfamilienhaus, fertiggestellt 2020; Karbon’ architecture, Pierre Stoffel, 3 ALJ Construct. **Reuse.** 3.000 Large-Boomse-Steen-Ziegel für 36 m² Fassade, 21 m² geschnittene Solvay-Wandfliesen; die Karte nennt zusätzlich 13,5 m² Bodenfliesen und 40 m² Blaustein, die in der aktuellen Opalis-Kurzfassung bildlich, aber nicht vollständig mengenmäßig wiedergegeben sind (`B/O`). FCRBE bilanziert 8.649 kg direkt wiederverwendete Materialien bei 75.970 kg Gesamtmasse. **Datenentscheidung.** 3.000 Stück/36 m² und 21 m² bestätigen; Nebenmengen mit Originalbeleg verknüpft lassen, FCRBE-Masse als Projektbilanz. Quellen: [Opalis](https://opalis.eu/en/node/5639), [FCRBE](https://www.cstb.fr/getmedia/602d356c-10ba-4b8f-b22b-50d7131b41e5/Projet-FCRBE-reemploi-materiaux-construction.pdf).

### P41 – MULTI Brussels

**Identität und Beteiligte.** Transformation des De-Brouckère-Turms, rund 45.000 m²; Whitewood, Conix RDBM, Cordeel, Rotor. **Reuse.** Sechs Lose: rund 1.300 m Aluminium-H-Profile der alten Fassade als Leuchten/Geländer; 82 Blausteinblöcke in situ; Blausteinplatten aus Brügge; Granit aus der Generale de Banque und einem Pariser Büro; Aufzugsmotoren um ein Geschoss versetzt. **Quote und Konflikt.** Das Ziel von 2 % externem Urban-Mining-Material wurde knapp verfehlt; Projektkommunikation nennt rund 89 % nicht neue Materialien einschließlich erhaltenem Tragwerk und etwa 3 % Urban Mining. Die Zahl **543 t** gehört nicht belastbar zu MULTI und darf nicht aus einer FCRBE-Nachbarkarte übernommen werden. **Datenentscheidung.** Bauteilmengen und sechs Lose führen; Erhalt, in-situ Reuse und externe Zuflüsse getrennt bilanzieren. Quellen: [Rotor](https://rotordb.org/en/projects/multi-de-brouckere-tower), [Whitewood](https://www.whitewood.eu/multibrussels), [Rotor Blaustein](https://rotordb.org/en/news/reuse-blue-limestone-multi).

### P42 – Musée de Folklore, Mouscron

**Identität.** Museumsneubau/-erweiterung, V+ und Projectiles, Bauzeit 2011–2018/19; 1.470 m² Erweiterung plus 430 m² Renovierung, Architekturkosten 2,976 Mio. €. **Reuse.** 28.500 gebrauchte Fassadenziegel beziehungsweise 34 m³ von acht lokalen Abbruchgebäuden; die Katalogzahl 30.000 ist eine Rundung. Jeder Herkunftsbereich ist über nummerierte „Kartellziegel“ im Kunstkonzept von Simon Boudvin dokumentiert. **Datenentscheidung.** **28.500 Stück/34 m³** als präzisere Liefermenge übernehmen, acht Spenderobjekte als Provenienzgruppe. Quellen: [Opalis](https://opalis.eu/fr/inspiration/5589), [V+](https://www.vplus.org/folklore-museum), [Greisch](https://www.greisch.com/projet/musee_folklore_mouscron_sb/).

### P43 – Lo-Reninge Town Hall Façade

**Identität.** Öffentlicher Rathausumbau/-anbau von noAarchitecten, fertiggestellt **2011**; Ausführung Verstraetebouw. **Reuse.** 205 m² gelbe gebrauchte Fassadenziegel, geliefert von Joël Devlieghere; traditioneller Kalkmörtel und Kalkschlämme ermöglichen materialverträgliche Fügung und einheitliches Erscheinungsbild. **Datenentscheidung.** Fehlendes Jahr und Menge sind nun belegt; ReUse-Ort „Gebäudehülle“ ergänzen. Quellen: [Opalis](https://opalis.eu/fr/projets/facade-en-briques-de-la-maison-communale-de-lo-reninge), [noAarchitecten](https://noaarchitecten.net/projects/1/046-lo-reninge-town-hall).

### P44 – Institut de Botanique ULiège

**Identität.** Energetische und denkmalgerechte Sanierung des Roger-Bastin-Baus; Abschluss/Publikation 2018/19. **Reuse.** 2.600 m² Barnwood-Fassadenholz aus Osteuropa über Österreich/Belgien; 140 m² Azobé-Dielen ehemaliger niederländischer Docks als Terrasse; vorhandene Metalldachbekleidung und Betonplatten in situ; ein 20 Jahre stillgelegtes Lüftungssystem mit 50.000 m³/h aus vorhandenen Kanälen reaktiviert. **Prozess.** Öffentliche Ausschreibung mit funktionsbezogener, herkunftsoffener Spezifikation; Marktprüfung, Muster und iterative Details. **Datenentscheidung.** Fehlendes Kartenjahr auf 2018 (Bauabschluss) mit Quellenhinweis setzen; Lüftungszahl als Volumenstrom, nicht Bauteilmenge. Quelle: [Opalis](https://opalis.eu/en/node/5628).

### P45 – Chiro d’Itterbeek, Dilbeek

**Identität.** Öffentliches Sanitärgebäude, fertiggestellt **2019**; Gemeinde Dilbeek, Rotor, Coopérative de construction Autrement. **Reuse.** Zwölf Lose Re-Use- und Überschussmaterial; darunter insgesamt acht Urinale, Wand-WCs und Waschbecken. Vollständige Fassade aus gebrauchten Ziegeln; außerdem Keramikfliesen, Tragstruktur und Dachziegel. Weniger als ein Drittel der Masse bestand aus Neuware. **Datenentscheidung.** Jahr, zwölf Lose und acht Sanitärobjekte ergänzen; „<1/3 neu“ entspricht „>2/3 Re-Use/Überschuss“ und ist keine reine Re-Use-Quote. Quellen: [Opalis](https://opalis.eu/fr/projets/reutilisation-de-sanitaire), [Adokin](https://adokin.eu/fr/2020/05/sanitary-block-with-reused-materials-fr/).

### P46 – Verbiest / Karreveld, Brüssel

**Abgrenzung.** Die aktuelle Karte vereint zwei eigenständige Projekte und muss vor Graphimport geteilt werden. **Verbiest, 2020.** Umbau eines über 1.000 m² großen Lagerhauses in Molenbeek; erhaltene Betonstruktur, 44 m² Beton-Terrassenplatten und 90 m² Dachziegel in situ, 11 m² rote Marmorfliesen, 25 m Geländer sowie ca. 20 m Naturstein-/Fliesenelemente aus anderen Projekten. **Karreveld, 2017–2022.** Büroumbau zur Schule: ca. 750 m² modulare Trennwände einschließlich Stahlrahmen, Laminat, Dämmung, Fenster und Türen; ca. 400 m² abgehängte Decken und Leuchten; technische Anlagen und Küche erhalten/angepasst. **Datenentscheidung.** Zwei Projektknoten, getrennte Jahre, Bauherren und Materialflüsse; keine gemeinsame Projektquote. Quelle: [Opalis](https://opalis.eu/nl/projecten/verbiest-en-karreveld-agwa), [As Found](https://dipot.ulb.ac.be/dspace/bitstream/2013/375830/3/AsFound.pdf).

### P47 – Zinneke/Masui4Ever, Brüssel

**Identität und Beteiligte.** Transformation einer ehemaligen Druckerei und dreier Häuser zu Werkstätten, Büros und Kulturflächen; rund 3.000–4.000 m² je nach Arealabgrenzung, fertiggestellt Dezember 2019. Zinneke, Ouest Architecture, Rotor und Matriciel. **Erhalt und Reuse.** 94 % beziehungsweise 8.089 t der ursprünglichen 8.600 t Gebäudemasse blieben erhalten. Von 331 t Materialzuflüssen waren 39 t/12 % wiederverwendet: etwa 30 Stahlträger, fünf Fensterrahmen, 450 m² Steinwolle, zwei Teile einer Stahltreppe, 90 m² Azobé-Terrasse, 300 m² Eichenparkett, ca. 20 Heizkörper, ca. 20 Türen und eine Lüftungsanlage. **Prozess.** 310 Einträge im Reuse-Inventar mit Menge, Zustand, Bildern, Demontage-, Reparatur- und Logistikhinweisen; kooperatives Vergabeverfahren, Projektzimmer sowie eigene Metall-/Holzwerkstätten. **Datenentscheidung.** Fehlendes Jahr auf 2019 setzen; 94 % Bestandserhalt und 12 % Re-Use der Zuflüsse strikt getrennt. Quellen: [Rotor-Bilanz](https://knowledgeplatform.gtb-lab.com/wp-content/uploads/2024/03/20220629-RIH-Rotor_presentation_ROTOR.pdf), [Brüsseler Reuse-Bericht](https://bma.brussels/app/uploads/2024/10/The-architecture-of-reuse-in-Brussels.pdf), [VUB-Fachartikel](https://researchportal.vub.be/en/publications/beyond-innovative-procurement-a-case-study-of-architectural-reuse/), [VAi](https://www.vai.be/gebouwen/cultuurinfrastructuur/zinneke).

### P48 – Alliander HQ, Duiven

**Identität.** Transformation von fünf Bürogebäuden zum energiepositiven Komplex für über 1.500 Beschäftigte, 21.852 m², fertiggestellt 2015; Alliander, RAU und VolkerWessels-Konsortium. **Erhalt/Recycling/Reuse.** Rund 80 % der vorhandenen Materialien blieben erhalten oder wurden wieder-/weiterverwendet: Betonfassaden als Granulat, eigenes Abfallholz als Innenfassade, Arbeitskleidung als Métisse-Dämmung, Sanitärkeramik gereinigt/remontiert, Möbel revitalisiert. Die Dachkonstruktion nutzt rund 30 % weniger Stahl als marktüblich. **Nachweis.** Materialpass, BREEAM-NL Outstanding, energiepositiver Betrieb. Die „80 %“ fasst Erhalt, direktes Re-Use und Recycling zusammen und darf nicht als reine Bauteil-ReUse-Quote importiert werden. **Datenentscheidung.** Sechs Kartenzeilen nach Verfahrensart aufspalten; Projektquote als zusammengesetzte Eigenangabe. Quellen: [Alliander-Berichtsarchiv](https://jaarverslag.alliander.com/downloads), [PBL](https://themasites.pbl.nl/o/circulariteit-in-de-bouw/alliander/), [Circular Future](https://grafisk.3xn.dk/CAC/Building-a-Circular-Future-3-3.pdf).

### P49 – The Green House, Utrecht

**Identität.** Temporäres Gastronomie-/Tagungspavillon für rund 15 Jahre, fertiggestellt 2017/18; Rijksvastgoedbedrijf, cepezed und R Creators. Zwei Geschosse plus 80-m²-Gewächshaus. **Reuse.** Rauchglas-Fassadenplatten der benachbarten Generaal-Knoop-Kaserne bestimmten das Raster; Straßenklinker von einer Kaimauer in Tiel als direkt auf Sand verlegter Boden; gebrauchte Holzelemente in der Geschossdecke. **Reversibilität.** Demontierbarer Stahl-/Holz-Bausatz einschließlich Fertigteilfundamenten, keine Pfahlgründung; Bauteile sollen nach der Standzeit andernorts remontiert werden. **Datenentscheidung.** Jahr auf 2018 präzisieren, falls die Quelle die Eröffnung statt Planungsjahr verwendet; keine Gesamt-ReUse-Masse publiziert. Quellen: [cepezed Projekt](https://www.cepezed.nl/projecten/the-green-house/), [cepezed Materialbericht](https://www.cepezed.nl/actueel/kit-of-parts-circulair/).

### P50 – Resource Rows, Kopenhagen

**Identität.** 92 Mietwohnungen – 29 Reihenhäuser und 63 Apartments –, 9.148 m², fertiggestellt 2019; NREP, Lendager, AG Group und Artelia. **Reuse/Upcycling.** Ganze etwa 3-m²-Mauerwerksfelder wurden aus Zementmörtel-Fassaden von Carlsberg-Bauten, Schulen und Industriebauten ausgesägt und zu neuen Fassadenmodulen montiert. Rund 300 t Holz aus dem Metro-Bau wurden für Fassade/Innenausbau genutzt; Fußböden aus Produktionsverschnitt, Dachgewächshäuser aus gebrauchten Fenstern/Glas. **Wirkung.** Pro erhaltenem Ziegel werden etwa 0,5 kg CO₂e genannt; Projektkommunikation nennt rund 29 % weniger Material-CO₂ pro m², einzelne Produktwerte 38 % für Ziegelwände und 44–88 % für Holz. **Datenentscheidung.** Module, Holz und Verschnitt getrennt führen; „recycled/upcycled“ nicht pauschal als direkte Wiederverwendung kodieren. Quellen: [Lendager](https://lendager.com/project/resource-rows/), [UNFCCC-Fallstudie](https://unfccc.int/sites/default/files/resource/Technical_Paper_2020-Low_Emission_Report.pdf), [NREP LCA/LCC](https://nrep.com/wp-content/uploads/2020/11/200923_Upcycle-Studios-RR-LCALCC_NREP.pdf).

### P51 – Upcycle Studios, Kopenhagen

**Identität.** 20 Reihenhäuser, 3.440–3.909 m² je nach Flächenabgrenzung, fertiggestellt 2018; NREP/AG Gruppen, Lendager, MOE. **Materialien.** 75 % der Fensterverglasungen aus Wohnbauten Nordjütlands; 840, 904, 1.000 oder 1.400 t Metro-Betonabfall werden je nach Quelle und Abgrenzung genannt (`K`). Es handelt sich überwiegend um Betonrecycling als Zuschlag, nicht um wiederverwendete Bauteile. Böden, Wände und Fassaden aus Dinesen-Holzverschnitt. **Wirkung.** NREP-LCA: 32 % weniger materialbezogene und 45 % weniger Lebenszyklus-CO₂ über 50 Jahre; Fensterprodukt 77–96 % weniger, wiederverwendetes Betonelement in einer Vergleichsstudie ca. 95–96 %, Recyclingbeton deutlich geringer. **Datenentscheidung.** Für die Plattform 904 t als dokumentierte NREP-Projektmenge mit Konfliktflag; Verfahren „Recycling/Upcycling“, nicht direktes Re-Use. Quellen: [NREP](https://nrep.no/prosjekt/upcycle-studios/), [Danske Arkitektvirksomheder](https://www.danskeark.com/content/upcycle-studios), [Buildings & Cities](https://journal-buildingscities.org/articles/10.5334/bc.55), [Concrete Quarterly](https://www.concretecentre.com/TCC/media/TCCMediaLibrary/Concrete%20Quarterly%20Archive/2021/CQ-Autumn-2021-digital-edition-V2.pdf).

### P52 – TRÆ High-Rise, Aarhus

**Identität.** 78 m hohes Holz-Hybridhochhaus, 14.850 m², fertiggestellt 2025; Kilden & Hindby/PFA, Lendager, Artelia, Kaj Ove Madsen. **Reuse.** Fassaden aus geborgenen Aluminiumblechen; weitere publizierte Ströme sind Trapezbleche, Metallpaneele, Rotorblattsegmente, Rückbauholz und Holzverschnitt, PET-/Textil-Akustik sowie Ziegel. Für die acht Kartenposten fehlen öffentliche Einzelmengen (`n. p.`). **Wirkung.** Projekt-LCA nennt 30–50 % weniger graue Emissionen als ein konventionelles Betonhochhaus. **Datenentscheidung.** Fertigstellungsstatus bestätigen; alle Mengen leer lassen, bis Produktpässe/Abschlussinventar veröffentlicht sind; Rotorblätter und PET als Funktionsänderung beziehungsweise Recycling kennzeichnen. Quelle: [Lendager](https://lendager.com/project/trae/), [KLH](https://www.klhuk.com/references/trae/).

### P53 – Woongroep Boschgaard, ’s-Hertogenbosch

**Identität.** Bewohnergetragenes Wohn- und Gemeinschaftsprojekt, Bauabschluss Anfang 2024 – nicht 2022; Zayaz, Superuse, Bouwbedrijf Versteegden. **Reuse.** Holzbalken, Fenster, Dämmung, Platten, Sandwichpaneele, Küchen und weitere Bauteile aus Rückbau/Industrie-Restströmen; Karte nennt 85,5 m³ Recyclingbeton. **Quote und Arbeit.** Ziel 90 %, Zwischen-/Abschlussangaben 84–85 % wiederverwendete Materialien; etwa 5.000 ehrenamtliche Baustunden wurden bereits 2022 konservativ geschätzt. **Datenentscheidung.** Fertigstellungsjahr auf 2024 korrigieren; 84 % als geprüftere Abschlussquote, 85 % als gerundete Projektkommunikation; Recyclingbeton getrennt vom direkten Re-Use. Quellen: [Boschgaard](https://boschgaard.nl/wat-is-boschgaard/), [Boschgaard Bau-Rückblick](https://boschgaard.nl/terugblik-op-2022/), [Boschgaard Abschlussquote](https://boschgaard.nl/slim-ontwerp-is-sleutel-voor-succesvol-hergebruik-aluminium-gevel/).

### P54 – Kindergarten Mööslistrasse, Zürich

**Identität.** Umbau von Wohnungen über einem städtischen Werkhof zum Kindergarten, fertiggestellt 2023; erstes Re-Use-Pilotprojekt der Stadt Zürich, Bischof Föhn. **Reuse.** Brandschutztüren, Stahlträger, Außentreppe/Geländer, Pergola, Holzvordach, Küche, Möbel und Sanitärkeramik. Quantifiziert sind unter anderem 3 Re-Use-Holzstützen, 3 Träger/585 kg, 173 kg Geländer, 641 kg Außentreppe, Pergola-Stahlprofile, 3 WC, 3 Waschtische, 1 kleines Waschbecken und 3 Schulwandbrunnen. **Wirkung und Kosten.** Re-Use senkte die projektbezogenen Erstellungsemissionen je nach Amortisationsannahme um ca. 29,1–32,4 % beziehungsweise 13,8 t CO₂e. Drei Schulwandbrunnen kosteten 2.920 CHF statt veranschlagter 10.500 CHF; einzelne Gewerke bis 40 % günstiger. **Datenentscheidung.** Kartenzeilen erweitern; CO₂-Ergebnis mit Methodenversion/Amortisation speichern. Quellen: [Stadt Zürich Baudokumentation](https://www.stadt-zuerich.ch/de/aktuell/publikationen/2025/baudokumentation-kindergarten-moeoeslistrasse.html), [THG-Studie](https://www.stadt-zuerich.ch/content/dam/web/de/aktuell/publikationen/2023/studien-netto-null/einsparung-treibhausgasemissionen-kindergarten-moeslistrasse-studie-v2.pdf), [Kostenstudie](https://www.wirtschaftsfoerderung.stadt-zuerich.ch/content/dam/web/de/aktuell/publikationen/2025/studien-netto-null/kostenauswertung-bauen-re-use-studie.pdf).

### P55 – Brighton Waste House

**Identität.** Dauerhafter Forschungs-/Lehrprototyp der University of Brighton, fertiggestellt 2014; Duncan Baker-Brown/BBM. **Reuse und Abfallströme.** 2.000 Teppichfliesen, ca. 20.000 Zahnbürsten, 2 t Denim, 4.000 VHS-Kassetten, etwa 4.000 DVD-Hüllen, 2.000 Disketten, 10 t Kreideabfall sowie Ziegel, Sperrholz und Bauholz. Bis auf Spezialbekleidung, Dreifachfenster sowie Leitungs-/Sanitärmaterial war nahezu alles gebraucht, überschüssig oder Abfall. **Prozess.** Rund 700 Lernende/Freiwillige leisteten 2.507 Personentage; Bauteile dienen als beobachtbarer Langzeittest. **Datenentscheidung.** Auffällige Kartenmengen sind mehrfach reproduziert; DVD-Hüllen/Disketten ergänzen und zwischen wiederverwendetem Produkt und Dämmfüllstoff aus Abfall unterscheiden. Quellen: [University/Bioregional-Kontext](https://storage.googleapis.com/www.bioregional.com/downloads/Brighton-Hove-annual-review-2013-2014.pdf), [Guardian](https://www.theguardian.com/sustainable-business/2014/sep/05/house-video-cassettes-jeans-toothbrushes-waste-brighton-circular-economy), [Waste House Zusammenfassung](https://en.wikipedia.org/wiki/Waste_House).

### P56 – Hastings Pier Visitor Centre

**Identität.** Wiederaufbau des denkmalgeschützten Piers, Wiedereröffnung 2016, Stirling Prize 2017; Hastings Pier Charity, dRMM, Ramboll. GIA 11.720 m², Auftragswert 14,2 Mio. £. **Reuse.** Begrenzte Menge des beim Brand 2010 erhaltenen Pier-Deckholzes wurde als Bekleidung des neuen CLT-Besucherzentrums und als vor Ort gefertigte Möbel/Bänke verwendet. Die publizierten 136 m³ Holz beziehen sich auf das neue CLT-Tragwerk und dürfen nicht als Re-Use-Menge gelten. **Datenentscheidung.** Katalogspaltenversatz korrigieren; Re-Use-Holzmenge `n. p.`, 136 m³ als neue Tragwerksmenge separat. Quellen: [RIBA Journal](https://www.ribaj.com/buildings/hastings-pier-hastings-and-st-leonards-drmm-architects-riba-awards-2017-south-east/), [Hastings Pier](https://hastingspier.org.uk/about/history-of-the-pier/), [KLH-Datenblatt](https://www.klhuk.com/references/hastings-pier/?mode=inline&pdf=yes&post_id=2004&template_id=8126).

### P57 – Kamikatsu Zero Waste Center

**Identität.** 1.176 m² großes Abfall-, Reuse-, Gemeinschafts- und Hotelzentrum, fertiggestellt März 2020; Kamikatsu, Hiroshi Nakamura & NAP, Yamada Noriaki, Kitajima. **Reuse.** Rund 700 von Einwohnern gespendete Fenster/Türen wurden vermessen, Glasdicke und Reparaturbedarf erfasst und als Doppelfenster-Patchwork eingesetzt. Zusätzlich Keramikscherben im Terrazzo, Möbel/Geräte als Ausstattung und Beschilderung, Erntekisten als Regale, Zeitung als Tapete, ca. 300 Glasflaschen als Leuchte, gebrauchte Ziegel und Textilien. **Konstruktion und Qualität.** Lokale Zedernstämme nur minimal verarbeitet; verschraubte, sichtbare Konstruktion erleichtert Austausch und spätere Demontage. Die Kommune akzeptierte für bereitgestellte Altteile angepasste Gewährleistungs-/Qualitätsregeln. **Datenentscheidung.** 700 Stück bestätigen; Jahr 2020, nicht 2021. Quellen: [Nakamura & NAP](https://www.nakam.info/jp/works/kamikatsu0/), [offizielle WHY-Seite](https://why-kamikatsu.jp/en/pages/why), [Detailinterview](https://www.stirworld.com/see-features-700-donated-windows-and-salvaged-waste-form-the-kamikatsu-zero-waste-center).

### P58 – People’s Pavilion, Eindhoven

**Identität.** 250-m²-Veranstaltungspavillon für neun Tage der Dutch Design Week 2017; bureau SLA, Overtreders W, Arup, New Horizon. **Leihmodell.** 100 % der Bauteile wurden geliehen und danach unbeschädigt zurückgegeben beziehungsweise weiterverwendet; Ausnahme im Kreislaufmodell waren die aus lokalem Kunststoffabfall gefertigten Fassadenfliesen. Quantifiziert sind 12 Beton-Fundamentpfähle, 19 Holzrahmen und 350 Spannbänder; außerdem Stahlmatten, Glasdach, Betonboden, Technik und Kirchenbänke. **Fügung.** Kein Sägen, Bohren, Kleben oder Schrauben; Verspannung mit Gurten/Stahlbändern. **Datenentscheidung.** Sieben Kartenzeilen um vorhandene Stückzahlen und Eigentümer/Lender ergänzen; „Leihgabe“ als eigene Merge-/Provenienzart. Quellen: [bureau SLA](https://bureausla.nl/projects/peoples-pavilion/?lang=en), [New Material Award](https://new-material-award.nl/en/peoples-pavilion-100-borrowed/), [Fallstudienhandbuch](https://greengrowthproject.eu/wp-content/uploads/2023/07/CaseStudies_Final_En.pdf).

### P59 – Pavillon Circulaire, Paris

**Identität.** Temporärer 750-ft²/ca. 70-m²-Pavillon zur COP21, eröffnet Oktober 2015; Pavillon de l’Arsenal, Encore Heureux, Tribu, Bonnefrite, Camping Design; später als Vereinsheim umgesetzt. **Reuse.** 180 Eichentüren aus einem Wohnungsbau der 1930er im 19. Arrondissement als Fassade, gebrauchte Mineralwolle aus Supermarktumbau, überschüssige Kiefernsparren, ausgemusterte Straßenleuchten, Paris-Plages-Gitterroste und 50 Fundstühle. **Quote.** Quellen nennen 60 % oder 80 % wieder-/weiterverwendete Bauteile (`K`, verschiedene Abgrenzungen). **Datenentscheidung.** Einzelmengen priorisieren; keine einheitliche Re-Use-Quote ohne Originalbilanz. Quellen: [Architect Magazine](https://www.architectmagazine.com/technology/architectural-detail/the-reclaimed-circular-pavilion_o/), [Pavillon-Pressemappe](https://www.pavilloncirculaire.com/data/pavillon-re-emploi_f9926/fiche/8873/1_-_dossier_de_presse_-_pavillon_circulaire_3b1b8.pdf), [Encore-Heureux-Projektmaterial](https://www.archilovers.com/projects/171107/circular-pavilion.html).

### P60 – Christ Pavilion, Hannover → Volkenroda

**Identität.** Expo-2000-Pavillon von gmp, 2.004 m² BGF; Bau 1999–2000, vollständige Demontage und Wiedererrichtung im Kloster Volkenroda August 2001. **Reuse.** Gesamtes modulares Ensemble aus beschichtetem Stahl, Sichtbeton, Glas und Marmor wurde in gleicher Ordnung versetzt; neun kreuzförmige Stützen tragen den etwa 18 m hohen Hauptraum. **Datenentscheidung.** 2.004 m² ist Gebäudefläche, keine Materialmenge. Als Gesamtstruktur-Transfer mit zwei Standorten und Remontagedatum modellieren; keine CO₂-/Massenbilanz publiziert. Quelle: [gmp](https://www.gmp.de/en/projects/415/christ-pavilion-expo-2000).

### P61 – Plattenvereinigung, Berlin

**Identität.** Temporäres Forschungs-, Bildungs- und Kulturgebäude, 2010/11 in der Peter-Behrens-Halle und auf Tempelhof; interdisziplinäre Initiative Zukunftsgeräusche/TU Berlin. **Reuse.** Ost- und westdeutsche Betonfertigteile aus Frankfurt/Oder, München und Berlin-Marzahn; erhaltene Treppe aus Frankfurt/Oder, recycelte Bodenplatte und diverse Türen/Fenster. Eine externe Inventartabelle für einen verwandten Re-Use-Pavillon nennt unter anderem 15 Dachbinder, 55 Balken/Stützen, 500 Bretter, 35 Fensterflügel, 11 Türen, 20 Heizkörper und weitere Ausstattung; diese Mengen dürfen ohne eindeutige Objektidentität **nicht** automatisch P61 zugeordnet werden (`K`). **Datenentscheidung.** Herkunftsorte und Verfahren ergänzen, Mengen weiter `n. p.`; nicht mit Plattenpalast oder Bestandsverpflanzung verschmelzen. Quellen: [Plattenvereinigung Abschlussbericht](https://www.plattenvereinigung.de/wp-content/uploads/2023/03/plv_abschlussbericht_web_doppelseiten.pdf), [BauNetz Wissen](https://www.baunetzwissen.de/beton/objekte/sonderbauten/infopavillon-der-initiative-plattenvereinigung-in-berlin-2313713).

### P62 – Plattenpalast, Berlin

**Identität.** TU-Berlin-Forschungsprototyp, zunächst 2004/05 montiert und umgesetzt, 2009 als Galerie eröffnet, 2015 zu „Wohnen im Minimalraum“ umgebaut; Wiewiorra Hopp. Grundfläche 36 m², Höhe 5,76 m. **Reuse.** 13 WBS70-Großplatten aus Berlin-Marzahn sowie 12 Aluminiumrahmen-Glasscheiben aus dem Palast der Republik. Zugeschnittene Platten wurden mit Carbonlamellen verstärkt; Oberfläche wasserabweisend behandelt, Gesamtkonzept demontierbar. **Datenentscheidung.** 13 + 12 bestätigen; Projektjahr als Zeitreihe statt Einzeljahr modellieren. Quellen: [Projektseite](https://wwstudio.de/projects/plattenpalst), [Atlas](https://concrete-reuse.epfl.ch/list?view=grid), [BauNetz Wissen](https://www.baunetzwissen.de/beton/objekte/sonderbauten/plattenpalast-in-berlin-841145/gallery-1/3).

### P63 – SUPERLOCAL Expogebouw, Kerkrade

**Identität.** Demonstrator aus einer 50 Jahre alten Ursulastraat-Hochhauswohnung, eröffnet **22.02.2018**; HEEMwonen, Gemeinde Kerkrade, IBA Parkstad, Maurer United, Volantis, Dusseldorp, Jongen. **Reuse.** Drei als Raumecken ausgesägte Wohnungssegmente von jeweils ca. 40–45 t wurden per 52-m-Kran ausgehoben und im Quartier remontiert; außerdem Aluminiumrohre, Heizkörper, Platten, Fenster, Türen, Geländer, Brüstungen, Küche und Außenbelag. 95 % des Demonstrators bestehen aus Materialien der Spenderwohnung/-anlage. **Prozess.** Bewehrung untersucht, Maße/Zustand nach Ausbau geprüft; Logistikproblem durch spät verfügbare Teile. **Datenentscheidung.** Kartenjahr von 2019 auf 2018 korrigieren; drei Segmente und ca. 120–135 t als Größenordnung mit Quellenbandbreite. Quelle: [SUPERLOCAL](https://www.superlocal.eu/superlocal/expogebouw/), [Projektpublikation](https://www.superlocal.eu/wp-content/uploads/2020/03/SUPERLOCAL-Play-Publicatie.pdf).

### P64 – CascadeUp Glulam Demonstrator, London

**Identität.** UCL-Forschungspavillon, erstmals 2024 gezeigt und 2025 erneut ausgestellt. **Reuse/Remanufacturing.** Rückbauholz, das sonst gehäckselt, verbrannt oder downgecycelt worden wäre, wurde zu geklebtem Sekundärholz (glulamST) für Rahmen sowie kreuzlagigem Sekundärholz (CLST) für Wand-/Bodenpaneele verarbeitet. Produktpässe dokumentieren Bodenpaneel, Wandpaneel und Träger. **Prozess.** Entnageln, Festigkeitssortierung, Zuschnitt/Lamellierung, Klebung und modular lösbare Montage; dies ist Remanufacturing auf höherer Produktebene, nicht direkte Wiederverwendung eines Trägers. Öffentliche Gesamtmasse `n. p.`. **Datenentscheidung.** Zwei Kartenzeilen beibehalten, Verfahrenscode auf Wiederaufarbeitung/Remanufacturing präzisieren. Quellen: [UCL](https://www.ucl.ac.uk/circular-economy-lab/research/reusing-wood-demolition-mass-timber-products), [WCTE-Fachartikel](https://discovery.ucl.ac.uk/10210949/1/Rose%20et%20al_WCTE%202025_full%20paper.pdf).

### P65 – Re:Crete Footbridge, Fribourg/Lausanne

**Identität.** EPFL-Prototyp von 2021; Standortangaben Fribourg beziehungsweise Lausanne beziehen sich auf Spender-/Forschungsort und Präsentation (`K`, im Graph getrennt modellieren). **Reuse.** 25 mit Diamantsäge beziehungsweise Wasserstrahl aus Wänden und Fundamentplatten eines im Umbau befindlichen Wohnbaus geschnittene Stahlbetonblöcke bilden einen 10 m langen nachgespannten Bogen; Transportdistanz rund 90 km. **Entwurf.** Iteratives Zuordnen vorhandener Elemente, Topologie- und Geometrieoptimierung minimierten Verschnitt; keine neue Betontragstruktur gegossen. **Datenentscheidung.** 25 Stück, 10 m und 90 km ergänzen; Kartenort auf tatsächlichen Empfängerstandort prüfen. Quellen: [EPFL Living Archives](https://livingarchives.epfl.ch/projects/4851/recrete-proof-of-concept/), [Atlas](https://concrete-reuse.epfl.ch/list?view=list), [Fachpublikation](https://www.researchgate.net/publication/361932838_ReCrete_-_reuse_of_concrete_elements_in_new_structures_A_footbridge_prototype).

### P66 – Bestandsverpflanzung, München

**Identität.** Studentisches/Diplom-Projekt 2008; drei Bungalows des Olympischen Dorfs wurden an Ratzingerplatz und Lenbachplatz als temporäre Stadtintervention versetzt. **Reuse und Logistik.** 51 Betonfertigteile, zusammen 120 t, wurden innerhalb von zehn Monaten dreimal demontiert und zweimal remontiert, legten insgesamt 80 km zurück und durchquerten zehn Stadtbezirke. Am Ende wurden sie zerstört – der Kreislauf wurde also nicht dauerhaft geschlossen. **Datenentscheidung.** Kartenmenge „3 Bungalows“ bestätigen und um 51 Teile/120 t/80 km ergänzen; End-of-Life „vernichtet“ zwingend dokumentieren. Quellen: [Detail](https://www.detail.de/de_de/bestandsverpflanzung-1092), [Umweltbundesamt](https://www.umweltbundesamt.de/sites/default/files/medien/378/publikationen/texte_93_2015_wiederverwertung_von_bauteilen_0.pdf), [Herito-Interview](https://herito.pl/en/artykul/concrete-reuse/).

### P67 – Montessori Maassluis

**Identität und aktueller Stand.** Neubau einer 1.534-m²-Schule; Kraaijvanger, Monton/Anculus, IMd u. a. Baustart mit erstem Pfahl am 02.07.2026, geplante Übergabe Mai 2027. **Reuse.** Hybridtragwerk aus Holzstützen und wiederverwendeten Hohldielen; Materialien der alten Schule werden zusätzlich im neuen Schulhof eingesetzt. Öffentliche Stück-/Flächen-/Massenangaben für die Hohldielen fehlen (`n. p.`). **Konstruktion.** Flexible große Räume, koppelbare Lernbereiche, für Erweiterung vorbereitetes Dach und reversible/adaptive Planung. **Datenentscheidung.** „Im Bau“ bestätigen; Hohldielen nicht mit den 7.400 m² aus Prinsenhof/CCN verknüpfen, solange der Spender nicht belegt ist. Quellen: [Gemeinde Maassluis](https://www.maassluis.nl/eerste-paal-geslagen-voor-nieuwe-montessorischool-maassluis), [Kraaijvanger](https://www.kraaijvanger.nl/nl/projecten/montessori-maassluis), [Van Miltenburg](https://vanmiltenburg.nl/projecten/nieuwbouw-montessorischool-maassluis).

## Korrekturregister aus der Tiefenprüfung

| ID | Bisher | Vertiefter Befund | Importentscheidung |
|---|---|---|---|
| P03 | ca. 150 t Stahl | 165 t | auf 165 t präzisieren |
| P08 | 9 t als leicht missverständliche Hauptmenge | 9 t Eigenbestand + 15 t Markt = 24 t | zwei Flüsse und Projektsumme führen |
| P17 | 16 t Stahl | Abschlussstand 40 t | 40 t; 16 t als früher Planungsstand |
| P18 | 139 t Stahl | 139 t ist gemeinsames Los für zwei Projekte; für P18 >20 t geplant, Ausführung offen | 139 t entfernen; Status unbestätigt |
| P20 | 22 t Stahl | 89 Profile; Massen-/CO₂-Angaben methodenabhängig | Stückzahl primär, Masse als Frühangabe |
| P24 | ca. 1.000 m² Trapezblech | Quelle belegt 1.000 m² Fassadenfläche, nicht eindeutig reine Blechmenge | Materialmenge offen markieren |
| P30 | Association House mit 17+14+1 | 17+14+1 gehört zum Plauen House; Association House hat 189 Paneele | Karte umbenennen oder Mengen austauschen |
| P31 | 200 Bodenplatten | 200 zugeschnittene Teile aus 110 Ausgangsplatten, 245 m³ | Bauteiltyp und Mengenlogik korrigieren |
| P37 | 27/25 Heizkörper, 18 Türen, 31 Sanitär, 200 m² Dämmung | Abschluss: 29/30 Heizkörper, 50 Türen, 20 Sanitär, 430 m² Dämmung | Abschlussmengen übernehmen |
| P42 | 30.000 Ziegel | 28.500 Stück/34 m³ | Präzisionswert übernehmen |
| P43 | Jahr/Menge offen | 2011; 205 m² Ziegel | ergänzen |
| P44 | Jahr offen | Bau-/Publikationsstand 2018/19 | 2018 mit Datumsart Bauabschluss |
| P45 | Jahr offen | 2019 | ergänzen |
| P46 | ein Vergleichscluster | zwei reale Projekte mit getrennten Flüssen | in zwei Projektknoten teilen |
| P47 | Jahr offen | Dezember 2019 | ergänzen |
| P49 | 2017 | Projektquellen führen Eröffnung 2018 | Datumsart prüfen und vereinheitlichen |
| P51 | Beton ohne Menge | 840–1.400 t je nach Quelle; 904 t in NREP-Bilanz | 904 t mit Konfliktflag, Verfahren Recycling |
| P53 | fertig 2022 | Bauabschluss Anfang 2024; Quote 84–85 % | Jahr 2024, Abschlussquote 84 % |
| P56 | unklare Holzmenge | 136 m³ ist neues CLT, nicht Re-Use-Deckholz | Re-Use-Menge offen lassen |
| P60 | 2.004 m² als Bauteilmenge | 2.004 m² ist BGF | als Gebäudefläche modellieren |
| P63 | 2019 | Eröffnung 22.02.2018 | Jahr 2018 |
| P65 | Lausanne | Brücken-/Spender-/Forschungsorte uneinheitlich benannt | Empfängerstandort vor Import klären |

## Abdeckung und verbleibende Lücken

- Projekte geprüft: **67/67**.
- Projekte mit mindestens einer zusätzlichen Quelle oder einem vertieften Primärbeleg: **67/67**.
- Projektkarten mit einer konkreten Korrektur oder zwingenden Modellierungsentscheidung: **22/67**.
- Kritische Identitäts-/Zuordnungskonflikte: **P18, P30, P31, P46, P65**.
- Vorhaben noch nicht abgeschlossen oder nicht abschließend belegt: **P18, P32, P33, P34, P67**; bei P16 ist der Live-Abschlussstatus separat zu bestätigen.
- Häufigste weiterhin offene Felder: exakte Masse bei Stück-/Flächenangaben, Netto-Mehrkosten der Aufbereitung, einheitliche LCA-Systemgrenze, namentlicher Spenderbau bei marktvermittelten Materialien und Verlustquote zwischen Ausbau und Wiedereinbau.

Für Neo4j sollten Aussagen aus diesem Lauf nicht als synthetische Quellenknoten modelliert werden. Die URLs gehören gemäß Intake-Regel an `primary_source_url`/`source_urls` der Projekt- und Bauteilknoten; Mengen-, Herkunfts-, Prüf- und Wirkungsaussagen erhalten auf den Beziehungen `evidence_url`, eine knappe `evidence_quote`, `evidence_confidence`, `evidence_basis` und den `review_run` dieses Recherchelaufs.
