% Auto-generiert aus Live-Verifikation des Akteursnetzes (Erhebungsstand: 13.08.2026). Arbeitsdatei – nicht direkt Teil des Berichtstexts, Grundlage für die Bereinigung des Netzes.

# Akteursnetz – Faktencheck (Review)

**Erhebungsstand:** 13.08.2026  ·  **Abdeckung:** 955 Knoten live geprüft (AT, BE, CH, DE, DK, FI, FR, GB, NL, NO, SE)  ·  **Entfernungskandidaten:** 91  ·  **Status-Legende:** offen / übernommen / abgelehnt

## Methode

Jeder gezeichnete Akteur wurde live im Web erneut nachgeschlagen (nicht nur der Datensatz gelesen) und nach drei Graden bewertet:

- **kern** — die Organisation stellt auf einer eigenen, erreichbaren Seite Bauteil-Wiederverwendung dauerhaft als Teil ihres Tuns dar.
- **bezug** — eine öffentliche Seite nennt die Organisation namentlich in einer benannten Reuse-Sache, ohne dass es ihr eigenes Kerngeschäft wäre.
- **ohne_beleg** — das Rechercheverfahren wurde vollständig durchlaufen, keine Quelle gefunden. Das ist eine Aussage über die Recherche, keine über die Organisation.

Gezeichnete Kanten wurden analog in `belegt` / `teilweise_belegt` / `unklar` eingeteilt; reine Verzeichnis-Kopplungen (Opalis, bauteilnetz.de, Cirkla u. Ä.) zählen fix als `unklar`.

**Entfernungskandidaten werden ausschließlich durch feste Regeln aus den Graden berechnet, nie von einer Agentin beurteilt:**

- R1 — als Duplikat geflaggt (Ziel bleibt erhalten)
- R2 — `ohne_beleg` **und** strukturell isoliert (keine gezeichnete Kante, oder jede vorhandene Kante `unklar`)
- R3 — als falsches Land geflaggt **und** das richtige Land ist selbst kein gezeichnetes Panel

`nicht_pruefbar`, `kern`, `bezug` und `defunkt` allein sind nie ein Entfernungsgrund.

## Ergebnis auf einen Blick

| Land | kern | bezug | ohne_beleg | Summe |
|---|---|---|---|---|
| AT (Österreich) | 10 | 21 | 0 | 31 |
| BE (Belgien) | 65 | 45 | 15 | 125 |
| CH (Schweiz) | 49 | 32 | 21 | 102 |
| DE (Deutschland) | 38 | 65 | 10 | 113 |
| DK (Dänemark) | 44 | 22 | 4 | 70 |
| FI (Finnland) | 20 | 21 | 5 | 46 |
| FR (Frankreich) | 85 | 24 | 17 | 126 |
| GB (Vereinigtes Königreich) | 64 | 65 | 4 | 133 |
| NL (Niederlande) | 61 | 57 | 7 | 125 |
| NO (Norwegen) | 22 | 13 | 2 | 37 |
| SE (Schweden) | 31 | 15 | 1 | 47 |
| **Gesamt** | **489** | **380** | **86** | **955** |

Kanten: **406** belegt · **46** teilweise_belegt · **88** unklar (davon Verzeichnis-Kopplungen fix ausgeschlossen).

Zum Vergleich: die vorausgehende Datenlagen-Prüfung hatte 45,6 % der gezeichneten Knoten als quellenlos „by construction“ eingestuft. Die Live-Nachprüfung findet **9.0 % ohne_beleg** — die fehlenden URLs im Export waren überwiegend eine Lücke der Datenlage, kein Beleg für periphere Akteure. Schweden ist der schärfste Einzelfall: 47/47 Knoten kamen ohne gespeicherte URL in die Prüfung und wurden **32× kern, 15× bezug, 0× ohne_beleg** bewertet — kein einziger Entfernungskandidat.

## Gegenprobe: Zitat-Reproduktionsrate

Jeder `kern`-Knoten und jede `belegt`-Kante trägt eine Beleg-URL und ein wörtliches Zitat. Eine zweite, unabhängige Agentin hat jede URL erneut geöffnet und nachgesehen, ob das Zitat wirklich dort steht (triviale Abweichungen wie Umlaut-Umschrift oder Auslassungspunkte zählen als bestätigt).

| Prüfgruppe | geprüft | bestätigt | sinngemäß | **Zitat nicht gefunden** | Seite nicht erreichbar |
|---|---|---|---|---|---|
| kern-Knoten | 493 | 436 | 29 | 4 | 24 |
| belegt-Kanten | 406 | 352 | 54 | 0 | 0 |

