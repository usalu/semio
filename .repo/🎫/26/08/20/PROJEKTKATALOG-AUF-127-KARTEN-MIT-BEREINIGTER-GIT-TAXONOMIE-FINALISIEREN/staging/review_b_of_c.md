# Ring-Review B → C: P110–P127

Stand: 2026-08-20  
Prüfer: Agent B  
Prüfokus: Agent-C-Paket `agent_c_p110_p127.md` gegen `BASELINE.md`, den aktuellen kanonischen Abschnitt in `projekte.tex`, die vorhandene Bibliografie und erneut geöffnete Beleg-URLs. Kanonische Dateien wurden nicht verändert.

## 1. Gesamturteil

Das Paket ist **mit Korrekturen reproduzierbar**:

- **18 Projekte** P110–P127 geprüft.
- **68 Ereignisse** geprüft und vollständig erhalten; 0 gelöscht, 0 zusammengeführt, 0 ergänzt.
- **54 eindeutige Projekt-Zitationsschlüssel** `reuseS202`–`reuseS255`; alle 54 lösen in der vorhandenen Bibliografie auf.
- Die **28 vorgeschlagenen Feldkorrekturen an 18 Ereignissen** wurden einzeln geprüft: 26 sind unverändert übernehmbar, 2 benötigen einen korrigierten Ortswert.
- Eine Metadatenkorrektur ist zusätzlich erforderlich: P112 `Objekttyp = Gebäude`, nicht `Bauteilsystem`.
- Zehn projektscharfe Quellenvereinigungen des Agent-C-Mappings sind falsch und müssen auf die unveränderte Zuordnung aus `projekte.tex` zurückgeführt werden.
- Ring-Review-Ausgang: **7 ACCEPT, 11 CORRECT, 0 REJECT**.

## 2. Harte Korrekturen am Agent-C-Paket

### 2.1 P112: Objekttyp

**Korrigierter Wert:** `Gebäude`

Die ICD-Projektseite bezeichnet den Pavillon ausdrücklich als `Forschungsgebäude`, `Holzbauwerk` und wiedererrichteten Pavillon. Er bildet ein eigenständiges Bauwerk. Die Baseline reserviert `Bauteilsystem` für Demonstratoren **ohne** eigenständiges Gebäude oder Infrastrukturbauwerk. `Projektcharakter = Prototyp` bleibt richtig.

### 2.2 P118-E02 und P118-E03: Fenster-Spender

**Korrigierter Wert für beide Ereignisse:** `Wohnbau Birsstrasse, Winterthur`

Der BFE-Zwischenbericht ordnet beide Mengen eindeutig derselben Mine zu:

- 48 Holzmetallfenster: `Mine: Birsstrasse, Winterthur`, anschließend `48 Stk. ausgewählt für Wiederverwendung`.
- 12 Holzmetallfenster/Balkontüren: `Mine: Birsstrasse, Winterthur`, anschließend `12 Stk. ausgewählt für Wiederverwendung`.

`Eisenbahnergenossenschaft` ist als Organisation kein zulässiges physisches Spenderobjekt. Der Agent-C-Vorschlag `Wohnbau Birsstrasse, Basel` enthält die falsche Stadt. Die im Bericht genannte Bauteilbörse Basel ist Ausbau-/Logistikakteur, nicht der Spender.

### 2.3 Projektscharfe Quellenvereinigung

Die Schlüssel bleiben gemäß Baseline stabil. Folgende Agent-C-Zuordnungen sind zu korrigieren:

| Projekt | Agent C | Verbindlich aus `projekte.tex` |
|---|---|---|
| P117 | reuseS219, reuseS220, reuseS221 | reuseS219, reuseS220 |
| P118 | reuseS222, reuseS223, reuseS224, reuseS225 | reuseS221, reuseS222, reuseS223, reuseS224, reuseS225 |
| P120 | reuseS230–reuseS234 | reuseS230–reuseS236 |
| P121 | reuseS235–reuseS237 | reuseS237, reuseS238 |
| P122 | reuseS238–reuseS240 | reuseS239, reuseS240, reuseS241 |
| P123 | reuseS241–reuseS245 | reuseS242, reuseS243 |
| P124 | reuseS246–reuseS249 | reuseS244, reuseS245, reuseS246, reuseS247 |
| P125 | reuseS250, reuseS251, reuseS252 | reuseS248, reuseS249 |
| P126 | reuseS253, reuseS254 | reuseS250, reuseS251 |
| P127 | reuseS255 | reuseS252, reuseS253, reuseS254, reuseS255 |

