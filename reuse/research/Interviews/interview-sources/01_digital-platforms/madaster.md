# Madaster – Materialpass / Digitale Ressourcen- und Datenlogik

Tags: #digital-platform #data-model #material-passport #reuse #interview-source #priority-high

---

## Einordnung im Forschungskontext

Madaster stellt eine zentrale Referenz für die Frage dar, wie **Materialien und Bauteile langfristig digital erfasst, dokumentiert und rückverfolgbar gemacht werden können**.  
Im Unterschied zu Marktplätzen oder Urban-Mining-Akteuren liegt der Fokus hier auf der **Persistenz von Daten über den gesamten Lebenszyklus eines Gebäudes**.

Im Kontext der Forschung wird Madaster insbesondere für folgende Fragestellungen herangezogen:

- Wie wird ein **Materialpass** strukturiert aufgebaut?
- Welche Daten müssen gespeichert werden, damit Bauteile **zukünftig wiederverwendbar** sind?
- Wie kann ein digitales System Materialien als **Ressourcen statt als Abfall** abbilden?
- Wie lässt sich diese Logik mit **Entwurfsprozessen** verbinden?

---

## Relevante Beispiele

### 1. Madaster Plattform
https://madaster.com/

**Relevanz für das Projekt**  
Es wird ein System bereitgestellt, in dem Gebäude als **Materialbanken** verstanden und digital dokumentiert werden.

**Beitrag zur Forschung**  
Es wird gezeigt, wie Materialien über den Lebenszyklus hinweg **identifizierbar und quantifizierbar** gemacht werden können.

**Verbindungen**
- [[bim-berlin-data-model]] → aktuelle, kurzfristige Bauteildaten
- [[concular]] → operative Nutzung von Bestandsdaten
- [[zrs-crclr-house]] → Anwendung im Entwurf

---

### 2. Materialpass / Cadastre for Materials
https://madaster.com/inspiration/madaster-cadastre-for-materials-from-now-on-available/

**Relevanz für das Projekt**  
Der Materialpass definiert, welche Informationen notwendig sind, um Materialien langfristig **nutzbar und dokumentiert** zu halten.

**Beitrag zur Forschung**  
Er liefert eine Referenz für die Entwicklung eines **standardisierten Datenmodells** für wiederverwendbare Bauteile.

**Verbindungen**
- [[concular-urban-mining]] → Datenerhebung im Bestand
- [[be-ware-engineering]] → technische Ergänzung um Zustandsdaten
- [[arup-reuse]] → strukturierte Bewertungslogik

---

### 3. Digitale Gebäuderessourcen / Datenpersistenz
https://madaster.com/

**Relevanz für das Projekt**  
Es wird ein Ansatz verfolgt, bei dem Materialien nicht nur projektbezogen, sondern **über mehrere Lebenszyklen hinweg gespeichert** werden.

**Beitrag zur Forschung**  
Dies adressiert die Frage, wie Bauteile nicht nur einmal, sondern **dauerhaft in digitalen Systemen verfügbar bleiben**.

**Verbindungen**
- [[bim-berlin-bauteilauktion]] → kurzfristige Verfügbarkeit vs. langfristige Dokumentation
- [[concular-supply]] → Übergang von Daten zu Nutzung
- [[haus-der-materialisierung]] → reale Materialkreisläufe

---

## Zentrale Erkenntnisse für die Forschung

- Materialien müssen als **persistente Dateneinheiten** gedacht werden
- Reuse erfordert:
  - Identifizierbarkeit
  - Dokumentation
  - Standardisierung
- Es besteht eine Lücke zwischen:
  - **langfristiger Dokumentation (Madaster)**
  - **kurzfristiger Nutzung im Entwurf**

---

## Ableitung für die Plattformentwicklung

- Es wird ein hybrides System benötigt:
  - **Materialpass (langfristig)**
  - **Bauteilkatalog (kurzfristig, entwurfsnah)**
- Daten müssen:
  - standardisiert
  - maschinenlesbar
  - kombinierbar sein
- Verbindung erforderlich zwischen:
  - Materialdaten
  - Geometrie
  - Performance (LCA + Tragwerk)

---

## Interviewfragen

### Datenstruktur
- Welche Datenfelder sind im Materialpass zwingend erforderlich?
- Wie wird Datenqualität sichergestellt?

### Lebenszyklus
- Wie wird sichergestellt, dass Daten über Jahrzehnte konsistent bleiben?
- Wie werden Änderungen dokumentiert?

### Nutzung
- Wie werden Materialpässe aktuell in Planungsprozessen genutzt?
- Wo bestehen Hürden für Architekt:innen?

### Integration
- Gibt es Schnittstellen zu BIM oder Entwurfssoftware?
- Welche Daten fehlen für eine direkte Entwurfsintegration?

---

## Relevante Hinweise

- Madaster adressiert primär die **Langzeitperspektive**
- Für die Forschung entscheidend ist die Verbindung zu:
  - **frühphasigem Entwurf**
  - **operativer Wiederverwendung**

---

## Connections

- [[bim-berlin-data-model]]
- [[bim-berlin-bauteilauktion]]
- [[concular]]
- [[concular-urban-mining]]
- [[concular-supply]]
- [[zrs-crclr-house]]
- [[be-ware-engineering]]
- [[arup-reuse]]
- [[haus-der-materialisierung]]
