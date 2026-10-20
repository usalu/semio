### Build-Werkzeuge im Alltag

Dieser Leitfaden ist die Arbeitssicht auf das werkzeugneutrale Build-System: welches Werkzeug du wählst, was jede Operation tut, wie Ergebnisse gecacht und geteilt werden, wie Arbeit auf Worker und Executor verteilt wird und wie du ein neues Target einordnest. Die Referenzabschnitte danach (Neutrales Modell bis Remote-Cache) enthalten die Verträge, die Messungen und die offenen Punkte. Jeder Befehl unten ist eine Operation des Katalogs `📚️library/🏗️build/⚙️operations/🔣️catalog.json`; `repo <command> --help --lang=en|de` gibt dieselbe Verwendung auf Englisch oder Deutsch aus. Wo `repo` nicht im `PATH` liegt, schreibe `bun repo`, das Skript des Wurzel-Pakets. Die englische Fassung steht im Kapitel „Build tools guide“.

#### Werkzeug wählen

- **Wählen.** `--tool=nx|bazel` gewinnt, dann die Umgebungsvariable `SEMIO_BUILD_TOOL`, dann der Standard der Richtlinie `nx`. Die Wahl wird als `SEMIO_BUILD_TOOL` an jede verschachtelte Task weitergegeben; eine Task, die `repo` erneut aufruft, bleibt also bei deinem Werkzeug. Die Dashboard-Achse `build-tool` setzt dieselbe Variable. Alles andere wird abgelehnt („Unknown build tool“).
- **Einrichten.** `repo tools setup all` installiert das gepinnte Nx und Bazel für diesen Rechner; `repo tools setup nx` oder `repo tools setup bazel` installiert eines davon. `workspace:setup` führt denselben Schritt aus.
- **Ein Graph.** Beide Werkzeuge führen denselben neutralen Graphen mit denselben Task-IDs aus (`<project>:<target>[:<configuration>]`). Eine Auswahl, die unter beiden Werkzeugen zu verschiedenen Tasks führt, ist ein Fehler und kein Merkmal.

| | Nx (Standard) | Bazel (`--tool=bazel`) |
|---|---|---|
| Was läuft | ein `nx run` je Auswahl | ein erzeugter Workspace je Auswahl; eine Task ist eine Bazel-Action, eine gewählte gecachte `test`-Task ein Bazel-Test, ein Dienst ein `bazel run`-Target |
| Lokaler Cache | Task-Ergebnisse in `nx/artifacts` der Rust-Partition | Action-Cache des Output-Base plus Disk-Cache `bazel/actions` der Produkte-Partition |
| Remote-Cache | selbst gehostete Nx-Cache-API | Bazel-HTTP-Cache-Protokoll, Token über einen Credential-Helper |
| Remote-Ausführung | nein | wählbar für Targets, die `metadata.semio.hermetic` deklarieren |
| Nebenläufigkeit | `NX_PARALLEL` | persistente Worker und `--jobs` |
| Wiederverwendung zwischen Auswahlen | über den Task-Hash im Task-Cache | über den Action-Schlüssel im Disk-Cache: Eine zweite Auswahl nutzt die Ergebnisse der ersten |

Nimm Nx für den Standardablauf. Wähle Bazel, wenn du Ergebnisse über Auswahlen und Rechner hinweg teilen, Worker oder Remote-Ausführung nutzen willst. Beide werden mit denselben Flags gesteuert; wo ein Werkzeug eine Einstellung nicht einhalten kann, listet `repo cache remote status` sie als `unsupported` (siehe Remote-Cache für die Nx-Felder).

#### Die neutralen Operationen

| Operation | Was sie tut | Beispiel |
|---|---|---|
| `run` | eine Task und die Tasks, von denen sie abhängt | `repo run @semio-tech/semio-tech-play:serve --tool=bazel` |
| `run-many` | dieselben Targets in mehreren Projekten (`--projects`, `--exclude`, Muster `tag:`, `name:`, `directory:`, `!`) | `repo run-many --targets=test --projects=tag:rust --tool=bazel` |
| `affected` | Targets in den gegenüber `HEAD` geänderten Projekten (verfolgte Änderungen plus unverfolgte Dateien) und ihren Abhängigen | `repo affected --targets=build --base=main --head=HEAD` |
| `exec` | ein Befehl je Projekt, Projektwurzel als Arbeitsverzeichnis, `SEMIO_PROJECT_NAME` und `SEMIO_PROJECT_ROOT` gesetzt | `repo exec --projects=@semio-tech/repo-client -- bun test` |
| `--dry-run`, `--graph` | geben den Task-Graphen als `DryRunPlan` aus und führen nichts aus; `--graph=<file>` schreibt ihn in eine Datei | `repo run-many --targets=build,test --dry-run` |
| `show`, `graph`, `watch` | der neutrale Graph und ein Befehl, der bei Quelländerungen neu läuft | `repo show projects --json` |
| `cache status`, `cache clear` | zählen oder leeren den lokalen Task-Cache des gewählten Werkzeugs | `repo cache status --tool=bazel` |
| `cache remote status`, `cache serve` | der Remote-Cache-Vertrag und ein lokaler Cache-Server (siehe Cache-Modell) | `repo cache remote status --tool=bazel` |
| `reset` | verwirft den Graphen und setzt den lokalen Zustand des Werkzeugs zurück | `repo reset --tool=bazel` |

