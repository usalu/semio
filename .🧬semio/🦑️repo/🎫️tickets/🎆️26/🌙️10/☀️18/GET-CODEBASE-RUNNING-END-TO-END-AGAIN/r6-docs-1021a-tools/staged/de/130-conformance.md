#### Die Konformanz-Suite

Eine sprachunabhängige Suite beurteilt jedes Build-Backend mit denselben Szenarien, damit ein Austausch nicht still eine Fähigkeit verliert. {{?conformance::::Der folgende Befehl gehört zum vorbereiteten Set. }}Sie ist Ebene 1 und läuft in Sekunden bis Minuten; die Harnesses auf dem echten Repository `repo incremental`, `repo verify build-parity` und `repo concurrency` bleiben Ebene 2. Zustand: {{@conformance}}.

```bash
repo conformance list
repo conformance plan --scenario=<id>
repo conformance run --implementations=semio
repo conformance run --implementations=nx,bazel,semio --file=<report> --json
```

- **Workspace.** 15 Stub-Projekte (ein Diamant, ein Abbruchpaar, mehrere Ausgaben, fehlschlagend, fehlschlagender Test, langsam, Single-Flight, drei parallele). Dieselben Daten werden zu einem Nx-Workspace, einem Bazel-Workspace oder dem neutralen Graphen der hauseigenen Engine. Grundwahrheit für „ausgeführt“ ist eine Markerdatei, die ein Stub-Task-Rumpf schreibt, nie die Aufzeichnung des Backends.
- **17 Fähigkeiten in 20 Szenarien**: Plan, Lauf (kalt, geordnet, träger Abschluss), Cache-Wiederherstellung (auch nach dem Löschen von Ausgaben), Invalidierung, früher Abbruch, ungecachte Tasks, verschachtelte Ausgaben, Fehlerisolation (ein Fehler wird nie gecacht), Testergebnisse, Affected, Abbruch (stoppt innerhalb von 5 s, nichts wird veröffentlicht, der nächste Lauf gelingt), Fortschritt, Parallelität (Grenze und Überlappung), gleichzeitige Aufrufe, Single-Flight, Remote-Cache (frischer Checkout, leerer lokaler Cache, geteilter Server) und Laufprotokoll. Jedes Szenario nennt die vorhandene Evidenz, die es verallgemeinert (Szenario-IDs von incremental, Stichproben-IDs von parity, Workloads von concurrency).
- **Urteile** verbinden die Zusicherung im Register mit der Messung: `pass`, `gap` (als fehlend zugesichert und fehlschlagend: dokumentiert), `regression` (zugesichert und fehlschlagend), `stale-claim` (als fehlend zugesichert, aber bestanden), `unobserved` (die Bindung kann es nicht beobachten: getrennt gezählt, nie ein Pass) und `error`. Der Richter hat ein unabhängiges Python-Orakel; Empfindlichkeitstests zeigen, dass ein Backend, das seinen Cache vergisst, über Exit-Codes lügt, Abbruch ignoriert oder Arbeit serialisiert, genau die Szenarien nicht besteht, die es fangen sollen.

| Fähigkeit | nx | bazel | semio |
|---|---|---|---|
| 18 von 20 Szenarien (Plan, Lauf, Cache-Wiederherstellung, Invalidierung, früher Abbruch, ungecacht, Ausgaben, Fehlerisolation, Testergebnisse, Affected, Abbruch, Parallelität, gleichzeitige Aufrufe, Remote-Cache, Laufprotokoll) | pass | pass | pass |
| Single-Flight: zwei gleichzeitige Aufrufe führen den Rumpf einmal aus | gap, wie deklariert (Nx-Prozesse koordinieren sich nicht) | pass | pass |
| Fortschritt: ein Start und ein Ende je ausgeführter Task | unobserved (Nx veröffentlicht keine Ereignisse je Task) | unobserved | pass |

Tatsachen, die die Läufe festgestellt haben: Unverändertes Nx erhält auch frühen Abbruch, wenn Abhängige `dependentTasksOutputFiles` deklarieren; eine Bazel-Action mit dem Tag `local` kann den Remote-Cache nie nutzen, ein Backend-Adapter, der Tasks `local` markiert, verliert also das Teilen; Bazel hält seine Produkte außerhalb des Workspaces, jede Bindung muss sie also zurückkopieren, auch schreibgeschützte Dateien. Die hauseigene Engine besteht alle 20 Szenarien über ihren Treiber und über das echte CLI (`repo conformance run --implementations=semio` und `repo --tool=semio tools setup semio`); echtes Nx 23.2.0 und echtes Bazel 8.4.2 ergeben 19 von 20 erfüllt plus die eine Lücke oder nicht beobachtete Zeile oben. Nicht gelaufen: die Szenarien über `repo --tool=semio run` eines echten Repository-Targets (der Graph-Build wartete in der Kapazitätswarteschlange) und die Harnesses der Ebene 2 mit `--implementations=semio`. Vorgehen nach der Anwendung: `repo --tool=semio run <cheap project:target>` zweimal (der zweite muss wiederherstellen), danach `--dry-run`, `affected` und `cache gc --area=semio-actions`.