P110–P116 und P119 sind im Agent-C-Mapping bereits richtig zugeordnet. Die Kohorte verwendet weiterhin genau 54 eindeutige Schlüssel.

## 3. Korrigiertes maschinenlesbares Projekt-Mapping

| ID | Stadt | Land | Jahr | Objekttyp | Projektcharakter | Projektphase | ReUse-Realisierung | Quellen |
|---|---|---|---:|---|---|---|---|---|
| P110 | Ingersheim | Deutschland | 2024 | Gebäude | Prototyp | Fertiggestellt | Umgesetzt | reuseS202, reuseS203, reuseS204 |
| P111 | Hamburg | Deutschland | 2021 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS205, reuseS206, reuseS207 |
| P112 | Mannheim | Deutschland | 2023 | Gebäude | Prototyp | Fertiggestellt | Umgesetzt | reuseS208, reuseS209 |
| P113 | Heilbronn | Deutschland | 2019 | Gebäude | Temporär | Fertiggestellt | Umgesetzt | reuseS210, reuseS211 |
| P114 | Cottbus | Deutschland | 2002 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS212, reuseS213, reuseS214 |
| P115 | Hohenmölsen | Deutschland | 2018 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS215, reuseS216 |
| P116 | Münchenstein | Schweiz | 2022 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS217, reuseS218 |
| P117 | St. Gallen | Schweiz | 2023 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS219, reuseS220 |
| P118 | Winterthur | Schweiz | 2023 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS221, reuseS222, reuseS223, reuseS224, reuseS225 |
| P119 | Basel | Schweiz | 2025 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS226, reuseS227, reuseS228, reuseS229 |
| P120 | Münchenstein | Schweiz | 2022 | Gebäude | Prototyp | Fertiggestellt | Umgesetzt | reuseS230, reuseS231, reuseS232, reuseS233, reuseS234, reuseS235, reuseS236 |
| P121 | Zürich | Schweiz | 2006 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS237, reuseS238 |
| P122 | Maishofen | Österreich | 2022 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS239, reuseS240, reuseS241 |
| P123 | Wien | Österreich | 1962 | Gebäude | Dauerhaft | Fertiggestellt | Umgesetzt | reuseS242, reuseS243 |
| P124 | Dübendorf | Schweiz | 2021 | Bauteilsystem | Prototyp | Fertiggestellt | Umgesetzt | reuseS244, reuseS245, reuseS246, reuseS247 |
| P125 | Fribourg | Schweiz | 2025 | Bauteilsystem | Prototyp | Fertiggestellt | Umgesetzt | reuseS248, reuseS249 |
| P126 | Arlesheim | Schweiz | 2027* | Gebäude | Dauerhaft | In Ausführung | Vorgesehen | reuseS250, reuseS251 |
| P127 | Zürich | Schweiz | 2028* | Gebäude | Dauerhaft | In Ausführung | Vorgesehen | reuseS252, reuseS253, reuseS254, reuseS255 |

## 4. Einzelprüfung der 28 vorgeschlagenen Feldkorrekturen

Alle nicht genannten Ereignisfelder bleiben unverändert.

