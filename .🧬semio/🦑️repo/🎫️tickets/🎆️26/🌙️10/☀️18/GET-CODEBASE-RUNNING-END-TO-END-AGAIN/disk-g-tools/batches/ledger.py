"""Batch `ledger`: one source of truth for the disk budget, a closed native root, the producers made honest.
1. Machine budget: the default total (was 40 GiB, below the 97.6 GiB of declared images and the 110 GiB held) is the sum of the declared image capacities and loose roots (121.8 GiB); the store weights are their shares of it (largest remainder); documentation EN+DE states the same numbers (graph 4 -> 6 GiB, repository root 20 -> 24 GiB, watermarks 80/60 -> 87.5/75 percent, control 1 -> 8 MiB). Laws: model-law.
2. The native root `.cache` gets the twin of the repository law (native-law) and the two real native producers are declared (`semio/typescript` of `verify typescript`, `test-sandbox`); the lane-named dirs (e1, e2, edfix, gg, impl-exp...) are no tooling and fall to the 14-day (native 7-day) rule; `framework-actor` writes into the declared command store instead of a private dir; the plugin-registry session test writes into the declared test evidence area instead of `🧪️plugin-registry`.
3. The stray cache roots below module directories (`🧰️framework/.cache` 8.1 GiB, `📥️inputs/💾️storage/.cache` 6.0 GiB, `📦️packages/🟦️typescript/.cache` 2.0 GiB, four small ones; four of their images are attached right now): a law lists them (git, ignored directories named `.cache` other than the root's) and is red on this machine until they are deleted (user decision).
4. The control directory holds the 4 MiB `metadata-headroom.bin` of the lease catalog, so its budget is 8 MiB (was 1 MiB: always over).
Apply AFTER the batches that edit README.md / 📓️build-guide.md as a whole (nxfix core, testmem2, walk-g): these are anchored edits."""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FILES = os.path.join(HERE, "..", "staged", "ledger", "files")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
S = f"{L}/⚡️caching/💾️storage"
LED = f"{S}/🧾️ledger"
BUD = f"{S}/🔒️enforcement/📦️budget"
R = "🧰️framework/🛍️products/🦑️repo"

NEW_AREAS = '''    {
      "id": "typescript-project",
      "root": "native",
      "kind": "tree",
      "directory": "semio",
      "budget": {
        "bytes": 67108864
      },
      "collect": {
        "unusedAgeMs": 2592000000,
        "quietMs": 3600000,
        "custody": "quiet"
      },
      "writers": [
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/✅️verify/🔷️typescript-project/🟦️.ts"
      ],
      "purpose": {
        "en": "Generated whole-workspace TypeScript project and logs of verify typescript (semio/typescript), derived from the checkout.",
        "de": "Erzeugtes Gesamt-TypeScript-Projekt und Protokolle von verify typescript (semio/typescript), aus dem Checkout abgeleitet."
      }
    },
    {
      "id": "test-sandbox",
      "root": "native",
      "kind": "tree",
      "directory": "test-sandbox",
      "budget": {
        "bytes": 33554432
      },
      "collect": {
        "unusedAgeMs": 604800000,
        "quietMs": 3600000,
        "custody": "quiet"
      },
      "writers": [
        "🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🏃️run/🟦️.ts"
      ],
      "purpose": {
        "en": "Checkout scratch of sandboxed test runs (the sandbox protocol's checkoutScratchDirectory) and its reclaim stamp.",
        "de": "Checkout-Arbeitsfläche sandboxed Testläufe (checkoutScratchDirectory des Sandbox-Protokolls) und ihr Reclaim-Stempel."
      }
    },
'''

# machine weights: largest-remainder shares of the declared capacities plus the loose roots
WEIGHTS = [("4400", "3940"), ("2300", "1970"), ("800", "657"), ("700", "657"), ("500", "493"), ("300", "164"), ("150", "123"), ("10", "5"), ("840", "1991")]

