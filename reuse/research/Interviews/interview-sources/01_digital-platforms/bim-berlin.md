# BIM Berlin – Bauteilauktion / Reuse-Datenmodell

Tags: #digital-platform #material-supply #urban-mining #public-sector #reuse #interview-source #priority-high

---

## Einordnung im Forschungskontext

Die Bauteilauktion der BIM Berlin stellt einen der wenigen realen Anwendungsfälle dar, in dem **öffentliche Bestände systematisch in wiederverwendbare Bauteile überführt und digital zugänglich gemacht werden**.  
Sie bildet damit eine zentrale Schnittstelle zwischen **Bestand, Rückbau, Materialangebot und digitaler Darstellung von Bauteilen**.

Im Kontext der Forschung wird sie als Referenz für folgende Fragestellungen genutzt:

- Wie werden Bauteile heute **digital beschrieben und vermittelt**?
- Welche Informationen reichen für Handel – und welche fehlen für **Entwurf und Analyse**?
- Wo liegt die Grenze zwischen **Marktplatz** und **Planungstool**?

---

## Relevante Beispiele

### 1. Bauteilauktion Plattform
https://www.bim-berlin.de/bauteilauktion

**Relevanz für das Projekt**  
Es wird ein realer Marktplatz für wiederverwendbare Bauteile bereitgestellt, der zeigt, wie **Materialverfügbarkeit sichtbar gemacht wird**.  
Dabei wird deutlich, dass die Plattform primär auf **Vermittlung und Verkauf** ausgerichtet ist, nicht auf Planung.

**Beitrag zur Forschung**  
Es wird sichtbar, welche **Minimalanforderungen an Bauteildaten** derzeit existieren und welche Informationen für Entwurfsprozesse fehlen.

**Verbindungen**
- [[concular]] → erweitert Marktplatzlogik um Bewertung und Planung
- [[kunst-stoffe-berlin]] → zeigt informelle Materialmärkte im Vergleich
- [[bim-berlin-reuse-pilots]] → Herkunft der Bauteile aus Bestand

---

### 2. Bauteildetailseiten / Datenstruktur
https://www.bim-berlin.de/bauteilauktion/bauteile-details

**Relevanz für das Projekt**  
Die einzelnen Bauteilseiten zeigen konkret, welche **Metadaten aktuell erfasst werden**: Geometrie, Material, Zustand, Verfügbarkeit.

**Beitrag zur Forschung**  
Es wird ein implizites Datenmodell sichtbar, das als Ausgangspunkt für die Definition eines **maschinenlesbaren Bauteilkatalogs** genutzt werden kann.

**Verbindungen**
- [[madaster]] → langfristige Materialpässe und Datenpersistenz
- [[zrs-crclr-house]] → Anwendung solcher Daten im Entwurf
- [[be-ware]] → Ergänzung um technische Prüf- und Zustandsdaten

---

### 3. Reuse-Pilot: Wasserrettungsstation Friedrichshagen
https://www.bim-berlin.de/landesimmobilien/projekte-und-news/reuse-pilot-wasserrettungsstation-friedrichshagen

**Relevanz für das Projekt**  
Es wird gezeigt, wie Bauteile aus einem konkreten Gebäude identifiziert, ausgebaut und der Wiederverwendung zugeführt werden.

**Beitrag zur Forschung**  
Die Prozesskette von **Bestand → Rückbau → Plattform → Wiederverwendung** wird nachvollziehbar.  
Damit wird die Schnittstelle zwischen **Urban Mining und digitalem Katalog** sichtbar.

**Verbindungen**
- [[concular]] → systematisierte Bestandsaufnahme
- [[haus-der-materialisierung]] → lokale Kreislaufstrukturen

---

## Zentrale Erkenntnisse für die Forschung

- Bauteile werden aktuell primär als **Handelsobjekte**, nicht als **Planungselemente** beschrieben  
- Metadaten sind vorhanden, aber **nicht ausreichend für Entwurf, Tragwerk oder LCA**  
- Es fehlt eine direkte Verbindung zwischen:
  - **Bauteilkatalog**
  - **Entwurfswerkzeug**
  - **Performancebewertung**

→ Genau an dieser Schnittstelle setzt das Forschungsvorhaben an.

---

## Ableitung für die Plattformentwicklung

Aus den Beispielen wird abgeleitet:

- Es wird ein **erweitertes Bauteildatenmodell** benötigt
- Daten müssen **maschinenlesbar und direkt entwurfsfähig** sein
- Plattformen müssen:
  - Material **finden**
  - Material **bewerten**
  - Material **in Entwurf integrieren**

---

## Interviewfragen

### Daten & Struktur
- Welche Bauteildaten werden aktuell standardmäßig erfasst?
- Welche Informationen fehlen aus Sicht der Planenden?
- Welche Daten sind schwer oder gar nicht zuverlässig zu erheben?

### Prozess
- Wie erfolgt die Auswahl von Bauteilen für die Auktion?
- Wie wird der Zustand bewertet und dokumentiert?
- Wo entstehen die größten Informationsverluste im Prozess?

### Nutzung
- Wer nutzt die Plattform tatsächlich und in welchem Kontext?
- Werden Bauteile bereits im Entwurf berücksichtigt oder erst später?

### Digitalisierung
- Gibt es Schnittstellen zu CAD/BIM-Systemen?
- Welche Anforderungen bestehen an eine Integration in Entwurfssoftware?

---

## Relevante Hinweise

- Die Plattform bildet eine **real existierende Datenbasis**, die für die Forschung direkt nutzbar ist  
- Gleichzeitig zeigt sie die **Grenzen bestehender Systeme** sehr deutlich  
- Besonders relevant ist die Differenz zwischen:
  - **verfügbaren Daten**
  - **benötigten Daten im Entwurf**

---

## Connections

- [[concular]]
- [[madaster]]
- [[zrs-crclr-house]]
- [[be-ware]]
- [[haus-der-materialisierung]]
- [[kunst-stoffe-berlin]]
- [[bim-berlin-reuse-pilots]]