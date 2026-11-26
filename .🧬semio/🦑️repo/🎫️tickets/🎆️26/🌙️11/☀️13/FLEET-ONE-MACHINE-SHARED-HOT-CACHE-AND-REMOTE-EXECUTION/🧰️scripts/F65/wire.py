#!/usr/bin/env python3
"""F65: wires `repo typecheck` into the catalogue, door, selection, requires table, MCP surface and nx input policy by anchored, atomic edits (re-reads every live file right before editing)."""
import json,os,sys
R="/Users/ueli/Documents/semio/"
B=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
M=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/"
def edit(path,fn):
    s=open(path,encoding="utf8").read();t=fn(s)
    if t==s:print("unchanged",path[-40:]);return
    tmp=path+".f65.tmp";open(tmp,"w",encoding="utf8").write(t);os.replace(tmp,path);print("edited",path[-50:])
def once(s,a,b):
    assert s.count(a)==1,(a,s.count(a));return s.replace(a,b)
def catalog(s):
    if '"name": "typecheck"' in s:return s
    s=once(s,'''      "name": "--bail",
      "value": "none",''','''      "name": "--no-importers",
      "value": "none",
      "operations": ["typecheck"],
      "description": { "en": "Check only the named files, not the files that import them.", "de": "Prüft nur die genannten Dateien, nicht die Dateien, die sie importieren." }
    },
    {
      "name": "--all-errors",
      "value": "none",
      "operations": ["typecheck"],
      "description": { "en": "Count every compiler error of the program that was read, also in files the call did not name or import-link; by default only errors in the named files and their importers fail the call.", "de": "Zählt jeden Compilerfehler des gelesenen Programms, auch in Dateien, die der Aufruf weder nennt noch über Importe verbindet; standardmäßig scheitern nur Fehler in den genannten Dateien und ihren Importeuren." }
    },
    {
      "name": "--declarations",
      "value": "none",
      "operations": ["typecheck"],
      "description": { "en": "Also compare every hand-written .d.mts below the library with its .mjs (names and parameter counts), not only the touched pairs; no path is needed then.", "de": "Vergleicht zusätzlich jede handgeschriebene .d.mts der Bibliothek mit ihrer .mjs (Namen und Parameterzahl), nicht nur die berührten Paare; dann ist kein Pfad nötig." }
    },
    {
      "name": "--bail",
      "value": "none",''')
    s=once(s,'"operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache", "doctor", "test-file"],','"operations": ["run", "run-many", "affected", "exec", "cache", "remote-cache", "doctor", "test-file", "typecheck"],')
    s=once(s,'Mit test-file: gibt den Ablauf jeder Datei aus und führt nichts aus."','Mit test-file: gibt den Ablauf jeder Datei aus und führt nichts aus. Mit typecheck: nennt die zu prüfenden Dateien und führt den Compiler nicht aus."')
    s=once(s,'With test-file: print the route of every file and run nothing.','With test-file: print the route of every file and run nothing. With typecheck: list the files to check and do not run the compiler.')
    s=once(s,'"conformance", "units", "service", "session", "test-file"],','"conformance", "units", "service", "session", "test-file", "typecheck"],')
    entry='''    {
      "name": "typecheck",
      "requires": "none",
      "synopsis": "typecheck [--json] [--dry-run] [--no-importers] [--all-errors] [--declarations] [--] <path>...",
      "summary": { "en": "Type-check the files a change touched and the files that import them in seconds, without reading the project graph, and compare every touched hand-written .d.mts with its .mjs.", "de": "Prüft die Typen der geänderten Dateien und der Dateien, die sie importieren, in Sekunden ohne den Projektgraphen zu lesen, und vergleicht jede berührte handgeschriebene .d.mts mit ihrer .mjs." },
      "dryRun": true,
      "progress": true,
      "cancellable": true,
      "flags": ["--json", "--dry-run", "--no-importers", "--all-errors", "--declarations"],
      "examples": [
        {
          "command": "repo typecheck path/to/changed-file.ts",
          "description": { "en": "Check one file and its direct importers; exit 0 when no error lies in them and no declaration drifts.", "de": "Prüft eine Datei und ihre direkten Importeure; Exit 0, wenn darin kein Fehler liegt und keine Deklaration abweicht." }
        },
        {
          "command": "repo typecheck path/to/folder",
          "description": { "en": "Check every TypeScript source below a folder and its importers.", "de": "Prüft jede TypeScript-Quelle unterhalb eines Ordners und ihre Importeure." }
        },
        {
          "command": "repo typecheck --declarations",
          "description": { "en": "Compare every hand-written .d.mts of the library with its .mjs; no compiler call.", "de": "Vergleicht jede handgeschriebene .d.mts der Bibliothek mit ihrer .mjs; kein Compiler-Aufruf." }
        }
      ],
      "notes": [
        {
          "en": "One incremental compiler call per project (the library with the process module, else the workspace configuration) over exactly the named files plus the files that import them directly (at most 400, else only the named files); the build state and the import index live in the machine cache (SEMIO_TYPECHECK_CACHE overrides it), so a repeated call takes seconds and nothing in the checkout is written. A path is resolved against the current directory when it exists there, else against the checkout; a folder stands for every source below it, a .mjs for its .d.mts. Errors in other files the program reads are counted as elsewhere and do not fail the call unless --all-errors is given. A .d.mts must declare every export of its .mjs and no function, constant or class the .mjs lacks, with the same parameter count. Exit code: 0 clean, 1 errors or drift, 2 refused.",
          "de": "Ein inkrementeller Compiler-Aufruf je Projekt (die Bibliothek mit dem Prozessmodul, sonst die Workspace-Konfiguration) über genau die genannten Dateien und die Dateien, die sie direkt importieren (höchstens 400, sonst nur die genannten); der Build-Zustand und der Importindex liegen im Maschinen-Cache (SEMIO_TYPECHECK_CACHE überschreibt ihn), sodass ein wiederholter Aufruf Sekunden dauert und nichts im Checkout geschrieben wird. Ein Pfad wird gegen das aktuelle Verzeichnis aufgelöst, wenn er dort existiert, sonst gegen den Checkout; ein Ordner steht für jede Quelle darunter, eine .mjs für ihre .d.mts. Fehler in anderen Dateien, die das Programm liest, zählen als anderswo und lassen den Aufruf nicht scheitern, außer mit --all-errors. Eine .d.mts muss jeden Export ihrer .mjs deklarieren und keine Funktion, Konstante oder Klasse, die der .mjs fehlt, mit gleicher Parameterzahl. Exit-Code: 0 sauber, 1 Fehler oder Abweichung, 2 abgelehnt."
        }
      ]
    },
'''
    return once(s,'''    {
      "name": "exec",
      "requires": "dependencies",''',entry+'''    {
      "name": "exec",
      "requires": "dependencies",''')