EN_GUIDE_OLD = "default 40 GiB (below the sum of the store floors it is refused); a host free-space floor of 15 GiB shrinks the effective budget. Quotas are weighted max-min fair per store (rust 44 %, products 23 %, tools 8 %, dependencies 7 %, graph 5 %, diagnostics 3 %, nx 1.5 %, leases 0.1 %, loose 8.4 %). This machine holds 69 GiB of content, so the default does not fit: set the budget per machine."
EN_GUIDE_NEW = "default 121.8 GiB (below the sum of the store floors it is refused); a host free-space floor of 15 GiB shrinks the effective budget. The default is derived, not chosen: the declared image capacities (97.6 GiB: rust 48, products 24, tools 8, dependencies 8, graph 6, diagnostics 2, nx 1.5 GiB, leases 64 MiB) plus the two loose roots (repository 24 GiB, native 256 MiB), the most a workspace with 6 build and 4 service slots and 16 Cargo lanes may hold; a law pins the sum. Quotas are weighted max-min fair per store, each weight being the share of the store in that total (rust 39.4 %, products 19.7 %, tools 6.6 %, dependencies 6.6 %, graph 4.9 %, diagnostics 1.6 %, nx 1.2 %, leases 0.05 %, loose 19.9 %): at the default every store may fill exactly its declared capacity, a lower budget shrinks all of them in proportion. A machine that cannot spare that much (a 16 GB laptop with a small disk) sets the budget per machine, for example `SEMIO_CACHE_BUDGET_BYTES=64GiB`."
DE_GUIDE_OLD = "Standard 40 GiB (unterhalb der Summe der Speicher-Untergrenzen wird es abgelehnt); eine Untergrenze von 15 GiB freiem Hostplatz verkleinert das wirksame Budget. Die Quoten sind gewichtet max-min-fair je Speicher (rust 44 %, products 23 %, tools 8 %, dependencies 7 %, graph 5 %, diagnostics 3 %, nx 1,5 %, leases 0,1 %, loose 8,4 %). Diese Maschine hält 69 GiB Inhalt, der Standard passt also nicht: Budget je Maschine setzen."
DE_GUIDE_NEW = "Standard 121,8 GiB (unterhalb der Summe der Speicher-Untergrenzen wird es abgelehnt); eine Untergrenze von 15 GiB freiem Hostplatz verkleinert das wirksame Budget. Der Standard ist abgeleitet, nicht gewählt: die deklarierten Image-Größen (97,6 GiB: rust 48, products 24, tools 8, dependencies 8, graph 6, diagnostics 2, nx 1,5 GiB, leases 64 MiB) plus die beiden losen Wurzeln (repository 24 GiB, native 256 MiB), mehr darf ein Arbeitsbereich mit 6 Build- und 4 Dienst-Slots und 16 Cargo-Lanes höchstens halten; ein Gesetz prüft die Summe. Die Quoten sind gewichtet max-min-fair je Speicher, jedes Gewicht ist der Anteil des Speichers an dieser Summe (rust 39,4 %, products 19,7 %, tools 6,6 %, dependencies 6,6 %, graph 4,9 %, diagnostics 1,6 %, nx 1,2 %, leases 0,05 %, loose 19,9 %): beim Standard darf jeder Speicher genau seine deklarierte Größe füllen, ein kleineres Budget verkleinert alle im gleichen Verhältnis. Wer so viel Platz nicht hat (ein 16-GB-Laptop mit kleiner Platte), setzt das Budget je Maschine, zum Beispiel `SEMIO_CACHE_BUDGET_BYTES=64GiB`."

