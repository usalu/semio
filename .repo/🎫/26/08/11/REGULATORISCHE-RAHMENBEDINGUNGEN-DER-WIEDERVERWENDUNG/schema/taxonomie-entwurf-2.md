# Taxonomie-Entwurf 2/2 · Verfeinerungsvorschläge zum Sieben-Achsen-Schema

Grundlage: Drei W0-Piloten (DE, Stichtag 2026-08-11) — `pilot-de-produkt.md` (12 Objekte, Feld 1), `pilot-de-abfall.md` (7 Objekte, Feld 3/4), `pilot-de-zie.md` (10 Objekte, Feld 2) — inkl. ihrer jeweiligen „Schema-Stresstest"-Abschnitte. Insgesamt 29 Regelungsobjekte, 27 Einzelbefunde zur Schemapassung.

Auftrag: konkrete Verfeinerungen des kontrollierten Vokabulars und der sieben Achsen A–G vorschlagen — **keine** Änderung der Grundstruktur (sieben Achsen, ID-Schema, Blockformat, Evidenzgrade E1–E3, Belegregeln B0–B4 bleiben bestehen). Wo ein Befund in mehreren Piloten unabhängig voneinander auftrat, ist das als Konvergenzsignal vermerkt — solche Punkte haben Priorität für den Freeze.

Methodischer Hinweis vorab: Kein einziger der 27 Stresstest-Befunde verlangt eine neue Achse oder eine andere Grundlogik. Alle Probleme sind vom Typ „Vokabularwert fehlt", „Achse erzwingt Einfachwert, wo Mehrfachwert nötig wäre" oder „Achse unterstellt eine Struktur (Linearität, Nachweisbarkeit, Einzelwirkung), die bei bestimmten Normtypen nicht zutrifft". Das Schema ist also im Kern tragfähig; es braucht gezielte Erweiterung, keine Revision.

---

## 1. Achse A — Jurisdiktion/Ebene

**Problem 1 (zie, Konfidenz: einzelbelegt, aber strukturell zwingend):** A kennt nur EU/EEA · national · sub-national. Für Normen mit internationalem Ursprung ohne EU/EEA-Spezifik (ISO 13822, REG-DE-2-006) fehlt ein Wert — sie werden erst durch nationale Übernahme (DIN ISO) zu einem A=national-Dokument, obwohl der materielle Text auf ISO-Ebene entsteht. Dasselbe Muster, verschärft, bei Eurocodes (REG-DE-2-007): Erarbeitungsebene ist CEN/EU, Bindungsebene ist national (erst über Nationalen Anhang + VV-TB-Listung). Erarbeitungsebene und Bindungsebene fallen hier systematisch auseinander, und A bildet nur eine davon ab.

**Vorschlag:** A bleibt dreiwertig für die **Bindungsebene** (das ist die Ebene, auf der die Regel für die handelnden Akteure verbindlich wird — daran hängt die praktische Relevanz). Zusätzlich ein optionales Attribut **A-Ursprung** mit den Werten `EU/EEA` · `national` · `sub-national` · `international (nicht EU/EEA, z. B. ISO)`, das nur bei Abweichung von A gesetzt wird (Default: A-Ursprung = A).
- Beispiel: REG-DE-2-006 (ISO 13822) → A = national (Bindung erst über DIN-Übernahme + VV-TB), A-Ursprung = international.
- Beispiel: REG-DE-2-007 (Eurocode-Entwurf) → A = EU/EEA (CEN-Erarbeitungsebene, noch keine nationale Bindung), A-Ursprung = EU/EEA (deckungsgleich, kein Zusatzwert nötig) — sobald ein Nationaler Anhang vorliegt, wechselt A perspektivisch zu national/sub-national, A-Ursprung bleibt EU/EEA.

**Problem 2 (zie):** ARGEBAU-Dokumente (16-Länder-Gremium, weder Bund noch Einzelland) lassen sich A-seitig nicht konsistent einordnen. Im Pilot wurde die MBO als A=sub-national kodiert (weil nahezu wortgleich in 16 Landesgesetze übernommen), die ARGEBAU-Standsicherheits-Hinweise (REG-DE-2-009) dagegen als A=national (weil als Merkblatt stehend, ohne verifizierte Landestransformation) — zwei strukturell ähnliche ARGEBAU-Produkte auf verschiedenen A-Werten, abhängig von einer nicht im Schema verankerten Zusatzprüfung.