| Ereignis | Feld | Geprüfter Endwert | Review | Evidenzentscheidung |
|---|---|---|---|---|
| P110-E01 | Spender | Fußgängertunnel am Stuttgarter Hauptbahnhof | ACCEPT | Projekt- und Hochschulquelle belegen die Schalung der südlichen Tunnelausgänge. |
| P110-E01 | Herkunftsweg | Rückbau | ACCEPT | Bereits verwendete Schalung wurde ausgebaut und weiterverwendet. |
| P110-E01 | Systemebene vorher → nachher | Infrastruktur → Struktur | ACCEPT | Infrastrukturkontext zu Tragwand. |
| P110-E02 | Spender | Fußgängertunnel am Stuttgarter Hauptbahnhof | ACCEPT | Wie P110-E01. |
| P110-E02 | Herkunftsweg | Rückbau | ACCEPT | Wie P110-E01. |
| P110-E02 | Systemebene vorher → nachher | Infrastruktur → Struktur | ACCEPT | Infrastrukturkontext zu Decke. |
| P111-E01 | Herkunftsweg | Rückbau | ACCEPT | Expo-Pavillon demontiert, aufbereitet und in Hamburg montiert. |
| P111-E02 | Herkunftsweg | Rückbau | ACCEPT | Wie P111-E01. |
| P111-E03 | Herkunftsweg | Rückbau | ACCEPT | Wie P111-E01. |
| P113-E01 | Herkunftsweg | Rückbau | ACCEPT | KIT nennt Stahl aus einem zurückgebauten Kohlekraftwerk. |
| P116-E01 | Spender | Hochregallager | ACCEPT | Primeo nennt rückgebaute Stahlträger aus einem Hochregallager. |
| P116-E01 | Herkunftsweg | Rückbau | ACCEPT | Direkt belegt. |
| P116-E02 | Spender | Kranhalle in Zürich | ACCEPT | Primeo nennt verzinkte Stahlbleche aus einer Kranhalle in Zürich. |
| P116-E02 | Herkunftsweg | Rückbau | ACCEPT | Direkt belegt. |
| P118-E02 | Spender | Wohnbau Birsstrasse, Winterthur | CORRECT | BFE: 12 Einheiten, Mine Birsstrasse in Winterthur; Basel ist nur Sitz der Bauteilbörse. |
| P118-E03 | Spender | Wohnbau Birsstrasse, Winterthur | CORRECT | BFE: 48 Fenster, Mine Birsstrasse in Winterthur. |
| P118-E05 | Systemebene vorher → nachher | Ausbau → Hülle | ACCEPT | Gefängnisbett-Gitterrost wird Balkonbrüstung. |
| P119-E04 | Spender | — | ACCEPT | `Stamm Bau AG` ist kein belegtes Spenderbauwerk; Quelle nennt nur Arlesheim als Herkunftsort. |
| P119-E06 | Spender | Gefängnisbetten aus Zürich | ACCEPT | Baunetz Wissen nennt die Betten als unmittelbare physische Herkunft. |
| P119-E06 | Systemebene vorher → nachher | Ausbau → Hülle | ACCEPT | Bettrost wird Laubengangbrüstung. |
| P119-E07 | Spender | Gefängnisbetten aus Zürich | ACCEPT | Wie P119-E06. |
| P119-E07 | Systemebene vorher → nachher | Ausbau → Hülle | ACCEPT | Bettrost wird Eingangstor. |
| P120-E03 | Spender | — | ACCEPT | OFFCUT ist Materialvermittlung/Lager; ein physisches Spenderobjekt ist nicht belegt. |
| P123-E01 | Neue Funktion | Museum | ACCEPT | Expo-Pavillon wurde für Museumsnutzung adaptiert. |
| P123-E01 | Herkunftsweg | Rückbau | ACCEPT | Transfer Brüssel–Wien setzt Demontage/Wiederaufbau voraus und ist projektgeschichtlich belegt. |
| P123-E01 | Prozess | Umnutzung | ACCEPT | Primäre Funktion wechselt von Expo-Pavillon zu Museum. |
| P124-E04 | Spender | Schulzimmer | ACCEPT | Husner nennt alte Wandtafeln aus Schulzimmern; kein Vermittler wird als Spender eingetragen. |
| P125-E01 | Herkunftsweg | Rückbau | ACCEPT | EPFL: Platten vor dem Abbruch aus Stahlbetongebäude geschnitten. |

Ergebnis der Feldprüfung: **26 ACCEPT, 2 CORRECT, 0 REJECT**. Nach den zwei Ortskorrekturen bleiben es genau **28 Feldkorrekturen an 18 Ereignissen**.

## 5. Sonderfallprüfung und Klassifikationslogik

### P112, P124 und P125: `Bauteilsystem`

- **P112:** `Gebäude`. Eigenständiger wiedererrichteter Holzpavillon; Prototypcharakter ändert den Objekttyp nicht.
- **P124:** `Bauteilsystem`. Die Sprint-Unit ist eine Büro-/Forschungseinheit **innerhalb** des modularen NEST-Gebäudes und damit kein eigenständiges Gebäude.
- **P125:** `Bauteilsystem`. EPFL beschreibt einen zweigeschossigen Forschungsdemonstrator für tragende Wandsysteme, nicht ein eigenständig genutztes Gebäude.