EN_ENFORCE = "- Enforcement: a declared area is enforced, not only declared. Every heavy command reads the size ledger (no tree is walked) and starts, detached at low priority and single-flight, `repo cache gc --area=…` when it is due: every collectable area, the retired roots and the undeclared entries older than 14 days every 12 hours, only the areas recorded over their budget every 30 minutes (stamp, log and status file `budget-sweep.*` in the `build` area, intervals in the ledger schema). Areas an owner manages keep their collector: the Cargo unit cache trims its parts to shares of the image (objects 25 %, lanes 20 %, admission memos 3 %: at most 23.0 of the 24 GiB it is granted; memos live at most 7 days) at every collection, collects hard above 87.5 % used down to 75 % and, between 75 % and 87.5 %, maintains once when its watcher starts and every 10 minutes; the Nx toolkit store needs a quiet tools image (see `nx-tooling` above). Cache images open only for the checkout root: a command started inside a module directory is refused in English and German instead of creating a second image set there. A law scans the tracked sources for writers below `.cache` as it does for the repository cache root. The tools image holds 8 GiB of content while its logical limit is 64 GiB, because hardlinked toolchains count once per name; the control directory is bounded at 8 MiB because it holds the 4 MiB metadata headroom file. Sizes of nested areas are apparent bytes (a cloned file counts at each place, `cache dedupe` clones on purpose): the physical use of an image is the partition row of `cache status`, so the Bazel workspaces and outputs report more than the products image holds.\n"
DE_ENFORCE = "- Durchsetzung: ein deklarierter Bereich wird durchgesetzt, nicht nur deklariert. Jeder schwere Befehl liest die Größenbilanz (kein Baum wird durchlaufen) und startet losgelöst, mit niedriger Priorität und einzeln `repo cache gc --area=…`, wenn es fällig ist: alle einsammelbaren Bereiche, die stillgelegten Wurzeln und die nicht deklarierten Einträge über 14 Tage alle 12 Stunden, nur die Bereiche mit erfasster Budgetüberschreitung alle 30 Minuten (Stempel, Protokoll und Statusdatei `budget-sweep.*` im Bereich `build`, Intervalle im Ledger-Schema). Bereiche, die ein Eigner verwaltet, behalten ihren Sammler: der Cargo-Unit-Cache beschneidet seine Teile bei jeder Sammlung auf Anteile des Images (Objekte 25 %, Lanes 20 %, Zulassungs-Merkzettel 3 %: höchstens 23,0 der gewährten 24 GiB; Merkzettel leben höchstens 7 Tage), sammelt oberhalb von 87,5 % Belegung hart bis 75 % und pflegt zwischen 75 % und 87,5 % einmal beim Start seines Beobachters und alle 10 Minuten; der Nx-Werkzeugspeicher braucht ein ruhiges Tools-Image (siehe `nx-tooling` oben). Cache-Images werden nur für die Checkout-Wurzel geöffnet: ein Befehl, der in einem Modulverzeichnis startet, wird auf Englisch und Deutsch abgelehnt, statt dort einen zweiten Satz Images anzulegen. Ein Gesetz durchsucht die verfolgten Quellen nach Schreibern unterhalb von `.cache`, wie es das für die Repository-Cache-Wurzel tut. Das Tools-Image hält 8 GiB Inhalt, sein logisches Limit beträgt 64 GiB, weil hartverlinkte Werkzeugketten je Name zählen; das Steuerverzeichnis ist auf 8 MiB begrenzt, weil es die 4-MiB-Metadaten-Reservedatei hält. Größen verschachtelter Bereiche sind scheinbare Bytes (eine geklonte Datei zählt an jedem Ort, `cache dedupe` klont absichtlich): die physische Belegung eines Images ist die Partitionszeile von `cache status`, die Bazel-Arbeitsbereiche und -Ausgaben melden daher mehr, als das Produkt-Image hält.\n"