**Vorschlag:** kein neuer A-Wert (ARGEBAU-Dokumente sind de facto immer entweder im Umsetzungsstadium sub-national oder im Vorstadium national — das Attribut ist korrekt, nur die Entscheidungsregel fehlt). Stattdessen verbindliche Kodierregel: **A = sub-national, wenn eine Downstream-Transformation in mindestens einer Landesnorm nachgewiesen ODER strukturell zwingend ist** (z. B. MBO-Wortlaut wird laut Vollzugspraxis in allen 16 LBOs nachvollzogen); **A = national, wenn das Dokument selbst der Endpunkt ist** und keine Rechtsform-Transformation in Landesrecht erfolgt (Merkblätter, Empfehlungen ohne Verordnungscharakter). Zusätzlich Pflichtfeld **„Downstream-Verifikationsstatus"**: `verifiziert in [Land(er)]` / `strukturell angenommen, nicht verifiziert` / `nicht geprüft` — macht sichtbar, ob die A-Einstufung auf Beleg oder auf Annahme beruht.

**Abgrenzungsregel A (Muster „gehört dazu / zählt nicht"):**
- EU-Verordnung mit unmittelbarer Geltung (CPR 2024/3110) → A = EU/EEA. EU-Richtlinie, die erst durch nationale Umsetzung wirkt (z. B. Abfallrahmenrichtlinie-Vorgaben, sofern im Projekt separat erhoben) → A = national/sub-national, je nachdem, auf welcher Ebene die Umsetzung erfolgt; die RL selbst wird nur im A-Ursprung-Feld referenziert, NICHT als A = EU/EEA kodiert, weil sie für Akteure vor Ort nicht unmittelbar bindend ist.
- Bundesgesetz/-verordnung (KrWG, GewAbfV, GefStoffV, BauPG) → A = national. Ein Landesgesetz, das denselben Regelungsgegenstand betrifft (NBauO), zählt NICHT als national, auch wenn es bundesweit strukturell gleichartige LBOs gibt — jede LBO ist ein eigenständiges A = sub-national-Objekt mit eigener Sub-Ebene-Kennung.

---

## 2. Achse B — Regelungsfeld

**Problem (produkt, abfall — konvergent):** B erzwingt Einfachauswahl aus den sieben Feldern, obwohl reale Instrumente mehrere Felder gleichzeitig bedienen. Belegt an MVV TB (Feld 1 CE-Ergänzung via Ü-Zeichen + Feld 2 Verwendbarkeitsnachweise + Feld 4 Brandschutz/Schallschutz + Feld 6 Normbezugnahme) und am DIBt-Verwendbarkeitsnachweis-System (liegt zwischen Feld 1 und Feld 2). Zusätzlich (abfall, REG-DE-3-001): B unterscheidet nicht zwischen einer **Grundnorm/Begriffsnorm** mit Gatekeeper-Funktion (KrWG § 3 bestimmt, ob das gesamte Feld 3 überhaupt eröffnet ist) und einer gewöhnlichen **operativen Vollzugsnorm** (GewAbfV § 8) — für die spätere Konflikttabelle relevant, weil Grundnormen typischerweise nicht mit operativen Normen „kollidieren", sondern deren Anwendungsbereich determinieren.

**Vorschlag 1 — Mehrfachauswahl mit Gewichtung:** B wird von Einfach- zu **Primärfeld + optionale Nebenfelder** umgestellt. Kodierregel: Primärfeld = das Feld, dessen Wirkmechanismus im Kernaussage-Text tragend ist; Nebenfelder = weitere Felder, die im selben Regelungsobjekt substanziell (nicht nur beiläufig erwähnt) mitgeregelt werden. Beispiel MVV TB: Primärfeld 2, Nebenfelder 1, 4, 6.

**Vorschlag 2 — Grundnorm-Flag:** zusätzliches optionales Attribut **„Normtyp"**: `Grundnorm/Begriffsnorm` (bestimmt Anwendbarkeit nachgelagerter Normen desselben oder verwandter Felder) vs. `operative Norm` (Regelfall, kein Flag nötig — Default). Wirkt sich auf die Relationen-Achse aus (s. Abschnitt 7): Grundnormen erhalten dort bevorzugt die Relation „determiniert Anwendbarkeit von" statt „kollidiert mit".

**Abgrenzungsregel B:** Ein Instrument zählt als **Primärfeld 2** (Bautechnische Zulassung/Standsicherheit), wenn sein Kern ein Nachweis-/Zulassungsverfahren für die Bauwerksstandsicherheit oder Bauteilverwendbarkeit ist (ZiE, abZ, aBG) — auch wenn es sekundär CE-/Ü-Zeichen-Fragen berührt. Ein Instrument zählt NICHT als Feld 2, sondern bleibt Feld 1, wenn sein Kern die Markt-Konformität eines Produkts ist (CPR) und die Standsicherheit nur als eine von mehreren Grundanforderungen mitgeregelt wird.

---

## 3. Achse C — Materialfamilie

**Problem 1 (abfall):** C ist rein einzelmaterialbezogen (Baustahl, Holz, Mauerwerk/mineralisch …) und kann **Verbundbauteile** — das eigentliche Forschungsobjekt „ganzes wiederverwendetes Bauteil" (Fenster: Glas + Rahmen + Beschlag + Dichtung) — nicht abbilden. Die zehn GewAbfV-Fraktionen bestätigen: Abfallrecht denkt in Werkstoffströmen, nicht in Bauteilen; „materialübergreifend" nivelliert diesen Befund, statt ihn sichtbar zu machen.

**Problem 2 (produkt, konvergent mit zie Punkt 10):** C ist für Feld 1 (Produktrecht) und für horizontales Verfahrensrecht in Feld 2 strukturell uninformativ — praktisch alle Objekte dort sind materialübergreifend, weil das Verfahrensrecht selbst materialneutral konstruiert ist. Das ist kein Defekt des Schemas, sondern ein Befund über das Recht (horizontales Recht dominiert Produkt-/Verfahrensrecht) — sollte aber dokumentiert werden, damit spätere Auswertungen nicht fälschlich C statt B als Gliederungsachse für Feld 1/2 nutzen.

**Vorschlag:** neuer C-Wert **„Verbund-/Systembauteil (mehrere Materialfamilien in einer nachweispflichtigen Einheit)"** — abzugrenzen von „materialübergreifend" (= die Norm gilt für alle Materialien gleichermaßen, regelt aber jeweils Einzelmaterial-Sachverhalte) durch die Regel: **„materialübergreifend" gilt, wenn die Norm einzelne Materialien anspricht, ohne sie zu differenzieren** (z. B. CPR gilt für Stahlprodukte, Holzprodukte, Glasprodukte je einzeln und regelt für jedes dieselben Pflichten); **„Verbund-/Systembauteil" gilt, wenn der Nachweisgegenstand selbst ein zusammengesetztes Objekt aus mehreren Materialfamilien ist**, das nicht sinnvoll in seine Materialbestandteile zerlegt bewertet werden kann (z. B. eine wiederzuverwendende Fenstereinheit als Ganzes). Mehrfachauswahl bleibt zulässig (z. B. C = Baustahl + Verbund-/Systembauteil, wenn eine Norm sowohl reine Stahlbauteile als auch gemischte Elemente mit Stahlanteil regelt).

**Abgrenzungsregel C:** Ein Objekt zählt als **Verbund-/Systembauteil**, wenn der Rechtstext selbst das zusammengesetzte Bauteil (nicht dessen Einzelmaterialien) als Bezugsobjekt der Pflicht benennt. Ein Objekt zählt NICHT als Verbund-/Systembauteil, sondern bleibt „materialübergreifend", wenn der Text zwar für verschiedene Materialien gleichermaßen gilt, den Nachweis aber pro Einzelmaterial verlangt (GewAbfV § 8: zehn getrennte Fraktionen — genau das Gegenteil eines Verbundbauteil-Ansatzes, obwohl auf den ersten Blick „viele Materialien" betroffen sind).

