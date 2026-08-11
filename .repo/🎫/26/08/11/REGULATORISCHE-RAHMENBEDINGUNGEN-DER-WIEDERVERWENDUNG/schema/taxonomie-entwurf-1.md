# Taxonomie-Entwurf 1/2 — Verfeinerung des Sieben-Achsen-Schemas

Grundlage: Schema-Stresstest-Abschnitte der drei W0-Piloten `pilot-de-produkt.md` (12 Regelungsobjekte, Feld 1), `pilot-de-abfall.md` (7 Regelungsobjekte, Feld 3/4) und `pilot-de-zie.md` (10 Regelungsobjekte, Feld 2). Insgesamt 29 primärquellenbasiert erhobene Regelungsobjekte, die das Schema an 23 benannten Einzelstellen an eine Grenze gebracht haben. Ziel dieses Entwurfs: das kontrollierte Vokabular je Achse verfeinern, Abgrenzungsregeln formulieren, Mehrfachwert- und Sonderfall-Handhabung klären — **ohne** die Grundstruktur (sieben Achsen, ID-Schema, Evidenzgrade, Beleg-Quellen-Stufen) zu verändern.

Konvention dieses Dokuments: Jeder Vorschlag nennt (1) den Befund mit Beleg-Regelungsobjekt(en), (2) den vorgeschlagenen Vokabularwert/-mechanismus, (3) eine Abgrenzungsregel nach dem Muster „A; B zählt nicht", (4) den Umgang mit Mehrfachwerten/Sonderfällen, soweit einschlägig.

---

## 1. Achse A — Jurisdiktion/Ebene

Der Dreiwertigkeit `EU/EEA | national | sub-national` fehlen zwei Zwischenlagen, die in den Piloten wiederholt zu Verlegenheitscodierungen zwangen.

### 1.1 Vorschlag: vierter Wert „international (nicht-EU/EEA)"
- **Befund:** ISO 13822 (REG-DE-2-006) ist genuin ISO-Ebene — weder EU/EEA noch national erarbeitet. Der erzwungene Wert „national" (weil erst über DIN-Übernahme in Deutschland wirksam) verwischt, dass der materielle Normtext auf einer dritten, im Schema nicht vorgesehenen Ebene entsteht.
- **Abgrenzungsregel:** International (nicht-EU/EEA) = Normtext wird von einem Gremium ohne EU/EEA-Mandat erarbeitet (ISO, IEC, bilaterale Abkommen wie CH-MRA); **national zählt nicht**, wenn nur die *Übernahme* (DIN-Kennzeichnung, Bindungsakt) national erfolgt, der materielle Text aber unverändert von der internationalen Ebene stammt.
- **Mechanismus, kein neuer Wert nötig zusätzlich dazu:** Eurocodes (REG-DE-2-007) zeigen, dass bei CEN-Normen *Erarbeitungsebene* (EU/EEA über CEN) und *Bindungsebene* (national über Nationalen Anhang + VV TB) systematisch auseinanderfallen — hier passt A=EU/EEA für die Erarbeitung, aber die Bindungswirkung ist rein national bedingt. Empfehlung: **A kodiert die Bindungsebene** (wo entfaltet die Regelung Wirkung), nicht die Erarbeitungsebene; wo beide auseinanderfallen (Eurocodes, ISO-Übernahmen, IVHB-artige Konstrukte), ist dies im Freitext „Kernaussage" explizit zu vermerken, nicht in A selbst zu kodieren. A bleibt damit dreiwertig plus dem neuen vierten Wert, aber die *Semantik* von A wird auf „Bindungsebene" präzisiert.

### 1.2 Vorschlag: Vermerk-Konvention „ARGEBAU/Bund-Länder-Gremium" statt neuer A-Wert
- **Befund:** MBO und die ARGEBAU-Hinweise Standsicherheit (REG-DE-2-009) sind beide 16-Länder-Koordinationsprodukte ohne eigene Rechtskraft, wurden im Pilot aber unterschiedlich kodiert (MBO → sub-national, weil wortgleicher Landesnachvollzug bekannt; ARGEBAU-Hinweise → national, weil kein flächendeckender Landesnachvollzug verifiziert). Das ist eine Projektkonvention, keine Schema-Kategorie, und erzeugt bei unterschiedlichen Bearbeitern voraussichtlich Inkonsistenzen.
- **Vorschlag:** Kein fünfter A-Wert (Gefahr der Kategorien-Inflation), sondern eine **Pflichtnotiz „Downstream-Verifikationsstatus"** im Feld Sub-Ebene, wenn ein Dokument ARGEBAU-/Muster-Charakter hat: entweder „Landesnachvollzug stichprobenartig bestätigt [Länder]" oder „Landesnachvollzug nicht verifiziert". Die A-Kodierung selbst folgt einer festen Regel: **A=sub-national, wenn und nur wenn** für mindestens eine Stichprobe ein wortgleicher oder inhaltsgleicher Landesrechtsakt nachgewiesen ist (wie bei MBO §16a/§20, s. REG-DE-2-001); **A=national, wenn** kein Landesnachvollzug geprüft oder das Dokument als bundesweite Fachempfehlung ohne Rechtsform firmiert (wie ARGEBAU-Hinweise). „National zählt nicht" für ein Muster, dessen materielle Wirkung ausschließlich über 16 Einzelakte entsteht und für das mindestens eine Stichprobe diese Übernahme belegt — dann sub-national trotz bundesweiter Erarbeitung.

### 1.3 Vorschlag: Kapazitätshinweis zur Sub-Ebene-Pflicht (kein Vokabular-, sondern Prozessvorschlag)
- **Befund:** Die Sub-Ebene-Pflicht bei A=sub-national scheiterte im Pilot an technischen Zugriffshürden (NBauO nur in veralteter 2012er-Fassung auffindbar, REG-DE-1-012); das ist kein Schemafehler, sondern ein Beschaffungsproblem, das für W2 (mehrere sub-national-Jurisdiktionen: DE, CH, BE, US) einzuplanen ist.
- **Vorschlag:** Keine Schemaänderung; Aufnahme einer Beschaffungs-Eskalationsregel in die Prozessdokumentation (nicht Gegenstand dieses Taxonomie-Entwurfs, an W2-Planung weiterzugeben): bei Zugriffsfehlern auf Landesportale zunächst Wolters-Kluwer-Alternative, dann Landesjustizportal, dann Zweitquelle (Kommentarliteratur mit Stand-Vermerk) probieren, mit Zeitbudget-Deckel pro Land.

