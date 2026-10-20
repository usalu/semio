import sys, os
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from lib import Doc
ROOT = os.environ.get("DOCS_ROOT", "/Users/ueli/Documents/semio")

C = Doc(ROOT + "/🎓️teaching/🏛️architecture/📚️course/README.md")
C.append("Not yet verified: no run of this chain under either tool", " Static checks of 2026-10-20: `playwright test --list`, started with Bun, enumerates 76 tests in 16 files; the Vite configuration resolves under both of Vite's config loaders for `build` and `serve`; the gate, the stack, the deploy and the learner scripts bundle without error; Playwright browsers are downloaded by `bun repo run workspace:deps-browsers` into `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`.")
C.append("Nicht verifiziert: Für diese Dokumentation ist kein Lauf dieser Kette", " Statische Prüfungen vom 2026-10-20: `playwright test --list`, mit Bun gestartet, zählt 76 Tests in 16 Dateien; die Vite-Konfiguration löst unter beiden Konfigurationsladern von Vite für `build` und `serve` auf; die Skripte von Gate, Stack, Deploy und Lernenden-Prüfung lassen sich ohne Fehler bündeln; die Playwright-Browser lädt `bun repo run workspace:deps-browsers` nach `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`.")
C.save()

for path, name, tests in [
    ("/🏢️semio-tech/🎡️play/README.md", "Play", "157 tests in one file"),
    ("/♻️mit-bestand/🧺️demonstrator/README.md", "Demonstrator", "12 tests in one file"),
]:
    P = Doc(ROOT + path)
    P.append("Not yet verified: no run of this chain under either tool", " Static checks of 2026-10-20: the Vite configuration could not load under Vite's default `runner` config loader (the inventory import of the build tool opens a blob URL), so `serve`, `serve-test` and the site `build` pass `configLoader: \"native\"`, and it resolves with it; `playwright test --list`, started with Bun (`node` fails with `ERR_UNSUPPORTED_ESM_URL_SCHEME`), enumerates " + tests + "; the Playwright browsers are downloaded by `bun repo run workspace:deps-browsers`.")
    P.append("Nicht verifiziert: Für diese Dokumentation ist kein Lauf dieser Kette", " Statische Prüfungen vom 2026-10-20: Die Vite-Konfiguration ließ sich unter Vites Standard-Konfigurationslader `runner` nicht laden (der Inventar-Import des Build-Werkzeugs öffnet eine Blob-URL), deshalb übergeben `serve`, `serve-test` und der Site-`build` `configLoader: \"native\"`, und sie löst damit auf; `playwright test --list`, mit Bun gestartet (`node` scheitert mit `ERR_UNSUPPORTED_ESM_URL_SCHEME`), zählt " + tests.replace("tests in one file", "Tests in einer Datei") + "; die Playwright-Browser lädt `bun repo run workspace:deps-browsers`.")
    P.save()

T = Doc(ROOT + "/🎓️teaching/🛂️proctor/README.md")
T.append("**Tools, prerequisites and cache.**", " The receipt repair path (run `build` twice, delete the compiler `published/<hash>` slot, run it again, expect one `[store-receipt] re-executing` line, under Nx and under Bazel) is the first proof recipe of the build chapter and has not been run.")
T.append("**Werkzeuge, Voraussetzungen und Cache.**", " Der Reparaturpfad des Belegs (`build` zweimal ausführen, den Slot `published/<hash>` des Compiler-Stores löschen, erneut ausführen, genau eine Zeile `[store-receipt] re-executing` erwarten, unter Nx und unter Bazel) ist das erste Nachweisrezept des Build-Kapitels und wurde nicht ausgeführt.")
T.save()
print("product docs ok")