---

## 4. Achse D — Rechtsform

Mit fünf unabhängig aufgeworfenen Lücken (Rechtsprechung, unverbindliches Muster-/Modellrecht, TRGS-artige Vermutungswirkungsregeln, DIN SPEC, Eurocode/CEN-Bemessungsnorm) ist D die am stärksten unterversorgte Achse des Piloten. Alle drei Piloten trugen unabhängig Lücken bei — stärkstes Konvergenzsignal im gesamten Stresstest.

**Problem 1 (produkt, REG-DE-1-011):** Kein Wert für Rechtsprechung. Das EuGH-Vertragsverletzungsurteil C-100/13 hat im Pilot die größte unmittelbare Wirkung auf das Ü-Zeichen-System — und passt in keinen der zehn Werte (kein Erlass einer Behörde = keine VV; kein Parlamentsakt = kein Gesetz).

**Problem 2 (produkt, REG-DE-2-009/REG-DE-1-010):** Kein Wert für unverbindliches Mustertext mit Verbindlichkeit erst durch Drittakt. MBO und MVV TB sind „Muster ohne Rechtskraft" (explizite Falle der Aufgabenstellung) — weder „Verwaltungsvorschrift" (kein Behördenerlass mit Außenwirkung) noch „Merkblatt" (kein bundesweiter Quasi-Standardisierungsanspruch mit 16-facher Verbindlichmachung als Zweck) trifft es.

**Problem 3 (abfall, REG-DE-4-003):** Kein Wert für TRGS/TRBS/TRBA-artige Regelwerke mit Vermutungswirkung. TRGS 519 wird von einem hoheitlich besetzten Ausschuss erarbeitet und per GMBl bekannt gegeben; sie entfaltet über § 7 Abs. 2 GefStoffV eine Vermutungswirkung gegenüber Privaten — mechanisch näher an einer Techn. Baubestimmung (Bezugnahme macht eine private Norm bindend) als an einer internen Verwaltungsvorschrift, aber ohne den Umweg über eine DIN-Norm.

**Problem 4 (zie, REG-DE-2-005):** Kein Wert für DIN SPEC. Durchläuft ein verkürztes Konsensverfahren (PAS-artig nach DIN 820), kein vollständiges Normungsverfahren — Einordnung zwischen „nat.Norm" und „Merkblatt" ist Verlegenheitslösung.

