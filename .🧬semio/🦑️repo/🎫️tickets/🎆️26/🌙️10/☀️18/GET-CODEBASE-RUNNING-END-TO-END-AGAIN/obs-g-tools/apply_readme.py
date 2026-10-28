#!/usr/bin/env python3
"""[obs-g] adds `repo fleet status` to the README of the repo product (EN + DE): command list, command table, feature table and one chapter each. usage: apply_readme.py <workspace dir>"""
import sys
p=sys.argv[1].rstrip("/")+"/🧰️framework/🛍️products/🦑️repo/README.md"
t=open(p,encoding="utf8").read()
if "### 🚢️ Fleet status" in t: sys.exit("already applied")
def put(old,new):
    global t
    assert t.count(old)==1,(old[:70],t.count(old))
    t=t.replace(old,old+new if new.startswith("\n") else new)
def after(old,add): 
    global t
    assert t.count(old)==1,(old[:70],t.count(old));t=t.replace(old,old+add)
def before(old,add):
    global t
    assert t.count(old)==1,(old[:70],t.count(old));t=t.replace(old,add+old)
LEASES_EN="repo leases status [--json]                          # holders and waiters of leases, lock-order violations, deadlock victims\n"
after(LEASES_EN,"repo fleet status [--json]                           # one picture of the shared workspace: runs, queues with waiting-time estimates, memory, Bazel servers, storage, sealed implementation, graph generation\n")
LEASES_DE="repo leases status [--json]                          # Halter und Wartende von Leases, Verstöße gegen die Sperrreihenfolge, Deadlock-Opfer\n"
after(LEASES_DE,"repo fleet status [--json]                           # ein Gesamtbild des gemeinsamen Arbeitsbereichs: Läufe, Warteschlangen mit Wartezeitschätzungen, Speicher, Bazel-Server, Platte, versiegelte Implementierung, Graph-Generation\n")
ROW_EN="| `leases status` | holders, waiters, lock-order violations and deadlock victims of the lease system (see Leases, lock order and queues) | `repo leases status` |\n"
after(ROW_EN,"| `fleet status` | one read-only picture of the shared workspace for agents and humans (see Fleet status) | `repo fleet status --json` |\n")
ROW_DE="| `leases status` | Halter, Wartende, Verstöße gegen die Sperrreihenfolge und Deadlock-Opfer des Lease-Systems (siehe Leases, Sperrreihenfolge und Warteschlangen) | `repo leases status` |\n"
after(ROW_DE,"| `fleet status` | ein schreibgeschütztes Gesamtbild des gemeinsamen Arbeitsbereichs für Agenten und Menschen (siehe Flottenstatus) | `repo fleet status --json` |\n")
F_EN="| Lock order, deadlock detection, lease status | `repo leases status`, `SEMIO_LEASE_ORDER` | live |\n"
after(F_EN,"| One picture of the shared workspace | `repo fleet status`, MCP tool `fleet_status` | live |\n")
F_DE="| Sperrreihenfolge, Deadlock-Erkennung, Lease-Status | `repo leases status`, `SEMIO_LEASE_ORDER` | live |\n"
after(F_DE,"| Ein Gesamtbild des gemeinsamen Arbeitsbereichs | `repo fleet status`, MCP-Werkzeug `fleet_status` | live |\n")
CH_EN='''### 🚢️ Fleet status

Many agents and humans share one workspace. `repo fleet status` answers in one read what is going on, for people as compact text and for agents as one JSON document (the MCP tool `fleet_status` returns the same document). It is a light front-door command: it takes no lease, loads no graph, starts no build and writes nothing but the host sample cache that `repo capacity status` writes as well.

```bash
repo fleet status                 # compact text in the chosen language
repo fleet status --json          # one document; its schema is validated before it is printed
repo fleet status --limit=50      # list more runs in the text output
```

- **What it shows.** (1) *Runs*: every process the ledgers know, joined into one run per build (the invocation that holds the memory grant and the children that hold its leases), with state (running, blocked, waiting), operation, phase and counter, time left, age, priority class, the freshness of its code against the newest loaded code, and the memory admitted against what the process tree really holds. (2) *Queues*: the graph custody (builder, computers, slots, waiters), the memory queue and every lease queue, each waiter with an estimated waiting time. (3) *Memory*: host free, floor, spare, pressure, swap, load, admitted against really used. (4) *Bazel servers*: live servers, their memory and the slot that holds them. (5) *Storage*: free disk, the images against their capacity and the machine budget plan. (6) *Sealed implementation*: mode, snapshots, whether the live tree moved on, blocked promotions. (7) *Graph generation*: number, how it was built, age and how many owners the last verification covered. (8) *Findings*: stuck or stale holders, deadlock cycles, memory pressure, under-reserved runs, long waits, low disk, full images, blocked promotions, failed readers.
- **Where the data comes from.** Only existing status readers, in parallel: `leases status` (registry, queues, standing), the capacity gate (sample, grants, queue, learned run durations), the Bazel slot files and server pid files (`🧵️sessions/📖️status`), the cache ledger and machine budget (`cache status` without a tree walk), the sealed store and the published pointer and manifest of the graph generation (read as plain files under their schema; the owner table is counted, not walked). A reader that fails or does not answer within five seconds is named under *sources*, produces a finding and leaves only its own section out.
- **Waiting-time estimates.** A lease waiter waits for the time left of the holder (its own progress counter, else the median of earlier holds of the same kind) plus the typical hold of everyone ahead of it; a memory waiter is admitted once the memory of the running grants, released after the typical duration of their operation minus the time they ran, covers its need and every waiter ahead is admitted. Where nothing is known no number is printed.
- **Laws.** The aggregation equals an independent Python oracle on seeded facts in both languages and the closed report schema is checked by the in-repo validator and by Ajv (`🚢️fleet-status/🧪️tests`). A guard test runs the command in a sandbox workspace while a real process holds an exclusive lease: the command sees the holder, writes nothing except the capacity sample cache, opens no database, starts only the read-only host probes and leaves the lease directory and registry as they were; a negative control shows the same spy seeing a lease taken by the worker. The startup law (`🚪️door/🧪️tests/🚦️startup`) holds the command, entered through the front door, to loading no module of the graph group, no build invocation and no selection.

'''
before("### 🧵️ Sessions: Bazel slots, Cargo lanes and Nx pools",CH_EN)
CH_DE='''#### Flottenstatus

Viele Agenten und Menschen teilen einen Arbeitsbereich. `repo fleet status` beantwortet in einem Lesevorgang, was gerade geschieht: für Menschen als kompakter Text, für Agenten als ein JSON-Dokument (das MCP-Werkzeug `fleet_status` liefert dasselbe Dokument). Es ist ein leichter Befehl der Eingangstür: Er nimmt kein Lease, lädt keinen Graphen, startet keinen Build und schreibt nichts außer dem Stichproben-Cache des Rechners, den auch `repo capacity status` schreibt.

```bash
repo fleet status                 # kompakter Text in der gewählten Sprache
repo fleet status --json          # ein Dokument; sein Schema wird vor der Ausgabe geprüft
repo fleet status --limit=50      # mehr Läufe in der Textausgabe auflisten
```

- **Was es zeigt.** (1) *Läufe*: jeder Prozess, den die Verzeichnisse kennen, zu einem Lauf je Build zusammengefasst (der Aufruf mit der Speicherzusage und die Kinder, die seine Leases halten), mit Zustand (läuft, blockiert, wartet), Operation, Phase und Zähler, Restzeit, Alter, Prioritätsklasse, Aktualität seines Codes gegenüber dem neuesten geladenen Code und dem zugesagten gegenüber dem tatsächlich belegten Speicher des Prozessbaums. (2) *Warteschlangen*: die Graph-Obhut (Builder, Rechner, Slots, Wartende), die Speicher-Warteschlange und jede Lease-Warteschlange, jeder Wartende mit geschätzter Wartezeit. (3) *Speicher*: frei, Untergrenze, Reserve, Druck, Swap, Last, zugesagt gegenüber tatsächlich genutzt. (4) *Bazel-Server*: aktive Server, ihr Speicher und der Slot, der sie hält. (5) *Platte*: freier Platz, die Abbilder gegen ihre Kapazität und der Plan des Rechnerbudgets. (6) *Versiegelte Implementierung*: Modus, Abbilder, ob der Arbeitsbaum weitergezogen ist, blockierte Übernahmen. (7) *Graph-Generation*: Nummer, wie sie gebaut wurde, Alter und wie viele Besitzer die letzte Prüfung abdeckte. (8) *Befunde*: hängende oder veraltete Halter, Verklemmungszyklen, Speicherdruck, zu knapp zugesagte Läufe, lange Wartezeiten, wenig Platz, volle Abbilder, blockierte Übernahmen, ausgefallene Leser.
- **Woher die Daten kommen.** Nur aus vorhandenen Statusquellen, gleichzeitig: `leases status` (Verzeichnis, Warteschlangen, Zustand), das Kapazitäts-Tor (Stichprobe, Zusagen, Warteschlange, gelernte Laufzeiten), die Bazel-Slot-Dateien und Server-pid-Dateien (`🧵️sessions/📖️status`), das Cache-Verzeichnis samt Rechnerbudget (`cache status` ohne Baumdurchlauf), der versiegelte Speicher sowie Zeiger und Manifest der Graph-Generation (als einfache Dateien unter ihrem Schema gelesen; die Besitzertabelle wird gezählt, nicht durchlaufen). Eine Quelle, die ausfällt oder nicht innerhalb von fünf Sekunden antwortet, steht unter *Quellen*, erzeugt einen Befund und lässt nur ihren eigenen Abschnitt aus.
- **Wartezeitschätzungen.** Ein Lease-Wartender wartet auf die Restzeit des Halters (sein eigener Fortschrittszähler, sonst der Median früherer Haltezeiten derselben Art) plus die übliche Haltezeit aller Vorderen; ein Speicher-Wartender wird zugelassen, sobald der Speicher der laufenden Zusagen, nach der üblichen Dauer ihrer Operation abzüglich ihrer Laufzeit freigegeben, seinen Bedarf deckt und alle Vorderen zugelassen sind. Wo nichts bekannt ist, wird keine Zahl gedruckt.
- **Gesetze.** Die Aggregation gleicht auf gesäten Fakten in beiden Sprachen einem unabhängigen Python-Orakel, und das geschlossene Berichtsschema wird vom Validator des Repositorys und von Ajv geprüft (`🚢️fleet-status/🧪️tests`). Ein Wächter-Test führt den Befehl in einem Sandbox-Arbeitsbereich aus, während ein echter Prozess ein exklusives Lease hält: Der Befehl sieht den Halter, schreibt nichts außer dem Stichproben-Cache der Kapazität, öffnet keine Datenbank, startet nur schreibgeschützte Rechner-Abfragen und lässt Lease-Verzeichnis und Registratur unverändert; eine Negativkontrolle zeigt, dass derselbe Spion ein vom Arbeiter genommenes Lease sieht. Das Startgesetz (`🚪️door/🧪️tests/🚦️startup`) hält den Befehl, durch die Eingangstür betreten, davon ab, ein Modul der Graph-Gruppe, einen Build-Aufruf oder eine Auswahl zu laden.

'''
before("#### Sitzungen: Bazel-Slots, Cargo-Lanes und Nx-Pools",CH_DE)
open(p,"w",encoding="utf8").write(t);print("readme edited")
