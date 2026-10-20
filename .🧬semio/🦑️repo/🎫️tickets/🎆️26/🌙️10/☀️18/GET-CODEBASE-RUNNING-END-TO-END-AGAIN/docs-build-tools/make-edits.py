#!/usr/bin/env python3
"""Builds staged/edits.json: the anchored edits of the product README (EN guide, DE guide, and the statements they make stale).

Reads the CURRENT README and the two guide fragments, derives every `old` string from the README itself (so no emoji is retyped),
asserts each occurs exactly once and writes the edit list with the README's sha256 as `base`. Read only on the repository.
Usage: make-edits.py [--root DIR]"""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
README = os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/README.md")
STAGED = os.path.join(HERE, "staged")
raw = open(README, "rb").read()
text = raw.decode("utf8")
lines = text.split("\n")
guide_en = open(os.path.join(STAGED, "guide-en.md"), encoding="utf8").read().rstrip("\n")
guide_de = open(os.path.join(STAGED, "guide-de.md"), encoding="utf8").read().rstrip("\n")
edits = []

EN_END = next(i for i, l in enumerate(lines) if l.startswith("## ") and "Deutsch" in l)
EN, DE = (0, EN_END), (EN_END, len(lines))


def line(prefix, region):
    rows = [l for i, l in enumerate(lines) if l.startswith(prefix) and region[0] <= i < region[1]]
    assert len(rows) == 1, (prefix, len(rows))
    return rows[0]


def add(identifier, old, new):
    assert text.count(old) == 1, (identifier, text.count(old))
    assert old != new, identifier
    edits.append({"id": identifier, "old": old, "new": new})


def swap(identifier, region, prefix, pairs, extra=""):
    row = line(prefix, region)
    new = row
    for old, replacement in pairs:
        assert new.count(old) == 1, (identifier, old, new.count(old))
        new = new.replace(old, replacement, 1)
    add(identifier, row, new + extra)


neutral_en = next(l for l in lines[:EN_END] if l.startswith("## ") and l.endswith("Neutral model"))
anchor_en = neutral_en + "\n\n| Layer | What it is | Source |"
add("guide-en", anchor_en, guide_en + "\n\n" + anchor_en)
edits[-1]["probe"] = guide_en.split("\n")[0]
anchor_de = "### Neutrales Modell\n\n| Schicht | Was sie ist | Quelle |"
add("guide-de", anchor_de, guide_de + "\n\n" + anchor_de)
edits[-1]["probe"] = guide_de.split("\n")[0]

intro_en = "A German version of this chapter follows its last section."
add("intro-en", intro_en, "Start with the Build tools guide; the sections after it are the reference. " + intro_en)
intro_de = "Pfade, die mit `📚️library/` beginnen, liegen unter `🔨️modules/` dieses Produkts."
add("intro-de", intro_de, intro_de + " Beginne mit „Build-Werkzeuge im Alltag“; die Abschnitte danach sind die Referenz.")

for lang, region, comments in (
    ("en", EN, ("the remote cache contract, its rendering for the tool, whether the endpoint answers", "serve a bounded remote cache for Bazel and Nx until stopped")),
    ("de", DE, ("der Remote-Cache-Vertrag, seine Umsetzung für das Werkzeug, ob der Endpunkt antwortet", "einen begrenzten Remote-Cache für Bazel und Nx bereitstellen, bis er gestoppt wird")),
):
    row = line("repo cache status|clear", region)
    column = row.index("#")
    add("commands-" + lang, row, row + "\n" + "repo cache remote status".ljust(column) + "# " + comments[0] + "\n" + "repo cache serve [--port <n>]".ljust(column) + "# " + comments[1])

swap("operation-en", EN, "| `cache status\\|clear` | adapter | adapter | no |", [("| `cache status\\|clear` |", "| `cache status\\|clear\\|remote status\\|serve` |"), (" sections. |", " sections. `remote status` and `serve` belong to the remote cache (see Remote cache). |")])
swap("operation-de", DE, "| `cache status\\|clear` | adapter | adapter | nein |", [("| `cache status\\|clear` |", "| `cache status\\|clear\\|remote status\\|serve` |"), ("Implementierungsabschnitte. |", "Implementierungsabschnitte. `remote status` und `serve` gehören zum Remote-Cache (siehe Remote-Cache). |")])

for lang, region, prefix, extra in (
    ("en", EN, "| `--tool=nx\\|bazel` | Select the implementation", "| `--workers=on\\|off` | Run actions in persistent workers (`on`) or one process each (`off`); otherwise `SEMIO_BUILD_WORKERS`, then the worker policy. | all |"),
    ("de", DE, "| `--tool=nx\\|bazel` | Implementierung wählen", "| `--workers=on\\|off` | Führt Actions in persistenten Workern (`on`) oder je in einem eigenen Prozess (`off`) aus; sonst `SEMIO_BUILD_WORKERS`, dann die Worker-Richtlinie. | alle |"),
):
    row = line(prefix, region)
    add("flag-workers-" + lang, row, row + "\n" + extra)