**Problem 5 (zie, REG-DE-2-007):** Kein Wert für Eurocodes/CEN-Bemessungsnormen. „hEN" ist im Schema an die CPR-Logik gebunden (harmonisierte Produktnorm mit CE-Vermutungswirkung); Eurocodes sind CEN-erarbeitete Bemessungsnormen ohne CE-Bezug.

**Vorschlag:** D wird um vier neue Werte erweitert (Rangfolge bleibt ordinal, neue Werte werden zwischen den bestehenden zehn eingefügt, keine Neuordnung der bestehenden Werte nötig):

| Neuer D-Wert | Abgrenzung „gehört dazu" | Abgrenzung „zählt NICHT dazu" |
|---|---|---|
| **Rechtsprechung/Urteil** | EuGH-, BVerfG-, BVerwG-Entscheidungen mit unmittelbarer normprägender Wirkung (C-100/13) | Eine Behörden-Bekanntmachung, die ein Urteil bloß vollzieht (DIBt-Bekanntmachung 2019/1 zur Aufhebung der Bauregellisten) — die bleibt separates Objekt, D = Verwaltungsvorschrift, mit Relation „setzt um" zum Urteil |
| **Muster-/Modellrecht (unverbindlich, Umsetzung durch Dritte erforderlich)** | MBO, MVV TB — bundesweit abgestimmte Vorlagen ohne eigene Rechtskraft, deren *Zweck* die identische Übernahme durch 16 Rechtsträger ist | Ein einzelnes Landesgesetz, das MBO-Wortlaut übernimmt (NBauO) — das ist D = Gesetz, keine Musterrechts-Kodierung mehr, sobald der Transformationsakt vollzogen ist |
| **Techn. Regel mit Vermutungswirkung (TRGS/TRBS/TRBA-Typ)** | Regelwerke eines hoheitlich besetzten Fachausschusses mit Bekanntgabe im GMBl und gesetzlich verankerter Vermutungswirkung (§ 7 Abs. 2 GefStoffV) | DIN-Normen, die erst über VV-TB-Bezugnahme bindend werden (klassische „nat.Norm"-Bindungskette) — dort fehlt die direkte gesetzliche Vermutungswirkungsnorm, die TRGS/TRBS/TRBA auszeichnet |
| **Eurocode/CEN-Bemessungsnorm** | CEN-erarbeitete Tragwerksbemessungsnormen (EN 1990 ff.), unabhängig vom Umsetzungsstadium (Entwurf/Endfassung/mit-ohne-NA) | CPR-harmonisierte Produktnormen (hEN) — die bleiben unter „hEN", auch wenn beide CEN-Ursprungs sind; Unterscheidungskriterium ist der Regelungsgegenstand (Bemessung vs. Produktkonformität), nicht die Erarbeitungsinstitution |

**DIN SPEC** erhält keinen eigenen Vollwert, sondern einen **Modifikator** `nat.Norm (reduziertes Konsensverfahren/DIN SPEC)` — das Verfahren ist eine Untervariante der nationalen Normung, kein eigenständiger Rechtsformtyp; die Modifikator-Lösung vermeidet Vokabular-Inflation, macht den Unterschied aber sichtbar (relevant u. a. für die Bindungsketten-Prüfung, da DIN SPECs seltener in VV-TB gelistet sind als reguläre DIN-Normen).

**Umgang mit Mehrfachwerten:** D bleibt grundsätzlich einwertig (Rechtsform ist im Unterschied zu B kein Bündelmerkmal). Ausnahme: Objekte mit zwei Rechtsakten unterschiedlicher Form im selben Block (z. B. REG-DE-1-011: EuGH-Urteil + DIBt-Bekanntmachung) werden — wie im Pilot bereits praktiziert — als zwei separate Regelungsobjekte mit Relation „setzt um" geführt, statt D mehrwertig zu machen. Diese bereits gelebte Pilot-Praxis wird als Regel festgeschrieben.

---

## 5. Achse E — Prozessphase

**Problem 1 (produkt, konvergent mit abfall):** E modelliert fließende Rechtsbegriffe als disjunkte Container. „Vorbereitung zur Wiederverwendung" (Art. 3 Nr. 16 AbfRRL) markiert exakt den Übergang zwischen „Abfallstatus" und „Aufbereitung/Prüfung" — sie IST die Grenzoperation, kein Punkt davor oder danach.

**Problem 2 (abfall, REG-DE-3-001):** Der zentrale Rechtshebel für Bauteil-Wiederverwendung besteht gerade darin, die Phase „Abfallstatus" NICHT zu durchlaufen (KrWG § 3 Abs. 21: Wiederverwendung ist nur bei Nicht-Abfällen definiert). Eine Phasenliste, die Abfallstatus als verpflichtende Zwischenstation zwischen Rückbau und Aufbereitung führt, bildet den bevorzugten — und rechtlich entscheidenden — Pfad nicht ab.

**Problem 3 (zie, Beobachtung ohne Korrekturbedarf):** horizontale Ermöglichungsnormen decken routinemäßig 2–5 von 8 Phasen gleichzeitig ab, was die Trennschärfe der Achse für gerade die wichtigsten Objekte reduziert.

**Vorschlag 1:** Zwei Phasen erhalten einen **Status-Zusatz** statt einer neuen Phase: „Abfallstatus" wird als `Abfallstatus (kann rechtlich vermieden/übersprungen werden)` gekennzeichnet — mit Pflichthinweis im Objekt, wenn eine Norm ihre Wirkung gerade aus dem Nicht-Erreichen dieser Phase zieht (Marker **„Phase bewusst vermieden"** im E-Feld, zusätzlich zu den durchlaufenen Phasen).

