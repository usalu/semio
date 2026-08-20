# Ring-Review C → A: P01–P67

Stand: 2026-08-20  
Reviewgegenstand: `staging/agent_a_p01_p67.md`  
Vergleichsbasis: aktuelles `projekte.tex`, `references.bib`, verbindliche `BASELINE.md` und die von Agent A angegebenen Direktbelege  
Schreibgrenze: nur diese Reviewdatei; kanonische Dateien, Bibliografie, Makros und Git blieben unverändert.

## 1. Gesamtergebnis

**Reviewausgang: CORRECT.**

- Der mechanische Korpusnachweis von Agent A ist korrekt: **67 Projekte, 230 Ereignisse, 230 Ereigniszeilen mit exakt acht Zellen**.
- Alle **74** in P01–P67 verwendeten Zitationsschlüssel lösen in der vorhandenen Bibliografie auf.
- Es gibt keinen direkten Gegenbeleg für eine Ereignislöschung oder -zusammenführung. **Alle 230 Ereignisse bleiben erhalten.**
- Die 16 fehlenden neuen Funktionen wurden vollständig geprüft. Die von Agent A vorgeschlagenen Werte sind tragfähig.
- Die Prozesskorrekturen P36-E01, P41-E01 und P56-E01 zu `Umnutzung` sind fachlich richtig.
- Alle von Agent A vorgeschlagenen Ereigniskorrekturen werden akzeptiert. Dies schließt die Spender-Änderungen auf `—` ein.
- Vier Projekt-Metadatenmappings müssen gegenüber Agent A korrigiert werden: **P33, P35, P36 und P54**.
- Ring-Review-Zählung relativ zum Agent-A-Paket: **63 ACCEPT, 4 CORRECT, 0 REJECT**.

## 2. Unabhängige mechanische Reproduktion

| Prüfung | Ergebnis |
|---|---:|
| Projektlabels P01–P67 | 67 |
| Ereignisse P01–P67 | 230 |
| Ereigniszeilen mit exakt acht Zellen | 230 |
| Ereigniszeilen mit anderer Zellzahl | 0 |
| eindeutige verwendete Zitationsschlüssel | 74 |
| nicht aufgelöste Zitationsschlüssel | 0 |
| Ereignisse nach Review zu erhalten | 230 |
| Ereignisse zu löschen/verschmelzen | 0 |

Die Zählung wurde direkt aus dem Abschnitt zwischen `proj:01` und `proj:68` im aktuellen `projekte.tex` reproduziert und nicht aus Agent As Summen übernommen.

## 3. Einheitliche Regel für Projektcharakter

### Generalisierte Entscheidung

`Projektcharakter` beschreibt den **physischen Empfänger**, nicht die Forschungs-, Pilot- oder Förderbezeichnung des ReUse-Vorgangs.

- `Prototyp`: Der physische Empfänger selbst wurde als Versuchsbau, Demonstrator oder prototypisches Objekt errichtet.
- `Temporär`: Der physische Empfänger besitzt eine ausdrücklich zeitlich begrenzte Nutzungsdauer.
- `Dauerhaft`: Reguläres, langfristig betriebenes Gebäude oder Infrastrukturwerk; ein darin untersuchter Pilotbauteileinsatz macht das Gesamtprojekt nicht zum Prototyp.

Damit bleibt die Vorrangregel `Prototyp vor Temporär vor Dauerhaft` erhalten, wird aber erst angewendet, nachdem geprüft wurde, ob `Prototyp` wirklich den Empfänger bezeichnet. Die bloße Bezeichnung als Forschungs-, Pilot- oder Pionierprojekt reicht nicht.

### Querschnittliche Anwendung

| Projekt | Agent A | Korrektur | Direkter Grund |
|---|---|---|---|
| P33 Juch-Areal Recyclingzentrum | Prototyp | **Dauerhaft** | Die Stadt Zürich beschreibt ein reguläres neues Recyclingzentrum mit Wertstoffsammelstelle, Betriebsgebäude, Lagerhalle und dauerhaftem Betrieb. `Pilotprojekt` bezeichnet die ReUse-Planung, nicht ein prototypisches Bauwerk. |
| P35 Härmälänranta | Prototyp | **Dauerhaft** | Die 25 Platten wurden im regulären Mietwohnungsbau Härmälänranta Ernst eingesetzt. Der ReCreate-`mini pilot` ist nur der untersuchte Bauteileinsatz. |
| P36 Lokomotion Technology Centre | Prototyp | **Dauerhaft** | Die 27 Platten wurden in zwei regulären Gebäuden eines langfristig betriebenen Industrie-/Technologiezentrums eingesetzt. `mini-pilot` bezeichnet die Forschungseinbettung. |
| P54 Kindergarten Mööslistrasse | Prototyp | **Temporär** | Die offizielle Baudokumentation nennt eine geplante Zwischennutzung von rund 10 bis 15 Jahren. Das ReUse-Pilotprojekt ist kein prototypischer Versuchsbau. |
| P109 TA Housing Block | Dauerhaft | **Dauerhaft** | Konsistenzanker: regulärer Wohnungsbau mit ReCreate-Bauteilen; Forschungsrahmung ändert den Empfängercharakter nicht. |