for lang, region, extra in (
    ("en", EN, "| `--port`, `--host`, `--directory`, `--max-bytes` | Port, address, store directory and size bound of the remote cache server (`--name value` or `--name=value`). | `cache serve` |"),
    ("de", DE, "| `--port`, `--host`, `--directory`, `--max-bytes` | Port, Adresse, Speicherverzeichnis und Größengrenze des Remote-Cache-Servers (`--name value` oder `--name=value`). | `cache serve` |"),
):
    row = line("| `--store=shared\\|isolated`, `--parallelism=<cores>` |", region)
    add("flag-serve-" + lang, row, row + "\n" + extra)

unclassified = {
    "en": "\n| `unclassified` | `no-cache-classification` | No row, declaration or verifying command classifies the target, so it never replays until the policy names its kind (fail-closed). | no |",
    "de": "\n| `unclassified` | `no-cache-classification` | Keine Zeile, Deklaration und kein prüfender Befehl ordnet das Target ein, es wird also nie wiedergegeben, bis die Richtlinie seine Art benennt (fail-closed). | nein |",
}
for lang, region, rows in (
    ("en", EN, (
        ("side-effecting", "| `side-effecting` |", [("`mutates-workspace` |", "`mutates-workspace`, `mutates-host-store` |")]),
        ("live", "| `live` |", [("`drives-running-process` | Observes or drives a running process (names with `e2e` or `live`). |", "`drives-running-process`, `exercises-host-resources` | Observes or drives a running process (names with `e2e` or `live`), or exercises the real locks, storage, process tree or clock of this machine. |")]),
        ("aggregate", "| `aggregate` |", [("`orders-prerequisites` |", "`orders-prerequisites`, `reenters-build-tool` |")]),
        ("diagnostic", "| `diagnostic` |", [("`measures-environment` |", "`measures-environment`, `reads-caller-input` |")]),
    )),
    ("de", DE, (
        ("side-effecting", "| `side-effecting` |", [("`mutates-workspace` |", "`mutates-workspace`, `mutates-host-store` |")]),
        ("live", "| `live` |", [("`drives-running-process` | Beobachtet oder steuert einen laufenden Prozess (Namen mit `e2e` oder `live`). |", "`drives-running-process`, `exercises-host-resources` | Beobachtet oder steuert einen laufenden Prozess (Namen mit `e2e` oder `live`) oder prüft die echten Locks, den Speicher, den Prozessbaum oder die Uhr dieses Rechners. |")]),
        ("aggregate", "| `aggregate` |", [("`orders-prerequisites` |", "`orders-prerequisites`, `reenters-build-tool` |")]),
        ("diagnostic", "| `diagnostic` |", [("`measures-environment` |", "`measures-environment`, `reads-caller-input` |")]),
    )),
):
    for name, prefix, pairs in rows:
        swap("class-" + name + "-" + lang, region, prefix, pairs, unclassified[lang] if name == "diagnostic" else "")

swap("classes-text-en", EN, "The Bazel adapter reads the decision (`bazelTargetCacheClassV1`", [
    ("A manifest does not write it; the synthesized", "A manifest may declare it itself (a target with `cache: false` records its reason this way, see the Build tools guide); the synthesized"),
    ("targets cached by the name rules below carry none.", "targets cached by the name rules below carry the class of their row.")])
swap("classes-text-de", DE, "Der Bazel-Adapter liest die Entscheidung (`bazelTargetCacheClassV1`", [
    ("Ein Manifest schreibt sie nicht; die synthetisierten", "Ein Manifest darf sie selbst deklarieren (ein Target mit `cache: false` hält seinen Grund so fest, siehe Build-Werkzeuge im Alltag); die synthetisierten"),
    ("per Namensregel gecachte Targets tragen keine.", "per Namensregel gecachte Targets tragen die Klasse ihrer Zeile.")])

policy_en = next(text[m.start() + 1 : m.end() - 1] for m in re.finditer(r"`[^`\s]*⚡️caching/🔣️policy\.json`", text))

