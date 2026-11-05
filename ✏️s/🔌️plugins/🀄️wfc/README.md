---
name: wfc
kind: user
---

# wfc

Five artifact editors share the WFC solver and plugin composition. Run commands from the repository root. The repository entry provisions the declared tools and dependencies and prepares the selected browser runtime.

Fünf Artefakteditoren teilen den WFC-Löser und die Plugin-Komposition. Befehle im Repository-Stamm ausführen. Der Repository-Einstieg stellt die deklarierten Werkzeuge und Abhängigkeiten bereit und bereitet die gewählte Browser-Laufzeit vor.

| Artifact / Artefakt | Variant / Variante | Browser |
| --- | --- | --- |
| Sample bitmap / Bitmap-Muster | `bitmap` | [localhost:6041](http://localhost:6041) |
| Tiled 2D grid / 2D-Kachelraster | `grid2d` | [localhost:6042](http://localhost:6042) |
| 2D slot graph / 2D-Slotgraph | `wfc2d` | [localhost:6043](http://localhost:6043) |
| Tiled 3D grid / 3D-Kachelraster | `grid3d` | [localhost:6044](http://localhost:6044) |
| 3D slot graph / 3D-Slotgraph | `wfc3d` | [localhost:6045](http://localhost:6045) |

```sh
repo run @semio-tech/framework-os-dev:dev-bitmap-react-prototype --parallel=1 --lang=en
repo run @semio-tech/framework-os-dev:dev-grid2d-react-prototype --parallel=1 --lang=en
repo run @semio-tech/framework-os-dev:dev-wfc2d-react-prototype --parallel=1 --lang=en
repo run @semio-tech/framework-os-dev:dev-grid3d-react-prototype --parallel=1 --lang=en
repo run @semio-tech/framework-os-dev:dev-wfc3d-react-prototype --parallel=1 --lang=en
```

Choose a variant and open its URL once the server reports readiness. Use `--lang=de` for German command output. Stop the selected server with Ctrl+C. The same commands apply in the devcontainer, Windows, macOS and Linux.

Eine Variante wählen und ihre URL öffnen, sobald der Server bereit ist. `--lang=de` wählt deutsche Befehlsausgaben. Strg+C beendet den gewählten Server. Dieselben Befehle gelten im Devcontainer sowie unter Windows, macOS und Linux.

Load a bundled example, author tiles and adjacency rules or paint the bitmap sample, then start **Solve**. The fill tool provides progress, pause, resume, stepping and cancellation. Solves produce a preview; the authored model, seed and pins are document content. The bitmap editor's **Pin Solution** action keeps its result as output pins. Clearing the example selection restores the editor's initial example.

Ein mitgeliefertes Beispiel laden, Kacheln und Nachbarschaftsregeln bearbeiten oder das Bitmap-Muster malen und **Lösen** starten. Das Füllwerkzeug bietet Fortschritt, Pause, Fortsetzen, Einzelschritte und Abbruch. Lösungen erscheinen als Vorschau; Modell, Startwert und Anheftungen gehören zum Dokument. Die Aktion **Lösung anheften** des Bitmap-Editors behält das Ergebnis als Ausgabe-Anheftungen. Eine leere Beispielauswahl stellt das Startbeispiel des Editors wieder her.
