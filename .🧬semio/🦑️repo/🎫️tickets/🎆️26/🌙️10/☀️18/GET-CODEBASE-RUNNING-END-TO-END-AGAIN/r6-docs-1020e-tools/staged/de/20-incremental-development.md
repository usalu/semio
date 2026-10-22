#### Inkrementelle Entwicklung

Ziel ist, dass eine Änderung die Arbeit kostet, die von ihr abhängt, und sonst nichts, für jeden Agenten gleichzeitig. Die Tabelle sagt je Schicht, was die Einheit der Wiederverwendung ist und was eine Änderung neu macht.

| Schicht | Einheit der Wiederverwendung | Eine Änderung macht neu | Zustand |
|---|---|---|---|
| Task-Schlüssel | die deklarierten Inputs einer Task | die Tasks, deren deklarierte Inputs die Datei enthalten, dann die Abhängigen, deren Input-Snapshot sich geändert hat (früher Abbruch bei Bazel) | live |
| Abgeleitete Inputs | eine per Regel abgeleitete Input-Menge statt eines Globs über das ganze Repository | Tasks, die breit lesen, deklarieren, was sie lesen: die Tasks über 100.000 deklarierten Dateien sanken von 243 auf 11, und die 947 geänderten Tasks deklarieren 12,0 M statt 34,3 M Input-Dateien (statische Rekonstruktion von 8.028 Tasks) | {{@input-precision}} |
| Rust-Schlüssel | {{^crate-keys::ein benannter Input `cargoToolchain`, der das Wurzel-`Cargo.toml`, das ganze Wurzel-`Cargo.lock`, die Konfiguration und die Toolchain enthält (139 Projekte, 824 gecachte Targets)::die Auflösungsidentität eines Crates: seine gesperrte Abhängigkeitsscheibe, die geerbten Workspace-Felder, Toolchain und Konfiguration}} | {{^crate-keys::jede Änderung des Wurzel-Locks oder dieser Dateien vergibt den Schlüssel von allen neu::die Crates, deren Scheibe oder geerbtes Feld sich geändert hat; Kommentare, Formatierung und Mitgliederlisten des Workspaces bewegen nichts}} | {{@crate-keys}} |
| Projektgraph | {{^graph-generations::ein versiegelter Graph des ganzen Workspaces::ein unveränderliches Fragment je Eigentümer innerhalb einer Generation}} | {{^graph-generations::jede Quelländerung macht das Siegel ungültig, und der nächste frische Leser baut unter einem exklusiven Lease neu::die schmutzigen Eigentümer und die Eigentümer, die denselben Fakt verbraucht haben; Leser nehmen keinen Lease}} | {{@graph-generations}} |
| Abhängigkeiten | eine Generation je Bereich (eine `bun.lock` und ihr `node_modules`) | die Bereiche, deren Generationsschlüssel sich bewegt hat; der Rest wird über Belege geprüft | {{@sync-incr}} |
| TypeScript | {{^ts-units::ein Programm und ein inkrementeller Zustand je Eigentümer, Compiler und Konfiguration::eine Prüfeinheit je Paket, durch eine Deklarationsgrenze abgeschnitten}} | {{^ts-units::jede Änderung prüft das ganze Programm neu::eine Rumpfänderung prüft eine Einheit neu (2,9 s gemessen gegen 27 s), eine Signaturänderung prüft ihre Abhängigen neu, bis deren Deklarationen gleich sind}} | {{@ts-units}} |
| Rust-Compile | {{^cargo-lanes::ein Aufruf von `repo cargo`::eine Compile-Einheit, inhaltsadressiert, in einem Cache, den alle Agenten teilen}} | {{^cargo-lanes::die Crates, die Cargo im einen geteilten Build-Verzeichnis als veraltet findet::die Einheiten, deren Quelle oder Abhängigkeits-Artefakte sich geändert haben; das mit `-p` gewählte Crate behält den inkrementellen Zustand von rustc in der eigenen Lane des Agenten}} | {{@cargo-lanes}} |
| Bazel | Action-Schlüssel im geteilten Disk-Cache | die Actions, deren Schlüssel sich bewegt hat; Abhängige werden verschont, wenn eine Ausgabe byte-identisch ist | live |
| Nx | Task-Hash im Task-Cache | die Tasks, deren Hash sich bewegt hat | live |

**Beweise es mit `repo incremental`** ({{@incremental}}). Der Befehl beurteilt in einem Wegwerf-Klon des Checkouts, dass Nx und Bazel genau das neu bauen, was eine Änderung betrifft, mit denselben Szenarien unter beiden Werkzeugen und den Ergebnissen nebeneinander. `prepare` klont die verfolgten und unverfolgten Dateien nach `--root` (Standard `/tmp/sinc`, unter Windows `<Laufwerk>:\sinc`; abgelehnt, wenn die Wurzel im Checkout liegt oder ihn enthält) und richtet den Klon mit eigenem `preflight`, `tools setup all` und `dependencies sync --all` ein; jede Änderung eines Szenarios geschieht im Klon, nie in deinem Baum.