### Belege

- P33: https://www.stadt-zuerich.ch/de/planen-und-bauen/projekte-und-ausschreibungen/hochbauvorhaben/planung-ausfuehrung/recyclingzentrum-juch-areal.html  
  Beleganker: reguläres Recyclingzentrum aus überdachtem Außenbereich, dreigeschossigem Betriebsgebäude und Lagerhalle; Bauzeit 2026–2027. Die Seite nennt das Vorhaben zugleich `Pionierprojekt`/`Pilotprojekt`, beschreibt aber keinen temporären oder prototypischen Empfänger.
- P35: https://recreate-project.eu/2025/02/13/first-elements-reclaimed-by-the-finnish-cluster-reused-in-a-real-life-mini-pilot/  
  Beleganker: 25 Hohldielen wurden in einem von Skanska für A-Kruunu gebauten Wohnblock verwendet; die Elemente bilden Geschossdecken über dem Schutzraum.
- P35 Fertigstellungsjahr: https://www.a-kruunu.fi/sites/default/files/flipping_books/a-kruunu-vuosikertomus-2025-2/20/ und https://www.a-kruunu.fi/sites/default/files/flipping_books/a-kruunu-vuosikertomus-2025-2/24/  
  Beleganker: Potkurinkatu 5/Härmälänranta wurde im Mai 2025 fertiggestellt; der ReUse-Einbau erfolgte bereits 2024.
- P36: https://recreate-project.eu/2026/02/24/second-reuse-mini-pilot-successful-in-finland/  
  Beleganker: Die Platten liegen in einem selbstständigen Technikgebäude und in Personalräumen innerhalb einer Industriehalle des Lokomotion Technology Centre.
- P36 Zieljahr: https://lokomotion.fi/usein-kysyttya/  
  Beleganker: Erste Bauphase 2024–2027, Fertigstellung im Frühsommer 2027.
- P54: https://www.stadt-zuerich.ch/content/dam/web/de/aktuell/publikationen/2025/baudokumentationen/kindergarten-moeoeslistrasse-baudokumentation.pdf  
  Beleganker: `Zwischennutzung für rund 10 bis 15 Jahre`; Betrieb ab Schuljahr 2023/2024.

### Maschinenlesbare Metadatenabweichungen

Nur diese Felder weichen vom Agent-A-Paket ab:

| ID | Feld | Agent A | Ring-Review C | Sicherheit |
|---|---|---|---|---|
| P33 | Projektcharakter | Prototyp | Dauerhaft | hoch |
| P35 | Projektcharakter | Prototyp | Dauerhaft | hoch |
| P35 | Jahr | 2024 | 2025 | hoch |
| P36 | Projektcharakter | Prototyp | Dauerhaft | hoch |
| P54 | Projektcharakter | Prototyp | Temporär | hoch |

Alle übrigen Metadatenwerte von Agent A bleiben unverändert. Insbesondere:

- P33 bleibt `Gebäude | In Ausführung | Vorgesehen | 2027*`.
- P35 bleibt `Gebäude | Fertiggestellt | Umgesetzt`; nur Charakter und Jahr werden korrigiert.
- P36 bleibt `Gebäude | In Ausführung | Umgesetzt | 2027*`.
- P54 bleibt `Gebäude | Fertiggestellt | Umgesetzt | 2023`.

## 4. Audit der 16 Pflichtkorrekturen `Neue Funktion`

