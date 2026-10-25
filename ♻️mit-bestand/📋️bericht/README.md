# 📋️ Bericht: Entwerfen mit Bestand

The reports of the research project "Entwerfen mit Bestand" (`@semio-tech/mit-bestand-bericht`): three LaTeX documents typeset with the [print product](../../🧰️framework/🛍️products/📓️print/README.md) and tectonic, plus the actor network (`akteursnetz`) that feeds the interim and the research report. The reports are German documents.

| Path | What |
|---|---|
| `📋️zwischenbericht/`, `📑️forschungsbericht/`, `📝️kompaktbericht/` | the three report sources (`.tex`, `📚️references.bib`, `🖼️asset/`) |
| `📎️anhang/` | the shared appendix; `🕸️akteursnetz-figuren.tex`, `📊️akteursnetz-tabellen.tex`, `🎯️akteursnetz-programme.tex` and `👥️akteursnetz.tex` are generated and never edited by hand |
| `🔨️modules/📄️documents/` | the report catalogue (`🔣️.json`, ids `zwischenbericht`, `forschungsbericht`, `kompaktbericht`) and its schema |
| `🔨️modules/👥️actor-network/` | the actor network data, its validation and the renderer of the generated appendix files |
| `📦️packages/🟦️typescript/` | the manifest `📋️project.json` with the targets below and `📜️script.ts`, the router of `test` and `preview-generated` |
| `🧪️tests/` | the fixtures of the report family and the PDF vectors |

## Commands

Every command is a target of the project `@semio-tech/mit-bestand-bericht` and runs through the tool-neutral build entry (`repo run <project>:<target>`; Bazel with `repo --tool=bazel run …`). In the dashboard (`repo dashboard`, **New Task**, `Ctrl+B n`) search `mit-bestand-bericht` plus the action. Never call `tectonic` or `cargo` yourself; the print toolchain, fonts and generated sources are prerequisites of the targets and are prepared by them.

```bash
repo run @semio-tech/mit-bestand-bericht:build
repo run @semio-tech/mit-bestand-bericht:build-zwischenbericht
repo run @semio-tech/mit-bestand-bericht:build-forschungsbericht
repo run @semio-tech/mit-bestand-bericht:build-kompaktbericht
repo run @semio-tech/mit-bestand-bericht:watch-forschungsbericht
repo run @semio-tech/mit-bestand-bericht:test
repo run @semio-tech/mit-bestand-bericht:test -- long
```

| Target | What it does |
|---|---|
| `build` | all three reports (depends on the three `build-*` targets) |
| `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht` | one report; cached; output below |
| `watch`, `watch-zwischenbericht`, `watch-forschungsbericht`, `watch-kompaktbericht` | build, then rebuild on source changes until stopped; take no arguments |
| `generate-actor-network` | renders the four generated appendix files from the actor network data |
| `akteursnetz-validate` | validates the actor network data and its assets |
| `check-generated` | fails when a generated appendix file is stale |
| `preview-generated` | shows the generated appendix without writing |
| `test` | report family, build contracts, citations and inputs; levels `quick` (default), `long`, `exhaustive`; the PDF content check (independent PDF.js read) runs from `long` |

The PDFs are published below `.cache/b/p/m/print/<build mode>/documents/report/<id>/` of the checkout (`<id>` is `zwischenbericht`, `forschungsbericht` or `kompaktbericht`). Task output prints the artifact path and the compile progress; the task menu offers restart and stop.

After a change of the actor network data run `generate-actor-network`; `check-generated` fails on a stale appendix, and so does `test` from level `long`.

## 🇩🇪️ Deutsch

### Befehle

Jeder Befehl ist ein Target des Projekts `@semio-tech/mit-bestand-bericht` und läuft über den werkzeugneutralen Build-Einstieg (`repo run <Projekt>:<Target>`; Bazel mit `repo --tool=bazel run …`). Im Dashboard (`repo dashboard`, **New Task**, `Ctrl+B n`) `mit-bestand-bericht` plus die Aktion suchen. `tectonic` und `cargo` nie selbst aufrufen; Print-Toolchain, Schriften und erzeugte Quellen sind Voraussetzungen der Targets und werden von ihnen vorbereitet.

```bash
repo run @semio-tech/mit-bestand-bericht:build
repo run @semio-tech/mit-bestand-bericht:build-zwischenbericht
repo run @semio-tech/mit-bestand-bericht:build-forschungsbericht
repo run @semio-tech/mit-bestand-bericht:build-kompaktbericht
repo run @semio-tech/mit-bestand-bericht:watch-forschungsbericht
repo run @semio-tech/mit-bestand-bericht:test
repo run @semio-tech/mit-bestand-bericht:test -- long
```

| Target | Wirkung |
|---|---|
| `build` | alle drei Berichte (hängt von den drei `build-*`-Targets ab) |
| `build-zwischenbericht`, `build-forschungsbericht`, `build-kompaktbericht` | ein Bericht; gecacht; Ausgabe siehe unten |
| `watch`, `watch-zwischenbericht`, `watch-forschungsbericht`, `watch-kompaktbericht` | bauen und bei Quelländerungen neu bauen, bis gestoppt wird; ohne Argumente |
| `generate-actor-network` | erzeugt die vier generierten Anhangsdateien aus den Akteursnetz-Daten |
| `akteursnetz-validate` | prüft die Akteursnetz-Daten und ihre Assets |
| `check-generated` | schlägt fehl, wenn eine generierte Anhangsdatei veraltet ist |
| `preview-generated` | zeigt den generierten Anhang, ohne zu schreiben |
| `test` | Berichtsfamilie, Build-Verträge, Zitate und Eingaben; Stufen `quick` (Standard), `long`, `exhaustive`; die PDF-Inhaltsprüfung (unabhängiges Lesen mit PDF.js) läuft ab `long` |

Die PDFs liegen unter `.cache/b/p/m/print/<Build-Modus>/documents/report/<id>/` des Checkouts (`<id>` ist `zwischenbericht`, `forschungsbericht` oder `kompaktbericht`). Die Task-Ausgabe nennt den Artefaktpfad und den Kompilierfortschritt; das Task-Menü bietet Neustart und Stopp. Nach einer Änderung der Akteursnetz-Daten `generate-actor-network` ausführen; `check-generated` schlägt bei einem veralteten Anhang fehl, `test` ab der Stufe `long` ebenso.
