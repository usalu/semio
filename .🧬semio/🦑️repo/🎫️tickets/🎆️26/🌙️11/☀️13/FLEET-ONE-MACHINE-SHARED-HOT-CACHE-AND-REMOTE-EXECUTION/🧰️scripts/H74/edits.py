"""H74: registers `repo land` in the operations catalogue, the front door, the command selection, the MCP build surface (+ golden vectors) and the build-test-file suite. Usage: edits.py ROOT (anchored, each anchor must match exactly once; idempotent guard on the operation name)."""
import json, sys
R = sys.argv[1]
M = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/"
B = M + "📚️library/🏗️build/"

def edit(path, pairs):
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (path, old[:80], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)

flag = lambda name, value, vname, en, de: '    {\n      "name": "%s",\n      "value": "%s",\n%s      "operations": ["land"],\n      "description": { "en": %s, "de": %s }\n    },\n' % (name, value, ('      "valueName": "%s",\n' % vname) if vname else "", json.dumps(en, ensure_ascii=False), json.dumps(de, ensure_ascii=False))
new_flags = (
 flag("--outage", "none", None, "Land outside the landing slot (a fix for a fleet-wide outage); every other gate still runs.", "Landet außerhalb des Landefensters (Behebung eines flottenweiten Ausfalls); jeder andere Prüfschritt läuft weiter.")
 + flag("--rebase", "none", None, "Rebase a staged file whose live version changed by a three-way merge instead of refusing; a conflict still refuses.", "Überträgt eine vorbereitete Datei, deren Live-Version sich geändert hat, per Drei-Wege-Zusammenführung statt abzulehnen; ein Konflikt lehnt weiterhin ab.")
 + flag("--no-wait", "none", None, "Outside the landing slot queue the changeset and stop (exit 3) instead of waiting for the next slot.", "Reiht den Änderungssatz außerhalb des Landefensters ein und endet (Exit 3), statt auf das nächste Fenster zu warten.")
 + flag("--bench", "none", None, "land: also run repo bench latency --strict as a gate (the command latency budgets of the live installation).", "land: führt zusätzlich repo bench latency --strict als Prüfschritt aus (die Latenzbudgets der installierten Befehle).")
 + flag("--title", "required", "title", "land: the title of the landings entry (default: the staged title).", "land: der Titel des Eintrags in den Landungen (Standard: der vorbereitete Titel).")
 + flag("--wave", "required", "wave", "land: the wave named in the landings entry header (default: SEMIO_LAND_WAVE, else Landing).", "land: die Welle im Kopf des Eintrags in den Landungen (Standard: SEMIO_LAND_WAVE, sonst Landing).")
)
land_op = '''    {
      "name": "land",
      "requires": "none",
      "synopsis": "land checkout <dir> <path>... | delete <dir> <path>... | [check] <dir> | rollback <id> | status [--json] [--dry-run] [--outage] [--rebase] [--no-wait] [--bench] [--lane <lane>] [--title <title>] [--wave <wave>]",
      "summary": { "en": "Land a staged changeset only once it holds: check it against a scratch overlay of the live tree, then apply all files atomically under the area locks, in the landing slot, with a rollback point and the landings entry.", "de": "Landet einen vorbereiteten Änderungssatz erst, wenn er hält: prüft ihn gegen ein Scratch-Overlay des Live-Baums und wendet dann alle Dateien atomar unter den Bereichssperren im Landefenster an, mit Rücksetzpunkt und Eintrag in den Landungen." },
      "dryRun": true,
      "progress": true,
      "cancellable": true,
      "flags": ["--json", "--dry-run", "--outage", "--rebase", "--no-wait", "--bench", "--lane", "--title", "--wave"],
      "examples": [
        {
          "command": "repo land checkout ./stage path/to/file.ts",
          "description": { "en": "Stage a live file: its content hash is remembered as the base and a working copy appears in ./stage/tree/.", "de": "Bereitet eine Live-Datei vor: ihr Inhalts-Hash wird als Basis gemerkt, eine Arbeitskopie erscheint in ./stage/tree/." }
        },
        {
          "command": "repo land ./stage",
          "description": { "en": "Check the changeset against the overlay (stale bases, dangling imports, type check, Rust check, suite law, declared tests), wait for the slot, apply atomically, record the rollback point and append the landings entry.", "de": "Prüft den Änderungssatz gegen das Overlay (veraltete Basen, hängende Importe, Typprüfung, Rust-Prüfung, Suite-Gesetz, deklarierte Tests), wartet auf das Fenster, wendet atomar an, zeichnet den Rücksetzpunkt auf und hängt den Eintrag an die Landungen an." }
        },
        {
          "command": "repo land check ./stage",
          "description": { "en": "Run every gate and apply nothing.", "de": "Führt jeden Prüfschritt aus und wendet nichts an." }
        },
        {
          "command": "repo land rollback <id>",
          "description": { "en": "Restore the files of an earlier landing when none of them changed since.", "de": "Stellt die Dateien einer früheren Landung wieder her, wenn sich seither keine geändert hat." }
        }
      ],
      "notes": [
        {
          "en": "A changeset is a staged directory: checkout copies live files into <dir>/tree and remembers their content hashes (the bases), new files are created in the tree, delete stages a removal. Nothing in the live tree is read-modified-written by the agent; the command compares every base with the live file (a changed file is a stale base and refuses, or with --rebase is merged three-way), builds a scratch overlay (a copy-on-write clone of the framework tree with the staged files applied, in the machine cache), runs the gates against that overlay and only then touches the live tree. Slot-bound areas (build, caching, process, manifests) land only in minutes 0 to 10 of each hour: otherwise the command waits for the slot (progress every minute, Ctrl-C cancels) or, with --no-wait, queues and exits 3; --outage lands at once. Tests and documents land any time. --dry-run (like check) runs every gate and applies nothing.",
          "de": "Ein Änderungssatz ist ein vorbereiteter Ordner: checkout kopiert Live-Dateien nach <dir>/tree und merkt ihre Inhalts-Hashes (die Basen), neue Dateien entstehen im Baum, delete bereitet ein Löschen vor. Der Agent liest und schreibt nichts im Live-Baum; der Befehl vergleicht jede Basis mit der Live-Datei (eine geänderte Datei ist eine veraltete Basis und lehnt ab, mit --rebase wird sie Drei-Wege-zusammengeführt), baut ein Scratch-Overlay (einen Copy-on-Write-Klon des Framework-Baums mit den vorbereiteten Dateien, im Maschinen-Cache), führt die Prüfschritte gegen dieses Overlay aus und berührt erst danach den Live-Baum. Fensterpflichtige Bereiche (build, caching, process, Manifeste) landen nur in den Minuten 0 bis 10 jeder Stunde: sonst wartet der Befehl auf das Fenster (Fortschritt jede Minute, Strg-C bricht ab) oder reiht mit --no-wait ein und endet mit 3; --outage landet sofort. Tests und Dokumente landen jederzeit. --dry-run (wie check) führt jeden Prüfschritt aus und wendet nichts an."
        },
        {
          "en": "The apply writes and syncs every new content to a hidden temporary file beside its target first, then takes landing.lock.<area> for every area in alphabetical order, renames the files (new files first, every imported file before its importer, deletions last) while landing.generation.<area> is odd, and releases the locks at once. A reader that needs one consistent view reads the generation before and after and retries unless both reads are equal and even; every other reader sees only whole old or whole new files and never an import of a missing file. The rollback point (repo land rollback <id>) and the append to landings.md are written for every landing. Exit code 0 landed, checked or rolled back, 1 a gate failed, 2 refused (stale base, locks busy or stale, usage, cancelled), 3 queued.",
          "de": "Beim Anwenden wird jeder neue Inhalt zuerst in eine versteckte temporäre Datei neben dem Ziel geschrieben und synchronisiert, dann wird landing.lock.<Bereich> für jeden Bereich in alphabetischer Reihenfolge genommen, die Dateien werden umbenannt (neue Dateien zuerst, jede importierte Datei vor ihrem Importeur, Löschungen zuletzt), während landing.generation.<Bereich> ungerade ist, und die Sperren werden sofort freigegeben. Ein Leser, der eine konsistente Sicht braucht, liest die Generation davor und danach und wiederholt, außer beide Werte sind gleich und gerade; jeder andere Leser sieht nur ganze alte oder ganze neue Dateien und nie einen Import einer fehlenden Datei. Der Rücksetzpunkt (repo land rollback <Kennung>) und der Anhang an landings.md entstehen bei jeder Landung. Exit-Code 0 gelandet, geprüft oder zurückgesetzt, 1 ein Prüfschritt fehlgeschlagen, 2 abgelehnt (veraltete Basis, Sperren belegt oder veraltet, Aufruf, abgebrochen), 3 eingereiht."
        }
      ]
    },
'''
cat = B + "⚙️operations/🔣️catalog.json"
edit(cat, [
 ('    {\n      "name": "--no-importers",', new_flags + '    {\n      "name": "--no-importers",'),
 ('"operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache", "doctor", "test-file", "typecheck"],\n      "description": {\n        "en": "Print the task graph', '"operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache", "doctor", "test-file", "typecheck", "land"],\n      "description": {\n        "en": "Print the task graph'),
 ('"scheduler", "leases", "cache", "remote-cache", "doctor", "tools", "conformance", "units", "service", "session", "test-file", "typecheck"],\n      "description": { "en": "Print lists as JSON', '"scheduler", "leases", "cache", "remote-cache", "doctor", "tools", "conformance", "units", "service", "session", "test-file", "typecheck", "land"],\n      "description": { "en": "Print lists as JSON'),
 ('"valueName": "lane",\n      "operations": ["scheduler"],\n      "description": { "en": "scheduler run: shared compiler lane (rustc or tsc) whose concurrency is capped.", "de": "scheduler run: gemeinsame Compiler-Spur (rustc oder tsc), deren Parallelität begrenzt ist." }', '"valueName": "lane",\n      "operations": ["scheduler", "land"],\n      "description": { "en": "scheduler run: shared compiler lane (rustc or tsc) whose concurrency is capped; land: the lane name of the landings entry and the lock owner (default: SEMIO_LAND_LANE, else agent).", "de": "scheduler run: gemeinsame Compiler-Spur (rustc oder tsc), deren Parallelität begrenzt ist; land: der Spurname im Eintrag der Landungen und als Sperreninhaber (Standard: SEMIO_LAND_LANE, sonst agent)." }'),
 ('    {\n      "name": "exec",\n      "requires": "dependencies",\n      "synopsis": "exec --projects', land_op + '    {\n      "name": "exec",\n      "requires": "dependencies",\n      "synopsis": "exec --projects'),
])
edit(B + "⚙️operations/🧬️schema/🔣️.json", [('"typecheck", "exec", "watch"', '"typecheck", "land", "exec", "watch"')])
edit(B + "⚙️operations/🧪️tests/🧫️fixtures/🧭️scoping/🔣️.json", [('"noteCount": 51,', '"noteCount": 53,')])
edit(B + "🚪️door/🟦️.ts", [(' typecheck:()=>import("../🔬️typecheck/⌨️command/🟦️.ts"),\n', ' typecheck:()=>import("../🔬️typecheck/⌨️command/🟦️.ts"),\n land:()=>import("../🚉️land/⌨️command/🟦️.ts"),\n')])
edit(B + "🚪️door/🔣️.json", [('{ "operation": "typecheck", "export": "runTypecheckV1Cli', 'x')] if False else [('    { "operation": "typecheck", "export": "runTypecheckCliV1", "call": "root-args-environment" }\n', '    { "operation": "typecheck", "export": "runTypecheckCliV1", "call": "root-args-environment" },\n    { "operation": "land", "export": "runLandCliV1", "call": "root-args-environment" }\n')])
edit(B + "⌨️command/🧭️selection/🔣️.json", [('"test-file", "typecheck"]}', '"test-file", "typecheck", "land"]}')])
edit(B + "⌨️command/🧭️selection/🧬️schema/🔣️.json", [('"test-file", "typecheck"]}', '"test-file", "typecheck", "land"]}')])
edit(B + "🗂️suites/🔣️.json", [('        "🔬️test-file",\n        "🔬️typecheck"\n      ],', '        "🔬️test-file",\n        "🔬️typecheck",\n        "🚉️land"\n      ],')])
edit(M + "🔌️mcp/🖼️assets/🏗️build-surface.json", [('''    {
      "operation": "typecheck",
      "kind": "execute",''', '''    {
      "operation": "land",
      "kind": "execute",
      "bind": { "--json": true },
      "positional": [
        {
          "name": "words",
          "argv": true,
          "description": {
            "en": "The words of repo land as an array: checkout <dir> <path>... | delete <dir> <path>... | [check] <dir> | rollback <id> | status. A staged changeset is checked against a scratch overlay of the live tree (stale bases, dangling imports, type check, Rust check, suite law, declared tests) and then applied atomically under the area locks inside the landing slot; the result lists the gates, the files and the rollback point.",
            "de": "Die Wörter von repo land als Array: checkout <Ordner> <Pfad>... | delete <Ordner> <Pfad>... | [check] <Ordner> | rollback <Kennung> | status. Ein vorbereiteter Änderungssatz wird gegen ein Scratch-Overlay des Live-Baums geprüft (veraltete Basen, hängende Importe, Typprüfung, Rust-Prüfung, Suite-Gesetz, deklarierte Tests) und dann atomar unter den Bereichssperren im Landefenster angewendet; das Ergebnis führt die Prüfschritte, die Dateien und den Rücksetzpunkt auf."
          }
        }
      ]
    },
    {
      "operation": "typecheck",
      "kind": "execute",''')])