### P120: Prototyp-Vorrang

`Projektcharakter = Prototyp` ist richtig. Die Dokumentation beschreibt den Basel Pavillon sowohl als temporären Veranstaltungsort als auch ausdrücklich als Prototyp; die Baseline setzt `Prototyp` vor `Temporär`.

### P123: Prozess und Projektphase

`P123-E01 = Umnutzung` mit `Neue Funktion = Museum` und `Herkunftsweg = Rückbau` ist zwingend. Die Projektphase des **Empfängerprojekts** bleibt `Fertiggestellt`; `Rückgebaut` beschreibt nicht die aktuelle Phase des Wiener Museums, sondern den Herkunftsweg des Expo-Pavillons.

### P115: Jahr

`Jahr = 2018` ist übernehmbar. Die Projektdokumentation nennt für die Reihenhäuser `Bauzeit RH (Rohbau): I–IV 2018`. Der Beleg ist enger als eine allgemeine Projektfertigstellung und wird deshalb im QA-Nachweis als **Rohbaujahr** gekennzeichnet; er ist dennoch ein dokumentiertes Baujahr und keine Schätzung.

### P125-E01: Prozess

`Angepasster Wiedereinsatz` bleibt richtig. Die Deckenplatten wurden aus dem Spendergebäude geschnitten und im Demonstrator wieder als Platten/Decken eingesetzt. Die EPFL-Quelle beschreibt die Wände aus Betonbruch als separates Konstruktionssystem; daraus darf für das vorhandene Deckenplatten-Ereignis keine `Umnutzung` abgeleitet werden.

### Spender-/Vermittlerregel

- Entfernen: `Eisenbahnergenossenschaft`, `Stamm Bau AG`, `OFFCUT` aus den betroffenen Spenderfeldern.
- Beibehalten/ersetzen durch physischen Ursprung: Birsstrasse-Wohnbau, Hochregallager, Kranhalle, Schulzimmer, Gefängnisbetten.
- Bauteilbörse Basel bleibt Logistik-/Ausbauakteur und wird nicht als Spender eingetragen.

## 6. Ereigniserhalt

| Projekt | Ereignisse | Erhalt |
|---|---:|---:|
| P110 | 2 | 2 |
| P111 | 3 | 3 |
| P112 | 3 | 3 |
| P113 | 1 | 1 |
| P114 | 1 | 1 |
| P115 | 3 | 3 |
| P116 | 2 | 2 |
| P117 | 8 | 8 |
| P118 | 13 | 13 |
| P119 | 13 | 13 |
| P120 | 4 | 4 |
| P121 | 1 | 1 |
| P122 | 2 | 2 |
| P123 | 1 | 1 |
| P124 | 4 | 4 |
| P125 | 1 | 1 |
| P126 | 1 | 1 |
| P127 | 5 | 5 |
| **Summe** | **68** | **68** |

Kein direkter Gegenbeleg rechtfertigt Löschung oder Zusammenführung. Nach den Korrekturen gibt es keine `Umnutzung` ohne `Neue Funktion` und keine `Neue Funktion` außerhalb von `Umnutzung`.

## 7. Wiedergeöffnete Belege und Prüfanker

Die folgenden Fundstellen wurden für die materiellen Korrekturen erneut geöffnet. Die kurzen Textanker dienen der Reproduktion; die Bibliografieschlüssel selbst bleiben unverändert.