```bash
repo incremental list
repo incremental plan --scenario=fw-ts-comment
repo incremental prepare
repo incremental run --scenario=fw-ts-comment --implementations=nx,bazel
repo incremental discard
```

- **Szenarien.** 17 Szenarien in 7 Bereichen (Framework-TypeScript, Rust-Crate, Wasm-Komponente, Vite-App, Test-Target, Lockfile, Produktaktivierung) und 6 Änderungsarten (nur Kommentar, Rumpfänderung, Signaturänderung, Konfiguration, Lockfile, unbeteiligtes Geschwister). Je Implementierung läuft ein Szenario als Prime, `warm` (keine Änderung: nur ungecachte Tasks dürfen laufen), `edit` (eine Zeile an eine Quelle angehängt) und `revert` (der exakte Originaltext wird wiederhergestellt, auch nach einem Fehler oder Abbruch, und alles muss aus dem Cache kommen). `--phases=warm,edit,revert` schränkt die Phasen ein.
- **Urteile.** Die erwartete Menge wird aus den deklarierten Inputs berechnet: Tasks, die die Datei direkt lesen, müssen laufen, Abhängige, deren Ausgabe sich geändert hat, müssen laufen, Abhängige, deren Ausgabe nachweislich identisch ist, dürfen verschont werden, ungecachte Tasks laufen immer. `pass`; `stale` (eine Task, die laufen musste, lief nicht); `excess` (eine Task lief, die nicht durfte); `stale-excess`; `failed`; `skipped`. Bazel folgt jeder Abhängigkeit und bricht an Ausgabe-Digests ab, Nx folgt nur Tasks, die Ausgaben verbrauchen: Das ist die eine Stelle, an der die Werkzeuge rechtmäßig abweichen, und der Bericht listet sie als `only nx:` und `only bazel:`.
- **Beobachtung und Ausgabe.** Bazel wird aus seinem kompakten Ausführungsprotokoll gelesen, Nx aus seinem eigenen Laufprotokoll. Der Bericht erscheint auf Englisch oder Deutsch (`--lang`), `--json` gibt JSON aus, `--file=<report>` schreibt ihn; der Fortschritt wird je Stufe veröffentlicht, jede Stufe ist ein Abbruchpunkt, und der Exit-Code scheitert, sobald ein Lauf kein Pass ist.

**Beweise Nebenläufigkeit mit `repo concurrency`** ({{@concurrency}}). {{?concurrency::::Die Befehle unten gehören zum vorbereiteten Set und existieren erst nach seiner Anwendung. }}Der Befehl ist der Abnahmetest für alles in diesem Kapitel, was „kein Warten“ behauptet: N simulierte Agenten arbeiten in demselben Wegwerf-Klon (dem, den `repo incremental prepare` einrichtet), ändern gleichzeitig verschiedene Projekte und führen Build, Test und Check mit beiden Werkzeugen aus; danach laufen dieselben Sitzungen noch einmal hintereinander als serielle Referenz.

```bash
repo concurrency list
repo concurrency plan --workloads=all
repo concurrency run --workloads=disjoint-projects --sessions=3 --implementations=nx,bazel
```

- **Workloads.** `disjoint-projects` (ein TypeScript-, ein Rust-, ein Go- und ein Schema-Projekt, von je einem Agenten geändert und gebaut), `same-target-cold` (eine Änderung, die alles veralten lässt, drei Agenten bauen dasselbe Target, 250 ms versetzt gestartet), `warm-fanout` (vorgewärmt, drei Agenten bauen dasselbe Target) und `readers-while-edit` (ein Bearbeiter listet nach einer Quelländerung die Projekte, während zwei Leser dasselbe tun). `--sessions=<n>` nimmt die ersten n Agenten (Standard 3, höchstens 8). `plan` lehnt einen Workload ab, in dem ein Agent eine Datei ändert, die die Targets eines anderen Agenten als Input deklarieren.
- **Kriterien** (`--criteria=default|warm|smoke` ersetzt die eigene Menge eines Workloads). `default`: jeder Schritt gelingt, die Ergebnisse gleichen dem seriellen Lauf (Exit-Code, deklarierte Ausgabe-Digests, Stdout-Digest), kein Warten über 5 s auf ein unabhängiges Projekt in irgendeiner Sperrklasse (Graph-Facts, Abhängigkeits-Image, Bazel-Server, Cargo-Einheiten, Kapazitätszulassung, Werkzeugbeschaffung, sonstige), und eine Task läuft nur in einer Sitzung, während die anderen für sie keine Action ausführen. `warm` verlangt null ausgeführte Actions; `smoke` verlangt nur Erfolg und Gleichheit mit dem seriellen Lauf. Der Bericht gibt ein Urteil je zuständigem Executor aus: pass, fail oder unbewiesen, wenn nur übersprungene Regeln bleiben.
- **Was eine Wartemessung beweisen kann.** Die Zeilen `Another command (pid=N) is running` von Bazel und `Blocking waiting for file lock` von Cargo liefern genaue Wartezeiten. Ein Lease veröffentlicht seine Fortschrittszeile nur alle 15 s, Lease-Wartezeiten sind deshalb eine Untergrenze: Eine Grenze über dem Limit lässt ein Kriterium trotzdem scheitern, eine Grenze darunter lässt es unbewiesen. Genaue Lease-Wartezeiten brauchen `SEMIO_LEASE_TRACE=<file>`, das das Lease-Modul für Beschaffungen schreibt, die 100 ms oder länger gewartet haben (ein optionaler Teil des Harness-Sets). Nx schreibt ein Laufprotokoll je Cache-Verzeichnis, gleichzeitige Nx-Läufe werden daher über das Zeitfenster zugeordnet und bei Überlappung als `ambiguous` markiert.
- **Kein Beschleunigungstest.** Die serielle Referenz läuft nach der nebenläufigen Phase und ist deshalb warm: Der Harness beweist gleiche Ergebnisse und fehlendes Warten, keine Gesamtlaufzeit.