**Vorschlag 2:** „Vorbereitung zur Wiederverwendung" wird nicht als eigene Phase geführt (das würde die Acht-Phasen-Struktur aufbrechen), sondern erhält eine **Kodierregel**: Normen, die diesen Begriff regeln, werden mit E = `Abfallstatus, Aufbereitung/Prüfung` (beide Grenz-Phasen) plus Freitext-Vermerk „Grenzoperation zwischen Abfallstatus und Aufbereitung/Prüfung" kodiert — kein neuer Wert, aber eine verbindliche Doppel-Kodierregel für Grenzbegriffe.

**Vorschlag 3 (Auswertungshinweis, kein Schemaeingriff):** Für Objekte mit hoher E-Phasenzahl (≥4) empfiehlt sich in W2/W4, E als Nebenachse für Netzwerkdarstellung statt als Filterachse zu nutzen, da die Trennschärfe dort gering ist. Keine Vokabularänderung nötig.

**Abgrenzungsregel E:** Eine Phase gilt als **durchlaufen**, wenn der Normtext eine Handlung/Pflicht in dieser Phase explizit adressiert. Eine Phase gilt NICHT als durchlaufen, nur weil sie kausal vorgelagert ist — Beispiel: CPR 2024/3110 Art. 3 Nr. 20 setzt denklogisch einen vorangegangenen Ausbau voraus, regelt aber keine Rückbau/Sicherung-Pflichten; REG-EU-1-001 erhält E = Inverkehrbringen (+ Bestandserkundung „mittelbar", ausdrücklich als Sonderfall markiert), NICHT E = Rückbau/Sicherung.

---

## 6. Achse F1/F2 — Wirkrichtung

Die am häufigsten beanstandete Achse (alle drei Piloten, mit drei unterschiedlichen, sich ergänzenden Detailproblemen).

**Problem 1 (produkt, REG-EU-1-006):** Zeitschichtung durch Übergangsrecht. Für dasselbe materielle Thema (DoP-Pflicht bei Gebrauchtprodukten) gilt je nach Produktfamilie entweder das alte, gebraucht-blinde VO-305/2011-Regime oder das neue, explizite VO-2024/3110-Regime — parallel, bis in die 2030er Jahre. Ein einzelner F1-Wert verdeckt, dass zum Stichtag für die meisten Produktfamilien faktisch noch die alte Rechtslage greift.

**Problem 2 (abfall, REG-DE-3-003/004):** Objektbezug-Ambiguität. Dieselbe Norm ist gegenüber Materialströmen bedingend-ermöglichend und gegenüber ganzen Bauteilen schlicht nicht existent — kein „schweigend" im Sinn von „regelt eine Frage nicht, obwohl sie könnte", sondern „das Bezugsobjekt Bauteil kommt im Tatbestand gar nicht vor".

**Problem 3 (zie, REG-DE-2-001/002):** Doppelnatur im selben Text. ZiE/vBG ermöglichen Reuse explizit (Zulassungsweg wird überhaupt erst eröffnet), begrenzen sie aber durch Konstruktion als Einzelfall-/vorhabenbezogenes Instrument zugleich strukturell (keine Übertragbarkeit auf Folgeprojekte). Das ist kein Auseinanderfallen von Rechtslage und Praxiswirkung (dafür sind F1/F2 gemacht), sondern eine im Rechtstext selbst angelegte Doppelwirkung auf zwei verschiedene Bezugsgegenstände.

**Vorschlag — ein gemeinsamer Mechanismus für alle drei Probleme:** F1/F2 bleiben im Kern zweiwertig (Rechtslage/Praxiswirkung), erhalten aber ein optionales **Bezugsgegenstand-Feld**, das bei Bedarf mehrere F1/F2-Paare pro Objekt zulässt, jedes mit eigenem Bezugsgegenstand-Label:
- Zeitschichtung (Problem 1): Bezugsgegenstand = `Produktfamilie unter altem Regime` vs. `Produktfamilie unter neuem Regime`, mit Stichtags-/Bedingungsangabe.
- Objektbezug (Problem 2): Bezugsgegenstand = `Materialstrom` vs. `ganzes Bauteil`.
- Doppelnatur (Problem 3): Bezugsgegenstand = `Zulassungsfähigkeit dem Grunde nach` vs. `Skalierbarkeit/Übertragbarkeit der Zulassung`.

Ein einzelnes F1/F2-Paar ohne Bezugsgegenstand-Angabe bleibt der Normalfall (Default, keine Mehrarbeit für die Mehrzahl der Objekte); das Zusatzfeld wird nur gesetzt, wenn ein Objekt nachweislich mehr als eine Wirkrichtung für unterscheidbare Teilaspekte hat.

**Abgrenzungsregel F1/F2:** Mehrere F1/F2-Paare sind angezeigt, wenn der Normtext selbst unterscheidbare Teilaspekte mit gegenläufiger oder unterschiedlich bedingter Wirkung regelt (Art. 26 Abs. 2 CPR wirkt anders auf hEN-erfasste als auf nicht-hEN-erfasste gebrauchte Produkte — das wäre ein Kandidat, im Pilot aber noch nicht so kodiert, da im vorliegenden Text nicht F1/F2-different ausgewertet). Mehrere Paare sind NICHT angezeigt, wenn lediglich Rechtslage (F1) und Praxiswirkung (F2) auseinanderfallen — das ist der Regelfall, für den die bestehende Zweiwertigkeit exakt gemacht ist, und bleibt unverändert ein Paar.

---

## 7. Achse G — Nachweisanforderung

**Problem 1 (produkt, REG-EU-1-003/004, konvergent mit abfall Problem 5):** G unterstellt, dass jede Norm einen Nachweistatbestand hat. Reine Anwendbarkeits-/Scope-Normen (CPR Art. 20 Abs. 1: WER fällt überhaupt unter das Regime; Erwägungsgrund 34: WELCHE Konstellation gilt nicht als Inverkehrbringen) legen fest, OB ein Regime greift, nicht WAS nachzuweisen ist — beide gehören zu den praktisch wichtigsten Fundstellen ihres Piloten und müssen notdürftig als „entfällt" markiert werden.

**Problem 2 (abfall, REG-DE-3-001, unabhängig konvergent):** Für den Nachweis, dass ein Regime NICHT eröffnet ist (kein Entledigungswille i. S. KrWG § 3 Abs. 21), gibt es keinen der sieben G-Werte — das ist kategorial etwas anderes als ein Konformitätsnachweis (Statusfeststellung vs. Konformitätsnachweis).

Diese beiden Befunde sind dieselbe Problemklasse und werden zusammengeführt.

**Vorschlag:** achter G-Wert **„Anwendbarkeits-/Statusprüfung (kein Konformitätsnachweis; stellt fest, OB ein Regime greift, nicht WAS nachzuweisen ist)"**. Deckt beide Pilot-Fälle ab: Scope-Normen (produkt) ebenso wie Statusfeststellungsnormen (abfall).

**Abgrenzungsregel G (neuer Wert):** Ein Nachweistatbestand zählt als **Anwendbarkeits-/Statusprüfung**, wenn der Prüfgegenstand die Zugehörigkeit zu einem Regime selbst ist (z. B.: Ist das Produkt hEN-erfasst? Liegt Entledigungswille vor?). Er zählt NICHT dazu, sondern bleibt z. B. „Dokumentenlage" oder „rechnerischer Nachweis", wenn die Zugehörigkeit zum Regime bereits feststeht und lediglich die inhaltliche Erfüllung einer Anforderung innerhalb des Regimes zu belegen ist (z. B. Leistungserklärung nach Art. 15 CPR für ein bereits als hEN-erfasst identifiziertes Produkt).

**Problem 3 (zie, REG-DE-2-010):** G bildet keine Stufenfolgen/Eskalationslogik ab. Reuse-Nachweise sind praktisch nie ein einzelner G-Typ, sondern eine bedingte Kaskade (Dokumentenlage → ggf. Sichtprüfung → ggf. zerstörungsfreie Prüfung → ggf. Probenahme/Materialprüfung → rechnerischer Nachweis → Einzelfallzulassung), bei der jede Stufe nur bei Auffälligkeiten der vorherigen ausgelöst wird — eine flache Werteliste verliert diese Sequenz- und Bedingtheitsinformation.

**Vorschlag:** G wird als **geordnete, mit Bedingtheit annotierte Liste** statt als ungeordnetes Set geführt, wo eine Norm explizit eine Stufenfolge vorsieht: `G: [1] Dokumentenlage (immer) → [2] Sichtprüfung (bei Auffälligkeit aus [1]) → [3] zerstörungsfreie Prüfung (bei Auffälligkeit aus [2]) → …`. Für die Mehrzahl der Objekte (kein Stufenverfahren) bleibt G ein einfaches, ungeordnetes Set — die Sequenzsyntax ist optional und nur bei belegter Kaskadenlogik zu nutzen (Regelfall bleibt unverändert).

**Umgang mit Mehrfachwerten (Bestandsregel bestätigt):** G war im Pilot bereits faktisch mehrwertig kodiert (z. B. REG-DE-3-004: „Dokumentenlage / Erklärung Dritter / Darlegung techn. Unmöglichkeit"). Diese Praxis wird als reguläre Mehrfachauswahl bestätigt und nicht geändert — nur die neue Stufenfolge-Syntax kommt als optionale Notation hinzu, wenn Reihenfolge/Bedingtheit belegt ist.

---

## 8. Relationen-Vokabular

Bislang: setzt um | ersetzt | konkretisiert | kollidiert mit. Zwei Lücken, beide mit konkretem Pilot-Beleg:

**Fehlt 1 (abfall, REG-DE-3-004→003):** „verdrängt (lex specialis)". § 8 Abs. 1a GewAbfV verweist für bestimmte Stoffgruppen „ausschließlich" auf § 24 EBV — das ist weder „ersetzt" (GewAbfV bleibt für andere Stoffe in Kraft) noch „konkretisiert" (EBV wiederholt GewAbfV nicht, sondern schließt sie für den Teilbereich aus) noch „kollidiert mit" (keine widersprüchliche, sondern eine geregelte Vorrangbeziehung).

**Fehlt 2 (zie, REG-DE-2-001↔002):** „wird kombiniert mit / ergänzt (parallele Verfahrensinstrumente für denselben Anwendungsfall)". ZiE (Bauprodukt-Ebene) und vBG (Bauart-Ebene) sind rechtlich getrennte Instrumente für getrennte Regelungsgegenstände, die bei Reuse-Bauteilen typischerweise gemeinsam beantragt werden (primärquellenbelegt durch den BW-Leitfaden REG-DE-2-010) — keine der vier Bestandsrelationen trifft diese Parallelbeziehung ohne Normkollision.

**Vorschlag:** Relationen-Vokabular wird um zwei Werte ergänzt:
- **„verdrängt (lex specialis)"** — für geregelte Vorrangbeziehungen zwischen zwei gleichrangigen Instrumenten (i. d. R. beide RVO oder beide Gesetz), bei der die speziellere Norm die allgemeinere für einen Teilbereich verdrängt, OHNE sie insgesamt zu ersetzen. Abgrenzung zu „ersetzt": „ersetzt" meint vollständige Ablösung des Vorgänger-Instruments (VO 2024/3110 ersetzt VO 305/2011); „verdrängt" meint Teilbereichs-Vorrang bei fortbestehender Grundnorm.
- **„wird kombiniert mit / ergänzt"** — für zwei oder mehr Instrumente, die denselben Anwendungsfall parallel und ergänzend regeln, ohne dass eines das andere voraussetzt, ersetzt oder verdrängt (bidirektionale Relation).

**Umgang mit Mehrfachwerten:** Relationen waren im Pilot bereits mehrwertig pro Objekt (mehrere Ziel-IDs mit unterschiedlichen Relationstypen im selben Block). Das bleibt unverändert Standard.

---

## 9. Beleg-/Zugänglichkeitsregeln (Meta-Achse, keine der sieben A–G, aber im Pilot wiederholt an ihre Grenze gebracht)

**Problem 1 (abfall, REG-DE-4-003, TRGS 519):** Das Zugänglichkeits-Vokabular kennt nur `frei-primär` / `paywalled-eingesehen` / `paywalled-nicht-eingesehen`. TRGS 519 ist rechtlich frei zugänglich, aber durch aktiven Bot-Schutz (JS-Challenge, HTTP 403) technisch nicht abrufbar — kein Paywall-Fall, aber auch kein „frei-primär" im Sinn von tatsächlich eingesehen. Sie wurde vorsorglich wie eine nicht eingesehene Paywall-Quelle behandelt (B2/Konfidenz „unklar"), was den entscheidenden Unterschied verdeckt: Bei echten Paywall-Fällen braucht es einen alternativen freien Bindungsakt; hier reicht ein erneuter Zugriffsversuch.

**Vorschlag:** vierter Zugänglichkeitswert **„frei-primär-blockiert (technischer Zugriffsfehler, kein Paywall-Grund)"**. Kodierregel: B0/B1-Status kann trotzdem NICHT vergeben werden, solange der Volltext nicht tatsächlich gelesen wurde (Belegregel bleibt hart) — aber der Grund wird von echten Paywall-Fällen unterschieden, damit W1/W2 wissen, ob ein Bindungsakt-Ersatz zu suchen ist (Paywall) oder nur ein erneuter Abrufversuch über einen anderen Kanal nötig ist (Blockade).

**Problem 2 (zie, REG-DE-2-005/006/007, DIN SPEC/ISO 13822/EN 1990-2):** Die Bindungsketten-Regel verlangt, den freien amtlichen Akt zu nennen, der eine kostenpflichtige Norm bindend macht. Für alle drei genannten Normen konnte im Pilot nur festgestellt werden, DASS ein Bindungsmechanismus (VV TB) existiert, nicht OB die jeweilige Norm dort tatsächlich gelistet ist — ein Zwischenzustand, den das Schema aktuell nicht von „kein Bindungsakt" unterscheidet.

**Vorschlag:** Bindungsakt-Feld erhält einen dritten möglichen Zustand neben „benannt" und „entfällt/keiner identifiziert": **„Bindungsmechanismus existiert, Listung im Einzelfall nicht verifiziert"** — mit Pflichtangabe, welche 16-Länder-VV-TB-Prüfung dafür noch aussteht. Verhindert, dass eine unverifizierte Listung stillschweigend als „kein Bindungsakt" (= unverbindlich) fehlinterpretiert wird, obwohl der Mechanismus nachweislich existiert und nur die Einzelprüfung fehlt.

**Sonderfall E1/E2/E3 vs. G-explizit/-inferiert (produkt, Punkt 9):** Beide Kennzeichnungen kodieren im Kern dieselbe Unterscheidung („steht das im Text oder wird es zugeordnet"), werden aber redundant parallel gepflegt. Für den Freeze wird empfohlen, **G-explizit/-inferiert als Unterfall von E1/E3 zu behandeln** statt als eigenes Tag-Paar zu pflegen — d. h. „G-explizit" entfällt als separates Label, sobald das übergeordnete Objekt bereits E1 trägt; „G-inferiert" entfällt entsprechend unter E3. Nur wenn G innerhalb eines ansonsten E1-eingestuften Objekts abweichend inferiert wurde (Mischfall, im Pilot z. B. REG-DE-2-009: Gesamtobjekt E2/E3, aber G-Wert selbst E1-explizit), bleibt eine abweichende G-Kennzeichnung nötig — dann aber als Ausnahmevermerk, nicht als Regelfall-Doppelpflege.

---

## 10. Priorisierung für den Freeze

Nach Konvergenz (Anzahl unabhängiger Piloten, die denselben Befund aufwarfen) und praktischer Tragweite geordnet:

**Hohe Priorität (in ≥2 Piloten unabhängig aufgeworfen, hohe Auswertungsrelevanz):**
1. D-Achse: vier neue Werte (Rechtsprechung, Muster-/Modellrecht, Techn. Regel mit Vermutungswirkung, Eurocode/CEN) + DIN-SPEC-Modifikator — höchste Konvergenz (alle drei Piloten).
2. G-Achse: achter Wert „Anwendbarkeits-/Statusprüfung" — unabhängig in produkt und abfall aufgeworfen, betrifft praktisch die wichtigsten Scope-Normen des gesamten Datensatzes.
3. F1/F2: optionales Bezugsgegenstand-Feld für Mehrfachpaare — in allen drei Piloten mit unterschiedlicher Begründung, aber gemeinsamem Lösungsmuster gefordert.
4. B-Achse: Primärfeld + Nebenfelder (Mehrfachauswahl) — in zwei Piloten (produkt, abfall) belegt.

**Mittlere Priorität (in einem Piloten belegt, aber strukturell wichtig für den weiteren Harvest):**
5. A-Achse: A-Ursprung-Attribut (Erarbeitungs- vs. Bindungsebene) + Downstream-Verifikationsstatus-Pflichtfeld.
6. Relationen: „verdrängt (lex specialis)" und „wird kombiniert mit/ergänzt".
7. C-Achse: „Verbund-/Systembauteil"-Wert.
8. Zugänglichkeit: „frei-primär-blockiert" + Bindungsakt-Zwischenzustand.

**Niedrige Priorität (Beobachtung/Auswertungshinweis, kein zwingender Schemaeingriff vor Freeze):**
9. E-Achse: „Phase bewusst vermieden"-Marker und Doppelkodierregel für Grenzbegriffe — sinnvoll, aber ohne Dringlichkeit, da im Freitextfeld bereits kommunizierbar.
10. G-Achse: Stufenfolge-Syntax für Kaskaden — Komfortfunktion, keine Blockade für W1.
11. E1/E3-G-explizit-Redundanz — Pflegeaufwand, keine inhaltliche Lücke.
12. Sub-Ebene-Beschaffungsrisiko (403/404 bei Landesportalen) — kein Schema-, sondern Kapazitäts-/Zeitplanungsproblem für W2; keine Vokabularänderung nötig, aber Empfehlung an die W2-Zeitplanung weiterzugeben.

**Nicht bestätigte bzw. entkräftete Kritikpunkte:** Keiner der 27 Stresstest-Befunde stellte die Grundstruktur (sieben Achsen, Blockformat, ID-Schema, E1–E3/B0–B4) infrage; alle sind additive Vokabular- oder Mehrfachwert-Erweiterungen. Das bestätigt die Eignung des Schemas für den Freeze, sofern mindestens die vier hochpriorisierten Punkte vor W1 entschieden werden.