---

## 2. Achse B — Regelungsfeld

### 2.1 Vorschlag: B als Primärfeld + Nebenfelder (Mehrfachauswahl mit Rangordnung)
- **Befund:** MVV TB berührt in einem Dokument Feld 1 (Ü-Zeichen-Bezug), Feld 2 (Verwendbarkeitsnachweise), Feld 4 (Brandschutz/Schallschutz) und Feld 6 (Normbezugnahme) gleichzeitig; das DIBt-Verwendbarkeitsnachweis-System (REG-DE-2-009 im Produkt-Piloten) liegt fast beliebig zwischen Feld 1 und Feld 2. Eine erzwungene Einfachauswahl führt zu bearbeiterabhängiger Willkür.
- **Vorschlag:** B wird **Primärfeld (Pflicht, ein Wert) + Nebenfelder (optional, mehrere Werte)**. Primärfeld = das Feld, dessen Pflichten/Rechtsfolgen im konkret zitierten Fundstellen-Ausschnitt (nicht im Gesamtdokument) ausgelöst werden. Nebenfelder = Felder, die im selben Dokument, aber nicht im zitierten Absatz geregelt werden.
- **Abgrenzungsregel:** Primärfeld richtet sich nach der Fundstelle des Regelungsobjekts, nicht nach dem Gesamtdokument — d. h. ein einzelnes Regelungsobjekt zu MVV TB Kap. X (Brandschutz) bekommt Primärfeld 4, auch wenn dasselbe Gesamtdokument an anderer Stelle Feld 1 regelt (dort separates Regelungsobjekt mit Primärfeld 1). **Nebenfelder zählen nicht** als Klassifikationskriterium für Filterung/Auswertung, nur als Kontext — die Konflikttabelle (Anlage RG) gruppiert ausschließlich nach Primärfeld.

### 2.2 Vorschlag: optionales Flag „Grundnorm/Begriffsnorm"
- **Befund:** KrWG § 3 (REG-DE-3-001) bestimmt die Anwendbarkeit aller anderen Feld-3-Normen (Gatekeeper-Funktion: legt fest, ob ein Bauteil überhaupt Abfallrecht unterliegt), wurde aber wie eine gewöhnliche operative Norm (z. B. § 8 GewAbfV) klassifiziert. Für die Konflikttabelle ist das folgenreich, weil Grundnormen typischerweise nicht mit operativen Normen „kollidieren", sondern deren Anwendungsbereich determinieren.
- **Vorschlag:** Kein neuer B-Wert, sondern ein binäres Zusatzflag `Grundnorm: ja/nein` neben B. **Abgrenzungsregel:** Grundnorm = definiert einen Tatbestand, von dessen Erfüllung/Nichterfüllung die Anwendbarkeit anderer Regelungsobjekte im selben Feld abhängt (Beispiele: KrWG §3 Abfallbegriff, CPR Art. 2/3 Anwendungsbereich); **operative Norm zählt nicht** als Grundnorm, auch wenn sie selbst Voraussetzungen enthält, solange diese Voraussetzungen nicht die Anwendbarkeit *anderer* Regelungsobjekte steuern, sondern nur die eigene Rechtsfolge auslösen.

### 2.3 Zurückgewiesener Vorschlag (zur Transparenz dokumentiert)
Ein initial erwogener B-Wert „Institutionelles Ausführungsrecht" (für BauPG, das rein organisatorisch ist und materiell auf die EU-VO verweist) wird **nicht** empfohlen: BauPG lässt sich sauber als B=1 mit explizitem Kernaussage-Vermerk „reines Ausführungsrecht ohne materielle Reuse-Norm" führen (wie im Pilot REG-DE-1-008 bereits praktiziert). Ein eigener B-Wert würde die Sieben-Feld-Grundstruktur aufweichen, ohne einen Auswertungsgewinn zu bringen, der das Primär-/Nebenfeld-Konzept (2.1) nicht bereits abdeckt.

---

## 3. Achse C — Materialfamilie

### 3.1 Vorschlag: neunter Wert „Verbund-/Systembauteil (mehrere Materialfamilien in einer Funktionseinheit)"
- **Befund:** Ein wiederzuverwendendes Fenster (Glas + Rahmenmaterial + Beschlag + Dichtung) oder eine Tür ist per Definition ein Verbund und lässt sich keiner der acht Einzelkategorien zuordnen — obwohl gerade die Wiederverwendung *ganzer, zusammengesetzter* Bauteile der Kern des Forschungsgegenstands ist. „Materialübergreifend" (der bestehende neunte Wert) meint im Schema bislang *horizontales Recht, das kein Material bevorzugt* (z. B. CPR, KrWG) — nicht *ein einzelnes physisches Objekt aus mehreren Materialien*. Beide Bedeutungen unter demselben Label zu führen verschluckt einen zentralen Befund (Bauteil-Abfallrecht denkt in Werkstoffströmen, nicht in Bauteilen — GewAbfV § 8 kennt zehn Werkstofffraktionen, aber keine Bauteilkategorie).
- **Vorschlag:** „materialübergreifend" wird in zwei Werte aufgespalten:
  - **„materialübergreifend-horizontal"**: die Regelung selbst differenziert nicht nach Material (z. B. CPR, KrWG-Grundnormen, MBO-Verfahrensnormen).
  - **„Verbund-/Systembauteil"**: der geregelte/betroffene Gegenstand ist ein aus mehreren Materialfamilien zusammengesetztes Einzelbauteil, unabhängig davon, ob die Regelung selbst materialspezifisch differenziert oder nicht.
