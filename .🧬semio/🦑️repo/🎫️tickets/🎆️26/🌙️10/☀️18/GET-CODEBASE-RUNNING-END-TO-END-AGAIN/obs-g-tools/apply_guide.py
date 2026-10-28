#!/usr/bin/env python3
"""[obs-g] replaces the three 'fleet status: not landed / in progress' lines of the build guide (EN + DE) by the landed command. usage: apply_guide.py <workspace dir>. Anchored and idempotent."""
import sys
p=sys.argv[1].rstrip("/")+"/🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
t=open(p,encoding="utf8").read()
rows=[
("- **Fleet status**: not landed. Today read `bun repo leases status`, `bun repo capacity status`, `bun repo session list` and `bun repo doctor`.",
 "- **Fleet status**: `bun repo fleet status` (`--json`, `--limit=<n>`; MCP tool `fleet_status`) is one read-only picture in under a second without a lease or a graph: who runs what (phase, counter, time left, age, priority, code freshness, memory admitted against really used), every queue with a waiting-time estimate, Bazel servers, storage against its budgets, sealed implementation, graph generation and findings (stuck holders, pressure, long waits, low disk). Start here, then `bun repo leases status` or `bun repo capacity status` for detail."),
("- **Flottenstatus**: nicht eingespielt. Heute `bun repo leases status`, `bun repo capacity status`, `bun repo session list` und `bun repo doctor` lesen.",
 "- **Flottenstatus**: `bun repo fleet status` (`--json`, `--limit=<n>`; MCP-Werkzeug `fleet_status`) ist ein schreibgeschütztes Gesamtbild in unter einer Sekunde ohne Lease und ohne Graph: wer was ausführt (Phase, Zähler, Restzeit, Alter, Priorität, Aktualität des Codes, zugesagter gegen tatsächlich genutzten Speicher), jede Warteschlange mit Wartezeitschätzung, Bazel-Server, Speicher gegen seine Budgets, versiegelte Implementierung, Graph-Generation und Befunde (festhängende Halter, Druck, lange Wartezeiten, wenig Platz). Hier anfangen, danach `bun repo leases status` oder `bun repo capacity status` für Einzelheiten."),
("- In progress: fleet status (obs-g), cross-platform CI for the suite tables (ci-g).","- In progress: cross-platform CI for the suite tables (ci-g)."),
("- In Arbeit: Flottenstatus (obs-g), plattformübergreifende CI für die Suite-Tabellen (ci-g).","- In Arbeit: plattformübergreifende CI für die Suite-Tabellen (ci-g)."),
]
done=0
for old,new in rows:
    if new in t: continue
    assert t.count(old)==1,(old[:60],t.count(old));t=t.replace(old,new);done+=1
open(p,"w",encoding="utf8").write(t);print("guide rows applied:",done)
