# 🧺️ Demonstrator: Entwerfen mit Bestand

The demonstrator of the research project "Entwerfen mit Bestand" (`@semio-tech/mit-bestand-demonstrator`): a German-locked landing page that shows eight live panes of the framework OS (`generator`, `koordinator`, `aggregator`, `aussuchen`, `bearbeiten`, `verfolgen`, `energy`, `fem3d`; the catalog is `🔨️modules/🧩️runtime/🔣️.json`, the brands `🪧️brand.ts`). The dev server listens on port `6029`; a release build assembles the static site (`build`). A German version of the developer chapter follows the last section.

| Path | What |
|---|---|
| `📋️project.json` | the neutral manifest: the targets below, their dependencies, ports and cache flags |
| `📜️script.ts` | the router of `test` (vitest, `🧪️tests/🎚️config/🟦️.ts`) and `runtime-catalog-test` |
| `🔨️modules/🧩️runtime/` | the pane catalog, `prepare`, `complete`, `activate`, `serve`, `prepare-test` and `serve-test` (`📜️script.ts`) and the activation reader (`♻️activation`) |
| `🔨️modules/📦️site/` | `build` (`📜️script.ts`) with the map tile prefetch (`🗺️map-tiles`) |
| `🔨️modules/🧪️e2e/` | the Playwright runner and its configuration (`🎚️config/🟦️.ts`) |
| `🏗️builder/🌐️vite/`, `🟦️.tsx`, `🌐️.html`, `🐚️shell` | the Vite configuration, the landing page and the shell |
| `🧪️tests/` | unit and law tests, and `🎭️acceptance/🟦️.ts`, the one spec Playwright runs |

## Develop