- **Abgrenzungsregel:** „Verbund-/Systembauteil" **zählt nicht**, wenn die Regelung sich auf einen einzelnen Werkstoff im Bauteil bezieht (dann C = die jeweilige Einzelmaterialfamilie, ggf. als Mehrfachwert, s. 3.2), und **materialübergreifend-horizontal zählt nicht**, wenn erkennbar ist, dass der Regelungsgegenstand faktisch nur bei zusammengesetzten Bauteilen auftritt (Fenster-, Tür-, Fassadenrecht).

### 3.2 Vorschlag: Mehrfachwert-Handhabung für C bei materialspezifischen Anhängen
- **Befund:** REG-DE-2-010 (BW-Leitfaden) hat einen materialneutralen Hauptteil, aber materialspezifische Anhänge A (Stahlbau) und B (Holzbau) — im Pilot als „Baustahl, Holz" (Doppelwert) korrekt gehandhabt.
- **Vorschlag:** Bestätigung als reguläre Praxis: C ist **grundsätzlich mehrwertig zulässig**, wenn ein Dokument mehrere, im Text unterscheidbare materialspezifische Abschnitte für unterschiedliche Materialfamilien enthält. Kein neues Vokabular nötig, nur die Klarstellung, dass Mehrfachwerte nicht auf horizontale/materialübergreifende Fälle beschränkt sind, sondern auch additiv für parallel geregelte Einzelmaterialien gelten.

---

## 4. Achse D — Rechtsform

Die D-Achse war mit vier separaten Fundstellen die am häufigsten an ihre Grenzen gebrachte Achse. Alle vier Befunde betreffen echte kategoriale Lücken, keine Abgrenzungsunschärfe innerhalb bestehender Werte.

### 4.1 Vorschlag: neuer Wert „Rechtsprechung/Urteil"
- **Befund:** REG-DE-1-011 (EuGH C-100/13) ist die Fundstelle mit der größten unmittelbaren Wirkung auf das gesamte Ü-Zeichen-System im Produkt-Piloten — bindender als jede Verwaltungsvorschrift — passt aber in keinen der zehn D-Werte („Verwaltungsvorschrift" wäre falsch, kein Behördenerlass; „Gesetz" wäre falsch, kein Parlamentsakt).
- **Abgrenzungsregel:** Rechtsprechung/Urteil = Fundstelle ist ein gerichtlicher Tenor mit unmittelbarer Bindungswirkung (EuGH-Vertragsverletzungsurteile, BVerfG-Entscheidungen, einschlägige BGH/BVerwG-Leitentscheidungen); **Verwaltungsvorschrift zählt nicht**, auch wenn eine Behörde (wie hier DIBt) das Urteil nachvollzieht — die *Bekanntmachung* der Behörde (hier: Aufhebung der Bauregellisten) ist ein separates, ggf. eigenes Regelungsobjekt mit D=Verwaltungsvorschrift, das im Feld „Relationen" auf das Urteil als „setzt um" verweist.

### 4.2 Vorschlag: neuer Wert „Muster-/Modellrecht (unverbindlich, Umsetzung durch Dritte erforderlich)"
- **Befund:** MBO und MVV TB sind „Muster" ohne eigene Rechtskraft, werden erst durch 16-fache Landesübernahme bindend — von der Fallenliste des Auftrags ausdrücklich als Falle benannt („MBO ist Muster ohne Rechtskraft"), aber im Vokabular selbst nicht abbildbar. „Verwaltungsvorschrift" trifft nicht (MBO ist keine Vorschrift einer Behörde, sondern ein ARGEBAU-Konsensdokument); „Merkblatt" trifft nicht (kein bundesweiter Quasi-Standardisierungsanspruch mit 16-facher Verbindlichmachung als *Zweck* des Dokuments).
- **Abgrenzungsregel:** Muster-/Modellrecht = Dokument ist ausdrücklich als Vorlage für wortgleiche oder inhaltsgleiche Übernahme durch mehrere separate Rechtsetzungsträger konzipiert (MBO für 16 LBOs, MVV TB für 16 Länder-VV-TB); **Merkblatt zählt nicht**, wenn das Dokument als eigenständige fachliche Empfehlung ohne Transformationszweck auftritt (ARGEBAU-Hinweise Standsicherheit, BW-Leitfaden REG-DE-2-010 — beide sind D=Merkblatt, weil sie nicht zur Übernahme in Landesrecht bestimmt sind, sondern direkt als Praxishilfe wirken). **Wichtig für die Sub-Ebene-Kodierung:** Bei D=Muster-/Modellrecht ist die A-Kodierung nach Regel 1.2 zu prüfen (sub-national nur bei verifizierter Übernahme).

### 4.3 Vorschlag: neuer Wert „DIN SPEC (reduziertes Konsensverfahren)"
- **Befund:** DIN SPEC 91484 (REG-DE-2-005) durchläuft kein vollständiges Normungsverfahren nach DIN 820, sondern ein schnelleres PAS-artiges Verfahren; die Einordnung zwischen „nat.Norm" und „Merkblatt" ist eine Verlegenheitslösung. DIN SPECs treten im Reuse-Feld erkennbar wiederholt auf (91484, laut Sekundärquelle auch 91525).
- **Abgrenzungsregel:** DIN SPEC zählt als eigener Wert, **nat.Norm zählt nicht** für Dokumente mit DIN-SPEC-Kennzeichnung im Titel/Katalogeintrag, unabhängig vom Inhalt — die Unterscheidung ist rein verfahrensformal (DIN 820-Vollverfahren vs. verkürztes Konsensverfahren) und am Dokumententitel eindeutig erkennbar, ohne inhaltliche Prüfung nötig zu machen.