g = M + "🔌️mcp/🧪️tests/🧫️fixtures/🏗️build-surface.json"
edit(g, [('    "inputs",\n    "leases_status",', '    "inputs",\n    "land",\n    "leases_status",')])
vectors = [
 {"id": "land-basic", "tool": "land", "arguments": {"words": ["./stage"]}, "valid": True, "argv": ["land", "--json", "--", "./stage"], "environment": {}, "agent": ""},
 {"id": "land-flags", "tool": "land", "arguments": {"words": ["check", "./stage"], "dry_run": True, "outage": True, "rebase": True, "no_wait": True, "bench": True, "lane": "H1", "title": "T", "wave": "Wave 1"}, "valid": True, "argv": ["land", "--json", "--dry-run", "--outage", "--rebase", "--no-wait", "--bench", "--lane", "H1", "--title", "T", "--wave", "Wave 1", "--", "check", "./stage"], "environment": {}, "agent": ""},
 {"id": "land-no-words", "tool": "land", "arguments": {}, "valid": False},
 {"id": "land-empty-words", "tool": "land", "arguments": {"words": []}, "valid": False},
]
block = ",\n".join("    " + json.dumps(v, ensure_ascii=False, indent=2).replace("\n", "\n    ") for v in vectors)
edit(g, [('\n    }\n  ]\n}\n', '\n    },\n' + block + '\n  ]\n}\n')])
print("edits ok")