| Projekt | URL | Reproduzierter Prüfanker |
|---|---|---|
| P110 | https://www.hft-stuttgart.de/architektur-und-gestaltung/projekte/imiad-international-workshop-2024-reallabor-jugendtreff-ingersheim | Schalung der südlichen Tunnelausgänge; Reinigung, Hobeln/Schleifen und Einbau im Jugendtreff. |
| P110 | https://klingelhoefer-kroetsch.de/projekte/jugendtreff-ingersheim/ | Zwölf Schalungselemente bilden Innenraum; Fertigstellung Oktober 2024. |
| P111 | https://www.dreso.com/de/projekte/details/hammerbrooklyn-digital-campus | Stahltragwerk und Holzfertigteildecken des US-Expo-Pavillons wurden aufbereitet und in Hamburg montiert. |
| P112 | https://www.icd.uni-stuttgart.de/de/projekte/buga-wood-pavilion-reuse-2023/ | `Forschungsgebäude / Prototypen`; Holzbauwerk 2023 in Mannheim wiedererrichtet. |
| P113 | https://www.kit.edu/kit/pi_2019_bundesgartenschau-2019-pavillon-aus-recycling-materialien.php | Tragende Stahlstruktur stammt größtenteils aus zurückgebautem Kohlekraftwerk. |
| P115 | https://www.envirobatgrandest.fr/wp-content/uploads/2022-06-30-mettke.pdf | Hohenmölsen: `Bauzeit RH (Rohbau): I–IV 2018`; 216 gelagerte Betonbauteile. |
| P116 | https://www.primeo-energie.ch/magnolia/dam/jcr%3Aff6cfb75-bf58-4c67-b667-e85358b1a8ec/Primeo%20Energie%20Nachhaltigkeitsbericht%202021%20DE_2.pdf | Halle aus rückgebauten Stahlträgern eines Hochregallagers und Blechen einer Kranhalle in Zürich. |
| P118 | https://www.mehralswohnen.ch/fileadmin/user_upload/20230505_BFE_Skalierbarkeit_Netto-Null_HOB_Zwischenbericht_02.pdf | S. 34–35: 48 Fenster und 12 Fenster/Balkontüren, jeweils Mine Birsstrasse, Winterthur. |
| P119 | https://www.baunetzwissen.de/fassade/objekte/wohnen/wohnhaus-lysp8-in-basel-10095918 | Gitterroste der Brüstungen/Tore stammen von Gefängnisbetten; Ziegelminen Winterthur, Zürich, Arlesheim. |
| P120 | https://architekturwochebasel.ch/2022/wp-content/uploads/AWB_Basel-Pavillon_Dokumentation_Digital.pdf | Temporärer Veranstaltungsort und ausdrücklich Prototyp; OFFCUT als Akteur, nicht belegtes Spenderobjekt. |
| P123 | https://www.belvedere.at/en/belvedere/history-and-architecture | Expo-Pavillon wurde für Museumszwecke adaptiert, in Wien wiederaufgebaut und 1962 eröffnet. |
| P124 | https://www.husner.ch/de/news/news-detail/empa-unit-sprint/ | Zehn Büroräume zwischen bestehenden NEST-Platten; alte Wandtafeln aus Schulzimmern als Innenverkleidung. |
| P125 | https://livingarchives.epfl.ch/projects/8197/rebble-tower-constructive-systems-for-structural-walls-from-reused-concrete-rubble/ | Zweigeschossiger Demonstrator; Platten vor Abbruch aus Stahlbetongebäude geschnitten. |
| P125 | https://www.epfl.ch/labs/sxl/research/rubble-reuse/ | Re:bble Tower, Fribourg; Fertigstellung 07.08.2025; Demonstratoren-Kontext. |
| P126 | https://www.schnetzerpuskas.com/en/projects/3567-uptownbasel-building-8 | Realisierung 2024–2027, Status `Under construction`; Stahl der Panzerhalle vor Ort demontiert und gelagert. |
| P127 | https://werkstadt-zuerich.ch/gebaeude-x/ | Baustart Mitte 2026, Fertigstellung Anfang 2028; Schienen als Stützen/Träger und Wagenfenster als Fassade. |

## 8. Verbleibende QA-Hinweise

- **P111:** Drees & Sommer nennt einen Projektzeitraum bis Mai 2022. Das vorhandene Jahr 2021 wird dadurch nicht direkt widerlegt; kein Jahreswechsel ohne eindeutigen Gegenbeleg.
- **P115:** 2018 ist direkt als Rohbauzeit belegt, nicht explizit als Schlussabnahme. Diese Einschränkung bleibt im QA-Nachweis sichtbar.
- **P118:** Die bisherige Unsicherheit ist aufgelöst. Beide Mengengruppen stammen laut BFE aus Birsstrasse, Winterthur.
- **P126/P127:** `Vorgesehen` bleibt richtig, solange der konkrete Einbau nicht als umgesetzt belegt ist. Spenderseitige Demontage/Lagerung allein ändert die ReUse-Realisierung nicht.
- Keine offene Frage verlangt eine Ereignislöschung oder einen neuen Bibliografieschlüssel.

