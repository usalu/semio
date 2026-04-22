# ZRS – CRCLR House / Reuse-basierter Entwurf und Tragwerksintegration

Tags: #design #engineering #reuse #interview-source #priority-high

---

## Einordnung im Forschungskontext

Das Projekt **CRCLR House** stellt einen der zentralen Referenzfälle dar, in dem **wiederverwendete Bauteile nicht nur technisch integriert, sondern entwurfsprägend eingesetzt werden**.  
Es wird ein Planungsansatz sichtbar, bei dem **Bestand, Tragwerk und Architektur simultan entwickelt** werden.

Im Kontext der Forschung ist das Projekt besonders relevant für:

- Integration von **diskreten Bestandsbauteilen in frühe Entwurfsphasen**
- Wechselwirkung zwischen **Tragwerkslogik und Gestaltung**
- Umgang mit **Unsicherheit und Variabilität von Bauteilen**
- Anforderungen an digitale Werkzeuge für **reuse-basiertes Entwerfen**

---

## Relevante Beispiele

### 1. CRCLR House Projektseite
https://www.zrs.berlin/project/crclr-house/

**Relevanz für das Projekt**  
Es wird ein Gebäude beschrieben, das auf einem **Bestandsbau aufsetzt und diesen erweitert**, wobei wiederverwendete Bauteile – insbesondere Stahl – gezielt integriert werden.

**Beitrag zur Forschung**  
Es wird sichtbar, dass Wiederverwendung nicht als nachgelagerte Entscheidung erfolgt, sondern **früh in die Entwurfslogik integriert werden muss**.

**Verbindungen**
- [[bim-berlin-bauteilauktion]] → Quelle realer Bauteile
- [[concular]] → vorgelagerte Bestandsanalyse und Audit
- [[zrs-engineering]] → integrierte Tragwerksplanung

---

### 2. Wiederverwendung tragender Stahlbauteile
https://www.zrs.berlin/project/crclr-house/

**Relevanz für das Projekt**  
Es wird gezeigt, dass **tragende Bauteile aus dem Bestand neue strukturelle Funktionen übernehmen**.

**Beitrag zur Forschung**  
Damit wird die zentrale Herausforderung sichtbar:  
Das Tragwerk entsteht nicht aus idealisierten Parametern, sondern aus **vorhandenen, diskreten Elementen**.

**Verbindungen**
- [[be-ware-engineering]] → strukturelle Bewertung wiederverwendeter Systeme
- [[arup-reuse]] → standardisierte Prüf- und Bewertungsansätze
- [[bim-berlin-data-model]] → notwendige Bauteildaten

---

### 3. Zirkulärer Planungsansatz (Holzaufstockung + Bestand)
https://www.zrs.berlin/project/crclr-house/

**Relevanz für das Projekt**  
Das Projekt kombiniert **Bestandsstruktur, neue Holzbauelemente und wiederverwendete Komponenten**.

**Beitrag zur Forschung**  
Es wird deutlich, dass reuse-basierter Entwurf immer eine **Hybridisierung von Systemen** bedeutet.

**Verbindungen**
- [[reallabor-zirkulaeres-bauen-design]] → experimentelle Entwurfsprozesse
- [[madaster]] → Materialpässe für hybride Systeme
- [[concular-supply]] → Verfügbarkeit unterschiedlicher Bauteiltypen

---

## Zentrale Erkenntnisse für die Forschung

- Entwurf mit Reuse beginnt nicht mit Geometrie, sondern mit **verfügbaren Bauteilen**
- Tragwerksplanung verschiebt sich von **Dimensionierung → Selektion und Kombination**
- Hohe Entwurfskomplexität entsteht durch:
  - heterogene Geometrien
  - variierende Zustände
  - begrenzte Datenlage
- Enge Kopplung zwischen:
  - **Architektur**
  - **Tragwerk**
  - **Materialverfügbarkeit**

---

## Ableitung für die Plattformentwicklung

- Es wird ein System benötigt, das **Bauteile aktiv in den Entwurfsprozess einbringt**
- Kombinatorische Komplexität muss durch:
  - Filter
  - Vorschläge
  - KI-gestützte Auswahl  
  reduziert werden
- Tragwerkslogik muss **früh integriert** werden
- Plattform muss ermöglichen:
  - Bauteile **zu kombinieren**
  - Bauteile **zu bewerten**
  - Bauteile **iterativ im Entwurf zu testen**

---

## Interviewfragen

### Entwurf
- Ab wann werden wiederverwendete Bauteile im Entwurf berücksichtigt?
- Wie verändert sich der Entwurfsprozess im Vergleich zum Neubau?

### Tragwerk
- Wie wird mit Unsicherheit in Materialeigenschaften umgegangen?
- Welche Daten sind notwendig, um Bauteile strukturell zu bewerten?

### Daten
- Welche Bauteildaten wären idealerweise verfügbar?
- Welche Informationen fehlen im aktuellen Prozess am meisten?

### Workflow
- Wie erfolgt die Abstimmung zwischen Architektur und Tragwerk?
- Wo entstehen Iterationsschleifen oder Engpässe?

### Digitalisierung
- Welche Tools werden aktuell verwendet?
- Welche Funktionen fehlen für reuse-basiertes Entwerfen?

---

## Relevante Hinweise

- CRCLR House zeigt exemplarisch, dass reuse-basierter Entwurf ein **grundlegend anderer Planungsprozess** ist
- Besonders relevant ist die Verschiebung von:
  - **Top-down Entwurf → Bottom-up Materiallogik**
- Genau diese Transformation ist zentraler Gegenstand der Forschung

---

## Connections

- [[zrs-engineering]]
- [[be-ware-engineering]]
- [[arup-reuse]]
- [[bim-berlin-bauteilauktion]]
- [[bim-berlin-data-model]]
- [[concular]]
- [[concular-urban-mining]]
- [[reallabor-zirkulaeres-bauen-design]]
- [[madaster]]
