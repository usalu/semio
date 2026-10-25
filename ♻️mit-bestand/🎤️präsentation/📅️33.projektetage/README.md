# 📅️ Projektetage: 33. Projektetage der Bauforschung

The deck "Entwerfen mit Bestand" for the 33rd Projektetage der Bauforschung (Zukunft Bau), project `@semio-tech/mit-bestand-praesentation-projektetage`. It declares its slides as a glob of slide files and mounts them through the renderer of the [presentation product](../../../🧰️framework/🛍️products/🎤️presentation/README.md). The deck is German.

| Path | What |
|---|---|
| `🎞️slide/` | the slides as `<chapter>/<sequence>/<thought>/<slide>/🟦️.ts`, one folder per chapter (`🌷️Einführung`, `🔎️Recherche`, `🛠️Entwurfswerkzeug`, `🧩️Bauteilportal`); an intro thought is `<thought>/🟦️.ts` |
| `🎬️deck/` | the deck assembly: loads the slide glob, adds the Zukunft Bau bookends and mounts the renderer |
| `🔖️spec/` | the deck specification shared by the deck and the slide files (package surface `./spec`) |
| `🌐️public/` | images, videos, PDFs and the `CNAME` of the published site |
| `🏗️builder/🌐️vite/` | the Vite configuration of the deck (served and built through the targets below) |
| `🧪️tests/` | the deck tests (`🧪️projektetage-deck`, `🎞️react-deck`) and their configuration |
| `📦️packages/🟦️typescript/` | the manifest `📋️project.json` with the targets below, `📜️script.ts`, the entry `🟦️.ts` and the `./spec` facade; both re-export the modules above |

## Commands

Every command is a target of the project `@semio-tech/mit-bestand-praesentation-projektetage` and runs through the tool-neutral build entry (`repo run <project>:<target>`; Bazel with `repo --tool=bazel run …`). In the dashboard (`repo dashboard`, **New Task**, `Ctrl+B n`) search `projektetage` plus the action; **Open ready address** (`Ctrl+B u`) opens the running deck. Never call `vite`, `bun install` or `vitest` yourself.

```bash
repo run @semio-tech/mit-bestand-praesentation-projektetage:dev --detach --wait-ready
repo run @semio-tech/mit-bestand-praesentation-projektetage:build
repo run @semio-tech/mit-bestand-praesentation-projektetage:test
repo run @semio-tech/mit-bestand-praesentation-projektetage:test -- long
repo run @semio-tech/mit-bestand-praesentation-projektetage:canonical-architecture
```

| Target | What it does | Cache |
|---|---|---|
| `dev` | serves the deck on port `6050` (override with `PRAESENTATION_PROJEKTETAGE_PORT`) until stopped; the port is fixed: a second run reuses a deck already served there and otherwise fails while another process holds `6050` | never |
| `build` | builds the static site into `📦️packages/🟦️typescript/dist/<build mode>/` | yes |
| `test` | the deck tests; levels `fundamental` (default), `quick`, `long`, `exhaustive` | yes |
| `canonical-architecture` | the `🎞️react-deck` test at level `long` (the interaction laws of the deck: focus-to-labels morph, ghosts and wrappers, label frames and advance) | yes |

The devcontainer forwards port `6050` ("Projektetage deck"). The renderer and the model are tested in the presentation product: `repo run @semio-tech/presentation:test -- quick` and `repo run @semio-tech/presentation-react:test -- quick`.

## 🇩🇪️ Deutsch

### Befehle

Jeder Befehl ist ein Target des Projekts `@semio-tech/mit-bestand-praesentation-projektetage` und läuft über den werkzeugneutralen Build-Einstieg (`repo run <Projekt>:<Target>`; Bazel mit `repo --tool=bazel run …`). Im Dashboard (`repo dashboard`, **New Task**, `Ctrl+B n`) `projektetage` plus die Aktion suchen; **Open ready address** (`Ctrl+B u`) öffnet die laufende Präsentation. `vite`, `bun install` und `vitest` nie selbst aufrufen.

```bash
repo run @semio-tech/mit-bestand-praesentation-projektetage:dev --detach --wait-ready
repo run @semio-tech/mit-bestand-praesentation-projektetage:build
repo run @semio-tech/mit-bestand-praesentation-projektetage:test
repo run @semio-tech/mit-bestand-praesentation-projektetage:test -- long
repo run @semio-tech/mit-bestand-praesentation-projektetage:canonical-architecture
```

| Target | Wirkung | Cache |
|---|---|---|
| `dev` | stellt die Präsentation auf Port `6050` bereit (änderbar mit `PRAESENTATION_PROJEKTETAGE_PORT`), bis sie gestoppt wird; der Port ist fest: ein zweiter Lauf nutzt eine dort bereits laufende Präsentation weiter und scheitert, solange ein anderer Prozess `6050` belegt | nie |
| `build` | baut die statische Seite nach `📦️packages/🟦️typescript/dist/<Build-Modus>/` | ja |
| `test` | die Tests der Präsentation; Stufen `fundamental` (Standard), `quick`, `long`, `exhaustive` | ja |
| `canonical-architecture` | der Test `🎞️react-deck` auf Stufe `long` (die Interaktionsgesetze der Präsentation: Fokus-zu-Beschriftung-Morph, Geister und Wrapper, Beschriftungsrahmen und Weiterschalten) | ja |

Der Devcontainer leitet Port `6050` ("Projektetage deck") weiter. Renderer und Modell werden im Präsentationsprodukt getestet: `repo run @semio-tech/presentation:test -- quick` und `repo run @semio-tech/presentation-react:test -- quick`.