row6_en = line("| 6 | A deterministic family:", EN)
row7_en = line("| 7 | Anything else |", EN)
add("order-en", row6_en + "\n" + row7_en, (
    "| 6 | The decision the target declares in `metadata.semio.cache`, else the first matching row of `targetClasses` in `" + policy_en + "` (ordered rows of exact names, heads, tails and tokens, each with a decision; an exact name beats the rest): `deterministic/check-without-outputs` for the heads `test`, `check`, `lint`, `typecheck`, `format-check`, `verify`, `schema`, `stdio`, a trailing `-check`, `-test`, `-oracle`, `-lint`, `-source` or `-native`, the token `contract` and a list of exact names; `deterministic/pure-function-of-inputs` for the heads `build` and `generate` and a list of exact producers; `side-effecting/mutates-workspace` for the head `generator` | yes when the class is `deterministic`, otherwise no | the class of the row; for `deterministic` the default inputs `default` and `^default`, outputs `[]` unless the manifest says otherwise |\n"
    "| 7 | A `verify` command in the target | yes | `deterministic/check-without-outputs` |\n"
    "| 8 | Anything else | **no** | `unclassified/no-cache-classification` |"))
row6_de = line("| 6 | Eine deterministische Familie:", DE)
row7_de = line("| 7 | Alles andere |", DE)
add("order-de", row6_de + "\n" + row7_de, (
    "| 6 | Die Entscheidung, die das Target in `metadata.semio.cache` deklariert, sonst die erste passende Zeile von `targetClasses` in `" + policy_en + "` (geordnete Zeilen aus exakten Namen, Anfängen, Endungen und Bestandteilen, jede mit einer Entscheidung; ein exakter Name schlägt den Rest): `deterministic/check-without-outputs` für die Anfänge `test`, `check`, `lint`, `typecheck`, `format-check`, `verify`, `schema`, `stdio`, eine Endung `-check`, `-test`, `-oracle`, `-lint`, `-source` oder `-native`, den Bestandteil `contract` und eine Liste exakter Namen; `deterministic/pure-function-of-inputs` für die Anfänge `build` und `generate` und eine Liste exakter Erzeuger; `side-effecting/mutates-workspace` für den Anfang `generator` | ja, wenn die Klasse `deterministic` ist, sonst nein | die Klasse der Zeile; bei `deterministic` die Standard-Inputs `default` und `^default`, Outputs `[]`, sofern das Manifest nichts anderes sagt |\n"
    "| 7 | Ein `verify`-Befehl im Target | ja | `deterministic/check-without-outputs` |\n"
    "| 8 | Alles andere | **nein** | `unclassified/no-cache-classification` |"))

law = "📚️library/⚖️laws/🔒️cache-correctness"
add("fail-closed-en", "Row 7 is fail-open under Nx (Bazel additionally requires an `outputs` array and a class that allows replay, see its cache decision): a target that touches host state and says nothing is cached, and a cache hit then skips its effect. Write `cache: false` for these reasons",
    "Row 8 is fail-closed under both tools: a target that nothing classifies runs every time, whatever its `cache` field says (`unclassified`); the cache-correctness law (`" + law + "`) reports it together with every `cache: true` that the policy overrode and every `cache: false` without a recorded reason. A `deterministic/pure-function-of-inputs` target that declares no output is demoted to `side-effecting/mutates-workspace` before either tool sees it (`cachedOutputsAdmittedV1`), because a hit would replay an empty success. Bazel additionally requires an `outputs` array and a class that allows replay, see its cache decision. Write `cache: false` plus a decision for these reasons")
add("fail-closed-de", "Rang 7 ist unter Nx fail-open (Bazel verlangt zusätzlich ein `outputs`-Array und eine Klasse, die Wiedergabe erlaubt, siehe seine Cache-Entscheidung): Eine Task, die Host-Zustand berührt und nichts sagt, wird gecacht, und ein Cache-Treffer überspringt ihre Wirkung. Schreibe `cache: false` bei",
    "Rang 8 ist unter beiden Werkzeugen fail-closed: Ein Target, das nichts einordnet, läuft jedes Mal, was auch immer sein Feld `cache` sagt (`unclassified`); das Cache-Korrektheits-Gesetz (`" + law + "`) meldet es zusammen mit jedem `cache: true`, das die Richtlinie überstimmt hat, und jedem `cache: false` ohne festgehaltenen Grund. Ein `deterministic/pure-function-of-inputs`-Target ohne deklarierten Output wird zu `side-effecting/mutates-workspace` herabgestuft, bevor eines der Werkzeuge es sieht (`cachedOutputsAdmittedV1`), weil ein Treffer einen leeren Erfolg wiedergäbe. Bazel verlangt zusätzlich ein `outputs`-Array und eine Klasse, die Wiedergabe erlaubt, siehe seine Cache-Entscheidung. Schreibe `cache: false` samt Entscheidung bei")

add("minimal-en", "A target that is not needed is never analyzed or run.", "A target that is not needed is never analyzed or run. A selected cached `test` task without declared outputs and without selected dependents runs as a Bazel test with cached test results (see Task dispositions in the Build tools guide).")
add("minimal-de", "Ein nicht benötigtes Target wird nie analysiert oder ausgeführt.", "Ein nicht benötigtes Target wird nie analysiert oder ausgeführt. Eine gewählte gecachte `test`-Task ohne deklarierte Outputs und ohne gewählte Abhängige läuft als Bazel-Test mit gecachten Testergebnissen (siehe Task-Dispositionen in Build-Werkzeuge im Alltag).")