- **Dry-Run und Graph.** `--dry-run` und `--graph` geben für jedes Werkzeug denselben `DryRunPlan` aus und starten weder Nx noch Bazel: `roots`, `tasks` und `dependencies` (unter Bazel zusätzlich `selected` und Abhängigkeiten auf Dienste in `continuousDependencies`). Der Plan sagt, was laufen würde, nicht, was gecacht ist; dafür dient `cache status`.
- **Affected.** Die Regel ist für beide Werkzeuge gleich: das tiefste Projekt, dem eine geänderte Datei gehört, Projekte, deren Target-Inputs eine geänderte `{workspaceRoot}/…`-Datei nennen, jedes Projekt bei einer Änderung von `nx.json`, dann alle Abhängigen. Bazel beantwortet sie mit einer `rdeps`-Abfrage über ein erzeugtes Paket, gegen die eigenen Locators von Nx bei vier von fünf Änderungsmengen geprüft; eine Änderung allein an `bun.lock` ist der bekannte Unterschied (Nx behandelt sie als „alles“, die neutrale Regel und Bazel sehen nur Pfad-Zugehörigkeit). `--projects` wird bei `affected` abgelehnt; schränke mit `run-many` ein.
- **Abbruch und Fortschritt.** Jede Operation lässt sich abbrechen (Strg+C) und meldet Fortschrittszeilen beim Durchlaufen, Warten und Herunterladen; ein abgebrochener Lauf endet am nächsten Prüfpunkt.
- **Bazel im Besonderen.** Eine Auswahl baut genau den transitiven Abschluss der gewählten Tasks, nichts sonst wird analysiert. `run-many` ist ein einziger Bazel-Aufruf mit allen Labels, gemeinsame Abhängigkeiten laufen also einmal. Ist eine gewählte Task ein Test, lautet der Aufruf `bazel test` (mit `--build_tests_only` nur, wenn jedes gewählte Label ein Test ist); `--skip-cache` fügt `--nocache_test_results` hinzu.

#### Cache-Modell

Ein Ergebnis wird wiederverwendet, wenn der Schlüssel der Task unverändert ist. Jedes Werkzeug leitet seinen Schlüssel aus denselben deklarierten `inputs` ab (Dateien und benannte Gruppen, Umgebungswerte, Ausgaben von `runtime`-Befehlen, der Abschluss der Dateien, die das Befehlsskript importiert, Plattform, Bun-Version, Build-Modus) und aus der Identität des Build-Werkzeugs selbst. Folgen: Eine Datei, die die Task liest, aber nicht deklariert, macht sie nicht ungültig (ein veralteter Treffer, kein Fehler); eine Änderung an der Implementierung des Build-Werkzeugs vergibt die Schlüssel seiner Tasks einmal neu (bei Bazel ist die erfasste Implementierung ein Input jeder Action), erwarte also nach dem Holen einer solchen Änderung einen kalten Lauf; eine Task, die das ganze Repository als Input deklariert, wird bei jeder Änderung neu geschlüsselt und ist ein Problem des Eigentümers, nicht des Caches.

| Schicht | Ort | Geleert durch |
|---|---|---|
| Nx-Task-Cache | `.🧬semio/🦑️repo/⚡️cache/rust/fixed/m/nx/artifacts` (Rust-Partition) | `repo cache clear` (`nx reset --onlyCache` im begrenzten Nx-Namensraum) |
| Bazel-Action-Cache | Output-Base `bazel/outputs/server/<hash>` der Produkte-Partition, einer je erzeugtem Workspace-Verzeichnis; der Server schläft nach 120 s ein | `repo cache clear` stoppt die Server und entfernt die Output-Bases |
| Bazel-Disk-Cache | `.cache/b/p/m/bazel/actions`, abgeräumt bis auf 40 Prozent des Budgets der Produkte-Partition, Einträge über 30 Tage werden verworfen | `repo cache clear`; übersteht `bazel clean` und den Verlust eines Servers |
| Remote-Cache | ein HTTP-Endpunkt, den du konfigurierst, oder `repo cache serve` (Speicher in `.cache/remote-cache`) | die eigene Grenze und Verdrängung des Servers |

