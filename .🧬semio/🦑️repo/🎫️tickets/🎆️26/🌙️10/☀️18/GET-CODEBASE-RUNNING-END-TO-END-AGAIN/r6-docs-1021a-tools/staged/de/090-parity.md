#### Paritätsprüfung

Nx und Bazel müssen aus demselben neutralen Graphen dieselben Ergebnisse erzeugen. `repo verify build-parity` beweist es je Stichprobe statt durch Ansehen ({{@parity}}). `verify` ist ein Wurzelbefehl, keine Build-Operation: Er führt zuerst `repo dependencies ensure` aus{{?launcher::.::, und wo `repo` nicht im `PATH` liegt, heißt er `bun ./📜️script.ts verify build-parity` an der Wurzel.}}

```bash
repo verify build-parity --plans-only
repo verify build-parity --only=area:framework
repo verify build-parity --samples=<file> --out=<directory> --keep --json
repo verify build-parity --recover=<directory>
repo inputs @teaching/architecture-course:test
```

- **Was eine Stichprobe tut.** Der Dry-Run-Task-Graph beider Werkzeuge und der Abschluss (Bazel-`cquery` gegen den Planer); dann `repo --tool=nx|bazel run <target>` frisch (`--skip-cache`) und aus dem Cache wiederhergestellt; die deklarierten Ausgaben jedes Werkzeugs in getrennte Wurzeln erfasst und Byte für Byte verglichen (Art, Modus, Link-Ziel, SHA-256); Exit-Codes, gescheiterte Tasks und Teststatus; die Nx-`inputs` gegen die Inputs und die Umgebung der Spawns von Bazel (aus seinem Ausführungsprotokoll gelesen); und die Cache-Wiederherstellung jedes Werkzeugs kalt gegen wiederhergestellt.
- **Nur eine Regel erklärt einen Unterschied.** `🏗️build/🪞️parity/🔣️rules.json` enthält 8 Normalisierungen, jede mit einer Begründung auf Englisch und Deutsch und ihrer Evidenz, und der Bericht zählt, wie oft jede galt. Alles andere ist eine Abweichung (35 Codes, jeder mit Hilfetext), und der Befehl endet mit einem Exit-Code ungleich null. 27 Stichproben in 6 Bereichen (`🏗️build/🪞️parity/🔣️samples.json`: `framework`, `s-plugins`, `hub`, `teaching`, `mit-bestand`, `semio-tech`) sind verfasst; `--only=<id>` oder `--only=area:<area>` (kommagetrennt) wählt aus, `--samples=<file>` ersetzt sie.
- **Sicher für deinen Baum.** Deklarierte Ausgabewurzeln werden vor jedem Lauf beiseitegelegt und in einem `finally` wiederhergestellt (auch bei Strg+C); verfolgte Wurzeln werden kopiert, unverfolgte verschoben; `--recover=<directory>` stellt nach einem Absturz aus dem Journal wieder her. Der Fortschritt wird je Stichprobe, Werkzeug und Phase veröffentlicht, und jeder Schritt ist ein Abbruchpunkt.
- **Berichte.** `report.json`, `report.en.md` und `report.de.md` landen im Evidenzverzeichnis (`--out`, mit `--keep` behalten); `--lang=en|de` wählt die Hilfesprache. Die Bazel-Seite zeichnet ihre Evidenz (Ausführungsprotokoll, Build-Ereignisse, Abschluss) nur auf, wenn `SEMIO_BUILD_EVIDENCE_DIR` gesetzt ist, was die Prüfung für ihre eigenen Bazel-Läufe selbst setzt.
- **`repo inputs <project>:<target>[:<configuration>]`** gibt aus, was Nx für eine Task hasht (Dateien, Umgebung, Laufzeitwerte, Abhängigkeits-Ausgaben, externe Pakete), als JSON. Bazel implementiert es nicht: Seine Inputs sind die Spawn-Inputs in der Paritäts-Evidenz.
- **Noch nicht verifiziert.** Die Antworten der echten Werkzeuge: Getestet wurde mit Fixtures, einem Schein-`repo` und Bazel-Evidenztests; die ersten Live-Läufe (der Kurs-Build, der Build von semio-tech play, der Release-Build des mit-bestand-Demonstrators, ein kompakter Berichtsdruck) sind offen. Eine schwere Stichprobe wie der Release-Build des Demonstrators kann das Stichproben-Limit von 3.600 s überschreiten. Führe einen Prozess nach dem anderen aus; die Kapazitätsregelung des Rechners gilt für die Kindläufe.