| Ereignis | Agent-A-Wert | Review | Reproduzierter Beleg |
|---|---|---|---|
| P04-E09 | Geländer | ACCEPT | KA13-Bericht: Gitterroste waren Boden einer Technikraum-Mezzanine und wurden als Geländer montiert. |
| P07-E01 | Fassadenbekleidung | ACCEPT | Villa-Welpeloo-Dokumentation: Latten aus Kabeltrommeln bilden die Fassade. |
| P07-E02 | Tragstruktur | ACCEPT | Dokumentation: Stahl einer Textilmaschine bildet die Haupt-/Tragstruktur der Villa. |
| P13-E06 | Innenwand | ACCEPT | LXSY beschreibt das stehende Holzraster aus Tischlereiverschnitt im Zusammenhang der Innenwände und Raumzonierung. |
| P14-E02 | Wanddämmung | ACCEPT | Cityförster: `wall insulations made from old jute bags`. |
| P19-E01 | Stütze | ACCEPT | ASBP: reclaimed tubulars wurden für die langen Stützen/columns gewählt. |
| P21-E01 | Tragstruktur | ACCEPT | Projektseite: Stahl aus der I-93-Infrastruktur bildet das architektonische Tragwerk. |
| P21-E02 | Geschossdecke und Dachplatte | ACCEPT | Washington Post: Betonplatten bilden `floors and roof`. Bauteilkorrektur zu `Betonplatte` ist ebenfalls belegt. |
| P21-E03 | Aussteifung | ACCEPT | Projektseite: `Steel framing and cross-bracing are salvaged from original highway offramp supports.` |
| P25-E04 | Entwässerungsrinne | ACCEPT | Opalis: 38 Betonfertigteile wurden als `channels`/Rinnen eingesetzt. |
| P37-E08 | Tischlerbauteil | ACCEPT | Adokin belegt `bois de menuiserie` aus Transportpaletten. Der generische Wert ist enger als eine unbelegte konkrete Produktannahme. |
| P48-E03 | Fassadenverkleidung | ACCEPT | ArchDaily: `using waste wood for the facades`. |
| P50-E02 | Fassadenverkleidung | ACCEPT | Lendager: Metro-Bauholz wird an der Fassade eingesetzt. |
| P52-E02 | Fassadenbekleidung | ACCEPT | TRÆ-Dokumentation: fehlproduzierte Briefkästen werden Fassadenmaterial. |
| P52-E05 | Bodenbelag | ACCEPT | TRÆ-Dokumentation: ausgesondertes Holz wird zu Bodenbrettern. |
| P55-E07 | Dämmung | ACCEPT | University of Brighton: Sperrholzplatten und Holzverschnitt wurden in Wandhohlräume zur Dämmwirkung eingebracht. |

Ergebnis nach Anwendung: 93 `Umnutzung`-Ereignisse, 93 nichtleere Werte `Neue Funktion`, keine neue Funktion außerhalb von `Umnutzung`.

## 5. Prozessprüfung P36, P41 und P56

| Ereignis | Agent A | Review | Begründung |
|---|---|---|---|
| P36-E01 | Umnutzung; Neue Funktion `Dachplatte` | ACCEPT | Die Hohldielen waren zuvor Zwischendecken und dienen nun als Dach. Das ist ein belegter primärer Funktionswechsel; die Aufarbeitung ist nachrangig. |
| P41-E01 | Umnutzung; Neue Funktion `Terrassenplatte und Innenwandbekleidung` | ACCEPT | Rotor belegt den Wechsel von Fassadenbekleidung zu Terrassenplatten und Innenwandbekleidung. |
| P56-E01 | Umnutzung; Neue Funktion `Fassadenbekleidung` | ACCEPT | dRMM: Das erhaltene Pierdeckholz bekleidet das neue Besucherzentrum. |

Direktbelege:

- P36: https://recreate-project.eu/2026/02/24/second-reuse-mini-pilot-successful-in-finland/
- P41: https://rotordb.org/en/news/reuse-blue-limestone-multi  
  Beleganker: `From 800kg facade slabs to terrace tiles and wall cladding`; 82 Blöcke.
- P56: https://drmmstudio.com/project/hastings-pier/  
  Beleganker: `The new visitor centre is ... clad in the timber decking that survived the 2010 fire.`

P41-E01 bleibt ein bewusst erhaltenes Aggregat. Die vorhandene Systemebene `Hülle $\to$ Außenraum` bildet den Terrassenanteil ab; die zweite Zielroute zur Innenwand bleibt im QA-Text und in `Neue Funktion` sichtbar. Ein Split würde das verbindliche 230/426-Ereignis-Gate verletzen.

## 6. Prüfung P19 Objekttyp und Ereignisnormalisierung

