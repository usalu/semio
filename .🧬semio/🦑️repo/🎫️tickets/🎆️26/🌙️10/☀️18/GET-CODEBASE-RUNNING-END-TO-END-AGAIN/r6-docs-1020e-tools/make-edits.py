#!/usr/bin/env python3
"""Builds staged/edits.json: the anchored edits of the product README for the concurrency / incremental / shared-cache guide (EN first, DE second).

Reads the CURRENT README and the fragments staged/en/*.md and staged/de/*.md, resolves the macros of the fragments against the live tree (lib.py),
derives every `old` string from the README itself (so no emoji is retyped), asserts that each occurs exactly once and writes the edit list with the README
sha256 as `base` and the feature states as `probes`. Edits with a `when` are generated only while that feature is live; the others are listed as deferred.
Read only on the repository. Usage: make-edits.py [--root DIR]"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import lib

raw = open(lib.README, "rb").read()
if os.path.isfile(lib.RECORD):
    record = json.load(open(lib.RECORD, encoding="utf8"))
    if True:
        raw = open(os.path.join(lib.BACKUP, "README." + record["before"][:16] + ".md"), "rb").read()
        assert hashlib.sha256(raw).hexdigest() == record["before"], "backup does not match its recorded hash"
        print("anchors are derived from the backup of the README before our apply " + record["before"][:16])
text = raw.decode("utf8")
lines = text.split("\n")
EN_END = next(i for i, l in enumerate(lines) if l.startswith("## ") and "Deutsch" in l)
REGION = {"en": (0, EN_END), "de": (EN_END, len(lines))}
states, suspicious = lib.probe_states()
if suspicious:
    sys.exit("probe paths with a missing parent directory (mistyped?): " + ", ".join(suspicious))
edits, deferred = [], []


def line(prefix, language):
    start, end = REGION[language]
    rows = [l for i, l in enumerate(lines) if start <= i < end and l.startswith(prefix)]
    assert len(rows) == 1, (language, prefix, len(rows))
    return rows[0]


def add(identifier, old, new, when=None, probe=None):
    if identifier.startswith("verified"):
        new = new.replace("`repo cache serve`", "`repo remote-cache serve`")
    assert text.count(old) == 1, (identifier, text.count(old), old[:80])
    assert old != new, identifier
    if when and not states[when]:
        deferred.append({"id": identifier, "when": when})
        return
    edit = {"id": identifier, "old": old, "new": new}
    if probe:
        edit["probe"] = probe
    edits.append(edit)


def swap(identifier, language, prefix, pairs, extra="", when=None):
    row = line(prefix, language)
    new = row
    for old, replacement in pairs:
        assert new.count(old) == 1, (identifier, old[:60], new.count(old))
        new = new.replace(old, replacement, 1)
    add(identifier, row, new + extra, when)


def padded(command, comment):
    return command + " " * max(2, 53 - len(command)) + "# " + comment


def block(language):
    return "\n\n".join(lib.render(content, language, states) for _, content in lib.sections(language))


NEW_COMMANDS = {
    "en": [
        ("repo dependencies ensure", "install only when missing or changed (commands that need dependencies run this first)"),
        ("repo capacity status", "free memory above the floor, running and waiting grants of the host"),
        ("repo capacity run [--memory=<MB>] -- <command>", "run a heavy command under a machine-wide capacity grant"),
        ("repo incremental list|plan|prepare|run|discard", "prove in a disposable clone that Nx and Bazel rebuild exactly what an edit affects"),
        ("repo verify build-parity [--only=<ids|area:x>]", "prove that Nx and Bazel produce the same outputs"),
    ],
    "de": [
        ("repo dependencies ensure", "nur installieren, wenn etwas fehlt oder geändert ist (Befehle, die Abhängigkeiten brauchen, führen dies zuerst aus)"),
        ("repo capacity status", "freier Speicher über der Untergrenze, laufende und wartende Zusagen des Rechners"),
        ("repo capacity run [--memory=<MB>] -- <command>", "einen schweren Befehl unter einer rechnerweiten Kapazitätszusage ausführen"),
        ("repo incremental list|plan|prepare|run|discard", "in einer Wegwerf-Kopie beweisen, dass Nx und Bazel genau das Betroffene neu bauen"),
        ("repo verify build-parity [--only=<ids|area:x>]", "beweisen, dass Nx und Bazel dieselben Ausgaben erzeugen"),
    ],
}
GATED_COMMANDS = {
    "en": [
        ("leases", ("repo leases status [--json]", "holders and waiters of leases, lock-order violations, deadlock victims")),
        ("concurrency", ("repo concurrency list|plan|run", "prove that simulated agents never wait for unrelated projects")),
    ],
    "de": [
        ("leases", ("repo leases status [--json]", "Halter und Wartende von Leases, Verstöße gegen die Sperrreihenfolge, Deadlock-Opfer")),
        ("concurrency", ("repo concurrency list|plan|run", "beweisen, dass simulierte Agenten nie auf unabhängige Projekte warten")),
    ],
}

# 1. the new guide sections, before the cross-platform notes
for language, heading_end, identifier in (("en", "Cross-platform notes", "sections-en"), ("de", None, "sections-de")):
    start, end = REGION[language]
    if language == "en":
        anchor = next(l for i, l in enumerate(lines) if start <= i < end and l.startswith("### ") and l.endswith(heading_end))
    else:
        anchor = line("#### Hinweise je Plattform", "de")
    content = block(language)
    first = content.split("\n")[0]
    add(identifier, anchor, content + "\n\n" + anchor, probe=first)

# 2. the entry sentence of the guide
swap("guide-entry-en", "en", "This guide is the working view", [("A German version of this guide follows in the German chapter (Build-Werkzeuge im Alltag).", "If several agents or developers share one checkout, start with Many agents in one folder. A German version of this guide follows in the German chapter (Build-Werkzeuge im Alltag).")])
swap("guide-entry-de", "de", "Dieser Leitfaden ist die Arbeitssicht", [("Die englische Fassung steht im Kapitel „Build tools guide“.", "Teilen sich mehrere Agenten oder Entwickler einen Checkout, beginne mit „Viele Agenten in einem Ordner“. Die englische Fassung steht im Kapitel „Build tools guide“.")])

# 3. the command block of the chapter (staged commands appear only once their feature is live)
for language in ("en", "de"):
    anchor = line("repo dependencies lock --all", language)
    rows = [padded(c, d) for c, d in NEW_COMMANDS[language]]
    for feature, (command, comment) in GATED_COMMANDS[language]:
        if states[feature]:
            rows.append(padded(command, comment))
        else:
            deferred.append({"id": f"commands-{feature}-{language}", "when": feature})
    add("commands-" + language, anchor, anchor + "\n" + "\n".join(rows))
    old_cache = line("repo cache status|clear", language)
    comment = {"en": "the cache ledger (--tasks: the task cache of the selected tool), its collection, and clearing the task cache", "de": "das Cache-Ledger (--tasks: der Task-Cache des gewählten Werkzeugs), seine Bereinigung und das Leeren des Task-Caches"}[language]
    add("commands-cache-" + language, old_cache, padded("repo cache status [--tasks]|gc [--dry-run]|clear", comment), when="cache-ledger")

# 4. the operations table
OPERATION_ROWS = {
    "en": [
        "| `dependencies ensure` | install the locked dependencies only when they are missing or changed; every command that needs them runs it first | `repo dependencies ensure` |",
        "| `capacity status`, `capacity run` | show the host capacity or run any command under a machine-wide grant (see Host capacity) | `repo capacity status` |",
        "| `incremental` | prove in a disposable clone that Nx and Bazel rebuild exactly what an edit affects (see Incremental development) | `repo incremental run --scenario=fw-ts-comment` |",
        "| `inputs` | print what Nx hashes for one task; Bazel records its spawn inputs in the parity evidence instead | `repo inputs @teaching/architecture-course:test` |",
        "| `verify build-parity` | root command: prove that Nx and Bazel produce the same outputs (see Parity verification) | `repo verify build-parity --plans-only` |",
    ],
    "de": [
        "| `dependencies ensure` | installiert die gesperrten Abhängigkeiten nur, wenn sie fehlen oder geändert sind; jeder Befehl, der sie braucht, führt es zuerst aus | `repo dependencies ensure` |",
        "| `capacity status`, `capacity run` | zeigt die Kapazität des Rechners oder führt einen beliebigen Befehl unter einer rechnerweiten Zusage aus (siehe Rechnerkapazität) | `repo capacity status` |",
        "| `incremental` | beweist in einer Wegwerf-Kopie, dass Nx und Bazel genau das Betroffene neu bauen (siehe Inkrementelle Entwicklung) | `repo incremental run --scenario=fw-ts-comment` |",
        "| `inputs` | gibt aus, was Nx für eine Task hasht; Bazel hält seine Spawn-Inputs stattdessen in den Evidenzen der Paritätsprüfung fest | `repo inputs @teaching/architecture-course:test` |",
        "| `verify build-parity` | Wurzelbefehl: beweist, dass Nx und Bazel dieselben Ausgaben erzeugen (siehe Paritätsprüfung) | `repo verify build-parity --plans-only` |",
    ],
}
GATED_ROWS = {
    "en": [
        ("leases", "| `leases status` | holders, waiters, lock-order violations and deadlock victims of the lease system (see Leases, lock order and queues) | `repo leases status` |"),
        ("concurrency", "| `concurrency` | prove that simulated agents in one clone never wait for unrelated projects (see Incremental development) | `repo concurrency run --workloads=disjoint-projects` |"),
    ],
    "de": [
        ("leases", "| `leases status` | Halter, Wartende, Verstöße gegen die Sperrreihenfolge und Deadlock-Opfer des Lease-Systems (siehe Leases, Sperrreihenfolge und Warteschlangen) | `repo leases status` |"),
        ("concurrency", "| `concurrency` | beweist, dass simulierte Agenten in einer Kopie nie auf unabhängige Projekte warten (siehe Inkrementelle Entwicklung) | `repo concurrency run --workloads=disjoint-projects` |"),
    ],
}
for language in ("en", "de"):
    anchor = line("| `reset` | invalidate the graph" if language == "en" else "| `reset` | verwirft den Graphen", language)
    rows = list(OPERATION_ROWS[language])
    for feature, row in GATED_ROWS[language]:
        if states[feature]:
            rows.append(row)
        else:
            deferred.append({"id": f"operations-{feature}-{language}", "when": feature})
    add("operations-" + language, anchor, anchor + "\n" + "\n".join(rows))
    old_row = line("| `cache status`, `cache clear` | count or empty" if language == "en" else "| `cache status`, `cache clear` | zählen oder leeren", language)
    if language == "en":
        new_rows = ["| `cache status --tasks`, `cache clear` | count or empty the selected tool's local task cache | `repo cache status --tasks --tool=bazel` |",
                    "| `cache status`, `cache gc` | the cache ledger: every home with budget and use, and lock-aware collection (see Shared caches and budgets) | `repo cache gc --dry-run` |"]
    else:
        new_rows = ["| `cache status --tasks`, `cache clear` | zählen oder leeren den lokalen Task-Cache des gewählten Werkzeugs | `repo cache status --tasks --tool=bazel` |",
                    "| `cache status`, `cache gc` | das Cache-Ledger: jede Heimat mit Budget und Verbrauch und sperrbewusste Bereinigung (siehe Geteilte Caches und Budgets) | `repo cache gc --dry-run` |"]
    add("operations-cache-" + language, old_row, "\n".join(new_rows), when="cache-ledger")

# 5. choose the tool: the state of the default
FLIP = {
    "en": (
        " Nx is still the default (`defaultTool` in `📚️library/🏗️build/🔣️policy.json`): Bazel is available behind `bun repo` for every operation, and the flip waits for three product runs under Bazel (the architecture course, semio-tech play and the mit-bestand demonstrator). It is one gated change of that policy value; afterwards Nx stays selectable with `--tool=nx` or `SEMIO_BUILD_TOOL=nx`. Pin the tool explicitly in scripts and CI (the build workflow already passes `--tool`), so the flip changes nothing for them.",
        " Bazel is the default (`defaultTool` in `📚️library/🏗️build/🔣️policy.json`); Nx stays selectable with `--tool=nx` or `SEMIO_BUILD_TOOL=nx`. Pin the tool explicitly in scripts and CI (the build workflow already passes `--tool`).",
    ),
    "de": (
        " Nx ist weiterhin der Standard (`defaultTool` in `📚️library/🏗️build/🔣️policy.json`): Bazel steht hinter `bun repo` für jede Operation bereit, und die Umstellung wartet auf drei Produktläufe unter Bazel (den Architekturkurs, semio-tech play und den mit-bestand-Demonstrator). Sie ist eine einzige geschützte Änderung dieses Richtlinienwerts; danach bleibt Nx mit `--tool=nx` oder `SEMIO_BUILD_TOOL=nx` wählbar. Lege das Werkzeug in Skripten und CI ausdrücklich fest (der Build-Workflow übergibt `--tool` bereits), dann ändert die Umstellung für sie nichts.",
        " Bazel ist der Standard (`defaultTool` in `📚️library/🏗️build/🔣️policy.json`); Nx bleibt mit `--tool=nx` oder `SEMIO_BUILD_TOOL=nx` wählbar. Lege das Werkzeug in Skripten und CI ausdrücklich fest (der Build-Workflow übergibt `--tool` bereits).",
    ),
}
for language, prefix in (("en", "Use Nx for the default flow."), ("de", "Nimm Nx für den Standardablauf.")):
    row = line(prefix, language)
    flipped = states["default-flip"]
    if flipped:
        continue
    add("default-" + language, row, row + FLIP[language][0])

# 6. remote execution: the class table
HERMETIC = {
    "en": ("No target in the repository declares it yet, so nothing runs remotely today.",
           "A class table (`🏗️build/🏃️execution/🔒️hermetic/🔣️.json`, schema `🏗️build/🏃️execution/🧬️schema/🔒️hermetic-classes.json`) can make a whole target family hermetic instead (`ts-test`, `ts-check`, `ts-build`, `ts-typecheck`, `ts-generate`: 944 of the 3,707 cacheable targets are eligible by static checks); it is rolled out to three canary projects, whose targets run their real scripts inside the action with a declared dependency tree and an issued original authority. No repository target has been run hermetically on the live tree yet, and none remotely, so nothing runs remotely today."),
    "de": ("Kein Target des Repositorys deklariert es bisher, also läuft heute nichts remote.",
           "Eine Klassentabelle (`🏗️build/🏃️execution/🔒️hermetic/🔣️.json`, Schema `🏗️build/🏃️execution/🧬️schema/🔒️hermetic-classes.json`) kann stattdessen eine ganze Target-Familie hermetisch machen (`ts-test`, `ts-check`, `ts-build`, `ts-typecheck`, `ts-generate`: 944 der 3.707 cachebaren Targets sind nach statischen Prüfungen geeignet); sie ist auf drei Canary-Projekte ausgerollt, deren Targets ihre echten Skripte in der Action mit einem deklarierten Abhängigkeitsbaum und einer ausgestellten Original-Berechtigung ausführen. Auf dem Live-Baum wurde noch kein Repository-Target hermetisch ausgeführt und keines remote, also läuft heute nichts remote."),
}
for language, prefix in (("en", "- **Declare.**"), ("de", "- **Deklarieren.**")):
    swap("hermetic-" + language, language, prefix, [HERMETIC[language]])

# 7. troubleshooting: a command waits
WAIT = {
    "en": ("Do not kill a holder that is making progress; read the head ticket first.",
           "Do not kill a holder that is making progress; read the head ticket first. A line that starts with `[capacity]` comes from the host capacity governor, not from a lease: it names the reason and your queue position (Host capacity); {{leases}} A dependency sync or a cold graph build holds its image or its facts lease for a long time (Leases, lock order and queues)."),
    "de": ("Beende keinen Halter, der vorankommt; lies zuerst das Kopf-Ticket.",
           "Beende keinen Halter, der vorankommt; lies zuerst das Kopf-Ticket. Eine Zeile, die mit `[capacity]` beginnt, stammt von der Kapazitätsregelung des Rechners und nicht von einem Lease: Sie nennt den Grund und deine Position in der Warteschlange (Rechnerkapazität); {{leases}} Ein Dependency-Sync oder ein kalter Graph-Build hält sein Image oder seinen Facts-Lease lange (Leases, Sperrreihenfolge und Warteschlangen)."),
}
LEASES_HINT = {
    "en": ("`repo leases status` lists who holds what and who waits for whom.", "the `lease-queue` line names the holder's process id."),
    "de": ("`repo leases status` listet auf, wer was hält und wer auf wen wartet.", "die Zeile `lease-queue` nennt die Prozess-ID des Halters."),
}
for language, prefix in (("en", "- **A command waits.**"), ("de", "- **Ein Befehl wartet.**")):
    old, new = WAIT[language]
    new = new.replace("{{leases}}", LEASES_HINT[language][0] if states["leases"] else LEASES_HINT[language][1])
    swap("wait-" + language, language, prefix, [(old, new)])

# 8. the incremental recipe points at the judge
POINTER = {
    "en": (" (the workflow has not run yet).", " (the workflow has not run yet). `repo incremental` repeats this proof in a disposable clone for both tools side by side and judges the executed tasks against the declared inputs (Incremental development)."),
    "de": (" (der Workflow ist noch nie gelaufen).", " (der Workflow ist noch nie gelaufen). `repo incremental` wiederholt diesen Beweis in einer Wegwerf-Kopie für beide Werkzeuge nebeneinander und beurteilt die ausgeführten Tasks gegen die deklarierten Inputs (Inkrementelle Entwicklung)."),
}
for language, prefix in (("en", "An incremental build re-runs exactly the tasks"), ("de", "Ein inkrementeller Build führt genau die Tasks")):
    swap("pointer-" + language, language, prefix, [POINTER[language]])

# 9. verified and not yet verified
VERIFIED = {
    "en": "Verified in copies of the tree on 2026-10-20, not yet through `repo` on the live checkout: the lock order with real child processes (deadlock victims, starvation, a wait-for graph against a SQLite oracle), Bazel server slots with real Bazel 8.4.2, Cargo lanes and the unit cache with real Cargo and sccache as oracle, graph generations against an independent Python model, the Nx graph receipt and concurrent writers against stock Nx 23.2.0, concurrent dependency generations with real Bun, the cache ledger against a SQLite oracle, TypeScript check units against `tsc -p`, the host capacity decision against an independent SQLite implementation, the incremental and concurrency judges (29 and 26 tests, with real Bazel), `repo verify build-parity` with fixtures and a fake `repo`, the boot sweep with real `hdiutil` and SIGKILL injection, and the test sandbox (16 tests; its law finds the same 1,042 findings in TypeScript and in a separate Python reference). Not yet verified: a production `repo --tool=bazel run`, a cold `repo show projects` on the new graph code, `repo incremental prepare` and its first judged scenarios, the first `repo concurrency run`, `repo verify build-parity` against the real tools, every staged set on the live tree, and Linux, Windows and the devcontainer for all of them.",
    "de": "Verifiziert in Kopien des Baums am 2026-10-20, noch nicht über `repo` im Live-Checkout: die Sperrreihenfolge mit echten Kindprozessen (Deadlock-Opfer, Aushungern, ein Wait-for-Graph gegen ein SQLite-Orakel), Bazel-Server-Slots mit echtem Bazel 8.4.2, Cargo-Lanes und der Compile-Unit-Cache mit echtem Cargo und sccache als Orakel, Graph-Generationen gegen ein unabhängiges Python-Modell, der Nx-Graph-Beleg und gleichzeitige Schreiber gegen unverändertes Nx 23.2.0, gleichzeitige Abhängigkeits-Generationen mit echtem Bun, das Cache-Ledger gegen ein SQLite-Orakel, TypeScript-Prüfeinheiten gegen `tsc -p`, die Entscheidung der Rechnerkapazität gegen eine unabhängige SQLite-Implementierung, die Richter für Inkrementalität und Nebenläufigkeit (29 und 26 Tests, mit echtem Bazel), `repo verify build-parity` mit Fixtures und einem Schein-`repo`, der Boot-Sweep mit echtem `hdiutil` und SIGKILL-Injektion sowie die Test-Sandbox (16 Tests; ihr Gesetz findet in TypeScript und in einer getrennten Python-Referenz dieselben 1.042 Befunde). Noch nicht verifiziert: ein Produktionslauf `repo --tool=bazel run`, ein kalter `repo show projects` auf dem neuen Graph-Code, `repo incremental prepare` und seine ersten beurteilten Szenarien, der erste `repo concurrency run`, `repo verify build-parity` gegen die echten Werkzeuge, jedes vorbereitete Set auf dem Live-Baum sowie Linux, Windows und der Devcontainer für alles davon.",
}
for language, prefix in (("en", "Verified on 2026-10-20 (macOS arm64"), ("de", "Verifiziert am 2026-10-20 (macOS arm64")):
    row = line(prefix, language)
    add("verified-" + language, row, row + "\n\n" + VERIFIED[language])

base = hashlib.sha256(raw).hexdigest()
out = {"readme": lib.PRODUCT + "/README.md", "base": base, "probes": {k: bool(v) for k, v in states.items()}, "edits": edits, "deferred": deferred}
os.makedirs(os.path.join(HERE, "staged"), exist_ok=True)
json.dump(out, open(os.path.join(HERE, "staged", "edits.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print(f"README {base[:16]}: {len(edits)} edits, {len(deferred)} deferred ({', '.join(d['id'] for d in deferred) or '-'})")
print("states: " + ", ".join(f"{k}={'live' if v else 'staged'}" for k, v in states.items()))