**Was eine Änderung kostet (Rust).** Statisches Modell über das Repository: 240 Cargo-Workspaces, 3.374 Crates und 12.134 abgeleitete oder verfasste gecachte Targets.

| Änderung | Heute neu geschlüsselte Crates und Targets | Mit Schlüsseln je Crate ({{@crate-keys}}) |
|---|---|---|
| Kommentar oder Formatierung im Wurzel-`Cargo.toml`, in `.cargo/config.toml` oder `rust-toolchain.toml`; ein an die Wurzel-`[workspace].members` angehängtes Crate | 3.374 und 12.134 | 0 und 0 |
| eine Workspace-Abhängigkeit der Wurzel, die ein Crate nutzt | 3.374 und 12.134 | 1 und 3 |
| ein `Cargo.lock`-Paket, das ein Crate nutzt | 139 und 827 (Nx), 564 und 2.964 (Bazel) | 1 und 5 |
| `[build].rustflags` oder der Toolchain-Kanal | 3.374 und 12.134 | 3.374 und 12.134 (richtig) |
| eine Änderung eines verschachtelten Workspace-Manifests (`🧰️framework/Cargo.toml`: Lints, Abhängigkeitseinträge) | 0 und 0 (ein veralteter Treffer) | 2.274 bis 3.258 Crates (eine Korrektheitsreparatur) |
| eine Quelländerung in einem Blatt-, Median-, 90.-Perzentil- oder Hub-Crate | 1, 21, 2.233 und 3.080 Crates | unverändert: Der Fan-in ist wesenseigen |

Der Fan-in ist eine Eigenschaft des Abhängigkeitsgraphen. Die korrigierte Zählung (Workspace-Abhängigkeiten über den besitzenden Workspace aufgelöst) zeigt eine mittlere Reichweite von 363 Crates (11,3 Prozent), eine längste Kette von 157 und 354 Crates mit einer Reichweite ab 2.000; eine Änderung in den acht untersten Crates des Value-Turms baut wegen der Orphan-Regel etwa 2.915 von 3.213 Crates neu. Aufteilungen der heißesten Crates sind geplant, nicht angewendet.

**Schnellere Schleifen.**

- Wähle eng: `repo run <project>:<target>` baut den Abschluss einer Task; prüfe den Plan vorher mit `--dry-run`.
- Nimm für Rust `repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib`: `check` ist billiger als `build`, und der Wechsel zwischen `prototype` und `release` vergibt alles neu.
- Bleibe bei einem Werkzeug und einem Modus, solange du iterierst: Jedes Werkzeug hat seinen eigenen Cache, und der Modus gehört zu jedem Schlüssel.
- Ändere die eigenen Dateien des Build-Werkzeugs nicht in einer Schleife: Jede Action wird bei jeder Änderung einmal neu geschlüsselt.
- TypeScript-Einheiten sind eine Bibliotheks-Engine; noch kein `repo`-Befehl führt sie aus, und das monolithische `typecheck` bleibt das Release-Tor: Auf dem gemessenen Programm meldete die einheitenweise Prüfung 37 von 43 Diagnosen, keine zusätzliche, sie ist also ein Entwicklungstor, bis die sechs nur monolithischen Diagnosen verstanden sind. Die Deklarationsausgabe muss jeden exportierten Typ benennen können; die Befunde (ein `brand`-Member, funktionslokale Klassen mit `private`-Membern, abgeleitete Typen, die `node_modules`-Pfade nennen) sind Korrekturen der Eigentümer, die bei der Engine aufgelistet sind.