P19 wird als **Infrastruktur** bestätigt. Der Empfänger ist die Einhausung/der Screen einer primären Umspannstation und zugleich ein dauerhaftes öffentliches Kunstwerk; die technische Infrastruktur ist die primäre Objektfunktion.

Die vorgeschlagene Ereigniskorrektur wird vollständig akzeptiert:

`Neue Funktion = Stütze`  
`Spender = Überschussrohre aus Öl- und Gaspipelineprojekten`  
`Herkunftsweg = Lager`  
`Systemebene vorher → nachher = Restposten $\to$ Struktur`  
`Prozess = Umnutzung`

ASBP bezeichnet die Rohre als Material aus abgesagten Öl-/Gasprojekten, in `as new`-Zustand, bezogen über den Lagerhalter Cleveland Steel and Tubes. Die alte Formulierung `alte Öl-Pipelines` behauptet deshalb eine nicht belegte vorherige Nutzung. Beleg: https://asbp.org.uk/case-studies/brent-cross-town-primary-substation

## 7. Spender-Änderungen auf `—`

Alle von Agent A vorgeschlagenen Spender-Änderungen auf `—` werden akzeptiert:

| Ereignis | alter Wert | Review | Grund |
|---|---|---|---|
| P16-E01 | Lagerbestand Cleveland Steel and Tubes | ACCEPT | Cleveland ist Lagerhalter/Lieferant, kein physischer Spender; Ursprungsobjekte der 115 t sind nicht projektscharf dokumentiert. `Herkunftsweg = Lager` bleibt. |
| P28-E01 | Re-Use-Markt / Händler De Roover | ACCEPT | Händler ist kein Spenderobjekt; das Abbruchobjekt der Ziegel ist nicht benannt. |
| P40-E01 | Ziegelhändler Franck | ACCEPT | Händler ist kein Spenderobjekt; konkretes Ursprungsgebäude nicht belegt. |
| P40-E04 | Rotor DC | ACCEPT | Rotor DC ist Vermittler/Händler; das physische Spenderobjekt der Bodenfliesen ist nicht belegt. |
| P54-E02 | kommunales Bauteillager | ACCEPT | Lager bezeichnet den Herkunftsweg. Die aktuell zitierte Projektseite benennt kein eindeutiges Spenderobjekt der Stahlträger. |

Für P54 existieren außerhalb der aktuell zitierten Projektseite Hinweise auf ein nicht näher benanntes `Provisorium außerhalb der Stadt`. Dies reicht nicht für eine eindeutige kanonische Spenderbezeichnung und begründet keine Rückkehr zu einem Lagerhalterwert.

## 8. Audit aller weiteren Agent-A-Korrekturen

| Projekt/Ereignis | Review | Ergebnis |
|---|---|---|
| P08-E02 | ACCEPT | Physische Herkunftsliste statt Lagerhalter; Zielsystem `Struktur`. Das heterogene Aggregat aus gebrauchten Bauteilen und Restposten bleibt ausdrücklich als QA-Konflikt dokumentiert. |
| P18 Metadaten | ACCEPT | `In Ausführung | Vorgesehen | 2026*`; `Live` und `will use 139 tonnes` belegen noch keine Umsetzung. |
| P32 Metadaten | ACCEPT | Baustellenstatus belegt `In Ausführung`; mangels belastbaren Zieljahrs bleibt `—`; Empfängereinbau nicht bestätigt, daher `Vorgesehen`. |
| P33 Phase/Realisierung/Jahr | ACCEPT | `In Ausführung | Vorgesehen | 2027*`; nur Charakter wird in diesem Review korrigiert. |
| P34 Metadaten | ACCEPT | `In Ausführung | Teilweise umgesetzt | 2027*` entspricht dem belegten Einbaustand. |
| P36 Status/Jahr | ACCEPT | Erste Gesamtbauphase endet 2027, Empfänger noch in Ausführung; alle 27 Platten bereits 2025 eingebaut, daher ReUse `Umgesetzt`. |
| P40-E01/E04 | ACCEPT | Vermittler werden aus Spenderfeldern entfernt; Herkunftsweg Lager bleibt. |
| P41-E01 | ACCEPT | Funktionswechsel und Umnutzung direkt belegt; Aggregat bleibt erhalten. |
| P52-E02 | ACCEPT | `fehlproduzierte Briefkästen` und `Restposten $\to$ Hülle` korrigieren die zuvor vermischte wasserbeschädigte Plattencharge. |
| P55-E07 | ACCEPT | Zielsystem `Hülle` statt `Ausbau`; Wandhohlraum-Dämmung direkt belegt. |
| P67 Metadaten | ACCEPT | Baustart 01.07.2026 belegt `In Ausführung`; Hohldielen sind festgelegt, Einbau aber nicht belegt, daher `Vorgesehen`. |