for lang, region, prefix in (("en", EN, "  5. A class other than `deterministic`"), ("de", DE, "  5. Eine Klasse außer `deterministic`")):
    swap("decision-" + lang, region, prefix, [("`aggregate`, `diagnostic`.", "`aggregate`, `diagnostic`, `unclassified`.")])

def conditional(identifier, guide, prefix, new, requires="remote-execution"):
    row = next(l for l in guide.split("\n") if l.startswith(prefix))
    assert guide.count(row) == 1, identifier
    edits.append({"id": identifier, "old": row, "new": new(row), "requires": requires})


conditional("rbe-operation-en", guide_en, "| `reset` |", lambda row: row + "\n| `remote-execution serve`, `status`, `env` | run, inspect or describe the optional local remote execution cluster (see Remote execution) | `repo remote-execution status` |")
conditional("rbe-operation-de", guide_de, "| `reset` |", lambda row: row + "\n| `remote-execution serve`, `status`, `env` | startet, prüft oder beschreibt den optionalen lokalen Remote-Execution-Cluster (siehe Remote-Ausführung) | `repo remote-execution status` |")
conditional("rbe-stack-en", guide_en, "- **Optional local stack.**", lambda row: "- **Optional local cluster.** `repo remote-execution serve` runs a local Buildbarn cluster (storage with content store and action cache, frontend, scheduler, worker, runner) in containers until you stop it and then removes the containers and volumes. It needs a running container runtime with the compose plugin; every image is an official one, pinned by digest. The frontend answers on `127.0.0.1:8980` and its health route on `8981` (`--port` and `--diagnostics-port` move them); the cluster has no authentication and listens on loopback only. `--platform=<id>` selects the worker platform (default: the one that matches this machine's CPU, `linux-arm64` or `linux-x64`). `repo remote-execution status` shows the runtime, the containers and whether the frontend answers, and `repo remote-execution env --platform=linux-x64` prints the variables a client sets, one `NAME=value` per line. The runner has no network, so a hermetic action there gets a real network namespace without a network. It is a developer service, never a dependency: with `SEMIO_REMOTE_EXECUTION_ENDPOINT` unset every action stays local.")
conditional("rbe-stack-de", guide_de, "- **Optionaler lokaler Stack.**", lambda row: "- **Optionaler lokaler Cluster.** `repo remote-execution serve` betreibt einen lokalen Buildbarn-Cluster (Speicher mit Inhalts-Store und Action-Cache, Frontend, Scheduler, Worker, Runner) in Containern, bis du ihn stoppst, und entfernt danach Container und Volumes. Er braucht eine laufende Container-Laufzeit mit dem Compose-Plugin; jedes Image ist ein offizielles und per Digest gepinnt. Das Frontend antwortet auf `127.0.0.1:8980`, seine Gesundheitsroute auf `8981` (`--port` und `--diagnostics-port` verschieben sie); der Cluster hat keine Authentifizierung und lauscht nur auf Loopback. `--platform=<id>` wählt die Worker-Plattform (Standard: die zur CPU dieses Rechners passende, `linux-arm64` oder `linux-x64`). `repo remote-execution status` zeigt Laufzeit, Container und ob das Frontend antwortet, und `repo remote-execution env --platform=linux-x64` gibt die Variablen aus, die ein Client setzt, je Zeile `NAME=Wert`. Der Runner hat kein Netzwerk, eine hermetische Action dort erhält also einen echten Netzwerk-Namensraum ohne Netzwerk. Es ist ein Entwicklerdienst, nie eine Abhängigkeit: Ist `SEMIO_REMOTE_EXECUTION_ENDPOINT` ungesetzt, bleibt jede Action lokal.")

edits = [e for e in edits if e is not None]
ids = [e["id"] for e in edits]
assert len(ids) == len(set(ids)), "duplicate edit ids"
joined = [e["old"] for e in edits]
for i, a in enumerate(joined):
    for j, b in enumerate(joined):
        if i != j and (a in b or b in a) and "requires" not in edits[i] and "requires" not in edits[j]:
            raise SystemExit(f"overlapping anchors: {edits[i]['id']} / {edits[j]['id']}")
document = {"version": 1, "readme": "🧰️framework/🛍️products/🦑️repo/README.md", "base": hashlib.sha256(raw).hexdigest(), "edits": edits}
open(os.path.join(STAGED, "edits.json"), "w", encoding="utf8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
print("edits", len(edits), "base", document["base"][:16])
