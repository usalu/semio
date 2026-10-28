#!/usr/bin/env python3
"""[obs-g] applies the catalogue, door, law and mcp wiring of `repo fleet status` to a workspace tree (the workbench). usage: apply_ops.py <workspace dir>   Every edit is anchored (old text must occur exactly once) and idempotent."""
import json,sys,re
W=sys.argv[1].rstrip("/")+"/"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=LIB+"🏗️build/";MCP="🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/"
def edit(path,old,new,count=1):
    t=open(W+path,encoding="utf8").read()
    if new in t: print("already",path[-50:]);return
    assert t.count(old)==count,(path,old[:80],t.count(old))
    open(W+path,"w",encoding="utf8").write(t.replace(old,new));print("edited",path[-50:])
edit(B+"⚙️operations/🧬️schema/🔣️.json",'"leases", "remote-execution"','"leases", "fleet", "remote-execution"')
cat=B+"⚙️operations/🔣️catalog.json"
t=open(W+cat,encoding="utf8").read()
i=t.index('"name": "--json"');j=t.index("\n    },",i) if "\n    }," in t[i:i+900] else None
line_start=t.rfind("\n",0,t.index('"operations":',i))+1;line_end=t.index("\n",line_start);line=t[line_start:line_end]
assert '"leases"' in line and '"capacity"' in line,line
if '"fleet"' not in line:
    t=t[:line_start]+line.replace('"capacity", "leases"','"capacity", "fleet", "leases"')+t[line_end:];open(W+cat,"w",encoding="utf8").write(t);print("edited --json flag")
LIMIT='''    {
      "name": "--limit",
      "value": "required",
      "valueName": "n",
      "operations": ["fleet"],
      "description": { "en": "fleet status: how many runs the text output lists (default: 20, the JSON output always lists all).", "de": "fleet status: wie viele Läufe die Textausgabe auflistet (Standard: 20, die JSON-Ausgabe listet immer alle)." }
    },
'''
t=open(W+cat,encoding="utf8").read()
if '"name": "--limit"' not in t:
    anchor='    {\n      "name": "--violations",'
    assert t.count(anchor)==1;t=t.replace(anchor,LIMIT+anchor);open(W+cat,"w",encoding="utf8").write(t);print("added --limit flag")
BLOCK='''    {
      "name": "fleet",
      "requires": "none",
      "synopsis": "fleet status",
      "summary": {
        "en": "Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.",
        "de": "Zeigt ein Gesamtbild des gemeinsamen Arbeitsbereichs: wer was ausführt (Phase, Fortschritt, Alter, Priorität, Aktualität des Codes), die Warteschlangen mit Wartezeitschätzungen, zugesagter gegen tatsächlich genutzten Speicher, die Bazel-Server, den Speicher auf der Platte gegen seine Budgets, die versiegelte Implementierung und die letzte Graph-Generation."
      },
      "dryRun": false,
      "progress": false,
      "cancellable": false,
      "subcommands": [
        {
          "name": "status",
          "synopsis": "fleet status [--json] [--limit=<n>]",
          "summary": {
            "en": "Print the fleet picture as compact text, or with --json as one document whose schema is validated before it is printed.",
            "de": "Gibt das Gesamtbild als kompakten Text aus, mit --json als ein Dokument, dessen Schema vor der Ausgabe geprüft wird."
          }
        }
      ],
      "flags": [
        "--json",
        "--limit"
      ],
      "examples": [
        {
          "command": "repo fleet status",
          "description": { "en": "See at a glance who runs what, who waits for what and for how long, and what the disk and memory budgets look like.", "de": "Zeigt auf einen Blick, wer was ausführt, wer worauf wie lange wartet und wie es um die Speicher- und Plattenbudgets steht." }
        },
        {
          "command": "repo fleet status --json",
          "description": { "en": "The same picture as one machine-readable document for an agent.", "de": "Dasselbe Bild als ein maschinenlesbares Dokument für einen Agenten." }
        }
      ],
      "notes": [
        {
          "en": "The command answers from the existing status readers of the leases, the capacity gate, the Bazel slots, the cache ledger, the sealed implementation store and the graph generation pointer in parallel; it takes no lease, loads no graph, starts no build and answers within a second on a loaded machine. A reader that fails is named under the sources and leaves only its own section out.",
          "de": "Der Befehl antwortet aus den vorhandenen Statusquellen der Leases, des Kapazitäts-Tors, der Bazel-Slots, des Cache-Verzeichnisses, des versiegelten Implementierungsspeichers und des Zeigers der Graph-Generation gleichzeitig; er nimmt kein Lease, lädt keinen Graphen, startet keinen Build und antwortet auch auf einem ausgelasteten Rechner innerhalb einer Sekunde. Eine Quelle, die ausfällt, steht unter den Quellen und lässt nur ihren eigenen Abschnitt aus."
        },
        {
          "en": "Waiting-time estimates add the time left of the holder (its own progress counter, else earlier holds of the same kind) to the typical hold of everyone ahead; a memory waiter is admitted once the memory of the running grants, released after the typical duration of their operation, covers its need. No estimate is printed where nothing is known.",
          "de": "Wartezeitschätzungen addieren die Restzeit des Halters (sein eigener Fortschrittszähler, sonst frühere Haltezeiten derselben Art) zur üblichen Haltezeit aller Vorderen; ein Speicher-Wartender wird zugelassen, sobald der Speicher der laufenden Zusagen, nach der üblichen Dauer ihrer Operation freigegeben, seinen Bedarf deckt. Wo nichts bekannt ist, wird keine Schätzung gedruckt."
        }
      ]
    },
'''
t=open(W+cat,encoding="utf8").read()
if '"name": "fleet"' not in t:
    anchor='    {\n      "name": "service",\n      "requires": "none",\n      "synopsis": "service status|start'
    assert t.count(anchor)==1;t=t.replace(anchor,BLOCK+anchor);open(W+cat,"w",encoding="utf8").write(t);print("added fleet operation")