Nicht separat aufgeführte Korrekturfelder aus Agent As Tabelle wurden gegen die gleiche Ereigniszeile und den angegebenen Direktbeleg geprüft und akzeptiert. Es gibt **keine Ereignisabweichung** zwischen Agent A und diesem Review.

## 9. Wiedereröffnung der Korrekturbelege

Die Materialkorrektur-URLs wurden erneut aufgerufen. Ergebnis:

- Direkt reproduzierbar: LXSY/CRCLR, Cityförster/Recyclinghaus, Washington Post/Big Dig, Opalis/Lycée, Adokin/Grande Halle, ArchDaily/Alliander, Lendager/Resource Rows, Aarhus/TRÆ-PDF, University of Brighton/Waste House, dRMM/Hastings Pier, Stadt Zürich/Juch-Areal, ReCreate/Lokomotion und die Statusseiten.
- Die FutureBuilt-KA13-PDF war für den Web-Extraktor zu groß. Derselbe direkte Wortlaut wurde im zugänglichen KA13-Erfahrungsbericht reproduziert: https://insenti.no/wp-content/uploads/2021/01/KA13-Erfaringsrapport-ombruk-20.01.2021.pdf
- Die Villa-Welpeloo-PDF lief beim Direktaufruf in ein Timeout; der indexierte PDF-Text und eine zweite Projektdarstellung reproduzieren `Cable Reels - Facade` und die Textilmaschinen-Stahlstruktur.
- ASBP P19 und Holbein Gardens waren beim ersten Direktaufruf instabil; die vollständigen gecachten Fallstudientexte wurden erneut geöffnet und bestätigen Herkunft, Mengen und Zielstruktur.
- Rotor P41 war im Web-Extraktor instabil; der direkte Seiteninhalt ließ sich erneut laden und enthält den oben zitierten Funktionswechsel.

Technische Abrufprobleme wurden nicht als sachliche Gegenbelege behandelt.

## 10. Genau ein Reviewausgang je Projekt

Die folgende Tabelle bewertet das **Agent-A-Paket**, nicht den noch unkorrigierten kanonischen Stand. `ACCEPT` bedeutet daher, dass die von Agent A verlangten kanonischen Korrekturen übernommen werden sollen. `CORRECT` bedeutet, dass Agent As Paket vor der Integration wie angegeben geändert werden muss.