EDITS = [
    (f"{LED}/🔣️policy.json", [(
        '    {\n      "id": "nx-artifacts",',
        NEW_AREAS + '    {\n      "id": "nx-artifacts",',
    )]),
    (f"{LED}/🟦️.ts", [
        ('/** 🚧️ Refuses a repository cache path whose first segments name no declared home; the one runtime door every TypeScript writer passes. */',
         '/** 🚧️ Refuses a cache path whose first segments name no declared home of `root` (the repository cache root by default, `native` for `.cache`); the one runtime door every TypeScript writer of the repository root passes and the test of the native law. */'),
        ('export function requireDeclaredCacheSegmentsV1(path:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1()):void{',
         'export function requireDeclaredCacheSegmentsV1(path:readonly string[],ledger:CacheLedgerV1=cacheLedgerV1(),root:CacheRootIdV1="repository"):void{'),
        ('declared=ledger.areas.filter(area=>area.root==="repository"&&area.kind!=="nested");',
         'declared=ledger.areas.filter(area=>area.root===root&&area.kind!=="nested");'),
    ]),
    (f"{LED}/🎚️machine/🔣️policy.json", [('"defaultTotalBytes": 42949672960,', '"defaultTotalBytes": 130795175936,')] + [
        (f'"weightBasisPoints": {old},', f'"weightBasisPoints": {new},') for old, new in WEIGHTS]),
    (f"{BUD}/🔣️policy.json", [('"maximumControlBytes": 1048576,', '"maximumControlBytes": 8388608,')]),
    (f"{BUD}/🧬️schema/🔣️.json", [('      "minimum": 4096,\n      "maximum": 1048576\n', '      "minimum": 4096,\n      "maximum": 16777216\n')]),
    (f"{BUD}/🧪️tests/🧫️fixtures/🔣️.json", [('"expectedHeadroomBytes": 468713472', '"expectedHeadroomBytes": 461373440')]),
    (f"{R}/📓️build-guide.md", [
        (EN_GUIDE_OLD, EN_GUIDE_NEW),
        (DE_GUIDE_OLD, DE_GUIDE_NEW),
        ("Laws that need the repo envelope run through their suite route. Ad-hoc scripts stream their input and stay below 500 MB.",
         "Laws that need the repo envelope run through their suite route. A test that opens cache storage runs from the checkout root: started in a module directory it is refused (English and German) instead of creating a second set of images there (that is how 16 GiB of images arose below `🧰️framework`). Ad-hoc scripts stream their input and stay below 500 MB."),
        ("Gesetze, die die Repo-Hülle brauchen, laufen über ihre Suite. Ad-hoc-Skripte lesen ihre Eingabe als Strom und bleiben unter 500 MB.",
         "Gesetze, die die Repo-Hülle brauchen, laufen über ihre Suite. Ein Test, der Cache-Speicher öffnet, läuft in der Checkout-Wurzel: in einem Modulverzeichnis gestartet wird er abgelehnt (Englisch und Deutsch), statt dort einen zweiten Satz Images anzulegen (so entstanden 16 GiB Images unter `🧰️framework`). Ad-hoc-Skripte lesen ihre Eingabe als Strom und bleiben unter 500 MB."),
        ("- New macOS images are created sparse (64 MiB, then resized", EN_ENFORCE + "- New macOS images are created sparse (64 MiB, then resized"),
        ("- Neue macOS-Abbilder entstehen spärlich (64 MiB, dann vergrößert", DE_ENFORCE + "- Neue macOS-Abbilder entstehen spärlich (64 MiB, dann vergrößert"),
    ]),
    (f"{R}/README.md", [
        ("above 80 percent used a niced single-flight background collector frees down to 60 percent (incremental sets, then lanes, then objects, oldest first);",
         "above 87.5 percent used a niced single-flight background collector frees down to 75 percent (incremental sets, then lanes, then objects, oldest first); between 75 and 87.5 percent its watcher still trims the lane, object and admission-memo shares once at its start and every 10 minutes, and the memos live at most 7 days;"),
        ("| Project graph (`.cache/b/g`) | every command | partition `g`, 4 GiB |", "| Project graph (`.cache/b/g`) | every command | partition `g`, 6 GiB |"),
        ("(`repository` capped at 20 GiB, `native` at 256 MiB)", "(`repository` capped at 24 GiB, `native` at 256 MiB)"),
        ("| `g` | neutral build graph and inference records | 4 GiB |", "| `g` | neutral build graph and inference records | 6 GiB |"),
        ("(no image, at most 1 MiB; it holds the lease database of the images, `budget.controlDirectory`)", "(no image, at most 8 MiB; it holds the lease database of the images and the 4 MiB metadata headroom file, `budget.controlDirectory`)"),
        ("oberhalb von 80 Prozent Belegung räumt ein niedrig priorisierter Einzelläufer-Collector im Hintergrund bis auf 60 Prozent (zuerst Inkrementell-Sätze, dann Lanes, dann Objekte, älteste zuerst);",
         "oberhalb von 87,5 Prozent Belegung räumt ein niedrig priorisierter Einzelläufer-Collector im Hintergrund bis auf 75 Prozent (zuerst Inkrementell-Sätze, dann Lanes, dann Objekte, älteste zuerst); zwischen 75 und 87,5 Prozent beschneidet sein Beobachter die Anteile von Lanes, Objekten und Admission-Merkzetteln einmal beim Start und alle 10 Minuten, die Merkzettel leben höchstens 7 Tage;"),
        ("| Projektgraph (`.cache/b/g`) | jedem Befehl | Partition `g`, 4 GiB |", "| Projektgraph (`.cache/b/g`) | jedem Befehl | Partition `g`, 6 GiB |"),
        ("(`repository` gedeckelt bei 20 GiB, `native` bei 256 MiB)", "(`repository` gedeckelt bei 24 GiB, `native` bei 256 MiB)"),
        ("`g` Graph (4 GiB)", "`g` Graph (6 GiB)"),
        ("(kein Image, höchstens 1 MiB; es hält die Lease-Datenbank der Images, `budget.controlDirectory`)", "(kein Image, höchstens 8 MiB; es hält die Lease-Datenbank der Images und die 4-MiB-Metadaten-Reservedatei, `budget.controlDirectory`)"),
    ]),
    ("🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/📜️script.ts", [(
        'resolve(this.repoRoot, ".cache", "framework-actor", "tests")',
        'resolve(this.repoRoot, ".cache", "command", "framework-actor", "tests")',
    )]),
    ("✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎮️session/🟦️.ts", [(
        '.🧬semio/🦑️repo/⚡️cache/🧪️plugin-registry/playground-session-output',
        '.🧬semio/🦑️repo/⚡️cache/🧪️tests/plugin-registry-session-output',
    )]),
]

NEW = [
    (f"{LED}/🧪️tests/⚖️law/🌐️native/🟦️.ts", "native-law.ts"),
    (f"{LED}/🧪️tests/📐️model/🟦️.ts", "model-law.ts"),
]
