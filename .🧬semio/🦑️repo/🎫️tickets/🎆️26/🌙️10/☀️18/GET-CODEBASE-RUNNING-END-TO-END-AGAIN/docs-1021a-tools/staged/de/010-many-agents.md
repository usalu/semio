#### Viele Agenten in einem Ordner

Mehrere Entwickler und Agenten arbeiten gleichzeitig in einem Checkout, an verschiedenen Crates und Paketen. Das Build-System ist so angelegt, dass unabhängige Arbeit nicht wartet: Ergebnisse sind inhaltsadressiert und geteilt, ein Schreiber hält eine geteilte Ressource nur, solange er veröffentlicht, Zustand je Sitzung ist privat, und jedes Warten ist eine First-in-first-out-Warteschlange, die den Halter der Ressource nennt. Dieser Abschnitt ist die Übersichtskarte; die Abschnitte danach enthalten die Einzelheiten. Jede Funktion trägt ihren Zustand. **Live** heißt: im Checkout angewendet. **Vorbereitet** heißt: gebaut, in einer Overlay-Kopie des Baums getestet und wartet auf die Anwendung; ihre Befehle gibt es noch nicht, und ihre Beschreibung ist der Vertrag, mit dem sie ausgeliefert wird (die genannten Zahlen sind Overlay-Messungen, keine Live-Messungen).

| Funktion | Befehl oder Schalter | Zustand |
|---|---|---|
| Kapazitätsregelung des Rechners | `repo capacity status`, `repo capacity run`, `SEMIO_HOST_CAPACITY=off` | {{@capacity}} |
| Anlauframpe der Kapazität und begrenztes Überholen | automatisch | {{@capacity-ramp}} |
| Faire Lease-Warteschlangen, die den Halter nennen | die Fortschrittszeile `lease-queue` | live |
| Sperrreihenfolge, Deadlock-Erkennung, Lease-Status | `repo leases status`, `SEMIO_LEASE_ORDER` | {{@leases}} |
| Richter für Inkrementalität | `repo incremental` | {{@incremental}} |
| Abnahmetest für gleichzeitige Sitzungen | `repo concurrency` | {{@concurrency}} |
| Cache-Ledger, Budgets, sperrbewusste Bereinigung | `repo cache status`, `repo cache gc` | {{@cache-ledger}} |
| Graph-Richtlinie und Graph-Generationen | `--graph-policy=fresh\|current` | {{@graph-policy}} |
| Bazel-Server-Slots je Sitzung | `SEMIO_BAZEL_SERVERS`, `SEMIO_BAZEL_SERVICES` | {{@bazel-sessions}} |
| Cargo-Lanes und Compile-Unit-Cache | {{?agent-sessions::`repo cargo --store=shared`, `SEMIO_AGENT_SESSION`, `SEMIO_UNIT_CACHE_SLOTS`::`repo cargo --store=shared`, `SEMIO_CARGO_LANE`, `SEMIO_UNIT_CACHE_SLOTS`}} | {{@cargo-lanes}} |
| Geteilte Cargo-Build-Skript-Läufe und Komponenten-Durchläufe | `SEMIO_UNIT_CACHE_BUILD_SCRIPTS` | {{@cargo-buildscript}} |
| Nx-Graph-Wiederverwendung und Pool ohne Hold-and-Wait | automatisch | {{@nx-generation}} |
| Nx-Daemon-Obergrenze und Leerlauf-Beendigung | `SEMIO_NX_DAEMONS`, `SEMIO_NX_DAEMON_IDLE_MS` | {{@nx-daemons}} |
| Nx-Cache-Put-Slots | `SEMIO_NX_PUT_SLOTS` | {{@nx-put-slots}} |
| Gleichzeitige Abhängigkeits-Generationen | `repo dependencies ensure`, `repo dependencies sync` | {{@deps-concurrent}} |
| Lease-freie Generationspflege und Größenzulassung | automatisch | {{@deps-concurrent-admission}} |
| Inkrementeller Dependency-Sync | `repo dependencies sync` | {{@sync-incr}} |
| Schlüssel je Crate | automatisch | {{@crate-keys}} |
| Schmale, abgeleitete Task-Inputs | automatisch | {{@input-precision}} |
| TypeScript-Prüfeinheiten je Paket | Bibliotheks-Engine, noch kein Befehl | {{@ts-units}} |
| Test-Sandbox | automatisch in `repo run <project>:test` | {{@sandbox}} |
| Ausgabeparität von Nx und Bazel | `repo verify build-parity` | {{@parity}} |
| Wiederherstellung nach Neustart oder Absturz | automatisch beim nächsten `repo`-Befehl | {{@boot-sweep}} |
| CI auf der neutralen Schnittstelle mit geteiltem Remote-Cache | `BUILD_TOOL`, `SEMIO_REMOTE_CACHE_ENDPOINT` | {{@ci}} |
| Nativer Launcher: `repo` ohne von Hand installierte JavaScript-Laufzeit | `repo`, `SEMIO_RUNTIME` | {{@launcher}} |
| Laufzeit-Fassade mit Bun- und Node-Adapter | automatisch | {{@runtime-seam}} |
| Werkzeugregister | `repo tools list\|verify\|exec` | {{@tool-registry}} |
| Gesetz gegen direkte Prozessstarts | `repo verify tool-spawns` | {{@tool-spawn-law}} |
| Paketmanager-Naht | `SEMIO_PACKAGE_MANAGER_<ECOSYSTEM>` | {{@pm-seam}} |
| Storage-Backend-Naht | `SEMIO_NATIVE_STORAGE_BACKEND` | {{@storage-seam}} |
| Build-Backend-Register und `--tool=semio` | `--tool=nx\|bazel\|semio` | {{@backend-registry}} |
| Konformanz-Suite für jedes Backend | `repo conformance` | {{@conformance}} |
| Polyglotte Einheiten mit frühem Abbruch je Sprache | `repo units` | {{@units}} |
| Gepinnte Toolchains für Go, Python, .NET, CMake | automatisch | {{@toolchains}} |
| Residenter Dienst für warme Befehle | `repo service`, `--service=auto\|off\|require` | {{@resident}} |
| Agent-Sitzungen | `repo session` | {{@agent-sessions}} |
| Server für den geteilten Ergebnis-Cache | `repo remote-cache serve\|status\|gc` | {{@remote-cache-server}} |
| Leistungsziele | `repo bench` | {{@bench}} |
| Implementierungs-Pinning: Builds überleben Änderungen | automatisch | {{@graph-pinning}} |
| Keine IDE-Konfiguration, Pläne gehören Tickets | das Zensurgesetz | {{@ide-free}} |