Every command is a target of the project `@semio-tech/mit-bestand-demonstrator` and runs through the tool-neutral build entry: `bun repo [--tool=nx|bazel] run @semio-tech/mit-bestand-demonstrator:<target>[:<configuration>] [-- <arguments>]`. Nx with Bun is the default; Bazel executes the same graph with `--tool=bazel` (or `SEMIO_BUILD_TOOL=bazel`). Bazel on macOS needs an ASCII checkout path. The [build chapter](../../🧰️framework/🛍️products/🦑️repo/README.md#-build-and-run) documents both implementations, the cache rules and the recovery steps; never call `nx`, `bazel`, raw `cargo` or `rustc` yourself. In the dashboard (`bun run dashboard`, **New Task**, `Ctrl+B n`) search for `mit-bestand-demonstrator` plus the operation; the dashboard command id is the `<project>:<target>` id used here, the global `build-tool` axis of its form selects Nx or Bazel, and **Open ready address** (`Ctrl+B u`) opens a ready server.

### Prerequisites

| Need | How | Needed by |
|---|---|---|
| Pinned build tools | `bun repo tools setup nx` (Bazel: `bun repo tools setup bazel`, both: `all`; `bun repo run workspace:setup` includes it) | every target |
| Dependencies | `bun repo dependencies sync --all`; after any `package.json` edit run `bun repo dependencies lock --all` first, and never a plain `bun install` | every target |
| Host check | `bun repo preflight` | before the cache partitions are created |
| Rust and wasm | no unmanaged installation: the toolchain and the wasm targets of `rust-toolchain.toml`, reached through `bun repo cargo --store=shared` by the lane and wasm tasks; a cold run compiles the nine lanes and four wasm engines (not timed) | `prepare-prototype`, `prepare-release`, `activate-prototype` and everything after them |
| Network | `build` prefetches the map tiles its bundle embeds and fails when the tile cache stays incomplete | `build` |
| Chromium for Playwright | `bun repo run workspace:deps-browsers` runs `playwright install chromium` into `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, which is a download; the revision is the one the installed `playwright-core` names | `prepare-e2e` depends on it |
| A GPU adapter | every pane drives a GPU surface and the map pane only boots on a real adapter: ANGLE Metal on macOS, ANGLE GL elsewhere; `DEMONSTRATOR_E2E_GPU=swiftshader` forces the software stack for hosts without one, and the map pane cannot be graded there | `test-e2e` |

The demonstrator needs neither Docker nor a Linux host.

### Commands

| Command (`bun repo run …`) | What it does | Cache | Port |
|---|---|---|---|
| `@semio-tech/mit-bestand-demonstrator:prepare-prototype` | after the nine `@semio-tech/framework-os-dev:prepare-<lane>-react-prototype` tasks (`aggregator`, `aussuchen`, `bearbeiten`, `energy`, `fem3d`, `generation3d`, `generator`, `koordinator`, `verfolgen`) and the wasm tasks of surface, editor, node graph and puzzle: verifies every runtime component and every staged session (`Mismatched Demonstrator session` otherwise) | yes (output-free) | |
| `@semio-tech/mit-bestand-demonstrator:prepare-release` | the same for the profile `release` | yes (output-free) | |
| `@semio-tech/mit-bestand-demonstrator:activate-lanes-prototype` | after `activate-energy-react-prototype`, `activate-fem3d-react-prototype` and `activate-generator-react-prototype`; only logs that the lanes are complete | yes | |
| `@semio-tech/mit-bestand-demonstrator:activate-prototype` | after `prepare-prototype` and `activate-lanes-prototype`: reads the lane receipts and writes the union receipt `dist/♻️activation/prototype` | yes | |
| `@semio-tech/mit-bestand-demonstrator:dev`, `serve` | the Vite server over the activated prototype; both need `activate-prototype` | never (continuous) | `6029` (`MIT_BESTAND_DEMONSTRATOR_PORT`) on `127.0.0.1`, or `0.0.0.0` when `DEVCONTAINER=true` |
| `@semio-tech/mit-bestand-demonstrator:build` | the static site: after `prepare-release` and the four wasm tasks, prefetches the map tiles and builds the Vite artifact; it declares its store receipt `.🧾️store/build.json` | never | |
| `@semio-tech/mit-bestand-demonstrator:test` | the unit and law tests; the level is an argument (`-- long`, `exhaustive`; the default is `fundamental`) | never | |
| `@semio-tech/mit-bestand-demonstrator:canonical-architecture`, `test-runtime-catalog` | the shell brand law at level `long`; the runtime catalog test | never | |
| `@semio-tech/mit-bestand-demonstrator:prepare-e2e` | after `activate-prototype` and `workspace:deps-browsers`: opens an isolated service session | never | |
| `@semio-tech/mit-bestand-demonstrator:serve-e2e` | after `prepare-e2e`: serves the activated prototype on a free OS port and announces its address through the session | never (continuous) | `0` (OS-selected) |
| `@semio-tech/mit-bestand-demonstrator:test-e2e` | after `serve-e2e`: waits for the announced address and runs Playwright (`🧪️tests/🎭️acceptance/🟦️.ts`, one chromium project, one worker, two retries when `CI` is set) against it | never | |

The lane tasks are inferred from the playground registry: `prepare-<variant>-react-<profile>` is driver-owned and never cached (the native build driver decides freshness itself), `activate-<variant>-react-<profile>` is cached (`deterministic`) with the outputs `.cache/b/p/m/browser/<profile>/runtime/react/<variant>`. `prepare-prototype`, `prepare-release`, `activate-lanes-prototype` and `activate-prototype` of the demonstrator are cached; `prepare-*` and `activate-prototype` hash the sources of the project and of its workspace dependencies without tests (`activationSources`) together with the outputs of their dependencies. `build` is not cached: it stages into the immutable products store rather than into a declared output tree. Under Bazel a cached target is an action with its declared inputs and outputs and uses Bazel's action cache; a target whose manifest does not allow replay is executed every time.

### The chain

```bash
bun repo tools setup all                                                       # once per host
bun repo dependencies sync --all                                               # once per checkout and after every lockfile change
bun repo run @semio-tech/mit-bestand-demonstrator:activate-prototype           # prepare, then activate (cached)
bun repo run @semio-tech/mit-bestand-demonstrator:serve                        # http://127.0.0.1:6029 until stopped
bun repo run workspace:deps-browsers                                           # Chromium; test-e2e also depends on it
bun repo run @semio-tech/mit-bestand-demonstrator:test-e2e                     # serve-e2e, then Playwright; independent of the dev server on 6029
bun repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:serve           # the same chain under Bazel
bun repo run @semio-tech/mit-bestand-demonstrator:build --configuration=release   # the static site
```

`test-e2e` runs its own server on a free port; the dev server on `6029` does not have to run. Reports land in `dist/reports/e2e/<session id>`, or in `🗑️generated/demonstrator-e2e/<session id>` of the ticket named by `SEMIO_TICKET_DIR`.

### Dashboard registry

The registry is data beside the manifest (`metadata.semio.dashboard` of `📋️project.json` and of the root `📋️project.json`, validated against `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json`):

| Entry | Content |
|---|---|
| readiness | `dev` and `serve` report ready on port `6029` (`MIT_BESTAND_DEMONSTRATOR_PORT`) |
| global axes | `build-tool` (Nx or Bazel), `cache` (use, skip local, skip all) and `build-mode`, offered on every verb that runs build tasks ([Command registry](../../🧰️framework/🛍️products/🦑️repo/README.md#-command-registry)) |

Not yet verified: no run of this chain under either tool is recorded for this documentation. Target names, dependencies, ports, cache flags and registry entries are read from the manifest, the scripts and the Playwright configuration on 2026-10-20. Browsers of the revision `playwright-core` names were not installed on 2026-10-19, so `test-e2e` first downloads them. The runtime catalog test (`🔨️modules/🧩️runtime/🧪️tests/🟦️.ts`, run as `bun test` on that file with `SEMIO_WORKSPACE_ROOT` set to the absolute repository root) passed on 2026-10-20; it is the only part of this chapter that was run.

## 🇩🇪️ Deutsch

### Entwickeln

Jeder Befehl ist ein Target des Projekts `@semio-tech/mit-bestand-demonstrator` und läuft über den werkzeugneutralen Build-Einstieg: `bun repo [--tool=nx|bazel] run @semio-tech/mit-bestand-demonstrator:<target>[:<configuration>] [-- <Argumente>]`. Nx mit Bun ist der Standard; Bazel führt denselben Graphen mit `--tool=bazel` (oder `SEMIO_BUILD_TOOL=bazel`) aus. Bazel braucht unter macOS einen ASCII-Checkout-Pfad. Das [Build-Kapitel](../../🧰️framework/🛍️products/🦑️repo/README.md#-deutsch) beschreibt beide Implementierungen, die Cache-Regeln und die Wiederherstellung; rufe weder `nx`, `bazel`, rohes `cargo` noch `rustc` selbst auf. Im Dashboard (`bun run dashboard`, **New Task**, `Ctrl+B n`) suchst du nach `mit-bestand-demonstrator` und der Operation; die Dashboard-Befehls-ID ist die hier verwendete ID `<Projekt>:<Target>`, die globale Achse `build-tool` im Formular wählt Nx oder Bazel, und **Open ready address** (`Ctrl+B u`) öffnet einen bereiten Server.

### Voraussetzungen

| Bedarf | Wie | Benötigt von |
|---|---|---|
| Gepinnte Build-Werkzeuge | `bun repo tools setup nx` (Bazel: `bun repo tools setup bazel`, beide: `all`; `bun repo run workspace:setup` enthält es) | jedes Target |
| Abhängigkeiten | `bun repo dependencies sync --all`; nach jeder Änderung einer `package.json` zuerst `bun repo dependencies lock --all`, nie ein einfaches `bun install` | jedes Target |
| Host-Prüfung | `bun repo preflight` | vor dem Anlegen der Cache-Partitionen |
| Rust und Wasm | keine selbst verwaltete Installation: Toolchain und Wasm-Ziele aus `rust-toolchain.toml`, von den Lane- und Wasm-Tasks über `bun repo cargo --store=shared` erreicht; ein kalter Lauf kompiliert die neun Lanes und vier Wasm-Engines (nicht gemessen) | `prepare-prototype`, `prepare-release`, `activate-prototype` und alles danach |
| Netzwerk | `build` lädt die Kartenkacheln vorab, die sein Bundle einbettet, und schlägt fehl, wenn der Kachel-Cache unvollständig bleibt | `build` |
| Chromium für Playwright | `bun repo run workspace:deps-browsers` führt `playwright install chromium` nach `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` aus, ein Download; die Revision nennt das installierte `playwright-core` | `prepare-e2e` hängt davon ab |
| Ein GPU-Adapter | jede Pane steuert eine GPU-Fläche, und die Karten-Pane startet nur mit einem echten Adapter: ANGLE Metal auf macOS, sonst ANGLE GL; `DEMONSTRATOR_E2E_GPU=swiftshader` erzwingt den Software-Stack für Hosts ohne GPU, die Karten-Pane lässt sich dort nicht bewerten | `test-e2e` |

Der Demonstrator braucht weder Docker noch einen Linux-Host.

### Befehle

| Befehl (`bun repo run …`) | Was er tut | Cache | Port |
|---|---|---|---|
| `@semio-tech/mit-bestand-demonstrator:prepare-prototype` | nach den neun Tasks `@semio-tech/framework-os-dev:prepare-<Lane>-react-prototype` (`aggregator`, `aussuchen`, `bearbeiten`, `energy`, `fem3d`, `generation3d`, `generator`, `koordinator`, `verfolgen`) und den Wasm-Tasks von Surface, Editor, Node-Graph und Puzzle: prüft jede Laufzeitkomponente und jede bereitgestellte Sitzung (sonst `Mismatched Demonstrator session`) | ja (ohne Ausgabedateien) | |
| `@semio-tech/mit-bestand-demonstrator:prepare-release` | dasselbe für das Profil `release` | ja (ohne Ausgabedateien) | |
| `@semio-tech/mit-bestand-demonstrator:activate-lanes-prototype` | nach `activate-energy-react-prototype`, `activate-fem3d-react-prototype` und `activate-generator-react-prototype`; meldet nur, dass die Lanes vollständig sind | ja | |
| `@semio-tech/mit-bestand-demonstrator:activate-prototype` | nach `prepare-prototype` und `activate-lanes-prototype`: liest die Lane-Quittungen und schreibt die Vereinigungsquittung `dist/♻️activation/prototype` | ja | |
| `@semio-tech/mit-bestand-demonstrator:dev`, `serve` | der Vite-Server über dem aktivierten Prototyp; beide brauchen `activate-prototype` | nie (dauerhaft) | `6029` (`MIT_BESTAND_DEMONSTRATOR_PORT`) auf `127.0.0.1`, bei `DEVCONTAINER=true` auf `0.0.0.0` |
| `@semio-tech/mit-bestand-demonstrator:build` | die statische Seite: nach `prepare-release` und den vier Wasm-Tasks lädt es die Kartenkacheln vorab und baut das Vite-Artefakt; es deklariert seine Store-Quittung `.🧾️store/build.json` | nie | |
| `@semio-tech/mit-bestand-demonstrator:test` | die Unit- und Gesetzestests; die Stufe ist ein Argument (`-- long`, `exhaustive`; Standard ist `fundamental`) | nie | |
| `@semio-tech/mit-bestand-demonstrator:canonical-architecture`, `test-runtime-catalog` | das Shell-Brand-Gesetz auf Stufe `long`; der Test des Laufzeitkatalogs | nie | |
| `@semio-tech/mit-bestand-demonstrator:prepare-e2e` | nach `activate-prototype` und `workspace:deps-browsers`: öffnet eine isolierte Dienst-Sitzung | nie | |
| `@semio-tech/mit-bestand-demonstrator:serve-e2e` | nach `prepare-e2e`: bedient den aktivierten Prototyp auf einem freien Port des Betriebssystems und meldet seine Adresse über die Sitzung | nie (dauerhaft) | `0` (vom Betriebssystem gewählt) |
| `@semio-tech/mit-bestand-demonstrator:test-e2e` | nach `serve-e2e`: wartet auf die gemeldete Adresse und führt Playwright dagegen aus (`🧪️tests/🎭️acceptance/🟦️.ts`, ein Chromium-Projekt, ein Worker, zwei Wiederholungen bei gesetztem `CI`) | nie | |

Die Lane-Tasks werden aus der Playground-Registry abgeleitet: `prepare-<Variante>-react-<Profil>` gehört dem Treiber und ist nie gecacht (der native Build-Treiber entscheidet selbst über Frische), `activate-<Variante>-react-<Profil>` ist gecacht (`deterministic`) mit den Ausgaben `.cache/b/p/m/browser/<Profil>/runtime/react/<Variante>`. Beim Demonstrator sind `prepare-prototype`, `prepare-release`, `activate-lanes-prototype` und `activate-prototype` gecacht; `prepare-*` und `activate-prototype` hashen die Quellen des Projekts und seiner Workspace-Abhängigkeiten ohne Tests (`activationSources`) zusammen mit den Ausgaben ihrer Abhängigkeiten. `build` ist nicht gecacht: Es legt in den unveränderlichen Produkt-Store ab, nicht in einen deklarierten Ausgabebaum. Unter Bazel ist ein gecachtes Target eine Aktion mit ihren deklarierten Ein- und Ausgaben und nutzt Bazels Aktions-Cache; ein Target, dessen Manifest keine Wiedergabe erlaubt, wird jedes Mal ausgeführt.

### Die Kette

```bash
bun repo tools setup all                                                       # einmal je Host
bun repo dependencies sync --all                                               # einmal je Checkout und nach jeder Lockfile-Änderung
bun repo run @semio-tech/mit-bestand-demonstrator:activate-prototype           # prepare, dann activate (gecacht)
bun repo run @semio-tech/mit-bestand-demonstrator:serve                        # http://127.0.0.1:6029 bis zum Stoppen
bun repo run workspace:deps-browsers                                           # Chromium; test-e2e hängt ebenfalls davon ab
bun repo run @semio-tech/mit-bestand-demonstrator:test-e2e                     # serve-e2e, dann Playwright; unabhängig vom Dev-Server auf 6029
bun repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:serve           # dieselbe Kette unter Bazel
bun repo run @semio-tech/mit-bestand-demonstrator:build --configuration=release   # die statische Seite
```

`test-e2e` startet einen eigenen Server auf einem freien Port; der Dev-Server auf `6029` muss nicht laufen. Berichte liegen in `dist/reports/e2e/<Sitzungs-ID>` oder in `🗑️generated/demonstrator-e2e/<Sitzungs-ID>` des Tickets, das `SEMIO_TICKET_DIR` nennt.

### Dashboard-Register

Das Register sind Daten neben dem Manifest (`metadata.semio.dashboard` in `📋️project.json` und im `📋️project.json` der Wurzel, gegen `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json` geprüft):

| Eintrag | Inhalt |
|---|---|
| Bereitschaft | `dev` und `serve` melden sich auf Port `6029` (`MIT_BESTAND_DEMONSTRATOR_PORT`) bereit |
| globale Achsen | `build-tool` (Nx oder Bazel), `cache` (nutzen, lokal überspringen, alles überspringen) und `build-mode`, angeboten bei jedem Verb, das Build-Tasks ausführt (siehe [Befehlsregister](../../🧰️framework/🛍️products/🦑️repo/README.md#-deutsch)) |

Nicht verifiziert: Für diese Dokumentation ist kein Lauf dieser Kette unter einem der beiden Werkzeuge aufgezeichnet. Targetnamen, Abhängigkeiten, Ports, Cache-Flags und Registereinträge sind am 2026-10-20 aus Manifest, Skripten und Playwright-Konfiguration gelesen. Browser in der Revision, die `playwright-core` nennt, waren am 2026-10-19 nicht installiert, `test-e2e` lädt sie daher zuerst herunter. Der Test des Laufzeitkatalogs (`🔨️modules/🧩️runtime/🧪️tests/🟦️.ts`, als `bun test` auf dieser Datei mit `SEMIO_WORKSPACE_ROOT` auf den absoluten Repository-Wurzelpfad) ist am 2026-10-20 durchgelaufen; er ist der einzige Teil dieses Kapitels, der ausgeführt wurde.