Aller Build-Zustand unter `.cache/b/<partition>` ist abgeleitet und darf gelöscht werden, wenn nichts läuft (Partitionen `d` Abhängigkeiten, `t` Werkzeuge, `g` Graph, `o` Beobachtungen, `p` Produkte, `l` Leases, `n` Nx-Pool; siehe Cache-Partitionen). `--skip-cache` ignoriert den lokalen Task-Cache des Laufs (bei Bazel nimmt es zusätzlich keine Remote-Treffer an), `--skip-remote-cache` ignoriert nur den Remote-Cache, und die Dashboard-Achse `cache` bietet `use`, `skip-local` und `skip-all`.

- **Früher Abbruch der Kette (early cutoff).** Unter Bazel führt eine Task, deren Ausgabe-Snapshot nach einer Input-Änderung byte-identisch bleibt, ihre Abhängigen nicht erneut aus (gemessen: Eine reine Leerraum-Änderung am Input eines Generators führte nur den Generator erneut aus).
- **Store-Belege.** Produkte, die in einem geleasten Store statt im Projektverzeichnis liegen, deklarieren einen Beleg als einzigen Output; beide Werkzeuge prüfen ihn nach jedem Lauf und führen genau diese eine Task ungecacht erneut aus, wenn der Beleg beschädigt ist. Siehe Store-Belege.

**Remote-Cache.** Er ist aus, bis `SEMIO_REMOTE_CACHE_ENDPOINT` einen `http://`- oder `https://`-Ursprung nennt; der Vertrag und seine Variablen stehen im Abschnitt Remote-Cache.

```bash
repo cache serve --port 6090
```

```bash
SEMIO_REMOTE_CACHE_ENDPOINT=http://127.0.0.1:6090 SEMIO_REMOTE_CACHE_ACCESS=read-write repo run @teaching/architecture-course:test --tool=bazel
repo cache remote status --tool=bazel
```

In PowerShell lautet dasselbe `$env:SEMIO_REMOTE_CACHE_ENDPOINT = "http://127.0.0.1:6090"`, `$env:SEMIO_REMOTE_CACHE_ACCESS = "read-write"`, danach `bun repo run @teaching/architecture-course:test --tool=bazel`.

- **Zugriff.** Der Standard ist `read-only`; Ergebnisse werden nur mit `SEMIO_REMOTE_CACHE_ACCESS=read-write` und einem `SEMIO_REMOTE_CACHE_UPLOAD`, das nicht `false` ist, hochgeladen. Das Token wird aus der Variablen `SEMIO_REMOTE_CACHE_TOKEN` gelesen (oder aus der Variablen, die `SEMIO_REMOTE_CACHE_TOKEN_ENVIRONMENT` nennt) und steht nie in einem Flag oder länger als einen Build auf der Platte: Bazel erhält es über einen Credential-Helper, Nx über seine eigene Variable. Nx kann Uploads nicht zurückhalten; für Nx bedeutet read-only daher ein Lese-Token auf dem Server.
- **`cache remote status`.** Gibt den aufgelösten Vertrag aus (`configured: false`, solange kein Endpunkt gesetzt ist), die Umsetzung für das gewählte Werkzeug mit ihren `unsupported`-Feldern, ob die Token-Variable gesetzt ist (nie ihren Wert) und eine lesende Sondierung des Endpunkts: `reachable`, `authorized` und den HTTP-Status. Es speichert nichts.
- **`cache serve`.** Stellt das HTTP-Cache-Protokoll von Bazel (`/ac/<sha256>`, `/cas/<sha256>`, Blobs gegen ihren Digest geprüft) und die selbst gehostete Cache-API von Nx (`/v1/cache/<hash>`) von diesem Rechner aus bereit, bis du ihn stoppst. Optionen: `--port` (Standard 6090, `0` wählt einen freien), `--host` (Standard `127.0.0.1`), `--directory`, `--max-bytes` (Standard 4 GiB, mindestens 1 MiB; die am längsten ungenutzten Einträge gehen zuerst). Die Tokens kommen aus `SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN` und `SEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN`; ohne Token weigert sich der Server, außerhalb von Loopback zu lauschen. Das Dashboard-Werkzeug `build-cache-serve` startet ihn.
- **Download-Modi.** `toplevel` (Standard) und `all`. `minimal` gehört nicht zum Vertrag, und das Schema lehnt es ab: Der Bazel-Adapter stellt den Snapshot jeder gewählten Task aus `bazel-bin` wieder her, und mit `minimal` bleibt der Snapshot im Remote-Store, die Wiederherstellung scheitert mit `ENOENT` und nichts erreicht den Checkout (gemessen). Nx holt immer alles.
- **Was geteilt wird.** Bazel-Schlüssel der heutigen Tasks enthalten den absoluten Bun-Pfad und die Checkout-Wurzel der erfassten Implementierung; ein Treffer über Rechner hinweg braucht daher denselben Benutzer und Checkout-Pfad. Hermetische Actions (nächster Abschnitt) haben hostunabhängige Schlüssel (gemessen: dieselben Inputs unter einem anderen Checkout-Pfad trafen den Disk-Cache).

#### Remote-Ausführung