**Gemessene Fehlerquote (Zitat nicht reproduzierbar): 4/899 = 0.44 %.**

**Bereits umgesetzt:** alle 4 Einträge wurden auf `ohne_beleg` herabgestuft (`verify_overrides.json`) — die Beleg-URL trägt das zitierte Zitat nicht, dreifach geprüft:

| Land | tid | Name | Beleg-URL | Befund |
|---|---|---|---|---|
| BE | P5 | gjG House | [Quelle](https://www.archdaily.com/951845/gjg-house-blaf-architecten) | Befund: Der Satz 'constructed using re-used bricks, metal, and wood' steht nicht auf der Seite (dreifach ueberprueft, auch mit vollstaendiger Textsuche). Die Seite beschreibt BLAFs allgemeine Forschun |
| FI | M01 | Ekomatti | [Quelle](https://www.ekomatti.fi/rakentaminen.html) | Das angegebene Zitat existiert so nicht als zusammenhaengende Stelle: Kurzform 'alk.' statt 'alkaen' auf der Seite nicht verwendet, und die Begriffe 'pesuhuonekalusteet' sowie 'lautatavaraa' kommen au |
| NO | U10 | Resirqel | [Quelle](https://www.resirqel.no/raadgivning) | Seite (per WebFetch und zusaetzlich per Browser-Tool mit vollem Seitentext geprueft) beschreibt Resirqels Beratungsleistungen, enthaelt aber an keiner Stelle eine Aussage, Resirqel sei Norwegens erste |
| SE | M06 | Kompanjonen (now Dacke Consulting) | [Quelle](https://kompanjonen.se/konsulttjanster/) | kompanjonen.se leitet per 301 auf dackeconsulting.com weiter. Dort taucht weder der Name 'Kompanjonen' noch die zitierte Formulierung 'hjälper företag att köpa, sälja och förvalta återbrukade produkte |

**Seite-nicht-erreichbar-Fälle (24):** jeder wurde manuell nachgeprüft (Retry, Wayback-Snapshot oder unabhängige Zweitquelle) — alle bestätigt, keiner entfernt. Details in `verify_overrides.json`.

## Berechnete Entfernungskandidaten

**91** Knoten erfüllen R1–R3. Nichts davon ist bereits gelöscht — diese Liste ist ein Vorschlag zur manuellen Freigabe (`prune_faktencheck.json`).

Nach Regel: R1 (Duplikat) 6 · R2 (ohne_beleg + isoliert) 84 · R3 (falsches Land, kein gezeichnetes Panel) 1

| Land | tid | Name | Grad | Regel/Begründung | Status |
|---|---|---|---|---|---|
| BE | I04 | Openbare Vlaamse Afvalstoffenmaatschappij (OVAM) | kern | R1 duplicate of 'OVAM (Openbare Vlaamse Afvalstoffenmaatschappij)' (I05) | offen |
| BE | M07 | Bouwmaterialen De Leyn | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | M13 | E.L.S. Garden | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | M19 | Heumatop | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | M21 | Hispantics | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | M26 | Kasseien Goyens | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | M28 | Kassico | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | N04 | Metabolism of Cities | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | U10 | Daidalos Peutz | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| BE | U14 | Détang | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| BE | U30 | Orbix Productions | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | U32 | Roosens Bétons SA | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | U33 | Salvage Architecture | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| BE | U34 | Sixco | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| BE | X02 | Bouwstocks | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| CH | F01 | Circular Construction Lab | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | M12 | PROMAISON | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| CH | O03 | Stiftung PWG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U01 | 2hs | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U02 | AFC Basel | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U03 | AG Landschaftsarch. | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U07 | Balzer Ingenieure AG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U10 | Caretta+Weidmann | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U12 | Ecovative | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U15 | GTI Engineering | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U19 | kaufmann zimmerei | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U20 | KIBAG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U22 | Magna Glaskeramik | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U27 | Nimbus | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U30 | Oxara AG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U34 | Pérez Schmidlin | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U35 | Repoxit AG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U37 | Senn Technology AG | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U41 | USUS Landschaftsarch. | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U43 | Weber Energie+Bauphysik | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| CH | U45 | Zehnder Holz und Bau Winterthur | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | I04 | MLR BW | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | M04 | Brita Marx Fläming Antik | kern | R1 duplicate of 'Bauteilbörse Berlin-Brandenburg (Fläming Antik / Brita Marx)' (M01) | offen |
| DE | M09 | Rombach Bauholz + Abbund GmbH | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | N05 | proHolz BW | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U03 | Architekturbüro Hose | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| DE | U06 | bauteilbörse gronau | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U12 | caspar. | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U16 | Dach & Fachwerk - G. Schneider & J. Depenbrock GbR | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U28 | Hervé Biele / Conclus | bezug | R1 duplicate of 'Architekturbüro Conclus' (U02) | offen |
| DE | U36 | MÖWE Altmaterialverwendung Osnabrück | bezug | R1 duplicate of 'MÖWE gGmbH (Möwe Altmaterialverwendung)' (M08) | offen |
| DE | U42 | Senatsverwaltung für Bildung, Jugend und Familie | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U45 | Tchoban Voss Architekten | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DE | U52 | ZÜBLIN Timber | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DK | G01 | MUDP | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DK | M02 | Bergsten ApS | kern | R1 duplicate of 'Jakobsen Tegl ApS' (M13) | offen |
| DK | M03 | Bregnebjerggaard Grusgrav | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DK | M04 | Brugte Mursten | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| DK | U07 | BOGL | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| FI | M01 | Ekomatti | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FI | U03 | Arkkitehdit LSV Oy | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FI | U04 | Aulis Lundell Oy | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FI | U18 | Rakennustoimisto K. Tervo Oy | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FI | U22 | SSAB | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M02 | Antiques Décoratives | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M03 | Arnaud Démolition | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M05 | Au Vieux Grenier | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M06 | Au Vieux Temps/Monsieur Fabrice Muller | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M12 | Bois et Patines | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M17 | Cheminées Pierres Poteries Matériaux | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M43 | Matériaux Authentiques | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M49 | Petti Matériaux Anciens | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M52 | Portes Antiques et Rééditions | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M54 | Provence Portes Anciennes | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M62 | Rossignol Démolition | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M65 | Serrurerie Ancienne Antiquités | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M66 | SK-Démolition | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M68 | Société Mâconnaise des Cheminées Anciennes | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M71 | The Reclamation Yard | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | M76 | Urbastone | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| FR | N04 | Collectif Bâti Récup' | kern | R1 duplicate of 'Bâti Récup'' (M15) | offen |
| FR | U17 | Terraterre | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| GB | I02 | Southwark Council | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| GB | M05 | Cardiff Reclamation | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| GB | U35 | Howells | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| GB | U38 | iQ Student Accommodation | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| NL | M03 | Brocantiek de Linde | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NL | M20 | Willem Schermerhorn | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NL | P1 | Aa en Maas Office Building | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NL | U25 | EDGE (SXB S.à r.l.) | bezug | R3 wrong country (manual: SXB S.a r.l. is registered in Luxembourg (land_ist=LU); Luxembourg is not a drawn panel) | offen |
| NL | U28 | Fred Stolwijk B.V. | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NL | U47 | Space&Matter | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NL | U52 | TKF / Twente cable factory | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| NL | U59 | Vermaat | ohne_beleg | R2 ohne_beleg + structurally isolated | offen |
| NO | M02 | Grønne Byggevarer (Monter Storkaas) | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |
| SE | M06 | Kompanjonen (now Dacke Consulting) | ohne_beleg | R2 ohne_beleg + all incident edges unklar | offen |

## Bestätigte Datenfehler (bleiben im Netz, aber korrekturbedürftig)

Diese Knoten sind geflaggt, erfüllen aber keine Entfernungsregel — sie bleiben im Netz, sollten aber im Datensatz korrigiert werden.

### Falsches Land

**Drei verschiedene Fälle unter diesem Flag, jeder von Hand gegen seine Begründung geprüft:**

- Vier DE-Fälle sind tatsächlich dänische Organisationen (Roskilde Universität/Kommune, Høje-Taastrup Kommune, Region Hovedstaden), gezeichnet im DE-Panel. Das korrekte Land (DK) ist selbst ein gezeichnetes Panel — keine Entfernung angezeigt, nur Panel-Korrektur.
- Zwei NL-Fälle (BlueCity, Workspot) sind **keine** Landfehler der Akteure, sondern **falsche gespeicherte Quell-URLs**, die zufällig zu gleichnamigen ausländischen Firmen zeigen. Die realen Akteure sind korrekt niederländisch. Bleiben im Netz, unten aufgeführt.
- Ein NL-Fall (EDGE/SXB, tid U25) ist ein **echter** Landfehler: die registrierte Klientin ist luxemburgisch, nicht niederländisch, und Luxemburg ist kein gezeichnetes Panel. `merge_verdicts.py`s allgemeine R3-Regel prüft weiterhin das falsche der beiden Landfelder (`land_soll` statt `land_ist`) und würde bei einer naiven Korrektur auch BlueCity und Workspot fälschlich zur Entfernung vorschlagen — deshalb bleibt die Regel unverändert, und dieser eine Fall wurde von Hand als R3-Kandidat ergänzt. **Steht daher nicht unten, sondern oben unter Entfernungskandidaten.**

| Land (gezeichnet) | tid | Name | Grad | Land laut Flag | Einordnung |
|---|---|---|---|---|---|
| DE | F11 | Roskilde Universitet | bezug | DK | Panel-Fehlzuordnung: Akteur ist real dänisch, gehört ins DK-Panel. (R3 nicht anwendbar — DK ist selbst ein gezeichnetes Panel) |
| DE | I03 | Høje-Taastrup Kommune | bezug | DK | Panel-Fehlzuordnung: Akteur ist real dänisch, gehört ins DK-Panel. (R3 nicht anwendbar — DK ist selbst ein gezeichnetes Panel) |
| DE | I06 | Roskilde Kommune | kern | DK | Panel-Fehlzuordnung: Akteur ist real dänisch, gehört ins DK-Panel. (R3 nicht anwendbar — DK ist selbst ein gezeichnetes Panel) |
| DE | U39 | Region Hovedstaden | bezug | DK | Panel-Fehlzuordnung: Akteur ist real dänisch, gehört ins DK-Panel. (R3 nicht anwendbar — DK ist selbst ein gezeichnetes Panel) |
| NL | O03 | BlueCity / Blue City 010 BV | bezug | CN | Gespeicherte Quell-URL zeigt auf eine gleichnamige chinesische Firma; das reale BlueCity Rotterdam ist korrekt niederländisch. (kein Landfehler des Akteurs) |
| NL | U65 | Workspot | bezug | US | Gespeicherte Quell-URL zeigt auf einen gleichnamigen US-SaaS-Anbieter; das reale Workspot (Bürovermieter Rotterdam) ist korrekt niederländisch. (kein Landfehler des Akteurs) |

### Falscher Typ (Person/Organisation als Projekt oder umgekehrt)

| Land | tid | Name | Grad |
|---|---|---|---|
| AT | U03 | Ferry-Dusika-Stadion Rückbau | bezug |
| BE | P12 | Plateforme Réemploi | bezug |
| BE | S02 | Preuse | kern |
| DE | P1 | Arche Naturhaus | bezug |
| DE | P18 | Saint-Gobain (Germany) | bezug |
| DE | P4 | CIRCOFIN (Circular Construction Finance) | bezug |
| DE | P5 | Consolis DW Systembau | kern |
| DE | P9 | HeidelbergCement | bezug |
| DK | M07 | Carlsberg | kern |
| FI | P1 | Antti Lehto | kern |
| FI | U07 | Havu Järvelä | bezug |
| FI | U09 | Johanna Saarela | bezug |
| FI | U12 | Markus Saarela | bezug |
| FR | P6 | Toulouse Métropole | kern |
| FR | S03 | Plateforme de réemploi des matériaux de voirie de la Ville de Paris | kern |
| GB | U46 | Opera | bezug |
| NL | M01 | Baars & Bloemhoff | bezug |
| NL | U42 | Pieters Bouwtechniek | kern |

### Defunkt (nachweislich eingestellt, Grad bleibt aus historischem Nachweis)

| Land | tid | Name | Grad |
|---|---|---|---|
| BE | P12 | Plateforme Réemploi | bezug |
| DE | F06 | IEMB / TU Berlin | bezug |
| DE | U02 | Architekturbüro Conclus | bezug |
| DE | U04 | bauteilbörse augsburg | bezug |
| DE | U05 | bauteilbörse giessen | bezug |
| DE | U08 | bauteilbörse köln | bezug |
| DE | U09 | bauteilbörse nordhausen | bezug |
| DE | U10 | bauteilbörse oldenburg | bezug |
| DE | U11 | bauteilbörse weißenburg | bezug |
| DE | U23 | gabb-GebrauchtBauMarkt Saarbrücken | bezug |
| DE | U47 | Urselmann Interior | kern |
| DK | P7 | Upcycle Studios Copenhag… | kern |
| DK | U25 | Orbicon | bezug |
| GB | N04 | Hastings & Bexhill Wood Recycling | bezug |
| GB | U09 | Blenheim House | bezug |
| GB | U12 | Cantillon | bezug |
| GB | U37 | IF_DO | bezug |
| NL | U48 | Stiho group | kern |
| NL | U64 | Volantis | bezug |

## Nicht (voll) prüfbar

Zugriffshindernisse sind ein ehrlicher Nicht-Befund, kein Qualitätsmangel — die Organisation kann trotzdem einen Grad tragen (`bezug + nicht_pruefbar` ist gültig).

| Land | tid | Name | Grad |
|---|---|---|---|
| BE | M15 | Fryns-Boret | kern |
| BE | M20 | Heyns Recycling | bezug |
| BE | M36 | Stadshout.be | bezug |
| BE | P10 | Musée de Folklore Vie… | kern |
| BE | U03 | BESP Stoffel & Partners / Pierre Stoffel | bezug |
| BE | U21 | Immobel | bezug |
| BE | U35 | Taktyk | bezug |
| CH | F03 | Empa | bezug |
| CH | O01 | Re:Crete Forschungsteam | bezug |
| CH | P10 | UMAR Unit | kern |
| CH | U11 | Desso / Tarkett | kern |
| CH | U33 | PIRMIN JUNG Schweiz AG | bezug |
| DE | F05 | HTWG Konstanz | bezug |
| DE | M07 | Materialrest24 | bezug |
| DE | P18 | Saint-Gobain (Germany) | bezug |
| DE | P4 | CIRCOFIN (Circular Construction Finance) | bezug |
| DE | P8 | Haus HOS | bezug |
| DE | U07 | bauteilbörse herzogenrath | bezug |
| DE | U24 | Greater Copenhagen Area (CirCoFin pilot) | bezug |
| DE | U38 | Petra Jablonická | bezug |
| DE | U39 | Region Hovedstaden | bezug |
| DE | U44 | Sven Urselmann | bezug |
| DE | U46 | TOMAS | bezug |
| DE | U47 | Urselmann Interior | kern |
| DE | U48 | Werner Sobek | bezug |
| DK | I02 | Gladsaxe Kommune / Gladsaxe Municipality | bezug |
| DK | M12 | Hverringe Centrum for Restaurering | bezug |
| DK | P2 | Resource Rows | kern |
| DK | U01 | 3XN | bezug |
| DK | U04 | Aksel V. Jensen A/S | bezug |
| DK | U11 | GXN | bezug |
| DK | U31 | Skave Nedbrydning | kern |
| FR | M13 | Bourgogne Matériaux Anciens | kern |
| FR | M23 | Enfin!Réemploi | bezug |
| FR | M32 | Labrouche Matériaux Anciens | bezug |
| FR | M44 | Matériaux d'Antan | kern |
| FR | N02 | Bellastock | bezug |
| FR | O02 | Le WIP | bezug |
| FR | P2 | Ferme du Rail Paris | bezug |
| FR | U18 | TRIBU | bezug |
| GB | M04 | Building Spares Market | kern |
| GB | M07 | D.J Giles Brick & Tile Merchant | bezug |
| GB | N04 | Hastings & Bexhill Wood Recycling | bezug |
| GB | P4 | Brighton Waste House | kern |
| GB | S02 | Sustainability Yard | bezug |
| GB | U09 | Blenheim House | bezug |
| GB | U11 | BSRIA | bezug |
| GB | U18 | Contrax Furniture | bezug |
| GB | U33 | Heyne Tillett Steel / HTS | kern |
| GB | U37 | IF_DO | bezug |
| GB | U41 | Mace | bezug |
| GB | U53 | Solus | bezug |
| GB | U64 | Whitby Wood | bezug |
| GB | X01 | Globechain | bezug |
| NL | M11 | Hoogeboom | bezug |
| NL | M16 | Snellen (Rijsbergen) | kern |
| NL | O06 | IBA Parkstad | bezug |
| NL | P14 | The Green House Utrecht | kern |
| NL | S03 | Ter Velde & Den Besten | bezug |
| NL | U02 | ABN AMRO | bezug |
| NL | U27 | Exasun | bezug |
| NL | U34 | Kraaijvanger Architects | bezug |
| NL | U35 | Lagemaat Heerde | bezug |
| NL | U55 | Turntoo / Circularity-Kontext | bezug |
| NL | U61 | Vic Obdam Staalbouw | bezug |
| NO | M05 | OMBYGG | bezug |
| NO | N03 | FutureBuilt | kern |
| NO | U08 | MAD arkitekter / Mad as | bezug |
| SE | F01 | KTH Royal Institute of Technology | bezug |
| SE | F02 | RISE (Research Institutes of Sweden) | bezug |
| SE | M01 | Brattöns Återbruk | kern |

## Abdeckung

**955 von 955 gezeichneten Knoten** live geprüft — alle 11 Panels vollständig, keine offenen Pakete, keine nicht wiederholten Agentenausfälle.


Keine Lücken.