### 4.4 Vorschlag: neuer Wert „Eurocode/CEN-Bemessungsnorm"
- **Befund:** Eurocodes (REG-DE-2-007, DIN EN 1990-2) sind keine „harmonisierten Normen" im CPR-Sinn (hEN ist im Schema an die CE-Vermutungswirkungs-Logik der CPR gebunden) — sie sind CEN-erarbeitete Bemessungsnormen ohne CE-Bezug, national mit Nationalem Anhang übernommen. Die Verlegenheitscodierung „nat.Norm (in Entstehung)" verdeckt sowohl den CEN-Ursprung als auch den grundsätzlich anderen Bindungsmechanismus (VV-TB-Listung statt CE-Vermutung).
- **Abgrenzungsregel:** Eurocode/CEN-Bemessungsnorm = CEN-Normen der Reihe EN 1990 ff. und funktional gleichgestellte Bemessungsnormen ohne CPR-Produktbezug; **hEN zählt nicht**, auch wenn eine Norm ebenfalls von CEN erarbeitet wird, sobald sie eine CPR-Produktnorm mit AVCP-Verfahren und CE-Kennzeichnungsfolge ist. Status (Entwurf/prEN vs. finale EN) wird weiterhin im Feld „Status" geführt, nicht in D.

### 4.5 Vorschlag: neuer Wert „Techn. Regel mit Vermutungswirkung (TRGS/TRBS/TRBA-Typ)"
- **Befund:** TRGS 519 (REG-DE-4-003) passt in keinen der zehn Werte sauber: keine klassische Verwaltungsvorschrift (bindet nicht primär die Verwaltung, sondern entfaltet über § 7 Abs. 2 GefStoffV Vermutungswirkung gegenüber privaten Arbeitgebern), aber auch keine „Techn.Baubestimmung" (dieser Wert ist im Schema an das Bauordnungsrecht/§85a-MBO-System gebunden). Der Wirkmechanismus (Ausschuss erarbeitet Regel, Ministerium macht sie im Amtsblatt bekannt, Gesetz/RVO verknüpft Einhaltung mit Vermutungswirkung) ist ein eigenständiges, wiederkehrendes Muster im deutschen Arbeitsschutz-/Gefahrstoffrecht.
- **Abgrenzungsregel:** Techn. Regel mit Vermutungswirkung = Regelwerk eines gesetzlich verankerten Fachausschusses (AGS, ABAS, AfPS u. Ä.), amtlich bekannt gemacht, mit gesetzlich verankerter Vermutungswirkung bei Einhaltung; **Techn.Baubestimmung zählt nicht**, wenn der Bindungsmechanismus über § 85a MBO/VV-TB-System läuft (Bauordnungsrecht), auch wenn die Wirkung strukturell ähnlich ist — die beiden Mechanismen haben unterschiedliche Rechtsgrundlagen (Arbeitsschutzrecht vs. Bauordnungsrecht) und sollten für die Konflikttabelle unterscheidbar bleiben.

### 4.6 Zusammenfassung D-Achse
D wird von zehn auf **fünfzehn Werte** erweitert: die zehn bestehenden plus Rechtsprechung/Urteil, Muster-/Modellrecht, DIN SPEC, Eurocode/CEN-Bemessungsnorm, Techn. Regel mit Vermutungswirkung. Alle fünf sind in mindestens einem Pilot mit konkreter Primärquelle belegt, keine ist hypothetisch. **Wichtig:** D bleibt eine Ordinalskala der „Projektkonvention formeller Verbindlichkeit" (keine Rechtshierarchie) — die fünf neuen Werte sind nicht automatisch am unteren oder oberen Ende der bestehenden Skala einzuordnen, sondern erfordern für den Freeze eine explizite Einordnungsentscheidung (z. B.: Rechtsprechung/Urteil vermutlich zwischen „EU-VO/Gesetz" und „RVO" einzuordnen, da unmittelbare Bindungswirkung ohne Umsetzungsakt).

---

## 5. Achse E — Prozessphase

### 5.1 Vorschlag: „Abfallstatus" als markierbar überspringbare Phase
- **Befund:** Der zentrale rechtliche Hebel für Bauteil-Wiederverwendung (REG-DE-3-001, KrWG § 3 Abs. 21) besteht gerade darin, die Phase „Abfallstatus" *nicht* zu durchlaufen. Eine Prozessphasen-Liste, die Abfallstatus als Pflichtstation zwischen Rückbau und Aufbereitung führt, bildet den bevorzugten (weil regulatorisch günstigsten) Pfad nicht ab.
- **Vorschlag:** Kein neuer Phasenwert, sondern ein optionales Attribut je Regelungsobjekt: `E-Wirkung: durchläuft | vermeidet | erzwingt [Phase]`. **Abgrenzungsregel:** „vermeidet" wird gesetzt, wenn die Norm ihre reuse-ermöglichende Wirkung *aus dem Nicht-Erreichen* einer Phase zieht (KrWG §3 Abs. 21 vermeidet Abfallstatus); „durchläuft" ist der Normalfall (Norm regelt eine Phase, die das Bauteil ohnehin durchläuft); „erzwingt" markiert Normen, die eine Phase verbindlich vorschreiben, obwohl ein kürzerer Weg denkbar wäre (z. B. GefStoffV-Erkundungspflicht erzwingt Bestandserkundung vor Rückbau).