def door_json(s):
    if "typecheck" in s:return s
    return once(s,'''{ "operation": "test-file", "export": "runTestFileCliV1", "call": "root-args-environment" }''','''{ "operation": "test-file", "export": "runTestFileCliV1", "call": "root-args-environment" },
    { "operation": "typecheck", "export": "runTypecheckCliV1", "call": "root-args-environment" }''')
def door_ts(s):
    if "typecheck" in s:return s
    return once(s,''' "test-file":()=>import("../🔬️test-file/⌨️command/🟦️.ts"),''',''' "test-file":()=>import("../🔬️test-file/⌨️command/🟦️.ts"),
 typecheck:()=>import("../🔬️typecheck/⌨️command/🟦️.ts"),''')
def selection(s):
    if '"typecheck"' in s:return s
    return once(s,'"scheduler", "test-file"]','"scheduler", "test-file", "typecheck"]')
def requires(s):
    if '"typecheck"' in s:return s
    return once(s,'"none": ["test-file",','"none": ["test-file", "typecheck",')
def policy(s):
    a='      "!{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/**/*",\n'
    b='      "!{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️typecheck/**/*",\n'
    return s if b in s else once(s,a,a+b)
def surface(s):
    if '"operation": "typecheck"' in s:return s
    entry='''    {
      "operation": "typecheck",
      "kind": "execute",
      "bind": { "--json": true },
      "positional": [
        {
          "name": "files",
          "argv": true,
          "description": {
            "en": "Source files or folders as an array of paths, relative to the checkout or absolute; they and the files that import them are type-checked incrementally in seconds without reading the project graph, and each touched .d.mts is compared with its .mjs. ",
            "de": "Quelldateien oder Ordner als Array von Pfaden, relativ zum Checkout oder absolut; sie und die Dateien, die sie importieren, werden inkrementell in Sekunden ohne den Projektgraphen auf Typen geprüft, und jede berührte .d.mts wird mit ihrer .mjs verglichen."
          }
        }
      ]
    },
'''
    return once(s,'    { "operation": "preflight", "kind": "query" },',entry+'    { "operation": "preflight", "kind": "query" },')
edit(B+"⚙️operations/🔣️catalog.json",catalog)
edit(B+"🚪️door/🔣️.json",door_json)
edit(B+"🚪️door/🟦️.ts",door_ts)
edit(B+"⌨️command/🧭️selection/🔣️.json",selection)
edit(B+"⚙️operations/🧪️tests/🛗️requires/🔣️.json",requires)
edit(B+"🔣️policy.json",policy)
edit(M+"🖼️assets/🏗️build-surface.json",surface)
for p in ["⚙️operations/🔣️catalog.json","🚪️door/🔣️.json","⌨️command/🧭️selection/🔣️.json","⚙️operations/🧪️tests/🛗️requires/🔣️.json","🔣️policy.json"]:json.load(open(B+p,encoding="utf8"))
json.load(open(M+"🖼️assets/🏗️build-surface.json",encoding="utf8"));print("json ok")