Regeln für das Zusammenarbeiten:

1. **Alles über `repo`.** Rufe nie selbst `nx`, `bazel`, `cargo`, `rustc` oder die JavaScript-Laufzeit auf und setze nie ein eigenes `CARGO_TARGET_DIR`: Die geteilten Stores sind Partitionen fester Größe, und private Kopien füllen die Platte (eine Flotte privater Cargo-Targets erreichte am 2026-10-20 926 GB).
2. **Arbeite an deinem eigenen Projekt.** Ändere das Crate oder Paket, das dir gehört, und führe dessen Targets aus; `repo run <project>:<target>` baut nur den Abschluss dieser Task. Dateien, die viele Projekte lesen, sind geteilte Inputs und vergeben die Schlüssel großer Bereiche neu: das Wurzel-`Cargo.toml` und `Cargo.lock`, `nx.json`, die `tsconfig`-Wurzeln, jede `package.json` und `bun.lock`, die Taxonomie-Dateien. Bündele solche Änderungen und sag den anderen Bescheid („Inkrementelle Entwicklung“ zeigt, was sie kosten).
3. **Frage den Rechner vor einem schweren Befehl.** Verpacke einen beliebigen schweren Befehl in `repo capacity run --memory=<megabytes> -- <command>` und lies `repo capacity status`, bevor du mehrere Dinge gleichzeitig startest („Rechnerkapazität“).
4. **Warten ist eine Warteschlange, kein Hängen.** Lies die Zeile, die ausgegeben wird; sie nennt den Halter. Ein Halter, der vorankommt, arbeitet für alle: Beende keinen Prozess, den du nicht gestartet hast. Strg+C bricht deinen ab und gibt zurück, was er hielt.
5. **Eine Abhängigkeitsänderung nach der anderen.** Lock und Sync gelten für das ganze Repository. Führe sie einmal nach der letzten Manifest-Änderung aus; alle anderen führen `repo dependencies ensure` aus, das nur installiert, wenn etwas fehlt oder geändert ist.
6. **Tests nutzen ihre Sandbox.** Ein Test schreibt nie in den Checkout, in eine geteilte Cache-Route oder an einen festen Port („Test-Sandbox“).
7. **Deine Identität ist eine Sitzung.** Deine Cargo-Lane, dein bevorzugter Bazel-Slot, deine Test-Sandbox, dein Lease-Eigentümer und dein Anteil an der Kapazitätswarteschlange leiten sich aus einer Agent-Sitzung ab; `repo session status` zeigt sie („Agent-Sitzungen“).
8. **Lass den Build-Zustand in Ruhe.** Lösche oder trenne nie etwas unterhalb von `.cache/b`, verschiebe nie einen Beleg; nutze die Kontrollen aus „Wiederherstellung nach Neustart oder Absturz“.
