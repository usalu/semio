# Audit des Projektbestands im Akteursnetz

Stand: 20.08.2026. Read-only-Auswertung des eingebetteten FINAL-DATA-Blocks.

## Ergebnis

- Aktuell gezählt: **148 Projektknoten**.
- Herkunft: **78 bestehende Projekte + 69 behaltene Projekte aus der Erweiterungsprüfung + TradLab TRE = 148**.
- Die Erweiterungsprüfung umfasste 71 Projektfälle: 69 keep, 2 prune; TradLab TRE kam anschließend zusätzlich hinzu.
- Daher wurden nicht nur „einige“, sondern praktisch sämtliche belegten Projektfälle der Erweiterungsrecherche als Knoten übernommen.
- Der Sprung stammt aus der Datenentscheidung, nicht aus dem Force-Graph oder Renderer.

## Gefundene Zählrisiken

1. **Sichere Dublette:** base:CH:P4 K.118 (Kopfbau Halle 118) und base:CH:P5 K.118 Winterthur bezeichnen dasselbe Projekt. Der sichere Gesamtstand ist daher höchstens **147**.
2. **Granularität prüfen:** CRCLR House und Impact Hub Berlin at CRCLR-House sind Gesamtgebäude und Innenausbau desselben Standorts.
3. **Granularität prüfen:** SUPERLOCAL und SUPERLOCAL Expogebouw sind Gesamtvorhaben und Teilprojekt.
4. **Granularität prüfen:** Housing Reuilly – Lot AE und La Caserne de Reuilly können Teilprojekt und Gesamtareal desselben Vorhabens sein.
5. **Projektregel prüfen:** 2 Aldermanbury Square und Nedre Sem Låve sind Spender-/Rückbauprojekte; uptownBasel Building 8 ist noch im Bau.
6. **Isolierte neue Projekte:** Bestandsverpflanzung Pavillon, Cottbus-Sachsendorf/Madlow, Reihenhäuser in Hohenmölsen und Museum des 20. Jahrhunderts besitzen aktuell keine Beziehung im Netz.

## Verteilung

| Land | Bestand | Neu | Gesamt |
|---|---:|---:|---:|
| AT – Österreich | 3 | 2 | 5 |
| BE – Belgien | 9 | 9 | 18 |
| CH – Schweiz | 10 | 11 | 21 |
| DE – Deutschland | 12 | 11 | 23 |
| DK – Dänemark | 5 | 1 | 6 |
| FI – Finnland | 5 | 1 | 6 |
| FR – Frankreich | 5 | 7 | 12 |
| GB – Vereinigtes Königreich | 12 | 6 | 18 |
| NL – Niederlande | 12 | 14 | 26 |
| NO – Norwegen | 2 | 2 | 4 |
| SE – Schweden | 3 | 6 | 9 |

## Bestehende 78 Projekte