| ID | Ausgang | Hinweis |
|---|---|---|
| P01 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P02 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P03 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P04 | ACCEPT | Neue Funktion P04-E09 = Geländer bestätigt. |
| P05 | ACCEPT | Mapping und Ereignis bestätigt. |
| P06 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P07 | ACCEPT | Beide neuen Funktionen bestätigt. |
| P08 | ACCEPT | Korrektur des heterogenen Herkunftsaggregats akzeptiert; QA-Hinweis erhalten. |
| P09 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P10 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P11 | ACCEPT | Physischer Pilotbau; `Prototyp` ist gerechtfertigt. |
| P12 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P13 | ACCEPT | Neue Funktion Innenwand bestätigt. |
| P14 | ACCEPT | Empfänger selbst ist experimenteller Prototyp; Wanddämmung bestätigt. |
| P15 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P16 | ACCEPT | Spender auf `—`; Fertigstellung 2026 belegt. |
| P17 | ACCEPT | Mapping und Ereignis bestätigt. |
| P18 | ACCEPT | In Ausführung/Vorgesehen konservativ belegt. |
| P19 | ACCEPT | Infrastruktur sowie Restposten→Struktur und Stützenfunktion bestätigt. |
| P20 | ACCEPT | Mapping und Ereignis bestätigt. |
| P21 | ACCEPT | Physischer Demonstrations-/Pilotbau; drei Funktionswerte bestätigt. |
| P22 | ACCEPT | Mapping und Ereignis bestätigt. |
| P23 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P24 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P25 | ACCEPT | Entwässerungsrinne bestätigt. |
| P26 | ACCEPT | Mapping und Ereignis bestätigt. |
| P27 | ACCEPT | Mapping und Ereignis bestätigt. |
| P28 | ACCEPT | Händler aus Spenderfeld entfernen. |
| P29 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P30 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P31 | ACCEPT | Physischer Pilotbau; `Prototyp` gerechtfertigt. |
| P32 | ACCEPT | In Ausführung/Vorgesehen bei fehlendem Jahr bestätigt. |
| P33 | CORRECT | Projektcharakter muss `Dauerhaft`, nicht `Prototyp`, sein. |
| P34 | ACCEPT | In Ausführung/Teilweise umgesetzt bestätigt. |
| P35 | CORRECT | Projektcharakter `Dauerhaft`; Jahr `2025`. |
| P36 | CORRECT | Projektcharakter `Dauerhaft`; Jahr/Phase/ReUse und P36-E01 sonst wie Agent A. |
| P37 | ACCEPT | Generische neue Funktion Tischlerbauteil ist engste belegbare Angabe. |
| P38 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P39 | ACCEPT | Mapping und Ereignis bestätigt. |
| P40 | ACCEPT | Beide Vermittler aus Spenderfeldern entfernen. |
| P41 | ACCEPT | Funktionswechsel/Umnutzung bestätigt; Aggregat bleibt dokumentiert. |
| P42 | ACCEPT | Mapping und Ereignis bestätigt. |
| P43 | ACCEPT | Geprüftes fehlendes Jahr bleibt. |
| P44 | ACCEPT | Geprüftes fehlendes Jahr bleibt. |
| P45 | ACCEPT | Geprüftes fehlendes Jahr bleibt. |
| P46 | ACCEPT | Mapping und sechs Ereignisse bestätigt. |
| P47 | ACCEPT | Geprüftes fehlendes Jahr und Ereignisse bleiben. |
| P48 | ACCEPT | Fassadenfunktion des Abfallholzes bestätigt. |
| P49 | ACCEPT | Temporärer Empfänger bestätigt. |
| P50 | ACCEPT | Fassadenfunktion bestätigt. |
| P51 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P52 | ACCEPT | Briefkasten-/Restpostenkorrektur und beide Funktionen bestätigt. |
| P53 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P54 | CORRECT | Projektcharakter `Temporär`; Spenderkorrektur P54-E02 bleibt akzeptiert. |
| P55 | ACCEPT | Physischer Forschungs-/Demonstrationsbau; Dämmfunktion und Zielsystem bestätigt. |
| P56 | ACCEPT | Pierdeckholz zu Fassadenbekleidung = Umnutzung. |
| P57 | ACCEPT | Mapping und Ereignisse bestätigt. |
| P58 | ACCEPT | Temporärer Empfänger bestätigt. |
| P59 | ACCEPT | Temporärer Empfänger bestätigt. |
| P60 | ACCEPT | Mapping und Gesamtbauwerk-Ereignis bestätigt. |
| P61 | ACCEPT | Temporärer Empfänger und Ereignisse bestätigt. |
| P62 | ACCEPT | Physischer Prototyp bestätigt. |
| P63 | ACCEPT | Physischer Demonstrationsbau bestätigt. |
| P64 | ACCEPT | Bauteilsystem/Prototyp für Demonstrator bestätigt. |
| P65 | ACCEPT | Infrastruktur/Prototyp für Fußbrücken-Demonstrator bestätigt. |
| P66 | ACCEPT | Physischer Pavillon-Prototyp bestätigt. |
| P67 | ACCEPT | In Ausführung/Vorgesehen/2027* bestätigt. |

Kontrollsumme: **63 ACCEPT + 4 CORRECT + 0 REJECT = 67 Projekte.**

## 11. Integrationsanweisung an den Lead

1. Agent As Ereigniskorrekturen vollständig übernehmen; keine Ereigniszeile löschen, splitten oder verschmelzen.
2. Vor Integration die fünf Metadatenfelder aus Abschnitt 3 ersetzen:
   - P33 Projektcharakter `Dauerhaft`
   - P35 Projektcharakter `Dauerhaft`
   - P35 Jahr `2025`
   - P36 Projektcharakter `Dauerhaft`
   - P54 Projektcharakter `Temporär`
3. Die QA-Hinweise zu P08-E02 und P41-E01 als erhaltene Aggregate beibehalten.
4. Nach Integration erneut bestätigen: 67 Karten, 230 Ereignisse, 93 Umnutzungen, 0 Umnutzungen ohne neue Funktion, 0 neue Funktionen außerhalb von Umnutzung.

Nach diesen Korrekturen ist das Agent-A-Paket für die Lead-Integration freigabefähig.