json.loads(open(W+cat,encoding="utf8").read())
edit(B+"⚙️operations/🧪️tests/🛗️requires/🔣️.json",'"capacity", "leases", "service"','"capacity", "leases", "fleet", "service"')
edit(B+"🚪️door/🔣️.json",'    { "operation": "capacity", "export": "runCapacityCommandV1", "call": "args-options" }\n','    { "operation": "capacity", "export": "runCapacityCommandV1", "call": "args-options" },\n    { "operation": "fleet", "export": "runFleetCliV1", "call": "root-args-environment" }\n')
edit(B+"🚪️door/🔣️.json",'"capacity", "leases", "remote-execution"]','"capacity", "leases", "fleet", "remote-execution"]')
edit(B+"🚪️door/🟦️.ts",' capacity:()=>import("../🌡️host-capacity/⌨️command/🟦️.ts"),\n',' capacity:()=>import("../🌡️host-capacity/⌨️command/🟦️.ts"),\n fleet:()=>import("../🚢️fleet-status/⌨️command/🟦️.ts"),\n')
# mcp
pol=MCP+"🖼️assets/🏗️build-surface.json"
edit(pol,'    { "operation": "capacity", "subcommand": "status", "kind": "query", "bind": { "--json": true } },\n','    { "operation": "capacity", "subcommand": "status", "kind": "query", "bind": { "--json": true } },\n    { "operation": "fleet", "subcommand": "status", "kind": "query", "flags": [], "bind": { "--json": true } },\n')
fx=MCP+"🧪️tests/🧫️fixtures/🏗️build-surface.json"
edit(fx,'    "exec",\n    "graph",','    "exec",\n    "fleet_status",\n    "graph",')
edit(fx,'    {\n      "id": "cache-status",\n      "tool": "cache_status",','    {\n      "id": "fleet",\n      "tool": "fleet_status",\n      "arguments": {},\n      "valid": true,\n      "argv": [\n        "fleet",\n        "status",\n        "--json"\n      ],\n      "environment": {},\n      "agent": ""\n    },\n    {\n      "id": "cache-status",\n      "tool": "cache_status",')
# help snapshots are generated by apply_help.ts (needs the rendered help of the catalogue above)
# suite route and project target of the fleet status tests
edit(B+"🗂️suites/🔣️.json",'''    {
      "id": "build-generation-claims",''','''    {
      "id": "build-fleet-status",
      "budgetMs": 300000,
      "typecheck": "none",
      "trees": [
        "🚢️fleet-status"
      ]
    },
    {
      "id": "build-generation-claims",''')
cachep=LIB+"⚡️caching/📋️project.json"
t=open(W+cachep,encoding="utf8").read()
if '"test-build-fleet-status"' not in t:
    marker='    "test-build-generation-claims": '
    assert t.count(marker)==1
    i=t.index(marker);j=t.index("\n",i)
    row=t[i:j].replace("test-build-generation-claims","test-build-fleet-status").replace("test build-generation-claims","test build-fleet-status")
    t=t[:i]+row+"\n"+t[i:];open(W+cachep,"w",encoding="utf8").write(t);print("added project target")