## 9. Projektscharfer Ring-Review-Ausgang

`ACCEPT` bedeutet: Das Agent-C-Paket ist für dieses Projekt fachlich unverändert übernehmbar. `CORRECT` bedeutet: Die unten bezeichnete Korrektur muss vor der Integration angewendet werden.

| Projekt | Ausgang | Begründung |
|---|---|---|
| P110 | ACCEPT | Metadaten und sechs Ereignisfeldkorrekturen reproduziert. |
| P111 | ACCEPT | Drei Rückbauwerte reproduziert; Jahr 2021 bleibt mangels Gegenbeleg. |
| P112 | CORRECT | Objekttyp auf `Gebäude` setzen; Prototyp bleibt. |
| P113 | ACCEPT | Temporäres Gebäude und Rückbauherkunft belegt. |
| P114 | ACCEPT | Metadaten, Ereignis und Quellenmapping konsistent. |
| P115 | ACCEPT | Jahr 2018 als dokumentiertes Rohbaujahr übernehmbar. |
| P116 | ACCEPT | Hochregallager/Kranhalle und beide Rückbauwerte reproduziert. |
| P117 | CORRECT | Quellenvereinigung auf `reuseS219, reuseS220` zurückführen. |
| P118 | CORRECT | Beide Fenster-Spender auf Birsstrasse, Winterthur korrigieren; `reuseS221` wieder zuordnen. |
| P119 | ACCEPT | Vermittlerregel, Gitterrost-Herkunft, Systemwechsel und Quellenmapping konsistent. |
| P120 | CORRECT | Inhaltliche Korrektur akzeptiert; Quellenvereinigung muss `reuseS230`–`reuseS236` enthalten. |
| P121 | CORRECT | Quellenvereinigung auf `reuseS237, reuseS238` korrigieren. |
| P122 | CORRECT | Quellenvereinigung auf `reuseS239, reuseS240, reuseS241` korrigieren. |
| P123 | CORRECT | Umnutzung/Rückbau akzeptiert; Quellenvereinigung auf `reuseS242, reuseS243` korrigieren. |
| P124 | CORRECT | `Bauteilsystem`/Prototyp und Schulzimmer akzeptiert; Quellenvereinigung auf `reuseS244`–`reuseS247` korrigieren. |
| P125 | CORRECT | `Bauteilsystem`/Prototyp und Rückbau akzeptiert; Quellenvereinigung auf `reuseS248, reuseS249` korrigieren. |
| P126 | CORRECT | Statuslogik akzeptiert; Quellenvereinigung auf `reuseS250, reuseS251` korrigieren. |
| P127 | CORRECT | Statuslogik und fünf Ereignisse akzeptiert; Quellenvereinigung auf `reuseS252`–`reuseS255` korrigieren. |

## 10. Abschlusszahlen

| Kennzahl | Ergebnis |
|---|---:|
| Projekte geprüft | 18 |
| Ereignisse geprüft | 68 |
| Ereignisse erhalten | 68 |
| Ereignisse gelöscht / zusammengeführt / neu | 0 / 0 / 0 |
| Vorhandene Projektquellen | 54 |
| Fehlende Bibliografieschlüssel | 0 |
| Agent-C-Feldkorrekturen geprüft | 28 |
| Feldkorrekturen unverändert akzeptiert | 26 |
| Feldkorrekturen mit korrigiertem Wert | 2 |
| ACCEPT-Projekte | 7 |
| CORRECT-Projekte | 11 |
| REJECT-Projekte | 0 |
| Umnutzung ohne neue Funktion nach Korrektur | 0 |
| Neue Funktion außerhalb Umnutzung nach Korrektur | 0 |
| Ungeklärte harte Konflikte | 0 |

**Gesamturteil: CORRECT.** Nach Anwendung der dokumentierten Metadaten-, Spenderorts- und Quellenmapping-Korrekturen ist das Agent-C-Paket mit dem beschlossenen Acht-Werte-/Acht-Spalten-Vertrag vereinbar und bewahrt alle 68 Ereignisse.