| Land | Schlüssel | Projekt | Beziehungen |
|---|---|---|---:|
| AT | base:AT:P1 | enna | 0 |
| AT | base:AT:P2 | Ferry-Dusika-Stadion Rückbau | 7 |
| AT | base:AT:P3 | MedUni Campus Wien | 1 |
| BE | base:BE:P1 | Build Reversible In Conception (B.R.I.C.) | 0 |
| BE | base:BE:P2 | Chiro d’Itterbeek | 2 |
| BE | base:BE:P3 | Circular Retrofit Lab | 0 |
| BE | base:BE:P4 | Maison DnA | 1 |
| BE | base:BE:P5 | Maison Vignette | 0 |
| BE | base:BE:P6 | Musée de Folklore de Mouscron | 1 |
| BE | base:BE:P7 | Recypark Demets | 2 |
| BE | base:BE:P8 | Verbiest + Karreveld | 0 |
| BE | base:BE:P9 | Zinneke | 3 |
| CH | base:CH:P1 | ELEMENTA Walkeweg | 1 |
| CH | base:CH:P2 | ELYS Kultur- und Gewerbezentrum Basel | 1 |
| CH | base:CH:P3 | Grubenstrasse 29 | 4 |
| CH | base:CH:P4 | K.118 (Kopfbau Halle 118) | 0 |
| CH | base:CH:P5 | K.118 Winterthur | 7 |
| CH | base:CH:P6 | Kindergarten Mööslistrasse | 5 |
| CH | base:CH:P7 | LYSP8 Basel | 2 |
| CH | base:CH:P8 | Re:Crete footbridge | 1 |
| CH | base:CH:P9 | Recyclingzentrum Juch-Areal | 4 |
| CH | base:CH:P10 | UMAR Unit | 1 |
| DE | base:DE:P1 | AWM Münster – zirkulärer Büroumbau | 4 |
| DE | base:DE:P2 | Berlin-Schildow Pilot | 0 |
| DE | base:DE:P3 | CRCLR House | 1 |
| DE | base:DE:P4 | Haus HOS | 0 |
| DE | base:DE:P5 | Impact Hub Berlin at CRCLR-House (Innenausbau) | 3 |
| DE | base:DE:P6 | Jugendtreff Ingersheim | 1 |
| DE | base:DE:P7 | Mehrow Pilot House | 0 |
| DE | base:DE:P8 | Plattenpalast Berlin | 1 |
| DE | base:DE:P9 | Plattenvereinigung Berlin | 3 |
| DE | base:DE:P10 | Reallabor B(e) Ware – Gebäudetragwerke aus Sekundärmaterialien Made in Berlin | 0 |
| DE | base:DE:P11 | Recyclinghaus Berlin | 0 |
| DE | base:DE:P12 | Recyclinghaus Hannover | 3 |
| DK | base:DK:P1 | Resource Rows | 1 |
| DK | base:DK:P2 | Svanen | 2 |
| DK | base:DK:P3 | Thoravej 29 Copenhagen | 1 |
| DK | base:DK:P4 | TRÆ High-Rise | 4 |
| DK | base:DK:P5 | Upcycle Studios Copenhagen | 1 |
| FI | base:FI:P1 | Closing Loops (Mustikkamaa pilot project) | 0 |
| FI | base:FI:P2 | Härmälänranta | 4 |
| FI | base:FI:P3 | Lokomotion Technology Center | 4 |
| FI | base:FI:P4 | Melkinlaituri Primary School and Daycare Centre | 4 |
| FI | base:FI:P5 | Rovastinkangas School Expansion | 0 |
| FR | base:FR:P1 | Circular Pavilion Paris | 1 |
| FR | base:FR:P2 | Ferme du Rail Paris | 4 |
| FR | base:FR:P3 | Grande Halle de Colombelles | 1 |
| FR | base:FR:P4 | Maison des Canaux, Paris | 2 |
| FR | base:FR:P5 | Résilience | 5 |
| GB | base:GB:P1 | 55 Great Suffolk Street | 5 |
| GB | base:GB:P2 | BedZED | 1 |
| GB | base:GB:P3 | Brighton Waste House | 1 |
| GB | base:GB:P4 | CascadeUp | 2 |
| GB | base:GB:P5 | Enterprise Centre UEA | 0 |
| GB | base:GB:P6 | Hastings Pier Visitor Centre | 1 |
| GB | base:GB:P7 | Holbein Gardens, London | 3 |
| GB | base:GB:P8 | House of Fraser | 5 |
| GB | base:GB:P9 | PLP Architecture London Studio Circular Fit-out | 2 |
| GB | base:GB:P10 | ReFrame | 0 |
| GB | base:GB:P11 | Roots in the Sky | 1 |
| GB | base:GB:P12 | Timber Square London | 2 |
| NL | base:NL:P1 | BioPartner 5 | 3 |
| NL | base:NL:P2 | BlueCity Offices | 0 |
| NL | base:NL:P3 | Circl | 1 |
| NL | base:NL:P4 | De Ceuvel | 0 |
| NL | base:NL:P5 | Jeugdkliniek Ithaka | 4 |
| NL | base:NL:P6 | Liander | 3 |
| NL | base:NL:P7 | Montessori Maassluis | 2 |
| NL | base:NL:P8 | SUPERLOCAL | 0 |
| NL | base:NL:P9 | SUPERLOCAL Expogebouw | 6 |
| NL | base:NL:P10 | The Green House Utrecht | 4 |
| NL | base:NL:P11 | Villa Welpeloo Enschede | 1 |
| NL | base:NL:P12 | Woongroep Boschgaard | 2 |
| NO | base:NO:P1 | B-camp | 0 |
| NO | base:NO:P2 | KA13 | 6 |
| SE | base:SE:P1 | H22 Pavilion / ReCreate Sweden Pilot | 0 |
| SE | base:SE:P2 | Kv Återbruket, Litteraturgatan/Selma stad, Göteborg | 2 |
| SE | base:SE:P3 | Återhus – att bygga hus av hus | 0 |

## Neu übernommene 70 Projekte