### 5.2 Vorschlag: „Vorbereitung zur Wiederverwendung" als Grenzoperation kennzeichnen
- **Befund:** Der abfallrechtliche Begriff „Vorbereitung zur Wiederverwendung" (KrWG § 3 Abs. 24, referenziert in mehreren Objekten) markiert exakt den Übergang zwischen den Phasen „Abfallstatus" und „Aufbereitung/Prüfung" — er *ist* die Grenzoperation, kein Punkt auf der Linie davor oder danach. Die Acht-Phasen-Liste behandelt Phasen wie disjunkte Container, was für diesen zentralen abfallrechtlichen Übergangsbegriff strukturell unpassend ist.
- **Vorschlag:** Für Regelungsobjekte, deren Kern eine Übergangsoperation zwischen zwei benachbarten E-Phasen ist, werden **beide angrenzenden Phasen als Doppelwert** kodiert (hier bereits im Pilot so gehandhabt: „Abfallstatus, Aufbereitung/Prüfung"), mit einem Hinweis „Grenzoperation" in der Kernaussage. Kein neuer Phasenwert nötig; die Klarstellung ist lediglich, dass Doppelwerte an Phasengrenzen die *korrekte* Kodierung sind, nicht ein Kompromiss.

### 5.3 Vorschlag: E als Netzwerk-/Filterachsen-Unterscheidung dokumentieren
- **Befund:** Horizontale Ermöglichungsnormen (REG-DE-2-001/002/009/010 im ZiE-Piloten) decken je 2–5 von 8 Phasen gleichzeitig ab — inhaltlich korrekt, aber die Diskriminierungskraft von E sinkt gerade für die Objekte, die für den Bericht am wichtigsten sind.
- **Vorschlag:** Keine Schemaänderung; Klarstellung für die Auswertungsmethodik (nicht Gegenstand der Vokabularpflege): E eignet sich gut als Achse für Netzwerk-/Prozessdiagramme (welche Regelungsobjekte berühren Phase X), weniger gut als scharfer Filter zur Objektunterscheidung bei horizontalen Normen. Für den Bericht ggf. B (mit Primärfeld, s. 2.1) als schärferes Filterkriterium nutzen.

---

## 6. Achse F1/F2 — Wirkrichtung

Drei voneinander unabhängige Erweiterungsbedarfe, alle durch die Einwertigkeit von F1/F2 pro Objekt verursacht.

### 6.1 Vorschlag: optionales Attribut „Wirksamkeitsstichtag/-bedingung"
- **Befund:** Das CPR-Übergangsregime (REG-EU-1-006) bedeutet, dass für dieselbe materielle Frage (DoP-Pflicht bei Gebrauchtprodukten) je nach Produktfamilie entweder das alte, gebraucht-blinde VO-305/2011-Regime oder das neue, explizite VO-2024/3110-Regime gilt — nicht abstrakt, sondern faktisch bis in die 2030er-Jahre parallel. Ein einzelner F1-Wert („ermöglichend" für die neue VO) verdeckt, dass zum Stichtag für die meisten Produktfamilien praktisch noch die alte Rechtslage greift.
- **Vorschlag:** F1/F2 erhalten ein optionales Attribut `Wirksamkeitsbedingung: [Freitext, z. B. "gilt erst ab Durchführungsrechtsakt zur jeweiligen hEN, spätestens 2040"]`. **Abgrenzungsregel:** Das Attribut wird gesetzt, wenn die F1/F2-Einordnung nicht ab dem Stichtag 2026-08-11 uneingeschränkt gilt, sondern von einem noch ausstehenden oder gestaffelten Ereignis abhängt; **kein Attribut nötig**, wenn die Norm bereits unbedingt in Kraft ist, selbst wenn sie ein Übergangsregime *für andere* Normen beschreibt (dann ist die Übergangsnorm selbst F1=widersprüchlich/hemmend, wie REG-EU-1-006 bereits zeigt — das Attribut betrifft nur die *abhängigen* Objekte).

### 6.2 Vorschlag: optionales Sub-Feld „Bezugsobjekt der Aussage" (Materialstrom vs. Bauteil)
- **Befund:** Bei REG-DE-3-003 (EBV) und REG-DE-3-004 (GewAbfV) reicht ein F1/F2-Einzelwert nicht: Die Norm ist *gegenüber Materialströmen* bedingend-ermöglichend, *gegenüber ganzen Bauteilen* aber schlicht nicht existent — kein „schweigend" im Sinne von „regelt eine Frage nicht, zu der sie etwas sagen könnte", sondern „das Objekt Bauteil kommt im Tatbestand gar nicht vor".
- **Vorschlag:** Optionales Sub-Feld `Bezugsobjekt: Materialstrom | ganzes Bauteil | beides`, das vor F1/F2 gesetzt wird, wenn eine Norm potenziell beide Objekttypen berühren könnte. **Abgrenzungsregel:** „schweigend" (bestehender F-Wert) bleibt reserviert für Fälle, in denen die Norm das Bauteil als Regelungsgegenstand *kennt*, aber keine Aussage trifft; **„nicht regelungsgegenständlich" ist ein neuer, von „schweigend" abzugrenzender Zustand** — zu kodieren als F1=schweigend **mit** Bezugsobjekt-Vermerk „ganzes Bauteil: tatbestandlich nicht erfasst" in der Kernaussage, um die Unterscheidung ohne zusätzlichen F-Wert zu erhalten.

### 6.3 Vorschlag: Vermerk-Konvention für Doppelnatur „ermöglichend UND strukturell begrenzend"
- **Befund:** REG-DE-2-001/-002 (aBG/vBG, ZiE) ermöglichen Reuse explizit, indem sie überhaupt einen Zulassungsweg für nicht-normierte Bauarten/-produkte schaffen — aber die Konstruktion als Einzelfall-/vorhabenbezogenes Instrument ist selbst der limitierende Faktor für Reuse-Skalierung. Das ist keine Rechtslage/Praxis-Diskrepanz (F1 vs. F2) und kein Normwiderspruch („widersprüchlich"), sondern eine im selben Rechtstext angelegte doppelte Wirkung auf zwei verschiedene Bezugsgegenstände.
- **Vorschlag:** Keine neue F-Kategorie (Gefahr der Verwässerung von „widersprüchlich"), sondern die Pflicht, in solchen Fällen **zwei F1-Teilaussagen mit je eigenem Bezugsgegenstand** in einem Feld zu formulieren, z. B. „F1 (E3): ermöglichend bzgl. Zulassungsfähigkeit dem Grunde nach; bedingend bzgl. Skalierbarkeit (Einzelfallbindung)" — wie im ZiE-Piloten bereits praktiziert. **Abgrenzungsregel:** „widersprüchlich" bleibt reserviert für Fälle, in denen zwei *unterschiedliche* Normen oder Normebenen sich gegenläufig auswirken (z. B. REG-EU-1-006: altes vs. neues CPR-Regime); die Doppelnatur-Konvention gilt, wenn *eine einzelne* Norm zwei Wirkungsebenen im selben Bezugsgegenstand hat.

---

## 7. Achse G — Nachweisanforderung

### 7.1 Vorschlag: achter G-Wert „Anwendbarkeits-/Ausnahmenorm (kein Nachweistatbestand)"
- **Befund:** REG-EU-1-003 (CPR Art. 20 Abs. 1, Wirtschaftsteilnehmerpflichten nur für hEN-/ETA-Produkte) und REG-EU-1-004 (Erwägungsgrund 34, Ausnahme für Wiederverwendung im selben Bauwerk) sind reine Scope-Normen — sie legen fest, OB ein Regime greift, nicht WAS nachzuweisen ist. Beide gehören zu den praktisch wichtigsten reuse-relevanten Fundstellen des gesamten Produkt-Piloten; das Schema zwang, sie notdürftig als „entfällt" zu markieren, was im Freitext funktioniert, aber im kontrollierten Vokabular selbst keinen Platz hat.
- **Abgrenzungsregel:** „Anwendbarkeitsnorm" zählt, wenn die zitierte Fundstelle den Geltungsbereich eines Regimes ab- oder eingrenzt, ohne selbst eine Handlungs- oder Nachweispflicht zu begründen; **„entfällt" (weiterhin als Wert zu führen) bleibt reserviert** für Fälle, in denen weder ein Nachweistatbestand noch eine Scope-Funktion vorliegt (seltener Grenzfall, z. B. rein deklaratorische Bestimmungen ohne Rechtsfolge).

### 7.2 Vorschlag: neunter G-Wert „Statusfeststellung/Anwendbarkeitsprüfung (Nicht-Geltung eines Regimes)"
- **Befund:** Für REG-DE-3-001 (KrWG-Abfallbegriff) ist der relevante Nachweis nicht „wie erfülle ich eine Regel", sondern „dass ein Regime (Abfallrecht) überhaupt nicht eröffnet ist" (kein Entledigungswille). Das wurde im Pilot notdürftig unter „Dokumentenlage" gefasst, ist aber kategorial etwas anderes als ein Konformitätsnachweis.
- **Abgrenzungsregel — Abgrenzung zu 7.1:** „Anwendbarkeitsnorm ohne Nachweis" (G-Wert aus 7.1) markiert eine Norm, die *keinen* Nachweistatbestand hat, weil sie reine Scope-Regel ist (Bezugsebene: die Norm selbst). „Statusfeststellung/Anwendbarkeitsprüfung" (dieser neue Wert) markiert einen Nachweistyp, der *im Vollzug* zu erbringen ist, um zu belegen, dass ein bestimmtes Regime auf einen konkreten Sachverhalt nicht anwendbar ist (Bezugsebene: der Einzelfall/das Bauteil) — z. B. der Nachweis fehlenden Entledigungswillens bei einem konkret ausgebauten Bauteil. **Statusfeststellung zählt nicht**, wenn der Nachweis auf die Erfüllung von Anforderungen zielt (dann einer der bestehenden sieben Werte); **Anwendbarkeitsnorm (7.1) zählt nicht**, wenn tatsächlich ein Nachweis im Vollzug verlangt wird, auch wenn dieser Nachweis negativ formuliert ist („Nicht-Abfall-Status").

### 7.3 Vorschlag: G als geordnete/bedingte Liste statt flaches Set
- **Befund:** REG-DE-2-010 (BW-Leitfaden) zeigt, dass Reuse-Nachweise praktisch nie ein einzelner G-Typ sind, sondern eine bedingte Kaskade (Dokumentenlage → ggf. Sichtprüfung → ggf. zerstörungsfreie Prüfung → ggf. Probenahme/Materialprüfung → rechnerischer Nachweis → Einzelfallzulassung), bei der jede Stufe nur bei Auffälligkeiten der vorherigen ausgelöst wird. Eine flache Liste verliert diese Sequenz- und Bedingtheitsinformation.
- **Vorschlag:** Wo eine Fundstelle ausdrücklich eine Stufenfolge vorschreibt (E1, textbelegt), wird G als **nummerierte Liste mit Bedingungspfeil** notiert, z. B. „G: 1. Dokumentenlage → 2. Sichtprüfung (falls 1. auffällig) → 3. zerstörungsfreie Prüfung (falls 2. auffällig) → 4. Probenahme/Materialprüfung → 5. rechnerischer Nachweis → 6. Einzelfallzulassung — alle explizit (E1)". **Abgrenzungsregel:** Die Kaskaden-Notation wird nur verwendet, wenn die Bedingtheit („nur wenn vorherige Stufe X ergibt") textlich oder aus der Verfahrenslogik der Quelle eindeutig hervorgeht; bei mehreren gleichzeitig, nicht gestuft nebeneinander verlangten Nachweisen (z. B. REG-DE-3-004: Dokumentenlage UND Erklärung Dritter kumulativ) bleibt die bisherige Mehrfachwert-Notation mit „/" (nicht „→") maßgeblich.

### 7.4 Klarstellung zur E1/E3-Doppelkennzeichnung bei G (kein neuer Wert, Vereinfachungsvorschlag)
- **Befund:** G-explizit/-inferiert und die Evidenzgrade E1/E2/E3 überschneiden sich teilweise redundant — für G ist die explizit/inferiert-Markierung im Kern eine Unterkategorie von E1/E3, wird aber separat gepflegt.
- **Vorschlag zur Prüfung für den Freeze (kein zwingender Vorschlag, offene Empfehlung):** G-explizit könnte als Kurzform für „G ist Teil der E1-Textbasis des Objekts" behandelt und G-inferiert als „G ist E3-Projektzuordnung" — d. h. auf das separate Tag-Paar verzichtet und stattdessen G immer im Kontext des ohnehin zu vergebenden Konfidenz-/Evidenzgrads gelesen werden. Da dies eine Vereinfachung (Streichung) statt Erweiterung ist, wird sie hier nur zur Entscheidung vorgelegt, nicht als Muss-Vorschlag geführt.

### 7.5 Zusammenfassung G-Achse
G wird von sieben auf **neun Werte** erweitert (Anwendbarkeitsnorm ohne Nachweis; Statusfeststellung/Anwendbarkeitsprüfung), zusätzlich die Kaskaden-Notation als Schreibkonvention für gestufte Nachweisverfahren.

---

## 8. Beleg-Quelle, Zugänglichkeit, Bindungsketten-Regel

### 8.1 Vorschlag: Zugänglichkeitswert „frei-primär-blockiert"
- **Befund:** TRGS 519 (REG-DE-4-003) ist nicht paywalled — es gibt keine Bezahlschranke —, aber ein aktiver Bot-Schutz (baua.de, HTTP 403/JS-Challenge) verhinderte den Volltextzugriff über WebFetch und lokale Tools gleichermaßen. Das bestehende Dreier-Vokabular „frei-primär / paywalled-eingesehen / paywalled-nicht-eingesehen" kennt diesen Fall nicht und zwang zur irreführenden Behandlung wie eine Paywall.
- **Abgrenzungsregel:** „frei-primär-blockiert" zählt, wenn die Quelle nachweislich ohne Bezahlschranke öffentlich zugänglich sein sollte, der Zugriff aber durch technische Hürden (Bot-Schutz, Geoblocking, defekte Serverkonfiguration) faktisch verhindert wurde; **„paywalled-nicht-eingesehen" zählt nicht** in diesem Fall, weil dort keine Bezahlschranke vorliegt und die Bindungsketten-Regel (Suche nach einem freien amtlichen Alternativakt) hier nicht zielführend ist — der Lösungsweg ist ein erneuter Zugriffsversuch, nicht eine Ersatzquelle. **Funktionale Konsequenz:** B2/B3-Einstufung und Konfidenz „unklar" bleiben trotzdem zwingend (kein Fakt ohne Volltexteinsicht) — der neue Wert ändert nur die Zugänglichkeits-Diagnose, nicht die Beleg-Quellen-Strenge.

### 8.2 Vorschlag: Zwischenzustand „Bindungsakt existiert, Listung nicht verifiziert"
- **Befund:** Für DIN SPEC 91484, ISO 13822 und EN 1990-2 (Produkt- und ZiE-Pilot) konnte jeweils nur festgestellt werden, DASS ein Bindungsmechanismus (VV TB) existiert, nicht OB die jeweilige Norm darin tatsächlich gelistet ist — eine 16-Länder-VV-TB-Volltextsuche pro Norm überstieg den Pilot-Zeitrahmen.
- **Vorschlag:** Das Feld „Bindungsakt" erhält einen dritten möglichen Eintrag neben „identifiziert und Volltext geprüft" und „kein Bindungsakt identifiziert": **„Bindungsakt existiert (Mechanismus bekannt), Listung im Einzelfall nicht verifiziert"**. **Abgrenzungsregel:** Dieser Zwischenzustand zählt, wenn der *generische* Bindungsmechanismus (hier: § 85a MBO/VV-TB-System) primärquellenbasiert belegt ist, aber die *konkrete Aufnahme* der fraglichen Norm in mindestens eine Länder-VV-TB nicht geprüft wurde; er zählt **nicht** als vollwertiger Bindungsakt-Nachweis für die Kernaussage — Aussagen zur tatsächlichen Verbindlichkeit der betroffenen Norm bleiben Konfidenz „unklar", bis die Listung geprüft ist.

---

## 9. Relationen-Vokabular

Das bestehende Vokabular `setzt um | ersetzt | konkretisiert | kollidiert mit` deckt zwei in den Piloten wiederholt auftretende Beziehungstypen nicht ab.

### 9.1 Vorschlag: „verdrängt (lex specialis)"
- **Befund:** § 8 Abs. 1a GewAbfV verweist für die EBV-Stoffgruppen „ausschließlich" auf § 24 EBV (REG-DE-3-004 → REG-DE-3-003) — weder „ersetzt" (GewAbfV bleibt für andere Stoffe in Kraft), noch „konkretisiert" (EBV wiederholt nicht GewAbfV, sondern schließt sie für den Teilbereich aus), noch „kollidiert mit" (kein Widerspruch, sondern eine geregelte Vorrangbeziehung).
- **Abgrenzungsregel:** „verdrängt (lex specialis)" zählt, wenn eine Norm für einen klar abgegrenzten Teilbereich den Vorrang einer anderen, spezielleren Norm ausdrücklich textlich anordnet (Verweisungsnorm mit „ausschließlich"/"stattdessen"); **„ersetzt" zählt nicht**, wenn die verdrängte Norm für andere Teilbereiche desselben Anwendungsbereichs unverändert fortgilt; **„kollidiert mit" zählt nicht**, wenn die Vorrangbeziehung selbst normativ geklärt ist (kein offener Konflikt, sondern eine gelöste Konkurrenz).

### 9.2 Vorschlag: „wird kombiniert mit / ergänzt (parallele Verfahrensinstrumente für denselben Anwendungsfall)"
- **Befund:** ZiE (Bauprodukt-Ebene, § 20 MBO) und vBG (Bauart-Ebene, § 16a Abs. 2 MBO) sind zwei rechtlich getrennte Instrumente für zwei getrennte Regelungsgegenstände, die bei Wiederverwendungs-Bauteilen laut primärquellenbasiertem BW-Leitfaden typischerweise **gemeinsam beantragt** werden. Das ist weder Ersetzung noch Konkretisierung noch Normkollision.
- **Abgrenzungsregel:** „wird kombiniert mit" zählt, wenn zwei oder mehr Regelungsobjekte in der dokumentierten Vollzugspraxis (belegt durch Primärquelle wie den BW-Leitfaden, nicht durch bloße Vermutung) routinemäßig gemeinsam für denselben Sachverhalt angewendet werden, ohne dass eines das andere ersetzt, konkretisiert oder mit ihm kollidiert; **„konkretisiert" zählt nicht**, wenn beide Instrumente einen jeweils eigenständigen, nicht ineinander aufgehenden Regelungsgegenstand haben (hier: Bauprodukt vs. Bauart) — Konkretisierung setzt ein Verhältnis von Abstraktem zu Konkretem voraus, nicht von Parallelität.

---

## 10. Evidenzgrade und E-Kategorien — ergänzende Klarstellung

Kein neuer Wert, aber eine aus dem Produkt-Piloten und dem ZiE-Piloten übereinstimmend aufgeworfene Klärungsfrage:

- **Befund:** Die G-explizit/-inferiert-Kennzeichnung überschneidet sich mit E1/E2/E3 (s. 7.4); zugleich zeigt der Produkt-Pilot bei REG-DE-2-009, dass ein Objekt in verschiedenen Feldern verschiedene Evidenzgrade tragen kann (G=Einzelfallzulassung ist dort E1-explizit, während die C/E-Einordnung desselben Objekts E2/E3 ist). Das Schema erlaubt dies bereits implizit (Evidenzgrade werden pro Aussage, nicht pro Objekt vergeben), aber es fehlt eine ausdrückliche Klarstellung.
- **Vorschlag:** Im Schema-Freeze explizit festhalten: **Evidenzgrade (E1/E2/E3) werden je Achse einzeln vergeben, nicht als ein einziger Wert pro Regelungsobjekt.** Ein Regelungsobjekt kann z. B. A/B/D/G = E1 (textbelegt) und gleichzeitig F1/F2 = E3 (Projektzuordnung) tragen — das ist der Normalfall, kein Ausnahmefall, und sollte in der Kopfzeile jeder Anlage einmal ausdrücklich erläutert werden, damit nachfolgende Bearbeiter nicht versuchen, einen einzigen Konfidenzwert pro Objekt zu erzwingen.

---

## 11. Zusammenfassende Tabelle der Vokabular-Erweiterungen

| Achse | Bisher | Vorschlag Ergänzung | Typ |
|---|---|---|---|
| A | 3 Werte | + „international (nicht-EU/EEA)"; Vermerk-Pflicht „Downstream-Verifikationsstatus" bei Muster-/ARGEBAU-Dokumenten | 1 neuer Wert + 1 Prozessregel |
| B | 7 Felder, Einfachauswahl | Primärfeld (Pflicht) + Nebenfelder (optional, mehrwertig); Flag „Grundnorm/Begriffsnorm" | Strukturänderung (Mehrfachauswahl) + 1 Flag |
| C | 9 Werte (inkl. materialübergreifend) | Aufspaltung „materialübergreifend" → „-horizontal" + neuer Wert „Verbund-/Systembauteil"; Mehrfachwert-Praxis bestätigen | 1 Aufspaltung + 1 neuer Wert |
| D | 10 Werte | + Rechtsprechung/Urteil; Muster-/Modellrecht; DIN SPEC; Eurocode/CEN-Bemessungsnorm; Techn. Regel mit Vermutungswirkung | 5 neue Werte |
| E | 8 Phasen | Attribut `E-Wirkung: durchläuft\|vermeidet\|erzwingt`; Doppelwert-Konvention an Phasengrenzen bestätigen | 1 neues Attribut |
| F1/F2 | je 5 Wirkrichtungswerte, einwertig | Attribut „Wirksamkeitsstichtag/-bedingung"; Sub-Feld „Bezugsobjekt der Aussage"; Doppelnatur-Notationskonvention | 2 neue Attribute + 1 Schreibkonvention |
| G | 7 Werte | + „Anwendbarkeits-/Ausnahmenorm (kein Nachweis)"; + „Statusfeststellung/Anwendbarkeitsprüfung"; Kaskaden-Notation für gestufte Verfahren | 2 neue Werte + 1 Schreibkonvention |
| Beleg-Quelle/Zugänglichkeit | 3 Zugänglichkeitswerte | + „frei-primär-blockiert"; Bindungsakt-Zwischenzustand „existiert, Listung nicht verifiziert" | 1 neuer Wert + 1 Zwischenzustand |
| Relationen | 4 Werte | + „verdrängt (lex specialis)"; + „wird kombiniert mit/ergänzt" | 2 neue Werte |
| Evidenzgrade | E1/E2/E3 | Klarstellung: Vergabe je Achse, nicht je Objekt (keine Vokabularänderung) | Klarstellung |

---

## 12. Nicht übernommene Beobachtungen (zur Transparenz)

- **B-Wert „Institutionelles Ausführungsrecht"** (erwogen für BauPG): zurückgewiesen, s. 2.3 — bereits durch Primärfeld-Konzept und Freitext abgedeckt.
- **G-explizit/-inferiert als eigenes Tag-Paar streichen**: als offene Empfehlung, nicht als Muss-Vorschlag geführt (s. 7.4) — Entscheidung wird dem Freeze-Termin überlassen, da es sich um eine Vereinfachung mit möglichem Informationsverlust handelt, nicht um eine Erweiterung.
- **Fünfter A-Wert für ARGEBAU-Ebene**: zurückgewiesen zugunsten einer Vermerk-Pflicht statt Vokabular-Erweiterung (s. 1.2), um Kategorien-Inflation auf der am stärksten strukturierenden Achse zu vermeiden.

---

## 13. Lücken dieses Entwurfs

- Die Vorschläge stützen sich ausschließlich auf drei DE-fokussierte Piloten (Feld 1–4, Materialfamilien Baustahl/Holz/mineralisch nur am Rand). Ob weitere Achsenprobleme bei anderen Feldern (5a Vergaberecht, 5b Förderung, 7 Haftung) oder anderen Jurisdiktionen (NL, CH, BE, Nordics — s. Fallenliste) auftreten, ist mit diesem Entwurf nicht geprüft und explizit an Taxonomie-Entwurf 2/2 bzw. W1 zu adressieren.
- Die vorgeschlagene D-Achsen-Erweiterung auf 15 Werte wurde nicht auf Redundanz mit den vier neuen G-/F-Werten rückgeprüft (z. B. ob „Techn. Regel mit Vermutungswirkung" und der G-Wert „Anwendbarkeitsnorm ohne Nachweis" in Kombination zu Doppelkodierungen führen) — Konsistenzprüfung ist Aufgabe des Freeze-Prozesses.
- Für die Primärfeld/Nebenfelder-Umstellung bei B (2.1) wurde nicht geprüft, wie das die 29 bereits im Pilot erhobenen Regelungsobjekte rückwirkend verändern würde (Migrationsaufwand); dies ist vor einer verbindlichen Einführung zu klären.