Remote-Ausführung ist optional und wird je Target gewählt. Das Build-System definiert den Vertrag und die Bazel-Umsetzung; es liefert keinen Executor mit.

- **Deklarieren.** Ein Target ist nur dann eine hermetische Action, wenn sein Manifest es sagt: `"metadata": { "semio": { "hermetic": { "version": 1, "tools": ["bun"] } } }` (Schema `🏗️build/🏃️execution/🧬️schema/🔒️hermetic.json`). Es muss außerdem gecacht sein und ein deklariertes `outputs`-Array haben, schlichte `command`-Strings ausführen, nur Datei-Inputs haben (kein `runtime`- oder `env`-Input, kein `{options.…}`- oder `{env.…}`-Token), nicht continuous sein und keine weiteren Builds anstoßen. Ein Target, das nicht passt, behält Host-Zustand; der Grund ist geschlossen (`undeclared`, `unsupported-execution`, `host-inputs`, `uncached`, `continuous`, `orchestration`, `tool`). Kein Target des Repositorys deklariert es bisher, also läuft heute nichts remote.
- **Was eine hermetische Action ist.** Sie läuft mit einem gepinnten Bun, der ein deklarierter Tool-Input ist (unter einem Lease mit Fortschritt geladen, vor dem Entpacken nach Größe und SHA-256 geprüft), in einer Sandbox oder auf einem Worker, mit strenger Umgebung (`LC_ALL=C`, `TZ=UTC`, `SOURCE_DATE_EPOCH=0`, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`), ohne Host-Pfad im Schlüssel, mit normalisierten Outputs, und sie scheitert bei einem nicht deklarierten Schreibzugriff. Hermetische Actions nutzen das gepinnte Bun, nicht das in deinem `PATH`.
- **Schalter.** Ungesetzt bleibt jede Action lokal; ein fehlerhafter Wert scheitert geschlossen.

| Variable | Bedeutung | Standard |
|---|---|---|
| `SEMIO_REMOTE_EXECUTION_ENDPOINT` | `grpc://`-, `grpcs://`- oder `unix:`-URI des Executors | keiner (lokal) |
| `SEMIO_REMOTE_EXECUTION_PLATFORM` | Worker-Plattform: `darwin`, `linux` oder `windows` plus `-arm64` oder `-x64` | `linux-x64` |
| `SEMIO_REMOTE_EXECUTION_PROPERTIES` | zusätzliche Worker-Eigenschaften, `name=value,…` (bis zu 16), zu `OSFamily` und `Arch` der Plattform hinzugefügt | keine |
| `SEMIO_REMOTE_EXECUTION_JOBS` | gleichzeitige Actions, solange der Executor an ist | 128 (höchstens 512) |

- **Routing.** Nur hermetische Actions gehen an den Executor, mit `--noremote_local_fallback`: Ein nicht erreichbarer Executor lässt den Build scheitern, die Action läuft nie lokal. Actions mit Host-Zustand bleiben lokal (`no-remote-exec`).
- **Anforderungen an den Executor.** Ein Dienst für die Remote Execution API v2 von Bazel mit Inhalts-Store, Action-Cache und Scheduler; Worker, deren Eigenschaften zur Plattform passen (Buildbarn vergleicht exakt; BuildBuddy nutzt `container-image` und `pool`); Linux-Worker brauchen nur das hochgeladene Bun, keinen Checkout. Die Ausführungs-Flags tragen nie Cache-Einstellungen: Cache-Endpunkt, Instanz, Upload, Download-Modus und Token bleiben beim Remote-Cache-Vertrag, dessen Endpunkt ein `http(s)`-Ursprung ist. Bazel auf einen einzigen gRPC-Dienst für Cache und Ausführung zu richten, definiert der Vertrag noch nicht.
- **Plattformen.** Die Netzwerkisolation einer hermetischen Action wird unter Linux und auf Remote-Workern durchgesetzt; die Sandbox von macOS kann sie nicht sperren (mit Bazel 8.4.2 gemessen). Unter Windows gibt es keine Sandbox, der Runner lehnt dort eine hermetische Action also ab (der Source-Link ist eine Junction); führe hermetische Windows-Targets remote aus.
- **Optionaler lokaler Stack.** Um den Vertrag ohne Dienst auszuprobieren, genügt jeder lokale Executor, der REAPI v2 auf Loopback spricht, zum Beispiel ein Buildbarn-Stack (Storage, Scheduler, Worker, Runner) in Docker. Er ist kein `repo`-Befehl und für nichts sonst in diesem Leitfaden nötig; das Build-System behandelt ihn wie jeden anderen Executor, setze also die vier Variablen oben auf seine Adresse.

#### Worker-Pools

Persistente Worker halten einen Prozess zwischen Actions am Leben, sodass eine Action nicht jedes Mal das Starten und Laden des Build-Werkzeugs bezahlt.

- **Schalter.** `--workers on|off`, dann `SEMIO_BUILD_WORKERS`, dann die Worker-Richtlinie in `📚️library/🏗️build/🔣️policy.json` (`enabled`, standardmäßig an). `--parallel <n>` oder `SEMIO_BUILD_PARALLEL` setzt die Zahl gleichzeitiger Tasks (Standard: die Zahl der Kerne, höchstens 8).
- **Arten.** Die Art ergibt sich aus Target-Name und Dauerbetrieb, nie aus dem Werkzeug; die erste passende Regel gewinnt.

| Art | Target-Namen | Höchstzahl Instanzen |
|---|---|---|
| `service` | `continuous: true` | keine (nie ein Worker) |
| `test` | `test`, `smoke`, `mutation`, `bench`, `e2e` (auch `<name>-…`) | 8 |
| `check` | `check`, `lint`, `verify`, `typecheck`, `format-check`, `contract`, `schema`, `stdio` (auch `<name>-…`) und Namen mit Endung `-check` | 8 |
| `generate` | Namen mit `generate` oder `generator` | 4 |
| `build` | `build`, `prepare`, `activate`, `wasm`, `native`, `package`, `bundle`, `cpp`, `publish`, `canonical`, `preview`, `setup`, `deps` (auch `<name>-…`) | 2 |
| `script` | alles andere | 4 |

- **Budget.** Alle Pools zusammen dürfen 30 Prozent des Hauptspeichers des Rechners belegen, 512 MB je Worker. Jede Art erhält zuerst eine Instanz (test, check, generate, build, script, soweit der Speicher reicht), die übrigen Instanzen gehen in derselben Reihenfolge an die Arten bis zu ihrer Grenze und der Jobzahl; eine Art, der keine bleibt, läuft ohne Worker.
- **Bazel.** Die Worker sprechen das JSON-Worker-Protokoll von Bazel, führen den Action-Rumpf im Worker-Prozess aus (Blattbefehle bleiben eigene Prozessbäume), unterstützen Abbruch und verkleinern den Pool unter Speicherdruck. Worker- und Nicht-Worker-Actions haben denselben Action-Schlüssel, Caches werden also über beide Modi geteilt. Actions, die nie Worker nutzen: Dienste, hermetische Actions und als `bazel test` laufende Tests. Cargo-Tasks laufen nacheinander.
- **Nx.** Nx startet je Task einen Prozess, abgebildet wird daher nur `NX_PARALLEL` (auf die Jobzahl); `NX_BATCH_MODE` wird nur für einen Executor gesetzt, der eine Batch-Implementierung registriert, was keiner tut. Jede andere Worker-Einstellung (Pools, Instanzgrenzen, Speicher, Sandbox) wird unter Nx nicht unterstützt.
- **Gemessen.** Die CPU-Zeit je Action sank im eingeschwungenen Zustand von 2,50 s auf 1,46 s; die Wanduhrzeit war auf einem ausgelasteten Rechner nicht durchgehend besser, weil 85 Prozent einer warmen Anfrage im Worker das Öffnen des Graph-Cache-Stores sind.
- **Ausschalten** mit `--workers off`, um gegen einen Prozess je Action zu vergleichen, eine einzelne Task zu debuggen oder wenn Speicher knapp ist.

#### Task-Dispositionen

Jede Task hat eine Werkzeugfamilie (`bun-script`, `tsc`, `vitest`, `bun-test`, `vite`, `playwright`, `go`, `python`, `dotnet`, `latex`, `wasm`, `cargo`, `docker`, `aggregate`) und eine Disposition. Nx führt alle als `nx run` aus; Bazel bildet sie so ab (`📚️library/🏗️build/🏃️execution/🧬️schema/🎯️coverage.json`).

| Disposition | Bedeutung | Unter Bazel |
|---|---|---|
| `action` | gecacht, aus dem Action-Cache wiedergegeben | gecachte Build-Action, Output-Snapshot wird in den Checkout zurückgespielt |
| `test` | gecacht und `test` oder `test-…` benannt, `outputs: []`, keine andere gewählte Task verbraucht sie, höchstens 10.000 deklarierte Dateien | `bazel test`: das bestandene Ergebnis wird gecacht, beim nächsten Lauf `(cached) PASSED`; nicht verwendet, wo die Plattform keine Runfiles hat (Windows) |
| `interactive` | `continuous`, läuft bis zum Stopp | `bazel run`-Target; es startet bei jedem Lauf und wird nie erzeugt |
| `uncached` | läuft immer; der festgehaltene Grund sagt warum | Action mit Tag `no-cache` |
| `refused` | `cache` oder `outputs` nicht deklariert (Gründe `undeclared`, `unbounded`) | fail-closed, bis der Eigentümer deklariert, was fehlt |

Statische Zählung vom 2026-10-20 (8.028 Tasks des rekonstruierten Graphen, mit den angewendeten Klassifikationsmengen): 3.771 gecacht (Actions und Tests), 1.320 interactive, 2.937 ungecacht mit festgehaltener Klasse, 0 refused und 0 ungecacht mit dem Grund `declared` (vorher 373).

#### Ein neues Target einordnen

Beantworte der Reihe nach; das erste Ja entscheidet.

1. **Läuft es bis zum Stopp** (Server, Watcher, Shell)? `"continuous": true`, Klasse `continuous`, Grund `serves-until-stopped`. Es wird `interactive`.
2. **Liest es nur Repository-Dateien, schreibt es nur deklarierte Outputs (oder keine), nutzt es kein Netzwerk, keinen Host-Store, kein Lease, keine Uhr und keinen laufenden Prozess?** Dann cache es: `"cache": true`, `inputs` für alles, was es außerhalb seines Projekts liest, `outputs` (`[]` für eine reine Prüfung) und gib ihm eine gecachte Namensfamilie oder seine eigene Entscheidung (unten).
3. **Sonst** `"cache": false` plus die Entscheidung, die den Grund nennt.

| Klasse | Gründe | Verwendung |
|---|---|---|
| `deterministic` | `pure-function-of-inputs` (Outputs deklarieren), `check-without-outputs` | gecachte Prüfungen, Tests, Generatoren mit deklarierten Outputs |
| `continuous` | `serves-until-stopped` | Server, Watcher, Shells |
| `live` | `drives-running-process`, `exercises-host-resources` | E2E gegen einen laufenden Prozess, Tests echter Locks, Speicher, Prozessbäume |
| `side-effecting` | `mutates-workspace`, `mutates-host-store`, `publishes-remote`, `publishes-live-state` | Formatierer, Gerüstbauer, Installer, Releases |
| `diagnostic` | `measures-environment`, `reads-caller-input`, `admission-refusal` | Benchmarks, Doctors, von ihren Argumenten gesteuerte Werkzeuge |
| `aggregate` | `orders-prerequisites`, `reenters-build-tool` | reine Reihenfolge; ein Verb, das andere Targets über `repo` ausführt, sollte sie stattdessen in `dependsOn` deklarieren |
| `managed-output`, `driver-owned` | `outputs-in-managed-store`, `outputs-are-store-links`, `freshness-owned-by-driver` | im Store veröffentlichte Builds und vom Treiber verantwortete Playground-Schritte |

Die Richtlinie löst einen Target-Namen in dieser Reihenfolge auf (`🎯️target-policy`): continuous-Namen, `cache: false`, die uncached-Namen der Richtlinie, mutierende und Live-Namen, dann deine Entscheidung in `metadata.semio.cache`, dann die `targetClasses` der Richtlinie (Namensfamilien wie `test`, `check`, `lint`, `typecheck`, `verify`, `build`, `generate`), dann ein `verify`-Befehl. **Ein Target, das nichts davon einordnet, wird nie wiedergegeben** (`unclassified`, `no-cache-classification`), und `"cache": true` allein ändert das nicht; gib einem ungewöhnlichen Namen daher seine Entscheidung:

```json
"digest-inputs": {
  "executor": "repo:script",
  "cache": true,
  "inputs": ["default", "^default"],
  "outputs": [],
  "options": { "command": "bun ./📜️script.ts digest-inputs" },
  "metadata": { "semio": { "cache": { "version": 1, "class": "deterministic", "reason": "check-without-outputs" } } }
}
```

```json
"measure-startup": {
  "executor": "repo:script",
  "cache": false,
  "outputs": [],
  "options": { "command": "bun ./📜️script.ts measure-startup" },
  "metadata": { "semio": { "cache": { "version": 1, "class": "diagnostic", "reason": "measures-environment" } } }
}
```

- **Outputs.** Ein gecachtes Target muss jeden deklarierten Output erzeugen, der kein Glob ist: Sonst scheitert es mit „Declared output of a replayed Bazel task was not produced“. Setze `metadata.semio.optionalOutputs: true` nur für einen Output, der zu Recht fehlen darf. Ein `pure-function-of-inputs`-Target ohne deklarierte Outputs wird zu `side-effecting` herabgestuft.
- **Test unter Bazel.** Nenne es `test` oder `test-<was>`, halte es gecacht mit `outputs: []` und unter 10.000 deklarierten Dateien (jede deklarierte Datei wird vor jedem Lauf in die Runfiles des Tests gelinkt, auch bei einem Treffer).
- **Remote-Ausführung.** Füge die `hermetic`-Deklaration (siehe Remote-Ausführung) nur hinzu, wenn das Target wirklich allein seine deklarierten Inputs liest.
- **Gesetz.** Das Cache-Korrektheits-Gesetz (`📚️library/⚖️laws/🔒️cache-correctness`) liest jedes Manifest durch die Produktionsrichtlinie und schlägt fehl bei einem nicht eingeordneten Target, einem gecachten Target, das die Richtlinie überstimmt, einem ungecachten Target ohne festgehaltenen Grund, leeren Inputs, einem Erzeuger ohne Outputs, einer nicht gehashten Umgebungsüberschreibung, einem benannten Input, den niemand deklariert, Outputs, die ausbrechen oder sich überlappen, und einem mutierenden oder Netzwerk-Befehl in einem gecachten Target.

#### Inkrementelle Builds und ihre Prüfung

Ein inkrementeller Build führt genau die Tasks erneut aus, deren deklarierte Inputs sich geändert haben, und gibt den Rest wieder. Beweise es mit den vorhandenen Operationen; der CI-Job `smoke` ist dafür geschrieben, dasselbe für beide Werkzeuge auf `@teaching/architecture-course:test` zu tun (der Workflow ist noch nie gelaufen).

```bash
repo cache clear --tool=bazel
repo cache status --tool=bazel
repo run @teaching/architecture-course:test --tool=bazel
repo cache status --tool=bazel
repo run @teaching/architecture-course:test --tool=bazel
repo cache status --tool=bazel
```

1. Notiere die Zahlen nach `cache clear`; nach dem ersten Lauf sind `entries` und `bytes` gewachsen, und bei Bazel zählt `activity.executed` die Actions, die liefen.
2. Der zweite Lauf startet keine Arbeit: Die Zusammenfassung von Bazel zeigt keinen `local`-, `worker`- oder Sandbox-Spawn (oder `disk cache hit`), `entries` wachsen nicht und `activity.executed` bleibt, während `activity.cacheHits` die Treffer zählt.
3. Ändere eine Datei, die die Task deklariert: Nur diese Task und ihre Abhängigen laufen (ein Abhängiger mit unverändertem Input-Snapshot wird wiedergegeben). Ändere eine Datei, die sie nicht deklariert, und nichts läuft: Die Input-Liste ist zu kurz.
4. `repo cache clear` gefolgt vom Lauf belegt einen kalten Fehltreffer; `--skip-cache` erzwingt die Ausführung einmal ohne Leeren; `--dry-run` zeigt den Plan, den der Lauf ausführen würde.
5. Erwarte zusätzliche Arbeit nach: Änderungen an den eigenen Dateien des Build-Werkzeugs (jede Action wird einmal neu geschlüsselt), dem Wechsel zwischen `prototype` und `release` (der Modus gehört zu jedem Schlüssel) und der Änderung eines deklarierten Inputs während des Builds (Bazel gibt `Skipping uploading outputs because of concurrent modifications` aus und der nächste Lauf führt erneut aus).

#### Hinweise je Plattform

| Rechner | Hinweise |
|---|---|
| macOS | Cache-Partitionen sind case-sensitive APFS-Images, ohne Privileg. Halte den Checkout-Pfad ASCII (Bazel stürzt beim Start bei einer Nicht-ASCII-Install-Base ab, und seine Sandbox kodiert eine Nicht-ASCII-Output-Base falsch; das Layout wird vorab abgelehnt). Der Adapter setzt eine UTF-8-Locale. |
| Linux | Verzeichnis-basierte Partitionen, kein Root und kein Loop-Device; verlangt ein case-sensitives Dateisystem mit atomarem Verzeichnisaustausch (ext4, xfs, btrfs). Ein Nicht-ASCII-`install`-Pfad wird abgelehnt; eine Nicht-ASCII-Output-Base funktioniert. In einem unprivilegierten Container fällt Bazel auf `processwrapper-sandbox` zurück (gleiche Ergebnisse, schwächere Isolation); `linux-sandbox` braucht einen privilegierten Container. Der Adapter nennt nie eine Sandbox-Strategie. |
| Devcontainer | Wie Linux. `.cache` ist ein benanntes Volume, privat für den Checkout (ein Bind-Mount von einem Host ohne Groß-/Kleinschreibung wird von `repo preflight` abgelehnt); `postCreateCommand` führt `repo run workspace:setup` aus. |
| Windows | Verzeichnis-Partitionen, getauscht durch eine journalisierte Umbenennung auf NTFS mit Groß-/Kleinschreibung je Verzeichnis. Verlangt `LongPathsEnabled` und eine kurze ASCII-Checkout-Wurzel (das Pfadbudget von Bazel ist `len(output base) + len(\execroot\_main\s\) + 256` gegen 259, oder 32766 mit langen Pfaden). Der Source-Link ist eine Junction (kein Entwicklermodus, keine Erhöhung), Runfiles sind aus (`--noenable_runfiles`, Startup-Option `--nowindows_enable_symlinks`), Tests laufen daher als gecachte Actions statt als `bazel test`, und hermetische Actions laufen nur remote. Setze Variablen in PowerShell mit `$env:NAME = "value"` und rufe `bun repo` auf. Unter Windows nicht ausgeführt. |

Kein Cache-Pfad darf auf irgendeinem Rechner 256 Zeichen überschreiten. Bazel und Bun sind je Betriebssystem und CPU (darwin, linux, windows auf arm64 und x64) mit Prüfsummen gepinnt, `repo tools setup` braucht also keinen manuellen Download.

#### Fehlersuche

- **Jeder Befehl wird nach einem Neustart oder Absturz abgelehnt** („Exact canonical persistent Native publication required …“). Belege halten die Boot-Sitzung fest. Unter macOS, wo ein Image-Beleg die Gerätenummer festschreibt, bindet das erste Öffnen jeder Partition nach einem Neustart ihren Beleg unter dem exklusiven Schreiber-Lease an die neue Gerätenummer um, aber nur, wenn das Image nicht angehängt ist, darunter nichts eingehängt ist und die Verzeichnisidentität (Inode, Größe, Belegung, Mount-Verzeichnis) unverändert ist; führe einen Befehl aus, der die Stores öffnet, etwa `repo cache status`, und lass ihn zu Ende laufen. Eine Gerätenummer, die sich innerhalb eines Starts unterscheidet, wird nie umgebunden. Nennt die Ablehnung veränderte Tatsachen, lege die Partition neu an (Wiederherstellung, Punkt 2).
- **Ein Befehl wartet.** Stores werden über eine First-in-first-out-Lease-Warteschlange beansprucht. Ein Wartender ab Position 2 gibt alle 15 Sekunden eine `lease-queue`-Zeile aus, die die Prozess-ID des Halters nennt; eine Zeile `remains active on …` ist der Fortschritt eines laufenden Durchlaufs, kein Warten. Beende keinen Halter, der vorankommt; lies zuerst das Kopf-Ticket.
- **Ein Beleg scheitert.** Store-Beleg: Eine Task läuft ungecacht erneut (`[store-receipt] re-executing`). Generations-Beleg der Abhängigkeiten: Die Änderung einer Datei seines Abschlusses kostet den nächsten Befehl einen vollständigen Durchlauf (Fortschritt `dependencies:generation-verify:<n>`). Image-Beleg: siehe den ersten Punkt.
- **Kein Cache-Treffer.** Prüfe der Reihe nach: `--skip-cache` oder die Dashboard-Achse `cache` auf einem Skip-Wert; die Entscheidung des Targets im Manifest (`unclassified` wird nie wiedergegeben); ein nicht deklarierter oder zu weiter Input; der Modus (`prototype` oder `release`); ein anderer Prozess, der deklarierte Inputs während des Builds ändert; bei einem Remote-Treffer ein anderer Benutzer oder Checkout-Pfad.
- **Remote-Cache.** `repo cache remote status` zeigt `configured: false`, wenn kein Endpunkt gesetzt ist, `reachable: false`, wenn der Endpunkt nicht antwortet, und `authorized: false` bei einem falschen Token (Nx bricht einen Lauf bei 401 ab und ignoriert 403 beim Upload stillschweigend).
- **Worker.** Starte mit `--workers off` neu, um ein Worker-Problem von einem Task-Problem zu trennen. Ein Build, der mit „Captured implementation Source changed“ scheitert, hat erlebt, dass sich eine Datei des Build-Werkzeugs unter ihm änderte; starte ihn neu.
- **Installierte Pakete.** `Cannot find module` für ein Workspace-Paket heißt, dass die Abhängigkeiten älter sind als die Quellen: `repo dependencies lock --all`, dann `repo dependencies sync --all`. Auf einem frischen Klon braucht die Nx-Einrichtung möglicherweise die installierten Workspace-Links: Stoppt sie mit `Cannot find module`, führe zuerst den Sync aus.
- **Nx meldet `.nx/installation` fehlend.** Führe `repo tools setup nx` aus.

#### Verifiziert und noch nicht verifiziert

Verifiziert am 2026-10-20 (macOS arm64, Bazel 8.4.2, Nx 23.2.0; Linux-arm64-Container für die Plattformzeilen): echte Bazel-Läufe kalt, warm, nach `clean` und nach Serververlust mit dem Produktions-Task-Eigentümer, früher Abbruch der Kette, Tests als gecachtes `bazel test`, `affected` gegen Bazel-`rdeps` und die Locators von Nx, echtes Bazel und ein unverändertes Nx gegen `repo cache serve` (Remote-Treffer nach dem Leeren des lokalen Caches, falsche und Nur-Lese-Tokens), persistente Worker gegen echtes Bazel, das Hermetik-Gesetz, die Einrichtung im Linux-Container und die Fixture-Suites jeder Komponente. Noch nicht verifiziert: ein Produktionslauf `repo --tool=bazel run` nach der Integration, Nx mit der gepatchten Installation des Repositorys gegen einen Remote-Cache, ein echter Remote-Executor und die Remote-Ausführung irgendeines Repository-Targets, Windows, der Lebenszyklus des Devcontainers und jede End-to-End-Kette unter beiden Werkzeugen (siehe Verifikationsstand).