| Land | Schlüssel | Projekt | Status | Beziehungen |
|---|---|---|---|---:|
| AT | proj:123 | Museum des 20. Jahrhunderts | completed | 0 |
| AT | proj:122 | Rieder Campus | completed | 1 |
| BE | proj:86 | BC Materials Production Hall | completed | 2 |
| BE | proj:87 | Cohousing De Schilders | completed | 1 |
| BE | proj:89 | Complexe Tour à Plomb | completed | 2 |
| BE | proj:23 | Europa Building (Résidence Palace) | completed | 2 |
| BE | proj:27 | gjG House | completed | 1 |
| BE | proj:44 | Institut de Botanique ULg | completed | 3 |
| BE | proj:43 | Lo-Reninge Town Hall Façade | completed | 1 |
| BE | proj:88 | Molenbeek-Saint-Jean Town Square | completed | 1 |
| BE | proj:41 | MULTI Brussels | completed | 6 |
| CH | proj:117 | Atelier Jim Dine | completed | 3 |
| CH | proj:120 | Basel Pavillon | completed | 2 |
| CH | proj:121 | FREITAG flagship store | completed | 2 |
| CH | proj:99 | Gebäude Q – Werkstadt Zürich | completed | 3 |
| CH | proj:118 | Hobelwerk Haus D | completed | 2 |
| CH | proj:124 | NEST-Unit Sprint | completed | 4 |
| CH | proj:98 | Provisorium Kantonsschule Uster | completed | 1 |
| CH | proj:125 | Re:bble Tower | completed | 6 |
| CH | proj:100 | Transa Headoffice – Reuse to the max | completed | 2 |
| CH | proj:116 | Unterstand Primeo | completed | 1 |
| CH | proj:126 | uptownBasel Building 8 | under-construction | 5 |
| DE | proj:29 | Association House Gröditz | completed | 1 |
| DE | proj:30 | Association House Plauen | completed | 2 |
| DE | proj:66 | Bestandsverpflanzung Pavillon | completed | 0 |
| DE | proj:12 | Bröthen Twin-House | completed | 2 |
| DE | proj:112 | BUGA Holzpavillon – Reuse 2023 | completed | 1 |
| DE | proj:60 | Christ Pavilion | completed | 1 |
| DE | proj:114 | Cottbus-Sachsendorf/Madlow, Theodor-Storm-Straße | completed | 0 |
| DE | proj:111 | DIGITAL PAVILLON | completed | 2 |
| DE | proj:113 | Mehr.WERT.Pavillon | completed | 3 |
| DE | proj:115 | Reihenhäuser in Hohenmölsen | completed | 0 |
| DE | proj:97 | Sportlerheim Kolkwitz | completed | 3 |
| DK | proj:107 | SMK Thy | completed | 3 |
| FI | proj:109 | TA Housing Block, Tehdaskartanonkatu 34 | under-construction | 4 |
| FR | proj:92 | Garden of Mellinet | under-construction | 2 |
| FR | proj:90 | Housing Reuilly – Lot AE | completed | 2 |
| FR | proj:95 | La Caserne de Reuilly | completed | 2 |
| FR | proj:94 | Promenade Jane-et-Paulette-Nardal | completed | 2 |
| FR | proj:96 | Pulse Offices | completed | 3 |
| FR | proj:91 | Recyclerie et déchèterie de Kaysersberg | completed | 3 |
| FR | proj:93 | Réaménagement de la Place de la Bastille | completed | 2 |
| GB | proj:72 | 2 Aldermanbury Square | completed | 2 |
| GB | proj:69 | 30 Duke Street St James’s | under-construction | 6 |
| GB | proj:68 | 9 Cambridge Avenue | completed | 2 |
| GB | proj:19 | Brent Cross Town Substation | completed | 5 |
| GB | proj:71 | Pontllanfraith Centre for Skills and Learning | completed | 4 |
| GB | proj:70 | SIX St Andrew Street | completed | 3 |
| NL | proj:74 | Bus Terminal Schiphol-Noord | completed | 3 |
| NL | proj:84 | Circulair Paviljoen KAAP | completed | 5 |
| NL | proj:83 | De Avenir – Avignonlaan | completed | 5 |
| NL | proj:78 | De Gouwehal | completed | 4 |
| NL | proj:76 | De HER Rotterdam | completed | 5 |
| NL | proj:79 | Doorlaatpost 90, Schiphol | completed | 4 |
| NL | proj:75 | Grondstoffenstation Afrikaanderplein | completed | 6 |
| NL | proj:81 | Hoge Brug Ulft–Silvolde | completed | 4 |
| NL | proj:85 | Hoogstraat 168–172 | completed | 2 |
| NL | proj:77 | Lamgatsebrug | completed | 4 |
| NL | proj:80 | Parkeergarage Foodspot Utrecht | completed | 2 |
| NL | proj:58 | People’s Pavilion | completed | 4 |
| NL | proj:73 | Techbank Enschede | completed | 4 |
| NL | proj:82 | Verkeersbrug Witte Paarden | completed | 3 |
| NO | proj:108 | Nedre Sem Låve | completed | 5 |
| NO | project:tradlab-tre | TradLab TRE | completed | 1 |
| SE | proj:106 | Borås nya tingsrätt | completed | 3 |
| SE | proj:102 | Droppen | completed | 3 |
| SE | proj:104 | Ekebäckshöjd – Peab-Etappe | completed | 3 |
| SE | proj:105 | Fredriksborgsskolan | completed | 1 |
| SE | proj:101 | Hoppet 2 – Friedländers Gata 20 | completed | 4 |
| SE | proj:103 | Hållbarhetshuset, Haga Norra | completed | 4 |

## Geprüft, aber nicht übernommen

| Schlüssel | Entscheidung | Grund |
|---|---|---|
| proj:32 | prune | Noch nicht fertiggestellt; nur geborgene und gelagerte, nicht eingebaute Bauteile belegt. |
| proj:127 | prune | Baubeginn 2026; Quellen beschreiben nur künftigen Wiedereinsatz. |

## Schlussfolgerung

Die Zahl 148 ist technisch nachvollziehbar, aber noch nicht fachlich sicher. Mindestens die K.118-Dublette muss zusammengeführt werden. Die drei Projekt-/Teilprojekt-Paare sowie die Spender-, Bau- und isolierten Fälle benötigen eine bewusste Darstellungsentscheidung, bevor eine neue Endzahl festgeschrieben wird.

